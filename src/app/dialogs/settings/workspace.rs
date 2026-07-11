use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_workspace_page(
    view: &gpui::Entity<AxShell>,
    shell: &AxShell,
) -> SettingPage {
    let lock_layout = shell.config.lock_layout();
    let color_inactive_tabs = shell.config.color_inactive_tabs();
    let settings_close_shortcut_confirms = shell.config.settings_close_shortcut_confirms();

    SettingPage::new(t!("settings_workspace").to_string())
        .icon(IconName::LayoutDashboard)
        .group(
            SettingGroup::new()
                .title(t!("settings_workspace").to_string())
                .item(
                    SettingItem::new(
                        t!("lock_layout").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Switch::new("lock-layout")
                                    .small()
                                    .checked(lock_layout)
                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                        this.config.set_lock_layout(*checked);
                                        this.config.save_logged("set_lock_layout");
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("lock_layout_hint").to_string()),
                )
                .item(
                    SettingItem::new(
                        t!("color_inactive_tabs").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Switch::new("color-inactive-tabs")
                                    .small()
                                    .checked(color_inactive_tabs)
                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                        this.config.set_color_inactive_tabs(*checked);
                                        this.config.save_logged("set_inactive_tab_color");
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("color_inactive_tabs_hint").to_string()),
                )
                .item(
                    SettingItem::new(
                        t!("settings_close_shortcut_confirms").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Switch::new("settings-close-shortcut-confirms")
                                    .small()
                                    .checked(settings_close_shortcut_confirms)
                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                        this.config.set_settings_close_shortcut_confirms(*checked);
                                        this.config.save_logged("set_settings_close_shortcut");
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("settings_close_shortcut_confirms_hint").to_string()),
                )
                .item(
                    SettingItem::new(
                        t!("reset_layout").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Button::new("reset-layout")
                                    .small()
                                    .label(t!("reset").to_string())
                                    .on_click(window.listener_for(&view, |this, _, window, cx| {
                                        this.reset_layout(window, cx);
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("reset_layout_hint").to_string()),
                ),
        )
}
