[English](development.en.md)

# AxAshell 开发与打包

## 环境要求

- Rust `1.85.0` 或更高版本
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
- 文件变化后会重新构建并重启应用
- `--release` 可切换到 `target/release/ax_ashell`

调试模式下还会额外写日志到：

```text
target/debug/dev-reload-logs/session-<timestamp>/
```

其中包含：

- dev-reload 自身事件
- `cargo build` 的 `stdout` / `stderr`
- 应用进程的 `stdout` / `stderr`

如果应用已经启动成功，后续某次改动导致编译失败，`cargo dev-reload` 会保留当前旧进程并继续监听；只有首次启动即编译失败时才会退出。

## macOS `.app` 打包

```bash
./scripts/package-macos-app.sh
open target/release/ax_ashell.app
```

脚本会：

- 先执行 `cargo build --release`
- 生成 `target/release/ax_ashell.app`
- 写入 `Info.plist`
- 复制 `assets/icons/terminal_icon_all_formats/terminal_icon.icns` 到 bundle

如果环境里有 `codesign`，脚本会自动签名。可通过环境变量覆盖签名身份：

```bash
SIGN_IDENTITY="Developer ID Application: Example" ./scripts/package-macos-app.sh
```

## Debian `.deb` 打包

```bash
cargo build --release
cargo deb
```

安装示例：

```bash
sudo dpkg -i target/debian/ax_ashell_<version>-1_amd64.deb
```

桌面入口定义位于：

```text
assets/ax_ashell.desktop
```

## 版本与资源

- Cargo 包版本当前使用 `2026.7.6` 这类 semver 兼容形式
- 对外展示版本按日期格式映射为 `YYYY.MM.DD`
- 图标资源位于 `assets/icons/terminal_icon_all_formats`

## 配置与日志

本地配置默认写入：

```text
~/.config/ax_ashell/sessions.json
```

运行日志默认写入：

```text
~/.config/ax_ashell/log
```

## 相关文档

- [使用指南](user-guide.md)
