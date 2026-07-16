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
        let session_sftp_path_input = self.session_sftp_path_input.clone();
        let serial_port_input = self.serial_port_input.clone();
        let serial_baud_rate_input = self.serial_baud_rate_input.clone();
        let serial_data_bits_input = self.serial_data_bits_input.clone();
        let serial_parity_input = self.serial_parity_input.clone();
        let serial_stop_bits_input = self.serial_stop_bits_input.clone();
        let serial_flow_control_input = self.serial_flow_control_input.clone();
        let focus_serial_port_input = serial_port_input.clone();
        let deferred_view = view.clone();

        window.open_dialog(cx, move |dialog: Dialog, _window, _cx| {
            dialog
                .title(t!("new_connection"))
                .w(px(560.))
                .overlay_closable(false)
                .on_ok({
                    let view = view.clone();
                    move |_, window, cx| {
                        view.update(cx, |this, cx| {
                            this.connect_session(window, cx);
                        });
                        false
                    }
                })
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
                    let session_sftp_path_input = session_sftp_path_input.clone();
                    let serial_port_input = serial_port_input.clone();
                    let serial_baud_rate_input = serial_baud_rate_input.clone();
                    let serial_data_bits_input = serial_data_bits_input.clone();
                    let serial_parity_input = serial_parity_input.clone();
                    let serial_stop_bits_input = serial_stop_bits_input.clone();
                    let serial_flow_control_input = serial_flow_control_input.clone();
                    move |content, window, cx| {
                        let shell = view.read(cx);
                        let session_kind = shell.session_kind;
                        let is_ssh = session_kind == SessionKind::Ssh;
                        let is_serial = session_kind == SessionKind::Serial;
                        let is_telnet = session_kind == SessionKind::Telnet;
                        let is_password = shell.ssh_auth_method == AuthMethod::Password;
                        let proxy_type = shell.ssh_proxy_type.clone();
                        let show_proxy_fields = proxy_type != "none";
                        let show_advanced_options = shell.ssh_advanced_options_visible;
                        let session_x11_forwarding = shell.session_x11_forwarding;
                        let session_legacy_ssh_compatibility =
                            shell.session_legacy_ssh_compatibility;
                        let recording_session_shortcut = shell.recording_session_shortcut;
                        let session_shortcut = shell.session_shortcut.clone();
                        let session_shortcut_error = shell.session_shortcut_error.clone();
                        let session_import_error = shell.session_import_error.clone();
                        let x11_server_missing = session_x11_forwarding
                            && !crate::platform::x_server::local_x_server_available(
                                shell.config.local_x_server_app_path(),
                            );
                        let saved_group_names = shell.saved_group_names();
                        let current_group_name =
                            session_group_input.read(cx).value().trim().to_string();
                        let available_serial_ports = shell.available_serial_ports.clone();

                        content.child(
                            v_flex()
                                .track_focus(&view.read(cx).focus_handle)
                                .on_key_down(window.listener_for(
                                    &view,
                                    |this, event, window, cx| {
                                        this.record_session_shortcut(event, window, cx);
                                    },
                                ))
                                .gap_3()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .p_3()
                                        .border_1()
                                        .border_color(cx.theme().border)
                                        .rounded_md()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .child(t!("connection_type").to_string()),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .child(
                                                    Button::new("session-kind-ssh")
                                                        .label("SSH")
                                                        .when(is_ssh, |button| button.primary())
                                                        .on_click(window.listener_for(
                                                            &view,
                                                            |this, _, window, cx| {
                                                                this.set_session_kind(
                                                                    SessionKind::Ssh,
                                                                    window,
                                                                    cx,
                                                                )
                                                            },
                                                        )),
                                                )
                                                .child(
                                                    Button::new("session-kind-serial")
                                                        .label(t!("serial_connection").to_string())
                                                        .when(is_serial, |button| button.primary())
                                                        .on_click(window.listener_for(
                                                            &view,
                                                            |this, _, window, cx| {
                                                                this.set_session_kind(
                                                                    SessionKind::Serial,
                                                                    window,
                                                                    cx,
                                                                )
                                                            },
                                                        )),
                                                )
                                                .child(
                                                    Button::new("session-kind-telnet")
                                                        .label("Telnet")
                                                        .when(is_telnet, |button| button.primary())
                                                        .on_click(window.listener_for(
                                                            &view,
                                                            |this, _, window, cx| {
                                                                this.set_session_kind(
                                                                    SessionKind::Telnet,
                                                                    window,
                                                                    cx,
                                                                )
                                                            },
                                                        )),
                                                ),
                                        ),
                                )
                                .when(is_serial, |this| {
                                    this.child(
                                        v_flex()
                                            .gap_2()
                                            .p_3()
                                            .border_1()
                                            .border_color(cx.theme().border)
                                            .rounded_md()
                                            .child(
                                                h_flex()
                                                    .justify_between()
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::BOLD)
                                                            .child(t!("serial_connection").to_string()),
                                                    )
                                                    .child(
                                                        Button::new("refresh-serial-ports")
                                                            .ghost()
                                                            .label(t!("refresh").to_string())
                                                            .on_click(window.listener_for(
                                                                &view,
                                                                |this, _, _, cx| {
                                                                    this.refresh_available_serial_ports(cx)
                                                                },
                                                            )),
                                                    ),
                                            )
                                            .child(div().text_sm().child(t!("serial_port").to_string()))
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .child(Input::new(&serial_port_input).flex_1().tab_index(0))
                                                    .child(
                                                        settings::fast_menu::fast_settings_menu_lazy_disabled(
                                                            "serial-port-dropdown",
                                                            t!("detected_serial_ports").to_string(),
                                                            Some(IconName::ChevronsUpDown),
                                                            px(220.),
                                                            Some(px(260.)),
                                                            available_serial_ports.is_empty(),
                                                            {
                                                                let ports = available_serial_ports.clone();
                                                                let selected = serial_port_input.read(cx).value().to_string();
                                                                move |_, _| ports
                                                                    .iter()
                                                                    .cloned()
                                                                    .map(|port| {
                                                                        let checked = port == selected;
                                                                        settings::fast_menu::FastMenuItem::new(
                                                                            port.clone(),
                                                                            checked,
                                                                            move |this, window, cx| {
                                                                                Self::set_input_value(
                                                                                    &this.serial_port_input,
                                                                                    port.clone(),
                                                                                    window,
                                                                                    cx,
                                                                                );
                                                                            },
                                                                        )
                                                                    })
                                                                    .collect::<Vec<_>>()
                                                            },
                                                            view.clone(),
                                                        ),
                                                    ),
                                            )
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .child(Input::new(&serial_baud_rate_input).flex_1().tab_index(1))
                                                    .child(Input::new(&serial_data_bits_input).w(px(64.)).tab_index(2))
                                                    .child(Input::new(&serial_parity_input).w(px(78.)).tab_index(3))
                                                    .child(Input::new(&serial_stop_bits_input).w(px(64.)).tab_index(4))
                                                    .child(Input::new(&serial_flow_control_input).flex_1().tab_index(5)),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child(t!("serial_settings_hint").to_string()),
                                            ),
                                    )
                                })
                                .when(!is_serial, |this| this.child(
                                    v_flex()
                                        .gap_2()
                                        .p_3()
                                        .border_1()
                                        .border_color(cx.theme().border)
                                        .rounded_md()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .child(if is_telnet {
                                                    t!("telnet_connection").to_string()
                                                } else {
                                                    t!("ssh_connection").to_string()
                                                }),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .text_sm()
                                                        .child(t!("host").to_string()),
                                                )
                                                .child(
                                                    div()
                                                        .w(px(108.))
                                                        .text_sm()
                                                        .child(t!("port").to_string()),
                                                ),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .child(
                                                    Input::new(&host_input)
                                                        .flex_1()
                                                        .tab_index(0),
                                                )
                                                .child(
                                                    Input::new(&port_input)
                                                        .w(px(108.))
                                                        .tab_index(1),
                                                ),
                                        )
                                        .when(is_ssh, |this| {
                                            this.child(
                                                div()
                                                    .text_sm()
                                                    .child(t!("user").to_string()),
                                            )
                                            .child(Input::new(&user_input).w_full().tab_index(2))
                                        })
                                ))
                                .when(is_ssh, |this| this.child(
                                    v_flex()
                                        .gap_2()
                                        .p_3()
                                        .border_1()
                                        .border_color(cx.theme().border)
                                        .rounded_md()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .child(t!("ssh_authentication").to_string()),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .child(
                                                    Button::new("ssh-auth-password")
                                                        .label(t!("password").to_string())
                                                        .when(is_password, |button| {
                                                            button.primary()
                                                        })
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
                                                        .label(t!("private_key").to_string())
                                                        .when(!is_password, |button| {
                                                            button.primary()
                                                        })
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
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .child(t!("password").to_string()),
                                                    )
                                                    .child(
                                                        Input::new(&password_input)
                                                            .mask_toggle()
                                                            .tab_index(3),
                                                    ),
                                            )
                                        })
                                        .when(!is_password, |this| {
                                            this.child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .child(
                                                                t!("private_key_path").to_string(),
                                                            ),
                                                    )
                                                    .child(
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
                                                                                this.pick_ssh_key_path(
                                                                                    window, cx,
                                                                                );
                                                                            },
                                                                        ),
                                                                    )
                                                                    .child(
                                                                        Input::new(&key_path_input)
                                                                            .tab_index(3),
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
                                                    ),
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .child(
                                                                t!("private_key_data").to_string(),
                                                            ),
                                                    )
                                                    .child(
                                                        Input::new(&key_inline_input)
                                                            .h(px(128.))
                                                            .tab_index(4),
                                                    ),
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .child(
                                                                t!("private_key_passphrase_optional")
                                                                    .to_string(),
                                                            ),
                                                    )
                                                    .child(
                                                        Input::new(&passphrase_input)
                                                            .mask_toggle()
                                                            .tab_index(5),
                                                    ),
                                            )
                                        }),
                                ))
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .p_3()
                                        .border_1()
                                        .border_color(cx.theme().border)
                                        .rounded_md()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .child(t!("ssh_organization_optional").to_string()),
                                        )
                                        .child(
                                            v_flex()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .child(
                                                            t!("connection_name_optional").to_string(),
                                                        ),
                                                )
                                                .child(
                                                    Input::new(&session_name_input).tab_index(6),
                                                ),
                                        )
                                        .child(
                                            v_flex()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .child(
                                                            t!("save_to_group_optional").to_string(),
                                                        ),
                                                )
                                                .child(
                                                    h_flex()
                                                        .gap_2()
                                                        .child(
                                                            Input::new(&session_group_input)
                                                                .flex_1()
                                                                .tab_index(7),
                                                        )
                                                        .child(
                                                            settings::fast_menu::fast_settings_menu_lazy_disabled(
                                                                "ssh-group-dropdown",
                                                                t!("choose_saved_group").to_string(),
                                                                Some(IconName::ChevronsUpDown),
                                                                px(192.),
                                                                Some(px(320.)),
                                                                saved_group_names.is_empty(),
                                                                {
                                                                    let saved_group_names =
                                                                        saved_group_names.clone();
                                                                    let current_group_name =
                                                                        current_group_name.clone();
                                                                    move |_, _| {
                                                                        let mut items = vec![
                                                                            settings::fast_menu::FastMenuItem::new(
                                                                                t!("ungrouped_group").to_string(),
                                                                                current_group_name.is_empty(),
                                                                                |this, window, cx| {
                                                                                    Self::set_input_value(
                                                                                        &this.session_group_input,
                                                                                        "",
                                                                                        window,
                                                                                        cx,
                                                                                    );
                                                                                },
                                                                            ),
                                                                        ];
                                                                        for group_name in &saved_group_names {
                                                                            let checked =
                                                                                current_group_name == *group_name;
                                                                            let group_name = group_name.clone();
                                                                            items.push(
                                                                                settings::fast_menu::FastMenuItem::new(
                                                                                    group_name.clone(),
                                                                                    checked,
                                                                                    move |this, window, cx| {
                                                                                        Self::set_input_value(
                                                                                            &this.session_group_input,
                                                                                            group_name.clone(),
                                                                                            window,
                                                                                            cx,
                                                                                        );
                                                                                    },
                                                                                ),
                                                                            );
                                                                        }
                                                                        items
                                                                    }
                                                                },
                                                                view.clone(),
                                                            ),
                                                        ),
                                                ),
                                        ),
                                )
                                .when(is_ssh, |this| this.child(
                                    Button::new("ssh-advanced-options")
                                        .ghost()
                                        .label(
                                            if show_advanced_options {
                                                t!("hide_advanced_ssh_options").to_string()
                                            } else {
                                                t!("show_advanced_ssh_options").to_string()
                                            },
                                        )
                                        .on_click(window.listener_for(
                                            &view,
                                            |this, _, _, cx| {
                                                this.ssh_advanced_options_visible =
                                                    !this.ssh_advanced_options_visible;
                                                cx.notify();
                                            },
                                        )),
                                ))
                                .when(is_ssh && show_advanced_options, |this| {
                                    this.child(
                                        v_flex()
                                            .gap_3()
                                            .p_3()
                                            .border_1()
                                            .border_color(cx.theme().border)
                                            .rounded_md()
                                            .child(
                                                v_flex()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::BOLD)
                                                            .child(t!("ssh_network").to_string()),
                                                    )
                                                    .child(
                                                        h_flex()
                                                            .gap_2()
                                                            .child(
                                                                Button::new("proxy-none")
                                                                    .label(
                                                                        t!("proxy_none").to_string(),
                                                                    )
                                                                    .when(
                                                                        proxy_type == "none",
                                                                        |button| button.primary(),
                                                                    )
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
                                                                    .when(
                                                                        proxy_type == "socks5",
                                                                        |button| button.primary(),
                                                                    )
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
                                                                    .when(
                                                                        proxy_type == "http",
                                                                        |button| button.primary(),
                                                                    )
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
                                                                .child(
                                                                    Input::new(&proxy_host_input)
                                                                        .flex_1(),
                                                                )
                                                                .child(
                                                                    Input::new(&proxy_port_input)
                                                                        .w(px(108.)),
                                                                ),
                                                        )
                                                        .child(
                                                            h_flex()
                                                                .gap_2()
                                                                .child(
                                                                    Input::new(&proxy_user_input)
                                                                        .flex_1(),
                                                                )
                                                                .child(
                                                                    Input::new(&proxy_password_input)
                                                                        .flex_1(),
                                                                ),
                                                        )
                                                    }),
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::BOLD)
                                                            .child(t!("ssh_file_access").to_string()),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .child(t!("sftp_initial_path_optional").to_string()),
                                                    )
                                                    .child(
                                                        Input::new(&session_sftp_path_input)
                                                            .tab_index(8),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(cx.theme().muted_foreground)
                                                            .child(
                                                                t!("sftp_initial_path_hint").to_string(),
                                                            ),
                                                    ),
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_2()
                                                    .child(
                                                        Checkbox::new(
                                                            "ssh-session-x11-forwarding",
                                                        )
                                                        .checked(session_x11_forwarding)
                                                        .label(t!("x11_forwarding").to_string())
                                                        .on_click(window.listener_for(
                                                            &view,
                                                            |this, checked, _, cx| {
                                                                this.session_x11_forwarding = *checked;
                                                                cx.notify();
                                                            },
                                                        )),
                                                    )
                                                    .child(
                                                        v_flex()
                                                            .gap_1()
                                                            .child(
                                                                Checkbox::new(
                                                                    "ssh-session-legacy-compatibility",
                                                                )
                                                                .checked(
                                                                    session_legacy_ssh_compatibility,
                                                                )
                                                                .label(
                                                                    t!(
                                                                        "legacy_ssh_compatibility"
                                                                    )
                                                                    .to_string(),
                                                                )
                                                                .on_click(window.listener_for(
                                                                    &view,
                                                                    |this, checked, _, cx| {
                                                                        this.session_legacy_ssh_compatibility =
                                                                            *checked;
                                                                        cx.notify();
                                                                    },
                                                                )),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(
                                                                        cx.theme().muted_foreground,
                                                                    )
                                                                    .child(
                                                                        t!(
                                                                            "legacy_ssh_compatibility_hint"
                                                                        )
                                                                        .to_string(),
                                                                    ),
                                                            ),
                                                    )
                                                    .when(x11_server_missing, |this| {
                                                        this.child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(cx.theme().muted_foreground)
                                                                .child(
                                                                    t!("x11_server_install_hint")
                                                                        .to_string(),
                                                                ),
                                                        )
                                                    })
                                                    .child(
                                                        h_flex()
                                                            .justify_between()
                                                            .items_center()
                                                            .child(
                                                                div()
                                                                    .text_sm()
                                                                    .child(
                                                                        t!("session_shortcut").to_string(),
                                                                    ),
                                                            )
                                                            .child(
                                                                h_flex()
                                                                    .gap_2()
                                                                    .child(
                                                                        Button::new(
                                                                            "record-session-shortcut",
                                                                        )
                                                                        .label(if recording_session_shortcut {
                                                                            t!("press_new_key").to_string()
                                                                        } else if session_shortcut.is_empty() {
                                                                            t!("none").to_string()
                                                                        } else {
                                                                            crate::app::keybinding_recorder::format_keystroke(
                                                                                &session_shortcut,
                                                                            )
                                                                        })
                                                                        .small()
                                                                        .when(
                                                                            recording_session_shortcut,
                                                                            |button| button.primary(),
                                                                        )
                                                                        .when(
                                                                            session_shortcut_error.is_some(),
                                                                            |button| button.danger(),
                                                                        )
                                                                        .on_click(window.listener_for(
                                                                            &view,
                                                                            |this, _, window, cx| {
                                                                                this.recording_session_shortcut = true;
                                                                                this.session_shortcut_error = None;
                                                                                window.focus(&this.focus_handle, cx);
                                                                                cx.notify();
                                                                            },
                                                                        )),
                                                                    )
                                                                    .child(
                                                                        Button::new(
                                                                            "clear-session-shortcut",
                                                                        )
                                                                        .ghost()
                                                                        .icon(IconName::Close)
                                                                        .when(
                                                                            session_shortcut.is_empty(),
                                                                            |button| button.disabled(true),
                                                                        )
                                                                        .on_click(window.listener_for(
                                                                            &view,
                                                                            |this, _, _, cx| {
                                                                                this.recording_session_shortcut = false;
                                                                                this.session_shortcut_error = None;
                                                                                this.session_shortcut.clear();
                                                                                cx.notify();
                                                                            },
                                                                        )),
                                                                    ),
                                                            ),
                                                    ),
                                            )
                                            .when_some(session_shortcut_error, |this, error| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(cx.theme().danger)
                                                        .child(error),
                                                )
                                            }),
                                    )
                                })
                                .when_some(session_import_error, |this, error| {
                                    this.child(
                                        div()
                                            .text_xs()
                                            .text_color(cx.theme().danger)
                                            .child(error),
                                    )
                                })
                                .child(
                                    h_flex()
                                        .justify_between()
                                        .gap_2()
                                        .child(
                                            Button::new("import-ssh-session-clipboard")
                                                .ghost()
                                                .label(t!("import_from_clipboard").to_string())
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, window, cx| {
                                                        this.import_ssh_session_from_clipboard(
                                                            window, cx,
                                                        );
                                                    },
                                                )),
                                        )
                                        .child(
                                            h_flex()
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
                                                    Button::new("save-ssh-session")
                                                        .label(t!("save").to_string())
                                                        .on_click(window.listener_for(
                                                            &view,
                                                            |this, _, window, cx| {
                                                                this.save_session(window, cx)
                                                            },
                                                        )),
                                                )
                                                .child(
                                                    Button::new("save-and-connect-ssh-session")
                                                        .primary()
                                                        .label(t!("save_and_connect").to_string())
                                                        .on_click(window.listener_for(
                                                            &view,
                                                            |this, _, window, cx| {
                                                                this.connect_session(window, cx)
                                                            },
                                                        )),
                                                ),
                                        ),
                                ),
                        )
                    }
                })
        });
        window.defer(cx, move |window, cx| {
            let input = if deferred_view.read(cx).session_kind == SessionKind::Serial {
                &focus_serial_port_input
            } else {
                &focus_host_input
            };
            window.focus(&input.read(cx).focus_handle(cx), cx);
        });
    }
}
