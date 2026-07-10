use std::sync::mpsc::Sender;

use crate::{
    session::config::SshConnectionMode,
    sftp::{PreviewData, RemoteEntry},
    system::SystemSnapshot,
};

use super::transfer::{TransferInfo, TransferState};

pub(crate) const BACKEND_EVENT_QUEUE_CAPACITY: usize = 256;
pub(crate) type BackendEventSender = tokio::sync::mpsc::Sender<BackendEvent>;

pub(crate) fn backend_event_channel() -> (
    BackendEventSender,
    tokio::sync::mpsc::Receiver<BackendEvent>,
) {
    tokio::sync::mpsc::channel(BACKEND_EVENT_QUEUE_CAPACITY)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabKind {
    Local,
    Ssh,
}

#[derive(Debug)]
pub enum BackendCommand {
    Input(Vec<u8>),
    Resize { cols: u16, rows: u16 },
    SampleMetrics,
    QueryWorkingDirectory,
    Close,
}

#[derive(Debug, Clone)]
pub enum BackendEvent {
    Output {
        tab_id: String,
        bytes: Vec<u8>,
    },
    Status {
        tab_id: String,
        text: String,
    },
    Connected {
        tab_id: String,
    },
    SshConnectionModeResolved {
        tab_id: String,
        session_id: String,
        mode: SshConnectionMode,
    },
    SftpEntries {
        tab_id: String,
        path: String,
        entries: Vec<RemoteEntry>,
        append: bool,
        has_more: bool,
        reached_limit: bool,
    },
    SftpPreview {
        tab_id: String,
        preview: PreviewData,
    },
    SftpStatus {
        tab_id: String,
        text: String,
    },
    RemoteSystem {
        tab_id: String,
        snapshot: SystemSnapshot,
    },
    RemoteSystemUnavailable {
        tab_id: String,
        reason: String,
    },
    SftpHome {
        tab_id: String,
        home: String,
    },
    TransferProgress {
        #[allow(dead_code)]
        tab_id: String,
        id: String,
        transferred: u64,
        total: Option<u64>,
        state: TransferState,
    },
    TransferStarted {
        tab_id: String,
        info: TransferInfo,
    },
    Closed {
        tab_id: String,
        reason: String,
    },
    TerminalTitleChanged {
        tab_id: String,
        title: String,
    },
    WorkingDirectoryChanged {
        tab_id: String,
        path: String,
    },
    WorkingDirectoryResolved {
        tab_id: String,
        path: String,
    },
    SyncFinished(crate::sync::SyncResult),
}

pub trait BackendShutdown: Send + Sync {
    /// Start a non-blocking, backend-specific shutdown and resource reap.
    fn shutdown(&self);
}

#[derive(Clone)]
pub enum BackendTx {
    Local {
        commands: Sender<BackendCommand>,
        shutdown: std::sync::Arc<dyn BackendShutdown>,
    },
    Ssh {
        commands: tokio::sync::mpsc::UnboundedSender<BackendCommand>,
        shutdown: std::sync::Arc<dyn BackendShutdown>,
    },
}

impl BackendTx {
    pub fn send(&self, command: BackendCommand) {
        if matches!(command, BackendCommand::Close) {
            self.shutdown();
            return;
        }

        match self {
            Self::Local { commands, .. } => {
                let _ = commands.send(command);
            }
            Self::Ssh { commands, .. } => {
                let _ = commands.send(command);
            }
        }
    }

    /// Signal the backend and schedule its resource reaper without blocking UI work.
    pub fn shutdown(&self) {
        match self {
            Self::Local { shutdown, .. } | Self::Ssh { shutdown, .. } => shutdown.shutdown(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    };

    use super::{
        BACKEND_EVENT_QUEUE_CAPACITY, BackendCommand, BackendEvent, BackendShutdown, BackendTx,
        backend_event_channel,
    };

    struct CountingShutdown(AtomicUsize);

    impl BackendShutdown for CountingShutdown {
        fn shutdown(&self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn close_command_uses_the_backend_shutdown_controller() {
        let (commands, receiver) = mpsc::channel();
        let shutdown = Arc::new(CountingShutdown(AtomicUsize::new(0)));
        let backend = BackendTx::Local {
            commands,
            shutdown: shutdown.clone(),
        };

        backend.send(BackendCommand::Close);

        assert_eq!(shutdown.0.load(Ordering::SeqCst), 1);
        assert!(receiver.try_recv().is_err());
    }

    #[test]
    fn backend_event_channel_has_a_fixed_capacity() {
        let (events, _receiver) = backend_event_channel();

        for _ in 0..BACKEND_EVENT_QUEUE_CAPACITY {
            events
                .try_send(BackendEvent::Connected {
                    tab_id: "tab".to_string(),
                })
                .expect("queue has spare capacity");
        }

        assert!(
            events
                .try_send(BackendEvent::Connected {
                    tab_id: "tab".to_string(),
                })
                .is_err()
        );
    }
}
