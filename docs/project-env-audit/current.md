# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮修复高 RTT 本地输入 overlay 的本地绘制缓存失效，不修改 SSH 协议、已确认终端 buffer、P8 输入语义或依赖图。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：已完成 P9 terminal layout cache 修复的环境验证记录。
- changes.md：保留既有历史，施工前预检、上游核对和完成验证均已追加。

## 运行环境

- 主技术栈：Rust 2024、GPUI、Tokio、russh、russh-sftp、reqwest、Argon2id、XChaCha20-Poly1305。
- 版本约束：`Cargo.toml` 声明 `rust-version = "1.88.0"`、edition `2024`；本机使用 `rustc 1.96.1`、`cargo 1.96.1`。
- 包管理器：`cargo`，依赖由 `Cargo.toml` 与 `Cargo.lock` 锁定。
- 构建 / 运行入口：`src/main.rs`、`src/app/lifecycle/startup.rs`、`src/app/lifecycle/event_loop.rs`。
- 本轮代码入口：`src/terminal/element.rs`。

## 测试环境

- 测试框架：Rust 单元测试、`cargo check`、`cargo test --quiet`、`git diff --check`、tracking docs validator。
- 默认测试命令：`rustfmt --edition 2024 <changed-rust-files>`、`cargo check`、定向单元测试、`cargo test --quiet`、`git diff --check`。
- CI 测试命令：`.github/workflows/ci.yml` 在 Windows、Linux x86_64/aarch64 和 macOS x86_64/aarch64 构建 release，并在独立 Linux job 安装 `cargo-audit` 审计 `Cargo.lock`。
- 外部依赖：真实 SSH 服务、长 scrollback 和可控 100/250/500 ms RTT 条件需要手工验收；本轮不新增服务端组件或协议依赖。
- 证据文件：`AGENTS.md`、`Cargo.toml`、`Cargo.lock`、`.github/workflows/ci.yml`、`src/terminal/element.rs`。

## 环境变化检查

- 是否发现变化：是。
- 变化摘要：项目运行环境、工具链、依赖、manifest/lock 和 CI 工作流均未变；P9 已从 `GridLayoutKey` 删除 paint-only composition，避免每个本地字符使完整可见行 shape cache 失效；composition 继续在 terminal paint 阶段消费。
- 受影响文件：`src/terminal/element.rs`、`docs/`。
- 是否需要更新 `current.md` / `changes.md`：是，已补充完成验证与手工验收边界。

## 开工判定

- 状态：允许开工。
- 原因：本机工具链满足仓库约束；缓存 key 和行布局的真实依赖已确认。变更只收窄无效 cache key，不改变 composition 绘制或 backend 输入路径。
- 开工前动作：已读取环境记录、实施记录、项目地图、manifest/lock、CI 和 `TerminalElement` 缓存/paint 流程；并联网核对 Zed 上游 terminal 的确认 cell / IME overlay 分层；不新增依赖、不使用多 agent。自动化验证已完成，仍需真实 GUI 验收。

## 最后确认时间

- 2026-07-18 15:48 +0800
