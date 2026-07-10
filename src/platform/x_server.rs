pub(crate) fn default_app_path() -> String {
    #[cfg(target_os = "macos")]
    {
        return "/Applications/Utilities/XQuartz.app".to_string();
    }
    #[cfg(target_os = "windows")]
    {
        let mut candidates = Vec::new();
        if let Ok(program_files) = std::env::var("ProgramFiles") {
            candidates.push(
                std::path::PathBuf::from(&program_files)
                    .join("VcXsrv")
                    .join("vcxsrv.exe"),
            );
            candidates.push(
                std::path::PathBuf::from(&program_files)
                    .join("Xming")
                    .join("Xming.exe"),
            );
        }
        if let Ok(program_files_x86) = std::env::var("ProgramFiles(x86)") {
            candidates.push(
                std::path::PathBuf::from(&program_files_x86)
                    .join("VcXsrv")
                    .join("vcxsrv.exe"),
            );
            candidates.push(
                std::path::PathBuf::from(&program_files_x86)
                    .join("Xming")
                    .join("Xming.exe"),
            );
        }
        return candidates
            .into_iter()
            .find(|path| path.exists())
            .unwrap_or_else(|| std::path::PathBuf::from(r"C:\Program Files\VcXsrv\vcxsrv.exe"))
            .to_string_lossy()
            .to_string();
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        String::new()
    }
}

pub(crate) fn default_display() -> String {
    display_from_env().unwrap_or_else(default_display_fallback)
}

fn display_from_env() -> Option<String> {
    std::env::var("DISPLAY")
        .ok()
        .map(|display| display.trim().to_string())
        .filter(|display| !display.is_empty())
}

fn default_display_fallback() -> String {
    if cfg!(target_os = "windows") {
        "127.0.0.1:0".to_string()
    } else {
        ":0".to_string()
    }
}

pub(crate) fn resolve_display(_path: &str, _launch_local_x_server: bool) -> String {
    let display = default_display();

    #[cfg(target_os = "windows")]
    {
        if _launch_local_x_server && windows_server_supports_display_arg(_path) {
            return select_available_windows_display(&display);
        }
    }

    display
}

#[cfg(target_os = "windows")]
pub(crate) fn launch_args(path: &str, display: &str) -> Vec<String> {
    if windows_server_supports_display_arg(path) {
        vec![
            windows_display_arg(display),
            "-multiwindow".to_string(),
            "-clipboard".to_string(),
            "-ac".to_string(),
        ]
    } else {
        Vec::new()
    }
}

pub(crate) fn should_launch_by_default() -> bool {
    cfg!(any(target_os = "macos", target_os = "windows"))
}

#[cfg(target_os = "windows")]
fn windows_server_supports_display_arg(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.ends_with("vcxsrv.exe") || lower.ends_with("xming.exe")
}

#[cfg(target_os = "windows")]
fn select_available_windows_display(preferred_display: &str) -> String {
    if !display_uses_localhost(preferred_display) {
        return preferred_display.to_string();
    }

    let Some(start_display) = display_number(preferred_display) else {
        return preferred_display.to_string();
    };

    for offset in 0..64u16 {
        let Some(display_number) = start_display.checked_add(offset) else {
            break;
        };
        let Some(port) = 6000u16.checked_add(display_number) else {
            break;
        };
        if windows_local_port_available(port) {
            return display_with_number(preferred_display, display_number);
        }
    }

    preferred_display.to_string()
}

#[cfg(target_os = "windows")]
fn windows_local_port_available(port: u16) -> bool {
    std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).is_ok()
}

#[cfg(target_os = "windows")]
fn windows_display_arg(display: &str) -> String {
    format!(":{}", display_number(display).unwrap_or(0))
}

#[cfg(target_os = "windows")]
fn display_number(display: &str) -> Option<u16> {
    let (_, rest) = display.rsplit_once(':')?;
    let number = rest.split('.').next().unwrap_or(rest);
    number.parse::<u16>().ok()
}

#[cfg(target_os = "windows")]
fn display_uses_localhost(display: &str) -> bool {
    if display.starts_with(':') {
        return true;
    }
    let Some((host, _)) = display.rsplit_once(':') else {
        return false;
    };
    let host = host.trim_start_matches("tcp/").trim();
    host.is_empty() || host.eq_ignore_ascii_case("localhost") || host == "127.0.0.1"
}

#[cfg(target_os = "windows")]
fn display_with_number(display: &str, display_number: u16) -> String {
    let screen_suffix = display
        .rsplit_once(':')
        .and_then(|(_, rest)| rest.split_once('.').map(|(_, screen)| format!(".{screen}")))
        .unwrap_or_default();

    if display.starts_with(':') {
        format!(":{display_number}{screen_suffix}")
    } else if let Some((host, _)) = display.rsplit_once(':') {
        format!("{host}:{display_number}{screen_suffix}")
    } else {
        format!("127.0.0.1:{display_number}{screen_suffix}")
    }
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::{
        display_number, display_with_number, launch_args, windows_display_arg,
        windows_server_supports_display_arg,
    };

    #[test]
    fn windows_display_helpers_parse_and_replace_display_numbers() {
        assert_eq!(display_number("127.0.0.1:0"), Some(0));
        assert_eq!(display_number(":12.0"), Some(12));
        assert_eq!(display_with_number("127.0.0.1:0", 3), "127.0.0.1:3");
        assert_eq!(display_with_number(":12.1", 7), ":7.1");
        assert_eq!(windows_display_arg("127.0.0.1:5"), ":5");
    }

    #[test]
    fn windows_launch_args_follow_selected_display() {
        assert!(windows_server_supports_display_arg(
            r"C:\Program Files\VcXsrv\vcxsrv.exe"
        ));
        assert_eq!(
            launch_args(r"C:\Program Files\VcXsrv\vcxsrv.exe", "127.0.0.1:2",),
            vec![
                ":2".to_string(),
                "-multiwindow".to_string(),
                "-clipboard".to_string(),
                "-ac".to_string(),
            ]
        );
    }
}
