use crate::app::resizable::{h_resizable, resizable_panel, v_resizable};
use gpui::{
    Context, ElementId, Focusable as _, FontWeight, Hsla, InteractiveElement as _, IntoElement,
    MouseButton, MouseDownEvent, ParentElement as _, PathBuilder, Pixels, Render,
    StatefulInteractiveElement as _, Styled as _, Window, canvas, div, point,
    prelude::FluentBuilder as _, px, rems, uniform_list,
};
use gpui_component::{
    ActiveTheme, Disableable as _, ElementExt, Icon, IconName, InteractiveElementExt as _, Root,
    Sizable as _, Size,
    button::{Button, ButtonVariants as _},
    checkbox::Checkbox,
    h_flex,
    input::Input,
    menu::{ContextMenuExt as _, PopupMenuItem},
    progress::Progress,
    scroll::{ScrollableElement as _, Scrollbar, ScrollbarShow},
    tab::{Tab, TabBar},
    v_flex,
};
use rust_i18n::t;

use crate::{
    AxShell, PaneLayout,
    app::WorkspacePage,
    app::constants::{COLLAPSED_SIDEBAR_WIDTH, SIDEBAR_WIDTH, TERMINAL_KEY_CONTEXT},
    sftp::format_mtime,
    sftp::ops::is_editable_text_file,
    system::format_bytes,
    terminal::{self, TabKind, TerminalTab},
};

mod helpers;
mod layout;
mod monitoring;
mod sftp_panel;
mod sidebar;
mod tab_bar;
mod terminal_panel;
