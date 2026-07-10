# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮为 Settings 关闭流程增加每次都会出现的确认弹窗，并让弹窗打开后的第二次 Settings 快捷键执行上次记住的动作。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取现有记录，并刷新为本轮 Settings 关闭确认任务语境。

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` / `gpui_component` UI，JSON 配置持久化，自定义 workspace action 与快捷键录制/绑定。
- 版本约束：仓库声明 `rust-version = 1.88.0`、edition `2024`；本机 `rustc 1.96.1`、`cargo 1.96.1` 可用。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/config/model.rs`，`src/config/store.rs`，`src/app/dialogs/settings_close_confirm.rs`，`src/app/workspace.rs`，`src/app/dialogs/settings/shell.rs`，`src/app/views/tab_bar.rs`。
- 依赖策略：不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立 test job。
- 当前实施验证命令：对全部变更 Rust 文件执行 `rustfmt --edition 2024`，运行设置二次快捷键动作定向测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；本轮均已执行通过。
- 外部依赖：无；本轮只调整本地 UI 与配置持久化，不需要联网或外部服务。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`.github/workflows/ci.yml`，`src/app/dialogs/sftp_close_confirm.rs`，`src/config/model.rs`，`src/config/store.rs`，`src/app/workspace.rs`。

## 环境变化检查

- 是否发现变化：否
- 变化摘要：运行时、依赖、工具链和测试入口未变化；本轮新增向后兼容的“第二次 Settings 快捷键动作”布尔偏好与关闭确认模块，确认弹窗不会被永久跳过。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs.rs`，`src/app/dialogs/settings_close_confirm.rs`，`src/app/dialogs/settings/shell.rs`，`src/app/dialogs/settings/workspace.rs`，`src/app/workspace.rs`，`src/app/views/tab_bar.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；当前任务边界、配置 schema 和验证范围已切换。

## 开工判定

- 状态：允许开工
- 原因：项目已有 SFTP “确认弹窗始终出现、第二次快捷键执行记住动作”的模式，可在现有边界内复用，不需要新增依赖。
- 开工前动作：已读取 `AGENTS.md`、环境记录、实施记录、项目地图、manifest/CI 证据和相关源码；无需联网和多 agent。
- 完成后动作：规定的格式化、2 项定向测试、`cargo check`、85 项完整测试、`git diff --check` 和 tracking docs validator 全部通过；真实 GUI 中的首次弹窗、第二次快捷键、remember 与保持打开仍需手工确认。

## 最后确认时间

- 2026-07-10 22:47 +0800
