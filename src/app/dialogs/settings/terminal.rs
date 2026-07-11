use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_terminal_page(view: &gpui::Entity<AxShell>, shell: &AxShell) -> SettingPage {
    let right_click_copy_paste = shell.config.right_click_copy_paste();
    let keyword_highlight = shell.config.keyword_highlight();
    let sftp_transfer_close_behavior = shell.config.sftp_transfer_close_behavior().to_string();
    let ssh_retry_count_input = shell.ssh_retry_count_input.clone();
    let ssh_retry_delays_input = shell.ssh_retry_delays_input.clone();

    SettingPage::new(t!("settings_terminal").to_string())
        .icon(IconName::SquareTerminal)
        .group(
            SettingGroup::new()
                .title(t!("settings_terminal").to_string())
                .item(
                    SettingItem::new(
                        t!("right_click_copy_paste").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Switch::new("right-click-copy-paste")
                                    .small()
                                    .checked(right_click_copy_paste)
                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                        this.config.set_right_click_copy_paste(*checked);
                                        this.config.save_logged("set_right_click_copy_paste");
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("copy_paste_hint").to_string()),
                )
                .item(SettingItem::new(
                    t!("keyword_highlight").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        move |_, window, _cx| {
                            Switch::new("keyword-highlight")
                                .small()
                                .checked(keyword_highlight)
                                .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                    this.config.set_keyword_highlight(*checked);
                                    this.config.save_logged("set_keyword_highlight");
                                    cx.notify();
                                }))
                                .into_any_element()
                        }
                    }),
                )),
        )
        .group(
            SettingGroup::new()
                .title(t!("settings_ssh_connection").to_string())
                .item(SettingItem::render({
                    let view = view.clone();
                    move |_, window, cx| {
                        let muted_foreground = cx.theme().muted_foreground;
                        v_flex()
                            .w_full()
                            .gap_3()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(div().text_sm().child(t!("ssh_retry_count").to_string()))
                                    .child(Input::new(&ssh_retry_count_input).w_full()),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div().text_sm().child(t!("ssh_retry_delays").to_string()),
                                    )
                                    .child(Input::new(&ssh_retry_delays_input).w_full())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(muted_foreground)
                                            .child(t!("ssh_retry_delays_hint").to_string()),
                                    ),
                            )
                            .child(
                                Button::new("save-ssh-retry-settings")
                                    .small()
                                    .primary()
                                    .label(t!("save_ssh_retry_settings").to_string())
                                    .on_click(window.listener_for(&view, |this, _, _, cx| {
                                        let retry_count = this
                                            .ssh_retry_count_input
                                            .read(cx)
                                            .value()
                                            .trim()
                                            .parse::<u32>()
                                            .unwrap_or(2);
                                        let delays = this
                                            .ssh_retry_delays_input
                                            .read(cx)
                                            .value()
                                            .split(',')
                                            .filter_map(|part| {
                                                let trimmed = part.trim();
                                                if trimmed.is_empty() {
                                                    return None;
                                                }
                                                trimmed.parse::<u64>().ok()
                                            })
                                            .collect::<Vec<_>>();
                                        this.config.set_ssh_connect_retry_count(retry_count);
                                        this.config.set_ssh_connect_retry_delays_ms(delays);
                                        this.config.save_logged("set_ssh_retry_policy");
                                        cx.notify();
                                    })),
                            )
                    }
                }))
                .description(t!("ssh_retry_defaults_hint").to_string()),
        )
        .group(
            SettingGroup::new()
                .title(t!("settings_sftp_transfers").to_string())
                .item(SettingItem::new(
                    t!("sftp_shortcut_confirm_behavior").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        move |_, _window, _cx| {
                            Button::new("sftp-transfer-close-behavior")
                                .small()
                                .label(match sftp_transfer_close_behavior.as_str() {
                                    "keep_page_open" => t!("sftp_keep_page_open").to_string(),
                                    "background" => t!("sftp_continue_in_background").to_string(),
                                    "cancel_disconnect" => {
                                        t!("sftp_cancel_and_disconnect").to_string()
                                    }
                                    _ => t!("sftp_shortcut_confirm_disabled").to_string(),
                                })
                                .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                    let view = view.clone();
                                    let selected = sftp_transfer_close_behavior.clone();
                                    move |mut menu, window, _cx| {
                                        for (value, label) in [
                                            (
                                                "ask",
                                                t!("sftp_shortcut_confirm_disabled").to_string(),
                                            ),
                                            (
                                                "keep_page_open",
                                                t!("sftp_keep_page_open").to_string(),
                                            ),
                                            (
                                                "background",
                                                t!("sftp_continue_in_background").to_string(),
                                            ),
                                            (
                                                "cancel_disconnect",
                                                t!("sftp_cancel_and_disconnect").to_string(),
                                            ),
                                        ] {
                                            menu = menu.item(
                                                PopupMenuItem::new(label)
                                                    .checked(selected == value)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        move |this, _, _window, cx| {
                                                            this.config
                                                                .set_sftp_transfer_close_behavior(
                                                                    value,
                                                                );
                                                            this.config.save_logged(
                                                                "set_sftp_close_behavior",
                                                            );
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
                ))
                .description(t!("sftp_shortcut_confirm_behavior_hint").to_string()),
        )
}
