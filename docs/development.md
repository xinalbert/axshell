[中文](development.zh.md)

# AxShell Development and Packaging

## Requirements

- Rust `1.88.0` or newer
- A working Cargo toolchain
- A desktop environment on macOS, Linux, or Windows

Debian / Ubuntu packaging also requires:

```bash
sudo apt install pkg-config libfontconfig1-dev
cargo install cargo-deb
```

## Run Locally

Build and run the app:

```bash
cargo run --release
```

## Restart-Based Dev Reload

The repository exposes this Cargo alias in `.cargo/config.toml`:

```bash
cargo dev-reload
```

It maps to:

```bash
cargo run --example dev_reload --
```

Current behavior:

- It is restart-based live development, not state-preserving hot reload
- It watches `src`, `assets`, `locales`, `Cargo.toml`, `Cargo.lock`, `build.rs`, and `.cargo` by default
- File changes trigger rebuild and relaunch; on Windows it stops the running app before rebuilding so the active `.exe` does not block replacement
- On macOS it launches through an isolated development app bundle so it does not share app identity or input focus with a running release `.app`
- `--release` switches to `target/release/ax_shell`

In debug mode it also writes logs to:

```text
target/debug/dev-reload-logs/session-<timestamp>/
```

That directory contains:

- dev-reload runner events
- `cargo build` `stdout` / `stderr`
- app process `stdout` / `stderr`

Whether the failure happens on the initial startup build or on a later rebuild, `cargo dev-reload` now keeps watching and waits for the next file change before trying again.

## macOS `.app` Packaging

```bash
./scripts/package-macos-app.sh
open target/release/AxShell.app
```

The script will:

- run `cargo build --release`
- create `target/release/AxShell.app`
- write `Info.plist`
- copy `assets/icons/terminal_icon_all_formats/terminal_icon.icns` into the bundle

If `codesign` is available, the script signs the bundle automatically. Override the signing identity with:

```bash
SIGN_IDENTITY="Developer ID Application: Example" ./scripts/package-macos-app.sh
```

The local `.app` packager and the GitHub Release workflow both read version rules from `scripts/release_version.py` instead of rebuilding them separately.

## Debian `.deb` Packaging

```bash
cargo build --release
cargo deb
```

Install example:

```bash
sudo dpkg -i target/debian/ax_shell_<version>-1_amd64.deb
```

The desktop entry metadata lives at:

```text
assets/ax_shell.desktop
```

## GitHub Release

Push one of these tag formats to trigger a published release:

```text
vYYYY.M.D
vYYYY.M.D-N
```

Current mapping:

- Tag / Cargo / runtime version: `v2026.7.6` / `2026.7.6`, or `v2026.7.6-1` / `2026.7.6-1`
- Public version: `2026.07.06` or `2026.07.06.1`
- macOS `CFBundleShortVersionString`: `2026.07.06`
- macOS `CFBundleVersion`: `20260706` or `20260706.1`

`Cargo.toml` cannot use `2026.07.06` directly because Cargo rejects semver components with leading zeros. The canonical tag and manifest version now stay Cargo-compatible, and the script derives the public-facing version separately.

On tag builds the workflow resolves the tag through `scripts/release_version.py`, updates `Cargo.toml` and `Cargo.lock` inside the runner, and only then runs `cargo build --release`. That keeps `env!("CARGO_PKG_VERSION")`, release asset names, and macOS bundle metadata on the same version source.

Manual `workflow_dispatch` runs do not create a GitHub Release. They build from the current `Cargo.toml` version and upload workflow artifacts only.

Current GitHub Releases publish:

- `ax_shell-<version-label>-windows-x86_64.zip`
- `ax_shell-<version-label>-linux-x86_64.tar.gz`
- `ax_shell-<version-label>-linux-x86_64.deb`
- `ax_shell-<version-label>-linux-aarch64.tar.gz`
- `ax_shell-<version-label>-linux-aarch64.deb`
- `ax_shell-<version-label>-macos-aarch64.zip`
- `ax_shell-<version-label>-macos-x86_64.zip`
- `ax_shell-<version-label>-macos-universal.zip`

Linux ARM64 builds use GitHub Actions native ARM64 Ubuntu runners. Windows ARM64 runners are still treated as preview capacity, so Windows ARM64 is not part of the formal release matrix yet.

## Versioning and Assets

- Published releases use the Git tag as the single version source
- The Cargo package keeps semver-compatible `YYYY.M.D` / `YYYY.M.D-N` versions
- Public-facing release labels map to `YYYY.MM.DD` / `YYYY.MM.DD.N`
- Icon assets live under `assets/icons/terminal_icon_all_formats`

## Config and Logs

Local config is written to:

```text
~/.config/ax_shell/sessions.json
```

For upgrades from the old name, `~/.config/ax_ashell/sessions.json` and `themes/` are copied into `~/.config/ax_shell/` when the new config files do not exist yet. The migration does not delete the old directory.

Runtime logs are written to:

```text
~/.config/ax_shell/log
```

The app logs `ax_shell=info,warn` by default. Override with `RUST_LOG` when needed, for example `RUST_LOG=ax_shell=debug,russh=debug`. Runtime logs roll by minute and keep the latest 48 files by default. The About page can open the runtime log directory directly.

When the app crashes because of a Rust panic, the panic hook also writes a crash report to:

```text
~/.config/ax_shell/crash/ax_shell-crash-*.log
```

Crash reports include the panic location, version, thread, runtime log directory, and backtrace. The About page can open the crash report directory directly. When filing an issue at `https://github.com/xinalbert/axshell/issues`, attach the crash file and the latest runtime logs.

## Related Docs

- [Documentation Index](README.md)
- [User Guide](user-guide.md)
