use super::super::*;

impl AxShell {
    pub(super) fn render_sftp_transfer_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
        let visible_count = transfers.len();
        let visible_running_count = transfers
            .iter()
            .filter(|transfer| matches!(transfer.state, crate::sftp::TransferState::Running))
            .count();
        let visible_paused_count = transfers
            .iter()
            .filter(|transfer| matches!(transfer.state, crate::sftp::TransferState::Paused))
            .count();
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
                    .child(div().flex_1())
                    .when(selected_tab == SftpTransferTab::Active, |this| {
                        this.child(
                            Button::new("sftp-transfer-resume-all")
                                .ghost()
                                .small()
                                .icon(IconName::Play)
                                .label(t!("resume_all").to_string())
                                .disabled(visible_paused_count == 0)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.resume_sftp_transfers_in_tab(selected_tab, cx);
                                })),
                        )
                        .child(
                            Button::new("sftp-transfer-pause-all")
                                .ghost()
                                .small()
                                .icon(IconName::Pause)
                                .label(t!("pause_all").to_string())
                                .disabled(visible_running_count == 0)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.pause_sftp_transfers_in_tab(selected_tab, cx);
                                })),
                        )
                        .child(
                            Button::new("sftp-transfer-cancel-remove-all")
                                .ghost()
                                .small()
                                .icon(IconName::Close)
                                .label(t!("cancel_remove_all").to_string())
                                .disabled(visible_count == 0)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.cancel_remove_sftp_transfers_in_tab(
                                        selected_tab,
                                        true,
                                        cx,
                                    );
                                })),
                        )
                    })
                    .when(selected_tab != SftpTransferTab::Active, |this| {
                        this.child(
                            Button::new("sftp-transfer-remove-visible")
                                .ghost()
                                .small()
                                .icon(IconName::Close)
                                .label(t!("remove_all").to_string())
                                .disabled(visible_count == 0)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.cancel_remove_sftp_transfers_in_tab(
                                        selected_tab,
                                        false,
                                        cx,
                                    );
                                })),
                        )
                    }),
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
                                selectable_plain_text(
                                    "sftp-transfers-empty",
                                    t!("no_transfers_yet").to_string(),
                                )
                                .size_full()
                                .p_4()
                                .text_center()
                                .text_color(cx.theme().muted_foreground)
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
        transfer: crate::sftp::Transfer,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let icon = match transfer.info.kind {
            crate::sftp::TransferType::Upload => IconName::ArrowUp,
            crate::sftp::TransferType::Download => IconName::ArrowDown,
        };
        let status_text = sftp_transfer_status_text(&transfer);
        let transfer_id = transfer.info.id.clone();
        let mut actions = h_flex().flex_none().items_center().gap_1();

        match &transfer.state {
            crate::sftp::TransferState::Running => {
                let pause_id = transfer.info.id.clone();
                let cancel_id = transfer.info.id.clone();
                let pause_group_id = transfer.tab_id.clone();
                let cancel_group_id = transfer.tab_id.clone();
                actions = actions
                    .child(
                        Button::new(ElementId::Name(format!("sftp-pause-{pause_id}").into()))
                            .ghost()
                            .small()
                            .icon(IconName::Pause)
                            .on_click(cx.listener(move |this, _, _, _| {
                                if let Some(handle) =
                                    this.ensure_sftp_handle_for_group(&pause_group_id)
                                {
                                    this.mark_sftp_activity_for_group(&pause_group_id);
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
                                if let Some(handle) =
                                    this.ensure_sftp_handle_for_group(&cancel_group_id)
                                {
                                    this.mark_sftp_activity_for_group(&cancel_group_id);
                                    handle.cancel_transfer(cancel_id.clone());
                                }
                            })),
                    );
            }
            crate::sftp::TransferState::Paused => {
                let resume_id = transfer.info.id.clone();
                let cancel_id = transfer.info.id.clone();
                let resume_group_id = transfer.tab_id.clone();
                let cancel_group_id = transfer.tab_id.clone();
                actions = actions
                    .child(
                        Button::new(ElementId::Name(format!("sftp-resume-{resume_id}").into()))
                            .ghost()
                            .small()
                            .icon(IconName::Play)
                            .on_click(cx.listener(move |this, _, _, _| {
                                if let Some(handle) =
                                    this.ensure_sftp_handle_for_group(&resume_group_id)
                                {
                                    this.mark_sftp_activity_for_group(&resume_group_id);
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
                                if let Some(handle) =
                                    this.ensure_sftp_handle_for_group(&cancel_group_id)
                                {
                                    this.mark_sftp_activity_for_group(&cancel_group_id);
                                    handle.cancel_transfer(cancel_id.clone());
                                }
                            })),
                    );
            }
            crate::sftp::TransferState::Completed => {
                if matches!(transfer.info.kind, crate::sftp::TransferType::Download) {
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
            crate::sftp::TransferState::Failed(_)
            | crate::sftp::TransferState::Interrupted(_)
            | crate::sftp::TransferState::Zombie(_) => {
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

        h_flex()
            .w_full()
            .h(px(32.))
            .items_center()
            .gap_2()
            .px_3()
            .border_b_1()
            .border_color(cx.theme().border.opacity(0.35))
            .child(
                Icon::new(icon)
                    .with_size(Size::Small)
                    .text_color(cx.theme().primary),
            )
            .child(
                selectable_plain_text(
                    ElementId::Name(format!("sftp-transfer-name-{transfer_id}").into()),
                    transfer.info.name.clone(),
                )
                .flex_1()
                .min_w(px(0.))
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .text_size(rems(0.917))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(cx.theme().foreground),
            )
            .child(
                selectable_plain_text(
                    ElementId::Name(format!("sftp-transfer-status-{transfer_id}").into()),
                    status_text,
                )
                .w(px(180.))
                .min_w(px(0.))
                .flex_shrink_1()
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .text_size(rems(0.833))
                .text_color(cx.theme().muted_foreground),
            )
            .child(
                selectable_plain_text(
                    ElementId::Name(format!("sftp-transfer-session-{transfer_id}").into()),
                    transfer.tab_title,
                )
                .w(px(140.))
                .min_w(px(0.))
                .flex_shrink_1()
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .text_size(rems(0.833))
                .text_color(cx.theme().muted_foreground),
            )
            .child(actions)
            .into_any_element()
    }
}

fn transfer_belongs_to_tab(transfer: &crate::sftp::Transfer, tab: SftpTransferTab) -> bool {
    match tab {
        SftpTransferTab::Active => matches!(
            transfer.state,
            crate::sftp::TransferState::Running | crate::sftp::TransferState::Paused
        ),
        SftpTransferTab::Failed => matches!(
            transfer.state,
            crate::sftp::TransferState::Failed(_)
                | crate::sftp::TransferState::Interrupted(_)
                | crate::sftp::TransferState::Zombie(_)
        ),
        SftpTransferTab::Completed => {
            matches!(transfer.state, crate::sftp::TransferState::Completed)
        }
    }
}

fn sftp_transfer_percent(transfer: &crate::sftp::Transfer) -> f32 {
    match transfer.state {
        crate::sftp::TransferState::Completed => 100.0,
        _ => transfer
            .total
            .filter(|total| *total > 0)
            .map(|total| (transfer.transferred as f64 / total as f64 * 100.0) as f32)
            .unwrap_or(0.0)
            .clamp(0.0, 100.0),
    }
}

fn sftp_transfer_status_text(transfer: &crate::sftp::Transfer) -> String {
    match &transfer.state {
        crate::sftp::TransferState::Running => {
            if let Some(total) = transfer.total.filter(|total| *total > 0) {
                format!(
                    "{:.1}% ({}/{})",
                    sftp_transfer_percent(transfer),
                    format_bytes(transfer.transferred),
                    format_bytes(total)
                )
            } else {
                match transfer.info.kind {
                    crate::sftp::TransferType::Upload => format!("{}...", t!("uploading")),
                    crate::sftp::TransferType::Download => {
                        format!("{}...", t!("downloading"))
                    }
                }
            }
        }
        crate::sftp::TransferState::Paused => t!("paused").to_string(),
        crate::sftp::TransferState::Completed => t!("completed").to_string(),
        crate::sftp::TransferState::Failed(err) => format!("{}: {err}", t!("failed")),
        crate::sftp::TransferState::Interrupted(reason) => {
            format!("{}: {reason}", t!("interrupted"))
        }
        crate::sftp::TransferState::Zombie(reason) => format!("{}: {reason}", t!("zombie")),
    }
}
