use std::{
    fs,
    path::{Component, Path, PathBuf},
    time::UNIX_EPOCH,
};

use directories::BaseDirs;
use gpui::{Context, PathPromptOptions, Pixels, Point, Window};

use crate::{
    AxShell, SftpContextMenuState,
    app::LocalFileEntry,
    sftp::{RemoteEntry, SftpHandle},
    terminal,
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
                    tracing::warn!("[local-browser] skipped unreadable entry in '{path}': {err}");
                    continue;
                }
            };
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(err) => {
                    tracing::warn!(
                        "[local-browser] skipped entry with unreadable metadata in '{path}': {err}"
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

    pub(crate) fn active_sftp(&self) -> Option<&terminal::SftpUiState> {
        self.active_group
            .as_ref()
            .and_then(|id| self.tab_groups.iter().find(|g| &g.id == id))
            .and_then(|g| g.sftp.as_ref())
    }

    pub(crate) fn active_sftp_mut(&mut self) -> Option<&mut terminal::SftpUiState> {
        let active_id = self.active_group.clone()?;
        self.tab_groups
            .iter_mut()
            .find(|g| g.id == active_id)
            .and_then(|g| g.sftp.as_mut())
    }

    pub(crate) fn active_sftp_handle(&self) -> Option<&SftpHandle> {
        self.active_group
            .as_ref()
            .and_then(|id| self.sftp_handles.get(id))
    }

    pub(crate) fn navigate_sftp(&mut self, path: String, cx: &mut Context<Self>) {
        if let Some(handle) = self.active_sftp_handle() {
            tracing::info!("[sftp] navigating to directory: '{}'", path);
            handle.list_dir(path.clone());
            if let Some(sftp) = self.active_sftp_mut() {
                sftp.current_path = path;
                self.pending_sftp_path_sync = Some(sftp.current_path.clone());
            }
            cx.notify();
        }
    }

    pub(crate) fn select_sftp_entry(&mut self, entry: RemoteEntry, cx: &mut Context<Self>) {
        if entry.is_dir {
            self.navigate_sftp(entry.full_path, cx);
            return;
        }
        self.mark_sftp_entry_selected(&entry.full_path, cx);
        if let Some(sftp) = self.active_sftp_mut() {
            if !sftp.selected_entries.remove(&entry.full_path) {
                sftp.selected_entries.insert(entry.full_path);
            }
        }
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

    pub(crate) fn navigate_local_file_browser(&mut self, path: String, cx: &mut Context<Self>) {
        let current_path = self.local_file_browser.current_path.clone();
        let resolved = Self::resolve_local_browser_path(&current_path, &path);
        let resolved_str = resolved.to_string_lossy().to_string();
        match Self::read_local_browser_entries(&resolved_str) {
            Ok(entries) => {
                self.local_file_browser.current_path = resolved_str.clone();
                self.local_file_browser.status = resolved_str.clone();
                self.local_file_browser.entries = entries;
                self.local_file_browser.selected_path = None;
                self.local_file_browser.selected_entries.clear();
                self.pending_local_sftp_path_sync = Some(resolved_str);
            }
            Err(err) => {
                self.local_file_browser.status = err;
            }
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
        if entry.is_dir {
            self.navigate_local_file_browser(entry.full_path, cx);
            return;
        }
        self.mark_local_file_entry_selected(&entry.full_path, cx);
        if !self
            .local_file_browser
            .selected_entries
            .remove(&entry.full_path)
        {
            self.local_file_browser
                .selected_entries
                .insert(entry.full_path);
        }
        cx.notify();
    }

    pub(crate) fn mark_local_file_entry_selected(&mut self, path: &str, cx: &mut Context<Self>) {
        self.local_file_browser.selected_path = Some(path.to_string());
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

    pub(crate) fn upload_selected_local_entries_to_sftp(&mut self, cx: &mut Context<Self>) {
        let selected: Vec<String> = self
            .local_file_browser
            .selected_entries
            .iter()
            .cloned()
            .collect();
        if selected.is_empty() {
            return;
        }
        let Some(remote_dir) = self.active_sftp().map(|sftp| sftp.current_path.clone()) else {
            return;
        };
        let Some(handle) = self.active_sftp_handle() else {
            return;
        };
        tracing::info!(
            "[sftp] initiating upload of {} local browser entries to '{}'",
            selected.len(),
            remote_dir
        );
        let _ = handle.commands.send(crate::sftp::SftpCommand::UploadPaths {
            locals: selected,
            remote_dir,
        });
        self.show_transfers_dialog = true;
        cx.notify();
    }

    pub(crate) fn open_sftp_context_menu(
        &mut self,
        remote_path: String,
        is_dir: bool,
        position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.sftp_context_menu = Some(SftpContextMenuState {
            remote_path,
            is_dir,
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
        self.download_sftp_entry(menu.remote_path, window, cx);
        cx.notify();
    }

    pub(crate) fn trigger_sftp_context_edit(&mut self, cx: &mut Context<Self>) {
        let Some(menu) = self.sftp_context_menu.take() else {
            return;
        };
        if let Some(handle) = self.active_sftp_handle() {
            tracing::info!("[sftp] triggering edit for file: '{}'", menu.remote_path);
            handle.edit_file(menu.remote_path);
        }
        cx.notify();
    }

    pub(crate) fn download_sftp_entry(
        &mut self,
        remote_path: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(handle) = self.active_sftp_handle().cloned() else {
            return;
        };
        let path_prompt = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Select Download Folder".into()),
        });
        cx.spawn_in(window, async move |this, cx| {
            match path_prompt.await {
                Ok(Ok(Some(mut paths))) => {
                    if let Some(folder) = paths.pop() {
                        let local_path = folder.to_string_lossy().to_string();
                        tracing::info!(
                            "[sftp] initiating download of '{}' to '{}'",
                            remote_path,
                            local_path
                        );
                        handle.download(remote_path, local_path);
                        this.update(cx, |this, cx| {
                            this.show_transfers_dialog = true;
                            cx.notify();
                        })?;
                    }
                }
                Ok(Err(err)) => {
                    this.update(cx, |this, cx| {
                        this.status = format!("download picker failed: {err}").into();
                        cx.notify();
                    })?;
                }
                _ => {}
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    pub(crate) fn upload_sftp_files(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(handle) = self.active_sftp_handle().cloned() else {
            return;
        };
        let remote_dir = self
            .active_sftp()
            .map(|sftp| sftp.current_path.clone())
            .unwrap_or_else(|| "/".into());
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
                            "[sftp] initiating upload of file '{}' to '{}'",
                            local_path,
                            remote_dir
                        );
                        handle.upload_paths(vec![local_path], remote_dir);
                        this.update(cx, |this, cx| {
                            this.show_transfers_dialog = true;
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
        let Some(handle) = self.active_sftp_handle().cloned() else {
            return;
        };
        let remote_dir = self
            .active_sftp()
            .map(|sftp| sftp.current_path.clone())
            .unwrap_or_else(|| "/".into());
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
                            "[sftp] initiating upload of folder '{}' to '{}'",
                            local_path,
                            remote_dir
                        );
                        handle.upload_paths(vec![local_path], remote_dir);
                        this.update(cx, |this, cx| {
                            this.show_transfers_dialog = true;
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(sftp) = self.active_sftp() else {
            return;
        };
        let selected: Vec<String> = sftp.selected_entries.iter().cloned().collect();
        if selected.is_empty() {
            return;
        }

        let Some(handle) = self.active_sftp_handle().cloned() else {
            return;
        };

        let path_prompt = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Select Download Folder".into()),
        });

        cx.spawn_in(window, async move |this, cx| {
            if let Ok(Ok(Some(mut paths))) = path_prompt.await {
                if let Some(folder) = paths.pop() {
                    let local_dir = folder.to_string_lossy().to_string();
                    tracing::info!(
                        "[sftp] initiating batch download of {} entries to '{}'",
                        selected.len(),
                        local_dir
                    );
                    for remote in selected {
                        let _ = handle.commands.send(crate::sftp::SftpCommand::Download {
                            remote,
                            local_dir: local_dir.clone(),
                        });
                    }

                    let _ = this.update(cx, |this, cx| {
                        if let Some(sftp_mut) = this.active_sftp_mut() {
                            sftp_mut.selected_entries.clear();
                        }
                        this.show_transfers_dialog = true;
                        cx.notify();
                    });
                }
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    pub(crate) fn upload_sftp_files_batch(&mut self, paths: Vec<String>, cx: &mut Context<Self>) {
        if paths.is_empty() {
            return;
        }
        if let Some(sftp) = self.active_sftp() {
            if let Some(handle) = self.active_sftp_handle() {
                tracing::info!(
                    "[sftp] initiating batch upload of {} files to '{}'",
                    paths.len(),
                    sftp.current_path
                );
                let _ = handle.commands.send(crate::sftp::SftpCommand::UploadPaths {
                    locals: paths,
                    remote_dir: sftp.current_path.clone(),
                });
                self.show_transfers_dialog = true;
                cx.notify();
            }
        }
    }
}
