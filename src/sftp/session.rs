use anyhow::{Context, Result};
use russh_sftp::client::{RawSftpSession, SftpSession};

use super::auth::SftpClientHandler;

pub(super) const SFTP_BROWSE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
pub(super) const SFTP_SHUTDOWN_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(2);

pub(super) async fn open_sftp_session(
    handle: &russh::client::Handle<SftpClientHandler>,
) -> Result<SftpSession> {
    let channel = handle
        .channel_open_session()
        .await
        .context("open sftp channel")?;
    channel
        .request_subsystem(true, "sftp")
        .await
        .context("request sftp subsystem")?;
    SftpSession::new(channel.into_stream())
        .await
        .context("sftp handshake")
}

pub(super) async fn open_browse_sftp_session(
    handle: &russh::client::Handle<SftpClientHandler>,
) -> Result<RawSftpSession> {
    let channel = handle
        .channel_open_session()
        .await
        .context("open browse sftp channel")?;
    channel
        .request_subsystem(true, "sftp")
        .await
        .context("request browse sftp subsystem")?;
    let session = RawSftpSession::new(channel.into_stream());
    session.set_timeout(SFTP_BROWSE_TIMEOUT.as_secs());
    session.init().await.context("browse sftp handshake")?;
    Ok(session)
}

pub(super) async fn open_transfer_sftp_session(
    handle: &russh::client::Handle<SftpClientHandler>,
) -> Result<SftpSession> {
    open_sftp_session(handle)
        .await
        .context("open transfer sftp session")
}
