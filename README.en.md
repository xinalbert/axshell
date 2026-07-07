[中文](README.md)

# AxAshell

![Preview](preview.png)

AxAshell is a desktop terminal workspace built with Rust, GPUI, and `alacritty_terminal`, focused on local shells, SSH sessions, and built-in SFTP file management.

Forked from <https://github.com/rust-kotlin/ashell.git>

Repository: <https://github.com/xinalbert/ax_ashell>

## Core Features

- Local terminal tabs and SSH session management with password and private key authentication
- Built-in SFTP panel for browsing, upload, download, delete, transfer control, and remote file editing
- Multi-tab, multi-pane workspace with search, pane split, focus switching, and pane close actions
- In-app settings for themes, fonts, keybindings, monitoring panels, and title bar style
- Session configuration sync over WebDAV or S3 with end-to-end encrypted uploads
- Global proxy and SSH X11 forwarding settings for remote operations and GUI forwarding workflows

## Quick Start

Run the desktop app:

```bash
cargo run --release
```

Use restart-based live development:

```bash
cargo dev-reload
```

See [Development and Packaging](docs/development.en.md) for the full run and packaging workflow.

## Documentation

- [User Guide](docs/user-guide.en.md): UI layout, SSH / SFTP workflows, sync, proxy, and X11 usage
- [Development and Packaging](docs/development.en.md): development commands, `cargo dev-reload`, macOS `.app`, and Debian `.deb` packaging

## Project Status

- Runtime and packaging icons come from `assets/icons/terminal_icon_all_formats`
- GitHub Actions still builds the app and uploads artifacts
- Token-backed publishing paths such as GitHub Releases and Homebrew cask updates are currently disabled

## License

Licensed under [GPL-3.0-or-later](LICENSE).
