use super::*;

pub(super) fn terminal_font_names(
    window: &mut Window,
    cx: &mut gpui::App,
    font_size: f32,
) -> Vec<String> {
    let mut names = cx.text_system().all_font_names();
    names.retain(|name| {
        crate::terminal::element::terminal_font_is_monospace(
            window,
            name.clone().into(),
            px(font_size),
        )
    });
    names.sort_unstable();
    names.dedup();
    names
}
