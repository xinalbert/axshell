use super::*;

#[derive(Clone)]
enum SavedSidebarRow {
    Local {
        is_active: bool,
    },
    Group {
        group_ix: usize,
        group_name: String,
        display_name: String,
        count: usize,
        is_renaming: bool,
        is_expanded: bool,
        is_active: bool,
    },
    Session {
        group_ix: usize,
        session_ix: usize,
        session_id: String,
        name: String,
        detail: String,
        full_detail: String,
        is_active: bool,
    },
}

#[derive(Clone)]
enum CollapsedSavedSidebarRow {
    Local {
        is_active: bool,
    },
    Group {
        group_ix: usize,
        group_name: String,
        display_name: String,
        count: usize,
        is_expanded: bool,
        is_active: bool,
    },
    Session {
        group_ix: usize,
        session_ix: usize,
        session_id: String,
        name: String,
        full_detail: String,
        is_active: bool,
    },
}

impl AxShell {
    fn saved_sidebar_rows(&self) -> Vec<SavedSidebarRow> {
        let active_session_id = self.active_session_id().map(ToOwned::to_owned);
        let mut rows = vec![SavedSidebarRow::Local {
            is_active: self.active_kind() == Some(TabKind::Local),
        }];

        for (group_ix, (group_name, sessions)) in
            self.saved_session_groups().into_iter().enumerate()
        {
            let is_renaming = self.renaming_saved_group.as_deref() == Some(group_name.as_str());
            let is_expanded =
                is_renaming || self.expanded_saved_groups.contains(group_name.as_str());
            let is_group_active = sessions
                .iter()
                .any(|session| active_session_id.as_deref() == Some(session.id.as_str()));

            rows.push(SavedSidebarRow::Group {
                group_ix,
                display_name: Self::display_group_name(&group_name),
                count: sessions.len(),
                is_renaming,
                is_expanded,
                is_active: is_group_active,
                group_name: group_name.clone(),
            });

            if is_expanded {
                rows.extend(
                    sessions
                        .into_iter()
                        .enumerate()
                        .map(|(session_ix, session)| {
                            let full_detail = self.session_connection_info(&session);
                            SavedSidebarRow::Session {
                                group_ix,
                                session_ix,
                                session_id: session.id.clone(),
                                name: session.name.clone(),
                                detail: self.session_detail(&session),
                                full_detail,
                                is_active: active_session_id.as_deref()
                                    == Some(session.id.as_str()),
                            }
                        }),
                );
            }
        }

        rows
    }

    fn collapsed_saved_sidebar_rows(&self) -> Vec<CollapsedSavedSidebarRow> {
        let active_session_id = self.active_session_id().map(ToOwned::to_owned);
        let mut rows = vec![CollapsedSavedSidebarRow::Local {
            is_active: self.active_kind() == Some(TabKind::Local),
        }];

        for (group_ix, (group_name, sessions)) in
            self.saved_session_groups().into_iter().enumerate()
        {
            let is_expanded = self.expanded_saved_groups.contains(group_name.as_str());
            let is_group_active = sessions
                .iter()
                .any(|session| active_session_id.as_deref() == Some(session.id.as_str()));
            let display_name = Self::display_group_name(&group_name);

            rows.push(CollapsedSavedSidebarRow::Group {
                group_ix,
                group_name: group_name.clone(),
                display_name,
                count: sessions.len(),
                is_expanded,
                is_active: is_group_active,
            });

            if is_expanded {
                rows.extend(
                    sessions
                        .into_iter()
                        .enumerate()
                        .map(|(session_ix, session)| CollapsedSavedSidebarRow::Session {
                            group_ix,
                            session_ix,
                            session_id: session.id.clone(),
                            name: session.name.clone(),
                            full_detail: self.session_connection_info(&session),
                            is_active: active_session_id.as_deref() == Some(session.id.as_str()),
                        }),
                );
            }
        }

        rows
    }

    pub(super) fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let saved_rows = self.saved_sidebar_rows();
        let saved_row_count = saved_rows.len();
        let saved_scroll_handle = self.saved_scroll_handle.clone();
        let saved_group_name_input = self.saved_group_name_input.clone();
        let view = cx.entity();

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
                                uniform_list(
                                    "saved-sessions-fast-list",
                                    saved_row_count,
                                    move |range, list_window, cx| {
                                        range
                                            .into_iter()
                                            .filter_map(|row_ix| {
                                                let row = saved_rows.get(row_ix)?.clone();
                                                Some(Self::render_saved_sidebar_row(
                                                    row_ix,
                                                    row,
                                                    saved_group_name_input.clone(),
                                                    view.clone(),
                                                    list_window,
                                                    cx,
                                                ))
                                            })
                                            .collect::<Vec<_>>()
                                    },
                                )
                                .track_scroll(&saved_scroll_handle)
                                .w_full()
                                .h_full(),
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
                                            &saved_scroll_handle,
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

    fn render_saved_sidebar_row(
        row_ix: usize,
        row: SavedSidebarRow,
        saved_group_name_input: gpui::Entity<gpui_component::input::InputState>,
        view: gpui::Entity<AxShell>,
        window: &mut Window,
        cx: &mut gpui::App,
    ) -> AnyElement {
        match row {
            SavedSidebarRow::Local { is_active } => div()
                .id(("saved-sidebar-row", row_ix))
                .w_full()
                .h(px(50.))
                .p_1()
                .child(
                    div()
                        .id("saved-local-terminal")
                        .size_full()
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
                        .fast_hover(cx)
                        .on_mouse_down(
                            MouseButton::Left,
                            window.listener_for(&view, |this, _, window, cx| {
                                this.open_local_and_focus(window, cx)
                            }),
                        )
                        .tooltip(|window, cx| {
                            gpui_component::tooltip::Tooltip::new(
                                t!("open_local_shell_tab").to_string(),
                            )
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
                        ),
                )
                .into_any_element(),
            SavedSidebarRow::Group {
                group_ix,
                group_name,
                display_name,
                count,
                is_renaming,
                is_expanded,
                is_active,
            } => {
                let group_key = group_name.clone();
                let rename_group_name = group_name.clone();
                let menu_group_name = group_name.clone();
                div()
                    .id(("saved-sidebar-row", row_ix))
                    .w_full()
                    .h(px(50.))
                    .p_1()
                    .child(
                        div()
                            .id(format!("saved-group-{group_ix}"))
                            .size_full()
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
                            .when(!is_renaming, |this| {
                                this.cursor_pointer().fast_hover(cx).on_mouse_down(
                                    MouseButton::Left,
                                    window.listener_for(&view, move |this, _, _, cx| {
                                        this.toggle_saved_group(group_key.clone(), cx)
                                    }),
                                )
                            })
                            .when(!is_renaming, |this| {
                                this.on_mouse_down(
                                    MouseButton::Right,
                                    window.listener_for(
                                        &view,
                                        move |this, event: &MouseDownEvent, _, cx| {
                                            this.open_saved_group_context_menu(
                                                menu_group_name.clone(),
                                                event.position,
                                                cx,
                                            );
                                            cx.stop_propagation();
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
                                        .text_color(cx.theme().muted_foreground),
                                    )
                                    .child(if is_renaming {
                                        Input::new(&saved_group_name_input)
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
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child(display_name)
                                            .into_any_element()
                                    })
                                    .child(
                                        div()
                                            .text_size(rems(0.833))
                                            .text_color(cx.theme().muted_foreground)
                                            .child(format!("{count}")),
                                    )
                                    .when(!group_name.is_empty() && !is_renaming, |this| {
                                        this.child(
                                            Button::new(format!("saved-group-rename-{group_ix}"))
                                                .ghost()
                                                .xsmall()
                                                .label(t!("rename").to_string())
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    |_, window, cx| {
                                                        window.prevent_default();
                                                        cx.stop_propagation();
                                                    },
                                                )
                                                .on_click(window.listener_for(
                                                    &view,
                                                    move |this, _, window, cx| {
                                                        window.prevent_default();
                                                        cx.stop_propagation();
                                                        this.begin_saved_group_rename(
                                                            rename_group_name.clone(),
                                                            window,
                                                            cx,
                                                        );
                                                    },
                                                )),
                                        )
                                    }),
                            ),
                    )
                    .into_any_element()
            }
            SavedSidebarRow::Session {
                group_ix,
                session_ix,
                session_id,
                name,
                detail,
                full_detail,
                is_active,
            } => {
                let connect_id = session_id.clone();
                let tooltip_detail = full_detail.clone();
                let menu_session_id = session_id;
                let menu_detail = full_detail;
                div()
                    .id(("saved-sidebar-row", row_ix))
                    .w_full()
                    .h(px(50.))
                    .pl_4()
                    .p_1()
                    .child(
                        div()
                            .id(format!("saved-connect-{group_ix}-{session_ix}"))
                            .size_full()
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
                            .fast_hover(cx)
                            .on_mouse_down(
                                MouseButton::Left,
                                window.listener_for(&view, move |this, _, window, cx| {
                                    this.connect_saved_session_and_focus(
                                        connect_id.clone(),
                                        window,
                                        cx,
                                    )
                                }),
                            )
                            .on_mouse_down(
                                MouseButton::Right,
                                window.listener_for(
                                    &view,
                                    move |this, event: &MouseDownEvent, _, cx| {
                                        this.open_saved_session_context_menu(
                                            menu_session_id.clone(),
                                            menu_detail.clone(),
                                            event.position,
                                            cx,
                                        );
                                        cx.stop_propagation();
                                    },
                                ),
                            )
                            .tooltip(move |window, cx| {
                                gpui_component::tooltip::Tooltip::new(tooltip_detail.clone())
                                    .build(window, cx)
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
                                            .text_color(cx.theme().muted_foreground)
                                            .child(detail),
                                    ),
                            ),
                    )
                    .into_any_element()
            }
        }
    }

    fn render_collapsed_saved_sidebar_row(
        row_ix: usize,
        row: CollapsedSavedSidebarRow,
        view: gpui::Entity<AxShell>,
        window: &mut Window,
        cx: &mut gpui::App,
    ) -> AnyElement {
        match row {
            CollapsedSavedSidebarRow::Local { is_active } => div()
                .id(("collapsed-saved-sidebar-row", row_ix))
                .w_full()
                .h(px(42.))
                .flex()
                .items_center()
                .justify_center()
                .child(
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
                        .fast_hover(cx)
                        .on_mouse_down(
                            MouseButton::Left,
                            window.listener_for(&view, |this, _, window, cx| {
                                this.open_local_and_focus(window, cx)
                            }),
                        )
                        .tooltip(|window, cx| {
                            gpui_component::tooltip::Tooltip::new(
                                t!("open_local_shell_tab").to_string(),
                            )
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
                        ),
                )
                .into_any_element(),
            CollapsedSavedSidebarRow::Group {
                group_ix,
                group_name,
                display_name,
                count,
                is_expanded,
                is_active,
            } => {
                let group_abbrev = Self::collapsed_sidebar_abbrev(&display_name);
                let group_tooltip = format!("{} ({})", display_name, count);
                let menu_group_name = group_name.clone();
                div()
                    .id(("collapsed-saved-sidebar-row", row_ix))
                    .w_full()
                    .h(px(42.))
                    .flex()
                    .items_center()
                    .justify_center()
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
                            .border_color(if is_active || is_expanded {
                                cx.theme().primary
                            } else {
                                cx.theme().border
                            })
                            .bg(if is_active {
                                cx.theme().tab_active
                            } else if is_expanded {
                                cx.theme().secondary
                            } else {
                                cx.theme().muted
                            })
                            .cursor_pointer()
                            .fast_hover(cx)
                            .on_mouse_down(
                                MouseButton::Left,
                                window.listener_for(&view, move |this, _, _, cx| {
                                    this.toggle_saved_group(group_name.clone(), cx)
                                }),
                            )
                            .on_mouse_down(
                                MouseButton::Right,
                                window.listener_for(
                                    &view,
                                    move |this, event: &MouseDownEvent, _, cx| {
                                        this.open_saved_group_context_menu(
                                            menu_group_name.clone(),
                                            event.position,
                                            cx,
                                        );
                                        cx.stop_propagation();
                                    },
                                ),
                            )
                            .tooltip(move |window, cx| {
                                gpui_component::tooltip::Tooltip::new(group_tooltip.clone())
                                    .build(window, cx)
                            })
                            .child(
                                v_flex()
                                    .items_center()
                                    .justify_center()
                                    .gap(px(1.))
                                    .child(
                                        Icon::new(IconName::Folder)
                                            .with_size(Size::Small)
                                            .text_color(if is_active || is_expanded {
                                                cx.theme().primary
                                            } else {
                                                cx.theme().muted_foreground
                                            }),
                                    )
                                    .child(
                                        div()
                                            .text_size(rems(0.625))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(if is_active || is_expanded {
                                                cx.theme().primary
                                            } else {
                                                cx.theme().foreground
                                            })
                                            .child(group_abbrev),
                                    ),
                            ),
                    )
                    .into_any_element()
            }
            CollapsedSavedSidebarRow::Session {
                group_ix,
                session_ix,
                session_id,
                name,
                full_detail,
                is_active,
            } => {
                let connect_id = session_id.clone();
                let tooltip_detail = full_detail.clone();
                let menu_session_id = session_id;
                let menu_detail = full_detail;
                let abbrev = Self::collapsed_sidebar_abbrev(&name);
                div()
                    .id(("collapsed-saved-sidebar-row", row_ix))
                    .w_full()
                    .h(px(42.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
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
                            .fast_hover(cx)
                            .on_mouse_down(
                                MouseButton::Left,
                                window.listener_for(&view, move |this, _, window, cx| {
                                    this.connect_saved_session_and_focus(
                                        connect_id.clone(),
                                        window,
                                        cx,
                                    )
                                }),
                            )
                            .on_mouse_down(
                                MouseButton::Right,
                                window.listener_for(
                                    &view,
                                    move |this, event: &MouseDownEvent, _, cx| {
                                        this.open_saved_session_context_menu(
                                            menu_session_id.clone(),
                                            menu_detail.clone(),
                                            event.position,
                                            cx,
                                        );
                                        cx.stop_propagation();
                                    },
                                ),
                            )
                            .tooltip(move |window, cx| {
                                gpui_component::tooltip::Tooltip::new(tooltip_detail.clone())
                                    .build(window, cx)
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
                            ),
                    )
                    .into_any_element()
            }
        }
    }

    pub(super) fn render_collapsed_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let collapsed_rows = self.collapsed_saved_sidebar_rows();
        let collapsed_row_count = collapsed_rows.len();
        let collapsed_scroll_handle = self.collapsed_saved_scroll_handle.clone();
        let view = cx.entity();

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
                        uniform_list(
                            "collapsed-saved-sessions-fast-list",
                            collapsed_row_count,
                            move |range, list_window, cx| {
                                range
                                    .into_iter()
                                    .filter_map(|row_ix| {
                                        let row = collapsed_rows.get(row_ix)?.clone();
                                        Some(Self::render_collapsed_saved_sidebar_row(
                                            row_ix,
                                            row,
                                            view.clone(),
                                            list_window,
                                            cx,
                                        ))
                                    })
                                    .collect::<Vec<_>>()
                            },
                        )
                        .track_scroll(&collapsed_scroll_handle)
                        .w_full()
                        .h_full(),
                    )
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .bottom_0()
                            .left_0()
                            .right_0()
                            .child(
                                gpui_component::scroll::Scrollbar::new(&collapsed_scroll_handle)
                                    .id("collapsed-saved-scrollbar")
                                    .axis(gpui_component::scroll::ScrollbarAxis::Vertical)
                                    .scrollbar_show(
                                        gpui_component::scroll::ScrollbarShow::Scrolling,
                                    ),
                            ),
                    ),
            )
    }
}
