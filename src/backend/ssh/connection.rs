use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use russh::{Disconnect, client};

use crate::{
    backend::auth::{load_session_private_key, private_keys_with_algs},
    session::config::{AuthMethod, Session, SshConnectionMode, ordered_ssh_connection_modes},
    terminal::BackendEvent,
};

use super::{ClientHandler, X11ForwardingState, legacy};

pub(super) async fn connect_and_authenticate(
    tab_id: &str,
    session: &Session,
    events: &std::sync::mpsc::Sender<BackendEvent>,
    x11: Option<Arc<X11ForwardingState>>,
) -> Result<russh::client::Handle<ClientHandler>> {
    let addr = format!("{}:{}", session.host, session.port);
    tracing::info!(
        "[ssh] initiating tcp connection to {} (user: {})",
        addr,
        session.user
    );
    let status_text =
        if let Some((ptype, phost, pport)) = crate::session::config::active_proxy(session) {
            let pport_val = pport.unwrap_or_else(|| if ptype == "http" { 8080 } else { 1080 });
            format!(
                "connecting to {addr} via {} proxy {}:{}",
                ptype.to_uppercase(),
                phost,
                pport_val
            )
        } else {
            format!("opening tcp connection to {addr}")
        };
    let _ = events.send(BackendEvent::Status {
        tab_id: tab_id.to_string(),
        text: status_text,
    });

    let (mut handle, connected_mode) =
        connect_with_mode_priority(tab_id, session, &addr, events, x11.clone()).await?;

    tracing::debug!("[ssh] tcp connected to {}", addr);

    let authed = match session.auth {
        AuthMethod::Password => {
            tracing::info!(
                "[ssh] sending password authentication for {}@{}",
                session.user,
                addr
            );
            let _ = events.send(BackendEvent::Status {
                tab_id: tab_id.to_string(),
                text: format!(
                    "connected to {addr}, sending password authentication for {}",
                    session.user
                ),
            });
            handle
                .authenticate_password(&session.user, &session.password)
                .await
                .context("password authentication failed")?
                .success()
        }
        AuthMethod::Key => {
            let source = key_source_label(session);
            tracing::info!(
                "[ssh] sending key authentication for {}@{} (key source: {})",
                session.user,
                addr,
                source
            );
            let _ = events.send(BackendEvent::Status {
                tab_id: tab_id.to_string(),
                text: format!("connected to {addr}, loading private key from {source}"),
            });
            let keypair = load_session_private_key(session)?;
            let algorithm = format!("{:?}", keypair.algorithm());
            let _ = events.send(BackendEvent::Status {
                tab_id: tab_id.to_string(),
                text: format!("private key loaded from {source}, algorithm {algorithm}, sending public key authentication for {}", session.user),
            });
            let keys = private_keys_with_algs(keypair).context("invalid private key")?;
            let mut success = false;
            for key in keys {
                match handle.authenticate_publickey(&session.user, key).await {
                    Ok(result) if result.success() => {
                        success = true;
                        break;
                    }
                    Ok(_) => {
                        tracing::debug!("[ssh] public key auth failed with algorithm, trying next");
                        continue;
                    }
                    Err(e) => {
                        tracing::debug!("[ssh] public key auth error: {:?}, trying next", e);
                        continue;
                    }
                }
            }
            if !success {
                return Err(anyhow::anyhow!(
                    "public key authentication failed for {}@{}:{} using {} ({})",
                    session.user,
                    session.host,
                    session.port,
                    source,
                    algorithm
                ));
            }
            success
        }
    };

    if !authed {
        tracing::warn!("[ssh] authentication failed for {}@{}", session.user, addr);
        let _ = handle
            .disconnect(Disconnect::ByApplication, "auth failed", "")
            .await;
        return Err(anyhow!(
            "{}",
            match session.auth {
                AuthMethod::Password => format!(
                    "authentication failed: server rejected password authentication for {}@{}:{}",
                    session.user, session.host, session.port
                ),
                AuthMethod::Key => format!(
                    "authentication failed: server rejected public key authentication for {}@{}:{} using {}",
                    session.user,
                    session.host,
                    session.port,
                    key_source_label(session)
                ),
            }
        ));
    }

    tracing::info!(
        "[ssh] authentication successful for {}@{}",
        session.user,
        addr
    );

    let _ = events.send(BackendEvent::Status {
        tab_id: tab_id.to_string(),
        text: format!(
            "authentication accepted, opening shell for {}@{}",
            session.user, session.host
        ),
    });
    let _ = events.send(BackendEvent::SshConnectionModeResolved {
        tab_id: tab_id.to_string(),
        session_id: session.id.clone(),
        mode: connected_mode,
    });

    Ok(handle)
}

async fn connect_with_mode_priority(
    tab_id: &str,
    session: &Session,
    addr: &str,
    events: &std::sync::mpsc::Sender<BackendEvent>,
    x11: Option<Arc<X11ForwardingState>>,
) -> Result<(russh::client::Handle<ClientHandler>, SshConnectionMode)> {
    let modes = ordered_ssh_connection_modes(session.last_successful_ssh_mode);
    let mut failures = Vec::new();

    for (index, mode) in modes.iter().copied().enumerate() {
        if index > 0 {
            let _ = events.send(BackendEvent::Status {
                tab_id: tab_id.to_string(),
                text: format!("retrying SSH connection in {} mode", mode.label()),
            });
        }

        match connect_with_mode(session, addr, mode, x11.clone()).await {
            Ok(handle) => {
                if mode == SshConnectionMode::Legacy {
                    tracing::warn!(
                        "[ssh] connected to {} using legacy SSH compatibility mode",
                        addr
                    );
                    let _ = events.send(BackendEvent::Status {
                        tab_id: tab_id.to_string(),
                        text: format!("connected to {addr} using legacy SSH compatibility mode"),
                    });
                } else if session.last_successful_ssh_mode == Some(SshConnectionMode::Legacy) {
                    let _ = events.send(BackendEvent::Status {
                        tab_id: tab_id.to_string(),
                        text: format!("connected to {addr} using default SSH mode"),
                    });
                }
                return Ok((handle, mode));
            }
            Err(err) => {
                let details = legacy::negotiation_error_details(&err);
                let failure = details.clone().unwrap_or_else(|| format!("{err:#}"));
                tracing::warn!(
                    "[ssh] {} mode connection failed for {}@{}: {}",
                    mode.label(),
                    session.user,
                    addr,
                    failure
                );
                failures.push(format!("{} mode: {failure}", mode.label()));

                let should_try_next = index + 1 < modes.len()
                    && (details.is_some() || session.last_successful_ssh_mode == Some(mode));
                if !should_try_next {
                    return Err(anyhow!("SSH connection failed. {}", failures.join(". ")));
                }

                let next = modes[index + 1];
                let reason = legacy::negotiation_error_short_reason(&err)
                    .map(|reason| format!("SSH negotiation failed ({reason})"))
                    .unwrap_or_else(|| "SSH connection failed with cached mode".to_string());
                let _ = events.send(BackendEvent::Status {
                    tab_id: tab_id.to_string(),
                    text: format!("{} {reason}, retrying {} mode", mode.label(), next.label()),
                });
            }
        }
    }

    Err(anyhow!(
        "SSH connection failed before any mode was attempted"
    ))
}

async fn connect_with_mode(
    session: &Session,
    addr: &str,
    mode: SshConnectionMode,
    x11: Option<Arc<X11ForwardingState>>,
) -> Result<russh::client::Handle<ClientHandler>> {
    let stream = crate::session::config::connect_proxy(session).await?;
    client::connect_stream(
        Arc::new(legacy::ssh_client_config(mode)),
        stream,
        ClientHandler::new(x11),
    )
    .await
    .with_context(|| format!("connect {addr} failed in {} mode", mode.label()))
}

fn key_source_label(session: &Session) -> String {
    let path = session.private_key_path.trim();
    let has_inline = !session.private_key_inline.trim().is_empty();
    match (!path.is_empty(), has_inline) {
        (true, true) => format!("inline key or {}", path),
        (true, false) => path.to_string(),
        (false, true) => "inline key text".to_string(),
        (false, false) => "unknown key source".to_string(),
    }
}
