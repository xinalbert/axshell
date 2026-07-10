use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

fn terminal_font_names(window: &mut Window, cx: &mut gpui::App, font_size: f32) -> Vec<String> {
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

pub(super) fn settings_fonts_page(view: &gpui::Entity<AxShell>, shell: &AxShell) -> SettingPage {
    let ui_font_size = shell.appearance.ui_font_size;
    let terminal_font_size = shell.appearance.terminal_font_size;
    let ui_font_family = shell.appearance.ui_font_family.to_string();
    let terminal_font_family = shell.appearance.terminal_font_family.to_string();
    let cursor_style = shell.appearance.cursor_style;

    SettingPage::new(t!("settings_fonts").to_string())
        .icon(IconName::Settings)
        .group(
            SettingGroup::new()
                .title(t!("settings_group_font").to_string())
                .item(SettingItem::new(
                    t!("ui_font_size").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        move |_, window, _cx| {
                            h_flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    Button::new("ui-font-size-down")
                                        .small()
                                        .label("-")
                                        .on_click(window.listener_for(&view, |this, _, _, cx| {
                                            this.change_ui_font_size(-1.0, cx)
                                        })),
                                )
                                .child(
                                    div()
                                        .min_w(px(64.))
                                        .text_center()
                                        .child(format!("{ui_font_size:.0}px")),
                                )
                                .child(Button::new("ui-font-size-up").small().label("+").on_click(
                                    window.listener_for(&view, |this, _, _, cx| {
                                        this.change_ui_font_size(1.0, cx)
                                    }),
                                ))
                                .into_any_element()
                        }
                    }),
                ))
                .item(SettingItem::new(
                    t!("terminal_font_size").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        move |_, window, _cx| {
                            h_flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    Button::new("terminal-font-size-down")
                                        .small()
                                        .label("-")
                                        .on_click(window.listener_for(&view, |this, _, _, cx| {
                                            this.change_terminal_font_size(-1.0, cx)
                                        })),
                                )
                                .child(
                                    div()
                                        .min_w(px(64.))
                                        .text_center()
                                        .child(format!("{terminal_font_size:.0}px")),
                                )
                                .child(
                                    Button::new("terminal-font-size-up")
                                        .small()
                                        .label("+")
                                        .on_click(window.listener_for(&view, |this, _, _, cx| {
                                            this.change_terminal_font_size(1.0, cx)
                                        })),
                                )
                                .into_any_element()
                        }
                    }),
                ))
                .item(SettingItem::new(
                    t!("ui_font_family").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        let current = ui_font_family.clone();
                        move |_, _window, cx| {
                            Button::new("ui-font-dropdown")
                                .small()
                                .icon(IconName::ChevronsUpDown)
                                .label({
                                    let names = cx.text_system().all_font_names();
                                    let using_system_maple = crate::app::theme::USING_SYSTEM_MAPLE
                                        .load(std::sync::atomic::Ordering::Relaxed);
                                    if current == *".SystemUIFont"
                                        || current.is_empty()
                                        || !names.contains(&current)
                                    {
                                        t!("system_default").to_string()
                                    } else if !using_system_maple && current == "Maple Mono NF CN" {
                                        format!("Maple Mono NF CN ({})", t!("software_builtin"))
                                    } else {
                                        current.clone()
                                    }
                                })
                                .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                    let view = view.clone();
                                    let current = current.clone();
                                    move |mut menu, window, cx| {
                                        let mut names = cx.text_system().all_font_names();
                                        menu = menu.min_w(200.).max_h(px(320.)).scrollable(true);
                                        menu = menu.item(
                                            PopupMenuItem::new(t!("system_default").to_string())
                                                .checked(
                                                    current == *".SystemUIFont"
                                                        || current.is_empty(),
                                                )
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, window, cx| {
                                                        this.change_ui_font_family(
                                                            ".SystemUIFont",
                                                            window,
                                                            cx,
                                                        );
                                                    },
                                                )),
                                        );
                                        let maple_font = "Maple Mono NF CN".to_string();
                                        let using_system_maple =
                                            crate::app::theme::USING_SYSTEM_MAPLE
                                                .load(std::sync::atomic::Ordering::Relaxed);
                                        if !using_system_maple && names.contains(&maple_font) {
                                            names.retain(|name| name != &maple_font);
                                            menu = menu
                                                .item(
                                                    PopupMenuItem::new(format!(
                                                        "{} ({})",
                                                        maple_font,
                                                        t!("software_builtin")
                                                    ))
                                                    .checked(current == maple_font)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        |this, _, window, cx| {
                                                            this.change_ui_font_family(
                                                                "Maple Mono NF CN",
                                                                window,
                                                                cx,
                                                            );
                                                        },
                                                    )),
                                                )
                                                .separator();
                                        }
                                        for name in names {
                                            let checked = name == current;
                                            menu = menu.item(
                                                PopupMenuItem::new(name.clone())
                                                    .checked(checked)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        move |this, _, window, cx| {
                                                            this.change_ui_font_family(
                                                                &name, window, cx,
                                                            );
                                                        },
                                                    )),
                                            );
                                        }
                                        menu
                                    }
                                })
                                .into_any_element()
                        }
                    }),
                ))
                .item(SettingItem::new(
                    t!("terminal_font_family").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        let current = terminal_font_family.clone();
                        move |_, window, _cx| {
                            Button::new("terminal-font-dropdown")
                                .small()
                                .icon(IconName::ChevronsUpDown)
                                .label({
                                    let using_system_maple = crate::app::theme::USING_SYSTEM_MAPLE
                                        .load(std::sync::atomic::Ordering::Relaxed);
                                    if !using_system_maple && current == "Maple Mono NF CN" {
                                        format!("Maple Mono NF CN ({})", t!("software_builtin"))
                                    } else if !crate::terminal::element::terminal_font_is_monospace(
                                        window,
                                        current.clone().into(),
                                        px(terminal_font_size),
                                    ) {
                                        format!("Maple Mono NF CN ({})", t!("software_builtin"))
                                    } else {
                                        current.clone()
                                    }
                                })
                                .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                    let view = view.clone();
                                    let current = current.clone();
                                    move |mut menu, window, cx| {
                                        let mut names =
                                            terminal_font_names(window, cx, terminal_font_size);
                                        menu = menu.min_w(200.).max_h(px(320.)).scrollable(true);
                                        let maple_font = "Maple Mono NF CN".to_string();
                                        let using_system_maple =
                                            crate::app::theme::USING_SYSTEM_MAPLE
                                                .load(std::sync::atomic::Ordering::Relaxed);
                                        if !using_system_maple && names.contains(&maple_font) {
                                            names.retain(|name| name != &maple_font);
                                            menu = menu
                                                .item(
                                                    PopupMenuItem::new(format!(
                                                        "{} ({})",
                                                        maple_font,
                                                        t!("software_builtin")
                                                    ))
                                                    .checked(current == maple_font)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        |this, _, _window, cx| {
                                                            this.change_terminal_font_family(
                                                                "Maple Mono NF CN",
                                                                cx,
                                                            );
                                                        },
                                                    )),
                                                )
                                                .separator();
                                        }
                                        for name in names {
                                            let checked = name == current;
                                            menu = menu.item(
                                                PopupMenuItem::new(name.clone())
                                                    .checked(checked)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        move |this, _, _window, cx| {
                                                            this.change_terminal_font_family(
                                                                &name, cx,
                                                            );
                                                        },
                                                    )),
                                            );
                                        }
                                        menu
                                    }
                                })
                                .into_any_element()
                        }
                    }),
                ))
                .item(SettingItem::new(
                    t!("cursor_style").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        move |_, _window, _cx| {
                            use crate::config::CursorStyle;

                            Button::new("cursor-style-dropdown")
                                .small()
                                .icon(IconName::ChevronsUpDown)
                                .label(match cursor_style {
                                    CursorStyle::Default => t!("cursor_style_default").to_string(),
                                    CursorStyle::Blink => t!("cursor_style_blink").to_string(),
                                    CursorStyle::Beam => t!("cursor_style_beam").to_string(),
                                    CursorStyle::BeamBlink => {
                                        t!("cursor_style_beam_blink").to_string()
                                    }
                                })
                                .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                    let view = view.clone();
                                    move |mut menu, window, _cx| {
                                        use crate::config::CursorStyle;

                                        menu = menu.min_w(160.).max_h(px(320.)).scrollable(true);
                                        for style in [
                                            CursorStyle::Default,
                                            CursorStyle::Blink,
                                            CursorStyle::Beam,
                                            CursorStyle::BeamBlink,
                                        ] {
                                            let checked = style == cursor_style;
                                            let label = match style {
                                                CursorStyle::Default => {
                                                    t!("cursor_style_default").to_string()
                                                }
                                                CursorStyle::Blink => {
                                                    t!("cursor_style_blink").to_string()
                                                }
                                                CursorStyle::Beam => {
                                                    t!("cursor_style_beam").to_string()
                                                }
                                                CursorStyle::BeamBlink => {
                                                    t!("cursor_style_beam_blink").to_string()
                                                }
                                            };
                                            menu = menu.item(
                                                PopupMenuItem::new(label)
                                                    .checked(checked)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        move |this, _, _window, cx| {
                                                            this.change_cursor_style(style, cx);
                                                        },
                                                    )),
                                            );
                                        }
                                        menu
                                    }
                                })
                                .into_any_element()
                        }
                    }),
                )),
        )
}
