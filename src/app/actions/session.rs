use gpui::{
    AppContext as _, Context, Entity, MouseButton, MouseDownEvent, SharedString, Window, px,
};
use gpui_component::{Theme, WindowExt as _, input::InputState};
use rust_i18n::t;
use std::{collections::HashSet, rc::Rc};
use uuid::Uuid;

use crate::{
    AxShell, PaneLayout, TabGroup,
    app::{
        WorkspacePage,
        constants::{DEFAULT_COLS, DEFAULT_ROWS},
        session_ui::should_prompt_for_terminal_password_before_connect,
        terminal_link_activation_modifier_pressed,
    },
    backend::{local, serial, ssh, telnet},
    config::LocalShellProfile,
    session::{AuthMethod, Session, SessionKind},
    terminal::{BackendCommand, BackendTx, RenderSnapshot, TabKind, TerminalTab},
};

pub(super) fn normalize_session_group_name(value: &str) -> String {
    value.trim().to_string()
}

fn default_port_for_session_kind(kind: SessionKind) -> u16 {
    match kind {
        SessionKind::Ssh => 22,
        SessionKind::Telnet => 23,
        SessionKind::Serial => 0,
    }
}

/// Route detached-window closure through AppKit so its CAMetalLayer drawables
/// are released before GPUI removes the window from its registry.
#[cfg(target_os = "macos")]
fn close_window_through_appkit(window: &Window) -> bool {
    use objc::{msg_send, runtime::Object, sel, sel_impl};
    use raw_window_handle::RawWindowHandle;

    let Ok(handle) = raw_window_handle::HasWindowHandle::window_handle(window) else {
        return false;
    };
    let RawWindowHandle::AppKit(handle) = handle.as_raw() else {
        return false;
    };

    unsafe {
        let ns_view = handle.ns_view.as_ptr().cast::<Object>();
        let ns_window: *mut Object = msg_send![ns_view, window];
        if ns_window.is_null() {
            return false;
        }
        let _: () = msg_send![ns_window, performClose: std::ptr::null_mut::<Object>()];
    }
    true
}

#[cfg(not(target_os = "macos"))]
fn close_window_through_appkit(_: &Window) -> bool {
    false
}

impl AxShell {
    pub(crate) fn return_workspace_to_main_window(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_detached_workspace {
            self.status = t!("workspace_return_unavailable").into();
            cx.notify();
            return;
        }
        let Some(main_workspace) = cx.try_global::<crate::app::MainWorkspace>() else {
            self.status = t!("workspace_return_main_closed").into();
            cx.notify();
            return;
        };
        let main_view = main_workspace.view.clone();
        let Some(group_id) = self.active_group.clone() else {
            self.status = t!("workspace_return_unavailable").into();
            cx.notify();
            return;
        };
        let Some(transfer) = self.take_workspace_transfer(&group_id) else {
            self.status = t!("workspace_return_unavailable").into();
            cx.notify();
            return;
        };

        let transfer = std::rc::Rc::new(std::cell::RefCell::new(Some(transfer)));
        let target_transfer = transfer.clone();
        let restored = cx
            .with_window(main_view.entity_id(), |main_window, cx| {
                let transfer = target_transfer
                    .borrow_mut()
                    .take()
                    .expect("workspace transfer remains available for the main window");
                main_view.update(cx, |main, cx| main.restore_workspace_transfer(transfer, cx));
                main_window.activate_window();
                let focus_handle = main_view.read(cx).focus_handle.clone();
                main_window.focus(&focus_handle, cx);
            })
            .is_some();

        if restored {
            // `performClose:` invokes the GPUI close callback, which updates
            // this window. Defer it until this transfer update has completed.
            window.defer(cx, |window, _| {
                if !close_window_through_appkit(window) {
                    window.remove_window();
                }
            });
            return;
        }

        let transfer = transfer
            .borrow_mut()
            .take()
            .expect("failed return keeps the workspace transfer");
        self.restore_workspace_transfer(transfer, cx);
        self.status = t!("workspace_return_main_closed").into();
        cx.notify();
    }

    pub(crate) fn move_workspace_group_to_new_window(
        &mut self,
        group_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_detached_workspace {
            self.status = t!("workspace_move_already_detached").into();
            cx.notify();
            return;
        }
        if self.group_has_active_sftp_transfer(&group_id) {
            self.status = t!("workspace_move_blocked_by_transfer").into();
            cx.notify();
            return;
        }

        let tab_ids = self
            .tab_groups
            .iter()
            .find(|group| group.id == group_id)
            .map(|group| {
                group
                    .pane_root
                    .tab_ids()
                    .into_iter()
                    .filter(|id| !id.is_empty())
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if tab_ids.is_empty()
            && !self
                .tab_groups
                .iter()
                .any(|group| group.id == group_id && group.sftp.is_some())
        {
            self.status = t!("workspace_move_unavailable").into();
            cx.notify();
            return;
        }

        let (events_tx, events_rx) =
            crate::events::backend_event_channel_with_router(self.runtime_state.events_tx.router());
        // Switch producers before removing UI ownership so queued source-window
        // events are forwarded to the target window instead of being consumed here.
        events_tx.register_route(group_id.clone());
        events_tx.register_routes(tab_ids.iter().map(String::as_str));
        let Some(transfer) = self.take_workspace_transfer(&group_id) else {
            self.status = t!("workspace_move_unavailable").into();
            cx.notify();
            return;
        };

        if let Err((err, transfer)) =
            crate::app::startup::open_workspace_window(transfer, events_tx, events_rx, cx)
        {
            self.restore_workspace_transfer(transfer, cx);
            tracing::error!(
                component = "workspace",
                operation = "move_to_new_window",
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to open detached workspace window"
            );
            self.status = format!("{}: {err:#}", t!("workspace_move_failed")).into();
            cx.notify();
            return;
        }

        if self.active_group.is_none() {
            self.activate_first_visible_group_or_home(window, cx);
        }
        self.sync_system_tab_to_active_group();
        cx.notify();
    }

    pub(crate) fn restore_workspace_transfer(
        &mut self,
        transfer: crate::app::WorkspaceTransfer,
        cx: &mut Context<Self>,
    ) {
        let crate::app::WorkspaceTransfer {
            group,
            tabs,
            sftp_handle,
            sftp_last_activity,
            connection_progress,
            terminal_password_prompt,
            terminal_password_retry_tabs,
            transfers,
            active_tab,
            focused_pane_path,
            workspace_page,
            runtime,
        } = transfer;
        let tab_ids = group
            .pane_root
            .tab_ids()
            .into_iter()
            .filter(|id| !id.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();
        self.runtime_state
            .events_tx
            .register_route(group.id.clone());
        self.runtime_state
            .events_tx
            .register_routes(tab_ids.iter().map(String::as_str));
        if let Some(runtime) = runtime {
            self.runtime_state.adopt_shared_runtime(runtime);
        }
        self.active_tab = active_tab.or_else(|| tab_ids.first().cloned());
        self.pane_root = group.pane_root.clone();
        self.active_group = Some(group.id.clone());
        self.workspace_page = workspace_page;
        self.focused_pane_path = focused_pane_path;
        self.tab_groups.push(group);
        self.tabs.extend(tabs);
        if let Some(handle) = sftp_handle {
            let group_id = self.active_group.clone().expect("restored group exists");
            self.sftp_handles.insert(group_id, handle);
        }
        if let Some(last_activity) = sftp_last_activity {
            let group_id = self.active_group.clone().expect("restored group exists");
            self.sftp_last_activity.insert(group_id, last_activity);
        }
        self.connection_progress = connection_progress;
        self.terminal_password_prompt = terminal_password_prompt;
        self.terminal_password_retry_tabs = terminal_password_retry_tabs;
        self.transfers.extend(transfers);
        self.sync_system_tab_to_active_group();
        cx.notify();
    }

    fn take_workspace_transfer(&mut self, group_id: &str) -> Option<crate::app::WorkspaceTransfer> {
        let workspace_page = if self.active_group.as_deref() == Some(group_id) {
            self.workspace_page
        } else {
            WorkspacePage::Terminal
        };
        let group_index = self
            .tab_groups
            .iter()
            .position(|group| group.id == group_id)?;
        let group = self.tab_groups.remove(group_index);
        let active_tab = self
            .active_group
            .as_deref()
            .filter(|active_group| *active_group == group_id)
            .and_then(|_| self.active_tab.clone());
        let focused_pane_path = self
            .active_group
            .as_deref()
            .filter(|active_group| *active_group == group_id)
            .map(|_| self.focused_pane_path.clone())
            .unwrap_or_default();
        let tab_ids = group
            .pane_root
            .tab_ids()
            .into_iter()
            .filter(|id| !id.is_empty())
            .map(str::to_string)
            .collect::<HashSet<_>>();
        let all_tabs = std::mem::take(&mut self.tabs);
        let (tabs, remaining_tabs): (Vec<_>, Vec<_>) = all_tabs
            .into_iter()
            .partition(|tab| tab_ids.contains(&tab.id));
        self.tabs = remaining_tabs;
        self.terminal_scrollbars
            .retain(|tab_id, _| !tab_ids.contains(tab_id));
        self.terminal_bounds
            .retain(|tab_id, _| !tab_ids.contains(tab_id));
        if self
            .hovered_url
            .as_ref()
            .is_some_and(|hovered| tab_ids.contains(&hovered.tab_id))
        {
            self.hovered_url = None;
        }
        if self
            .terminal_composition
            .as_ref()
            .is_some_and(|composition| tab_ids.contains(&composition.tab_id))
        {
            self.terminal_composition = None;
        }
        if self
            .terminal_frozen_selection
            .as_ref()
            .is_some_and(|selection| tab_ids.contains(&selection.tab_id))
        {
            self.terminal_frozen_selection = None;
        }
        if self
            .search
            .target_tab
            .as_ref()
            .is_some_and(|tab_id| tab_ids.contains(tab_id))
        {
            self.search.active = false;
            self.search.query.clear();
            self.search.matches.clear();
            self.search.current = 0;
            self.search.target_tab = None;
        }
        if self
            .active_tab
            .as_ref()
            .is_some_and(|tab_id| tab_ids.contains(tab_id))
        {
            self.active_tab = None;
            self.terminal_selecting = false;
        }
        if self.active_group.as_deref() == Some(group_id) {
            self.active_group = None;
            self.pane_root = PaneLayout::Single(String::new());
            self.focused_pane_path = Vec::new();
            self.workspace_page = WorkspacePage::Terminal;
        }

        let connection_progress = self
            .connection_progress
            .as_ref()
            .is_some_and(|progress| tab_ids.contains(&progress.tab_id))
            .then(|| self.connection_progress.take())
            .flatten();
        let terminal_password_prompt = self
            .terminal_password_prompt
            .as_ref()
            .is_some_and(|prompt| tab_ids.contains(&prompt.tab_id))
            .then(|| self.terminal_password_prompt.take())
            .flatten();
        let retry_tabs = std::mem::take(&mut self.terminal_password_retry_tabs);
        let (terminal_password_retry_tabs, remaining_retry_tabs): (HashSet<_>, HashSet<_>) =
            retry_tabs
                .into_iter()
                .partition(|tab_id| tab_ids.contains(tab_id));
        self.terminal_password_retry_tabs = remaining_retry_tabs;

        let sftp_handle = self.sftp_handles.remove(group_id);
        let sftp_last_activity = self.sftp_last_activity.remove(group_id);
        let all_transfers = std::mem::take(&mut self.transfers);
        let (transfers, remaining_transfers): (Vec<_>, Vec<_>) = all_transfers
            .into_iter()
            .partition(|transfer| transfer.tab_id == group_id);
        self.transfers = remaining_transfers;
        self.monitoring.invalidate_remote_samples();

        Some(crate::app::WorkspaceTransfer {
            group,
            tabs,
            sftp_handle,
            sftp_last_activity,
            connection_progress,
            terminal_password_prompt,
            terminal_password_retry_tabs,
            transfers,
            active_tab,
            focused_pane_path,
            workspace_page,
            runtime: self.runtime_state.shared_runtime(),
        })
    }

    pub(crate) fn sync_local_shell_profile_inputs(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let profile = self.config.default_local_shell_profile();
        Self::set_input_value(
            &self.local_shell_profile_name_input,
            profile.name,
            window,
            cx,
        );
        Self::set_input_value(
            &self.local_shell_profile_program_input,
            profile.program,
            window,
            cx,
        );
        Self::set_input_value(
            &self.local_shell_profile_args_input,
            profile.args.join("\n"),
            window,
            cx,
        );
    }

    pub(crate) fn select_default_local_shell_profile(
        &mut self,
        id: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.config.set_default_local_shell_profile(id) {
            self.config.save_logged("set_default_local_shell_profile");
            self.sync_local_shell_profile_inputs(window, cx);
        }
        cx.notify();
    }

    pub(crate) fn save_default_local_shell_profile(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let current = self.config.default_local_shell_profile();
        let name = self
            .local_shell_profile_name_input
            .read(cx)
            .text()
            .to_string();
        let program = self
            .local_shell_profile_program_input
            .read(cx)
            .text()
            .to_string();
        let args = self
            .local_shell_profile_args_input
            .read(cx)
            .text()
            .to_string();
        let profile = LocalShellProfile {
            id: current.id,
            name: name.trim().to_string(),
            program: program.trim().to_string(),
            args: args
                .lines()
                .filter(|line| !line.is_empty())
                .map(str::to_string)
                .collect(),
        };
        if self.config.update_local_shell_profile(profile) {
            self.config.save_logged("save_local_shell_profile");
            self.sync_local_shell_profile_inputs(window, cx);
            self.status = "local shell profile saved".into();
        } else {
            self.status = "local shell program is required".into();
        }
        cx.notify();
    }

    pub(crate) fn duplicate_local_shell_profile(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let source = self.config.default_local_shell_profile();
        let name = format!("{} Copy", source.name);
        if let Some(profile) =
            self.config
                .add_local_shell_profile(name, source.program, source.args)
        {
            self.config.set_default_local_shell_profile(&profile.id);
            self.config.save_logged("duplicate_local_shell_profile");
            self.sync_local_shell_profile_inputs(window, cx);
            self.status = "local shell profile added".into();
        }
        cx.notify();
    }

    pub(crate) fn remove_default_local_shell_profile(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let id = self.config.default_local_shell_profile().id;
        if self.config.remove_local_shell_profile(&id) {
            self.config.save_logged("remove_local_shell_profile");
            self.sync_local_shell_profile_inputs(window, cx);
            self.status = "local shell profile removed".into();
        }
        cx.notify();
    }

    /// Stop terminal and SFTP backends without blocking the GPUI event loop.
    pub(crate) fn shutdown_all_backends(&mut self) {
        for tab in &self.tabs {
            tab.shutdown_backend();
        }

        let group_ids = self.sftp_handles.keys().cloned().collect::<Vec<_>>();
        for group_id in group_ids {
            self.release_sftp_handle_for_group(&group_id, true);
        }
        self.runtime_state.release_runtime_if_idle();
    }

    pub(crate) fn open_local(&mut self, cx: &mut Context<Self>) {
        let id = Uuid::new_v4().to_string();
        let profile = self.config.default_local_shell_profile();
        match local::spawn_local_terminal(
            id.clone(),
            DEFAULT_COLS,
            DEFAULT_ROWS,
            &profile,
            self.runtime_state.events_tx.clone(),
        ) {
            Ok(backend) => {
                let mut tab = TerminalTab::new_local(
                    id.clone(),
                    profile.name.clone(),
                    profile,
                    backend,
                    self.runtime_state.events_tx.clone(),
                );
                tab.resize(DEFAULT_COLS, DEFAULT_ROWS);
                self.tabs.push(tab);
                self.active_tab = Some(id.clone());
                self.pane_root = PaneLayout::Single(id.clone());
                self.focused_pane_path = vec![];
                let group_id = Uuid::new_v4().to_string();
                let title = "Local".to_string();
                let instance_number = self.next_workspace_group_instance(&title);
                self.tab_groups.push(TabGroup {
                    id: group_id.clone(),
                    title,
                    instance_number,
                    pane_root: PaneLayout::Single(id),
                    sftp: None,
                    sftp_page_open: false,
                    sftp_session: None,
                });
                self.active_group = Some(group_id);
                self.status = "local terminal opened".into();
                self.set_workspace_page(WorkspacePage::Terminal, cx);
            }
            Err(err) => {
                tracing::error!(
                    component = "local_terminal",
                    operation = "open",
                    error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                    "Failed to open local terminal"
                );
                self.status = format!("failed to open local terminal: {err:#}").into();
            }
        }
        cx.notify();
    }

    pub(crate) fn open_local_and_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open_local(cx);
        self.focus_terminal_workspace(window, cx);
    }

    fn save_session_from_form(&mut self, cx: &mut Context<Self>) -> Option<Session> {
        let session_name = self.session_name_input.read(cx).value().trim().to_string();
        let group_name = normalize_session_group_name(&self.session_group_input.read(cx).value());
        let host = self.host_input.read(cx).value().trim().to_string();
        let parsed_port = self.port_input.read(cx).value().trim().parse::<u16>().ok();
        let user = self.user_input.read(cx).value().trim().to_string();
        let password = self.password_input.read(cx).value().to_string();
        let key_path = self.key_path_input.read(cx).value().trim().to_string();
        let key_inline = self.key_inline_input.read(cx).value().to_string();
        let passphrase = self.passphrase_input.read(cx).value().to_string();
        let sftp_path = self
            .session_sftp_path_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        let kind = self.session_kind;
        let port = parsed_port.unwrap_or_else(|| default_port_for_session_kind(kind));
        let serial_port = self.serial_port_input.read(cx).value().trim().to_string();
        let baud_rate = self
            .serial_baud_rate_input
            .read(cx)
            .value()
            .trim()
            .parse::<u32>()
            .unwrap_or(115_200);

        if kind == SessionKind::Ssh && (host.is_empty() || user.is_empty()) {
            self.status = t!("host_and_user_required").into();
            cx.notify();
            return None;
        }
        if kind == SessionKind::Telnet && host.is_empty() {
            self.status = t!("telnet_host_required").into();
            cx.notify();
            return None;
        }
        if kind == SessionKind::Serial && serial_port.is_empty() {
            self.status = t!("serial_port_required").into();
            cx.notify();
            return None;
        }

        if matches!(kind, SessionKind::Ssh | SessionKind::Telnet) && self.ssh_proxy_type != "none" {
            let proxy_host = self.proxy_host_input.read(cx).value().trim().to_string();
            let proxy_port_str = self.proxy_port_input.read(cx).value().trim().to_string();
            let proxy_port = proxy_port_str.parse::<u16>().ok();
            if proxy_host.is_empty() || proxy_port.is_none() {
                self.status = "Proxy host and port are required".into();
                cx.notify();
                return None;
            }
        }

        let name = if session_name.is_empty() {
            match kind {
                SessionKind::Ssh | SessionKind::Telnet => host.clone(),
                SessionKind::Serial => serial_port.clone(),
            }
        } else {
            session_name
        };
        let existing_id = self.editing_session_id.clone();
        let existing_session = existing_id
            .as_deref()
            .and_then(|id| self.config.get(id))
            .cloned();
        let existing_last_used = existing_session
            .as_ref()
            .and_then(|session| session.last_used.clone());
        let existing_last_successful_ssh_mode = existing_session
            .as_ref()
            .filter(|session| {
                kind == SessionKind::Ssh
                    && session.kind == SessionKind::Ssh
                    && session.host == host
                    && session.port == port
                    && session.user == user
            })
            .and_then(|session| session.last_successful_ssh_mode);

        let mut session = match kind {
            SessionKind::Ssh => match self.ssh_auth_method {
                AuthMethod::Password => Session::password(host, port, user, password),
                AuthMethod::Key => {
                    if key_path.is_empty() && key_inline.trim().is_empty() {
                        self.status = "private key path or content is required".into();
                        cx.notify();
                        return None;
                    }
                    Session::key(host, port, user, key_path, key_inline, passphrase)
                }
            },
            SessionKind::Telnet => Session::telnet(host, if port == 0 { 23 } else { port }),
            SessionKind::Serial => Session::serial(serial_port, baud_rate),
        };
        session.name = name;
        session.group_name = group_name;
        if let Some(id) = existing_id {
            session.id = id;
        }
        session.last_used = existing_last_used;
        session.last_successful_ssh_mode = existing_last_successful_ssh_mode;
        if matches!(kind, SessionKind::Ssh | SessionKind::Telnet) {
            session.proxy_type = self.ssh_proxy_type.clone();
            session.proxy_host = self.proxy_host_input.read(cx).value().trim().to_string();
            session.proxy_port = self
                .proxy_port_input
                .read(cx)
                .value()
                .trim()
                .parse::<u16>()
                .ok();
            session.proxy_user = self.proxy_user_input.read(cx).value().trim().to_string();
            session.proxy_password = self.proxy_password_input.read(cx).value().to_string();
        }
        if kind == SessionKind::Ssh {
            session.sftp_path = sftp_path;
            session.x11_forwarding = self.session_x11_forwarding;
        }
        if kind == SessionKind::Serial {
            session.data_bits = self
                .serial_data_bits_input
                .read(cx)
                .value()
                .trim()
                .parse::<u8>()
                .unwrap_or(8);
            session.parity = self
                .serial_parity_input
                .read(cx)
                .value()
                .trim()
                .to_ascii_lowercase();
            session.stop_bits = self
                .serial_stop_bits_input
                .read(cx)
                .value()
                .trim()
                .parse::<u8>()
                .unwrap_or(1);
            session.flow_control = self
                .serial_flow_control_input
                .read(cx)
                .value()
                .trim()
                .to_ascii_lowercase();
        }
        session.shortcut = self.session_shortcut.clone();
        self.config.upsert(session.clone());
        self.config.save_logged("save_session");

        Some(session)
    }

    pub(crate) fn save_session(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.save_session_from_form(cx).is_none() {
            return;
        }

        self.editing_session_id = None;
        self.active_dialog = None;
        self.status = "session saved".into();
        window.close_dialog(cx);
        cx.notify();
    }

    pub(crate) fn connect_session(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.save_session_from_form(cx) else {
            return;
        };

        self.open_session(session, cx);
        self.editing_session_id = None;
        self.active_dialog = None;
        self.focus_terminal_workspace(window, cx);
        window.close_dialog(cx);
        cx.notify();
    }

    pub(crate) fn set_input_value(
        input: &Entity<InputState>,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        input.update(cx, |state, cx| state.set_value(value, window, cx));
    }

    pub(crate) fn reset_session_form(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editing_session_id = None;
        self.recording_session_shortcut = false;
        self.session_shortcut_error = None;
        self.session_import_error = None;
        self.session_shortcut.clear();
        self.session_kind = SessionKind::Ssh;
        self.available_serial_ports = crate::backend::serial::available_port_names();
        self.ssh_auth_method = AuthMethod::Password;
        self.ssh_advanced_options_visible = false;
        Self::set_input_value(&self.session_name_input, "", window, cx);
        Self::set_input_value(&self.session_group_input, "", window, cx);
        Self::set_input_value(&self.host_input, "", window, cx);
        Self::set_input_value(&self.port_input, "22", window, cx);
        Self::set_input_value(&self.user_input, "root", window, cx);
        Self::set_input_value(&self.password_input, "", window, cx);
        Self::set_input_value(&self.key_path_input, "", window, cx);
        Self::set_input_value(&self.key_inline_input, "", window, cx);
        Self::set_input_value(&self.passphrase_input, "", window, cx);
        self.ssh_proxy_type = "none".to_string();
        Self::set_input_value(&self.proxy_host_input, "", window, cx);
        Self::set_input_value(&self.proxy_port_input, "", window, cx);
        Self::set_input_value(&self.proxy_user_input, "", window, cx);
        Self::set_input_value(&self.proxy_password_input, "", window, cx);
        Self::set_input_value(&self.session_sftp_path_input, "", window, cx);
        Self::set_input_value(&self.serial_port_input, "", window, cx);
        Self::set_input_value(&self.serial_baud_rate_input, "115200", window, cx);
        Self::set_input_value(&self.serial_data_bits_input, "8", window, cx);
        Self::set_input_value(&self.serial_parity_input, "none", window, cx);
        Self::set_input_value(&self.serial_stop_bits_input, "1", window, cx);
        Self::set_input_value(&self.serial_flow_control_input, "none", window, cx);
        self.session_x11_forwarding = true;
    }

    pub(crate) fn load_session_into_form(
        &mut self,
        session: &Session,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editing_session_id = Some(session.id.clone());
        self.session_kind = session.kind;
        self.ssh_auth_method = session.auth;
        Self::set_input_value(&self.session_name_input, session.name.clone(), window, cx);
        Self::set_input_value(
            &self.session_group_input,
            session.group_name.clone(),
            window,
            cx,
        );
        Self::set_input_value(&self.host_input, session.host.clone(), window, cx);
        Self::set_input_value(&self.port_input, session.port.to_string(), window, cx);
        Self::set_input_value(&self.user_input, session.user.clone(), window, cx);
        Self::set_input_value(&self.password_input, session.password.clone(), window, cx);
        Self::set_input_value(
            &self.key_path_input,
            session.private_key_path.clone(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.key_inline_input,
            session.private_key_inline.clone(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.passphrase_input,
            session.passphrase.clone(),
            window,
            cx,
        );
        self.ssh_proxy_type = if session.proxy_type.is_empty() {
            "none".to_string()
        } else {
            session.proxy_type.clone()
        };
        Self::set_input_value(
            &self.proxy_host_input,
            session.proxy_host.clone(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.proxy_port_input,
            session
                .proxy_port
                .map(|p| p.to_string())
                .unwrap_or_default(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.proxy_user_input,
            session.proxy_user.clone(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.proxy_password_input,
            session.proxy_password.clone(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.session_sftp_path_input,
            session.sftp_path.clone(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.serial_port_input,
            session.serial_port.clone(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.serial_baud_rate_input,
            session.baud_rate.to_string(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.serial_data_bits_input,
            session.data_bits.to_string(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.serial_parity_input,
            session.parity.clone(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.serial_stop_bits_input,
            session.stop_bits.to_string(),
            window,
            cx,
        );
        Self::set_input_value(
            &self.serial_flow_control_input,
            session.flow_control.clone(),
            window,
            cx,
        );
        self.session_x11_forwarding = session.x11_forwarding;
        self.ssh_advanced_options_visible = session.proxy_type != "none"
            || !session.sftp_path.trim().is_empty()
            || !session.shortcut.trim().is_empty()
            || !session.x11_forwarding;
        self.recording_session_shortcut = false;
        self.session_shortcut_error = None;
        self.session_import_error = None;
        self.session_shortcut = session.shortcut.clone();
    }

    pub(crate) fn pick_ssh_key_path(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let start_dir = directories::BaseDirs::new()
            .map(|d| d.home_dir().join(".ssh"))
            .unwrap_or_else(|| std::path::PathBuf::from("/"));

        let file_dialog = rfd::AsyncFileDialog::new()
            .set_directory(start_dir)
            .pick_file();

        cx.spawn_in(window, async move |this, mut cx| {
            if let Some(file) = file_dialog.await {
                let _ = gpui::AsyncWindowContext::update(&mut cx, |window, cx| {
                    let _ = this.update(cx, |this, cx| {
                        Self::set_input_value(
                            &this.key_path_input,
                            file.path().to_string_lossy().to_string(),
                            window,
                            cx,
                        );
                    });
                });
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    pub(crate) fn pick_xquartz_app_path(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let start_dir = if cfg!(target_os = "macos") {
            let configured = self
                .xquartz_app_path_input
                .read(cx)
                .value()
                .trim()
                .to_string();
            std::path::Path::new(&configured)
                .parent()
                .map(std::path::Path::to_path_buf)
                .unwrap_or_else(|| std::path::PathBuf::from("/Applications"))
        } else if cfg!(target_os = "windows") {
            std::env::var("ProgramFiles")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| std::path::PathBuf::from("C:\\"))
        } else {
            std::path::PathBuf::from("/usr/bin")
        };
        let dialog = rfd::AsyncFileDialog::new().set_directory(start_dir);
        #[cfg(target_os = "macos")]
        let file_dialog = dialog.pick_file_or_folder();
        #[cfg(not(target_os = "macos"))]
        let file_dialog = dialog.pick_file();

        cx.spawn_in(window, async move |this, mut cx| {
            if let Some(folder) = file_dialog.await {
                let _ = gpui::AsyncWindowContext::update(&mut cx, |window, cx| {
                    let _ = this.update(cx, |this, cx| {
                        Self::set_input_value(
                            &this.xquartz_app_path_input,
                            folder.path().to_string_lossy().to_string(),
                            window,
                            cx,
                        );
                    });
                });
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    pub(crate) fn reset_xquartz_app_path(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let default_path = crate::platform::x_server::default_app_path();
        Self::set_input_value(
            &self.xquartz_app_path_input,
            default_path.clone(),
            window,
            cx,
        );
        self.config.set_local_x_server_app_path(default_path);
        if let Err(err) = self.config.save() {
            tracing::error!(
                component = "x11",
                operation = "reset_local_x_server_path",
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to save local X server path"
            );
            self.status = format!("failed to save local X server path: {err:#}").into();
        } else {
            self.status = "local X server path reset".into();
        }
        cx.notify();
    }

    pub(crate) fn save_x11_settings(&mut self, cx: &mut Context<Self>) {
        let path = self
            .xquartz_app_path_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        self.config.set_local_x_server_app_path(path);
        if let Err(err) = self.config.save() {
            tracing::error!(
                component = "x11",
                operation = "save_settings",
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to save X11 settings"
            );
            self.status = format!("failed to save X11 settings: {err:#}").into();
        } else {
            self.status = "X11 settings saved".into();
        }
        cx.notify();
    }

    pub(crate) fn open_configured_xquartz(&mut self, cx: &mut Context<Self>) {
        self.save_x11_settings(cx);
        match crate::app::startup::launch_local_x_server_app(self.config.local_x_server_app_path())
        {
            Ok(display) => {
                self.status = format!("local X server launch requested at {display}").into();
            }
            Err(err) => {
                self.status = format!("failed to launch local X server: {err:#}").into();
            }
        }
        cx.notify();
    }

    pub(crate) fn open_new_ssh_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.reset_session_form(window, cx);
        self.show_ssh_dialog(window, cx);
    }

    pub(crate) fn edit_saved_session(
        &mut self,
        session_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(session) = self.config.get(&session_id).cloned() else {
            self.status = "saved session not found".into();
            cx.notify();
            return;
        };
        self.load_session_into_form(&session, window, cx);
        self.show_ssh_dialog(window, cx);
    }

    pub(crate) fn clone_saved_session(
        &mut self,
        session_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(session) = self.config.get(&session_id).cloned() else {
            self.status = "saved session not found".into();
            cx.notify();
            return;
        };
        self.load_session_into_form(&session, window, cx);
        self.editing_session_id = None;
        Self::set_input_value(
            &self.session_name_input,
            format!("{}-copy", session.name),
            window,
            cx,
        );
        self.show_ssh_dialog(window, cx);
    }

    pub(crate) fn terminal_cell_width(&self) -> f32 {
        self.appearance.terminal_font_metrics.cell_width
    }

    pub(crate) fn terminal_line_height(&self) -> f32 {
        self.appearance.terminal_font_metrics.line_height
    }

    pub(crate) fn update_terminal_font_metrics(&mut self, cell_width: f32, line_height: f32) {
        let next = crate::app::TerminalFontMetrics {
            cell_width: cell_width.max(6.0),
            line_height: line_height.max(self.appearance.terminal_font_size + 2.0),
        };
        if self.appearance.terminal_font_metrics != next {
            self.appearance.terminal_font_metrics = next;
        }
    }

    pub(crate) fn change_terminal_font_size(&mut self, delta: f32, cx: &mut Context<Self>) {
        self.appearance.terminal_font_size =
            (self.appearance.terminal_font_size + delta).clamp(10.0, 24.0);
        self.appearance.terminal_font_metrics =
            crate::app::TerminalFontMetrics::fallback(self.appearance.terminal_font_size);
        self.config
            .set_terminal_font_size(self.appearance.terminal_font_size);
        self.config.save_logged("set_terminal_font_size");
        self.status = format!(
            "terminal font size: {:.0}px",
            self.appearance.terminal_font_size
        )
        .into();
        cx.notify();
    }

    pub(crate) fn change_ui_font_size(&mut self, delta: f32, cx: &mut Context<Self>) {
        self.appearance.ui_font_size = (self.appearance.ui_font_size + delta).clamp(8.0, 24.0);
        self.config.set_ui_font_size(self.appearance.ui_font_size);
        self.config.save_logged("set_ui_font_size");
        Theme::global_mut(cx).font_size = px(self.appearance.ui_font_size);
        self.status = format!("UI font size: {:.0}px", self.appearance.ui_font_size).into();
        cx.notify();
    }

    pub(crate) fn change_ui_font_brightness(
        &mut self,
        delta: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config
            .set_ui_font_brightness(self.appearance.ui_font_brightness + delta);
        self.appearance.ui_font_brightness = self.config.ui_font_brightness();
        self.config.save_logged("set_ui_font_brightness");
        self.apply_theme_preferences(window, cx);
        self.status = format!(
            "UI font brightness: {:.2}",
            self.appearance.ui_font_brightness
        )
        .into();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn change_terminal_font_brightness(
        &mut self,
        delta: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config
            .set_terminal_font_brightness(self.appearance.terminal_font_brightness + delta);
        self.appearance.terminal_font_brightness = self.config.terminal_font_brightness();
        self.config.save_logged("set_terminal_font_brightness");
        self.status = format!(
            "terminal font brightness: {:.2}",
            self.appearance.terminal_font_brightness
        )
        .into();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn change_ui_font_family(
        &mut self,
        family: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Err(err) = crate::app::theme::ensure_embedded_font_family_loaded(family, cx) {
            tracing::warn!(
                component = "theme",
                operation = "load_selected_ui_font",
                font_family = family,
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to load selected embedded UI font"
            );
            self.status = format!("failed to load UI font: {family}").into();
            cx.notify();
            return;
        }
        self.appearance.ui_font_family = family.into();
        self.config.set_ui_font_family(family);
        self.config.save_logged("set_ui_font_family");
        crate::app::theme::set_theme_font_names(
            Theme::global_mut(cx),
            &self.appearance.ui_font_family,
        );
        cx.notify();
        window.refresh();
    }

    pub(crate) fn change_terminal_font_family(&mut self, family: &str, cx: &mut Context<Self>) {
        if let Err(err) = crate::app::theme::ensure_embedded_font_family_loaded(family, cx) {
            tracing::warn!(
                component = "theme",
                operation = "load_selected_terminal_font",
                font_family = family,
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to load selected embedded terminal font"
            );
            self.status = format!("failed to load terminal font: {family}").into();
            cx.notify();
            return;
        }
        self.appearance.terminal_font_family = family.into();
        self.appearance.terminal_font_metrics =
            crate::app::TerminalFontMetrics::fallback(self.appearance.terminal_font_size);
        self.config.set_terminal_font_family(family);
        self.config.save_logged("set_terminal_font_family");
        cx.notify();
    }

    pub(crate) fn change_cursor_style(
        &mut self,
        style: crate::config::CursorStyle,
        cx: &mut Context<Self>,
    ) {
        self.appearance.cursor_style = style;
        self.config.set_cursor_style(style);
        self.config.save_logged("set_cursor_style");
        cx.notify();
    }

    pub(crate) fn reset_layout(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.config.set_layout_state(None, None, None);
        self.config.save_logged("reset_layout");

        self.is_layout_reset = true;
        self.workspace_panels = cx.new(|_| crate::app::resizable::ResizableState::default());
        self.body_panels = cx.new(|_| crate::app::resizable::ResizableState::default());

        cx.notify();
    }

    pub(crate) fn set_ssh_auth_method(&mut self, method: AuthMethod, cx: &mut Context<Self>) {
        self.ssh_auth_method = method;
        cx.notify();
    }

    pub(crate) fn set_session_kind(
        &mut self,
        kind: SessionKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.session_kind = kind;
        if kind == SessionKind::Serial {
            self.available_serial_ports = crate::backend::serial::available_port_names();
        }
        if kind == SessionKind::Telnet && self.port_input.read(cx).value().trim() == "22" {
            Self::set_input_value(&self.port_input, "23", window, cx);
        }
        if kind == SessionKind::Ssh && self.port_input.read(cx).value().trim() == "23" {
            Self::set_input_value(&self.port_input, "22", window, cx);
        }
        cx.notify();
    }

    pub(crate) fn refresh_available_serial_ports(&mut self, cx: &mut Context<Self>) {
        self.available_serial_ports = crate::backend::serial::available_port_names();
        cx.notify();
    }

    pub(crate) fn set_ssh_proxy_type(&mut self, proxy_type: String, cx: &mut Context<Self>) {
        self.ssh_proxy_type = proxy_type;
        cx.notify();
    }

    pub(crate) fn connect_saved_session(&mut self, session_id: String, cx: &mut Context<Self>) {
        tracing::info!(
            component = "session",
            operation = "connect_saved",
            session_id,
            "Connecting saved session"
        );
        let Some(session) = self.config.get(&session_id).cloned() else {
            self.status = "saved session not found".into();
            cx.notify();
            return;
        };
        self.open_session(session, cx);
        self.set_workspace_page(WorkspacePage::Terminal, cx);
    }

    pub(crate) fn connect_saved_session_and_focus(
        &mut self,
        session_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.connect_saved_session(session_id, cx);
        self.focus_terminal_workspace(window, cx);
    }

    pub(crate) fn record_session_shortcut(
        &mut self,
        event: &gpui::KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.recording_session_shortcut {
            return false;
        }

        window.prevent_default();
        cx.stop_propagation();
        if event.keystroke.key == "escape" {
            self.recording_session_shortcut = false;
            self.session_shortcut_error = None;
            cx.notify();
            return true;
        }

        let Some(shortcut) = crate::app::keybinding_recorder::session_shortcut_for_event(event)
        else {
            self.recording_session_shortcut = false;
            self.session_shortcut_error =
                Some(t!("session_shortcut_requires_modifier").to_string());
            cx.notify();
            return true;
        };

        if let Some(conflict) = crate::app::keybinding_recorder::find_session_shortcut_conflict(
            &self.config,
            self.editing_session_id.as_deref(),
            &shortcut,
        ) {
            self.recording_session_shortcut = false;
            self.session_shortcut_error = Some(
                t!(
                    "session_shortcut_conflict",
                    key = crate::app::keybinding_recorder::format_keystroke(&shortcut),
                    target = conflict
                )
                .to_string(),
            );
            cx.notify();
            return true;
        }

        self.recording_session_shortcut = false;
        self.session_shortcut_error = None;
        self.session_shortcut = shortcut;
        cx.notify();
        true
    }

    pub(crate) fn connect_session_shortcut_if_matched(
        &mut self,
        event: &gpui::KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(session_id) =
            crate::app::keybinding_recorder::session_shortcut_match(&self.config, event)
        else {
            return false;
        };

        self.connect_saved_session_and_focus(session_id, window, cx);
        window.prevent_default();
        cx.stop_propagation();
        true
    }

    pub(crate) fn open_saved_session_sftp_only(
        &mut self,
        session_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        tracing::info!(
            component = "sftp",
            operation = "open_saved_sftp_only",
            session_id,
            "Opening saved SSH session as SFTP-only page"
        );
        let Some(session) = self.config.get(&session_id).cloned() else {
            self.status = "saved session not found".into();
            cx.notify();
            return;
        };
        if !session.kind.supports_sftp() {
            self.status = t!("sftp_requires_ssh_session").into();
            cx.notify();
            return;
        }

        self.expanded_saved_groups
            .insert(normalize_session_group_name(&session.group_name));
        if let Some(active_group_id) = self.active_group.clone()
            && let Some(group) = self
                .tab_groups
                .iter_mut()
                .find(|group| group.id == active_group_id)
        {
            group.pane_root = self.pane_root.clone();
        }

        let group_id = Uuid::new_v4().to_string();
        let title = session.name.clone();
        let instance_number = self.next_workspace_group_instance(&title);
        self.tab_groups.push(TabGroup {
            id: group_id.clone(),
            title,
            instance_number,
            pane_root: PaneLayout::Single(String::new()),
            sftp: Some(crate::app::SftpUiState {
                current_path: "/".into(),
                has_opened_directory: false,
                status: rust_i18n::t!("sftp_connecting").to_string(),
                entries: Vec::new(),
                has_more_entries: false,
                loading_more_entries: false,
                reached_entries_limit: false,
                selected_path: None,
                preview: None,
                selected_entries: std::collections::HashSet::new(),
                edit_sessions: Vec::new(),
                opening_edit_paths: std::collections::HashSet::new(),
                home_dir: "/".into(),
                connection_may_be_stale: false,
            }),
            sftp_page_open: true,
            sftp_session: Some(session),
        });

        self.active_group = Some(group_id.clone());
        self.active_tab = None;
        self.pane_root = PaneLayout::Single(String::new());
        self.focused_pane_path = vec![];
        self.pending_sftp_path_sync = Some("/".into());
        self.ensure_sftp_handle_for_group(&group_id);
        self.mark_sftp_activity_for_group(&group_id);
        self.set_workspace_page(WorkspacePage::Sftp, cx);
        self.focus_handle.focus(window, cx);
        self.status = "SFTP page opened".into();
        cx.notify();
    }

    pub(crate) fn focus_terminal_workspace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_workspace_page(WorkspacePage::Terminal, cx);
        self.focus_handle.focus(window, cx);
    }

    pub(crate) fn open_session(&mut self, session: Session, cx: &mut Context<Self>) {
        match session.kind {
            SessionKind::Ssh => self.open_ssh_session(session, cx),
            SessionKind::Serial | SessionKind::Telnet => self.open_raw_session(session, cx),
        }
    }

    fn open_raw_session(&mut self, session: Session, cx: &mut Context<Self>) {
        self.expanded_saved_groups
            .insert(normalize_session_group_name(&session.group_name));
        let id = Uuid::new_v4().to_string();
        let backend = match session.kind {
            SessionKind::Serial => serial::spawn_serial_terminal(
                id.clone(),
                session.clone(),
                self.runtime_state.events_tx.clone(),
            ),
            SessionKind::Telnet => {
                let (runtime, task_tracker) = self.runtime_state.runtime_handle_and_tracker();
                telnet::spawn_telnet_terminal(
                    &runtime,
                    task_tracker,
                    id.clone(),
                    session.clone(),
                    DEFAULT_COLS,
                    DEFAULT_ROWS,
                    self.runtime_state.events_tx.clone(),
                )
            }
            SessionKind::Ssh => unreachable!("SSH uses open_ssh_session"),
        };
        self.tabs.push(TerminalTab::new_serial_or_telnet(
            id.clone(),
            &session,
            backend,
            self.runtime_state.events_tx.clone(),
        ));
        self.active_tab = Some(id.clone());
        self.connection_progress = Some(crate::app::ConnectionProgress {
            tab_id: id.clone(),
            title: t!("connecting").into(),
            lines: vec![t!("starting_connection").into()],
            failed: false,
        });
        self.pane_root = PaneLayout::Single(id.clone());
        self.focused_pane_path = vec![];
        let group_id = Uuid::new_v4().to_string();
        let title = session.name.clone();
        let instance_number = self.next_workspace_group_instance(&title);
        self.tab_groups.push(TabGroup {
            id: group_id.clone(),
            title,
            instance_number,
            pane_root: PaneLayout::Single(id),
            sftp: None,
            sftp_page_open: false,
            sftp_session: None,
        });
        self.active_group = Some(group_id);
        self.ensure_active_workspace_tab_visible();
        self.set_workspace_page(WorkspacePage::Terminal, cx);
        self.status = match session.kind {
            SessionKind::Serial => "serial session opened".into(),
            SessionKind::Telnet => "telnet session opened".into(),
            SessionKind::Ssh => unreachable!(),
        };
        cx.notify();
    }

    pub(crate) fn open_ssh_session(&mut self, session: Session, cx: &mut Context<Self>) {
        tracing::info!(
            component = "session",
            operation = "open_ssh",
            session_name = %crate::diagnostics::mask_value(&session.name),
            user = %crate::diagnostics::mask_value(&session.user),
            host = %crate::diagnostics::mask_host(&session.host),
            port = session.port,
            "Opening SSH tab"
        );
        self.expanded_saved_groups
            .insert(normalize_session_group_name(&session.group_name));
        let id = Uuid::new_v4().to_string();
        let prompt_for_password = should_prompt_for_terminal_password_before_connect(&session);
        let backend = if prompt_for_password {
            BackendTx::inactive()
        } else {
            let (runtime, task_tracker) = self.runtime_state.runtime_handle_and_tracker();
            ssh::spawn_ssh_terminal(
                &runtime,
                task_tracker,
                id.clone(),
                session.clone(),
                DEFAULT_COLS,
                DEFAULT_ROWS,
                self.runtime_state.events_tx.clone(),
            )
        };
        self.tabs.push(TerminalTab::new_ssh(
            id.clone(),
            &session,
            backend,
            self.runtime_state.events_tx.clone(),
        ));
        self.active_tab = Some(id.clone());
        if prompt_for_password {
            self.connection_progress = None;
            self.terminal_password_prompt =
                Some(crate::app::TerminalPasswordPrompt::new(id.clone()));
        } else {
            self.connection_progress = Some(crate::app::ConnectionProgress {
                tab_id: id.clone(),
                title: rust_i18n::t!("connecting").into(),
                lines: vec![rust_i18n::t!("starting_connection").into()],
                failed: false,
            });
        }
        self.pane_root = PaneLayout::Single(id.clone());
        self.focused_pane_path = vec![];
        let group_id = Uuid::new_v4().to_string();
        let title = session.name.clone();
        let instance_number = self.next_workspace_group_instance(&title);
        self.tab_groups.push(TabGroup {
            id: group_id.clone(),
            title,
            instance_number,
            pane_root: PaneLayout::Single(id.clone()),
            sftp: Some(crate::app::SftpUiState {
                current_path: "/".into(),
                has_opened_directory: false,
                status: rust_i18n::t!("sftp_connecting").to_string(),
                entries: Vec::new(),
                has_more_entries: false,
                loading_more_entries: false,
                reached_entries_limit: false,
                selected_path: None,
                preview: None,
                selected_entries: std::collections::HashSet::new(),
                edit_sessions: Vec::new(),
                opening_edit_paths: std::collections::HashSet::new(),
                home_dir: "/".into(),
                connection_may_be_stale: false,
            }),
            sftp_page_open: false,
            sftp_session: Some(session.clone()),
        });
        self.active_group = Some(group_id.clone());
        self.ensure_active_workspace_tab_visible();
        if let Some(session_id) = self.active_session_id() {
            if let Some(index) = self.saved_sidebar_visible_row_index_for_session(session_id) {
                self.saved_scroll_handle
                    .scroll_to_item(index, gpui::ScrollStrategy::Nearest);
            }
        }
        self.active_tab = Some(id.clone());
        self.pending_sftp_path_sync = Some("/".into());
        if prompt_for_password {
            if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == id) {
                tab.status = "waiting for password".into();
            }
            self.feed_terminal_tab_bytes(&id, b"Password: ");
            self.status = "waiting for ssh password".into();
        } else {
            self.status = "ssh tab opened".into();
        }
        cx.notify();
    }

    pub(crate) fn remove_saved_session(&mut self, session_id: String, cx: &mut Context<Self>) {
        let removed_group_name = self
            .config
            .get(&session_id)
            .map(|session| normalize_session_group_name(&session.group_name))
            .unwrap_or_default();
        self.config.remove(&session_id);
        if !removed_group_name.is_empty()
            && !self.config.sessions().iter().any(|session| {
                normalize_session_group_name(&session.group_name) == removed_group_name
            })
        {
            self.expanded_saved_groups.remove(&removed_group_name);
            if self.renaming_saved_group.as_deref() == Some(removed_group_name.as_str()) {
                self.renaming_saved_group = None;
            }
        }
        self.config.save_logged("remove_saved_session");
        self.status = "session removed".into();
        cx.notify();
    }

    /// Retry a single disconnected tab by its ID.
    /// Reopens the matching local, SSH, serial, or Telnet backend.
    ///
    /// The existing `TerminalTab` (including its `term` scrollback history)
    /// is preserved — only the backend is swapped via `set_backend()`.
    pub(crate) fn retry_disconnected_tab(&mut self, tab_id: &str, cx: &mut Context<Self>) {
        let Some(ix) = self.tabs.iter().position(|t| t.id == tab_id) else {
            return;
        };
        if self.tabs[ix].connected || self.tabs[ix].disconnected_reason.is_none() {
            return;
        }

        let kind = self.tabs[ix].kind;
        let session = self.tabs[ix].session.clone();
        let local_shell_profile = self.tabs[ix].local_shell_profile.clone();
        let new_generation = self.tabs[ix].backend_generation + 1;
        let cols = self.tabs[ix].cols;
        let rows = self.tabs[ix].rows;

        // Close old backend (sends Close through the shared Arc<Mutex>)
        self.tabs[ix].send_backend(BackendCommand::Close);

        if let Some(session) = session {
            let backend = match kind {
                TabKind::Ssh => {
                    let (runtime, task_tracker) = self.runtime_state.runtime_handle_and_tracker();
                    ssh::spawn_ssh_terminal(
                        &runtime,
                        task_tracker,
                        tab_id.to_string(),
                        session.clone(),
                        cols,
                        rows,
                        self.runtime_state.events_tx.clone(),
                    )
                }
                TabKind::Serial => serial::spawn_serial_terminal(
                    tab_id.to_string(),
                    session.clone(),
                    self.runtime_state.events_tx.clone(),
                ),
                TabKind::Telnet => {
                    let (runtime, task_tracker) = self.runtime_state.runtime_handle_and_tracker();
                    telnet::spawn_telnet_terminal(
                        &runtime,
                        task_tracker,
                        tab_id.to_string(),
                        session.clone(),
                        cols,
                        rows,
                        self.runtime_state.events_tx.clone(),
                    )
                }
                TabKind::Local => unreachable!("local terminals do not have sessions"),
            };

            // Swap the backend — the Term's internal listener shares the
            // same Arc<Mutex<BackendTx>>, so user input is automatically
            // routed to the new backend. Terminal history is preserved.
            self.tabs[ix].set_backend(backend);
            self.tabs[ix].connected = false;
            self.tabs[ix].status = "connecting".into();
            self.tabs[ix].disconnected_reason = None;
            self.tabs[ix].backend_generation = new_generation;
            self.tabs[ix].backend_initialized = false;

            // Only SSH workspaces have an associated SFTP backend.
            if kind == TabKind::Ssh
                && let Some(group) = self
                    .tab_groups
                    .iter()
                    .find(|g| g.pane_root.contains(tab_id))
            {
                let group_id = group.id.clone();
                let group_session = self
                    .tabs
                    .iter()
                    .find(|t| group.pane_root.contains(&t.id) && t.session.is_some())
                    .and_then(|t| t.session.clone());

                if group_session.is_some() {
                    self.restart_sftp_handle_for_group(&group_id);
                }
            }
        } else {
            // Local tab: spawn new local shell
            let profile =
                local_shell_profile.unwrap_or_else(|| self.config.default_local_shell_profile());
            match local::spawn_local_terminal(
                tab_id.to_string(),
                cols,
                rows,
                &profile,
                self.runtime_state.events_tx.clone(),
            ) {
                Ok(backend) => {
                    // Swap the backend — preserves terminal history.
                    self.tabs[ix].set_backend(backend);
                    self.tabs[ix].connected = true;
                    self.tabs[ix].status = "local shell".into();
                    self.tabs[ix].local_shell_profile = Some(profile);
                    self.tabs[ix].disconnected_reason = None;
                    self.tabs[ix].backend_generation = new_generation;
                    self.tabs[ix].backend_initialized = false;
                    // Resize the new PTY to match the pane dimensions.
                    self.tabs[ix].send_backend(BackendCommand::Resize { cols, rows });
                }
                Err(err) => {
                    tracing::error!(
                        component = "local_terminal",
                        operation = "reopen",
                        tab_id,
                        error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                        "Failed to reopen local terminal"
                    );
                    self.status = format!("failed to reopen local terminal: {err:#}").into();
                    cx.notify();
                    return;
                }
            }
        }

        self.status = match kind {
            TabKind::Local => "local tab reopened",
            TabKind::Ssh => "ssh tab retrying",
            TabKind::Serial => "serial session reopening",
            TabKind::Telnet => "telnet session reopening",
        }
        .into();
        cx.notify();
    }

    #[allow(dead_code)]
    pub(crate) fn activate_tab(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
        // Save current group state
        if let Some(group_id) = self.active_group.clone() {
            if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == group_id) {
                group.pane_root = self.pane_root.clone();
            }
        }
        // Find which group this tab belongs to and restore its pane_root
        let tab_group = self
            .tab_groups
            .iter_mut()
            .find(|g| g.pane_root.contains(&id));
        if let Some(group) = tab_group {
            self.pane_root = group.pane_root.clone();
            self.active_group = Some(group.id.clone());
            // Focus the activated tab in the pane tree
            self.focus_pane_with_id(id.clone());
        } else {
            self.pane_root = PaneLayout::Single(id.clone());
            self.focused_pane_path = vec![];
        }
        if self.tabs.iter().any(|t| t.id == id) {
            if let Some(session_id) = self.active_session_id() {
                if let Some(index) = self.saved_sidebar_visible_row_index_for_session(session_id) {
                    self.saved_scroll_handle
                        .scroll_to_item(index, gpui::ScrollStrategy::Nearest);
                }
            }
            self.ensure_active_workspace_tab_visible();
        }
        self.focus_handle.focus(window, cx);
        self.sync_system_tab_to_active_group();
        cx.notify();
    }

    pub(crate) fn close_tab(&mut self, id: String, cx: &mut Context<Self>) {
        self.handle_tab_close(id, cx);
        cx.notify();
    }

    /// Discard UI state that is meaningful only while a terminal tab exists.
    fn clear_tab_ui_state(&mut self, tab_id: &str) {
        self.terminal_scrollbars.remove(tab_id);
        self.terminal_bounds.remove(tab_id);

        if self
            .hovered_url
            .as_ref()
            .is_some_and(|hovered| hovered.tab_id == tab_id)
        {
            self.hovered_url = None;
        }
        if self
            .terminal_composition
            .as_ref()
            .is_some_and(|composition| composition.tab_id == tab_id)
        {
            self.terminal_composition = None;
        }
        if self
            .terminal_frozen_selection
            .as_ref()
            .is_some_and(|selection| selection.tab_id == tab_id)
        {
            self.terminal_frozen_selection = None;
        }
        if self
            .connection_progress
            .as_ref()
            .is_some_and(|progress| progress.tab_id == tab_id)
        {
            self.connection_progress = None;
        }
        if self
            .terminal_password_prompt
            .as_ref()
            .is_some_and(|prompt| prompt.tab_id == tab_id)
        {
            self.terminal_password_prompt = None;
        }
        self.terminal_password_retry_tabs.remove(tab_id);
        if self.search.target_tab.as_deref() == Some(tab_id) {
            self.search.query.clear();
            self.search.matches.clear();
            self.search.current = 0;
            self.search.target_tab = None;
        }
        if self.active_tab.as_deref() == Some(tab_id) {
            self.terminal_selecting = false;
        }
    }

    pub(crate) fn handle_tab_close(&mut self, id: String, cx: &mut Context<Self>) {
        let group_ix = self
            .tab_groups
            .iter()
            .position(|g| g.pane_root.contains(&id));
        let Some(ref group) = group_ix.map(|i| self.tab_groups[i].clone()) else {
            // Fallback: find and close individual tab
            tracing::info!(
                "[handle_tab_close] no group found for tab '{}', closing individually",
                id
            );
            self.clear_tab_ui_state(&id);
            if let Some(ix) = self.tabs.iter().position(|tab| tab.id == id) {
                self.tabs[ix].send_backend(BackendCommand::Close);
                self.tabs.remove(ix);
            }
            return;
        };

        let pane_ids = group.pane_root.tab_ids();
        let pane_ids_str: Vec<&str> = pane_ids.iter().map(|s| *s).collect();
        let is_group_close = pane_ids.len() <= 1;
        tracing::info!(
            "[handle_tab_close] id='{}' group_panes={:?} is_group_close={}",
            id,
            pane_ids_str,
            is_group_close
        );

        let was_active = self.active_tab.as_deref() == Some(id.as_str());
        let mut next_active_id = None;
        if was_active {
            let tabs_in_group: Vec<&str> = group
                .pane_root
                .tab_ids()
                .into_iter()
                .filter(|tab_id| !tab_id.is_empty())
                .collect();
            if let Some(pos) = tabs_in_group.iter().position(|&s| s == id.as_str()) {
                if pos > 0 {
                    next_active_id = Some(tabs_in_group[pos - 1].to_string());
                } else if pos + 1 < tabs_in_group.len() {
                    next_active_id = Some(tabs_in_group[pos + 1].to_string());
                }
            }
            if next_active_id.is_none() {
                // Find next group's active tab
                let all_groups = &self.tab_groups;
                if let Some(pos) = all_groups.iter().position(|g| g.id == group.id) {
                    if pos > 0 {
                        next_active_id = all_groups[pos - 1]
                            .pane_root
                            .tab_ids()
                            .into_iter()
                            .find(|tab_id| !tab_id.is_empty())
                            .map(String::from);
                    } else if pos + 1 < all_groups.len() {
                        next_active_id = all_groups[pos + 1]
                            .pane_root
                            .tab_ids()
                            .into_iter()
                            .find(|tab_id| !tab_id.is_empty())
                            .map(String::from);
                    }
                }
            }
        }
        if is_group_close {
            // Close all tabs in the group
            let tab_ids: Vec<String> = group
                .pane_root
                .tab_ids()
                .iter()
                .filter(|tab_id| !tab_id.is_empty())
                .map(|s| s.to_string())
                .collect();
            for tab_id in &tab_ids {
                self.clear_tab_ui_state(tab_id);
                if let Some(ix) = self.tabs.iter().position(|tab| tab.id == *tab_id) {
                    self.tabs[ix].send_backend(BackendCommand::Close);
                    self.tabs.retain(|t| t.id != *tab_id);
                }
            }
            self.release_sftp_handle_for_group(&group.id, true);
            self.tab_groups.remove(group_ix.unwrap());
            self.pane_root.remove_tab(&id);
        } else {
            // Just remove this tab from the group
            self.clear_tab_ui_state(&id);
            if let Some(ix) = self.tabs.iter().position(|tab| tab.id == id) {
                self.tabs[ix].send_backend(BackendCommand::Close);
                self.tabs.retain(|t| t.id != id);
            }
            if let Some(g) = self
                .tab_groups
                .iter_mut()
                .find(|g| g.pane_root.contains(&id))
            {
                g.pane_root.remove_tab(&id);
            }
            self.pane_root.remove_tab(&id);
            self.sync_pane_root_to_group();
        }

        if self.tab_groups.is_empty() {
            self.pane_root = PaneLayout::Single(String::new());
            self.focused_pane_path = vec![];
            self.active_tab = None;
            self.active_group = None;
            self.workspace_page = WorkspacePage::Terminal;
            self.tab_groups.clear();
            self.tabs.clear();
            self.monitoring.system_tab_id = None;
            self.monitoring.cpu_history.clear();
            self.monitoring.net_rx_history.clear();
            self.monitoring.net_tx_history.clear();
            self.monitoring.status = None;
            let group_ids = self.sftp_handles.keys().cloned().collect::<Vec<_>>();
            for group_id in group_ids {
                self.release_sftp_handle_for_group(&group_id, true);
            }
            return;
        }

        if self.tabs.is_empty() {
            self.active_tab = None;
            self.monitoring.system_tab_id = None;
            self.monitoring.cpu_history.clear();
            self.monitoring.net_rx_history.clear();
            self.monitoring.net_tx_history.clear();
            self.monitoring.status = None;
            if let Some(group) = self
                .tab_groups
                .iter()
                .find(|group| group.sftp.is_some() && group.sftp_page_open)
            {
                self.active_group = Some(group.id.clone());
                self.pane_root = group.pane_root.clone();
                self.focused_pane_path = vec![];
                self.workspace_page = WorkspacePage::Sftp;
                self.restore_active_local_sftp_path(cx);
                self.ensure_active_workspace_tab_visible();
            } else {
                self.active_group = None;
                self.pane_root = PaneLayout::Single(String::new());
                self.focused_pane_path = vec![];
                self.workspace_page = WorkspacePage::Terminal;
            }
            return;
        }

        if was_active
            || self
                .active_tab
                .as_ref()
                .is_some_and(|active_id| !self.tabs.iter().any(|tab| &tab.id == active_id))
        {
            // Activate next available pane
            let new_id = next_active_id.or_else(|| {
                self.pane_root
                    .tab_ids()
                    .into_iter()
                    .find(|tab_id| !tab_id.is_empty())
                    .map(String::from)
                    .or_else(|| self.tabs.first().map(|t| t.id.clone()))
            });
            if let Some(new_id) = new_id {
                self.active_tab = Some(new_id.clone());
                if let Some(g) = self
                    .tab_groups
                    .iter()
                    .find(|g| g.pane_root.contains(&new_id))
                {
                    self.active_group = Some(g.id.clone());
                    self.pane_root = g.pane_root.clone();
                }
                self.focus_pane_with_id(new_id);
            }
        } else {
            // Pane root structure may have changed (e.g. sibling removed), recalc path
            if let Some(active_id) = self.active_tab.clone() {
                self.focus_pane_with_id(active_id);
            }
        }
        if self.workspace_page == WorkspacePage::Sftp && !self.active_group_sftp_page_open() {
            self.workspace_page = WorkspacePage::Terminal;
        }
        self.sync_system_tab_to_active_group();
        self.ensure_active_workspace_tab_visible();
    }

    pub(crate) fn focus_terminal(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // If the search bar is visible and the click is inside it, let the
        // search bar handle the event instead of switching pane focus.
        if self.search.active {
            if let Some(bounds) = self.search.bar_bounds {
                if bounds.contains(&event.position) {
                    return;
                }
            }
        }
        self.focus_handle.focus(window, cx);
        // Check if click is in a different pane and focus it
        let click_pos = event.position;
        let current_active = self.active_tab.clone();
        let clicked_tab_id = self.terminal_bounds.iter().find_map(|(id, bounds)| {
            if bounds.contains(&click_pos) {
                Some(id.clone())
            } else {
                None
            }
        });
        if let Some(tab_id) = clicked_tab_id {
            if current_active.as_deref() != Some(tab_id.as_str()) {
                self.focus_pane_with_id(tab_id.clone());
                cx.notify();
            }
        }
        if event.button == MouseButton::Left {
            if terminal_link_activation_modifier_pressed(&event.modifiers) {
                if let Some((row, col, _side)) = self.terminal_grid_point_and_side(event.position) {
                    if let Some(snapshot) = self.active_snapshot() {
                        if let Some((target, _)) =
                            crate::terminal::highlight::find_terminal_target_at_cell(
                                &snapshot.visible_rows,
                                row,
                                col,
                            )
                        {
                            match target {
                                crate::terminal::highlight::TerminalTarget::Url(url) => {
                                    let _ = open::that(&url);
                                    return;
                                }
                                crate::terminal::highlight::TerminalTarget::Path(path) => {
                                    if self.open_sftp_and_reveal_path(&path, window, cx) {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if self.config.right_click_copy_paste() {
                if let Some(text) = self.active_terminal_selection_text() {
                    if !text.is_empty() {
                        cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
                        if let Some(active_id) = self.active_tab.clone() {
                            self.clear_terminal_selection_for_tab(&active_id);
                        }
                    }
                }
            }
            self.begin_terminal_selection(event, cx);
        }
        cx.notify();
    }

    pub(crate) fn active_snapshot(&self) -> Option<Rc<RenderSnapshot>> {
        let keyword_highlight_enabled = self.config.keyword_highlight();
        self.active_tab
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))
            .map(|tab| tab.render_snapshot(keyword_highlight_enabled))
    }

    pub(crate) fn active_kind(&self) -> Option<TabKind> {
        self.active_tab
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))
            .map(|tab| tab.kind)
    }

    pub(crate) fn active_title(&self) -> String {
        self.active_tab
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))
            .map(|t| t.title.clone())
            .or_else(|| {
                self.active_group
                    .as_ref()
                    .and_then(|group_id| self.tab_groups.iter().find(|group| &group.id == group_id))
                    .and_then(|group| group.sftp_session.as_ref())
                    .map(|session| format!("sftp / {}", session.name))
            })
            .unwrap_or_else(|| t!("idle_no_session").into())
    }

    pub(crate) fn active_ssh_session(&self) -> Option<(String, Session)> {
        let active_id = self.active_tab.as_ref()?;
        let tab = self.tabs.iter().find(|tab| &tab.id == active_id)?;
        if tab.kind != TabKind::Ssh || !tab.connected {
            return None;
        }
        Some((tab.id.clone(), tab.session.clone()?))
    }

    pub(crate) fn active_session_id(&self) -> Option<&str> {
        self.active_tab
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|tab| &tab.id == id))
            .and_then(|tab| tab.session.as_ref())
            .map(|session| session.id.as_str())
            .or_else(|| {
                self.active_group
                    .as_ref()
                    .and_then(|group_id| self.tab_groups.iter().find(|group| &group.id == group_id))
                    .and_then(|group| group.sftp_session.as_ref())
                    .map(|session| session.id.as_str())
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::session::SessionKind;

    use super::default_port_for_session_kind;

    #[test]
    fn session_kind_uses_protocol_default_port() {
        assert_eq!(default_port_for_session_kind(SessionKind::Ssh), 22);
        assert_eq!(default_port_for_session_kind(SessionKind::Telnet), 23);
        assert_eq!(default_port_for_session_kind(SessionKind::Serial), 0);
    }
}
