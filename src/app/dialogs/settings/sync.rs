use super::*;

use gpui_component::setting::{SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_sync_page(view: &gpui::Entity<AxShell>, shell: &AxShell) -> SettingPage {
    let endpoint = shell.sync_endpoint_input.clone();
    let username = shell.sync_username_input.clone();
    let webdav_password = shell.sync_webdav_password_input.clone();
    let s3_endpoint = shell.sync_s3_endpoint_input.clone();
    let s3_region = shell.sync_s3_region_input.clone();
    let s3_bucket = shell.sync_s3_bucket_input.clone();
    let s3_object_key = shell.sync_s3_object_key_input.clone();
    let s3_access_key = shell.sync_s3_access_key_input.clone();
    let s3_secret_key = shell.sync_s3_secret_key_input.clone();
    let s3_session_token = shell.sync_s3_session_token_input.clone();
    let encryption_password = shell.sync_encryption_password_input.clone();
    let in_progress = shell.sync_in_progress;
    let status = shell.sync_status.clone();
    let is_s3 = shell.config.sync_backend() == "s3";

    SettingPage::new(t!("settings_sync").to_string())
        .icon(IconName::Globe)
        .group(
            SettingGroup::new()
                .title(t!("settings_sync").to_string())
                .item(SettingItem::render({
                    let view = view.clone();
                    move |_, window, cx| {
                        let muted_foreground = cx.theme().muted_foreground;
                        v_flex()
                            .w_full()
                            .gap_3()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Button::new("sync-backend-webdav")
                                            .small()
                                            .label("WebDAV")
                                            .when(!is_s3, |button| button.primary())
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, _, cx| {
                                                    this.set_sync_backend("webdav", cx)
                                                },
                                            )),
                                    )
                                    .child(
                                        Button::new("sync-backend-s3")
                                            .small()
                                            .label("S3")
                                            .when(is_s3, |button| button.primary())
                                            .on_click(
                                                window.listener_for(&view, |this, _, _, cx| {
                                                    this.set_sync_backend("s3", cx)
                                                }),
                                            ),
                                    ),
                            )
                            .when(!is_s3, |this| {
                                this.child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div().text_sm().child(t!("sync_endpoint").to_string()),
                                        )
                                        .child(Input::new(&endpoint).w_full()),
                                )
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div().text_sm().child(t!("sync_username").to_string()),
                                        )
                                        .child(Input::new(&username).w_full()),
                                )
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_sm()
                                                .child(t!("sync_webdav_password").to_string()),
                                        )
                                        .child(Input::new(&webdav_password).w_full()),
                                )
                            })
                            .when(is_s3, |this| {
                                this.child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_sm()
                                                .child(t!("sync_s3_endpoint").to_string()),
                                        )
                                        .child(Input::new(&s3_endpoint).w_full()),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            v_flex()
                                                .flex_1()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .child(t!("sync_s3_region").to_string()),
                                                )
                                                .child(Input::new(&s3_region).w_full()),
                                        )
                                        .child(
                                            v_flex()
                                                .flex_1()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .child(t!("sync_s3_bucket").to_string()),
                                                )
                                                .child(Input::new(&s3_bucket).w_full()),
                                        ),
                                )
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_sm()
                                                .child(t!("sync_s3_object_key").to_string()),
                                        )
                                        .child(Input::new(&s3_object_key).w_full()),
                                )
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_sm()
                                                .child(t!("sync_s3_access_key").to_string()),
                                        )
                                        .child(Input::new(&s3_access_key).w_full()),
                                )
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_sm()
                                                .child(t!("sync_s3_secret_key").to_string()),
                                        )
                                        .child(Input::new(&s3_secret_key).w_full()),
                                )
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_sm()
                                                .child(t!("sync_s3_session_token").to_string()),
                                        )
                                        .child(Input::new(&s3_session_token).w_full()),
                                )
                            })
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .child(t!("sync_encryption_password").to_string()),
                                    )
                                    .child(Input::new(&encryption_password).w_full()),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(muted_foreground)
                                    .child(t!("sync_security_hint").to_string()),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Button::new("sync-download")
                                            .small()
                                            .disabled(in_progress)
                                            .label(t!("sync_download").to_string())
                                            .on_click(
                                                window.listener_for(&view, |this, _, _, cx| {
                                                    this.download_sync_config(cx)
                                                }),
                                            ),
                                    )
                                    .child(
                                        Button::new("sync-upload")
                                            .small()
                                            .disabled(in_progress)
                                            .label(t!("sync_upload").to_string())
                                            .on_click(
                                                window.listener_for(&view, |this, _, _, cx| {
                                                    this.upload_sync_config(cx)
                                                }),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(muted_foreground)
                                    .child(status.clone()),
                            )
                    }
                })),
        )
}
