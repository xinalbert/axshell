use std::ops::Range;

use gpui::{Bounds, Context, Pixels, Window, point, px, size};
use gpui_component::WindowExt as _;
use rust_i18n::t;

use crate::{
    AxShell,
    app::{ConnectionProgress, PaneLayout, SftpUiState},
    config::ConfigStore,
    terminal::{BackendCommand, TabKind},
};

#[derive(Clone)]
pub(crate) struct TabGroup {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) pane_root: PaneLayout,
    pub(crate) sftp: Option<SftpUiState>,
    pub(crate) sftp_page_open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum WorkspacePage {
    #[default]
    Terminal,
    Sftp,
    Settings,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorkspaceTabDescriptor {
    pub(crate) group_id: Option<String>,
    pub(crate) group_index: Option<usize>,
    pub(crate) page: WorkspacePage,
}

fn workspace_tab_selected_for(
    entry: &WorkspaceTabDescriptor,
    workspace_page: WorkspacePage,
    active_group: Option<&str>,
    settings_page_open: bool,
) -> bool {
    match entry.page {
        WorkspacePage::Settings => settings_page_open && workspace_page == WorkspacePage::Settings,
        page => workspace_page == page && entry.group_id.as_deref() == active_group,
    }
}

fn active_workspace_tab_index_for(
    tabs: &[WorkspaceTabDescriptor],
    workspace_page: WorkspacePage,
    active_group: Option<&str>,
    settings_page_open: bool,
) -> usize {
    tabs.iter()
        .position(|entry| {
            workspace_tab_selected_for(entry, workspace_page, active_group, settings_page_open)
        })
        .or_else(|| {
            active_group.and_then(|group_id| {
                tabs.iter().position(|entry| {
                    entry.page == WorkspacePage::Terminal
                        && entry.group_id.as_deref() == Some(group_id)
                })
            })
        })
        .unwrap_or(0)
}

impl AxShell {
    pub(crate) fn workspace_tabs(&self) -> Vec<WorkspaceTabDescriptor> {
        let mut tabs = Vec::new();

        for (group_index, group) in self.tab_groups.iter().enumerate() {
            tabs.push(WorkspaceTabDescriptor {
                group_id: Some(group.id.clone()),
                group_index: Some(group_index),
                page: WorkspacePage::Terminal,
            });

            if group.sftp.is_some() && group.sftp_page_open {
                tabs.push(WorkspaceTabDescriptor {
                    group_id: Some(group.id.clone()),
                    group_index: Some(group_index),
                    page: WorkspacePage::Sftp,
                });
            }
        }

        if self.settings_page_open {
            tabs.push(WorkspaceTabDescriptor {
                group_id: None,
                group_index: None,
                page: WorkspacePage::Settings,
            });
        }

        tabs
    }

    pub(crate) fn workspace_tab_selected(&self, entry: &WorkspaceTabDescriptor) -> bool {
        workspace_tab_selected_for(
            entry,
            self.workspace_page,
            self.active_group.as_deref(),
            self.settings_page_open,
        )
    }

    pub(crate) fn active_workspace_tab_index(&self, tabs: &[WorkspaceTabDescriptor]) -> usize {
        active_workspace_tab_index_for(
            tabs,
            self.workspace_page,
            self.active_group.as_deref(),
            self.settings_page_open,
        )
    }

    /// Scroll the tab bar just enough to reveal the active rendered workspace tab.
    pub(crate) fn ensure_active_workspace_tab_visible(&self) {
        let workspace_tabs = self.workspace_tabs();
        if workspace_tabs.is_empty() {
            return;
        }

        self.tabs_scroll_handle
            .scroll_to_item(self.active_workspace_tab_index(&workspace_tabs));
    }

    pub(crate) fn active_group_sftp_page_open(&self) -> bool {
        self.active_group
            .as_ref()
            .and_then(|group_id| self.tab_groups.iter().find(|group| &group.id == group_id))
            .is_some_and(|group| group.sftp.is_some() && group.sftp_page_open)
    }

    pub(crate) fn transfer_source_title(&self, tab_id: &str) -> String {
        self.tabs
            .iter()
            .find(|tab| tab.id == tab_id)
            .map(|tab| tab.title.clone())
            .or_else(|| {
                self.tab_groups
                    .iter()
                    .find(|group| group.id == tab_id)
                    .map(|group| group.title.clone())
            })
            .or_else(|| {
                self.tab_groups
                    .iter()
                    .find(|group| group.pane_root.contains(tab_id))
                    .map(|group| group.title.clone())
            })
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub(crate) fn set_workspace_page(&mut self, page: WorkspacePage, cx: &mut Context<Self>) {
        let page = if page == WorkspacePage::Sftp && !self.active_group_sftp_page_open() {
            WorkspacePage::Terminal
        } else {
            page
        };

        if self.workspace_page == page {
            self.ensure_active_workspace_tab_visible();
            return;
        }

        if self.workspace_page == WorkspacePage::Settings {
            self.keybinds_suspended = false;
            self.recording_action = None;
            self.keybind_error = None;
            crate::app::keybinding_recorder::bind_workspace_keys_from_config(cx, &self.config);
            crate::app::app_menu::refresh(cx);
        }

        if self.workspace_page == WorkspacePage::Terminal && page != WorkspacePage::Terminal {
            self.search.active = false;
            self.search.query.clear();
            self.search.matches.clear();
            self.search.current = 0;
            self.search.target_tab = None;
            self.search.bar_bounds = None;
        }

        if page == WorkspacePage::Settings {
            crate::app::keybinding_recorder::unbind_all_workspace_keys(cx, &self.config);
            self.keybinds_suspended = true;
        }

        self.workspace_page = page;
        if page == WorkspacePage::Sftp && self.active_sftp_should_sync_shell_dir_on_entry() {
            self.sync_active_sftp_to_shell_working_dir(cx);
        }
        self.ensure_active_workspace_tab_visible();
        cx.notify();
    }

    pub(crate) fn open_settings_page(&mut self, cx: &mut Context<Self>) {
        self.settings_page_open = true;
        self.set_workspace_page(WorkspacePage::Settings, cx);
    }

    pub(crate) fn close_settings_page(&mut self, cx: &mut Context<Self>) {
        self.settings_page_open = false;
        if self.workspace_page == WorkspacePage::Settings {
            self.set_workspace_page(WorkspacePage::Terminal, cx);
        } else {
            self.ensure_active_workspace_tab_visible();
            cx.notify();
        }
    }

    pub(crate) fn switch_workspace_tab(
        &mut self,
        step: isize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace_tabs = self.workspace_tabs();
        if workspace_tabs.len() <= 1 {
            return;
        }

        let current_index = self.active_workspace_tab_index(&workspace_tabs);
        let next_index =
            (current_index as isize + step).rem_euclid(workspace_tabs.len() as isize) as usize;
        let Some(target) = workspace_tabs.get(next_index).cloned() else {
            return;
        };

        match target.page {
            WorkspacePage::Settings => self.open_settings_page(cx),
            page => {
                let Some(group_id) = target.group_id else {
                    return;
                };
                self.activate_group_page(group_id, page, window, cx);
            }
        }
    }

    pub(crate) fn toggle_active_sftp_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.workspace_page == WorkspacePage::Sftp {
            if let Some(active_group_id) = self.active_group.clone() {
                if self.confirm_sftp_close_with_shortcut(&active_group_id, window, cx) {
                    return;
                }
                self.close_sftp_page(active_group_id, window, cx);
            } else {
                self.set_workspace_page(WorkspacePage::Terminal, cx);
                self.focus_handle.focus(window, cx);
            }
            return;
        }

        let Some(active_group_id) = self.active_group.clone() else {
            self.status = t!("open_ssh_tab_sftp").into();
            cx.notify();
            return;
        };

        if let Some(group) = self
            .tab_groups
            .iter_mut()
            .find(|group| group.id == active_group_id)
            && group.sftp.is_some()
        {
            group.sftp_page_open = true;
            self.ensure_sftp_handle_for_group(&active_group_id);
            self.mark_sftp_activity_for_group(&active_group_id);
            self.set_workspace_page(WorkspacePage::Sftp, cx);
            self.focus_handle.focus(window, cx);
        } else {
            self.status = t!("open_ssh_tab_sftp").into();
            cx.notify();
        }
    }

    pub(crate) fn open_sftp_transfers_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_group_id) = self.active_group.clone() else {
            self.status = t!("open_ssh_tab_sftp").into();
            cx.notify();
            return;
        };

        if let Some(group) = self
            .tab_groups
            .iter_mut()
            .find(|group| group.id == active_group_id)
            && group.sftp.is_some()
        {
            group.sftp_page_open = true;
            self.sftp_transfer_tab = crate::app::SftpTransferTab::Active;
            self.ensure_sftp_handle_for_group(&active_group_id);
            self.mark_sftp_activity_for_group(&active_group_id);
            self.set_workspace_page(WorkspacePage::Sftp, cx);
            self.focus_handle.focus(window, cx);
        } else {
            self.status = t!("open_ssh_tab_sftp").into();
            cx.notify();
        }
    }

    pub(crate) fn close_sftp_page(
        &mut self,
        group_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.group_has_active_sftp_transfer(&group_id) {
            self.close_sftp_page_now(group_id, window, cx);
            return;
        }

        self.show_sftp_transfer_close_dialog(group_id, window, cx);
    }

    pub(crate) fn confirm_sftp_close_with_shortcut(
        &mut self,
        group_id: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.active_dialog != Some(crate::app::DialogKind::SftpCloseConfirm)
            || self.sftp_close_confirm_group_id.as_deref() != Some(group_id)
        {
            return false;
        }

        let choice = self.config.sftp_transfer_close_behavior().to_string();
        if choice == "ask" {
            self.status = rust_i18n::t!("sftp_shortcut_choice_not_set").into();
            cx.notify();
            return true;
        }

        self.apply_sftp_transfer_close_choice(group_id.to_string(), &choice, false, window, cx);
        self.active_dialog = None;
        self.sftp_close_confirm_group_id = None;
        window.close_dialog(cx);
        true
    }

    pub(crate) fn apply_sftp_transfer_close_choice(
        &mut self,
        group_id: String,
        choice: &str,
        remember: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if remember {
            self.config.set_sftp_transfer_close_behavior(choice);
            if let Err(err) = self.config.save() {
                tracing::warn!("failed to save SFTP close preference: {err:#}");
            }
        }
        self.sftp_close_remember_choice = false;
        self.sftp_close_confirm_group_id = None;

        match choice {
            "keep_page_open" => {
                self.status = rust_i18n::t!("sftp_close_kept_for_transfer").into();
                cx.notify();
            }
            "background" => self.close_sftp_page_now(group_id, window, cx),
            "cancel_disconnect" => {
                self.release_sftp_handle_for_group(&group_id, true);
                self.close_sftp_page_now(group_id, window, cx);
            }
            _ => unreachable!("invalid SFTP close choice"),
        }
    }

    pub(crate) fn close_sftp_page_now(
        &mut self,
        group_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let was_active_sftp_page = self.workspace_page == WorkspacePage::Sftp
            && self.active_group.as_deref() == Some(group_id.as_str());

        if let Some(group) = self
            .tab_groups
            .iter_mut()
            .find(|group| group.id == group_id)
        {
            group.sftp_page_open = false;
        }

        if was_active_sftp_page {
            self.set_workspace_page(WorkspacePage::Terminal, cx);
            self.focus_handle.focus(window, cx);
        } else {
            cx.notify();
        }
    }

    pub(crate) fn request_active_system_snapshot(&mut self) {
        if !self.lifecycle.is_foreground() || !self.is_monitoring_visible() {
            return;
        }
        let Some(ref tab_id) = self.monitoring.system_tab_id.clone() else {
            return;
        };
        let Some(backend) = (|| {
            let tab = self.tabs.iter().find(|t| t.id == *tab_id)?;
            if !tab.connected {
                return None;
            }
            Some(tab.backend.clone())
        })() else {
            return;
        };
        if self.monitoring.remote_sample_in_flight {
            return;
        }
        self.monitoring.remote_sample_in_flight = true;
        if let Ok(backend) = backend.lock() {
            backend.send(BackendCommand::SampleMetrics);
        }
    }

    pub(crate) fn sync_active_sftp_to_shell_working_dir(&mut self, cx: &mut Context<Self>) {
        if self.pending_sftp_selection_path.is_some() {
            return;
        }
        if self.workspace_page != WorkspacePage::Sftp {
            return;
        }
        let Some(active_group_id) = self.active_group.clone() else {
            return;
        };
        let Some(tab_id) = self.group_primary_ssh_tab_id(&active_group_id) else {
            return;
        };
        let Some(tab_index) = self.tabs.iter().position(|tab| tab.id == tab_id) else {
            return;
        };
        let shell_working_dir = self.tabs[tab_index].shell_working_dir.clone();
        if let Some(path) = shell_working_dir {
            let already_current = self
                .active_sftp()
                .is_some_and(|sftp| sftp.current_path == path);
            if !already_current {
                self.navigate_sftp(path, cx);
            }
            return;
        }

        self.tabs[tab_index].send_backend(BackendCommand::QueryWorkingDirectory);
    }

    pub(crate) fn sync_sftp_to_shell_working_dir_for_tab(
        &mut self,
        tab_id: &str,
        path: &str,
        cx: &mut Context<Self>,
    ) {
        if self.pending_sftp_selection_path.is_some() {
            return;
        }
        if self.workspace_page != WorkspacePage::Sftp {
            return;
        }
        let Some(active_group_id) = self.active_group.clone() else {
            return;
        };
        if self.group_primary_ssh_tab_id(&active_group_id).as_deref() != Some(tab_id) {
            return;
        }
        let already_current = self
            .active_sftp()
            .is_some_and(|sftp| sftp.current_path == path);
        if !already_current {
            self.navigate_sftp(path.to_string(), cx);
        }
    }

    pub(crate) fn group_primary_ssh_tab_id(&self, group_id: &str) -> Option<String> {
        let group = self.tab_groups.iter().find(|group| group.id == group_id)?;
        if let Some(active_tab) = self.active_tab.as_ref()
            && group.pane_root.contains(active_tab)
            && self
                .tabs
                .iter()
                .any(|tab| tab.id == *active_tab && tab.kind == TabKind::Ssh && tab.connected)
        {
            return Some(active_tab.clone());
        }
        group.pane_root.tab_ids().into_iter().find_map(|tab_id| {
            self.tabs
                .iter()
                .find(|tab| tab.id == tab_id && tab.kind == TabKind::Ssh && tab.connected)
                .map(|tab| tab.id.clone())
        })
    }

    pub(crate) fn is_monitoring_visible(&self) -> bool {
        if !self.config.show_monitoring_dashboard() {
            return false;
        }
        match self.config.monitoring_position() {
            "Bottom" => true,
            "Sidebar" => !self.sidebar_collapsed,
            _ => false,
        }
    }

    pub(crate) fn terminal_ime_bounds_for_range(
        &self,
        range_utf16: Range<usize>,
        element_bounds: Bounds<Pixels>,
        cell_width: f32,
        line_height: f32,
    ) -> Option<Bounds<Pixels>> {
        let active_id = self.active_tab.as_ref()?;
        let snapshot = self.active_snapshot()?;
        let (row, col, range_start) = self
            .terminal_composition
            .as_ref()
            .filter(|composition| composition.tab_id == *active_id)
            .map(|composition| {
                (
                    composition.anchor_row,
                    composition.anchor_col,
                    range_utf16
                        .start
                        .min(composition.text.encode_utf16().count()),
                )
            })
            .or_else(|| {
                snapshot
                    .cursor
                    .map(|cursor| (cursor.row, cursor.col, range_utf16.start))
            })?;

        let row = row.min(snapshot.rows.saturating_sub(1));
        let col = col.min(snapshot.cols.saturating_sub(1));
        let x = element_bounds.origin.x
            + px(cell_width) * col as f32
            + px(cell_width) * range_start as f32;
        let y = element_bounds.origin.y + px(line_height) * row as f32;
        Some(Bounds::new(
            point(x, y),
            size(px(cell_width), px(line_height)),
        ))
    }

    pub(crate) fn remove_transfer(&mut self, transfer_id: &str, cx: &mut Context<Self>) {
        self.transfers.retain(|t| t.info.id != transfer_id);
        self.config.set_transfers(self.transfers.clone());
        cx.notify();
    }

    pub(crate) fn retry_connection_progress(&mut self, cx: &mut Context<Self>) {
        let Some(progress) = self.connection_progress.clone() else {
            return;
        };
        self.connection_progress = None;
        let mut retry_tabs = Vec::new();
        for (ix, tab) in self.tabs.iter().enumerate() {
            if !tab.connected && tab.session.is_some() && tab.id == progress.tab_id {
                retry_tabs.push((ix, tab.id.clone(), tab.session.clone().unwrap()));
            }
        }

        if retry_tabs.is_empty() {
            cx.notify();
            return;
        }

        for (ix, tab_id, session) in retry_tabs {
            self.tabs[ix].send_backend(crate::terminal::BackendCommand::Close);

            let backend = crate::backend::ssh::spawn_ssh_terminal(
                self.runtime_state.runtime.handle(),
                tab_id.clone(),
                session.clone(),
                self.tabs[ix].cols,
                self.tabs[ix].rows,
                self.runtime_state.events_tx.clone(),
            );

            self.tabs[ix].set_backend(backend);
            self.tabs[ix].connected = false;
            self.tabs[ix].status = "connecting".into();
            self.tabs[ix].disconnected_reason = None;
            self.tabs[ix].backend_initialized = false;

            if let Some(group) = self
                .tab_groups
                .iter()
                .find(|g| g.pane_root.contains(&tab_id))
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
        }

        self.connection_progress = Some(ConnectionProgress {
            tab_id: progress.tab_id.clone(),
            title: t!("connecting").into(),
            lines: vec![t!("starting_connection").into()],
            failed: false,
        });
        self.status = "ssh tabs retrying".into();
        cx.notify();
    }

    pub(crate) fn cancel_connection_progress(&mut self, cx: &mut Context<Self>) {
        if let Some(progress) = &self.connection_progress {
            let tab_id = progress.tab_id.clone();
            self.connection_progress = None;
            self.handle_tab_close(tab_id);
        }
        cx.notify();
    }

    pub(crate) fn save_layout_state(&self, window: &mut gpui::Window, cx: &gpui::App) {
        if self.is_layout_reset {
            tracing::info!("[ui] layout was reset, skipping save layout state.");
            return;
        }
        let current_bounds = window.window_bounds();
        let bounds = match current_bounds {
            gpui::WindowBounds::Fullscreen(b) => b,
            gpui::WindowBounds::Maximized(b) => b,
            gpui::WindowBounds::Windowed(b) => b,
        };
        let size = bounds.size;
        if size.width.as_f32() > 400.0 && size.height.as_f32() > 300.0 {
            tracing::info!("[ui] saving layout state...");
            let mut config = ConfigStore::load().unwrap_or_else(|_| ConfigStore::in_memory());
            let saved_bounds = match current_bounds {
                gpui::WindowBounds::Fullscreen(b) => crate::config::SavedWindowBounds::Fullscreen {
                    x: b.origin.x.into(),
                    y: b.origin.y.into(),
                    width: b.size.width.into(),
                    height: b.size.height.into(),
                },
                gpui::WindowBounds::Maximized(b) => {
                    let mut restore_bounds = (
                        b.origin.x.into(),
                        b.origin.y.into(),
                        b.size.width.into(),
                        b.size.height.into(),
                    );
                    if let Some(existing_bounds) = config.window_bounds() {
                        match existing_bounds {
                            crate::config::SavedWindowBounds::Windowed {
                                x,
                                y,
                                width,
                                height,
                            } => {
                                restore_bounds = (*x, *y, *width, *height);
                            }
                            crate::config::SavedWindowBounds::Maximized {
                                x,
                                y,
                                width,
                                height,
                            } => {
                                restore_bounds = (*x, *y, *width, *height);
                            }
                            _ => {}
                        }
                    }
                    crate::config::SavedWindowBounds::Maximized {
                        x: restore_bounds.0,
                        y: restore_bounds.1,
                        width: restore_bounds.2,
                        height: restore_bounds.3,
                    }
                }
                gpui::WindowBounds::Windowed(b) => crate::config::SavedWindowBounds::Windowed {
                    x: b.origin.x.into(),
                    y: b.origin.y.into(),
                    width: b.size.width.into(),
                    height: b.size.height.into(),
                },
            };
            let workspace_sizes: Vec<f32> = self
                .workspace_panels
                .read(cx)
                .sizes()
                .iter()
                .map(|s| s.into())
                .collect();
            let body_sizes: Vec<f32> = self
                .body_panels
                .read(cx)
                .sizes()
                .iter()
                .map(|s| s.into())
                .collect();

            config.set_layout_state(Some(saved_bounds), Some(workspace_sizes), Some(body_sizes));
            config.set_sidebar_collapsed(self.sidebar_collapsed);
            let _ = config.save();
        } else {
            tracing::warn!(
                "[ui] window size is too small ({:?}), skipping save layout state to prevent corrupting saved bounds.",
                size
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{WorkspacePage, WorkspaceTabDescriptor, active_workspace_tab_index_for};

    fn workspace_tab(group_id: Option<&str>, page: WorkspacePage) -> WorkspaceTabDescriptor {
        WorkspaceTabDescriptor {
            group_id: group_id.map(str::to_string),
            group_index: None,
            page,
        }
    }

    #[test]
    fn active_workspace_tab_index_uses_rendered_tab_order() {
        let tabs = vec![
            workspace_tab(Some("group-a"), WorkspacePage::Terminal),
            workspace_tab(Some("group-a"), WorkspacePage::Sftp),
            workspace_tab(Some("group-b"), WorkspacePage::Terminal),
            workspace_tab(None, WorkspacePage::Settings),
        ];

        assert_eq!(
            active_workspace_tab_index_for(&tabs, WorkspacePage::Terminal, Some("group-a"), true,),
            0
        );
        assert_eq!(
            active_workspace_tab_index_for(&tabs, WorkspacePage::Sftp, Some("group-a"), true),
            1
        );
        assert_eq!(
            active_workspace_tab_index_for(&tabs, WorkspacePage::Terminal, Some("group-b"), true,),
            2
        );
        assert_eq!(
            active_workspace_tab_index_for(&tabs, WorkspacePage::Settings, None, true),
            3
        );
    }
}
