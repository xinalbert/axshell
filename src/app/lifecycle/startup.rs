use anyhow::{Context as _, Result, anyhow};
use gpui::{App, AppContext as _, Bounds, WindowOptions, point, px, size};
use gpui_component::Root;
use std::path::PathBuf;
use std::sync::Once;

use crate::AxShell;
use crate::app::constants::ISSUES_URL;
use crate::config::ConfigStore;

const INSTANCE_KIND_ENV: &str = "AX_SHELL_INSTANCE_KIND";
const INSTANCE_APP_ID_ENV: &str = "AX_SHELL_APP_ID";
const DEV_RELOAD_INSTANCE_KIND: &str = "dev-reload";
const LOG_FILES_TO_KEEP: usize = 48;
static CRASH_HOOK: Once = Once::new();

fn current_instance_kind() -> Option<String> {
    std::env::var(INSTANCE_KIND_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn current_window_title() -> &'static str {
    if matches!(
        current_instance_kind().as_deref(),
        Some(DEV_RELOAD_INSTANCE_KIND)
    ) {
        "AxShell [dev]"
    } else {
        "AxShell"
    }
}

fn current_window_app_id() -> Option<String> {
    std::env::var(INSTANCE_APP_ID_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn should_force_app_activation() -> bool {
    matches!(
        current_instance_kind().as_deref(),
        Some(DEV_RELOAD_INSTANCE_KIND)
    )
}

pub(crate) fn bind_workspace_keys(cx: &mut gpui::App) {
    let config = ConfigStore::load().unwrap_or_else(|_| ConfigStore::in_memory());
    crate::app::keybinding_recorder::bind_workspace_keys_from_config(cx, &config);
}

fn app_config_dir() -> PathBuf {
    ConfigStore::config_root_dir_path().unwrap_or_else(|_| PathBuf::from("."))
}

pub(crate) fn runtime_log_dir() -> PathBuf {
    app_config_dir().join("log")
}

pub(crate) fn crash_report_dir() -> PathBuf {
    app_config_dir().join("crash")
}

struct LocalMinutelyRoller {
    dir: std::path::PathBuf,
    prefix: String,
    current_minute: u32,
    file: Option<std::fs::File>,
}

impl LocalMinutelyRoller {
    fn new(dir: std::path::PathBuf, prefix: String) -> Self {
        Self {
            dir,
            prefix,
            current_minute: 60,
            file: None,
        }
    }

    fn rollover(&mut self, now: chrono::DateTime<chrono::Local>) -> std::io::Result<()> {
        use chrono::Timelike;
        let minute = now.minute();
        if self.current_minute != minute || self.file.is_none() {
            let filename = format!("{}-{}.log", self.prefix, now.format("%Y-%m-%d-%H-%M"));
            let path = self.dir.join(filename);
            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)?;
            self.file = Some(file);
            self.current_minute = minute;

            // Cleanup old files keeping last 6
            if let Ok(entries) = std::fs::read_dir(&self.dir) {
                let mut files: Vec<_> = entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_name().to_string_lossy().starts_with(&self.prefix))
                    .collect();
                files.sort_by_key(|e| {
                    e.metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                });
                if files.len() > LOG_FILES_TO_KEEP {
                    for file in files.iter().take(files.len() - LOG_FILES_TO_KEEP) {
                        let _ = std::fs::remove_file(file.path());
                    }
                }
            }
        }
        Ok(())
    }
}

impl std::io::Write for LocalMinutelyRoller {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let now = chrono::Local::now();
        let _ = self.rollover(now);
        if let Some(f) = &mut self.file {
            f.write(buf)
        } else {
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(f) = &mut self.file {
            f.flush()
        } else {
            Ok(())
        }
    }
}

pub(crate) fn init_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let log_dir = runtime_log_dir();

    std::fs::create_dir_all(&log_dir).ok();

    let roller = LocalMinutelyRoller::new(log_dir.clone(), "ax_shell".to_string());

    let (non_blocking, _guard) = tracing_appender::non_blocking(roller);
    // Leak the guard so it lives for the entire duration of the app since GPUI's run might not return
    std::mem::forget(_guard);

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("ax_shell=info,warn"));

    let stdout_layer = if cfg!(debug_assertions) {
        Some(
            tracing_subscriber::fmt::layer()
                .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
                .with_target(true),
        )
    } else {
        None
    };

    let file_layer = tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    log_startup_summary(&log_dir);
}

fn log_startup_summary(log_dir: &std::path::Path) {
    let instance_kind = current_instance_kind().unwrap_or_else(|| "default".to_string());
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        os = std::env::consts::OS,
        arch = std::env::consts::ARCH,
        profile,
        instance_kind,
        config_dir = %app_config_dir().display(),
        log_dir = %log_dir.display(),
        crash_dir = %crash_report_dir().display(),
        log_files_to_keep = LOG_FILES_TO_KEEP,
        "AxShell startup"
    );
}

pub(crate) fn install_crash_hook() {
    CRASH_HOOK.call_once(|| {
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let crash_path = write_crash_report(panic_info);

            match &crash_path {
                Some(path) => {
                    tracing::error!(
                        "AxShell crashed; crash report saved to {}. Please report it at {}",
                        path.display(),
                        ISSUES_URL
                    );
                    eprintln!(
                        "AxShell crashed. Crash report saved to: {}\nPlease report it at: {}",
                        path.display(),
                        ISSUES_URL
                    );
                }
                None => {
                    tracing::error!(
                        "AxShell crashed, but writing the crash report failed. Please report it at {}",
                        ISSUES_URL
                    );
                    eprintln!(
                        "AxShell crashed, but writing the crash report failed.\nPlease report it at: {}",
                        ISSUES_URL
                    );
                }
            }

            notify_user_about_crash(crash_path.as_ref());
            previous_hook(panic_info);
        }));
    });
}

fn write_crash_report(panic_info: &std::panic::PanicHookInfo<'_>) -> Option<PathBuf> {
    let crash_dir = crash_report_dir();
    if std::fs::create_dir_all(&crash_dir).is_err() {
        return None;
    }

    let now = chrono::Local::now();
    let path = crash_dir.join(format!(
        "ax_shell-crash-{}.log",
        now.format("%Y-%m-%d-%H-%M-%S")
    ));
    let report = build_crash_report(panic_info, now);

    if std::fs::write(&path, report).is_err() {
        return None;
    }
    cleanup_old_crash_reports(&crash_dir);
    Some(path)
}

fn build_crash_report(
    panic_info: &std::panic::PanicHookInfo<'_>,
    now: chrono::DateTime<chrono::Local>,
) -> String {
    use std::fmt::Write as _;

    let current_thread = std::thread::current();
    let thread_name = current_thread.name().unwrap_or("<unnamed>");
    let panic_payload = panic_payload_to_string(panic_info);
    let location = panic_info
        .location()
        .map(|location| {
            format!(
                "{}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            )
        })
        .unwrap_or_else(|| "<unknown>".to_string());
    let instance_kind = current_instance_kind().unwrap_or_else(|| "default".to_string());
    let backtrace = std::backtrace::Backtrace::force_capture();

    let mut report = String::new();
    let _ = writeln!(report, "AxShell crash report");
    let _ = writeln!(report, "======================");
    let _ = writeln!(report, "time: {}", now.to_rfc3339());
    let _ = writeln!(report, "version: {}", env!("CARGO_PKG_VERSION"));
    let _ = writeln!(
        report,
        "target: {}-{}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let _ = writeln!(report, "instance_kind: {instance_kind}");
    let _ = writeln!(report, "thread: {thread_name}");
    let _ = writeln!(report, "location: {location}");
    let _ = writeln!(report, "panic: {panic_payload}");
    let _ = writeln!(report, "runtime_log_dir: {}", runtime_log_dir().display());
    let _ = writeln!(report, "feedback: {ISSUES_URL}");
    let _ = writeln!(report);
    let _ = writeln!(report, "backtrace:");
    let _ = writeln!(report, "{backtrace}");
    report
}

fn panic_payload_to_string(panic_info: &std::panic::PanicHookInfo<'_>) -> String {
    if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
        (*message).to_string()
    } else if let Some(message) = panic_info.payload().downcast_ref::<String>() {
        message.clone()
    } else {
        "<non-string panic payload>".to_string()
    }
}

fn notify_user_about_crash(crash_path: Option<&PathBuf>) {
    let crash_path = crash_path
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "<failed to write crash report>".to_string());
    let description = format!(
        "AxShell crashed.\n\nCrash report:\n{crash_path}\n\nPlease create an issue and attach this file:\n{ISSUES_URL}"
    );

    let _ = std::panic::catch_unwind(|| {
        let _ = rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("AxShell crashed")
            .set_description(description)
            .set_buttons(rfd::MessageButtons::Ok)
            .show();
    });
}

fn cleanup_old_crash_reports(crash_dir: &std::path::Path) {
    let Ok(entries) = std::fs::read_dir(crash_dir) else {
        return;
    };
    let mut files: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("ax_shell-crash-")
        })
        .collect();
    files.sort_by_key(|entry| {
        entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    if files.len() <= 20 {
        return;
    }
    for file in files.iter().take(files.len() - 20) {
        let _ = std::fs::remove_file(file.path());
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn sync_macos_launch_environment() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());
    let Ok(output) = std::process::Command::new(&shell)
        .args(["-l", "-c", "env -0"])
        .output()
    else {
        return;
    };
    if !output.status.success() {
        return;
    }

    for entry in output.stdout.split(|b| *b == 0) {
        if entry.is_empty() {
            continue;
        }
        let Some(eq) = entry.iter().position(|b| *b == b'=') else {
            continue;
        };
        let Ok(key) = std::str::from_utf8(&entry[..eq]) else {
            continue;
        };
        let Ok(value) = std::str::from_utf8(&entry[eq + 1..]) else {
            continue;
        };

        let should_import = matches!(
            key,
            "PATH"
                | "MANPATH"
                | "INFOPATH"
                | "LANG"
                | "LC_ALL"
                | "LC_CTYPE"
                | "SHELL"
                | "HOME"
                | "HOMEBREW_PREFIX"
                | "HOMEBREW_CELLAR"
                | "HOMEBREW_REPOSITORY"
                | "HTTP_PROXY"
                | "HTTPS_PROXY"
                | "ALL_PROXY"
                | "http_proxy"
                | "https_proxy"
                | "all_proxy"
        ) || key.starts_with("LC_");

        if should_import {
            unsafe {
                std::env::set_var(key, value);
            }
        }
    }
}

fn read_proxy_from_env() -> Option<(String, String, Option<u16>, String, String)> {
    let vars = [
        "ALL_PROXY",
        "all_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy",
    ];
    for var in vars {
        if let Ok(val) = std::env::var(var) {
            if val.is_empty() {
                continue;
            }
            if let Ok(url) = reqwest::Url::parse(&val) {
                let scheme = url.scheme();
                let proxy_type = match scheme {
                    "socks5" | "socks5h" => "socks5".to_string(),
                    "http" | "https" => "http".to_string(),
                    _ => "socks5".to_string(),
                };
                let host = url.host_str().unwrap_or("").to_string();
                let port = url.port();
                let user = url.username().to_string();
                let password = url.password().unwrap_or("").to_string();
                return Some((proxy_type, host, port, user, password));
            }
        }
    }
    None
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn sync_macos_launch_environment() {}

#[cfg(target_os = "macos")]
pub(crate) fn launch_local_x_server_app(path: &str) -> Result<String> {
    let path = path.trim();
    if path.is_empty() {
        return Err(anyhow!("local X server app path is empty"));
    }
    let app_path = std::path::Path::new(path);
    if !app_path.exists() {
        return Err(anyhow!(
            "local X server app not found at {}",
            app_path.display()
        ));
    }
    std::process::Command::new("open")
        .arg("-g")
        .arg(app_path)
        .spawn()
        .with_context(|| format!("launch local X server at {}", app_path.display()))?;
    Ok(crate::platform::x_server::default_display())
}

#[cfg(target_os = "windows")]
pub(crate) fn launch_local_x_server_app(path: &str) -> Result<String> {
    let path = path.trim();
    if path.is_empty() {
        return Err(anyhow!("local X server executable path is empty"));
    }
    let app_path = std::path::Path::new(path);
    if !app_path.exists() {
        return Err(anyhow!(
            "local X server executable not found at {}",
            app_path.display()
        ));
    }
    let display = crate::platform::x_server::resolve_display(path, true);
    let mut command = std::process::Command::new(app_path);
    for arg in crate::platform::x_server::launch_args(path, &display) {
        command.arg(arg);
    }
    command
        .spawn()
        .with_context(|| format!("launch local X server at {}", app_path.display()))?;
    Ok(display)
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub(crate) fn launch_local_x_server_app(path: &str) -> Result<String> {
    let path = path.trim();
    if path.is_empty() {
        return Ok(crate::platform::x_server::default_display());
    }
    let app_path = std::path::Path::new(path);
    if !app_path.exists() {
        return Err(anyhow!(
            "local X server executable not found at {}",
            app_path.display()
        ));
    }
    std::process::Command::new(app_path)
        .spawn()
        .with_context(|| format!("launch local X server at {}", app_path.display()))?;
    Ok(crate::platform::x_server::default_display())
}

pub(crate) fn open_main_window(cx: &mut App) {
    let config = ConfigStore::load().unwrap_or_else(|_| ConfigStore::in_memory());
    let title_bar_style = config.effective_title_bar_style();

    let _ = crate::backend::proxy::ENV_PROXY.get_or_init(|| {
        read_proxy_from_env().map(|(proxy_type, host, port, user, password)| {
            tracing::info!(
                "[proxy] Loaded proxy configuration from environment: type={}, host={}, port={:?}, user={}",
                proxy_type,
                host,
                port,
                user
            );
            crate::backend::proxy::EnvProxy {
                proxy_type,
                host,
                port,
                user,
                pass: password,
            }
        })
    });

    let mut window_options = WindowOptions::default();
    window_options.app_id = current_window_app_id();

    if title_bar_style == crate::config::TitleBarStyle::Integrated {
        window_options.titlebar = Some(gpui::TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(gpui::point(px(9.0), px(9.0))),
        });
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            // Use app-controlled drag zones so tab content inside the integrated titlebar
            // does not fall back to platform-native window dragging.
            window_options.is_movable = false;
        }
    }

    #[cfg(not(target_os = "macos"))]
    if let Ok(img) = image::load_from_memory(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/icons/terminal_icon_all_formats/terminal_icon_256.png"
    ))) {
        window_options.icon = Some(std::sync::Arc::new(img.into_rgba8()));
    }

    if let Some(bounds) = config.window_bounds() {
        window_options.window_bounds = Some(match bounds {
            crate::config::SavedWindowBounds::Fullscreen {
                x,
                y,
                width,
                height,
            } => gpui::WindowBounds::Fullscreen(Bounds::new(
                point(px(*x), px(*y)),
                size(px(*width), px(*height)),
            )),
            crate::config::SavedWindowBounds::Maximized {
                x,
                y,
                width,
                height,
            } => gpui::WindowBounds::Maximized(Bounds::new(
                point(px(*x), px(*y)),
                size(px(*width), px(*height)),
            )),
            crate::config::SavedWindowBounds::Windowed {
                x,
                y,
                width,
                height,
            } => gpui::WindowBounds::Windowed(Bounds::new(
                point(px(*x), px(*y)),
                size(px(*width), px(*height)),
            )),
        });
    } else if let Some(display) = cx.displays().first().cloned() {
        let display_bounds = display.bounds();
        let width = display_bounds.size.width * 0.8;
        let height = display_bounds.size.height * 0.9;

        let x = display_bounds.origin.x + (display_bounds.size.width - width) / 2.0;

        #[cfg(target_os = "macos")]
        let y = display_bounds.origin.y;
        #[cfg(not(target_os = "macos"))]
        let y = display_bounds.origin.y + (display_bounds.size.height - height) / 2.0;

        window_options.window_bounds = Some(gpui::WindowBounds::Windowed(Bounds::new(
            point(x, y),
            size(width, height),
        )));
    }

    cx.open_window(window_options, |window, cx| {
        window.set_window_title(current_window_title());
        gpui_component::Theme::sync_system_appearance(Some(window), cx);
        let view = cx.new(|cx| AxShell::new(window, cx));

        tracing::info!("[ui] main application window opened");
        let focus_handle = view.read(cx).focus_handle.clone();
        let deferred_focus_handle = focus_handle.clone();
        let should_activate = should_force_app_activation();
        window.defer(cx, move |window, cx| {
            if should_activate {
                cx.activate(true);
            }
            window.activate_window();
            window.focus(&deferred_focus_handle, cx);
        });

        let view_clone = view.clone();
        window.on_window_should_close(cx, move |window: &mut gpui::Window, cx: &mut gpui::App| {
            let handle = window.window_handle();
            if !cx.windows().contains(&handle) {
                tracing::warn!(
                    "[ui] window not found in app during close, skipping save layout state."
                );
                return true;
            }
            let _ = view_clone.update(cx, |app, _| app.shutdown_all_backends());
            view_clone.read(cx).save_layout_state(window, cx);
            true
        });

        cx.new(|cx| Root::new(view, window, cx))
    })
    .expect("failed to open window");
}
