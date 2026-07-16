use std::time::Duration;

use anyhow::{Context, Result};
use russh::keys::ssh_key::{HashAlg, PublicKey};
use tokio::sync::oneshot;

use crate::{
    config::{ConfigStore, HostKeyTrust, TrustedHostKey},
    events::{BackendEvent, BackendEventSender},
};

const HOST_KEY_DECISION_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostKeyPromptKind {
    FirstSeen,
    Changed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostKeyDecision {
    Reject,
    Trust,
}

#[derive(Debug)]
pub(crate) struct HostKeyVerificationRequest {
    pub(crate) tab_id: String,
    pub(crate) key: TrustedHostKey,
    pub(crate) kind: HostKeyPromptKind,
    pub(crate) response: oneshot::Sender<HostKeyDecision>,
}

#[derive(Clone)]
pub(crate) struct HostKeyVerifier {
    tab_id: String,
    host: String,
    port: u16,
    events: BackendEventSender,
}

impl HostKeyVerifier {
    pub(crate) fn new(
        tab_id: impl Into<String>,
        host: impl AsRef<str>,
        port: u16,
        events: BackendEventSender,
    ) -> Self {
        Self {
            tab_id: tab_id.into(),
            host: normalize_host(host.as_ref()),
            port,
            events,
        }
    }

    pub(crate) async fn verify(&self, server_public_key: &PublicKey) -> Result<bool> {
        let key = TrustedHostKey {
            host: self.host.clone(),
            port: self.port,
            algorithm: server_public_key.algorithm().to_string(),
            public_key: server_public_key
                .to_openssh()
                .context("encode SSH server host key")?,
            fingerprint: server_public_key.fingerprint(HashAlg::Sha256).to_string(),
        };
        let trust = ConfigStore::load()
            .context("load SSH host key trust store")?
            .host_key_trust(&key);
        let kind = match trust {
            HostKeyTrust::Trusted => return Ok(true),
            HostKeyTrust::Unknown => HostKeyPromptKind::FirstSeen,
            HostKeyTrust::Changed => HostKeyPromptKind::Changed,
        };
        let (response, receiver) = oneshot::channel();
        self.events
            .send(BackendEvent::HostKeyVerification {
                request: HostKeyVerificationRequest {
                    tab_id: self.tab_id.clone(),
                    key,
                    kind,
                    response,
                },
            })
            .await
            .context("request SSH host key confirmation")?;

        Ok(matches!(
            tokio::time::timeout(HOST_KEY_DECISION_TIMEOUT, receiver).await,
            Ok(Ok(HostKeyDecision::Trust))
        ))
    }
}

fn normalize_host(host: &str) -> String {
    host.trim().trim_end_matches('.').to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::normalize_host;

    #[test]
    fn normalizes_hostnames_for_trust_lookup() {
        assert_eq!(normalize_host(" Example.COM. "), "example.com");
        assert_eq!(normalize_host("2001:db8::1"), "2001:db8::1");
    }
}
