# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮在 SSH 会话既无密码也无私钥时直接进入终端内密码输入，不先用空密码发起认证。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取现有记录，并刷新为“无密码/无私钥 SSH 会话直接在终端内输入密码”任务语境。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、Tokio、`tracing`。
- 版本约束：仓库声明 `rust-version = "1.88.0"`、edition `2024`；本机 `rustc 1.96.1`、`cargo 1.96.1` 可用。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/terminal.rs`
- 本轮代码入口：`src/app/session_ui.rs`，`src/app/actions/terminal.rs`，`src/app/workspace.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/session.rs`，`src/backend/ssh/connection.rs`。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖，不修改外部 cargo 缓存源码。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立 test job。
- 当前实施验证命令：已执行受影响 Rust 文件 `rustfmt --edition 2024`、`cargo check`、新增聚焦测试、完整 `cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 外部依赖：无；不需要联网。真实 GUI 中终端密码输入、回车重连和焦点体验仍需手工确认。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`src/app/session_ui.rs`，`src/app/actions/terminal.rs`，`src/app/workspace.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/session.rs`，`src/backend/ssh/connection.rs`，`docs/project-implementation-tracker/project-map.md`。

## 环境变化检查

- 是否发现变化：否
- 变化摘要：项目语言、依赖管理、CI 入口和测试命令不变；本轮在 app 层先判断密码和私钥是否都为空，满足时创建未连接 SSH tab 并直接写入本地 `Password: ` 提示，回车后再用输入密码发起连接。
- 受影响文件：`src/app.rs`，`src/app/session_ui.rs`，`src/app/actions/session.rs`，`src/app/actions/terminal.rs`，`src/app/workspace.rs`，`src/app/lifecycle/event_loop.rs`，`src/terminal/backend.rs`，跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；当前任务和验证范围已切换。

## 开工判定

- 状态：允许开工
- 原因：现有 SSH 认证由 backend 一次性调用 `authenticate_password`，远端不会自然产生 password prompt；在密码和私钥都为空时可以避免空密码认证，先用 inactive backend 打开 tab 并在终端内收集密码。
- 开工前动作：已读取 `AGENTS.md`、环境记录、实施记录、项目地图、manifest 和相关源码；确认不需要联网或多 agent。
- 完成后动作：Rust 格式化、`cargo check`、新增聚焦测试、完整测试、空白检查和 tracking docs validator 均已完成；真实 GUI 终端密码输入仍需手工确认。

## 最后确认时间

- 2026-07-12 11:32 +0800
