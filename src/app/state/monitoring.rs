use std::time::Instant;

use gpui::SharedString;

use crate::system::{SystemSampler, SystemSnapshot};

pub(crate) struct MonitoringState {
    pub(crate) status: Option<SharedString>,
    pub(crate) sampler: SystemSampler,
    pub(crate) system: SystemSnapshot,
    pub(crate) cpu_history: Vec<f32>,
    pub(crate) net_rx_history: Vec<f32>,
    pub(crate) net_tx_history: Vec<f32>,
    pub(crate) last_sample: Instant,
    pub(crate) system_tab_id: Option<String>,
    pub(crate) remote_sample_in_flight: bool,
}
