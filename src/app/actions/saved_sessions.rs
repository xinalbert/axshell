use std::{collections::HashSet, fs, path::PathBuf};

use anyhow::{Context as _, Result, anyhow};
use gpui::{Context, Focusable as _, KeyDownEvent, Pixels, Point, Window};
use gpui_component::WindowExt as _;
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AxShell, SelectorEntry,
    app::{SavedGroupContextMenuState, SavedSessionContextMenuState},
    config::ConfigStore,
    session::Session,
};

use super::session::normalize_session_group_name;
use crate::diagnostics::{mask_host, mask_value};

const SAVED_SESSIONS_SHARE_FORMAT: &str = "ax-shell-saved-sessions";
const SAVED_SESSIONS_SHARE_SCHEMA_VERSION: u32 = 1;

fn default_share_ssh_port() -> u16 {
    22
}

fn default_share_auth_method() -> crate::session::AuthMethod {
    crate::session::AuthMethod::Password
}

fn default_share_proxy_type() -> String {
    "none".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedSessionsShareFile {
    format: String,
    schema_version: u32,
    exported_at: String,
    sessions: Vec<SavedSessionShareEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedSessionShareEntry {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    group_name: String,
    #[serde(default)]
    host: String,
    #[serde(default = "default_share_ssh_port")]
    port: u16,
    #[serde(default)]
    user: String,
    #[serde(default = "default_share_auth_method")]
    auth: crate::session::AuthMethod,
    #[serde(default = "default_share_proxy_type")]
    proxy_type: String,
    #[serde(default)]
    proxy_host: String,
    #[serde(default)]
    proxy_port: Option<u16>,
    #[serde(default)]
    proxy_user: String,
}

#[derive(Debug)]
struct ParsedSavedSessionsShare {
    sessions: Vec<Session>,
    skipped_invalid: usize,
}

#[derive(Debug)]
struct ImportSavedSessionsOutcome {
    sessions: Vec<Session>,
    imported: usize,
    skipped_duplicate: usize,
    skipped_invalid: usize,
    imported_group_names: Vec<String>,
}

impl ImportSavedSessionsOutcome {
    fn skipped_total(&self) -> usize {
        self.skipped_duplicate + self.skipped_invalid
    }
}

impl SavedSessionShareEntry {
    fn from_session(session: &Session) -> Self {
        Self {
            id: session.id.trim().to_string(),
            name: session.name.trim().to_string(),
            group_name: normalize_session_group_name(&session.group_name),
            host: session.host.trim().to_string(),
            port: session.port,
            user: session.user.trim().to_string(),
            auth: session.auth,
            proxy_type: normalize_share_proxy_type(&session.proxy_type),
            proxy_host: session.proxy_host.trim().to_string(),
            proxy_port: session.proxy_port,
            proxy_user: session.proxy_user.trim().to_string(),
        }
    }

    fn into_session(self) -> Option<Session> {
        let host = self.host.trim().to_string();
        let user = self.user.trim().to_string();
        if host.is_empty() || user.is_empty() {
            return None;
        }

        let port = if self.port == 0 { 22 } else { self.port };
        let mut session = match self.auth {
            crate::session::AuthMethod::Password => {
                Session::password(host, port, user, String::new())
            }
            crate::session::AuthMethod::Key => Session::key(
                host,
                port,
                user,
                String::new(),
                String::new(),
                String::new(),
            ),
        };
        session.id = self.id.trim().to_string();
        session.name = if self.name.trim().is_empty() {
            session.host.clone()
        } else {
            self.name.trim().to_string()
        };
        session.group_name = normalize_session_group_name(&self.group_name);

        session.proxy_type = normalize_share_proxy_type(&self.proxy_type);
        if session.proxy_type == "none" {
            session.proxy_host.clear();
            session.proxy_port = None;
            session.proxy_user.clear();
        } else {
            session.proxy_host = self.proxy_host.trim().to_string();
            session.proxy_port = self.proxy_port;
            session.proxy_user = self.proxy_user.trim().to_string();
        }

        Some(session)
    }
}

fn normalize_share_proxy_type(value: &str) -> String {
    match value.trim().to_ascii_lowercase().as_str() {
        "socks5" => "socks5".to_string(),
        "socks5h" => "socks5h".to_string(),
        "http" => "http".to_string(),
        _ => "none".to_string(),
    }
}

fn saved_sessions_share_file(sessions: &[Session]) -> SavedSessionsShareFile {
    SavedSessionsShareFile {
        format: SAVED_SESSIONS_SHARE_FORMAT.to_string(),
        schema_version: SAVED_SESSIONS_SHARE_SCHEMA_VERSION,
        exported_at: chrono::Utc::now().to_rfc3339(),
        sessions: sessions
            .iter()
            .map(SavedSessionShareEntry::from_session)
            .collect(),
    }
}

fn saved_sessions_share_json(sessions: &[Session]) -> Result<String> {
    serde_json::to_string_pretty(&saved_sessions_share_file(sessions))
        .context("serialize saved SSH sessions export")
}

fn saved_sessions_export_file_name(label: &str) -> String {
    let mut slug = String::new();
    for ch in label.trim().chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
            slug.push(ch);
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-');
    let slug = if slug.is_empty() { "saved-ssh" } else { slug };
    format!("ax-shell-{slug}.json")
}

fn parse_saved_sessions_share_json(content: &str) -> Result<ParsedSavedSessionsShare> {
    let share_file: SavedSessionsShareFile =
        serde_json::from_str(content).context("parse saved SSH sessions export")?;
    if share_file.format != SAVED_SESSIONS_SHARE_FORMAT {
        return Err(anyhow!("unsupported saved SSH sessions export format"));
    }
    if share_file.schema_version != SAVED_SESSIONS_SHARE_SCHEMA_VERSION {
        return Err(anyhow!(
            "unsupported saved SSH sessions export schema version {}",
            share_file.schema_version
        ));
    }

    let mut sessions = Vec::new();
    let mut skipped_invalid = 0;
    for entry in share_file.sessions {
        if let Some(session) = entry.into_session() {
            sessions.push(session);
        } else {
            skipped_invalid += 1;
        }
    }

    Ok(ParsedSavedSessionsShare {
        sessions,
        skipped_invalid,
    })
}

fn session_share_fingerprint(session: &Session) -> String {
    let auth = match session.auth {
        crate::session::AuthMethod::Password => "password",
        crate::session::AuthMethod::Key => "key",
    };
    [
        session.name.trim().to_string(),
        normalize_session_group_name(&session.group_name),
        session.host.trim().to_string(),
        session.port.to_string(),
        session.user.trim().to_string(),
        auth.to_string(),
        normalize_share_proxy_type(&session.proxy_type),
        session.proxy_host.trim().to_string(),
        session
            .proxy_port
            .map(|port| port.to_string())
            .unwrap_or_default(),
        session.proxy_user.trim().to_string(),
    ]
    .join("\0")
}

fn merge_imported_saved_sessions(
    existing: &[Session],
    parsed: ParsedSavedSessionsShare,
) -> ImportSavedSessionsOutcome {
    let mut sessions = existing.to_vec();
    let mut fingerprints = sessions
        .iter()
        .map(session_share_fingerprint)
        .collect::<HashSet<_>>();
    let mut used_ids = sessions
        .iter()
        .filter_map(|session| {
            let id = session.id.trim();
            (!id.is_empty()).then(|| id.to_string())
        })
        .collect::<HashSet<_>>();
    let mut imported = 0;
    let mut skipped_duplicate = 0;
    let mut imported_group_names = Vec::new();

    for mut session in parsed.sessions {
        let fingerprint = session_share_fingerprint(&session);
        if !fingerprints.insert(fingerprint) {
            skipped_duplicate += 1;
            continue;
        }

        if session.id.trim().is_empty() || used_ids.contains(session.id.trim()) {
            session.id = Uuid::new_v4().to_string();
        }
        used_ids.insert(session.id.clone());

        let group_name = normalize_session_group_name(&session.group_name);
        if !group_name.is_empty() && !imported_group_names.iter().any(|name| name == &group_name) {
            imported_group_names.push(group_name);
        }

        sessions.push(session);
        imported += 1;
    }

    ImportSavedSessionsOutcome {
        sessions,
        imported,
        skipped_duplicate,
        skipped_invalid: parsed.skipped_invalid,
        imported_group_names,
    }
}

fn saved_sessions_share_start_dir() -> PathBuf {
    ConfigStore::config_root_dir_path().unwrap_or_else(|_| PathBuf::from("."))
}

fn saved_sessions_for_group(sessions: &[Session], group_name: &str) -> Vec<Session> {
    let group_name = normalize_session_group_name(group_name);
    sessions
        .iter()
        .filter(|session| normalize_session_group_name(&session.group_name) == group_name)
        .cloned()
        .collect()
}

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
                self.selector_scroll_handle
                    .scroll_to_item(next - 2, gpui::ScrollStrategy::Nearest);
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
                self.open_local_and_focus(window, cx);
                window.close_dialog(cx);
            }
            SelectorEntry::NewSsh => {
                window.close_dialog(cx);
                self.open_new_ssh_dialog(window, cx);
            }
            SelectorEntry::Saved(session_id) => {
                self.connect_saved_session_and_focus(session_id, window, cx);
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

    pub(crate) fn saved_sidebar_visible_row_index_for_session(
        &self,
        session_id: &str,
    ) -> Option<usize> {
        let mut row_ix = 1; // Local Terminal row is always first.

        for (group_name, sessions) in self.saved_session_groups() {
            let group_row_ix = row_ix;
            row_ix += 1;

            let is_expanded = self.renaming_saved_group.as_deref() == Some(group_name.as_str())
                || self.expanded_saved_groups.contains(group_name.as_str());
            if !is_expanded {
                if sessions.iter().any(|session| session.id == session_id) {
                    return Some(group_row_ix);
                }
                continue;
            }

            for session in sessions {
                if session.id == session_id {
                    return Some(row_ix);
                }
                row_ix += 1;
            }
        }

        None
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

    pub(crate) fn export_saved_sessions_share_file(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.export_saved_sessions_share_file_with_sessions(
            self.config.sessions().to_vec(),
            "saved-ssh",
            window,
            cx,
        );
    }

    pub(crate) fn export_saved_session_share_file(
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
        let file_label = format!("{}-saved-ssh", session.name.trim());
        self.export_saved_sessions_share_file_with_sessions(vec![session], &file_label, window, cx);
    }

    pub(crate) fn export_saved_group_share_file(
        &mut self,
        group_name: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let sessions = saved_sessions_for_group(self.config.sessions(), &group_name);
        let display_name = Self::display_group_name(&group_name);
        let file_label = format!("{}-group-saved-ssh", display_name.trim());
        self.export_saved_sessions_share_file_with_sessions(sessions, &file_label, window, cx);
    }

    fn export_saved_sessions_share_file_with_sessions(
        &mut self,
        sessions: Vec<Session>,
        file_label: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if sessions.is_empty() {
            self.status = t!("saved_sessions_export_empty").into();
            cx.notify();
            return;
        }

        let content = match saved_sessions_share_json(&sessions) {
            Ok(content) => content,
            Err(err) => {
                self.status = format!("{}: {err:#}", t!("saved_sessions_export_failed")).into();
                cx.notify();
                return;
            }
        };
        let count = sessions.len();
        let file_dialog = rfd::AsyncFileDialog::new()
            .set_directory(saved_sessions_share_start_dir())
            .set_file_name(saved_sessions_export_file_name(file_label))
            .add_filter("AxShell Saved SSH", &["json"])
            .save_file();

        cx.spawn_in(window, async move |this, mut cx| {
            if let Some(file) = file_dialog.await {
                let path = file.path().to_path_buf();
                let write_result = fs::write(&path, content.as_bytes())
                    .with_context(|| format!("failed to write {}", path.display()))
                    .map(|_| path);

                let _ = gpui::AsyncWindowContext::update(&mut cx, |_, cx| {
                    let _ = this.update(cx, |this, cx| match write_result {
                        Ok(path) => {
                            tracing::info!(
                                component = "saved_sessions",
                                operation = "export_share",
                                count,
                                path = %crate::diagnostics::mask_path(path.to_string_lossy().as_ref()),
                                "Exported saved SSH sessions without credentials"
                            );
                            this.status =
                                t!("saved_sessions_exported", count = count).to_string().into();
                            cx.notify();
                        }
                        Err(err) => {
                            tracing::error!(
                                component = "saved_sessions",
                                operation = "export_share",
                                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                                "Failed to export saved SSH sessions"
                            );
                            this.status =
                                format!("{}: {err:#}", t!("saved_sessions_export_failed")).into();
                            cx.notify();
                        }
                    });
                });
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    pub(crate) fn import_saved_sessions_share_file(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let file_dialog = rfd::AsyncFileDialog::new()
            .set_directory(saved_sessions_share_start_dir())
            .add_filter("AxShell Saved SSH", &["json"])
            .pick_file();

        cx.spawn_in(window, async move |this, mut cx| {
            if let Some(file) = file_dialog.await {
                let source_path = file.path().to_path_buf();
                let import_result = fs::read_to_string(&source_path)
                    .with_context(|| format!("failed to read {}", source_path.display()))
                    .and_then(|content| parse_saved_sessions_share_json(&content));

                let _ =
                    gpui::AsyncWindowContext::update(&mut cx, |_, cx| {
                        let _ = this.update(cx, |this, cx| match import_result {
                        Ok(parsed) => {
                            let outcome =
                                merge_imported_saved_sessions(this.config.sessions(), parsed);
                            let imported = outcome.imported;
                            let skipped = outcome.skipped_total();
                            if imported > 0 {
                                for group_name in &outcome.imported_group_names {
                                    this.expanded_saved_groups.insert(group_name.clone());
                                }
                                this.config.replace_sessions(outcome.sessions);
                                this.config.save_logged("import_saved_sessions_share");
                            }

                            tracing::info!(
                                component = "saved_sessions",
                                operation = "import_share",
                                imported,
                                skipped,
                                "Imported saved SSH sessions without credentials"
                            );
                            this.status = if imported == 0 {
                                t!("saved_sessions_imported_none", skipped = skipped)
                                    .to_string()
                                    .into()
                            } else {
                                t!("saved_sessions_imported", count = imported, skipped = skipped)
                                    .to_string()
                                    .into()
                            };
                            cx.notify();
                        }
                        Err(err) => {
                            tracing::error!(
                                component = "saved_sessions",
                                operation = "import_share",
                                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                                "Failed to import saved SSH sessions"
                            );
                            this.status =
                                format!("{}: {err:#}", t!("saved_sessions_import_failed")).into();
                            cx.notify();
                        }
                    });
                    });
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    pub(crate) fn open_saved_session_context_menu(
        &mut self,
        session_id: String,
        connection_info: String,
        position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.sftp_context_menu = None;
        self.saved_group_context_menu = None;
        self.saved_session_context_menu = Some(SavedSessionContextMenuState {
            session_id,
            connection_info,
            position,
        });
        cx.notify();
    }

    pub(crate) fn open_saved_group_context_menu(
        &mut self,
        group_name: String,
        position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.sftp_context_menu = None;
        self.saved_session_context_menu = None;
        self.saved_group_context_menu = Some(SavedGroupContextMenuState {
            group_name,
            position,
        });
        cx.notify();
    }

    pub(crate) fn dismiss_saved_session_context_menu(&mut self, cx: &mut Context<Self>) {
        if self.saved_session_context_menu.take().is_some() {
            cx.notify();
        }
    }

    pub(crate) fn dismiss_saved_group_context_menu(&mut self, cx: &mut Context<Self>) {
        if self.saved_group_context_menu.take().is_some() {
            cx.notify();
        }
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
        self.config.save_logged("rename_session_group");
        self.renaming_saved_group = None;
        if self.expanded_saved_groups.remove(&old_group_name) {
            self.expanded_saved_groups.insert(new_group_name);
        }
        self.status = t!("group_renamed").into();
        cx.notify();
    }

    pub(crate) fn session_detail(&self, session: &Session) -> String {
        format!("{}@{}", mask_value(&session.user), mask_host(&session.host))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{AuthMethod, Session};

    fn sample_password_session(id: &str, name: &str, host: &str) -> Session {
        let mut session = Session::password(host.into(), 22, "root".into(), "secret".into());
        session.id = id.into();
        session.name = name.into();
        session.group_name = "Prod".into();
        session.proxy_type = "socks5".into();
        session.proxy_host = "proxy.internal".into();
        session.proxy_port = Some(1080);
        session.proxy_user = "proxy-user".into();
        session.proxy_password = "proxy-secret".into();
        session.last_used = Some("2026-07-12T00:00:00Z".into());
        session
    }

    #[test]
    fn saved_sessions_share_export_omits_credentials_and_key_material() {
        let mut session = Session::key(
            "example.com".into(),
            2200,
            "alice".into(),
            "/home/alice/.ssh/id_ed25519".into(),
            "PRIVATE KEY DATA".into(),
            "key-passphrase".into(),
        );
        session.id = "session-1".into();
        session.name = "Example".into();
        session.group_name = "Shared".into();
        session.password = "password-secret".into();
        session.proxy_password = "proxy-secret".into();

        let json = saved_sessions_share_json(&[session]).expect("share export serializes");

        assert!(json.contains("example.com"));
        assert!(json.contains("\"auth\": \"key\""));
        assert!(!json.contains("password-secret"));
        assert!(!json.contains("PRIVATE KEY DATA"));
        assert!(!json.contains("key-passphrase"));
        assert!(!json.contains("proxy-secret"));
        assert!(!json.contains("password"));
        assert!(!json.contains("private_key"));
        assert!(!json.contains("passphrase"));
        assert!(!json.contains("proxy_password"));
    }

    #[test]
    fn saved_sessions_share_import_merges_without_duplicate_or_id_collision() {
        let existing = sample_password_session("same-id", "Prod Shell", "prod.example.com");
        let duplicate = SavedSessionShareEntry::from_session(&existing);
        let mut different = SavedSessionShareEntry::from_session(&existing);
        different.host = "staging.example.com".into();
        different.name = "Staging Shell".into();
        different.id = "same-id".into();

        let parsed = ParsedSavedSessionsShare {
            sessions: vec![
                duplicate.into_session().expect("duplicate is valid"),
                different.into_session().expect("different is valid"),
            ],
            skipped_invalid: 0,
        };

        let outcome = merge_imported_saved_sessions(&[existing.clone()], parsed);

        assert_eq!(outcome.imported, 1);
        assert_eq!(outcome.skipped_duplicate, 1);
        assert_eq!(outcome.sessions.len(), 2);
        let imported = outcome
            .sessions
            .iter()
            .find(|session| session.host == "staging.example.com")
            .expect("different session imported");
        assert_ne!(imported.id, "same-id");
        assert_eq!(imported.group_name, "Prod");
        assert_eq!(imported.auth, AuthMethod::Password);
        assert!(imported.password.is_empty());
        assert!(imported.private_key_path.is_empty());
        assert!(imported.private_key_inline.is_empty());
        assert!(imported.passphrase.is_empty());
        assert!(imported.proxy_password.is_empty());
    }

    #[test]
    fn saved_sessions_share_parser_skips_entries_without_host_or_user() {
        let share = SavedSessionsShareFile {
            format: SAVED_SESSIONS_SHARE_FORMAT.into(),
            schema_version: SAVED_SESSIONS_SHARE_SCHEMA_VERSION,
            exported_at: "2026-07-12T00:00:00Z".into(),
            sessions: vec![
                SavedSessionShareEntry {
                    name: "Missing Host".into(),
                    host: String::new(),
                    user: "root".into(),
                    ..SavedSessionShareEntry::from_session(&sample_password_session(
                        "valid",
                        "Valid",
                        "valid.example.com",
                    ))
                },
                SavedSessionShareEntry::from_session(&sample_password_session(
                    "valid",
                    "Valid",
                    "valid.example.com",
                )),
            ],
        };
        let json = serde_json::to_string(&share).expect("share serializes");

        let parsed = parse_saved_sessions_share_json(&json).expect("share parses");

        assert_eq!(parsed.sessions.len(), 1);
        assert_eq!(parsed.skipped_invalid, 1);
    }

    #[test]
    fn saved_sessions_for_group_filters_by_normalized_group_name() {
        let prod = sample_password_session("prod", "Prod", "prod.example.com");
        let mut prod_with_spaces =
            sample_password_session("prod-2", "Prod 2", "prod-2.example.com");
        prod_with_spaces.group_name = " Prod ".into();
        let mut dev = sample_password_session("dev", "Dev", "dev.example.com");
        dev.group_name = "Dev".into();

        let sessions = saved_sessions_for_group(&[prod, prod_with_spaces, dev], "Prod");

        assert_eq!(sessions.len(), 2);
        assert!(
            sessions
                .iter()
                .all(|session| { normalize_session_group_name(&session.group_name) == "Prod" })
        );
    }
}
