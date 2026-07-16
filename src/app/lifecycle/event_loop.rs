use std::time::{Duration, SystemTime};

use gpui::{Context, Window, px};
use gpui_component::input::{InputEvent, InputState};
use rust_i18n::t;

use crate::{
    AxShell,
    app::{WorkspacePage, state::lifecycle::WindowLifecycleState},
    events::{BACKEND_EVENT_QUEUE_CAPACITY, BackendEvent},
};

const IDLE_PUMP_INTERVAL: Duration = Duration::from_millis(33);
const BACKGROUND_PUMP_INTERVAL: Duration = Duration::from_millis(250);
const DEEP_SLEEP_PUMP_INTERVAL: Duration = Duration::from_secs(1);
const BACKGROUND_REFRESH_INTERVAL: Duration = Duration::from_millis(250);
const DEEP_SLEEP_REFRESH_INTERVAL: Duration = Duration::from_secs(1);
const FOREGROUND_SFTP_IDLE_SWEEP_INTERVAL: Duration = Duration::from_secs(1);
const BACKGROUND_SFTP_IDLE_SWEEP_INTERVAL: Duration = Duration::from_secs(30);
const IDLE_NOTIFY_INTERVAL_TICKS: u32 = 30;
const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(600);

#[derive(Clone, Copy, Default)]
struct DrainResult {
    terminal_changed: bool,
    ui_changed: bool,
}

// Output batches end before every non-output event so status and close events
// observe all preceding terminal bytes.
#[derive(Default)]
struct TerminalOutputBatch(Vec<(String, Vec<u8>)>);

impl TerminalOutputBatch {
    fn push(&mut self, tab_id: String, bytes: Vec<u8>) {
        if let Some((_, pending_bytes)) = self
            .0
            .iter_mut()
            .find(|(pending_tab_id, _)| *pending_tab_id == tab_id)
        {
            pending_bytes.extend_from_slice(&bytes);
        } else {
            self.0.push((tab_id, bytes));
        }
    }

    fn take(&mut self) -> Vec<(String, Vec<u8>)> {
        std::mem::take(&mut self.0)
    }
}

fn parse_rayon_threads_input(value: &str) -> Option<usize> {
    value
        .trim()
        .parse::<usize>()
        .ok()
        .filter(|value| *value > 0)
}

impl AxShell {
    pub(crate) fn start_event_pump(&self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            let mut idle_ticks = 0u32;
            let mut last_blink_time = std::time::Instant::now();
            loop {
                let sleep_for =
                    match this.read_with(cx, |this, _| this.event_pump_interval(idle_ticks)) {
                        Ok(interval) => interval,
                        Err(_) => break,
                    };
                cx.background_executor().timer(sleep_for).await;
                if this
                    .update(cx, |this, cx| {
                        let now = std::time::Instant::now();
                        let system_resumed = this.detect_system_resume(now, SystemTime::now(), cx);
                        let lifecycle_changed = this.advance_window_lifecycle(now);
                        let drain = this.drain_backend_events(cx);
                        if drain.ui_changed {
                            this.schedule_ui_refresh();
                        }
                        let terminal_due = this.should_flush_terminal_refresh();
                        let highlight_due = this.terminal_highlight_refresh_due(now);
                        let ui_due = this.should_flush_ui_refresh();
                        let resume_health_check_started = this.lifecycle.is_foreground()
                            && this.request_active_ssh_resume_health_check();
                        let system_sampled = this.lifecycle.is_foreground()
                            && !system_resumed
                            && !resume_health_check_started
                            && this.sample_system_if_due();
                        let sftp_closed = this.sweep_idle_sftp_connections_if_due(now);
                        let runtime_released = this.runtime_state.release_runtime_if_idle();
                        if this.lifecycle.is_foreground() {
                            this.sync_theme_if_due(cx);
                        }
                        let selecting = this.active_terminal_has_selection();
                        let is_blinking = matches!(
                            this.appearance.cursor_style,
                            crate::config::CursorStyle::Blink
                                | crate::config::CursorStyle::BeamBlink
                        );
                        let blink_due = this.lifecycle.is_foreground()
                            && !selecting
                            && is_blinking
                            && now.duration_since(last_blink_time) >= CURSOR_BLINK_INTERVAL;
                        if ui_due
                            || terminal_due
                            || highlight_due
                            || system_sampled
                            || blink_due
                            || sftp_closed
                            || runtime_released
                            || resume_health_check_started
                            || system_resumed
                            || lifecycle_changed
                        {
                            cx.notify();
                            idle_ticks = 0;
                            if blink_due {
                                last_blink_time = now;
                            }
                            if terminal_due {
                                this.mark_terminal_refresh_flushed(now);
                            }
                            if ui_due {
                                this.mark_ui_refresh_flushed(now);
                            }
                        } else {
                            idle_ticks = idle_ticks.saturating_add(1);
                            if this.lifecycle.is_foreground()
                                && idle_ticks >= IDLE_NOTIFY_INTERVAL_TICKS
                                && !selecting
                                && !this.runtime_state.pending_terminal_refresh
                                && !this.runtime_state.pending_ui_refresh
                            {
                                cx.notify();
                                idle_ticks = 1;
                            }
                        }
                        if drain.terminal_changed
                            || ui_due
                            || system_sampled
                            || blink_due
                            || sftp_closed
                            || runtime_released
                            || resume_health_check_started
                            || system_resumed
                            || lifecycle_changed
                        {
                            idle_ticks = 0;
                        }
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
        .detach();
    }

    pub(crate) fn on_window_activation_changed(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let now = std::time::Instant::now();
        if !self
            .lifecycle
            .set_window_active(window.is_window_active(), now)
        {
            return;
        }

        if self.lifecycle.is_foreground() {
            self.monitoring.last_sample = now - crate::monitoring::SystemSampler::interval();
            self.appearance.last_theme_sync = now - Duration::from_secs(1);
            self.sync_theme_if_due(cx);
            self.schedule_terminal_refresh();
            self.schedule_ui_refresh();
            let system_resumed = self.detect_system_resume(now, SystemTime::now(), cx);
            if !system_resumed && !self.request_active_ssh_resume_health_check() {
                self.sample_system_if_due();
            }
        } else {
            self.runtime_state.reset_frame_cadence();
        }
        cx.notify();
    }

    pub(crate) fn on_window_bounds_changed(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // GPUI updates the current display before invoking bounds observers.
        self.runtime_state.reset_frame_cadence();
    }

    fn detect_system_resume(
        &mut self,
        now: std::time::Instant,
        wall_clock: SystemTime,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.lifecycle.observe_event_pump_tick(now, wall_clock) {
            return false;
        }

        self.handle_system_resume(now, cx);
        true
    }

    fn handle_system_resume(&mut self, now: std::time::Instant, cx: &mut Context<Self>) {
        tracing::info!(
            component = "lifecycle",
            operation = "resume",
            "Detected a long event-pump gap; checking the active context"
        );
        self.monitoring.invalidate_remote_samples();
        if let Some(sampler) = self.monitoring.sampler.as_mut() {
            sampler.reset_after_resume();
        }
        self.runtime_state.reset_frame_cadence();
        for tab in &mut self.tabs {
            if tab.kind == crate::terminal::TabKind::Ssh && tab.connected {
                tab.connection_may_be_stale = true;
                tab.status = rust_i18n::t!("connection_may_need_check").to_string();
            }
        }
        let sftp_marked_stale = self.mark_idle_sftp_connections_stale();

        self.monitoring.last_sample = now - crate::monitoring::SystemSampler::interval();
        self.appearance.last_theme_sync = now - Duration::from_secs(1);
        if self.lifecycle.is_foreground() {
            self.sync_theme_if_due(cx);
            self.request_active_ssh_resume_health_check();
        }
        self.schedule_terminal_refresh();
        self.schedule_ui_refresh();
        if sftp_marked_stale {
            self.status = rust_i18n::t!("sftp_resume_reconnect_active").into();
        }
    }

    pub(crate) fn on_input_event(
        &mut self,
        input: &gpui::Entity<InputState>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if input == &self.sftp_path_input {
            if let InputEvent::PressEnter { .. } = event {
                let path = self
                    .sftp_path_input
                    .read(cx)
                    .text()
                    .to_string()
                    .trim()
                    .to_string();
                self.navigate_sftp(path, cx);
                window.prevent_default();
                cx.stop_propagation();
            }
        } else if input == &self.local_sftp_path_input {
            if let InputEvent::PressEnter { .. } = event {
                let path = self
                    .local_sftp_path_input
                    .read(cx)
                    .text()
                    .to_string()
                    .trim()
                    .to_string();
                self.navigate_local_file_browser(path, cx);
                window.prevent_default();
                cx.stop_propagation();
            }
        } else if input == &self.sftp_new_folder_input {
            match event {
                InputEvent::PressEnter { .. } => {
                    let name = self.sftp_new_folder_input.read(cx).text().to_string();
                    if !name.is_empty() {
                        let base_path = self.sftp_path_input.read(cx).text().to_string();
                        let path = crate::sftp::join_remote(&base_path, &name);
                        if let Some(handle) = self.ensure_active_sftp_handle() {
                            let _ = handle.create_dir(path);
                        }
                    }
                    self.sftp_creating_folder = false;
                    window.prevent_default();
                    cx.stop_propagation();
                }
                InputEvent::Blur => {
                    self.sftp_creating_folder = false;
                }
                _ => {}
            }
        } else if input == &self.search.input {
            if let InputEvent::PressEnter { .. } = event {
                if self.search.query.is_empty()
                    || self.search.input.read(cx).text().to_string() != self.search.query
                {
                    self.perform_search(window, cx);
                } else {
                    self.search_goto_next(cx);
                }
                window.prevent_default();
                cx.stop_propagation();
            }
        } else if input == &self.saved_group_name_input {
            match event {
                InputEvent::PressEnter { .. } => {
                    self.commit_saved_group_rename(cx);
                    window.prevent_default();
                    cx.stop_propagation();
                }
                InputEvent::Blur => {
                    self.commit_saved_group_rename(cx);
                }
                _ => {}
            }
        } else if input == &self.rayon_threads_input {
            match event {
                InputEvent::PressEnter { .. } => {
                    self.commit_rayon_threads_input(window, cx);
                    window.prevent_default();
                    cx.stop_propagation();
                }
                InputEvent::Blur => self.commit_rayon_threads_input(window, cx),
                _ => {}
            }
        } else if self
            .custom_theme_inputs
            .values()
            .any(|custom_input| input == custom_input)
        {
            match event {
                InputEvent::Change => {
                    self.preview_custom_theme_input(input, window, cx);
                }
                InputEvent::PressEnter { .. } => {
                    self.save_custom_appearance(window, cx);
                    window.prevent_default();
                    cx.stop_propagation();
                }
                _ => {}
            }
        } else if input == &self.custom_theme_save_path_input
            && matches!(event, InputEvent::PressEnter { .. })
        {
            self.save_custom_appearance(window, cx);
            window.prevent_default();
            cx.stop_propagation();
        }
        cx.notify();
    }

    fn commit_rayon_threads_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let input_value = self.rayon_threads_input.read(cx).text().to_string();
        let requested = parse_rayon_threads_input(&input_value);
        if let Some(requested) = requested {
            self.config.set_rayon_threads(requested);
            self.config.save_logged("set_rayon_threads");
        }
        Self::set_input_value(
            &self.rayon_threads_input,
            self.config.rayon_threads().to_string(),
            window,
            cx,
        );
    }

    fn drain_backend_events(&mut self, cx: &mut Context<Self>) -> DrainResult {
        let mut result = DrainResult::default();
        let mut transfers_changed = false;
        let mut terminal_output = TerminalOutputBatch::default();
        // Leave time for rendering and input even while a producer keeps the
        // bounded queue full.
        for _ in 0..BACKEND_EVENT_QUEUE_CAPACITY {
            let Ok(event) = self.runtime_state.events_rx.try_recv() else {
                break;
            };
            if let Some(resource_id) = event.resource_id()
                && !self.owns_event_resource(resource_id)
                && !self
                    .runtime_state
                    .events_tx
                    .is_routed_to_current_window(&event)
            {
                // The event was queued immediately before this resource moved
                // to another window. Re-route it using the current owner.
                let _ = self.runtime_state.events_tx.try_send(event);
                continue;
            }
            match event {
                BackendEvent::Output { tab_id, bytes } => {
                    terminal_output.push(tab_id, bytes);
                }
                event => {
                    self.flush_terminal_output(&mut terminal_output, &mut result);
                    match event {
                        BackendEvent::Status { tab_id, text } => {
                            result.ui_changed = true;
                            if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                                tab.backend_initialized = true;
                                tab.status = text.clone();
                            }
                            if let Some(progress) = self.connection_progress.as_mut()
                                && progress.tab_id == tab_id
                            {
                                progress.lines.push(text.clone().into());
                                self.connection_scroll_handle
                                    .set_offset(gpui::point(px(0.), px(-99999.0)));
                            }
                            self.status = text.into();
                        }
                        BackendEvent::Connected { tab_id } => {
                            result.ui_changed = true;
                            if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                                tab.backend_initialized = true;
                                tab.connected = true;
                                tab.connection_may_be_stale = false;
                                tab.disconnected_reason = None;
                            }
                            if self
                                .terminal_password_prompt
                                .as_ref()
                                .is_some_and(|prompt| prompt.tab_id == tab_id)
                            {
                                self.terminal_password_prompt = None;
                            }
                            self.terminal_password_retry_tabs.remove(&tab_id);
                            self.sync_system_tab_to_active_group();
                            self.request_active_system_snapshot();
                            if self.connection_progress.as_ref().is_some_and(|progress| {
                                progress.tab_id == tab_id && !progress.failed
                            }) {
                                self.connection_progress = None;
                            }
                        }
                        BackendEvent::SftpEntries {
                            tab_id,
                            path,
                            entries,
                            append,
                            has_more,
                            reached_limit,
                        } => {
                            result.ui_changed = true;
                            self.mark_sftp_activity_for_group(&tab_id);
                            if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == tab_id)
                                && let Some(sftp) = group.sftp.as_mut()
                            {
                                sftp.current_path = path;
                                sftp.has_opened_directory = true;
                                if append {
                                    sftp.entries.extend(entries);
                                } else {
                                    sftp.entries = entries;
                                    sftp.selected_path = None;
                                    sftp.selected_entries.clear();
                                }
                                sftp.has_more_entries = has_more;
                                sftp.loading_more_entries = false;
                                sftp.reached_entries_limit = reached_limit;
                                if let Some(target_path) = self.pending_sftp_selection_path.clone()
                                {
                                    let matched = sftp
                                        .entries
                                        .iter()
                                        .find(|entry| entry.full_path == target_path);
                                    if let Some(entry) = matched {
                                        sftp.selected_path = Some(entry.full_path.clone());
                                        sftp.selected_entries.clear();
                                        sftp.selected_entries.insert(entry.full_path.clone());
                                    } else if sftp.current_path == target_path {
                                        sftp.selected_path = None;
                                        sftp.selected_entries.clear();
                                    }
                                    if sftp.current_path == target_path
                                        || sftp
                                            .selected_path
                                            .as_deref()
                                            .is_some_and(|selected| selected == target_path)
                                    {
                                        self.pending_sftp_selection_path = None;
                                    }
                                }
                                self.pending_sftp_path_sync = Some(sftp.current_path.clone());
                            }
                        }
                        BackendEvent::SftpPreview { tab_id, preview } => {
                            result.ui_changed = true;
                            self.mark_sftp_activity_for_group(&tab_id);
                            if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == tab_id)
                                && let Some(sftp) = group.sftp.as_mut()
                            {
                                sftp.selected_path = Some(preview.path.clone());
                                sftp.preview = Some(preview);
                            }
                        }
                        BackendEvent::SftpStatus { tab_id, text } => {
                            result.ui_changed = true;
                            self.mark_sftp_activity_for_group(&tab_id);
                            if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == tab_id)
                                && let Some(sftp) = group.sftp.as_mut()
                            {
                                sftp.status = text.clone();
                            }
                            if self.active_group.as_ref() == Some(&tab_id) {
                                self.status = text.into();
                            }
                        }
                        BackendEvent::SftpEditOpened {
                            tab_id,
                            remote_path,
                            local_path,
                        } => {
                            result.ui_changed = true;
                            self.mark_sftp_activity_for_group(&tab_id);
                            if let Some(sftp) = self
                                .tab_groups
                                .iter_mut()
                                .find(|group| group.id == tab_id)
                                .and_then(|group| group.sftp.as_mut())
                            {
                                sftp.opening_edit_paths.remove(&remote_path);
                                if !sftp
                                    .edit_sessions
                                    .iter()
                                    .any(|session| session.local_path == local_path)
                                {
                                    sftp.edit_sessions.push(crate::app::SftpEditSession {
                                        remote_path,
                                        local_path,
                                        dirty: false,
                                        uploading: false,
                                    });
                                }
                            }
                        }
                        BackendEvent::SftpEditOpenFailed {
                            tab_id,
                            remote_path,
                            reason,
                        } => {
                            result.ui_changed = true;
                            if let Some(sftp) = self
                                .tab_groups
                                .iter_mut()
                                .find(|group| group.id == tab_id)
                                .and_then(|group| group.sftp.as_mut())
                            {
                                sftp.opening_edit_paths.remove(&remote_path);
                                sftp.edit_sessions
                                    .retain(|session| session.remote_path != remote_path);
                                sftp.status = format!("{}: {reason}", t!("sftp_edit_open_failed"));
                            }
                            if self.active_group.as_deref() == Some(tab_id.as_str()) {
                                self.status =
                                    format!("{}: {reason}", t!("sftp_edit_open_failed")).into();
                            }
                        }
                        BackendEvent::SftpEditChanged {
                            tab_id,
                            remote_path,
                            local_path,
                            dirty,
                        } => {
                            result.ui_changed = true;
                            if let Some(session) = self
                                .tab_groups
                                .iter_mut()
                                .find(|group| group.id == tab_id)
                                .and_then(|group| group.sftp.as_mut())
                                .and_then(|sftp| {
                                    sftp.edit_sessions.iter_mut().find(|session| {
                                        session.remote_path == remote_path
                                            && session.local_path == local_path
                                    })
                                })
                            {
                                session.dirty = dirty;
                            }
                        }
                        BackendEvent::SftpEditUploadFinished {
                            tab_id,
                            remote_path,
                            local_path,
                            result: upload_result,
                        } => {
                            result.ui_changed = true;
                            let mut upload_succeeded = false;
                            if let Some(sftp) = self
                                .tab_groups
                                .iter_mut()
                                .find(|group| group.id == tab_id)
                                .and_then(|group| group.sftp.as_mut())
                            {
                                if let Some(session) =
                                    sftp.edit_sessions.iter_mut().find(|session| {
                                        session.remote_path == remote_path
                                            && session.local_path == local_path
                                    })
                                {
                                    session.uploading = false;
                                    upload_succeeded = upload_result.is_ok();
                                }
                            }
                            if !upload_succeeded {
                                if let Err(error) = upload_result {
                                    self.status =
                                        format!("{}: {error}", t!("sftp_edit_upload_failed"))
                                            .into();
                                }
                                self.sftp_edit_close_group_id = None;
                            } else {
                                self.discard_sftp_edit_session(&tab_id, &local_path);
                            }
                        }
                        BackendEvent::HostKeyVerification { request } => {
                            result.ui_changed = true;
                            self.host_key_verification_requests.push_back(request);
                        }
                        BackendEvent::SftpOverwriteConflict { request } => {
                            result.ui_changed = true;
                            self.mark_sftp_activity_for_group(&request.tab_id);
                            tracing::debug!(
                                component = "sftp",
                                transfer_id = %request.transfer_id,
                                "Waiting for local overwrite decision"
                            );
                            if self.sftp_replace_all_for_run {
                                let _ = request
                                    .response
                                    .send(crate::sftp::SftpOverwriteDecision::Replace);
                            } else {
                                self.sftp_overwrite_requests.push_back(request);
                            }
                        }
                        BackendEvent::RemoteSystem {
                            tab_id,
                            generation,
                            snapshot,
                        } => {
                            result.ui_changed = true;
                            if self.monitoring.finish_remote_sample(generation) == Some(false)
                                && self.monitoring.system_tab_id.as_deref() == Some(tab_id.as_str())
                            {
                                self.monitoring.status = None;
                                self.monitoring.system = snapshot.clone();
                                self.monitoring.cpu_history.push(snapshot.cpu_percent);
                                if self.monitoring.cpu_history.len() > 20 {
                                    self.monitoring.cpu_history.remove(0);
                                }
                                self.monitoring
                                    .net_rx_history
                                    .push(snapshot.net_rx_rate as f32);
                                if self.monitoring.net_rx_history.len() > 20 {
                                    self.monitoring.net_rx_history.remove(0);
                                }
                                self.monitoring
                                    .net_tx_history
                                    .push(snapshot.net_tx_rate as f32);
                                if self.monitoring.net_tx_history.len() > 20 {
                                    self.monitoring.net_tx_history.remove(0);
                                }
                            }
                        }
                        BackendEvent::RemoteSystemUnavailable {
                            tab_id,
                            generation,
                            reason,
                        } => {
                            tracing::warn!(
                                component = "monitoring",
                                operation = "remote_sample",
                                tab_id = %tab_id,
                                error = %crate::diagnostics::sanitize_error(&reason),
                                "Remote system monitoring is unavailable"
                            );
                            result.ui_changed = true;
                            if self.monitoring.finish_remote_sample(generation) == Some(false)
                                && self.monitoring.system_tab_id.as_deref() == Some(tab_id.as_str())
                            {
                                self.monitoring.status = Some(reason.clone().into());
                                self.status = reason.into();
                            }
                        }
                        BackendEvent::ConnectionHealthy {
                            tab_id,
                            generation,
                            backend_generation,
                        } => {
                            result.ui_changed = true;
                            let is_current_backend = self
                                .tabs
                                .iter()
                                .find(|tab| tab.id == tab_id)
                                .is_some_and(|tab| tab.backend_generation == backend_generation);
                            if !is_current_backend {
                                let _ = self.monitoring.finish_remote_sample(generation);
                                continue;
                            }
                            if self.active_tab.as_deref() != Some(tab_id.as_str()) {
                                self.monitoring.invalidate_remote_samples();
                            } else if self.monitoring.finish_remote_sample(generation) == Some(true)
                            {
                                if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id)
                                {
                                    tab.connection_may_be_stale = false;
                                    tab.status = rust_i18n::t!("connection_healthy").to_string();
                                }
                                if self.active_tab.as_deref() == Some(tab_id.as_str()) {
                                    if self.monitoring.system_tab_id.as_deref()
                                        != Some(tab_id.as_str())
                                    {
                                        self.monitoring.system_tab_id = Some(tab_id);
                                        self.monitoring.cpu_history.clear();
                                        self.monitoring.net_rx_history.clear();
                                        self.monitoring.net_tx_history.clear();
                                    }
                                    self.monitoring.status = None;
                                }
                            }
                        }
                        BackendEvent::ConnectionUnhealthy {
                            tab_id,
                            generation,
                            backend_generation,
                            reason,
                        } => {
                            result.ui_changed = true;
                            let is_current_backend = self
                                .tabs
                                .iter()
                                .find(|tab| tab.id == tab_id)
                                .is_some_and(|tab| tab.backend_generation == backend_generation);
                            if !is_current_backend {
                                let _ = self.monitoring.finish_remote_sample(generation);
                                continue;
                            }
                            if self.active_tab.as_deref() != Some(tab_id.as_str()) {
                                self.monitoring.invalidate_remote_samples();
                            } else if self.monitoring.finish_remote_sample(generation) == Some(true)
                            {
                                if let Some(tab) = self.tabs.iter().find(|tab| tab.id == tab_id) {
                                    tab.shutdown_backend();
                                }
                                if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id)
                                {
                                    tab.connected = false;
                                    tab.connection_may_be_stale = false;
                                    tab.status = reason.clone();
                                    tab.disconnected_reason = Some(
                                        rust_i18n::t!(
                                            "connection_resume_check_failed",
                                            "reason" = reason
                                        )
                                        .to_string(),
                                    );
                                }
                                if self.monitoring.system_tab_id.as_deref() == Some(tab_id.as_str())
                                {
                                    self.monitoring.status = Some(reason.clone().into());
                                    self.status = reason.into();
                                }
                            }
                        }
                        BackendEvent::Closed { tab_id, reason } => {
                            result.ui_changed = true;
                            self.monitoring.invalidate_remote_samples();
                            let is_stale =
                                self.tabs
                                    .iter()
                                    .find(|t| t.id == tab_id)
                                    .is_some_and(|tab| {
                                        tab.backend_generation > 0 && !tab.backend_initialized
                                    });
                            if is_stale {
                                continue;
                            }
                            if let Some(tab) = self.tabs.iter().find(|tab| tab.id == tab_id) {
                                tab.shutdown_backend();
                            }
                            let is_graceful_exit =
                                reason == "local shell closed" || reason == "ssh session closed";
                            if is_graceful_exit {
                                self.handle_tab_close(tab_id.clone(), cx);
                                self.status = reason.into();
                                continue;
                            }
                            if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                                tab.connected = false;
                                tab.status = reason.clone();
                                tab.disconnected_reason = Some(reason.clone());
                            }
                            if self.monitoring.system_tab_id.as_deref() == Some(tab_id.as_str()) {
                                self.monitoring.status = Some(reason.clone().into());
                            }
                            if self.should_begin_terminal_password_prompt(&tab_id, &reason) {
                                result.terminal_changed |=
                                    self.begin_terminal_password_prompt(&tab_id, &reason, cx);
                                continue;
                            }
                            if let Some(progress) = self.connection_progress.as_mut()
                                && progress.tab_id == tab_id
                            {
                                progress.lines.push(reason.clone().into());
                                self.connection_scroll_handle
                                    .set_offset(gpui::point(px(0.), px(-99999.0)));
                                progress.title = rust_i18n::t!("connection_failed").into();
                                progress.failed = true;
                            }
                            self.status = reason.into();
                        }
                        BackendEvent::TransferProgress {
                            tab_id,
                            id,
                            transferred,
                            total,
                            state,
                        } => {
                            result.ui_changed = true;
                            if let Some(t) = self
                                .transfers
                                .iter_mut()
                                .find(|t| t.tab_id == tab_id && t.info.id == id)
                            {
                                t.transferred = transferred;
                                if let Some(total) = total {
                                    t.total = Some(total);
                                }
                                if state.is_terminal() && t.finished_at.is_none() {
                                    t.finished_at = Some(crate::sftp::unix_timestamp_secs());
                                }
                                match &state {
                                    crate::sftp::TransferState::Paused => {
                                        if let Some(file) = t.files.iter_mut().rev().find(|file| {
                                            matches!(
                                                file.state,
                                                crate::sftp::TransferFileState::Running
                                            )
                                        }) {
                                            file.state = crate::sftp::TransferFileState::Paused;
                                        }
                                    }
                                    crate::sftp::TransferState::Running => {
                                        if let Some(file) = t.files.iter_mut().rev().find(|file| {
                                            matches!(
                                                file.state,
                                                crate::sftp::TransferFileState::Paused
                                            )
                                        }) {
                                            file.state = crate::sftp::TransferFileState::Running;
                                        }
                                    }
                                    crate::sftp::TransferState::Interrupted(reason)
                                    | crate::sftp::TransferState::Zombie(reason) => {
                                        for file in &mut t.files {
                                            if !file.state.is_terminal() {
                                                file.state =
                                                    crate::sftp::TransferFileState::Interrupted(
                                                        reason.clone(),
                                                    );
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                t.state = state;
                                transfers_changed = true;
                            }
                        }
                        BackendEvent::TransferStarted { tab_id, info } => {
                            result.ui_changed = true;
                            self.mark_sftp_activity_for_group(&tab_id);
                            let tab_title = self.transfer_source_title(&tab_id);
                            self.transfers.insert(
                                0,
                                crate::sftp::Transfer {
                                    tab_id,
                                    tab_title,
                                    info,
                                    transferred: 0,
                                    total: None,
                                    state: crate::sftp::TransferState::Running,
                                    started_at: crate::sftp::unix_timestamp_secs(),
                                    finished_at: None,
                                    files: Vec::new(),
                                },
                            );
                            if self.transfers.len() > 100 {
                                self.transfers.truncate(100);
                            }
                            transfers_changed = true;
                        }
                        BackendEvent::TransferFileStarted {
                            tab_id,
                            transfer_id,
                            file,
                        } => {
                            result.ui_changed = true;
                            if let Some(transfer) = self.transfers.iter_mut().find(|transfer| {
                                transfer.tab_id == tab_id && transfer.info.id == transfer_id
                            }) {
                                if let Some(existing) = transfer
                                    .files
                                    .iter_mut()
                                    .find(|existing| existing.id == file.id)
                                {
                                    *existing = file;
                                } else {
                                    transfer.files.push(file);
                                }
                                transfers_changed = true;
                            }
                        }
                        BackendEvent::TransferFileFinished {
                            tab_id,
                            transfer_id,
                            file_id,
                            state,
                        } => {
                            result.ui_changed = true;
                            if let Some(file) = self
                                .transfers
                                .iter_mut()
                                .find(|transfer| {
                                    transfer.tab_id == tab_id && transfer.info.id == transfer_id
                                })
                                .and_then(|transfer| {
                                    transfer.files.iter_mut().find(|file| file.id == file_id)
                                })
                            {
                                file.state = state;
                                transfers_changed = true;
                            }
                        }
                        BackendEvent::SftpHome { tab_id, home } => {
                            result.ui_changed = true;
                            self.mark_sftp_activity_for_group(&tab_id);
                            if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == tab_id)
                                && let Some(sftp) = group.sftp.as_mut()
                            {
                                sftp.home_dir = home;
                            }
                        }
                        BackendEvent::TerminalTitleChanged { tab_id, title } => {
                            result.ui_changed = true;
                            if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                                tab.title = title.clone();
                            }
                        }
                        BackendEvent::WorkingDirectoryChanged { tab_id, path }
                        | BackendEvent::WorkingDirectoryResolved { tab_id, path } => {
                            result.ui_changed = true;
                            if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                                tab.shell_working_dir = Some(path.clone());
                            }
                            self.open_pending_sftp_terminal_working_dir(&tab_id, path, cx);
                        }
                        BackendEvent::SyncFinished(sync_result) => {
                            result.ui_changed = true;
                            self.sync_in_progress = false;
                            match sync_result {
                                crate::sync::SyncResult::Uploaded { etag } => {
                                    if etag.is_some() {
                                        self.config.set_sync_etag(etag);
                                    }
                                    self.sync_status = rust_i18n::t!("sync_upload_complete").into();
                                    if let Err(err) = self.config.save() {
                                        tracing::error!(
                                            component = "sync",
                                            operation = "save_upload_state",
                                            error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                                            "Failed to save sync upload state"
                                        );
                                        self.sync_status =
                                            format!("{}: {err:#}", rust_i18n::t!("sync_failed"))
                                                .into();
                                    }
                                }
                                crate::sync::SyncResult::Downloaded { payload, etag } => {
                                    self.config.replace_sessions(payload.sessions);
                                    self.config.set_sync_etag(etag);
                                    match self.config.save() {
                                        Ok(()) => {
                                            self.sync_status =
                                                rust_i18n::t!("sync_download_complete").into()
                                        }
                                        Err(err) => {
                                            tracing::error!(
                                                component = "sync",
                                                operation = "save_download",
                                                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                                                "Failed to save downloaded configuration"
                                            );
                                            self.sync_status =
                                                format!("{}: {err:#}", rust_i18n::t!("sync_failed"))
                                                    .into()
                                        }
                                    }
                                }
                                crate::sync::SyncResult::Failed(error) => {
                                    self.sync_status =
                                        format!("{}: {error}", rust_i18n::t!("sync_failed")).into();
                                }
                            }
                        }
                        BackendEvent::Output { .. } => {
                            unreachable!("output events are batched above")
                        }
                    }
                }
            }
        }
        self.flush_terminal_output(&mut terminal_output, &mut result);
        if result.terminal_changed {
            self.schedule_terminal_refresh();
        }
        if transfers_changed {
            self.persist_transfers();
        }
        result
    }

    fn owns_event_resource(&self, resource_id: &str) -> bool {
        self.tabs.iter().any(|tab| tab.id == resource_id)
            || self.tab_groups.iter().any(|group| group.id == resource_id)
    }

    fn flush_terminal_output(
        &mut self,
        terminal_output: &mut TerminalOutputBatch,
        result: &mut DrainResult,
    ) {
        for (tab_id, bytes) in terminal_output.take() {
            if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
                tab.backend_initialized = true;
                result.terminal_changed |= tab.feed(&bytes);
            }
        }
    }

    fn active_terminal_has_selection(&self) -> bool {
        let Some(active_id) = self.active_tab.as_ref() else {
            return self.terminal_selecting;
        };

        self.terminal_selecting
            || self
                .terminal_frozen_selection
                .as_ref()
                .is_some_and(|frozen| frozen.tab_id == *active_id)
            || self
                .tabs
                .iter()
                .find(|tab| &tab.id == active_id)
                .is_some_and(crate::terminal::TerminalTab::selection_active)
    }

    fn schedule_terminal_refresh(&mut self) {
        self.runtime_state.pending_terminal_refresh = true;
        self.note_foreground_refresh_activity();
    }

    fn schedule_ui_refresh(&mut self) {
        self.runtime_state.pending_ui_refresh = true;
        self.note_foreground_refresh_activity();
    }

    fn should_flush_terminal_refresh(&self) -> bool {
        if !self.runtime_state.pending_terminal_refresh {
            return false;
        }

        self.runtime_state.last_terminal_refresh.elapsed() >= self.refresh_interval()
    }

    fn should_flush_ui_refresh(&self) -> bool {
        self.runtime_state.pending_ui_refresh
            && self.runtime_state.last_ui_refresh.elapsed() >= self.refresh_interval()
    }

    fn terminal_highlight_refresh_due(&self, now: std::time::Instant) -> bool {
        if self.workspace_page != WorkspacePage::Terminal || !self.config.keyword_highlight() {
            return false;
        }

        self.pane_root.tab_ids().into_iter().any(|tab_id| {
            self.tabs
                .iter()
                .find(|tab| tab.id == tab_id)
                .is_some_and(|tab| tab.highlight_refresh_due(now))
        })
    }

    fn refresh_interval(&self) -> Duration {
        match self.lifecycle.state() {
            WindowLifecycleState::Foreground => self
                .runtime_state
                .foreground_refresh_interval(std::time::Instant::now()),
            WindowLifecycleState::Background => BACKGROUND_REFRESH_INTERVAL,
            WindowLifecycleState::DeepSleep => DEEP_SLEEP_REFRESH_INTERVAL,
        }
    }

    fn event_pump_interval(&self, idle_ticks: u32) -> Duration {
        match self.lifecycle.state() {
            WindowLifecycleState::Foreground if idle_ticks == 0 => self.refresh_interval(),
            WindowLifecycleState::Foreground => IDLE_PUMP_INTERVAL,
            WindowLifecycleState::Background => BACKGROUND_PUMP_INTERVAL,
            WindowLifecycleState::DeepSleep => DEEP_SLEEP_PUMP_INTERVAL,
        }
    }

    fn note_foreground_refresh_activity(&mut self) {
        if self.lifecycle.is_foreground() {
            self.runtime_state
                .note_foreground_refresh_activity(std::time::Instant::now());
        }
    }

    fn mark_terminal_refresh_flushed(&mut self, now: std::time::Instant) {
        self.runtime_state.pending_terminal_refresh = false;
        self.runtime_state.last_terminal_refresh = now;
    }

    fn mark_ui_refresh_flushed(&mut self, now: std::time::Instant) {
        self.runtime_state.pending_ui_refresh = false;
        self.runtime_state.last_ui_refresh = now;
    }

    fn advance_window_lifecycle(&mut self, now: std::time::Instant) -> bool {
        let changed = self
            .lifecycle
            .advance(now, self.config.deep_sleep_after_minutes());
        if changed {
            tracing::debug!(state = ?self.lifecycle.state(), "window lifecycle changed");
        }
        changed
    }

    fn sweep_idle_sftp_connections_if_due(&mut self, now: std::time::Instant) -> bool {
        let interval = if self.lifecycle.is_foreground() {
            FOREGROUND_SFTP_IDLE_SWEEP_INTERVAL
        } else {
            BACKGROUND_SFTP_IDLE_SWEEP_INTERVAL
        };
        if now.duration_since(self.runtime_state.last_sftp_idle_sweep) < interval {
            return false;
        }

        self.runtime_state.last_sftp_idle_sweep = now;
        self.sweep_idle_sftp_connections()
    }

    pub(crate) fn sample_system_if_due(&mut self) -> bool {
        if !self.lifecycle.is_foreground() || !self.is_monitoring_visible() {
            return false;
        }
        if self.monitoring.last_sample.elapsed() >= crate::monitoring::SystemSampler::interval() {
            self.monitoring.last_sample = std::time::Instant::now();
            if let Some(ref tab_id) = self.monitoring.system_tab_id.clone()
                && self.tabs.iter().any(|t| {
                    t.id == *tab_id
                        && t.kind == crate::terminal::TabKind::Ssh
                        && t.connected
                        && !t.connection_may_be_stale
                })
                && self.monitoring.status.is_none()
            {
                self.request_active_system_snapshot();
                return false;
            }
            let snapshot = self
                .monitoring
                .sampler
                .get_or_insert_with(crate::monitoring::SystemSampler::new)
                .sample();
            let cpu_usage = snapshot.cpu_percent;
            self.monitoring.cpu_history.push(cpu_usage);
            if self.monitoring.cpu_history.len() > 20 {
                self.monitoring.cpu_history.remove(0);
            }
            self.monitoring
                .net_rx_history
                .push(snapshot.net_rx_rate as f32);
            if self.monitoring.net_rx_history.len() > 20 {
                self.monitoring.net_rx_history.remove(0);
            }
            self.monitoring
                .net_tx_history
                .push(snapshot.net_tx_rate as f32);
            if self.monitoring.net_tx_history.len() > 20 {
                self.monitoring.net_tx_history.remove(0);
            }
            self.monitoring.system = snapshot;
            return true;
        }
        false
    }

    pub(crate) fn sync_theme_if_due(&mut self, cx: &mut Context<Self>) {
        if self.appearance.follow_system_theme
            && self.appearance.last_theme_sync.elapsed() >= Duration::from_secs(1)
        {
            self.appearance.last_theme_sync = std::time::Instant::now();
            self.apply_theme_preferences_for_system(cx);
            cx.refresh_windows();
        }
    }
}

#[cfg(test)]
mod terminal_output_batch_tests {
    use super::{TerminalOutputBatch, parse_rayon_threads_input};

    #[test]
    fn output_batch_merges_by_tab_and_preserves_first_seen_order() {
        let mut batch = TerminalOutputBatch::default();

        batch.push("first".to_string(), b"a".to_vec());
        batch.push("second".to_string(), b"b".to_vec());
        batch.push("first".to_string(), b"c".to_vec());

        assert_eq!(
            batch.take(),
            vec![
                ("first".to_string(), b"ac".to_vec()),
                ("second".to_string(), b"b".to_vec()),
            ]
        );
    }

    #[test]
    fn taking_output_batch_clears_pending_bytes() {
        let mut batch = TerminalOutputBatch::default();
        batch.push("tab".to_string(), b"output".to_vec());

        assert_eq!(batch.take(), vec![("tab".to_string(), b"output".to_vec())]);
        assert!(batch.take().is_empty());
    }

    #[test]
    fn rayon_thread_input_requires_a_positive_integer() {
        assert_eq!(parse_rayon_threads_input(" 17 "), Some(17));
        assert_eq!(parse_rayon_threads_input("1"), Some(1));
        assert_eq!(parse_rayon_threads_input("0"), None);
        assert_eq!(parse_rayon_threads_input(""), None);
        assert_eq!(parse_rayon_threads_input("four"), None);
    }
}
