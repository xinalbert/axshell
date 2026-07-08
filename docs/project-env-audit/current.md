# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用的真实功能改动；本轮目标是把 SFTP 从每个连接下方的常驻面板改成独立工作区页面，并补齐编号标签页和快捷键聚焦行为。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“SFTP 独立页面与编号标签页”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / rust-i18n / alacritty_terminal
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/app/workspace.rs`，`src/session/pane.rs`，`src/app/ui/layout.rs`，`src/app/ui/tab_bar.rs`，`src/app/ui/terminal_panel.rs`，`src/app/ui/sftp_panel.rs`
- 渲染依据：工作区页面由 `WorkspacePage`、`set_workspace_page()` 和 `render_terminal_panel()` / `render_tab_bar()` 协同决定；SFTP 数据状态仍按 connection group 挂在 `TabGroup.sftp`
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/app/types.rs`，`src/app/workspace.rs`，`src/session/pane.rs`，`src/app/ui/layout.rs`，`src/app/ui/tab_bar.rs`，`src/app/ui/terminal_panel.rs`，`src/app/ui/sftp_panel.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 --config skip_children=true src/app/types.rs src/app/mod.rs src/app/init.rs src/app/workspace.rs src/session/pane.rs src/session/mod.rs src/session/config.rs src/app/ui/mod.rs src/app/ui/layout.rs src/app/ui/tab_bar.rs src/app/ui/terminal_panel.rs src/app/ui/sftp_panel.rs src/app/app_menu.rs src/app/search.rs`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；标签页交互和 SFTP 页面切换仍需要 GUI 手工验证
- 工具可用性：本机 `cargo` 可正常执行；当前工程已有 Rust 测试可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/app/types.rs`，`src/app/workspace.rs`，`src/session/pane.rs`，`src/app/ui/layout.rs`
- 本轮验证结果：`rustfmt --edition 2024 --config skip_children=true src/app/types.rs src/app/mod.rs src/app/init.rs src/app/workspace.rs src/session/pane.rs src/session/mod.rs src/session/config.rs src/app/ui/mod.rs src/app/ui/layout.rs src/app/ui/tab_bar.rs src/app/ui/terminal_panel.rs src/app/ui/sftp_panel.rs src/app/app_menu.rs src/app/search.rs` 通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` 通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 对“当前连接 SFTP 快捷键直接聚焦到已打开页面”的行为未手工验证

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从“终端交互期延迟输出”切换到“工作区页面与标签模型调整”；运行环境不变，但页面路由、标签生成和快捷键语义需要同时修改
- 受影响文件：`src/app/types.rs`，`src/app/mod.rs`，`src/app/init.rs`，`src/app/workspace.rs`，`src/session/pane.rs`，`src/session/mod.rs`，`src/session/config.rs`，`src/app/ui/mod.rs`，`src/app/ui/layout.rs`，`src/app/ui/tab_bar.rs`，`src/app/ui/terminal_panel.rs`，`src/app/ui/sftp_panel.rs`，`src/app/app_menu.rs`，`src/app/search.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：现有 `WorkspacePage`、`TabGroup.sftp` 和 `set_workspace_page()` 已具备重构基础，不需要更换 SFTP 后端或新增依赖
- 开工前动作：已复查工作区页面切换、group 激活、SFTP 事件回写和顶部标签渲染路径；已确认不需要联网、不使用多 agent
