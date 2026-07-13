use std::{
    collections::HashSet,
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

fn normalize_last_local_sftp_paths(config: &mut ConfigFile) {
    let session_ids = config
        .sessions
        .iter()
        .map(|session| session.id.as_str())
        .collect::<HashSet<_>>();
    config.last_local_sftp_paths.retain(|session_id, path| {
        session_ids.contains(session_id.as_str()) && !path.trim().is_empty()
    });
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
        normalize_last_local_sftp_paths(&mut cache);

        let mut store = Self { path, cache };
        store.normalize_theme_profiles();
        store.migrate_global_font_brightness_from_legacy();
        store.sync_custom_theme_draft_to_active_profile();
        Ok(store)
    }

    pub fn in_memory() -> Self {
        let cache = ConfigFile {
            sync_device_id: Uuid::new_v4().to_string(),
            ..ConfigFile::default()
        };
        let mut store = Self {
            path: PathBuf::new(),
            cache,
        };
        store.normalize_theme_profiles();
        store.migrate_global_font_brightness_from_legacy();
        store.sync_custom_theme_draft_to_active_profile();
        store
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
        normalize_last_local_sftp_paths(&mut self.cache);
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

    pub fn active_theme_profile_id(&self) -> &str {
        &self.cache.active_theme_profile_id
    }

    pub fn theme_profiles(&self) -> &[ThemeProfileConfig] {
        &self.cache.theme_profiles
    }

    pub fn active_theme_profile(&self) -> Option<&ThemeProfileConfig> {
        let active_id = self.active_theme_profile_id();
        self.cache
            .theme_profiles
            .iter()
            .find(|profile| profile.id == active_id)
    }

    fn active_theme_profile_mut(&mut self) -> Option<&mut ThemeProfileConfig> {
        let active_id = self.cache.active_theme_profile_id.clone();
        self.cache
            .theme_profiles
            .iter_mut()
            .find(|profile| profile.id == active_id)
    }

    pub fn set_active_theme_profile(&mut self, id: &str) -> Option<ThemeProfileConfig> {
        let profile = self
            .cache
            .theme_profiles
            .iter()
            .find(|profile| profile.id == id)
            .cloned()?;
        self.cache.active_theme_profile_id = profile.id.clone();
        self.cache.light_theme_name = profile.light_theme_name.clone();
        self.cache.dark_theme_name = profile.dark_theme_name.clone();
        self.sync_custom_theme_draft_from_profile(&profile);
        Some(profile)
    }

    pub fn activate_imported_theme_profile(
        &mut self,
        name: String,
        light_theme_name: String,
        dark_theme_name: String,
    ) -> ThemeProfileConfig {
        let name = if name.trim().is_empty() {
            "Imported Theme".to_string()
        } else {
            name.trim().to_string()
        };
        let base_id = format!("imported-{}", normalize_theme_profile_id("", &name));
        let mut id = base_id.clone();
        let mut suffix = 2;
        while self
            .cache
            .theme_profiles
            .iter()
            .any(|profile| profile.id == id)
        {
            id = format!("{base_id}-{suffix}");
            suffix += 1;
        }

        let profile = ThemeProfileConfig {
            id,
            name,
            light_theme_name: light_theme_name.trim().to_string(),
            dark_theme_name: dark_theme_name.trim().to_string(),
            custom_theme: None,
            custom_theme_save_path: String::new(),
        };
        self.cache.active_theme_profile_id = profile.id.clone();
        self.cache.light_theme_name = profile.light_theme_name.clone();
        self.cache.dark_theme_name = profile.dark_theme_name.clone();
        self.cache.theme_profiles.push(profile.clone());
        profile
    }

    pub fn custom_theme_save_path(&self) -> &str {
        self.active_theme_profile()
            .map(|profile| profile.custom_theme_save_path.trim())
            .unwrap_or("")
    }

    pub fn set_custom_theme_save_path(&mut self, path: &str) {
        if let Some(profile) = self.active_theme_profile_mut() {
            profile.custom_theme_save_path = path.trim().to_string();
        }
    }

    fn normalize_theme_profiles(&mut self) {
        let had_profiles = !self.cache.theme_profiles.is_empty();
        if self.cache.theme_profiles.is_empty() {
            self.cache.theme_profiles = default_theme_profiles();
        }

        if !had_profiles && self.has_legacy_theme_profile_values() {
            let legacy = self.legacy_theme_profile();
            self.cache.active_theme_profile_id = legacy.id.clone();
            self.cache.theme_profiles.insert(0, legacy);
        }

        let default_profiles = default_theme_profiles();
        for default_profile in &default_profiles {
            if let Some(profile) = self
                .cache
                .theme_profiles
                .iter_mut()
                .find(|profile| profile.id == default_profile.id)
            {
                if is_plain_builtin_theme_profile(profile) {
                    profile.name = default_profile.name.clone();
                    profile.light_theme_name = default_profile.light_theme_name.clone();
                    profile.dark_theme_name = default_profile.dark_theme_name.clone();
                }
            } else {
                self.cache.theme_profiles.push(default_profile.clone());
            }
        }

        let mut used_ids = HashSet::new();
        for index in 0..self.cache.theme_profiles.len() {
            let profile = &mut self.cache.theme_profiles[index];
            if profile.name.trim().is_empty() {
                profile.name = format!("Theme {}", index + 1);
            } else {
                profile.name = profile.name.trim().to_string();
            }

            let base_id = normalize_theme_profile_id(&profile.id, &profile.name);
            let mut candidate = base_id.clone();
            let mut suffix = 2;
            while used_ids.contains(&candidate) {
                candidate = format!("{base_id}-{suffix}");
                suffix += 1;
            }
            profile.id = candidate.clone();
            used_ids.insert(candidate);
            profile.light_theme_name = profile.light_theme_name.trim().to_string();
            profile.dark_theme_name = profile.dark_theme_name.trim().to_string();
            profile.custom_theme_save_path = profile.custom_theme_save_path.trim().to_string();
        }

        if let Some(replacement_id) =
            deprecated_builtin_theme_profile_replacement(&self.cache.active_theme_profile_id)
        {
            self.cache.active_theme_profile_id = replacement_id.to_string();
        }
        self.cache.theme_profiles.retain(|profile| {
            deprecated_builtin_theme_profile_replacement(&profile.id).is_none()
                || !is_plain_builtin_theme_profile(profile)
        });

        if self.cache.active_theme_profile_id.trim().is_empty()
            || !self
                .cache
                .theme_profiles
                .iter()
                .any(|profile| profile.id == self.cache.active_theme_profile_id)
        {
            self.cache.active_theme_profile_id = self
                .cache
                .theme_profiles
                .first()
                .map(|profile| profile.id.clone())
                .unwrap_or_else(default_active_theme_profile_id);
        }

        if default_profiles
            .iter()
            .any(|profile| profile.id == self.cache.active_theme_profile_id)
        {
            if let Some(profile) = self.active_theme_profile().cloned() {
                if is_plain_builtin_theme_profile(&profile) {
                    self.cache.light_theme_name = profile.light_theme_name;
                    self.cache.dark_theme_name = profile.dark_theme_name;
                }
            }
        }
    }

    fn migrate_global_font_brightness_from_legacy(&mut self) {
        self.cache.ui_font_brightness = normalize_font_brightness(self.cache.ui_font_brightness);
        let mut terminal_brightness =
            normalize_font_brightness(self.cache.terminal_font_brightness);

        if is_default_normalized_font_brightness(terminal_brightness) {
            if !is_default_normalized_font_brightness(self.cache.custom_font_brightness) {
                terminal_brightness = normalize_font_brightness(self.cache.custom_font_brightness);
            } else if let Some(legacy_brightness) = self.legacy_custom_theme_font_brightness() {
                terminal_brightness = legacy_brightness;
            }
        }

        self.cache.terminal_font_brightness = terminal_brightness;
        self.cache.custom_font_brightness = default_font_brightness();
        reset_custom_theme_font_brightness(&mut self.cache.custom_theme);
        for profile in &mut self.cache.theme_profiles {
            if let Some(custom_theme) = profile.custom_theme.as_mut() {
                reset_custom_theme_font_brightness(custom_theme);
            }
        }
    }

    fn legacy_custom_theme_font_brightness(&self) -> Option<f32> {
        self.active_theme_profile()
            .and_then(|profile| profile.custom_theme.as_ref())
            .and_then(|custom_theme| {
                custom_theme_legacy_font_brightness(custom_theme, self.cache.theme_mode.as_str())
            })
            .or_else(|| {
                custom_theme_legacy_font_brightness(
                    &self.cache.custom_theme,
                    self.cache.theme_mode.as_str(),
                )
            })
    }

    fn has_legacy_theme_profile_values(&self) -> bool {
        !self.cache.light_theme_name.trim().is_empty()
            || !self.cache.dark_theme_name.trim().is_empty()
            || self.has_legacy_custom_theme_values()
    }

    fn has_legacy_custom_theme_values(&self) -> bool {
        self.has_structured_custom_theme()
            || !self.cache.custom_primary_color.trim().is_empty()
            || !self.cache.custom_background_color.trim().is_empty()
            || self.cache.custom_theme_name.trim() != default_custom_theme_name()
    }

    fn legacy_theme_profile(&self) -> ThemeProfileConfig {
        let fallback = default_theme_profiles()
            .into_iter()
            .next()
            .unwrap_or_else(|| ThemeProfileConfig {
                id: default_active_theme_profile_id(),
                name: "Balanced".to_string(),
                light_theme_name: String::new(),
                dark_theme_name: String::new(),
                custom_theme: None,
                custom_theme_save_path: String::new(),
            });
        let light_theme_name = if self.cache.light_theme_name.trim().is_empty() {
            fallback.light_theme_name
        } else {
            self.cache.light_theme_name.trim().to_string()
        };
        let dark_theme_name = if self.cache.dark_theme_name.trim().is_empty() {
            fallback.dark_theme_name
        } else {
            self.cache.dark_theme_name.trim().to_string()
        };
        let custom_theme = self
            .has_legacy_custom_theme_values()
            .then(|| self.legacy_custom_theme(&light_theme_name, &dark_theme_name));
        ThemeProfileConfig {
            id: "current".to_string(),
            name: "Current".to_string(),
            light_theme_name,
            dark_theme_name,
            custom_theme,
            custom_theme_save_path: String::new(),
        }
    }

    fn legacy_custom_theme(
        &self,
        light_theme_name: &str,
        dark_theme_name: &str,
    ) -> CustomThemeConfig {
        let mut draft = if self.has_structured_custom_theme() {
            self.cache.custom_theme.clone()
        } else {
            CustomThemeConfig::default()
        };

        if draft.theme_name.trim().is_empty() {
            draft.theme_name = default_custom_theme_name();
        }
        if draft.theme_name == default_custom_theme_name()
            && self.cache.custom_theme_name.trim() != default_custom_theme_name()
        {
            draft.theme_name = self.cache.custom_theme_name.trim().to_string();
        }
        if draft.light.base_theme_name.trim().is_empty() {
            draft.light.base_theme_name = light_theme_name.to_string();
        }
        if draft.dark.base_theme_name.trim().is_empty() {
            draft.dark.base_theme_name = dark_theme_name.to_string();
        }

        if !self.cache.custom_primary_color.trim().is_empty() {
            for mode_cfg in [&mut draft.light, &mut draft.dark] {
                mode_cfg.overrides.insert(
                    "primary.background".to_string(),
                    self.cache.custom_primary_color.trim().to_string(),
                );
            }
        }
        if !self.cache.custom_background_color.trim().is_empty() {
            for mode_cfg in [&mut draft.light, &mut draft.dark] {
                mode_cfg.overrides.insert(
                    "background".to_string(),
                    self.cache.custom_background_color.trim().to_string(),
                );
            }
        }
        draft
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

    pub fn ui_font_brightness(&self) -> f32 {
        normalize_font_brightness(self.cache.ui_font_brightness)
    }

    pub fn set_ui_font_brightness(&mut self, brightness: f32) {
        self.cache.ui_font_brightness = normalize_font_brightness(brightness);
    }

    pub fn terminal_font_brightness(&self) -> f32 {
        normalize_font_brightness(self.cache.terminal_font_brightness)
    }

    pub fn set_terminal_font_brightness(&mut self, brightness: f32) {
        self.cache.terminal_font_brightness = normalize_font_brightness(brightness);
    }

    pub fn custom_theme_name(&self) -> &str {
        if self.cache.custom_theme_name.trim().is_empty() {
            "Custom Theme"
        } else {
            self.cache.custom_theme_name.trim()
        }
    }

    fn has_structured_custom_theme(&self) -> bool {
        let draft = &self.cache.custom_theme;
        draft.theme_name.trim() != default_custom_theme_name()
            || !draft.light.base_theme_name.trim().is_empty()
            || !draft.dark.base_theme_name.trim().is_empty()
            || !draft.light.overrides.is_empty()
            || !draft.dark.overrides.is_empty()
    }

    fn draft_from_theme_profile(profile: &ThemeProfileConfig) -> CustomThemeConfig {
        let profile_name = if profile.name.trim().is_empty() {
            default_custom_theme_name()
        } else {
            format!("{} Custom", profile.name.trim())
        };
        CustomThemeConfig {
            theme_name: profile_name,
            light: CustomThemeModeConfig {
                base_theme_name: profile.light_theme_name.trim().to_string(),
                ..CustomThemeModeConfig::default()
            },
            dark: CustomThemeModeConfig {
                base_theme_name: profile.dark_theme_name.trim().to_string(),
                ..CustomThemeModeConfig::default()
            },
        }
    }

    fn sync_custom_theme_draft_from_profile(&mut self, profile: &ThemeProfileConfig) {
        let draft = profile
            .custom_theme
            .clone()
            .unwrap_or_else(|| Self::draft_from_theme_profile(profile));
        self.cache.custom_theme = draft.clone();
        self.cache.custom_theme_name = draft.theme_name;
    }

    fn sync_custom_theme_draft_to_active_profile(&mut self) {
        if let Some(profile) = self.active_theme_profile().cloned() {
            self.sync_custom_theme_draft_from_profile(&profile);
        }
    }

    fn effective_custom_theme(&self) -> CustomThemeConfig {
        let mut draft = if self.has_structured_custom_theme() {
            self.cache.custom_theme.clone()
        } else if let Some(custom_theme) = self
            .active_theme_profile()
            .and_then(|profile| profile.custom_theme.clone())
        {
            custom_theme
        } else if let Some(profile) = self.active_theme_profile() {
            Self::draft_from_theme_profile(profile)
        } else {
            CustomThemeConfig::default()
        };

        let legacy_has_values = !self.cache.custom_primary_color.trim().is_empty()
            || !self.cache.custom_background_color.trim().is_empty()
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
            }
        }

        if draft.theme_name.trim().is_empty() {
            draft.theme_name = default_custom_theme_name();
        }
        if draft.light.font_brightness <= 0.0 {
            draft.light.font_brightness = default_font_brightness();
        }
        if draft.dark.font_brightness <= 0.0 {
            draft.dark.font_brightness = default_font_brightness();
        }

        draft
    }

    fn set_effective_custom_theme(&mut self, draft: CustomThemeConfig) {
        self.cache.custom_theme = draft.clone();
        self.cache.custom_theme_name = draft.theme_name;
    }

    pub fn promote_active_theme_profile_to_custom(
        &mut self,
        light_theme_name: String,
        dark_theme_name: String,
    ) {
        let draft = self.effective_custom_theme();
        if let Some(profile) = self.active_theme_profile_mut() {
            profile.name = draft.theme_name.clone();
            profile.light_theme_name = light_theme_name;
            profile.dark_theme_name = dark_theme_name;
            profile.custom_theme = Some(draft);
        }
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
        self.set_effective_custom_theme(draft);
    }

    pub fn custom_theme_base_name(&self, mode: ThemeMode) -> String {
        let draft = self.effective_custom_theme();
        let mode_cfg = Self::custom_theme_mode_ref(&draft, mode);
        mode_cfg.base_theme_name.trim().to_string()
    }

    pub fn set_custom_theme_base_name(&mut self, mode: ThemeMode, name: &str) {
        let mut draft = self.effective_custom_theme();
        Self::custom_theme_mode_mut(&mut draft, mode).base_theme_name = name.trim().to_string();
        self.set_effective_custom_theme(draft);
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
        self.set_effective_custom_theme(draft);
    }

    pub fn reset_custom_theme_draft(&mut self) {
        let draft = if let Some(custom_theme) = self
            .active_theme_profile()
            .and_then(|profile| profile.custom_theme.clone())
        {
            CustomThemeConfig {
                theme_name: custom_theme.theme_name,
                light: CustomThemeModeConfig {
                    base_theme_name: custom_theme.light.base_theme_name,
                    ..CustomThemeModeConfig::default()
                },
                dark: CustomThemeModeConfig {
                    base_theme_name: custom_theme.dark.base_theme_name,
                    ..CustomThemeModeConfig::default()
                },
            }
        } else if let Some(profile) = self.active_theme_profile() {
            Self::draft_from_theme_profile(profile)
        } else {
            CustomThemeConfig::default()
        };
        self.cache.custom_theme = draft.clone();
        self.cache.custom_primary_color.clear();
        self.cache.custom_background_color.clear();
        self.cache.custom_font_brightness = default_font_brightness();
        self.cache.custom_theme_name = draft.theme_name;
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

    pub fn last_local_sftp_path(&self, session_id: &str) -> Option<&str> {
        self.get(session_id)?;
        self.cache
            .last_local_sftp_paths
            .get(session_id)
            .map(String::as_str)
    }

    pub fn set_last_local_sftp_path(&mut self, session_id: &str, path: &str) -> bool {
        if self.get(session_id).is_none() {
            return false;
        }

        let path = path.trim();
        if path.is_empty() {
            return self.cache.last_local_sftp_paths.remove(session_id).is_some();
        }
        if self.last_local_sftp_path(session_id) == Some(path) {
            return false;
        }

        self.cache
            .last_local_sftp_paths
            .insert(session_id.to_string(), path.to_string());
        true
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
        self.cache.last_local_sftp_paths.remove(id);
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

fn is_plain_builtin_theme_profile(profile: &ThemeProfileConfig) -> bool {
    profile.custom_theme.is_none() && profile.custom_theme_save_path.trim().is_empty()
}

fn deprecated_builtin_theme_profile_replacement(id: &str) -> Option<&'static str> {
    match id.trim() {
        "phyger" => Some("balanced"),
        "soft-moon" => Some("tokyo-moon"),
        "terminal-green" => Some("matrix"),
        _ => None,
    }
}

fn legacy_mode_font_brightness(mode: &CustomThemeModeConfig) -> Option<f32> {
    (!is_default_normalized_font_brightness(mode.font_brightness))
        .then(|| normalize_font_brightness(mode.font_brightness))
}

fn custom_theme_legacy_font_brightness(
    custom_theme: &CustomThemeConfig,
    theme_mode: &str,
) -> Option<f32> {
    let light = legacy_mode_font_brightness(&custom_theme.light);
    let dark = legacy_mode_font_brightness(&custom_theme.dark);

    match (light, dark) {
        (Some(light), Some(dark)) if (light - dark).abs() <= f32::EPSILON => Some(light),
        (Some(_light), Some(dark)) if theme_mode == "dark" => Some(dark),
        (Some(light), Some(_dark)) => Some(light),
        (Some(light), None) => Some(light),
        (None, Some(dark)) => Some(dark),
        (None, None) => None,
    }
}

fn reset_custom_theme_font_brightness(custom_theme: &mut CustomThemeConfig) {
    custom_theme.light.font_brightness = default_font_brightness();
    custom_theme.dark.font_brightness = default_font_brightness();
}

fn normalize_theme_profile_id(id: &str, name: &str) -> String {
    let source = if id.trim().is_empty() { name } else { id };
    let mut slug = String::new();
    for ch in source.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        default_active_theme_profile_id()
    } else {
        slug.to_string()
    }
}

#[cfg(test)]
mod theme_profile_tests {
    use super::{
        ConfigFile, ConfigStore, CustomThemeConfig, CustomThemeModeConfig, ThemeProfileConfig,
        default_font_brightness, default_theme_profiles,
    };
    use gpui_component::ThemeMode;

    #[test]
    fn in_memory_config_has_default_theme_profiles() {
        let config = ConfigStore::in_memory();

        assert_eq!(config.active_theme_profile_id(), "balanced");
        assert!(config.theme_profiles().len() >= default_theme_profiles().len());
        assert!(
            config
                .theme_profiles()
                .iter()
                .any(|profile| profile.name == "Solarized")
        );
        assert!(config.theme_profiles().len() >= default_theme_profiles().len());
        assert!(
            config
                .theme_profiles()
                .iter()
                .any(|profile| profile.name == "Tokyo Storm")
        );
        assert!(
            config
                .theme_profiles()
                .iter()
                .any(|profile| profile.name == "Matrix")
        );
        assert!(
            config
                .theme_profiles()
                .iter()
                .any(|profile| profile.name == "Catppuccin")
        );
        assert!(
            config
                .theme_profiles()
                .iter()
                .any(|profile| profile.name == "Dracula")
        );
        assert!(
            config
                .theme_profiles()
                .iter()
                .any(|profile| profile.name == "Nord")
        );
        assert!(
            config
                .theme_profiles()
                .iter()
                .any(|profile| profile.name == "Rose Pine")
        );
    }

    #[test]
    fn default_theme_profiles_use_distinct_actual_theme_pairs() {
        let profiles = default_theme_profiles();
        let mut pairs = std::collections::HashSet::new();

        for profile in &profiles {
            assert!(
                pairs.insert((
                    profile.light_theme_name.as_str(),
                    profile.dark_theme_name.as_str()
                )),
                "duplicate theme pair in default profile {}",
                profile.name
            );
        }

        let tokyo_storm = profiles
            .iter()
            .find(|profile| profile.id == "tokyo-storm")
            .expect("tokyo storm default profile exists");
        assert_eq!(tokyo_storm.light_theme_name, "Tokyo Storm Light");
        assert_eq!(tokyo_storm.dark_theme_name, "Tokyo Storm");

        let matrix = profiles
            .iter()
            .find(|profile| profile.id == "matrix")
            .expect("matrix default profile exists");
        assert_eq!(matrix.light_theme_name, "Matrix Light");
        assert_eq!(matrix.dark_theme_name, "Matrix");

        let catppuccin = profiles
            .iter()
            .find(|profile| profile.id == "catppuccin")
            .expect("catppuccin default profile exists");
        assert_eq!(catppuccin.light_theme_name, "Catppuccin Latte");
        assert_eq!(catppuccin.dark_theme_name, "Catppuccin Mocha");

        let dracula = profiles
            .iter()
            .find(|profile| profile.id == "dracula")
            .expect("dracula default profile exists");
        assert_eq!(dracula.light_theme_name, "Dracula Alucard");
        assert_eq!(dracula.dark_theme_name, "Dracula");

        let rose_pine_moon = profiles
            .iter()
            .find(|profile| profile.id == "rose-pine-moon")
            .expect("rose pine moon default profile exists");
        assert_eq!(rose_pine_moon.light_theme_name, "Rose Pine Dawn");
        assert_eq!(rose_pine_moon.dark_theme_name, "Rose Pine Moon");

        assert!(
            profiles
                .iter()
                .all(|profile| profile.id != "terminal-green")
        );
    }

    #[test]
    fn saved_builtin_theme_profiles_are_refreshed_to_current_defaults() {
        let mut store = ConfigStore {
            path: std::path::PathBuf::new(),
            cache: ConfigFile {
                active_theme_profile_id: "tokyo-storm".to_string(),
                theme_profiles: vec![ThemeProfileConfig {
                    id: "tokyo-storm".to_string(),
                    name: "Tokyo Storm".to_string(),
                    light_theme_name: "Phyger Light".to_string(),
                    dark_theme_name: "Tokyo Storm".to_string(),
                    custom_theme: None,
                    custom_theme_save_path: String::new(),
                }],
                light_theme_name: "Phyger Light".to_string(),
                dark_theme_name: "Tokyo Storm".to_string(),
                ..ConfigFile::default()
            },
        };

        store.normalize_theme_profiles();

        let profile = store
            .theme_profiles()
            .iter()
            .find(|profile| profile.id == "tokyo-storm")
            .expect("tokyo storm profile exists");
        assert_eq!(profile.light_theme_name, "Tokyo Storm Light");
        assert_eq!(profile.dark_theme_name, "Tokyo Storm");
        assert_eq!(store.light_theme_name(), "Tokyo Storm Light");
        assert_eq!(store.dark_theme_name(), "Tokyo Storm");
    }

    #[test]
    fn deprecated_duplicate_builtin_profiles_move_to_replacements() {
        let mut store = ConfigStore {
            path: std::path::PathBuf::new(),
            cache: ConfigFile {
                active_theme_profile_id: "terminal-green".to_string(),
                theme_profiles: vec![ThemeProfileConfig {
                    id: "terminal-green".to_string(),
                    name: "Terminal Green".to_string(),
                    light_theme_name: "Phyger Light".to_string(),
                    dark_theme_name: "Matrix".to_string(),
                    custom_theme: None,
                    custom_theme_save_path: String::new(),
                }],
                light_theme_name: "Phyger Light".to_string(),
                dark_theme_name: "Matrix".to_string(),
                ..ConfigFile::default()
            },
        };

        store.normalize_theme_profiles();

        assert_eq!(store.active_theme_profile_id(), "matrix");
        assert!(
            store
                .theme_profiles()
                .iter()
                .all(|profile| profile.id != "terminal-green")
        );
        assert_eq!(store.light_theme_name(), "Matrix Light");
        assert_eq!(store.dark_theme_name(), "Matrix");
    }

    #[test]
    fn legacy_theme_names_become_current_profile() {
        let mut store = ConfigStore {
            path: std::path::PathBuf::new(),
            cache: ConfigFile {
                active_theme_profile_id: String::new(),
                theme_profiles: Vec::new(),
                light_theme_name: "Gruvbox Light".to_string(),
                dark_theme_name: "Gruvbox Dark".to_string(),
                ..ConfigFile::default()
            },
        };

        store.normalize_theme_profiles();

        assert_eq!(store.active_theme_profile_id(), "current");
        let active = store.active_theme_profile().expect("active profile exists");
        assert_eq!(active.light_theme_name, "Gruvbox Light");
        assert_eq!(active.dark_theme_name, "Gruvbox Dark");
    }

    #[test]
    fn custom_theme_draft_uses_active_profile_as_base() {
        let mut config = ConfigStore::in_memory();
        config.set_active_theme_profile("solarized");

        let draft = config.custom_theme_draft();

        assert_eq!(draft.theme_name, "Solarized Custom");
        assert_eq!(draft.light.base_theme_name, "Solarized Light");
        assert_eq!(draft.dark.base_theme_name, "Solarized Dark");
    }

    #[test]
    fn saving_theme_preferences_does_not_mutate_builtin_profile() {
        let mut config = ConfigStore::in_memory();
        config.set_active_theme_profile("matrix");

        config.set_theme_preferences(
            true,
            "dark",
            "Matrix Custom [Custom Light]",
            "Matrix Custom [Custom Dark]",
        );

        let matrix = config
            .theme_profiles()
            .iter()
            .find(|profile| profile.id == "matrix")
            .expect("matrix profile exists");
        assert_eq!(matrix.light_theme_name, "Matrix Light");
        assert_eq!(matrix.dark_theme_name, "Matrix");
        assert!(matrix.custom_theme.is_none());
        assert_eq!(config.light_theme_name(), "Matrix Custom [Custom Light]");
        assert_eq!(config.dark_theme_name(), "Matrix Custom [Custom Dark]");
    }

    #[test]
    fn custom_theme_draft_edits_do_not_promote_builtin_profile() {
        let mut config = ConfigStore::in_memory();
        config.set_active_theme_profile("solarized");

        config.set_custom_theme_override(ThemeMode::Dark, "background", "#101820");

        let draft = config.custom_theme_draft();
        assert_eq!(
            draft.dark.overrides.get("background").map(String::as_str),
            Some("#101820")
        );

        let solarized = config
            .theme_profiles()
            .iter()
            .find(|profile| profile.id == "solarized")
            .expect("solarized profile exists");
        assert_eq!(solarized.light_theme_name, "Solarized Light");
        assert_eq!(solarized.dark_theme_name, "Solarized Dark");
        assert!(solarized.custom_theme.is_none());
    }

    #[test]
    fn switching_builtin_profiles_resets_custom_theme_draft_base() {
        let mut config = ConfigStore::in_memory();
        config.set_active_theme_profile("solarized");
        config.set_custom_theme_override(ThemeMode::Dark, "background", "#101820");

        config.set_active_theme_profile("matrix");

        let draft = config.custom_theme_draft();
        assert_eq!(draft.theme_name, "Matrix Custom");
        assert_eq!(draft.light.base_theme_name, "Matrix Light");
        assert_eq!(draft.dark.base_theme_name, "Matrix");
        assert!(!draft.dark.overrides.contains_key("background"));
    }

    #[test]
    fn legacy_custom_font_brightness_becomes_global_terminal_brightness() {
        let mut config = ConfigStore {
            path: std::path::PathBuf::new(),
            cache: ConfigFile {
                theme_profiles: Vec::new(),
                custom_font_brightness: 1.15,
                ..ConfigFile::default()
            },
        };

        config.normalize_theme_profiles();
        config.migrate_global_font_brightness_from_legacy();
        config.sync_custom_theme_draft_to_active_profile();

        assert_eq!(config.active_theme_profile_id(), "balanced");
        assert_eq!(config.terminal_font_brightness(), 1.15);
        assert_eq!(
            config.cache.custom_font_brightness,
            default_font_brightness()
        );
        assert!(
            config
                .active_theme_profile()
                .expect("active profile exists")
                .custom_theme
                .is_none()
        );
    }

    #[test]
    fn legacy_custom_theme_brightness_only_does_not_make_custom_profile() {
        let mut config = ConfigStore {
            path: std::path::PathBuf::new(),
            cache: ConfigFile {
                theme_mode: "dark".to_string(),
                theme_profiles: Vec::new(),
                custom_theme: CustomThemeConfig {
                    light: CustomThemeModeConfig {
                        font_brightness: 1.05,
                        ..CustomThemeModeConfig::default()
                    },
                    dark: CustomThemeModeConfig {
                        font_brightness: 1.18,
                        ..CustomThemeModeConfig::default()
                    },
                    ..CustomThemeConfig::default()
                },
                ..ConfigFile::default()
            },
        };

        config.normalize_theme_profiles();
        config.migrate_global_font_brightness_from_legacy();
        config.sync_custom_theme_draft_to_active_profile();

        assert_eq!(config.active_theme_profile_id(), "balanced");
        assert_eq!(config.terminal_font_brightness(), 1.18);
        let draft = config.custom_theme_draft();
        assert_eq!(draft.light.font_brightness, default_font_brightness());
        assert_eq!(draft.dark.font_brightness, default_font_brightness());
    }

    #[test]
    fn imported_theme_profile_is_unique_and_active() {
        let mut config = ConfigStore::in_memory();

        let first = config.activate_imported_theme_profile(
            "Imported Theme".to_string(),
            "Imported Light".to_string(),
            "Imported Dark".to_string(),
        );
        let second = config.activate_imported_theme_profile(
            "Imported Theme".to_string(),
            "Imported Light 2".to_string(),
            "Imported Dark 2".to_string(),
        );

        assert_ne!(first.id, second.id);
        assert_eq!(config.active_theme_profile_id(), second.id);
        assert_eq!(config.light_theme_name(), "Imported Light 2");
        assert_eq!(config.dark_theme_name(), "Imported Dark 2");
    }
}

#[cfg(test)]
mod font_brightness_settings_tests {
    use super::{ConfigFile, ConfigStore, default_font_brightness};

    #[test]
    fn font_brightness_defaults_for_existing_configs() {
        let config: ConfigFile = serde_json::from_str("{}").expect("config should deserialize");

        assert_eq!(config.ui_font_brightness, default_font_brightness());
        assert_eq!(config.terminal_font_brightness, default_font_brightness());
    }

    #[test]
    fn font_brightness_settings_are_clamped() {
        let mut config = ConfigStore::in_memory();

        config.set_ui_font_brightness(9.0);
        config.set_terminal_font_brightness(0.1);

        assert_eq!(config.ui_font_brightness(), 1.2);
        assert_eq!(config.terminal_font_brightness(), 0.6);
    }
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

#[cfg(test)]
mod local_sftp_path_tests {
    use crate::session::Session;

    use super::{ConfigFile, ConfigStore};

    fn saved_session(id: &str) -> Session {
        let mut session = Session::password(
            "example.com".to_string(),
            22,
            "user".to_string(),
            "password".to_string(),
        );
        session.id = id.to_string();
        session
    }

    #[test]
    fn local_sftp_paths_default_to_empty_for_existing_configs() {
        let config: ConfigFile = serde_json::from_str("{}").expect("config should deserialize");

        assert!(config.last_local_sftp_paths.is_empty());
    }

    #[test]
    fn local_sftp_path_requires_a_saved_session() {
        let mut config = ConfigStore::in_memory();

        assert!(!config.set_last_local_sftp_path("temporary", "/tmp/temporary"));
        config.upsert(saved_session("saved"));
        assert!(config.set_last_local_sftp_path("saved", "/tmp/saved"));
        assert_eq!(config.last_local_sftp_path("saved"), Some("/tmp/saved"));
    }

    #[test]
    fn removing_a_session_removes_its_local_sftp_path() {
        let mut config = ConfigStore::in_memory();
        config.upsert(saved_session("saved"));
        assert!(config.set_last_local_sftp_path("saved", "/tmp/saved"));

        config.remove("saved");

        assert_eq!(config.last_local_sftp_path("saved"), None);
    }

    #[test]
    fn replacing_sessions_prunes_removed_local_sftp_paths() {
        let mut config = ConfigStore::in_memory();
        config.replace_sessions(vec![saved_session("keep"), saved_session("remove")]);
        assert!(config.set_last_local_sftp_path("keep", "/tmp/keep"));
        assert!(config.set_last_local_sftp_path("remove", "/tmp/remove"));

        config.replace_sessions(vec![saved_session("keep")]);

        assert_eq!(config.last_local_sftp_path("keep"), Some("/tmp/keep"));
        assert_eq!(config.last_local_sftp_path("remove"), None);
    }
}
