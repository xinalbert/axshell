[中文](development.md)

# AxAshell Development and Packaging

## Requirements

- Rust `1.85.0` or newer
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
- File changes trigger rebuild and relaunch
- `--release` switches to `target/release/ax_ashell`

In debug mode it also writes logs to:

```text
target/debug/dev-reload-logs/session-<timestamp>/
```

That directory contains:

- dev-reload runner events
- `cargo build` `stdout` / `stderr`
- app process `stdout` / `stderr`

If the app has already launched successfully, a later rebuild failure keeps the old process running and continues watching. Only an initial startup build failure exits the command.

## macOS `.app` Packaging

```bash
./scripts/package-macos-app.sh
open target/release/ax_ashell.app
```

The script will:

- run `cargo build --release`
- create `target/release/ax_ashell.app`
- write `Info.plist`
- copy `assets/icons/terminal_icon_all_formats/terminal_icon.icns` into the bundle

If `codesign` is available, the script signs the bundle automatically. Override the signing identity with:

```bash
SIGN_IDENTITY="Developer ID Application: Example" ./scripts/package-macos-app.sh
```

## Debian `.deb` Packaging

```bash
cargo build --release
cargo deb
```

Install example:

```bash
sudo dpkg -i target/debian/ax_ashell_<version>-1_amd64.deb
```

The desktop entry metadata lives at:

```text
assets/ax_ashell.desktop
```

## Versioning and Assets

- The Cargo package currently uses semver-compatible versions such as `2026.7.6`
- Public-facing release labels are mapped to `YYYY.MM.DD`
- Icon assets live under `assets/icons/terminal_icon_all_formats`

## Config and Logs

Local config is written to:

```text
~/.config/ax_ashell/sessions.json
```

Runtime logs are written to:

```text
~/.config/ax_ashell/log
```

## Related Docs

- [User Guide](user-guide.en.md)
