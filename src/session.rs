use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn default_global_proxy_type() -> String {
    "socks5".to_string()
}

fn default_x11_forwarding() -> bool {
    true
}

fn default_session_kind() -> SessionKind {
    SessionKind::Ssh
}

fn default_serial_baud_rate() -> u32 {
    115_200
}

fn default_serial_data_bits() -> u8 {
    8
}

fn default_serial_stop_bits() -> u8 {
    1
}

fn default_serial_parity() -> String {
    "none".to_string()
}

fn default_serial_flow_control() -> String {
    "none".to_string()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum SessionKind {
    Ssh,
    Serial,
    Telnet,
}

impl SessionKind {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Ssh => "SSH",
            Self::Serial => "Serial",
            Self::Telnet => "Telnet",
        }
    }

    pub(crate) fn supports_sftp(self) -> bool {
        matches!(self, Self::Ssh)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum AuthMethod {
    Password,
    Key,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Session {
    pub(crate) id: String,
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) group_name: String,
    #[serde(default = "default_session_kind")]
    pub(crate) kind: SessionKind,
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
    pub(crate) legacy_ssh_compatibility: bool,
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
    #[serde(default)]
    pub(crate) sftp_path: String,
    #[serde(default = "default_x11_forwarding")]
    pub(crate) x11_forwarding: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub(crate) shortcut: String,
    #[serde(default)]
    pub(crate) serial_port: String,
    #[serde(default = "default_serial_baud_rate")]
    pub(crate) baud_rate: u32,
    #[serde(default = "default_serial_data_bits")]
    pub(crate) data_bits: u8,
    #[serde(default = "default_serial_parity")]
    pub(crate) parity: String,
    #[serde(default = "default_serial_stop_bits")]
    pub(crate) stop_bits: u8,
    #[serde(default = "default_serial_flow_control")]
    pub(crate) flow_control: String,
}

impl Session {
    pub(crate) fn password(host: String, port: u16, user: String, password: String) -> Self {
        let name = format!("{user}@{host}");
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            group_name: String::new(),
            kind: SessionKind::Ssh,
            host,
            port,
            user,
            auth: AuthMethod::Password,
            password,
            private_key_path: String::new(),
            private_key_inline: String::new(),
            passphrase: String::new(),
            last_used: None,
            legacy_ssh_compatibility: false,
            proxy_type: "none".to_string(),
            proxy_host: String::new(),
            proxy_port: None,
            proxy_user: String::new(),
            proxy_password: String::new(),
            sftp_path: String::new(),
            x11_forwarding: default_x11_forwarding(),
            shortcut: String::new(),
            serial_port: String::new(),
            baud_rate: default_serial_baud_rate(),
            data_bits: default_serial_data_bits(),
            parity: default_serial_parity(),
            stop_bits: default_serial_stop_bits(),
            flow_control: default_serial_flow_control(),
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
            kind: SessionKind::Ssh,
            host,
            port,
            user,
            auth: AuthMethod::Key,
            password: String::new(),
            private_key_path,
            private_key_inline,
            passphrase,
            last_used: None,
            legacy_ssh_compatibility: false,
            proxy_type: "none".to_string(),
            proxy_host: String::new(),
            proxy_port: None,
            proxy_user: String::new(),
            proxy_password: String::new(),
            sftp_path: String::new(),
            x11_forwarding: default_x11_forwarding(),
            shortcut: String::new(),
            serial_port: String::new(),
            baud_rate: default_serial_baud_rate(),
            data_bits: default_serial_data_bits(),
            parity: default_serial_parity(),
            stop_bits: default_serial_stop_bits(),
            flow_control: default_serial_flow_control(),
        }
    }

    pub(crate) fn serial(serial_port: String, baud_rate: u32) -> Self {
        let name = serial_port.clone();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            group_name: String::new(),
            kind: SessionKind::Serial,
            host: String::new(),
            port: 0,
            user: String::new(),
            auth: AuthMethod::Password,
            password: String::new(),
            private_key_path: String::new(),
            private_key_inline: String::new(),
            passphrase: String::new(),
            last_used: None,
            legacy_ssh_compatibility: false,
            proxy_type: "none".to_string(),
            proxy_host: String::new(),
            proxy_port: None,
            proxy_user: String::new(),
            proxy_password: String::new(),
            sftp_path: String::new(),
            x11_forwarding: false,
            shortcut: String::new(),
            serial_port,
            baud_rate: baud_rate.max(1),
            data_bits: default_serial_data_bits(),
            parity: default_serial_parity(),
            stop_bits: default_serial_stop_bits(),
            flow_control: default_serial_flow_control(),
        }
    }

    pub(crate) fn telnet(host: String, port: u16) -> Self {
        let name = format!("telnet://{host}:{port}");
        let mut session = Self::password(host, port, String::new(), String::new());
        session.name = name;
        session.kind = SessionKind::Telnet;
        session.x11_forwarding = false;
        session
    }

    pub(crate) fn ssh_connection_mode(&self) -> SshConnectionMode {
        if self.legacy_ssh_compatibility {
            SshConnectionMode::Legacy
        } else {
            SshConnectionMode::Default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Session, SessionKind, SshConnectionMode};

    #[test]
    fn legacy_ssh_mode_requires_explicit_session_opt_in() {
        let mut session =
            Session::password("example.com".into(), 22, "root".into(), "password".into());
        assert_eq!(session.ssh_connection_mode(), SshConnectionMode::Default);

        session.legacy_ssh_compatibility = true;
        assert_eq!(session.ssh_connection_mode(), SshConnectionMode::Legacy);
    }

    #[test]
    fn new_session_fields_default_when_loading_existing_sessions() {
        let session =
            super::Session::password("example.com".into(), 22, "root".into(), "password".into());
        let mut value = serde_json::to_value(session).expect("session serializes");
        value
            .as_object_mut()
            .expect("session is an object")
            .remove("sftp_path");
        value
            .as_object_mut()
            .expect("session is an object")
            .remove("x11_forwarding");
        value
            .as_object_mut()
            .expect("session is an object")
            .remove("shortcut");
        value
            .as_object_mut()
            .expect("session is an object")
            .remove("kind");
        value
            .as_object_mut()
            .expect("session is an object")
            .remove("legacy_ssh_compatibility");
        value
            .as_object_mut()
            .expect("session is an object")
            .remove("baud_rate");
        value.as_object_mut().expect("session is an object").insert(
            "last_successful_ssh_mode".to_string(),
            serde_json::json!("legacy"),
        );

        let session: super::Session =
            serde_json::from_value(value).expect("existing session deserializes");

        assert!(session.sftp_path.is_empty());
        assert!(session.x11_forwarding);
        assert!(session.shortcut.is_empty());
        assert_eq!(session.kind, SessionKind::Ssh);
        assert_eq!(session.baud_rate, 115_200);
        assert!(!session.legacy_ssh_compatibility);
        let serialized = serde_json::to_value(session).expect("session should serialize");
        assert!(serialized.get("last_successful_ssh_mode").is_none());
    }

    #[test]
    fn serial_session_uses_safe_console_defaults() {
        let session = super::Session::serial("/dev/tty.usbserial-1".into(), 115_200);

        assert_eq!(session.kind, SessionKind::Serial);
        assert_eq!(session.data_bits, 8);
        assert_eq!(session.parity, "none");
        assert_eq!(session.stop_bits, 1);
        assert_eq!(session.flow_control, "none");
        assert!(!session.kind.supports_sftp());
    }
}
