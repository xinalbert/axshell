use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn default_global_proxy_type() -> String {
    "socks5".to_string()
}

fn default_custom_font_brightness() -> f32 {
    1.0
}

fn default_custom_theme_name() -> String {
    "Custom Theme".to_string()
}

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
    pub proxy_type: String,
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
