use gpui::{
    Anchor, Context, Focusable as _, FontWeight, InteractiveElement as _, MouseButton,
    ParentElement as _, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _, px, rems,
};
use gpui_component::{
    ActiveTheme as _, Disableable as _, IconName, Sizable as _, WindowExt as _,
    button::{Button, ButtonVariants as _},
    dialog::Dialog,
    h_flex,
    input::Input,
    menu::{DropdownMenu as _, PopupMenuItem},
    progress::Progress,
    scroll::{Scrollbar, ScrollbarShow},
    switch::Switch,
    v_flex,
};
use rust_i18n::t;

use crate::{AxShell, session::config::AuthMethod, system::format_bytes};

mod delete_confirm;
mod selector;
mod settings;
mod ssh;
mod transfers;
