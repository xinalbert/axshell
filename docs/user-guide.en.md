[中文](user-guide.md)

# AxAshell User Guide

## What It Covers

AxAshell is built for these workflows:

- Managing local terminals and multiple SSH sessions in one desktop workspace
- Browsing remote directories, transferring files, and editing remote files from the same SSH session
- Keeping connection presets, appearance settings, and keybindings in a local config file
- Syncing saved SSH sessions between devices through WebDAV or S3

## Workspace Layout

The main window is organized around a terminal workspace with these areas:

- Session area: open a local terminal, select a saved SSH session, or create a new one
- Terminal area: local or remote terminals with multi-tab and multi-pane layout
- Remote files area: SFTP panel for SSH sessions
- Transfers: upload and download history with task state
- Settings: appearance, fonts, keybindings, sync, proxy, and X11

## Local Terminal and SSH

### Open a Local Terminal

Launch the app and open a local terminal tab from the session area.

### Create an SSH Session

1. Open `New SSH` or the session selector.
2. Enter host, port, and username.
3. Choose password or private key authentication.
4. Fill in per-session proxy settings when needed.
5. Use `Save` or `Save & Connect`.

Current SSH session support includes:

- Password authentication
- Private key path or inline private key content
- Optional passphrase
- Per-session proxy settings
- Last-used timestamp persistence

Saved sessions are written to:

```text
~/.config/ax_ashell/sessions.json
```

## SFTP Workflows

The SFTP panel is available inside SSH sessions. Current actions include:

- Browse remote directories
- Show or hide hidden files
- Upload files or folders
- Download files or folders
- Create folders
- Delete selected paths
- Open a remote file in the system editor and auto-upload it after save

Transfers are tracked in the transfers panel and support:

- Pause
- Resume
- Cancel
- Completed, failed, or interrupted state review

Transfer history keeps up to 100 records.

## Multi-Pane and Keybindings

AxAshell exposes workspace actions as configurable keybindings. The default main modifier is:

- macOS: `Cmd`
- Linux / Windows: `Ctrl`

Default bindings include:

- Open settings: `Cmd/Ctrl + ,`
- Open session selector: `Cmd/Ctrl + O`
- New SSH: `Cmd/Ctrl + N`
- Open transfers: `Cmd/Ctrl + T`
- Search: `Cmd/Ctrl + F`
- Toggle sidebar: `Cmd/Ctrl + S`
- Focus adjacent pane: `Cmd/Ctrl + H/J/K/L`
- Split current pane: `Cmd/Ctrl + Shift + H/J/K/L`
- Close current pane: `Cmd/Ctrl + W`
- Copy / Paste:
  - macOS: `Cmd + C` / `Cmd + V`
  - Linux / Windows: `Ctrl + Shift + C` / `Ctrl + Shift + V`

All workspace bindings can be changed in the `Key Bindings` settings page.

## Settings

### Appearance and Fonts

The settings page currently supports:

- Follow-system or manual light/dark mode
- Separate light and dark theme selection
- UI and terminal font family selection
- UI and terminal font size controls
- Cursor style selection
- Title bar style selection
- Custom theme color, background, font brightness, and custom theme name

### Monitoring and Layout

Available controls include:

- Show or hide the monitoring dashboard
- Place monitoring at the bottom or in the sidebar
- Lock workspace layout
- Right-click copy and paste
- Terminal keyword highlighting

### Configuration Sync

Configuration sync supports:

- WebDAV
- S3-compatible object storage

Behavior and boundaries:

- Payloads are encrypted locally before upload
- Download replaces the locally saved SSH session list
- WebDAV passwords, S3 credentials, and the encryption password stay in process memory only
- The local config stores connection parameters such as WebDAV endpoint, username, S3 endpoint, bucket, and object key

The default object name is:

```text
ax_ashell-sync.json
```

### Proxy

Proxy settings support:

- Enable or disable the global proxy layer
- Read proxy values from environment variables at startup
- Choose `socks5` or `http`
- Configure host, port, username, and password

When environment proxy loading is enabled, the app prefers `ALL_PROXY`, `HTTPS_PROXY`, `HTTP_PROXY`, and their lowercase variants.

### X11 Forwarding

X11 forwarding is intended for GUI apps launched through SSH. The current platform expectations are:

- macOS: XQuartz
- Windows: VcXsrv or Xming
- Linux / Wayland: local `DISPLAY` or Xwayland

Before using it, make sure:

- A local X server is available
- The remote `sshd` allows `X11Forwarding yes`
- The remote GUI app supports X11

## Logs and Local Files

Runtime logs are written to:

```text
~/.config/ax_ashell/log
```

SFTP remote editing and temporary archive flows use temp files under the app config directory.

## Related Docs

- [Development and Packaging](development.en.md)
