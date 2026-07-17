# 当前项目实施记录

## 当前目标

- 目标：修复终端工作区弹出并收回后 macOS Metal drawable 持续堆积的问题。
- 交付物：通过 AppKit 正常关闭 detached 窗口，释放其 CAMetalLayer 资源；完成 Rust 与 tracking 验证。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`Cargo.toml`、`Cargo.lock`、`src/app/actions/session.rs`、`docs/project-implementation-tracker/current.md`、`docs/project-implementation-tracker/changes/2026/07.md`。
- 不在本轮范围内：terminal 后端、scrollback 容量、窗口池、GPUI/Metal 上游依赖升级、发布工作流与用户文档。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 让 macOS detached 窗口通过 AppKit close 路径回收 Metal drawable | 定向代码审阅、`rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator | `vmmap` 已确认 18 个 2000x1400 CAMetalLayer drawable 约占 202.5 MB；终端 transfer 已先完成再关闭窗口 |

## 已完成

- 已通过 `sample` 和 `vmmap` 定位：不是 PTY 或终端 scrollback 的重复持有；当前进程存在约 18 个 2000x1400、每个 10.9 MB 的 `CAMetalLayer Display Drawable`，`IOSurface` 合计 202.5 MB。
- 已核对收回路径：workspace transfer 已先从 detached view 移出，随后仅调用 `Window::remove_window()`；可安全改为原生关闭请求。
- 已将 macOS 收回路径改为在 transfer 完成后 deferred `performClose:`；AppKit close callback 会移除 GPUI window，并沿正常原生生命周期释放 CAMetalLayer。非 macOS 或无法取得 AppKit handle 时仍回退 `remove_window()`。

## 验证

- 已完成：项目环境、项目地图、窗口转移/收回路径、GPUI macOS `MacWindow::Drop` 和当前进程 `vmmap` 审阅；`rustfmt --edition 2024 src/app/actions/session.rs`；`cargo check`；`cargo test --quiet`（225 passed）；`git diff --check`；tracking docs validator。
- 未完成：目标机循环弹出/收回的 `vmmap` 复测；需验证 IOSurface drawable 数量不再随操作次数增长。

## 风险与阻塞

- 仍需人工循环复测，因为 Rust 单元测试不能检查 macOS CAMetalLayer / IOSurface 释放。

## 下一步

- 在新构建中连续弹出/收回同一 terminal workspace，并通过 `vmmap <pid>` 对比 `CAMetalLayer Display Drawable` 数量与 `IOSurface` 常驻值。

## 最后更新时间

- 2026-07-17 16:59 +0800
