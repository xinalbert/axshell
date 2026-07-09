use std::time::Instant;

use gpui::SharedString;
use gpui_component::ThemeMode;

use crate::{app::TerminalFontMetrics, session::config::CursorStyle};

pub(crate) struct AppearanceState {
    pub(crate) follow_system_theme: bool,
    pub(crate) theme_mode: ThemeMode,
    pub(crate) light_theme_name: SharedString,
    pub(crate) dark_theme_name: SharedString,
    pub(crate) ui_font_size: f32,
    pub(crate) terminal_font_size: f32,
    pub(crate) terminal_font_metrics: TerminalFontMetrics,
    pub(crate) terminal_zoom_accumulator: f32,
    pub(crate) ui_font_family: SharedString,
    pub(crate) terminal_font_family: SharedString,
    pub(crate) cursor_style: CursorStyle,
    pub(crate) last_theme_sync: Instant,
}
