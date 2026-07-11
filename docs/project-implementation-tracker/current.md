# 当前项目实施记录

## 当前目标

- 目标：修正 Appearance 页普通 Theme profile 选择只更新标签/配置而窗口主题未可靠切换的问题。
- 交付物：内置/导入 profile 直接应用其 registry light/dark 配置对；Custom profile 仍走动态构造路径。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/theme.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：主题 JSON 内容、主题 profile schema、Settings 页面布局、外部 `gpui-component` 源码、依赖与 `Cargo.toml` / `Cargo.lock`。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 复核截图对应的 profile 选择链路与已保存配置 | 源码与脱敏 theme 配置字段复核 | `matrix` profile 和 Matrix Light/Matrix 名称已正确写入，问题不在保存路径 |
| P2 | completed | 普通 profile 直接应用明确的 registry 配置对 | `rustfmt --edition 2024 src/app/theme.rs`，`cargo check`，`cargo test --quiet`，`cargo build` | 防止 legacy custom draft 参与普通预设解析 |
| P3 | completed | 回归验证与收口记录 | `git diff --check`，tracking docs validator | 需以新构建进程手工切换多个预设确认 |

## 已完成

- 已读取 `AGENTS.md`、环境记录、实施记录、项目地图和相关 skills。
- 已确认截图中的 Theme Mode 是 Follow System，Matrix profile 的配置保存值为 `Matrix Light` / `Matrix`，并非保存路径失效。
- 已确认 Matrix、Tokyo、Gruvbox、Solarized 的内置 JSON 都含不同的实际色调值。
- 已确认普通 profile 目前复用含 custom fallback 的 `resolve_selected_theme()`；本轮将让普通 profile 直接解析对应 registry 名称，Custom profile 继续动态生成。
- 已确认 project map 已覆盖 `src/app/theme.rs`，本轮不改变模块结构。
- 已抽取 registry-only 解析 helper；`apply_theme_profile()` 对普通 profile 直接安装其明确的 light/dark 配置对，Custom profile 保留动态生成。
- 已执行 `rustfmt --edition 2024 src/app/theme.rs`、`cargo check`、`cargo test --quiet` 和 `cargo build`；完整单测 110 项通过。
- 已执行 `git diff --check` 和 tracking docs validator，均通过。

## 验证

- 已完成：profile 保存、内置主题资源、普通/Custom 应用链路和系统模式同步复核、Rust 格式化、编译、完整单测、debug 构建、空白检查和 tracking docs validator。
- 未完成：新构建进程的 GUI 预设切换确认。

## 风险与阻塞

- 风险：Custom profile 需要继续使用其保存的动态 draft；不能把所有 profile 一律改为 registry 查找。
- 无阻塞。

## 下一步

- 关闭旧进程后启动新 debug 构建，连续切换 Matrix、Tokyo Night 和 Gruvbox 确认立即生效。

## 最后更新时间

- 2026-07-11 23:18 +0800
