# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮修正 Appearance 页普通 Theme profile 选择未可靠应用到窗口的问题。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取现有记录，并刷新为“普通 Theme profile 直接应用修正”任务语境。

## 运行环境

- 主技术栈：Rust 2024、GPUI、gpui-component、Tokio、`tracing`。
- 版本约束：仓库声明 `rust-version = "1.88.0"`、edition `2024`；本机可用 Rust 工具链已在既有记录中确认高于最低版本。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/init.rs`
- 本轮代码入口：`src/app/theme.rs`。
- 依赖策略：不修改 `Cargo.toml` / `Cargo.lock`，不新增依赖，不直接修改 cargo 缓存中的外部 `gpui-component` 源码。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立 test job。
- 当前实施验证命令：已执行 `rustfmt --edition 2024 src/app/theme.rs`、`cargo check`、`cargo test --quiet`、`cargo build`、`git diff --check` 和 tracking docs validator。
- 外部依赖：本轮不需要联网；真实普通 Theme profile 连续切换仍需后续 GUI 手工验证。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/custom.rs`，`src/app/dialogs/settings/shell.rs`，`src/app/theme.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/session.rs`，`docs/project-implementation-tracker/project-map.md`。

## 环境变化检查

- 是否发现变化：是
- 变化摘要：项目语言、依赖管理和 CI 入口不变；本轮从 Custom Theme 实时预览切换到 Appearance 普通 profile 的直接应用，验证范围仍集中在主题应用链路。
- 受影响文件：`src/app/theme.rs` 和跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；当前任务、代码入口、验证命令和 GUI 验证边界已切换。

## 开工判定

- 状态：允许开工
- 原因：当前问题可在仓库内主题应用层收窄修复；普通 profile 可直接安装 registry 的明确 light/dark 配置对，不让 custom compatibility fallback 参与解析。
- 开工前动作：已读取 `AGENTS.md`、环境记录、实施记录、项目地图、相关 skills、`src/app/theme.rs`、内置主题 JSON 和 gpui-component Theme/ThemeRegistry 实现；确认不需要多 agent。
- 完成后动作：已完成受影响 Rust 文件格式化、编译、完整单测、debug 构建、空白检查和 tracking docs validator；真实 GUI 连续 profile 切换保留手工确认。

## 最后确认时间

- 2026-07-11 23:18 +0800
