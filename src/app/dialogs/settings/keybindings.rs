use super::*;

use gpui_component::setting::SettingPage;

pub(super) fn settings_keybindings_page(
    view: &gpui::Entity<AxShell>,
    config: &crate::config::ConfigStore,
    recording_action: Option<&str>,
    keybind_error: Option<&(String, String)>,
) -> SettingPage {
    let mut page = SettingPage::new(t!("settings_key_bindings").to_string())
        .icon(IconName::SquareTerminal)
        .default_open(true);
    for group in crate::app::keybinding_recorder::KeybindingsPage::render_groups(
        view,
        config,
        recording_action,
        keybind_error,
    ) {
        page = page.group(group);
    }
    page
}
