use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_appearance_page(
    view: &gpui::Entity<AxShell>,
    shell: &AxShell,
) -> SettingPage {
    let follow_system_theme = shell.appearance.follow_system_theme;
    let theme_mode_is_dark = shell.appearance.theme_mode.is_dark();
    let light_theme_name = shell.appearance.light_theme_name.to_string();
    let dark_theme_name = shell.appearance.dark_theme_name.to_string();
    let title_bar_style = shell.config.effective_title_bar_style();

    SettingPage::new(t!("settings_appearance").to_string())
        .icon(IconName::Palette)
        .default_open(true)
        .group(
            SettingGroup::new()
                .title(t!("settings_group_appearance").to_string())
                .item(SettingItem::new(
                    t!("theme_mode").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        move |_, _window, _cx| {
                            Button::new("theme-mode-dropdown")
                                .small()
                                .icon(if follow_system_theme {
                                    IconName::Sun
                                } else if theme_mode_is_dark {
                                    IconName::Moon
                                } else {
                                    IconName::Sun
                                })
                                .label(if follow_system_theme {
                                    t!("follow_system").to_string()
                                } else if theme_mode_is_dark {
                                    t!("use_dark_mode").to_string()
                                } else {
                                    t!("use_light_mode").to_string()
                                })
                                .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                    let view = view.clone();
                                    move |mut menu, window, _cx| {
                                        menu = menu
                                            .min_w(160.)
                                            .item(
                                                PopupMenuItem::new(t!("follow_system").to_string())
                                                    .checked(follow_system_theme)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        |this, _, window, cx| {
                                                            this.set_follow_system_theme(
                                                                true, window, cx,
                                                            )
                                                        },
                                                    )),
                                            )
                                            .item(
                                                PopupMenuItem::new(t!("use_light_mode").to_string())
                                                    .checked(
                                                        !follow_system_theme && !theme_mode_is_dark,
                                                    )
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        |this, _, window, cx| {
                                                            this.switch_theme_mode(
                                                                gpui_component::ThemeMode::Light,
                                                                window,
                                                                cx,
                                                            )
                                                        },
                                                    )),
                                            )
                                            .item(
                                                PopupMenuItem::new(t!("use_dark_mode").to_string())
                                                    .checked(
                                                        !follow_system_theme && theme_mode_is_dark,
                                                    )
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        |this, _, window, cx| {
                                                            this.switch_theme_mode(
                                                                gpui_component::ThemeMode::Dark,
                                                                window,
                                                                cx,
                                                            )
                                                        },
                                                    )),
                                            );
                                        menu
                                    }
                                })
                                .into_any_element()
                        }
                    }),
                ))
                .item(SettingItem::new(
                    t!("light_theme").to_string(),
                    SettingField::render({
                        let view = view.clone();
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
                                        let themes =
                                            gpui_component::ThemeRegistry::global(cx).sorted_themes();
                                        let light_themes: Vec<_> = themes
                                            .into_iter()
                                            .filter(|theme| !theme.mode.is_dark())
                                            .map(|theme| theme.name.clone())
                                            .collect();
                                        menu = menu.min_w(160.).max_h(px(320.)).scrollable(true);
                                        for theme_name in light_themes {
                                            let checked = theme_name == current_theme;
                                            menu = menu.item(
                                                PopupMenuItem::new(theme_name.clone())
                                                    .checked(checked)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        move |this, _, window, cx| {
                                                            this.apply_theme(
                                                                theme_name.clone(),
                                                                window,
                                                                cx,
                                                            )
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
                    t!("dark_theme").to_string(),
                    SettingField::render({
                        let view = view.clone();
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
                                        let themes =
                                            gpui_component::ThemeRegistry::global(cx).sorted_themes();
                                        let dark_themes: Vec<_> = themes
                                            .into_iter()
                                            .filter(|theme| theme.mode.is_dark())
                                            .map(|theme| theme.name.clone())
                                            .collect();
                                        menu = menu.min_w(160.).max_h(px(320.)).scrollable(true);
                                        for theme_name in dark_themes {
                                            let checked = theme_name == current_theme;
                                            menu = menu.item(
                                                PopupMenuItem::new(theme_name.clone())
                                                    .checked(checked)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        move |this, _, window, cx| {
                                                            this.apply_theme(
                                                                theme_name.clone(),
                                                                window,
                                                                cx,
                                                            )
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
                    format!("{}{}", t!("title_bar_style"), t!("restart_hint")),
                    SettingField::render({
                        let view = view.clone();
                        move |_, _window, _cx| {
                            let supports_integrated = cfg!(target_os = "macos");
                            Button::new("title-bar-style-dropdown")
                                .small()
                                .label(match title_bar_style {
                                    crate::config::TitleBarStyle::Native => {
                                        t!("title_bar_native").to_string()
                                    }
                                    crate::config::TitleBarStyle::Integrated => {
                                        t!("title_bar_integrated").to_string()
                                    }
                                })
                                .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                    let view = view.clone();
                                    move |mut menu, window, _cx| {
                                        menu = menu.min_w(160.).item(
                                            PopupMenuItem::new(t!("title_bar_native").to_string())
                                                .checked(
                                                    title_bar_style
                                                        == crate::config::TitleBarStyle::Native,
                                                )
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.config.set_title_bar_style(
                                                            crate::config::TitleBarStyle::Native,
                                                        );
                                                        let _ = this.config.save();
                                                        cx.notify();
                                                    },
                                                )),
                                        );
                                        if supports_integrated {
                                            menu = menu.item(
                                                PopupMenuItem::new(
                                                    t!("title_bar_integrated").to_string(),
                                                )
                                                .checked(
                                                    title_bar_style
                                                        == crate::config::TitleBarStyle::Integrated,
                                                )
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.config.set_title_bar_style(
                                                            crate::config::TitleBarStyle::Integrated,
                                                        );
                                                        let _ = this.config.save();
                                                        cx.notify();
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
