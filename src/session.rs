use gpui::{
    App, AppContext as _, Context, Entity, KeyDownEvent, MouseButton,
    MouseDownEvent, SharedString, Window, px,
};
use gpui_component::{
    Theme, WindowExt as _,
    input::InputState,
};
use rust_i18n::t;
use uuid::Uuid;

use crate::{
    Ashell, ConnectionProgress, SelectorEntry,
    config::{AuthMethod, Session},
    local_terminal,
    sftp,
    ssh_terminal,
    terminal::{BackendCommand, RenderSnapshot, TabKind, TerminalTab},
    DEFAULT_COLS, DEFAULT_ROWS, SIDEBAR_WIDTH, TAB_BAR_HEIGHT, TERMINAL_PADDING_X,
    TERMINAL_PADDING_Y,
};

impl Ashell {
    pub(crate) fn open_local(&mut self, cx: &mut Context<Self>) {
        let id = Uuid::new_v4().to_string();
        match local_terminal::spawn_local_terminal(
            id.clone(),
            DEFAULT_COLS,
            DEFAULT_ROWS,
            self.events_tx.clone(),
        ) {
            Ok(backend) => {
                let title = if cfg!(windows) { "PowerShell" } else { "Local" }.to_string();
                let mut tab =
                    TerminalTab::new_local(id.clone(), title, backend, self.events_tx.clone());
                tab.resize(DEFAULT_COLS, DEFAULT_ROWS);
                self.tabs.push(tab);
                self.active_tab = Some(id);
                self.tabs_scroll_handle.scroll_to_item(self.tabs.len() - 1);
                self.status = "local terminal opened".into();
            }
            Err(err) => {
                self.status = format!("failed to open local terminal: {err:#}").into();
            }
        }
        cx.notify();
    }

    pub(crate) fn connect_ssh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let session_name = self.session_name_input.read(cx).value().trim().to_string();
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

        if host.is_empty() || user.is_empty() {
            self.status = t!("host_and_user_required").into();
            cx.notify();
            return;
        }

        let name = if session_name.is_empty() {
            host.clone()
        } else {
            session_name
        };
        let existing_id = self.editing_session_id.clone();
        let existing_last_used = existing_id
            .as_deref()
            .and_then(|id| self.config.get(id))
            .and_then(|session| session.last_used.clone());

        let mut session = match self.ssh_auth_method {
            AuthMethod::Password => Session::password(host, port, user, password),
            AuthMethod::Key => {
                if key_path.is_empty() && key_inline.trim().is_empty() {
                    self.status = "private key path or content is required".into();
                    cx.notify();
                    return;
                }
                Session::key(host, port, user, key_path, key_inline)
            }
        };
        session.name = name;
        if let Some(id) = existing_id {
            session.id = id;
        }
        session.last_used = existing_last_used;
        self.config.upsert(session.clone());
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save config: {err:#}");
        }

        self.open_ssh_session(session, cx);
        self.editing_session_id = None;
        window.close_dialog(cx);
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
        Self::set_input_value(&self.host_input, "", window, cx);
        Self::set_input_value(&self.port_input, "22", window, cx);
        Self::set_input_value(&self.user_input, "root", window, cx);
        Self::set_input_value(&self.password_input, "", window, cx);
        Self::set_input_value(&self.key_path_input, "", window, cx);
        Self::set_input_value(&self.key_inline_input, "", window, cx);
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

    pub(crate) fn terminal_cell_width(&self) -> f32 {
        (self.terminal_font_size * 0.646).max(6.0)
    }

    pub(crate) fn terminal_line_height(&self) -> f32 {
        (self.terminal_font_size * 1.385).max(self.terminal_font_size + 2.0)
    }

    pub(crate) fn change_terminal_font_size(&mut self, delta: f32, cx: &mut Context<Self>) {
        self.terminal_font_size = (self.terminal_font_size + delta).clamp(10.0, 24.0);
        self.config.set_terminal_font_size(self.terminal_font_size);
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save terminal font size: {err:#}");
        }
        self.status = format!("terminal font size: {:.0}px", self.terminal_font_size).into();
        cx.notify();
    }

    pub(crate) fn change_ui_font_size(&mut self, delta: f32, cx: &mut Context<Self>) {
        self.ui_font_size = (self.ui_font_size + delta).clamp(8.0, 24.0);
        self.config.set_ui_font_size(self.ui_font_size);
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save UI font size: {err:#}");
        }
        Theme::global_mut(cx).font_size = px(self.ui_font_size);
        self.status = format!("UI font size: {:.0}px", self.ui_font_size).into();
        cx.notify();
    }

    pub(crate) fn change_ui_font_family(
        &mut self,
        family: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.ui_font_family = family.into();
        self.config.set_ui_font_family(family);
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save UI font family: {err:#}");
        }
        crate::theme::set_theme_font_names(Theme::global_mut(cx), &self.ui_font_family);
        cx.notify();
        window.refresh();
    }

    pub(crate) fn change_terminal_font_family(&mut self, family: &str, cx: &mut Context<Self>) {
        self.terminal_font_family = family.into();
        self.config.set_terminal_font_family(family);
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save terminal font family: {err:#}");
        }
        cx.notify();
    }

    pub(crate) fn reset_layout(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.config.set_layout_state(None, None, None);
        let _ = self.config.save();

        self.workspace_panels = cx.new(|_| gpui_component::resizable::ResizableState::default());
        self.body_panels = cx.new(|_| gpui_component::resizable::ResizableState::default());

        self.status = t!("reset_layout_success").into();
        cx.notify();
    }

    pub(crate) fn set_ssh_auth_method(&mut self, method: AuthMethod, cx: &mut Context<Self>) {
        self.ssh_auth_method = method;
        cx.notify();
    }

    pub(crate) fn connect_saved_session(&mut self, session_id: String, cx: &mut Context<Self>) {
        let Some(session) = self.config.get(&session_id).cloned() else {
            self.status = "saved session not found".into();
            cx.notify();
            return;
        };
        self.open_ssh_session(session, cx);
    }

    pub(crate) fn selector_entries(&self) -> Vec<SelectorEntry> {
        let mut entries = vec![SelectorEntry::Local, SelectorEntry::NewSsh];
        entries.extend(
            self.config
                .sessions()
                .iter()
                .map(|session| SelectorEntry::Saved(session.id.clone())),
        );
        entries
    }

    pub(crate) fn default_selector_index(&self) -> usize {
        if self.config.sessions().is_empty() {
            0
        } else {
            2
        }
    }

    pub(crate) fn move_selector_selection(&mut self, delta: i32, cx: &mut Context<Self>) {
        let entries = self.selector_entries();
        if entries.is_empty() {
            return;
        }
        let current = self.selector_selection.min(entries.len().saturating_sub(1)) as i32;
        let next = (current + delta).clamp(0, entries.len() as i32 - 1) as usize;
        if next != self.selector_selection {
            self.selector_selection = next;
            if next >= 2 {
                self.selector_scroll_handle.scroll_to_item(next - 2);
            }
            cx.notify();
        }
    }

    pub(crate) fn activate_selector_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let entries = self.selector_entries();
        let Some(entry) = entries.get(self.selector_selection).cloned() else {
            return;
        };

        match entry {
            SelectorEntry::Local => {
                self.open_local(cx);
                window.close_dialog(cx);
            }
            SelectorEntry::NewSsh => {
                window.close_dialog(cx);
                self.open_new_ssh_dialog(window, cx);
            }
            SelectorEntry::Saved(session_id) => {
                self.connect_saved_session(session_id, cx);
                window.close_dialog(cx);
            }
        }
    }

    pub(crate) fn on_selector_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = event.keystroke.key.to_ascii_lowercase();
        match key.as_str() {
            "up" | "arrowup" => {
                self.move_selector_selection(-1, cx);
                window.prevent_default();
                cx.stop_propagation();
            }
            "down" | "arrowdown" => {
                self.move_selector_selection(1, cx);
                window.prevent_default();
                cx.stop_propagation();
            }
            "enter" | "return" => {
                self.activate_selector_selection(window, cx);
                window.prevent_default();
                cx.stop_propagation();
            }
            _ => {}
        }
    }

    pub(crate) fn open_ssh_session(&mut self, session: Session, cx: &mut Context<Self>) {
        let id = Uuid::new_v4().to_string();
        let backend = ssh_terminal::spawn_ssh_terminal(
            self.runtime.handle(),
            id.clone(),
            session.clone(),
            DEFAULT_COLS,
            DEFAULT_ROWS,
            self.events_tx.clone(),
        );
        self.tabs.push(TerminalTab::new_ssh(
            id.clone(),
            &session,
            backend,
            self.events_tx.clone(),
        ));
        self.active_tab = Some(id.clone());
        self.tabs_scroll_handle.scroll_to_item(self.tabs.len() - 1);
        if let Some(session_id) = self.active_session_id() {
            if let Some(index) = self.config.sessions().iter().position(|s| s.id == session_id) {
                self.saved_scroll_handle.scroll_to_item(index);
            }
        }
        cx.notify();
        let sftp_handle = sftp::spawn_sftp(
            self.runtime.handle(),
            id.clone(),
            session,
            self.events_tx.clone(),
        );
        self.sftp_handles.insert(id.clone(), sftp_handle);
        self.active_tab = Some(id.clone());
        self.pending_sftp_path_sync = Some("/".into());
        self.connection_progress = Some(ConnectionProgress {
            tab_id: id,
            title: t!("connecting").into(),
            lines: vec![t!("starting_connection").into()],
            failed: false,
        });
        self.status = "ssh tab opened".into();
        cx.notify();
    }

    pub(crate) fn remove_saved_session(&mut self, session_id: String, cx: &mut Context<Self>) {
        self.config.remove(&session_id);
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save config: {err:#}");
        }
        self.status = "session removed".into();
        cx.notify();
    }

    pub(crate) fn retry_connection_progress(&mut self, cx: &mut Context<Self>) {
        let Some(progress) = self.connection_progress.clone() else {
            return;
        };
        let Some(ix) = self.tabs.iter().position(|tab| tab.id == progress.tab_id) else {
            self.connection_progress = None;
            cx.notify();
            return;
        };
        let Some(session) = self.tabs[ix].session.clone() else {
            self.connection_progress = None;
            cx.notify();
            return;
        };

        self.tabs[ix].backend.send(BackendCommand::Close);
        if let Some(handle) = self.sftp_handles.remove(&progress.tab_id) {
            handle.close();
        }
        self.tabs.remove(ix);
        if self.active_tab.as_deref() == Some(progress.tab_id.as_str()) {
            self.active_tab = self
                .tabs
                .get(ix)
                .or_else(|| self.tabs.get(ix.saturating_sub(1)))
                .map(|tab| tab.id.clone());
        }
        self.connection_progress = None;
        self.open_ssh_session(session, cx);
    }

    pub(crate) fn cancel_connection_progress(&mut self, cx: &mut Context<Self>) {
        let Some(progress) = self.connection_progress.clone() else {
            return;
        };
        self.connection_progress = None;
        self.close_tab(progress.tab_id, cx);
    }

    pub(crate) fn activate_tab(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(path) = self
            .tabs
            .iter()
            .find(|tab| tab.id == id)
            .and_then(|tab| tab.sftp.as_ref())
            .map(|sftp| sftp.current_path.clone())
        {
            self.pending_sftp_path_sync = Some(path);
        }
        self.active_tab = Some(id.clone());
        self.cpu_history.clear();
        self.net_rx_history.clear();
        self.net_tx_history.clear();
        if let Some(index) = self.tabs.iter().position(|t| t.id == id) {
            self.tabs_scroll_handle.scroll_to_item(index);
        }
        if self.tabs.iter().any(|t| t.id == id) {
            if let Some(session_id) = self.active_session_id() {
                if let Some(index) = self.config.sessions().iter().position(|s| s.id == session_id) {
                    self.saved_scroll_handle.scroll_to_item(index);
                }
            }
        }
        self.remote_sample_in_flight = false;
        self.request_active_system_snapshot();
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    pub(crate) fn close_tab(&mut self, id: String, cx: &mut Context<Self>) {
        if let Some(ix) = self.tabs.iter().position(|tab| tab.id == id) {
            let was_active = self.active_tab.as_deref() == Some(id.as_str());
            self.tabs[ix].backend.send(BackendCommand::Close);
            if let Some(handle) = self.sftp_handles.remove(&id) {
                handle.close();
            }
            self.tabs.remove(ix);
            if was_active
                || self
                    .active_tab
                    .as_ref()
                    .is_some_and(|active_id| !self.tabs.iter().any(|tab| &tab.id == active_id))
            {
                self.active_tab = self
                    .tabs
                    .get(ix)
                    .or_else(|| self.tabs.get(ix.saturating_sub(1)))
                    .map(|tab| tab.id.clone());
                self.cpu_history.clear();
                self.net_rx_history.clear();
                self.net_tx_history.clear();
                self.remote_sample_in_flight = false;
                self.request_active_system_snapshot();
            }
            cx.notify();
        }
    }

    pub(crate) fn focus_terminal(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus_handle.focus(window, cx);
        if event.button == MouseButton::Left {
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

    pub(crate) fn session_detail(&self, session: &Session) -> String {
        format!("{}@{}:{}", session.user, session.host, session.port)
    }

    pub(crate) fn sync_terminal_size(&mut self, window: &Window, cx: &App) {
        let viewport = window.viewport_size();
        let sidebar_width = self
            .workspace_panels
            .read(cx)
            .sizes()
            .first()
            .map(|size| size.as_f32())
            .unwrap_or(SIDEBAR_WIDTH);
        let terminal_height = self
            .body_panels
            .read(cx)
            .sizes()
            .first()
            .map(|size| size.as_f32())
            .unwrap_or(viewport.height.as_f32() - TAB_BAR_HEIGHT - 248.0);
        let width = (viewport.width.as_f32() - sidebar_width - TERMINAL_PADDING_X - 8.0)
            .max(self.terminal_cell_width());
        let height = (terminal_height - TERMINAL_PADDING_Y).max(self.terminal_line_height());
        let cols = (width / self.terminal_cell_width()).floor().max(1.0) as u16;
        let rows = (height / self.terminal_line_height()).floor().max(1.0) as u16;

        for tab in &mut self.tabs {
            tab.resize(cols, rows);
        }
    }
}
