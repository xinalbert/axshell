use gpui::{
    Anchor, Context, Focusable as _, FontWeight, InteractiveElement as _, MouseButton,
    ParentElement as _, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _, px, rems,
};
use gpui_component::{
    ActiveTheme as _, Disableable as _, IconName, Sizable as _, WindowExt as _,
    button::{Button, ButtonVariants as _},
    dialog::Dialog,
    h_flex,
    input::Input,
    menu::{DropdownMenu as _, PopupMenuItem},
    progress::Progress,
    scroll::{Scrollbar, ScrollbarShow},
    switch::Switch,
    v_flex,
};
use rust_i18n::t;

use crate::{AxShell, session::config::AuthMethod, system::format_bytes};

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

impl AxShell {
    pub(crate) fn show_ssh_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_dialog.is_some() {
            return;
        }
        self.active_dialog = Some(crate::app::DialogKind::NewSsh);

        let view = cx.entity();
        let session_name_input = self.session_name_input.clone();
        let session_group_input = self.session_group_input.clone();
        let host_input = self.host_input.clone();
        let focus_host_input = host_input.clone();
        let port_input = self.port_input.clone();
        let user_input = self.user_input.clone();
        let password_input = self.password_input.clone();
        let key_path_input = self.key_path_input.clone();
        let key_inline_input = self.key_inline_input.clone();
        let passphrase_input = self.passphrase_input.clone();
        let proxy_host_input = self.proxy_host_input.clone();
        let proxy_port_input = self.proxy_port_input.clone();
        let proxy_user_input = self.proxy_user_input.clone();
        let proxy_password_input = self.proxy_password_input.clone();

        window.open_dialog(cx, move |dialog: Dialog, _window, _cx| {
            dialog
                .title(t!("new_ssh_connection"))
                .w(px(520.))
                .overlay_closable(true)
                .on_close({
                    let view = view.clone();
                    move |_, _, cx| {
                        view.update(cx, |this, cx| {
                            this.active_dialog = None;
                            cx.notify();
                        });
                    }
                })
                .content({
                    let view = view.clone();
                    let session_name_input = session_name_input.clone();
                    let session_group_input = session_group_input.clone();
                    let host_input = host_input.clone();
                    let port_input = port_input.clone();
                    let user_input = user_input.clone();
                    let password_input = password_input.clone();
                    let key_path_input = key_path_input.clone();
                    let key_inline_input = key_inline_input.clone();
                    let passphrase_input = passphrase_input.clone();
                    let proxy_host_input = proxy_host_input.clone();
                    let proxy_port_input = proxy_port_input.clone();
                    let proxy_user_input = proxy_user_input.clone();
                    let proxy_password_input = proxy_password_input.clone();
                    move |content, window, cx| {
                        let is_password = view.read(cx).ssh_auth_method == AuthMethod::Password;
                        let is_editing = view.read(cx).editing_session_id.is_some();
                        let proxy_type = view.read(cx).ssh_proxy_type.clone();
                        let show_proxy_fields = proxy_type != "none";
                        let saved_group_names = view.read(cx).saved_group_names();
                        let current_group_name =
                            session_group_input.read(cx).value().trim().to_string();
                        content.child(
                            v_flex()
                                .gap_3()
                                .child(Input::new(&session_name_input).tab_index(0))
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            Input::new(&session_group_input)
                                                .flex_1()
                                                .tab_index(1),
                                        )
                                        .child(
                                            Button::new("ssh-group-dropdown")
                                                .small()
                                                .icon(IconName::ChevronsUpDown)
                                                .label(t!("choose_group").to_string())
                                                .disabled(saved_group_names.is_empty())
                                                .dropdown_menu_with_anchor(
                                                    Anchor::BottomRight,
                                                    {
                                                        let view = view.clone();
                                                        let saved_group_names =
                                                            saved_group_names.clone();
                                                        let current_group_name =
                                                            current_group_name.clone();
                                                        move |mut menu, window, _cx| {
                                                            menu = menu
                                                                .min_w(180.)
                                                                .max_h(px(320.))
                                                                .scrollable(true)
                                                                .item(
                                                                    PopupMenuItem::new(
                                                                        t!("ungrouped_group")
                                                                            .to_string(),
                                                                    )
                                                                    .checked(
                                                                        current_group_name
                                                                            .is_empty(),
                                                                    )
                                                                    .on_click(
                                                                        window.listener_for(
                                                                            &view,
                                                                            |this, _, window, cx| {
                                                                                Self::set_input_value(
                                                                                    &this.session_group_input,
                                                                                    "",
                                                                                    window,
                                                                                    cx,
                                                                                );
                                                                            },
                                                                        ),
                                                                    ),
                                                                );
                                                            if !saved_group_names.is_empty() {
                                                                menu = menu.separator();
                                                            }
                                                            for group_name in &saved_group_names {
                                                                let checked =
                                                                    current_group_name
                                                                        == *group_name;
                                                                let group_name =
                                                                    group_name.clone();
                                                                menu = menu.item(
                                                                    PopupMenuItem::new(
                                                                        group_name.clone(),
                                                                    )
                                                                    .checked(checked)
                                                                    .on_click(
                                                                        window.listener_for(
                                                                            &view,
                                                                            move |this, _, window, cx| {
                                                                                Self::set_input_value(
                                                                                    &this.session_group_input,
                                                                                    group_name.clone(),
                                                                                    window,
                                                                                    cx,
                                                                                );
                                                                            },
                                                                        ),
                                                                    ),
                                                                );
                                                            }
                                                            menu
                                                        }
                                                    },
                                                ),
                                        ),
                                )
                                .child(Input::new(&host_input).tab_index(2))
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(Input::new(&port_input).w(px(96.)).tab_index(3))
                                        .child(Input::new(&user_input).flex_1().tab_index(4)),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            Button::new("ssh-auth-password")
                                                .label(t!("password").to_string())
                                                .when(is_password, |button| button.primary())
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.set_ssh_auth_method(
                                                            AuthMethod::Password,
                                                            cx,
                                                        )
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("ssh-auth-key")
                                                .label(t!("key").to_string())
                                                .when(!is_password, |button| button.primary())
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.set_ssh_auth_method(
                                                            AuthMethod::Key,
                                                            cx,
                                                        )
                                                    },
                                                )),
                                        ),
                                )
                                .when(is_password, |this| {
                                    this.child(
                                        Input::new(&password_input).mask_toggle().tab_index(5),
                                    )
                                })
                                .when(!is_password, |this| {
                                    this.child(
                                        h_flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .cursor_pointer()
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        window.listener_for(
                                                            &view,
                                                            |this, _, window, cx| {
                                                                this.pick_ssh_key_path(window, cx);
                                                            },
                                                        ),
                                                    )
                                                    .child(
                                                        Input::new(&key_path_input).tab_index(5),
                                                    ),
                                            )
                                            .child(
                                                Button::new("clear-key-path")
                                                    .ghost()
                                                    .icon(IconName::Close)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        |this, _, window, cx| {
                                                            Self::set_input_value(
                                                                &this.key_path_input,
                                                                "",
                                                                window,
                                                                cx,
                                                            );
                                                        },
                                                    )),
                                            ),
                                    )
                                    .child(Input::new(&key_inline_input).h(px(128.)).tab_index(6))
                                    .child(Input::new(&passphrase_input).mask_toggle().tab_index(7))
                                })
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .child(t!("proxy").to_string()),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            Button::new("proxy-none")
                                                .label(t!("proxy_none").to_string())
                                                .when(proxy_type == "none", |button| {
                                                    button.primary()
                                                })
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.set_ssh_proxy_type(
                                                            "none".to_string(),
                                                            cx,
                                                        )
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("proxy-socks5")
                                                .label("SOCKS5")
                                                .when(proxy_type == "socks5", |button| {
                                                    button.primary()
                                                })
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.set_ssh_proxy_type(
                                                            "socks5".to_string(),
                                                            cx,
                                                        )
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("proxy-http")
                                                .label("HTTP")
                                                .when(proxy_type == "http", |button| {
                                                    button.primary()
                                                })
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.set_ssh_proxy_type(
                                                            "http".to_string(),
                                                            cx,
                                                        )
                                                    },
                                                )),
                                        ),
                                )
                                .when(show_proxy_fields, |this| {
                                    this.child(
                                        h_flex()
                                            .gap_2()
                                            .child(Input::new(&proxy_host_input).flex_1())
                                            .child(Input::new(&proxy_port_input).w(px(96.))),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(Input::new(&proxy_user_input).flex_1())
                                            .child(Input::new(&proxy_password_input).flex_1()),
                                    )
                                })
                                .child(
                                    h_flex()
                                        .justify_end()
                                        .gap_2()
                                        .child(
                                            Button::new("connect-ssh-cancel")
                                                .label(t!("cancel").to_string())
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, window, cx| {
                                                        this.active_dialog = None;
                                                        window.close_dialog(cx);
                                                        cx.notify();
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("connect-ssh-confirm")
                                                .primary()
                                                .label(if is_editing {
                                                    t!("save")
                                                } else {
                                                    t!("connect")
                                                })
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, window, cx| {
                                                        this.connect_ssh(window, cx)
                                                    },
                                                )),
                                        ),
                                ),
                        )
                    }
                })
        });
        window.defer(cx, move |window, cx| {
            window.focus(&focus_host_input.read(cx).focus_handle(cx), cx);
        });
    }
    pub(crate) fn show_selector_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_dialog.is_some() {
            return;
        }
        self.active_dialog = Some(crate::app::DialogKind::SessionSelector);

        let view = cx.entity();
        let selector_focus_handle = self.selector_focus_handle.clone();
        let deferred_selector_focus_handle = selector_focus_handle.clone();
        let sessions = self.config.sessions().to_vec();
        let active_session_id = self.active_session_id().map(ToOwned::to_owned);
        self.selector_selection = self.default_selector_index();
        window.open_dialog(cx, move |dialog: Dialog, _window, _| {
            dialog
                .title(t!("open_session").to_string())
                .w(px(520.))
                .on_close({
                    let view = view.clone();
                    move |_, _, cx| {
                        view.update(cx, |this, cx| {
                            this.active_dialog = None;
                            cx.notify();
                        });
                    }
                })
                .on_ok({
                    let view = view.clone();
                    move |_, window, cx| {
                        view.update(cx, |this, cx| {
                            this.activate_selector_selection(window, cx);
                        });
                        false
                    }
                })
                .content({
                    let view = view.clone();
                    let sessions = sessions.clone();
                    let _active_session_id = active_session_id.clone();
                    let selector_focus_handle = selector_focus_handle.clone();
                    move |content, window, _cx| {
                        let selected_index = view.read(_cx).selector_selection;
                        let scroll_handle = view.read(_cx).selector_scroll_handle.clone();
                        content.child(
                            v_flex()
                                .track_focus(&selector_focus_handle)
                                .on_key_down(window.listener_for(
                                    &view,
                                    |this, event, window, cx| {
                                        this.on_selector_key_down(event, window, cx)
                                    },
                                ))
                                .gap_2()
                                .child(
                                    div()
                                        .w_full()
                                        .p_2()
                                        .rounded_md()
                                        .border_1()
                                        .border_color(if selected_index == 0 {
                                            _cx.theme().primary
                                        } else {
                                            _cx.theme().border
                                        })
                                        .bg(if selected_index == 0 {
                                            _cx.theme().tab_active
                                        } else {
                                            _cx.theme().muted
                                        })
                                        .cursor_pointer()
                                        .hover(|this| this.bg(_cx.theme().secondary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            window.listener_for(&view, |this, _, window, cx| {
                                                this.active_dialog = None;
                                                this.open_local(cx);
                                                window.close_dialog(cx);
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            v_flex()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_size(rems(1.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child(t!("local_terminal")),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(rems(0.917))
                                                        .text_color(_cx.theme().muted_foreground)
                                                        .child(t!("open_local_shell_tab")),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .p_2()
                                        .rounded_md()
                                        .border_1()
                                        .border_color(if selected_index == 1 {
                                            _cx.theme().primary
                                        } else {
                                            _cx.theme().border
                                        })
                                        .bg(if selected_index == 1 {
                                            _cx.theme().tab_active
                                        } else {
                                            _cx.theme().muted
                                        })
                                        .cursor_pointer()
                                        .hover(|this| this.bg(_cx.theme().secondary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            window.listener_for(&view, |this, _, window, cx| {
                                                this.active_dialog = None;
                                                window.close_dialog(cx);
                                                this.open_new_ssh_dialog(window, cx);
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            v_flex()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_size(rems(1.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child(t!("new_ssh_connection")),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(rems(0.917))
                                                        .text_color(_cx.theme().muted_foreground)
                                                        .child(t!("create_or_edit_ssh_session")),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .relative()
                                        .max_h(px(320.))
                                        .size_full()
                                        .child(
                                            v_flex()
                                                .size_full()
                                                .id("selector-scroll-view")
                                                .track_scroll(&scroll_handle)
                                                .overflow_y_scroll()
                                                .gap_2()
                                                .children(
                                                    sessions.clone().into_iter().enumerate().map(
                                                        |(ix, session)| {
                                                            let connect_id = session.id.clone();
                                                            let is_selected =
                                                                selected_index == ix + 2;
                                                            let name = session.name.clone();
                                                            let detail = format!(
                                                                "{}@{}:{}",
                                                                session.user,
                                                                session.host,
                                                                session.port
                                                            );
                                                            div()
                                                    .id(("selector-open", ix))
                                                    .w_full()
                                                    .p_2()
                                                    .rounded_md()
                                                    .border_1()
                                                    .border_color(if is_selected {
                                                        _cx.theme().primary
                                                    } else {
                                                        _cx.theme().border
                                                    })
                                                    .bg(if is_selected {
                                                        _cx.theme().tab_active
                                                    } else {
                                                        _cx.theme().muted
                                                    })
                                                    .cursor_pointer()
                                                    .hover(|this| this.bg(_cx.theme().secondary))
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        window.listener_for(
                                                            &view,
                                                            move |this, _, window, cx| {
                                                                this.active_dialog = None;
                                                                this.connect_saved_session(
                                                                    connect_id.clone(),
                                                                    cx,
                                                                );
                                                                window.close_dialog(cx);
                                                                cx.notify();
                                                            },
                                                        ),
                                                    )
                                                    .child(
                                                        v_flex()
                                                            .gap_1()
                                                            .child(
                                                                div()
                                                                    .text_size(rems(1.0))
                                                                    .font_weight(
                                                                        FontWeight::SEMIBOLD,
                                                                    )
                                                                    .child(name),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_size(rems(0.917))
                                                                    .text_color(
                                                                        _cx.theme()
                                                                            .muted_foreground,
                                                                    )
                                                                    .child(detail),
                                                            ),
                                                    )
                                                        },
                                                    ),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .absolute()
                                                .top_0()
                                                .bottom_0()
                                                .left_0()
                                                .right_0()
                                                .child(
                                                gpui_component::scroll::Scrollbar::new(
                                                    &scroll_handle,
                                                )
                                                .id("selector-scrollbar")
                                                .axis(
                                                    gpui_component::scroll::ScrollbarAxis::Vertical,
                                                )
                                                .scrollbar_show(
                                                    gpui_component::scroll::ScrollbarShow::Always,
                                                ),
                                            ),
                                        ),
                                ),
                        )
                    }
                })
        });
        window.defer(cx, move |window, cx| {
            window.focus(&deferred_selector_focus_handle, cx);
        });
    }
    pub(crate) fn show_transfers_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_dialog.is_some() {
            return;
        }
        self.active_dialog = Some(crate::app::DialogKind::Transfers);

        let view = cx.entity();
        window.open_dialog(cx, move |dialog: Dialog, _window, _| {
            dialog
                .w(px(600.))
                .close_button(false)
                .on_close({
                    let view = view.clone();
                    move |_, _, cx| {
                        view.update(cx, |this, cx| {
                            this.active_dialog = None;
                            cx.notify();
                        });
                    }
                })
                .content({
                    let view = view.clone();
                    move |content, window, cx| {
                        let can_clear = view.read(cx).transfers.iter().any(|t| {
                            !matches!(
                                t.state,
                                crate::terminal::TransferState::Running
                                    | crate::terminal::TransferState::Paused
                            )
                        });

                        let clear_btn = if can_clear {
                            Some(
                                Button::new("clear_transfers_btn")
                                    .small()
                                    .ghost()
                                    .icon(IconName::Delete)
                                    .label(t!("clear_transfers").to_string())
                                    .on_click(window.listener_for(&view, |this, _, _, cx| {
                                        this.transfers.retain(|t| {
                                            matches!(
                                                t.state,
                                                crate::terminal::TransferState::Running
                                                    | crate::terminal::TransferState::Paused
                                            )
                                        });
                                        this.config.set_transfers(this.transfers.clone());
                                        cx.notify();
                                    })),
                            )
                        } else {
                            None
                        };

                        let header = h_flex()
                            .w_full()
                            .justify_between()
                            .items_center()
                            .child(
                                h_flex()
                                    .items_baseline()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child(t!("transfers").to_string()),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground)
                                            .ml_2()
                                            .child(t!("transfers_limit").to_string()),
                                    ),
                            )
                            .child(
                                h_flex().gap_2().children(clear_btn).child(
                                    Button::new("close_dialog")
                                        .small()
                                        .ghost()
                                        .icon(IconName::Close)
                                        .on_click(window.listener_for(
                                            &view,
                                            |this, _, window, cx| {
                                                this.active_dialog = None;
                                                window.close_dialog(cx);
                                                cx.notify();
                                            },
                                        )),
                                ),
                            );

                        let mut transfers = view.read(cx).transfers.clone();
                        transfers.sort_by_key(|t| match t.state {
                            crate::terminal::TransferState::Running
                            | crate::terminal::TransferState::Paused => 0,
                            _ => 1,
                        });

                        if transfers.is_empty() {
                            return content.child(
                                v_flex().gap_2().child(header).child(
                                    div()
                                        .p_4()
                                        .text_center()
                                        .text_color(cx.theme().muted_foreground)
                                        .child(t!("no_transfers_yet").to_string()),
                                ),
                            );
                        }
                        let list = v_flex().gap_2().children(transfers.into_iter().map(|t| {
                            let (icon, _color) = match t.info.kind {
                                crate::terminal::TransferType::Upload => {
                                    (IconName::ArrowUp, cx.theme().primary)
                                }
                                crate::terminal::TransferType::Download => {
                                    (IconName::ArrowDown, cx.theme().success)
                                }
                            };

                            let (status_text, actions) = match t.state {
                                crate::terminal::TransferState::Running => {
                                    let percent = t
                                        .total
                                        .map(|tot| {
                                            (t.transferred as f64 / tot as f64 * 100.0)
                                                .clamp(0.0, 100.0)
                                        })
                                        .unwrap_or(0.0);
                                    let txt = if let Some(tot) = t.total {
                                        format!(
                                            "{:.1}% ({}/{})",
                                            percent,
                                            format_bytes(t.transferred),
                                            format_bytes(tot)
                                        )
                                    } else {
                                        match t.info.kind {
                                            crate::terminal::TransferType::Upload => {
                                                format!("{}...", t!("uploading"))
                                            }
                                            crate::terminal::TransferType::Download => {
                                                format!("{}...", t!("downloading"))
                                            }
                                        }
                                    };
                                    let btn_pause = Button::new(SharedString::from(format!(
                                        "pause-{}",
                                        t.info.id
                                    )))
                                    .ghost()
                                    .small()
                                    .icon(IconName::Pause)
                                    .on_click(window.listener_for(&view, {
                                        let id = t.info.id.clone();
                                        move |this, _, _, _| {
                                            if let Some(handle) = this.active_sftp_handle() {
                                                handle.pause_transfer(id.clone());
                                            }
                                        }
                                    }));
                                    let btn_cancel = Button::new(SharedString::from(format!(
                                        "cancel-{}",
                                        t.info.id
                                    )))
                                    .ghost()
                                    .small()
                                    .icon(IconName::Close)
                                    .on_click(window.listener_for(&view, {
                                        let id = t.info.id.clone();
                                        move |this, _, _, _| {
                                            if let Some(handle) = this.active_sftp_handle() {
                                                handle.cancel_transfer(id.clone());
                                            }
                                        }
                                    }));
                                    (txt, h_flex().gap_1().child(btn_pause).child(btn_cancel))
                                }
                                crate::terminal::TransferState::Paused => {
                                    let txt = t!("paused").to_string();
                                    let btn_resume = Button::new(SharedString::from(format!(
                                        "resume-{}",
                                        t.info.id
                                    )))
                                    .ghost()
                                    .small()
                                    .icon(IconName::Play)
                                    .on_click(window.listener_for(&view, {
                                        let id = t.info.id.clone();
                                        move |this, _, _, _| {
                                            if let Some(handle) = this.active_sftp_handle() {
                                                handle.resume_transfer(id.clone());
                                            }
                                        }
                                    }));
                                    let btn_cancel = Button::new(SharedString::from(format!(
                                        "cancel-{}",
                                        t.info.id
                                    )))
                                    .ghost()
                                    .small()
                                    .icon(IconName::Close)
                                    .on_click(window.listener_for(&view, {
                                        let id = t.info.id.clone();
                                        move |this, _, _, _| {
                                            if let Some(handle) = this.active_sftp_handle() {
                                                handle.cancel_transfer(id.clone());
                                            }
                                        }
                                    }));
                                    (txt, h_flex().gap_1().child(btn_resume).child(btn_cancel))
                                }
                                crate::terminal::TransferState::Interrupted(ref reason) => {
                                    let txt = format!("{}: {}", t!("interrupted"), reason);
                                    let btn_remove = Button::new(SharedString::from(format!(
                                        "remove-{}",
                                        t.info.id
                                    )))
                                    .ghost()
                                    .small()
                                    .icon(IconName::Close)
                                    .on_click(window.listener_for(&view, {
                                        let id = t.info.id.clone();
                                        move |this, _, _, cx| {
                                            this.remove_transfer(&id, cx);
                                        }
                                    }));
                                    (txt, h_flex().gap_1().child(btn_remove))
                                }
                                crate::terminal::TransferState::Completed => {
                                    let txt = t!("completed").to_string();
                                    let mut actions = h_flex().gap_1();
                                    if matches!(
                                        t.info.kind,
                                        crate::terminal::TransferType::Download
                                    ) {
                                        let btn_folder = Button::new(SharedString::from(format!(
                                            "folder-{}",
                                            t.info.id
                                        )))
                                        .ghost()
                                        .small()
                                        .icon(IconName::Folder)
                                        .on_click({
                                            let target = t.info.target.clone();
                                            move |_, _, _| {
                                                let _ = std::process::Command::new("open")
                                                    .arg(&target)
                                                    .spawn();
                                            }
                                        });
                                        actions = actions.child(btn_folder);
                                    }
                                    let btn_remove = Button::new(SharedString::from(format!(
                                        "remove-{}",
                                        t.info.id
                                    )))
                                    .ghost()
                                    .small()
                                    .icon(IconName::Close)
                                    .on_click(window.listener_for(&view, {
                                        let id = t.info.id.clone();
                                        move |this, _, _, cx| {
                                            this.remove_transfer(&id, cx);
                                        }
                                    }));
                                    actions = actions.child(btn_remove);
                                    (txt, actions)
                                }
                                crate::terminal::TransferState::Failed(ref err) => {
                                    let txt = format!("{}: {}", t!("failed"), err);
                                    let btn_remove = Button::new(SharedString::from(format!(
                                        "remove-{}",
                                        t.info.id
                                    )))
                                    .ghost()
                                    .small()
                                    .icon(IconName::Close)
                                    .on_click(window.listener_for(&view, {
                                        let id = t.info.id.clone();
                                        move |this, _, _, cx| {
                                            this.remove_transfer(&id, cx);
                                        }
                                    }));
                                    (txt, h_flex().gap_1().child(btn_remove))
                                }
                                crate::terminal::TransferState::Zombie(ref reason) => {
                                    let txt = format!("{}: {}", t!("zombie"), reason);
                                    let btn_remove = Button::new(SharedString::from(format!(
                                        "remove-{}",
                                        t.info.id
                                    )))
                                    .ghost()
                                    .small()
                                    .icon(IconName::Close)
                                    .on_click(window.listener_for(&view, {
                                        let id = t.info.id.clone();
                                        move |this, _, _, cx| {
                                            this.remove_transfer(&id, cx);
                                        }
                                    }));
                                    (txt, h_flex().gap_1().child(btn_remove))
                                }
                            };

                            let percent = match t.state {
                                crate::terminal::TransferState::Completed => 100.0,
                                _ => t
                                    .total
                                    .map(|tot| t.transferred as f64 / tot as f64 * 100.0)
                                    .unwrap_or(0.0),
                            };

                            v_flex()
                                .gap_1()
                                .p_2()
                                .rounded_md()
                                .border_1()
                                .border_color(cx.theme().border)
                                .bg(cx.theme().muted)
                                .child(
                                    h_flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            Button::new(SharedString::from(format!(
                                                "icon-{}",
                                                t.info.id
                                            )))
                                            .icon(icon)
                                            .ghost()
                                            .small()
                                            .disabled(true),
                                        )
                                        .child(
                                            v_flex()
                                                .flex_1()
                                                .min_w(px(0.))
                                                .overflow_hidden()
                                                .child(
                                                    div()
                                                        .text_size(px(12.))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(cx.theme().foreground)
                                                        .overflow_hidden()
                                                        .child(t.info.name.clone()),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(10.))
                                                        .text_color(cx.theme().muted_foreground)
                                                        .overflow_hidden()
                                                        .child(format!(
                                                            "{}: {}",
                                                            t!("session"),
                                                            t.tab_title
                                                        )),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(11.))
                                                        .text_color(cx.theme().muted_foreground)
                                                        .child(status_text.clone()),
                                                ),
                                        )
                                        .child(actions),
                                )
                                .when(
                                    matches!(
                                        t.state,
                                        crate::terminal::TransferState::Running
                                            | crate::terminal::TransferState::Paused
                                    ),
                                    |this| {
                                        this.child(
                                            Progress::new(format!("progress-{}", t.info.id))
                                                .with_size(px(4.))
                                                .value(percent as f32)
                                                .color(cx.theme().primary)
                                                .w_full(),
                                        )
                                    },
                                )
                        }));

                        let scroll_handle = window
                            .use_keyed_state("transfers-scroll", cx, |_, _| {
                                gpui::ScrollHandle::default()
                            })
                            .read(cx)
                            .clone();

                        content.child(
                            v_flex().gap_2().child(header).child(
                                div()
                                    .w_full()
                                    .relative()
                                    .child(
                                        div()
                                            .w_full()
                                            .max_h(px(400.))
                                            .flex_col()
                                            .id("transfers-scroll-view")
                                            .track_scroll(&scroll_handle)
                                            .overflow_y_scroll()
                                            .pr(px(14.))
                                            .child(list),
                                    )
                                    .child(
                                        div()
                                            .absolute()
                                            .top_0()
                                            .right_0()
                                            .bottom_0()
                                            .w(px(16.))
                                            .child(
                                                Scrollbar::vertical(&scroll_handle)
                                                    .scrollbar_show(ScrollbarShow::Always),
                                            ),
                                    ),
                            ),
                        )
                    }
                })
        });
    }
    pub(crate) fn show_delete_confirm_dialog(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let view = cx.entity();
        let selected_entries = self
            .active_sftp()
            .map(|s| s.selected_entries.clone())
            .unwrap_or_default();
        if selected_entries.is_empty() {
            return;
        }

        let has_system_path = selected_entries.iter().any(|path| {
            let p = path.as_str();
            p.starts_with("/bin/")
                || p == "/bin"
                || p.starts_with("/etc/")
                || p == "/etc"
                || p.starts_with("/usr/")
                || p == "/usr"
                || p.starts_with("/var/")
                || p == "/var"
                || p.starts_with("/sys/")
                || p == "/sys"
                || p.starts_with("/dev/")
                || p == "/dev"
                || p.starts_with("/boot/")
                || p == "/boot"
                || p.starts_with("/lib/")
                || p == "/lib"
                || p.starts_with("/opt/")
                || p == "/opt"
                || p.starts_with("/run/")
                || p == "/run"
                || p.starts_with("/sbin/")
                || p == "/sbin"
        });

        window.open_dialog(cx, move |dialog: Dialog, _window, _| {
            dialog
                .title(t!("confirm_delete").to_string())
                .w(px(500.))
                .keyboard(false)
                .on_ok({
                    let view = view.clone();
                    let paths_to_delete: Vec<String> =
                        selected_entries.clone().into_iter().collect();
                    move |_, window, cx| {
                        view.update(cx, |this, cx| {
                            if let Some(handle) = this.active_sftp_handle() {
                                let _ = handle.commands.send(
                                    crate::sftp::SftpCommand::DeletePaths(paths_to_delete.clone()),
                                );
                            }
                            if let Some(sftp) = this.active_sftp_mut() {
                                sftp.selected_entries.clear();
                            }
                            cx.notify();
                        });
                        window.close_dialog(cx);
                        true
                    }
                })
                .content({
                    let view = view.clone();
                    move |content, _window, cx| {
                        let scroll_handle = view.read(cx).sftp_delete_scroll_handle.clone();
                        let selected_paths: Vec<String> = view
                            .read(cx)
                            .active_sftp()
                            .map(|s| s.selected_entries.clone().into_iter().collect())
                            .unwrap_or_default();

                        let warning_block = if has_system_path {
                            Some(
                                div()
                                    .w_full()
                                    .p_3()
                                    .mb_3()
                                    .rounded_md()
                                    .bg(gpui::rgba(0xff00001a))
                                    .border_1()
                                    .border_color(gpui::rgba(0xff000080))
                                    .child(
                                        div()
                                            .text_color(gpui::rgba(0xff0000ff))
                                            .font_weight(FontWeight::BOLD)
                                            .child(t!("system_path_warning").to_string()),
                                    ),
                            )
                        } else {
                            None
                        };

                        let paths_list = div()
                            .relative()
                            .max_h(px(200.))
                            .w_full()
                            .border_1()
                            .border_color(cx.theme().border)
                            .bg(cx.theme().background)
                            .rounded_md()
                            .child(
                                v_flex()
                                    .id("delete-scroll-view")
                                    .size_full()
                                    .track_scroll(&scroll_handle)
                                    .overflow_y_scroll()
                                    .p_2()
                                    .gap_1()
                                    .children(selected_paths.into_iter().map(|path| {
                                        div()
                                            .text_size(rems(0.917))
                                            .text_color(cx.theme().muted_foreground)
                                            .child(path)
                                    })),
                            )
                            .child(
                                div().absolute().top_0().bottom_0().right_0().child(
                                    gpui_component::scroll::Scrollbar::vertical(&scroll_handle)
                                        .scrollbar_show(
                                            gpui_component::scroll::ScrollbarShow::Always,
                                        ),
                                ),
                            );

                        content.child(
                            v_flex()
                                .w_full()
                                .gap_2()
                                .children(warning_block)
                                .child(
                                    div().text_size(rems(1.0)).mb_2().child(
                                        t!(
                                            "confirm_delete_desc",
                                            count = view
                                                .read(cx)
                                                .active_sftp()
                                                .map(|s| s.selected_entries.len())
                                                .unwrap_or(0)
                                        )
                                        .to_string(),
                                    ),
                                )
                                .child(paths_list),
                        )
                    }
                })
                .footer({
                    let view = view.clone();
                    let paths_to_delete: Vec<String> =
                        selected_entries.clone().into_iter().collect();
                    h_flex()
                        .w_full()
                        .justify_end()
                        .gap_2()
                        .child(
                            Button::new("cancel")
                                .ghost()
                                .label(t!("cancel").to_string())
                                .on_click(move |_, window, cx| {
                                    window.close_dialog(cx);
                                }),
                        )
                        .child(
                            Button::new("confirm")
                                .danger()
                                .label(t!("confirm").to_string())
                                .on_click({
                                    let view = view.clone();
                                    move |_, window, cx| {
                                        view.update(cx, |this, cx| {
                                            if let Some(handle) = this.active_sftp_handle() {
                                                let _ = handle.commands.send(
                                                    crate::sftp::SftpCommand::DeletePaths(
                                                        paths_to_delete.clone(),
                                                    ),
                                                );
                                            }
                                            if let Some(sftp) = this.active_sftp_mut() {
                                                sftp.selected_entries.clear();
                                            }
                                            cx.notify();
                                        });
                                        window.close_dialog(cx);
                                    }
                                }),
                        )
                })
        });
    }
    pub(crate) fn show_settings_dialog(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.open_settings_page(cx);
    }

    pub(crate) fn render_settings_page(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        use gpui::IntoElement;
        use gpui_component::setting::{
            SettingField, SettingGroup, SettingItem, SettingPage, Settings,
        };

        let view = cx.entity();
        let version = crate::app::constants::public_version_label();
        let view_clone_for_general = view.clone();
        let view_clone_for_custom = view.clone();
        let follow_system_theme = self.follow_system_theme;
        let theme_mode_is_dark = self.theme_mode.is_dark();
        let light_theme_name = self.light_theme_name.to_string();
        let dark_theme_name = self.dark_theme_name.to_string();
        let title_bar_style = self.config.effective_title_bar_style();
        let ui_font_size = self.ui_font_size;
        let terminal_font_size = self.terminal_font_size;
        let ui_font_family = self.ui_font_family.to_string();
        let terminal_font_family = self.terminal_font_family.to_string();
        let cursor_style = self.cursor_style;
        let right_click_copy_paste = self.config.right_click_copy_paste();
        let keyword_highlight = self.config.keyword_highlight();
        let lock_layout = self.config.lock_layout();
        let show_monitoring_dashboard = self.config.show_monitoring_dashboard();
        let monitoring_position = self.config.monitoring_position().to_string();
        let current_locale = self.config.locale().to_string();
        let sync_endpoint_input = self.sync_endpoint_input.clone();
        let sync_username_input = self.sync_username_input.clone();
        let sync_webdav_password_input = self.sync_webdav_password_input.clone();
        let sync_s3_endpoint_input = self.sync_s3_endpoint_input.clone();
        let sync_s3_region_input = self.sync_s3_region_input.clone();
        let sync_s3_bucket_input = self.sync_s3_bucket_input.clone();
        let sync_s3_object_key_input = self.sync_s3_object_key_input.clone();
        let sync_s3_access_key_input = self.sync_s3_access_key_input.clone();
        let sync_s3_secret_key_input = self.sync_s3_secret_key_input.clone();
        let sync_s3_session_token_input = self.sync_s3_session_token_input.clone();
        let sync_encryption_password_input = self.sync_encryption_password_input.clone();
        let sync_in_progress = self.sync_in_progress;
        let sync_status = self.sync_status.clone();
        let sync_backend_is_s3 = self.config.sync_backend() == "s3";
        let muted_foreground = cx.theme().muted_foreground;
        let use_proxy = self.config.use_proxy();
        let read_env_proxy = self.config.read_env_proxy();
        let x11_forwarding_enabled = self.config.x11_forwarding_enabled();
        let x11_launch_xquartz = self.config.x11_launch_xquartz();
        let xquartz_app_path_input = self.xquartz_app_path_input.clone();
        let global_proxy_host_input = self.global_proxy_host_input.clone();
        let global_proxy_port_input = self.global_proxy_port_input.clone();
        let global_proxy_user_input = self.global_proxy_user_input.clone();
        let global_proxy_password_input = self.global_proxy_password_input.clone();
        let global_proxy_type = self.global_proxy_type.clone();
        let custom_theme_name_input = self
            .custom_theme_inputs
            .get(crate::app::theme::custom_theme_name_input_key())
            .expect("custom theme name input missing")
            .clone();
        let custom_light_base_name =
            self.resolved_custom_theme_base_name(gpui_component::ThemeMode::Light, cx);
        let custom_dark_base_name =
            self.resolved_custom_theme_base_name(gpui_component::ThemeMode::Dark, cx);
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
                    let input = self
                        .custom_theme_inputs
                        .get(&input_key)
                        .expect("custom theme input missing")
                        .clone();
                    let width =
                        if field.domain == crate::app::theme::CustomThemeFieldDomain::Brightness {
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
                                move |_, _window, _cx| {
                                    Input::new(&input).w(width).into_any_element()
                                }
                            }),
                        )
                        .description(description),
                    );
                }
            }

            custom_theme_page = custom_theme_page.group(group);
        }
        div()
            .flex()
            .flex_col()
            .size_full()
            .track_focus(&self.focus_handle)
            .on_key_down({
                let view = view.clone();
                move |ev: &gpui::KeyDownEvent, window, cx| {
                    view.update(cx, |this, cx| {
                        if window.focused(cx) != Some(this.focus_handle.clone()) {
                            if this.recording_action.is_some() {
                                this.recording_action = None;
                                cx.notify();
                            }
                            return;
                        }

                        if this.recording_action.is_none() {
                            if crate::app::keybinding_recorder::event_matches_action(
                                &this.config,
                                "PrevTab",
                                ev,
                            ) {
                                this.switch_workspace_tab(-1, window, cx);
                                window.prevent_default();
                                cx.stop_propagation();
                                return;
                            }

                            if crate::app::keybinding_recorder::event_matches_action(
                                &this.config,
                                "NextTab",
                                ev,
                            ) {
                                this.switch_workspace_tab(1, window, cx);
                                window.prevent_default();
                                cx.stop_propagation();
                                return;
                            }
                        }

                        let Some(action) = this.recording_action.clone() else {
                            return;
                        };

                        window.prevent_default();
                        cx.stop_propagation();

                        if ev.keystroke.key == "escape" {
                            this.recording_action = None;
                            cx.notify();
                            return;
                        }

                        let Some(new_key) =
                            crate::app::keybinding_recorder::normalize_recorded_keystroke(ev)
                        else {
                            return;
                        };

                        if let Some((_conflict_id, conflict_label)) =
                            crate::app::keybinding_recorder::find_conflict(
                                &this.config,
                                &action,
                                &new_key,
                            )
                        {
                            let formatted =
                                crate::app::keybinding_recorder::format_keystroke(&new_key);
                            this.recording_action = None;
                            this.keybind_error = Some((
                                action.clone(),
                                t!("keybind_conflict", key = formatted, action = conflict_label)
                                    .to_string(),
                            ));
                            cx.notify();
                            return;
                        }

                        this.recording_action = None;
                        this.keybind_error = None;
                        this.config.set_key_binding(&action, &new_key);
                        if let Err(err) = this.config.save() {
                            tracing::error!("failed to save key binding: {err:#}");
                        }
                        cx.notify();
                    });
                }
            })
            .on_mouse_down_out({
                let view = view.clone();
                move |_, _window, cx| {
                    view.update(cx, |this, cx| {
                        if this.recording_action.is_some() {
                            this.recording_action = None;
                            cx.notify();
                        }
                    });
                }
            })
            .child(
                Settings::new("settings")
                    .sidebar_width(px(180.))
                    .sidebar_style(div().bg(cx.theme().background).style())
                                .page(
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
                                )
                                .page(custom_theme_page)
                                /*
                                .page(
                                    SettingPage::new(t!("settings_custom").to_string())
                                        .icon(IconName::Settings)
                                        .group(
                                            SettingGroup::new()
                                                .title(t!("settings_custom_theme").to_string())
                                                .description(
                                                    t!("settings_custom_config_hint").to_string(),
                                                )
                                                .item(SettingItem::render(|_, _window, _cx| {
                                                    div()
                                                        .text_sm()
                                                        .font_weight(FontWeight::BOLD)
                                                        .child(t!("settings_custom_theme_mode").to_string())
                                                }))
                                                .item(
                                                    SettingItem::new(
                                                        t!("theme_mode").to_string(),
                                                        SettingField::render({
                                                            let view = view_clone_for_general.clone();
                                                            let follow_system = follow_system_theme;
                                                            let is_dark_mode = theme_mode_is_dark;
                                                            move |_, _window, _cx| {
                                                                Button::new("custom-theme-mode-dropdown")
                                                                    .small()
                                                                    .icon(if follow_system {
                                                                        IconName::Sun
                                                                    } else if is_dark_mode {
                                                                        IconName::Moon
                                                                    } else {
                                                                        IconName::Sun
                                                                    })
                                                                    .label(if follow_system {
                                                                        t!("follow_system").to_string()
                                                                    } else if is_dark_mode {
                                                                        t!("use_dark_mode").to_string()
                                                                    } else {
                                                                        t!("use_light_mode").to_string()
                                                                    })
                                                                    .dropdown_menu_with_anchor(
                                                                        Anchor::BottomRight,
                                                                        {
                                                                            let view = view.clone();
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
                                                                        },
                                                                    )
                                                                    .into_any_element()
                                                            }
                                                        }),
                                                    )
                                                    .description(
                                                        "keys: follow_system_theme, theme_mode; default: follow system",
                                                    ),
                                                )
                                                .item(SettingItem::render(|_, _window, _cx| {
                                                    div()
                                                        .pt_2()
                                                        .text_sm()
                                                        .font_weight(FontWeight::BOLD)
                                                        .child(t!("settings_custom_theme_presets").to_string())
                                                }))
                                                .item(
                                                    SettingItem::new(
                                                        t!("light_theme").to_string(),
                                                        SettingField::render({
                                                            let view = view_clone_for_general.clone();
                                                            let current_theme = light_theme_name.clone();
                                                            let custom_theme = custom_theme_name.clone();
                                                            move |_, _window, _cx| {
                                                                Button::new("custom-light-theme-dropdown")
                                                                    .small()
                                                                    .icon(IconName::Sun)
                                                                    .label(current_theme.clone())
                                                                    .dropdown_menu_with_anchor(
                                                                        Anchor::BottomRight,
                                                                        {
                                                                            let view = view.clone();
                                                                            let current_theme = current_theme.clone();
                                                                            let custom_theme = custom_theme.clone();
                                                                            move |mut menu, window, cx| {
                                                                                let themes = gpui_component::ThemeRegistry::global(cx).sorted_themes();
                                                                                let light_themes: Vec<_> = themes
                                                                                    .into_iter()
                                                                                    .filter(|t| !t.mode.is_dark())
                                                                                    .map(|t| t.name.clone())
                                                                                    .collect();
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
                                                                                menu = menu.separator().item(
                                                                                    PopupMenuItem::new(custom_theme.clone())
                                                                                        .checked(current_theme == custom_theme)
                                                                                        .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                                            this.apply_custom_theme(gpui_component::ThemeMode::Light, window, cx)
                                                                                        }))
                                                                                );
                                                                                menu
                                                                            }
                                                                        },
                                                                    )
                                                                    .into_any_element()
                                                            }
                                                        }),
                                                    )
                                                    .description("key: light_theme_name; default: empty"),
                                                )
                                                .item(
                                                    SettingItem::new(
                                                        t!("dark_theme").to_string(),
                                                        SettingField::render({
                                                            let view = view_clone_for_general.clone();
                                                            let current_theme = dark_theme_name.clone();
                                                            let custom_theme = custom_theme_name.clone();
                                                            move |_, _window, _cx| {
                                                                Button::new("custom-dark-theme-dropdown")
                                                                    .small()
                                                                    .icon(IconName::Moon)
                                                                    .label(current_theme.clone())
                                                                    .dropdown_menu_with_anchor(
                                                                        Anchor::BottomRight,
                                                                        {
                                                                            let view = view.clone();
                                                                            let current_theme = current_theme.clone();
                                                                            let custom_theme = custom_theme.clone();
                                                                            move |mut menu, window, cx| {
                                                                                let themes = gpui_component::ThemeRegistry::global(cx).sorted_themes();
                                                                                let dark_themes: Vec<_> = themes
                                                                                    .into_iter()
                                                                                    .filter(|t| t.mode.is_dark())
                                                                                    .map(|t| t.name.clone())
                                                                                    .collect();
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
                                                                                menu = menu.separator().item(
                                                                                    PopupMenuItem::new(custom_theme.clone())
                                                                                        .checked(current_theme == custom_theme)
                                                                                        .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                                            this.apply_custom_theme(gpui_component::ThemeMode::Dark, window, cx)
                                                                                        }))
                                                                                );
                                                                                menu
                                                                            }
                                                                        },
                                                                    )
                                                                    .into_any_element()
                                                            }
                                                        }),
                                                    )
                                                    .description("key: dark_theme_name; default: empty"),
                                                )
                                                .item(
                                                    SettingItem::new(
                                                        format!(
                                                            "{}{}",
                                                            t!("title_bar_style"),
                                                            t!("restart_hint")
                                                        ),
                                                        SettingField::render({
                                                            let view = view_clone_for_general.clone();
                                                            let current_style = title_bar_style;
                                                            move |_, _window, _cx| {
                                                                let supports_integrated =
                                                                    cfg!(target_os = "macos");
                                                                Button::new("custom-title-bar-style-dropdown")
                                                                    .small()
                                                                    .label(match current_style {
                                                                        crate::session::config::TitleBarStyle::Native => t!("title_bar_native").to_string(),
                                                                        crate::session::config::TitleBarStyle::Integrated => t!("title_bar_integrated").to_string(),
                                                                    })
                                                                    .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                                                        let view = view.clone();
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
                                                        }),
                                                    )
                                                    .description("key: title_bar_style; default: integrated"),
                                                )
                                                .item(SettingItem::render(|_, _window, _cx| {
                                                    div()
                                                        .pt_2()
                                                        .text_sm()
                                                        .font_weight(FontWeight::BOLD)
                                                        .child(t!("settings_custom_theme_fonts").to_string())
                                                }))
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
                                                                    .child(Button::new("custom-ui-font-size-down").small().label("-").on_click(window.listener_for(&view, |this, _, _, cx| this.change_ui_font_size(-1.0, cx))))
                                                                    .child(div().min_w(px(64.)).text_center().child(format!("{:.0}px", current_ui_font_size)))
                                                                    .child(Button::new("custom-ui-font-size-up").small().label("+").on_click(window.listener_for(&view, |this, _, _, cx| this.change_ui_font_size(1.0, cx))))
                                                                    .into_any_element()
                                                            }
                                                        }),
                                                    )
                                                    .description("key: ui_font_size; default: 14.0"),
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
                                                                    .child(Button::new("custom-terminal-font-size-down").small().label("-").on_click(window.listener_for(&view, |this, _, _, cx| this.change_terminal_font_size(-1.0, cx))))
                                                                    .child(div().min_w(px(64.)).text_center().child(format!("{:.0}px", current_terminal_font_size)))
                                                                    .child(Button::new("custom-terminal-font-size-up").small().label("+").on_click(window.listener_for(&view, |this, _, _, cx| this.change_terminal_font_size(1.0, cx))))
                                                                    .into_any_element()
                                                            }
                                                        }),
                                                    )
                                                    .description("key: terminal_font_size; default: 18.0"),
                                                )
                                                .item(
                                                    SettingItem::new(
                                                        t!("ui_font_family").to_string(),
                                                        SettingField::render({
                                                            let view = view_clone_for_general.clone();
                                                            let current = ui_font_family.clone();
                                                            move |_, _window, cx| {
                                                                Button::new("custom-ui-font-dropdown")
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
                                                        }),
                                                    )
                                                    .description(
                                                        "key: ui_font_family; default: .SystemUIFont",
                                                    ),
                                                )
                                                .item(
                                                    SettingItem::new(
                                                        t!("terminal_font_family").to_string(),
                                                        SettingField::render({
                                                            let view = view_clone_for_general.clone();
                                                            let current = terminal_font_family.clone();
                                                            let current_terminal_font_size = terminal_font_size;
                                                            move |_, window, _cx| {
                                                                Button::new("custom-terminal-font-dropdown")
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
                                                        }),
                                                    )
                                                    .description(
                                                        "key: terminal_font_family; default: Maple Mono NF CN",
                                                    ),
                                                )
                                                .item(SettingItem::render(|_, _window, _cx| {
                                                    div()
                                                        .pt_2()
                                                        .text_sm()
                                                        .font_weight(FontWeight::BOLD)
                                                        .child(t!("settings_custom_theme_overrides").to_string())
                                                }))
                                                .item(
                                                    SettingItem::new(
                                                        t!("custom_theme_name").to_string(),
                                                        SettingField::render({
                                                            let input = custom_theme_name_input.clone();
                                                            move |_, _window, _cx| {
                                                                Input::new(&input)
                                                                    .w(px(180.))
                                                                    .into_any_element()
                                                            }
                                                        }),
                                                    )
                                                    .description(
                                                        "key: custom_theme_name; default: Custom Theme",
                                                    ),
                                                )
                                                .item(
                                                    SettingItem::new(
                                                        t!("custom_primary_color").to_string(),
                                                        SettingField::render({
                                                            let input = custom_primary_color_input.clone();
                                                            move |_, _window, _cx| {
                                                                Input::new(&input)
                                                                    .w(px(160.))
                                                                    .into_any_element()
                                                            }
                                                        }),
                                                    )
                                                    .description(
                                                        format!(
                                                            "{} key: custom_primary_color; default: empty",
                                                            t!("custom_color_hint")
                                                        ),
                                                    ),
                                                )
                                                .item(
                                                    SettingItem::new(
                                                        t!("custom_background_color").to_string(),
                                                        SettingField::render({
                                                            let input = custom_background_color_input.clone();
                                                            move |_, _window, _cx| {
                                                                Input::new(&input)
                                                                    .w(px(160.))
                                                                    .into_any_element()
                                                            }
                                                        }),
                                                    )
                                                    .description(
                                                        format!(
                                                            "{} key: custom_background_color; default: empty",
                                                            t!("custom_background_hint")
                                                        ),
                                                    ),
                                                )
                                                .item(
                                                    SettingItem::new(
                                                        t!("custom_font_brightness").to_string(),
                                                        SettingField::render({
                                                            let input = custom_font_brightness_input.clone();
                                                            move |_, _window, _cx| {
                                                                Input::new(&input)
                                                                    .w(px(96.))
                                                                    .into_any_element()
                                                            }
                                                        }),
                                                    )
                                                    .description(
                                                        format!(
                                                            "{} key: custom_font_brightness; default: 1.0",
                                                            t!("custom_font_brightness_hint")
                                                        ),
                                                    ),
                                                )
                                                .item(SettingItem::new(
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
                                                                        .on_click(window.listener_for(
                                                                            &view,
                                                                            |this, _, window, cx| {
                                                                                this.save_custom_appearance(window, cx);
                                                                            },
                                                                        )),
                                                                )
                                                                .child(
                                                                    Button::new("custom-appearance-reset")
                                                                        .ghost()
                                                                        .label(t!("reset").to_string())
                                                                        .on_click(window.listener_for(
                                                                            &view,
                                                                            |this, _, window, cx| {
                                                                                this.reset_custom_appearance(window, cx);
                                                                            },
                                                                        )),
                                                                )
                                                                .into_any_element()
                                                        }
                                                    }),
                                                ))
                                        )
                                )
                                */
                                .page(
                                    SettingPage::new(t!("settings_sync").to_string())
                                        .icon(IconName::Globe)
                                        .group(
                                            SettingGroup::new()
                                                .title(t!("settings_sync").to_string())
                                                .item(SettingItem::render({
                                                    let view = view.clone();
                                                    let endpoint = sync_endpoint_input.clone();
                                                    let username = sync_username_input.clone();
                                                    let webdav_password = sync_webdav_password_input.clone();
                                                    let s3_endpoint = sync_s3_endpoint_input.clone();
                                                    let s3_region = sync_s3_region_input.clone();
                                                    let s3_bucket = sync_s3_bucket_input.clone();
                                                    let s3_object_key = sync_s3_object_key_input.clone();
                                                    let s3_access_key = sync_s3_access_key_input.clone();
                                                    let s3_secret_key = sync_s3_secret_key_input.clone();
                                                    let s3_session_token = sync_s3_session_token_input.clone();
                                                    let encryption_password = sync_encryption_password_input.clone();
                                                    let in_progress = sync_in_progress;
                                                    let status = sync_status.clone();
                                                    let is_s3 = sync_backend_is_s3;
                                                    let muted_foreground = muted_foreground;
                                                    move |_, window, _cx| {
                                                        v_flex()
                                                            .w_full()
                                                            .gap_3()
                                                            .child(
                                                                h_flex()
                                                                    .gap_2()
                                                                    .child(
                                                                        Button::new("sync-backend-webdav")
                                                                            .small()
                                                                            .label("WebDAV")
                                                                            .when(!is_s3, |button| button.primary())
                                                                            .on_click(window.listener_for(&view, |this, _, _, cx| this.set_sync_backend("webdav", cx)))
                                                                    )
                                                                    .child(
                                                                        Button::new("sync-backend-s3")
                                                                            .small()
                                                                            .label("S3")
                                                                            .when(is_s3, |button| button.primary())
                                                                            .on_click(window.listener_for(&view, |this, _, _, cx| this.set_sync_backend("s3", cx)))
                                                                    )
                                                            )
                                                            .when(!is_s3, |this| this
                                                                .child(v_flex().gap_1().child(div().text_sm().child(t!("sync_endpoint").to_string())).child(Input::new(&endpoint).w_full()))
                                                                .child(v_flex().gap_1().child(div().text_sm().child(t!("sync_username").to_string())).child(Input::new(&username).w_full()))
                                                                .child(v_flex().gap_1().child(div().text_sm().child(t!("sync_webdav_password").to_string())).child(Input::new(&webdav_password).w_full())))
                                                            .when(is_s3, |this| this
                                                                .child(v_flex().gap_1().child(div().text_sm().child(t!("sync_s3_endpoint").to_string())).child(Input::new(&s3_endpoint).w_full()))
                                                                .child(h_flex().gap_2()
                                                                    .child(v_flex().flex_1().gap_1().child(div().text_sm().child(t!("sync_s3_region").to_string())).child(Input::new(&s3_region).w_full()))
                                                                    .child(v_flex().flex_1().gap_1().child(div().text_sm().child(t!("sync_s3_bucket").to_string())).child(Input::new(&s3_bucket).w_full())))
                                                                .child(v_flex().gap_1().child(div().text_sm().child(t!("sync_s3_object_key").to_string())).child(Input::new(&s3_object_key).w_full()))
                                                                .child(v_flex().gap_1().child(div().text_sm().child(t!("sync_s3_access_key").to_string())).child(Input::new(&s3_access_key).w_full()))
                                                                .child(v_flex().gap_1().child(div().text_sm().child(t!("sync_s3_secret_key").to_string())).child(Input::new(&s3_secret_key).w_full()))
                                                                .child(v_flex().gap_1().child(div().text_sm().child(t!("sync_s3_session_token").to_string())).child(Input::new(&s3_session_token).w_full())))
                                                            .child(v_flex().gap_1().child(div().text_sm().child(t!("sync_encryption_password").to_string())).child(Input::new(&encryption_password).w_full()))
                                                            .child(div().text_sm().text_color(muted_foreground).child(t!("sync_security_hint").to_string()))
                                                            .child(
                                                                h_flex()
                                                                    .gap_2()
                                                                    .child(Button::new("sync-download").small().disabled(in_progress).label(t!("sync_download").to_string()).on_click(window.listener_for(&view, |this, _, _, cx| this.download_sync_config(cx))))
                                                                    .child(Button::new("sync-upload").small().disabled(in_progress).label(t!("sync_upload").to_string()).on_click(window.listener_for(&view, |this, _, _, cx| this.upload_sync_config(cx)))),
                                                            )
                                                            .child(div().text_sm().text_color(muted_foreground).child(status.clone()))
                                                    }
                                                }))
                                        )
                                )
                                .page(
                                    SettingPage::new(t!("settings_proxy").to_string())
                                        .icon(IconName::Network)
                                        .group(
                                            SettingGroup::new()
                                                .title(t!("settings_proxy").to_string())
                                                .item(
                                                    SettingItem::new(
                                                        t!("enable_proxy").to_string(),
                                                        SettingField::render({
                                                            let view = view.clone();
                                                            let enabled = use_proxy;
                                                            move |_, window, _cx| {
                                                                Switch::new("use-proxy")
                                                                    .small()
                                                                    .checked(enabled)
                                                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                                                        this.config.set_use_proxy(*checked);
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
                                                        t!("read_env_proxy").to_string(),
                                                        SettingField::render({
                                                            let view = view.clone();
                                                            let enabled = read_env_proxy;
                                                            move |_, window, _cx| {
                                                                Switch::new("read-env-proxy")
                                                                    .small()
                                                                    .checked(enabled)
                                                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                                                        this.config.set_read_env_proxy(*checked);
                                                                        let _ = this.config.save();
                                                                        cx.notify();
                                                                    }))
                                                                    .into_any_element()
                                                            }
                                                        })
                                                    ).description(t!("read_env_proxy_desc").to_string())
                                                )
                                                .item(SettingItem::render({
                                                    let view = view.clone();
                                                    let global_proxy_host_input = global_proxy_host_input.clone();
                                                    let global_proxy_port_input = global_proxy_port_input.clone();
                                                    let global_proxy_user_input = global_proxy_user_input.clone();
                                                    let global_proxy_password_input = global_proxy_password_input.clone();
                                                    let proxy_type = global_proxy_type.clone();
                                                    move |_, window, _cx| {
                                                        v_flex()
                                                            .w_full()
                                                            .gap_3()
                                                            .child(div().text_sm().font_weight(FontWeight::BOLD).child(t!("global_proxy_settings").to_string()))
                                                            .child(
                                                                h_flex()
                                                                    .gap_2()
                                                                    .child(
                                                                        Button::new("global-proxy-type-socks5")
                                                                            .small()
                                                                            .label("SOCKS5")
                                                                            .when(proxy_type == "socks5", |b| b.primary())
                                                                            .on_click(window.listener_for(&view, |this, _, _, cx| {
                                                                                this.global_proxy_type = "socks5".to_string();
                                                                                cx.notify();
                                                                            }))
                                                                    )
                                                                    .child(
                                                                        Button::new("global-proxy-type-http")
                                                                            .small()
                                                                            .label("HTTP")
                                                                            .when(proxy_type == "http", |b| b.primary())
                                                                            .on_click(window.listener_for(&view, |this, _, _, cx| {
                                                                                this.global_proxy_type = "http".to_string();
                                                                                cx.notify();
                                                                            }))
                                                                    )
                                                            )
                                                            .child(v_flex().gap_1().child(div().text_sm().child(t!("global_proxy_host").to_string())).child(Input::new(&global_proxy_host_input).w_full()))
                                                            .child(v_flex().gap_1().child(div().text_sm().child(t!("global_proxy_port").to_string())).child(Input::new(&global_proxy_port_input).w_full()))
                                                            .child(v_flex().gap_1().child(div().text_sm().child(t!("global_proxy_user").to_string())).child(Input::new(&global_proxy_user_input).w_full()))
                                                            .child(v_flex().gap_1().child(div().text_sm().child(t!("global_proxy_password").to_string())).child(Input::new(&global_proxy_password_input).w_full()))
                                                            .child(
                                                                Button::new("save-global-proxy")
                                                                    .small()
                                                                    .primary()
                                                                    .label(t!("save_proxy").to_string())
                                                                    .on_click(window.listener_for(&view, |this, _, _, cx| {
                                                                        let host = this.global_proxy_host_input.read(cx).value().trim().to_string();
                                                                        let port_str = this.global_proxy_port_input.read(cx).value();
                                                                        let port = port_str.trim().parse::<u16>().ok();
                                                                        let user = this.global_proxy_user_input.read(cx).value().trim().to_string();
                                                                        let password = this.global_proxy_password_input.read(cx).value().to_string();

                                                                        if host.is_empty() || port.is_none() {
                                                                            return;
                                                                        }

                                                                        this.config.set_global_proxy_type(this.global_proxy_type.clone());
                                                                        this.config.set_global_proxy_host(host);
                                                                        this.config.set_global_proxy_port(port);
                                                                        this.config.set_global_proxy_user(user);
                                                                        this.config.set_global_proxy_password(password);
                                                                        let _ = this.config.save();
                                                                        cx.notify();
                                                                    }))
                                                            )
                                                    }
                                                }))
                                        )
                                        .group(
                                            SettingGroup::new()
                                                .title(t!("settings_x11").to_string())
                                                .item(
                                                    SettingItem::new(
                                                        t!("enable_x11_forwarding").to_string(),
                                                        SettingField::render({
                                                            let view = view.clone();
                                                            let enabled = x11_forwarding_enabled;
                                                            move |_, window, _cx| {
                                                                Switch::new("x11-forwarding-enabled")
                                                                    .small()
                                                                    .checked(enabled)
                                                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                                                        this.config.set_x11_forwarding_enabled(*checked);
                                                                        let _ = this.config.save();
                                                                        cx.notify();
                                                                    }))
                                                                    .into_any_element()
                                                            }
                                                        })
                                                    ).description(t!("enable_x11_forwarding_desc").to_string())
                                                )
                                                .item(
                                                    SettingItem::new(
                                                        t!("x11_launch_xquartz").to_string(),
                                                        SettingField::render({
                                                            let view = view.clone();
                                                            let enabled = x11_launch_xquartz;
                                                            move |_, window, _cx| {
                                                                Switch::new("x11-launch-xquartz")
                                                                    .small()
                                                                    .checked(enabled)
                                                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                                                        this.config.set_x11_launch_xquartz(*checked);
                                                                        let _ = this.config.save();
                                                                        cx.notify();
                                                                    }))
                                                                    .into_any_element()
                                                            }
                                                        })
                                                    ).description(t!("x11_launch_xquartz_desc").to_string())
                                                )
                                                .item(SettingItem::render({
                                                    let view = view.clone();
                                                    let xquartz_app_path_input = xquartz_app_path_input.clone();
                                                    move |_, window, _cx| {
                                                        v_flex()
                                                            .w_full()
                                                            .items_stretch()
                                                            .gap_3()
                                                            .child(div().text_sm().font_weight(FontWeight::BOLD).child(t!("xquartz_app_path").to_string()))
                                                            .child(
                                                                div()
                                                                    .w_full()
                                                                    .min_w(px(320.))
                                                                    .child(
                                                                        Input::new(&xquartz_app_path_input)
                                                                            .w_full(),
                                                                    ),
                                                            )
                                                            .child(
                                                                h_flex()
                                                                    .w_full()
                                                                    .gap_2()
                                                                    .child(
                                                                        Button::new("browse-xquartz-app")
                                                                            .small()
                                                                            .label(t!("browse").to_string())
                                                                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                                this.pick_xquartz_app_path(window, cx);
                                                                            }))
                                                                    )
                                                                    .child(
                                                                        Button::new("reset-xquartz-app")
                                                                            .small()
                                                                            .label(t!("reset_default").to_string())
                                                                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                                                                this.reset_xquartz_app_path(window, cx);
                                                                            }))
                                                                    )
                                                                    .child(
                                                                        Button::new("save-x11-settings")
                                                                            .small()
                                                                            .primary()
                                                                            .label(t!("save_x11_settings").to_string())
                                                                            .on_click(window.listener_for(&view, |this, _, _, cx| {
                                                                                this.save_x11_settings(cx);
                                                                            }))
                                                                    )
                                                                    .child(
                                                                        Button::new("open-xquartz")
                                                                            .small()
                                                                            .label(t!("open_xquartz").to_string())
                                                                            .on_click(window.listener_for(&view, |this, _, _, cx| {
                                                                                this.open_configured_xquartz(cx);
                                                                            }))
                                                                    )
                                                            )
                                                            .child(
                                                                div()
                                                                    .w_full()
                                                                    .text_xs()
                                                                    .text_color(muted_foreground)
                                                                    .child(t!("xquartz_app_path_desc").to_string()),
                                                            )
                                                    }
                                                }))
                                        )
                                )
                                .page({
                                    let mut page = SettingPage::new(t!("settings_key_bindings").to_string())
                                        .icon(IconName::SquareTerminal)
                                        .default_open(true);
                                    for group in crate::app::keybinding_recorder::KeybindingsPage::render_groups(
                                        &view,
                                        &self.config,
                                        self.recording_action.as_deref(),
                                        self.keybind_error.as_ref(),
                                    ) {
                                        page = page.group(group);
                                    }
                                    page
                                })
                                .page(
                                    SettingPage::new(t!("settings_help").to_string())
                                        .icon(IconName::BookOpen)
                                )
                                .page(
                                    SettingPage::new(t!("settings_about").to_string())
                                        .icon(IconName::Info)
                                        .group(
                                            SettingGroup::new()
                                                .item(SettingItem::render(move |_, _window, cx| {
                                                    v_flex()
                                                        .gap_2()
                                                        .items_center()
                                                        .child(div().text_size(rems(1.5)).font_weight(FontWeight::BOLD).child("AxShell"))
                                                        .child(div().text_size(rems(0.9)).child(format!("Version {}", version)))
                                                        .child(
                                                            div()
                                                                .text_size(rems(0.9))
                                                                .text_color(cx.theme().muted_foreground)
                                                                .child("A GPUI Component based SSH and local terminal client"),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(rems(0.9))
                                                                .text_color(cx.theme().muted_foreground)
                                                                .child(t!("about_feedback_hint")),
                                                        )
                                                        .child(
                                                            Button::new("github-link")
                                                                .label(crate::app::constants::REPOSITORY_URL)
                                                                .ghost()
                                                                .on_click(|_, _window, _cx| {
                                                                    let _ = open::that(crate::app::constants::REPOSITORY_URL);
                                                                }),
                                                        )
                                                }))
                                        )
                                )
            )
    }
}
