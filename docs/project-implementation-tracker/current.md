# 当前项目实施记录

## 当前目标

- 目标：降低终端动态等待输出时的无效整屏刷新，先完成第一阶段的内容比较刷新和选区期节流
- 交付物：`src/terminal/mod.rs` 中的 viewport 签名缓存、`src/app/event_loop.rs` 中的终端刷新节流、`src/app/mod.rs` / `src/app/init.rs` 中新增的刷新状态字段，以及更新后的实施/环境记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal/mod.rs`，`src/app/event_loop.rs`，`src/app/mod.rs`，`src/app/init.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：行级 layout cache、局部 dirty-rect 绘制、GPUI 渲染模型重构、GUI 手工回归

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 本轮环境预检、项目地图复核和实施计划刷新 | tracking docs validator | 当前工作树干净，可直接进入第一阶段实现 |
| P2 | completed | `TerminalTab` viewport 签名比较 | `cargo check`，`cargo test` | 只比较当前 viewport，不做 row cache |
| P3 | completed | 选区期终端刷新节流与 blink/空闲刷新避让 | `cargo check`，`cargo test` | 选区存在时，终端输出刷新最多每 100ms 触发一次 |
| P4 | completed | 文档收口与最终验证 | `rustfmt`，`cargo check`，`cargo test`，tracking docs validator | GUI 选区体验仍留作手工验证 |

## 已完成

- 已确认当前问题的主因是终端区域在动态等待输出时仍频繁整块重绘，而不是输出路径主动清 selection
- 已在 `TerminalTab` 增加 viewport 签名缓存，并在输出、resize、scroll 后只根据可见内容变化决定是否需要终端刷新
- 已在事件泵中把“终端刷新”与“其他 UI 变化”拆开，避免 SFTP、状态栏等普通界面更新被终端节流拖慢
- 已在选区存在时暂停 blink 驱动刷新，并将待刷新的终端输出节流为最多每 100ms 一次
- 已保留无选区时的即时刷新路径，避免常规终端交互明显变钝

## 验证

- 已完成：源码级确认终端输出刷新、blink 和空闲 notify 的触发路径
- 已完成：`rustfmt --edition 2024 src/terminal/mod.rs src/app/mod.rs src/app/init.rs src/app/event_loop.rs`
- 已完成：`cargo check`
- 已完成：`cargo test`，18 个测试全部通过
- 已完成：tracking docs validator
- 未完成：GUI 终端选区与动态输出体验手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：viewport 签名当前按整个可见网格 hash，能减少无意义刷新，但不会带来真正的局部重绘
- 风险二：选区期 100ms 节流会让超高频 spinner 的更新略慢一点，但这是为换取选区稳定性做的显式权衡
- 风险三：若后续仍存在明显闪烁，下一阶段需要上 row hash + row layout cache，改动面会明显变大

## 下一步

- 可在 GUI 中重点验证：动态输出时选区是否更稳定、无选区时终端是否仍保持及时刷新
- 若第一阶段收益仍不够，再进入 row hash + row layout cache 的第二阶段

## 最后更新时间

- 2026-07-08 15:46 +0800
