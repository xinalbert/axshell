use gpui::{App, Menu, MenuItem, OsAction, SystemMenuType};

use crate::{
    ClosePane, Copy, FocusPaneDown, FocusPaneLeft, FocusPaneRight, FocusPaneUp, NewSsh, NextTab,
    OpenSearch, OpenSession, OpenSettings, OpenTransfers, Paste, PrevTab, SplitPaneDown,
    SplitPaneLeft, SplitPaneRight, SplitPaneUp, ToggleSftpZoom, ToggleSidebar,
};

gpui::actions!(ax_shell_app_menu, [Quit]);

pub(crate) fn install(cx: &mut App) {
    cx.on_action(quit);
    refresh(cx);
}

pub(crate) fn refresh(cx: &mut App) {
    cx.set_menus(app_menus());
}

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

fn app_menus() -> [Menu; 6] {
    [
        Menu::new("AxShell").items([
            MenuItem::os_submenu("Services", SystemMenuType::Services),
            MenuItem::separator(),
            MenuItem::action("Settings", OpenSettings),
            MenuItem::separator(),
            MenuItem::action("Quit AxShell", Quit),
        ]),
        Menu::new("File").items([
            MenuItem::action("Open Session", OpenSession),
            MenuItem::action("New SSH Connection", NewSsh),
            MenuItem::separator(),
            MenuItem::action("Transfers", OpenTransfers),
        ]),
        Menu::new("Edit").items([
            MenuItem::os_action("Copy", Copy, OsAction::Copy),
            MenuItem::os_action("Paste", Paste, OsAction::Paste),
            MenuItem::separator(),
            MenuItem::action("Search", OpenSearch),
        ]),
        Menu::new("View").items([
            MenuItem::action("Toggle Sidebar", ToggleSidebar),
            MenuItem::action("Open SFTP Page", ToggleSftpZoom),
            MenuItem::separator(),
            MenuItem::action("Transfers", OpenTransfers),
        ]),
        Menu::new("Pane").items([
            MenuItem::submenu(Menu::new("Focus Pane").items([
                MenuItem::action("Left", FocusPaneLeft),
                MenuItem::action("Right", FocusPaneRight),
                MenuItem::action("Up", FocusPaneUp),
                MenuItem::action("Down", FocusPaneDown),
            ])),
            MenuItem::submenu(Menu::new("Split Pane").items([
                MenuItem::action("Left", SplitPaneLeft),
                MenuItem::action("Right", SplitPaneRight),
                MenuItem::action("Up", SplitPaneUp),
                MenuItem::action("Down", SplitPaneDown),
            ])),
            MenuItem::separator(),
            MenuItem::action("Close Pane", ClosePane),
        ]),
        Menu::new("Window").items([
            MenuItem::action("Previous Tab", PrevTab),
            MenuItem::action("Next Tab", NextTab),
        ]),
    ]
}
