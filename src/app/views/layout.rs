use super::*;

impl Render for AxShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self
            .active_tab
            .as_ref()
            .is_some_and(|active_id| !self.tabs.iter().any(|tab| &tab.id == active_id))
        {
            self.active_tab = self.tabs.first().map(|tab| tab.id.clone());
        }
        self.sync_sftp_path_input(window, cx);
        self.sync_local_sftp_path_input(window, cx);

        if self.show_transfers_dialog {
            self.show_transfers_dialog = false;
            self.show_transfers_dialog(window, cx);
        }
        if let Some(active_id) = self.active_tab.clone() {
            if let Some(scrollbar) = self.terminal_scrollbars.get(&active_id) {
                if let Some(new_display_offset) = scrollbar.future_display_offset.take() {
                    if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == active_id) {
                        let current = tab.render_snapshot().display_offset;
                        match new_display_offset.cmp(&current) {
                            std::cmp::Ordering::Greater => {
                                tab.scroll_up_by(new_display_offset - current)
                            }
                            std::cmp::Ordering::Less => {
                                tab.scroll_down_by(current - new_display_offset)
                            }
                            std::cmp::Ordering::Equal => {}
                        }
                    }
                }
            }
            if let Some(snapshot) = self.active_snapshot().as_ref() {
                if let Some(scrollbar) = self.terminal_scrollbars.get(&active_id) {
                    scrollbar.update(snapshot, px(self.terminal_line_height()));
                }
            }
        }

        let show_bottom_monitoring = self.config.show_monitoring_dashboard()
            && self.config.monitoring_position() == "Bottom";
        let show_platform_menu_bar = cfg!(any(target_os = "windows", target_os = "linux"))
            && self.active_title_bar_style == crate::config::TitleBarStyle::Native;
        let sftp_context_remote_ready = self.active_sftp().is_some();

        let body_panel = v_flex()
            .size_full()
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .child(self.render_terminal_panel(window, cx)),
            )
            .when(show_bottom_monitoring, |this| {
                this.child(
                    div()
                        .flex_none()
                        .child(self.render_monitoring_panel(window.viewport_size().width, cx)),
                )
            })
            .into_any_element();

        let workspace = if self.sidebar_collapsed {
            h_flex()
                .size_full()
                .child(
                    div()
                        .flex_none()
                        .w(px(COLLAPSED_SIDEBAR_WIDTH))
                        .h_full()
                        .child(self.render_collapsed_sidebar(cx)),
                )
                .child(
                    div().flex_1().h_full().min_w(px(0.)).child(
                        v_flex()
                            .size_full()
                            .relative()
                            .overflow_hidden()
                            .when(
                                self.active_title_bar_style == crate::config::TitleBarStyle::Native,
                                |this| {
                                    this.child(
                                        div()
                                            .flex_none()
                                            .h(px(32.))
                                            .w_full()
                                            .bg(cx.theme().tab_bar)
                                            .border_b_1()
                                            .border_color(cx.theme().border)
                                            .child(self.render_tab_bar(cx)),
                                    )
                                },
                            )
                            .child(body_panel),
                    ),
                )
                .into_any_element()
        } else {
            let sidebar_area = resizable_panel()
                .size(px(self
                    .config
                    .workspace_panels()
                    .and_then(|s| s.first().copied())
                    .unwrap_or(SIDEBAR_WIDTH)))
                .size_range(px(240.)..px(520.))
                .flex_none()
                .child(self.sidebar(cx));

            let main_area = resizable_panel().child(
                v_flex()
                    .size_full()
                    .relative()
                    .overflow_hidden()
                    .when(
                        self.active_title_bar_style == crate::config::TitleBarStyle::Native,
                        |this| {
                            this.child(
                                div()
                                    .flex_none()
                                    .h(px(32.))
                                    .w_full()
                                    .bg(cx.theme().tab_bar)
                                    .border_b_1()
                                    .border_color(cx.theme().border)
                                    .child(self.render_tab_bar(cx)),
                            )
                        },
                    )
                    .child(body_panel),
            );

            h_resizable("ax_shell-workspace")
                .lock(self.config.lock_layout())
                .with_state(&self.workspace_panels)
                .child(sidebar_area)
                .child(main_area)
                .into_any_element()
        };

        v_flex()
            .id("ax_shell-root")
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .font_family(self.appearance.ui_font_family.clone())
            .on_action(cx.listener(|this, _: &crate::OpenSettings, _, cx| this.open_settings_page(cx)))
            .on_action(cx.listener(|this, _: &crate::OpenSession, window, cx| this.show_selector_dialog(window, cx)))
            .on_action(cx.listener(|this, _: &crate::OpenTransfers, window, cx| {
                this.open_sftp_transfers_page(window, cx);
            }))
            .on_action(cx.listener(|this, _: &crate::NewSsh, window, cx| this.show_ssh_dialog(window, cx)))
            .on_action(cx.listener(|this, _: &crate::ImportSavedSessions, window, cx| {
                this.import_saved_sessions_share_file(window, cx);
            }))
            .on_action(cx.listener(|this, _: &crate::ExportSavedSessions, window, cx| {
                this.export_saved_sessions_share_file(window, cx);
            }))
            .on_action(cx.listener(|this, _: &crate::OpenSearch, window, cx| this.toggle_search(window, cx)))
            .on_action(cx.listener(|this, _: &crate::PrevTab, window, cx| {
                this.switch_workspace_tab(-1, window, cx);
                window.prevent_default();
                cx.stop_propagation();
            }))
            .on_action(cx.listener(|this, _: &crate::NextTab, window, cx| {
                this.switch_workspace_tab(1, window, cx);
                window.prevent_default();
                cx.stop_propagation();
            }))
            .on_action(cx.listener(|this, _: &crate::ToggleSidebar, _, cx| {
                this.sidebar_collapsed = !this.sidebar_collapsed;
                this.config.set_sidebar_collapsed(this.sidebar_collapsed);
                this.config.save_logged("toggle_sidebar");
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &crate::ToggleSftpZoom, window, cx| {
                this.toggle_active_sftp_page(window, cx);
            }))
            .on_action(cx.listener(|this, _: &crate::FocusPaneLeft, _, _| {
                if this.workspace_page == WorkspacePage::Terminal {
                    this.focus_adjacent_pane("left");
                }
            }))
            .on_action(cx.listener(|this, _: &crate::FocusPaneRight, _, _| {
                if this.workspace_page == WorkspacePage::Terminal {
                    this.focus_adjacent_pane("right");
                }
            }))
            .on_action(cx.listener(|this, _: &crate::FocusPaneUp, _, _| {
                if this.workspace_page == WorkspacePage::Terminal {
                    this.focus_adjacent_pane("up");
                }
            }))
            .on_action(cx.listener(|this, _: &crate::FocusPaneDown, _, _| {
                if this.workspace_page == WorkspacePage::Terminal {
                    this.focus_adjacent_pane("down");
                }
            }))
            .on_action(cx.listener(|this, _: &crate::SplitPaneLeft, _, cx| {
                if this.workspace_page == WorkspacePage::Terminal {
                    this.split_current_pane("left", cx);
                }
            }))
            .on_action(cx.listener(|this, _: &crate::SplitPaneRight, _, cx| {
                if this.workspace_page == WorkspacePage::Terminal {
                    this.split_current_pane("right", cx);
                }
            }))
            .on_action(cx.listener(|this, _: &crate::SplitPaneUp, _, cx| {
                if this.workspace_page == WorkspacePage::Terminal {
                    this.split_current_pane("up", cx);
                }
            }))
            .on_action(cx.listener(|this, _: &crate::SplitPaneDown, _, cx| {
                if this.workspace_page == WorkspacePage::Terminal {
                    this.split_current_pane("down", cx);
                }
            }))
            .on_action(cx.listener(|this, _: &crate::ClosePane, window, cx| {
                if this.workspace_page == WorkspacePage::Sftp {
                    if let Some(group_id) = this.active_group.clone() {
                        this.close_sftp_page(group_id, window, cx);
                    }
                } else if let Some(active_id) = this.active_tab.clone() {
                    this.close_tab(active_id, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &crate::Copy, window, cx| {
                if window.focused(cx) == Some(this.focus_handle.clone()) {
                    if let Some(text) = this.active_terminal_selection_text() {
                        cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
                        if let Some(active_id) = this.active_tab.clone() {
                            this.clear_terminal_selection_for_tab(&active_id);
                        }
                        window.prevent_default();
                        cx.stop_propagation();
                    }
                } else {
                    cx.propagate();
                }
            }))
            .on_action(cx.listener(|this, _: &crate::Paste, window, cx| {
                if window.focused(cx) == Some(this.focus_handle.clone()) {
                    if let Some(clipboard) = cx.read_from_clipboard() {
                        if let Some(text) = clipboard.text() {
                            this.paste_into_terminal(&text, window, cx);
                        }
                    }
                } else {
                    cx.propagate();
                }
            }))
            .when(self.active_title_bar_style == crate::config::TitleBarStyle::Integrated, |this| {
                this.child(
                    div()
                        .id("title-bar")
                        .relative()
                        .flex()
                        .items_center()
                        .h(px(34.))
                        .w_full()
                        .bg(cx.theme().tab_bar)
                        .when(cfg!(target_os = "macos") && !window.is_fullscreen(), |this| {
                            this.child(
                                Self::bind_titlebar_drag(
                                    div()
                                        .id("macos-traffic-light-spacer")
                                        .flex_none()
                                        .w(px(80.))
                                        .h_full(),
                                    cx,
                                ),
                            )
                        })
                        .child(
                            Self::bind_titlebar_drag(
                                div()
                                    .id("tab-bar-drag")
                                    .flex_1()
                                    .min_w(px(0.))
                                    .h_full()
                                    .on_double_click(|_, window, _| {
                                        #[cfg(target_os = "macos")]
                                        window.titlebar_double_click();
                                        #[cfg(not(target_os = "macos"))]
                                        window.zoom_window();
                                    })
                                    .child(self.render_tab_bar(cx))
                                    .child(
                                        div()
                                            .absolute()
                                            .left_0()
                                            .right_0()
                                            .bottom_0()
                                            .h(px(1.))
                                            .bg(cx.theme().tab_bar),
                                    ),
                                cx,
                            ),
                        )
                        .child(
                            div()
                                .id("title-bar-bottom-border")
                                .absolute()
                                .left_0()
                                .right_0()
                                .bottom_0()
                                .h(px(1.))
                                .bg(cx.theme().border),
                        ),
                )
            })
            .when(show_platform_menu_bar, |this| {
                this.child(
                    div()
                        .flex_none()
                        .h(px(30.))
                        .w_full()
                        .px_2()
                        .items_center()
                        .bg(cx.theme().tab_bar)
                        .border_b_1()
                        .border_color(cx.theme().border)
                        .child(
                            self.app_menu_bar
                                .clone()
                                .expect("windows/linux app menu bar initialized"),
                        ),
                )
            })
            .child(
                div().flex_1().min_h_0().child(workspace),
            )
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .when_some(self.sftp_context_menu.clone(), |this, menu| {
                let menu_hover_tokens = fast_hover_tokens(cx);
                let menu_radius = cx.theme().radius;
                let menu_fg = cx.theme().popover_foreground;
                let menu_width = px(190.);
                let menu_height = px(128.);
                let menu_margin = px(8.);
                let viewport_size = window.viewport_size();
                let max_left = (viewport_size.width - menu_width - menu_margin).max(menu_margin);
                let max_top = (viewport_size.height - menu_height - menu_margin).max(menu_margin);
                let menu_left = menu.position.x.min(max_left).max(menu_margin);
                let menu_top = menu.position.y.min(max_top).max(menu_margin);
                let menu_item = move |id: &'static str, label: String| {
                    div()
                        .id(id)
                        .w_full()
                        .h(px(30.))
                        .px_2()
                        .flex()
                        .items_center()
                        .justify_start()
                        .rounded(menu_radius)
                        .text_size(rems(0.917))
                        .text_color(menu_fg)
                        .cursor_pointer()
                        .fast_hover_with_tokens(menu_hover_tokens)
                        .child(label)
                };
                let menu_body = match menu.target.clone() {
                    SftpContextMenuTarget::Remote { path, is_dir } => {
                        let download_label = if is_dir {
                            t!("download_folder").to_string()
                        } else {
                            t!("download").to_string()
                        };
                        v_flex()
                            .w_full()
                            .when(is_dir, |this| {
                                this.child(menu_item("sftp-context-open", t!("open_folder").to_string())
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.trigger_sftp_context_open(cx);
                                            cx.stop_propagation();
                                        }),
                                    ))
                            })
                            .child(
                                menu_item("sftp-context-download", download_label).on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, window, cx| {
                                        this.trigger_sftp_context_download(window, cx);
                                        cx.stop_propagation();
                                    }),
                                ),
                            )
                            .when(!is_dir && is_editable_text_file(&path), |this| {
                                this.child(
                                    menu_item("sftp-context-edit", t!("edit_file").to_string())
                                        .tooltip(|window, cx| {
                                            gpui_component::tooltip::Tooltip::new(
                                                t!("edit_file_tooltip").to_string(),
                                            )
                                            .build(window, cx)
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                            this.trigger_sftp_context_edit(cx);
                                                cx.stop_propagation();
                                            }),
                                        ),
                                )
                            })
                            .child(
                                menu_item("sftp-context-refresh", t!("refresh").to_string())
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                        this.trigger_sftp_context_refresh(cx);
                                            cx.stop_propagation();
                                        }),
                                    ),
                            )
                            .child(
                                menu_item("sftp-context-new-folder", t!("new_folder").to_string())
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, window, cx| {
                                        this.trigger_sftp_context_new_folder(window, cx);
                                            cx.stop_propagation();
                                        }),
                                    ),
                            )
                            .child(
                                menu_item("sftp-context-upload-file", t!("upload_file").to_string())
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, window, cx| {
                                        this.trigger_sftp_context_upload_file(window, cx);
                                            cx.stop_propagation();
                                        }),
                                    ),
                            )
                            .child(
                                menu_item("sftp-context-upload-folder", t!("upload_folder").to_string())
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, window, cx| {
                                        this.trigger_sftp_context_upload_folder(window, cx);
                                            cx.stop_propagation();
                                        }),
                                    ),
                            )
                            .child(
                                menu_item("sftp-context-delete", t!("delete").to_string())
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, window, cx| {
                                        this.trigger_sftp_context_delete(window, cx);
                                            cx.stop_propagation();
                                        }),
                                    ),
                            )
                            .into_any_element()
                    }
                    SftpContextMenuTarget::Local { is_dir, .. } => v_flex()
                        .w_full()
                        .child(
                            menu_item(
                                "sftp-context-local-open",
                                if is_dir {
                                    t!("open_folder").to_string()
                                } else {
                                    t!("open_file").to_string()
                                },
                            )
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.trigger_sftp_context_open(cx);
                                    cx.stop_propagation();
                                }),
                            ),
                        )
                        .child({
                            let upload_item =
                                menu_item("sftp-context-local-upload", t!("upload").to_string())
                                    .when(!sftp_context_remote_ready, |this| this.opacity(0.55));
                            if sftp_context_remote_ready {
                                upload_item
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                    this.trigger_local_context_upload(cx);
                                            cx.stop_propagation();
                                        }),
                                    )
                                    .into_any_element()
                            } else {
                                upload_item.into_any_element()
                            }
                        })
                        .child(
                            menu_item("sftp-context-local-refresh", t!("refresh").to_string())
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                    this.trigger_sftp_context_refresh(cx);
                                        cx.stop_propagation();
                                    }),
                                ),
                        )
                        .into_any_element(),
                };
                this.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .bottom_0()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.dismiss_sftp_context_menu(cx);
                            }),
                        )
                        .on_mouse_down(
                            MouseButton::Right,
                            cx.listener(|this, _, _, cx| {
                                this.dismiss_sftp_context_menu(cx);
                            }),
                        )
                        .child(
                            div()
                                .absolute()
                                .left(menu_left)
                                .top(menu_top)
                                .w(menu_width)
                                .p_1()
                                .rounded_md()
                                .border_1()
                                .border_color(cx.theme().border)
                                .bg(cx.theme().popover)
                                .shadow_lg()
                                .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                })
                                .on_mouse_down(MouseButton::Right, |_, window, cx| {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                })
                                .child(menu_body),
                        ),
                )
            })
            .when_some(self.saved_session_context_menu.clone(), |this, menu| {
                let view = cx.entity();
                let menu_hover_tokens = fast_hover_tokens(cx);
                let menu_radius = cx.theme().radius;
                let menu_fg = cx.theme().popover_foreground;
                let menu_item = move |id: &'static str, label: String| {
                    div()
                        .id(id)
                        .w_full()
                        .h(px(30.))
                        .px_2()
                        .flex()
                        .items_center()
                        .justify_start()
                        .rounded(menu_radius)
                        .text_size(rems(0.917))
                        .text_color(menu_fg)
                        .cursor_pointer()
                        .fast_hover_with_tokens(menu_hover_tokens)
                        .child(label)
                };
                let copy_value = menu.connection_info.clone();
                let export_id = menu.session_id.clone();
                let clone_id = menu.session_id.clone();
                let edit_id = menu.session_id.clone();
                let delete_id = menu.session_id.clone();
                let menu_body = v_flex()
                    .w_full()
                    .child(menu_item("saved-context-copy", t!("copy_connection_info").to_string())
                        .on_mouse_down(
                            MouseButton::Left,
                            window.listener_for(&view, move |this, _, _, cx| {
                                cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                                    copy_value.clone(),
                                ));
                                this.dismiss_saved_session_context_menu(cx);
                                cx.stop_propagation();
                            }),
                        ))
                    .child(
                        menu_item("saved-context-export", t!("export_ssh").to_string())
                            .on_mouse_down(
                                MouseButton::Left,
                                window.listener_for(&view, move |this, _, window, cx| {
                                    this.dismiss_saved_session_context_menu(cx);
                                    this.export_saved_session_share_file(
                                        export_id.clone(),
                                        window,
                                        cx,
                                    );
                                    cx.stop_propagation();
                                }),
                            ),
                    )
                    .child(
                        menu_item("saved-context-clone", t!("clone").to_string()).on_mouse_down(
                            MouseButton::Left,
                            window.listener_for(&view, move |this, _, window, cx| {
                                this.dismiss_saved_session_context_menu(cx);
                                this.clone_saved_session(clone_id.clone(), window, cx);
                                cx.stop_propagation();
                            }),
                        ),
                    )
                    .child(
                        menu_item("saved-context-edit", t!("edit").to_string()).on_mouse_down(
                            MouseButton::Left,
                            window.listener_for(&view, move |this, _, window, cx| {
                                this.dismiss_saved_session_context_menu(cx);
                                this.edit_saved_session(edit_id.clone(), window, cx);
                                cx.stop_propagation();
                            }),
                        ),
                    )
                    .child(
                        menu_item("saved-context-delete", t!("delete").to_string()).on_mouse_down(
                            MouseButton::Left,
                            window.listener_for(&view, move |this, _, _, cx| {
                                this.dismiss_saved_session_context_menu(cx);
                                this.remove_saved_session(delete_id.clone(), cx);
                                cx.stop_propagation();
                            }),
                        ),
                    );
                this.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .bottom_0()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.dismiss_saved_session_context_menu(cx);
                            }),
                        )
                        .on_mouse_down(
                            MouseButton::Right,
                            cx.listener(|this, _, _, cx| {
                                this.dismiss_saved_session_context_menu(cx);
                            }),
                        )
                        .child(
                            div()
                                .absolute()
                                .left(menu.position.x)
                                .top(menu.position.y)
                                .w(px(190.))
                                .p_1()
                                .rounded_md()
                                .border_1()
                                .border_color(cx.theme().border)
                                .bg(cx.theme().popover)
                                .shadow_lg()
                                .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                })
                                .on_mouse_down(MouseButton::Right, |_, window, cx| {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                })
                                .child(menu_body),
                        ),
                )
            })
            .when_some(self.saved_group_context_menu.clone(), |this, menu| {
                let view = cx.entity();
                let menu_hover_tokens = fast_hover_tokens(cx);
                let menu_radius = cx.theme().radius;
                let menu_fg = cx.theme().popover_foreground;
                let menu_item = move |id: &'static str, label: String| {
                    div()
                        .id(id)
                        .w_full()
                        .h(px(30.))
                        .px_2()
                        .flex()
                        .items_center()
                        .justify_start()
                        .rounded(menu_radius)
                        .text_size(rems(0.917))
                        .text_color(menu_fg)
                        .cursor_pointer()
                        .fast_hover_with_tokens(menu_hover_tokens)
                        .child(label)
                };
                let export_group_name = menu.group_name.clone();
                let menu_body = v_flex().w_full().child(
                    menu_item("saved-group-context-export", t!("export_group").to_string())
                        .on_mouse_down(
                            MouseButton::Left,
                            window.listener_for(&view, move |this, _, window, cx| {
                                this.dismiss_saved_group_context_menu(cx);
                                this.export_saved_group_share_file(
                                    export_group_name.clone(),
                                    window,
                                    cx,
                                );
                                cx.stop_propagation();
                            }),
                        ),
                );
                this.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .bottom_0()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.dismiss_saved_group_context_menu(cx);
                            }),
                        )
                        .on_mouse_down(
                            MouseButton::Right,
                            cx.listener(|this, _, _, cx| {
                                this.dismiss_saved_group_context_menu(cx);
                            }),
                        )
                        .child(
                            div()
                                .absolute()
                                .left(menu.position.x)
                                .top(menu.position.y)
                                .w(px(190.))
                                .p_1()
                                .rounded_md()
                                .border_1()
                                .border_color(cx.theme().border)
                                .bg(cx.theme().popover)
                                .shadow_lg()
                                .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                })
                                .on_mouse_down(MouseButton::Right, |_, window, cx| {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                })
                                .child(menu_body),
                        ),
                )
            })
            .when_some(self.connection_progress.clone(), |this, progress| {
                this.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .bottom_0()
                        .occlude()
                        .on_any_mouse_down(|_, _, cx| {
                            cx.stop_propagation();
                        })
                        .bg(gpui::Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.0,
                            a: 0.48,
                        })
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w(px(420.))
                                .p_5()
                                .rounded_lg()
                                .border_1()
                                .border_color(cx.theme().border)
                                .bg(cx.theme().popover)
                                .shadow_lg()
                                .child(
                                    v_flex()
                                        .gap_4()
                                        .child(
                                            Button::new("ssh-connect-progress")
                                                .primary()
                                                .loading(!progress.failed)
                                                .label(progress.title.clone()),
                                        )
                                        .child(
                                            div()
                                                .relative()
                                                .min_h(px(0.))
                                                .max_h(px(220.))
                                                .child(
                                                    div()
                                                        .id("connection-progress-scroll")
                                                        .max_h(px(220.))
                                                        .overflow_hidden()
                                                        .overflow_y_scroll()
                                                        .track_scroll(&self.connection_scroll_handle)
                                                        .child(
                                                            v_flex().gap_2().children(
                                                                progress.lines.iter().enumerate().map(|(line_ix, line)| {
                                                                    selectable_plain_text(
                                                                        ("connection-progress-line", line_ix),
                                                                        line,
                                                                    )
                                                                    .text_size(rems(1.0))
                                                                    .text_color(if progress.failed {
                                                                        cx.theme().danger
                                                                    } else {
                                                                        cx.theme().muted_foreground
                                                                    })
                                                                }),
                                                            ),
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .absolute()
                                                        .top_0()
                                                        .right_0()
                                                        .bottom_0()
                                                        .w(px(16.))
                                                        .child(
                                                            Scrollbar::vertical(&self.connection_scroll_handle)
                                                                .scrollbar_show(ScrollbarShow::Scrolling)
                                                        )
                                                )
                                        )
                                        .when(!progress.failed, |this| {
                                            this.child(
                                                h_flex()
                                                    .justify_end()
                                                    .child(
                                                        Button::new("ssh-connect-progress-cancel")
                                                            .label(t!("cancel").to_string())
                                                            .on_click(cx.listener(
                                                                |this, _, _, cx| {
                                                                    this.cancel_connection_progress(
                                                                        cx,
                                                                    )
                                                                },
                                                            )),
                                                    ),
                                            )
                                        })
                                        .when(progress.failed, |this| {
                                            this.child(
                                                h_flex()
                                                    .justify_end()
                                                    .gap_2()
                                                    .child(
                                                        Button::new("ssh-connect-progress-retry")
                                                            .primary()
                                                            .label(t!("retry").to_string())
                                                            .on_click(cx.listener(
                                                                |this, _, _, cx| {
                                                                    this.retry_connection_progress(
                                                                        cx,
                                                                    )
                                                                },
                                                            )),
                                                    )
                                                    .child(
                                                        Button::new("ssh-connect-progress-close")
                                                            .label(t!("cancel").to_string())
                                                            .on_click(cx.listener(
                                                                |this, _, _, cx| {
                                                                    this.cancel_connection_progress(
                                                                        cx,
                                                                    )
                                                                },
                                                            )),
                                                    ),
                                            )
                                        }),
                                ),
                        ),
                )
            })
            .on_prepaint({
                let view = cx.entity().clone();
                move |_, window, cx| {
                    view.update(cx, |this, cx| {
                        let current_win_size = window.viewport_size();
                        let size_changed = this.last_window_size.map_or(true, |prev| prev != current_win_size);
                        this.last_window_size = Some(current_win_size);

                        let current_sizes = this.workspace_panels.read(cx).sizes().clone();
                        if let Some(current_first_size) = current_sizes.first().copied() {
                            if size_changed {
                                if let Some(target_width) = this.last_sidebar_width {
                                    if current_first_size != target_width {
                                        this.workspace_panels.update(cx, |state, cx| {
                                            state.resize_panel(0, target_width, window, cx);
                                        });
                                    }
                                }
                            } else {
                                this.last_sidebar_width = Some(current_first_size);
                            }
                        }
                    });
                }
            })
    }
}
