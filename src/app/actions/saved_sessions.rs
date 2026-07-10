use gpui::{Context, Focusable as _, KeyDownEvent, Window};
use gpui_component::WindowExt as _;
use rust_i18n::t;

use crate::{AxShell, SelectorEntry, session::Session};

use super::session::{mask_session_host, mask_session_part, normalize_session_group_name};

impl AxShell {
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

    pub(crate) fn activate_selector_selection(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entries = self.selector_entries();
        let Some(entry) = entries.get(self.selector_selection).cloned() else {
            return;
        };

        self.active_dialog = None;
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
        cx.notify();
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

    pub(crate) fn saved_session_groups(&self) -> Vec<(String, Vec<Session>)> {
        let mut groups: Vec<(String, Vec<Session>)> = Vec::new();
        for session in self.config.sessions().iter().cloned() {
            let group_name = normalize_session_group_name(&session.group_name);
            if let Some((_, entries)) = groups
                .iter_mut()
                .find(|(existing_group_name, _)| *existing_group_name == group_name)
            {
                entries.push(session);
            } else {
                groups.push((group_name, vec![session]));
            }
        }
        groups
    }

    pub(crate) fn saved_group_names(&self) -> Vec<String> {
        let mut group_names = Vec::new();
        for session in self.config.sessions() {
            let group_name = normalize_session_group_name(&session.group_name);
            if group_name.is_empty() || group_names.iter().any(|name| name == &group_name) {
                continue;
            }
            group_names.push(group_name);
        }
        group_names
    }

    pub(crate) fn display_group_name(group_name: &str) -> String {
        if group_name.trim().is_empty() {
            t!("ungrouped_group").to_string()
        } else {
            group_name.trim().to_string()
        }
    }

    pub(crate) fn toggle_saved_group(&mut self, group_name: String, cx: &mut Context<Self>) {
        if !self.expanded_saved_groups.insert(group_name.clone()) {
            self.expanded_saved_groups.remove(&group_name);
        }
        cx.notify();
    }

    pub(crate) fn begin_saved_group_rename(
        &mut self,
        group_name: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let group_name = normalize_session_group_name(&group_name);
        if group_name.is_empty() {
            return;
        }
        self.renaming_saved_group = Some(group_name.clone());
        self.expanded_saved_groups.insert(group_name.clone());
        Self::set_input_value(&self.saved_group_name_input, group_name, window, cx);
        let input = self.saved_group_name_input.clone();
        window.defer(cx, move |window, cx| {
            window.focus(&input.read(cx).focus_handle(cx), cx);
        });
        cx.notify();
    }

    pub(crate) fn cancel_saved_group_rename(&mut self, cx: &mut Context<Self>) {
        self.renaming_saved_group = None;
        cx.notify();
    }

    pub(crate) fn commit_saved_group_rename(&mut self, cx: &mut Context<Self>) {
        let Some(old_group_name) = self.renaming_saved_group.clone() else {
            return;
        };
        let new_group_name =
            normalize_session_group_name(&self.saved_group_name_input.read(cx).value());
        if new_group_name.is_empty() {
            self.cancel_saved_group_rename(cx);
            return;
        }
        if new_group_name == old_group_name {
            self.cancel_saved_group_rename(cx);
            return;
        }

        let mut sessions = self.config.sessions().to_vec();
        let mut changed = false;
        for session in sessions.iter_mut() {
            if normalize_session_group_name(&session.group_name) == old_group_name {
                session.group_name = new_group_name.clone();
                changed = true;
            }
        }
        if !changed {
            self.cancel_saved_group_rename(cx);
            return;
        }

        self.config.replace_sessions(sessions);
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save config: {err:#}");
        }
        self.renaming_saved_group = None;
        if self.expanded_saved_groups.remove(&old_group_name) {
            self.expanded_saved_groups.insert(new_group_name);
        }
        self.status = t!("group_renamed").into();
        cx.notify();
    }

    pub(crate) fn session_detail(&self, session: &Session) -> String {
        format!(
            "{}@{}",
            mask_session_part(&session.user),
            mask_session_host(&session.host)
        )
    }

    pub(crate) fn session_connection_info(&self, session: &Session) -> String {
        format!(
            "{}\n{}@{}:{}\nssh {}@{} -p {}",
            session.name,
            session.user,
            session.host,
            session.port,
            session.user,
            session.host,
            session.port
        )
    }
}
