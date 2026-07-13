[简体中文](README.zh.md)

# AxShell

![AxShell workspace preview](preview.png)

AxShell is a Rust and GPUI desktop terminal workspace for local shells, SSH sessions, SFTP file management, and repeatable remote operations.

Forked from <https://github.com/rust-kotlin/ashell.git>. The current project is maintained at <https://github.com/xinalbert/axshell>.

## Highlights

- Local terminals and saved SSH sessions with password or private-key authentication
- Multi-tab and multi-pane workspaces with configurable keybindings and search
- Built-in SFTP browsing, transfer control, remote editing, and large-directory pagination
- Themes, fonts, tab color behavior, monitoring, and workspace preferences
- Encrypted session sync over WebDAV or S3-compatible storage
- Global and per-session proxy support plus SSH X11 forwarding

## Quick Start

AxShell requires Rust `1.88.0` or newer.

```bash
cargo run --release
```

For restart-based development reload:

```bash
cargo dev-reload
```

## Documentation

- [Documentation index](docs/README.md)
- [Getting started](docs/getting-started.md)
- [Feature guides](docs/README.md#feature-guides)
- [Bundled fonts](docs/features/bundled-fonts.md)
- [Development and packaging](docs/development.md)

## Project Notes

- Release tags use `vYYYY.M.D` and `vYYYY.M.D-N`.
- Release automation builds Windows x86_64, Linux x86_64/aarch64, and macOS architecture-specific and universal packages.
- Existing `ax_ashell` configuration is copied into the current `ax_shell` configuration directory when migration is needed; the old directory is left untouched.

## Contributing And Support

Use [GitHub Issues](https://github.com/xinalbert/axshell/issues) for bugs and feature requests. See [Development and Packaging](docs/development.md) before preparing code or release changes.

## License

Licensed under [GPL-3.0-or-later](LICENSE).
