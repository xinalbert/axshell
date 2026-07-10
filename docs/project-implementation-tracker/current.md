# 当前项目实施记录

## 当前目标

- 目标：把 SSH 登录阶段的网络重试改为可配置，并在设置页提供重试次数与延时配置
- 交付物：SSH 传输重试配置持久化；SSH/SFTP 共用重试 helper；设置页输入与文案；实施记录同步更新

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/backend/ssh/connection.rs`，`src/sftp/auth.rs`，`src/config/store.rs`，`src/app/dialogs/settings/terminal.rs`，`locales/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：会话级单独重试策略、认证失败重试、SSH 协议库升级、release/tag、提交

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 重试配置落点和默认值依据结论 | 已读取 SSH/SFTP 传输重试、配置 store、设置页和外部默认值资料 | 使用全局配置；SSH/SFTP 复用同一 helper；默认值先保持当前 2 次重试与 0.5s/1.5s 延时 |
| P2 | completed | 配置字段、重试 helper 和 SSH/SFTP 接线 | `rustfmt --edition 2024 src/backend/ssh.rs src/backend/ssh/connection.rs src/sftp/auth.rs src/config/store.rs src/app.rs src/app/lifecycle/init.rs src/app/dialogs/settings/terminal.rs` 通过 | 原硬编码 delay 已改为从 `ConfigStore` 读取，SFTP 复用同一 transport retry helper |
| P3 | completed | 设置页输入、保存逻辑和中英文文案 | `cargo check` 通过 | 设置放在 Terminal 页，使用 AxShell 常驻输入状态 |
| P4 | completed | 编译、测试、空白和 tracking 校验 | `cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator 均通过 | GUI 手工验证仍需用户实机确认 |

## 已完成

- 已读取环境记录、项目地图、当前实施记录和 SSH 网络重试实现
- 已确认当前 SSH 传输重试硬编码在 `src/backend/ssh/connection.rs`，为 0.5s / 1.5s 两次短退避
- 已确认 SFTP 连接链路目前没有同级别的 transport retry helper，可通过抽公共 helper 复用
- 已确认配置持久化与设置页现有模式足以承载“次数 + 延时列表/输入”这类全局选项
- 已完成外部核实，OpenSSH `ConnectionAttempts` 官方默认是 1 次尝试；本轮默认值将先保持当前产品行为，避免无意回退用户体验
- 已在 `ConfigStore` 中增加 SSH 重试次数和延时配置，并对异常输入做归一化
- 已让 SSH 和 SFTP 共用同一 transport retry helper，统一读取全局配置
- 已在 Terminal 设置页增加重试次数、延时输入和保存动作，并补齐中英文文案

## 验证

- 已完成：联网检索；本地源码定位；本轮 plan-first 记录初始化；`rustfmt --edition 2024 src/backend/ssh.rs src/backend/ssh/connection.rs src/sftp/auth.rs src/config/store.rs src/app.rs src/app/lifecycle/init.rs src/app/dialogs/settings/terminal.rs`；`cargo check`；`cargo test --quiet`，54 个测试通过；`git diff --check`；tracking docs validator
- 未完成：GUI 手工验证

## 风险与阻塞

- 风险一：若把延时输入设计得过于自由，需要额外处理空值、负值、非数字和超大值的归一化
- 风险二：SFTP 复用 transport retry 后，连接耗时会略增；需要保持默认值保守，避免对快速失败场景造成明显拖慢
- 风险三：当前未做会话级覆盖；若用户后续要求某些主机更激进或更保守，需要再扩展 schema

## 下一步

- 在真实 GUI 中确认 Terminal 设置页保存后，新 SSH/SFTP 连接会按配置显示重试进度，并检查 0 次重试和单延时输入等边界手感

## 最后更新时间

- 2026-07-10 10:57 +0800
