[简体中文](README.zh.md) · [Project README](../README.md)

# AxShell Documentation

This directory contains task-focused user guides, development notes, and design references. Start with the short setup guide, then open only the feature page relevant to your workflow.

## Start Here

- [Getting Started](getting-started.md): requirements, launch commands, and the main workspace areas
- [User Guide Index](user-guide.md): compatibility entry point for the former single-page guide
- [Development and Packaging](development.md): local development, packages, and release automation

## Feature Guides

| Feature | Guide |
| --- | --- |
| Local terminal and SSH sessions | [Terminal And SSH](features/terminal-ssh.md) |
| Tabs, panes, search, and keybindings | [Workspace](features/workspace.md) |
| Remote files and transfers | [SFTP](features/sftp.md) |
| Themes, fonts, tab colors, and settings behavior | [Appearance And Settings](features/appearance-settings.md) |
| Built-in font families and upstream repositories | [Bundled Fonts](features/bundled-fonts.md) |
| Encrypted WebDAV/S3 session sync | [Configuration Sync](features/configuration-sync.md) |
| SOCKS5/HTTP proxy and X11 forwarding | [Proxy And X11](features/proxy-x11.md) |
| Monitoring, background state, and deep sleep | [Monitoring And Lifecycle](features/monitoring-lifecycle.md) |
| Config files, logs, migration, and problem reports | [Local Data And Troubleshooting](features/local-data-troubleshooting.md) |

## Design And Maintenance

- [Resource Lifecycle](resource-lifecycle.md): background resource policy and deep-sleep design
- [Screenshot Guide](images/README.md): filenames and placement for future documentation images
- [Project Implementation Tracker](project-implementation-tracker/current.md): current repository implementation record

## Documentation Convention

- Root README files, development references, and feature guides use English `name.md` with Chinese `name.zh.md` counterparts.
- Keep language pairs structurally aligned and use relative links.
- Put feature screenshots under `docs/images/features/`; each feature guide contains commented insertion targets.
