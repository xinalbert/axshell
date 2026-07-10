# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust / GPUI 桌面应用；本轮修复关闭 terminal tab 后遗留的按 tab ID UI 状态，避免缓存增长和失效 pane 鼠标命中。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已复核并刷新为“terminal tab UI 状态回收”任务语境。

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`tokio` 多线程运行时，`russh` SSH / SFTP 后端。
- 版本约束：`rust-version = 1.88.0`，edition `2024`；本机为 `rustc 1.96.1` / `cargo 1.96.1`。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/app/actions/session.rs`，`src/app.rs`，`src/app/actions/terminal.rs`
- 依赖统一策略：复用现有 `AxShell` 状态字段和 tab 关闭入口，不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：CI 配置当前以构建为主，未声明独立 test job。
- 当前实施验证命令：`rustfmt --edition 2024` 覆盖修改 Rust 文件、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 外部依赖：不需要联网；完整交互验证需要关闭正在连接、悬浮链接或有输入法预编辑的 terminal tab。
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/app/actions/session.rs`，`src/app/actions/terminal.rs`，`src/app.rs`，`src/app/core/types.rs`，`AGENTS.md`。

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行时与工具链未变；`handle_tab_close()` 移除 terminal tab 但未清除 `terminal_scrollbars`、`terminal_bounds` 和关联短期 UI 状态，其中残留 bounds 会参与 pane 鼠标命中。
- 受影响文件：`src/app/actions/session.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：所有关闭路径均收敛到 `handle_tab_close()`，可在此增加单一清理 helper，不影响终端后端的 `Close` 命令或 pane 选中策略。
- 开工前动作：已读取 `AGENTS.md`、环境记录、项目地图及 tab 关闭、鼠标命中、IME/hover/connection progress 状态链路；已确认本机 Rust 工具链可用。
- 完成后动作：已执行格式化、编译、完整测试和空白检查；GUI 侧仍需人工验证关闭有悬浮链接、IME 预编辑和连接进度的 tab。

## 最后确认时间

- 2026-07-10 15:01 +0800
