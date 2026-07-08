use super::*;

impl AxShell {
    pub(super) fn render_sftp_panel(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let active_sftp = self.active_sftp().cloned();

        let build_summary = |kind: crate::terminal::TransferType| -> Option<(String, String, f32)> {
            let active: Vec<&crate::terminal::Transfer> = self
                .transfers
                .iter()
                .filter(|t| {
                    matches!(
                        t.state,
                        crate::terminal::TransferState::Running
                            | crate::terminal::TransferState::Paused
                    ) && t.info.kind == kind
                })
                .collect();
            if active.is_empty() {
                return None;
            }
            Some(if active.len() == 1 {
                let transfer = &active[0];
                let pct = transfer.total.and_then(|total| {
                    if total > 0 {
                        Some((transfer.transferred as f64 / total as f64 * 100.0) as f32)
                    } else {
                        None
                    }
                });
                match pct {
                    Some(pct) => (transfer.info.name.clone(), format!("{pct:.0}%"), pct),
                    None => (transfer.info.name.clone(), "-".to_string(), 0.0),
                }
            } else {
                let total_transferred: u64 = active.iter().map(|t| t.transferred).sum();
                let total_total: u64 = active.iter().filter_map(|t| t.total).sum();
                let pct = if total_total > 0 {
                    Some((total_transferred as f64 / total_total as f64 * 100.0) as f32)
                } else {
                    None
                };
                let label = match kind {
                    crate::terminal::TransferType::Download => {
                        t!("files_downloading", count = active.len()).to_string()
                    }
                    crate::terminal::TransferType::Upload => {
                        t!("files_uploading", count = active.len()).to_string()
                    }
                };
                match pct {
                    Some(pct) => (label, format!("{pct:.0}%"), pct),
                    None => (label, "-".to_string(), 0.0),
                }
            })
        };
        let dl_summary = build_summary(crate::terminal::TransferType::Download);
        let ul_summary = build_summary(crate::terminal::TransferType::Upload);
        let has_transfers = dl_summary.is_some() || ul_summary.is_some();

        let remote_entries = active_sftp
            .as_ref()
            .map(|sftp| {
                sftp.entries
                    .clone()
                    .into_iter()
                    .filter(|entry| self.show_hidden_files || !entry.name.starts_with('.'))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let remote_selected_entries = active_sftp
            .as_ref()
            .map(|sftp| sftp.selected_entries.clone())
            .unwrap_or_default();
        let remote_selected_path = active_sftp
            .as_ref()
            .and_then(|sftp| sftp.selected_path.clone());
        let remote_status = active_sftp
            .as_ref()
            .map(|sftp| sftp.status.clone())
            .unwrap_or_default();
        let remote_current_path = active_sftp
            .as_ref()
            .map(|sftp| sftp.current_path.clone())
            .unwrap_or_else(|| "/".to_string());
        let remote_parent_path = Self::sftp_parent_path(&remote_current_path);
        let remote_all_selected = !remote_entries.is_empty()
            && remote_entries
                .iter()
                .all(|entry| remote_selected_entries.contains(&entry.full_path));

        let local_entries = self
            .local_file_browser
            .entries
            .clone()
            .into_iter()
            .filter(|entry| self.show_hidden_files || !entry.name.starts_with('.'))
            .collect::<Vec<_>>();
        let local_selected_entries = self.local_file_browser.selected_entries.clone();
        let local_selected_path = self.local_file_browser.selected_path.clone();
        let local_status = self.local_file_browser.status.clone();
        let local_current_path = self.local_file_browser.current_path.clone();
        let local_parent_path = Self::local_browser_parent_path(&local_current_path);
        let local_all_selected = !local_entries.is_empty()
            && local_entries
                .iter()
                .all(|entry| local_selected_entries.contains(&entry.full_path));

        let remote_ready = active_sftp.is_some();
        let remote_selected_count = remote_selected_entries.len();
        let local_selected_count = local_selected_entries.len();
        let can_upload_local_selection = remote_ready && local_selected_count > 0;
        let view = cx.entity();
        let icon_col_width = px(14.);
        let size_col_width = px(96.);
        let modified_col_width = px(152.);

        let remote_pane = v_flex()
            .flex_1()
            .h_full()
            .min_w(px(0.))
            .min_h(px(0.))
            .overflow_hidden()
            .child(
                h_flex()
                    .flex_none()
                    .h(px(34.))
                    .px_3()
                    .items_center()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().tab_bar)
                    .child(
                        div()
                            .text_size(rems(1.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(cx.theme().primary)
                            .child(t!("remote_files")),
                    )
                    .child(div().flex_1())
                    .child(
                        Checkbox::new("sftp-show-hidden")
                            .small()
                            .label(t!("hidden").to_string())
                            .checked(self.show_hidden_files)
                            .tab_stop(false)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.show_hidden_files = *checked;
                                this.config.set_show_hidden_files(*checked);
                                let _ = this.config.save();
                                cx.notify();
                            })),
                    ),
            )
            .child(
                h_flex()
                    .flex_none()
                    .h(px(34.))
                    .pl_3()
                    .pr_2()
                    .items_center()
                    .gap_1()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().muted.opacity(0.8))
                    .child(
                        Button::new("sftp-up")
                            .ghost()
                            .small()
                            .icon(IconName::ChevronUp)
                            .tooltip(t!("parent_folder").to_string())
                            .disabled(!remote_ready)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.navigate_sftp(remote_parent_path.clone(), cx);
                            })),
                    )
                    .child(Input::new(&self.sftp_path_input).flex_1().tab_index(0))
                    .child(
                        Button::new("sftp-refresh")
                            .ghost()
                            .small()
                            .icon(IconName::ArrowRight)
                            .tooltip(t!("refresh").to_string())
                            .disabled(!remote_ready)
                            .on_click(cx.listener(|this, _, _, cx| this.refresh_sftp(cx))),
                    )
                    .child(
                        Button::new("sftp-new-folder")
                            .ghost()
                            .small()
                            .icon(IconName::Folder)
                            .tooltip(t!("new_folder").to_string())
                            .disabled(!remote_ready)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.sftp_creating_folder = true;
                                this.sftp_new_folder_input.update(cx, |input, cx| {
                                    input.set_value("", window, cx);
                                    input.focus_handle(cx).focus(window, cx);
                                });
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("sftp-upload-file")
                            .ghost()
                            .small()
                            .icon(IconName::Plus)
                            .tooltip(t!("upload_file").to_string())
                            .disabled(!remote_ready)
                            .on_click(
                                cx.listener(|this, _, window, cx| this.upload_sftp_files(window, cx)),
                            ),
                    )
                    .child(
                        Button::new("sftp-upload-folder")
                            .ghost()
                            .small()
                            .icon(IconName::Folder)
                            .tooltip(t!("upload_folder").to_string())
                            .disabled(!remote_ready)
                            .on_click(
                                cx.listener(|this, _, window, cx| this.upload_sftp_folder(window, cx)),
                            ),
                    )
                    .child(
                        Button::new("sftp-download-selected")
                            .ghost()
                            .small()
                            .icon(IconName::ArrowDown)
                            .tooltip(if remote_selected_count == 0 {
                                t!("download").to_string()
                            } else {
                                t!("download_count", count = remote_selected_count).to_string()
                            })
                            .disabled(remote_selected_count == 0)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.download_selected_sftp_entries(window, cx);
                            })),
                    )
                    .child(
                        Button::new("sftp-delete-selected")
                            .ghost()
                            .small()
                            .icon(IconName::Close)
                            .tooltip(if remote_selected_count == 0 {
                                t!("delete_selected").to_string()
                            } else {
                                format!("{} ({remote_selected_count})", t!("delete_selected"))
                            })
                            .disabled(remote_selected_count == 0)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.show_delete_confirm_dialog(window, cx);
                            })),
                    ),
            )
            .child(
                h_flex()
                    .flex_none()
                    .h(px(26.))
                    .px_3()
                    .items_center()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().muted.opacity(0.8))
                    .child(
                        h_flex()
                            .w(px(24.))
                            .flex_none()
                            .items_center()
                            .justify_center()
                            .child(
                                Checkbox::new("sftp-select-all")
                                    .checked(remote_all_selected)
                                    .disabled(!remote_ready)
                                    .on_click(cx.listener(move |this, checked, _, cx| {
                                        this.toggle_all_sftp_entries(*checked, cx);
                                    })),
                            ),
                    )
                    .child(
                        h_flex()
                            .flex_1()
                            .min_w(px(0.))
                            .items_center()
                            .gap_2()
                            .child(div().w(icon_col_width).flex_none())
                            .child(
                                div()
                                    .flex_1()
                                    .text_size(rems(0.917))
                                    .text_color(cx.theme().muted_foreground)
                                    .child(t!("name")),
                            ),
                    )
                    .child(
                        div()
                            .w(size_col_width)
                            .flex_none()
                            .text_size(rems(0.917))
                            .text_color(cx.theme().muted_foreground)
                            .child(t!("size")),
                    )
                    .child(
                        div()
                            .w(modified_col_width)
                            .flex_none()
                            .text_size(rems(0.917))
                            .text_color(cx.theme().muted_foreground)
                            .child(t!("modified")),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .relative()
                    .min_h(px(0.))
                    .when(remote_ready, |this| {
                        this.child({
                            let entries = remote_entries.clone();
                            let selected_entries = remote_selected_entries.clone();
                            let selected_path = remote_selected_path.clone();
                            let view = view.clone();
                            let theme = cx.theme().clone();
                            uniform_list(
                                "sftp-entries-list",
                                entries.len(),
                                move |range, list_window, _cx| {
                                    range
                                        .into_iter()
                                        .filter_map(|ix| {
                                            let entry = entries.get(ix)?;
                                            let entry = entry.clone();
                                            let is_checked =
                                                selected_entries.contains(&entry.full_path);
                                            let is_selected = selected_path.as_deref()
                                                == Some(entry.full_path.as_str());
                                            let name_color = if entry.is_dir {
                                                theme.primary
                                            } else {
                                                theme.foreground
                                            };
                                            let bg = if is_selected {
                                                theme.secondary
                                            } else if ix % 2 == 0 {
                                                theme.background
                                            } else {
                                                theme.muted.opacity(0.5)
                                            };
                                            Some(
                                                h_flex()
                                                    .w_full()
                                                    .h(px(28.))
                                                    .items_center()
                                                    .px_3()
                                                    .gap_2()
                                                    .bg(bg)
                                                    .hover(|style| style.bg(theme.muted.opacity(0.8)))
                                                    .border_b_1()
                                                    .border_color(theme.border.opacity(0.35))
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        list_window.listener_for(&view, {
                                                            let entry = entry.clone();
                                                            move |this, _, _, cx| {
                                                                this.dismiss_sftp_context_menu(cx);
                                                                this.select_sftp_entry(entry.clone(), cx);
                                                            }
                                                        }),
                                                    )
                                                    .on_mouse_down(
                                                        MouseButton::Right,
                                                        list_window.listener_for(&view, {
                                                            let entry = entry.clone();
                                                            let remote_path = entry.full_path.clone();
                                                            move |this, event: &MouseDownEvent, _, cx| {
                                                                this.mark_sftp_entry_selected(
                                                                    &entry.full_path,
                                                                    cx,
                                                                );
                                                                this.open_sftp_context_menu(
                                                                    remote_path.clone(),
                                                                    entry.is_dir,
                                                                    event.position,
                                                                    cx,
                                                                );
                                                            }
                                                        }),
                                                    )
                                                    .child(
                                                        h_flex()
                                                            .w(px(24.))
                                                            .flex_none()
                                                            .items_center()
                                                            .justify_center()
                                                            .on_mouse_down(
                                                                MouseButton::Left,
                                                                |_, _, cx| cx.stop_propagation(),
                                                            )
                                                            .on_mouse_down(
                                                                MouseButton::Right,
                                                                |_, _, cx| cx.stop_propagation(),
                                                            )
                                                            .child(
                                                                Checkbox::new(ElementId::Name(
                                                                    format!("check-{}", entry.full_path)
                                                                        .into(),
                                                                ))
                                                                .checked(is_checked)
                                                                .on_click(list_window.listener_for(&view, {
                                                                    let path = entry.full_path.clone();
                                                                    move |this, checked, _, cx| {
                                                                        this.toggle_sftp_entry(
                                                                            path.clone(),
                                                                            *checked,
                                                                            cx,
                                                                        );
                                                                    }
                                                                })),
                                                            ),
                                                    )
                                                    .child(
                                                        h_flex()
                                                            .flex_1()
                                                            .min_w(px(0.))
                                                            .items_center()
                                                            .gap_2()
                                                            .child(
                                                                div()
                                                                    .w(icon_col_width)
                                                                    .flex_none()
                                                                    .text_size(rems(1.0))
                                                                    .text_color(name_color)
                                                                    .child(if entry.is_dir { "📁" } else { "📄" }),
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex_1()
                                                                    .min_w(px(0.))
                                                                    .overflow_hidden()
                                                                    .text_size(rems(1.0))
                                                                    .text_color(name_color)
                                                                    .child(entry.name),
                                                            ),
                                                    )
                                                    .child(
                                                        div()
                                                            .w(size_col_width)
                                                            .flex_none()
                                                            .text_size(rems(0.917))
                                                            .text_color(theme.muted_foreground)
                                                            .child(if entry.is_dir {
                                                                "-".to_string()
                                                            } else {
                                                                format_bytes(entry.size)
                                                            }),
                                                    )
                                                    .child(
                                                        div()
                                                            .w(modified_col_width)
                                                            .flex_none()
                                                            .text_size(rems(0.917))
                                                            .text_color(theme.muted_foreground)
                                                            .child(format_mtime(entry.modified)),
                                                    )
                                                    .child(div().w(px(12.)).flex_none())
                                                    .into_any_element(),
                                            )
                                        })
                                        .collect::<Vec<_>>()
                                },
                            )
                            .size_full()
                            .track_scroll(&self.remote_files_scroll_handle)
                        })
                        .child(
                            div()
                                .absolute()
                                .top_0()
                                .right_0()
                                .bottom_0()
                                .w(px(16.))
                                .child(
                                    Scrollbar::vertical(&self.remote_files_scroll_handle)
                                        .scrollbar_show(ScrollbarShow::Always),
                                ),
                        )
                    })
                    .when(!remote_ready, |this| {
                        this.child(
                            v_flex()
                                .size_full()
                                .items_center()
                                .justify_center()
                                .p_3()
                                .child(
                                    div()
                                        .text_size(rems(1.0))
                                        .text_color(cx.theme().muted_foreground)
                                        .child(t!("open_ssh_tab_sftp")),
                                ),
                        )
                    }),
            )
            .child(
                h_flex()
                    .flex_none()
                    .h(px(24.))
                    .px_3()
                    .items_center()
                    .border_t_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().tab_bar)
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(0.))
                            .overflow_hidden()
                            .text_size(rems(0.833))
                            .text_color(cx.theme().primary)
                            .italic()
                            .child(remote_status.clone()),
                    ),
            );

        let local_pane = v_flex()
            .flex_1()
            .h_full()
            .min_w(px(0.))
            .min_h(px(0.))
            .overflow_hidden()
            .child(
                h_flex()
                    .flex_none()
                    .h(px(34.))
                    .px_3()
                    .items_center()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().tab_bar)
                    .child(
                        div()
                            .text_size(rems(1.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(cx.theme().primary)
                            .child(t!("local_files")),
                    )
                    .child(div().flex_1()),
            )
            .child(
                h_flex()
                    .flex_none()
                    .h(px(34.))
                    .pl_3()
                    .pr_2()
                    .items_center()
                    .gap_1()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().muted.opacity(0.8))
                    .child(
                        Button::new("local-up")
                            .ghost()
                            .small()
                            .icon(IconName::ChevronUp)
                            .tooltip(t!("parent_folder").to_string())
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.navigate_local_file_browser(local_parent_path.clone(), cx);
                            })),
                    )
                    .child(Input::new(&self.local_sftp_path_input).flex_1().tab_index(0))
                    .child(
                        Button::new("local-refresh")
                            .ghost()
                            .small()
                            .icon(IconName::ArrowRight)
                            .tooltip(t!("refresh").to_string())
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.refresh_local_file_browser(cx);
                            })),
                    )
                    .child(
                        Button::new("local-upload-selected")
                            .ghost()
                            .small()
                            .icon(IconName::ArrowUp)
                            .tooltip(if local_selected_count == 0 {
                                t!("upload_selected").to_string()
                            } else {
                                format!("{} ({local_selected_count})", t!("upload_selected"))
                            })
                            .disabled(!can_upload_local_selection)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.upload_selected_local_entries_to_sftp(cx);
                            })),
                    ),
            )
            .child(
                h_flex()
                    .flex_none()
                    .h(px(26.))
                    .px_3()
                    .items_center()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().muted.opacity(0.8))
                    .child(
                        h_flex()
                            .w(px(24.))
                            .flex_none()
                            .items_center()
                            .justify_center()
                            .child(
                                Checkbox::new("local-select-all")
                                    .checked(local_all_selected)
                                    .on_click(cx.listener(move |this, checked, _, cx| {
                                        this.toggle_all_local_file_entries(*checked, cx);
                                    })),
                            ),
                    )
                    .child(
                        h_flex()
                            .flex_1()
                            .min_w(px(0.))
                            .items_center()
                            .gap_2()
                            .child(div().w(icon_col_width).flex_none())
                            .child(
                                div()
                                    .flex_1()
                                    .text_size(rems(0.917))
                                    .text_color(cx.theme().muted_foreground)
                                    .child(t!("name")),
                            ),
                    )
                    .child(
                        div()
                            .w(size_col_width)
                            .flex_none()
                            .text_size(rems(0.917))
                            .text_color(cx.theme().muted_foreground)
                            .child(t!("size")),
                    )
                    .child(
                        div()
                            .w(modified_col_width)
                            .flex_none()
                            .text_size(rems(0.917))
                            .text_color(cx.theme().muted_foreground)
                            .child(t!("modified")),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .relative()
                    .min_h(px(0.))
                    .child({
                        let entries = local_entries.clone();
                        let selected_entries = local_selected_entries.clone();
                        let selected_path = local_selected_path.clone();
                        let view = view.clone();
                        let theme = cx.theme().clone();
                        uniform_list(
                            "local-entries-list",
                            entries.len(),
                            move |range, list_window, _cx| {
                                range
                                    .into_iter()
                                    .filter_map(|ix| {
                                        let entry = entries.get(ix)?;
                                        let entry = entry.clone();
                                        let is_checked =
                                            selected_entries.contains(&entry.full_path);
                                        let is_selected = selected_path.as_deref()
                                            == Some(entry.full_path.as_str());
                                        let name_color = if entry.is_dir {
                                            theme.primary
                                        } else {
                                            theme.foreground
                                        };
                                        let bg = if is_selected {
                                            theme.secondary
                                        } else if ix % 2 == 0 {
                                            theme.background
                                        } else {
                                            theme.muted.opacity(0.5)
                                        };
                                        Some(
                                            h_flex()
                                                .w_full()
                                                .h(px(28.))
                                                .items_center()
                                                .px_3()
                                                .gap_2()
                                                .bg(bg)
                                                .hover(|style| style.bg(theme.muted.opacity(0.8)))
                                                .border_b_1()
                                                .border_color(theme.border.opacity(0.35))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    list_window.listener_for(&view, {
                                                        let entry = entry.clone();
                                                        move |this, _, _, cx| {
                                                            this.select_local_file_entry(entry.clone(), cx);
                                                        }
                                                    }),
                                                )
                                                .child(
                                                    h_flex()
                                                        .w(px(24.))
                                                        .flex_none()
                                                        .items_center()
                                                        .justify_center()
                                                        .on_mouse_down(
                                                            MouseButton::Left,
                                                            |_, _, cx| cx.stop_propagation(),
                                                        )
                                                        .child(
                                                            Checkbox::new(ElementId::Name(
                                                                format!("local-check-{}", entry.full_path)
                                                                    .into(),
                                                            ))
                                                            .checked(is_checked)
                                                            .on_click(list_window.listener_for(&view, {
                                                                let path = entry.full_path.clone();
                                                                move |this, checked, _, cx| {
                                                                    this.toggle_local_file_entry(
                                                                        path.clone(),
                                                                        *checked,
                                                                        cx,
                                                                    );
                                                                }
                                                            })),
                                                        ),
                                                )
                                                .child(
                                                    h_flex()
                                                        .flex_1()
                                                        .min_w(px(0.))
                                                        .items_center()
                                                        .gap_2()
                                                        .child(
                                                            div()
                                                                .w(icon_col_width)
                                                                .flex_none()
                                                                .text_size(rems(1.0))
                                                                .text_color(name_color)
                                                                .child(if entry.is_dir { "📁" } else { "📄" }),
                                                        )
                                                        .child(
                                                            div()
                                                                .flex_1()
                                                                .min_w(px(0.))
                                                                .overflow_hidden()
                                                                .text_size(rems(1.0))
                                                                .text_color(name_color)
                                                                .child(entry.name),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .w(size_col_width)
                                                        .flex_none()
                                                        .text_size(rems(0.917))
                                                        .text_color(theme.muted_foreground)
                                                        .child(if entry.is_dir {
                                                            "-".to_string()
                                                        } else {
                                                            format_bytes(entry.size)
                                                        }),
                                                )
                                                .child(
                                                    div()
                                                        .w(modified_col_width)
                                                        .flex_none()
                                                        .text_size(rems(0.917))
                                                        .text_color(theme.muted_foreground)
                                                        .child(format_mtime(entry.modified)),
                                                )
                                                .child(div().w(px(12.)).flex_none())
                                                .into_any_element(),
                                        )
                                    })
                                    .collect::<Vec<_>>()
                            },
                        )
                        .size_full()
                        .track_scroll(&self.local_files_scroll_handle)
                    })
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .right_0()
                            .bottom_0()
                            .w(px(16.))
                            .child(
                                Scrollbar::vertical(&self.local_files_scroll_handle)
                                    .scrollbar_show(ScrollbarShow::Always),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .flex_none()
                    .h(px(24.))
                    .px_3()
                    .items_center()
                    .border_t_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().tab_bar)
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(0.))
                            .overflow_hidden()
                            .text_size(rems(0.833))
                            .text_color(cx.theme().primary)
                            .italic()
                            .child(local_status),
                    ),
            );

        let mut outer = v_flex()
            .gap_0()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .size_full()
            .on_drop(
                cx.listener(|this, paths: &gpui::ExternalPaths, _window, cx| {
                    let paths_to_upload: Vec<String> = paths
                        .0
                        .iter()
                        .map(|path| path.to_string_lossy().to_string())
                        .collect();
                    this.upload_sftp_files_batch(paths_to_upload, cx);
                }),
            );

        outer = outer.child(
            h_flex()
                .items_stretch()
                .flex_1()
                .w_full()
                .h_full()
                .min_h(px(0.))
                .child(remote_pane)
                .child(div().w(px(1.)).h_full().bg(cx.theme().border))
                .child(local_pane),
        );
        outer = outer.child(
            h_flex()
                .flex_none()
                .h(px(24.))
                .px_3()
                .items_center()
                .border_t_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().tab_bar)
                .child(
                    div()
                        .text_size(rems(0.833))
                        .text_color(cx.theme().muted_foreground)
                        .child(remote_status),
                )
                .child(div().flex_1())
                .child(
                    Button::new("open-transfers")
                        .ghost()
                        .small()
                        .when(has_transfers, |this| {
                            let mut content = h_flex().items_center().gap_2();
                            if let Some((ref label, ref pct_display, pct)) = dl_summary {
                                content = content.child(
                                    h_flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            Icon::new(IconName::ArrowDown)
                                                .with_size(Size::Small)
                                                .text_color(cx.theme().primary),
                                        )
                                        .child(
                                            div()
                                                .text_size(rems(0.833))
                                                .text_color(cx.theme().primary)
                                                .italic()
                                                .child(label.clone()),
                                        )
                                        .child(
                                            Progress::new("sftp-status-dl")
                                                .with_size(px(4.))
                                                .value(pct)
                                                .color(cx.theme().primary)
                                                .w(px(50.0)),
                                        )
                                        .child(
                                            div()
                                                .text_size(rems(0.833))
                                                .text_color(cx.theme().primary)
                                                .italic()
                                                .child(pct_display.clone()),
                                        ),
                                );
                            }
                            if let Some((ref label, ref pct_display, pct)) = ul_summary {
                                if dl_summary.is_some() {
                                    content = content.child(div().w(px(6.)));
                                }
                                content = content.child(
                                    h_flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            Icon::new(IconName::ArrowUp)
                                                .with_size(Size::Small)
                                                .text_color(cx.theme().primary),
                                        )
                                        .child(
                                            div()
                                                .text_size(rems(0.833))
                                                .text_color(cx.theme().primary)
                                                .italic()
                                                .child(label.clone()),
                                        )
                                        .child(
                                            Progress::new("sftp-status-ul")
                                                .with_size(px(4.))
                                                .value(pct)
                                                .color(cx.theme().primary)
                                                .w(px(50.0)),
                                        )
                                        .child(
                                            div()
                                                .text_size(rems(0.833))
                                                .text_color(cx.theme().primary)
                                                .italic()
                                                .child(pct_display.clone()),
                                        ),
                                );
                            }
                            this.child(content)
                        })
                        .when(!has_transfers, |this| {
                            this.icon(IconName::ArrowDown)
                                .label(t!("transfers").to_string())
                        })
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.show_transfers_dialog(window, cx);
                        })),
                ),
        );

        outer.into_any_element()
    }
}
