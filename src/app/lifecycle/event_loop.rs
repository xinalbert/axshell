use std::time::Duration;

use gpui::{Context, Window, px};
use gpui_component::input::{InputEvent, InputState};

use crate::{
    AxShell,
    app::state::lifecycle::WindowLifecycleState,
    events::{BACKEND_EVENT_QUEUE_CAPACITY, BackendEvent},
};

const ACTIVE_PUMP_INTERVAL: Duration = Duration::from_millis(16);
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

impl AxShell {
    pub(crate) fn start_event_pump(&self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            let mut idle_ticks = 0u32;
            let mut last_blink_time = std::time::Instant::now();
            loop {
                let sleep_for = match this.read_with(cx, |this, _| this.lifecycle.state()) {
                    Ok(WindowLifecycleState::Foreground) if idle_ticks == 0 => ACTIVE_PUMP_INTERVAL,
                    Ok(WindowLifecycleState::Foreground) => IDLE_PUMP_INTERVAL,
                    Ok(WindowLifecycleState::Background) => BACKGROUND_PUMP_INTERVAL,
                    Ok(WindowLifecycleState::DeepSleep) => DEEP_SLEEP_PUMP_INTERVAL,
                    Err(_) => break,
                };
                cx.background_executor().timer(sleep_for).await;
                if this
                    .update(cx, |this, cx| {
                        let now = std::time::Instant::now();
                        let lifecycle_changed = this.advance_window_lifecycle(now);
                        let drain = this.drain_backend_events(cx);
                        if drain.ui_changed {
                            this.schedule_ui_refresh();
                        }
                        let terminal_due = this.should_flush_terminal_refresh();
                        let ui_due = this.should_flush_ui_refresh();
                        let system_sampled =
                            this.lifecycle.is_foreground() && this.sample_system_if_due();
                        let sftp_closed = this.sweep_idle_sftp_connections_if_due(now);
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
                            || system_sampled
                            || blink_due
                            || sftp_closed
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
            self.sample_system_if_due();
        }
        cx.notify();
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
        } else if self
            .custom_theme_inputs
            .values()
            .any(|custom_input| input == custom_input)
            && matches!(event, InputEvent::PressEnter { .. })
        {
            self.save_custom_appearance(window, cx);
            window.prevent_default();
            cx.stop_propagation();
        }
        cx.notify();
    }

    fn drain_backend_events(&mut self, cx: &mut Context<Self>) -> DrainResult {
        let mut result = DrainResult::default();
        let mut transfers_changed = false;
        // Leave time for rendering and input even while a producer keeps the
        // bounded queue full.
        for _ in 0..BACKEND_EVENT_QUEUE_CAPACITY {
            let Ok(event) = self.runtime_state.events_rx.try_recv() else {
                break;
            };
            match event {
                BackendEvent::Output { tab_id, bytes } => {
                    if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                        tab.backend_initialized = true;
                        result.terminal_changed |= tab.feed(&bytes);
                    }
                }
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
                        tab.disconnected_reason = None;
                    }
                    self.sync_system_tab_to_active_group();
                    self.request_active_system_snapshot();
                    if self
                        .connection_progress
                        .as_ref()
                        .is_some_and(|progress| progress.tab_id == tab_id && !progress.failed)
                    {
                        self.connection_progress = None;
                    }
                }
                BackendEvent::SshConnectionModeResolved {
                    tab_id,
                    session_id,
                    mode,
                } => {
                    result.ui_changed = true;
                    for tab in self.tabs.iter_mut() {
                        if tab.id == tab_id
                            || tab
                                .session
                                .as_ref()
                                .is_some_and(|session| session.id == session_id)
                        {
                            if let Some(session) = tab.session.as_mut() {
                                session.last_successful_ssh_mode = Some(mode);
                            }
                        }
                    }
                    if self
                        .config
                        .set_session_last_successful_ssh_mode(&session_id, mode)
                    {
                        let _ = self.config.save();
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
                        if let Some(target_path) = self.pending_sftp_selection_path.clone() {
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
                BackendEvent::RemoteSystem { tab_id, snapshot } => {
                    result.ui_changed = true;
                    self.monitoring.remote_sample_in_flight = false;
                    if self.monitoring.system_tab_id.as_deref() == Some(tab_id.as_str()) {
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
                BackendEvent::RemoteSystemUnavailable { tab_id, reason } => {
                    result.ui_changed = true;
                    self.monitoring.remote_sample_in_flight = false;
                    if self.monitoring.system_tab_id.as_deref() == Some(tab_id.as_str()) {
                        self.monitoring.status = Some(reason.clone().into());
                        self.status = reason.into();
                    }
                }
                BackendEvent::Closed { tab_id, reason } => {
                    result.ui_changed = true;
                    self.monitoring.remote_sample_in_flight = false;
                    let is_stale = self
                        .tabs
                        .iter()
                        .find(|t| t.id == tab_id)
                        .is_some_and(|tab| tab.backend_generation > 0 && !tab.backend_initialized);
                    if is_stale {
                        continue;
                    }
                    if let Some(tab) = self.tabs.iter().find(|tab| tab.id == tab_id) {
                        tab.shutdown_backend();
                    }
                    let is_graceful_exit =
                        reason == "local shell closed" || reason == "ssh session closed";
                    if is_graceful_exit {
                        self.handle_tab_close(tab_id.clone());
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
                    tab_id: _,
                    id,
                    transferred,
                    total,
                    state,
                } => {
                    result.ui_changed = true;
                    if let Some(t) = self.transfers.iter_mut().find(|t| t.info.id == id) {
                        t.transferred = transferred;
                        if let Some(total) = total {
                            t.total = Some(total);
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
                        },
                    );
                    if self.transfers.len() > 100 {
                        self.transfers.truncate(100);
                    }
                    transfers_changed = true;
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
                    self.sync_sftp_to_shell_working_dir_for_tab(&tab_id, &path, cx);
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
                            let _ = self.config.save();
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
                                    self.sync_status =
                                        format!("{}: {err:#}", rust_i18n::t!("sync_failed")).into()
                                }
                            }
                        }
                        crate::sync::SyncResult::Failed(error) => {
                            self.sync_status =
                                format!("{}: {error}", rust_i18n::t!("sync_failed")).into();
                        }
                    }
                }
            }
        }
        if result.terminal_changed {
            self.schedule_terminal_refresh();
        }
        if transfers_changed {
            self.config.set_transfers(self.transfers.clone());
        }
        result
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
    }

    fn schedule_ui_refresh(&mut self) {
        self.runtime_state.pending_ui_refresh = true;
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

    fn refresh_interval(&self) -> Duration {
        match self.lifecycle.state() {
            WindowLifecycleState::Foreground => ACTIVE_PUMP_INTERVAL,
            WindowLifecycleState::Background => BACKGROUND_REFRESH_INTERVAL,
            WindowLifecycleState::DeepSleep => DEEP_SLEEP_REFRESH_INTERVAL,
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
                    t.id == *tab_id && t.kind == crate::terminal::TabKind::Ssh && t.connected
                })
                && self.monitoring.status.is_none()
            {
                self.request_active_system_snapshot();
                return false;
            }
            let snapshot = self.monitoring.sampler.sample();
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
            gpui_component::Theme::sync_system_appearance(None, cx);
            cx.refresh_windows();
        }
    }
}
