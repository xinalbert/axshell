[简体中文](monitoring-lifecycle.zh.md) · [Documentation](../README.md)

# Monitoring And Lifecycle

## Monitoring

The monitoring page can display local or active-SSH system information, including CPU, memory, swap, network, disk, and platform details. Settings can show or hide monitoring and place it in the sidebar or bottom area.

## Window Background State

When the window loses focus:

- monitoring refresh, theme polling, and cursor blinking stop immediately;
- terminal processes, SSH commands, and SFTP transfers continue running.

## Deep Sleep

The configurable delay is Off, 1, 5, 15, or 30 minutes; the default is 5 minutes.

After the delay, AxShell keeps only low-frequency backend event handling. Deep sleep does not disconnect local terminals or SSH sessions. Refocusing the window restores rendering, monitoring, theme updates, and the active page immediately.

For implementation boundaries and lifecycle rationale, see [Resource Lifecycle](../resource-lifecycle.md).

<!-- Screenshot target: ../images/features/monitoring-dashboard.png -->
<!-- Screenshot target: ../images/features/monitoring-lifecycle-settings.png -->
