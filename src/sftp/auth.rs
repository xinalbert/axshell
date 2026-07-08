use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use russh::{
    Disconnect,
    client::{self, Handler},
};

use crate::{
    backend::auth::{load_session_private_key, private_keys_with_algs},
    session::config::{AuthMethod, Session},
};

pub(super) async fn connect_and_authenticate(
    session: &Session,
) -> Result<Arc<russh::client::Handle<SftpClientHandler>>> {
    let config = Arc::new(client::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(600)),
        keepalive_interval: Some(std::time::Duration::from_secs(3)),
        keepalive_max: 2,
        ..Default::default()
    });
    let addr = format!("{}:{}", session.host, session.port);
    let stream = crate::session::config::connect_proxy(session).await?;
    let mut handle = client::connect_stream(config, stream, SftpClientHandler)
        .await
        .with_context(|| format!("connect {addr} failed"))?;

    let authed = match session.auth {
        AuthMethod::Password => handle
            .authenticate_password(&session.user, &session.password)
            .await
            .context("password authentication failed")?
            .success(),
        AuthMethod::Key => {
            let keypair = load_session_private_key(session)?;
            let keys = private_keys_with_algs(keypair).context("invalid private key")?;
            let mut success = false;
            for key in keys {
                match handle.authenticate_publickey(&session.user, key).await {
                    Ok(result) if result.success() => {
                        success = true;
                        break;
                    }
                    Ok(_) => {
                        tracing::debug!(
                            "[sftp] public key auth failed with algorithm, trying next"
                        );
                        continue;
                    }
                    Err(e) => {
                        tracing::debug!("[sftp] public key auth error: {:?}, trying next", e);
                        continue;
                    }
                }
            }
            if !success {
                return Err(anyhow!(
                    "public key authentication failed for {}@{}:{}",
                    session.user,
                    session.host,
                    session.port
                ));
            }
            success
        }
    };

    if !authed {
        let _ = handle
            .disconnect(Disconnect::ByApplication, "auth failed", "")
            .await;
        return Err(anyhow!(
            "authentication failed: server rejected {} authentication for {}@{}:{}",
            match session.auth {
                AuthMethod::Password => "password",
                AuthMethod::Key => "public key",
            },
            session.user,
            session.host,
            session.port
        ));
    }

    Ok(Arc::new(handle))
}

#[derive(Clone)]
pub(super) struct SftpClientHandler;

impl Handler for SftpClientHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}
