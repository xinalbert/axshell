use std::sync::Arc;

use anyhow::{Context, Result};
use russh::{
    Channel, ChannelMsg, ChannelOpenFailure, Disconnect,
    client::{self, Handler, Msg},
};
use tokio::sync::mpsc;

use crate::{
    session::config::{ConfigStore, Session},
    terminal::{BackendCommand, BackendEvent, BackendTx},
};

mod connection;
mod legacy;
mod system_probe;
mod x11;

use connection::connect_and_authenticate;
pub(crate) use legacy::{negotiation_error_details, ssh_client_config};
use system_probe::sample_remote_system_with_handle;
use x11::X11ForwardingState;

pub fn spawn_ssh_terminal(
    runtime: &tokio::runtime::Handle,
    tab_id: String,
    session: Session,
    cols: u16,
    rows: u16,
    events: std::sync::mpsc::Sender<BackendEvent>,
) -> BackendTx {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<BackendCommand>();
    let task_tab = tab_id.clone();
    runtime.spawn(async move {
        if let Err(err) = run_ssh(
            task_tab.clone(),
            session,
            cols,
            rows,
            cmd_rx,
            events.clone(),
        )
        .await
        {
            let _ = events.send(BackendEvent::Closed {
                tab_id: task_tab,
                reason: format!("{err:#}"),
            });
        }
    });
    BackendTx::Ssh(cmd_tx)
}

async fn run_ssh(
    tab_id: String,
    session: Session,
    cols: u16,
    rows: u16,
    mut commands: mpsc::UnboundedReceiver<BackendCommand>,
    events: std::sync::mpsc::Sender<BackendEvent>,
) -> Result<()> {
    let config = ConfigStore::load().unwrap_or_else(|_| ConfigStore::in_memory());
    let x11 = X11ForwardingState::from_config(&config);
    let _ = events.send(BackendEvent::Status {
        tab_id: tab_id.clone(),
        text: format!(
            "connecting {}@{}:{}...",
            session.user, session.host, session.port
        ),
    });

    let handle = Arc::new(tokio::sync::Mutex::new(
        connect_and_authenticate(&tab_id, &session, &events, x11.clone()).await?,
    ));

    let mut channel = handle
        .lock()
        .await
        .channel_open_session()
        .await
        .context("open session")?;
    channel
        .request_pty(true, "xterm-256color", cols.into(), rows.into(), 0, 0, &[])
        .await
        .context("request pty")?;
    if let Some(x11) = x11.as_ref() {
        match channel
            .request_x11(
                true,
                false,
                "MIT-MAGIC-COOKIE-1",
                x11.fake_cookie_hex.clone(),
                x11.screen_number,
            )
            .await
        {
            Ok(()) => {
                let _ = channel
                    .set_env(false, "DISPLAY", x11.remote_display.clone())
                    .await;
                let _ = events.send(BackendEvent::Status {
                    tab_id: tab_id.clone(),
                    text: format!("X11 forwarding requested, DISPLAY={}", x11.remote_display),
                });
            }
            Err(err) => {
                tracing::warn!("[ssh] X11 forwarding request failed: {err}");
                let _ = events.send(BackendEvent::Status {
                    tab_id: tab_id.clone(),
                    text: format!("X11 forwarding unavailable: {err}"),
                });
            }
        }
    }
    channel.request_shell(true).await.context("request shell")?;

    let _ = events.send(BackendEvent::Status {
        tab_id: tab_id.clone(),
        text: format!("connected {}@{}", session.user, session.host),
    });
    let _ = events.send(BackendEvent::Connected {
        tab_id: tab_id.clone(),
    });

    let exit_reason;
    let mut is_graceful_close = false;

    loop {
        tokio::select! {
            command = commands.recv() => {
                match command {
                    Some(BackendCommand::Input(bytes)) => {
                        if let Err(err) = channel.data(bytes.as_slice()).await {
                            tracing::error!("[ssh] write error on tab {}: {}", tab_id, err);
                            exit_reason = format!("ssh write error: {err}");
                            break;
                        }
                    }
                    Some(BackendCommand::Resize { cols, rows }) => {
                        let _ = channel.window_change(cols.into(), rows.into(), 0, 0).await;
                    }
                    Some(BackendCommand::SampleMetrics) => {
                        let handle_clone = handle.clone();
                        let tab_id_clone = tab_id.clone();
                        let events_clone = events.clone();
                        tokio::spawn(async move {
                            match sample_remote_system_with_handle(handle_clone).await {
                                Ok(snapshot) => {
                                    let _ = events_clone.send(BackendEvent::RemoteSystem {
                                        tab_id: tab_id_clone,
                                        snapshot,
                                    });
                                }
                                Err(err) => {
                                    let _ = events_clone.send(BackendEvent::RemoteSystemUnavailable {
                                        tab_id: tab_id_clone,
                                        reason: format!("remote metrics unavailable: {err:#}"),
                                    });
                                }
                            }
                        });
                    }
                    Some(BackendCommand::Close) | None => {
                        tracing::info!("[ssh] local client closed the session for tab {}", tab_id);
                        let _ = channel.eof().await;
                        exit_reason = "ssh session closed".to_string();
                        break;
                    }
                }
            }
            msg = channel.wait() => {
                match msg {
                    Some(ChannelMsg::Data { data }) | Some(ChannelMsg::ExtendedData { data, ext: _ }) => {
                        let _ = events.send(BackendEvent::Output {
                            tab_id: tab_id.clone(),
                            bytes: data.to_vec(),
                        });
                    }
                    Some(ChannelMsg::ExitStatus { exit_status: _ }) | Some(ChannelMsg::Eof) => {
                        is_graceful_close = true;
                    }
                    Some(ChannelMsg::Close) => {
                        if is_graceful_close {
                            tracing::info!("[ssh] session gracefully closed by server for tab {}", tab_id);
                            exit_reason = "ssh session closed".to_string();
                        } else {
                            tracing::warn!("[ssh] connection abruptly closed by server for tab {}", tab_id);
                            exit_reason = "ssh connection lost (abrupt close)".to_string();
                        }
                        break;
                    }
                    None => {
                        if is_graceful_close {
                            tracing::info!("[ssh] network stream ended gracefully for tab {}", tab_id);
                            exit_reason = "ssh session closed".to_string();
                        } else {
                            tracing::warn!("[ssh] network drop detected for tab {}", tab_id);
                            exit_reason = "ssh connection lost (network drop)".to_string();
                        }
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    let _ = handle
        .lock()
        .await
        .disconnect(Disconnect::ByApplication, "bye", "")
        .await;
    let _ = events.send(BackendEvent::Closed {
        tab_id,
        reason: exit_reason,
    });
    Ok(())
}

#[derive(Clone)]
struct ClientHandler {
    x11: Option<Arc<X11ForwardingState>>,
}

impl ClientHandler {
    fn new(x11: Option<Arc<X11ForwardingState>>) -> Self {
        Self { x11 }
    }
}

impl Handler for ClientHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }

    fn server_channel_open_x11(
        &mut self,
        channel: Channel<Msg>,
        originator_address: &str,
        originator_port: u32,
        reply: client::ChannelOpenHandle,
        _session: &mut client::Session,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let x11 = self.x11.clone();
        let originator_address = originator_address.to_string();
        async move {
            let Some(x11) = x11 else {
                reply
                    .reject(ChannelOpenFailure::AdministrativelyProhibited)
                    .await;
                return Ok(());
            };

            x11::handle_x11_channel(channel, originator_address, originator_port, reply, x11).await
        }
    }
}
