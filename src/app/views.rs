use crate::app::resizable::{h_resizable, resizable_panel, v_resizable};
use gpui::{
    AnyElement, Context, ElementId, Focusable as _, FontWeight, Hsla, InteractiveElement as _,
    IntoElement, MouseButton, MouseDownEvent, ParentElement as _, PathBuilder, Pixels, Render,
    StatefulInteractiveElement as _, Styled as _, Window, canvas, div, point,
    prelude::FluentBuilder as _, px, rems, uniform_list,
};
use gpui_component::{
    ActiveTheme, Disableable as _, ElementExt, Icon, IconName, InteractiveElementExt as _, Root,
    Selectable as _, Sizable as _, Size,
    button::{Button, ButtonVariants as _},
    checkbox::Checkbox,
    h_flex,
    input::Input,
    menu::{ContextMenuExt as _, PopupMenuItem},
    progress::Progress,
    scroll::{Scrollbar, ScrollbarShow},
    tab::{Tab, TabBar},
    text::{TextView, TextViewStyle},
    v_flex,
};
use rust_i18n::t;

use crate::{
    AxShell, PaneLayout,
    app::actions::sftp::is_editable_text_file,
    app::constants::{
        COLLAPSED_SIDEBAR_WIDTH, SIDEBAR_WIDTH, TERMINAL_KEY_CONTEXT, WORKSPACE_TAB_MAX_WIDTH,
    },
    app::{
        LocalFileEntry, SftpContextMenuTarget, SftpSortColumn, SftpTransferTab, SortDirection,
        WorkspacePage,
    },
    sftp::format_mtime,
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
