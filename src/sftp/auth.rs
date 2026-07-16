use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use russh::{
    Disconnect,
    client::{self, Handler},
};

use crate::{
    backend::{
        auth::{load_session_private_key, private_keys_with_algs},
        host_key::HostKeyVerifier,
        ssh::{
            connection::connect_transport_with_retries, negotiation_error_details,
            ssh_client_config,
        },
    },
    diagnostics::{mask_host, mask_value, sanitize_error_with_values},
    events::BackendEventSender,
    session::{AuthMethod, Session, SshConnectionMode},
};

fn sanitize_session_error(message: &str, session: &Session) -> String {
    sanitize_error_with_values(
        message,
        &[
            &session.user,
            &session.host,
            &session.private_key_path,
            &session.password,
            &session.passphrase,
            &session.proxy_host,
            &session.proxy_user,
            &session.proxy_password,
        ],
    )
}

pub(super) async fn connect_and_authenticate(
    tab_id: &str,
    session: &Session,
    events: &BackendEventSender,
) -> Result<Arc<russh::client::Handle<SftpClientHandler>>> {
    let addr = format!("{}:{}", session.host, session.port);
    let log_host = mask_host(&session.host);
    let log_user = mask_value(&session.user);
    tracing::info!(
        component = "sftp",
        operation = "connect",
        host = %log_host,
        port = session.port,
        user = %log_user,
        "Initiating SFTP SSH connection"
    );
    let mut handle = connect_with_selected_mode(tab_id, session, &addr, events).await?;

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
                            component = "sftp",
                            operation = "authenticate_key",
                            host = %log_host,
                            port = session.port,
                            user = %log_user,
                            "SFTP public key algorithm was rejected; trying next"
                        );
                        continue;
                    }
                    Err(e) => {
                        tracing::debug!(
                            component = "sftp",
                            operation = "authenticate_key",
                            host = %log_host,
                            port = session.port,
                            user = %log_user,
                            error = %sanitize_session_error(&e.to_string(), session),
                            "SFTP public key algorithm failed; trying next"
                        );
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
        tracing::warn!(
            component = "sftp",
            operation = "authenticate",
            host = %log_host,
            port = session.port,
            user = %log_user,
            "SFTP authentication failed"
        );
        if let Err(err) = handle
            .disconnect(Disconnect::ByApplication, "auth failed", "")
            .await
        {
            tracing::debug!(
                component = "sftp",
                operation = "disconnect",
                host = %log_host,
                port = session.port,
                error = %sanitize_session_error(&err.to_string(), session),
                "SFTP disconnect after authentication failure was rejected"
            );
        }
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

    tracing::info!(
        component = "sftp",
        operation = "authenticate",
        host = %log_host,
        port = session.port,
        user = %log_user,
        "SFTP authentication succeeded"
    );

    Ok(Arc::new(handle))
}

async fn connect_with_selected_mode(
    tab_id: &str,
    session: &Session,
    addr: &str,
    events: &BackendEventSender,
) -> Result<russh::client::Handle<SftpClientHandler>> {
    let mode = session.ssh_connection_mode();
    if mode == SshConnectionMode::Legacy {
        tracing::warn!(
            component = "sftp",
            operation = "connect",
            host = %mask_host(&session.host),
            port = session.port,
            "Using explicitly enabled legacy SSH compatibility algorithms"
        );
    }

    match connect_with_mode(tab_id, session, addr, mode, events).await {
        Ok(handle) => Ok(handle),
        Err(err) => {
            let failure = negotiation_error_details(&err).unwrap_or_else(|| format!("{err:#}"));
            tracing::warn!(
                component = "sftp",
                operation = "connect_mode",
                host = %mask_host(&session.host),
                port = session.port,
                user = %mask_value(&session.user),
                mode = mode.label(),
                error = %sanitize_session_error(&failure, session),
                "SFTP SSH connection mode failed without a downgrade retry"
            );
            Err(anyhow!(
                "sftp SSH connection failed in {} mode: {failure}",
                mode.label()
            ))
        }
    }
}

async fn connect_with_mode(
    tab_id: &str,
    session: &Session,
    addr: &str,
    mode: SshConnectionMode,
    events: &BackendEventSender,
) -> Result<russh::client::Handle<SftpClientHandler>> {
    let stream = connect_transport_with_retries(None, session, addr, mode, None).await?;
    client::connect_stream(
        Arc::new(ssh_client_config(mode)),
        stream,
        SftpClientHandler {
            host_key_verifier: HostKeyVerifier::new(
                tab_id,
                &session.host,
                session.port,
                events.clone(),
            ),
        },
    )
    .await
    .with_context(|| format!("connect {addr} failed in {} mode", mode.label()))
}

#[derive(Clone)]
pub(super) struct SftpClientHandler {
    host_key_verifier: HostKeyVerifier,
}

impl Handler for SftpClientHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &russh::keys::ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        self.host_key_verifier.verify(server_public_key).await
    }
}
