use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_proxy_page(view: &gpui::Entity<AxShell>, shell: &AxShell) -> SettingPage {
    let use_proxy = shell.config.use_proxy();
    let read_env_proxy = shell.config.read_env_proxy();
    let x11_forwarding_enabled = shell.config.x11_forwarding_enabled();
    let x11_launch_xquartz = shell.config.x11_launch_xquartz();
    let xquartz_app_path_input = shell.xquartz_app_path_input.clone();
    let global_proxy_host_input = shell.global_proxy_host_input.clone();
    let global_proxy_port_input = shell.global_proxy_port_input.clone();
    let global_proxy_user_input = shell.global_proxy_user_input.clone();
    let global_proxy_password_input = shell.global_proxy_password_input.clone();
    let proxy_type = shell.global_proxy_type.clone();

    SettingPage::new(t!("settings_proxy").to_string())
        .icon(IconName::Network)
        .group(
            SettingGroup::new()
                .title(t!("settings_proxy").to_string())
                .item(SettingItem::new(
                    t!("enable_proxy").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        move |_, window, _cx| {
                            Switch::new("use-proxy")
                                .small()
                                .checked(use_proxy)
                                .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                    this.config.set_use_proxy(*checked);
                                    let _ = this.config.save();
                                    cx.notify();
                                }))
                                .into_any_element()
                        }
                    }),
                ))
                .item(
                    SettingItem::new(
                        t!("read_env_proxy").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Switch::new("read-env-proxy")
                                    .small()
                                    .checked(read_env_proxy)
                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                        this.config.set_read_env_proxy(*checked);
                                        let _ = this.config.save();
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("read_env_proxy_desc").to_string()),
                )
                .item(SettingItem::render({
                    let view = view.clone();
                    move |_, window, _cx| {
                        v_flex()
                            .w_full()
                            .gap_3()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .child(t!("global_proxy_settings").to_string()),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Button::new("global-proxy-type-socks5")
                                            .small()
                                            .label("SOCKS5")
                                            .when(proxy_type == "socks5", |b| b.primary())
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, _, cx| {
                                                    this.global_proxy_type = "socks5".to_string();
                                                    cx.notify();
                                                },
                                            )),
                                    )
                                    .child(
                                        Button::new("global-proxy-type-http")
                                            .small()
                                            .label("HTTP")
                                            .when(proxy_type == "http", |b| b.primary())
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, _, cx| {
                                                    this.global_proxy_type = "http".to_string();
                                                    cx.notify();
                                                },
                                            )),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div().text_sm().child(t!("global_proxy_host").to_string()),
                                    )
                                    .child(Input::new(&global_proxy_host_input).w_full()),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div().text_sm().child(t!("global_proxy_port").to_string()),
                                    )
                                    .child(Input::new(&global_proxy_port_input).w_full()),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div().text_sm().child(t!("global_proxy_user").to_string()),
                                    )
                                    .child(Input::new(&global_proxy_user_input).w_full()),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .child(t!("global_proxy_password").to_string()),
                                    )
                                    .child(Input::new(&global_proxy_password_input).w_full()),
                            )
                            .child(
                                Button::new("save-global-proxy")
                                    .small()
                                    .primary()
                                    .label(t!("save_proxy").to_string())
                                    .on_click(window.listener_for(&view, |this, _, _, cx| {
                                        let host = this
                                            .global_proxy_host_input
                                            .read(cx)
                                            .value()
                                            .trim()
                                            .to_string();
                                        let port_str =
                                            this.global_proxy_port_input.read(cx).value();
                                        let port = port_str.trim().parse::<u16>().ok();
                                        let user = this
                                            .global_proxy_user_input
                                            .read(cx)
                                            .value()
                                            .trim()
                                            .to_string();
                                        let password = this
                                            .global_proxy_password_input
                                            .read(cx)
                                            .value()
                                            .to_string();

                                        if host.is_empty() || port.is_none() {
                                            return;
                                        }

                                        this.config
                                            .set_global_proxy_type(this.global_proxy_type.clone());
                                        this.config.set_global_proxy_host(host);
                                        this.config.set_global_proxy_port(port);
                                        this.config.set_global_proxy_user(user);
                                        this.config.set_global_proxy_password(password);
                                        let _ = this.config.save();
                                        cx.notify();
                                    })),
                            )
                    }
                })),
        )
        .group(
            SettingGroup::new()
                .title(t!("settings_x11").to_string())
                .item(
                    SettingItem::new(
                        t!("enable_x11_forwarding").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Switch::new("x11-forwarding-enabled")
                                    .small()
                                    .checked(x11_forwarding_enabled)
                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                        this.config.set_x11_forwarding_enabled(*checked);
                                        let _ = this.config.save();
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("enable_x11_forwarding_desc").to_string()),
                )
                .item(
                    SettingItem::new(
                        t!("x11_launch_xquartz").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Switch::new("x11-launch-xquartz")
                                    .small()
                                    .checked(x11_launch_xquartz)
                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                        this.config.set_x11_launch_xquartz(*checked);
                                        let _ = this.config.save();
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("x11_launch_xquartz_desc").to_string()),
                )
                .item(SettingItem::render({
                    let view = view.clone();
                    move |_, window, cx| {
                        let muted_foreground = cx.theme().muted_foreground;
                        v_flex()
                            .w_full()
                            .items_stretch()
                            .gap_3()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .child(t!("xquartz_app_path").to_string()),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .min_w(px(320.))
                                    .child(Input::new(&xquartz_app_path_input).w_full()),
                            )
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_2()
                                    .child(
                                        Button::new("browse-xquartz-app")
                                            .small()
                                            .label(t!("browse").to_string())
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, window, cx| {
                                                    this.pick_xquartz_app_path(window, cx);
                                                },
                                            )),
                                    )
                                    .child(
                                        Button::new("reset-xquartz-app")
                                            .small()
                                            .label(t!("reset_default").to_string())
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, window, cx| {
                                                    this.reset_xquartz_app_path(window, cx);
                                                },
                                            )),
                                    )
                                    .child(
                                        Button::new("save-x11-settings")
                                            .small()
                                            .primary()
                                            .label(t!("save_x11_settings").to_string())
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, _, cx| {
                                                    this.save_x11_settings(cx);
                                                },
                                            )),
                                    )
                                    .child(
                                        Button::new("open-xquartz")
                                            .small()
                                            .label(t!("open_xquartz").to_string())
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, _, cx| {
                                                    this.open_configured_xquartz(cx);
                                                },
                                            )),
                                    ),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .text_xs()
                                    .text_color(muted_foreground)
                                    .child(t!("xquartz_app_path_desc").to_string()),
                            )
                    }
                })),
        )
}
