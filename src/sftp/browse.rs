use std::collections::VecDeque;

use anyhow::{Context, Result, anyhow};
use russh_sftp::{
    client::{RawSftpSession, SftpSession, error::Error as SftpClientError},
    protocol::StatusCode,
};
use rust_i18n::t;

use crate::terminal::{BackendEvent, BackendEventSender};

use super::{
    auth::SftpClientHandler,
    model::RemoteEntry,
    path::{format_bytes, join_remote, parent_dir},
    session::{
        SFTP_BROWSE_TIMEOUT, SFTP_SHUTDOWN_TIMEOUT, open_browse_sftp_session, open_sftp_session,
    },
};

pub(super) const SFTP_BROWSE_ENTRY_LIMIT: usize = 2_000;
pub(super) const SFTP_BROWSE_NAME_BYTES_LIMIT: usize = 2 * 1024 * 1024;
const SFTP_BROWSE_PAGE_ENTRY_LIMIT: usize = 250;

pub(super) struct DirectoryListing {
    pub(super) entries: Vec<RemoteEntry>,
    pub(super) truncated: bool,
}

pub(super) struct DirectoryPage {
    pub(super) entries: Vec<RemoteEntry>,
    pub(super) has_more: bool,
    pub(super) reached_limit: bool,
}

pub(super) struct BrowseCursor {
    sftp: Option<RawSftpSession>,
    directory_handle: Option<String>,
    path: String,
    pending_entries: VecDeque<RemoteEntry>,
    retained_entries: usize,
    retained_name_bytes: usize,
    reached_limit: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RevealPathKind {
    Directory,
    File,
    Missing,
}

pub(super) async fn emit_browser_page(
    events: &BackendEventSender,
    tab_id: &str,
    path: &str,
    page: DirectoryPage,
    append: bool,
) -> Result<()> {
    let _ = events
        .send(BackendEvent::SftpEntries {
            tab_id: tab_id.to_string(),
            path: path.to_string(),
            entries: page.entries,
            append,
            has_more: page.has_more,
            reached_limit: page.reached_limit,
        })
        .await;
    let status = if page.reached_limit {
        t!(
            "sftp_directory_truncated",
            entries = SFTP_BROWSE_ENTRY_LIMIT,
            bytes = format_bytes(SFTP_BROWSE_NAME_BYTES_LIMIT as u64)
        )
        .to_string()
    } else {
        path.to_string()
    };
    let _ = events
        .send(BackendEvent::SftpStatus {
            tab_id: tab_id.to_string(),
            text: status,
        })
        .await;
    Ok(())
}

pub(super) async fn open_and_emit_browser_page(
    events: &BackendEventSender,
    tab_id: &str,
    handle: &russh::client::Handle<SftpClientHandler>,
    path: &str,
    cursor: &mut Option<BrowseCursor>,
) -> Result<()> {
    close_browse_cursor(cursor).await;
    let new_cursor = tokio::time::timeout(SFTP_BROWSE_TIMEOUT, open_browse_cursor(handle, path))
        .await
        .map_err(|_| {
            anyhow!(
                "list directory timed out after {}s: {path}",
                SFTP_BROWSE_TIMEOUT.as_secs()
            )
        })??;
    *cursor = Some(new_cursor);

    if let Err(err) = emit_next_browser_page(events, tab_id, cursor, false).await {
        close_browse_cursor(cursor).await;
        return Err(err);
    }
    Ok(())
}

pub(super) async fn emit_next_browser_page(
    events: &BackendEventSender,
    tab_id: &str,
    cursor: &mut Option<BrowseCursor>,
    append: bool,
) -> Result<()> {
    let Some(cursor) = cursor.as_mut() else {
        return Err(anyhow!("directory cursor is closed"));
    };
    let path = cursor.path.clone();
    let page = tokio::time::timeout(SFTP_BROWSE_TIMEOUT, read_next_browser_page(cursor))
        .await
        .map_err(|_| {
            anyhow!(
                "list directory timed out after {}s: {path}",
                SFTP_BROWSE_TIMEOUT.as_secs()
            )
        })??;
    emit_browser_page(events, tab_id, &path, page, append).await
}

pub(super) async fn read_browser_listing_with_timeout(
    handle: &russh::client::Handle<SftpClientHandler>,
    path: &str,
    entry_limit: usize,
    name_bytes_limit: usize,
) -> Result<DirectoryListing> {
    tokio::time::timeout(SFTP_BROWSE_TIMEOUT, async {
        let sftp = open_browse_sftp_session(handle).await?;
        list_dir_for_browser(&sftp, path, entry_limit, name_bytes_limit).await
    })
    .await
    .map_err(|_| {
        anyhow!(
            "list directory timed out after {}s: {path}",
            SFTP_BROWSE_TIMEOUT.as_secs()
        )
    })?
}

async fn open_browse_cursor(
    handle: &russh::client::Handle<SftpClientHandler>,
    path: &str,
) -> Result<BrowseCursor> {
    let sftp = open_browse_sftp_session(handle).await?;
    let directory_handle = sftp
        .opendir(path)
        .await
        .with_context(|| format!("opendir {path}"))?;
    Ok(BrowseCursor {
        sftp: Some(sftp),
        directory_handle: Some(directory_handle.handle),
        path: path.to_string(),
        pending_entries: VecDeque::new(),
        retained_entries: 0,
        retained_name_bytes: 0,
        reached_limit: false,
    })
}

async fn read_next_browser_page(cursor: &mut BrowseCursor) -> Result<DirectoryPage> {
    let mut entries = Vec::with_capacity(SFTP_BROWSE_PAGE_ENTRY_LIMIT);

    while entries.len() < SFTP_BROWSE_PAGE_ENTRY_LIMIT {
        while entries.len() < SFTP_BROWSE_PAGE_ENTRY_LIMIT {
            let Some(entry) = cursor.pending_entries.pop_front() else {
                break;
            };
            entries.push(entry);
        }
        if entries.len() == SFTP_BROWSE_PAGE_ENTRY_LIMIT
            || cursor.reached_limit
            || cursor.directory_handle.is_none()
        {
            break;
        }

        let Some(sftp) = cursor.sftp.as_ref() else {
            break;
        };
        let Some(directory_handle) = cursor.directory_handle.clone() else {
            break;
        };
        let batch = match sftp.readdir(directory_handle).await {
            Ok(batch) => batch,
            Err(SftpClientError::Status(status)) if status.status_code == StatusCode::Eof => {
                close_open_browse_cursor(cursor).await;
                break;
            }
            Err(err) => return Err(err).with_context(|| format!("readdir {} failed", cursor.path)),
        };

        for entry in batch.files {
            if entry.filename == "." || entry.filename == ".." {
                continue;
            }
            if !browser_entry_fits_budget(
                cursor.retained_entries,
                cursor.retained_name_bytes,
                &cursor.path,
                &entry.filename,
                SFTP_BROWSE_ENTRY_LIMIT,
                SFTP_BROWSE_NAME_BYTES_LIMIT,
            ) {
                cursor.reached_limit = true;
                close_open_browse_cursor(cursor).await;
                break;
            }

            cursor.retained_entries += 1;
            cursor.retained_name_bytes += cursor.path.len() + entry.filename.len() + 1;
            let permissions = entry.attrs.permissions.unwrap_or(0);
            cursor.pending_entries.push_back(RemoteEntry {
                full_path: join_remote(&cursor.path, &entry.filename),
                is_dir: (permissions & 0o170_000) == 0o040_000,
                size: entry.attrs.size.unwrap_or(0),
                modified: entry.attrs.mtime.unwrap_or(0),
                name: entry.filename,
            });
        }
    }

    let (has_more, reached_limit) = browser_page_state(
        cursor.pending_entries.len(),
        cursor.directory_handle.is_some(),
        cursor.reached_limit,
    );
    Ok(DirectoryPage {
        entries,
        has_more,
        reached_limit,
    })
}

fn browser_page_state(
    pending_entries: usize,
    directory_open: bool,
    reached_limit: bool,
) -> (bool, bool) {
    (pending_entries > 0 || directory_open, reached_limit)
}

pub(super) async fn close_browse_cursor(cursor: &mut Option<BrowseCursor>) {
    if let Some(mut cursor) = cursor.take() {
        close_open_browse_cursor(&mut cursor).await;
    }
}

async fn close_open_browse_cursor(cursor: &mut BrowseCursor) {
    if let Some(sftp) = cursor.sftp.as_ref() {
        if let Some(directory_handle) = cursor.directory_handle.take() {
            let _ = tokio::time::timeout(SFTP_SHUTDOWN_TIMEOUT, sftp.close(directory_handle)).await;
        }
        let _ = sftp.close_session();
    }
    cursor.sftp = None;
}

pub(super) async fn reveal_path_target(
    handle: &russh::client::Handle<SftpClientHandler>,
    path: &str,
) -> Result<String> {
    let sftp = open_sftp_session(handle).await?;
    match sftp.metadata(path).await {
        Ok(metadata) => {
            let is_dir = metadata
                .permissions
                .map(|mode| (mode & 0o170_000) == 0o040_000)
                .unwrap_or(false);
            Ok(reveal_target_directory(
                path,
                if is_dir {
                    RevealPathKind::Directory
                } else {
                    RevealPathKind::File
                },
            ))
        }
        Err(_) => Ok(reveal_target_directory(path, RevealPathKind::Missing)),
    }
}

fn reveal_target_directory(path: &str, kind: RevealPathKind) -> String {
    match kind {
        RevealPathKind::Directory => path.to_string(),
        RevealPathKind::File | RevealPathKind::Missing => {
            parent_dir(path).unwrap_or_else(|| "/".to_string())
        }
    }
}

pub(super) async fn list_dir_impl(sftp: &SftpSession, path: &str) -> Result<Vec<RemoteEntry>> {
    let raw = sftp
        .read_dir(path)
        .await
        .with_context(|| format!("read_dir {path} failed"))?;

    let mut entries = raw
        .into_iter()
        .filter(|entry| {
            let name = entry.file_name();
            name != "." && name != ".."
        })
        .map(|entry| {
            let name = entry.file_name().to_string();
            let full_path = join_remote(path, &name);
            let meta = entry.metadata();
            let permissions = meta.permissions.unwrap_or(0);
            let is_dir = (permissions & 0o170_000) == 0o040_000;
            let size = meta.size.unwrap_or(0);
            let modified = meta.mtime.unwrap_or(0);
            RemoteEntry {
                name,
                full_path,
                is_dir,
                size,
                modified,
            }
        })
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}

async fn list_dir_for_browser(
    sftp: &RawSftpSession,
    path: &str,
    entry_limit: usize,
    name_bytes_limit: usize,
) -> Result<DirectoryListing> {
    let handle = sftp
        .opendir(path)
        .await
        .with_context(|| format!("opendir {path}"))?;
    let mut entries = Vec::new();
    let mut name_bytes = 0usize;
    let mut truncated = false;

    loop {
        let batch = match sftp.readdir(handle.handle.clone()).await {
            Ok(batch) => batch,
            Err(SftpClientError::Status(status)) if status.status_code == StatusCode::Eof => {
                break;
            }
            Err(err) => {
                let _ = sftp.close(handle.handle.clone()).await;
                let _ = sftp.close_session();
                return Err(err).with_context(|| format!("readdir {path} failed"));
            }
        };

        for entry in batch.files {
            if entry.filename == "." || entry.filename == ".." {
                continue;
            }
            if !browser_entry_fits_budget(
                entries.len(),
                name_bytes,
                path,
                &entry.filename,
                entry_limit,
                name_bytes_limit,
            ) {
                truncated = true;
                break;
            }

            name_bytes += path.len() + entry.filename.len() + 1;
            let permissions = entry.attrs.permissions.unwrap_or(0);
            entries.push(RemoteEntry {
                full_path: join_remote(path, &entry.filename),
                is_dir: (permissions & 0o170_000) == 0o040_000,
                size: entry.attrs.size.unwrap_or(0),
                modified: entry.attrs.mtime.unwrap_or(0),
                name: entry.filename,
            });
        }
        if truncated {
            break;
        }
    }

    let _ = sftp.close(handle.handle).await;
    let _ = sftp.close_session();
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(DirectoryListing { entries, truncated })
}

fn browser_entry_fits_budget(
    entry_count: usize,
    name_bytes: usize,
    path: &str,
    name: &str,
    entry_limit: usize,
    name_bytes_limit: usize,
) -> bool {
    entry_count < entry_limit
        && name_bytes.saturating_add(path.len() + name.len() + 1) <= name_bytes_limit
}

#[cfg(test)]
mod tests {
    use super::{
        RevealPathKind, SFTP_BROWSE_ENTRY_LIMIT, SFTP_BROWSE_NAME_BYTES_LIMIT,
        browser_entry_fits_budget, browser_page_state, reveal_target_directory,
    };

    #[test]
    fn browser_listing_stops_at_the_entry_limit() {
        assert!(browser_entry_fits_budget(
            SFTP_BROWSE_ENTRY_LIMIT - 1,
            0,
            "/remote",
            "entry",
            SFTP_BROWSE_ENTRY_LIMIT,
            SFTP_BROWSE_NAME_BYTES_LIMIT,
        ));
        assert!(!browser_entry_fits_budget(
            SFTP_BROWSE_ENTRY_LIMIT,
            0,
            "/remote",
            "entry",
            SFTP_BROWSE_ENTRY_LIMIT,
            SFTP_BROWSE_NAME_BYTES_LIMIT,
        ));
    }

    #[test]
    fn browser_listing_stops_at_the_name_byte_limit() {
        let path = "/";
        let name = "entry";
        let exact_bytes = SFTP_BROWSE_NAME_BYTES_LIMIT - path.len() - name.len() - 1;

        assert!(browser_entry_fits_budget(
            0,
            exact_bytes,
            path,
            name,
            SFTP_BROWSE_ENTRY_LIMIT,
            SFTP_BROWSE_NAME_BYTES_LIMIT,
        ));
        assert!(!browser_entry_fits_budget(
            0,
            exact_bytes + 1,
            path,
            name,
            SFTP_BROWSE_ENTRY_LIMIT,
            SFTP_BROWSE_NAME_BYTES_LIMIT,
        ));
    }

    #[test]
    fn browser_page_state_preserves_pending_entries_after_the_cursor_closes() {
        assert_eq!(browser_page_state(3, false, true), (true, true));
        assert_eq!(browser_page_state(0, false, true), (false, true));
        assert_eq!(browser_page_state(0, true, false), (true, false));
        assert_eq!(browser_page_state(0, false, false), (false, false));
    }

    #[test]
    fn reveal_target_directory_enters_existing_directory() {
        assert_eq!(
            reveal_target_directory("/srv/app/logs", RevealPathKind::Directory),
            "/srv/app/logs"
        );
    }

    #[test]
    fn reveal_target_directory_falls_back_for_file_or_missing_target() {
        assert_eq!(
            reveal_target_directory("/srv/app/logs/output.log", RevealPathKind::File),
            "/srv/app/logs"
        );
        assert_eq!(
            reveal_target_directory("/srv/app/missing/output.log", RevealPathKind::Missing),
            "/srv/app/missing"
        );
    }
}
