# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用；本轮目标是修正 SFTP 页面传输默认位置、传输信息呈现和列表点击习惯

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“SFTP 交互体验修正”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`tokio` 运行时
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/init.rs`
- 本轮代码入口：`src/app/actions/sftp.rs`，`src/app/views/sftp_panel.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`src/terminal.rs`
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`.github/workflows/release.yml`，`src/main.rs`，`src/app.rs`，`src/app/actions/sftp.rs`，`src/app/views/sftp_panel.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`src/terminal.rs`，`docs/project-implementation-tracker/project-map.md`

## 测试环境

- 测试框架：Rust 编译检查、Rust 单元测试、tracking docs validator
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 src/app/actions/sftp.rs src/app/views/sftp_panel.rs src/app/views/sftp_panel/transfer_panel.rs`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 运行 `cargo check --all-targets` 和 `cargo test --all`
- 外部依赖：无额外服务依赖；如需完整 GUI 行为验证，仍需本机手工运行桌面应用
- 工具可用性：本机可直接执行 `cargo check`、`cargo test` 与 tracking docs validator
- 证据文件：`Cargo.toml`，`.github/workflows/release.yml`
- 本轮验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，25 个测试全部通过；tracking docs validator 通过

## 环境变化检查

- 是否发现变化：是
- 变化摘要：当前环境主体未变，但 `current.md` 语境从源码结构重构切换到 SFTP UI 行为修正；验证重点收敛到 `src/app/actions/sftp.rs` 与 `src/app/views/sftp_panel.rs` 的交互和可编译性
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目边界清晰，运行环境稳定，本轮不依赖联网和外部服务；SFTP 页面和 action 层入口已明确，代码修改已通过本机编译和测试
- 开工前动作：已复查 `Cargo.toml`、`.github/workflows/ci.yml`、`src/app/actions/sftp.rs`、`src/app/views/sftp_panel.rs`、`src/app/views/sftp_panel/transfer_panel.rs` 与现有 tracking 文档
- 开工前动作：已确认先改默认传输目标和点击语义，再用 `cargo check` / `cargo test` 验证
- 完成后动作：已执行 `rustfmt`、`cargo check`、`cargo test` 和 tracking docs validator；GUI 手工验证仍需在真实 SFTP 连接中确认
