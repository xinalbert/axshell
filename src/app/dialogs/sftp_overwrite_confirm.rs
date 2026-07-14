use std::sync::{Arc, Mutex};

use super::*;

impl AxShell {
    pub(crate) fn show_next_sftp_overwrite_dialog(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.active_dialog.is_some() {
            return;
        }
        let request = loop {
            let Some(request) = self.sftp_overwrite_requests.pop_front() else {
                return;
            };
            let transfer_is_active = self.transfers.iter().any(|transfer| {
                transfer.info.id == request.transfer_id
                    && matches!(
                        transfer.state,
                        crate::sftp::TransferState::Running | crate::sftp::TransferState::Paused
                    )
            });
            if transfer_is_active {
                break request;
            }
            let _ = request
                .response
                .send(crate::sftp::SftpOverwriteDecision::Skip);
        };

        self.active_dialog = Some(crate::app::DialogKind::SftpOverwriteConfirm);
        let view = cx.entity();
        let remote_path = request.remote_path;
        let local_path = request.local_path;
        let response = Arc::new(Mutex::new(Some(request.response)));

        window.open_dialog(cx, move |dialog: Dialog, _window, _| {
            dialog
                .title(t!("sftp_overwrite_title").to_string())
                .w(px(560.))
                .keyboard(false)
                .close_button(false)
                .overlay_closable(false)
                .on_close({
                    let view = view.clone();
                    let response = response.clone();
                    move |_, _, cx| {
                        let response = response
                            .lock()
                            .unwrap_or_else(|poisoned| poisoned.into_inner())
                            .take();
                        if let Some(response) = response {
                            let _ = response.send(crate::sftp::SftpOverwriteDecision::Skip);
                        }
                        view.update(cx, |this, cx| {
                            this.active_dialog = None;
                            cx.notify();
                        });
                    }
                })
                .content({
                    let remote_path = remote_path.clone();
                    let local_path = local_path.clone();
                    move |content, _window, _cx| {
                        content.child(
                            v_flex()
                                .w_full()
                                .gap_3()
                                .child(
                                    selectable_plain_text(
                                        "sftp-overwrite-description",
                                        t!("sftp_overwrite_description").to_string(),
                                    )
                                    .text_base(),
                                )
                                .child(
                                    v_flex()
                                        .w_full()
                                        .gap_1()
                                        .child(
                                            selectable_plain_text(
                                                "sftp-overwrite-remote",
                                                t!("sftp_overwrite_remote", path = remote_path)
                                                    .to_string(),
                                            )
                                            .text_sm(),
                                        )
                                        .child(
                                            selectable_plain_text(
                                                "sftp-overwrite-local",
                                                t!("sftp_overwrite_local", path = local_path)
                                                    .to_string(),
                                            )
                                            .text_sm()
                                            .text_color(_cx.theme().muted_foreground),
                                        ),
                                ),
                        )
                    }
                })
                .footer({
                    let skip_view = view.clone();
                    let skip_response = response.clone();
                    let replace_view = view.clone();
                    let replace_response = response.clone();
                    let transfer_view = view.clone();
                    let transfer_response = response.clone();
                    let run_view = view.clone();
                    let run_response = response.clone();

                    h_flex()
                        .w_full()
                        .justify_end()
                        .gap_2()
                        .child(
                            Button::new("sftp-overwrite-skip")
                                .ghost()
                                .label(t!("sftp_overwrite_skip").to_string())
                                .on_click(move |_, window, cx| {
                                    respond_to_sftp_overwrite(
                                        &skip_view,
                                        &skip_response,
                                        crate::sftp::SftpOverwriteDecision::Skip,
                                        false,
                                        window,
                                        cx,
                                    );
                                }),
                        )
                        .child(
                            Button::new("sftp-overwrite-replace")
                                .ghost()
                                .label(t!("sftp_overwrite_replace").to_string())
                                .on_click(move |_, window, cx| {
                                    respond_to_sftp_overwrite(
                                        &replace_view,
                                        &replace_response,
                                        crate::sftp::SftpOverwriteDecision::Replace,
                                        false,
                                        window,
                                        cx,
                                    );
                                }),
                        )
                        .child(
                            Button::new("sftp-overwrite-transfer")
                                .primary()
                                .label(t!("sftp_overwrite_replace_transfer").to_string())
                                .on_click(move |_, window, cx| {
                                    respond_to_sftp_overwrite(
                                        &transfer_view,
                                        &transfer_response,
                                        crate::sftp::SftpOverwriteDecision::ReplaceAllInTransfer,
                                        false,
                                        window,
                                        cx,
                                    );
                                }),
                        )
                        .child(
                            Button::new("sftp-overwrite-run")
                                .primary()
                                .label(t!("sftp_overwrite_replace_run").to_string())
                                .on_click(move |_, window, cx| {
                                    respond_to_sftp_overwrite(
                                        &run_view,
                                        &run_response,
                                        crate::sftp::SftpOverwriteDecision::Replace,
                                        true,
                                        window,
                                        cx,
                                    );
                                }),
                        )
                })
        });
    }

    fn approve_queued_sftp_overwrites(&mut self) {
        while let Some(request) = self.sftp_overwrite_requests.pop_front() {
            let _ = request
                .response
                .send(crate::sftp::SftpOverwriteDecision::Replace);
        }
    }
}

fn respond_to_sftp_overwrite(
    view: &gpui::Entity<AxShell>,
    response: &Arc<Mutex<Option<tokio::sync::oneshot::Sender<crate::sftp::SftpOverwriteDecision>>>>,
    decision: crate::sftp::SftpOverwriteDecision,
    apply_for_run: bool,
    window: &mut Window,
    cx: &mut gpui::App,
) {
    view.update(cx, |this, cx| {
        if apply_for_run {
            this.sftp_replace_all_for_run = true;
            this.approve_queued_sftp_overwrites();
        }
        let response = response
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        if let Some(response) = response {
            let _ = response.send(decision);
        }
        this.active_dialog = None;
        cx.notify();
    });
    window.close_dialog(cx);
}
