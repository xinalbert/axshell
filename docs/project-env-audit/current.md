# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust / GPUI 桌面应用；本轮目标是把 SSH 登录阶段的网络重试改为可配置，并在设置中暴露重试次数与延时

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“SFTP 空闲断开”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`alacritty_terminal` 终端模型，`tokio` 运行时，`russh` SSH / SFTP 后端
- 版本约束：`rust-version = 1.88.0`，edition `2024`
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮代码入口：`src/backend/ssh/connection.rs`，`src/sftp/auth.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs/settings/terminal.rs`
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`AGENTS.md`，`src/backend/ssh/connection.rs`，`src/sftp/auth.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs/settings/terminal.rs`

## 测试环境

- 测试框架：Rust 编译检查、Rust 单元测试、tracking docs validator
- 默认测试命令：`cargo check`
- 当前实施验证命令：计划执行 `rustfmt --edition 2024 src/backend/ssh/connection.rs src/sftp/auth.rs src/config/store.rs src/app.rs src/app/lifecycle/init.rs src/app/dialogs/settings/terminal.rs`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator
- 外部依赖：本轮默认不依赖联网；仅为“主流默认值依据”做了一次官方文档检索
- 工具可用性：本机此前已成功执行 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 与 tracking docs validator；本轮待复验
- 证据文件：`Cargo.toml`，`docs/project-implementation-tracker/project-map.md`，`AGENTS.md`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：当前环境主体未变；本轮范围从 SFTP 生命周期切换到 SSH transport retry 配置化，关键链路是 `ConfigStore` 持久化、SSH/SFTP transport connect helper 和 Settings Terminal 页输入状态
- 受影响文件：`src/backend/ssh/connection.rs`，`src/sftp/auth.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs/settings/terminal.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/research.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目工具链和依赖策略未变；当前问题可限制在配置层、SSH/SFTP transport connect helper 和设置页，不需要修改认证协议或新增依赖
- 开工前动作：已读取 `AGENTS.md`、环境记忆、项目地图、SSH/SFTP 连接实现和 Settings 页面；已完成一次外部默认值依据检索；本轮不使用多 agent
- 完成后动作：对改动文件执行 Rust 格式化、编译检查、完整测试、空白检查和 tracking docs validator；GUI 侧仍需手工确认设置页输入、保存和连接提示文案
