use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use russh::{
    Disconnect,
    client::{self, Handler},
};

use crate::{
    backend::{
        auth::{load_session_private_key, private_keys_with_algs},
        ssh::{
            connection::connect_transport_with_retries, negotiation_error_details,
            ssh_client_config,
        },
    },
    session::{AuthMethod, Session, SshConnectionMode, ordered_ssh_connection_modes},
};

pub(super) async fn connect_and_authenticate(
    session: &Session,
) -> Result<(
    Arc<russh::client::Handle<SftpClientHandler>>,
    SshConnectionMode,
)> {
    let addr = format!("{}:{}", session.host, session.port);
    let (mut handle, connected_mode) = connect_with_mode_priority(session, &addr).await?;

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

    Ok((Arc::new(handle), connected_mode))
}

async fn connect_with_mode_priority(
    session: &Session,
    addr: &str,
) -> Result<(russh::client::Handle<SftpClientHandler>, SshConnectionMode)> {
    let modes = ordered_ssh_connection_modes(session.last_successful_ssh_mode);
    let mut failures = Vec::new();

    for (index, mode) in modes.iter().copied().enumerate() {
        match connect_with_mode(session, addr, mode).await {
            Ok(handle) => return Ok((handle, mode)),
            Err(err) => {
                let details = negotiation_error_details(&err);
                let failure = details.clone().unwrap_or_else(|| format!("{err:#}"));
                tracing::warn!(
                    "[sftp] {} mode connection failed for {}@{}: {}",
                    mode.label(),
                    session.user,
                    addr,
                    failure
                );
                failures.push(format!("{} mode: {failure}", mode.label()));

                let should_try_next = index + 1 < modes.len()
                    && (details.is_some() || session.last_successful_ssh_mode == Some(mode));
                if !should_try_next {
                    return Err(anyhow!(
                        "sftp SSH connection failed. {}",
                        failures.join(". ")
                    ));
                }
            }
        }
    }

    Err(anyhow!(
        "sftp SSH connection failed before any mode was attempted"
    ))
}

async fn connect_with_mode(
    session: &Session,
    addr: &str,
    mode: SshConnectionMode,
) -> Result<russh::client::Handle<SftpClientHandler>> {
    let stream = connect_transport_with_retries(None, session, addr, mode, None).await?;
    client::connect_stream(Arc::new(ssh_client_config(mode)), stream, SftpClientHandler)
        .await
        .with_context(|| format!("connect {addr} failed in {} mode", mode.label()))
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
