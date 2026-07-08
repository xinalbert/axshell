# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust 桌面应用的真实重构任务；本轮目标是把已确认的重复逻辑收敛到公共 helper 和常量中，同时保持现有行为。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“公共 helper 抽取与 URL 常量统一”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / Tokio / russh / russh-sftp
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/backend/ssh.rs`，`src/sftp/auth.rs`，`src/sftp/mod.rs`，`src/app/constants.rs`，`src/app/dialogs.rs`，`src/app/startup.rs`
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/backend/ssh.rs`，`src/sftp/auth.rs`，`src/sftp/mod.rs`，`src/app/constants.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 ...`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；验证集中在本地编译、测试和文档 contract
- 工具可用性：本机 `cargo` 可正常执行；当前工程已有 Rust 测试可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/backend/`，`src/sftp/`，`src/app/`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从上一轮结构拆分切换到重复逻辑公共化；运行环境不变，验证入口仍为全仓 `cargo check` / `cargo test`
- 受影响文件：`src/backend/`，`src/sftp/`，`src/app/constants.rs`，`src/app/dialogs.rs`，`src/app/startup.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目工具链、依赖锁定和基础测试环境都已就位；本轮主要是行为保持型抽取和常量统一，可通过格式化、编译和测试回归验证
- 开工前动作：已复查重复点和相关源码；已确认不需要联网、不使用多 agent，且本轮不做 UI 大拆分或 session 生命周期合并
