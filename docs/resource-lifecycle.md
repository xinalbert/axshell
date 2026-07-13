[中文](resource-lifecycle.zh.md)

# Resource Lifecycle and Deep Sleep

## Goal

Reduce CPU, network, and repaint work while AxShell is unfocused without interrupting SSH commands, local PTYs, or SFTP transfers. Focus changes come from GPUI window activation events; AxShell does not create a polling thread to detect focus.

## State Machine

```text
Foreground --window deactivates--> Background --timeout--> DeepSleep
    ^                                  |                    |
    +----------window activates--------+--------------------+
```

- Foreground: normal terminal, cursor, monitoring, and theme refresh rates.
- Background: stop local/remote monitoring, theme polling, and cursor blinking immediately. Backend events still drain and rendering is coalesced at a lower rate so SSH output cannot accumulate in memory or a channel.
- DeepSleep: retain the same low-frequency event pump for backend events and required SFTP idle cleanup. Focus is never polled; the system window activation event restores foreground work immediately.

The default deep-sleep delay is 5 minutes after the window loses focus. Settings allow Off, 1, 5, 15, or 30 minutes. Off disables deep sleep only; background throttling still applies.

## Phase One: Safe Throttling

Phase one does not close SSH, PTY, or SFTP workers. It provides:

- A persistent deep-sleep timeout setting with a 5-minute default.
- `Foreground / Background / DeepSleep` state driven by `observe_window_activation`.
- No monitoring samples, theme polling, or cursor blinking while backgrounded or asleep.
- Continued backend draining with coalesced lower-frequency terminal and UI refreshes.
- A throttled SFTP idle-reclaim check that retains the existing five-minute idle behavior.

It intentionally does not pause remote commands, close local shells, disconnect SSH, or pause/cancel transfers. Phase one adds no deep-sleep SFTP close rule; existing idle reclamation remains unchanged, and remote-edit watcher pin/refcount protection is deferred to phase two.

## Later Defenses

### Phase Two: SFTP pins and deep-sleep reclamation

Each SFTP group gains explicit pins/refcounts. Transfers, remote-edit watchers, sync, directory work, and preview downloads hold a pin. Only an unpinned, unfocused group that has reached deep sleep may release its worker. Refocus reconnects on demand, never all pages at once.

Implementation semantics: a pin is acquired before a command is queued, so work waiting for the worker is protected too. Short work releases on completion; transfers and auto-uploads release when their child task ends; a remote-edit watcher releases when the editor closes or the worker is explicitly closed. Explicit close, transfer cancellation, and reconnect may still force worker shutdown. The ordinary five-minute background idle rule remains unchanged; deep sleep immediately reclaims an unpinned, non-current group.

### Phase Three: SSH, PTY, and query task ownership

Implemented. A terminal backend now owns both its command channel and a non-blocking shutdown controller, so tab close, reconnect, and a natural `Closed` event converge on one path. The SSH primary task receives `Close`, waits for up to two seconds, and is aborted only after that timeout. Remote monitoring and CWD queries are held in the primary session's `JoinSet` and aborted/joined when it exits. Local PTY shutdown kills the shell first, then a background reaper joins its reader and writer without blocking the UI.

Window close and the application Quit menu both call `shutdown_all_backends()`, including SFTP handles; layout saving remains synchronous during the close request. This phase does not recursively terminate background process trees started by a shell, and cannot guarantee the two-second graceful window if the OS force-kills the application.

### Phase Four: Process exit and system sleep

Add application-level `shutdown_all()`: stop new work, cancel and join backends, then save layout. After OS sleep/resume, validate connection health and resume monitoring only for the focused page, avoiding a reconnect or probe storm.

## Resource Policy

| Resource | Background | DeepSleep | Resume |
| --- | --- | --- | --- |
| SSH terminal / local PTY | Keep running | Keep running | Continue unchanged; bounded cleanup on close/reconnect |
| Backend events | Low-frequency drain | Low-frequency drain | Refresh immediately |
| Terminal paint / cursor | Coalesced paint; no blink | Lower-rate paint; no blink | Normal refresh |
| Local and remote monitoring | No new samples | No new samples | Sample focused page only |
| Follow-system theme | No polling | No polling | Sync once |
| SFTP transfers | Continue | Continue | No reconnect needed |
| Idle SFTP worker | Existing timeout applies | Phase two evaluates pins | Reconnect on demand |

## Verification Boundary

- Unit tests: state transitions, timeouts, disabled deep sleep, and configuration normalization.
- Local checks: `rustfmt`, `cargo check`, `cargo test --quiet`, and `git diff --check`.
- GUI checks: monitoring stops after focus loss; five minutes reaches deep sleep; refocus restores monitoring and terminal rendering; background SSH output remains bounded.
- Connected checks: closing a tab or window while SSH is connecting or while a remote probe/CWD query is running should exit within two seconds or log an abort; closing and reconnecting a local shell must not leave reader/writer threads behind.
