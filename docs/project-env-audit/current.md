# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust 桌面应用的真实结构重构任务；本轮目标是继续拆分 `src/app/dialogs/` 目录模块，并将 `src/app/ui.rs` 拆分为 `src/app/ui/` 目录模块，保持外部模块路径和运行行为不变。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“dialogs 与 ui 目录模块拆分”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / rust-i18n
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/app/dialogs.rs`，`src/app/dialogs/`，`src/app/dialogs/settings/`，`src/app/ui.rs`，`src/app/ui/`，`src/app/mod.rs`
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/app/mod.rs`，`src/app/dialogs.rs`，`src/app/dialogs/`，`src/app/ui.rs`，`src/app/ui/`，`docs/project-implementation-tracker/project-map.md`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 ...`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；GUI dialog 行为需要用户手工验证
- 工具可用性：本机 `cargo` 可正常执行；当前工程已有 Rust 测试可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/app/`

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从 app dialogs 模块目录迁移扩展为 dialogs 继续拆分与 `src/app/ui.rs` 目录模块拆分；运行环境不变，验证入口仍为格式化、全仓编译、全仓测试和 tracking docs 校验
- 受影响文件：`src/app/dialogs.rs`，`src/app/dialogs/`，`src/app/dialogs/settings/`，`src/app/ui.rs`，`src/app/ui/`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：`src/app/mod.rs` 使用 `pub mod dialogs;` 和 `pub mod ui;`，Rust 会自动解析对应目录模块；继续拆分子模块可通过编译验证行为保持
- 开工前动作：已复查 `dialogs` 子模块体量、`settings` 内部边界和 `ui.rs` 函数分布；已确认不需要联网、不使用多 agent
