use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_custom_page(
    view: &gpui::Entity<AxShell>,
    shell: &AxShell,
    cx: &mut Context<AxShell>,
) -> SettingPage {
    let view_clone_for_custom = view.clone();
    let custom_theme_name_input = shell
        .custom_theme_inputs
        .get(crate::app::theme::custom_theme_name_input_key())
        .expect("custom theme name input missing")
        .clone();
    let custom_light_base_name =
        shell.resolved_custom_theme_base_name(gpui_component::ThemeMode::Light, cx);
    let custom_dark_base_name =
        shell.resolved_custom_theme_base_name(gpui_component::ThemeMode::Dark, cx);
    let mut custom_theme_meta_group = SettingGroup::new()
        .title(t!("settings_custom_theme").to_string())
        .description(t!("settings_custom_config_hint").to_string())
        .item(
            SettingItem::new(
                t!("custom_theme_name").to_string(),
                SettingField::render({
                    let input = custom_theme_name_input.clone();
                    move |_, _window, _cx| Input::new(&input).w(px(220.)).into_any_element()
                }),
            )
            .description(t!("custom_theme_saved_name_hint").to_string()),
        );

    for (mode, label, button_id, current_base_name) in [
        (
            gpui_component::ThemeMode::Light,
            t!("custom_theme_light_base").to_string(),
            "custom-theme-light-base-dropdown",
            custom_light_base_name.clone(),
        ),
        (
            gpui_component::ThemeMode::Dark,
            t!("custom_theme_dark_base").to_string(),
            "custom-theme-dark-base-dropdown",
            custom_dark_base_name.clone(),
        ),
    ] {
        custom_theme_meta_group = custom_theme_meta_group.item(
            SettingItem::new(
                label,
                SettingField::render({
                    let view = view_clone_for_custom.clone();
                    let current_base_name = current_base_name.clone();
                    move |_, _window, _cx| {
                        Button::new(button_id)
                            .small()
                            .icon(if mode.is_dark() {
                                IconName::Moon
                            } else {
                                IconName::Sun
                            })
                            .label(current_base_name.clone())
                            .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                let view = view.clone();
                                let current_base_name = current_base_name.clone();
                                move |mut menu, window, cx| {
                                    menu = menu.min_w(220.).max_h(px(320.)).scrollable(true);
                                    for theme in gpui_component::ThemeRegistry::global(cx)
                                        .sorted_themes()
                                        .into_iter()
                                        .filter(|theme| theme.mode == mode)
                                    {
                                        let theme_name = theme.name.clone();
                                        let checked =
                                            theme_name.as_ref() == current_base_name.as_str();
                                        menu = menu.item(
                                            PopupMenuItem::new(theme_name.clone())
                                                .checked(checked)
                                                .on_click(window.listener_for(
                                                    &view,
                                                    move |this, _, _window, cx| {
                                                        this.set_custom_theme_base_preset(
                                                            mode,
                                                            &theme_name,
                                                            cx,
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
            )
            .description(t!("custom_theme_base_hint").to_string()),
        );
    }

    custom_theme_meta_group = custom_theme_meta_group.item(SettingItem::new(
        t!("save").to_string(),
        SettingField::render({
            let view = view_clone_for_custom.clone();
            move |_, window, _cx| {
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("custom-appearance-save")
                            .primary()
                            .label(t!("save").to_string())
                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                this.save_custom_appearance(window, cx);
                            })),
                    )
                    .child(
                        Button::new("custom-appearance-reset")
                            .ghost()
                            .label(t!("reset").to_string())
                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                this.reset_custom_appearance(window, cx);
                            })),
                    )
                    .into_any_element()
            }
        }),
    ));

    let mut custom_theme_page = SettingPage::new(t!("settings_custom").to_string())
        .icon(IconName::Settings)
        .group(custom_theme_meta_group);

    for mode in crate::app::theme::custom_theme_modes() {
        let mut group = SettingGroup::new()
            .title(if mode.is_dark() {
                t!("dark_theme").to_string()
            } else {
                t!("light_theme").to_string()
            })
            .description(t!("settings_custom_theme_overrides").to_string());

        for section in crate::app::theme::CUSTOM_THEME_SECTION_SPECS {
            let section_title = section.title.to_string();
            group = group.item(SettingItem::render(move |_, _window, _cx| {
                div()
                    .pt_2()
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .child(section_title.clone())
            }));

            for field in section.fields {
                let input_key = crate::app::theme::custom_theme_input_key(mode, field.key);
                let input = shell
                    .custom_theme_inputs
                    .get(&input_key)
                    .expect("custom theme input missing")
                    .clone();
                let width = if field.domain == crate::app::theme::CustomThemeFieldDomain::Brightness
                {
                    px(96.)
                } else {
                    px(180.)
                };
                let description =
                    if field.domain == crate::app::theme::CustomThemeFieldDomain::Brightness {
                        format!("{} key: {}", t!("custom_font_brightness_hint"), field.key)
                    } else {
                        format!(
                            "{} key: {}; example: {}",
                            t!("custom_theme_inherit_hint"),
                            field.key,
                            field.placeholder
                        )
                    };

                group = group.item(
                    SettingItem::new(
                        field.label.to_string(),
                        SettingField::render({
                            let input = input.clone();
                            move |_, _window, _cx| Input::new(&input).w(width).into_any_element()
                        }),
                    )
                    .description(description),
                );
            }
        }

        custom_theme_page = custom_theme_page.group(group);
    }

    custom_theme_page
}
