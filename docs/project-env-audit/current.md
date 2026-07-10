# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust / GPUI 桌面应用；本轮目标是修复终端滚动查看历史输出时旧 ANSI 背景色块可能残留的问题

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“终端历史滚动背景色块残留修复”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`alacritty_terminal` 终端模型，`tokio` 运行时，`russh` SSH / SFTP 后端
- 版本约束：`rust-version = 1.88.0`，edition `2024`
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/terminal/element.rs`，`src/terminal.rs`，`src/app/actions/terminal.rs`
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`AGENTS.md`，`src/terminal/element.rs`，`src/terminal.rs`，`src/app/actions/terminal.rs`

## 测试环境

- 测试框架：Rust 编译检查、Rust 单元测试、tracking docs validator
- 默认测试命令：`cargo check`
- 当前实施验证命令：`rustfmt --edition 2024 src/terminal/element.rs`，`cargo check`，`cargo test --quiet`，`git diff --check`，tracking docs validator，均已通过
- 外部依赖：本轮只依赖本地源码事实，不需要联网
- 工具可用性：本机已成功执行 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 与 tracking docs validator
- 证据文件：`Cargo.toml`，`docs/project-implementation-tracker/project-map.md`，`AGENTS.md`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：当前环境主体未变；本轮范围从 SFTP 页面快捷键切换为终端渲染层，关键链路是 `TerminalTab::render_snapshot()` 生成当前 viewport cell，`TerminalElement::paint()` 绘制背景矩形、文字和光标
- 受影响文件：`src/terminal/element.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目工具链和依赖策略未变；问题定位到终端元素绘制层未在每帧显式清理终端 bounds，修复可限制在 `src/terminal/element.rs`，不需要改 terminal buffer、滚动历史或 PTY 后端
- 开工前动作：已读取 `AGENTS.md`、环境记忆、项目地图、终端 snapshot、滚动 action 和元素绘制代码；本轮不联网，不使用多 agent
- 完成后动作：Rust 格式化、编译检查、完整测试、空白检查和 tracking docs validator 已通过；GUI 实际历史滚动渲染仍需后续手工确认
