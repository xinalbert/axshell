[简体中文](getting-started.zh.md) · [Documentation](README.md)

# Getting Started

## Requirements

- Rust `1.88.0` or newer
- Cargo
- A macOS, Linux, or Windows desktop environment

## Run AxShell

```bash
cargo run --release
```

For restart-based development reload:

```bash
cargo dev-reload
```

Packaging prerequisites and platform commands are documented in [Development and Packaging](development.md).

## Main Workspace

- **Saved sessions:** open local terminals, organize SSH connections by group, and create connections.
- **Terminal workspace:** work with tabs, split panes, terminal search, and configurable keybindings.
- **SFTP page:** browse remote files and manage transfers from an SSH session.
- **Monitoring:** inspect local or remote CPU, memory, network, disk, and system information.
- **Settings:** configure appearance, terminal behavior, workspace rules, sync, proxy, and X11.

## Next Steps

- Connect to a host: [Terminal And SSH](features/terminal-ssh.md)
- Learn workspace controls: [Workspace](features/workspace.md)
- Transfer remote files: [SFTP](features/sftp.md)
- Find config and logs: [Local Data And Troubleshooting](features/local-data-troubleshooting.md)

<!-- Screenshot target: images/features/getting-started-workspace.png -->
