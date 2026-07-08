use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use directories::BaseDirs;
use gpui_component::ThemeMode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AuthMethod {
    Password,
    Key,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SshConnectionMode {
    Default,
    Legacy,
}

impl SshConnectionMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Legacy => "legacy compatibility",
        }
    }
}

pub fn ordered_ssh_connection_modes(
    preferred: Option<SshConnectionMode>,
) -> Vec<SshConnectionMode> {
    let mut modes = Vec::with_capacity(2);
    if let Some(preferred) = preferred {
        modes.push(preferred);
    }
    for mode in [SshConnectionMode::Default, SshConnectionMode::Legacy] {
        if !modes.contains(&mode) {
            modes.push(mode);
        }
    }
    modes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub group_name: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub auth: AuthMethod,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub private_key_path: String,
    #[serde(default)]
    pub private_key_inline: String,
    #[serde(default)]
    pub passphrase: String,
    #[serde(default)]
    pub last_used: Option<String>,
    #[serde(default)]
    pub last_successful_ssh_mode: Option<SshConnectionMode>,
    #[serde(default = "default_global_proxy_type")]
    pub proxy_type: String, // "none", "socks5", "http"
    #[serde(default)]
    pub proxy_host: String,
    #[serde(default)]
    pub proxy_port: Option<u16>,
    #[serde(default)]
    pub proxy_user: String,
    #[serde(default)]
    pub proxy_password: String,
}

impl Session {
    pub fn password(host: String, port: u16, user: String, password: String) -> Self {
        let name = format!("{user}@{host}");
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            group_name: String::new(),
            host,
            port,
            user,
            auth: AuthMethod::Password,
            password,
            private_key_path: String::new(),
            private_key_inline: String::new(),
            passphrase: String::new(),
            last_used: None,
            last_successful_ssh_mode: None,
            proxy_type: "none".to_string(),
            proxy_host: String::new(),
            proxy_port: None,
            proxy_user: String::new(),
            proxy_password: String::new(),
        }
    }

    pub fn key(
        host: String,
        port: u16,
        user: String,
        private_key_path: String,
        private_key_inline: String,
        passphrase: String,
    ) -> Self {
        let name = format!("{user}@{host}");
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            group_name: String::new(),
            host,
            port,
            user,
            auth: AuthMethod::Key,
            password: String::new(),
            private_key_path,
            private_key_inline,
            passphrase,
            last_used: None,
            last_successful_ssh_mode: None,
            proxy_type: "none".to_string(),
            proxy_host: String::new(),
            proxy_port: None,
            proxy_user: String::new(),
            proxy_password: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SavedWindowBounds {
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
pub enum TitleBarStyle {
    Native,
    #[default]
    Integrated,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum CursorStyle {
    #[default]
    Default,
    Blink,
    Beam,
    BeamBlink,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomThemeModeConfig {
    #[serde(default)]
    pub base_theme_name: String,
    #[serde(default)]
    pub overrides: BTreeMap<String, String>,
    #[serde(default = "default_custom_font_brightness")]
    pub font_brightness: f32,
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
pub struct CustomThemeConfig {
    #[serde(default = "default_custom_theme_name")]
    pub theme_name: String,
    #[serde(default)]
    pub light: CustomThemeModeConfig,
    #[serde(default)]
    pub dark: CustomThemeModeConfig,
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
pub struct ConfigFile {
    #[serde(default = "default_follow_system_theme")]
    pub follow_system_theme: bool,
    #[serde(default)]
    pub theme_mode: String,
    #[serde(default)]
    pub light_theme_name: String,
    #[serde(default)]
    pub dark_theme_name: String,
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default = "default_terminal_font_size")]
    pub terminal_font_size: f32,
    #[serde(default = "default_ui_font_size")]
    pub ui_font_size: f32,
    #[serde(default)]
    pub custom_primary_color: String,
    #[serde(default)]
    pub custom_background_color: String,
    #[serde(default = "default_custom_font_brightness")]
    pub custom_font_brightness: f32,
    #[serde(default = "default_custom_theme_name")]
    pub custom_theme_name: String,
    #[serde(default)]
    pub custom_theme: CustomThemeConfig,
    #[serde(default)]
    pub right_click_copy_paste: bool,
    #[serde(default)]
    pub keyword_highlight: bool,
    #[serde(default = "default_ui_font_family")]
    pub ui_font_family: String,
    #[serde(default = "default_terminal_font_family")]
    pub terminal_font_family: String,
    #[serde(default)]
    pub title_bar_style: TitleBarStyle,
    #[serde(default)]
    pub cursor_style: CursorStyle,
    #[serde(default)]
    pub sessions: Vec<Session>,
    #[serde(default)]
    pub window_bounds: Option<SavedWindowBounds>,
    #[serde(default)]
    pub workspace_panels: Option<Vec<f32>>,
    #[serde(default)]
    pub body_panels: Option<Vec<f32>>,
    #[serde(default)]
    pub transfers: Vec<crate::terminal::Transfer>,
    #[serde(default)]
    pub show_hidden_files: bool,
    #[serde(default)]
    pub lock_layout: bool,
    #[serde(default = "default_monitoring_position")]
    pub monitoring_position: String,
    #[serde(default = "default_show_monitoring_dashboard")]
    pub show_monitoring_dashboard: bool,
    #[serde(default)]
    pub sidebar_collapsed: bool,
    #[serde(default)]
    pub sftp_panel_minimized: bool,
    #[serde(default)]
    pub key_bindings: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub sync_endpoint: String,
    #[serde(default)]
    pub sync_username: String,
    #[serde(default)]
    pub sync_etag: Option<String>,
    #[serde(default)]
    pub sync_device_id: String,
    #[serde(default)]
    pub sync_backend: String,
    #[serde(default)]
    pub sync_etag_backend: String,
    #[serde(default)]
    pub sync_s3_endpoint: String,
    #[serde(default = "default_s3_region")]
    pub sync_s3_region: String,
    #[serde(default)]
    pub sync_s3_bucket: String,
    #[serde(default = "default_s3_object_key")]
    pub sync_s3_object_key: String,
    #[serde(default)]
    pub use_proxy: bool,
    #[serde(default = "default_read_env_proxy")]
    pub read_env_proxy: bool,
    #[serde(default = "default_global_proxy_type")]
    pub global_proxy_type: String,
    #[serde(default)]
    pub global_proxy_host: String,
    #[serde(default)]
    pub global_proxy_port: Option<u16>,
    #[serde(default)]
    pub global_proxy_user: String,
    #[serde(default)]
    pub global_proxy_password: String,
    #[serde(default)]
    pub x11_forwarding_enabled: bool,
    #[serde(default = "default_x11_launch_xquartz")]
    pub x11_launch_xquartz: bool,
    #[serde(default = "default_xquartz_app_path")]
    pub xquartz_app_path: String,
}

fn default_read_env_proxy() -> bool {
    true
}

fn default_global_proxy_type() -> String {
    "socks5".to_string()
}

pub fn default_xquartz_app_path() -> String {
    default_local_x_server_app_path()
}

fn normalize_local_x_server_app_path(value: &str) -> String {
    let value = value.trim();
    if !value.is_empty() {
        return value.to_string();
    }

    default_local_x_server_app_path()
}

pub fn default_local_x_server_app_path() -> String {
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

pub fn default_local_x_display() -> String {
    std::env::var("DISPLAY")
        .ok()
        .map(|display| display.trim().to_string())
        .filter(|display| !display.is_empty())
        .unwrap_or_else(|| {
            if cfg!(target_os = "windows") {
                "127.0.0.1:0".to_string()
            } else {
                ":0".to_string()
            }
        })
}

#[cfg(target_os = "windows")]
pub fn default_local_x_server_launch_args(path: &str) -> Vec<&'static str> {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with("vcxsrv.exe") || lower.ends_with("xming.exe") {
        vec![":0", "-multiwindow", "-clipboard", "-ac"]
    } else {
        Vec::new()
    }
}

pub fn should_launch_local_x_server_by_default() -> bool {
    cfg!(any(target_os = "macos", target_os = "windows"))
}

pub fn default_x11_launch_xquartz() -> bool {
    should_launch_local_x_server_by_default()
}

fn default_monitoring_position() -> String {
    "Sidebar".to_string()
}

fn default_show_monitoring_dashboard() -> bool {
    true
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

fn default_terminal_font_size() -> f32 {
    18.0
}

fn default_ui_font_size() -> f32 {
    14.0
}

fn default_custom_font_brightness() -> f32 {
    1.0
}

const CUSTOM_FONT_BRIGHTNESS_MIN: f32 = 0.6;
const CUSTOM_FONT_BRIGHTNESS_MAX: f32 = 1.2;

fn default_custom_theme_name() -> String {
    "Custom Theme".to_string()
}

pub fn default_ui_font_family() -> String {
    // ".SystemUIFont" is a GPUI sentinel that resolves to the platform system UI font.
    // This matches gpui-component's own Theme default.
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
            lock_layout: false,
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

pub fn effective_title_bar_style(style: TitleBarStyle) -> TitleBarStyle {
    if cfg!(target_os = "macos") {
        style
    } else {
        TitleBarStyle::Native
    }
}

pub struct ConfigStore {
    path: PathBuf,
    cache: ConfigFile,
}

impl ConfigStore {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create config dir {}", parent.display()))?;
            Self::migrate_legacy_config_if_needed(parent)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(mut perms) = fs::metadata(parent).map(|m| m.permissions()) {
                    perms.set_mode(0o700);
                    let _ = fs::set_permissions(parent, perms);
                }
            }

            let tmp_dir = parent.join("tmp");
            let _ = fs::remove_dir_all(&tmp_dir);
            let _ = fs::create_dir_all(&tmp_dir);
        }

        let mut cache = if path.exists() {
            let raw = fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            match serde_json::from_str::<ConfigFile>(&raw) {
                Ok(cache) => cache,
                Err(err) => {
                    let backup_path = path.with_extension("json.bak");
                    if let Err(backup_err) = fs::write(&backup_path, raw.as_bytes()) {
                        tracing::warn!(
                            "failed to parse config {}; backup to {} also failed: {backup_err:#}; parse error: {err:#}",
                            path.display(),
                            backup_path.display(),
                        );
                    } else {
                        tracing::warn!(
                            "failed to parse config {}; backed up the original to {} and loaded defaults: {err:#}",
                            path.display(),
                            backup_path.display(),
                        );
                    }
                    ConfigFile::default()
                }
            }
        } else {
            ConfigFile::default()
        };

        if cache.sync_device_id.is_empty() {
            cache.sync_device_id = Uuid::new_v4().to_string();
        }
        cache.xquartz_app_path = normalize_local_x_server_app_path(&cache.xquartz_app_path);
        if cache.monitoring_position == "Hidden" {
            cache.show_monitoring_dashboard = false;
            cache.monitoring_position = default_monitoring_position();
        } else if cache.monitoring_position != "Bottom" && cache.monitoring_position != "Sidebar" {
            cache.monitoring_position = default_monitoring_position();
        }
        Ok(Self { path, cache })
    }

    pub fn in_memory() -> Self {
        let cache = ConfigFile {
            sync_device_id: Uuid::new_v4().to_string(),
            ..ConfigFile::default()
        };
        Self {
            path: PathBuf::new(),
            cache,
        }
    }

    pub fn config_root_dir_path() -> Result<PathBuf> {
        Self::config_root_dir()
    }

    fn config_root_dir() -> Result<PathBuf> {
        let dirs = BaseDirs::new().context("could not determine user home directory")?;
        Ok(dirs.home_dir().join(".config").join("ax_shell"))
    }

    fn legacy_config_root_dir() -> Result<PathBuf> {
        let dirs = BaseDirs::new().context("could not determine user home directory")?;
        Ok(dirs.home_dir().join(".config").join("ax_ashell"))
    }

    fn migrate_legacy_config_if_needed(config_root: &Path) -> Result<()> {
        let legacy_root = Self::legacy_config_root_dir()?;
        if !legacy_root.exists() {
            return Ok(());
        }

        let legacy_config = legacy_root.join("sessions.json");
        let current_config = config_root.join("sessions.json");
        if legacy_config.exists() && !current_config.exists() {
            fs::copy(&legacy_config, &current_config).with_context(|| {
                format!(
                    "failed to migrate legacy config {} to {}",
                    legacy_config.display(),
                    current_config.display()
                )
            })?;
            tracing::info!(
                "migrated legacy config from {} to {}",
                legacy_config.display(),
                current_config.display()
            );
        }

        let legacy_themes = legacy_root.join("themes");
        let current_themes = config_root.join("themes");
        if legacy_themes.exists() && !current_themes.exists() {
            copy_dir_recursive(&legacy_themes, &current_themes).with_context(|| {
                format!(
                    "failed to migrate legacy themes {} to {}",
                    legacy_themes.display(),
                    current_themes.display()
                )
            })?;
            tracing::info!(
                "migrated legacy themes from {} to {}",
                legacy_themes.display(),
                current_themes.display()
            );
        }

        Ok(())
    }

    pub fn theme_dir_path() -> Result<PathBuf> {
        Ok(Self::config_root_dir()?.join("themes"))
    }

    fn config_path() -> Result<PathBuf> {
        Ok(Self::config_root_dir()?.join("sessions.json"))
    }

    pub fn sessions(&self) -> &[Session] {
        &self.cache.sessions
    }

    pub fn replace_sessions(&mut self, sessions: Vec<Session>) {
        self.cache.sessions = sessions;
    }

    pub fn sync_endpoint(&self) -> &str {
        &self.cache.sync_endpoint
    }

    pub fn sync_username(&self) -> &str {
        &self.cache.sync_username
    }

    pub fn sync_etag(&self) -> Option<&str> {
        (self.cache.sync_etag_backend == self.sync_backend())
            .then_some(self.cache.sync_etag.as_deref())
            .flatten()
    }

    pub fn sync_device_id(&self) -> &str {
        &self.cache.sync_device_id
    }

    pub fn sync_backend(&self) -> &str {
        if self.cache.sync_backend == "s3" {
            "s3"
        } else {
            "webdav"
        }
    }

    pub fn set_sync_backend(&mut self, backend: &str) {
        self.cache.sync_backend = if backend == "s3" { "s3" } else { "webdav" }.to_string();
    }

    pub fn sync_s3_endpoint(&self) -> &str {
        &self.cache.sync_s3_endpoint
    }

    pub fn sync_s3_region(&self) -> &str {
        if self.cache.sync_s3_region.is_empty() {
            "us-east-1"
        } else {
            &self.cache.sync_s3_region
        }
    }

    pub fn sync_s3_bucket(&self) -> &str {
        &self.cache.sync_s3_bucket
    }

    pub fn sync_s3_object_key(&self) -> &str {
        if self.cache.sync_s3_object_key.is_empty() {
            "ax_shell-sync.json"
        } else {
            &self.cache.sync_s3_object_key
        }
    }

    pub fn set_sync_connection(&mut self, endpoint: String, username: String) {
        self.cache.sync_endpoint = endpoint;
        self.cache.sync_username = username;
    }

    pub fn set_sync_s3_connection(
        &mut self,
        endpoint: String,
        region: String,
        bucket: String,
        object_key: String,
    ) {
        self.cache.sync_s3_endpoint = endpoint;
        self.cache.sync_s3_region = region;
        self.cache.sync_s3_bucket = bucket;
        self.cache.sync_s3_object_key = object_key;
    }

    pub fn set_sync_etag(&mut self, etag: Option<String>) {
        self.cache.sync_etag = etag;
        self.cache.sync_etag_backend = self.sync_backend().to_string();
    }

    pub fn tmp_dir(&self) -> Option<PathBuf> {
        self.path.parent().map(|p| p.join("tmp"))
    }

    pub fn follow_system_theme(&self) -> bool {
        self.cache.follow_system_theme
    }

    pub fn theme_mode(&self) -> &str {
        &self.cache.theme_mode
    }

    pub fn light_theme_name(&self) -> &str {
        &self.cache.light_theme_name
    }

    pub fn dark_theme_name(&self) -> &str {
        &self.cache.dark_theme_name
    }

    pub fn locale(&self) -> &str {
        if self.cache.locale.is_empty() {
            "system"
        } else {
            &self.cache.locale
        }
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.cache.locale = locale.to_string();
    }

    pub fn key_bindings(&self) -> &std::collections::HashMap<String, String> {
        &self.cache.key_bindings
    }

    pub fn set_key_binding(&mut self, action_name: &str, keystroke: &str) {
        self.cache
            .key_bindings
            .insert(action_name.to_string(), keystroke.to_string());
    }

    pub fn monitoring_position(&self) -> &str {
        if self.cache.monitoring_position == "Bottom" {
            "Bottom"
        } else {
            "Sidebar"
        }
    }

    pub fn set_monitoring_position(&mut self, pos: &str) {
        self.cache.monitoring_position =
            if pos == "Bottom" { "Bottom" } else { "Sidebar" }.to_string();
    }

    pub fn show_monitoring_dashboard(&self) -> bool {
        self.cache.show_monitoring_dashboard
    }

    pub fn set_show_monitoring_dashboard(&mut self, val: bool) {
        self.cache.show_monitoring_dashboard = val;
    }

    pub fn terminal_font_size(&self) -> f32 {
        if self.cache.terminal_font_size <= 0.0 {
            default_terminal_font_size()
        } else {
            self.cache.terminal_font_size
        }
    }

    pub fn set_theme_preferences(
        &mut self,
        follow_system_theme: bool,
        theme_mode: impl Into<String>,
        light_theme_name: impl Into<String>,
        dark_theme_name: impl Into<String>,
    ) {
        self.cache.follow_system_theme = follow_system_theme;
        self.cache.theme_mode = theme_mode.into();
        self.cache.light_theme_name = light_theme_name.into();
        self.cache.dark_theme_name = dark_theme_name.into();
    }

    pub fn window_bounds(&self) -> Option<&SavedWindowBounds> {
        self.cache.window_bounds.as_ref()
    }

    pub fn workspace_panels(&self) -> Option<&Vec<f32>> {
        self.cache.workspace_panels.as_ref()
    }

    #[allow(dead_code)]
    pub fn body_panels(&self) -> Option<&Vec<f32>> {
        self.cache.body_panels.as_ref()
    }

    pub fn transfers(&self) -> Vec<crate::terminal::Transfer> {
        self.cache.transfers.clone()
    }

    pub fn set_transfers(&mut self, transfers: Vec<crate::terminal::Transfer>) {
        self.cache.transfers = transfers;
        if let Err(err) = self.save() {
            tracing::error!("failed to save config: {err:#}");
        }
    }

    pub fn set_layout_state(
        &mut self,
        window_bounds: Option<SavedWindowBounds>,
        workspace_panels: Option<Vec<f32>>,
        body_panels: Option<Vec<f32>>,
    ) {
        self.cache.window_bounds = window_bounds;
        self.cache.workspace_panels = workspace_panels;
        self.cache.body_panels = body_panels;
    }

    pub fn set_terminal_font_size(&mut self, terminal_font_size: f32) {
        self.cache.terminal_font_size = terminal_font_size.max(10.0);
    }

    pub fn ui_font_size(&self) -> f32 {
        if self.cache.ui_font_size <= 0.0 {
            default_ui_font_size()
        } else {
            self.cache.ui_font_size
        }
    }

    pub fn set_ui_font_size(&mut self, ui_font_size: f32) {
        self.cache.ui_font_size = ui_font_size.max(8.0);
    }

    pub fn custom_font_brightness(&self) -> f32 {
        let value = self.cache.custom_font_brightness;
        if value <= 0.0 {
            default_custom_font_brightness()
        } else {
            value.clamp(CUSTOM_FONT_BRIGHTNESS_MIN, CUSTOM_FONT_BRIGHTNESS_MAX)
        }
    }

    pub fn custom_theme_name(&self) -> &str {
        if self.cache.custom_theme_name.trim().is_empty() {
            "Custom Theme"
        } else {
            self.cache.custom_theme_name.trim()
        }
    }

    pub fn set_custom_theme_name(&mut self, name: &str) {
        let name = name.trim();
        self.cache.custom_theme_name = if name.is_empty() {
            default_custom_theme_name()
        } else {
            name.to_string()
        };
    }

    fn has_structured_custom_theme(&self) -> bool {
        let draft = &self.cache.custom_theme;
        draft.theme_name.trim() != default_custom_theme_name()
            || !draft.light.base_theme_name.trim().is_empty()
            || !draft.dark.base_theme_name.trim().is_empty()
            || !draft.light.overrides.is_empty()
            || !draft.dark.overrides.is_empty()
            || (draft.light.font_brightness - default_custom_font_brightness()).abs() > f32::EPSILON
            || (draft.dark.font_brightness - default_custom_font_brightness()).abs() > f32::EPSILON
    }

    fn effective_custom_theme(&self) -> CustomThemeConfig {
        let mut draft = if self.has_structured_custom_theme() {
            self.cache.custom_theme.clone()
        } else {
            CustomThemeConfig::default()
        };

        let legacy_has_values = !self.cache.custom_primary_color.trim().is_empty()
            || !self.cache.custom_background_color.trim().is_empty()
            || (self.cache.custom_font_brightness - default_custom_font_brightness()).abs()
                > f32::EPSILON
            || self.cache.custom_theme_name.trim() != default_custom_theme_name();

        if !self.has_structured_custom_theme() && legacy_has_values {
            draft.theme_name = if self.cache.custom_theme_name.trim().is_empty() {
                default_custom_theme_name()
            } else {
                self.cache.custom_theme_name.trim().to_string()
            };

            for mode_cfg in [&mut draft.light, &mut draft.dark] {
                if !self.cache.custom_primary_color.trim().is_empty() {
                    mode_cfg.overrides.insert(
                        "primary.background".to_string(),
                        self.cache.custom_primary_color.trim().to_string(),
                    );
                }
                if !self.cache.custom_background_color.trim().is_empty() {
                    mode_cfg.overrides.insert(
                        "background".to_string(),
                        self.cache.custom_background_color.trim().to_string(),
                    );
                }
                mode_cfg.font_brightness = self.custom_font_brightness();
            }
        }

        if draft.theme_name.trim().is_empty() {
            draft.theme_name = default_custom_theme_name();
        }
        if draft.light.font_brightness <= 0.0 {
            draft.light.font_brightness = default_custom_font_brightness();
        }
        if draft.dark.font_brightness <= 0.0 {
            draft.dark.font_brightness = default_custom_font_brightness();
        }

        draft
    }

    fn custom_theme_mode_ref(draft: &CustomThemeConfig, mode: ThemeMode) -> &CustomThemeModeConfig {
        if mode.is_dark() {
            &draft.dark
        } else {
            &draft.light
        }
    }

    fn custom_theme_mode_mut(
        draft: &mut CustomThemeConfig,
        mode: ThemeMode,
    ) -> &mut CustomThemeModeConfig {
        if mode.is_dark() {
            &mut draft.dark
        } else {
            &mut draft.light
        }
    }

    pub fn custom_theme_draft(&self) -> CustomThemeConfig {
        self.effective_custom_theme()
    }

    pub fn set_custom_theme_draft_name(&mut self, name: &str) {
        let mut draft = self.effective_custom_theme();
        let name = name.trim();
        draft.theme_name = if name.is_empty() {
            default_custom_theme_name()
        } else {
            name.to_string()
        };
        self.cache.custom_theme = draft;
        self.set_custom_theme_name(name);
    }

    pub fn custom_theme_base_name(&self, mode: ThemeMode) -> String {
        let draft = self.effective_custom_theme();
        let mode_cfg = Self::custom_theme_mode_ref(&draft, mode);
        mode_cfg.base_theme_name.trim().to_string()
    }

    pub fn set_custom_theme_base_name(&mut self, mode: ThemeMode, name: &str) {
        let mut draft = self.effective_custom_theme();
        Self::custom_theme_mode_mut(&mut draft, mode).base_theme_name = name.trim().to_string();
        self.cache.custom_theme = draft;
    }

    pub fn set_custom_theme_override(&mut self, mode: ThemeMode, key: &str, value: &str) {
        let mut draft = self.effective_custom_theme();
        let overrides = &mut Self::custom_theme_mode_mut(&mut draft, mode).overrides;
        let value = value.trim();
        if value.is_empty() {
            overrides.remove(key);
        } else {
            overrides.insert(key.to_string(), value.to_string());
        }
        self.cache.custom_theme = draft;
    }

    pub fn custom_theme_font_brightness_for_mode(&self, mode: ThemeMode) -> f32 {
        let draft = self.effective_custom_theme();
        let value = Self::custom_theme_mode_ref(&draft, mode).font_brightness;
        if value <= 0.0 {
            default_custom_font_brightness()
        } else {
            value.clamp(CUSTOM_FONT_BRIGHTNESS_MIN, CUSTOM_FONT_BRIGHTNESS_MAX)
        }
    }

    pub fn set_custom_theme_font_brightness_for_mode(&mut self, mode: ThemeMode, brightness: f32) {
        let mut draft = self.effective_custom_theme();
        Self::custom_theme_mode_mut(&mut draft, mode).font_brightness =
            brightness.clamp(CUSTOM_FONT_BRIGHTNESS_MIN, CUSTOM_FONT_BRIGHTNESS_MAX);
        self.cache.custom_theme = draft;
    }

    pub fn reset_custom_theme_draft(&mut self) {
        self.cache.custom_theme = CustomThemeConfig::default();
        self.cache.custom_primary_color.clear();
        self.cache.custom_background_color.clear();
        self.cache.custom_font_brightness = default_custom_font_brightness();
        self.cache.custom_theme_name = default_custom_theme_name();
    }

    pub fn theme_dir(&self) -> Option<PathBuf> {
        self.path.parent().map(|path| path.join("themes"))
    }

    pub fn ui_font_family(&self) -> &str {
        if self.cache.ui_font_family.is_empty() {
            ".SystemUIFont"
        } else {
            &self.cache.ui_font_family
        }
    }

    pub fn set_ui_font_family(&mut self, family: &str) {
        self.cache.ui_font_family = family.to_string();
    }

    pub fn right_click_copy_paste(&self) -> bool {
        self.cache.right_click_copy_paste
    }

    pub fn set_right_click_copy_paste(&mut self, val: bool) {
        self.cache.right_click_copy_paste = val;
    }

    pub fn keyword_highlight(&self) -> bool {
        self.cache.keyword_highlight
    }

    pub fn set_keyword_highlight(&mut self, val: bool) {
        self.cache.keyword_highlight = val;
    }

    pub fn terminal_font_family(&self) -> &str {
        if self.cache.terminal_font_family.is_empty() {
            "Maple Mono NF CN"
        } else {
            &self.cache.terminal_font_family
        }
    }

    pub fn set_terminal_font_family(&mut self, family: &str) {
        self.cache.terminal_font_family = family.to_string();
    }

    pub fn effective_title_bar_style(&self) -> TitleBarStyle {
        effective_title_bar_style(self.cache.title_bar_style)
    }

    pub fn set_title_bar_style(&mut self, style: TitleBarStyle) {
        self.cache.title_bar_style = style;
    }

    pub fn cursor_style(&self) -> CursorStyle {
        self.cache.cursor_style
    }

    pub fn set_cursor_style(&mut self, style: CursorStyle) {
        self.cache.cursor_style = style;
    }

    pub fn use_proxy(&self) -> bool {
        self.cache.use_proxy
    }
    pub fn set_use_proxy(&mut self, val: bool) {
        self.cache.use_proxy = val;
    }
    pub fn read_env_proxy(&self) -> bool {
        self.cache.read_env_proxy
    }
    pub fn set_read_env_proxy(&mut self, val: bool) {
        self.cache.read_env_proxy = val;
    }
    pub fn global_proxy_type(&self) -> &str {
        &self.cache.global_proxy_type
    }
    pub fn set_global_proxy_type(&mut self, val: String) {
        self.cache.global_proxy_type = val;
    }
    pub fn global_proxy_host(&self) -> &str {
        &self.cache.global_proxy_host
    }
    pub fn set_global_proxy_host(&mut self, val: String) {
        self.cache.global_proxy_host = val;
    }
    pub fn global_proxy_port(&self) -> Option<u16> {
        self.cache.global_proxy_port
    }
    pub fn set_global_proxy_port(&mut self, val: Option<u16>) {
        self.cache.global_proxy_port = val;
    }
    pub fn global_proxy_user(&self) -> &str {
        &self.cache.global_proxy_user
    }
    pub fn set_global_proxy_user(&mut self, val: String) {
        self.cache.global_proxy_user = val;
    }
    pub fn global_proxy_password(&self) -> &str {
        &self.cache.global_proxy_password
    }
    pub fn set_global_proxy_password(&mut self, val: String) {
        self.cache.global_proxy_password = val;
    }

    pub fn x11_forwarding_enabled(&self) -> bool {
        self.cache.x11_forwarding_enabled
    }

    pub fn set_x11_forwarding_enabled(&mut self, val: bool) {
        self.cache.x11_forwarding_enabled = val;
    }

    pub fn x11_launch_xquartz(&self) -> bool {
        self.cache.x11_launch_xquartz
    }

    pub fn x11_launch_local_x_server(&self) -> bool {
        self.cache.x11_launch_xquartz
    }

    pub fn set_x11_launch_xquartz(&mut self, val: bool) {
        self.cache.x11_launch_xquartz = val;
    }

    pub fn local_x_server_app_path(&self) -> &str {
        self.cache.xquartz_app_path.trim()
    }

    pub fn set_local_x_server_app_path(&mut self, val: String) {
        self.cache.xquartz_app_path = normalize_local_x_server_app_path(&val);
    }

    pub fn show_hidden_files(&self) -> bool {
        self.cache.show_hidden_files
    }

    pub fn set_show_hidden_files(&mut self, val: bool) {
        self.cache.show_hidden_files = val;
    }

    pub fn lock_layout(&self) -> bool {
        self.cache.lock_layout
    }

    pub fn set_lock_layout(&mut self, val: bool) {
        self.cache.lock_layout = val;
    }

    pub fn sidebar_collapsed(&self) -> bool {
        self.cache.sidebar_collapsed
    }

    pub fn set_sidebar_collapsed(&mut self, val: bool) {
        self.cache.sidebar_collapsed = val;
    }

    pub fn get(&self, id: &str) -> Option<&Session> {
        self.cache.sessions.iter().find(|s| s.id == id)
    }

    pub fn upsert(&mut self, session: Session) {
        if let Some(existing) = self.cache.sessions.iter_mut().find(|s| s.id == session.id) {
            *existing = session;
        } else {
            self.cache.sessions.push(session);
        }
    }

    pub fn set_session_last_successful_ssh_mode(
        &mut self,
        id: &str,
        mode: SshConnectionMode,
    ) -> bool {
        let Some(session) = self.cache.sessions.iter_mut().find(|s| s.id == id) else {
            return false;
        };
        if session.last_successful_ssh_mode == Some(mode) {
            return false;
        }
        session.last_successful_ssh_mode = Some(mode);
        true
    }

    pub fn remove(&mut self, id: &str) {
        self.cache.sessions.retain(|s| s.id != id);
    }

    pub fn save(&self) -> Result<()> {
        if self.path.as_os_str().is_empty() {
            return Ok(());
        }
        let raw = serde_json::to_string_pretty(&self.cache)?;
        fs::write(&self.path, raw)
            .with_context(|| format!("failed to write {}", self.path.display()))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(mut perms) = fs::metadata(&self.path).map(|m| m.permissions()) {
                perms.set_mode(0o600);
                let _ = fs::set_permissions(&self.path, perms);
            }
        }

        Ok(())
    }
}

fn copy_dir_recursive(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to).with_context(|| format!("failed to create {}", to.display()))?;
    for entry in fs::read_dir(from).with_context(|| format!("failed to read {}", from.display()))? {
        let entry = entry.with_context(|| format!("failed to read entry in {}", from.display()))?;
        let source = entry.path();
        let target = to.join(entry.file_name());
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to read file type for {}", source.display()))?;
        if file_type.is_dir() {
            copy_dir_recursive(&source, &target)?;
        } else if file_type.is_file() {
            fs::copy(&source, &target).with_context(|| {
                format!(
                    "failed to copy legacy file {} to {}",
                    source.display(),
                    target.display()
                )
            })?;
        }
    }
    Ok(())
}

pub trait ProxyStream:
    tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + Sync + 'static
{
}
impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + Sync + 'static> ProxyStream
    for T
{
}

use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct EnvProxy {
    pub proxy_type: String,
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub pass: String,
}

pub static ENV_PROXY: OnceLock<Option<EnvProxy>> = OnceLock::new();

pub async fn connect_proxy(session: &Session) -> Result<Box<dyn ProxyStream>> {
    let target_host = &session.host;
    let target_port = session.port;

    let config = ConfigStore::load().unwrap_or_else(|_| ConfigStore::in_memory());
    let (proxy_type, proxy_host, proxy_port, proxy_user, proxy_password) = {
        if !session.proxy_type.is_empty() && session.proxy_type != "none" {
            (
                session.proxy_type.clone(),
                session.proxy_host.clone(),
                session.proxy_port,
                session.proxy_user.clone(),
                session.proxy_password.clone(),
            )
        } else if config.cache.read_env_proxy
            && ENV_PROXY.get().and_then(|opt| opt.as_ref()).is_some()
        {
            let env_p = ENV_PROXY.get().and_then(|opt| opt.as_ref()).unwrap();
            (
                env_p.proxy_type.clone(),
                env_p.host.clone(),
                env_p.port,
                env_p.user.clone(),
                env_p.pass.clone(),
            )
        } else if config.cache.use_proxy {
            (
                config.cache.global_proxy_type.clone(),
                config.cache.global_proxy_host.clone(),
                config.cache.global_proxy_port,
                config.cache.global_proxy_user.clone(),
                config.cache.global_proxy_password.clone(),
            )
        } else {
            (
                "none".to_string(),
                String::new(),
                None,
                String::new(),
                String::new(),
            )
        }
    };

    if proxy_type != "none" && (proxy_host.is_empty() || proxy_port.is_none()) {
        let addr = format!("{}:{}", target_host, target_port);
        let stream = tokio::net::TcpStream::connect(&addr).await?;
        return Ok(Box::new(stream));
    }

    match proxy_type.as_str() {
        "socks5" | "socks5h" => {
            let proxy_port = proxy_port.unwrap_or(1080);
            let proxy_addr = format!("{}:{}", proxy_host, proxy_port);

            if !proxy_user.is_empty() {
                let stream = tokio_socks::tcp::Socks5Stream::connect_with_password(
                    proxy_addr.as_str(),
                    (target_host.as_str(), target_port),
                    &proxy_user,
                    &proxy_password,
                )
                .await
                .map_err(|e| anyhow::anyhow!("SOCKS5 proxy connection failed: {}", e))?;
                Ok(Box::new(stream))
            } else {
                let stream = tokio_socks::tcp::Socks5Stream::connect(
                    proxy_addr.as_str(),
                    (target_host.as_str(), target_port),
                )
                .await
                .map_err(|e| anyhow::anyhow!("SOCKS5 proxy connection failed: {}", e))?;
                Ok(Box::new(stream))
            }
        }
        "http" => {
            let proxy_port = proxy_port.unwrap_or(8080);
            let proxy_addr = format!("{}:{}", proxy_host, proxy_port);

            use tokio::io::AsyncWriteExt;
            let mut stream = tokio::net::TcpStream::connect(&proxy_addr)
                .await
                .map_err(|e| anyhow::anyhow!("HTTP proxy connection failed: {}", e))?;

            let mut request = format!(
                "CONNECT {}:{} HTTP/1.1\r\nHost: {}:{}\r\n",
                target_host, target_port, target_host, target_port
            );
            if !proxy_user.is_empty() {
                use base64::Engine as _;
                let auth = format!("{}:{}", proxy_user, proxy_password);
                let encoded = base64::engine::general_purpose::STANDARD.encode(auth);
                request.push_str(&format!("Proxy-Authorization: Basic {}\r\n", encoded));
            }
            request.push_str("\r\n");

            stream.write_all(request.as_bytes()).await?;

            let mut response = [0u8; 1024];
            let n = tokio::io::AsyncReadExt::read(&mut stream, &mut response).await?;
            let resp_str = String::from_utf8_lossy(&response[..n]);
            if !resp_str.contains("200") && !resp_str.contains("established") {
                return Err(anyhow::anyhow!("HTTP proxy CONNECT failed: {}", resp_str));
            }

            Ok(Box::new(stream))
        }
        _ => {
            let addr = format!("{}:{}", target_host, target_port);
            let stream = tokio::net::TcpStream::connect(&addr).await?;
            Ok(Box::new(stream))
        }
    }
}

pub fn active_proxy(session: &Session) -> Option<(String, String, Option<u16>)> {
    let config = ConfigStore::load().unwrap_or_else(|_| ConfigStore::in_memory());
    let (proxy_type, proxy_host, proxy_port, _, _) = {
        if !session.proxy_type.is_empty() && session.proxy_type != "none" {
            (
                session.proxy_type.clone(),
                session.proxy_host.clone(),
                session.proxy_port,
                session.proxy_user.clone(),
                session.proxy_password.clone(),
            )
        } else if config.cache.read_env_proxy
            && ENV_PROXY.get().and_then(|opt| opt.as_ref()).is_some()
        {
            let env_p = ENV_PROXY.get().and_then(|opt| opt.as_ref()).unwrap();
            (
                env_p.proxy_type.clone(),
                env_p.host.clone(),
                env_p.port,
                env_p.user.clone(),
                env_p.pass.clone(),
            )
        } else if config.cache.use_proxy {
            (
                config.cache.global_proxy_type.clone(),
                config.cache.global_proxy_host.clone(),
                config.cache.global_proxy_port,
                config.cache.global_proxy_user.clone(),
                config.cache.global_proxy_password.clone(),
            )
        } else {
            (
                "none".to_string(),
                String::new(),
                None,
                String::new(),
                String::new(),
            )
        }
    };

    if proxy_type != "none" && !proxy_host.is_empty() && proxy_port.is_some() {
        Some((proxy_type, proxy_host, proxy_port))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{SshConnectionMode, ordered_ssh_connection_modes};

    #[test]
    fn ssh_connection_modes_default_to_safe_order() {
        assert_eq!(
            ordered_ssh_connection_modes(None),
            vec![SshConnectionMode::Default, SshConnectionMode::Legacy]
        );
    }

    #[test]
    fn ssh_connection_modes_prioritize_last_successful_mode() {
        assert_eq!(
            ordered_ssh_connection_modes(Some(SshConnectionMode::Legacy)),
            vec![SshConnectionMode::Legacy, SshConnectionMode::Default]
        );
        assert_eq!(
            ordered_ssh_connection_modes(Some(SshConnectionMode::Default)),
            vec![SshConnectionMode::Default, SshConnectionMode::Legacy]
        );
    }
}
