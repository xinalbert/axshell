use gpui::{
    Anchor, Context, ElementId, Focusable as _, FontWeight, InteractiveElement as _, MouseButton,
    ParentElement as _, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _, px, rems,
};
use gpui_component::{
    ActiveTheme as _, Disableable as _, IconName, Sizable as _, WindowExt as _,
    button::{Button, ButtonVariants as _},
    checkbox::Checkbox,
    dialog::Dialog,
    h_flex,
    input::Input,
    menu::{DropdownMenu as _, PopupMenuItem},
    progress::Progress,
    scroll::{Scrollbar, ScrollbarShow},
    switch::Switch,
    text::{TextView, TextViewStyle},
    v_flex,
};
use rust_i18n::t;

use crate::{AxShell, monitoring::format_bytes, session::AuthMethod};

mod delete_confirm;
mod selector;
mod settings;
mod sftp_close_confirm;
mod ssh;
mod transfers;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DialogKind {
    SessionSelector,
    Transfers,
    NewSsh,
    SftpCloseConfirm,
}

fn escape_markdown_text(text: impl AsRef<str>) -> String {
    let text = text.as_ref();
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '.'
            | '!' | '|' | '>' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn selectable_plain_text(id: impl Into<ElementId>, text: impl AsRef<str>) -> TextView {
    TextView::markdown(id, escape_markdown_text(text))
        .style(TextViewStyle::default().paragraph_gap(rems(0.0)))
        .selectable(true)
}
