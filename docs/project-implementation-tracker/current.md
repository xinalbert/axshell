# 当前项目实施记录

## 当前目标

- 目标：将已确认的重复逻辑收敛到公共 helper 和常量中，降低 SSH / SFTP / About URL 维护成本
- 交付物：共享 SSH 私钥 helper、统一 app 仓库 URL 常量、SFTP 传输状态发送 helper、格式化/编译/测试验证结果，以及同步的 tracking 记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/backend/`，`src/sftp/`，`src/app/constants.rs`，`src/app/dialogs.rs`，`src/app/startup.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：UI render 闭包大拆分、session/tab 生命周期合并、README / release workflow 仓库 URL 批量迁移、GUI 手工回归

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 本轮 plan、环境预检和项目地图范围更新 | tracking docs validator | 已完成 plan-first 文档和地图刷新 |
| P2 | completed | 共享 SSH key 解析与 public key 算法 helper | `cargo check`，相关源码 diff 检查 | 未合并 SSH / SFTP 连接主流程 |
| P3 | completed | 统一 app 仓库 URL / issues URL 常量 | `rg -n 'github.com/xinalbert' src/app`，`cargo check` | 只统一代码中的 app URL，不做文档批量迁移 |
| P4 | completed | SFTP 状态和传输进度发送 helper | `cargo check`，源码级行为检查 | 只抽小 helper，不抽上传/下载大流程 |
| P5 | completed | 格式化、编译、测试和跟踪文档校验收口 | `rustfmt`，`cargo check`，`cargo test`，tracking docs validator | GUI 手工验证不在本轮自动执行范围 |

## 已完成

- 已完成重复点评估，确认 SSH 私钥 helper 和 URL 常量属于低风险高收益抽取点
- 已确认本轮不需要联网，不使用多 agent
- 已新增 `src/backend/auth.rs`，让 SSH 终端和 SFTP 共用 private key 解析与 RSA public key hash fallback helper
- 已将 app 内仓库 URL / issues URL 统一到 `src/app/constants.rs`
- 已在 `src/sftp/mod.rs` 中抽出 SFTP status、transfer progress 和 transfer error 发送 helper

## 验证

- 已完成：源码热点只读评估
- 已完成：施工前环境预检刷新
- 已完成：`rustfmt --edition 2024 src/backend/auth.rs src/backend/mod.rs src/backend/ssh.rs src/sftp/auth.rs src/sftp/path.rs src/sftp/mod.rs src/app/constants.rs src/app/dialogs.rs src/app/startup.rs`
- 已完成：`rg -n "load_session_private_key|private_keys_with_algs|normalize_inline_private_key|expand_key_path|FEEDBACK_ISSUES_URL|github.com/xinalbert" src/backend src/sftp src/app`
- 已完成：`cargo check`
- 已完成：`cargo test`，13 个测试全部通过
- 已完成：`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 未完成：GUI / 真实 SSH / SFTP 联机手工验证

## 风险与阻塞

- 阻塞：无
- 风险一：SSH key helper 需要保持 `russh` RSA hash fallback 顺序不变，否则可能影响老服务器 key auth
- 风险二：SFTP 事件 helper 只应收敛重复发送逻辑，不应改变取消、失败和完成状态语义
- 风险三：GUI / 真实 SSH / SFTP 联机行为仍需用户后续手工验证
- 风险四：仍保留既有 `block v0.1.6` future-incompat warning，来源于传递依赖

## 下一步

- 如需进一步公共化，可另起任务拆 `drain_backend_events` 或统一字节格式化策略

## 最后更新时间

- 2026-07-08 11:12 CST
