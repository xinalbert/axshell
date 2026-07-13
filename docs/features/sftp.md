[简体中文](sftp.zh.md) · [Documentation](../README.md)

# SFTP

## Open The SFTP Page

Open SFTP from an active SSH session. The page combines a remote browser, a local browser, and transfer state for that session.

## Local Directory Memory

For a saved SSH session, the local browser reopens its own last successfully opened directory. New or unsaved connections start in the user home directory and do not save a local-directory entry. If a remembered directory is deleted or unreadable, AxShell opens the user home directory without replacing the remembered value.

These local paths stay on the current computer and are not included in WebDAV or S3 session sync.

## File Operations

- Browse remote directories and show or hide hidden files.
- Sort loaded entries and navigate by path.
- Upload files or folders.
- Download files or folders; directory downloads use a temporary archive when appropriate.
- Create folders and recursively delete selected paths.
- Open a remote file in the system editor and upload changes after save.
- Preview supported files and bounded directory contents.

## Large Directories

Remote listings load on demand instead of reading the entire directory into the UI.

- Each page displays up to 250 additional entries.
- Use **Load More** while more entries are available.
- A listing keeps at most 2,000 entries or 2 MiB of retained name/path data.
- When the safety budget is reached, AxShell stops loading and shows a truncation state instead of presenting the result as end-of-directory.

## Transfers

Transfer tasks support pause, resume, and cancel. Completed, failed, interrupted, and active tasks are shown in transfer history, which keeps up to 100 records.

Closing an SFTP page with active work uses the configured confirmation flow so ongoing transfers are not discarded silently.

<!-- Screenshot target: ../images/features/sftp-browser.png -->
<!-- Screenshot target: ../images/features/sftp-transfer-panel.png -->
