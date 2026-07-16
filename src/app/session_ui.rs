use gpui::SharedString;

use crate::session::{AuthMethod, Session};

#[derive(Clone)]
pub(crate) enum SelectorEntry {
    Local,
    NewSsh,
    Saved(String),
}

#[derive(Clone)]
pub(crate) struct ConnectionProgress {
    pub(crate) tab_id: String,
    pub(crate) title: SharedString,
    pub(crate) lines: Vec<SharedString>,
    pub(crate) failed: bool,
}

#[derive(Clone)]
pub(crate) struct TerminalPasswordPrompt {
    pub(crate) tab_id: String,
    pub(crate) password: String,
}

impl TerminalPasswordPrompt {
    pub(crate) fn new(tab_id: String) -> Self {
        Self {
            tab_id,
            password: String::new(),
        }
    }
}

pub(crate) fn should_use_terminal_password_prompt(
    session: &Session,
    reason: &str,
    password_from_terminal_prompt: bool,
) -> bool {
    matches!(session.auth, AuthMethod::Password)
        && (session.password.is_empty() || password_from_terminal_prompt)
        && is_password_auth_rejection(reason)
}

pub(crate) fn should_prompt_for_terminal_password_before_connect(session: &Session) -> bool {
    matches!(session.auth, AuthMethod::Password)
        && session.password.is_empty()
        && session.private_key_path.trim().is_empty()
        && session.private_key_inline.trim().is_empty()
}

fn is_password_auth_rejection(reason: &str) -> bool {
    let reason = reason.to_ascii_lowercase();
    reason.contains("password authentication failed")
        || reason.contains("server rejected password authentication")
}

#[cfg(test)]
mod tests {
    use crate::{
        app::session_ui::{
            should_prompt_for_terminal_password_before_connect, should_use_terminal_password_prompt,
        },
        session::Session,
    };

    fn password_session(password: &str) -> Session {
        Session::password("example.com".into(), 22, "root".into(), password.into())
    }

    #[test]
    fn terminal_password_prompt_requires_empty_password_auth_rejection() {
        let session = password_session("");

        assert!(should_use_terminal_password_prompt(
            &session,
            "authentication failed: server rejected password authentication for root@example.com:22",
            false,
        ));
        assert!(!should_use_terminal_password_prompt(
            &session,
            "tcp connect failed: connection refused",
            false,
        ));
    }

    #[test]
    fn terminal_password_prompt_skips_saved_password_and_key_auth() {
        let session = password_session("secret");
        assert!(!should_use_terminal_password_prompt(
            &session,
            "authentication failed: server rejected password authentication for root@example.com:22",
            false,
        ));

        let key_session = Session::key(
            "example.com".into(),
            22,
            "root".into(),
            "~/.ssh/id_ed25519".into(),
            String::new(),
            String::new(),
        );
        assert!(!should_use_terminal_password_prompt(
            &key_session,
            "authentication failed: server rejected password authentication for root@example.com:22",
            false,
        ));
    }

    #[test]
    fn terminal_password_prompt_allows_retrying_password_entered_in_terminal() {
        let session = password_session("wrong");

        assert!(should_use_terminal_password_prompt(
            &session,
            "password authentication failed",
            true,
        ));
    }

    #[test]
    fn terminal_password_prompt_before_connect_requires_no_auth_secret() {
        assert!(should_prompt_for_terminal_password_before_connect(
            &password_session("")
        ));
        assert!(!should_prompt_for_terminal_password_before_connect(
            &password_session("secret")
        ));

        let key_session = Session::key(
            "example.com".into(),
            22,
            "root".into(),
            "~/.ssh/id_ed25519".into(),
            String::new(),
            String::new(),
        );
        assert!(!should_prompt_for_terminal_password_before_connect(
            &key_session
        ));
    }
}
