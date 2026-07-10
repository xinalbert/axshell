# 当前项目实施记录

## 当前目标

- 目标：为终端右侧滚动条预留独立内容区，并让 Windows/Linux 平台菜单始终独占应用全宽顶部。
- 交付物：固定的 terminal scrollbar gutter、重组后的根菜单/工作区布局和实施记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/views/terminal_panel.rs`，`src/app/views/layout.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：终端字体/颜色、滚动行为、原生菜单 action、macOS 集成标题栏、SFTP 布局、release/tag、提交。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 终端滚动条和平台菜单的布局根因定位 | GPUI `Scrollbar` 源码与现有 pane/root layout 已核对 | scrollbar 为右侧绝对覆盖层 |
| P2 | completed | terminal scrollbar gutter 与全宽平台菜单布局 | `rustfmt --edition 2024`、`cargo check` | 保持终端 scroll handle 与菜单 action 不变 |
| P3 | completed | 回归验证和实施记录收口 | `cargo test --quiet`、`git diff --check`、tracking validator | GUI 手工验证长行与侧栏状态 |

## 已完成

- 已确认 terminal `vertical_scrollbar` 的 16px 轨道为绝对覆盖层，现有 pane 缺少对应右侧安全区。
- 已确认 Windows/Linux 菜单在展开/收起侧栏的主列内重复渲染，而不是根布局的独立行。
- 已将 terminal pane 与 16px scrollbar gutter 分为独立布局列，避免滚动轨道覆盖长行末尾内容。
- 已将 Windows/Linux 平台菜单提升为根布局的独立全宽行，侧栏开合不再改变其起点和宽度。

## 验证

- 已完成：环境预检、项目地图、terminal pane / root layout / GPUI scrollbar 的调用与渲染定位；`rustfmt --edition 2024`、`cargo check`、`cargo test --quiet`（70 项）、`git diff --check` 和 tracking docs validator。
- 未完成：Windows/Linux GUI 手工验证长行末尾、滚动条拖动和侧栏展开/收起状态。

## 风险与阻塞

- 风险一：右侧 gutter 会少一列或部分可用宽度；这是避免文字落在 16px 滚动轨道下的必要取舍。
- 风险二：菜单栏迁移必须只在 Windows/Linux Native 标题栏生效，不能影响 macOS Integrated 标题栏。
- 无阻塞：GUI 交互行为需在桌面端人工确认。

## 下一步

- 在 Windows/Linux GUI 中验证终端长行、滚动条拖动和侧栏展开/收起后的菜单位置。

## 最后更新时间

- 2026-07-10 14:37 +0800
