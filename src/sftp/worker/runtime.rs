use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use anyhow::{Context, Result};
use russh::Disconnect;
use rust_i18n::t;
use sha2::{Digest, Sha256};
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinSet,
};

use crate::{
    events::{BackendEvent, BackendEventSender},
    session::Session,
};

use super::{SftpCommand, SftpInitialRequest, SftpWorkPin, SftpWorkTracker};
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
        DownloadOverwritePolicy, TransferStateFlag, download_file_impl, download_path_impl,
        fail_transfer_start, send_sftp_status, send_transfer_error, send_transfer_progress,
        upload_file_impl, upload_paths_impl,
    },
};

pub(super) async fn run_sftp(
    tab_id: String,
    session: Session,
    initial_request: SftpInitialRequest,
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

    let handle = connect_and_authenticate(&tab_id, &session, &events).await?;
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
    let mut browse_path = match initial_request {
        SftpInitialRequest::Browse(initial_path) => {
            let initial_path = sftp_initial_path(initial_path.as_deref(), &home);
            open_and_emit_browser_page(
                &events,
                &tab_id,
                &handle,
                &initial_path,
                &mut browse_cursor,
            )
            .await?;
            initial_path
        }
        SftpInitialRequest::Reveal(path) => {
            let target_path = sftp_initial_path(Some(&path), &home);
            let directory = reveal_path_target(&handle, &target_path).await?;
            open_and_emit_browser_page(&events, &tab_id, &handle, &directory, &mut browse_cursor)
                .await?;
            directory
        }
    };
    drop(initial_pin);

    let mut active_transfers: std::collections::HashMap<String, TransferStateFlag> =
        std::collections::HashMap::new();
    let mut edit_watchers: std::collections::HashMap<String, tokio::sync::oneshot::Sender<()>> =
        std::collections::HashMap::new();
    let mut child_tasks = JoinSet::new();

    loop {
        let command = tokio::select! {
            command = commands.recv() => command,
            result = child_tasks.join_next(), if !child_tasks.is_empty() => {
                if let Some(Err(err)) = result
                    && !err.is_cancelled()
                {
                    tracing::warn!(
                        component = "sftp",
                        operation = "child_task",
                        tab_id = %tab_id,
                        error = %crate::diagnostics::sanitize_error(&err.to_string()),
                        "SFTP child task failed"
                    );
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
            SftpCommand::FinishEditFile { local_path } => {
                if let Some(stop) = edit_watchers.remove(&local_path) {
                    let _ = stop.send(());
                }
            }
            SftpCommand::EditWatcherFinished { local_path } => {
                edit_watchers.remove(&local_path);
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
                        log_sftp_error("list_dir", &tab_id, &err);
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
                    log_sftp_error("load_more", &tab_id, &err);
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
                                log_sftp_error("reveal_list", &tab_id, &err);
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
                        log_sftp_error("reveal_path", &tab_id, &err);
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
                        log_sftp_error("preview", &tab_id, &err);
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: t!("preview_failed", err = format!("{err:#}")).into(),
                            })
                            .await;
                    }
                }
            }
            SftpCommand::DownloadPaths {
                remotes,
                local_dir,
                pin,
            } => {
                let id = uuid::Uuid::new_v4().to_string();
                let flag = TransferStateFlag::new();
                active_transfers.insert(id.clone(), TransferStateFlag(flag.0.clone()));

                let item_count = remotes.len();
                let name = if item_count == 1 {
                    base_name(&remotes[0]).to_string()
                } else {
                    t!("n_files", files = item_count).to_string()
                };
                let info = crate::sftp::TransferInfo {
                    id: id.clone(),
                    name,
                    source: "remote".to_string(),
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

                    let local_dir = PathBuf::from(local_dir);
                    let mut failures = Vec::new();
                    let mut overwrite_policy = DownloadOverwritePolicy::default();
                    for remote in remotes {
                        if flag.0.load(Ordering::SeqCst) == 2 {
                            failures.clear();
                            failures.push("transfer cancelled".to_string());
                            break;
                        }
                        send_sftp_status(
                            &events_clone,
                            &tab_id_clone,
                            t!("downloading_file", base = base_name(&remote)).to_string(),
                        )
                        .await;

                        if let Err(err) = download_path_impl(
                            &sftp_session,
                            &remote,
                            &local_dir,
                            &flag,
                            &mut overwrite_policy,
                            &events_clone,
                            &tab_id_clone,
                            &id,
                            false,
                        )
                        .await
                        {
                            failures.push(format!("{}: {err:#}", base_name(&remote)));
                            if flag.0.load(Ordering::SeqCst) == 2 {
                                failures.clear();
                                failures.push("transfer cancelled".to_string());
                                break;
                            }
                        }
                    }
                    if failures.is_empty() {
                        send_transfer_progress(
                            &events_clone,
                            &tab_id_clone,
                            &id,
                            0,
                            None,
                            crate::sftp::TransferState::Completed,
                        )
                        .await;
                    } else {
                        let err_msg = failures.join("; ");
                        send_transfer_error(
                            &events_clone,
                            &tab_id_clone,
                            &id,
                            err_msg.clone(),
                            t!("download_failed", err = err_msg).to_string(),
                        )
                        .await;
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
            SftpCommand::OpenFile {
                remote_path,
                watch_changes,
                pin,
            } => {
                let operation = if watch_changes {
                    "edit_remote_file"
                } else {
                    "open_remote_file"
                };
                let id = uuid::Uuid::new_v4().to_string();
                let config = crate::config::ConfigStore::load()
                    .unwrap_or_else(|_| crate::config::ConfigStore::in_memory());
                let local_dir = if watch_changes {
                    config
                        .sftp_edit_dir()
                        .unwrap_or_else(|| std::env::temp_dir().join("ax-shell-sftp-edits"))
                } else {
                    config.tmp_dir().unwrap_or_else(std::env::temp_dir)
                };
                let base = base_name(&remote_path);
                let local_path = local_dir.join(format!("{}-{}", id, base));

                let handle_clone = handle.clone();
                let events_clone = events.clone();
                let tab_id_clone = tab_id.clone();
                let edit_watcher_path = local_path.to_string_lossy().to_string();
                let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel();
                if watch_changes {
                    edit_watchers.insert(edit_watcher_path.clone(), stop_tx);
                }
                let watcher_cleanup = watch_changes.then(|| EditWatcherCleanup {
                    commands: commands_tx.clone(),
                    local_path: edit_watcher_path,
                });

                child_tasks.spawn(async move {
                    let _edit_watcher_pin = pin;
                    let _watcher_cleanup = watcher_cleanup;
                    if watch_changes && let Err(err) = tokio::fs::create_dir_all(&local_dir).await {
                        log_sftp_error("create_edit_directory", &tab_id_clone, &err);
                        let _ = events_clone
                            .send(BackendEvent::SftpEditOpenFailed {
                                tab_id: tab_id_clone.clone(),
                                remote_path,
                                reason: format!("{err:#}"),
                            })
                            .await;
                        return;
                    }
                    let flag = TransferStateFlag::new();
                    let sftp_session = match open_transfer_sftp_session(&handle_clone).await {
                        Ok(session) => session,
                        Err(err) => {
                            log_sftp_error(operation, &tab_id_clone, &err);
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("File download failed: {err:#}"),
                                })
                                .await;
                            if watch_changes {
                                let _ = events_clone
                                    .send(BackendEvent::SftpEditOpenFailed {
                                        tab_id: tab_id_clone.clone(),
                                        remote_path,
                                        reason: format!("{err:#}"),
                                    })
                                    .await;
                            }
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
                        None,
                        &events_clone,
                        &tab_id_clone,
                        operation,
                        true,
                        false,
                    )
                    .await
                    {
                        log_sftp_error(operation, &tab_id_clone, &err);
                        let _ = events_clone
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id_clone.clone(),
                                text: format!("File download failed: {err:#}"),
                            })
                            .await;
                        if watch_changes {
                            let _ = events_clone
                                .send(BackendEvent::SftpEditOpenFailed {
                                    tab_id: tab_id_clone.clone(),
                                    remote_path,
                                    reason: format!("{err:#}"),
                                })
                                .await;
                        }
                        return;
                    }

                    if !watch_changes {
                        if let Err(err) = open::that(&local_path) {
                            log_sftp_error(operation, &tab_id_clone, &err);
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Failed to open file: {err:#}"),
                                })
                                .await;
                        }
                        return;
                    }

                    let original_fingerprint = match local_file_fingerprint(&local_path).await {
                        Ok(fingerprint) => fingerprint,
                        Err(err) => {
                            log_sftp_error("edit_fingerprint", &tab_id_clone, &err);
                            let _ = events_clone
                                .send(BackendEvent::SftpEditOpenFailed {
                                    tab_id: tab_id_clone.clone(),
                                    remote_path,
                                    reason: format!("{err:#}"),
                                })
                                .await;
                            return;
                        }
                    };
                    let local_path_string = local_path.to_string_lossy().to_string();

                    use notify::Watcher;
                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                    let watcher_tab_id = tab_id_clone.clone();
                    let watched_file = local_path.clone();
                    let mut watcher = match notify::recommended_watcher(
                        move |res: notify::Result<notify::Event>| match res {
                            Ok(event) if is_edit_file_event(&event, &watched_file) => {
                                let _ = tx.send(EditWatcherSignal::Changed);
                            }
                            Ok(_) => {}
                            Err(err) => {
                                log_sftp_error("edit_watch_event", &watcher_tab_id, &err);
                            }
                        },
                    ) {
                        Ok(w) => w,
                        Err(err) => {
                            log_sftp_error("edit_create_watcher", &tab_id_clone, &err);
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Failed to watch local edit file: {err:#}"),
                                })
                                .await;
                            return;
                        }
                    };

                    let watch_path = local_path.parent().unwrap_or_else(|| Path::new("."));
                    if let Err(err) = watcher.watch(watch_path, notify::RecursiveMode::NonRecursive)
                    {
                        log_sftp_error("edit_watch_file", &tab_id_clone, &err);
                        let _ = events_clone
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id_clone.clone(),
                                text: format!("Failed to watch local edit file: {err:#}"),
                            })
                            .await;
                        return;
                    }

                    let _ = events_clone
                        .send(BackendEvent::SftpEditOpened {
                            tab_id: tab_id_clone.clone(),
                            remote_path: remote_path.clone(),
                            local_path: local_path_string.clone(),
                        })
                        .await;
                    if let Err(err) = open::that(&local_path) {
                        log_sftp_error(operation, &tab_id_clone, &err);
                        let _ = events_clone
                            .send(BackendEvent::SftpEditOpenFailed {
                                tab_id: tab_id_clone.clone(),
                                remote_path,
                                reason: format!("{err:#}"),
                            })
                            .await;
                        return;
                    }

                    let mut dirty = false;
                    loop {
                        tokio::select! {
                            _ = &mut stop_rx => break,
                            signal = rx.recv() => if signal.is_none() { break },
                        };
                        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                        while rx.try_recv().is_ok() {}

                        if let Ok(fingerprint) = local_file_fingerprint(&local_path).await {
                            let changed = fingerprint != original_fingerprint;
                            if changed != dirty {
                                dirty = changed;
                                let _ = events_clone
                                    .send(BackendEvent::SftpEditChanged {
                                        tab_id: tab_id_clone.clone(),
                                        remote_path: remote_path.clone(),
                                        local_path: local_path_string.clone(),
                                        dirty,
                                    })
                                    .await;
                            }
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
                            let message = format!("{err:#}");
                            log_sftp_error("edit_upload_session", &tab_id_clone, &err);
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Upload failed: {message}"),
                                })
                                .await;
                            let _ = events_clone
                                .send(BackendEvent::SftpEditUploadFinished {
                                    tab_id: tab_id_clone.clone(),
                                    remote_path,
                                    local_path,
                                    result: Err(message),
                                })
                                .await;
                            return;
                        }
                    };

                    let transferred = Arc::new(AtomicU64::new(0));
                    let result = upload_file_impl(
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
                    .await;
                    match result {
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
                            let _ = events_clone
                                .send(BackendEvent::SftpEditUploadFinished {
                                    tab_id: tab_id_clone.clone(),
                                    remote_path,
                                    local_path,
                                    result: Ok(()),
                                })
                                .await;
                        }
                        Err(err) => {
                            let message = format!("{err:#}");
                            log_sftp_error("edit_upload", &tab_id_clone, &err);
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Upload failed: {message}"),
                                })
                                .await;
                            let _ = events_clone
                                .send(BackendEvent::SftpEditUploadFinished {
                                    tab_id: tab_id_clone.clone(),
                                    remote_path,
                                    local_path,
                                    result: Err(message),
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

                tracing::info!(
                    component = "sftp",
                    operation = "create_dir",
                    tab_id = %tab_id,
                    remote_path = %crate::diagnostics::mask_path(&actual_path),
                    "Creating SFTP directory"
                );

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
                        log_sftp_error("create_dir", &tab_id, &err);
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
                tracing::info!(
                    component = "sftp",
                    operation = "delete_paths",
                    tab_id = %tab_id,
                    item_count = paths.len(),
                    "Deleting SFTP paths"
                );
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
                    let error = crate::diagnostics::sanitize_error(&errors.join(", "));
                    tracing::error!(
                        component = "sftp",
                        operation = "delete_paths",
                        tab_id = %tab_id,
                        failed_count = errors.len(),
                        error = %error,
                        "SFTP delete failed"
                    );
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

fn log_sftp_error(operation: &'static str, tab_id: &str, error: &dyn std::fmt::Display) {
    let error = crate::diagnostics::sanitize_error(&error.to_string());
    tracing::error!(
        component = "sftp",
        operation,
        tab_id,
        error = %error,
        "SFTP operation failed"
    );
}

fn queue_list_dir(
    commands: &UnboundedSender<SftpCommand>,
    work_tracker: &Arc<SftpWorkTracker>,
    path: String,
) {
    let pin = work_tracker.pin();
    let _ = commands.send(SftpCommand::ListDir { path, pin });
}

fn sftp_initial_path(initial_path: Option<&str>, home: &str) -> String {
    if let Some(path) = initial_path.filter(|path| !path.trim().is_empty()) {
        return crate::sftp::resolve_remote_path(home, path, home);
    }
    home.to_string()
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

#[derive(Clone, Copy)]
enum EditWatcherSignal {
    Changed,
}

struct EditWatcherCleanup {
    commands: UnboundedSender<SftpCommand>,
    local_path: String,
}

impl Drop for EditWatcherCleanup {
    fn drop(&mut self) {
        let _ = self.commands.send(SftpCommand::EditWatcherFinished {
            local_path: self.local_path.clone(),
        });
    }
}

fn is_edit_file_event(event: &notify::Event, local_path: &Path) -> bool {
    (event.kind.is_create() || event.kind.is_modify() || event.kind.is_remove())
        && event.paths.iter().any(|path| path == local_path)
}

async fn local_file_fingerprint(path: &Path) -> Result<[u8; 32]> {
    let contents = tokio::fs::read(path)
        .await
        .with_context(|| format!("read local edit copy {}", path.display()))?;
    Ok(Sha256::digest(contents).into())
}

#[cfg(test)]
mod lifecycle_tests {
    use std::{collections::HashMap, path::PathBuf, sync::atomic::Ordering};

    use notify::{
        Event, EventKind,
        event::{CreateKind, ModifyKind, RemoveKind},
    };
    use tokio::task::JoinSet;

    use super::{
        SftpWorkTracker, TransferStateFlag, cancel_sftp_child_tasks, is_edit_file_event,
        sftp_initial_path,
    };

    #[test]
    fn sftp_initial_path_uses_the_selected_path_or_server_home() {
        let home = "/C:/Users/Administrator";

        assert_eq!(sftp_initial_path(None, home), home);
        assert_eq!(
            sftp_initial_path(Some("/srv/last-opened"), home),
            "/srv/last-opened"
        );
        assert_eq!(
            sftp_initial_path(Some("~/2026"), home),
            "/C:/Users/Administrator/2026"
        );
        assert_eq!(
            sftp_initial_path(Some("/G:/albertxin/2026"), home),
            "/G:/albertxin/2026"
        );
    }

    #[test]
    fn edit_watcher_only_reacts_to_the_managed_copy() {
        let managed = PathBuf::from("/tmp/sftp-edits/managed.txt");
        let other = PathBuf::from("/tmp/sftp-edits/other.txt");

        for kind in [
            EventKind::Create(CreateKind::File),
            EventKind::Modify(ModifyKind::Any),
            EventKind::Remove(RemoveKind::File),
        ] {
            assert!(is_edit_file_event(
                &Event::new(kind).add_path(managed.clone()),
                &managed
            ));
            assert!(!is_edit_file_event(
                &Event::new(kind).add_path(other.clone()),
                &managed
            ));
        }
    }

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
