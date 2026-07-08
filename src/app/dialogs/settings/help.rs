use super::*;

use gpui_component::setting::SettingPage;

pub(super) fn settings_help_page() -> SettingPage {
    SettingPage::new(t!("settings_help").to_string()).icon(IconName::BookOpen)
}
