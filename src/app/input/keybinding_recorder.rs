use gpui::{
    Action as _, App, Entity, IntoElement, KeyBinding, KeyDownEvent, Keystroke, Unbind, prelude::*,
};
use gpui_component::{
    Sizable,
    button::{Button, ButtonVariants},
    kbd::Kbd,
    setting::{SettingField, SettingGroup, SettingItem},
};
use rust_i18n::t;

use crate::{AxShell, config::ConfigStore};

gpui::actions!(
    ax_shell_workspace,
    [
        OpenSettings,
        OpenSession,
        OpenTransfers,
        NewSsh,
        ImportSavedSessions,
        ExportSavedSessions,
        OpenSearch,
        PrevTab,
        NextTab,
        ToggleSidebar,
        ToggleSftpZoom,
        FocusPaneLeft,
        FocusPaneRight,
        FocusPaneUp,
        FocusPaneDown,
        SplitPaneLeft,
        SplitPaneRight,
        SplitPaneUp,
        SplitPaneDown,
        ClosePane,
        Copy,
        Paste
    ]
);

pub struct KeybindingsPage;

#[derive(Clone, Copy)]
enum DefaultKeystroke {
    Secondary(&'static str),
    Literal(&'static str),
    Platform {
        macos: Option<&'static str>,
        other: Option<&'static str>,
    },
}

#[derive(Clone, Copy)]
pub(crate) struct KeybindingAction {
    id: &'static str,
    label_key: &'static str,
    default: DefaultKeystroke,
}

pub(crate) const CONFIGURABLE_ACTIONS: &[KeybindingAction] = &[
    KeybindingAction {
        id: "OpenSettings",
        label_key: "settings_open_settings",
        default: DefaultKeystroke::Secondary(","),
    },
    KeybindingAction {
        id: "OpenSession",
        label_key: "settings_open_session",
        default: DefaultKeystroke::Secondary("o"),
    },
    KeybindingAction {
        id: "OpenTransfers",
        label_key: "settings_open_transfers",
        default: DefaultKeystroke::Secondary("t"),
    },
    KeybindingAction {
        id: "NewSsh",
        label_key: "settings_new_ssh",
        default: DefaultKeystroke::Secondary("n"),
    },
    KeybindingAction {
        id: "OpenSearch",
        label_key: "settings_open_search",
        default: DefaultKeystroke::Secondary("f"),
    },
    KeybindingAction {
        id: "PrevTab",
        label_key: "settings_prev_tab",
        default: DefaultKeystroke::Platform {
            macos: Some("cmd-shift-["),
            other: Some("ctrl-shift-tab"),
        },
    },
    KeybindingAction {
        id: "NextTab",
        label_key: "settings_next_tab",
        default: DefaultKeystroke::Platform {
            macos: Some("cmd-shift-]"),
            other: Some("ctrl-tab"),
        },
    },
    KeybindingAction {
        id: "ToggleSidebar",
        label_key: "settings_toggle_sidebar",
        default: DefaultKeystroke::Secondary("s"),
    },
    KeybindingAction {
        id: "ToggleSftpZoom",
        label_key: "settings_toggle_sftp_zoom",
        default: DefaultKeystroke::Secondary("m"),
    },
    KeybindingAction {
        id: "FocusPaneLeft",
        label_key: "settings_focus_pane_left",
        default: DefaultKeystroke::Secondary("h"),
    },
    KeybindingAction {
        id: "FocusPaneRight",
        label_key: "settings_focus_pane_right",
        default: DefaultKeystroke::Secondary("l"),
    },
    KeybindingAction {
        id: "FocusPaneUp",
        label_key: "settings_focus_pane_up",
        default: DefaultKeystroke::Secondary("k"),
    },
    KeybindingAction {
        id: "FocusPaneDown",
        label_key: "settings_focus_pane_down",
        default: DefaultKeystroke::Secondary("j"),
    },
    KeybindingAction {
        id: "SplitPaneLeft",
        label_key: "settings_split_pane_left",
        default: DefaultKeystroke::Secondary("shift-h"),
    },
    KeybindingAction {
        id: "SplitPaneRight",
        label_key: "settings_split_pane_right",
        default: DefaultKeystroke::Secondary("shift-l"),
    },
    KeybindingAction {
        id: "SplitPaneUp",
        label_key: "settings_split_pane_up",
        default: DefaultKeystroke::Secondary("shift-k"),
    },
    KeybindingAction {
        id: "SplitPaneDown",
        label_key: "settings_split_pane_down",
        default: DefaultKeystroke::Secondary("shift-j"),
    },
    KeybindingAction {
        id: "ClosePane",
        label_key: "settings_close_pane",
        default: DefaultKeystroke::Secondary("w"),
    },
    KeybindingAction {
        id: "Copy",
        label_key: "settings_copy",
        default: DefaultKeystroke::Platform {
            macos: Some("cmd-c"),
            other: Some("ctrl-shift-c"),
        },
    },
    KeybindingAction {
        id: "Paste",
        label_key: "settings_paste",
        default: DefaultKeystroke::Platform {
            macos: Some("cmd-v"),
            other: Some("ctrl-shift-v"),
        },
    },
    KeybindingAction {
        id: "TerminalSendTab",
        label_key: "settings_terminal_send_tab",
        default: DefaultKeystroke::Literal("tab"),
    },
    KeybindingAction {
        id: "TerminalSendBacktab",
        label_key: "settings_terminal_send_backtab",
        default: DefaultKeystroke::Literal("shift-tab"),
    },
    KeybindingAction {
        id: "TerminalOpenSession",
        label_key: "settings_terminal_open_session",
        default: DefaultKeystroke::Secondary("shift-o"),
    },
    KeybindingAction {
        id: "TerminalCopySelection",
        label_key: "settings_terminal_copy_selection",
        default: DefaultKeystroke::Platform {
            macos: None,
            other: Some("ctrl-c"),
        },
    },
    KeybindingAction {
        id: "TerminalPasteClipboard",
        label_key: "settings_terminal_paste_clipboard",
        default: DefaultKeystroke::Platform {
            macos: None,
            other: Some("ctrl-v"),
        },
    },
    KeybindingAction {
        id: "TerminalFocusPaneLeft",
        label_key: "settings_terminal_focus_pane_left",
        default: DefaultKeystroke::Literal("alt-h"),
    },
    KeybindingAction {
        id: "TerminalFocusPaneRight",
        label_key: "settings_terminal_focus_pane_right",
        default: DefaultKeystroke::Literal("alt-l"),
    },
    KeybindingAction {
        id: "TerminalFocusPaneUp",
        label_key: "settings_terminal_focus_pane_up",
        default: DefaultKeystroke::Literal("alt-k"),
    },
    KeybindingAction {
        id: "TerminalFocusPaneDown",
        label_key: "settings_terminal_focus_pane_down",
        default: DefaultKeystroke::Literal("alt-j"),
    },
    KeybindingAction {
        id: "TerminalSplitPaneLeft",
        label_key: "settings_terminal_split_pane_left",
        default: DefaultKeystroke::Literal("alt-shift-h"),
    },
    KeybindingAction {
        id: "TerminalSplitPaneRight",
        label_key: "settings_terminal_split_pane_right",
        default: DefaultKeystroke::Literal("alt-shift-l"),
    },
    KeybindingAction {
        id: "TerminalSplitPaneUp",
        label_key: "settings_terminal_split_pane_up",
        default: DefaultKeystroke::Literal("alt-shift-k"),
    },
    KeybindingAction {
        id: "TerminalSplitPaneDown",
        label_key: "settings_terminal_split_pane_down",
        default: DefaultKeystroke::Literal("alt-shift-j"),
    },
    KeybindingAction {
        id: "TerminalCloseTab",
        label_key: "settings_terminal_close_tab",
        default: DefaultKeystroke::Literal("alt-q"),
    },
];

const KEYBINDING_GROUPS: &[(&str, &[&str])] = &[
    (
        "settings_group_keybind_general",
        &[
            "OpenSettings",
            "OpenSession",
            "OpenTransfers",
            "NewSsh",
            "OpenSearch",
            "PrevTab",
            "NextTab",
            "Copy",
            "Paste",
        ],
    ),
    (
        "settings_group_keybind_zoom",
        &["ToggleSidebar", "ToggleSftpZoom"],
    ),
    (
        "settings_group_keybind_focus",
        &[
            "FocusPaneLeft",
            "FocusPaneRight",
            "FocusPaneUp",
            "FocusPaneDown",
        ],
    ),
    (
        "settings_group_keybind_panel",
        &[
            "SplitPaneLeft",
            "SplitPaneRight",
            "SplitPaneUp",
            "SplitPaneDown",
            "ClosePane",
        ],
    ),
    (
        "settings_group_keybind_terminal",
        &[
            "TerminalSendTab",
            "TerminalSendBacktab",
            "TerminalOpenSession",
            "TerminalCopySelection",
            "TerminalPasteClipboard",
            "TerminalFocusPaneLeft",
            "TerminalFocusPaneRight",
            "TerminalFocusPaneUp",
            "TerminalFocusPaneDown",
            "TerminalSplitPaneLeft",
            "TerminalSplitPaneRight",
            "TerminalSplitPaneUp",
            "TerminalSplitPaneDown",
            "TerminalCloseTab",
        ],
    ),
];

impl DefaultKeystroke {
    fn value(self) -> Option<String> {
        match self {
            Self::Secondary(suffix) => Some(format!("{}-{}", default_modifier(), suffix)),
            Self::Literal(keystroke) => Some(keystroke.to_string()),
            Self::Platform { macos, other } => {
                let keystroke = if cfg!(target_os = "macos") {
                    macos
                } else {
                    other
                };
                keystroke.map(str::to_string)
            }
        }
    }
}

pub(crate) fn default_modifier() -> &'static str {
    if cfg!(target_os = "macos") {
        "cmd"
    } else {
        "ctrl"
    }
}

pub(crate) fn default_keystroke(action_id: &str) -> Option<String> {
    CONFIGURABLE_ACTIONS
        .iter()
        .find(|action| action.id == action_id)
        .and_then(|action| action.default.value())
}

pub(crate) fn configured_keystroke(config: &ConfigStore, action_id: &str) -> Option<String> {
    config
        .key_bindings()
        .get(action_id)
        .cloned()
        .or_else(|| default_keystroke(action_id))
}

pub(crate) fn event_matches_action(
    config: &ConfigStore,
    action_id: &str,
    event: &KeyDownEvent,
) -> bool {
    let Some(configured) = configured_keystroke(config, action_id) else {
        return false;
    };
    if default_keystroke(action_id).as_deref() == Some(configured.as_str())
        && matches_default_action_event(action_id, event)
    {
        return true;
    }
    normalize_recorded_keystroke(event)
        .is_some_and(|keystroke| keystroke_strings_match(&keystroke, &configured))
}

fn matches_default_action_event(action_id: &str, event: &KeyDownEvent) -> bool {
    match action_id {
        "PrevTab" => {
            if cfg!(target_os = "macos") {
                matches_macos_brace_shortcut(event, "[", "{")
            } else {
                matches_non_macos_tab_shortcut(event, true)
            }
        }
        "NextTab" => {
            if cfg!(target_os = "macos") {
                matches_macos_brace_shortcut(event, "]", "}")
            } else {
                matches_non_macos_tab_shortcut(event, false)
            }
        }
        _ => false,
    }
}

fn matches_macos_brace_shortcut(event: &KeyDownEvent, base_key: &str, shifted_key: &str) -> bool {
    event.keystroke.modifiers.secondary()
        && event.keystroke.modifiers.shift
        && !event.keystroke.modifiers.alt
        && !event.keystroke.modifiers.control
        && !event.keystroke.modifiers.function
        && (event.keystroke.key == base_key
            || event.keystroke.key == shifted_key
            || event.keystroke.key_char.as_deref() == Some(base_key)
            || event.keystroke.key_char.as_deref() == Some(shifted_key))
}

fn matches_non_macos_tab_shortcut(event: &KeyDownEvent, require_shift: bool) -> bool {
    event.keystroke.modifiers.secondary()
        && event.keystroke.modifiers.shift == require_shift
        && !event.keystroke.modifiers.alt
        && !event.keystroke.modifiers.platform
        && !event.keystroke.modifiers.function
        && event.keystroke.key.eq_ignore_ascii_case("tab")
}

pub(crate) fn normalize_recorded_keystroke(event: &KeyDownEvent) -> Option<String> {
    let key = event.keystroke.key.trim();
    let key = if key.is_empty() {
        if event.keystroke.modifiers.shift {
            "shift"
        } else if event.keystroke.modifiers.control {
            "control"
        } else if event.keystroke.modifiers.alt {
            "alt"
        } else if event.keystroke.modifiers.platform {
            "platform"
        } else if event.keystroke.modifiers.function {
            "function"
        } else {
            return None;
        }
    } else {
        key
    };

    let key = match (event.keystroke.modifiers.shift, key) {
        (true, "{") => "[",
        (true, "}") => "]",
        _ => key,
    };

    let mut parts = Vec::new();
    if event.keystroke.modifiers.control {
        parts.push("ctrl".to_string());
    }
    if event.keystroke.modifiers.alt {
        parts.push("alt".to_string());
    }
    if event.keystroke.modifiers.platform {
        parts.push("cmd".to_string());
    }
    if event.keystroke.modifiers.shift {
        parts.push("shift".to_string());
    }
    if event.keystroke.modifiers.function {
        parts.push("fn".to_string());
    }

    parts.push(key.to_ascii_lowercase());
    let keystroke = parts.join("-");
    Keystroke::parse(&keystroke).ok().map(|_| keystroke)
}

pub(crate) fn format_keystroke(keystroke: &str) -> String {
    Keystroke::parse(keystroke)
        .map(|stroke| Kbd::format(&stroke))
        .unwrap_or_else(|_| keystroke.to_string())
}

fn keystroke_strings_match(left: &str, right: &str) -> bool {
    if left == right {
        return true;
    }

    match (Keystroke::parse(left), Keystroke::parse(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

pub(crate) fn bind_workspace_keys_from_config(cx: &mut App, config: &ConfigStore) {
    bind_workspace_actions(cx, config);
}

/// Unbind all workspace keybindings (used when entering keybinding settings to prevent interference).
pub(crate) fn unbind_all_workspace_keys(cx: &mut App, config: &ConfigStore) {
    let mut bindings = Vec::new();

    macro_rules! unbind_action {
        ($id:literal, $action:expr) => {
            let default = default_keystroke($id).expect("workspace action has default key");
            let configured = configured_keystroke(config, $id).unwrap_or_else(|| default.clone());
            let action_name = $action.name();

            // Unbind both the default and configured keystroke
            bindings.push(KeyBinding::new(&default, Unbind(action_name.into()), None));
            if configured != default {
                bindings.push(KeyBinding::new(
                    &configured,
                    Unbind(action_name.into()),
                    None,
                ));
            }
        };
    }
    macro_rules! unbind_terminal_action {
        ($id:literal, $action:expr) => {
            let default = default_keystroke($id).expect("terminal action has default key");
            let configured = configured_keystroke(config, $id).unwrap_or_else(|| default.clone());
            let action_name = $action.name();
            let context = Some(crate::app::constants::TERMINAL_KEY_CONTEXT);

            bindings.push(KeyBinding::new(
                &default,
                Unbind(action_name.into()),
                context,
            ));
            if configured != default {
                bindings.push(KeyBinding::new(
                    &configured,
                    Unbind(action_name.into()),
                    context,
                ));
            }
        };
    }

    unbind_action!("OpenSettings", crate::OpenSettings);
    unbind_action!("OpenSession", crate::OpenSession);
    unbind_action!("OpenTransfers", crate::OpenTransfers);
    unbind_action!("NewSsh", crate::NewSsh);
    unbind_action!("OpenSearch", crate::OpenSearch);
    unbind_action!("PrevTab", crate::PrevTab);
    unbind_action!("NextTab", crate::NextTab);
    unbind_action!("ToggleSidebar", crate::ToggleSidebar);
    unbind_action!("ToggleSftpZoom", crate::ToggleSftpZoom);
    unbind_action!("FocusPaneLeft", crate::FocusPaneLeft);
    unbind_action!("FocusPaneRight", crate::FocusPaneRight);
    unbind_action!("FocusPaneUp", crate::FocusPaneUp);
    unbind_action!("FocusPaneDown", crate::FocusPaneDown);
    unbind_action!("SplitPaneLeft", crate::SplitPaneLeft);
    unbind_action!("SplitPaneRight", crate::SplitPaneRight);
    unbind_action!("SplitPaneUp", crate::SplitPaneUp);
    unbind_action!("SplitPaneDown", crate::SplitPaneDown);
    unbind_action!("ClosePane", crate::ClosePane);
    unbind_action!("Copy", crate::Copy);
    unbind_action!("Paste", crate::Paste);
    unbind_terminal_action!("TerminalSendTab", crate::TerminalTabKey);
    unbind_terminal_action!("TerminalSendBacktab", crate::TerminalBacktabKey);

    cx.bind_keys(bindings);
}

/// Check if a keystroke conflicts with any other action's binding.
/// Returns Some((conflicting_action_id, label)) if there is a conflict.
pub(crate) fn find_conflict(
    config: &ConfigStore,
    current_action_id: &str,
    new_keystroke: &str,
) -> Option<(String, String)> {
    for action in CONFIGURABLE_ACTIONS {
        if action.id == current_action_id {
            continue;
        }
        let existing = configured_keystroke(config, action.id).unwrap_or_default();
        if !existing.is_empty() && keystroke_strings_match(&existing, new_keystroke) {
            return Some((action.id.to_string(), t!(action.label_key).to_string()));
        }
    }
    None
}

fn bind_workspace_actions(cx: &mut App, config: &ConfigStore) {
    let mut bindings = Vec::new();

    macro_rules! bind_action {
        ($id:literal, $action:expr) => {
            let default = default_keystroke($id).expect("workspace action has default key");
            let configured = configured_keystroke(config, $id).unwrap_or_else(|| default.clone());
            let action_name = $action.name();

            if configured != default {
                bindings.push(KeyBinding::new(&default, Unbind(action_name.into()), None));
            }

            bindings.push(KeyBinding::new(&configured, $action, None));
        };
    }
    macro_rules! bind_terminal_action {
        ($id:literal, $action:expr) => {
            let default = default_keystroke($id).expect("terminal action has default key");
            let configured = configured_keystroke(config, $id).unwrap_or_else(|| default.clone());
            let action_name = $action.name();
            let context = Some(crate::app::constants::TERMINAL_KEY_CONTEXT);

            if configured != default {
                bindings.push(KeyBinding::new(
                    &default,
                    Unbind(action_name.into()),
                    context,
                ));
            }

            bindings.push(KeyBinding::new(&configured, $action, context));
        };
    }

    bind_action!("OpenSettings", crate::OpenSettings);
    bind_action!("OpenSession", crate::OpenSession);
    bind_action!("OpenTransfers", crate::OpenTransfers);
    bind_action!("NewSsh", crate::NewSsh);
    bind_action!("OpenSearch", crate::OpenSearch);
    bind_action!("PrevTab", crate::PrevTab);
    bind_action!("NextTab", crate::NextTab);
    bind_action!("ToggleSidebar", crate::ToggleSidebar);
    bind_action!("ToggleSftpZoom", crate::ToggleSftpZoom);
    bind_action!("FocusPaneLeft", crate::FocusPaneLeft);
    bind_action!("FocusPaneRight", crate::FocusPaneRight);
    bind_action!("FocusPaneUp", crate::FocusPaneUp);
    bind_action!("FocusPaneDown", crate::FocusPaneDown);
    bind_action!("SplitPaneLeft", crate::SplitPaneLeft);
    bind_action!("SplitPaneRight", crate::SplitPaneRight);
    bind_action!("SplitPaneUp", crate::SplitPaneUp);
    bind_action!("SplitPaneDown", crate::SplitPaneDown);
    bind_action!("ClosePane", crate::ClosePane);
    bind_action!("Copy", crate::Copy);
    bind_action!("Paste", crate::Paste);
    bind_terminal_action!("TerminalSendTab", crate::TerminalTabKey);
    bind_terminal_action!("TerminalSendBacktab", crate::TerminalBacktabKey);

    cx.bind_keys(bindings);
}

impl KeybindingsPage {
    pub fn render_groups(
        view: &Entity<AxShell>,
        config: &ConfigStore,
        recording_action: Option<&str>,
        keybind_error: Option<&(String, String)>,
    ) -> Vec<SettingGroup> {
        let mut result = Vec::new();

        for &(group_label, action_ids) in KEYBINDING_GROUPS {
            let mut group = SettingGroup::new().title(t!(group_label).to_string());

            for action_id in action_ids {
                let action = CONFIGURABLE_ACTIONS
                    .iter()
                    .find(|a| a.id == *action_id)
                    .expect("action exists");

                let recording = recording_action == Some(action.id);
                let has_error = keybind_error.is_some_and(|(id, _)| id == action.id);
                let error_msg = if has_error {
                    keybind_error.map(|(_, msg)| msg.clone())
                } else {
                    None
                };

                let keystroke = configured_keystroke(config, action.id).unwrap_or_default();

                let btn_label = if recording {
                    t!("press_new_key").to_string()
                } else if keystroke.is_empty() {
                    t!("none").to_string()
                } else {
                    format_keystroke(&keystroke)
                };

                let mut item = SettingItem::new(
                    t!(action.label_key).to_string(),
                    SettingField::render({
                        let view = view.clone();
                        let action_id = action.id.to_string();
                        move |_, _window, _cx| {
                            Button::new(gpui::SharedString::from(format!("keybind-{action_id}")))
                                .label(btn_label.clone())
                                .small()
                                .when(recording, |this| this.primary())
                                .when(has_error, |this| this.danger())
                                .on_click({
                                    let view = view.clone();
                                    let action_id = action_id.clone();
                                    move |_event, window, cx| {
                                        view.update(cx, |this, cx| {
                                            // Clear any previous error when starting new recording
                                            this.keybind_error = None;
                                            this.recording_action = Some(action_id.clone());
                                            this.focus_handle.focus(window, cx);
                                            cx.notify();
                                        });
                                    }
                                })
                                .into_any_element()
                        }
                    }),
                );

                if let Some(msg) = error_msg {
                    item = item.description(msg);
                }

                group = group.item(item);
            }

            result.push(group);
        }

        result
    }
}
