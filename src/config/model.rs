use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{platform::x_server, session::Session, sftp::Transfer};

pub(super) fn default_custom_font_brightness() -> f32 {
    1.0
}

pub(super) fn default_custom_theme_name() -> String {
    "Custom Theme".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum SavedWindowBounds {
    Fullscreen {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    Maximized {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    Windowed {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub(crate) enum TitleBarStyle {
    Native,
    #[default]
    Integrated,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum CursorStyle {
    #[default]
    Default,
    Blink,
    Beam,
    BeamBlink,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CustomThemeModeConfig {
    #[serde(default)]
    pub(crate) base_theme_name: String,
    #[serde(default)]
    pub(crate) overrides: BTreeMap<String, String>,
    #[serde(default = "default_custom_font_brightness")]
    pub(crate) font_brightness: f32,
}

impl Default for CustomThemeModeConfig {
    fn default() -> Self {
        Self {
            base_theme_name: String::new(),
            overrides: BTreeMap::new(),
            font_brightness: default_custom_font_brightness(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CustomThemeConfig {
    #[serde(default = "default_custom_theme_name")]
    pub(crate) theme_name: String,
    #[serde(default)]
    pub(crate) light: CustomThemeModeConfig,
    #[serde(default)]
    pub(crate) dark: CustomThemeModeConfig,
}

impl Default for CustomThemeConfig {
    fn default() -> Self {
        Self {
            theme_name: default_custom_theme_name(),
            light: CustomThemeModeConfig::default(),
            dark: CustomThemeModeConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ConfigFile {
    #[serde(default = "default_follow_system_theme")]
    pub(super) follow_system_theme: bool,
    #[serde(default)]
    pub(super) theme_mode: String,
    #[serde(default)]
    pub(super) light_theme_name: String,
    #[serde(default)]
    pub(super) dark_theme_name: String,
    #[serde(default = "default_locale")]
    pub(super) locale: String,
    #[serde(default = "default_terminal_font_size")]
    pub(super) terminal_font_size: f32,
    #[serde(default = "default_ui_font_size")]
    pub(super) ui_font_size: f32,
    #[serde(default)]
    pub(super) custom_primary_color: String,
    #[serde(default)]
    pub(super) custom_background_color: String,
    #[serde(default = "default_custom_font_brightness")]
    pub(super) custom_font_brightness: f32,
    #[serde(default = "default_custom_theme_name")]
    pub(super) custom_theme_name: String,
    #[serde(default)]
    pub(super) custom_theme: CustomThemeConfig,
    #[serde(default)]
    pub(super) right_click_copy_paste: bool,
    #[serde(default)]
    pub(super) keyword_highlight: bool,
    #[serde(default = "default_ssh_connect_retry_count")]
    pub(super) ssh_connect_retry_count: u32,
    #[serde(default = "default_ssh_connect_retry_delays_ms")]
    pub(super) ssh_connect_retry_delays_ms: Vec<u64>,
    #[serde(default = "default_ui_font_family")]
    pub(super) ui_font_family: String,
    #[serde(default = "default_terminal_font_family")]
    pub(super) terminal_font_family: String,
    #[serde(default)]
    pub(super) title_bar_style: TitleBarStyle,
    #[serde(default)]
    pub(super) cursor_style: CursorStyle,
    #[serde(default)]
    pub(super) sessions: Vec<Session>,
    #[serde(default)]
    pub(super) window_bounds: Option<SavedWindowBounds>,
    #[serde(default)]
    pub(super) workspace_panels: Option<Vec<f32>>,
    #[serde(default)]
    pub(super) body_panels: Option<Vec<f32>>,
    #[serde(default)]
    pub(super) transfers: Vec<Transfer>,
    #[serde(default)]
    pub(super) show_hidden_files: bool,
    #[serde(default = "default_sftp_transfer_close_behavior")]
    pub(super) sftp_transfer_close_behavior: String,
    #[serde(default = "default_deep_sleep_after_minutes")]
    pub(super) deep_sleep_after_minutes: u32,
    #[serde(default)]
    pub(super) lock_layout: bool,
    #[serde(default)]
    pub(super) color_inactive_tabs: bool,
    #[serde(default = "default_monitoring_position")]
    pub(super) monitoring_position: String,
    #[serde(default = "default_show_monitoring_dashboard")]
    pub(super) show_monitoring_dashboard: bool,
    #[serde(default)]
    pub(super) sidebar_collapsed: bool,
    #[serde(default)]
    pub(super) sftp_panel_minimized: bool,
    #[serde(default)]
    pub(super) key_bindings: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub(super) sync_endpoint: String,
    #[serde(default)]
    pub(super) sync_username: String,
    #[serde(default)]
    pub(super) sync_etag: Option<String>,
    #[serde(default)]
    pub(super) sync_device_id: String,
    #[serde(default)]
    pub(super) sync_backend: String,
    #[serde(default)]
    pub(super) sync_etag_backend: String,
    #[serde(default)]
    pub(super) sync_s3_endpoint: String,
    #[serde(default = "default_s3_region")]
    pub(super) sync_s3_region: String,
    #[serde(default)]
    pub(super) sync_s3_bucket: String,
    #[serde(default = "default_s3_object_key")]
    pub(super) sync_s3_object_key: String,
    #[serde(default)]
    pub(super) use_proxy: bool,
    #[serde(default = "default_read_env_proxy")]
    pub(super) read_env_proxy: bool,
    #[serde(default = "default_global_proxy_type")]
    pub(super) global_proxy_type: String,
    #[serde(default)]
    pub(super) global_proxy_host: String,
    #[serde(default)]
    pub(super) global_proxy_port: Option<u16>,
    #[serde(default)]
    pub(super) global_proxy_user: String,
    #[serde(default)]
    pub(super) global_proxy_password: String,
    #[serde(default)]
    pub(super) x11_forwarding_enabled: bool,
    #[serde(default = "default_x11_launch_xquartz")]
    pub(super) x11_launch_xquartz: bool,
    #[serde(default = "default_xquartz_app_path")]
    pub(super) xquartz_app_path: String,
}

pub(super) const CUSTOM_FONT_BRIGHTNESS_MIN: f32 = 0.6;
pub(super) const CUSTOM_FONT_BRIGHTNESS_MAX: f32 = 1.2;

pub(super) fn normalize_local_x_server_app_path(value: &str) -> String {
    let value = value.trim();
    if !value.is_empty() {
        return value.to_string();
    }
    x_server::default_app_path()
}

pub(super) fn normalize_deep_sleep_after_minutes(value: u32) -> u32 {
    match value {
        0 | 1 | 5 | 15 | 30 => value,
        _ => default_deep_sleep_after_minutes(),
    }
}

pub(super) fn normalize_sftp_transfer_close_behavior(value: &str) -> String {
    match value {
        "keep_page_open" | "background" | "cancel_disconnect" => value.to_string(),
        _ => default_sftp_transfer_close_behavior(),
    }
}

pub(super) fn normalize_ssh_connect_retry_count(value: u32) -> u32 {
    value.min(10)
}

pub(super) fn normalize_ssh_connect_retry_delays_ms(mut delays: Vec<u64>, count: u32) -> Vec<u64> {
    let target_len = count as usize;
    if target_len == 0 {
        return Vec::new();
    }

    delays.retain(|delay| *delay > 0);
    if delays.is_empty() {
        delays = default_ssh_connect_retry_delays_ms();
    }

    let mut normalized = Vec::with_capacity(target_len);
    for index in 0..target_len {
        let delay = delays
            .get(index)
            .copied()
            .or_else(|| delays.last().copied())
            .unwrap_or(500)
            .clamp(100, 60_000);
        normalized.push(delay);
    }
    normalized
}

pub(super) fn effective_title_bar_style(style: TitleBarStyle) -> TitleBarStyle {
    if cfg!(target_os = "macos") {
        style
    } else {
        TitleBarStyle::Native
    }
}

pub(super) fn default_read_env_proxy() -> bool {
    true
}

pub(super) fn default_ssh_connect_retry_count() -> u32 {
    2
}

pub(super) fn default_ssh_connect_retry_delays_ms() -> Vec<u64> {
    vec![500, 1500]
}

pub(super) fn default_global_proxy_type() -> String {
    "socks5".to_string()
}

fn default_xquartz_app_path() -> String {
    x_server::default_app_path()
}

fn default_x11_launch_xquartz() -> bool {
    x_server::should_launch_by_default()
}

pub(super) fn default_monitoring_position() -> String {
    "Sidebar".to_string()
}

pub(super) fn default_show_monitoring_dashboard() -> bool {
    true
}

pub(super) fn default_sftp_transfer_close_behavior() -> String {
    "ask".to_string()
}

pub(super) fn default_deep_sleep_after_minutes() -> u32 {
    5
}

fn default_s3_region() -> String {
    "us-east-1".to_string()
}

fn default_s3_object_key() -> String {
    "ax_shell-sync.json".to_string()
}

fn default_follow_system_theme() -> bool {
    true
}

fn default_locale() -> String {
    "system".to_string()
}

pub(super) fn default_terminal_font_size() -> f32 {
    18.0
}

pub(super) fn default_ui_font_size() -> f32 {
    14.0
}

pub(super) fn default_ui_font_family() -> String {
    ".SystemUIFont".to_string()
}

fn default_terminal_font_family() -> String {
    "Maple Mono NF CN".to_string()
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            follow_system_theme: default_follow_system_theme(),
            theme_mode: String::new(),
            light_theme_name: String::new(),
            dark_theme_name: String::new(),
            locale: default_locale(),
            terminal_font_size: default_terminal_font_size(),
            ui_font_size: default_ui_font_size(),
            custom_primary_color: String::new(),
            custom_background_color: String::new(),
            custom_font_brightness: default_custom_font_brightness(),
            custom_theme_name: default_custom_theme_name(),
            custom_theme: CustomThemeConfig::default(),
            right_click_copy_paste: false,
            keyword_highlight: false,
            ssh_connect_retry_count: default_ssh_connect_retry_count(),
            ssh_connect_retry_delays_ms: default_ssh_connect_retry_delays_ms(),
            ui_font_family: default_ui_font_family(),
            terminal_font_family: default_terminal_font_family(),
            title_bar_style: TitleBarStyle::default(),
            cursor_style: CursorStyle::default(),
            sessions: Vec::new(),
            window_bounds: None,
            workspace_panels: None,
            body_panels: None,
            transfers: Vec::new(),
            show_hidden_files: false,
            sftp_transfer_close_behavior: default_sftp_transfer_close_behavior(),
            deep_sleep_after_minutes: default_deep_sleep_after_minutes(),
            lock_layout: false,
            color_inactive_tabs: false,
            monitoring_position: default_monitoring_position(),
            show_monitoring_dashboard: default_show_monitoring_dashboard(),
            sidebar_collapsed: false,
            sftp_panel_minimized: false,
            key_bindings: std::collections::HashMap::new(),
            sync_endpoint: String::new(),
            sync_username: String::new(),
            sync_etag: None,
            sync_device_id: String::new(),
            sync_backend: String::new(),
            sync_etag_backend: String::new(),
            sync_s3_endpoint: String::new(),
            sync_s3_region: default_s3_region(),
            sync_s3_bucket: String::new(),
            sync_s3_object_key: default_s3_object_key(),
            use_proxy: false,
            read_env_proxy: true,
            global_proxy_type: default_global_proxy_type(),
            global_proxy_host: String::new(),
            global_proxy_port: None,
            global_proxy_user: String::new(),
            global_proxy_password: String::new(),
            x11_forwarding_enabled: false,
            x11_launch_xquartz: default_x11_launch_xquartz(),
            xquartz_app_path: default_xquartz_app_path(),
        }
    }
}
