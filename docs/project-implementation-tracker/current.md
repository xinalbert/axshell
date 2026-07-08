# 当前项目实施记录

## 当前目标

- 目标：把 SFTP 从每个连接下方的常驻面板改成独立工作区页面，并让顶部标签页按连接编号展示 terminal / SFTP 对应关系
- 交付物：`WorkspacePage::Sftp` 页面态、编号 terminal/SFTP 标签模型、快捷键对当前连接 SFTP 的聚焦逻辑，以及更新后的实施/环境记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/types.rs`，`src/app/workspace.rs`，`src/session/pane.rs`，`src/session/mod.rs`，`src/app/ui/layout.rs`，`src/app/ui/tab_bar.rs`，`src/app/ui/terminal_panel.rs`，`src/app/ui/sftp_panel.rs`，`src/app/app_menu.rs`，`src/app/keybinding_recorder.rs`，`locales/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：SFTP 后端协议流程、传输实现、SSH 认证策略、监控面板样式重构、设置页结构调整

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 刷新环境/实施记录到 SFTP 独立页面任务 | tracking docs validator | 已确认本轮不需要联网或多 agent |
| P2 | completed | `WorkspacePage::Sftp`、快捷键聚焦与页面切换 helper | `cargo check` | 快捷键命中已打开的当前连接 SFTP 时直接聚焦过去 |
| P3 | completed | 顶部编号 terminal/SFTP 标签页和对应切换逻辑 | `cargo check`，`cargo test` | 终端 tab 可关闭，SFTP tab 只切换不关闭 |
| P4 | completed | 菜单/快捷键文案、布局收口与验证 | `rustfmt`，`cargo test`，tracking docs validator | 底部 SFTP 面板已移除 |

## 已完成

- 已确认现有 SFTP UI 仍是 `layout.rs` 中的底部面板，而不是独立页面
- 已确认现有 `WorkspacePage` / `set_workspace_page()` 已能承载新增页面态
- 已确认 SFTP 状态已经按 `TabGroup.sftp` 绑定 connection group，不需要重做后端状态模型
- 已确认 `ToggleSftpZoom` 当前仍是面板最小化语义，需要改成当前连接 SFTP 的打开/聚焦语义
- 已新增 `WorkspacePage::Sftp`，并把快捷键逻辑改为“若当前连接有 SFTP，就直接打开/聚焦该页面”
- 已将顶部标签栏改成编号 terminal/SFTP 对应标签，例如 `2 Host` / `2 SFTP`
- 已移除底部常驻 SFTP 面板和最小化逻辑，SFTP 改为通过工作区页面承载
- 已把终端搜索、pane focus、pane split 等终端专属操作限制在 `WorkspacePage::Terminal`

## 验证

- 已完成：源码级确认工作区页面切换、group 激活、SFTP 事件回写和顶部标签渲染路径
- 已完成：`git status --short`
- 已完成：`rustfmt --edition 2024 --config skip_children=true src/app/types.rs src/app/mod.rs src/app/init.rs src/app/workspace.rs src/session/pane.rs src/session/mod.rs src/app/ui/layout.rs src/app/ui/tab_bar.rs src/app/ui/terminal_panel.rs src/app/ui/sftp_panel.rs src/app/app_menu.rs src/app/search.rs src/app/keybinding_recorder.rs src/session/config.rs`
- 已完成：`cargo check`
- 已完成：`cargo test`
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：GUI 手工验证编号标签、SFTP 页面切换和快捷键聚焦行为

## 风险与阻塞

- 阻塞：无
- 风险一：`activate_group()` 之前默认强制切回 Terminal，若遗漏切换点，SFTP 页面会被意外打回终端页
- 风险二：底部 monitoring 与原 SFTP 共用 body panel，若布局裁剪处理不干净，可能留下空白或错误的 panel state
- 风险三：关闭连接组后若当前页仍停留在 `WorkspacePage::Sftp`，需要同步回退到有效页面

## 下一步

- 用 GUI 验证 `ToggleSftpZoom` 在当前连接 SFTP 已存在时是否稳定聚焦到对应 `N SFTP` 标签
- 结合实际体验决定是否彻底删除 `session/config.rs` 中遗留的 `sftp_panel_minimized` 持久化字段，或保留一次兼容迁移

## 最后更新时间

- 2026-07-08 17:46 +0800
