# 当前项目实施记录

## 当前目标

- 目标：继续拆分 `src/app/dialogs/` 目录模块，并将 `src/app/ui.rs` 拆分为 `src/app/ui/` 目录模块
- 交付物：`src/app/dialogs/` 按 dialog 类型拆分后的子模块、`src/app/dialogs/settings/` 低风险子模块、`src/app/ui/` 按 UI 区域拆分后的目录模块、更新后的项目地图和验证结果

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/dialogs.rs`，`src/app/dialogs/`，`src/app/ui.rs`，`src/app/ui/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：重写 settings 页面结构、拆成独立 Cargo crate、改 UI 行为、删除旧注释代码、GUI 手工回归

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 本轮环境预检和实施计划刷新 | tracking docs validator | 不联网、不使用多 agent |
| P2 | completed | `src/app/dialogs.rs` 拆分为 `src/app/dialogs/` 多文件目录模块 | `cargo check`，源码级路径检查 | 保持 `pub mod dialogs;` 外部路径不变 |
| P3 | completed | `src/app/dialogs/settings/` 低风险子模块和 `src/app/ui/` 目录模块拆分 | `cargo check`，源码级路径检查 | `settings` 只抽低耦合 helper/page；`ui` 按功能区拆分 |
| P4 | completed | 格式化、全仓编译/测试和收口验证 | `rustfmt`，`cargo check`，`cargo test` | GUI dialog 行为留作手工验证 |

## 已完成

- 已评估 `src/app/dialogs.rs` 适合迁移为目录模块，不适合做独立 Cargo 子包
- 已将 `src/app/dialogs.rs` 拆分为 `src/app/dialogs/mod.rs`、`ssh.rs`、`selector.rs`、`transfers.rs`、`delete_confirm.rs` 和 `settings/`
- 已将 Settings 页面的低耦合部分拆到 `settings/fonts.rs`、`settings/about.rs`、`settings/help.rs`、`settings/keybindings.rs`、`settings/sync.rs`、`settings/proxy.rs`
- 已确认 `src/app/ui.rs` 不适合合并到 dialogs/session 等模块，已拆分为 `src/app/ui/mod.rs`、`helpers.rs`、`layout.rs`、`monitoring.rs`、`sftp_panel.rs`、`sidebar.rs`、`tab_bar.rs`、`terminal_panel.rs`
- 已刷新项目地图，记录当前 dialogs 与 ui 子模块边界

## 验证

- 已完成：结构评估
- 已完成：施工前环境预检刷新
- 已完成：`rustfmt --edition 2024 src/app/ui/*.rs src/app/dialogs/*.rs src/app/dialogs/settings/*.rs`
- 已完成：`cargo check`
- 已完成：`cargo test`，15 个测试全部通过
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：GUI dialog 手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：路径迁移必须保持 `crate::app::dialogs` 模块名不变，否则外部调用会失效
- 风险二：本轮只改文件布局，不应夹带 UI 行为调整
- 风险三：GUI dialog 交互仍需用户手工验证
- 风险四：`src/app/dialogs/settings/mod.rs` 仍有约 942 行，主要剩余 General 和 Custom 两个高耦合页面；后续若继续拆需要先设计页面上下文结构体，避免大量闭包参数散落
- 风险五：`src/app/ui/sftp_panel.rs` 仍有约 1025 行，`src/app/ui/sidebar.rs` 仍有约 925 行；后续可继续把列表行、context menu 和工具栏构造抽成更小模块

## 下一步

- 后续可继续把 `settings/mod.rs` 的 General 和 Custom 页面按上下文结构体拆分
- 后续可继续拆 `src/app/ui/sftp_panel.rs` 的远端列表、本地列表、传输摘要和 context menu
- 后续可继续拆 `src/app/ui/sidebar.rs` 的 expanded/collapsed saved entry 渲染

## 最后更新时间

- 2026-07-08 12:40 CST
