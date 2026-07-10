use std::{io, sync::Arc, time::Duration};

use anyhow::{Context, Result, anyhow};
use russh::{Disconnect, client};
use tokio::time::sleep;

use crate::{
    backend::{
        auth::{load_session_private_key, private_keys_with_algs},
        proxy::{self, ProxyStream},
    },
    events::{BackendEvent, BackendEventSender},
    session::{AuthMethod, Session, SshConnectionMode, ordered_ssh_connection_modes},
};

use super::{ClientHandler, X11ForwardingState, legacy};

pub(super) async fn connect_and_authenticate(
    tab_id: &str,
    session: &Session,
    events: &BackendEventSender,
    x11: Option<Arc<X11ForwardingState>>,
) -> Result<russh::client::Handle<ClientHandler>> {
    let addr = format!("{}:{}", session.host, session.port);
    tracing::info!(
        "[ssh] initiating tcp connection to {} (user: {})",
        addr,
        session.user
    );
    let status_text = if let Some((ptype, phost, pport)) = proxy::active(session) {
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
    let _ = events
        .send(BackendEvent::Status {
            tab_id: tab_id.to_string(),
            text: status_text,
        })
        .await;

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
            let _ = events
                .send(BackendEvent::Status {
                    tab_id: tab_id.to_string(),
                    text: format!(
                        "connected to {addr}, sending password authentication for {}",
                        session.user
                    ),
                })
                .await;
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
            let _ = events
                .send(BackendEvent::Status {
                    tab_id: tab_id.to_string(),
                    text: format!("connected to {addr}, loading private key from {source}"),
                })
                .await;
            let keypair = load_session_private_key(session)?;
            let algorithm = format!("{:?}", keypair.algorithm());
            let _ = events.send(BackendEvent::Status {
                tab_id: tab_id.to_string(),
                text: format!("private key loaded from {source}, algorithm {algorithm}, sending public key authentication for {}", session.user),
            }).await;
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

    let _ = events
        .send(BackendEvent::Status {
            tab_id: tab_id.to_string(),
            text: format!(
                "authentication accepted, opening shell for {}@{}",
                session.user, session.host
            ),
        })
        .await;
    let _ = events
        .send(BackendEvent::SshConnectionModeResolved {
            tab_id: tab_id.to_string(),
            session_id: session.id.clone(),
            mode: connected_mode,
        })
        .await;

    Ok(handle)
}

async fn connect_with_mode_priority(
    tab_id: &str,
    session: &Session,
    addr: &str,
    events: &BackendEventSender,
    x11: Option<Arc<X11ForwardingState>>,
) -> Result<(russh::client::Handle<ClientHandler>, SshConnectionMode)> {
    let modes = ordered_ssh_connection_modes(session.last_successful_ssh_mode);
    let mut failures = Vec::new();

    for (index, mode) in modes.iter().copied().enumerate() {
        if index > 0 {
            let _ = events
                .send(BackendEvent::Status {
                    tab_id: tab_id.to_string(),
                    text: format!("retrying SSH connection in {} mode", mode.label()),
                })
                .await;
        }

        match connect_with_mode(tab_id, session, addr, mode, events, x11.clone()).await {
            Ok(handle) => {
                if mode == SshConnectionMode::Legacy {
                    tracing::warn!(
                        "[ssh] connected to {} using legacy SSH compatibility mode",
                        addr
                    );
                    let _ = events
                        .send(BackendEvent::Status {
                            tab_id: tab_id.to_string(),
                            text: format!(
                                "connected to {addr} using legacy SSH compatibility mode"
                            ),
                        })
                        .await;
                } else if session.last_successful_ssh_mode == Some(SshConnectionMode::Legacy) {
                    let _ = events
                        .send(BackendEvent::Status {
                            tab_id: tab_id.to_string(),
                            text: format!("connected to {addr} using default SSH mode"),
                        })
                        .await;
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

                let is_transport_error = is_retryable_transport_error(&err);
                let should_try_next = index + 1 < modes.len()
                    && !is_transport_error
                    && (details.is_some() || session.last_successful_ssh_mode == Some(mode));
                if !should_try_next {
                    return Err(anyhow!("SSH connection failed. {}", failures.join(". ")));
                }

                let next = modes[index + 1];
                let reason = legacy::negotiation_error_short_reason(&err)
                    .map(|reason| format!("SSH negotiation failed ({reason})"))
                    .unwrap_or_else(|| "SSH connection failed with cached mode".to_string());
                let _ = events
                    .send(BackendEvent::Status {
                        tab_id: tab_id.to_string(),
                        text: format!("{} {reason}, retrying {} mode", mode.label(), next.label()),
                    })
                    .await;
            }
        }
    }

    Err(anyhow!(
        "SSH connection failed before any mode was attempted"
    ))
}

async fn connect_with_mode(
    tab_id: &str,
    session: &Session,
    addr: &str,
    mode: SshConnectionMode,
    events: &BackendEventSender,
    x11: Option<Arc<X11ForwardingState>>,
) -> Result<russh::client::Handle<ClientHandler>> {
    let stream =
        connect_transport_with_retries(Some(tab_id), session, addr, mode, Some(events)).await?;
    client::connect_stream(
        Arc::new(legacy::ssh_client_config(mode)),
        stream,
        ClientHandler::new(x11),
    )
    .await
    .with_context(|| format!("connect {addr} failed in {} mode", mode.label()))
}

pub(crate) async fn connect_transport_with_retries(
    tab_id: Option<&str>,
    session: &Session,
    addr: &str,
    mode: SshConnectionMode,
    events: Option<&BackendEventSender>,
) -> Result<Box<dyn ProxyStream>> {
    let config = crate::config::ConfigStore::load()
        .unwrap_or_else(|_| crate::config::ConfigStore::in_memory());
    let retry_delays = config
        .ssh_connect_retry_delays_ms()
        .into_iter()
        .map(Duration::from_millis)
        .collect::<Vec<_>>();
    let mut attempt = 0usize;

    loop {
        match proxy::connect(session).await {
            Ok(stream) => return Ok(stream),
            Err(err) => {
                let Some(delay) = retry_delays.get(attempt).copied() else {
                    return Err(err);
                };
                if !is_retryable_transport_error(&err) {
                    return Err(err);
                }

                attempt += 1;
                tracing::warn!(
                    "[ssh] transport connection to {} failed in {} mode on attempt {}: {:#}; retrying in {:?}",
                    addr,
                    mode.label(),
                    attempt,
                    err,
                    delay
                );
                if let (Some(tab_id), Some(events)) = (tab_id, events) {
                    let _ = events
                        .send(BackendEvent::Status {
                            tab_id: tab_id.to_string(),
                            text: format!(
                                "tcp connection to {addr} failed ({err}); retrying in {:.1}s",
                                delay.as_secs_f32()
                            ),
                        })
                        .await;
                }
                sleep(delay).await;
            }
        }
    }
}

fn is_retryable_transport_error(err: &anyhow::Error) -> bool {
    err.chain().any(|cause| {
        cause
            .downcast_ref::<io::Error>()
            .is_some_and(is_retryable_io_error)
    })
}

fn is_retryable_io_error(err: &io::Error) -> bool {
    matches!(
        err.kind(),
        io::ErrorKind::ConnectionRefused
            | io::ErrorKind::ConnectionReset
            | io::ErrorKind::ConnectionAborted
            | io::ErrorKind::TimedOut
            | io::ErrorKind::NotConnected
            | io::ErrorKind::AddrNotAvailable
    ) || matches!(
        err.raw_os_error(),
        // macOS/BSD: EHOSTDOWN, EHOSTUNREACH, ENETUNREACH, ETIMEDOUT, ECONNREFUSED
        Some(64 | 65 | 51 | 60 | 61)
            // Linux: EHOSTUNREACH, ENETUNREACH, ETIMEDOUT, ECONNREFUSED
            | Some(113 | 101 | 110 | 111)
            // Windows: WSAEHOSTUNREACH, WSAENETUNREACH, WSAETIMEDOUT, WSAECONNREFUSED
            | Some(10065 | 10051 | 10060 | 10061)
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retries_macos_no_route_to_host() {
        let err = anyhow::Error::new(io::Error::from_raw_os_error(65));

        assert!(is_retryable_transport_error(&err));
    }

    #[test]
    fn retries_wrapped_connection_refused() {
        let err = anyhow::Error::new(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            "connection refused",
        ))
        .context("connect target failed");

        assert!(is_retryable_transport_error(&err));
    }

    #[test]
    fn does_not_retry_non_transport_error() {
        let err = anyhow!("authentication failed");

        assert!(!is_retryable_transport_error(&err));
    }
}
