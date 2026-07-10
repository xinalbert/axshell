mod runtime;

use std::sync::{Arc, Mutex};

use tokio::{
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};

use crate::{
    events::{BackendEvent, BackendEventSender},
    session::Session,
};

use self::runtime::run_sftp;
use super::session::SFTP_SHUTDOWN_TIMEOUT;

enum SftpCommand {
    ListDir {
        path: String,
        pin: SftpWorkPin,
    },
    LoadMoreEntries {
        pin: SftpWorkPin,
    },
    RevealPath {
        path: String,
        pin: SftpWorkPin,
    },
    #[allow(dead_code)]
    Preview {
        path: String,
        pin: SftpWorkPin,
    },
    Download {
        remote: String,
        local_dir: String,
        pin: SftpWorkPin,
    },
    EditFile {
        remote_path: String,
        pin: SftpWorkPin,
    },
    CreateDir {
        path: String,
        pin: SftpWorkPin,
    },
    DeletePaths {
        paths: Vec<String>,
        pin: SftpWorkPin,
    },
    UploadEditedFile {
        local_path: String,
        remote_path: String,
        pin: SftpWorkPin,
    },
    UploadPaths {
        locals: Vec<String>,
        remote_dir: String,
        pin: SftpWorkPin,
    },
    PauseTransfer(String),
    ResumeTransfer(String),
    CancelTransfer(String),
    TransferFinished(String),
    Close,
}

use std::sync::atomic::{AtomicUsize, Ordering};

struct SftpWorkTracker {
    pins: AtomicUsize,
}

impl SftpWorkTracker {
    fn pin(self: &Arc<Self>) -> SftpWorkPin {
        self.pins.fetch_add(1, Ordering::SeqCst);
        SftpWorkPin {
            tracker: self.clone(),
        }
    }

    fn active_pins(&self) -> usize {
        self.pins.load(Ordering::SeqCst)
    }
}

struct SftpWorkPin {
    tracker: Arc<SftpWorkTracker>,
}

impl Drop for SftpWorkPin {
    fn drop(&mut self) {
        let previous = self.tracker.pins.fetch_sub(1, Ordering::SeqCst);
        debug_assert!(previous > 0, "SFTP work pin underflow");
    }
}

pub(crate) struct SftpHandle {
    commands: UnboundedSender<SftpCommand>,
    worker: Arc<SftpWorker>,
}

struct SftpWorker {
    runtime: tokio::runtime::Handle,
    join: Mutex<Option<JoinHandle<()>>>,
    closing: std::sync::atomic::AtomicBool,
    work_tracker: Arc<SftpWorkTracker>,
}

impl Clone for SftpHandle {
    fn clone(&self) -> Self {
        Self {
            commands: self.commands.clone(),
            worker: self.worker.clone(),
        }
    }
}

impl SftpHandle {
    fn send_work_command(&self, build: impl FnOnce(SftpWorkPin) -> SftpCommand) -> bool {
        let pin = self.worker.work_tracker.pin();
        self.commands.send(build(pin)).is_ok()
    }

    pub(crate) fn commands_closed(&self) -> bool {
        self.commands.is_closed()
    }

    pub(crate) fn active_work_pins(&self) -> usize {
        self.worker.work_tracker.active_pins()
    }

    pub(crate) fn list_dir(&self, path: String) -> bool {
        self.send_work_command(|pin| SftpCommand::ListDir { path, pin })
    }

    pub(crate) fn load_more_entries(&self) -> bool {
        self.send_work_command(|pin| SftpCommand::LoadMoreEntries { pin })
    }

    pub(crate) fn reveal_path(&self, path: String) -> bool {
        self.send_work_command(|pin| SftpCommand::RevealPath { path, pin })
    }

    #[allow(dead_code)]
    pub(crate) fn preview(&self, path: String) -> bool {
        self.send_work_command(|pin| SftpCommand::Preview { path, pin })
    }

    pub(crate) fn download(&self, remote: String, local_dir: String) -> bool {
        self.send_work_command(|pin| SftpCommand::Download {
            remote,
            local_dir,
            pin,
        })
    }

    pub(crate) fn upload_paths(&self, locals: Vec<String>, remote_dir: String) -> bool {
        self.send_work_command(|pin| SftpCommand::UploadPaths {
            locals,
            remote_dir,
            pin,
        })
    }

    pub(crate) fn edit_file(&self, remote_path: String) -> bool {
        self.send_work_command(|pin| SftpCommand::EditFile { remote_path, pin })
    }

    pub(crate) fn create_dir(&self, path: String) -> bool {
        self.send_work_command(|pin| SftpCommand::CreateDir { path, pin })
    }

    pub(crate) fn delete_paths(&self, paths: Vec<String>) -> bool {
        self.send_work_command(|pin| SftpCommand::DeletePaths { paths, pin })
    }

    pub(crate) fn close(&self) {
        let _ = self.commands.send(SftpCommand::Close);
        if self.worker.closing.swap(true, Ordering::SeqCst) {
            return;
        }

        let join = self
            .worker
            .join
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        let Some(mut join) = join else {
            return;
        };

        self.worker.runtime.spawn(async move {
            if tokio::time::timeout(SFTP_SHUTDOWN_TIMEOUT, &mut join)
                .await
                .is_ok()
            {
                return;
            }

            tracing::warn!("[sftp] graceful shutdown timed out; aborting worker");
            join.abort();
            let _ = join.await;
        });
    }

    pub(crate) fn pause_transfer(&self, id: String) {
        let _ = self.commands.send(SftpCommand::PauseTransfer(id));
    }

    pub(crate) fn resume_transfer(&self, id: String) {
        let _ = self.commands.send(SftpCommand::ResumeTransfer(id));
    }

    pub(crate) fn cancel_transfer(&self, id: String) {
        let _ = self.commands.send(SftpCommand::CancelTransfer(id));
    }
}

pub fn spawn_sftp(
    runtime: &tokio::runtime::Handle,
    tab_id: String,
    session: Session,
    events: BackendEventSender,
) -> SftpHandle {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let cmd_tx_clone = cmd_tx.clone();
    let work_tracker = Arc::new(SftpWorkTracker {
        pins: AtomicUsize::new(0),
    });
    let initial_pin = work_tracker.pin();
    let worker_tracker = work_tracker.clone();
    let join = runtime.spawn(async move {
        if let Err(err) = run_sftp(
            tab_id.clone(),
            session,
            cmd_rx,
            cmd_tx_clone,
            events.clone(),
            worker_tracker,
            initial_pin,
        )
        .await
        {
            let _ = events
                .send(BackendEvent::SftpStatus {
                    tab_id: tab_id.clone(),
                    text: format!("sftp error: {err:#}"),
                })
                .await;
            let _ = events
                .send(BackendEvent::Closed {
                    tab_id,
                    reason: format!("sftp error: {err:#}"),
                })
                .await;
        }
    });
    SftpHandle {
        commands: cmd_tx,
        worker: Arc::new(SftpWorker {
            runtime: runtime.clone(),
            join: Mutex::new(Some(join)),
            closing: std::sync::atomic::AtomicBool::new(false),
            work_tracker,
        }),
    }
}
