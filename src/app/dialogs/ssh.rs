use super::*;

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
}
