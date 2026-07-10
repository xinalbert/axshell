use std::time::Instant;

use tokio::runtime::Runtime;

use crate::events::{BackendEvent, BackendEventSender};

pub(crate) struct RuntimeState {
    pub(crate) runtime: Runtime,
    pub(crate) events_rx: tokio::sync::mpsc::Receiver<BackendEvent>,
    pub(crate) events_tx: BackendEventSender,
    pub(crate) pending_terminal_refresh: bool,
    pub(crate) last_terminal_refresh: Instant,
    pub(crate) pending_ui_refresh: bool,
    pub(crate) last_ui_refresh: Instant,
    pub(crate) last_sftp_idle_sweep: Instant,
}
