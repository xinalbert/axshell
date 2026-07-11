use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use directories::BaseDirs;
use gpui_component::ThemeMode;
use uuid::Uuid;

use crate::session::{Session, SshConnectionMode};

use super::model::*;

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
                    let config_path = crate::diagnostics::mask_path(&path.to_string_lossy());
                    let backup_label =
                        crate::diagnostics::mask_path(&backup_path.to_string_lossy());
                    if let Err(backup_err) = fs::write(&backup_path, raw.as_bytes()) {
                        tracing::warn!(
                            component = "config",
                            operation = "load",
                            config_path = %config_path,
                            backup_path = %backup_label,
                            parse_error = %crate::diagnostics::sanitize_error(&err.to_string()),
                            backup_error = %crate::diagnostics::sanitize_error(&format!("{backup_err:#}")),
                            "Failed to parse configuration and write backup"
                        );
                    } else {
                        tracing::warn!(
                            component = "config",
                            operation = "load",
                            config_path = %config_path,
                            backup_path = %backup_label,
                            error = %crate::diagnostics::sanitize_error(&err.to_string()),
                            "Failed to parse configuration; loaded defaults"
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
        cache.ssh_connect_retry_count =
            normalize_ssh_connect_retry_count(cache.ssh_connect_retry_count);
        cache.ssh_connect_retry_delays_ms = normalize_ssh_connect_retry_delays_ms(
            std::mem::take(&mut cache.ssh_connect_retry_delays_ms),
            cache.ssh_connect_retry_count,
        );
        cache.sftp_transfer_close_behavior =
            normalize_sftp_transfer_close_behavior(&cache.sftp_transfer_close_behavior);
        cache.deep_sleep_after_minutes =
            normalize_deep_sleep_after_minutes(cache.deep_sleep_after_minutes);
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
                component = "config",
                operation = "migrate_legacy_config",
                source = %crate::diagnostics::mask_path(&legacy_config.to_string_lossy()),
                destination = %crate::diagnostics::mask_path(&current_config.to_string_lossy()),
                "Migrated legacy configuration"
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
                component = "config",
                operation = "migrate_legacy_themes",
                source = %crate::diagnostics::mask_path(&legacy_themes.to_string_lossy()),
                destination = %crate::diagnostics::mask_path(&current_themes.to_string_lossy()),
                "Migrated legacy theme directory"
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

    pub fn transfers(&self) -> Vec<crate::sftp::Transfer> {
        self.cache.transfers.clone()
    }

    pub fn set_transfers(&mut self, transfers: Vec<crate::sftp::Transfer>) {
        self.cache.transfers = transfers;
        self.save_logged("persist_transfers");
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

    pub fn ssh_connect_retry_count(&self) -> u32 {
        normalize_ssh_connect_retry_count(self.cache.ssh_connect_retry_count)
    }

    pub fn set_ssh_connect_retry_count(&mut self, val: u32) {
        self.cache.ssh_connect_retry_count = normalize_ssh_connect_retry_count(val);
        let delays = std::mem::take(&mut self.cache.ssh_connect_retry_delays_ms);
        self.cache.ssh_connect_retry_delays_ms =
            normalize_ssh_connect_retry_delays_ms(delays, self.cache.ssh_connect_retry_count);
    }

    pub fn ssh_connect_retry_delays_ms(&self) -> Vec<u64> {
        normalize_ssh_connect_retry_delays_ms(
            self.cache.ssh_connect_retry_delays_ms.clone(),
            self.ssh_connect_retry_count(),
        )
    }

    pub fn set_ssh_connect_retry_delays_ms(&mut self, delays: Vec<u64>) {
        self.cache.ssh_connect_retry_delays_ms =
            normalize_ssh_connect_retry_delays_ms(delays, self.ssh_connect_retry_count());
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

    pub fn sftp_transfer_close_behavior(&self) -> &str {
        &self.cache.sftp_transfer_close_behavior
    }

    pub fn set_sftp_transfer_close_behavior(&mut self, value: &str) {
        self.cache.sftp_transfer_close_behavior = normalize_sftp_transfer_close_behavior(value);
    }

    pub fn settings_close_shortcut_confirms(&self) -> bool {
        self.cache.settings_close_shortcut_confirms
    }

    pub fn set_settings_close_shortcut_confirms(&mut self, value: bool) {
        self.cache.settings_close_shortcut_confirms = value;
    }

    pub fn deep_sleep_after_minutes(&self) -> u32 {
        self.cache.deep_sleep_after_minutes
    }

    pub fn set_deep_sleep_after_minutes(&mut self, value: u32) {
        self.cache.deep_sleep_after_minutes = normalize_deep_sleep_after_minutes(value);
    }

    pub fn lock_layout(&self) -> bool {
        self.cache.lock_layout
    }

    pub fn set_lock_layout(&mut self, val: bool) {
        self.cache.lock_layout = val;
    }

    pub fn color_inactive_tabs(&self) -> bool {
        self.cache.color_inactive_tabs
    }

    pub fn set_color_inactive_tabs(&mut self, val: bool) {
        self.cache.color_inactive_tabs = val;
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

    pub fn save_logged(&self, operation: &'static str) {
        if let Err(err) = self.save() {
            let error = crate::diagnostics::sanitize_error(&format!("{err:#}"));
            tracing::error!(
                component = "config",
                operation,
                error = %error,
                "Failed to save configuration"
            );
        }
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

#[cfg(test)]
mod retry_settings_tests {
    use super::{
        default_ssh_connect_retry_delays_ms, normalize_ssh_connect_retry_count,
        normalize_ssh_connect_retry_delays_ms,
    };

    #[test]
    fn retry_count_is_clamped() {
        assert_eq!(normalize_ssh_connect_retry_count(2), 2);
        assert_eq!(normalize_ssh_connect_retry_count(99), 10);
    }

    #[test]
    fn retry_delays_fill_from_last_value() {
        assert_eq!(
            normalize_ssh_connect_retry_delays_ms(vec![250, 750], 4),
            vec![250, 750, 750, 750]
        );
    }

    #[test]
    fn retry_delays_fall_back_to_defaults_when_empty() {
        assert_eq!(
            normalize_ssh_connect_retry_delays_ms(vec![], 2),
            default_ssh_connect_retry_delays_ms()
        );
    }
}

#[cfg(test)]
mod sftp_transfer_close_tests {
    use super::{default_sftp_transfer_close_behavior, normalize_sftp_transfer_close_behavior};

    #[test]
    fn sftp_transfer_close_behavior_accepts_supported_values() {
        assert_eq!(
            normalize_sftp_transfer_close_behavior("background"),
            "background"
        );
        assert_eq!(
            normalize_sftp_transfer_close_behavior("cancel_disconnect"),
            "cancel_disconnect"
        );
    }

    #[test]
    fn sftp_transfer_close_behavior_falls_back_to_ask() {
        assert_eq!(
            normalize_sftp_transfer_close_behavior("unexpected"),
            default_sftp_transfer_close_behavior()
        );
    }
}

#[cfg(test)]
mod settings_close_confirmation_tests {
    use super::{ConfigFile, ConfigStore};

    #[test]
    fn missing_settings_close_shortcut_action_defaults_to_close() {
        let config: ConfigFile = serde_json::from_str("{}").expect("config should deserialize");

        assert!(config.settings_close_shortcut_confirms);
    }

    #[test]
    fn settings_close_shortcut_action_can_keep_open_or_close() {
        let mut config = ConfigStore::in_memory();

        assert!(config.settings_close_shortcut_confirms());
        config.set_settings_close_shortcut_confirms(false);
        assert!(!config.settings_close_shortcut_confirms());
        config.set_settings_close_shortcut_confirms(true);
        assert!(config.settings_close_shortcut_confirms());
    }
}

#[cfg(test)]
mod deep_sleep_settings_tests {
    use super::{default_deep_sleep_after_minutes, normalize_deep_sleep_after_minutes};

    #[test]
    fn deep_sleep_after_minutes_accepts_supported_values() {
        for value in [0, 1, 5, 15, 30] {
            assert_eq!(normalize_deep_sleep_after_minutes(value), value);
        }
    }

    #[test]
    fn deep_sleep_after_minutes_falls_back_to_five_minutes() {
        assert_eq!(
            normalize_deep_sleep_after_minutes(2),
            default_deep_sleep_after_minutes()
        );
    }
}

#[cfg(test)]
mod tab_color_settings_tests {
    use super::ConfigFile;

    #[test]
    fn inactive_tab_colors_default_to_disabled_for_existing_configs() {
        let config: ConfigFile = serde_json::from_str("{}").expect("config should deserialize");

        assert!(!config.color_inactive_tabs);
    }

    #[test]
    fn inactive_tab_colors_round_trip_when_enabled() {
        let config = ConfigFile {
            color_inactive_tabs: true,
            ..ConfigFile::default()
        };
        let raw = serde_json::to_string(&config).expect("config should serialize");
        let restored: ConfigFile =
            serde_json::from_str(&raw).expect("config should deserialize after serialization");

        assert!(restored.color_inactive_tabs);
    }
}
