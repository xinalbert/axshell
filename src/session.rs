use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn default_global_proxy_type() -> String {
    "socks5".to_string()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum AuthMethod {
    Password,
    Key,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum SshConnectionMode {
    Default,
    Legacy,
}

impl SshConnectionMode {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Legacy => "legacy compatibility",
        }
    }
}

pub(crate) fn ordered_ssh_connection_modes(
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
pub(crate) struct Session {
    pub(crate) id: String,
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) group_name: String,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) user: String,
    pub(crate) auth: AuthMethod,
    #[serde(default)]
    pub(crate) password: String,
    #[serde(default)]
    pub(crate) private_key_path: String,
    #[serde(default)]
    pub(crate) private_key_inline: String,
    #[serde(default)]
    pub(crate) passphrase: String,
    #[serde(default)]
    pub(crate) last_used: Option<String>,
    #[serde(default)]
    pub(crate) last_successful_ssh_mode: Option<SshConnectionMode>,
    #[serde(default = "default_global_proxy_type")]
    pub(crate) proxy_type: String,
    #[serde(default)]
    pub(crate) proxy_host: String,
    #[serde(default)]
    pub(crate) proxy_port: Option<u16>,
    #[serde(default)]
    pub(crate) proxy_user: String,
    #[serde(default)]
    pub(crate) proxy_password: String,
}

impl Session {
    pub(crate) fn password(host: String, port: u16, user: String, password: String) -> Self {
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

    pub(crate) fn key(
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
