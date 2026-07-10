use gpui::Keystroke;

pub fn encode_key(
    keystroke: &Keystroke,
    app_cursor_mode: bool,
    option_as_meta: bool,
) -> Option<Vec<u8>> {
    encode_key_for_platform(
        keystroke,
        app_cursor_mode,
        option_as_meta,
        terminal_platform(),
    )
}

fn encode_key_for_platform(
    keystroke: &Keystroke,
    app_cursor_mode: bool,
    option_as_meta: bool,
    platform: TerminalPlatform,
) -> Option<Vec<u8>> {
    zed_like_to_esc_str(keystroke, app_cursor_mode, option_as_meta, platform)
        .map(|text| text.into_owned().into_bytes())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TerminalPlatform {
    Mac,
    Other,
}

fn terminal_platform() -> TerminalPlatform {
    if cfg!(target_os = "macos") {
        TerminalPlatform::Mac
    } else {
        TerminalPlatform::Other
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TerminalModifiers {
    None,
    Alt,
    Ctrl,
    Shift,
    CtrlShift,
    Other,
}

impl TerminalModifiers {
    fn new(ks: &Keystroke) -> Self {
        match (
            ks.modifiers.alt,
            ks.modifiers.control,
            ks.modifiers.shift,
            ks.modifiers.platform,
        ) {
            (false, false, false, false) => Self::None,
            (true, false, false, false) => Self::Alt,
            (false, true, false, false) => Self::Ctrl,
            (false, false, true, false) => Self::Shift,
            (false, true, true, false) => Self::CtrlShift,
            _ => Self::Other,
        }
    }

    fn any(&self) -> bool {
        !matches!(self, Self::None)
    }
}

fn zed_like_to_esc_str(
    keystroke: &Keystroke,
    app_cursor_mode: bool,
    option_as_meta: bool,
    platform: TerminalPlatform,
) -> Option<std::borrow::Cow<'static, str>> {
    let modifiers = TerminalModifiers::new(keystroke);
    let key = keystroke.key.to_ascii_lowercase();

    if let Some(esc) = platform_text_navigation_escape(key.as_str(), keystroke, platform) {
        return Some(esc.into());
    }

    let manual_esc_str = match (key.as_str(), &modifiers) {
        ("tab", TerminalModifiers::None) => Some("\x09"),
        ("tab", TerminalModifiers::Shift) => Some("\x1b[Z"),
        ("escape", TerminalModifiers::None) => Some("\x1b"),
        ("enter", TerminalModifiers::None) => Some("\x0d"),
        ("enter", TerminalModifiers::Shift) => Some("\x0a"),
        ("enter", TerminalModifiers::Alt) => Some("\x1b\x0d"),
        ("backspace", TerminalModifiers::None) => Some("\x7f"),
        ("backspace", TerminalModifiers::Ctrl) => Some("\x08"),
        ("backspace", TerminalModifiers::Alt) => Some("\x1b\x7f"),
        ("backspace", TerminalModifiers::Shift) => Some("\x7f"),
        ("space", TerminalModifiers::Ctrl) => Some("\x00"),
        ("home", TerminalModifiers::None) if app_cursor_mode => Some("\x1bOH"),
        ("home", TerminalModifiers::None) if !app_cursor_mode => Some("\x1b[H"),
        ("end", TerminalModifiers::None) if app_cursor_mode => Some("\x1bOF"),
        ("end", TerminalModifiers::None) if !app_cursor_mode => Some("\x1b[F"),
        ("up", TerminalModifiers::None) if app_cursor_mode => Some("\x1bOA"),
        ("up", TerminalModifiers::None) if !app_cursor_mode => Some("\x1b[A"),
        ("down", TerminalModifiers::None) if app_cursor_mode => Some("\x1bOB"),
        ("down", TerminalModifiers::None) if !app_cursor_mode => Some("\x1b[B"),
        ("right", TerminalModifiers::None) if app_cursor_mode => Some("\x1bOC"),
        ("right", TerminalModifiers::None) if !app_cursor_mode => Some("\x1b[C"),
        ("left", TerminalModifiers::None) if app_cursor_mode => Some("\x1bOD"),
        ("left", TerminalModifiers::None) if !app_cursor_mode => Some("\x1b[D"),
        ("insert", TerminalModifiers::None) => Some("\x1b[2~"),
        ("delete", TerminalModifiers::None) => Some("\x1b[3~"),
        ("pageup", TerminalModifiers::None) => Some("\x1b[5~"),
        ("pagedown", TerminalModifiers::None) => Some("\x1b[6~"),
        ("a", TerminalModifiers::Ctrl) | ("A", TerminalModifiers::CtrlShift) => Some("\x01"),
        ("b", TerminalModifiers::Ctrl) | ("B", TerminalModifiers::CtrlShift) => Some("\x02"),
        ("c", TerminalModifiers::Ctrl) | ("C", TerminalModifiers::CtrlShift) => Some("\x03"),
        ("d", TerminalModifiers::Ctrl) | ("D", TerminalModifiers::CtrlShift) => Some("\x04"),
        ("e", TerminalModifiers::Ctrl) | ("E", TerminalModifiers::CtrlShift) => Some("\x05"),
        ("f", TerminalModifiers::Ctrl) | ("F", TerminalModifiers::CtrlShift) => Some("\x06"),
        ("g", TerminalModifiers::Ctrl) | ("G", TerminalModifiers::CtrlShift) => Some("\x07"),
        ("h", TerminalModifiers::Ctrl) | ("H", TerminalModifiers::CtrlShift) => Some("\x08"),
        ("i", TerminalModifiers::Ctrl) | ("I", TerminalModifiers::CtrlShift) => Some("\x09"),
        ("j", TerminalModifiers::Ctrl) | ("J", TerminalModifiers::CtrlShift) => Some("\x0a"),
        ("k", TerminalModifiers::Ctrl) | ("K", TerminalModifiers::CtrlShift) => Some("\x0b"),
        ("l", TerminalModifiers::Ctrl) | ("L", TerminalModifiers::CtrlShift) => Some("\x0c"),
        ("m", TerminalModifiers::Ctrl) | ("M", TerminalModifiers::CtrlShift) => Some("\x0d"),
        ("n", TerminalModifiers::Ctrl) | ("N", TerminalModifiers::CtrlShift) => Some("\x0e"),
        ("o", TerminalModifiers::Ctrl) | ("O", TerminalModifiers::CtrlShift) => Some("\x0f"),
        ("p", TerminalModifiers::Ctrl) | ("P", TerminalModifiers::CtrlShift) => Some("\x10"),
        ("q", TerminalModifiers::Ctrl) | ("Q", TerminalModifiers::CtrlShift) => Some("\x11"),
        ("r", TerminalModifiers::Ctrl) | ("R", TerminalModifiers::CtrlShift) => Some("\x12"),
        ("s", TerminalModifiers::Ctrl) | ("S", TerminalModifiers::CtrlShift) => Some("\x13"),
        ("t", TerminalModifiers::Ctrl) | ("T", TerminalModifiers::CtrlShift) => Some("\x14"),
        ("u", TerminalModifiers::Ctrl) | ("U", TerminalModifiers::CtrlShift) => Some("\x15"),
        ("v", TerminalModifiers::Ctrl) | ("V", TerminalModifiers::CtrlShift) => Some("\x16"),
        ("w", TerminalModifiers::Ctrl) | ("W", TerminalModifiers::CtrlShift) => Some("\x17"),
        ("x", TerminalModifiers::Ctrl) | ("X", TerminalModifiers::CtrlShift) => Some("\x18"),
        ("y", TerminalModifiers::Ctrl) | ("Y", TerminalModifiers::CtrlShift) => Some("\x19"),
        ("z", TerminalModifiers::Ctrl) | ("Z", TerminalModifiers::CtrlShift) => Some("\x1a"),
        ("@", TerminalModifiers::Ctrl) => Some("\x00"),
        ("[", TerminalModifiers::Ctrl) => Some("\x1b"),
        ("\\", TerminalModifiers::Ctrl) => Some("\x1c"),
        ("]", TerminalModifiers::Ctrl) => Some("\x1d"),
        ("^", TerminalModifiers::Ctrl) => Some("\x1e"),
        ("_", TerminalModifiers::Ctrl) => Some("\x1f"),
        ("?", TerminalModifiers::Ctrl) => Some("\x7f"),
        _ => None,
    };
    if let Some(esc) = manual_esc_str {
        return Some(esc.into());
    }

    if modifiers.any() {
        let modifier_code = modifier_code(keystroke);
        let modified = match key.as_str() {
            "up" => Some(format!("\x1b[1;{}A", modifier_code)),
            "down" => Some(format!("\x1b[1;{}B", modifier_code)),
            "right" => Some(format!("\x1b[1;{}C", modifier_code)),
            "left" => Some(format!("\x1b[1;{}D", modifier_code)),
            "insert" => Some(format!("\x1b[2;{}~", modifier_code)),
            "pageup" => Some(format!("\x1b[5;{}~", modifier_code)),
            "pagedown" => Some(format!("\x1b[6;{}~", modifier_code)),
            "end" => Some(format!("\x1b[1;{}F", modifier_code)),
            "home" => Some(format!("\x1b[1;{}H", modifier_code)),
            _ => None,
        };
        if let Some(esc) = modified {
            return Some(esc.into());
        }
    }

    if !cfg!(target_os = "macos") || option_as_meta {
        let is_alt_lowercase_ascii =
            modifiers == TerminalModifiers::Alt && keystroke.key.is_ascii();
        let is_alt_uppercase_ascii =
            keystroke.modifiers.alt && keystroke.modifiers.shift && keystroke.key.is_ascii();
        if is_alt_lowercase_ascii || is_alt_uppercase_ascii {
            let key = if is_alt_uppercase_ascii {
                keystroke.key.to_ascii_uppercase()
            } else {
                keystroke.key.clone()
            };
            return Some(format!("\x1b{}", key).into());
        }
    }

    if let Some(text) = &keystroke.key_char {
        return Some(text.clone().into());
    }

    if keystroke.key.len() == 1 {
        return Some(keystroke.key.clone().into());
    }

    None
}

fn platform_text_navigation_escape(
    key: &str,
    keystroke: &Keystroke,
    platform: TerminalPlatform,
) -> Option<&'static str> {
    if platform != TerminalPlatform::Mac {
        return None;
    }

    let modifiers = &keystroke.modifiers;
    if modifiers.platform
        && !modifiers.alt
        && !modifiers.control
        && !modifiers.shift
        && !modifiers.function
    {
        return match key {
            "left" => Some("\x01"),
            "right" => Some("\x05"),
            _ => None,
        };
    }

    if modifiers.alt
        && !modifiers.platform
        && !modifiers.control
        && !modifiers.shift
        && !modifiers.function
    {
        return match key {
            "left" => Some("\x1bb"),
            "right" => Some("\x1bf"),
            _ => None,
        };
    }

    None
}

fn modifier_code(keystroke: &Keystroke) -> u32 {
    let mut modifier_code = 0;
    if keystroke.modifiers.shift {
        modifier_code |= 1;
    }
    if keystroke.modifiers.alt {
        modifier_code |= 1 << 1;
    }
    if keystroke.modifiers.control {
        modifier_code |= 1 << 2;
    }
    modifier_code + 1
}

#[cfg(test)]
mod tests {
    use gpui::{Keystroke, Modifiers};

    use super::{TerminalPlatform, encode_key, encode_key_for_platform};

    fn key(key: &str, modifiers: Modifiers) -> Keystroke {
        Keystroke {
            modifiers,
            key: key.to_string(),
            key_char: None,
        }
    }

    #[test]
    fn encodes_ctrl_arrow_as_xterm_modified_cursor() {
        assert_eq!(
            encode_key(&key("left", Modifiers::control()), false, false).as_deref(),
            Some(b"\x1b[1;5D".as_slice())
        );
        assert_eq!(
            encode_key(&key("right", Modifiers::control()), false, false).as_deref(),
            Some(b"\x1b[1;5C".as_slice())
        );
    }

    #[test]
    fn encodes_macos_command_arrow_as_readline_line_navigation() {
        assert_eq!(
            encode_key_for_platform(
                &key("left", Modifiers::command()),
                false,
                false,
                TerminalPlatform::Mac,
            )
            .as_deref(),
            Some(b"\x01".as_slice())
        );
        assert_eq!(
            encode_key_for_platform(
                &key("right", Modifiers::command()),
                false,
                false,
                TerminalPlatform::Mac,
            )
            .as_deref(),
            Some(b"\x05".as_slice())
        );
    }

    #[test]
    fn encodes_macos_option_arrow_as_readline_word_navigation() {
        assert_eq!(
            encode_key_for_platform(
                &key("left", Modifiers::alt()),
                false,
                false,
                TerminalPlatform::Mac,
            )
            .as_deref(),
            Some(b"\x1bb".as_slice())
        );
        assert_eq!(
            encode_key_for_platform(
                &key("right", Modifiers::alt()),
                false,
                false,
                TerminalPlatform::Mac,
            )
            .as_deref(),
            Some(b"\x1bf".as_slice())
        );
    }

    #[test]
    fn keeps_non_macos_alt_arrow_as_xterm_modified_cursor() {
        assert_eq!(
            encode_key_for_platform(
                &key("left", Modifiers::alt()),
                false,
                false,
                TerminalPlatform::Other,
            )
            .as_deref(),
            Some(b"\x1b[1;3D".as_slice())
        );
        assert_eq!(
            encode_key_for_platform(
                &key("right", Modifiers::alt()),
                false,
                false,
                TerminalPlatform::Other,
            )
            .as_deref(),
            Some(b"\x1b[1;3C".as_slice())
        );
    }
}
