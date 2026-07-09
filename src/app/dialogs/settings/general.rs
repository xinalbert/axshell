use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

use super::fonts::terminal_font_names;

pub(super) fn settings_general_page(
    view: &gpui::Entity<AxShell>,
    shell: &AxShell,
    _cx: &mut Context<AxShell>,
) -> SettingPage {
    let view_clone_for_general = view.clone();
    let follow_system_theme = shell.appearance.follow_system_theme;
    let theme_mode_is_dark = shell.appearance.theme_mode.is_dark();
    let light_theme_name = shell.appearance.light_theme_name.to_string();
    let dark_theme_name = shell.appearance.dark_theme_name.to_string();
    let title_bar_style = shell.config.effective_title_bar_style();
    let ui_font_size = shell.appearance.ui_font_size;
    let terminal_font_size = shell.appearance.terminal_font_size;
    let ui_font_family = shell.appearance.ui_font_family.to_string();
    let terminal_font_family = shell.appearance.terminal_font_family.to_string();
    let cursor_style = shell.appearance.cursor_style;
    let right_click_copy_paste = shell.config.right_click_copy_paste();
    let keyword_highlight = shell.config.keyword_highlight();
    let lock_layout = shell.config.lock_layout();
    let show_monitoring_dashboard = shell.config.show_monitoring_dashboard();
    let monitoring_position = shell.config.monitoring_position().to_string();
    let current_locale = shell.config.locale().to_string();
    SettingPage::new(t!("settings_general").to_string())
                                            .icon(IconName::Settings)
                                            .default_open(true)
                                            .group(
                                                SettingGroup::new()
                                                    .title(t!("settings_group_appearance").to_string())
                                                    .item(
                                                        SettingItem::new(
                                                            t!("theme_mode").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let follow_system = follow_system_theme;
                                                                let is_dark_mode = theme_mode_is_dark;
                                                                move |_, _window, _cx| {
                                                                    Button::new("theme-mode-dropdown")
                                                                        .small()
                                                                        .icon(if follow_system { IconName::Sun } else if is_dark_mode { IconName::Moon } else { IconName::Sun })
                                                                        .label(if follow_system { t!("follow_system").to_string() } else if is_dark_mode { t!("use_dark_mode").to_string() } else { t!("use_light_mode").to_string() })
                                                                        .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                                                            let view = view.clone();
                                                                            let follow_system = follow_system;
                                                                            let is_dark_mode = is_dark_mode;
                                                                            move |mut menu, window, _cx| {
                                                                                menu = menu.min_w(160.)
                                                                                    .item(
                                                                                        PopupMenuItem::new(t!("follow_system").to_string())
                                                                                            .checked(follow_system)
                                                                                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                                                this.set_follow_system_theme(true, window, cx)
                                                                                            }))
                                                                                    )
                                                                                    .item(
                                                                                        PopupMenuItem::new(t!("use_light_mode").to_string())
                                                                                            .checked(!follow_system && !is_dark_mode)
                                                                                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                                                this.switch_theme_mode(gpui_component::ThemeMode::Light, window, cx)
                                                                                            }))
                                                                                    )
                                                                                    .item(
                                                                                        PopupMenuItem::new(t!("use_dark_mode").to_string())
                                                                                            .checked(!follow_system && is_dark_mode)
                                                                                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                                                this.switch_theme_mode(gpui_component::ThemeMode::Dark, window, cx)
                                                                                            }))
                                                                                    );
                                                                                menu
                                                                            }
                                                                        })
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("light_theme").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let current_theme = light_theme_name.clone();
                                                                move |_, _window, _cx| {
                                                                    Button::new("light-theme-dropdown")
                                                                        .small()
                                                                        .icon(IconName::Sun)
                                                                        .label(current_theme.clone())
                                                                        .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                                                            let view = view.clone();
                                                                            let current_theme = current_theme.clone();
                                                                            move |mut menu, window, cx| {
                                                                                let themes = gpui_component::ThemeRegistry::global(cx).sorted_themes();
                                                                                let light_themes: Vec<_> = themes.into_iter().filter(|t| !t.mode.is_dark()).map(|t| t.name.clone()).collect();
                                                                                menu = menu.min_w(160.).max_h(px(320.)).scrollable(true);
                                                                                for theme_name in light_themes {
                                                                                    let checked = theme_name == current_theme;
                                                                                    menu = menu.item(
                                                                                        PopupMenuItem::new(theme_name.clone())
                                                                                            .checked(checked)
                                                                                            .on_click(window.listener_for(&view, move |this, _, window, cx| {
                                                                                                this.apply_theme(theme_name.clone(), window, cx)
                                                                                            }))
                                                                                    );
                                                                                }
                                                                                menu
                                                                            }
                                                                        })
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("dark_theme").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let current_theme = dark_theme_name.clone();
                                                                move |_, _window, _cx| {
                                                                    Button::new("dark-theme-dropdown")
                                                                        .small()
                                                                        .icon(IconName::Moon)
                                                                        .label(current_theme.clone())
                                                                        .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                                                            let view = view.clone();
                                                                            let current_theme = current_theme.clone();
                                                                            move |mut menu, window, cx| {
                                                                                let themes = gpui_component::ThemeRegistry::global(cx).sorted_themes();
                                                                                let dark_themes: Vec<_> = themes.into_iter().filter(|t| t.mode.is_dark()).map(|t| t.name.clone()).collect();
                                                                                menu = menu.min_w(160.).max_h(px(320.)).scrollable(true);
                                                                                for theme_name in dark_themes {
                                                                                    let checked = theme_name == current_theme;
                                                                                    menu = menu.item(
                                                                                        PopupMenuItem::new(theme_name.clone())
                                                                                            .checked(checked)
                                                                                            .on_click(window.listener_for(&view, move |this, _, window, cx| {
                                                                                                this.apply_theme(theme_name.clone(), window, cx)
                                                                                            }))
                                                                                    );
                                                                                }
                                                                                menu
                                                                            }
                                                                        })
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            format!("{}{}", t!("title_bar_style"), t!("restart_hint")),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let current_style = title_bar_style;
                                                                move |_, _window, _cx| {
                                                                    let supports_integrated =
                                                                        cfg!(target_os = "macos");
                                                                    Button::new("title-bar-style-dropdown")
                                                                        .small()
                                                                        .label(match current_style {
                                                                            crate::session::config::TitleBarStyle::Native => t!("title_bar_native").to_string(),
                                                                            crate::session::config::TitleBarStyle::Integrated => t!("title_bar_integrated").to_string(),
                                                                        })
                                                                        .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                                                            let view = view.clone();
                                                                            let current_style = current_style;
                                                                            move |mut menu, window, _cx| {
                                                                                menu = menu.min_w(160.)
                                                                                    .item(
                                                                                        PopupMenuItem::new(t!("title_bar_native").to_string())
                                                                                            .checked(current_style == crate::session::config::TitleBarStyle::Native)
                                                                                            .on_click(window.listener_for(&view, |this, _, _, cx| {
                                                                                                this.config.set_title_bar_style(crate::session::config::TitleBarStyle::Native);
                                                                                                let _ = this.config.save();
                                                                                                cx.notify();
                                                                                            }))
                                                                                    );
                                                                                if supports_integrated {
                                                                                    menu = menu.item(
                                                                                        PopupMenuItem::new(t!("title_bar_integrated").to_string())
                                                                                            .checked(current_style == crate::session::config::TitleBarStyle::Integrated)
                                                                                            .on_click(window.listener_for(&view, |this, _, _, cx| {
                                                                                                this.config.set_title_bar_style(crate::session::config::TitleBarStyle::Integrated);
                                                                                                let _ = this.config.save();
                                                                                                cx.notify();
                                                                                            }))
                                                                                    );
                                                                                }
                                                                                menu
                                                                            }
                                                                        })
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                            )
                                            .group(
                                                SettingGroup::new()
                                                    .title(t!("settings_group_font").to_string())
                                                    .item(
                                                        SettingItem::new(
                                                            t!("ui_font_size").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let current_ui_font_size = ui_font_size;
                                                                move |_, window, _cx| {
                                                                    h_flex()
                                                                        .items_center()
                                                                        .gap_3()
                                                                        .child(Button::new("ui-font-size-down").small().label("-").on_click(window.listener_for(&view, |this, _, _, cx| this.change_ui_font_size(-1.0, cx))))
                                                                        .child(div().min_w(px(64.)).text_center().child(format!("{:.0}px", current_ui_font_size)))
                                                                        .child(Button::new("ui-font-size-up").small().label("+").on_click(window.listener_for(&view, |this, _, _, cx| this.change_ui_font_size(1.0, cx))))
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("terminal_font_size").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let current_terminal_font_size = terminal_font_size;
                                                                move |_, window, _cx| {
                                                                    h_flex()
                                                                        .items_center()
                                                                        .gap_3()
                                                                        .child(Button::new("terminal-font-size-down").small().label("-").on_click(window.listener_for(&view, |this, _, _, cx| this.change_terminal_font_size(-1.0, cx))))
                                                                        .child(div().min_w(px(64.)).text_center().child(format!("{:.0}px", current_terminal_font_size)))
                                                                        .child(Button::new("terminal-font-size-up").small().label("+").on_click(window.listener_for(&view, |this, _, _, cx| this.change_terminal_font_size(1.0, cx))))
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("ui_font_family").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let current = ui_font_family.clone();
                                                                move |_, _window, cx| {
                                                                    Button::new("ui-font-dropdown")
                                                                        .small()
                                                                        .icon(IconName::ChevronsUpDown)
                                                                        .label({
                                                                            let names = cx.text_system().all_font_names();
                                                                            let using_system_maple = crate::app::theme::USING_SYSTEM_MAPLE.load(std::sync::atomic::Ordering::Relaxed);
                                                                            if current == *".SystemUIFont" || current.is_empty() || !names.contains(&current) {
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
                                                                                        .checked(current == *".SystemUIFont" || current.is_empty())
                                                                                        .on_click(window.listener_for(&view, move |this, _, window, cx| {
                                                                                            this.change_ui_font_family(".SystemUIFont", window, cx);
                                                                                        }))
                                                                                );
                                                                                let maple_font = "Maple Mono NF CN".to_string();
                                                                                let using_system_maple = crate::app::theme::USING_SYSTEM_MAPLE.load(std::sync::atomic::Ordering::Relaxed);
                                                                                if !using_system_maple && names.contains(&maple_font) {
                                                                                    names.retain(|n| n != &maple_font);
                                                                                    menu = menu.item(
                                                                                        PopupMenuItem::new(format!("{} ({})", maple_font, t!("software_builtin")))
                                                                                            .checked(current == maple_font)
                                                                                            .on_click(window.listener_for(&view, move |this, _, window, cx| {
                                                                                                this.change_ui_font_family("Maple Mono NF CN", window, cx);
                                                                                            }))
                                                                                    ).separator();
                                                                                }
                                                                                for name in names {
                                                                                    let checked = name == current;
                                                                                    menu = menu.item(
                                                                                        PopupMenuItem::new(name.clone())
                                                                                            .checked(checked)
                                                                                            .on_click(window.listener_for(&view, move |this, _, window, cx| {
                                                                                                this.change_ui_font_family(&name, window, cx);
                                                                                            }))
                                                                                    );
                                                                                }
                                                                                menu
                                                                            }
                                                                        })
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("terminal_font_family").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let current = terminal_font_family.clone();
                                                                let current_terminal_font_size = terminal_font_size;
                                                                move |_, window, _cx| {
                                                                    Button::new("terminal-font-dropdown")
                                                                        .small()
                                                                        .icon(IconName::ChevronsUpDown)
                                                                        .label({
                                                                            let using_system_maple = crate::app::theme::USING_SYSTEM_MAPLE.load(std::sync::atomic::Ordering::Relaxed);
                                                                            if !using_system_maple && current == "Maple Mono NF CN" {
                                                                                format!("Maple Mono NF CN ({})", t!("software_builtin"))
                                                                            } else if !crate::terminal::element::terminal_font_is_monospace(window, current.clone().into(), px(current_terminal_font_size)) {
                                                                                format!("Maple Mono NF CN ({})", t!("software_builtin"))
                                                                            } else {
                                                                                current.clone()
                                                                            }
                                                                        })
                                                                        .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                                                            let view = view.clone();
                                                                            let current = current.clone();
                                                                            move |mut menu, window, cx| {
                                                                                let mut names = terminal_font_names(window, cx, current_terminal_font_size);
                                                                                menu = menu.min_w(200.).max_h(px(320.)).scrollable(true);
                                                                                let maple_font = "Maple Mono NF CN".to_string();
                                                                                let using_system_maple = crate::app::theme::USING_SYSTEM_MAPLE.load(std::sync::atomic::Ordering::Relaxed);
                                                                                if !using_system_maple && names.contains(&maple_font) {
                                                                                    names.retain(|n| n != &maple_font);
                                                                                    menu = menu.item(
                                                                                        PopupMenuItem::new(format!("{} ({})", maple_font, t!("software_builtin")))
                                                                                            .checked(current == maple_font)
                                                                                            .on_click(window.listener_for(&view, move |this, _, _window, cx| {
                                                                                                this.change_terminal_font_family("Maple Mono NF CN", cx);
                                                                                            }))
                                                                                    ).separator();
                                                                                }
                                                                                for name in names {
                                                                                    let checked = name == current;
                                                                                    menu = menu.item(
                                                                                        PopupMenuItem::new(name.clone())
                                                                                            .checked(checked)
                                                                                            .on_click(window.listener_for(&view, move |this, _, _window, cx| {
                                                                                                this.change_terminal_font_family(&name, cx);
                                                                                            }))
                                                                                    );
                                                                                }
                                                                                menu
                                                                            }
                                                                        })
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("cursor_style").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let current = cursor_style;
                                                                move |_, _window, _cx| {
                                                                    use crate::session::config::CursorStyle;
                                                                    Button::new("cursor-style-dropdown")
                                                                        .small()
                                                                        .icon(IconName::ChevronsUpDown)
                                                                        .label(match current {
                                                                            CursorStyle::Default => t!("cursor_style_default").to_string(),
                                                                            CursorStyle::Blink => t!("cursor_style_blink").to_string(),
                                                                            CursorStyle::Beam => t!("cursor_style_beam").to_string(),
                                                                            CursorStyle::BeamBlink => t!("cursor_style_beam_blink").to_string(),
                                                                        })
                                                                        .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                                                            let view = view.clone();
                                                                            let current = current;
                                                                            move |mut menu, window, _cx| {
                                                                                use crate::session::config::CursorStyle;
                                                                                menu = menu.min_w(160.).max_h(px(320.)).scrollable(true);
                                                                                for style in [
                                                                                    CursorStyle::Default,
                                                                                    CursorStyle::Blink,
                                                                                    CursorStyle::Beam,
                                                                                    CursorStyle::BeamBlink,
                                                                                ] {
                                                                                    let checked = style == current;
                                                                                    let label = match style {
                                                                                        CursorStyle::Default => t!("cursor_style_default").to_string(),
                                                                                        CursorStyle::Blink => t!("cursor_style_blink").to_string(),
                                                                                        CursorStyle::Beam => t!("cursor_style_beam").to_string(),
                                                                                        CursorStyle::BeamBlink => t!("cursor_style_beam_blink").to_string(),
                                                                                    };
                                                                                    menu = menu.item(
                                                                                        PopupMenuItem::new(label)
                                                                                            .checked(checked)
                                                                                            .on_click(window.listener_for(&view, move |this, _, _window, cx| {
                                                                                                this.change_cursor_style(style, cx);
                                                                                            }))
                                                                                    );
                                                                                }
                                                                                menu
                                                                            }
                                                                        })
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                            )
                                            .group(
                                                SettingGroup::new()
                                                    .title(t!("settings_group_other").to_string())
                                                    .item(
                                                        SettingItem::new(
                                                            t!("right_click_copy_paste").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let enabled = right_click_copy_paste;
                                                                move |_, window, _cx| {
                                                                    Switch::new("right-click-copy-paste")
                                                                        .small()
                                                                        .checked(enabled)
                                                                        .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                                                            this.config.set_right_click_copy_paste(*checked);
                                                                            let _ = this.config.save();
                                                                            cx.notify();
                                                                        }))
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        ).description(t!("copy_paste_hint").to_string())
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("keyword_highlight").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let enabled = keyword_highlight;
                                                                move |_, window, _cx| {
                                                                    Switch::new("keyword-highlight")
                                                                        .small()
                                                                        .checked(enabled)
                                                                        .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                                                            this.config.set_keyword_highlight(*checked);
                                                                            let _ = this.config.save();
                                                                            cx.notify();
                                                                        }))
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("lock_layout").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let enabled = lock_layout;
                                                                move |_, window, _cx| {
                                                                    Switch::new("lock-layout")
                                                                        .small()
                                                                        .checked(enabled)
                                                                        .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                                                            this.config.set_lock_layout(*checked);
                                                                            let _ = this.config.save();
                                                                            cx.notify();
                                                                        }))
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        ).description(t!("lock_layout_hint").to_string())
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("show_monitoring_dashboard").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let enabled = show_monitoring_dashboard;
                                                                move |_, window, _cx| {
                                                                    Switch::new("show-monitoring-dashboard")
                                                                        .small()
                                                                        .checked(enabled)
                                                                        .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                                                            this.config.set_show_monitoring_dashboard(*checked);
                                                                            let _ = this.config.save();
                                                                            cx.notify();
                                                                        }))
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                        .description(t!("show_monitoring_dashboard_hint").to_string())
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("monitoring_position").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let show_monitoring = show_monitoring_dashboard;
                                                                let pos = monitoring_position.clone();
                                                                move |_, _window, _cx| {
                                                                    Button::new("monitoring-position-dropdown")
                                                                        .small()
                                                                        .icon(IconName::PanelLeftOpen)
                                                                        .label({
                                                                            if pos == "Sidebar" {
                                                                                t!("position_sidebar").to_string()
                                                                            } else {
                                                                                t!("position_bottom").to_string()
                                                                            }
                                                                        })
                                                                        .disabled(!show_monitoring)
                                                                        .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                                                            let view = view.clone();
                                                                            let pos = pos.clone();
                                                                            move |mut menu, window, _cx| {
                                                                                menu = menu.min_w(160.)
                                                                                    .item(
                                                                                        PopupMenuItem::new(t!("position_bottom").to_string())
                                                                                            .checked(pos == "Bottom")
                                                                                            .on_click(window.listener_for(&view, |this, _, _window, cx| {
                                                                                                this.config.set_monitoring_position("Bottom");
                                                                                                let _ = this.config.save();
                                                                                                cx.notify();
                                                                                            }))
                                                                                    )
                                                                                    .item(
                                                                                        PopupMenuItem::new(t!("position_sidebar").to_string())
                                                                                            .checked(pos == "Sidebar")
                                                                                            .on_click(window.listener_for(&view, |this, _, _window, cx| {
                                                                                                this.config.set_monitoring_position("Sidebar");
                                                                                                let _ = this.config.save();
                                                                                                cx.notify();
                                                                                            }))
                                                                                    );
                                                                                menu
                                                                            }
                                                                        })
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("language").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                let locale = current_locale.clone();
                                                                move |_, _window, _cx| {
                                                                    Button::new("language-dropdown")
                                                                        .small()
                                                                        .icon(IconName::Globe)
                                                                        .label({
                                                                            if locale == "en" {
                                                                                t!("english").to_string()
                                                                            } else if locale == "zh-CN" {
                                                                                t!("chinese").to_string()
                                                                            } else {
                                                                                t!("follow_system").to_string()
                                                                            }
                                                                        })
                                                                        .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                                                            let view = view.clone();
                                                                            let current_locale = locale.clone();
                                                                            move |mut menu, window, _cx| {
                                                                                menu = menu.min_w(160.)
                                                                                    .item(
                                                                                        PopupMenuItem::new(t!("follow_system").to_string())
                                                                                            .checked(current_locale == "system")
                                                                                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                                                this.set_display_language("system", window, cx)
                                                                                            }))
                                                                                    )
                                                                                    .separator()
                                                                                    .item(
                                                                                        PopupMenuItem::new(t!("english").to_string())
                                                                                            .checked(current_locale == "en")
                                                                                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                                                this.set_display_language("en", window, cx)
                                                                                            }))
                                                                                    )
                                                                                    .item(
                                                                                        PopupMenuItem::new(t!("chinese").to_string())
                                                                                            .checked(current_locale == "zh-CN")
                                                                                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                                                this.set_display_language("zh-CN", window, cx)
                                                                                            }))
                                                                                    );
                                                                                menu
                                                                            }
                                                                        })
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        )
                                                    )
                                                    .item(
                                                        SettingItem::new(
                                                            t!("reset_layout").to_string(),
                                                            SettingField::render({
                                                                let view = view_clone_for_general.clone();
                                                                move |_, window, _cx| {
                                                                    Button::new("reset-layout")
                                                                        .small()
                                                                        .label(t!("reset").to_string())
                                                                        .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                            this.reset_layout(window, cx);
                                                                        }))
                                                                        .into_any_element()
                                                                }
                                                            })
                                                        ).description(t!("reset_layout_hint").to_string())
                                                    )
                                            )
}
