use super::*;

impl AxShell {
    pub(crate) fn show_sftp_transfer_close_dialog(
        &mut self,
        group_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.active_dialog.is_some() {
            return;
        }
        self.active_dialog = Some(crate::app::DialogKind::SftpCloseConfirm);
        self.sftp_close_remember_choice = false;
        self.sftp_close_confirm_group_id = Some(group_id.clone());

        let active_transfer_count = self
            .transfers
            .iter()
            .filter(|transfer| {
                transfer.tab_id == group_id
                    && matches!(
                        transfer.state,
                        crate::sftp::TransferState::Running | crate::sftp::TransferState::Paused
                    )
            })
            .count();
        let view = cx.entity();

        window.open_dialog(cx, move |dialog: Dialog, _window, _| {
            dialog
                .title(t!("sftp_transfer_close_title").to_string())
                .w(px(520.))
                .keyboard(false)
                .close_button(false)
                .overlay_closable(false)
                .on_close({
                    let view = view.clone();
                    move |_, _, cx| {
                        view.update(cx, |this, cx| {
                            this.active_dialog = None;
                            this.sftp_close_remember_choice = false;
                            this.sftp_close_confirm_group_id = None;
                            cx.notify();
                        });
                    }
                })
                .content({
                    let view = view.clone();
                    move |content, window, cx| {
                        let remember = view.read(cx).sftp_close_remember_choice;
                        content.child(
                            v_flex()
                                .w_full()
                                .gap_3()
                                .child(
                                    selectable_plain_text(
                                        "sftp-transfer-close-description",
                                        t!(
                                            "sftp_transfer_close_description",
                                            count = active_transfer_count
                                        )
                                        .to_string(),
                                    )
                                    .text_base(),
                                )
                                .child(
                                    Checkbox::new("sftp-transfer-close-remember")
                                        .checked(remember)
                                        .label(
                                            t!("remember_sftp_transfer_close_choice").to_string(),
                                        )
                                        .on_click(window.listener_for(
                                            &view,
                                            |this, checked, _, cx| {
                                                this.sftp_close_remember_choice = *checked;
                                                cx.notify();
                                            },
                                        )),
                                ),
                        )
                    }
                })
                .footer({
                    let group_id = group_id.clone();
                    h_flex()
                        .w_full()
                        .justify_end()
                        .gap_2()
                        .child(
                            Button::new("sftp-transfer-close-cancel")
                                .ghost()
                                .label(t!("cancel").to_string())
                                .on_click({
                                    let view = view.clone();
                                    move |_, window, cx| {
                                        view.update(cx, |this, cx| {
                                            this.active_dialog = None;
                                            this.sftp_close_remember_choice = false;
                                            this.sftp_close_confirm_group_id = None;
                                            cx.notify();
                                        });
                                        window.close_dialog(cx);
                                    }
                                }),
                        )
                        .child(
                            Button::new("sftp-transfer-keep-page")
                                .ghost()
                                .label(t!("sftp_keep_page_open").to_string())
                                .on_click({
                                    let view = view.clone();
                                    let group_id = group_id.clone();
                                    move |_, window, cx| {
                                        view.update(cx, |this, cx| {
                                            let remember = this.sftp_close_remember_choice;
                                            this.apply_sftp_transfer_close_choice(
                                                group_id.clone(),
                                                "keep_page_open",
                                                remember,
                                                window,
                                                cx,
                                            );
                                            this.active_dialog = None;
                                        });
                                        window.close_dialog(cx);
                                    }
                                }),
                        )
                        .child(
                            Button::new("sftp-transfer-background")
                                .primary()
                                .label(t!("sftp_continue_in_background").to_string())
                                .on_click({
                                    let view = view.clone();
                                    let group_id = group_id.clone();
                                    move |_, window, cx| {
                                        view.update(cx, |this, cx| {
                                            let remember = this.sftp_close_remember_choice;
                                            this.apply_sftp_transfer_close_choice(
                                                group_id.clone(),
                                                "background",
                                                remember,
                                                window,
                                                cx,
                                            );
                                            this.active_dialog = None;
                                        });
                                        window.close_dialog(cx);
                                    }
                                }),
                        )
                        .child(
                            Button::new("sftp-transfer-cancel-disconnect")
                                .danger()
                                .label(t!("sftp_cancel_and_disconnect").to_string())
                                .on_click({
                                    let view = view.clone();
                                    move |_, window, cx| {
                                        view.update(cx, |this, cx| {
                                            let remember = this.sftp_close_remember_choice;
                                            this.apply_sftp_transfer_close_choice(
                                                group_id.clone(),
                                                "cancel_disconnect",
                                                remember,
                                                window,
                                                cx,
                                            );
                                            this.active_dialog = None;
                                        });
                                        window.close_dialog(cx);
                                    }
                                }),
                        )
                })
        });
    }
}
