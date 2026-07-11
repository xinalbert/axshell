use super::*;

impl AxShell {
    pub(super) fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let session_groups = self.saved_session_groups();
        let active_session_id = self.active_session_id().map(ToOwned::to_owned);
        let renaming_saved_group = self.renaming_saved_group.clone();
        let is_local_active = self.active_kind() == Some(TabKind::Local);

        v_flex()
            .gap_4()
            .w_full()
            .h_full()
            .min_w(px(0.))
            .p_4()
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .bg(cx.theme().sidebar)
            .overflow_hidden()
            .child(
                v_flex()
                    .gap_1()
                    .min_w(px(0.))
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .font_weight(FontWeight::BOLD)
                                    .text_size(rems(1.667))
                                    .text_color(cx.theme().primary)
                                    .child("AxShell"),
                            )
                            .child(div().flex_1())
                            .child(
                                Button::new("sidebar-collapse-toggle")
                                    .ghost()
                                    .icon(IconName::PanelLeftClose)
                                    .tooltip(t!("settings_toggle_sidebar").to_string())
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.sidebar_collapsed = true;
                                        this.config.set_sidebar_collapsed(true);
                                        this.config.save_logged("collapse_sidebar");
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("sidebar-settings")
                                    .ghost()
                                    .icon(IconName::Settings)
                                    .tooltip(t!("settings_open_settings").to_string())
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.open_settings_page(cx)),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .text_size(rems(0.917))
                            .text_color(cx.theme().muted_foreground)
                            .child({
                                if self.workspace_page == WorkspacePage::Settings {
                                    t!("settings").to_string()
                                } else if let Some(kind) = self.active_kind() {
                                    match kind {
                                        TabKind::Local => t!("local_terminal").to_string(),
                                        TabKind::Ssh => {
                                            if let Some((_, session)) = self.active_ssh_session() {
                                                format!("ssh / {}", session.name)
                                            } else {
                                                "ssh".to_string()
                                            }
                                        }
                                    }
                                } else {
                                    self.active_title()
                                }
                            }),
                    ),
            )
            .when(
                self.config.show_monitoring_dashboard()
                    && self.config.monitoring_position() == "Sidebar",
                |this| this.child(self.render_sidebar_monitoring_panel(cx)),
            )
            .child(
                Button::new("open-ssh-panel")
                    .primary()
                    .label(t!("add_ssh").to_string())
                    .on_click(
                        cx.listener(|this, _, window, cx| this.open_new_ssh_dialog(window, cx)),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h(px(0.))
                    .gap_2()
                    .child(
                        div()
                            .text_size(rems(1.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(cx.theme().primary)
                            .child(t!("saved")),
                    )
                    .child(
                        div()
                            .relative()
                            .flex_1()
                            .min_h(px(0.))
                            .size_full()
                            .child(
                                v_flex()
                                    .size_full()
                                    .id("saved-sessions-scroll")
                                    .track_scroll(&self.saved_scroll_handle)
                                    .overflow_y_scroll()
                                    .gap_2()
                                    .child(self.render_saved_local_terminal_entry(
                                        "saved-local-terminal",
                                        is_local_active,
                                        cx,
                                    ))
                                    .children(session_groups.into_iter().enumerate().map(
                                        |(group_ix, (group_name, sessions))| {
                                            let display_group_name =
                                                Self::display_group_name(&group_name);
                                            let is_renaming = renaming_saved_group.as_deref()
                                                == Some(group_name.as_str());
                                            let is_expanded = is_renaming
                                                || self
                                                    .expanded_saved_groups
                                                    .contains(group_name.as_str());
                                            let is_group_active = sessions.iter().any(|session| {
                                                active_session_id.as_deref()
                                                    == Some(session.id.as_str())
                                            });
                                            let group_key = group_name.clone();
                                            let rename_group_name = group_name.clone();
                                            let group_count = sessions.len();
                                            v_flex()
                                                .w_full()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .id(format!("saved-group-{group_ix}"))
                                                        .w_full()
                                                        .p_2()
                                                        .rounded_md()
                                                        .border_1()
                                                        .border_color(if is_group_active {
                                                            cx.theme().primary
                                                        } else {
                                                            cx.theme().border
                                                        })
                                                        .bg(if is_group_active {
                                                            cx.theme().tab_active
                                                        } else {
                                                            cx.theme().muted
                                                        })
                                                        .when(!is_renaming, |this| {
                                                            this.cursor_pointer()
                                                                .hover(|this| {
                                                                    this.bg(cx.theme().secondary)
                                                                })
                                                                .on_mouse_down(
                                                                    MouseButton::Left,
                                                                    cx.listener(
                                                                        move |this, _, _, cx| {
                                                                            this.toggle_saved_group(
                                                                                group_key.clone(),
                                                                                cx,
                                                                            )
                                                                        },
                                                                    ),
                                                                )
                                                        })
                                                        .child(
                                                            h_flex()
                                                                .w_full()
                                                                .items_center()
                                                                .gap_2()
                                                                .child(
                                                                    Icon::new(if is_expanded {
                                                                        IconName::ChevronUp
                                                                    } else {
                                                                        IconName::ChevronDown
                                                                    })
                                                                    .with_size(Size::Small)
                                                                    .text_color(
                                                                        cx.theme().muted_foreground,
                                                                    ),
                                                                )
                                                                .child(
                                                                    if is_renaming {
                                                                        Input::new(
                                                                            &self.saved_group_name_input,
                                                                        )
                                                                        .flex_1()
                                                                        .into_any_element()
                                                                    } else {
                                                                        div()
                                                                            .flex_1()
                                                                            .min_w(px(0.))
                                                                            .overflow_hidden()
                                                                            .text_ellipsis()
                                                                            .whitespace_nowrap()
                                                                            .text_size(rems(1.0))
                                                                            .font_weight(
                                                                                FontWeight::SEMIBOLD,
                                                                            )
                                                                            .child(
                                                                                display_group_name,
                                                                            )
                                                                            .into_any_element()
                                                                    },
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(rems(0.833))
                                                                        .text_color(
                                                                            cx.theme()
                                                                                .muted_foreground,
                                                                        )
                                                                        .child(format!(
                                                                            "{}",
                                                                            group_count
                                                                        )),
                                                                )
                                                                .when(
                                                                    !group_name.is_empty()
                                                                        && !is_renaming,
                                                                    |this| {
                                                                        this.child(
                                                                            Button::new(format!(
                                                                                "saved-group-rename-{group_ix}"
                                                                            ))
                                                                            .ghost()
                                                                            .xsmall()
                                                                            .label(
                                                                                t!("rename")
                                                                                    .to_string(),
                                                                            )
                                                                            .on_mouse_down(
                                                                                MouseButton::Left,
                                                                                |_, window, cx| {
                                                                                    window
                                                                                        .prevent_default();
                                                                                    cx.stop_propagation();
                                                                                },
                                                                            )
                                                                            .on_click(
                                                                                cx.listener(
                                                                                    move |this, _, window, cx| {
                                                                                        window
                                                                                            .prevent_default();
                                                                                        cx.stop_propagation();
                                                                                        this.begin_saved_group_rename(
                                                                                            rename_group_name
                                                                                                .clone(),
                                                                                            window,
                                                                                            cx,
                                                                                        );
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        )
                                                                    },
                                                                ),
                                                        ),
                                                )
                                                .when(is_expanded, |this| {
                                                    this.child(
                                                        v_flex()
                                                            .w_full()
                                                            .pl_4()
                                                            .gap_2()
                                                            .children(sessions.into_iter().enumerate().map(
                                                                |(session_ix, session)| {
                                                                    let connect_id =
                                                                        session.id.clone();
                                                                    let edit_id =
                                                                        session.id.clone();
                                                                    let delete_id =
                                                                        session.id.clone();
                                                                    let is_active = active_session_id
                                                                        .as_deref()
                                                                        == Some(session.id.as_str());
                                                                    let name = session.name.clone();
                                                                    let detail =
                                                                        self.session_detail(&session);
                                                                    let full_detail = self
                                                                        .session_connection_info(
                                                                            &session,
                                                                        );
                                                                    let tooltip_detail =
                                                                        full_detail.clone();
                                                                    let menu_detail =
                                                                        full_detail.clone();
                                                                    div()
                                                                        .id(format!(
                                                                            "saved-connect-{group_ix}-{session_ix}"
                                                                        ))
                                                                        .w_full()
                                                                        .p_2()
                                                                        .rounded_md()
                                                                        .border_1()
                                                                        .border_color(if is_active {
                                                                            cx.theme().primary
                                                                        } else {
                                                                            cx.theme().border
                                                                        })
                                                                        .bg(if is_active {
                                                                            cx.theme().tab_active
                                                                        } else {
                                                                            cx.theme().muted
                                                                        })
                                                                        .cursor_pointer()
                                                                        .hover(|this| {
                                                                            this.bg(cx.theme().secondary)
                                                                        })
                                                                        .on_mouse_down(
                                                                            MouseButton::Left,
                                                                            cx.listener(
                                                                                move |this, _, _, cx| {
                                                                                    this.connect_saved_session(
                                                                                        connect_id
                                                                                            .clone(),
                                                                                        cx,
                                                                                    )
                                                                                },
                                                                            ),
                                                                        )
                                                                        .tooltip({
                                                                            let tooltip_text =
                                                                                tooltip_detail.clone();
                                                                            move |window, cx| {
                                                                                gpui_component::tooltip::Tooltip::new(
                                                                                    tooltip_text
                                                                                        .clone(),
                                                                                )
                                                                                .build(window, cx)
                                                                            }
                                                                        })
                                                                        .context_menu({
                                                                            let view = cx.entity();
                                                                            move |menu, window, _| {
                                                                                let copy_value =
                                                                                    menu_detail.clone();
                                                                                let edit_value =
                                                                                    edit_id.clone();
                                                                                let clone_value =
                                                                                    edit_id.clone();
                                                                                let delete_value =
                                                                                    delete_id.clone();
                                                                                menu.item(
                                                                                    PopupMenuItem::new(
                                                                                        t!("copy_connection_info")
                                                                                            .to_string(),
                                                                                    )
                                                                                    .on_click(window.listener_for(
                                                                                        &view,
                                                                                        move |_, _, _, cx| {
                                                                                            cx.write_to_clipboard(
                                                                                                gpui::ClipboardItem::new_string(
                                                                                                    copy_value.clone(),
                                                                                                ),
                                                                                            );
                                                                                        },
                                                                                    )),
                                                                                )
                                                                                .item(
                                                                                    PopupMenuItem::new(
                                                                                        t!("clone")
                                                                                            .to_string(),
                                                                                    )
                                                                                    .on_click(window.listener_for(
                                                                                        &view,
                                                                                        move |this, _, window, cx| {
                                                                                            this.clone_saved_session(
                                                                                                clone_value.clone(),
                                                                                                window,
                                                                                                cx,
                                                                                            )
                                                                                        },
                                                                                    )),
                                                                                )
                                                                                .item(
                                                                                    PopupMenuItem::new(
                                                                                        t!("edit")
                                                                                            .to_string(),
                                                                                    )
                                                                                    .on_click(window.listener_for(
                                                                                        &view,
                                                                                        move |this, _, window, cx| {
                                                                                            this.edit_saved_session(
                                                                                                edit_value.clone(),
                                                                                                window,
                                                                                                cx,
                                                                                            )
                                                                                        },
                                                                                    )),
                                                                                )
                                                                                .item(
                                                                                    PopupMenuItem::new(
                                                                                        t!("delete")
                                                                                            .to_string(),
                                                                                    )
                                                                                    .on_click(window.listener_for(
                                                                                        &view,
                                                                                        move |this, _, _, cx| {
                                                                                            this.remove_saved_session(
                                                                                                delete_value.clone(),
                                                                                                cx,
                                                                                            )
                                                                                        },
                                                                                    )),
                                                                                )
                                                                            }
                                                                        })
                                                                        .child(
                                                                            h_flex()
                                                                                .w_full()
                                                                                .min_w(px(0.))
                                                                                .gap_2()
                                                                                .items_center()
                                                                                .child(
                                                                                    div()
                                                                                        .max_w(px(180.))
                                                                                        .min_w(px(0.))
                                                                                        .overflow_hidden()
                                                                                        .text_ellipsis()
                                                                                        .whitespace_nowrap()
                                                                                        .text_size(rems(1.0))
                                                                                        .font_weight(FontWeight::SEMIBOLD)
                                                                                        .child(name),
                                                                                )
                                                                                .child(
                                                                                    div()
                                                                                        .flex_1()
                                                                                        .min_w(px(0.))
                                                                                        .overflow_hidden()
                                                                                        .text_ellipsis()
                                                                                        .whitespace_nowrap()
                                                                                        .text_size(rems(0.917))
                                                                                        .text_color(
                                                                                            cx.theme()
                                                                                                .muted_foreground,
                                                                                        )
                                                                                        .child(detail),
                                                                                ),
                                                                        )
                                                                },
                                                            )),
                                                    )
                                                })
                                        },
                                    )),
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
                                            &self.saved_scroll_handle,
                                        )
                                        .id("saved-scrollbar")
                                        .axis(gpui_component::scroll::ScrollbarAxis::Vertical)
                                        .scrollbar_show(
                                            gpui_component::scroll::ScrollbarShow::Always,
                                        ),
                                    ),
                            ),
                    ),
            )
    }

    fn render_saved_local_terminal_entry(
        &self,
        id: &'static str,
        is_active: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(id)
            .w_full()
            .p_2()
            .rounded_md()
            .border_1()
            .border_color(if is_active {
                cx.theme().primary
            } else {
                cx.theme().border
            })
            .bg(if is_active {
                cx.theme().tab_active
            } else {
                cx.theme().muted
            })
            .cursor_pointer()
            .hover(|this| this.bg(cx.theme().secondary))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.open_local(cx)),
            )
            .tooltip(|window, cx| {
                gpui_component::tooltip::Tooltip::new(t!("open_local_shell_tab").to_string())
                    .build(window, cx)
            })
            .child(
                h_flex()
                    .w_full()
                    .min_w(px(0.))
                    .items_center()
                    .gap_2()
                    .child(
                        Icon::new(IconName::SquareTerminal)
                            .with_size(Size::Small)
                            .text_color(if is_active {
                                cx.theme().primary
                            } else {
                                cx.theme().muted_foreground
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(0.))
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .text_size(rems(1.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(t!("local_terminal").to_string()),
                    )
                    .child(
                        div()
                            .max_w(px(180.))
                            .min_w(px(0.))
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .text_size(rems(0.833))
                            .text_color(cx.theme().muted_foreground)
                            .child(t!("open_local_shell_tab").to_string()),
                    ),
            )
    }

    fn render_collapsed_saved_local_terminal_entry(
        &self,
        is_active: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("collapsed-saved-local-terminal")
            .w(px(36.))
            .h(px(36.))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .border_1()
            .border_color(if is_active {
                cx.theme().primary
            } else {
                cx.theme().border
            })
            .bg(if is_active {
                cx.theme().tab_active
            } else {
                cx.theme().muted
            })
            .cursor_pointer()
            .hover(|this| this.bg(cx.theme().secondary))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.open_local(cx)),
            )
            .tooltip(|window, cx| {
                gpui_component::tooltip::Tooltip::new(t!("open_local_shell_tab").to_string())
                    .build(window, cx)
            })
            .child(
                Icon::new(IconName::SquareTerminal)
                    .with_size(Size::Small)
                    .text_color(if is_active {
                        cx.theme().primary
                    } else {
                        cx.theme().foreground
                    }),
            )
    }

    pub(super) fn render_collapsed_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let session_groups = self.saved_session_groups();
        let active_session_id = self.active_session_id().map(ToOwned::to_owned);
        let is_local_active = self.active_kind() == Some(TabKind::Local);

        v_flex()
            .w_full()
            .h_full()
            .min_w(px(0.))
            .p_2()
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .bg(cx.theme().sidebar)
            .overflow_hidden()
            .items_center()
            // Top: expand button only
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .pb_2()
                    .child(
                        Button::new("sidebar-expand-toggle")
                            .ghost()
                            .icon(IconName::PanelLeftOpen)
                            .tooltip(t!("settings_toggle_sidebar").to_string())
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.sidebar_collapsed = false;
                                this.config.set_sidebar_collapsed(false);
                                this.config.save_logged("expand_sidebar");
                                cx.notify();
                            })),
                    ),
            )
            // Saved sessions as compact cards
            .child(
                div()
                    .relative()
                    .flex_1()
                    .min_h(px(0.))
                    .w_full()
                    .child(
                        v_flex()
                            .size_full()
                            .id("collapsed-saved-sessions-scroll")
                            .track_scroll(&self.collapsed_saved_scroll_handle)
                            .overflow_y_scroll()
                            .gap_2()
                            .items_center()
                            .child(self.render_collapsed_saved_local_terminal_entry(
                                is_local_active,
                                cx,
                            ))
                            .children(session_groups.into_iter().enumerate().map(
                                |(group_ix, (group_name, sessions))| {
                                    let display_group_name = Self::display_group_name(&group_name);
                                    let group_abbrev =
                                        Self::collapsed_sidebar_abbrev(&display_group_name);
                                    let group_tooltip =
                                        format!("{} ({})", display_group_name, sessions.len());
                                    let is_expanded =
                                        self.expanded_saved_groups.contains(group_name.as_str());
                                    let is_group_active = sessions.iter().any(|session| {
                                        active_session_id.as_deref()
                                            == Some(session.id.as_str())
                                    });
                                    let toggle_group_name = group_name.clone();

                                    v_flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .id(format!("collapsed-saved-group-{group_ix}"))
                                                .w(px(36.))
                                                .h(px(36.))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .rounded_md()
                                                .border_1()
                                                .border_color(if is_group_active || is_expanded {
                                                    cx.theme().primary
                                                } else {
                                                    cx.theme().border
                                                })
                                                .bg(if is_group_active {
                                                    cx.theme().tab_active
                                                } else if is_expanded {
                                                    cx.theme().secondary
                                                } else {
                                                    cx.theme().muted
                                                })
                                                .cursor_pointer()
                                                .hover(|this| this.bg(cx.theme().secondary))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _, _, cx| {
                                                        this.toggle_saved_group(
                                                            toggle_group_name.clone(),
                                                            cx,
                                                        )
                                                    }),
                                                )
                                                .tooltip({
                                                    let tooltip_text = group_tooltip.clone();
                                                    move |window, cx| {
                                                        gpui_component::tooltip::Tooltip::new(
                                                            tooltip_text.clone(),
                                                        )
                                                        .build(window, cx)
                                                    }
                                                })
                                                .child(
                                                    v_flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .gap(px(1.))
                                                        .child(
                                                            Icon::new(IconName::Folder)
                                                                .with_size(Size::Small)
                                                                .text_color(if is_group_active
                                                                    || is_expanded
                                                                {
                                                                    cx.theme().primary
                                                                } else {
                                                                    cx.theme().muted_foreground
                                                                }),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(rems(0.625))
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(if is_group_active
                                                                    || is_expanded
                                                                {
                                                                    cx.theme().primary
                                                                } else {
                                                                    cx.theme().foreground
                                                                })
                                                                .child(group_abbrev),
                                                        ),
                                                ),
                                        )
                                        .when(is_expanded, |this| {
                                            this.child(
                                                v_flex().items_center().gap_1().children(
                                                    sessions.into_iter().enumerate().map(
                                                        |(session_ix, session)| {
                                                            let connect_id = session.id.clone();
                                                            let edit_id = session.id.clone();
                                                            let delete_id = session.id.clone();
                                                            let is_active = active_session_id
                                                                .as_deref()
                                                                == Some(session.id.as_str());
                                                            let abbrev = Self::collapsed_sidebar_abbrev(
                                                                &session.name,
                                                            );
                                                            let full_detail = self
                                                                .session_connection_info(&session);
                                                            let tooltip_detail =
                                                                full_detail.clone();
                                                            let menu_detail = full_detail.clone();

                                                            div()
                                                                .id(format!(
                                                                    "collapsed-saved-group-{group_ix}-session-{session_ix}"
                                                                ))
                                                                .w(px(28.))
                                                                .h(px(28.))
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .rounded_md()
                                                                .border_1()
                                                                .border_color(if is_active {
                                                                    cx.theme().primary
                                                                } else {
                                                                    cx.theme().border
                                                                })
                                                                .bg(if is_active {
                                                                    cx.theme().tab_active
                                                                } else {
                                                                    cx.theme().background
                                                                })
                                                                .cursor_pointer()
                                                                .hover(|this| {
                                                                    this.bg(cx.theme().secondary)
                                                                })
                                                                .on_mouse_down(
                                                                    MouseButton::Left,
                                                                    cx.listener(
                                                                        move |this, _, _, cx| {
                                                                            this.connect_saved_session(
                                                                                connect_id.clone(),
                                                                                cx,
                                                                            )
                                                                        },
                                                                    ),
                                                                )
                                                                .tooltip({
                                                                    let tooltip_text =
                                                                        tooltip_detail.clone();
                                                                    move |window, cx| {
                                                                        gpui_component::tooltip::Tooltip::new(
                                                                            tooltip_text.clone(),
                                                                        )
                                                                        .build(window, cx)
                                                                    }
                                                                })
                                                                .context_menu({
                                                                    let view = cx.entity();
                                                                    move |menu, window, _| {
                                                                        let copy_value =
                                                                            menu_detail.clone();
                                                                        let edit_value =
                                                                            edit_id.clone();
                                                                        let clone_value =
                                                                            edit_id.clone();
                                                                        let delete_value =
                                                                            delete_id.clone();
                                                                        menu.item(
                                                                            PopupMenuItem::new(
                                                                                t!("copy_connection_info")
                                                                                    .to_string(),
                                                                            )
                                                                            .on_click(window.listener_for(
                                                                                &view,
                                                                                move |_, _, _, cx| {
                                                                                    cx.write_to_clipboard(
                                                                                        gpui::ClipboardItem::new_string(
                                                                                            copy_value.clone(),
                                                                                        ),
                                                                                    );
                                                                                },
                                                                            )),
                                                                        )
                                                                        .item(
                                                                            PopupMenuItem::new(
                                                                                t!("clone")
                                                                                    .to_string(),
                                                                            )
                                                                            .on_click(window.listener_for(
                                                                                &view,
                                                                                move |this, _, window, cx| {
                                                                                    this.clone_saved_session(
                                                                                        clone_value.clone(),
                                                                                        window,
                                                                                        cx,
                                                                                    )
                                                                                },
                                                                            )),
                                                                        )
                                                                        .item(
                                                                            PopupMenuItem::new(
                                                                                t!("edit")
                                                                                    .to_string(),
                                                                            )
                                                                            .on_click(window.listener_for(
                                                                                &view,
                                                                                move |this, _, window, cx| {
                                                                                    this.edit_saved_session(
                                                                                        edit_value.clone(),
                                                                                        window,
                                                                                        cx,
                                                                                    )
                                                                                },
                                                                            )),
                                                                        )
                                                                        .item(
                                                                            PopupMenuItem::new(
                                                                                t!("delete")
                                                                                    .to_string(),
                                                                            )
                                                                            .on_click(window.listener_for(
                                                                                &view,
                                                                                move |this, _, _, cx| {
                                                                                    this.remove_saved_session(
                                                                                        delete_value.clone(),
                                                                                        cx,
                                                                                    )
                                                                                },
                                                                            )),
                                                                        )
                                                                    }
                                                                })
                                                                .child(
                                                                    div()
                                                                        .text_size(rems(0.708))
                                                                        .font_weight(FontWeight::BOLD)
                                                                        .text_color(if is_active {
                                                                            cx.theme().primary
                                                                        } else {
                                                                            cx.theme().foreground
                                                                        })
                                                                        .child(abbrev),
                                                                )
                                                        },
                                                    ),
                                                ),
                                            )
                                        })
                                },
                            )),
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
                                    &self.collapsed_saved_scroll_handle,
                                )
                                .id("collapsed-saved-scrollbar")
                                .axis(gpui_component::scroll::ScrollbarAxis::Vertical)
                                .scrollbar_show(gpui_component::scroll::ScrollbarShow::Scrolling),
                            ),
                    ),
            )
    }
}
