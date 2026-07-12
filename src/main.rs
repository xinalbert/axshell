#![windows_subsystem = "windows"]

use gpui_component_assets::Assets;

mod app;
mod backend;
mod config;
mod diagnostics;
mod events;
mod monitoring;
mod platform;
mod session;
mod sftp;
mod sync;
mod terminal;

rust_i18n::i18n!("locales", fallback = "en");

gpui::actions!(ax_shell_terminal, [TerminalTabKey, TerminalBacktabKey]);

pub(crate) use app::keybinding_recorder::{
    ClosePane, Copy, ExportSavedSessions, FocusPaneDown, FocusPaneLeft, FocusPaneRight,
    FocusPaneUp, ImportSavedSessions, NewSsh, NextTab, OpenSearch, OpenSession, OpenSettings,
    OpenTransfers, Paste, PrevTab, SplitPaneDown, SplitPaneLeft, SplitPaneRight, SplitPaneUp,
    ToggleSftpZoom, ToggleSidebar,
};

pub(crate) use app::{AxShell, PaneLayout, SelectorEntry, SftpContextMenuState, TabGroup};

fn main() {
    app::startup::install_crash_hook();
    app::startup::sync_macos_launch_environment();
    let _log_guard = app::startup::init_logging();

    #[cfg(target_os = "macos")]
    let app = gpui_platform::application()
        .with_assets(Assets)
        .with_quit_mode(gpui::QuitMode::LastWindowClosed);

    #[cfg(not(target_os = "macos"))]
    let app = gpui_platform::application().with_assets(Assets);

    app.on_reopen(|cx| {
        if cx.windows().is_empty() {
            app::startup::open_main_window(cx);
        }
    });
    app.run(move |cx| {
        gpui_component::init(cx);
        app::startup::bind_workspace_keys(cx);
        app::app_menu::install(cx);
        app::theme::load_embedded_themes(cx);
        app::theme::load_user_themes(cx);
        if let Err(err) = app::theme::load_fonts(cx) {
            tracing::warn!(
                component = "theme",
                operation = "load_fonts",
                error = %diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to load embedded fonts"
            );
        }
        app::startup::open_main_window(cx);
    });
}
