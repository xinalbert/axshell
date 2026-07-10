use crate::{
    monitoring::SystemSnapshot,
    session::SshConnectionMode,
    sftp::{PreviewData, RemoteEntry, TransferInfo, TransferState},
};

pub(crate) const BACKEND_EVENT_QUEUE_CAPACITY: usize = 256;
pub(crate) type BackendEventSender = tokio::sync::mpsc::Sender<BackendEvent>;

pub(crate) fn backend_event_channel() -> (
    BackendEventSender,
    tokio::sync::mpsc::Receiver<BackendEvent>,
) {
    tokio::sync::mpsc::channel(BACKEND_EVENT_QUEUE_CAPACITY)
}

#[derive(Debug, Clone)]
pub(crate) enum BackendEvent {
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

#[cfg(test)]
mod tests {
    use super::{BACKEND_EVENT_QUEUE_CAPACITY, BackendEvent, backend_event_channel};

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
