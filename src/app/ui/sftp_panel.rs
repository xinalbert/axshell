use std::cmp::Ordering;

use super::*;

impl AxShell {
    fn set_remote_sftp_sort(&mut self, column: SftpSortColumn, cx: &mut Context<Self>) {
        if self.remote_sftp_sort_column == column {
            self.remote_sftp_sort_direction = self.remote_sftp_sort_direction.toggled();
        } else {
            self.remote_sftp_sort_column = column;
            self.remote_sftp_sort_direction = SortDirection::Asc;
        }
        cx.notify();
    }

    fn set_local_sftp_sort(&mut self, column: SftpSortColumn, cx: &mut Context<Self>) {
        if self.local_sftp_sort_column == column {
            self.local_sftp_sort_direction = self.local_sftp_sort_direction.toggled();
        } else {
            self.local_sftp_sort_column = column;
            self.local_sftp_sort_direction = SortDirection::Asc;
        }
        cx.notify();
    }

    fn set_sftp_transfer_tab(&mut self, tab: SftpTransferTab, cx: &mut Context<Self>) {
        self.sftp_transfer_tab = tab;
        cx.notify();
    }

    pub(super) fn render_sftp_panel(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let active_sftp = self.active_sftp().cloned();

        let mut remote_entries = active_sftp
            .as_ref()
            .map(|sftp| {
                sftp.entries
                    .clone()
                    .into_iter()
                    .filter(|entry| self.show_hidden_files || !entry.name.starts_with('.'))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        sort_sftp_entries(
            &mut remote_entries,
            self.remote_sftp_sort_column,
            self.remote_sftp_sort_direction,
        );
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

        let mut local_entries = self
            .local_file_browser
            .entries
            .clone()
            .into_iter()
            .filter(|entry| self.show_hidden_files || !entry.name.starts_with('.'))
            .collect::<Vec<_>>();
        sort_sftp_entries(
            &mut local_entries,
            self.local_sftp_sort_column,
            self.local_sftp_sort_direction,
        );
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
        let size_col_width = px(92.);
        let modified_col_width = px(148.);
        let remote_sort_column = self.remote_sftp_sort_column;
        let remote_sort_direction = self.remote_sftp_sort_direction;
        let local_sort_column = self.local_sftp_sort_column;
        let local_sort_direction = self.local_sftp_sort_direction;

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
                            .min_w(px(96.))
                            .items_center()
                            .gap_2()
                            .child(div().w(icon_col_width).flex_none())
                            .child(
                                h_flex()
                                    .flex_1()
                                    .min_w(px(0.))
                                    .items_center()
                                    .gap_1()
                                    .overflow_hidden()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(cx.theme().muted))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.set_remote_sftp_sort(SftpSortColumn::Name, cx);
                                        }),
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
                                            .child(t!("name")),
                                    )
                                    .when(remote_sort_column == SftpSortColumn::Name, |this| {
                                        this.child(
                                            Icon::new(match remote_sort_direction {
                                                SortDirection::Asc => IconName::ChevronUp,
                                                SortDirection::Desc => IconName::ChevronDown,
                                            })
                                            .with_size(Size::XSmall)
                                            .text_color(cx.theme().primary),
                                        )
                                    }),
                            ),
                    )
                    .child(
                        h_flex()
                            .w(size_col_width)
                            .min_w(px(0.))
                            .flex_shrink_1()
                            .items_center()
                            .gap_1()
                            .overflow_hidden()
                            .cursor_pointer()
                            .hover(|style| style.bg(cx.theme().muted))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.set_remote_sftp_sort(SftpSortColumn::Size, cx);
                                }),
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
                                    .child(t!("size")),
                            )
                            .when(remote_sort_column == SftpSortColumn::Size, |this| {
                                this.child(
                                    Icon::new(match remote_sort_direction {
                                        SortDirection::Asc => IconName::ChevronUp,
                                        SortDirection::Desc => IconName::ChevronDown,
                                    })
                                    .with_size(Size::XSmall)
                                    .text_color(cx.theme().primary),
                                )
                            }),
                    )
                    .child(
                        h_flex()
                            .w(modified_col_width)
                            .min_w(px(0.))
                            .flex_shrink_1()
                            .items_center()
                            .gap_1()
                            .overflow_hidden()
                            .cursor_pointer()
                            .hover(|style| style.bg(cx.theme().muted))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.set_remote_sftp_sort(SftpSortColumn::Modified, cx);
                                }),
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
                                    .child(t!("modified")),
                            )
                            .when(remote_sort_column == SftpSortColumn::Modified, |this| {
                                this.child(
                                    Icon::new(match remote_sort_direction {
                                        SortDirection::Asc => IconName::ChevronUp,
                                        SortDirection::Desc => IconName::ChevronDown,
                                    })
                                    .with_size(Size::XSmall)
                                    .text_color(cx.theme().primary),
                                )
                            }),
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
                                                            .min_w(px(96.))
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
                                                                    .text_ellipsis()
                                                                    .whitespace_nowrap()
                                                                    .text_size(rems(1.0))
                                                                    .text_color(name_color)
                                                                    .child(entry.name),
                                                            ),
                                                    )
                                                    .child(
                                                        div()
                                                            .w(size_col_width)
                                                            .min_w(px(0.))
                                                            .flex_shrink_1()
                                                            .overflow_hidden()
                                                            .text_ellipsis()
                                                            .whitespace_nowrap()
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
                                                            .min_w(px(0.))
                                                            .flex_shrink_1()
                                                            .overflow_hidden()
                                                            .text_ellipsis()
                                                            .whitespace_nowrap()
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
                            .text_ellipsis()
                            .whitespace_nowrap()
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
                            .min_w(px(96.))
                            .items_center()
                            .gap_2()
                            .child(div().w(icon_col_width).flex_none())
                            .child(
                                h_flex()
                                    .flex_1()
                                    .min_w(px(0.))
                                    .items_center()
                                    .gap_1()
                                    .overflow_hidden()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(cx.theme().muted))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.set_local_sftp_sort(SftpSortColumn::Name, cx);
                                        }),
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
                                            .child(t!("name")),
                                    )
                                    .when(local_sort_column == SftpSortColumn::Name, |this| {
                                        this.child(
                                            Icon::new(match local_sort_direction {
                                                SortDirection::Asc => IconName::ChevronUp,
                                                SortDirection::Desc => IconName::ChevronDown,
                                            })
                                            .with_size(Size::XSmall)
                                            .text_color(cx.theme().primary),
                                        )
                                    }),
                            ),
                    )
                    .child(
                        h_flex()
                            .w(size_col_width)
                            .min_w(px(0.))
                            .flex_shrink_1()
                            .items_center()
                            .gap_1()
                            .overflow_hidden()
                            .cursor_pointer()
                            .hover(|style| style.bg(cx.theme().muted))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.set_local_sftp_sort(SftpSortColumn::Size, cx);
                                }),
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
                                    .child(t!("size")),
                            )
                            .when(local_sort_column == SftpSortColumn::Size, |this| {
                                this.child(
                                    Icon::new(match local_sort_direction {
                                        SortDirection::Asc => IconName::ChevronUp,
                                        SortDirection::Desc => IconName::ChevronDown,
                                    })
                                    .with_size(Size::XSmall)
                                    .text_color(cx.theme().primary),
                                )
                            }),
                    )
                    .child(
                        h_flex()
                            .w(modified_col_width)
                            .min_w(px(0.))
                            .flex_shrink_1()
                            .items_center()
                            .gap_1()
                            .overflow_hidden()
                            .cursor_pointer()
                            .hover(|style| style.bg(cx.theme().muted))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.set_local_sftp_sort(SftpSortColumn::Modified, cx);
                                }),
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
                                    .child(t!("modified")),
                            )
                            .when(local_sort_column == SftpSortColumn::Modified, |this| {
                                this.child(
                                    Icon::new(match local_sort_direction {
                                        SortDirection::Asc => IconName::ChevronUp,
                                        SortDirection::Desc => IconName::ChevronDown,
                                    })
                                    .with_size(Size::XSmall)
                                    .text_color(cx.theme().primary),
                                )
                            }),
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
                                                        .min_w(px(96.))
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
                                                                .text_ellipsis()
                                                                .whitespace_nowrap()
                                                                .text_size(rems(1.0))
                                                                .text_color(name_color)
                                                                .child(entry.name),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .w(size_col_width)
                                                        .min_w(px(0.))
                                                        .flex_shrink_1()
                                                        .overflow_hidden()
                                                        .text_ellipsis()
                                                        .whitespace_nowrap()
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
                                                        .min_w(px(0.))
                                                        .flex_shrink_1()
                                                        .overflow_hidden()
                                                        .text_ellipsis()
                                                        .whitespace_nowrap()
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
                            .text_ellipsis()
                            .whitespace_nowrap()
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

        let file_panes = h_flex()
            .items_stretch()
            .flex_1()
            .w_full()
            .h_full()
            .min_h(px(0.))
            .child(remote_pane)
            .child(div().w(px(1.)).h_full().bg(cx.theme().border))
            .child(local_pane);

        outer = outer.child(
            v_resizable("sftp-transfer-panels")
                .with_state(&self.sftp_transfer_panels)
                .child(
                    resizable_panel()
                        .size(px(360.))
                        .size_range(px(180.)..Pixels::MAX)
                        .child(file_panes),
                )
                .child(
                    resizable_panel()
                        .size(px(240.))
                        .size_range(px(120.)..Pixels::MAX)
                        .child(self.render_sftp_transfer_panel(cx)),
                ),
        );

        outer.into_any_element()
    }

    fn render_sftp_transfer_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_tab = self.sftp_transfer_tab;
        let active_count = self
            .transfers
            .iter()
            .filter(|transfer| transfer_belongs_to_tab(transfer, SftpTransferTab::Active))
            .count();
        let failed_count = self
            .transfers
            .iter()
            .filter(|transfer| transfer_belongs_to_tab(transfer, SftpTransferTab::Failed))
            .count();
        let completed_count = self
            .transfers
            .iter()
            .filter(|transfer| transfer_belongs_to_tab(transfer, SftpTransferTab::Completed))
            .count();
        let transfers = self
            .transfers
            .iter()
            .filter(|transfer| transfer_belongs_to_tab(transfer, selected_tab))
            .cloned()
            .collect::<Vec<_>>();
        let rows = transfers
            .into_iter()
            .map(|transfer| self.render_sftp_transfer_row(transfer, cx))
            .collect::<Vec<_>>();
        let scroll_handle = self.sftp_transfer_scroll_handle.clone();

        v_flex()
            .size_full()
            .min_h(px(0.))
            .border_t_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(
                h_flex()
                    .flex_none()
                    .h(px(36.))
                    .px_3()
                    .items_center()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().tab_bar)
                    .child(Self::sftp_transfer_tab_button(
                        SftpTransferTab::Active,
                        t!("sftp_transfer_active").to_string(),
                        active_count,
                        selected_tab,
                        cx,
                    ))
                    .child(Self::sftp_transfer_tab_button(
                        SftpTransferTab::Failed,
                        t!("failed").to_string(),
                        failed_count,
                        selected_tab,
                        cx,
                    ))
                    .child(Self::sftp_transfer_tab_button(
                        SftpTransferTab::Completed,
                        t!("completed").to_string(),
                        completed_count,
                        selected_tab,
                        cx,
                    ))
                    .child(div().flex_1()),
            )
            .child(
                div()
                    .flex_1()
                    .relative()
                    .min_h(px(0.))
                    .child(
                        div()
                            .id("sftp-transfer-scroll-view")
                            .size_full()
                            .overflow_y_scroll()
                            .track_scroll(&scroll_handle)
                            .pr(px(14.))
                            .child(if rows.is_empty() {
                                div()
                                    .size_full()
                                    .p_4()
                                    .text_center()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(t!("no_transfers_yet").to_string())
                                    .into_any_element()
                            } else {
                                v_flex().w_full().children(rows).into_any_element()
                            }),
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
            )
    }

    fn sftp_transfer_tab_button(
        tab: SftpTransferTab,
        label: String,
        count: usize,
        selected_tab: SftpTransferTab,
        cx: &mut Context<Self>,
    ) -> Button {
        Button::new(ElementId::Name(format!("sftp-transfer-tab-{tab:?}").into()))
            .ghost()
            .small()
            .selected(tab == selected_tab)
            .label(format!("{label} ({count})"))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.set_sftp_transfer_tab(tab, cx);
            }))
    }

    fn render_sftp_transfer_row(
        &self,
        transfer: crate::terminal::Transfer,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let icon = match transfer.info.kind {
            crate::terminal::TransferType::Upload => IconName::ArrowUp,
            crate::terminal::TransferType::Download => IconName::ArrowDown,
        };
        let status_text = sftp_transfer_status_text(&transfer);
        let percent = sftp_transfer_percent(&transfer);
        let mut actions = h_flex().flex_none().items_center().gap_1();

        match &transfer.state {
            crate::terminal::TransferState::Running => {
                let pause_id = transfer.info.id.clone();
                let cancel_id = transfer.info.id.clone();
                actions = actions
                    .child(
                        Button::new(ElementId::Name(format!("sftp-pause-{pause_id}").into()))
                            .ghost()
                            .small()
                            .icon(IconName::Pause)
                            .on_click(cx.listener(move |this, _, _, _| {
                                if let Some(handle) = this.active_sftp_handle() {
                                    handle.pause_transfer(pause_id.clone());
                                }
                            })),
                    )
                    .child(
                        Button::new(ElementId::Name(format!("sftp-cancel-{cancel_id}").into()))
                            .ghost()
                            .small()
                            .icon(IconName::Close)
                            .on_click(cx.listener(move |this, _, _, _| {
                                if let Some(handle) = this.active_sftp_handle() {
                                    handle.cancel_transfer(cancel_id.clone());
                                }
                            })),
                    );
            }
            crate::terminal::TransferState::Paused => {
                let resume_id = transfer.info.id.clone();
                let cancel_id = transfer.info.id.clone();
                actions = actions
                    .child(
                        Button::new(ElementId::Name(format!("sftp-resume-{resume_id}").into()))
                            .ghost()
                            .small()
                            .icon(IconName::Play)
                            .on_click(cx.listener(move |this, _, _, _| {
                                if let Some(handle) = this.active_sftp_handle() {
                                    handle.resume_transfer(resume_id.clone());
                                }
                            })),
                    )
                    .child(
                        Button::new(ElementId::Name(format!("sftp-cancel-{cancel_id}").into()))
                            .ghost()
                            .small()
                            .icon(IconName::Close)
                            .on_click(cx.listener(move |this, _, _, _| {
                                if let Some(handle) = this.active_sftp_handle() {
                                    handle.cancel_transfer(cancel_id.clone());
                                }
                            })),
                    );
            }
            crate::terminal::TransferState::Completed => {
                if matches!(transfer.info.kind, crate::terminal::TransferType::Download) {
                    let target = transfer.info.target.clone();
                    actions = actions.child(
                        Button::new(ElementId::Name(
                            format!("sftp-open-folder-{}", transfer.info.id).into(),
                        ))
                        .ghost()
                        .small()
                        .icon(IconName::Folder)
                        .on_click(move |_, _, _| {
                            let _ = std::process::Command::new("open").arg(&target).spawn();
                        }),
                    );
                }
                let remove_id = transfer.info.id.clone();
                actions = actions.child(
                    Button::new(ElementId::Name(format!("sftp-remove-{remove_id}").into()))
                        .ghost()
                        .small()
                        .icon(IconName::Close)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.remove_transfer(&remove_id, cx);
                        })),
                );
            }
            crate::terminal::TransferState::Failed(_)
            | crate::terminal::TransferState::Interrupted(_)
            | crate::terminal::TransferState::Zombie(_) => {
                let remove_id = transfer.info.id.clone();
                actions = actions.child(
                    Button::new(ElementId::Name(format!("sftp-remove-{remove_id}").into()))
                        .ghost()
                        .small()
                        .icon(IconName::Close)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.remove_transfer(&remove_id, cx);
                        })),
                );
            }
        }

        v_flex()
            .w_full()
            .gap_1()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(cx.theme().border.opacity(0.35))
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .gap_2()
                    .child(
                        Icon::new(icon)
                            .with_size(Size::Small)
                            .text_color(cx.theme().primary),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w(px(0.))
                            .overflow_hidden()
                            .child(
                                div()
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .whitespace_nowrap()
                                    .text_size(rems(0.917))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(cx.theme().foreground)
                                    .child(transfer.info.name.clone()),
                            )
                            .child(
                                div()
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .whitespace_nowrap()
                                    .text_size(rems(0.833))
                                    .text_color(cx.theme().muted_foreground)
                                    .child(format!("{}: {}", t!("session"), transfer.tab_title)),
                            )
                            .child(
                                div()
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .whitespace_nowrap()
                                    .text_size(rems(0.833))
                                    .text_color(cx.theme().muted_foreground)
                                    .child(status_text),
                            ),
                    )
                    .child(actions),
            )
            .when(
                matches!(
                    transfer.state,
                    crate::terminal::TransferState::Running
                        | crate::terminal::TransferState::Paused
                ),
                |this| {
                    this.child(
                        Progress::new(format!("sftp-progress-{}", transfer.info.id))
                            .with_size(px(4.))
                            .value(percent)
                            .color(cx.theme().primary)
                            .w_full(),
                    )
                },
            )
            .into_any_element()
    }
}

trait SftpSortableEntry {
    fn sort_name(&self) -> &str;
    fn sort_is_dir(&self) -> bool;
    fn sort_size(&self) -> u64;
    fn sort_modified(&self) -> u32;
}

impl SftpSortableEntry for crate::sftp::RemoteEntry {
    fn sort_name(&self) -> &str {
        &self.name
    }

    fn sort_is_dir(&self) -> bool {
        self.is_dir
    }

    fn sort_size(&self) -> u64 {
        self.size
    }

    fn sort_modified(&self) -> u32 {
        self.modified
    }
}

impl SftpSortableEntry for LocalFileEntry {
    fn sort_name(&self) -> &str {
        &self.name
    }

    fn sort_is_dir(&self) -> bool {
        self.is_dir
    }

    fn sort_size(&self) -> u64 {
        self.size
    }

    fn sort_modified(&self) -> u32 {
        self.modified
    }
}

fn sort_sftp_entries<T: SftpSortableEntry>(
    entries: &mut [T],
    column: SftpSortColumn,
    direction: SortDirection,
) {
    entries.sort_by(|a, b| {
        let dir_order = b.sort_is_dir().cmp(&a.sort_is_dir());
        if dir_order != Ordering::Equal {
            return dir_order;
        }

        let name_order = || {
            a.sort_name()
                .to_ascii_lowercase()
                .cmp(&b.sort_name().to_ascii_lowercase())
                .then_with(|| a.sort_name().cmp(b.sort_name()))
        };
        let order = match column {
            SftpSortColumn::Name => name_order(),
            SftpSortColumn::Size => a.sort_size().cmp(&b.sort_size()).then_with(name_order),
            SftpSortColumn::Modified => a
                .sort_modified()
                .cmp(&b.sort_modified())
                .then_with(name_order),
        };

        match direction {
            SortDirection::Asc => order,
            SortDirection::Desc => order.reverse(),
        }
    });
}

fn transfer_belongs_to_tab(transfer: &crate::terminal::Transfer, tab: SftpTransferTab) -> bool {
    match tab {
        SftpTransferTab::Active => matches!(
            transfer.state,
            crate::terminal::TransferState::Running | crate::terminal::TransferState::Paused
        ),
        SftpTransferTab::Failed => matches!(
            transfer.state,
            crate::terminal::TransferState::Failed(_)
                | crate::terminal::TransferState::Interrupted(_)
                | crate::terminal::TransferState::Zombie(_)
        ),
        SftpTransferTab::Completed => {
            matches!(transfer.state, crate::terminal::TransferState::Completed)
        }
    }
}

fn sftp_transfer_percent(transfer: &crate::terminal::Transfer) -> f32 {
    match transfer.state {
        crate::terminal::TransferState::Completed => 100.0,
        _ => transfer
            .total
            .filter(|total| *total > 0)
            .map(|total| (transfer.transferred as f64 / total as f64 * 100.0) as f32)
            .unwrap_or(0.0)
            .clamp(0.0, 100.0),
    }
}

fn sftp_transfer_status_text(transfer: &crate::terminal::Transfer) -> String {
    match &transfer.state {
        crate::terminal::TransferState::Running => {
            if let Some(total) = transfer.total.filter(|total| *total > 0) {
                format!(
                    "{:.1}% ({}/{})",
                    sftp_transfer_percent(transfer),
                    format_bytes(transfer.transferred),
                    format_bytes(total)
                )
            } else {
                match transfer.info.kind {
                    crate::terminal::TransferType::Upload => format!("{}...", t!("uploading")),
                    crate::terminal::TransferType::Download => {
                        format!("{}...", t!("downloading"))
                    }
                }
            }
        }
        crate::terminal::TransferState::Paused => t!("paused").to_string(),
        crate::terminal::TransferState::Completed => t!("completed").to_string(),
        crate::terminal::TransferState::Failed(err) => format!("{}: {err}", t!("failed")),
        crate::terminal::TransferState::Interrupted(reason) => {
            format!("{}: {reason}", t!("interrupted"))
        }
        crate::terminal::TransferState::Zombie(reason) => format!("{}: {reason}", t!("zombie")),
    }
}
