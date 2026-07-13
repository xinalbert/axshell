use std::{
    fs,
    path::{Component, Path, PathBuf},
    time::UNIX_EPOCH,
};

use directories::BaseDirs;
use gpui::{Context, Focusable as _, PathPromptOptions, Pixels, Point, Window};

use crate::{
    AxShell, SftpContextMenuState,
    app::{LocalFileEntry, SftpContextMenuTarget, WorkspacePage},
    sftp::{RemoteEntry, SftpHandle},
};

pub(crate) fn is_editable_text_file(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    let ext = std::path::Path::new(&lower)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let known_exts = [
        "txt", "conf", "json", "yaml", "yml", "xml", "ini", "sh", "py", "rs", "js", "ts", "html",
        "css", "md", "toml", "csv", "log", "cfg",
    ];
    if known_exts.contains(&ext) {
        return true;
    }
    let known_names = ["dockerfile", "makefile", ".gitignore", ".env"];
    if known_names.contains(&lower.as_str()) {
        return true;
    }
    false
}

impl AxShell {
    const SFTP_IDLE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(300);

    pub(crate) fn default_local_browser_dir() -> String {
        BaseDirs::new()
            .map(|dirs| dirs.home_dir().to_string_lossy().to_string())
            .or_else(|| {
                std::env::current_dir()
                    .ok()
                    .map(|path| path.to_string_lossy().to_string())
            })
            .unwrap_or_else(|| ".".to_string())
    }

    fn expand_local_browser_path(value: &str) -> PathBuf {
        if value == "~" {
            return BaseDirs::new()
                .map(|dirs| dirs.home_dir().to_path_buf())
                .unwrap_or_else(|| PathBuf::from(value));
        }
        if let Some(rest) = value
            .strip_prefix("~/")
            .or_else(|| value.strip_prefix("~\\"))
        {
            return BaseDirs::new()
                .map(|dirs| dirs.home_dir().join(rest))
                .unwrap_or_else(|| PathBuf::from(value));
        }
        PathBuf::from(value)
    }

    fn normalize_local_browser_path(path: PathBuf) -> PathBuf {
        let anchored = path.is_absolute();
        let mut normalized = PathBuf::new();
        for component in path.components() {
            match component {
                Component::CurDir => {}
                Component::ParentDir => {
                    if !normalized.pop() && !anchored {
                        normalized.push(component.as_os_str());
                    }
                }
                _ => normalized.push(component.as_os_str()),
            }
        }
        if normalized.as_os_str().is_empty() {
            PathBuf::from(Path::new(std::path::MAIN_SEPARATOR_STR))
        } else {
            normalized
        }
    }

    fn resolve_local_browser_path(current_path: &str, value: &str) -> PathBuf {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return PathBuf::from(current_path);
        }
        let candidate = Self::expand_local_browser_path(trimmed);
        if candidate.is_absolute() {
            return Self::normalize_local_browser_path(candidate);
        }
        Self::normalize_local_browser_path(PathBuf::from(current_path).join(candidate))
    }

    pub(crate) fn local_browser_parent_path(path: &str) -> String {
        Path::new(path)
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map(|parent| parent.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string())
    }

    pub(crate) fn read_local_browser_entries(path: &str) -> Result<Vec<LocalFileEntry>, String> {
        let read_dir = fs::read_dir(path).map_err(|err| format!("local read_dir failed: {err}"))?;
        let mut entries = Vec::new();
        for entry in read_dir {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    tracing::warn!(
                        component = "local_browser",
                        operation = "read_entry",
                        local_path = %crate::diagnostics::mask_path(path),
                        error = %crate::diagnostics::sanitize_error(&err.to_string()),
                        "Skipped unreadable local entry"
                    );
                    continue;
                }
            };
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(err) => {
                    tracing::warn!(
                        component = "local_browser",
                        operation = "read_metadata",
                        local_path = %crate::diagnostics::mask_path(path),
                        error = %crate::diagnostics::sanitize_error(&err.to_string()),
                        "Skipped local entry with unreadable metadata"
                    );
                    continue;
                }
            };
            let name = entry.file_name().to_string_lossy().to_string();
            if name == "." || name == ".." {
                continue;
            }
            let modified = metadata
                .modified()
                .ok()
                .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs().min(u32::MAX as u64) as u32)
                .unwrap_or(0);
            let is_dir = metadata.is_dir();
            entries.push(LocalFileEntry {
                name,
                full_path: entry.path().to_string_lossy().to_string(),
                is_dir,
                size: if is_dir { 0 } else { metadata.len() },
                modified,
            });
        }
        entries.sort_by(|left, right| {
            right
                .is_dir
                .cmp(&left.is_dir)
                .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
        });
        Ok(entries)
    }

    pub(crate) fn active_sftp(&self) -> Option<&crate::app::SftpUiState> {
        self.active_group
            .as_ref()
            .and_then(|id| self.tab_groups.iter().find(|g| &g.id == id))
            .and_then(|g| g.sftp.as_ref())
    }

    pub(crate) fn active_sftp_mut(&mut self) -> Option<&mut crate::app::SftpUiState> {
        let active_id = self.active_group.clone()?;
        self.tab_groups
            .iter_mut()
            .find(|g| g.id == active_id)
            .and_then(|g| g.sftp.as_mut())
    }

    pub(crate) fn active_sftp_should_sync_shell_dir_on_entry(&self) -> bool {
        self.active_sftp().is_some_and(|sftp| {
            sftp.current_path == "/"
                && sftp.entries.is_empty()
                && sftp.selected_path.is_none()
                && sftp.selected_entries.is_empty()
        })
    }

    fn session_for_sftp_group(&self, group_id: &str) -> Option<crate::session::Session> {
        let group = self
            .tab_groups
            .iter()
            .find(|group| group.id == group_id && group.sftp.is_some())?;
        if let Some(active_tab_id) = self.active_tab.as_ref()
            && group.pane_root.contains(active_tab_id)
            && let Some(session) = self
                .tabs
                .iter()
                .find(|tab| tab.id == *active_tab_id)
                .and_then(|tab| tab.session.clone())
        {
            return Some(session);
        }
        group.pane_root.tab_ids().into_iter().find_map(|tab_id| {
            self.tabs
                .iter()
                .find(|tab| tab.id == tab_id)
                .and_then(|tab| tab.session.clone())
        })
    }

    fn active_sftp_saved_session_id(&self) -> Option<String> {
        let group_id = self.active_group.as_deref()?;
        let session = self.session_for_sftp_group(group_id)?;
        self.config.get(&session.id).map(|_| session.id)
    }

    pub(crate) fn ensure_sftp_handle_for_group(&mut self, group_id: &str) -> Option<SftpHandle> {
        if self
            .tab_groups
            .iter()
            .find(|group| group.id == group_id)
            .is_none_or(|group| group.sftp.is_none())
        {
            return None;
        }
        if let Some(handle) = self.sftp_handles.get(group_id)
            && !handle.commands_closed()
        {
            return Some(handle.clone());
        }
        if let Some(handle) = self.sftp_handles.remove(group_id) {
            handle.close();
        }

        let session = self.session_for_sftp_group(group_id)?;
        let restore_path = self
            .tab_groups
            .iter()
            .find(|group| group.id == group_id)
            .and_then(|group| group.sftp.as_ref())
            .filter(|sftp| !sftp.current_path.is_empty())
            .filter(|sftp| sftp.current_path != "/" || sftp.home_dir != "/")
            .map(|sftp| sftp.current_path.clone());
        let handle = crate::sftp::spawn_sftp(
            self.runtime_state.runtime.handle(),
            group_id.to_string(),
            session,
            self.runtime_state.events_tx.clone(),
        );
        self.sftp_handles
            .insert(group_id.to_string(), handle.clone());
        self.mark_sftp_activity_for_group(group_id);
        if let Some(path) = restore_path
            && !path.is_empty()
        {
            handle.list_dir(path);
        }
        if let Some(group) = self
            .tab_groups
            .iter_mut()
            .find(|group| group.id == group_id)
            && let Some(sftp) = group.sftp.as_mut()
        {
            sftp.status = rust_i18n::t!("sftp_connecting").to_string();
            sftp.has_more_entries = false;
            sftp.loading_more_entries = false;
            sftp.reached_entries_limit = false;
        }
        Some(handle)
    }

    pub(crate) fn ensure_active_sftp_handle(&mut self) -> Option<SftpHandle> {
        let active_group_id = self.active_group.clone()?;
        self.ensure_sftp_handle_for_group(&active_group_id)
    }

    pub(crate) fn restart_sftp_handle_for_group(&mut self, group_id: &str) -> Option<SftpHandle> {
        let should_restart = self.sftp_handles.contains_key(group_id)
            || self
                .tab_groups
                .iter()
                .find(|group| group.id == group_id)
                .is_some_and(|group| group.sftp_page_open);
        if !should_restart {
            return None;
        }
        self.release_sftp_handle_for_group(group_id, true);
        self.ensure_sftp_handle_for_group(group_id)
    }

    pub(crate) fn mark_sftp_activity_for_group(&mut self, group_id: &str) {
        self.sftp_last_activity
            .insert(group_id.to_string(), std::time::Instant::now());
    }

    pub(crate) fn mark_active_sftp_activity(&mut self) {
        if let Some(group_id) = self.active_group.clone() {
            self.mark_sftp_activity_for_group(&group_id);
        }
    }

    pub(crate) fn group_has_active_sftp_transfer(&self, group_id: &str) -> bool {
        self.transfers.iter().any(|transfer| {
            transfer.tab_id == group_id
                && matches!(
                    transfer.state,
                    crate::sftp::TransferState::Running | crate::sftp::TransferState::Paused
                )
        })
    }

    pub(crate) fn release_sftp_handle_for_group(
        &mut self,
        group_id: &str,
        cancel_active_transfers: bool,
    ) {
        if cancel_active_transfers {
            let transfer_ids = self
                .transfers
                .iter()
                .filter(|transfer| {
                    transfer.tab_id == group_id
                        && matches!(
                            transfer.state,
                            crate::sftp::TransferState::Running
                                | crate::sftp::TransferState::Paused
                        )
                })
                .map(|transfer| transfer.info.id.clone())
                .collect::<Vec<_>>();
            if let Some(handle) = self.sftp_handles.get(group_id) {
                for transfer_id in &transfer_ids {
                    handle.cancel_transfer(transfer_id.clone());
                }
            }
            if !transfer_ids.is_empty() {
                for transfer in &mut self.transfers {
                    if transfer.tab_id == group_id
                        && matches!(
                            transfer.state,
                            crate::sftp::TransferState::Running
                                | crate::sftp::TransferState::Paused
                        )
                    {
                        transfer.state = crate::sftp::TransferState::Interrupted(
                            "SFTP connection closed".to_string(),
                        );
                    }
                }
                self.config.set_transfers(self.transfers.clone());
            }
        }

        if let Some(handle) = self.sftp_handles.remove(group_id) {
            handle.close();
        }
        self.sftp_last_activity.remove(group_id);
    }

    pub(crate) fn sweep_idle_sftp_connections(&mut self) -> bool {
        let now = std::time::Instant::now();
        let active_group = self.active_group.clone();
        let deep_sleep =
            self.lifecycle.state() == crate::app::state::lifecycle::WindowLifecycleState::DeepSleep;
        let mut closed_any = false;
        let group_ids: Vec<String> = self.sftp_handles.keys().cloned().collect();
        for group_id in group_ids {
            let is_active_group = active_group.as_deref() == Some(group_id.as_str());
            if (deep_sleep && is_active_group)
                || (!deep_sleep && is_active_group && self.active_group_sftp_page_open())
            {
                continue;
            }
            let active_work_pins = self
                .sftp_handles
                .get(&group_id)
                .map(SftpHandle::active_work_pins)
                .unwrap_or_default();
            if active_work_pins > 0 {
                self.mark_sftp_activity_for_group(&group_id);
                continue;
            }

            let last_activity = self.sftp_last_activity.get(&group_id).copied();
            if !should_reclaim_sftp_worker(
                deep_sleep,
                active_work_pins,
                last_activity,
                now,
                Self::SFTP_IDLE_TIMEOUT,
            ) {
                if last_activity.is_none() {
                    self.mark_sftp_activity_for_group(&group_id);
                }
                continue;
            }

            self.release_sftp_handle_for_group(&group_id, false);
            closed_any = true;
            if let Some(group) = self
                .tab_groups
                .iter_mut()
                .find(|group| group.id == group_id)
                && let Some(sftp) = group.sftp.as_mut()
            {
                sftp.status = if deep_sleep {
                    "sftp deep sleep, reconnecting on next use".to_string()
                } else {
                    "sftp idle, reconnecting on next use".to_string()
                };
            }
        }
        closed_any
    }

    fn transfer_belongs_to_sftp_tab(
        transfer: &crate::sftp::Transfer,
        tab: crate::app::SftpTransferTab,
    ) -> bool {
        match tab {
            crate::app::SftpTransferTab::Active => matches!(
                transfer.state,
                crate::sftp::TransferState::Running | crate::sftp::TransferState::Paused
            ),
            crate::app::SftpTransferTab::Failed => matches!(
                transfer.state,
                crate::sftp::TransferState::Failed(_)
                    | crate::sftp::TransferState::Interrupted(_)
                    | crate::sftp::TransferState::Zombie(_)
            ),
            crate::app::SftpTransferTab::Completed => {
                matches!(transfer.state, crate::sftp::TransferState::Completed)
            }
        }
    }

    fn remove_local_download_output_for_transfer(
        transfer: &crate::sftp::Transfer,
    ) -> Option<Result<(), String>> {
        if !matches!(transfer.info.kind, crate::sftp::TransferType::Download) {
            return None;
        }
        if transfer.info.target.trim().is_empty() || transfer.info.name.trim().is_empty() {
            return None;
        }

        let path = Path::new(&transfer.info.target).join(&transfer.info.name);
        if !path.exists() {
            return None;
        }

        let result = if path.is_dir() {
            fs::remove_dir_all(&path)
        } else {
            fs::remove_file(&path)
        };
        Some(result.map_err(|err| format!("remove {} failed: {err}", path.display())))
    }

    pub(crate) fn pause_sftp_transfers_in_tab(
        &mut self,
        tab: crate::app::SftpTransferTab,
        cx: &mut Context<Self>,
    ) {
        let targets = self
            .transfers
            .iter()
            .filter(|transfer| Self::transfer_belongs_to_sftp_tab(transfer, tab))
            .filter(|transfer| matches!(transfer.state, crate::sftp::TransferState::Running))
            .map(|transfer| (transfer.tab_id.clone(), transfer.info.id.clone()))
            .collect::<Vec<_>>();

        for (group_id, transfer_id) in targets {
            if let Some(handle) = self.ensure_sftp_handle_for_group(&group_id) {
                self.mark_sftp_activity_for_group(&group_id);
                handle.pause_transfer(transfer_id);
            }
        }
        cx.notify();
    }

    pub(crate) fn resume_sftp_transfers_in_tab(
        &mut self,
        tab: crate::app::SftpTransferTab,
        cx: &mut Context<Self>,
    ) {
        let targets = self
            .transfers
            .iter()
            .filter(|transfer| Self::transfer_belongs_to_sftp_tab(transfer, tab))
            .filter(|transfer| matches!(transfer.state, crate::sftp::TransferState::Paused))
            .map(|transfer| (transfer.tab_id.clone(), transfer.info.id.clone()))
            .collect::<Vec<_>>();

        for (group_id, transfer_id) in targets {
            if let Some(handle) = self.ensure_sftp_handle_for_group(&group_id) {
                self.mark_sftp_activity_for_group(&group_id);
                handle.resume_transfer(transfer_id);
            }
        }
        cx.notify();
    }

    pub(crate) fn cancel_remove_sftp_transfers_in_tab(
        &mut self,
        tab: crate::app::SftpTransferTab,
        delete_local_downloads: bool,
        cx: &mut Context<Self>,
    ) {
        let transfers = self
            .transfers
            .iter()
            .filter(|transfer| Self::transfer_belongs_to_sftp_tab(transfer, tab))
            .cloned()
            .collect::<Vec<_>>();
        if transfers.is_empty() {
            return;
        }

        let mut cleanup_errors = Vec::new();
        for transfer in &transfers {
            if matches!(
                transfer.state,
                crate::sftp::TransferState::Running | crate::sftp::TransferState::Paused
            ) && let Some(handle) = self.ensure_sftp_handle_for_group(&transfer.tab_id)
            {
                self.mark_sftp_activity_for_group(&transfer.tab_id);
                handle.cancel_transfer(transfer.info.id.clone());
            }
            if delete_local_downloads
                && let Some(Err(err)) = Self::remove_local_download_output_for_transfer(transfer)
            {
                cleanup_errors.push(err);
            }
        }

        let ids = transfers
            .iter()
            .map(|transfer| transfer.info.id.as_str())
            .collect::<std::collections::HashSet<_>>();
        self.transfers
            .retain(|transfer| !ids.contains(transfer.info.id.as_str()));
        self.config.set_transfers(self.transfers.clone());
        if cleanup_errors.is_empty() {
            self.status = rust_i18n::t!("removed_transfers", count = ids.len()).into();
        } else {
            self.status = cleanup_errors.join("; ").into();
        }
        cx.notify();
    }

    pub(crate) fn active_shell_working_dir(&self) -> Option<String> {
        let active_group_id = self.active_group.as_ref()?;
        let tab_id = self.group_primary_ssh_tab_id(active_group_id)?;
        self.tabs
            .iter()
            .find(|tab| tab.id == tab_id)
            .and_then(|tab| tab.shell_working_dir.clone())
    }

    pub(crate) fn resolve_active_sftp_path(&self, path: &str) -> String {
        let current_dir = self
            .active_shell_working_dir()
            .or_else(|| self.active_sftp().map(|sftp| sftp.current_path.clone()))
            .unwrap_or_else(|| "/".to_string());
        let home_dir = self
            .active_sftp()
            .map(|sftp| sftp.home_dir.clone())
            .unwrap_or_else(|| "/".to_string());
        crate::sftp::resolve_remote_path(&current_dir, path, &home_dir)
    }

    pub(crate) fn navigate_sftp(&mut self, path: String, cx: &mut Context<Self>) {
        let resolved = self.resolve_active_sftp_path(&path);
        if let Some(handle) = self.ensure_active_sftp_handle() {
            self.mark_active_sftp_activity();
            tracing::info!(
                component = "sftp",
                operation = "navigate",
                remote_path = %crate::diagnostics::mask_path(&resolved),
                "Navigating SFTP directory"
            );
            handle.list_dir(resolved.clone());
            if let Some(sftp) = self.active_sftp_mut() {
                sftp.status = resolved;
                sftp.has_more_entries = false;
                sftp.loading_more_entries = false;
                sftp.reached_entries_limit = false;
            }
            cx.notify();
        }
    }

    pub(crate) fn load_more_sftp_entries(&mut self, cx: &mut Context<Self>) {
        let can_load_more = self
            .active_sftp()
            .is_some_and(|sftp| sftp.has_more_entries && !sftp.loading_more_entries);
        if !can_load_more {
            return;
        }
        let cursor_is_live = self
            .active_group
            .as_ref()
            .and_then(|group_id| self.sftp_handles.get(group_id))
            .is_some_and(|handle| !handle.commands_closed());
        if let Some(handle) = self.ensure_active_sftp_handle() {
            if !cursor_is_live {
                // A reclaimed worker cannot resume its old server-side handle.
                // It has already queued a fresh first page for the current path.
                cx.notify();
                return;
            }
            self.mark_active_sftp_activity();
            if handle.load_more_entries()
                && let Some(sftp) = self.active_sftp_mut()
            {
                sftp.loading_more_entries = true;
            }
            cx.notify();
        }
    }

    pub(crate) fn open_sftp_and_reveal_path(
        &mut self,
        path: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(active_group_id) = self.active_group.clone() else {
            return false;
        };
        let resolved = self.resolve_active_sftp_path(path);

        if let Some(group) = self
            .tab_groups
            .iter_mut()
            .find(|group| group.id == active_group_id)
        {
            if group.sftp.is_none() {
                return false;
            }
            group.sftp_page_open = true;
        } else {
            return false;
        }

        self.pending_sftp_selection_path = Some(resolved);
        self.activate_group_page(active_group_id, WorkspacePage::Sftp, window, cx);
        if let Some(handle) = self.ensure_active_sftp_handle() {
            if let Some(target_path) = self.pending_sftp_selection_path.clone() {
                self.mark_active_sftp_activity();
                handle.reveal_path(target_path.clone());
                if let Some(sftp) = self.active_sftp_mut() {
                    sftp.status = target_path;
                }
                cx.notify();
                return true;
            }
        }
        true
    }

    pub(crate) fn select_sftp_entry(&mut self, entry: RemoteEntry, cx: &mut Context<Self>) {
        let was_selected = self
            .active_sftp()
            .and_then(|sftp| sftp.selected_path.as_deref())
            == Some(entry.full_path.as_str());
        if entry.is_dir && was_selected {
            self.navigate_sftp(entry.full_path, cx);
            return;
        }
        if let Some(sftp) = self.active_sftp_mut() {
            sftp.selected_path = Some(entry.full_path.clone());
            sftp.selected_entries.clear();
            sftp.selected_entries.insert(entry.full_path);
        }
        cx.notify();
    }

    pub(crate) fn mark_sftp_entry_selected(&mut self, path: &str, cx: &mut Context<Self>) {
        if let Some(sftp) = self.active_sftp_mut() {
            sftp.selected_path = Some(path.to_string());
        }
        cx.notify();
    }

    pub(crate) fn sftp_parent_path(path: &str) -> String {
        if path == "/" {
            return "/".to_string();
        }
        path.trim_end_matches('/')
            .rsplit_once('/')
            .map(|(parent, _)| {
                if parent.is_empty() {
                    "/".to_string()
                } else {
                    parent.to_string()
                }
            })
            .unwrap_or_else(|| "/".to_string())
    }

    pub(crate) fn refresh_sftp(&mut self, cx: &mut Context<Self>) {
        if let Some(path) = self.active_sftp().map(|sftp| sftp.current_path.clone()) {
            self.navigate_sftp(path, cx);
        }
    }

    pub(crate) fn sync_sftp_path_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(path) = self.pending_sftp_path_sync.take() else {
            return;
        };
        self.sftp_path_input.update(cx, |state, cx| {
            state.set_value(path, window, cx);
        });
    }

    fn load_local_file_browser(&mut self, path: &str) -> Result<String, String> {
        let current_path = self.local_file_browser.current_path.clone();
        let resolved = Self::resolve_local_browser_path(&current_path, path);
        let resolved_str = resolved.to_string_lossy().to_string();
        let entries = Self::read_local_browser_entries(&resolved_str)?;
        self.local_file_browser.current_path = resolved_str.clone();
        self.local_file_browser.status = resolved_str.clone();
        self.local_file_browser.entries = entries;
        self.local_file_browser.selected_path = None;
        self.local_file_browser.selected_entries.clear();
        self.pending_local_sftp_path_sync = Some(resolved_str.clone());
        Ok(resolved_str)
    }

    fn persist_active_local_sftp_path(&mut self, path: &str) {
        let Some(session_id) = self.active_sftp_saved_session_id() else {
            return;
        };
        if self.config.set_last_local_sftp_path(&session_id, path) {
            self.config.save_logged("set_last_local_sftp_path");
        }
    }

    pub(crate) fn restore_active_local_sftp_path(&mut self, cx: &mut Context<Self>) {
        let saved_path = self
            .active_sftp_saved_session_id()
            .and_then(|session_id| self.config.last_local_sftp_path(&session_id))
            .map(str::to_string);

        let result = match saved_path.as_deref() {
            Some(path) => self.load_local_file_browser(path),
            None => self.load_local_file_browser(&Self::default_local_browser_dir()),
        };
        if let Err(err) = result {
            if let Some(path) = saved_path {
                tracing::warn!(
                    component = "local_browser",
                    operation = "restore_last_path",
                    local_path = %crate::diagnostics::mask_path(&path),
                    error = %crate::diagnostics::sanitize_error(&err),
                    "Saved local SFTP directory is unavailable; falling back to the home directory"
                );
                if let Err(fallback_err) =
                    self.load_local_file_browser(&Self::default_local_browser_dir())
                {
                    self.local_file_browser.status = fallback_err;
                }
            } else {
                self.local_file_browser.status = err;
            }
        }
        cx.notify();
    }

    pub(crate) fn navigate_local_file_browser(&mut self, path: String, cx: &mut Context<Self>) {
        match self.load_local_file_browser(&path) {
            Ok(path) => self.persist_active_local_sftp_path(&path),
            Err(err) => self.local_file_browser.status = err,
        }
        cx.notify();
    }

    pub(crate) fn refresh_local_file_browser(&mut self, cx: &mut Context<Self>) {
        let current_path = self.local_file_browser.current_path.clone();
        self.navigate_local_file_browser(current_path, cx);
    }

    pub(crate) fn sync_local_sftp_path_input(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(path) = self.pending_local_sftp_path_sync.take() else {
            return;
        };
        self.local_sftp_path_input.update(cx, |state, cx| {
            state.set_value(path, window, cx);
        });
    }

    pub(crate) fn select_local_file_entry(
        &mut self,
        entry: LocalFileEntry,
        cx: &mut Context<Self>,
    ) {
        let was_selected =
            self.local_file_browser.selected_path.as_deref() == Some(entry.full_path.as_str());
        if entry.is_dir && was_selected {
            self.navigate_local_file_browser(entry.full_path, cx);
            return;
        }

        if !entry.is_dir && was_selected {
            match open::that(&entry.full_path) {
                Ok(()) => {
                    self.local_file_browser.status = entry.full_path;
                }
                Err(err) => {
                    self.local_file_browser.status = format!("open failed: {err:#}");
                }
            }
            cx.notify();
            return;
        }

        self.local_file_browser.selected_path = Some(entry.full_path.clone());
        self.local_file_browser.selected_entries.clear();
        self.local_file_browser
            .selected_entries
            .insert(entry.full_path);
        cx.notify();
    }

    pub(crate) fn toggle_local_file_entry(
        &mut self,
        path: String,
        checked: bool,
        cx: &mut Context<Self>,
    ) {
        if checked {
            self.local_file_browser.selected_entries.insert(path);
        } else {
            self.local_file_browser.selected_entries.remove(&path);
        }
        cx.notify();
    }

    pub(crate) fn toggle_all_local_file_entries(&mut self, checked: bool, cx: &mut Context<Self>) {
        if checked {
            let paths: Vec<String> = self
                .local_file_browser
                .entries
                .iter()
                .filter(|entry| self.show_hidden_files || !entry.name.starts_with('.'))
                .map(|entry| entry.full_path.clone())
                .collect();
            self.local_file_browser.selected_entries.extend(paths);
        } else {
            self.local_file_browser.selected_entries.clear();
        }
        cx.notify();
    }

    pub(crate) fn mark_local_file_entry_selected(&mut self, path: &str, cx: &mut Context<Self>) {
        self.local_file_browser.selected_path = Some(path.to_string());
        cx.notify();
    }

    fn open_local_file_browser_entry(
        &mut self,
        path: String,
        is_dir: bool,
        cx: &mut Context<Self>,
    ) {
        if is_dir {
            self.navigate_local_file_browser(path, cx);
            return;
        }

        match open::that(&path) {
            Ok(()) => {
                self.local_file_browser.status = path;
            }
            Err(err) => {
                self.local_file_browser.status = format!("open failed: {err:#}");
            }
        }
        cx.notify();
    }

    fn upload_local_paths_to_sftp(&mut self, paths: Vec<String>, cx: &mut Context<Self>) -> bool {
        if paths.is_empty() {
            return false;
        }
        let Some(remote_dir) = self.active_sftp().map(|sftp| sftp.current_path.clone()) else {
            return false;
        };
        let Some(handle) = self.ensure_active_sftp_handle() else {
            return false;
        };
        self.mark_active_sftp_activity();
        let selected_count = paths.len();
        tracing::info!(
            component = "sftp",
            operation = "upload_selected",
            item_count = selected_count,
            remote_path = %crate::diagnostics::mask_path(&remote_dir),
            "Starting SFTP upload"
        );
        let _ = handle.upload_paths(paths, remote_dir.clone());
        self.sftp_transfer_tab = crate::app::SftpTransferTab::Active;
        self.local_file_browser.status =
            format!("uploading {selected_count} item(s) to {remote_dir}");
        cx.notify();
        true
    }

    pub(crate) fn upload_selected_local_entries_to_sftp(&mut self, cx: &mut Context<Self>) {
        let selected: Vec<String> = self
            .local_file_browser
            .selected_entries
            .iter()
            .cloned()
            .collect();
        if self.upload_local_paths_to_sftp(selected, cx) {
            self.local_file_browser.selected_entries.clear();
            cx.notify();
        }
    }

    pub(crate) fn open_sftp_context_menu(
        &mut self,
        remote_path: String,
        is_dir: bool,
        position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.saved_session_context_menu = None;
        self.saved_group_context_menu = None;
        self.sftp_context_menu = Some(SftpContextMenuState {
            target: SftpContextMenuTarget::Remote {
                path: remote_path,
                is_dir,
            },
            position,
        });
        cx.notify();
    }

    pub(crate) fn open_local_sftp_context_menu(
        &mut self,
        local_path: String,
        is_dir: bool,
        position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.saved_session_context_menu = None;
        self.saved_group_context_menu = None;
        self.sftp_context_menu = Some(SftpContextMenuState {
            target: SftpContextMenuTarget::Local {
                path: local_path,
                is_dir,
            },
            position,
        });
        cx.notify();
    }

    pub(crate) fn dismiss_sftp_context_menu(&mut self, cx: &mut Context<Self>) {
        if self.sftp_context_menu.take().is_some() {
            cx.notify();
        }
    }

    pub(crate) fn trigger_sftp_context_download(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(menu) = self.sftp_context_menu.take() else {
            return;
        };
        if let SftpContextMenuTarget::Remote { path, .. } = menu.target {
            self.download_sftp_entry(path, window, cx);
        }
        cx.notify();
    }

    pub(crate) fn trigger_sftp_context_edit(&mut self, cx: &mut Context<Self>) {
        let Some(menu) = self.sftp_context_menu.take() else {
            return;
        };
        if let SftpContextMenuTarget::Remote { path, .. } = menu.target
            && let Some(handle) = self.ensure_active_sftp_handle()
        {
            self.mark_active_sftp_activity();
            tracing::info!(
                component = "sftp",
                operation = "edit_remote_file",
                remote_path = %crate::diagnostics::mask_path(&path),
                "Opening remote file for editing"
            );
            handle.edit_file(path);
        }
        cx.notify();
    }

    pub(crate) fn trigger_sftp_context_open(&mut self, cx: &mut Context<Self>) {
        let Some(menu) = self.sftp_context_menu.take() else {
            return;
        };
        match menu.target {
            SftpContextMenuTarget::Remote { path, is_dir } => {
                if is_dir {
                    self.navigate_sftp(path, cx);
                }
            }
            SftpContextMenuTarget::Local { path, is_dir } => {
                self.open_local_file_browser_entry(path, is_dir, cx);
            }
        }
        cx.notify();
    }

    pub(crate) fn trigger_sftp_context_refresh(&mut self, cx: &mut Context<Self>) {
        let Some(menu) = self.sftp_context_menu.take() else {
            return;
        };
        match menu.target {
            SftpContextMenuTarget::Remote { .. } => self.refresh_sftp(cx),
            SftpContextMenuTarget::Local { .. } => self.refresh_local_file_browser(cx),
        }
        cx.notify();
    }

    pub(crate) fn trigger_sftp_context_new_folder(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(menu) = self.sftp_context_menu.take() else {
            return;
        };
        if matches!(menu.target, SftpContextMenuTarget::Remote { .. }) {
            self.sftp_creating_folder = true;
            self.sftp_new_folder_input.update(cx, |input, cx| {
                input.set_value("", window, cx);
                input.focus_handle(cx).focus(window, cx);
            });
        }
        cx.notify();
    }

    pub(crate) fn trigger_sftp_context_upload_file(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(menu) = self.sftp_context_menu.take() else {
            return;
        };
        if matches!(menu.target, SftpContextMenuTarget::Remote { .. }) {
            self.upload_sftp_files(window, cx);
        }
        cx.notify();
    }

    pub(crate) fn trigger_sftp_context_upload_folder(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(menu) = self.sftp_context_menu.take() else {
            return;
        };
        if matches!(menu.target, SftpContextMenuTarget::Remote { .. }) {
            self.upload_sftp_folder(window, cx);
        }
        cx.notify();
    }

    pub(crate) fn trigger_sftp_context_delete(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(menu) = self.sftp_context_menu.take() else {
            return;
        };
        if let SftpContextMenuTarget::Remote { path, .. } = menu.target {
            if let Some(sftp) = self.active_sftp_mut() {
                sftp.selected_path = Some(path.clone());
                sftp.selected_entries.clear();
                sftp.selected_entries.insert(path);
            }
            self.show_delete_confirm_dialog(window, cx);
        }
        cx.notify();
    }

    pub(crate) fn trigger_local_context_upload(&mut self, cx: &mut Context<Self>) {
        let Some(menu) = self.sftp_context_menu.take() else {
            return;
        };
        if let SftpContextMenuTarget::Local { path, .. } = menu.target {
            self.upload_local_paths_to_sftp(vec![path], cx);
        }
        cx.notify();
    }

    pub(crate) fn download_sftp_entry(
        &mut self,
        remote_path: String,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(handle) = self.ensure_active_sftp_handle() else {
            return;
        };
        self.mark_active_sftp_activity();
        let local_dir = self.local_file_browser.current_path.clone();
        tracing::info!(
            component = "sftp",
            operation = "download",
            remote_path = %crate::diagnostics::mask_path(&remote_path),
            local_path = %crate::diagnostics::mask_path(&local_dir),
            "Starting SFTP download"
        );
        handle.download(remote_path, local_dir.clone());
        self.sftp_transfer_tab = crate::app::SftpTransferTab::Active;
        self.local_file_browser.status = format!("downloading to {local_dir}");
        cx.notify();
    }

    pub(crate) fn upload_sftp_files(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let remote_dir = self
            .active_sftp()
            .map(|sftp| sftp.current_path.clone())
            .unwrap_or_else(|| "/".into());
        let Some(handle) = self.ensure_active_sftp_handle() else {
            return;
        };
        self.mark_active_sftp_activity();
        let path_prompt = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Select File to Upload".into()),
        });
        cx.spawn_in(window, async move |this, cx| {
            match path_prompt.await {
                Ok(Ok(Some(mut paths))) => {
                    if let Some(file) = paths.pop() {
                        let local_path = file.to_string_lossy().to_string();
                        tracing::info!(
                            component = "sftp",
                            operation = "upload_file",
                            local_path = %crate::diagnostics::mask_path(&local_path),
                            remote_path = %crate::diagnostics::mask_path(&remote_dir),
                            "Starting SFTP file upload"
                        );
                        handle.upload_paths(vec![local_path], remote_dir);
                        this.update(cx, |this, cx| {
                            this.sftp_transfer_tab = crate::app::SftpTransferTab::Active;
                            cx.notify();
                        })?;
                    }
                }
                Ok(Err(err)) => {
                    this.update(cx, |this, cx| {
                        this.status = format!("upload picker failed: {err}").into();
                        cx.notify();
                    })?;
                }
                _ => {}
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    pub(crate) fn upload_sftp_folder(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let remote_dir = self
            .active_sftp()
            .map(|sftp| sftp.current_path.clone())
            .unwrap_or_else(|| "/".into());
        let Some(handle) = self.ensure_active_sftp_handle() else {
            return;
        };
        self.mark_active_sftp_activity();
        let path_prompt = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Select Folder to Upload".into()),
        });
        cx.spawn_in(window, async move |this, cx| {
            match path_prompt.await {
                Ok(Ok(Some(mut paths))) => {
                    if let Some(folder) = paths.pop() {
                        let local_path = folder.to_string_lossy().to_string();
                        tracing::info!(
                            component = "sftp",
                            operation = "upload_folder",
                            local_path = %crate::diagnostics::mask_path(&local_path),
                            remote_path = %crate::diagnostics::mask_path(&remote_dir),
                            "Starting SFTP folder upload"
                        );
                        handle.upload_paths(vec![local_path], remote_dir);
                        this.update(cx, |this, cx| {
                            this.sftp_transfer_tab = crate::app::SftpTransferTab::Active;
                            cx.notify();
                        })?;
                    }
                }
                Ok(Err(err)) => {
                    this.update(cx, |this, cx| {
                        this.status = format!("upload picker failed: {err}").into();
                        cx.notify();
                    })?;
                }
                _ => {}
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    pub(crate) fn toggle_sftp_entry(
        &mut self,
        path: String,
        checked: bool,
        cx: &mut Context<Self>,
    ) {
        if let Some(sftp) = self.active_sftp_mut() {
            if checked {
                sftp.selected_entries.insert(path);
            } else {
                sftp.selected_entries.remove(&path);
            }
            cx.notify();
        }
    }

    pub(crate) fn toggle_all_sftp_entries(&mut self, checked: bool, cx: &mut Context<Self>) {
        if let Some(sftp) = self.active_sftp_mut() {
            if checked {
                let paths: Vec<String> = sftp.entries.iter().map(|e| e.full_path.clone()).collect();
                for path in paths {
                    sftp.selected_entries.insert(path);
                }
            } else {
                sftp.selected_entries.clear();
            }
            cx.notify();
        }
    }

    pub(crate) fn download_selected_sftp_entries(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(sftp) = self.active_sftp() else {
            return;
        };
        let selected: Vec<String> = sftp.selected_entries.iter().cloned().collect();
        if selected.is_empty() {
            return;
        }

        let Some(handle) = self.ensure_active_sftp_handle() else {
            return;
        };
        self.mark_active_sftp_activity();
        let local_dir = self.local_file_browser.current_path.clone();
        tracing::info!(
            component = "sftp",
            operation = "download_batch",
            item_count = selected.len(),
            local_path = %crate::diagnostics::mask_path(&local_dir),
            "Starting SFTP batch download"
        );
        for remote in selected {
            let _ = handle.download(remote, local_dir.clone());
        }
        if let Some(sftp_mut) = self.active_sftp_mut() {
            sftp_mut.selected_entries.clear();
        }
        self.sftp_transfer_tab = crate::app::SftpTransferTab::Active;
        self.local_file_browser.status = format!("downloading to {local_dir}");
        cx.notify();
    }

    pub(crate) fn upload_sftp_files_batch(&mut self, paths: Vec<String>, cx: &mut Context<Self>) {
        if paths.is_empty() {
            return;
        }
        let Some(remote_dir) = self.active_sftp().map(|sftp| sftp.current_path.clone()) else {
            return;
        };
        let Some(handle) = self.ensure_active_sftp_handle() else {
            return;
        };
        self.mark_active_sftp_activity();
        tracing::info!(
            component = "sftp",
            operation = "upload_batch",
            item_count = paths.len(),
            remote_path = %crate::diagnostics::mask_path(&remote_dir),
            "Starting SFTP batch upload"
        );
        let _ = handle.upload_paths(paths, remote_dir);
        self.sftp_transfer_tab = crate::app::SftpTransferTab::Active;
        cx.notify();
    }
}

fn should_reclaim_sftp_worker(
    deep_sleep: bool,
    active_work_pins: usize,
    last_activity: Option<std::time::Instant>,
    now: std::time::Instant,
    idle_timeout: std::time::Duration,
) -> bool {
    if active_work_pins > 0 {
        return false;
    }
    if deep_sleep {
        return true;
    }
    last_activity.is_some_and(|last_activity| now.duration_since(last_activity) >= idle_timeout)
}

#[cfg(test)]
mod idle_reclaim_tests {
    use std::time::{Duration, Instant};

    use super::should_reclaim_sftp_worker;

    #[test]
    fn deep_sleep_reclaims_only_unpinned_workers() {
        let now = Instant::now();

        assert!(should_reclaim_sftp_worker(
            true,
            0,
            Some(now),
            now,
            Duration::from_secs(300),
        ));
        assert!(!should_reclaim_sftp_worker(
            true,
            1,
            Some(now - Duration::from_secs(3600)),
            now,
            Duration::from_secs(300),
        ));
    }

    #[test]
    fn background_reclaim_still_requires_idle_timeout() {
        let now = Instant::now();
        let timeout = Duration::from_secs(300);

        assert!(!should_reclaim_sftp_worker(
            false,
            0,
            Some(now - Duration::from_secs(299)),
            now,
            timeout,
        ));
        assert!(should_reclaim_sftp_worker(
            false,
            0,
            Some(now - timeout),
            now,
            timeout,
        ));
        assert!(!should_reclaim_sftp_worker(false, 0, None, now, timeout));
    }
}
