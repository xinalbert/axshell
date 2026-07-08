use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::Rc,
};

use gpui::{Pixels, Point, SharedString, Size, point, px, size};
use gpui_component::scroll::ScrollbarHandle;

use crate::terminal;

#[derive(Clone, Debug)]
pub(crate) enum PaneLayout {
    Single(String),
    Horizontal(Vec<PaneLayout>, f32), // children, split_ratio (0.0-1.0)
    Vertical(Vec<PaneLayout>, f32),   // children, split_ratio (0.0-1.0)
}

#[derive(Clone)]
pub(crate) struct TabGroup {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) pane_root: PaneLayout,
    pub(crate) sftp: Option<crate::terminal::SftpUiState>,
}

impl PaneLayout {
    pub fn tab_ids(&self) -> Vec<&str> {
        match self {
            PaneLayout::Single(id) => vec![id.as_str()],
            PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _) => {
                children.iter().flat_map(|c| c.tab_ids()).collect()
            }
        }
    }

    pub fn contains(&self, tab_id: &str) -> bool {
        match self {
            PaneLayout::Single(id) => id == tab_id,
            PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _) => {
                children.iter().any(|c| c.contains(tab_id))
            }
        }
    }

    pub fn focused_tab_id(&self, path: &[usize]) -> Option<&str> {
        match self {
            PaneLayout::Single(id) if path.is_empty() => Some(id.as_str()),
            PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _) => {
                let (&first, rest) = path.split_first()?;
                children.get(first).and_then(|c| c.focused_tab_id(rest))
            }
            _ => None,
        }
    }

    pub fn replace_at(&mut self, path: &[usize], replacement: PaneLayout) {
        match (self, path) {
            (this @ PaneLayout::Single(_), []) => *this = replacement,
            (
                PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _),
                [first, rest @ ..],
            ) => {
                if let Some(child) = children.get_mut(*first) {
                    child.replace_at(rest, replacement);
                }
            }
            _ => {}
        }
    }

    pub fn remove_tab(&mut self, tab_id: &str) -> bool {
        match self {
            PaneLayout::Single(id) if id == tab_id => {
                *self = PaneLayout::Single(String::new());
                true
            }
            PaneLayout::Single(_) => false,
            PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _) => {
                for child in children.iter_mut() {
                    child.remove_tab(tab_id);
                }
                children.retain(|c| !matches!(c, PaneLayout::Single(id) if id.is_empty()));
                if children.is_empty() {
                    *self = PaneLayout::Single(String::new());
                } else if children.len() == 1 {
                    if let Some(replacement) = children.pop() {
                        *self = replacement;
                    }
                }
                true
            }
        }
    }

    #[allow(dead_code)]
    pub fn total_panes(&self) -> usize {
        match self {
            PaneLayout::Single(_) => 1,
            PaneLayout::Horizontal(children, _) | PaneLayout::Vertical(children, _) => {
                children.iter().map(|c| c.total_panes()).sum()
            }
        }
    }
}

pub(crate) struct TerminalScrollbarState {
    line_height: Pixels,
    total_lines: usize,
    viewport_lines: usize,
    display_offset: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TerminalFontMetrics {
    pub(crate) cell_width: f32,
    pub(crate) line_height: f32,
}

impl TerminalFontMetrics {
    pub(crate) fn fallback(font_size: f32) -> Self {
        Self {
            cell_width: (font_size * 0.646).max(6.0),
            line_height: (font_size * 1.385).max(font_size + 2.0),
        }
    }
}

#[derive(Clone, Default)]
pub(crate) struct TerminalScrollbarHandle {
    state: Rc<RefCell<Option<TerminalScrollbarState>>>,
    pub(crate) future_display_offset: Rc<Cell<Option<usize>>>,
}

impl TerminalScrollbarHandle {
    pub(crate) fn update(&self, snapshot: &terminal::RenderSnapshot, line_height: Pixels) {
        self.state.replace(Some(TerminalScrollbarState {
            line_height,
            total_lines: snapshot.history_size + snapshot.rows,
            viewport_lines: snapshot.rows,
            display_offset: snapshot.display_offset,
        }));
    }
}

impl ScrollbarHandle for TerminalScrollbarHandle {
    fn offset(&self) -> Point<Pixels> {
        let state_ref = self.state.borrow();
        let Some(state) = state_ref.as_ref() else {
            return point(px(0.), px(0.));
        };
        let scroll_offset = state
            .total_lines
            .saturating_sub(state.viewport_lines)
            .saturating_sub(state.display_offset);
        point(px(0.), -(scroll_offset as f32 * state.line_height))
    }

    fn set_offset(&self, offset: Point<Pixels>) {
        let state_ref = self.state.borrow();
        let Some(state) = state_ref.as_ref() else {
            return;
        };
        let offset_delta = (offset.y / state.line_height).round() as i32;
        let max_offset = state.total_lines.saturating_sub(state.viewport_lines);
        let display_offset = (max_offset as i32 + offset_delta).clamp(0, max_offset as i32);
        self.future_display_offset
            .set(Some(display_offset as usize));
    }

    fn content_size(&self) -> Size<Pixels> {
        let state_ref = self.state.borrow();
        let Some(state) = state_ref.as_ref() else {
            return size(px(0.), px(0.));
        };
        size(
            px(0.),
            state.total_lines.max(state.viewport_lines) as f32 * state.line_height,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DialogKind {
    SessionSelector,
    Transfers,
    NewSsh,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HoveredUrl {
    pub(crate) url: String,
    pub(crate) tab_id: String,
    pub(crate) cells: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
pub(crate) struct LocalFileEntry {
    pub(crate) name: String,
    pub(crate) full_path: String,
    pub(crate) is_dir: bool,
    pub(crate) size: u64,
    pub(crate) modified: u32,
}

#[derive(Clone, Default)]
pub(crate) struct LocalFileBrowserState {
    pub(crate) current_path: String,
    pub(crate) status: String,
    pub(crate) entries: Vec<LocalFileEntry>,
    pub(crate) selected_path: Option<String>,
    pub(crate) selected_entries: HashSet<String>,
}

#[derive(Clone)]
pub(crate) enum SelectorEntry {
    Local,
    NewSsh,
    Saved(String),
}

#[derive(Clone)]
pub(crate) struct ConnectionProgress {
    pub(crate) tab_id: String,
    pub(crate) title: SharedString,
    pub(crate) lines: Vec<SharedString>,
    pub(crate) failed: bool,
}

#[derive(Clone)]
pub(crate) struct SftpContextMenuState {
    pub(crate) remote_path: String,
    pub(crate) is_dir: bool,
    pub(crate) position: Point<Pixels>,
}
