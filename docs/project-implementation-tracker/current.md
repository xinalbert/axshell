# 当前项目实施记录

## 当前目标

- 目标：在 SSH 会话既无密码也无私钥时，不先用空密码尝试连接，而是直接打开终端 tab 输入密码并用该密码连接。
- 交付物：打开 SSH tab 前判断密码和私钥是否都为空；满足条件时使用 inactive backend 创建未连接 tab 并显示 `Password: `；终端输入期间本地截获密码、回显掩码并回车连接；私钥认证或已填写密码失败时保留现有连接失败 overlay。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app.rs`，`src/app/session_ui.rs`，`src/app/actions/session.rs`，`src/app/actions/terminal.rs`，`src/app/workspace.rs`，`src/app/lifecycle/event_loop.rs`，`src/terminal/backend.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 不在本轮范围内：修改 `russh` 认证协议、交互式 keyboard-interactive 认证、多轮服务器 challenge、保存用户临时输入的密码到配置、修改配置 schema、修改 `Cargo.toml` / `Cargo.lock`、真实 GUI 交互验收。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 复核 SSH 认证、连接进度 overlay、tab 创建和终端输入路径 | 读取 `connection.rs`、`event_loop.rs`、`workspace.rs`、`session.rs`、`terminal.rs` | 确认可以在 open tab 前用 inactive backend 承载本地密码提示 |
| P2 | completed | 新增无密码无私钥时的终端内密码提示和临时密码连接 | `rustfmt --edition 2024`，`cargo check`，聚焦测试 | 打开 tab 前直接提示；输入密码后再启动 SSH backend；临时密码不写入配置 |
| P3 | completed | 补齐完整测试和 tracking docs 校验 | 完整 `cargo test --quiet`，`git diff --check`，tracking validator | GUI 实际密码输入体验仍需手工确认 |

## 已完成

- 已读取 `AGENTS.md`、环境记录、当前实施记录和项目地图。
- 已确认 SSH 密码认证当前由 `src/backend/ssh/connection.rs` 调用 `authenticate_password`，失败时只返回错误，不会打开 shell 或产生可直接输入的远端 password prompt。
- 已确认连接失败事件在 `src/app/lifecycle/event_loop.rs` 中把 `connection_progress` 标记为 failed，`src/app/views/layout.rs` 显示 Retry / Cancel overlay。
- 已确认终端输入统一经过 `src/app/actions/terminal.rs`，可以在 app 层对指定 tab 临时截获输入。
- 已按用户约束收窄触发条件：`AuthMethod::Password`、`session.password` 为空、`private_key_path` 和 `private_key_inline` 都为空；私钥认证或已填密码失败时保留现有 overlay。
- 已新增 inactive backend，用于创建尚未发起网络连接的 SSH tab；用户在终端输入密码并回车后才替换为真实 SSH backend。
- 已新增终端密码提示状态，输入期间截获普通字符、Backspace、粘贴、IME commit、Enter 和 Ctrl-C；只回显掩码，不把密码发到旧 backend。
- 已实现终端输入密码后的临时重连；临时密码只写入当前 tab 的 `Session` 副本，不保存到配置。
- 已保留“终端输入的密码被服务器拒绝后再次提示”的路径；非认证失败仍进入现有连接失败 overlay。
- 已确认项目地图覆盖本轮相关文件，不需要刷新 `project-map.md`。

## 验证

- 已完成：相关源码路径复核；确认不需要联网、不使用多 agent、不新增依赖、不修改配置 schema；终端密码提示实现；`rustfmt --edition 2024`；`cargo check`；新增聚焦测试；完整 `cargo test --quiet`；`git diff --check`；tracking validator。
- 未完成：真实 GUI 终端密码输入手工确认。

## 风险与阻塞

- 风险：这是 app 层本地密码提示，不是 SSH keyboard-interactive 协议实现；适合“无密码、无私钥”的普通 password auth 首次输入。真实焦点、掩码回显和连接中 overlay 仍需 GUI 手工确认。
- 无阻塞。

## 下一步

- 按 git hygiene 准备提交；随后在真实 GUI 中确认无密码无私钥 SSH 会话打开 tab 后直接显示 `Password: `，输入错误密码可再次提示，输入正确密码可连接。

## 最后更新时间

- 2026-07-12 11:32 +0800
