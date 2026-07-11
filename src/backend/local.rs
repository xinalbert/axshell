use std::{
    io::{Read, Write},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
    },
    thread,
};

use anyhow::{Context, Result};
use portable_pty::{ChildKiller, CommandBuilder, PtySize, native_pty_system};

use crate::{
    events::{BackendEvent, BackendEventSender},
    terminal::{BackendCommand, BackendShutdown, BackendTx},
};

const LOCAL_CHILD_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(250);
const LOCAL_TERM: &str = "xterm-256color";

struct LocalBackendThreads {
    reader: thread::JoinHandle<()>,
    writer: thread::JoinHandle<()>,
}

struct LocalBackendShutdown {
    commands: Sender<BackendCommand>,
    child_killer: Mutex<Option<Box<dyn ChildKiller + Send + Sync>>>,
    threads: Mutex<Option<LocalBackendThreads>>,
    shutdown_requested: Arc<AtomicBool>,
}

impl BackendShutdown for LocalBackendShutdown {
    fn shutdown(&self) {
        if self.shutdown_requested.swap(true, Ordering::SeqCst) {
            return;
        }

        let _ = self.commands.send(BackendCommand::Close);
        if let Some(mut child_killer) = self
            .child_killer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take()
        {
            if let Err(err) = child_killer.kill() {
                tracing::debug!(
                    component = "local_terminal",
                    operation = "shutdown_kill",
                    error = %crate::diagnostics::sanitize_error(&err.to_string()),
                    "Failed to kill local shell during shutdown"
                );
            }
        }

        let threads = self
            .threads
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        let Some(threads) = threads else {
            return;
        };

        thread::spawn(move || {
            if threads.writer.join().is_err() {
                tracing::warn!(
                    component = "local_terminal",
                    "Local terminal writer thread panicked"
                );
            }
            if threads.reader.join().is_err() {
                tracing::warn!(
                    component = "local_terminal",
                    "Local terminal reader thread panicked"
                );
            }
        });
    }
}

pub fn spawn_local_terminal(
    tab_id: String,
    cols: u16,
    rows: u16,
    events: BackendEventSender,
) -> Result<BackendTx> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .context("open local PTY")?;

    let shell = std::env::var("SHELL").unwrap_or_else(|_| {
        if cfg!(windows) {
            "powershell.exe".into()
        } else {
            "/bin/zsh".into()
        }
    });
    let shell_label = crate::diagnostics::mask_path(&shell);
    tracing::info!(
        component = "local_terminal",
        operation = "start",
        tab_id = %tab_id,
        shell = %shell_label,
        cols,
        rows,
        "Starting local terminal"
    );

    let cmd = local_shell_command(&shell);
    let mut child = pair.slave.spawn_command(cmd).context("spawn local shell")?;
    let child_killer = child.clone_killer();
    drop(pair.slave);

    let master = pair.master;
    let mut reader = master.try_clone_reader().context("clone PTY reader")?;
    let mut writer = master.take_writer().context("take PTY writer")?;
    let (cmd_tx, cmd_rx) = mpsc::channel::<BackendCommand>();
    let shutdown_requested = Arc::new(AtomicBool::new(false));

    let read_tab = tab_id.clone();
    let read_events = events.clone();
    let reader_shutdown_requested = shutdown_requested.clone();
    let reader_thread = thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let _ = read_events.blocking_send(BackendEvent::Output {
                        tab_id: read_tab.clone(),
                        bytes: buf[..n].to_vec(),
                    });
                }
                Err(err) => {
                    if !reader_shutdown_requested.load(Ordering::SeqCst) {
                        let error = crate::diagnostics::sanitize_error(&err.to_string());
                        tracing::error!(
                            component = "local_terminal",
                            operation = "read",
                            tab_id = %read_tab,
                            error = %error,
                            "Local terminal read failed"
                        );
                        let _ = read_events.blocking_send(BackendEvent::Closed {
                            tab_id: read_tab.clone(),
                            reason: format!("local read error: {err}"),
                        });
                    }
                    return;
                }
            }
        }
        if !reader_shutdown_requested.load(Ordering::SeqCst) {
            tracing::info!(
                component = "local_terminal",
                operation = "read",
                tab_id = %read_tab,
                "Local shell output closed"
            );
            let _ = read_events.blocking_send(BackendEvent::Closed {
                tab_id: read_tab,
                reason: "local shell closed".into(),
            });
        }
    });

    let write_tab = tab_id.clone();
    let write_events = events.clone();
    let writer_shutdown_requested = shutdown_requested.clone();
    let writer_thread = thread::spawn(move || {
        loop {
            match cmd_rx.recv_timeout(LOCAL_CHILD_POLL_INTERVAL) {
                Ok(command) => match command {
                    BackendCommand::Input(bytes) => {
                        if let Err(err) = writer.write_all(&bytes) {
                            if !writer_shutdown_requested.load(Ordering::SeqCst) {
                                let error = crate::diagnostics::sanitize_error(&err.to_string());
                                tracing::error!(
                                    component = "local_terminal",
                                    operation = "write",
                                    tab_id = %write_tab,
                                    error = %error,
                                    "Local terminal write failed"
                                );
                                let _ = write_events.blocking_send(BackendEvent::Closed {
                                    tab_id: write_tab.clone(),
                                    reason: format!("local write error: {err}"),
                                });
                            }
                            break;
                        }
                        if let Err(err) = writer.flush() {
                            let error = crate::diagnostics::sanitize_error(&err.to_string());
                            tracing::warn!(
                                component = "local_terminal",
                                operation = "flush",
                                tab_id = %write_tab,
                                error = %error,
                                "Local terminal flush failed"
                            );
                        }
                    }
                    BackendCommand::Resize { cols, rows } => {
                        if let Err(err) = master.resize(PtySize {
                            rows,
                            cols,
                            pixel_width: 0,
                            pixel_height: 0,
                        }) {
                            let error = crate::diagnostics::sanitize_error(&err.to_string());
                            tracing::warn!(
                                component = "local_terminal",
                                operation = "resize",
                                tab_id = %write_tab,
                                error = %error,
                                cols,
                                rows,
                                "Local terminal resize failed"
                            );
                        }
                    }
                    BackendCommand::Close => break,
                    BackendCommand::SampleMetrics | BackendCommand::QueryWorkingDirectory => {}
                },
                Err(mpsc::RecvTimeoutError::Timeout) => match child.try_wait() {
                    Ok(Some(status)) => {
                        tracing::info!(
                            component = "local_terminal",
                            operation = "wait",
                            tab_id = %write_tab,
                            status = %status,
                            "Local shell exited"
                        );
                        if !writer_shutdown_requested.load(Ordering::SeqCst) {
                            let _ = write_events.blocking_send(BackendEvent::Closed {
                                tab_id: write_tab,
                                reason: format!("local shell exited: {status}"),
                            });
                        }
                        return;
                    }
                    Ok(None) => {}
                    Err(err) => {
                        let error = crate::diagnostics::sanitize_error(&err.to_string());
                        tracing::error!(
                            component = "local_terminal",
                            operation = "wait",
                            tab_id = %write_tab,
                            error = %error,
                            "Failed to query local shell status"
                        );
                        if !writer_shutdown_requested.load(Ordering::SeqCst) {
                            let _ = write_events.blocking_send(BackendEvent::Closed {
                                tab_id: write_tab,
                                reason: format!("local shell status error: {err}"),
                            });
                        }
                        return;
                    }
                },
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
        if let Err(err) = child.kill() {
            tracing::debug!(
                component = "local_terminal",
                operation = "kill",
                tab_id = %write_tab,
                error = %crate::diagnostics::sanitize_error(&err.to_string()),
                "Local shell was already stopped or could not be killed"
            );
        }
    });

    let _ = events.try_send(BackendEvent::Status {
        tab_id,
        text: "local shell ready".into(),
    });

    Ok(BackendTx::Local {
        commands: cmd_tx.clone(),
        shutdown: Arc::new(LocalBackendShutdown {
            commands: cmd_tx,
            child_killer: Mutex::new(Some(child_killer)),
            threads: Mutex::new(Some(LocalBackendThreads {
                reader: reader_thread,
                writer: writer_thread,
            })),
            shutdown_requested,
        }),
    })
}

fn local_shell_command(shell: &str) -> CommandBuilder {
    let mut cmd = CommandBuilder::new(shell);
    cmd.env("TERM", LOCAL_TERM);
    cmd.env(
        "COLORTERM",
        std::env::var("COLORTERM").unwrap_or_else(|_| "truecolor".into()),
    );
    cmd.env("TERM_PROGRAM", "AxShell");
    if let Ok(path) = std::env::var("PATH") {
        cmd.env("PATH", path);
    }
    if let Ok(lang) = std::env::var("LANG") {
        cmd.env("LANG", lang);
    } else {
        cmd.env("LANG", "en_US.UTF-8");
    }
    if let Ok(home) = std::env::var("HOME") {
        cmd.env("HOME", home);
    }
    cmd.env("SHELL", shell);
    cmd
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use super::{LOCAL_TERM, local_shell_command};

    #[test]
    fn local_shell_declares_the_supported_terminal_type() {
        let command = local_shell_command("test-shell");

        assert_eq!(command.get_env("TERM"), Some(OsStr::new(LOCAL_TERM)));
    }
}
