# 当前项目实施记录

## 当前目标

- 目标：关闭 terminal tab 时回收按 tab ID 保存的 UI 状态，避免缓存增长、失效 pane 命中和连接进度残留。
- 交付物：统一的 tab UI 状态回收 helper、覆盖三类 tab 关闭路径的接线和实施记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/actions/session.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：终端后端关闭语义、pane 布局、SFTP worker、全局连接状态模型、release/tag、提交。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | tab-ID UI 状态与关闭路径定位 | 已读取 close、mouse hit-test、IME、hover 和 progress 链路 | 三类关闭路径收敛到 `handle_tab_close()` |
| P2 | completed | 统一状态回收 helper 与关闭路径接线 | `rustfmt --edition 2024`、`cargo check` | 清理 map 和匹配 tab 的短期状态，不改 backend 关闭 |
| P3 | completed | 回归验证和实施记录收口 | `cargo test --quiet`、`git diff --check`、tracking validator | GUI 手工验证关闭连接中/IME/hover tab |

## 已完成

- 已确认 `terminal_scrollbars` 和 `terminal_bounds` 以 tab ID 为 key，关闭路径未删除对应条目。
- 已确认 `terminal_bounds` 直接参与 `focus_terminal()` pane 命中，残留条目可使点击定位到已关闭 ID。
- 已确认 `hovered_url`、`terminal_composition`、`terminal_frozen_selection` 和 `connection_progress` 都可携带 tab ID，适合在统一入口按 ID 回收。
- 已在 `handle_tab_close()` 内增加 `clear_tab_ui_state()`，并由 fallback、单 tab 和整组关闭路径调用。
- 已回收 scrollbar/bounds、hover、IME、冻结选区、连接进度和已关闭 tab 的搜索结果；只在关闭 active tab 时复位 `terminal_selecting`。

## 验证

- 已完成：环境预检、项目地图、tab 关闭、终端 pane 鼠标命中和短期 UI 状态调用链定位；`rustfmt --edition 2024 src/app/actions/session.rs`、`cargo check`、`cargo test --quiet`（70 项）、`git diff --check` 和 tracking docs validator。
- 未完成：GUI 手工验证关闭连接中、悬浮链接、IME 预编辑或选择中的 tab。

## 风险与阻塞

- 风险一：`terminal_selecting` 没有 tab ID，只能在关闭当前 active tab 时清除，避免错误终止其他 pane 的选择。
- 风险二：整组关闭要对 group 中每个 tab 清理，不能只清理触发关闭的 tab。
- 无阻塞：GUI 交互行为需在桌面端人工确认。

## 下一步

- 在 GUI 中关闭连接中、悬浮链接、IME 预编辑或选择中的 tab，确认无残留 overlay、选区或 pane 命中。

## 最后更新时间

- 2026-07-10 15:01 +0800
