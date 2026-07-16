use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc::{self, error::TrySendError};

use crate::{
    monitoring::SystemSnapshot,
    sftp::{
        PreviewData, RemoteEntry, TransferFile, TransferFileState, TransferInfo, TransferState,
    },
};

pub(crate) const BACKEND_EVENT_QUEUE_CAPACITY: usize = 256;

#[derive(Clone, Default)]
pub(crate) struct BackendEventRouter(Arc<Mutex<HashMap<String, mpsc::Sender<BackendEvent>>>>);

/// Sends backend events to the window that currently owns their terminal tab
/// or SFTP group. A sender remains valid after that resource moves windows.
#[derive(Clone)]
pub(crate) struct BackendEventSender {
    router: BackendEventRouter,
    fallback: mpsc::Sender<BackendEvent>,
}

pub(crate) fn backend_event_channel() -> (BackendEventSender, mpsc::Receiver<BackendEvent>) {
    backend_event_channel_with_router(BackendEventRouter::default())
}

pub(crate) fn backend_event_channel_with_router(
    router: BackendEventRouter,
) -> (BackendEventSender, mpsc::Receiver<BackendEvent>) {
    let (fallback, receiver) = mpsc::channel(BACKEND_EVENT_QUEUE_CAPACITY);
    (BackendEventSender { router, fallback }, receiver)
}

impl BackendEventSender {
    pub(crate) fn router(&self) -> BackendEventRouter {
        self.router.clone()
    }

    pub(crate) fn register_route(&self, resource_id: impl Into<String>) {
        let mut routes = self
            .router
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        routes.insert(resource_id.into(), self.fallback.clone());
    }

    pub(crate) fn register_routes<'a>(&self, resource_ids: impl IntoIterator<Item = &'a str>) {
        let mut routes = self
            .router
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        for resource_id in resource_ids {
            routes.insert(resource_id.to_string(), self.fallback.clone());
        }
    }

    pub(crate) fn try_send(&self, event: BackendEvent) -> Result<(), TrySendError<BackendEvent>> {
        self.destination_for(&event).try_send(event)
    }

    pub(crate) async fn send(
        &self,
        event: BackendEvent,
    ) -> Result<(), mpsc::error::SendError<BackendEvent>> {
        self.destination_for(&event).send(event).await
    }

    pub(crate) fn blocking_send(
        &self,
        event: BackendEvent,
    ) -> Result<(), mpsc::error::SendError<BackendEvent>> {
        self.destination_for(&event).blocking_send(event)
    }

    pub(crate) fn is_routed_to_current_window(&self, event: &BackendEvent) -> bool {
        self.destination_for(event).same_channel(&self.fallback)
    }

    fn destination_for(&self, event: &BackendEvent) -> mpsc::Sender<BackendEvent> {
        let Some(resource_id) = event.resource_id() else {
            return self.fallback.clone();
        };
        self.router
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(resource_id)
            .cloned()
            .unwrap_or_else(|| self.fallback.clone())
    }
}

#[derive(Debug)]
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
    SftpEditOpened {
        tab_id: String,
        remote_path: String,
        local_path: String,
    },
    SftpEditOpenFailed {
        tab_id: String,
        remote_path: String,
        reason: String,
    },
    SftpEditChanged {
        tab_id: String,
        remote_path: String,
        local_path: String,
        dirty: bool,
    },
    SftpEditUploadFinished {
        tab_id: String,
        remote_path: String,
        local_path: String,
        result: Result<(), String>,
    },
    HostKeyVerification {
        request: crate::backend::host_key::HostKeyVerificationRequest,
    },
    SftpOverwriteConflict {
        request: crate::sftp::SftpOverwriteRequest,
    },
    RemoteSystem {
        tab_id: String,
        generation: u64,
        snapshot: SystemSnapshot,
    },
    RemoteSystemUnavailable {
        tab_id: String,
        generation: u64,
        reason: String,
    },
    ConnectionHealthy {
        tab_id: String,
        generation: u64,
        backend_generation: u32,
    },
    ConnectionUnhealthy {
        tab_id: String,
        generation: u64,
        backend_generation: u32,
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
    TransferFileStarted {
        tab_id: String,
        transfer_id: String,
        file: TransferFile,
    },
    TransferFileFinished {
        tab_id: String,
        transfer_id: String,
        file_id: String,
        state: TransferFileState,
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

impl BackendEvent {
    pub(crate) fn resource_id(&self) -> Option<&str> {
        match self {
            Self::Output { tab_id, .. }
            | Self::Status { tab_id, .. }
            | Self::Connected { tab_id }
            | Self::SftpEntries { tab_id, .. }
            | Self::SftpPreview { tab_id, .. }
            | Self::SftpStatus { tab_id, .. }
            | Self::SftpEditOpened { tab_id, .. }
            | Self::SftpEditOpenFailed { tab_id, .. }
            | Self::SftpEditChanged { tab_id, .. }
            | Self::SftpEditUploadFinished { tab_id, .. }
            | Self::RemoteSystem { tab_id, .. }
            | Self::RemoteSystemUnavailable { tab_id, .. }
            | Self::ConnectionHealthy { tab_id, .. }
            | Self::ConnectionUnhealthy { tab_id, .. }
            | Self::SftpHome { tab_id, .. }
            | Self::TransferProgress { tab_id, .. }
            | Self::TransferStarted { tab_id, .. }
            | Self::TransferFileStarted { tab_id, .. }
            | Self::TransferFileFinished { tab_id, .. }
            | Self::Closed { tab_id, .. }
            | Self::TerminalTitleChanged { tab_id, .. }
            | Self::WorkingDirectoryChanged { tab_id, .. }
            | Self::WorkingDirectoryResolved { tab_id, .. } => Some(tab_id),
            Self::HostKeyVerification { request } => Some(&request.tab_id),
            Self::SftpOverwriteConflict { request } => Some(&request.tab_id),
            Self::SyncFinished(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BACKEND_EVENT_QUEUE_CAPACITY, BackendEvent, backend_event_channel,
        backend_event_channel_with_router,
    };

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

    #[test]
    fn registered_route_moves_existing_senders_to_the_new_receiver() {
        let (source, mut source_events) = backend_event_channel();
        source.register_route("terminal-a");
        let (target, mut target_events) = backend_event_channel_with_router(source.router());
        target.register_route("terminal-a");

        source
            .try_send(BackendEvent::Connected {
                tab_id: "terminal-a".to_string(),
            })
            .expect("route accepts event");

        assert!(source_events.try_recv().is_err());
        assert!(matches!(
            target_events.try_recv(),
            Ok(BackendEvent::Connected { tab_id }) if tab_id == "terminal-a"
        ));
    }
}
