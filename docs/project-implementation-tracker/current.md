# 当前项目实施记录

## 当前目标

- 目标：修复从 terminal 明确路径打开 SFTP 时的重复跳转，并明确远端初始目录优先级。
- 交付物：显式路径直达、无显式路径时会话预设路径、无预设路径时服务器 home，以及覆盖该决策的回归测试与验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/actions/sftp.rs`、`src/app/actions/session.rs`、`src/app/sftp.rs`、`src/app/lifecycle/event_loop.rs`、`src/config/model.rs`、`src/config/store.rs`、`src/sftp.rs`、`src/sftp/worker.rs`、`src/sftp/worker/runtime.rs`、`docs/features/`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：`Cargo.toml` / `Cargo.lock`、SSH/SFTP 认证和传输协议、终端 CWD 采集、配置 schema 与本地目录逻辑。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 定位 terminal 路径链接与 SFTP worker 初始列举链 | 源码路径审查 | 明确目标在 worker 已启动前会先按默认路径列举、再发送 `RevealPath` |
| P2 | completed | 分离显式定位、会话预设和 home 回退的启动意图 | 聚焦单元测试 | 显式路径以首次 worker 请求直接定位；普通新页面不再使用上次目录作为初始路径 |
| P3 | completed | 格式化、构建、完整回归和追踪记录收口 | `rustfmt`、`cargo check`、`cargo test --quiet`、validator | 保留真实 SSH/SFTP GUI 手工验收 |

## 已完成

- 已确认 terminal 中的路径链接由 `open_sftp_and_reveal_path` 打开 SFTP；它创建 worker 后再额外发送 `RevealPath`，因此首次显示默认目录后才到目标路径。
- 已以 `Browse` 和 `Reveal` 两种初始意图传给 worker：`Reveal` 会在首次列举前解析文件/目录目标，避免 home 或预设目录的中间列举。
- 已实现初始路径优先级：terminal 路径链接、远端地址栏输入和“打开终端当前目录”的明确路径最高；无明确路径时使用会话 `sftp_path`；无会话预设时使用服务器 home。已打开页面在 worker 回收或连接失效后仍恢复当前目录，包含根目录 `/`。
- 已删除不再参与决策的 `last_remote_sftp_paths` 配置；旧配置可正常读取并会在下次保存时自动丢弃该废弃字段。

## 验证

- 已完成：环境与实施记录、项目地图、terminal 路径链接、SFTP action、worker、配置迁移和事件消费路径审查；受影响 Rust 文件 `rustfmt`；初始路径、旧配置迁移和 reveal 聚焦测试；`cargo check`；完整 `cargo test --quiet`（208 项）；`git diff --check`；tracking docs validator。
- 未完成：真实 SSH/SFTP GUI 验收，确认 terminal 绝对目录/文件路径、地址栏、终端当前目录和无路径打开均无中间跳转。

## 风险与阻塞

- 无阻塞；显式路径指向文件时继续列出其父目录并选中该文件。

## 下一步

- 在真实 SSH/SFTP 服务器上确认 terminal 绝对目录/文件路径、地址栏、终端当前目录均不会出现 home/预设目录闪现，以及无路径打开按会话预设或服务器 home 进入。

## 最后更新时间

- 2026-07-16 10:23 +0800
