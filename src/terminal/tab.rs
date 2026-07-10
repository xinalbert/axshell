use crate::session::config::Session;
use crate::sftp::{PreviewData, RemoteEntry};
use alacritty_terminal::{
    grid::{Dimensions, Scroll},
    index::{Column, Line, Point, Side},
    selection::{Selection, SelectionRange, SelectionType},
    term::{Term, TermMode, cell::Cell, point_to_viewport, viewport_to_point},
    vte::ansi::Color as AnsiColor,
    vte::ansi::{CursorShape, Processor},
};
use std::hash::{Hash, Hasher};
use std::ops::Range;

use super::{
    backend::{BackendCommand, BackendEvent, BackendEventSender, BackendTx, TabKind},
    cwd::extract_shell_working_directory,
    listener::{TerminalListener, TerminalSize, new_term},
};

pub struct TerminalTab {
    pub id: String,
    pub title: String,
    pub kind: TabKind,
    pub status: String,
    pub connected: bool,
    pub disconnected_reason: Option<String>,
    /// Incremented each time the tab is reconnected. Used to ignore stale
    /// `BackendEvent::Closed` from the previous backend after a retry.
    pub backend_generation: u32,
    /// Set to `true` when the current backend sends its first `Output` or
    /// `Connected` event. Used to skip stale `Closed` events that arrive
    /// before the new backend has started producing output.
    pub backend_initialized: bool,
    pub session: Option<Session>,
    processor: Processor,
    term: Term<TerminalListener>,
    pub cols: u16,
    pub rows: u16,
    pub backend: std::sync::Arc<std::sync::Mutex<BackendTx>>,
    pub shell_working_dir: Option<String>,
    cwd_osc_buffer: Vec<u8>,
    events: BackendEventSender,
    pub scroll_pixel_y: f32,
    pub(crate) highlight_cache: std::cell::RefCell<
        Option<(
            Vec<RenderCell>,
            std::collections::HashMap<(i32, i32), gpui::Hsla>,
        )>,
    >,
    viewport_signature: u64,
}

#[derive(Clone, Copy)]
pub struct CursorState {
    pub row: usize,
    pub col: usize,
    pub shape: CursorShape,
}

#[derive(Clone, PartialEq)]
pub struct RenderCell {
    pub row: i32,
    pub col: i32,
    pub cell: Cell,
}

#[derive(Clone)]
pub struct RenderSnapshot {
    pub cells: Vec<RenderCell>,
    pub cursor: Option<CursorState>,
    pub selection: Option<ViewportSelection>,
    pub display_offset: usize,
    pub history_size: usize,
    pub rows: usize,
    pub cols: usize,
    pub highlights: std::collections::HashMap<(i32, i32), gpui::Hsla>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalComposition {
    pub tab_id: String,
    pub text: String,
    pub selected_range_utf16: Option<Range<usize>>,
    pub anchor_row: usize,
    pub anchor_col: usize,
}

#[derive(Clone)]
pub struct TerminalFrozenSelection {
    pub tab_id: String,
    pub selection: ViewportSelection,
    pub text: String,
}

#[derive(Clone, Copy, Default)]
pub struct TerminalMouseTrackingMode {
    pub mouse_tracking: bool,
    pub alternate_scroll: bool,
    pub sgr_mouse: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ViewportSignature {
    pub value: u64,
}

#[derive(Clone, Copy)]
pub struct ViewportSelection {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
    pub is_block: bool,
}

#[derive(Clone, Default)]
pub struct SftpUiState {
    pub current_path: String,
    pub status: String,
    pub entries: Vec<RemoteEntry>,
    pub has_more_entries: bool,
    pub loading_more_entries: bool,
    pub reached_entries_limit: bool,
    pub selected_path: Option<String>,
    pub preview: Option<PreviewData>,
    pub selected_entries: std::collections::HashSet<String>,
    pub home_dir: String,
}

impl TerminalTab {
    pub fn new_local(
        id: String,
        title: String,
        backend: BackendTx,
        events: BackendEventSender,
    ) -> Self {
        Self::new(
            id,
            title,
            TabKind::Local,
            "local shell".into(),
            backend,
            events,
        )
    }

    pub fn new_ssh(
        id: String,
        session: &Session,
        backend: BackendTx,
        events: BackendEventSender,
    ) -> Self {
        let mut tab = Self::new(
            id,
            session.name.clone(),
            TabKind::Ssh,
            format!(
                "connecting {}@{}:{}",
                session.user, session.host, session.port
            ),
            backend,
            events,
        );
        tab.session = Some(session.clone());
        tab.connected = false;
        tab
    }

    fn new(
        id: String,
        title: String,
        kind: TabKind,
        status: String,
        backend: BackendTx,
        events: BackendEventSender,
    ) -> Self {
        let shared_backend = std::sync::Arc::new(std::sync::Mutex::new(backend));
        let mut this = Self {
            id: id.clone(),
            title,
            kind,
            status,
            connected: matches!(kind, TabKind::Local),
            disconnected_reason: None,
            backend_generation: 0,
            backend_initialized: true,
            session: None,
            processor: Processor::new(),
            term: new_term(100, 30, shared_backend.clone(), id, events.clone()),
            cols: 100,
            rows: 30,
            backend: shared_backend,
            shell_working_dir: None,
            cwd_osc_buffer: Vec::new(),
            events: events.clone(),
            scroll_pixel_y: 0.0,
            highlight_cache: std::cell::RefCell::new(None),
            viewport_signature: 0,
        };
        this.refresh_viewport_signature();
        this
    }

    pub fn feed(&mut self, bytes: &[u8]) -> bool {
        self.capture_working_directory(bytes);
        self.processor.advance(&mut self.term, bytes);
        self.refresh_viewport_signature()
    }

    fn capture_working_directory(&mut self, bytes: &[u8]) {
        let mut buffered = Vec::with_capacity(self.cwd_osc_buffer.len() + bytes.len());
        buffered.extend_from_slice(&self.cwd_osc_buffer);
        buffered.extend_from_slice(bytes);
        let (path, pending) = extract_shell_working_directory(&buffered);
        self.cwd_osc_buffer = pending;

        let Some(path) = path else {
            return;
        };
        if path.is_empty() || self.shell_working_dir.as_deref() == Some(path.as_str()) {
            return;
        }
        self.shell_working_dir = Some(path.clone());
        // This runs while draining backend events on the UI thread, so it
        // must not wait for space in the same queue.
        let _ = self.events.try_send(BackendEvent::WorkingDirectoryChanged {
            tab_id: self.id.clone(),
            path,
        });
    }

    /// Send a command to the backend. Thread-safe via the shared Arc<Mutex>.
    pub fn send_backend(&self, command: BackendCommand) {
        if let Ok(backend) = self.backend.lock() {
            backend.send(command);
        }
    }

    /// Replace the backend with a new one. The `Term`'s internal listener
    /// shares the same `Arc`, so user input is automatically routed to the
    /// new backend. The previous backend is always asked to stop first.
    pub fn set_backend(&mut self, new_backend: BackendTx) {
        let old_backend = self
            .backend
            .lock()
            .ok()
            .map(|mut backend| std::mem::replace(&mut *backend, new_backend));
        if let Some(old_backend) = old_backend {
            old_backend.shutdown();
        }
    }

    pub fn shutdown_backend(&self) {
        if let Ok(backend) = self.backend.lock() {
            backend.shutdown();
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        let new_cols = cols.max(1);
        let new_rows = rows.max(1);
        if self.cols != new_cols || self.rows != new_rows {
            self.cols = new_cols;
            self.rows = new_rows;
            tracing::info!(
                "[ui] terminal resized to {}x{} (cols x rows)",
                self.cols,
                self.rows
            );
            self.term.resize(TerminalSize::new(self.cols, self.rows));
            self.refresh_viewport_signature();
            self.send_backend(BackendCommand::Resize { cols, rows });
        }
    }

    pub fn cursor_state(&self) -> Option<CursorState> {
        let content = self.term.renderable_content();
        if matches!(content.cursor.shape, CursorShape::Hidden) || content.display_offset > 0 {
            return None;
        }
        let row = content.cursor.point.line.0;
        if row < 0 {
            return None;
        }
        let row = row as usize;
        if row >= self.rows as usize {
            return None;
        }

        Some(CursorState {
            row,
            col: content.cursor.point.column.0,
            shape: content.cursor.shape,
        })
    }

    pub fn app_cursor_mode(&self) -> bool {
        self.term.mode().contains(TermMode::APP_CURSOR)
    }

    pub fn mouse_tracking_mode(&self) -> TerminalMouseTrackingMode {
        let mode = self.term.mode();
        TerminalMouseTrackingMode {
            mouse_tracking: mode.intersects(
                TermMode::MOUSE_REPORT_CLICK | TermMode::MOUSE_MOTION | TermMode::MOUSE_DRAG,
            ),
            alternate_scroll: mode.contains(TermMode::ALT_SCREEN | TermMode::ALTERNATE_SCROLL),
            sgr_mouse: mode.contains(TermMode::SGR_MOUSE),
        }
    }

    pub fn render_snapshot(&self) -> RenderSnapshot {
        let rows = self.rows;
        let cols = self.cols;
        let content = self.term.renderable_content();
        let display_offset = content.display_offset as i32;
        let mut cells = Vec::with_capacity((rows as usize) * (cols as usize));

        for indexed in content.display_iter {
            let line = indexed.point.line.0;
            let row = line + display_offset;
            if row < 0 {
                continue;
            }
            if row >= rows as i32 {
                continue;
            }

            let col = indexed.point.column.0 as i32;
            if col >= cols as i32 {
                continue;
            }

            cells.push(RenderCell {
                row,
                col,
                cell: indexed.cell.clone(),
            });
        }

        // Get highlights from cache or recompute, only if keyword_highlight is enabled.
        let is_enabled = crate::session::config::ConfigStore::load()
            .map(|c| c.keyword_highlight())
            .unwrap_or(false);

        let highlights = if is_enabled {
            let mut cache = self.highlight_cache.borrow_mut();
            let cache_valid = cache
                .as_ref()
                .is_some_and(|(cached_cells, _)| cached_cells == &cells);
            if cache_valid {
                cache.as_ref().unwrap().1.clone()
            } else {
                let computed = super::highlight::highlight_cells(&cells, rows as usize);
                *cache = Some((cells.clone(), computed.clone()));
                computed
            }
        } else {
            std::collections::HashMap::new()
        };

        RenderSnapshot {
            cells,
            cursor: self.cursor_state(),
            selection: viewport_selection_from_range(
                content.display_offset,
                self.rows as usize,
                self.cols as usize,
                &content.selection,
            ),
            display_offset: content.display_offset,
            history_size: self.term.grid().history_size(),
            rows: self.rows as usize,
            cols: self.cols as usize,
            highlights,
        }
    }

    /// Return `(grid_line_base, rows_data)` for the **entire** terminal buffer
    /// including scrollback history. `grid_line_base` is the grid line index of
    /// the first row (typically `-history_size`). Each entry in `rows_data` is
    /// a sorted `Vec<(col, char)>` for that row.
    pub fn full_grid_rows(&self) -> (i32, Vec<Vec<(i32, char)>>) {
        let grid = self.term.grid();
        let history = grid.history_size() as i32;
        let screen = grid.screen_lines() as i32;
        let total = history + screen;
        let cols = self.cols as i32;
        let start_line = -history;

        let mut rows_data: Vec<Vec<(i32, char)>> = Vec::with_capacity(total as usize);
        for line_idx in start_line..(start_line + total) {
            let line = Line(line_idx);
            let mut cells: Vec<(i32, char)> = Vec::new();
            for col_idx in 0..cols {
                let point = Point::new(line, Column(col_idx as usize));
                let c = grid[point].c;
                if c != ' ' && c != '\0' {
                    cells.push((col_idx, c));
                }
            }
            rows_data.push(cells);
        }
        (start_line, rows_data)
    }

    pub fn scroll_history(&mut self, delta: i32) {
        if delta != 0 {
            self.term.scroll_display(Scroll::Delta(delta));
            self.refresh_viewport_signature();
        }
    }

    pub fn scroll_up_by(&mut self, lines: usize) {
        if lines != 0 {
            self.term.scroll_display(Scroll::Delta(lines as i32));
            self.refresh_viewport_signature();
        }
    }

    pub fn scroll_down_by(&mut self, lines: usize) {
        if lines != 0 {
            self.term.scroll_display(Scroll::Delta(-(lines as i32)));
            self.refresh_viewport_signature();
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        self.term.scroll_display(Scroll::Bottom);
        self.refresh_viewport_signature();
    }

    #[allow(dead_code)]
    pub fn has_selection(&self) -> bool {
        self.term
            .selection_to_string()
            .is_some_and(|text| !text.is_empty())
    }

    pub fn selection_active(&self) -> bool {
        self.term
            .selection
            .as_ref()
            .is_some_and(|selection| !selection.is_empty())
    }

    pub fn clear_selection(&mut self) {
        self.term.selection = None;
    }

    pub fn selection_text(&self) -> Option<String> {
        self.term
            .selection_to_string()
            .filter(|text| !text.is_empty())
    }

    pub fn begin_selection(
        &mut self,
        row: usize,
        col: usize,
        side: Side,
        selection_type: SelectionType,
    ) {
        let point = viewport_to_point(
            self.term.grid().display_offset(),
            Point::new(row, Column(col)),
        );
        self.term.selection = Some(Selection::new(selection_type, point, side));
    }

    pub fn update_selection(&mut self, row: usize, col: usize, side: Side) {
        let point = viewport_to_point(
            self.term.grid().display_offset(),
            Point::new(row, Column(col)),
        );
        if let Some(selection) = self.term.selection.as_mut() {
            selection.update(point, side);
        }
    }

    pub fn paste_text(&mut self, text: &str) {
        let bracketed = self.term.mode().contains(TermMode::BRACKETED_PASTE);
        let paste_text = text
            .replace('\x1b', "")
            .replace("\r\n", "\r")
            .replace('\n', "\r");

        let mut bytes = Vec::new();
        if bracketed {
            bytes.extend_from_slice(b"\x1b[200~");
        }
        bytes.extend_from_slice(paste_text.as_bytes());
        if bracketed {
            bytes.extend_from_slice(b"\x1b[201~");
        }

        self.send_backend(BackendCommand::Input(bytes));
    }

    pub fn refresh_viewport_signature(&mut self) -> bool {
        let signature = self.compute_viewport_signature().value;
        let changed = self.viewport_signature != signature;
        self.viewport_signature = signature;
        changed
    }

    fn compute_viewport_signature(&self) -> ViewportSignature {
        let content = self.term.renderable_content();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        self.rows.hash(&mut hasher);
        self.cols.hash(&mut hasher);
        content.display_offset.hash(&mut hasher);
        self.term.grid().history_size().hash(&mut hasher);

        if content.display_offset == 0 {
            if let Some(cursor) = self.cursor_state() {
                cursor.row.hash(&mut hasher);
                cursor.col.hash(&mut hasher);
                std::mem::discriminant(&cursor.shape).hash(&mut hasher);
            } else {
                0usize.hash(&mut hasher);
            }
        } else {
            usize::MAX.hash(&mut hasher);
        }

        for indexed in content.display_iter {
            let cell = indexed.cell;
            let point = indexed.point;

            point.line.0.hash(&mut hasher);
            point.column.0.hash(&mut hasher);
            cell.c.hash(&mut hasher);
            hash_ansi_color(cell.fg, &mut hasher);
            hash_ansi_color(cell.bg, &mut hasher);
            cell.flags.bits().hash(&mut hasher);
            if let Some(zerowidth) = cell.zerowidth() {
                zerowidth.hash(&mut hasher);
            }
        }

        ViewportSignature {
            value: hasher.finish(),
        }
    }
}

fn viewport_selection_from_range(
    display_offset: usize,
    rows: usize,
    cols: usize,
    selection: &Option<SelectionRange>,
) -> Option<ViewportSelection> {
    let SelectionRange {
        start,
        end,
        is_block,
    } = selection.as_ref().copied()?;

    let top_point = viewport_to_point(display_offset, Point::new(0, Column(0)));
    let bottom_point = viewport_to_point(
        display_offset,
        Point::new(rows.saturating_sub(1), Column(0)),
    );

    let top_line = top_point.line;
    let bottom_line = bottom_point.line;

    let start_vp = if start.line < top_line {
        Point::new(0, Column(0))
    } else if start.line > bottom_line {
        Point::new(rows.saturating_sub(1), Column(cols.saturating_sub(1)))
    } else {
        point_to_viewport(display_offset, start).unwrap_or(Point::new(0, Column(0)))
    };

    let end_vp = if end.line < top_line {
        Point::new(0, Column(0))
    } else if end.line > bottom_line {
        Point::new(rows.saturating_sub(1), Column(cols.saturating_sub(1)))
    } else {
        point_to_viewport(display_offset, end).unwrap_or(Point::new(
            rows.saturating_sub(1),
            Column(cols.saturating_sub(1)),
        ))
    };

    Some(ViewportSelection {
        start_row: start_vp.line,
        start_col: start_vp.column.0,
        end_row: end_vp.line,
        end_col: end_vp.column.0,
        is_block,
    })
}

fn hash_ansi_color<H: Hasher>(color: AnsiColor, state: &mut H) {
    match color {
        AnsiColor::Named(named) => {
            0u8.hash(state);
            std::mem::discriminant(&named).hash(state);
        }
        AnsiColor::Spec(rgb) => {
            1u8.hash(state);
            rgb.r.hash(state);
            rgb.g.hash(state);
            rgb.b.hash(state);
        }
        AnsiColor::Indexed(index) => {
            2u8.hash(state);
            index.hash(state);
        }
    }
}
