use super::*;

use gpui_component::setting::{SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_about_page() -> SettingPage {
    let version = crate::app::constants::public_version_label();
    let runtime_log_dir = crate::app::startup::runtime_log_dir();
    let crash_report_dir = crate::app::startup::crash_report_dir();
    let runtime_log_dir_label = runtime_log_dir.display().to_string();
    let crash_report_dir_label = crash_report_dir.display().to_string();

    SettingPage::new(t!("settings_about").to_string())
        .icon(IconName::Info)
        .group(
            SettingGroup::new().item(SettingItem::render(move |_, _window, cx| {
                v_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        selectable_plain_text("about-app-name", "AxShell")
                            .text_size(rems(1.5))
                            .font_weight(FontWeight::BOLD),
                    )
                    .child(
                        selectable_plain_text("about-version", format!("Version {}", version))
                            .text_size(rems(0.9)),
                    )
                    .child(
                        selectable_plain_text(
                            "about-description",
                            "A GPUI Component based SSH and local terminal client",
                        )
                        .text_size(rems(0.9))
                        .text_color(cx.theme().muted_foreground),
                    )
                    .child(
                        selectable_plain_text("about-feedback-hint", t!("about_feedback_hint"))
                            .text_size(rems(0.9))
                            .text_color(cx.theme().muted_foreground),
                    )
                    .child(
                        Button::new("github-link")
                            .label(crate::app::constants::REPOSITORY_URL)
                            .ghost()
                            .on_click(|_, _window, _cx| {
                                let _ = open::that(crate::app::constants::REPOSITORY_URL);
                            }),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .items_center()
                            .child(
                                selectable_plain_text(
                                    "about-runtime-log-dir",
                                    format!(
                                        "{}: {}",
                                        t!("about_runtime_log_dir"),
                                        runtime_log_dir_label
                                    ),
                                )
                                .text_size(rems(0.8))
                                .text_color(cx.theme().muted_foreground),
                            )
                            .child(
                                selectable_plain_text(
                                    "about-crash-report-dir",
                                    format!(
                                        "{}: {}",
                                        t!("about_crash_report_dir"),
                                        crash_report_dir_label
                                    ),
                                )
                                .text_size(rems(0.8))
                                .text_color(cx.theme().muted_foreground),
                            ),
                    )
                    .child({
                        let log_dir = runtime_log_dir.clone();
                        let crash_dir = crash_report_dir.clone();
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("open-log-dir")
                                    .small()
                                    .label(t!("about_open_log_dir").to_string())
                                    .on_click(move |_, _window, _cx| {
                                        let _ = std::fs::create_dir_all(&log_dir);
                                        if let Err(err) = open::that(&log_dir) {
                                            tracing::warn!(
                                                component = "diagnostics",
                                                operation = "open_runtime_log_dir",
                                                log_path = %crate::diagnostics::mask_path(&log_dir.to_string_lossy()),
                                                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                                                "Failed to open runtime log directory"
                                            );
                                        }
                                    }),
                            )
                            .child(
                                Button::new("open-crash-dir")
                                    .small()
                                    .label(t!("about_open_crash_dir").to_string())
                                    .on_click(move |_, _window, _cx| {
                                        let _ = std::fs::create_dir_all(&crash_dir);
                                        if let Err(err) = open::that(&crash_dir) {
                                            tracing::warn!(
                                                component = "diagnostics",
                                                operation = "open_crash_report_dir",
                                                crash_path = %crate::diagnostics::mask_path(&crash_dir.to_string_lossy()),
                                                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                                                "Failed to open crash report directory"
                                            );
                                        }
                                    }),
                            )
                    })
            })),
        )
}
