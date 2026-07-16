# 当前项目实施记录

## 当前目标

- 目标：完善保存 SSH 会话的快捷键、悬浮信息、无凭据 JSON 复制/导入与菜单“关于”入口。
- 交付物：会话快捷键、tooltip 配置字段、剪贴板 JSON 往返、分组导入兼容、菜单栏“关于”直达与自动化验证。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/session.rs`、`src/app.rs`、`src/app/actions/saved_sessions.rs`、`src/app/actions/session.rs`、`src/app/actions/terminal.rs`、`src/app/dialogs/ssh.rs`、`src/app/dialogs/settings.rs`、`src/app/input/`、`src/app/views/`、`src/app/workspace.rs`、`src/main.rs`、`locales/`、`docs/features/`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：`Cargo.toml` / `Cargo.lock`、SSH/SFTP 协议、分享 JSON schema、密码、私钥、passphrase 或代理密码导出。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 确认分享 JSON、右键复制和 SSH 表单路径 | 源码路径审查 | 复用既有无凭据 schema，不改文件导入/导出格式 |
| P2 | completed | 会话快捷键、tooltip、JSON 剪贴板往返、分组导入和菜单“关于” | 聚焦单元测试、`cargo check` | 单条剪贴板导入保留编辑中的本机凭据与快捷键；分组沿用标准分享格式 |
| P3 | completed | 完整回归与追踪记录收口 | `rustfmt`、`cargo test --quiet`、validator | GUI 剪贴板、tooltip 与菜单跳转保留手工验收 |

## 已完成

- 已确认单条 JSON 导出已复用 `ax-shell-saved-sessions` schema，并排除密码、私钥、passphrase、代理密码和本机快捷键。
- 已确认现有右键“复制信息”只复制三行文本；SSH 新建和编辑共用一个表单，适合增加不自动保存的剪贴板导入动作。
- 已确定剪贴板导入只接受恰好一个有效 JSON 会话；编辑已有会话时保留当前 ID、本机快捷键及未导出的凭据。
- 已实现会话快捷键录制、冲突检查与终端/SFTP 工作区连接；侧栏 hover 展示快捷键和会话 SFTP 路径。
- 已将右键复制改为标准无凭据单会话 JSON，并在 SSH 表单加入“从剪贴板导入”；分组导出文件可直接通过现有文件导入恢复全部会话及其组名。
- 已在原生应用菜单增加 About AxShell，并使其切换到设置页中的“关于”分页。

## 验证

- 已完成：环境与实施记录、项目地图、分享 schema、右键菜单、SSH 表单和剪贴板 API 路径审查；`rustfmt`；分享 JSON 聚焦测试 5 项；会话快捷键聚焦测试 3 项；`cargo check`；完整 `cargo test --quiet`（207 项）；`git diff --check`；hover 静态审计。
- 未完成：真实 GUI 中的快捷键录制/连接、展开与折叠侧栏 tooltip、剪贴板 JSON 往返、分组文件导入和菜单栏 About 跳转验收。

## 风险与阻塞

- 无阻塞。

## 下一步

- 在桌面 GUI 中完成上述交互验收。

## 最后更新时间

- 2026-07-16 11:30 +0800
