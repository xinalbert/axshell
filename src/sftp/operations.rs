use anyhow::{Context, Result};
use russh_sftp::client::SftpSession;

use std::future::Future;
use std::pin::Pin;

pub(super) fn recursive_delete<'a>(
    sftp: &'a SftpSession,
    path: String,
) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
    Box::pin(async move {
        match sftp.read_dir(&path).await {
            Ok(entries) => {
                for entry in entries {
                    let name = entry.file_name();
                    if name == "." || name == ".." {
                        continue;
                    }
                    let child_path = crate::sftp::join_remote(&path, &name);

                    let meta = entry.metadata();
                    let permissions = meta.permissions.unwrap_or(0);
                    let is_dir = (permissions & 0o170_000) == 0o040_000;

                    if is_dir {
                        recursive_delete(sftp, child_path).await?;
                    } else {
                        sftp.remove_file(&child_path)
                            .await
                            .with_context(|| format!("Failed to delete file {child_path}"))?;
                    }
                }
                sftp.remove_dir(&path)
                    .await
                    .with_context(|| format!("Failed to delete dir {path}"))?;
            }
            Err(_) => {
                sftp.remove_file(&path)
                    .await
                    .with_context(|| format!("Failed to delete {path}"))?;
            }
        }
        Ok(())
    })
}
