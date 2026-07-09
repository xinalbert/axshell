use std::ops::Range;

use alacritty_terminal::index::Side;
use alacritty_terminal::selection::SelectionType;
use gpui::{
    ClipboardItem, Context, Focusable as _, KeyDownEvent, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, Point, ScrollDelta, ScrollWheelEvent, Window, px,
};

use crate::{
    AxShell, TerminalBacktabKey, TerminalTabKey,
    terminal::{
        BackendCommand, FrozenRenderCell, TerminalComposition, TerminalFrozenSelection, encode_key,
    },
};

thread_local! {
    static LAST_DRAG_SCROLL: std::cell::Cell<Option<std::time::Instant>> = std::cell::Cell::new(None);
}

impl AxShell {
    pub(crate) fn on_terminal_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cmd_ctrl_pressed = event.keystroke.modifiers.platform;
        // If the search input is focused, skip terminal key processing
        // so the input can handle text entry, paste, etc. normally.
        if self
            .search
            .input
            .read(cx)
            .focus_handle(cx)
            .is_focused(window)
        {
            return;
        }

        if crate::app::keybinding_recorder::event_matches_action(&self.config, "PrevTab", event) {
            self.switch_workspace_tab(-1, window, cx);
            window.prevent_default();
            cx.stop_propagation();
            return;
        }

        if crate::app::keybinding_recorder::event_matches_action(&self.config, "NextTab", event) {
            self.switch_workspace_tab(1, window, cx);
            window.prevent_default();
            cx.stop_propagation();
            return;
        }

        // Pane navigation: Alt + h/j/k/l
        if event.keystroke.modifiers.alt
            && !event.keystroke.modifiers.shift
            && !event.keystroke.modifiers.control
            && !event.keystroke.modifiers.platform
        {
            match event.keystroke.key.to_ascii_lowercase().as_str() {
                "h" => self.focus_adjacent_pane("left"),
                "j" => self.focus_adjacent_pane("down"),
                "k" => self.focus_adjacent_pane("up"),
                "l" => self.focus_adjacent_pane("right"),
                "q" => {
                    if let Some(active_id) = self.active_tab.clone() {
                        self.close_tab(active_id, cx);
                    }
                }
                _ => return,
            }
            window.prevent_default();
            cx.stop_propagation();
            cx.notify();
            return;
        }

        // Pane split: Shift+Alt + h/j/k/l
        if event.keystroke.modifiers.shift
            && event.keystroke.modifiers.alt
            && !event.keystroke.modifiers.control
            && !event.keystroke.modifiers.platform
        {
            let direction = match event.keystroke.key.to_ascii_lowercase().as_str() {
                "h" => Some("left"),
                "j" => Some("down"),
                "k" => Some("up"),
                "l" => Some("right"),
                _ => None,
            };
            if let Some(dir) = direction {
                self.split_current_pane(dir, cx);
                window.prevent_default();
                cx.stop_propagation();
                cx.notify();
                return;
            }
        }

        if event.keystroke.modifiers.secondary() && event.keystroke.key == "," {
            self.show_settings_dialog(window, cx);
            window.prevent_default();
            cx.stop_propagation();
            return;
        }
        if event.keystroke.modifiers.shift
            && event.keystroke.modifiers.secondary()
            && event.keystroke.key == "o"
        {
            self.show_selector_dialog(window, cx);
            window.prevent_default();
            cx.stop_propagation();
            return;
        }
        if event.keystroke.modifiers.secondary() && event.keystroke.key.eq_ignore_ascii_case("c") {
            if let Some(text) = self.active_terminal_selection_text() {
                cx.write_to_clipboard(ClipboardItem::new_string(text));
                window.prevent_default();
                cx.stop_propagation();
                return;
            }
        }
        if event.keystroke.modifiers.secondary() && event.keystroke.key.eq_ignore_ascii_case("v") {
            if let Some(clipboard) = cx.read_from_clipboard() {
                if let Some(text) = clipboard.text() {
                    self.paste_into_terminal(&text, window, cx);
                    return;
                }
            }
        }

        // If the active tab is disconnected and user presses Enter, reconnect
        if event.keystroke.key == "enter"
            && !event.keystroke.modifiers.shift
            && !event.keystroke.modifiers.control
            && !event.keystroke.modifiers.alt
            && !event.keystroke.modifiers.platform
        {
            if let Some(progress) = &self.connection_progress {
                if progress.failed {
                    self.retry_connection_progress(cx);
                    window.prevent_default();
                    cx.stop_propagation();
                    return;
                }
            }

            let active_id = self.active_tab.clone();
            if let Some(active_id) = active_id {
                let is_disconnected = self
                    .tabs
                    .iter()
                    .find(|t| t.id == active_id)
                    .is_some_and(|tab| tab.disconnected_reason.is_some());
                if is_disconnected {
                    self.retry_disconnected_tab(&active_id, cx);
                    window.prevent_default();
                    cx.stop_propagation();
                    return;
                }
            }
        }

        if event.prefer_character_input {
            if let Some(text) = event.keystroke.key_char.as_deref() {
                if !text.is_empty()
                    && !event.keystroke.modifiers.control
                    && !event.keystroke.modifiers.function
                    && !event.keystroke.modifiers.platform
                {
                    self.send_terminal_input(text.as_bytes().to_vec(), window, cx);
                }
            }
            return;
        }

        let Some(active_id) = self.active_tab.clone() else {
            return;
        };
        {
            let Some(tab) = self.tabs.iter_mut().find(|t| t.id == active_id) else {
                return;
            };

            if tab.render_snapshot().display_offset > 0 {
                tab.scroll_to_bottom();
            }
        }
        self.clear_terminal_selection_for_tab(&active_id);

        let Some(tab) = self.tabs.iter_mut().find(|t| t.id == active_id) else {
            return;
        };
        let app_cursor_mode = tab.app_cursor_mode();
        if let Some(bytes) = encode_key(&event.keystroke, app_cursor_mode, false) {
            self.send_terminal_input(bytes, window, cx);
        }
    }

    pub(crate) fn on_terminal_tab_action(
        &mut self,
        _: &TerminalTabKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.send_terminal_input(vec![b'\t'], window, cx);
    }

    pub(crate) fn on_terminal_backtab_action(
        &mut self,
        _: &TerminalBacktabKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.send_terminal_input(b"\x1b[Z".to_vec(), window, cx);
    }

    fn send_terminal_input(&mut self, bytes: Vec<u8>, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_id) = self.active_tab.clone() else {
            return;
        };
        self.clear_terminal_marked_text(window, cx);
        {
            let Some(tab) = self.tabs.iter_mut().find(|t| t.id == active_id) else {
                return;
            };

            if tab.render_snapshot().display_offset > 0 {
                tab.scroll_to_bottom();
            }
        }
        self.clear_terminal_selection_for_tab(&active_id);
        let Some(tab) = self.tabs.iter_mut().find(|t| t.id == active_id) else {
            return;
        };
        tab.send_backend(BackendCommand::Input(bytes));
        window.prevent_default();
        cx.stop_propagation();
        cx.notify();
    }

    pub(crate) fn active_terminal_selection_text(&self) -> Option<String> {
        let active_id = self.active_tab.as_ref()?;
        if let Some(frozen) = self
            .terminal_frozen_selection
            .as_ref()
            .filter(|frozen| frozen.tab_id == *active_id && !frozen.text.is_empty())
        {
            return Some(frozen.text.clone());
        }

        self.tabs
            .iter()
            .find(|tab| &tab.id == active_id)
            .and_then(|tab| tab.selection_text())
    }

    pub(crate) fn paste_into_terminal(
        &mut self,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_id) = self.active_tab.clone() else {
            return;
        };
        {
            let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == active_id) else {
                return;
            };

            if tab.render_snapshot().display_offset > 0 {
                tab.scroll_to_bottom();
            }
        }
        self.clear_terminal_selection_for_tab(&active_id);
        let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == active_id) else {
            return;
        };
        tab.paste_text(text);
        window.prevent_default();
        cx.stop_propagation();
        cx.notify();
    }

    pub(crate) fn terminal_accepts_text_input(&self) -> bool {
        self.active_tab.is_some()
    }

    pub(crate) fn terminal_marked_text_range(&self) -> Option<Range<usize>> {
        let active_id = self.active_tab.as_ref()?;
        self.terminal_composition
            .as_ref()
            .filter(|composition| composition.tab_id == *active_id)
            .map(|composition| 0..composition.text.encode_utf16().count())
    }

    pub(crate) fn terminal_composition_for_tab(&self, tab_id: &str) -> Option<TerminalComposition> {
        self.terminal_composition
            .as_ref()
            .filter(|composition| composition.tab_id == tab_id && !composition.text.is_empty())
            .cloned()
    }

    pub(crate) fn terminal_frozen_selection_for_tab(
        &self,
        tab_id: &str,
    ) -> Option<TerminalFrozenSelection> {
        self.terminal_frozen_selection
            .as_ref()
            .filter(|frozen| frozen.tab_id == tab_id)
            .cloned()
    }

    pub(crate) fn clear_terminal_selection_for_tab(&mut self, tab_id: &str) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
            tab.clear_selection();
        }
        if self
            .terminal_frozen_selection
            .as_ref()
            .is_some_and(|frozen| frozen.tab_id == tab_id)
        {
            self.terminal_frozen_selection = None;
        }
    }

    pub(crate) fn set_terminal_marked_text(
        &mut self,
        text: String,
        selected_range_utf16: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_id) = self.active_tab.clone() else {
            return;
        };

        if text.is_empty() {
            self.clear_terminal_marked_text(window, cx);
            return;
        }

        let selected_range_utf16 = normalize_utf16_range(selected_range_utf16, &text);
        let existing_anchor = self
            .terminal_composition
            .as_ref()
            .filter(|composition| composition.tab_id == active_id)
            .map(|composition| (composition.anchor_row, composition.anchor_col));
        let cursor_anchor = self
            .active_snapshot()
            .and_then(|snapshot| snapshot.cursor.map(|cursor| (cursor.row, cursor.col)));
        let (anchor_row, anchor_col) = existing_anchor.or(cursor_anchor).unwrap_or((0, 0));

        self.terminal_composition = Some(TerminalComposition {
            tab_id: active_id,
            text,
            selected_range_utf16,
            anchor_row,
            anchor_col,
        });
        window.invalidate_character_coordinates();
        cx.notify();
    }

    pub(crate) fn clear_terminal_marked_text(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.terminal_composition.take().is_some() {
            window.invalidate_character_coordinates();
            cx.notify();
        }
    }

    pub(crate) fn commit_terminal_ime_text(
        &mut self,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_id) = self.active_tab.clone() else {
            return;
        };
        {
            let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == active_id) else {
                return;
            };
            if tab.render_snapshot().display_offset > 0 {
                tab.scroll_to_bottom();
            }
        }
        self.clear_terminal_selection_for_tab(&active_id);
        self.terminal_composition = None;
        let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == active_id) else {
            return;
        };
        if !text.is_empty() {
            tab.send_backend(BackendCommand::Input(text.as_bytes().to_vec()));
        }
        window.invalidate_character_coordinates();
        cx.notify();
    }

    pub(crate) fn on_terminal_right_click(
        &mut self,
        _event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.config.right_click_copy_paste() {
            return;
        }

        let mut handled = false;
        if let Some(text) = self.active_terminal_selection_text() {
            if !text.is_empty() {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));

                let active_id = self.active_tab.clone();
                if let Some(active_id) = active_id {
                    self.clear_terminal_selection_for_tab(&active_id);
                }
                cx.notify();
                handled = true;
            }
        }

        if !handled {
            if let Some(clipboard_item) = cx.read_from_clipboard() {
                if let Some(text) = clipboard_item.text() {
                    if !text.is_empty() {
                        self.paste_into_terminal(&text, window, cx);
                    }
                }
            }
        }
    }

    pub(crate) fn begin_terminal_selection(
        &mut self,
        event: &MouseDownEvent,
        cx: &mut Context<Self>,
    ) {
        let click_count = event.click_count.max(1);
        let selection_type = match click_count {
            1 => SelectionType::Simple,
            2 => SelectionType::Semantic,
            3 => SelectionType::Lines,
            _ => SelectionType::Simple,
        };
        let Some((row, col, side)) = self.terminal_grid_point_and_side(event.position) else {
            return;
        };
        let Some(active_id) = self.active_tab.clone() else {
            return;
        };
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == active_id) {
            tab.begin_selection(row, col, side, selection_type);
            self.terminal_selecting = true;
        }
        self.capture_active_terminal_frozen_selection();
        cx.notify();
    }

    pub(crate) fn on_terminal_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Handle split drag
        if self.dragging_splitter.is_some() {
            if event.pressed_button == Some(MouseButton::Left) {
                self.on_split_drag_move(event, window, cx);
                cx.notify();
            } else {
                self.end_drag_split();
                cx.notify();
            }
            return;
        }

        // Track URL hover
        let mut hovered_url = None;
        let cmd_ctrl_pressed = event.modifiers.platform;
        if let Some((row, col, _side)) = self.terminal_grid_point_and_side(event.position) {
            if let Some(snapshot) = self.active_snapshot() {
                if let Some(active_id) = &self.active_tab {
                    if let Some((url, url_cells)) = crate::terminal::highlight::find_url_at_cell(
                        &snapshot.cells,
                        snapshot.rows,
                        row,
                        col,
                    ) {
                        hovered_url = Some(crate::app::HoveredUrl {
                            url,
                            tab_id: active_id.clone(),
                            cells: url_cells,
                        });
                    }
                }
            }
        }

        if self.hovered_url != hovered_url || self.cmd_ctrl_pressed != cmd_ctrl_pressed {
            self.hovered_url = hovered_url;
            self.cmd_ctrl_pressed = cmd_ctrl_pressed;
            cx.notify();
        }

        if !self.terminal_selecting || event.pressed_button != Some(MouseButton::Left) {
            return;
        }
        let Some((row, col, side)) = self.terminal_grid_point_and_side(event.position) else {
            return;
        };
        let Some(active_id) = self.active_tab.clone() else {
            return;
        };
        let snapshot = match self.active_snapshot() {
            Some(s) => s,
            None => return,
        };
        let max_row = snapshot.rows.saturating_sub(1);

        let mut scroll_delta = 0i32;
        if max_row >= 6 {
            if row <= 2 || row >= max_row.saturating_sub(2) {
                let now = std::time::Instant::now();
                let should_scroll = LAST_DRAG_SCROLL.with(|last| {
                    if let Some(last_time) = last.get() {
                        if now.duration_since(last_time) >= std::time::Duration::from_millis(80) {
                            last.set(Some(now));
                            true
                        } else {
                            false
                        }
                    } else {
                        last.set(Some(now));
                        true
                    }
                });

                if should_scroll {
                    if row == 0 {
                        scroll_delta = 2;
                    } else if row == 1 {
                        scroll_delta = 1;
                    } else if row == 2 {
                        scroll_delta = 1;
                    } else if row == max_row {
                        scroll_delta = -2;
                    } else if row == max_row.saturating_sub(1) {
                        scroll_delta = -1;
                    } else if row == max_row.saturating_sub(2) {
                        scroll_delta = -1;
                    }
                }
            } else {
                LAST_DRAG_SCROLL.with(|last| last.set(None));
            }
        }

        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == active_id) {
            if scroll_delta != 0 {
                tab.scroll_history(scroll_delta);
            }
            tab.update_selection(row, col, side);
        }
        self.capture_active_terminal_frozen_selection();
        cx.notify();
    }

    pub(crate) fn on_terminal_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.dragging_splitter.is_some() {
            self.end_drag_split();
        }
        self.terminal_selecting = false;
        cx.notify();
    }

    pub(crate) fn terminal_grid_point_and_side(
        &self,
        position: Point<Pixels>,
    ) -> Option<(usize, usize, Side)> {
        let active_id = self.active_tab.as_ref()?;
        let bounds = self.terminal_bounds.get(active_id)?;
        if !bounds.contains(&position) {
            // Try other pane bounds
            for (_, b) in &self.terminal_bounds {
                if b.contains(&position) {
                    // Found a different pane - focus it
                    // (this path is for click-to-focus; handled via focus_terminal)
                    return None;
                }
            }
            return None;
        }
        let local_x = (position.x - bounds.origin.x).max(px(0.));
        let local_y = (position.y - bounds.origin.y).max(px(0.));
        let cell_width = px(self.terminal_cell_width());
        let line_height = px(self.terminal_line_height());
        let snapshot = self.active_snapshot()?;
        let max_col = snapshot.cols.saturating_sub(1);
        let max_row = snapshot.rows.saturating_sub(1);
        let col = ((local_x / cell_width).floor() as usize).min(max_col);
        let row = ((local_y / line_height).floor() as usize).min(max_row);
        let cell_offset_x = px(local_x.as_f32() % cell_width.as_f32());
        let side = if cell_offset_x >= (cell_width / 2.) {
            Side::Right
        } else {
            Side::Left
        };
        Some((row, col, side))
    }

    pub(crate) fn on_terminal_scroll(
        &mut self,
        event: &ScrollWheelEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Platform modifier (Cmd on macOS, Ctrl on Windows/Linux) + scroll → zoom terminal font size
        if event.modifiers.platform {
            let delta = match event.delta {
                ScrollDelta::Lines(point) => point.y as f32 * 20.0,
                ScrollDelta::Pixels(point) => point.y.as_f32(),
            };
            self.appearance.terminal_zoom_accumulator += delta;
            let step = 20.0;
            if self.appearance.terminal_zoom_accumulator.abs() >= step {
                let zoom_steps = (self.appearance.terminal_zoom_accumulator / step).trunc();
                self.appearance.terminal_zoom_accumulator -= zoom_steps * step;
                self.change_terminal_font_size(zoom_steps * 0.5, cx);
            }
            window.prevent_default();
            cx.stop_propagation();
            return;
        }

        let Some(active_id) = self.active_tab.clone() else {
            return;
        };

        // Get coordinates before mutably borrowing tabs
        let grid_point = self.terminal_grid_point_and_side(event.position);

        let line_height = self.terminal_line_height();

        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == active_id) {
            let delta_lines = match event.delta {
                ScrollDelta::Lines(point) => point.y.round() as i32,
                ScrollDelta::Pixels(point) => {
                    tab.scroll_pixel_y += point.y.as_f32();
                    let lines = (tab.scroll_pixel_y / line_height).trunc() as i32;
                    tab.scroll_pixel_y -= (lines as f32) * line_height;
                    lines
                }
            };

            if delta_lines == 0 {
                return;
            }

            let mouse_mode = tab.mouse_tracking_mode();

            if mouse_mode.mouse_tracking {
                if let Some((row, col, _)) = grid_point {
                    let button = if delta_lines > 0 { 64 } else { 65 };
                    let times = delta_lines.abs();
                    let mut bytes = Vec::new();
                    for _ in 0..times {
                        if mouse_mode.sgr_mouse {
                            bytes.extend_from_slice(
                                format!("\x1b[<{};{};{}M", button, col + 1, row + 1).as_bytes(),
                            );
                        } else {
                            if col < 223 && row < 223 {
                                bytes.extend_from_slice(b"\x1b[M");
                                bytes.push(button as u8 + 32);
                                bytes.push(col as u8 + 33);
                                bytes.push(row as u8 + 33);
                            }
                        }
                    }
                    if !bytes.is_empty() {
                        tab.send_backend(crate::terminal::BackendCommand::Input(bytes));
                    }
                }
                window.prevent_default();
                cx.stop_propagation();
                return;
            } else if mouse_mode.alternate_scroll {
                let times = delta_lines.abs();
                let code = if delta_lines > 0 { b'A' } else { b'B' };
                let mut bytes = Vec::with_capacity((times * 3) as usize);
                for _ in 0..times {
                    bytes.extend_from_slice(&[b'\x1b', b'O', code]);
                }
                if !bytes.is_empty() {
                    tab.send_backend(crate::terminal::BackendCommand::Input(bytes));
                }
                window.prevent_default();
                cx.stop_propagation();
                return;
            }

            tab.scroll_history(delta_lines);
            window.prevent_default();
            cx.stop_propagation();
            cx.notify();
        }
    }
}

impl AxShell {
    fn capture_active_terminal_frozen_selection(&mut self) {
        let Some(active_id) = self.active_tab.clone() else {
            self.terminal_frozen_selection = None;
            return;
        };
        let Some(tab) = self.tabs.iter().find(|tab| tab.id == active_id) else {
            self.terminal_frozen_selection = None;
            return;
        };

        let snapshot = tab.render_snapshot();
        let Some(selection) = snapshot.selection else {
            self.terminal_frozen_selection = None;
            return;
        };
        let (start_row, end_row) = selection_row_range(selection);
        let cells = snapshot
            .cells
            .iter()
            .filter(|cell| {
                let row = cell.row.max(0) as usize;
                row >= start_row && row <= end_row
            })
            .filter_map(|cell| {
                let bottom_index = bottom_index_for_row(snapshot.rows, cell.row)?;
                Some(FrozenRenderCell {
                    bottom_index,
                    col: cell.col,
                    cell: cell.cell.clone(),
                })
            })
            .collect();
        let highlights = snapshot
            .highlights
            .iter()
            .filter_map(|(&(row, col), &color)| {
                let row_usize = row.max(0) as usize;
                if row_usize < start_row || row_usize > end_row {
                    return None;
                }
                let bottom_index = bottom_index_for_row(snapshot.rows, row)?;
                Some(((bottom_index, col), color))
            })
            .collect();

        self.terminal_frozen_selection = Some(TerminalFrozenSelection {
            tab_id: active_id,
            selection,
            viewport_rows: snapshot.rows,
            history_size: snapshot.history_size,
            display_offset: snapshot.display_offset,
            cells,
            highlights,
            text: tab.selection_text().unwrap_or_default(),
        });
    }
}

fn selection_row_range(selection: crate::terminal::ViewportSelection) -> (usize, usize) {
    (
        selection.start_row.min(selection.end_row),
        selection.start_row.max(selection.end_row),
    )
}

fn bottom_index_for_row(rows: usize, row: i32) -> Option<usize> {
    let row = usize::try_from(row).ok()?;
    (row < rows).then_some(rows.saturating_sub(1).saturating_sub(row))
}

fn normalize_utf16_range(range: Option<Range<usize>>, text: &str) -> Option<Range<usize>> {
    let len = text.encode_utf16().count();
    let range = range?;
    let start = range.start.min(len);
    let end = range.end.min(len);
    if start <= end {
        Some(start..end)
    } else {
        Some(end..start)
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_utf16_range;

    #[test]
    fn terminal_ime_selection_range_is_clamped_to_utf16_len() {
        assert_eq!(normalize_utf16_range(Some(1..99), "a中"), Some(1..2));
    }

    #[test]
    fn terminal_ime_selection_range_handles_surrogate_pairs() {
        assert_eq!(normalize_utf16_range(Some(0..99), "a🏀"), Some(0..3));
    }
}
