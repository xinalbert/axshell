[简体中文](local-data-troubleshooting.zh.md) · [Documentation](../README.md)

# Local Data And Troubleshooting

## Configuration

Saved sessions and application preferences are stored under the platform configuration directory. On typical Unix-like systems, the main session file is:

```text
~/.config/ax_shell/sessions.json
```

Custom themes are stored under the adjacent `themes/` directory.

When the current configuration is absent, AxShell can copy data from the former `ax_ashell` directory. Migration does not delete the old files.

## Logs And Crash Reports

Runtime logs are written under:

```text
~/.config/ax_shell/log
```

Crash reports use:

```text
~/.config/ax_shell/crash/ax_shell-crash-*.log
```

Platform configuration roots can differ, so use the log-directory action in Settings when available.

## Reporting A Problem

1. Reproduce the issue with the smallest affected workflow.
2. Record the platform, AxShell version, and whether the session is local or SSH.
3. Attach the crash report when one exists.
4. Include the latest relevant runtime log after removing hosts, usernames, paths, and credentials.
5. Open an issue at <https://github.com/xinalbert/axshell/issues>.

For build failures, also include the command and compiler output. Development logging details are in [Development and Packaging](../development.md).

<!-- Screenshot target: ../images/features/local-data-log-settings.png -->
