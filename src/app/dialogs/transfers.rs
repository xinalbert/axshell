use super::*;

impl AxShell {
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
                .overlay_closable(false)
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
                                crate::sftp::TransferState::Running
                                    | crate::sftp::TransferState::Paused
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
                                                crate::sftp::TransferState::Running
                                                    | crate::sftp::TransferState::Paused
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
                                        selectable_plain_text(
                                            "transfers-title",
                                            t!("transfers").to_string(),
                                        )
                                        .text_lg()
                                        .font_weight(FontWeight::SEMIBOLD),
                                    )
                                    .child(
                                        selectable_plain_text(
                                            "transfers-limit",
                                            t!("transfers_limit").to_string(),
                                        )
                                        .text_sm()
                                        .text_color(cx.theme().muted_foreground)
                                        .ml_2(),
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
                            crate::sftp::TransferState::Running
                            | crate::sftp::TransferState::Paused => 0,
                            _ => 1,
                        });

                        if transfers.is_empty() {
                            return content.child(
                                v_flex().gap_2().child(header).child(
                                    selectable_plain_text(
                                        "transfers-empty",
                                        t!("no_transfers_yet").to_string(),
                                    )
                                    .p_4()
                                    .text_center()
                                    .text_color(cx.theme().muted_foreground),
                                ),
                            );
                        }
                        let list = v_flex().gap_2().children(transfers.into_iter().map(|t| {
                            let (icon, _color) = match t.info.kind {
                                crate::sftp::TransferType::Upload => {
                                    (IconName::ArrowUp, cx.theme().primary)
                                }
                                crate::sftp::TransferType::Download => {
                                    (IconName::ArrowDown, cx.theme().success)
                                }
                            };

                            let (status_text, actions) = match t.state {
                                crate::sftp::TransferState::Running => {
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
                                            crate::sftp::TransferType::Upload => {
                                                format!("{}...", t!("uploading"))
                                            }
                                            crate::sftp::TransferType::Download => {
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
                                            if let Some(handle) = this.ensure_active_sftp_handle() {
                                                this.mark_active_sftp_activity();
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
                                            if let Some(handle) = this.ensure_active_sftp_handle() {
                                                this.mark_active_sftp_activity();
                                                handle.cancel_transfer(id.clone());
                                            }
                                        }
                                    }));
                                    (txt, h_flex().gap_1().child(btn_pause).child(btn_cancel))
                                }
                                crate::sftp::TransferState::Paused => {
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
                                            if let Some(handle) = this.ensure_active_sftp_handle() {
                                                this.mark_active_sftp_activity();
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
                                            if let Some(handle) = this.ensure_active_sftp_handle() {
                                                this.mark_active_sftp_activity();
                                                handle.cancel_transfer(id.clone());
                                            }
                                        }
                                    }));
                                    (txt, h_flex().gap_1().child(btn_resume).child(btn_cancel))
                                }
                                crate::sftp::TransferState::Interrupted(ref reason) => {
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
                                crate::sftp::TransferState::Completed => {
                                    let txt = t!("completed").to_string();
                                    let mut actions = h_flex().gap_1();
                                    if matches!(t.info.kind, crate::sftp::TransferType::Download) {
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
                                crate::sftp::TransferState::Failed(ref err) => {
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
                                crate::sftp::TransferState::Zombie(ref reason) => {
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
                                crate::sftp::TransferState::Completed => 100.0,
                                _ => t
                                    .total
                                    .map(|tot| t.transferred as f64 / tot as f64 * 100.0)
                                    .unwrap_or(0.0),
                            };
                            let transfer_id = t.info.id.clone();

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
                                                    selectable_plain_text(
                                                        ElementId::Name(
                                                            format!("transfer-name-{transfer_id}")
                                                                .into(),
                                                        ),
                                                        t.info.name.clone(),
                                                    )
                                                    .text_size(px(12.))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(cx.theme().foreground)
                                                    .overflow_hidden(),
                                                )
                                                .child(
                                                    selectable_plain_text(
                                                        ElementId::Name(
                                                            format!(
                                                                "transfer-session-{transfer_id}"
                                                            )
                                                            .into(),
                                                        ),
                                                        format!(
                                                            "{}: {}",
                                                            t!("session"),
                                                            t.tab_title
                                                        ),
                                                    )
                                                    .text_size(px(10.))
                                                    .text_color(cx.theme().muted_foreground)
                                                    .overflow_hidden(),
                                                )
                                                .child(
                                                    selectable_plain_text(
                                                        ElementId::Name(
                                                            format!(
                                                                "transfer-status-{transfer_id}"
                                                            )
                                                            .into(),
                                                        ),
                                                        status_text.clone(),
                                                    )
                                                    .text_size(px(11.))
                                                    .text_color(cx.theme().muted_foreground),
                                                ),
                                        )
                                        .child(actions),
                                )
                                .when(
                                    matches!(
                                        t.state,
                                        crate::sftp::TransferState::Running
                                            | crate::sftp::TransferState::Paused
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
}
