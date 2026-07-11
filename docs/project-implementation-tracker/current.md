# 当前项目实施记录

## 当前目标

- 目标：根据日志覆盖审查建立可靠、结构化、可脱敏的全应用诊断路径，使关键失败在 UI 状态之外同时进入持久日志。
- 交付物：可靠日志 writer 和轮转、诊断脱敏 helper、SFTP/同步/本地 PTY/监控/配置保存日志覆盖、敏感字段收敛、单元测试和完整验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/main.rs`，`src/diagnostics.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/config_sync.rs`，`src/backend/local.rs`，`src/backend/ssh/connection.rs`，`src/config/store.rs`，`src/sftp/`，相关 app 设置/视图保存调用点，跟踪文档。
- 不在本轮范围内：记录终端输入输出内容、采集密码/密钥/token、引入远程 telemetry、修改业务协议或配置 schema、增加依赖、修改 manifest/lock/release/tag。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 日志覆盖审查、环境预检、风险分级和项目地图刷新 | 基线 `cargo check`、`git diff --check`、日志调用统计 | 103 个 Rust 文件中 22 个有日志调用 |
| P2 | completed | 非静默 writer、稳定时间桶、非丢失缓冲和脱敏 helper | 日志轮转/脱敏单元测试、`cargo check` | 不记录凭据与终端内容 |
| P3 | completed | SFTP、同步、本地 PTY、监控和 backend 关闭错误日志 | 定向测试、调用点复核、`cargo check` | UI 事件保留，同时增加持久日志 |
| P4 | completed | 配置保存统一日志入口并替换静默 `save()` | 配置测试、静默保存检索、`cargo check` | 不改变保存时机或 UI 行为 |
| P5 | completed | SSH/SFTP 现有日志结构化和敏感主机/用户/路径脱敏 | 脱敏测试、敏感日志检索、`cargo check` | 保留诊断上下文但不写完整标识 |
| P6 | completed | 项目地图、完整回归和文档收口 | `cargo test --quiet`、`git diff --check`、tracking validator | GUI/真实远端服务保留手工验证 |

## 已完成

- 已完成全仓日志调用、writer、轮转、事件错误传播、配置保存和敏感字段审查。
- 已确认日志基础覆盖启动、崩溃、SSH/X11、部分配置和部分 SFTP，但 SFTP 顶层、同步、本地 PTY、监控不可用和大量设置保存只进入 UI 或被静默忽略。
- 已确认 `tracing_appender::non_blocking` 默认使用 lossy 缓冲，当前 `LocalMinutelyRoller::write` 又会忽略文件打开失败并返回成功。
- 已确认 SSH 日志包含完整用户名/主机/私钥路径，SFTP 日志包含完整本地和远程路径，需要先建立统一脱敏 helper。
- 已完成施工前环境预检；基线 `cargo check` 和 `git diff --check` 通过，工作树干净，不需要联网、多 agent 或新增依赖。
- 已将 runtime log 改为完整小时桶并保留 168 个小时文件，修复间隔整小时但分钟相同不轮转的问题。
- 已让日志目录/文件初始化失败回退到 stderr，writer 的 rollover/write/flush 错误不再伪装为成功；non-blocking buffer 改为无损模式。
- 已让 `WorkerGuard` 由 `main` 持有到应用退出，避免泄漏 guard 和跳过正常 flush。
- 已新增 `src/diagnostics.rs`，集中提供用户、主机和路径脱敏，并把 saved session 展示复用到该 helper。
- 已完成 4 项 diagnostics 测试、2 项日志桶/保留测试和阶段 `cargo check`，无新增源码告警。
- 已为 SFTP worker 顶层失败、目录浏览/分页/reveal/预览、远程编辑 watcher、自动上传、创建目录、批量删除和 transfer 失败增加结构化错误日志。
- 已为 WebDAV/S3 上传下载增加 backend/operation/session_count 日志，为同步设置和结果持久化失败增加错误日志，不记录 endpoint 或凭据。
- 已为本地 PTY 启动、读写、flush、resize、wait、进程退出、kill 和线程 panic 增加日志，并为初次/重开失败补齐 action 边界日志。
- 已为远端监控不可用增加结构化 warning；错误文本统一单行化、路径脱敏并限制为 512 字符。
- 阶段 `cargo check`、14 项 SFTP 测试、1 项 local backend 测试和 5 项 sync 测试通过，无新增源码告警。
- 已新增 `ConfigStore::save_logged(operation)`，替换设置、布局、侧栏、监控、代理、SFTP 偏好和会话动作中静默忽略的配置保存错误；静默 `config.save()` 检索无剩余命中。
- 已将 SSH terminal、connection、X11 和 SFTP auth/worker 日志改为结构化字段；用户名、主机、私钥路径、代理标识和错误链中的已知敏感值均在写日志前清洗。
- 已补齐 SSH 顶层 terminal、shutdown、write、resize、CWD、环境变量、disconnect、close 和 X11 relay 日志，保持终端输入输出内容不进入日志。
- 最近一轮 `rustfmt`、`cargo check` 和 `git diff --check` 通过；仅保留既有 `block v0.1.6` future-incompat warning。
- 已完成 4 项 diagnostics、2 项 logging、8 项 backend、14 项 SFTP、11 项 config、5 项 sync 定向测试；`cargo test --quiet` 92 项全部通过。
- tracking docs validator 通过；旧式 SSH/SFTP 日志、完整环境代理字段、完整迁移路径和静默配置保存检索无剩余命中；`Cargo.toml` / `Cargo.lock` 未修改。

## 验证

- 已完成：日志调用分布、writer/轮转、核心错误路径、配置保存、SSH/SFTP/X11 脱敏、格式化、编译、定向测试、92 项完整回归、空白检查和 tracking validator。
- 未完成：真实 GUI、SSH/SFTP/X11、WebDAV/S3 和日志目录不可写故障注入手工验证。

## 风险与阻塞

- 风险一：日志覆盖不能演变成记录终端内容、密码、私钥、token 或完整用户路径。
- 风险二：同一错误在生产者和事件边界重复记录会产生噪声；按“底层失败记录一次，UI 事件只在缺少底层日志时补记”的规则实施。
- 风险三：把 non-blocking writer 改为无损缓冲可能在极端日志洪峰时施加背压；本应用不记录终端输出，并将高频状态维持在 debug 或不记录。
- 风险四：结构化日志已覆盖关键失败，但真实 SSH/SFTP/X11、同步服务和日志目录不可写场景仍需要手工触发确认。
- 无阻塞。

## 下一步

- 启动应用确认小时日志文件创建；分别触发本地终端、SSH/SFTP、同步和监控失败，并检查日志字段与脱敏结果；临时让日志目录不可写以确认 stderr fallback。

## 最后更新时间

- 2026-07-11 08:40 +0800
