use gpui::{
    AppContext as _, Context, Entity, MouseButton, MouseDownEvent, SharedString, Window, px,
};
use gpui_component::{Theme, WindowExt as _, input::InputState};
use rust_i18n::t;
use uuid::Uuid;

use crate::{
    AxShell, PaneLayout, TabGroup,
    app::WorkspacePage,
    app::constants::{DEFAULT_COLS, DEFAULT_ROWS},
    backend::{local, ssh},
    session::{AuthMethod, Session},
    terminal::{BackendCommand, RenderSnapshot, TabKind, TerminalTab},
};

pub(super) fn mask_session_part(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "***".to_string();
    }

    let chars: Vec<char> = trimmed.chars().collect();
    if chars.len() <= 2 {
        return "*".to_string();
    }
    if chars.len() <= 4 {
        return format!("{}*", chars[0]);
    }

    let prefix: String = chars.iter().take(2).collect();
    let suffix: String = chars.iter().skip(chars.len().saturating_sub(2)).collect();
    format!("{prefix}*{suffix}")
}

pub(super) fn mask_session_host(host: &str) -> String {
    let trimmed = host.trim();
    let ipv4_parts: Vec<&str> = trimmed.split('.').collect();
    if ipv4_parts.len() == 4
        && ipv4_parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()))
    {
        return format!("{}.*.*.{}", ipv4_parts[0], ipv4_parts[3]);
    }

    let ipv6_parts: Vec<&str> = trimmed.split(':').filter(|part| !part.is_empty()).collect();
    if trimmed.contains(':') && ipv6_parts.len() >= 2 {
        return format!(
            "{}:****:{}",
            ipv6_parts.first().unwrap_or(&""),
            ipv6_parts.last().unwrap_or(&"")
        );
    }

    mask_session_part(trimmed)
}

pub(super) fn normalize_session_group_name(value: &str) -> String {
    value.trim().to_string()
}

impl AxShell {
    /// Stop terminal and SFTP backends without blocking the GPUI event loop.
    pub(crate) fn shutdown_all_backends(&mut self) {
        for tab in &self.tabs {
            tab.shutdown_backend();
        }

        let group_ids = self.sftp_handles.keys().cloned().collect::<Vec<_>>();
        for group_id in group_ids {
            self.release_sftp_handle_for_group(&group_id, true);
        }
    }

    pub(crate) fn open_local(&mut self, cx: &mut Context<Self>) {
        let id = Uuid::new_v4().to_string();
        match local::spawn_local_terminal(
            id.clone(),
            DEFAULT_COLS,
            DEFAULT_ROWS,
            self.runtime_state.events_tx.clone(),
        ) {
            Ok(backend) => {
                let title = if cfg!(windows) { "PowerShell" } else { "Local" }.to_string();
                let mut tab = TerminalTab::new_local(
                    id.clone(),
                    title,
                    backend,
                    self.runtime_state.events_tx.clone(),
                );
                tab.resize(DEFAULT_COLS, DEFAULT_ROWS);
                self.tabs.push(tab);
                self.active_tab = Some(id.clone());
                self.pane_root = PaneLayout::Single(id.clone());
                self.focused_pane_path = vec![];
                let group_id = Uuid::new_v4().to_string();
                self.tab_groups.push(TabGroup {
                    id: group_id.clone(),
                    title: "Local".to_string(),
                    pane_root: PaneLayout::Single(id),
                    sftp: None,
                    sftp_page_open: false,
                });
                self.active_group = Some(group_id);
                self.status = "local terminal opened".into();
                self.set_workspace_page(WorkspacePage::Terminal, cx);
            }
            Err(err) => {
                self.status = format!("failed to open local terminal: {err:#}").into();
            }
        }
        cx.notify();
    }

    pub(crate) fn connect_ssh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        tracing::info!("[ui] user initiating new ssh connection from form");
        let session_name = self.session_name_input.read(cx).value().trim().to_string();
        let group_name = normalize_session_group_name(&self.session_group_input.read(cx).value());
        let host = self.host_input.read(cx).value().trim().to_string();
        let port = self
            .port_input
            .read(cx)
            .value()
            .trim()
            .parse::<u16>()
            .unwrap_or(22);
        let user = self.user_input.read(cx).value().trim().to_string();
        let password = self.password_input.read(cx).value().to_string();
        let key_path = self.key_path_input.read(cx).value().trim().to_string();
        let key_inline = self.key_inline_input.read(cx).value().to_string();
        let passphrase = self.passphrase_input.read(cx).value().to_string();

        if host.is_empty() || user.is_empty() {
            self.status = t!("host_and_user_required").into();
            cx.notify();
            return;
        }

        if self.ssh_proxy_type != "none" {
            let proxy_host = self.proxy_host_input.read(cx).value().trim().to_string();
            let proxy_port_str = self.proxy_port_input.read(cx).value().trim().to_string();
            let proxy_port = proxy_port_str.parse::<u16>().ok();
            if proxy_host.is_empty() || proxy_port.is_none() {
                self.status = "Proxy host and port are required".into();
                cx.notify();
                return;
            }
        }

        let name = if session_name.is_empty() {
            host.clone()
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
            .filter(|session| session.host == host && session.port == port && session.user == user)
            .and_then(|session| session.last_successful_ssh_mode);

        let mut session = match self.ssh_auth_method {
            AuthMethod::Password => Session::password(host, port, user, password),
            AuthMethod::Key => {
                if key_path.is_empty() && key_inline.trim().is_empty() {
                    self.status = "private key path or content is required".into();
                    cx.notify();
                    return;
                }
                Session::key(host, port, user, key_path, key_inline, passphrase)
            }
        };
        session.name = name;
        session.group_name = group_name;
        if let Some(id) = existing_id {
            session.id = id;
        }
        session.last_used = existing_last_used;
        session.last_successful_ssh_mode = existing_last_successful_ssh_mode;
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
        self.config.upsert(session.clone());
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save config: {err:#}");
        }

        self.open_ssh_session(session, cx);
        self.editing_session_id = None;
        self.active_dialog = None;
        self.set_workspace_page(WorkspacePage::Terminal, cx);
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

    pub(crate) fn reset_ssh_form(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editing_session_id = None;
        self.ssh_auth_method = AuthMethod::Password;
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
    }

    pub(crate) fn load_session_into_form(
        &mut self,
        session: &Session,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editing_session_id = Some(session.id.clone());
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
            std::path::PathBuf::from("/Applications/Utilities")
        } else if cfg!(target_os = "windows") {
            std::env::var("ProgramFiles")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| std::path::PathBuf::from("C:\\"))
        } else {
            std::path::PathBuf::from("/usr/bin")
        };
        let dialog = rfd::AsyncFileDialog::new().set_directory(start_dir);
        #[cfg(target_os = "macos")]
        let file_dialog = dialog.pick_folder();
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
        self.reset_ssh_form(window, cx);
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
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save terminal font size: {err:#}");
        }
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
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save UI font size: {err:#}");
        }
        Theme::global_mut(cx).font_size = px(self.appearance.ui_font_size);
        self.status = format!("UI font size: {:.0}px", self.appearance.ui_font_size).into();
        cx.notify();
    }

    pub(crate) fn change_ui_font_family(
        &mut self,
        family: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.appearance.ui_font_family = family.into();
        self.config.set_ui_font_family(family);
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save UI font family: {err:#}");
        }
        crate::app::theme::set_theme_font_names(
            Theme::global_mut(cx),
            &self.appearance.ui_font_family,
        );
        cx.notify();
        window.refresh();
    }

    pub(crate) fn change_terminal_font_family(&mut self, family: &str, cx: &mut Context<Self>) {
        self.appearance.terminal_font_family = family.into();
        self.appearance.terminal_font_metrics =
            crate::app::TerminalFontMetrics::fallback(self.appearance.terminal_font_size);
        self.config.set_terminal_font_family(family);
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save terminal font family: {err:#}");
        }
        cx.notify();
    }

    pub(crate) fn change_cursor_style(
        &mut self,
        style: crate::config::CursorStyle,
        cx: &mut Context<Self>,
    ) {
        self.appearance.cursor_style = style;
        self.config.set_cursor_style(style);
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save cursor style: {err:#}");
        }
        cx.notify();
    }

    pub(crate) fn reset_layout(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.config.set_layout_state(None, None, None);
        let _ = self.config.save();

        self.is_layout_reset = true;
        self.workspace_panels = cx.new(|_| crate::app::resizable::ResizableState::default());
        self.body_panels = cx.new(|_| crate::app::resizable::ResizableState::default());

        cx.notify();
    }

    pub(crate) fn set_ssh_auth_method(&mut self, method: AuthMethod, cx: &mut Context<Self>) {
        self.ssh_auth_method = method;
        cx.notify();
    }

    pub(crate) fn set_ssh_proxy_type(&mut self, proxy_type: String, cx: &mut Context<Self>) {
        self.ssh_proxy_type = proxy_type;
        cx.notify();
    }

    pub(crate) fn connect_saved_session(&mut self, session_id: String, cx: &mut Context<Self>) {
        tracing::info!(
            "[ui] user clicked to connect saved session '{}'",
            session_id
        );
        let Some(session) = self.config.get(&session_id).cloned() else {
            self.status = "saved session not found".into();
            cx.notify();
            return;
        };
        self.open_ssh_session(session, cx);
    }

    pub(crate) fn open_ssh_session(&mut self, session: Session, cx: &mut Context<Self>) {
        tracing::info!(
            "[session] opening ssh tab for session '{}' ({}@{})",
            session.name,
            session.user,
            session.host
        );
        self.expanded_saved_groups
            .insert(normalize_session_group_name(&session.group_name));
        let id = Uuid::new_v4().to_string();
        let backend = ssh::spawn_ssh_terminal(
            self.runtime_state.runtime.handle(),
            id.clone(),
            session.clone(),
            DEFAULT_COLS,
            DEFAULT_ROWS,
            self.runtime_state.events_tx.clone(),
        );
        self.tabs.push(TerminalTab::new_ssh(
            id.clone(),
            &session,
            backend,
            self.runtime_state.events_tx.clone(),
        ));
        self.active_tab = Some(id.clone());
        self.connection_progress = Some(crate::app::ConnectionProgress {
            tab_id: id.clone(),
            title: rust_i18n::t!("connecting").into(),
            lines: vec![rust_i18n::t!("starting_connection").into()],
            failed: false,
        });
        self.pane_root = PaneLayout::Single(id.clone());
        self.focused_pane_path = vec![];
        let group_id = Uuid::new_v4().to_string();
        self.tab_groups.push(TabGroup {
            id: group_id.clone(),
            title: session.name.clone(),
            pane_root: PaneLayout::Single(id.clone()),
            sftp: Some(crate::app::SftpUiState {
                current_path: "/".into(),
                status: rust_i18n::t!("sftp_connecting").to_string(),
                entries: Vec::new(),
                has_more_entries: false,
                loading_more_entries: false,
                reached_entries_limit: false,
                selected_path: None,
                preview: None,
                selected_entries: std::collections::HashSet::new(),
                home_dir: "/".into(),
            }),
            sftp_page_open: false,
        });
        self.active_group = Some(group_id.clone());
        self.ensure_active_workspace_tab_visible();
        if let Some(session_id) = self.active_session_id() {
            if let Some(index) = self
                .config
                .sessions()
                .iter()
                .position(|s| s.id == session_id)
            {
                self.saved_scroll_handle.scroll_to_item(index);
            }
        }
        self.active_tab = Some(id.clone());
        self.pending_sftp_path_sync = Some("/".into());
        self.status = "ssh tab opened".into();
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
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save config: {err:#}");
        }
        self.status = "session removed".into();
        cx.notify();
    }

    /// Retry a single disconnected tab by its ID.
    /// For SSH tabs: spawns a new SSH connection and restarts SFTP.
    /// For local tabs: spawns a new local shell.
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

        let is_ssh = self.tabs[ix].session.is_some();
        let session = self.tabs[ix].session.clone();
        let new_generation = self.tabs[ix].backend_generation + 1;
        let cols = self.tabs[ix].cols;
        let rows = self.tabs[ix].rows;

        // Close old backend (sends Close through the shared Arc<Mutex>)
        self.tabs[ix].send_backend(BackendCommand::Close);

        if let Some(session) = session {
            // SSH tab: spawn new SSH connection
            let backend = ssh::spawn_ssh_terminal(
                self.runtime_state.runtime.handle(),
                tab_id.to_string(),
                session.clone(),
                cols,
                rows,
                self.runtime_state.events_tx.clone(),
            );

            // Swap the backend — the Term's internal listener shares the
            // same Arc<Mutex<BackendTx>>, so user input is automatically
            // routed to the new backend. Terminal history is preserved.
            self.tabs[ix].set_backend(backend);
            self.tabs[ix].connected = false;
            self.tabs[ix].status = "connecting".into();
            self.tabs[ix].disconnected_reason = None;
            self.tabs[ix].backend_generation = new_generation;
            self.tabs[ix].backend_initialized = false;

            // Restart SFTP for the group containing this tab
            if let Some(group) = self
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
            match local::spawn_local_terminal(
                tab_id.to_string(),
                cols,
                rows,
                self.runtime_state.events_tx.clone(),
            ) {
                Ok(backend) => {
                    // Swap the backend — preserves terminal history.
                    self.tabs[ix].set_backend(backend);
                    self.tabs[ix].connected = true;
                    self.tabs[ix].status = "local shell".into();
                    self.tabs[ix].disconnected_reason = None;
                    self.tabs[ix].backend_generation = new_generation;
                    self.tabs[ix].backend_initialized = false;
                    // Resize the new PTY to match the pane dimensions.
                    self.tabs[ix].send_backend(BackendCommand::Resize { cols, rows });
                }
                Err(err) => {
                    self.status = format!("failed to reopen local terminal: {err:#}").into();
                    cx.notify();
                    return;
                }
            }
        }

        self.status = if is_ssh {
            "ssh tab retrying"
        } else {
            "local tab reopened"
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
                if let Some(index) = self
                    .config
                    .sessions()
                    .iter()
                    .position(|s| s.id == session_id)
                {
                    self.saved_scroll_handle.scroll_to_item(index);
                }
            }
            self.ensure_active_workspace_tab_visible();
        }
        self.focus_handle.focus(window, cx);
        self.sync_system_tab_to_active_group();
        cx.notify();
    }

    pub(crate) fn close_tab(&mut self, id: String, cx: &mut Context<Self>) {
        self.handle_tab_close(id);
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

    pub(crate) fn handle_tab_close(&mut self, id: String) {
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
            let tabs_in_group = group.pane_root.tab_ids();
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
                            .first()
                            .copied()
                            .map(String::from);
                    } else if pos + 1 < all_groups.len() {
                        next_active_id = all_groups[pos + 1]
                            .pane_root
                            .tab_ids()
                            .first()
                            .copied()
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

        if self.tabs.is_empty() || self.tab_groups.is_empty() {
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
                    .first()
                    .copied()
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
            if event.modifiers.platform {
                if let Some((row, col, _side)) = self.terminal_grid_point_and_side(event.position) {
                    if let Some(snapshot) = self.active_snapshot() {
                        if let Some((target, _)) =
                            crate::terminal::highlight::find_terminal_target_at_cell(
                                &snapshot.cells,
                                snapshot.rows,
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

    pub(crate) fn active_snapshot(&self) -> Option<RenderSnapshot> {
        self.active_tab
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))
            .map(TerminalTab::render_snapshot)
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
            .unwrap_or_else(|| t!("idle_no_session").into())
    }

    pub(crate) fn active_ssh_session(&self) -> Option<(String, Session)> {
        let active_id = self.active_tab.as_ref()?;
        let tab = self.tabs.iter().find(|tab| &tab.id == active_id)?;
        if !tab.connected {
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
    }
}
