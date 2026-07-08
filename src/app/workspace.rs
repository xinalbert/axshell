use std::ops::Range;

use gpui::{Bounds, Context, Pixels, Window, point, px, size};
use rust_i18n::t;

use crate::{
    AxShell,
    app::{ConnectionProgress, WorkspacePage, WorkspaceTabDescriptor},
    session::config::ConfigStore,
};

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
        match entry.page {
            WorkspacePage::Settings => {
                self.settings_page_open && self.workspace_page == WorkspacePage::Settings
            }
            page => {
                self.workspace_page == page
                    && entry.group_id.as_deref() == self.active_group.as_deref()
            }
        }
    }

    pub(crate) fn active_workspace_tab_index(&self, tabs: &[WorkspaceTabDescriptor]) -> usize {
        tabs.iter()
            .position(|entry| self.workspace_tab_selected(entry))
            .or_else(|| {
                self.active_group.as_ref().and_then(|group_id| {
                    tabs.iter().position(|entry| {
                        entry.page == WorkspacePage::Terminal
                            && entry.group_id.as_deref() == Some(group_id.as_str())
                    })
                })
            })
            .unwrap_or(0)
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
            self.search_active = false;
            self.search_query.clear();
            self.search_matches.clear();
            self.search_current = 0;
            self.search_target_tab = None;
            self.search_bar_bounds = None;
        }

        if page == WorkspacePage::Settings {
            crate::app::keybinding_recorder::unbind_all_workspace_keys(cx, &self.config);
            self.keybinds_suspended = true;
        }

        self.workspace_page = page;
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
            self.set_workspace_page(WorkspacePage::Terminal, cx);
            self.focus_handle.focus(window, cx);
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
        if !self.is_monitoring_visible() {
            return;
        }
        let Some(ref tab_id) = self.system_tab_id.clone() else {
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
        if self.remote_sample_in_flight {
            return;
        }
        self.remote_sample_in_flight = true;
        if let Ok(backend) = backend.lock() {
            backend.send(crate::terminal::BackendCommand::SampleMetrics);
        }
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
        let snapshot = self.active_snapshot()?;
        let cursor = snapshot.cursor?;
        let x = element_bounds.origin.x
            + px(cell_width) * cursor.col as f32
            + px(cell_width) * range_utf16.start as f32;
        let y = element_bounds.origin.y + px(line_height) * cursor.row as f32;
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
                self.runtime.handle(),
                tab_id.clone(),
                session.clone(),
                self.tabs[ix].cols,
                self.tabs[ix].rows,
                self.events_tx.clone(),
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

                if let Some(session) = group_session {
                    if let Some(old_handle) = self.sftp_handles.remove(&group_id) {
                        old_handle.close();
                    }
                    let sftp_handle = crate::sftp::spawn_sftp(
                        self.runtime.handle(),
                        group_id.clone(),
                        session,
                        self.events_tx.clone(),
                    );
                    self.sftp_handles.insert(group_id.clone(), sftp_handle);

                    if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == group_id) {
                        if let Some(sftp) = group.sftp.as_mut() {
                            sftp.status = rust_i18n::t!("sftp_connecting").to_string();
                        }
                    }
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
                gpui::WindowBounds::Fullscreen(b) => {
                    crate::session::config::SavedWindowBounds::Fullscreen {
                        x: b.origin.x.into(),
                        y: b.origin.y.into(),
                        width: b.size.width.into(),
                        height: b.size.height.into(),
                    }
                }
                gpui::WindowBounds::Maximized(b) => {
                    let mut restore_bounds = (
                        b.origin.x.into(),
                        b.origin.y.into(),
                        b.size.width.into(),
                        b.size.height.into(),
                    );
                    if let Some(existing_bounds) = config.window_bounds() {
                        match existing_bounds {
                            crate::session::config::SavedWindowBounds::Windowed {
                                x,
                                y,
                                width,
                                height,
                            } => {
                                restore_bounds = (*x, *y, *width, *height);
                            }
                            crate::session::config::SavedWindowBounds::Maximized {
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
                    crate::session::config::SavedWindowBounds::Maximized {
                        x: restore_bounds.0,
                        y: restore_bounds.1,
                        width: restore_bounds.2,
                        height: restore_bounds.3,
                    }
                }
                gpui::WindowBounds::Windowed(b) => {
                    crate::session::config::SavedWindowBounds::Windowed {
                        x: b.origin.x.into(),
                        y: b.origin.y.into(),
                        width: b.size.width.into(),
                        height: b.size.height.into(),
                    }
                }
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
