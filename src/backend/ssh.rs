use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use anyhow::{Context, Result};
use russh::{
    Channel, ChannelMsg, ChannelOpenFailure, Disconnect,
    client::{self, Handler, Msg},
};
use tokio::{
    sync::mpsc,
    task::{JoinHandle, JoinSet},
};

use crate::{
    config::ConfigStore,
    diagnostics::{mask_host, mask_value, sanitize_error, sanitize_error_with_values},
    events::{BackendEvent, BackendEventSender},
    session::Session,
    terminal::{BackendCommand, BackendShutdown, BackendTx},
};

pub(crate) mod connection;
mod legacy;
mod system_probe;
mod x11;

use connection::connect_and_authenticate;
pub(crate) use legacy::{negotiation_error_details, ssh_client_config};
use system_probe::sample_remote_system_with_handle;
use x11::X11ForwardingState;

const BASH_CWD_PROMPT_COMMAND: &str =
    r#"printf '\033]7;file://%s%s\033\\' "$(hostname 2>/dev/null || printf localhost)" "$PWD""#;
const SSH_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);

struct SshBackendShutdown {
    commands: mpsc::UnboundedSender<BackendCommand>,
    join: Mutex<Option<JoinHandle<()>>>,
    runtime: tokio::runtime::Handle,
    shutdown_requested: Arc<AtomicBool>,
}

impl BackendShutdown for SshBackendShutdown {
    fn shutdown(&self) {
        if self.shutdown_requested.swap(true, Ordering::SeqCst) {
            return;
        }

        let _ = self.commands.send(BackendCommand::Close);
        let join = self
            .join
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        let Some(mut join) = join else {
            return;
        };

        self.runtime.spawn(async move {
            if tokio::time::timeout(SSH_SHUTDOWN_TIMEOUT, &mut join)
                .await
                .is_ok()
            {
                return;
            }

            tracing::warn!(
                component = "ssh",
                operation = "shutdown",
                timeout_ms = SSH_SHUTDOWN_TIMEOUT.as_millis(),
                "SSH terminal shutdown timed out; aborting task"
            );
            join.abort();
            let _ = join.await;
        });
    }
}

pub fn spawn_ssh_terminal(
    runtime: &tokio::runtime::Handle,
    tab_id: String,
    session: Session,
    cols: u16,
    rows: u16,
    events: BackendEventSender,
) -> BackendTx {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<BackendCommand>();
    let task_tab = tab_id.clone();
    let shutdown_requested = Arc::new(AtomicBool::new(false));
    let task_shutdown_requested = shutdown_requested.clone();
    let log_host = mask_host(&session.host);
    let log_user = mask_value(&session.user);
    let log_port = session.port;
    let sensitive_values = [
        session.user.clone(),
        session.host.clone(),
        session.private_key_path.clone(),
        session.password.clone(),
        session.passphrase.clone(),
        session.proxy_host.clone(),
        session.proxy_user.clone(),
        session.proxy_password.clone(),
    ];
    let join = runtime.spawn(async move {
        if let Err(err) = run_ssh(
            task_tab.clone(),
            session,
            cols,
            rows,
            cmd_rx,
            events.clone(),
            task_shutdown_requested.clone(),
        )
        .await
        {
            let sensitive_values = sensitive_values
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>();
            tracing::error!(
                component = "ssh",
                operation = "terminal",
                host = %log_host,
                port = log_port,
                user = %log_user,
                error = %sanitize_error_with_values(&format!("{err:#}"), &sensitive_values),
                "SSH terminal failed"
            );
            if !task_shutdown_requested.load(Ordering::SeqCst) {
                let _ = events
                    .send(BackendEvent::Closed {
                        tab_id: task_tab,
                        reason: format!("{err:#}"),
                    })
                    .await;
            }
        }
    });
    BackendTx::Ssh {
        commands: cmd_tx.clone(),
        shutdown: Arc::new(SshBackendShutdown {
            commands: cmd_tx,
            join: Mutex::new(Some(join)),
            runtime: runtime.clone(),
            shutdown_requested,
        }),
    }
}

async fn run_ssh(
    tab_id: String,
    session: Session,
    cols: u16,
    rows: u16,
    mut commands: mpsc::UnboundedReceiver<BackendCommand>,
    events: BackendEventSender,
    shutdown_requested: Arc<AtomicBool>,
) -> Result<()> {
    let config = ConfigStore::load().unwrap_or_else(|_| ConfigStore::in_memory());
    let x11 = X11ForwardingState::from_config(&config);
    let _ = events
        .send(BackendEvent::Status {
            tab_id: tab_id.clone(),
            text: format!(
                "connecting {}@{}:{}...",
                session.user, session.host, session.port
            ),
        })
        .await;

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
    request_shell_integration_env(&mut channel).await;
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
                if let Err(err) = channel
                    .set_env(false, "DISPLAY", x11.remote_display.clone())
                    .await
                {
                    tracing::warn!(
                        component = "ssh",
                        operation = "set_x11_display",
                        error = %sanitize_error(&err.to_string()),
                        "SSH server rejected the X11 DISPLAY environment"
                    );
                }
                let _ = events
                    .send(BackendEvent::Status {
                        tab_id: tab_id.clone(),
                        text: format!("X11 forwarding requested, DISPLAY={}", x11.remote_display),
                    })
                    .await;
            }
            Err(err) => {
                tracing::warn!(
                    component = "ssh",
                    operation = "request_x11",
                    error = %sanitize_error(&err.to_string()),
                    "SSH X11 forwarding request failed"
                );
                let _ = events
                    .send(BackendEvent::Status {
                        tab_id: tab_id.clone(),
                        text: format!("X11 forwarding unavailable: {err}"),
                    })
                    .await;
            }
        }
    }
    channel.request_shell(true).await.context("request shell")?;

    let _ = events
        .send(BackendEvent::Status {
            tab_id: tab_id.clone(),
            text: format!("connected {}@{}", session.user, session.host),
        })
        .await;
    let _ = events
        .send(BackendEvent::Connected {
            tab_id: tab_id.clone(),
        })
        .await;

    let exit_reason;
    let mut is_graceful_close = false;
    let mut child_tasks = JoinSet::new();

    loop {
        tokio::select! {
            command = commands.recv() => {
                match command {
                    Some(BackendCommand::Input(bytes)) => {
                        if let Err(err) = channel.data(bytes.as_slice()).await {
                            tracing::error!(
                                component = "ssh",
                                operation = "write",
                                tab_id,
                                error = %sanitize_error(&err.to_string()),
                                "SSH terminal write failed"
                            );
                            exit_reason = format!("ssh write error: {err}");
                            break;
                        }
                    }
                    Some(BackendCommand::Resize { cols, rows }) => {
                        if let Err(err) = channel.window_change(cols.into(), rows.into(), 0, 0).await {
                            tracing::warn!(
                                component = "ssh",
                                operation = "resize",
                                tab_id,
                                cols,
                                rows,
                                error = %sanitize_error(&err.to_string()),
                                "SSH terminal resize failed"
                            );
                        }
                    }
                    Some(BackendCommand::SampleMetrics) => {
                        let handle_clone = handle.clone();
                        let tab_id_clone = tab_id.clone();
                        let events_clone = events.clone();
                        child_tasks.spawn(async move {
                            match sample_remote_system_with_handle(handle_clone).await {
                                Ok(snapshot) => {
                                    let _ = events_clone.send(BackendEvent::RemoteSystem {
                                        tab_id: tab_id_clone,
                                        snapshot,
                                    }).await;
                                }
                                Err(err) => {
                                    let _ = events_clone.send(BackendEvent::RemoteSystemUnavailable {
                                        tab_id: tab_id_clone,
                                        reason: format!("remote metrics unavailable: {err:#}"),
                                    }).await;
                                }
                            }
                        });
                    }
                    Some(BackendCommand::QueryWorkingDirectory) => {
                        let handle_clone = handle.clone();
                        let tab_id_clone = tab_id.clone();
                        let events_clone = events.clone();
                        child_tasks.spawn(async move {
                            match query_remote_working_directory_with_handle(handle_clone).await {
                                Ok(path) => {
                                    let _ = events_clone.send(BackendEvent::WorkingDirectoryResolved {
                                        tab_id: tab_id_clone,
                                        path,
                                    }).await;
                                }
                                Err(err) => {
                                    tracing::debug!(
                                        component = "ssh",
                                        operation = "query_working_directory",
                                        tab_id = tab_id_clone,
                                        error = %sanitize_error(&format!("{err:#}")),
                                        "SSH working directory query failed"
                                    );
                                }
                            }
                        });
                    }
                    Some(BackendCommand::Close) | None => {
                        tracing::info!(
                            component = "ssh",
                            operation = "close",
                            tab_id,
                            close_source = "client",
                            graceful = true,
                            "SSH session close requested"
                        );
                        let _ = channel.eof().await;
                        exit_reason = "ssh session closed".to_string();
                        break;
                    }
                }
            }
            Some(_) = child_tasks.join_next(), if !child_tasks.is_empty() => {}
            msg = channel.wait() => {
                match msg {
                    Some(ChannelMsg::Data { data }) | Some(ChannelMsg::ExtendedData { data, ext: _ }) => {
                        let _ = events.send(BackendEvent::Output {
                            tab_id: tab_id.clone(),
                            bytes: data.to_vec(),
                        }).await;
                    }
                    Some(ChannelMsg::ExitStatus { exit_status: _ }) | Some(ChannelMsg::Eof) => {
                        is_graceful_close = true;
                    }
                    Some(ChannelMsg::Close) => {
                        if is_graceful_close {
                            tracing::info!(
                                component = "ssh",
                                operation = "close",
                                tab_id,
                                close_source = "server",
                                graceful = true,
                                "SSH session closed"
                            );
                            exit_reason = "ssh session closed".to_string();
                        } else {
                            tracing::warn!(
                                component = "ssh",
                                operation = "close",
                                tab_id,
                                close_source = "server",
                                graceful = false,
                                "SSH connection closed abruptly"
                            );
                            exit_reason = "ssh connection lost (abrupt close)".to_string();
                        }
                        break;
                    }
                    None => {
                        if is_graceful_close {
                            tracing::info!(
                                component = "ssh",
                                operation = "close",
                                tab_id,
                                close_source = "network",
                                graceful = true,
                                "SSH network stream ended"
                            );
                            exit_reason = "ssh session closed".to_string();
                        } else {
                            tracing::warn!(
                                component = "ssh",
                                operation = "close",
                                tab_id,
                                close_source = "network",
                                graceful = false,
                                "SSH network connection dropped"
                            );
                            exit_reason = "ssh connection lost (network drop)".to_string();
                        }
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    cancel_ssh_child_tasks(&mut child_tasks).await;
    if let Err(err) = handle
        .lock()
        .await
        .disconnect(Disconnect::ByApplication, "bye", "")
        .await
    {
        tracing::debug!(
            component = "ssh",
            operation = "disconnect",
            tab_id,
            error = %sanitize_error(&err.to_string()),
            "SSH disconnect request failed"
        );
    }
    if !shutdown_requested.load(Ordering::SeqCst) {
        let _ = events
            .send(BackendEvent::Closed {
                tab_id,
                reason: exit_reason,
            })
            .await;
    }
    Ok(())
}

async fn cancel_ssh_child_tasks(child_tasks: &mut JoinSet<()>) {
    child_tasks.abort_all();
    while child_tasks.join_next().await.is_some() {}
}

async fn request_shell_integration_env(channel: &mut Channel<Msg>) {
    for (name, value) in [
        ("TERM_PROGRAM", "AxShell"),
        ("AXSHELL_SHELL_INTEGRATION", "1"),
        ("PROMPT_COMMAND", BASH_CWD_PROMPT_COMMAND),
    ] {
        if let Err(err) = channel.set_env(false, name, value).await {
            tracing::debug!(
                component = "ssh",
                operation = "set_shell_environment",
                variable = name,
                error = %sanitize_error(&err.to_string()),
                "SSH server rejected a shell integration environment variable"
            );
        }
    }
}

async fn query_remote_working_directory_with_handle(
    handle: Arc<tokio::sync::Mutex<russh::client::Handle<ClientHandler>>>,
) -> Result<String> {
    let mut channel = handle
        .lock()
        .await
        .channel_open_session()
        .await
        .context("open cwd query session")?;
    channel
        .exec(true, "pwd -P")
        .await
        .context("exec cwd query")?;

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut exit_status = None;
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        while let Some(msg) = channel.wait().await {
            match msg {
                ChannelMsg::Data { data } => stdout.extend_from_slice(&data),
                ChannelMsg::ExtendedData { data, ext: _ } => stderr.extend_from_slice(&data),
                ChannelMsg::ExitStatus { exit_status: code } => exit_status = Some(code),
                ChannelMsg::Close => break,
                _ => {}
            }
        }
    })
    .await
    .context("cwd query timeout")?;

    if exit_status.unwrap_or(0) != 0 {
        let stderr = String::from_utf8_lossy(&stderr).trim().to_string();
        anyhow::bail!(
            "cwd query exited with {}: {}",
            exit_status.unwrap_or(0),
            stderr
        );
    }

    let path = String::from_utf8_lossy(&stdout).trim().to_string();
    if !path.starts_with('/') {
        anyhow::bail!("cwd query returned non-absolute path: {path}");
    }
    Ok(path)
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

#[cfg(test)]
mod lifecycle_tests {
    use std::sync::{Arc, Mutex, atomic::AtomicBool};

    use tokio::{sync::mpsc, task::JoinSet};

    use crate::terminal::{BackendCommand, BackendShutdown};

    use super::{SshBackendShutdown, cancel_ssh_child_tasks};

    #[tokio::test]
    async fn closing_ssh_session_aborts_auxiliary_tasks() {
        let mut child_tasks = JoinSet::new();
        child_tasks.spawn(async {
            std::future::pending::<()>().await;
        });

        cancel_ssh_child_tasks(&mut child_tasks).await;

        assert!(child_tasks.is_empty());
    }

    #[tokio::test]
    async fn shutdown_controller_sends_close_and_reaps_finished_task() {
        let (commands, mut receiver) = mpsc::unbounded_channel();
        let controller = SshBackendShutdown {
            commands,
            join: Mutex::new(Some(tokio::spawn(async {}))),
            runtime: tokio::runtime::Handle::current(),
            shutdown_requested: Arc::new(AtomicBool::new(false)),
        };

        controller.shutdown();

        assert!(matches!(receiver.recv().await, Some(BackendCommand::Close)));
        tokio::task::yield_now().await;
        assert!(
            controller
                .join
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .is_none()
        );
    }
}
