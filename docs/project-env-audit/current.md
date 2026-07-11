# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮根据日志覆盖审查，逐步补齐日志写入可靠性、核心错误路径、结构化字段和敏感信息脱敏。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取并刷新为“全软件日志覆盖与诊断可靠性”任务语境。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、`tracing`、`tracing-subscriber`、`tracing-appender`。
- 版本约束：仓库声明 `rust-version = 1.88.0`、edition `2024`；本机 `rustc 1.96.1`、`cargo 1.96.1`。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app/lifecycle/startup.rs`
- 本轮代码入口：`src/main.rs`，`src/diagnostics.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/config_sync.rs`，`src/backend/local.rs`，`src/backend/ssh/connection.rs`，`src/config/store.rs`，`src/sftp/`，相关设置与视图保存调用点。
- 依赖策略：复用现有 `tracing` 依赖，不修改 `Cargo.toml` / `Cargo.lock`，不新增日志或脱敏依赖。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、tracking docs validator。
- 默认测试命令：`cargo check`，`cargo test --quiet`
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立 test job。
- 当前实施验证命令：对全部变更 Rust 文件执行 `rustfmt --edition 2024`，运行日志轮转/脱敏定向测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 外部依赖：无；静态与单元测试不需要真实 SSH/SFTP/WebDAV/S3/X11 服务，真实日志目录和 GUI 诊断流程保留手工验证。
- 证据文件：`AGENTS.md`，`Cargo.toml`，`Cargo.lock`，`.github/workflows/ci.yml`，`src/app/lifecycle/startup.rs`，`src/events.rs`，`src/app/lifecycle/event_loop.rs`，`src/backend/`，`src/sftp/`，`src/sync.rs`，`src/config/store.rs`。

## 环境变化检查

- 是否发现变化：否
- 变化摘要：运行时、依赖和工具链不变；当前 103 个 Rust 文件中 22 个包含日志调用，已完成 writer、SFTP/同步/本地 PTY/监控、配置保存和 SSH/SFTP/X11 敏感日志缺口修复。
- 受影响文件：日志初始化、诊断 helper、backend/SFTP/sync/app event/config 保存调用点和跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；当前任务、验证范围和项目地图均已切换。

## 开工判定

- 状态：允许开工
- 原因：现有依赖已经提供结构化日志、非阻塞 writer 和过滤能力；可以在不改变业务协议、配置 schema 或依赖的前提下补齐覆盖。
- 开工前动作：已读取 `AGENTS.md`、环境记录、实施记录、项目地图、日志初始化、事件总线、SFTP、同步、本地 PTY、SSH、监控和配置保存路径；已执行基线 `cargo check` 与 `git diff --check`。
- 完成后动作：已完成全部变更格式化、编译、定向测试、92 项完整测试、空白检查和 tracking validator；真实外部服务、GUI 和日志目录故障注入保留手工验证。

## 最后确认时间

- 2026-07-11 08:40 +0800
