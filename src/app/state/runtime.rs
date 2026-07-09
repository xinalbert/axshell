use std::{sync::mpsc, time::Instant};

use tokio::runtime::Runtime;

use crate::terminal::BackendEvent;

pub(crate) struct RuntimeState {
    pub(crate) runtime: Runtime,
    pub(crate) events_rx: mpsc::Receiver<BackendEvent>,
    pub(crate) events_tx: mpsc::Sender<BackendEvent>,
    pub(crate) pending_terminal_refresh: bool,
    pub(crate) last_terminal_refresh: Instant,
}
