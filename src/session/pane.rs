use gpui::{Context, MouseDownEvent, MouseMoveEvent, Window};
use uuid::Uuid;

use crate::{
    AxShell, PaneLayout,
    app::WorkspacePage,
    app::constants::{DEFAULT_COLS, DEFAULT_ROWS},
    backend::{local, ssh},
    terminal::{TabKind, TerminalTab},
};

impl AxShell {
    pub(crate) fn split_current_pane(&mut self, direction: &str, cx: &mut Context<Self>) {
        tracing::info!(
            "[split] direction={} pane_root={:?} focused_path={:?} active_tab={:?} tabs={}",
            direction,
            self.pane_root,
            self.focused_pane_path,
            self.active_tab,
            self.tabs.len(),
        );
        let current_id = match self.pane_root.focused_tab_id(&self.focused_pane_path) {
            Some(id) if !id.is_empty() => id.to_string(),
            _ => return,
        };
        let (current_kind, current_session) = match self.tabs.iter().find(|t| t.id == current_id) {
            Some(tab) => (tab.kind, tab.session.clone()),
            None => return,
        };
        let new_id = Uuid::new_v4().to_string();
        let mut tab = match current_kind {
            TabKind::Local => {
                match local::spawn_local_terminal(
                    new_id.clone(),
                    DEFAULT_COLS,
                    DEFAULT_ROWS,
                    self.events_tx.clone(),
                ) {
                    Ok(backend) => TerminalTab::new_local(
                        new_id.clone(),
                        "Local".into(),
                        backend,
                        self.events_tx.clone(),
                    ),
                    Err(err) => {
                        self.status = format!("failed to split: {err:#}").into();
                        cx.notify();
                        return;
                    }
                }
            }
            TabKind::Ssh => {
                let Some(session) = current_session else {
                    self.status = "cannot split: no session info".into();
                    cx.notify();
                    return;
                };
                let backend = ssh::spawn_ssh_terminal(
                    self.runtime.handle(),
                    new_id.clone(),
                    session.clone(),
                    DEFAULT_COLS,
                    DEFAULT_ROWS,
                    self.events_tx.clone(),
                );
                let sftp_handle = crate::sftp::spawn_sftp(
                    self.runtime.handle(),
                    new_id.clone(),
                    session.clone(),
                    self.events_tx.clone(),
                );
                self.sftp_handles.insert(new_id.clone(), sftp_handle);
                TerminalTab::new_ssh(new_id.clone(), &session, backend, self.events_tx.clone())
            }
        };
        tab.resize(DEFAULT_COLS, DEFAULT_ROWS);
        self.tabs.push(tab);

        let current_pane = PaneLayout::Single(current_id);
        let new_pane = PaneLayout::Single(new_id.clone());

        let split_layout = match direction {
            "left" | "right" => {
                let children = match direction {
                    "left" => vec![new_pane, current_pane],
                    _ => vec![current_pane, new_pane],
                };
                PaneLayout::Vertical(children, 0.5)
            }
            "up" | "down" => {
                let children = match direction {
                    "up" => vec![new_pane, current_pane],
                    _ => vec![current_pane, new_pane],
                };
                PaneLayout::Horizontal(children, 0.5)
            }
            _ => return,
        };

        self.pane_root
            .replace_at(&self.focused_pane_path, split_layout);
        self.sync_pane_root_to_group();
        let parent_path = self.focused_pane_path.clone();
        let mut new_full_path = parent_path;
        if direction == "right" || direction == "down" {
            new_full_path.push(1);
        } else {
            new_full_path.push(0);
        }
        self.focused_pane_path = new_full_path;
        self.active_tab = Some(new_id);
        self.status = "pane split".into();
        tracing::info!(
            "[split] DONE: pane_root={:?} focused_path={:?} active_tab={:?} tabs={}",
            self.pane_root,
            self.focused_pane_path,
            self.active_tab,
            self.tabs.len(),
        );
        cx.notify();
    }

    pub(crate) fn focus_adjacent_pane(&mut self, direction: &str) {
        if self.focused_pane_path.is_empty() {
            return;
        }
        let path = self.focused_pane_path.clone();
        if let Some(new_path) = Self::find_adjacent_pane(&self.pane_root, &path, direction) {
            self.focused_pane_path = new_path;
            if let Some(id) = self.pane_root.focused_tab_id(&self.focused_pane_path) {
                let id_owned = id.to_string();
                let changed = self.active_tab.as_deref() != Some(id_owned.as_str());
                self.active_tab = Some(id_owned);
                if changed && self.search_active {
                    self.search_query.clear();
                    self.search_matches.clear();
                    self.search_current = 0;
                    self.search_target_tab = None;
                }
            }
        }
    }

    fn first_leaf_path(layout: &PaneLayout) -> Vec<usize> {
        match layout {
            PaneLayout::Single(_) => vec![],
            PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _) => {
                let mut path = vec![0];
                path.extend(Self::first_leaf_path(&children[0]));
                path
            }
        }
    }

    fn leaf_at_index(layout: &PaneLayout, index: usize) -> Vec<usize> {
        match layout {
            PaneLayout::Single(_) => vec![],
            PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _) => {
                if children.is_empty() {
                    return vec![];
                }
                let i = index.min(children.len() - 1);
                let mut path = vec![i];
                path.extend(Self::first_leaf_path(&children[i]));
                path
            }
        }
    }

    fn find_adjacent_pane(
        layout: &PaneLayout,
        path: &[usize],
        direction: &str,
    ) -> Option<Vec<usize>> {
        if path.is_empty() {
            return None;
        }
        match layout {
            PaneLayout::Single(_) => None,
            PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _) => {
                let is_horizontal = matches!(layout, PaneLayout::Horizontal(_, _));
                let idx = path[0];

                let vert = direction == "up" || direction == "down";
                let horiz = direction == "left" || direction == "right";
                let moves_in_this_split = (vert && is_horizontal) || (horiz && !is_horizontal);

                if path.len() == 1 {
                    if moves_in_this_split {
                        let delta: i32 = if direction == "up" || direction == "left" {
                            -1
                        } else {
                            1
                        };
                        let new_idx = idx as i32 + delta;
                        if new_idx >= 0 && (new_idx as usize) < children.len() {
                            let mut path = vec![new_idx as usize];
                            path.extend(Self::first_leaf_path(&children[new_idx as usize]));
                            Some(path)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else if let Some(mut child_path) =
                    Self::find_adjacent_pane(&children[idx], &path[1..], direction)
                {
                    child_path.insert(0, idx);
                    Some(child_path)
                } else if moves_in_this_split {
                    let delta: i32 = if direction == "up" || direction == "left" {
                        -1
                    } else {
                        1
                    };
                    let new_idx = idx as i32 + delta;
                    if new_idx >= 0 && (new_idx as usize) < children.len() {
                        let inner_idx = *path.get(1).unwrap_or(&0);
                        let mut path = vec![new_idx as usize];
                        path.extend(Self::leaf_at_index(&children[new_idx as usize], inner_idx));
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    pub(crate) fn activate_group_page(
        &mut self,
        group_id: String,
        page: WorkspacePage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(current_group_id) = self.active_group.clone() {
            if current_group_id != group_id {
                if let Some(group) = self
                    .tab_groups
                    .iter_mut()
                    .find(|g| g.id == current_group_id)
                {
                    group.pane_root = self.pane_root.clone();
                }
            }
        }

        let switching_group = self.active_group.as_deref() != Some(group_id.as_str());

        if let Some((pane_root, has_sftp)) = self
            .tab_groups
            .iter()
            .find(|g| g.id == group_id)
            .map(|group| (group.pane_root.clone(), group.sftp.is_some()))
        {
            if switching_group {
                self.pane_root = pane_root.clone();
                self.active_group = Some(group_id.clone());
                let ids = pane_root.tab_ids();
                if let Some(&first_id) = ids.first() {
                    self.active_tab = Some(first_id.to_string());
                    self.focus_pane_with_id(first_id.to_string());
                }
            }

            let target_page = if page == WorkspacePage::Sftp && !has_sftp {
                WorkspacePage::Terminal
            } else {
                page
            };

            if target_page == WorkspacePage::Terminal {
                self.focus_handle.focus(window, cx);
            }

            self.set_workspace_page(target_page, cx);
            self.sync_system_tab_to_active_group();
            cx.notify();
        }
    }
    pub(crate) fn sync_pane_root_to_group(&mut self) {
        if let Some(group_id) = self.active_group.clone() {
            if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == group_id) {
                group.pane_root = self.pane_root.clone();
            }
        }
    }

    pub(crate) fn sync_system_tab_to_active_group(&mut self) {
        let mut group_ssh_tabs = vec![];
        if let Some(group_id) = &self.active_group {
            if let Some(group) = self.tab_groups.iter().find(|g| g.id == *group_id) {
                let ids = group.pane_root.tab_ids();
                for id in ids {
                    if let Some(tab) = self.tabs.iter().find(|t| t.id == *id) {
                        if tab.kind == TabKind::Ssh && tab.connected {
                            group_ssh_tabs.push(tab.id.clone());
                        }
                    }
                }
            }
        }

        let is_current_valid = self
            .system_tab_id
            .as_ref()
            .is_some_and(|id| group_ssh_tabs.contains(id));

        if !is_current_valid {
            let new_id = group_ssh_tabs.into_iter().next();
            if self.system_tab_id != new_id {
                self.system_tab_id = new_id;
                self.cpu_history.clear();
                self.net_rx_history.clear();
                self.net_tx_history.clear();
                self.remote_sample_in_flight = false;
                if self.system_tab_id.is_none() {
                    self.system_status = Some("monitored session closed".to_string().into());
                } else {
                    self.system_status = None;
                }
                self.request_active_system_snapshot();
            }
        }
    }

    pub(crate) fn start_drag_split(
        &mut self,
        parent_path: Vec<usize>,
        child_index: usize,
        event: &MouseDownEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.dragging_splitter = Some((parent_path, child_index));
        self.drag_split_origin = Some(event.position);
    }

    pub(crate) fn on_split_drag_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        let Some((ref parent_path, child_idx)) = self.dragging_splitter.clone() else {
            return;
        };
        let Some(origin) = self.drag_split_origin else {
            return;
        };
        let total = window.viewport_size();
        let is_horizontal = Self::is_layout_horizontal_at(&self.pane_root, parent_path);
        let delta: f32 = if is_horizontal {
            (event.position.y - origin.y).into()
        } else {
            (event.position.x - origin.x).into()
        };
        let total_size: f32 = if is_horizontal {
            total.height.into()
        } else {
            total.width.into()
        };
        if delta.abs() < 5.0 {
            return;
        }
        let ratio_delta = delta / total_size;
        Self::adjust_split_ratio(&mut self.pane_root, parent_path, child_idx, ratio_delta);
        self.drag_split_origin = Some(event.position);
        self.sync_pane_root_to_group();
    }

    pub(crate) fn end_drag_split(&mut self) {
        self.dragging_splitter = None;
        self.drag_split_origin = None;
    }

    fn is_layout_horizontal_at(layout: &PaneLayout, path: &[usize]) -> bool {
        match (layout, path) {
            (PaneLayout::Horizontal(_, _), []) => true,
            (PaneLayout::Vertical(_, _), []) => false,
            (
                PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _),
                [first, rest @ ..],
            ) => children
                .get(*first)
                .is_some_and(|c| Self::is_layout_horizontal_at(c, rest)),
            _ => false,
        }
    }

    fn adjust_split_ratio(layout: &mut PaneLayout, path: &[usize], _child_idx: usize, delta: f32) {
        if let PaneLayout::Horizontal(children, ratio) | PaneLayout::Vertical(children, ratio) =
            layout
        {
            if path.is_empty() {
                *ratio = (*ratio + delta).clamp(0.1, 0.9);
            } else {
                let (&first, rest) = path.split_first().unwrap();
                if let Some(child) = children.get_mut(first) {
                    Self::adjust_split_ratio(child, rest, _child_idx, delta);
                }
            }
        }
    }

    pub(crate) fn focus_pane_with_id(&mut self, tab_id: String) {
        fn find_path(layout: &PaneLayout, target: &str, path: &mut Vec<usize>) -> bool {
            match layout {
                PaneLayout::Single(id) => id == target,
                PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _) => {
                    for (i, child) in children.iter().enumerate() {
                        path.push(i);
                        if find_path(child, target, path) {
                            return true;
                        }
                        path.pop();
                    }
                    false
                }
            }
        }
        let mut path = Vec::new();
        if find_path(&self.pane_root, &tab_id, &mut path) {
            let changed = self.active_tab.as_deref() != Some(tab_id.as_str());
            if changed && let Some(previous_id) = self.active_tab.clone() {
                self.flush_terminal_output_for_tab(&previous_id);
            }
            self.focused_pane_path = path;
            self.active_tab = Some(tab_id);
            if changed && self.search_active {
                self.search_query.clear();
                self.search_matches.clear();
                self.search_current = 0;
                self.search_target_tab = None;
            }
        }
    }
}
