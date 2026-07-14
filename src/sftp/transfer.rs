use std::{
    path::{Component, Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU8, AtomicU64, Ordering},
    },
};

use anyhow::{Context, Result};
use russh_sftp::client::SftpSession;
use rust_i18n::t;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use walkdir::WalkDir;

use crate::events::{BackendEvent, BackendEventSender};

use super::model::{SftpOverwriteDecision, SftpOverwriteRequest, TransferState};

pub(super) struct TransferStateFlag(pub(super) Arc<AtomicU8>);

impl TransferStateFlag {
    pub(super) fn new() -> Self {
        Self(Arc::new(AtomicU8::new(0)))
    }

    pub(super) fn pause(&self) {
        self.0.store(1, Ordering::SeqCst);
    }
    pub(super) fn resume(&self) {
        self.0.store(0, Ordering::SeqCst);
    }
    pub(super) fn cancel(&self) {
        self.0.store(2, Ordering::SeqCst);
    }

    pub(super) async fn yield_if_paused(
        &self,
        events: &BackendEventSender,
        tab_id: &str,
        id: &str,
        transferred: u64,
        total: Option<u64>,
    ) -> anyhow::Result<()> {
        let mut was_paused = false;
        loop {
            let state = self.0.load(Ordering::SeqCst);
            if state == 2 {
                return Err(anyhow::anyhow!("transfer cancelled"));
            }
            if state == 1 {
                if !was_paused {
                    send_transfer_progress(
                        events,
                        tab_id,
                        id,
                        transferred,
                        total,
                        TransferState::Paused,
                    )
                    .await;
                    was_paused = true;
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            } else {
                if was_paused {
                    send_transfer_progress(
                        events,
                        tab_id,
                        id,
                        transferred,
                        total,
                        TransferState::Running,
                    )
                    .await;
                }
                return Ok(());
            }
        }
    }
}

pub(super) async fn send_sftp_status(
    events: &BackendEventSender,
    tab_id: &str,
    text: impl Into<String>,
) {
    let _ = events
        .send(BackendEvent::SftpStatus {
            tab_id: tab_id.to_string(),
            text: text.into(),
        })
        .await;
}

pub(super) async fn send_transfer_progress(
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
    transferred: u64,
    total: Option<u64>,
    state: TransferState,
) {
    let _ = events
        .send(BackendEvent::TransferProgress {
            tab_id: tab_id.to_string(),
            id: id.to_string(),
            transferred,
            total,
            state,
        })
        .await;
}

pub(super) async fn send_transfer_error(
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
    err_msg: String,
    failed_status: String,
) {
    let is_cancelled = err_msg.contains("transfer cancelled");
    let error = crate::diagnostics::sanitize_error(&err_msg);
    if is_cancelled {
        tracing::info!(
            component = "sftp",
            operation = "transfer",
            tab_id,
            transfer_id = id,
            "SFTP transfer cancelled"
        );
    } else {
        tracing::error!(
            component = "sftp",
            operation = "transfer",
            tab_id,
            transfer_id = id,
            error = %error,
            "SFTP transfer failed"
        );
    }
    let state = if is_cancelled {
        TransferState::Interrupted("User cancelled".to_string())
    } else {
        TransferState::Failed(err_msg)
    };
    send_sftp_status(
        events,
        tab_id,
        if is_cancelled {
            "Transmission cancelled".to_string()
        } else {
            failed_status
        },
    )
    .await;
    send_transfer_progress(events, tab_id, id, 0, None, state).await;
}

pub(super) async fn fail_transfer_start(
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
    action: &str,
    err: anyhow::Error,
) {
    let err_msg = format!("{err:#}");
    send_transfer_error(
        events,
        tab_id,
        id,
        err_msg.clone(),
        format!("{action} failed: {err_msg}"),
    )
    .await;
}

use super::{
    browse::list_dir_impl,
    path::{base_name, join_remote},
};

#[derive(Default)]
pub(super) struct DownloadOverwritePolicy {
    replace_remaining_in_transfer: bool,
}

impl DownloadOverwritePolicy {
    fn replace_existing_file(&self) -> bool {
        self.replace_remaining_in_transfer
    }

    fn apply(&mut self, decision: SftpOverwriteDecision) {
        if decision == SftpOverwriteDecision::ReplaceAllInTransfer {
            self.replace_remaining_in_transfer = true;
        }
    }
}

pub(super) async fn download_path_impl(
    sftp: &SftpSession,
    remote: &str,
    local_dir: &Path,
    flag: &TransferStateFlag,
    overwrite_policy: &mut DownloadOverwritePolicy,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
    report_completion: bool,
) -> Result<String> {
    tokio::fs::create_dir_all(local_dir)
        .await
        .with_context(|| format!("create {}", local_dir.display()))?;

    // Check for cancellation after initial setup
    let state = flag.0.load(Ordering::SeqCst);
    if state == 2 {
        return Err(anyhow::anyhow!("transfer cancelled"));
    }

    let metadata = sftp
        .metadata(remote)
        .await
        .with_context(|| format!("metadata {remote}"))?;
    let is_dir = metadata
        .permissions
        .map(|mode| (mode & 0o170_000) == 0o040_000)
        .unwrap_or(false);

    if is_dir {
        let local_path = local_path_for_remote_entry(local_dir, &base_name(remote))?;
        download_dir_recursive(
            sftp,
            remote,
            &local_path,
            flag,
            overwrite_policy,
            events,
            tab_id,
            id,
        )
        .await?;
        return Ok(t!("downloaded_folder", path = local_path.display()).to_string());
    }

    let local_path = local_path_for_remote_entry(local_dir, &base_name(remote))?;
    let downloaded = download_file_impl(
        sftp,
        remote,
        &local_path,
        flag,
        Some(overwrite_policy),
        events,
        tab_id,
        id,
        report_completion,
    )
    .await?;
    if !downloaded {
        send_sftp_status(
            events,
            tab_id,
            t!("download_skipped", path = local_path.display()).to_string(),
        )
        .await;
    }
    Ok(t!("downloaded_file", path = local_path.display()).to_string())
}

async fn download_dir_recursive(
    sftp: &SftpSession,
    remote_dir: &str,
    local_dir: &Path,
    flag: &TransferStateFlag,
    overwrite_policy: &mut DownloadOverwritePolicy,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
) -> Result<()> {
    flag.yield_if_paused(events, tab_id, id, 0, None).await?;
    tokio::fs::create_dir_all(local_dir)
        .await
        .with_context(|| format!("create {}", local_dir.display()))?;
    let entries = list_dir_impl(sftp, remote_dir).await?;
    for entry in entries {
        flag.yield_if_paused(events, tab_id, id, 0, None).await?;
        let local_path = local_path_for_remote_entry(local_dir, &entry.name)?;
        if entry.is_dir {
            Box::pin(download_dir_recursive(
                sftp,
                &entry.full_path,
                &local_path,
                flag,
                overwrite_policy,
                events,
                tab_id,
                id,
            ))
            .await?;
        } else {
            let downloaded = download_file_impl(
                sftp,
                &entry.full_path,
                &local_path,
                flag,
                Some(overwrite_policy),
                events,
                tab_id,
                id,
                false,
            )
            .await?;
            if !downloaded {
                send_sftp_status(
                    events,
                    tab_id,
                    t!("download_skipped", path = local_path.display()).to_string(),
                )
                .await;
            }
        }
    }
    Ok(())
}

pub(super) async fn download_file_impl(
    sftp: &SftpSession,
    remote: &str,
    local: &Path,
    flag: &TransferStateFlag,
    overwrite_policy: Option<&mut DownloadOverwritePolicy>,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
    report_completion: bool,
) -> Result<bool> {
    if !confirm_local_overwrite(local, remote, flag, overwrite_policy, events, tab_id, id).await? {
        return Ok(false);
    }

    let mut remote_file = sftp
        .open(remote)
        .await
        .with_context(|| format!("open remote {remote}"))?;
    let mut local_file = tokio::fs::File::create(local)
        .await
        .with_context(|| format!("create local {}", local.display()))?;

    let total = sftp.metadata(remote).await.ok().and_then(|m| m.size);
    let mut transferred = 0u64;

    let mut buffer = vec![0u8; 128 * 1024];
    loop {
        flag.yield_if_paused(events, tab_id, id, transferred, total)
            .await?;
        let read = remote_file
            .read(&mut buffer)
            .await
            .context("read remote file")?;
        if read == 0 {
            break;
        }
        local_file
            .write_all(&buffer[..read])
            .await
            .with_context(|| format!("write {}", local.display()))?;

        transferred += read as u64;
        send_transfer_progress(
            events,
            tab_id,
            id,
            transferred,
            total,
            TransferState::Running,
        )
        .await;
    }
    local_file.flush().await.context("flush local file")?;

    if report_completion {
        send_transfer_progress(
            events,
            tab_id,
            id,
            transferred,
            total,
            TransferState::Completed,
        )
        .await;
    }

    Ok(true)
}

fn local_path_for_remote_entry(parent: &Path, name: &str) -> Result<PathBuf> {
    let mut components = Path::new(name).components();
    let is_single_normal_component =
        matches!(components.next(), Some(Component::Normal(_))) && components.next().is_none();
    if name.is_empty()
        || name == "."
        || name == ".."
        || name.contains('/')
        || name.contains('\\')
        || !is_single_normal_component
    {
        return Err(anyhow::anyhow!("unsafe remote entry name: {name:?}"));
    }
    Ok(parent.join(name))
}

async fn confirm_local_overwrite(
    local: &Path,
    remote: &str,
    flag: &TransferStateFlag,
    overwrite_policy: Option<&mut DownloadOverwritePolicy>,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
) -> Result<bool> {
    let exists = tokio::fs::try_exists(local)
        .await
        .with_context(|| format!("check local destination {}", local.display()))?;
    if !exists {
        return Ok(true);
    }

    let metadata = tokio::fs::metadata(local)
        .await
        .with_context(|| format!("inspect local destination {}", local.display()))?;
    if metadata.is_dir() {
        return Err(anyhow::anyhow!(
            "local destination is a directory: {}",
            local.display()
        ));
    }

    let Some(overwrite_policy) = overwrite_policy else {
        return Ok(true);
    };
    if overwrite_policy.replace_existing_file() {
        return Ok(true);
    }

    let (response, mut decision_rx) = tokio::sync::oneshot::channel();
    events
        .send(BackendEvent::SftpOverwriteConflict {
            request: SftpOverwriteRequest {
                tab_id: tab_id.to_string(),
                transfer_id: id.to_string(),
                remote_path: remote.to_string(),
                local_path: local.display().to_string(),
                response,
            },
        })
        .await
        .map_err(|_| anyhow::anyhow!("application stopped while waiting for overwrite choice"))?;

    let decision = loop {
        tokio::select! {
            response = &mut decision_rx => {
                break response.map_err(|_| anyhow::anyhow!("overwrite choice dialog was closed"))?;
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {
                if flag.0.load(Ordering::SeqCst) == 2 {
                    return Err(anyhow::anyhow!("transfer cancelled"));
                }
            }
        }
    };

    match decision {
        SftpOverwriteDecision::Skip => Ok(false),
        SftpOverwriteDecision::Replace => Ok(true),
        SftpOverwriteDecision::ReplaceAllInTransfer => {
            overwrite_policy.apply(decision);
            Ok(true)
        }
    }
}

pub(super) async fn upload_paths_impl(
    sftp: &SftpSession,
    locals: &[String],
    remote_dir: &str,
    flag: TransferStateFlag,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
) -> Result<String> {
    // Check for cancellation before starting
    let state = flag.0.load(Ordering::SeqCst);
    if state == 2 {
        return Err(anyhow::anyhow!("transfer cancelled"));
    }

    create_remote_dir_all(sftp, remote_dir).await?;
    let mut file_count = 0usize;
    let mut folder_count = 0usize;

    let mut total_bytes = 0u64;
    let mut files_to_upload = Vec::new();
    let mut dirs_to_create = Vec::new();

    for local in locals {
        let p = PathBuf::from(local);
        if p.is_dir() {
            folder_count += 1;
            let root_name = p.file_name().and_then(|n| n.to_str()).unwrap_or("folder");
            let remote_root = join_remote(remote_dir, root_name);
            dirs_to_create.push(remote_root.clone());

            for entry in WalkDir::new(&p) {
                let entry = entry?;
                let path = entry.path();
                if path == p {
                    continue;
                }

                if let Ok(meta) = tokio::fs::metadata(&path).await {
                    let relative = path.strip_prefix(&p)?;
                    let remote_path = if relative.as_os_str().is_empty() {
                        remote_root.clone()
                    } else {
                        let rel = relative
                            .components()
                            .map(|c| c.as_os_str().to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                            .join("/");
                        join_remote(&remote_root, &rel)
                    };

                    if path.is_dir() {
                        dirs_to_create.push(remote_path);
                    } else {
                        total_bytes += meta.len();
                        files_to_upload.push((path.to_path_buf(), remote_path));
                    }
                }
            }
        } else if let Ok(meta) = tokio::fs::metadata(&p).await {
            total_bytes += meta.len();
            let file_name = p.file_name().and_then(|n| n.to_str()).unwrap_or("file");
            files_to_upload.push((p.clone(), join_remote(remote_dir, file_name)));
            file_count += 1;
        }
    }

    // Check for cancellation before creating directories
    let state = flag.0.load(Ordering::SeqCst);
    if state == 2 {
        return Err(anyhow::anyhow!("transfer cancelled"));
    }

    // Create directories sequentially first
    for dir in dirs_to_create {
        // Check for cancellation between each directory creation
        let state = flag.0.load(Ordering::SeqCst);
        if state == 2 {
            return Err(anyhow::anyhow!("transfer cancelled"));
        }
        create_remote_dir_all(sftp, &dir).await?;
    }

    let transferred = Arc::new(AtomicU64::new(0));
    let mut futures = Vec::new();

    for (local_path, remote_path) in files_to_upload {
        let flag_clone = TransferStateFlag(Arc::clone(&flag.0));
        let events_clone = events.clone();
        let tab_id_clone = tab_id.to_string();
        let id_clone = id.to_string();
        let transferred_clone = Arc::clone(&transferred);

        futures.push(async move {
            upload_file_impl(
                sftp,
                &local_path,
                &remote_path,
                &flag_clone,
                &events_clone,
                &tab_id_clone,
                &id_clone,
                transferred_clone,
                Some(total_bytes),
            )
            .await
        });
    }

    use futures::StreamExt as _;
    let mut stream = futures::stream::iter(futures).buffer_unordered(4);
    while let Some(res) = stream.next().await {
        res?;
    }

    send_transfer_progress(
        events,
        tab_id,
        id,
        total_bytes,
        Some(total_bytes),
        TransferState::Completed,
    )
    .await;

    let summary = if file_count == 1 && folder_count == 0 {
        t!("uploaded_file").to_string()
    } else if file_count == 0 && folder_count == 1 {
        t!("uploaded_folder").to_string()
    } else if file_count > 0 && folder_count == 0 {
        t!("uploaded_n_files", files = file_count).to_string()
    } else if file_count == 0 && folder_count > 0 {
        t!("uploaded_n_folders", folders = folder_count).to_string()
    } else {
        t!(
            "uploaded_files_and_folders",
            files = file_count,
            folders = folder_count
        )
        .to_string()
    };
    Ok(summary)
}

pub(super) async fn upload_file_impl(
    sftp: &SftpSession,
    local_file: &Path,
    remote_path: &str,
    flag: &TransferStateFlag,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
    transferred: Arc<AtomicU64>,
    total: Option<u64>,
) -> Result<()> {
    let mut local = tokio::fs::File::open(local_file)
        .await
        .with_context(|| format!("open local {}", local_file.display()))?;
    let mut remote = sftp
        .create(remote_path)
        .await
        .with_context(|| format!("create remote {remote_path}"))?;

    let mut buffer = vec![0u8; 128 * 1024];
    loop {
        let cur = transferred.load(Ordering::Relaxed);
        flag.yield_if_paused(events, tab_id, id, cur, total).await?;
        let read = local.read(&mut buffer).await.context("read local file")?;
        if read == 0 {
            break;
        }
        remote
            .write_all(&buffer[..read])
            .await
            .with_context(|| format!("write remote {remote_path}"))?;

        let new_cur = transferred.fetch_add(read as u64, Ordering::Relaxed) + read as u64;
        send_transfer_progress(events, tab_id, id, new_cur, total, TransferState::Running).await;
    }
    remote.flush().await.context("flush remote file")?;
    Ok(())
}

async fn create_remote_dir_all(sftp: &SftpSession, remote_dir: &str) -> Result<()> {
    if remote_dir.is_empty() || remote_dir == "/" {
        return Ok(());
    }

    let mut current = String::from("/");
    for segment in remote_dir.split('/').filter(|segment| !segment.is_empty()) {
        current = join_remote(&current, segment);
        let _ = sftp.create_dir(&current).await;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{DownloadOverwritePolicy, SftpOverwriteDecision, local_path_for_remote_entry};

    #[test]
    fn local_download_path_rejects_unsafe_remote_entry_names() {
        let parent = Path::new("downloads");

        assert_eq!(
            local_path_for_remote_entry(parent, "report.txt").unwrap(),
            parent.join("report.txt")
        );
        for name in ["", ".", "..", "nested/report.txt", "nested\\report.txt"] {
            assert!(
                local_path_for_remote_entry(parent, name).is_err(),
                "{name:?}"
            );
        }
    }

    #[test]
    fn replace_all_in_transfer_only_changes_its_own_policy() {
        let mut first = DownloadOverwritePolicy::default();
        let second = DownloadOverwritePolicy::default();

        assert!(!first.replace_existing_file());
        assert!(!second.replace_existing_file());
        first.apply(SftpOverwriteDecision::ReplaceAllInTransfer);

        assert!(first.replace_existing_file());
        assert!(!second.replace_existing_file());
    }
}
