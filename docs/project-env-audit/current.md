# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮新增串口和 Telnet terminal session，保留既有 SSH、本地 terminal、SFTP、监控和生命周期架构。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已按串口/Telnet 运行依赖和验证结果刷新。
- changes.md：已追加本轮环境与验证记录。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、Alacritty Terminal、russh、russh-sftp、`serialport 4.9.0`。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机使用 `rustc 1.96.1`、`cargo 1.96.1`。
- 包管理器：`cargo`；新增 `serialport = "4"` 后锁定 `serialport 4.9.0` 及其平台相关依赖。
- 构建 / 运行入口：`src/main.rs`、`src/app/lifecycle/startup.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/lifecycle/init.rs`。
- 本轮代码入口：`src/session.rs`、`src/backend/serial.rs`、`src/backend/telnet.rs`、`src/terminal/backend.rs`、`src/terminal/tab.rs`、`src/app/actions/session.rs`、`src/app/dialogs/ssh.rs`。
- 平台前提：Linux 构建 `serialport` 需要 `libudev` 开发包；`.github/workflows/ci.yml` 已安装 `libudev-dev`。运行时还取决于用户的串口设备驱动、访问权限和端口占用状态。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、tracking docs validator。
- 默认测试命令：`rustfmt --edition 2024 <changed-rust-files>`、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Ubuntu x86_64/aarch64 和 macOS x86_64/aarch64 运行 release build；Linux runner 安装 GPUI 与 `libudev` 系统库。
- 外部服务：自动化测试不依赖实体串口或 Telnet server；真实收发、设备拔出、端口占用、Telnet 登录/协商与断线重试需在目标平台手动测试。
- 证据文件：`AGENTS.md`、`Cargo.toml`、`Cargo.lock`、`.github/workflows/ci.yml`、`src/session.rs`、`src/backend/serial.rs`、`src/backend/telnet.rs`、`src/app/actions/session.rs`、`src/app/dialogs/ssh.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：新增跨平台 `serialport` 依赖；Linux lockfile 引入 `libudev`/`libudev-sys`，CI 已具备对应 `libudev-dev`；Telnet 复用现有 Tokio 网络与代理依赖，不增加额外网络 crate。
- 受影响文件：`Cargo.toml`、`Cargo.lock`、`src/session.rs`、`src/backend/serial.rs`、`src/backend/telnet.rs`、`src/terminal/`、`src/app/`、`locales/`、`docs/project-env-audit/`、`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是；两者已更新。

## 开工判定

- 状态：已完成自动化验证。
- 原因：本机 toolchain 高于仓库 MSRV；新增依赖已解析、可编译，并与 CI Linux 系统库前提一致。串口端口枚举和设备 I/O 均不在 UI render/hover 路径；Telnet 连接使用已有 TCP/proxy transport。
- 完成动作：已读取环境、实施记录、项目地图和 fast hover 工作流；已审查参考项目；已运行受影响 Rust 文件格式化、`cargo check`、完整 `cargo test --quiet`（220 项）、`git diff --check` 和 tracking docs validator。仅保留依赖 `block v0.1.6` future-incompat warning。

## 最后确认时间

- 2026-07-16 17:00 +0800
