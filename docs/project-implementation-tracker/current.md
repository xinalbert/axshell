# 当前项目实施记录

## 当前目标

- 目标：修复终端滚动查看历史输出时，某些程序输出的 ANSI 背景色块可能在内容刷新后残留的问题
- 交付物：`src/terminal/element.rs` 中终端绘制区域每帧背景清理；环境记录和月度实施记录同步更新

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal/element.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：alacritty buffer 语义、PTY 后端、滚动输入策略、主题系统、终端搜索高亮策略、release/tag、提交

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 终端背景残留链路定位结论 | 本地源码检索和 `src/terminal/element.rs` / `src/terminal.rs` 读取完成 | 当前 paint 只叠加显式背景色块和文字，没有先清理终端 bounds |
| P2 | completed | 每帧清理终端绘制区域的实现 | `rustfmt --edition 2024 src/terminal/element.rs` 通过 | 先填充主题背景，再绘制 ANSI 背景色块和文字 |
| P3 | completed | 编译、测试与空白检查结果 | `cargo check`，`cargo test --quiet`，`git diff --check` 均通过 | GUI 历史滚动验证保留为风险 |
| P4 | completed | tracking docs 校验结果 | `python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .` 通过 | 收口前执行 |

## 已完成

- 已完成施工前环境预检，并确认本轮不新增依赖、不改配置 schema、不联网、不使用多 agent
- 已确认 `TerminalTab::render_snapshot()` 会按当前 `display_offset` 生成 viewport cell，并把 ANSI 背景色保存在 `RenderCell.cell.bg`
- 已确认 `TerminalElement::layout_grid()` 会把非默认背景转换为 `LayoutRect`，但 `TerminalElement::paint()` 开始时没有显式填充整个终端绘制区域背景
- 已修改 `TerminalElement::paint()`，每帧先用当前主题背景填充整个终端元素 bounds，再绘制本帧 ANSI 背景色块、文字、下划线、custom block、IME composition 和光标

## 验证

- 已完成：环境记录读取；项目地图读取；终端背景残留链路定位；本轮 plan-first 记录初始化；`rustfmt --edition 2024 src/terminal/element.rs`；`cargo check`；`cargo test --quiet`，50 个测试通过；`git diff --check`；tracking docs validator
- 未完成：GUI 历史滚动手工验证

## 风险与阻塞

- 风险一：GUI 手工验证需要真实运行会输出 ANSI 背景色的程序，并反复滚动历史确认旧色块不再残留
- 风险二：本轮只修复绘制层残留，不改变终端程序输出的真实 ANSI 背景语义

## 下一步

- 在真实 GUI 中运行会输出 ANSI 背景色的程序，反复滚动历史输出，确认旧色块不再残留

## 最后更新时间

- 2026-07-10 08:53 +0800
