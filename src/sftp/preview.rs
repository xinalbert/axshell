use anyhow::{Context, Result};
use russh_sftp::client::SftpSession;
use rust_i18n::t;
use tokio::io::AsyncReadExt;

use super::{
    auth::SftpClientHandler,
    browse::{DirectoryListing, read_browser_listing_with_timeout},
    model::PreviewData,
    path::{base_name, format_bytes},
};

const SFTP_PREVIEW_BYTE_LIMIT: usize = 128 * 1024;
const SFTP_DIRECTORY_PREVIEW_ENTRY_LIMIT: usize = 200;
const SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT: usize = SFTP_PREVIEW_BYTE_LIMIT - 512;

pub(super) async fn preview_impl(
    sftp: &SftpSession,
    handle: &russh::client::Handle<SftpClientHandler>,
    path: &str,
) -> Result<PreviewData> {
    let metadata = sftp
        .metadata(path)
        .await
        .with_context(|| format!("metadata {path}"))?;
    let is_dir = metadata
        .permissions
        .map(|mode| (mode & 0o170_000) == 0o040_000)
        .unwrap_or(false);

    if is_dir {
        let listing = read_browser_listing_with_timeout(
            handle,
            path,
            SFTP_DIRECTORY_PREVIEW_ENTRY_LIMIT,
            SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT,
        )
        .await?;
        let body = directory_preview_body(path, listing);
        return Ok(PreviewData {
            path: path.to_string(),
            title: base_name(path),
            body,
            is_binary: false,
        });
    }

    let mut remote_file = sftp
        .open(path)
        .await
        .with_context(|| format!("open remote {path}"))?;
    let mut buffer = vec![0u8; SFTP_PREVIEW_BYTE_LIMIT];
    let read = remote_file
        .read(&mut buffer)
        .await
        .context("read preview bytes")?;
    buffer.truncate(read);

    let nul_ratio = if buffer.is_empty() {
        0.0
    } else {
        buffer.iter().filter(|byte| **byte == 0).count() as f32 / buffer.len() as f32
    };
    let is_binary = nul_ratio > 0.01;
    let body = if is_binary {
        format!(
            "Binary file\npath: {path}\nsize: {}\npreview: unavailable in-app",
            format_bytes(metadata.size.unwrap_or(0)),
        )
    } else {
        String::from_utf8_lossy(&buffer).into_owned()
    };

    Ok(PreviewData {
        path: path.to_string(),
        title: base_name(path),
        body,
        is_binary,
    })
}

fn directory_preview_body(path: &str, listing: DirectoryListing) -> String {
    let mut body = String::with_capacity(SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT);
    let mut content_truncated = append_preview_text(
        &mut body,
        "Directory: ",
        SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT,
    );
    content_truncated |=
        append_preview_text(&mut body, path, SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT);
    content_truncated |=
        append_preview_text(&mut body, "\n\n", SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT);

    for entry in listing.entries {
        let kind = if entry.is_dir { "dir " } else { "file" };
        let line_len = kind.len() + 2 + entry.name.len() + 1;
        if body.len().saturating_add(line_len) > SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT {
            content_truncated = true;
            break;
        }
        body.push_str(kind);
        body.push_str("  ");
        body.push_str(&entry.name);
        body.push('\n');
    }

    if listing.truncated || content_truncated {
        let notice = t!("sftp_directory_preview_truncated").to_string();
        append_preview_text(&mut body, "\n", SFTP_PREVIEW_BYTE_LIMIT);
        append_preview_text(&mut body, &notice, SFTP_PREVIEW_BYTE_LIMIT);
    }
    body
}

fn append_preview_text(body: &mut String, text: &str, byte_limit: usize) -> bool {
    let remaining = byte_limit.saturating_sub(body.len());
    if text.len() <= remaining {
        body.push_str(text);
        return false;
    }

    let suffix = if remaining >= 3 { "..." } else { "" };
    let prefix_len = remaining.saturating_sub(suffix.len());
    let prefix_end = text
        .char_indices()
        .map(|(index, character)| index + character.len_utf8())
        .take_while(|end| *end <= prefix_len)
        .last()
        .unwrap_or(0);
    body.push_str(&text[..prefix_end]);
    body.push_str(suffix);
    true
}

#[cfg(test)]
mod tests {
    use super::{DirectoryListing, SFTP_PREVIEW_BYTE_LIMIT, directory_preview_body};
    use crate::sftp::RemoteEntry;

    #[test]
    fn directory_preview_has_a_fixed_byte_budget_and_marks_truncation() {
        let listing = DirectoryListing {
            entries: vec![RemoteEntry {
                name: "x".repeat(SFTP_PREVIEW_BYTE_LIMIT),
                full_path: "/remote/x".to_string(),
                is_dir: false,
                size: 0,
                modified: 0,
            }],
            truncated: true,
        };

        let body = directory_preview_body("/remote", listing);

        assert!(body.len() <= SFTP_PREVIEW_BYTE_LIMIT);
        assert!(body.starts_with("Directory: /remote"));
    }

    #[test]
    fn directory_preview_truncates_a_long_utf8_path_without_exceeding_the_limit() {
        let path = "路径".repeat(SFTP_PREVIEW_BYTE_LIMIT);
        let body = directory_preview_body(
            &path,
            DirectoryListing {
                entries: Vec::new(),
                truncated: false,
            },
        );

        assert!(body.len() <= SFTP_PREVIEW_BYTE_LIMIT);
        assert!(body.starts_with("Directory: "));
        assert!(body.contains("..."));
    }
}
