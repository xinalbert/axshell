use super::*;

mod about;
mod custom;
mod fonts;
mod general;
mod help;
mod keybindings;
mod proxy;
mod shell;
mod sync;

impl AxShell {
    pub(crate) fn show_settings_dialog(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.open_settings_page(cx);
    }

    pub(crate) fn render_settings_page(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        use gpui_component::setting::Settings;

        let view = cx.entity();
        let general_page = general::settings_general_page(&view, self, cx);
        let custom_theme_page = custom::settings_custom_page(&view, self, cx);
        shell::settings_page_shell(
            view.clone(),
            &self.focus_handle,
            Settings::new("settings")
                .sidebar_width(px(180.))
                .sidebar_style(div().bg(cx.theme().background).style())
                .page(general_page)
                .page(custom_theme_page)
                .page(sync::settings_sync_page(&view, self))
                .page(proxy::settings_proxy_page(&view, self))
                .page(keybindings::settings_keybindings_page(
                    &view,
                    &self.config,
                    self.recording_action.as_deref(),
                    self.keybind_error.as_ref(),
                ))
                .page(help::settings_help_page())
                .page(about::settings_about_page()),
        )
    }
}
