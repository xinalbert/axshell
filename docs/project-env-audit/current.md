# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust / GPUI 桌面应用；本轮为终端右侧滚动条预留独立内容区，并使 Windows/Linux 平台菜单独占全宽顶部。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已复核并刷新为“终端滚动槽与平台菜单布局”任务语境。

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`tokio` 多线程运行时，`russh` SSH / SFTP 后端。
- 版本约束：`rust-version = 1.88.0`，edition `2024`；本机为 `rustc 1.96.1` / `cargo 1.96.1`。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/app/views/terminal_panel.rs`，`src/app/views/layout.rs`
- 依赖统一策略：使用现有 GPUI Component `Scrollbar` / `vertical_scrollbar` 和 root workspace 布局，不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：CI 配置当前以构建为主，未声明独立 test job。
- 当前实施验证命令：`rustfmt --edition 2024` 覆盖修改 Rust 文件、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 外部依赖：不需要联网；完整交互验证需要运行 Windows/Linux GUI 并输出长行文本。
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/app/views/terminal_panel.rs`，`src/app/views/layout.rs`，`src/app/core/types.rs`，`AGENTS.md`。

## 环境变化检查

- 是否发现变化：是
- 变化摘要：运行时与工具链未变；`vertical_scrollbar` 默认以绝对定位覆盖 terminal 元素右侧，现有 pane 只保留左侧 inset；Windows/Linux 平台菜单被嵌入 workspace 主列，随侧栏状态改变起点和宽度。
- 受影响文件：`src/app/views/terminal_panel.rs`，`src/app/views/layout.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：现有 scrollbar 支持被放入专用布局列，且根布局已经是纵向容器；仅需调整布局层级与固定槽宽，不影响终端滚动状态或菜单 action。
- 开工前动作：已读取 `AGENTS.md`、环境记录、项目地图、terminal pane / root layout / scrollbar 实现及现有应用菜单初始化；已确认本机 Rust 工具链可用。
- 完成后动作：执行格式化、编译、完整测试、空白检查和 tracking docs validator；GUI 侧需人工验证长行文本、滚动条拖动和两种侧栏状态。

## 最后确认时间

- 2026-07-10 14:04 +0800
