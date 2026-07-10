use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicU64},
};

use anyhow::Result;
use russh::Disconnect;
use rust_i18n::t;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinSet,
};

use crate::{
    events::{BackendEvent, BackendEventSender},
    session::Session,
};

use super::{SftpCommand, SftpWorkPin, SftpWorkTracker};
use crate::sftp::{
    auth::connect_and_authenticate,
    browse::{
        DirectoryPage, close_browse_cursor, emit_browser_page, emit_next_browser_page,
        open_and_emit_browser_page, reveal_path_target,
    },
    operations::recursive_delete,
    path::{base_name, parent_dir},
    preview::preview_impl,
    session::{open_sftp_session, open_transfer_sftp_session},
    transfer::{
        TransferStateFlag, download_file_impl, download_path_impl, fail_transfer_start,
        send_sftp_status, send_transfer_error, upload_file_impl, upload_paths_impl,
    },
};

pub(super) async fn run_sftp(
    tab_id: String,
    session: Session,
    mut commands: UnboundedReceiver<SftpCommand>,
    commands_tx: UnboundedSender<SftpCommand>,
    events: BackendEventSender,
    work_tracker: Arc<SftpWorkTracker>,
    initial_pin: SftpWorkPin,
) -> Result<()> {
    let _ = events
        .send(BackendEvent::SftpStatus {
            tab_id: tab_id.clone(),
            text: t!("sftp_connecting").to_string(),
        })
        .await;

    let session_id = session.id.clone();
    let (handle, connected_mode) = connect_and_authenticate(&session).await?;
    let _ = events
        .send(BackendEvent::SshConnectionModeResolved {
            tab_id: tab_id.clone(),
            session_id,
            mode: connected_mode,
        })
        .await;
    let sftp = open_sftp_session(&handle).await?;

    let home = sftp
        .canonicalize(".")
        .await
        .unwrap_or_else(|_| "/".to_string());

    let _ = events
        .send(BackendEvent::SftpHome {
            tab_id: tab_id.clone(),
            home: home.clone(),
        })
        .await;

    let mut browse_cursor = None;
    open_and_emit_browser_page(&events, &tab_id, &handle, &home, &mut browse_cursor).await?;
    let mut browse_path = home.clone();
    drop(initial_pin);

    let mut active_transfers: std::collections::HashMap<String, TransferStateFlag> =
        std::collections::HashMap::new();
    let mut child_tasks = JoinSet::new();

    loop {
        let command = tokio::select! {
            command = commands.recv() => command,
            result = child_tasks.join_next(), if !child_tasks.is_empty() => {
                if let Some(Err(err)) = result
                    && !err.is_cancelled()
                {
                    tracing::warn!("[sftp] child task failed: {err}");
                }
                continue;
            }
        };
        let Some(command) = command else {
            cancel_sftp_child_tasks(&mut active_transfers, &mut child_tasks).await;
            close_browse_cursor(&mut browse_cursor).await;
            break;
        };
        match command {
            SftpCommand::Close => {
                cancel_sftp_child_tasks(&mut active_transfers, &mut child_tasks).await;
                close_browse_cursor(&mut browse_cursor).await;
                break;
            }
            SftpCommand::PauseTransfer(id) => {
                if let Some(flag) = active_transfers.get(&id) {
                    flag.pause();
                }
            }
            SftpCommand::ResumeTransfer(id) => {
                if let Some(flag) = active_transfers.get(&id) {
                    flag.resume();
                }
            }
            SftpCommand::CancelTransfer(id) => {
                if let Some(flag) = active_transfers.remove(&id) {
                    flag.cancel();
                }
            }
            SftpCommand::TransferFinished(id) => {
                active_transfers.remove(&id);
            }
            SftpCommand::ListDir { path, pin: _pin } => {
                let actual_path = if path == "~" {
                    home.clone()
                } else if let Some(rest) = path.strip_prefix("~/") {
                    crate::sftp::join_remote(&home, rest)
                } else {
                    path
                };

                match open_and_emit_browser_page(
                    &events,
                    &tab_id,
                    &handle,
                    &actual_path,
                    &mut browse_cursor,
                )
                .await
                {
                    Ok(()) => browse_path = actual_path,
                    Err(err) => {
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: format!("list failed: {err:#}"),
                            })
                            .await;
                    }
                }
            }
            SftpCommand::LoadMoreEntries { pin: _pin } => {
                if let Err(err) =
                    emit_next_browser_page(&events, &tab_id, &mut browse_cursor, true).await
                {
                    close_browse_cursor(&mut browse_cursor).await;
                    let _ = emit_browser_page(
                        &events,
                        &tab_id,
                        &browse_path,
                        DirectoryPage {
                            entries: Vec::new(),
                            has_more: false,
                            reached_limit: false,
                        },
                        true,
                    )
                    .await;
                    let _ = events
                        .send(BackendEvent::SftpStatus {
                            tab_id: tab_id.clone(),
                            text: format!("list failed: {err:#}"),
                        })
                        .await;
                }
            }
            SftpCommand::RevealPath { path, pin: _pin } => {
                let actual_path = if path == "~" {
                    home.clone()
                } else if let Some(rest) = path.strip_prefix("~/") {
                    crate::sftp::join_remote(&home, rest)
                } else {
                    path
                };

                match reveal_path_target(&handle, &actual_path).await {
                    Ok(directory) => {
                        match open_and_emit_browser_page(
                            &events,
                            &tab_id,
                            &handle,
                            &directory,
                            &mut browse_cursor,
                        )
                        .await
                        {
                            Ok(()) => browse_path = directory,
                            Err(err) => {
                                let _ = events
                                    .send(BackendEvent::SftpStatus {
                                        tab_id: tab_id.clone(),
                                        text: format!("list failed: {err:#}"),
                                    })
                                    .await;
                            }
                        }
                    }
                    Err(err) => {
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: format!("list failed: {err:#}"),
                            })
                            .await;
                    }
                }
            }
            SftpCommand::Preview { path, pin: _pin } => {
                match preview_impl(&sftp, &handle, &path).await {
                    Ok(preview) => {
                        let _ = events
                            .send(BackendEvent::SftpPreview {
                                tab_id: tab_id.clone(),
                                preview,
                            })
                            .await;
                    }
                    Err(err) => {
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: t!("preview_failed", err = format!("{err:#}")).into(),
                            })
                            .await;
                    }
                }
            }
            SftpCommand::Download {
                remote,
                local_dir,
                pin,
            } => {
                let id = uuid::Uuid::new_v4().to_string();
                let flag = TransferStateFlag::new();
                active_transfers.insert(id.clone(), TransferStateFlag(flag.0.clone()));

                let info = crate::sftp::TransferInfo {
                    id: id.clone(),
                    name: base_name(&remote).to_string(),
                    source: remote.clone(),
                    target: local_dir.clone(),
                    kind: crate::sftp::TransferType::Download,
                    total_bytes: None,
                };
                let _ = events
                    .send(BackendEvent::TransferStarted {
                        tab_id: tab_id.clone(),
                        info,
                    })
                    .await;

                let handle_clone = handle.clone();
                let events_clone = events.clone();
                let tab_id_clone = tab_id.clone();
                let commands_tx_clone = commands_tx.clone();

                child_tasks.spawn(async move {
                    let _transfer_pin = pin;
                    let sftp_session = match open_transfer_sftp_session(&handle_clone).await {
                        Ok(session) => session,
                        Err(err) => {
                            fail_transfer_start(&events_clone, &tab_id_clone, &id, "download", err)
                                .await;
                            let _ = commands_tx_clone.send(SftpCommand::TransferFinished(id));
                            return;
                        }
                    };

                    send_sftp_status(
                        &events_clone,
                        &tab_id_clone,
                        t!("downloading_file", base = base_name(&remote)).to_string(),
                    )
                    .await;

                    match download_path_impl(
                        &handle_clone,
                        &sftp_session,
                        &remote,
                        Path::new(&local_dir),
                        flag,
                        &events_clone,
                        &tab_id_clone,
                        &id,
                    )
                    .await
                    {
                        Ok(summary) => {
                            send_sftp_status(&events_clone, &tab_id_clone, summary).await;
                        }
                        Err(err) => {
                            let err_msg = format!("{err:#}");
                            send_transfer_error(
                                &events_clone,
                                &tab_id_clone,
                                &id,
                                err_msg.clone(),
                                t!("download_failed", err = err_msg).to_string(),
                            )
                            .await;
                        }
                    }
                    let _ = commands_tx_clone.send(SftpCommand::TransferFinished(id));
                });
            }
            SftpCommand::UploadPaths {
                locals,
                remote_dir,
                pin,
            } => {
                let id = uuid::Uuid::new_v4().to_string();
                let flag = TransferStateFlag::new();
                active_transfers.insert(id.clone(), TransferStateFlag(flag.0.clone()));

                let name = if locals.len() == 1 {
                    base_name(&locals[0]).to_string()
                } else {
                    let mut file_count = 0;
                    let mut folder_count = 0;
                    for local in &locals {
                        if std::path::Path::new(local).is_dir() {
                            folder_count += 1;
                        } else {
                            file_count += 1;
                        }
                    }
                    if file_count > 0 && folder_count == 0 {
                        t!("n_files", files = file_count).to_string()
                    } else if file_count == 0 && folder_count > 0 {
                        t!("n_folders", folders = folder_count).to_string()
                    } else {
                        t!(
                            "n_files_and_folders",
                            files = file_count,
                            folders = folder_count
                        )
                        .to_string()
                    }
                };

                let info = crate::sftp::TransferInfo {
                    id: id.clone(),
                    name,
                    source: "local".to_string(),
                    target: remote_dir.clone(),
                    kind: crate::sftp::TransferType::Upload,
                    total_bytes: None,
                };
                let _ = events
                    .send(BackendEvent::TransferStarted {
                        tab_id: tab_id.clone(),
                        info,
                    })
                    .await;

                let handle_clone = handle.clone();
                let events_clone = events.clone();
                let tab_id_clone = tab_id.clone();
                let commands_tx_clone = commands_tx.clone();
                let work_tracker_clone = work_tracker.clone();

                child_tasks.spawn(async move {
                    let _transfer_pin = pin;
                    let sftp_session = match open_transfer_sftp_session(&handle_clone).await {
                        Ok(session) => session,
                        Err(err) => {
                            fail_transfer_start(&events_clone, &tab_id_clone, &id, "upload", err)
                                .await;
                            let _ = commands_tx_clone.send(SftpCommand::TransferFinished(id));
                            return;
                        }
                    };

                    send_sftp_status(&events_clone, &tab_id_clone, t!("uploading").to_string())
                        .await;

                    match upload_paths_impl(
                        &sftp_session,
                        &locals,
                        &remote_dir,
                        flag,
                        &events_clone,
                        &tab_id_clone,
                        &id,
                    )
                    .await
                    {
                        Ok(summary) => {
                            send_sftp_status(&events_clone, &tab_id_clone, summary).await;
                            queue_list_dir(&commands_tx_clone, &work_tracker_clone, remote_dir);
                        }
                        Err(err) => {
                            let err_msg = format!("{err:#}");
                            send_transfer_error(
                                &events_clone,
                                &tab_id_clone,
                                &id,
                                err_msg.clone(),
                                t!("upload_failed", err = err_msg).to_string(),
                            )
                            .await;
                        }
                    }
                    let _ = commands_tx_clone.send(SftpCommand::TransferFinished(id));
                });
            }
            SftpCommand::EditFile { remote_path, pin } => {
                let id = uuid::Uuid::new_v4().to_string();
                let config = crate::config::ConfigStore::load()
                    .unwrap_or_else(|_| crate::config::ConfigStore::in_memory());
                let tmp_dir = config.tmp_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
                let base = base_name(&remote_path);
                let local_path = tmp_dir.join(format!("{}-{}", id, base));

                let handle_clone = handle.clone();
                let commands_tx_clone = commands_tx.clone();
                let events_clone = events.clone();
                let tab_id_clone = tab_id.clone();
                let work_tracker_clone = work_tracker.clone();

                child_tasks.spawn(async move {
                    let _edit_watcher_pin = pin;
                    let flag = TransferStateFlag::new();
                    let sftp_session = match open_transfer_sftp_session(&handle_clone).await {
                        Ok(session) => session,
                        Err(err) => {
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Edit download failed: {err:#}"),
                                })
                                .await;
                            return;
                        }
                    };

                    let _ = events_clone
                        .send(BackendEvent::SftpStatus {
                            tab_id: tab_id_clone.clone(),
                            text: t!("downloading_file", base = base).to_string(),
                        })
                        .await;

                    if let Err(err) = download_file_impl(
                        &sftp_session,
                        &remote_path,
                        &local_path,
                        &flag,
                        &events_clone,
                        &tab_id_clone,
                        "edit-download",
                    )
                    .await
                    {
                        let _ = events_clone
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id_clone.clone(),
                                text: format!("Edit download failed: {err:#}"),
                            })
                            .await;
                        return;
                    }

                    if let Err(err) = open::that(&local_path) {
                        let _ = events_clone
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id_clone.clone(),
                                text: format!("Failed to open editor: {err:#}"),
                            })
                            .await;
                        return;
                    }

                    use notify::Watcher;
                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                    let mut watcher = match notify::recommended_watcher(
                        move |res: notify::Result<notify::Event>| {
                            if let Ok(event) = res {
                                if event.kind.is_modify() {
                                    let _ = tx.send(());
                                }
                            }
                        },
                    ) {
                        Ok(w) => w,
                        Err(err) => {
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Failed to watch local edit file: {err:#}"),
                                })
                                .await;
                            return;
                        }
                    };

                    if let Err(err) =
                        watcher.watch(&local_path, notify::RecursiveMode::NonRecursive)
                    {
                        let _ = events_clone
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id_clone.clone(),
                                text: format!("Failed to watch local edit file: {err:#}"),
                            })
                            .await;
                        return;
                    }

                    while let Some(_) = rx.recv().await {
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        while let Ok(_) = rx.try_recv() {} // drain pending

                        let upload_pin = work_tracker_clone.pin();
                        if commands_tx_clone
                            .send(SftpCommand::UploadEditedFile {
                                local_path: local_path.to_string_lossy().to_string(),
                                remote_path: remote_path.clone(),
                                pin: upload_pin,
                            })
                            .is_err()
                        {
                            break;
                        }
                    }
                });
            }
            SftpCommand::UploadEditedFile {
                local_path,
                remote_path,
                pin,
            } => {
                let handle_clone = handle.clone();
                let events_clone = events.clone();
                let tab_id_clone = tab_id.clone();

                child_tasks.spawn(async move {
                    let _auto_upload_pin = pin;
                    let flag = TransferStateFlag::new();
                    let sftp_session = match open_transfer_sftp_session(&handle_clone).await {
                        Ok(session) => session,
                        Err(err) => {
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Auto-upload failed: {err:#}"),
                                })
                                .await;
                            return;
                        }
                    };

                    let transferred = Arc::new(AtomicU64::new(0));
                    match upload_file_impl(
                        &sftp_session,
                        Path::new(&local_path),
                        &remote_path,
                        &flag,
                        &events_clone,
                        &tab_id_clone,
                        "edit-upload",
                        transferred,
                        None,
                    )
                    .await
                    {
                        Ok(_) => {
                            let now = chrono::Local::now().format("%H:%M:%S");
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!(
                                        "{} ({})",
                                        t!(
                                            "auto_saved_and_uploaded",
                                            base = base_name(&remote_path)
                                        ),
                                        now
                                    ),
                                })
                                .await;
                        }
                        Err(err) => {
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Auto-upload failed: {err:#}"),
                                })
                                .await;
                        }
                    }
                });
            }
            SftpCommand::CreateDir { path, pin: _pin } => {
                let actual_path = if path == "~" {
                    home.clone()
                } else if let Some(rest) = path.strip_prefix("~/") {
                    crate::sftp::join_remote(&home, rest)
                } else {
                    path.clone()
                };

                tracing::info!("[sftp] creating directory: '{}'", actual_path);

                match sftp.create_dir(&actual_path).await {
                    Ok(_) => {
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: t!("create_folder_success", name = base_name(&actual_path))
                                    .to_string(),
                            })
                            .await;

                        // Re-fetch the parent directory to show the newly created folder
                        if let Some(parent) = parent_dir(&actual_path) {
                            queue_list_dir(&commands_tx, &work_tracker, parent);
                        } else {
                            queue_list_dir(&commands_tx, &work_tracker, "/".to_string());
                        }
                    }
                    Err(err) => {
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: t!("create_folder_failed", err = format!("{err:#}"))
                                    .to_string(),
                            })
                            .await;
                    }
                }
            }
            SftpCommand::DeletePaths { paths, pin: _pin } => {
                tracing::info!("[sftp] batch deleting {} paths", paths.len());
                let _ = events
                    .send(BackendEvent::SftpStatus {
                        tab_id: tab_id.clone(),
                        text: t!("deleting_paths", count = paths.len()).to_string(),
                    })
                    .await;

                let mut errors = Vec::new();
                for path in paths.clone() {
                    let actual_path = if path == "~" {
                        home.clone()
                    } else if let Some(rest) = path.strip_prefix("~/") {
                        crate::sftp::join_remote(&home, rest)
                    } else {
                        path.clone()
                    };

                    if let Err(e) = recursive_delete(&sftp, actual_path).await {
                        errors.push(format!("{path}: {e:#}"));
                    }
                }

                if errors.is_empty() {
                    let _ = events
                        .send(BackendEvent::SftpStatus {
                            tab_id: tab_id.clone(),
                            text: t!("delete_success", count = paths.len()).to_string(),
                        })
                        .await;
                } else {
                    let _ = events
                        .send(BackendEvent::SftpStatus {
                            tab_id: tab_id.clone(),
                            text: t!("delete_failed", err = errors.join(", ")).to_string(),
                        })
                        .await;
                }

                if let Some(first) = paths.first() {
                    let actual_path = if first == "~" {
                        home.clone()
                    } else if let Some(rest) = first.strip_prefix("~/") {
                        crate::sftp::join_remote(&home, rest)
                    } else {
                        first.clone()
                    };
                    if let Some(parent) = parent_dir(&actual_path) {
                        queue_list_dir(&commands_tx, &work_tracker, parent);
                    } else {
                        queue_list_dir(&commands_tx, &work_tracker, "/".to_string());
                    }
                }
            }
        }
    }

    let _ = handle
        .disconnect(Disconnect::ByApplication, "bye", "")
        .await;
    Ok(())
}

fn queue_list_dir(
    commands: &UnboundedSender<SftpCommand>,
    work_tracker: &Arc<SftpWorkTracker>,
    path: String,
) {
    let pin = work_tracker.pin();
    let _ = commands.send(SftpCommand::ListDir { path, pin });
}

async fn cancel_sftp_child_tasks(
    active_transfers: &mut std::collections::HashMap<String, TransferStateFlag>,
    child_tasks: &mut JoinSet<()>,
) {
    for transfer in active_transfers.values() {
        transfer.cancel();
    }
    active_transfers.clear();
    child_tasks.abort_all();
    while child_tasks.join_next().await.is_some() {}
}

#[cfg(test)]
mod lifecycle_tests {
    use std::{collections::HashMap, sync::atomic::Ordering};

    use tokio::task::JoinSet;

    use super::{SftpWorkTracker, TransferStateFlag, cancel_sftp_child_tasks};

    #[tokio::test]
    async fn closing_worker_cancels_transfers_and_aborts_child_tasks() {
        let transfer = TransferStateFlag::new();
        let transfer_state = transfer.0.clone();
        let mut active_transfers = HashMap::from([("transfer-1".to_string(), transfer)]);
        let mut child_tasks = JoinSet::new();
        child_tasks.spawn(async {
            std::future::pending::<()>().await;
        });

        cancel_sftp_child_tasks(&mut active_transfers, &mut child_tasks).await;

        assert_eq!(transfer_state.load(Ordering::SeqCst), 2);
        assert!(active_transfers.is_empty());
        assert!(child_tasks.is_empty());
    }

    #[test]
    fn work_pins_remain_active_until_the_last_guard_drops() {
        let tracker = std::sync::Arc::new(SftpWorkTracker {
            pins: std::sync::atomic::AtomicUsize::new(0),
        });
        let first = tracker.pin();
        let second = tracker.pin();

        assert_eq!(tracker.active_pins(), 2);
        drop(first);
        assert_eq!(tracker.active_pins(), 1);
        drop(second);
        assert_eq!(tracker.active_pins(), 0);
    }
}
