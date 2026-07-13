[English](development.md)

# AxShell 开发与打包

## 环境要求

- Rust `1.88.0` 或更高版本
- 可用的 Cargo 工具链
- macOS、Linux 或 Windows 桌面环境

Debian / Ubuntu 打包额外需要：

```bash
sudo apt install pkg-config libfontconfig1-dev
cargo install cargo-deb
```

## 本地运行

直接构建并运行：

```bash
cargo run --release
```

## 开发期自动重载

仓库通过 `.cargo/config.toml` 提供了别名：

```bash
cargo dev-reload
```

它对应：

```bash
cargo run --example dev_reload --
```

当前行为：

- 属于重启式开发重载，不是状态保持型 hot reload
- 默认监听 `src`、`assets`、`locales`、`Cargo.toml`、`Cargo.lock`、`build.rs` 和 `.cargo`
- 文件变化后会重新构建并重启应用；其中 Windows 会先停掉旧进程再构建，避免运行中的 `.exe` 阻塞覆盖
- 在 macOS 上会以独立的开发 app bundle 启动，避免和已运行的 release `.app` 共用应用身份或抢错输入焦点
- `--release` 可切换到 `target/release/ax_shell`

调试模式下还会额外写日志到：

```text
target/debug/dev-reload-logs/session-<timestamp>/
```

其中包含：

- dev-reload 自身事件
- `cargo build` 的 `stdout` / `stderr`
- 应用进程的 `stdout` / `stderr`

无论是首次启动还是后续某次改动，只要编译失败，`cargo dev-reload` 都会保留当前状态并继续监听，等下一次文件变化后再尝试重新构建。

## macOS `.app` 打包

```bash
./scripts/package-macos-app.sh
open target/release/AxShell.app
```

脚本会：

- 先执行 `cargo build --release`
- 生成 `target/release/AxShell.app`
- 写入 `Info.plist`
- 复制 `assets/icons/terminal_icon_all_formats/terminal_icon.icns` 到 bundle

如果环境里有 `codesign`，脚本会自动签名。可通过环境变量覆盖签名身份：

```bash
SIGN_IDENTITY="Developer ID Application: Example" ./scripts/package-macos-app.sh
```

本地 `.app` 打包和 GitHub Release workflow 共用 `scripts/release_version.py` 里的版本规则，不再各自拼版本字符串。

## Debian `.deb` 打包

```bash
cargo build --release
cargo deb
```

安装示例：

```bash
sudo dpkg -i target/debian/ax_shell_<version>-1_amd64.deb
```

桌面入口定义位于：

```text
assets/ax_shell.desktop
```

## GitHub Release

推送以下格式的 tag 会触发正式发布：

```text
vYYYY.M.D
vYYYY.M.D-N
```

当前映射规则：

- Tag / Cargo / 运行时版本：`v2026.7.6` / `2026.7.6`，或 `v2026.7.6-1` / `2026.7.6-1`
- 对外版本：`2026.07.06` 或 `2026.07.06.1`
- macOS `CFBundleShortVersionString`：`2026.07.06`
- macOS `CFBundleVersion`：`20260706` 或 `20260706.1`

之所以不把 `Cargo.toml` 直接写成 `2026.07.06`，是因为 Cargo 会拒绝带前导零的 semver 段。因此现在把 tag 和 `Cargo.toml` 统一为 Cargo 兼容格式，再由脚本派生对外展示版本。

workflow 在 tag 构建时会先用 `scripts/release_version.py` 同步 runner 内的 `Cargo.toml` 和 `Cargo.lock`，再执行 `cargo build --release`。这样 `env!("CARGO_PKG_VERSION")`、release 产物名和 macOS bundle 版本都来自同一个 tag。

手动运行 `workflow_dispatch` 时不会创建 GitHub Release，只会基于当前 `Cargo.toml` 版本构建并上传 workflow artifacts。

当前 GitHub Release 会发布：

- `ax_shell-<version-label>-windows-x86_64.zip`
- `ax_shell-<version-label>-linux-x86_64.tar.gz`
- `ax_shell-<version-label>-linux-x86_64.deb`
- `ax_shell-<version-label>-linux-aarch64.tar.gz`
- `ax_shell-<version-label>-linux-aarch64.deb`
- `ax_shell-<version-label>-macos-aarch64.zip`
- `ax_shell-<version-label>-macos-x86_64.zip`
- `ax_shell-<version-label>-macos-universal.zip`

Linux ARM64 使用 GitHub Actions 的 native ARM64 Ubuntu runner 构建。Windows ARM64 runner 目前仍按 preview 能力处理，暂不并入正式发布矩阵。

## 版本与资源

- 已发布版本以 Git tag 为唯一版本源
- Cargo 包版本保持 `YYYY.M.D` / `YYYY.M.D-N` 这类 semver 兼容形式
- 对外展示版本按日期格式映射为 `YYYY.MM.DD` / `YYYY.MM.DD.N`
- 图标资源位于 `assets/icons/terminal_icon_all_formats`

## 配置与日志

本地配置默认写入：

```text
~/.config/ax_shell/sessions.json
```

旧版 `~/.config/ax_ashell/sessions.json` 和 `themes/` 会在新目录缺失时自动复制到 `~/.config/ax_shell/`，用于平滑升级；迁移不会删除旧目录。

运行日志默认写入：

```text
~/.config/ax_shell/log
```

应用默认记录 `ax_shell=info,warn` 级别日志；可通过 `RUST_LOG` 覆盖，例如 `RUST_LOG=ax_shell=debug,russh=debug`。运行日志按分钟滚动，默认保留最近 48 个日志文件。设置页 About 中可直接打开运行日志目录。

程序发生 Rust panic 崩溃时，panic hook 会额外写入崩溃报告：

```text
~/.config/ax_shell/crash/ax_shell-crash-*.log
```

崩溃报告包含 panic 位置、版本、线程、常规运行日志目录和 backtrace。设置页 About 中可直接打开崩溃报告目录。提交反馈时请在 `https://github.com/xinalbert/axshell/issues` 附上对应 crash 文件和最近的运行日志。

## 相关文档

- [文档导航](README.zh.md)
- [使用指南](user-guide.zh.md)
