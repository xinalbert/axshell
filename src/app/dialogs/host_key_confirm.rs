use std::sync::{Arc, Mutex};

use super::*;

impl AxShell {
    pub(crate) fn show_next_host_key_verification_dialog(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.active_dialog.is_some() {
            return;
        }
        let request = loop {
            let Some(request) = self.host_key_verification_requests.pop_front() else {
                return;
            };
            if self.config.host_key_trust(&request.key) == crate::config::HostKeyTrust::Trusted {
                let _ = request
                    .response
                    .send(crate::backend::host_key::HostKeyDecision::Trust);
                continue;
            }
            break request;
        };

        let is_changed = request.kind == crate::backend::host_key::HostKeyPromptKind::Changed;
        self.active_dialog = Some(crate::app::DialogKind::HostKeyConfirm);
        let view = cx.entity();
        let target = format!("{}:{}", request.key.host, request.key.port);
        let algorithm = request.key.algorithm.clone();
        let fingerprint = request.key.fingerprint.clone();
        let key = request.key;
        let response = Arc::new(Mutex::new(Some(request.response)));

        window.open_dialog(cx, move |dialog: Dialog, _window, _| {
            dialog
                .title(
                    if is_changed {
                        t!("ssh_host_key_changed_title")
                    } else {
                        t!("ssh_host_key_first_seen_title")
                    }
                    .to_string(),
                )
                .w(px(600.))
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
                            let _ =
                                response.send(crate::backend::host_key::HostKeyDecision::Reject);
                        }
                        view.update(cx, |this, cx| {
                            this.active_dialog = None;
                            cx.notify();
                        });
                    }
                })
                .content({
                    let target = target.clone();
                    let algorithm = algorithm.clone();
                    let fingerprint = fingerprint.clone();
                    move |content, _window, cx| {
                        let description = if is_changed {
                            t!("ssh_host_key_changed_description", host = target.clone())
                        } else {
                            t!("ssh_host_key_first_seen_description", host = target.clone())
                        };
                        content.child(
                            v_flex()
                                .w_full()
                                .gap_3()
                                .child(
                                    selectable_plain_text(
                                        "ssh-host-key-description",
                                        description.to_string(),
                                    )
                                    .text_base(),
                                )
                                .child(
                                    v_flex()
                                        .w_full()
                                        .gap_1()
                                        .child(
                                            selectable_plain_text(
                                                "ssh-host-key-target",
                                                t!("ssh_host_key_target", host = target)
                                                    .to_string(),
                                            )
                                            .text_sm(),
                                        )
                                        .child(
                                            selectable_plain_text(
                                                "ssh-host-key-algorithm",
                                                t!("ssh_host_key_algorithm", algorithm = algorithm)
                                                    .to_string(),
                                            )
                                            .text_sm(),
                                        )
                                        .child(
                                            selectable_plain_text(
                                                "ssh-host-key-fingerprint",
                                                t!(
                                                    "ssh_host_key_fingerprint",
                                                    fingerprint = fingerprint
                                                )
                                                .to_string(),
                                            )
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground),
                                        ),
                                ),
                        )
                    }
                })
                .footer({
                    let reject_view = view.clone();
                    let reject_response = response.clone();
                    let trust_view = view.clone();
                    let trust_response = response.clone();
                    let trust_key = key.clone();

                    h_flex()
                        .w_full()
                        .justify_end()
                        .gap_2()
                        .child(
                            Button::new("ssh-host-key-reject")
                                .ghost()
                                .label(t!("ssh_host_key_reject").to_string())
                                .on_click(move |_, window, cx| {
                                    respond_to_host_key_verification(
                                        &reject_view,
                                        &reject_response,
                                        crate::backend::host_key::HostKeyDecision::Reject,
                                        window,
                                        cx,
                                    );
                                }),
                        )
                        .child(
                            Button::new("ssh-host-key-trust")
                                .primary()
                                .label(if is_changed {
                                    t!("ssh_host_key_replace").to_string()
                                } else {
                                    t!("ssh_host_key_trust").to_string()
                                })
                                .on_click(move |_, window, cx| {
                                    trust_host_key(
                                        &trust_view,
                                        &trust_response,
                                        trust_key.clone(),
                                        window,
                                        cx,
                                    );
                                }),
                        )
                })
        });
    }
}

fn respond_to_host_key_verification(
    view: &gpui::Entity<AxShell>,
    response: &Arc<
        Mutex<Option<tokio::sync::oneshot::Sender<crate::backend::host_key::HostKeyDecision>>>,
    >,
    decision: crate::backend::host_key::HostKeyDecision,
    window: &mut Window,
    cx: &mut gpui::App,
) {
    view.update(cx, |this, cx| {
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

fn trust_host_key(
    view: &gpui::Entity<AxShell>,
    response: &Arc<
        Mutex<Option<tokio::sync::oneshot::Sender<crate::backend::host_key::HostKeyDecision>>>,
    >,
    key: crate::config::TrustedHostKey,
    window: &mut Window,
    cx: &mut gpui::App,
) {
    let trusted = view.update(cx, |this, cx| {
        this.config.trust_host_key(key);
        match this.config.save() {
            Ok(()) => true,
            Err(error) => {
                this.status = format!("{}: {error}", t!("ssh_host_key_save_failed")).into();
                cx.notify();
                false
            }
        }
    });
    respond_to_host_key_verification(
        view,
        response,
        if trusted {
            crate::backend::host_key::HostKeyDecision::Trust
        } else {
            crate::backend::host_key::HostKeyDecision::Reject
        },
        window,
        cx,
    );
}
