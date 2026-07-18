use alacritty_terminal::{
    term::cell::Flags,
    vte::ansi::{Color as AnsiColor, CursorShape, NamedColor},
};
use gpui::{
    App, Bounds, Element, ElementId, Entity, FocusHandle, Font, FontStyle, FontWeight,
    GlobalElementId, Hsla, InputHandler, IntoElement, LayoutId, Pixels, Point, Rgba, ShapedLine,
    SharedString, StrikethroughStyle, TextRun, TextStyle, UTF16Selection, UnderlineStyle, Window,
    fill, point, px, relative, rgb,
};
use gpui_component::ActiveTheme as _;
use std::{collections::HashMap, rc::Rc};

use crate::terminal::custom_blocks::{is_custom_block_supported, paint_custom_block};
use crate::terminal::{
    RenderSnapshot, TerminalComposition, TerminalFrozenSelection, ViewportSelection,
};
use crate::{
    AxShell,
    app::{HoveredUrl, terminal_link_visual_active},
};

#[derive(Clone, Copy)]
struct TerminalMetrics {
    cell_width: Pixels,
    line_height: Pixels,
}

#[derive(Clone)]
struct LayoutRect {
    row: i32,
    col: i32,
    cells: usize,
    color: Hsla,
}

impl LayoutRect {
    fn paint(&self, origin: Point<Pixels>, metrics: TerminalMetrics, window: &mut Window) {
        self.paint_with_row_offset(origin, metrics, 0, window);
    }

    fn paint_with_row_offset(
        &self,
        origin: Point<Pixels>,
        metrics: TerminalMetrics,
        row_offset: i32,
        window: &mut Window,
    ) {
        let position = point(
            origin.x + metrics.cell_width * self.col as f32,
            origin.y + metrics.line_height * (self.row + row_offset) as f32,
        );
        let size = gpui::size(metrics.cell_width * self.cells as f32, metrics.line_height);
        window.paint_quad(fill(Bounds::new(position, size), self.color));
    }
}

#[derive(Clone)]
struct LayoutUnderline {
    row: i32,
    col: i32,
    cells: usize,
    color: Hsla,
}

impl LayoutUnderline {
    fn paint(&self, origin: Point<Pixels>, metrics: TerminalMetrics, window: &mut Window) {
        self.paint_with_row_offset(origin, metrics, 0, window);
    }

    fn paint_with_row_offset(
        &self,
        origin: Point<Pixels>,
        metrics: TerminalMetrics,
        row_offset: i32,
        window: &mut Window,
    ) {
        let thickness = px(1.0);
        let position = point(
            origin.x + metrics.cell_width * self.col as f32,
            origin.y + metrics.line_height * (self.row + row_offset) as f32 + metrics.line_height
                - thickness,
        );
        let size = gpui::size(metrics.cell_width * self.cells as f32, thickness);
        window.paint_quad(fill(Bounds::new(position, size), self.color));
    }
}

#[derive(Clone)]
struct BatchedTextRun {
    row: i32,
    col: i32,
    cell_count: usize,
    text: String,
    text_hash: u64,
    style: TextRun,
    font_size: Pixels,
}

impl BatchedTextRun {
    fn new(row: i32, col: i32, ch: char, style: TextRun, font_size: Pixels) -> Self {
        let text = ch.to_string();
        Self {
            row,
            col,
            cell_count: 1,
            text_hash: fnv1a_hash(&text),
            text,
            style,
            font_size,
        }
    }

    fn can_append(&self, other: &TextRun, row: i32, col: i32) -> bool {
        self.row == row
            && self.col + self.cell_count as i32 == col
            && self.style.font == other.font
            && self.style.color == other.color
            && self.style.background_color == other.background_color
            && self.style.underline == other.underline
            && self.style.strikethrough == other.strikethrough
    }

    fn append(&mut self, ch: char, zerowidth: Option<&[char]>) {
        self.text.push(ch);
        self.text_hash = fnv1a_update(self.text_hash, ch.encode_utf8(&mut [0; 4]).as_bytes());
        self.cell_count += 1;
        self.style.len += ch.len_utf8();
        if let Some(chars) = zerowidth {
            for c in chars {
                self.append_zerowidth(*c);
            }
        }
    }

    fn append_zerowidth(&mut self, ch: char) {
        self.text.push(ch);
        self.text_hash = fnv1a_update(self.text_hash, ch.encode_utf8(&mut [0; 4]).as_bytes());
        self.style.len += ch.len_utf8();
    }

    fn into_shaped(self, cell_width: Pixels, window: &mut Window) -> ShapedTextRun {
        let shaped = window.text_system().shape_line_by_hash(
            self.text_hash,
            self.text.len(),
            self.font_size,
            std::slice::from_ref(&self.style),
            Some(cell_width),
            || self.text.clone().into(),
        );

        ShapedTextRun {
            row: self.row,
            col: self.col,
            shaped,
        }
    }
}

const FNV1A_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV1A_PRIME: u64 = 0x100000001b3;

fn fnv1a_hash(text: &str) -> u64 {
    fnv1a_update(FNV1A_OFFSET_BASIS, text.as_bytes())
}

fn fnv1a_update(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV1A_PRIME);
    }
    hash
}

#[derive(Clone, Copy)]
struct CursorLayout {
    row: usize,
    col: usize,
    shape: CursorShape,
    color: Hsla,
}

pub struct TerminalElement {
    view: Entity<AxShell>,
    focus_handle: FocusHandle,
    snapshot: Rc<RenderSnapshot>,
    composition: Option<TerminalComposition>,
    frozen_selection: Option<TerminalFrozenSelection>,
    font_family: SharedString,
    effective_font_family: Option<SharedString>,
    font_size: Pixels,
    line_height: Pixels,
    cell_width: Pixels,
    tab_id: String,
    search_highlights: Option<std::collections::HashMap<(i32, i32), Hsla>>,
}

pub struct PrepaintState {
    bounds: Bounds<Pixels>,
    metrics: TerminalMetrics,
    selection_rects: Vec<LayoutRect>,
    grid_rows: Vec<Rc<RowLayout>>,
    cursor: Option<CursorLayout>,
    underlines: Vec<LayoutUnderline>,
}

#[derive(Clone, Default)]
struct RowLayout {
    rects: Vec<LayoutRect>,
    runs: Vec<ShapedTextRun>,
    custom_blocks: Vec<LayoutCustomBlock>,
}

#[derive(Clone)]
struct ShapedTextRun {
    row: i32,
    col: i32,
    shaped: ShapedLine,
}

impl ShapedTextRun {
    fn paint_with_row_offset(
        &self,
        origin: Point<Pixels>,
        metrics: TerminalMetrics,
        row_offset: i32,
        window: &mut Window,
        cx: &mut App,
    ) {
        let pos = point(
            origin.x + metrics.cell_width * self.col as f32,
            origin.y + metrics.line_height * (self.row + row_offset) as f32,
        );

        self.shaped
            .paint(
                pos,
                metrics.line_height,
                gpui::TextAlign::Left,
                None,
                window,
                cx,
            )
            .ok();
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ColorKey {
    h: u32,
    s: u32,
    l: u32,
    a: u32,
}

impl From<Hsla> for ColorKey {
    fn from(color: Hsla) -> Self {
        Self {
            h: color.h.to_bits(),
            s: color.s.to_bits(),
            l: color.l.to_bits(),
            a: color.a.to_bits(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct GridStyleKey {
    font_family: SharedString,
    font_size: u32,
    cell_width: u32,
    font_brightness: u32,
    foreground: ColorKey,
    background: ColorKey,
    muted_foreground: ColorKey,
    primary: ColorKey,
}

#[derive(Clone, PartialEq, Eq)]
struct SelectionKey {
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
    is_block: bool,
}

impl From<ViewportSelection> for SelectionKey {
    fn from(selection: ViewportSelection) -> Self {
        Self {
            start_row: selection.start_row,
            start_col: selection.start_col,
            end_row: selection.end_row,
            end_col: selection.end_col,
            is_block: selection.is_block,
        }
    }
}

// IME and local input composition are painted after cached grid rows.
#[derive(Clone, PartialEq, Eq)]
struct GridLayoutKey {
    style: GridStyleKey,
    selection: Option<SelectionKey>,
}

#[derive(Clone, Default)]
struct RowHighlights {
    keyword: Vec<(i32, Hsla)>,
    search: Vec<(i32, Hsla)>,
}

#[derive(Clone, PartialEq, Eq)]
struct RowHighlightKey {
    keyword: Vec<(i32, ColorKey)>,
    search: Vec<(i32, ColorKey)>,
}

impl From<&RowHighlights> for RowHighlightKey {
    fn from(highlights: &RowHighlights) -> Self {
        Self {
            keyword: highlights
                .keyword
                .iter()
                .map(|(col, color)| (*col, (*color).into()))
                .collect(),
            search: highlights
                .search
                .iter()
                .map(|(col, color)| (*col, (*color).into()))
                .collect(),
        }
    }
}

struct CachedRowLayout {
    source: Rc<super::RenderRow>,
    highlights: RowHighlightKey,
    layout: Rc<RowLayout>,
}

struct GridLayoutCache {
    key: GridLayoutKey,
    rows: Vec<CachedRowLayout>,
}

#[derive(Clone)]
struct LayoutCustomBlock {
    c: char,
    row: i32,
    col: i32,
    cells: usize,
    color: Hsla,
}

struct TerminalInputHandler {
    view: Entity<AxShell>,
    element_bounds: Bounds<Pixels>,
    cell_width: f32,
    line_height: f32,
}

impl InputHandler for TerminalInputHandler {
    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        cx: &mut App,
    ) -> Option<UTF16Selection> {
        self.view
            .read(cx)
            .terminal_accepts_text_input()
            .then_some(UTF16Selection {
                range: 0..0,
                reversed: false,
            })
    }

    fn marked_text_range(
        &mut self,
        _window: &mut Window,
        cx: &mut App,
    ) -> Option<std::ops::Range<usize>> {
        self.view.read(cx).terminal_marked_text_range()
    }

    fn text_for_range(
        &mut self,
        _range_utf16: std::ops::Range<usize>,
        _adjusted_range: &mut Option<std::ops::Range<usize>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<String> {
        None
    }

    fn replace_text_in_range(
        &mut self,
        _replacement_range: Option<std::ops::Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.view.update(cx, |view, cx| {
            view.commit_terminal_ime_text(text, window, cx);
        });
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range_utf16: Option<std::ops::Range<usize>>,
        new_text: &str,
        new_selected_range: Option<std::ops::Range<usize>>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.view.update(cx, |view, cx| {
            view.set_terminal_marked_text(new_text.to_string(), new_selected_range, window, cx);
        });
    }

    fn unmark_text(&mut self, window: &mut Window, cx: &mut App) {
        self.view.update(cx, |view, cx| {
            view.clear_terminal_marked_text(window, cx);
        });
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: std::ops::Range<usize>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Option<Bounds<Pixels>> {
        self.view.read(cx).terminal_ime_bounds_for_range(
            range_utf16,
            self.element_bounds,
            self.cell_width,
            self.line_height,
        )
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<usize> {
        None
    }

    fn accepts_text_input(&mut self, _window: &mut Window, cx: &mut App) -> bool {
        self.view.read(cx).terminal_accepts_text_input()
    }

    fn apple_press_and_hold_enabled(&mut self) -> bool {
        false
    }

    fn prefers_ime_for_printable_keys(&mut self, _window: &mut Window, cx: &mut App) -> bool {
        self.view.read(cx).terminal_accepts_text_input()
    }
}

impl TerminalElement {
    pub fn new(
        view: Entity<AxShell>,
        focus_handle: FocusHandle,
        snapshot: Rc<RenderSnapshot>,
        composition: Option<TerminalComposition>,
        frozen_selection: Option<TerminalFrozenSelection>,
        font_family: SharedString,
        font_size: Pixels,
        line_height: Pixels,
        cell_width: Pixels,
        tab_id: String,
        search_highlights: Option<std::collections::HashMap<(i32, i32), Hsla>>,
    ) -> Self {
        Self {
            view,
            focus_handle,
            snapshot,
            composition,
            frozen_selection,
            font_family,
            effective_font_family: None,
            font_size,
            line_height,
            cell_width,
            tab_id,
            search_highlights,
        }
    }

    fn base_text_style(&self, cx: &App) -> TextStyle {
        TextStyle {
            color: cx.theme().foreground,
            font_family: self.active_font_family(),
            font_size: self.font_size.into(),
            line_height: self.line_height.into(),
            ..Default::default()
        }
    }

    fn active_font_family(&self) -> SharedString {
        self.effective_font_family
            .clone()
            .unwrap_or_else(|| self.font_family.clone())
    }

    fn measured_metrics(&mut self, window: &mut Window, cx: &mut App) -> TerminalMetrics {
        let font_family =
            terminal_monospace_font_family(window, self.font_family.clone(), self.font_size);
        ensure_terminal_font_family_loaded(&font_family, cx);
        self.effective_font_family = Some(font_family.clone());
        let font = Font {
            family: font_family,
            ..Font::default()
        };
        let font_id = window.text_system().resolve_font(&font);
        let measured_width = window
            .text_system()
            .ch_advance(font_id, self.font_size)
            .or_else(|_| window.text_system().em_advance(font_id, self.font_size))
            .map(|width| width.as_f32())
            .unwrap_or_else(|_| self.cell_width.as_f32())
            .max(6.0);
        TerminalMetrics {
            cell_width: px(measured_width),
            line_height: self.line_height,
        }
    }

    fn cell_run_style(
        &self,
        cell: &alacritty_terminal::term::cell::Cell,
        font_brightness: f32,
        cx: &App,
    ) -> TextRun {
        let mut fg = color_to_hsla(cell.fg, true, font_brightness, cx);
        let mut bg = color_to_hsla(cell.bg, false, font_brightness, cx);
        if cell.flags.contains(Flags::INVERSE) {
            std::mem::swap(&mut fg, &mut bg);
        }
        if cell.flags.contains(Flags::DIM) {
            fg.a *= 0.7;
        }

        let underline = cell
            .flags
            .intersects(Flags::ALL_UNDERLINES)
            .then(|| UnderlineStyle {
                color: Some(fg),
                thickness: px(1.0),
                wavy: cell.flags.contains(Flags::UNDERCURL),
            });
        let strikethrough = cell
            .flags
            .contains(Flags::STRIKEOUT)
            .then(|| StrikethroughStyle {
                color: Some(fg),
                thickness: px(1.0),
            });

        let weight = if cell.flags.intersects(Flags::BOLD | Flags::DIM_BOLD) {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        };
        let style = if cell.flags.intersects(Flags::ITALIC | Flags::BOLD_ITALIC) {
            FontStyle::Italic
        } else {
            FontStyle::Normal
        };

        TextRun {
            len: cell.c.len_utf8(),
            color: fg,
            background_color: None,
            font: Font {
                family: self.active_font_family(),
                weight,
                style,
                ..Font::default()
            },
            underline,
            strikethrough,
        }
    }

    fn grid_style_key(
        &self,
        font_brightness: f32,
        metrics: TerminalMetrics,
        cx: &App,
    ) -> GridStyleKey {
        GridStyleKey {
            font_family: self.active_font_family(),
            font_size: self.font_size.as_f32().to_bits(),
            cell_width: metrics.cell_width.as_f32().to_bits(),
            font_brightness: font_brightness.to_bits(),
            foreground: cx.theme().foreground.into(),
            background: cx.theme().background.into(),
            muted_foreground: cx.theme().muted_foreground.into(),
            primary: cx.theme().primary.into(),
        }
    }

    fn active_selection(&self) -> Option<ViewportSelection> {
        if let Some(frozen) = self.frozen_selection.as_ref() {
            remap_frozen_selection(frozen, &self.snapshot)
        } else {
            self.snapshot.selection
        }
    }

    fn selection_key(&self) -> Option<SelectionKey> {
        self.active_selection().map(SelectionKey::from)
    }

    fn hovered_url_for_tab(&self, cx: &App) -> Option<HoveredUrl> {
        self.view
            .read(cx)
            .hovered_url
            .as_ref()
            .filter(|hovered_url| hovered_url.tab_id == self.tab_id)
            .cloned()
    }

    fn row_highlights(&self) -> Vec<RowHighlights> {
        let mut rows = vec![RowHighlights::default(); self.snapshot.visible_rows.len()];
        for (&(row, col), &color) in self.snapshot.highlights.iter() {
            if let Some(highlights) = rows.get_mut(row as usize) {
                highlights.keyword.push((col, color));
            }
        }
        if let Some(search_highlights) = self.search_highlights.as_ref() {
            for (&(row, col), &color) in search_highlights {
                if let Some(highlights) = rows.get_mut(row as usize) {
                    highlights.search.push((col, color));
                }
            }
        }
        for highlights in &mut rows {
            highlights.keyword.sort_by_key(|(col, _)| *col);
            highlights.search.sort_by_key(|(col, _)| *col);
        }
        rows
    }

    fn cached_grid_rows(
        &self,
        cache: Option<GridLayoutCache>,
        metrics: TerminalMetrics,
        window: &mut Window,
        cx: &App,
    ) -> (Vec<Rc<RowLayout>>, GridLayoutCache) {
        let font_brightness = self.view.read(cx).appearance.terminal_font_brightness;
        let style = self.grid_style_key(font_brightness, metrics, cx);
        let key = GridLayoutKey {
            style: style.clone(),
            selection: self.selection_key(),
        };
        let previous = cache.filter(|cache| cache.key == key);
        let highlights = self.row_highlights();
        let mut rows = Vec::with_capacity(self.snapshot.visible_rows.len());
        let mut cached_rows = Vec::with_capacity(self.snapshot.visible_rows.len());

        for (row_index, source) in self.snapshot.visible_rows.iter().enumerate() {
            let row_highlights = &highlights[row_index];
            let highlight_key = RowHighlightKey::from(row_highlights);
            let layout = previous
                .as_ref()
                .and_then(|cache| {
                    cache
                        .rows
                        .iter()
                        .find(|cached| Rc::ptr_eq(&cached.source, source))
                })
                .filter(|cached| cached.highlights == highlight_key)
                .map(|cached| cached.layout.clone())
                .unwrap_or_else(|| {
                    Rc::new(self.layout_row(
                        source,
                        row_highlights,
                        font_brightness,
                        metrics,
                        window,
                        cx,
                    ))
                });
            rows.push(layout.clone());
            cached_rows.push(CachedRowLayout {
                source: source.clone(),
                highlights: highlight_key,
                layout,
            });
        }

        (
            rows,
            GridLayoutCache {
                key,
                rows: cached_rows,
            },
        )
    }

    fn layout_row(
        &self,
        row: &super::RenderRow,
        highlights: &RowHighlights,
        font_brightness: f32,
        metrics: TerminalMetrics,
        window: &mut Window,
        cx: &App,
    ) -> RowLayout {
        let keyword_highlights: HashMap<_, _> = highlights.keyword.iter().copied().collect();
        let search_highlights: HashMap<_, _> = highlights.search.iter().copied().collect();
        let mut rects = Vec::new();
        let mut runs = Vec::new();
        let mut custom_blocks = Vec::new();
        let mut current_run: Option<BatchedTextRun> = None;

        for render_cell in &row.cells {
            let cell = &render_cell.cell;
            if cell.flags.intersects(
                Flags::HIDDEN | Flags::WIDE_CHAR_SPACER | Flags::LEADING_WIDE_CHAR_SPACER,
            ) {
                continue;
            }

            let bg = color_to_hsla(cell.bg, false, font_brightness, cx);
            if !is_default_bg(cell.bg) || cell.flags.contains(Flags::INVERSE) {
                rects.push(LayoutRect {
                    row: 0,
                    col: render_cell.col,
                    cells: if cell.flags.contains(Flags::WIDE_CHAR) {
                        2
                    } else {
                        1
                    },
                    color: if cell.flags.contains(Flags::INVERSE) {
                        color_to_hsla(cell.fg, false, font_brightness, cx)
                    } else {
                        bg
                    },
                });
            }

            if is_blank(cell) {
                if let Some(run) = current_run.take() {
                    runs.push(run.into_shaped(metrics.cell_width, window));
                }
                continue;
            }

            let mut style = self.cell_run_style(cell, font_brightness, cx);
            if let Some(hl_color) = search_highlights.get(&render_cell.col).copied() {
                style.color = adjust_terminal_foreground_brightness(hl_color, font_brightness);
            } else if let Some(hl_color) = keyword_highlights.get(&render_cell.col).copied()
                && keyword_highlight_allowed(cell)
            {
                // Preserve original ANSI/truecolor emphasis from terminal output.
                style.color = adjust_terminal_foreground_brightness(hl_color, font_brightness);
            }

            if is_custom_block_supported(cell.c) {
                if let Some(run) = current_run.take() {
                    runs.push(run.into_shaped(metrics.cell_width, window));
                }
                custom_blocks.push(LayoutCustomBlock {
                    c: cell.c,
                    row: 0,
                    col: render_cell.col,
                    cells: if cell.flags.contains(Flags::WIDE_CHAR) {
                        2
                    } else {
                        1
                    },
                    color: style.color,
                });
                continue;
            }

            if let Some(run) = current_run.as_mut()
                && run.can_append(&style, 0, render_cell.col)
            {
                run.append(cell.c, cell.zerowidth());
                continue;
            }

            if let Some(run) = current_run.take() {
                runs.push(run.into_shaped(metrics.cell_width, window));
            }

            let mut run = BatchedTextRun::new(0, render_cell.col, cell.c, style, self.font_size);
            if let Some(chars) = cell.zerowidth() {
                for ch in chars {
                    run.append_zerowidth(*ch);
                }
            }
            current_run = Some(run);
        }

        if let Some(run) = current_run {
            runs.push(run.into_shaped(metrics.cell_width, window));
        }

        RowLayout {
            rects: merge_rects(rects),
            runs,
            custom_blocks,
        }
    }

    fn selection_rects(&self, cx: &App) -> Vec<LayoutRect> {
        self.active_selection()
            .map(|selection| {
                selection_background_rects(
                    selection,
                    self.snapshot.rows,
                    self.snapshot.cols,
                    cx.theme().selection,
                )
            })
            .unwrap_or_default()
    }

    fn hovered_url_underlines(&self, cx: &App) -> Vec<LayoutUnderline> {
        let Some(hovered_url) = self.hovered_url_for_tab(cx) else {
            return Vec::new();
        };
        let view_read = self.view.read(cx);
        if !terminal_link_visual_active(true, view_read.cmd_ctrl_pressed) {
            return Vec::new();
        }
        let font_brightness = view_read.appearance.terminal_font_brightness;
        let mut underlines = Vec::with_capacity(hovered_url.cells.len());
        for &(row, col) in &hovered_url.cells {
            let Some(cell) = self
                .snapshot
                .visible_rows
                .get(row)
                .and_then(|render_row| render_row.cells.iter().find(|cell| cell.col == col as i32))
                .map(|render_cell| &render_cell.cell)
            else {
                continue;
            };
            if cell.flags.intersects(
                Flags::HIDDEN | Flags::WIDE_CHAR_SPACER | Flags::LEADING_WIDE_CHAR_SPACER,
            ) {
                continue;
            }
            let mut style = self.cell_run_style(cell, font_brightness, cx);
            if let Some(color) = self
                .snapshot
                .highlights
                .get(&(row as i32, col as i32))
                .copied()
                && keyword_highlight_allowed(cell)
            {
                style.color = adjust_terminal_foreground_brightness(color, font_brightness);
            }
            if let Some(color) = self
                .search_highlights
                .as_ref()
                .and_then(|highlights| highlights.get(&(row as i32, col as i32)))
                .copied()
            {
                style.color = adjust_terminal_foreground_brightness(color, font_brightness);
            }
            underlines.push(LayoutUnderline {
                row: row as i32,
                col: col as i32,
                cells: usize::from(cell.flags.contains(Flags::WIDE_CHAR)) + 1,
                color: style.color,
            });
        }
        merge_underlines(underlines)
    }

    fn cursor_layout(&self, cx: &App) -> Option<CursorLayout> {
        use crate::config::CursorStyle;
        let cursor_style = self.view.read(cx).appearance.cursor_style;
        let show_cursor = match cursor_style {
            CursorStyle::Blink | CursorStyle::BeamBlink => {
                if let Ok(duration) =
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                {
                    (duration.as_millis() / 600) % 2 == 0
                } else {
                    true
                }
            }
            _ => true,
        };

        self.snapshot.cursor.map(|cursor| {
            let mut shape = match cursor_style {
                CursorStyle::Default => cursor.shape,
                CursorStyle::Blink => CursorShape::Block,
                CursorStyle::Beam => CursorShape::Beam,
                CursorStyle::BeamBlink => CursorShape::Beam,
            };
            if !show_cursor {
                shape = CursorShape::Hidden;
            }
            let background = self.cursor_background_color(cursor.row, cursor.col, cx);
            CursorLayout {
                row: cursor.row,
                col: cursor.col,
                shape,
                color: high_contrast_cursor_color(background),
            }
        })
    }

    fn cursor_background_color(&self, row: usize, col: usize, cx: &App) -> Hsla {
        let Some(render_cell) = self
            .snapshot
            .visible_rows
            .get(row)
            .and_then(|render_row| render_row.cells.iter().find(|cell| cell.col == col as i32))
        else {
            return cx.theme().background;
        };

        visible_cell_background(&render_cell.cell, cx)
    }

    fn paint_composition(
        &self,
        composition: &TerminalComposition,
        draw_origin: Point<Pixels>,
        metrics: TerminalMetrics,
        window: &mut Window,
        cx: &mut App,
    ) {
        if composition.text.is_empty() {
            return;
        }

        let row = composition
            .anchor_row
            .min(self.snapshot.rows.saturating_sub(1));
        let col = composition
            .anchor_col
            .min(self.snapshot.cols.saturating_sub(1));
        let pos = point(
            draw_origin.x + metrics.cell_width * col as f32,
            draw_origin.y + metrics.line_height * row as f32,
        );

        let mut base_style = self.base_text_style(cx);
        if composition.underline {
            base_style.underline = Some(UnderlineStyle {
                color: Some(base_style.color),
                thickness: px(1.0),
                wavy: false,
            });
        }

        let selected_bytes = composition_selected_byte_range(
            &composition.text,
            composition.selected_range_utf16.as_ref(),
        );
        let mut runs = Vec::new();
        let font = Font {
            family: self.active_font_family(),
            ..Font::default()
        };
        let selection_bg = cx.theme().selection;
        let selection_fg = high_contrast_cursor_color(selection_bg);
        let text_len = composition.text.len();

        let push_run = |runs: &mut Vec<TextRun>, len: usize, color: Hsla| {
            if len == 0 {
                return;
            }
            runs.push(TextRun {
                len,
                font: font.clone(),
                color,
                underline: base_style.underline.clone(),
                ..Default::default()
            });
        };

        if let Some(range) = selected_bytes.as_ref().filter(|range| !range.is_empty()) {
            push_run(&mut runs, range.start, base_style.color);
            push_run(&mut runs, range.end - range.start, selection_fg);
            push_run(&mut runs, text_len - range.end, base_style.color);
        } else {
            push_run(&mut runs, text_len, base_style.color);
        }

        let shaped = window.text_system().shape_line(
            composition.text.clone().into(),
            self.font_size,
            &runs,
            None,
        );
        let bg_bounds = Bounds::new(pos, gpui::size(shaped.width, metrics.line_height));
        window.paint_quad(fill(bg_bounds, cx.theme().background));

        if let Some(range) = selected_bytes.filter(|range| !range.is_empty()) {
            let prefix_width = composition_text_width(
                &composition.text[..range.start],
                self.font_size,
                font.clone(),
                base_style.color,
                base_style.underline.clone(),
                window,
            );
            let selected_width = composition_text_width(
                &composition.text[range.clone()],
                self.font_size,
                font,
                selection_fg,
                base_style.underline.clone(),
                window,
            );
            if selected_width > px(0.0) {
                window.paint_quad(fill(
                    Bounds::new(
                        point(pos.x + prefix_width, pos.y),
                        gpui::size(selected_width, metrics.line_height),
                    ),
                    selection_bg,
                ));
            }
        }

        shaped
            .paint(
                pos,
                metrics.line_height,
                gpui::TextAlign::Left,
                None,
                window,
                cx,
            )
            .ok();

        if let Some(cursor_utf16) = composition.cursor_utf16 {
            let cursor_utf16 = cursor_utf16.min(composition.text.encode_utf16().count());
            window.paint_quad(fill(
                Bounds::new(
                    point(pos.x + metrics.cell_width * cursor_utf16 as f32, pos.y),
                    gpui::size(px(2.), metrics.line_height),
                ),
                base_style.color,
            ));
        }
    }
}

fn ensure_terminal_font_family_loaded(family: &str, cx: &mut App) {
    if let Err(err) = crate::app::theme::ensure_embedded_font_family_loaded(family, cx) {
        tracing::warn!(
            component = "theme",
            operation = "load_visible_terminal_font",
            font_family = family,
            error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
            "Failed to load embedded terminal font for visible terminal"
        );
    }
}

pub(crate) fn terminal_monospace_font_family(
    window: &mut Window,
    family: SharedString,
    font_size: Pixels,
) -> SharedString {
    if terminal_font_is_monospace(window, family.clone(), font_size) {
        family
    } else {
        "Maple Mono NF CN".into()
    }
}

pub(crate) fn terminal_font_is_monospace(
    window: &mut Window,
    family: SharedString,
    font_size: Pixels,
) -> bool {
    let font = Font {
        family,
        ..Font::default()
    };
    let text_system = window.text_system();
    let font_id = text_system.resolve_font(&font);
    let Ok(zero) = text_system.ch_advance(font_id, font_size) else {
        return false;
    };

    ['i', 'm', 'W', ' '].into_iter().all(|ch| {
        text_system
            .advance(font_id, font_size, ch)
            .map(|advance| (advance.width - zero).abs() <= px(0.5))
            .unwrap_or(false)
    })
}

impl IntoElement for TerminalElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TerminalElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        Some(format!("terminal-grid-{}", self.tab_id).into())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = gpui::Style::default();
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();
        (window.request_layout(style, None, cx), ())
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        ensure_terminal_font_family_loaded(&self.font_family, cx);
        let _ = self.base_text_style(cx);
        let metrics = self.measured_metrics(window, cx);
        let grid_rows = if let Some(id) = id {
            window.with_element_state(id, |cache: Option<GridLayoutCache>, window| {
                self.cached_grid_rows(cache, metrics, window, cx)
            })
        } else {
            self.cached_grid_rows(None, metrics, window, cx).0
        };
        let selection_rects = self.selection_rects(cx);
        let underlines = self.hovered_url_underlines(cx);

        // Save the precise GPUI-rendered bounds of this terminal element.
        // This is 100% accurate because it is recorded during layout prepaint.
        let view = self.view.clone();
        let tab_id = self.tab_id.clone();
        let _ = view.update(cx, |this, cx| {
            let old_bounds = this.terminal_bounds.insert(tab_id.clone(), bounds);

            // Sync PTY size unconditionally on every prepaint layout pass to ensure
            // absolute synchronization with GPUI layout regardless of intermediate events.
            this.update_terminal_font_metrics(
                metrics.cell_width.as_f32(),
                metrics.line_height.as_f32(),
            );
            let line_height = metrics.line_height.as_f32();
            let cell_width = metrics.cell_width.as_f32();
            let w = bounds.size.width.as_f32();
            let h = bounds.size.height.as_f32();
            let cols = (w / cell_width).floor().max(1.0) as u16;
            let rows = (h / line_height).floor().max(1.0) as u16;

            if let Some(tab) = this.tabs.iter_mut().find(|t| t.id == tab_id) {
                tab.resize(cols, rows);
            }

            if old_bounds != Some(bounds) {
                cx.notify();
            }
        });

        PrepaintState {
            bounds,
            metrics: TerminalMetrics {
                cell_width: metrics.cell_width,
                line_height: metrics.line_height,
            },
            selection_rects,
            grid_rows,
            cursor: self.cursor_layout(cx),
            underlines,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        // Compute a vertical offset to center the text grid vertically,
        // distributing the leftover pixel remainder evenly to the top and bottom.
        let grid_height = prepaint.metrics.line_height
            * (prepaint.bounds.size.height.as_f32() / prepaint.metrics.line_height.as_f32())
                .floor();
        let y_offset = ((prepaint.bounds.size.height - grid_height) / 2.0).max(px(0.0));
        let draw_origin = point(
            prepaint.bounds.origin.x,
            prepaint.bounds.origin.y + y_offset,
        );
        window.paint_quad(fill(prepaint.bounds, cx.theme().background));

        for (row, row_layout) in prepaint.grid_rows.iter().enumerate() {
            for rect in &row_layout.rects {
                rect.paint_with_row_offset(draw_origin, prepaint.metrics, row as i32, window);
            }
        }

        for rect in &prepaint.selection_rects {
            rect.paint(draw_origin, prepaint.metrics, window);
        }

        for (row, row_layout) in prepaint.grid_rows.iter().enumerate() {
            for run in &row_layout.runs {
                run.paint_with_row_offset(draw_origin, prepaint.metrics, row as i32, window, cx);
            }
        }

        for u in &prepaint.underlines {
            u.paint(draw_origin, prepaint.metrics, window);
        }

        for (row, row_layout) in prepaint.grid_rows.iter().enumerate() {
            for block in &row_layout.custom_blocks {
                let x = draw_origin.x.as_f32()
                    + block.col as f32 * prepaint.metrics.cell_width.as_f32();
                let y = draw_origin.y.as_f32()
                    + prepaint.metrics.line_height.as_f32() * (block.row + row as i32) as f32;
                paint_custom_block(
                    window,
                    block.c,
                    x,
                    y,
                    prepaint.metrics.cell_width.as_f32() * block.cells as f32,
                    prepaint.metrics.line_height.as_f32(),
                    block.color,
                );
            }
        }

        window.handle_input(
            &self.focus_handle,
            TerminalInputHandler {
                view: self.view.clone(),
                element_bounds: Bounds::new(draw_origin, prepaint.bounds.size),
                cell_width: prepaint.metrics.cell_width.as_f32(),
                line_height: prepaint.metrics.line_height.as_f32(),
            },
            cx,
        );

        if let Some(composition) = self.composition.as_ref() {
            self.paint_composition(composition, draw_origin, prepaint.metrics, window, cx);
        }

        if let Some(cursor) = prepaint.cursor {
            if self
                .composition
                .as_ref()
                .is_some_and(|composition| !composition.text.is_empty())
            {
                return;
            }
            let x = draw_origin.x + prepaint.metrics.cell_width * cursor.col as f32;
            let y = draw_origin.y + prepaint.metrics.line_height * cursor.row as f32;
            match cursor.shape {
                CursorShape::Hidden => {}
                CursorShape::Beam => {
                    window.paint_quad(fill(
                        Bounds::new(
                            point(x, y),
                            gpui::size(px(2.), prepaint.metrics.line_height),
                        ),
                        cursor.color,
                    ));
                }
                CursorShape::Underline => {
                    window.paint_quad(fill(
                        Bounds::new(
                            point(x, y + prepaint.metrics.line_height - px(2.)),
                            gpui::size(prepaint.metrics.cell_width, px(2.)),
                        ),
                        cursor.color,
                    ));
                }
                CursorShape::Block | CursorShape::HollowBlock => {
                    let alpha = if matches!(cursor.shape, CursorShape::HollowBlock) {
                        0.35
                    } else {
                        0.72
                    };
                    window.paint_quad(fill(
                        Bounds::new(
                            point(x, y),
                            gpui::size(prepaint.metrics.cell_width, prepaint.metrics.line_height),
                        ),
                        cursor.color.opacity(alpha),
                    ));
                }
            }
        }
    }
}

fn merge_rects(mut rects: Vec<LayoutRect>) -> Vec<LayoutRect> {
    rects.sort_by_key(|rect| (rect.row, rect.col));
    let mut merged: Vec<LayoutRect> = Vec::with_capacity(rects.len());

    for rect in rects {
        if let Some(last) = merged.last_mut() {
            if last.row == rect.row
                && last.color == rect.color
                && last.col + last.cells as i32 == rect.col
            {
                last.cells += rect.cells;
                continue;
            }
        }
        merged.push(rect);
    }

    merged
}

fn merge_underlines(mut underlines: Vec<LayoutUnderline>) -> Vec<LayoutUnderline> {
    underlines.sort_by_key(|u| (u.row, u.col));
    let mut merged: Vec<LayoutUnderline> = Vec::with_capacity(underlines.len());

    for u in underlines {
        if let Some(last) = merged.last_mut() {
            if last.row == u.row && last.color == u.color && last.col + last.cells as i32 == u.col {
                last.cells += u.cells;
                continue;
            }
        }
        merged.push(u);
    }

    merged
}

fn remap_frozen_selection(
    frozen_selection: &TerminalFrozenSelection,
    snapshot: &RenderSnapshot,
) -> Option<ViewportSelection> {
    let raw_start = frozen_selection.selection.start_row as i32;
    let raw_end = frozen_selection.selection.end_row as i32;
    let rows = snapshot.rows as i32;
    if raw_end < 0 || raw_start >= rows {
        return None;
    }

    let start_row = raw_start.max(0) as usize;
    let end_row = raw_end.min(rows.saturating_sub(1)) as usize;
    Some(ViewportSelection {
        start_row,
        start_col: if raw_start < 0 {
            0
        } else {
            frozen_selection.selection.start_col
        },
        end_row,
        end_col: if raw_end >= rows {
            snapshot.cols.saturating_sub(1)
        } else {
            frozen_selection.selection.end_col
        },
        is_block: frozen_selection.selection.is_block,
    })
}

fn selection_background_rects(
    selection: ViewportSelection,
    rows: usize,
    cols: usize,
    color: Hsla,
) -> Vec<LayoutRect> {
    let mut rects = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            if selection_contains(selection, row as i32, col as i32) {
                rects.push(LayoutRect {
                    row: row as i32,
                    col: col as i32,
                    cells: 1,
                    color,
                });
            }
        }
    }
    rects
}

fn composition_text_width(
    text: &str,
    font_size: Pixels,
    font: Font,
    color: Hsla,
    underline: Option<UnderlineStyle>,
    window: &mut Window,
) -> Pixels {
    if text.is_empty() {
        return px(0.0);
    }

    window
        .text_system()
        .shape_line(
            text.to_string().into(),
            font_size,
            &[TextRun {
                len: text.len(),
                font,
                color,
                underline,
                ..Default::default()
            }],
            None,
        )
        .width
}

fn composition_selected_byte_range(
    text: &str,
    range_utf16: Option<&std::ops::Range<usize>>,
) -> Option<std::ops::Range<usize>> {
    let range = range_utf16?;
    let len_utf16 = text.encode_utf16().count();
    let start_utf16 = range.start.min(len_utf16);
    let end_utf16 = range.end.min(len_utf16);
    let (start_utf16, end_utf16) = if start_utf16 <= end_utf16 {
        (start_utf16, end_utf16)
    } else {
        (end_utf16, start_utf16)
    };

    let start = byte_index_for_utf16_offset(text, start_utf16);
    let end = byte_index_for_utf16_offset(text, end_utf16);
    Some(start..end)
}

fn byte_index_for_utf16_offset(text: &str, target_utf16: usize) -> usize {
    let mut current_utf16 = 0;
    for (byte_idx, ch) in text.char_indices() {
        if current_utf16 >= target_utf16 {
            return byte_idx;
        }
        let next_utf16 = current_utf16 + ch.len_utf16();
        if target_utf16 < next_utf16 {
            return byte_idx;
        }
        current_utf16 = next_utf16;
    }
    text.len()
}

fn selection_contains(selection: ViewportSelection, row: i32, col: i32) -> bool {
    let row = row.max(0) as usize;
    let col = col.max(0) as usize;

    if row < selection.start_row || row > selection.end_row {
        return false;
    }

    if selection.is_block {
        return col >= selection.start_col && col <= selection.end_col;
    }

    let after_start = row > selection.start_row || col >= selection.start_col;
    let before_end = row < selection.end_row || col <= selection.end_col;
    after_start && before_end
}

fn is_blank(cell: &alacritty_terminal::term::cell::Cell) -> bool {
    cell.c == ' '
        && cell.zerowidth().is_none()
        && !cell
            .flags
            .intersects(Flags::ALL_UNDERLINES | Flags::STRIKEOUT)
}

fn is_default_bg(color: AnsiColor) -> bool {
    matches!(color, AnsiColor::Named(NamedColor::Background))
}

fn is_default_fg(color: AnsiColor) -> bool {
    matches!(color, AnsiColor::Named(NamedColor::Foreground))
}

fn keyword_highlight_allowed(cell: &alacritty_terminal::term::cell::Cell) -> bool {
    let (visible_fg, visible_bg) = if cell.flags.contains(Flags::INVERSE) {
        (cell.bg, cell.fg)
    } else {
        (cell.fg, cell.bg)
    };

    is_default_fg(visible_fg) && is_default_bg(visible_bg)
}

fn adjust_terminal_foreground_brightness(color: Hsla, factor: f32) -> Hsla {
    if (factor - 1.0).abs() <= f32::EPSILON {
        return color;
    }

    Hsla {
        l: (color.l * factor).clamp(0.02, 0.98),
        ..color
    }
}

fn visible_cell_background(cell: &alacritty_terminal::term::cell::Cell, cx: &App) -> Hsla {
    if cell.flags.contains(Flags::INVERSE) {
        color_to_hsla(cell.fg, false, 1.0, cx)
    } else if is_default_bg(cell.bg) {
        cx.theme().background
    } else {
        color_to_hsla(cell.bg, false, 1.0, cx)
    }
}

fn high_contrast_cursor_color(background: Hsla) -> Hsla {
    let black = Hsla::black();
    let white = Hsla::white();

    if contrast_ratio(black, background) >= contrast_ratio(white, background) {
        black
    } else {
        white
    }
}

fn contrast_ratio(a: Hsla, b: Hsla) -> f32 {
    let a = relative_luminance(a);
    let b = relative_luminance(b);
    let (lighter, darker) = if a >= b { (a, b) } else { (b, a) };
    (lighter + 0.05) / (darker + 0.05)
}

fn relative_luminance(color: Hsla) -> f32 {
    let rgba = Rgba::from(color);
    let channel = |value: f32| {
        if value <= 0.04045 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    };

    0.2126 * channel(rgba.r) + 0.7152 * channel(rgba.g) + 0.0722 * channel(rgba.b)
}

fn color_to_hsla(color: AnsiColor, foreground: bool, font_brightness: f32, cx: &App) -> Hsla {
    let color = match color {
        AnsiColor::Spec(rgb) => Hsla::from(Rgba {
            r: rgb.r as f32 / 255.0,
            g: rgb.g as f32 / 255.0,
            b: rgb.b as f32 / 255.0,
            a: 1.0,
        }),
        AnsiColor::Indexed(index) => ansi_index_color(index),
        AnsiColor::Named(named) => named_color(named, foreground, cx),
    };

    if foreground {
        adjust_terminal_foreground_brightness(color, font_brightness)
    } else {
        color
    }
}

fn ansi_index_color(index: u8) -> Hsla {
    const ANSI_16: [u32; 16] = [
        0x1f2430, 0xff5c57, 0x5af78e, 0xf3f99d, 0x57c7ff, 0xff6ac1, 0x9aedfe, 0xf1f1f0, 0x686868,
        0xff5c57, 0x5af78e, 0xf3f99d, 0x57c7ff, 0xff6ac1, 0x9aedfe, 0xffffff,
    ];

    if (index as usize) < ANSI_16.len() {
        return Hsla::from(rgb(ANSI_16[index as usize]));
    }

    if index >= 232 {
        let gray = 8 + (index - 232) * 10;
        return Hsla::from(Rgba {
            r: gray as f32 / 255.0,
            g: gray as f32 / 255.0,
            b: gray as f32 / 255.0,
            a: 1.0,
        });
    }

    let i = index - 16;
    let r = i / 36;
    let g = (i % 36) / 6;
    let b = i % 6;
    let conv = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
    Hsla::from(Rgba {
        r: conv(r) as f32 / 255.0,
        g: conv(g) as f32 / 255.0,
        b: conv(b) as f32 / 255.0,
        a: 1.0,
    })
}

fn named_color(named: NamedColor, _foreground: bool, cx: &App) -> Hsla {
    match named {
        NamedColor::Foreground => cx.theme().foreground,
        NamedColor::Background => cx.theme().background,
        NamedColor::Black => Hsla::from(rgb(0x1f2430)),
        NamedColor::Red => Hsla::from(rgb(0xff5c57)),
        NamedColor::Green => Hsla::from(rgb(0x5af78e)),
        NamedColor::Yellow => Hsla::from(rgb(0xf3f99d)),
        NamedColor::Blue => Hsla::from(rgb(0x57c7ff)),
        NamedColor::Magenta => Hsla::from(rgb(0xff6ac1)),
        NamedColor::Cyan => Hsla::from(rgb(0x9aedfe)),
        NamedColor::White => Hsla::from(rgb(0xf1f1f0)),
        NamedColor::BrightBlack => Hsla::from(rgb(0x686868)),
        NamedColor::BrightRed => Hsla::from(rgb(0xff5c57)),
        NamedColor::BrightGreen => Hsla::from(rgb(0x5af78e)),
        NamedColor::BrightYellow => Hsla::from(rgb(0xf3f99d)),
        NamedColor::BrightBlue => Hsla::from(rgb(0x57c7ff)),
        NamedColor::BrightMagenta => Hsla::from(rgb(0xff6ac1)),
        NamedColor::BrightCyan => Hsla::from(rgb(0x9aedfe)),
        NamedColor::BrightWhite => Hsla::from(rgb(0xffffff)),
        NamedColor::Cursor => cx.theme().primary,
        NamedColor::DimForeground => cx.theme().muted_foreground,
        NamedColor::BrightForeground => cx.theme().foreground,
        NamedColor::DimBlack => Hsla::from(rgb(0x3b4252)),
        NamedColor::DimRed => Hsla::from(rgb(0xbf616a)),
        NamedColor::DimGreen => Hsla::from(rgb(0xa3be8c)),
        NamedColor::DimYellow => Hsla::from(rgb(0xebcb8b)),
        NamedColor::DimBlue => Hsla::from(rgb(0x81a1c1)),
        NamedColor::DimMagenta => Hsla::from(rgb(0xb48ead)),
        NamedColor::DimCyan => Hsla::from(rgb(0x88c0d0)),
        NamedColor::DimWhite => Hsla::from(rgb(0xe5e9f0)),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ColorKey, FNV1A_OFFSET_BASIS, GridLayoutKey, GridStyleKey, SelectionKey,
        byte_index_for_utf16_offset, composition_selected_byte_range, contrast_ratio, fnv1a_hash,
        fnv1a_update, high_contrast_cursor_color, keyword_highlight_allowed,
        remap_frozen_selection, selection_background_rects,
    };
    use crate::terminal::{RenderSnapshot, TerminalFrozenSelection, ViewportSelection};
    use alacritty_terminal::{
        term::cell::{Cell, Flags},
        vte::ansi::{Color as AnsiColor, NamedColor},
    };
    use gpui::{Hsla, SharedString, rgb};
    use std::collections::HashMap;

    #[test]
    fn keyword_highlight_allows_default_cells() {
        assert!(keyword_highlight_allowed(&Cell::default()));
    }

    #[test]
    fn keyword_highlight_skips_explicit_foreground_colors() {
        let mut cell = Cell::default();
        cell.fg = AnsiColor::Named(NamedColor::Red);

        assert!(!keyword_highlight_allowed(&cell));
    }

    #[test]
    fn keyword_highlight_skips_explicit_background_colors() {
        let mut cell = Cell::default();
        cell.bg = AnsiColor::Named(NamedColor::Blue);

        assert!(!keyword_highlight_allowed(&cell));
    }

    #[test]
    fn keyword_highlight_skips_inverse_cells() {
        let mut cell = Cell::default();
        cell.flags.insert(Flags::INVERSE);

        assert!(!keyword_highlight_allowed(&cell));
    }

    #[test]
    fn cursor_contrast_uses_white_on_dark_background() {
        let background = Hsla::from(rgb(0x101010));
        assert_eq!(high_contrast_cursor_color(background), Hsla::white());
    }

    #[test]
    fn cursor_contrast_uses_black_on_light_background() {
        let background = Hsla::from(rgb(0xf5f5f5));
        assert_eq!(high_contrast_cursor_color(background), Hsla::black());
    }

    #[test]
    fn cursor_contrast_selects_stronger_black_or_white_ratio() {
        let background = Hsla::from(rgb(0x777777));
        let cursor = high_contrast_cursor_color(background);
        let selected = contrast_ratio(cursor, background);
        let alternative = if cursor == Hsla::black() {
            contrast_ratio(Hsla::white(), background)
        } else {
            contrast_ratio(Hsla::black(), background)
        };

        assert!(selected >= alternative);
    }

    #[test]
    fn composition_byte_index_uses_utf16_offsets() {
        let text = "a中🏀";

        assert_eq!(byte_index_for_utf16_offset(text, 0), 0);
        assert_eq!(byte_index_for_utf16_offset(text, 1), "a".len());
        assert_eq!(byte_index_for_utf16_offset(text, 2), "a中".len());
        assert_eq!(byte_index_for_utf16_offset(text, 4), text.len());
    }

    #[test]
    fn composition_selected_range_handles_non_ascii_text() {
        let text = "pin中🏀yin";

        assert_eq!(
            composition_selected_byte_range(text, Some(&(3..6))),
            Some("pin".len().."pin中🏀".len())
        );
    }

    #[test]
    fn composition_selected_range_clamps_to_text() {
        let text = "abc";

        assert_eq!(
            composition_selected_byte_range(text, Some(&(1..99))),
            Some(1..3)
        );
    }

    #[test]
    fn text_layout_hash_tracks_zero_width_combining_characters() {
        let base = fnv1a_hash("a");
        let combined = fnv1a_update(base, "\u{0301}".as_bytes());

        assert_ne!(combined, base);
        assert_eq!(combined, fnv1a_hash("a\u{0301}"));
        assert_eq!(fnv1a_hash(""), FNV1A_OFFSET_BASIS);
    }

    #[test]
    fn grid_layout_key_tracks_only_state_that_changes_shaped_rows() {
        let base = GridLayoutKey {
            style: style_key(8.0),
            selection: None,
        };

        let selection = GridLayoutKey {
            selection: Some(SelectionKey::from(ViewportSelection {
                start_row: 1,
                start_col: 2,
                end_row: 3,
                end_col: 4,
                is_block: false,
            })),
            ..base.clone()
        };
        let cell_width = GridLayoutKey {
            style: style_key(9.0),
            ..base.clone()
        };

        assert!(base != selection);
        assert!(base != cell_width);
    }

    #[test]
    fn frozen_selection_stays_at_viewport_position_with_stream_history() {
        let frozen = frozen_selection(8, 8);
        let snapshot = snapshot(10, 7);

        assert_eq!(
            remap_frozen_selection(&frozen, &snapshot).map(|selection| selection.start_row),
            Some(8)
        );
    }

    #[test]
    fn frozen_selection_ignores_live_selection_after_refresh() {
        let frozen = frozen_selection(8, 8);
        let live_selection = ViewportSelection {
            start_row: 6,
            start_col: 0,
            end_row: 6,
            end_col: 3,
            is_block: false,
        };
        let snapshot = snapshot_with_selection(10, 2000, Some(live_selection));

        assert_eq!(
            remap_frozen_selection(&frozen, &snapshot).map(|selection| selection.start_row),
            Some(8)
        );
    }

    #[test]
    fn frozen_selection_paints_background_without_live_cells() {
        let frozen = frozen_selection(8, 8);
        let snapshot = snapshot(10, 7);
        let selection = remap_frozen_selection(&frozen, &snapshot).unwrap();
        let color = Hsla::from(rgb(0x336699));
        let rects = selection_background_rects(selection, snapshot.rows, snapshot.cols, color);
        let coords: Vec<_> = rects
            .iter()
            .map(|rect| (rect.row, rect.col, rect.cells))
            .collect();

        assert_eq!(coords, vec![(8, 0, 1), (8, 1, 1), (8, 2, 1), (8, 3, 1)]);
    }

    fn frozen_selection(start_row: usize, end_row: usize) -> TerminalFrozenSelection {
        TerminalFrozenSelection {
            tab_id: "tab".to_string(),
            selection: ViewportSelection {
                start_row,
                start_col: 0,
                end_row,
                end_col: 3,
                is_block: false,
            },
            text: "text".to_string(),
        }
    }

    fn snapshot(rows: usize, history_size: usize) -> RenderSnapshot {
        snapshot_with_selection(rows, history_size, None)
    }

    fn snapshot_with_selection(
        rows: usize,
        history_size: usize,
        selection: Option<ViewportSelection>,
    ) -> RenderSnapshot {
        RenderSnapshot {
            visible_rows: std::rc::Rc::new(
                (0..rows)
                    .map(|_| std::rc::Rc::new(crate::terminal::RenderRow { cells: Vec::new() }))
                    .collect(),
            ),
            cursor: None,
            selection,
            display_offset: 0,
            history_size,
            rows,
            cols: 10,
            highlights: std::rc::Rc::new(HashMap::new()),
        }
    }

    fn style_key(cell_width: f32) -> GridStyleKey {
        GridStyleKey {
            font_family: SharedString::from("Maple Mono NF CN"),
            font_size: 14.0f32.to_bits(),
            cell_width: cell_width.to_bits(),
            font_brightness: 1.0f32.to_bits(),
            foreground: ColorKey::from(Hsla::from(rgb(0xffffff))),
            background: ColorKey::from(Hsla::from(rgb(0x000000))),
            muted_foreground: ColorKey::from(Hsla::from(rgb(0x888888))),
            primary: ColorKey::from(Hsla::from(rgb(0x57c7ff))),
        }
    }
}
