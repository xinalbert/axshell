# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮对 `src/terminal.rs` 和 `src/sftp.rs` 做行为保持的现代模块拆分。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已复核并刷新为“terminal/SFTP 大文件模块拆分”任务语境。

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` / `gpui_component` UI，`alacritty_terminal` 终端模型，`tokio` runtime，`russh` / `russh-sftp` 后端。
- 版本约束：`rust-version = 1.88.0`，edition `2024`；本机为可用 Rust/Cargo 工具链。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/terminal.rs`，`src/terminal/`，`src/sftp.rs`，`src/sftp/`
- 依赖统一策略：不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`；通过 root module 声明、`pub use` / `pub(crate) use` 和最小 `pub(super)` 可见性完成迁移。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：CI 配置当前以构建为主，未声明独立 test job。
- 当前实施验证命令：`rustfmt --edition 2024` 覆盖所有迁移 Rust 文件、terminal 定向测试、SFTP lifecycle 定向测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 外部依赖：不需要联网；模块迁移不改变 GUI 行为，上一轮标签视觉检查仍需桌面端手工完成。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`.github/workflows/ci.yml`，`src/terminal.rs`，`src/sftp.rs`，`docs/project-implementation-tracker/project-map.md`。

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行时、依赖和工具链不变；已新增 terminal/SFTP 具名子模块并把 root 文件收敛为模块入口和兼容 re-export。
- 受影响文件：`src/terminal.rs`，`src/terminal/`，`src/sftp.rs`，`src/sftp/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是；模块结构变化还必须刷新 `project-map.md`。

## 开工判定

- 状态：允许开工
- 原因：职责边界清楚，可先做纯移动和最小可见性调整；现有测试覆盖按键编码、CWD、backend shutdown、事件队列、SFTP 生命周期、分页预算和路径 reveal。
- 开工前动作：已读取 `AGENTS.md`、环境记录、实施记录、项目地图、两份 root 文件、现有子模块和外部调用点。
- 完成后动作：terminal 38 项、SFTP 14 项定向测试、`cargo check`、完整 78 项测试、空白检查和 tracking docs validator 均通过；项目地图已刷新。

## 最后确认时间

- 2026-07-10 20:21 +0800
