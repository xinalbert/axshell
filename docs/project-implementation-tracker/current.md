# 当前项目实施记录

## 当前目标

- 目标：按 `AGENTS.md` 的现代 Rust 模块布局拆分过大的 `src/terminal.rs` 和 `src/sftp.rs`，保持行为与现有公开路径兼容。
- 交付物：terminal backend/tab/input/CWD/transfer 子模块，SFTP model/session/browse/preview/transfer/archive/worker 子模块，兼容 re-export、迁移后的测试和项目地图。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal.rs`，`src/terminal/`，`src/sftp.rs`，`src/sftp/`，直接受模块可见性影响的 `src/` 调用点，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：改变 terminal/SFTP 业务行为、协议、并发所有权、公开 API 名称、依赖、`Cargo.toml`、`Cargo.lock`、release/tag 和上一轮标签功能行为。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 大文件职责、依赖和公开路径清单 | 已统计文件行数、符号分布、调用点和现有子模块 | `sftp.rs` 2542 行，`terminal.rs` 1505 行 |
| P2 | completed | terminal 子模块与入口兼容 re-export | `rustfmt --edition 2024`、terminal 定向测试、`cargo check` | backend、tab、listener、key encoding、CWD 和 transfer 已迁移 |
| P3 | completed | SFTP 子模块与单一 worker 所有权 | `rustfmt --edition 2024`、SFTP lifecycle 测试、`cargo check` | cursor、pin、JoinSet 和 transfer flags 仍由单一 runtime 持有 |
| P4 | completed | 项目地图、完整回归和文档收口 | `cargo test --quiet`、`git diff --check`、tracking validator | 不新增 `mod.rs`，保持 root entry 文件 |

## 已完成

- 已确认 `src/sftp.rs` 同时承担 worker、分页浏览、预览、传输、远程操作和归档，`run_sftp()` 单函数约 670 行。
- 已确认 `src/terminal.rs` 同时承担 backend 协议、terminal tab、listener、按键编码、OSC/CWD 解析、SFTP UI state 和 transfer 模型。
- 已确认项目只有根级 `AGENTS.md`；`src/sftp.rs` 和 `src/terminal.rs` 必须保留为模块入口，新子模块使用 `foo.rs` / `foo/bar.rs`，不能新增 `mod.rs`。
- 已确认当前工作树包含上一轮标签功能的未提交改动；本轮保留这些改动，不修改其行为。
- 已将 `src/terminal.rs` 收敛为模块入口；新增 `backend.rs`、`listener.rs`、`tab.rs`、`key_encoding.rs`、`cwd.rs` 和 `transfer.rs`，并通过 re-export 保持原路径。
- terminal 拆分后 38 项 `terminal::` 定向测试通过；期间仅补充 sibling 路径、`pub(super)` 和 trait import。
- 已将 `src/sftp.rs` 收敛为模块入口；model、session、browse、preview、transfer、archive、operations、worker 和 `worker/runtime.rs` 已按职责拆分。
- `SftpHandle`、`SftpCommand`、work pin 和 shutdown controller 保留在 `worker.rs`；cursor、active transfer flags 和 `JoinSet` 仍由单一 `worker/runtime.rs` 持有。
- SFTP 拆分后 `cargo check` 与 14 项 `sftp::` 定向测试通过，迁移产生的 unused imports 已清理。
- `src/terminal.rs` 从 1505 行收敛为 17 行入口，`src/sftp.rs` 从 2542 行收敛为 20 行入口；最大业务文件按单一职责保留为 `worker/runtime.rs` 768 行和 `tab.rs` 615 行。
- 项目地图已刷新为新模块职责；没有新增 `mod.rs`，`Cargo.toml` / `Cargo.lock` 未修改。

## 验证

- 已完成：环境预检、文件规模/调用点审查、terminal/SFTP 拆分、全量格式化、38 项 terminal 定向测试、14 项 SFTP 定向测试、`cargo check`、`cargo test --quiet`（78 项）、`git diff --check` 和 tracking docs validator。
- 未完成：无自动化验证缺口；上一轮标签视觉行为仍需桌面 GUI 手工检查，本轮纯模块迁移无新增 GUI 行为。

## 风险与阻塞

- 剩余风险：`worker/runtime.rs` 仍有 768 行，但职责已收敛为单一命令循环；继续拆分 handler 需要引入运行时上下文，不应和本轮纯迁移混在同一提交。
- 无阻塞；必要内部接口只提升到 `pub(super)`，SFTP 单 worker 所有权和 root 兼容路径均已保留。

## 下一步

- 后续若继续缩短 `worker/runtime.rs`，单独设计 command handler context 并保持同一所有权；桌面端补做上一轮标签视觉检查。

## 最后更新时间

- 2026-07-10 20:21 +0800
