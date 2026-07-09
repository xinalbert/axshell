# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用；本轮目标是修正终端路径跳转里“最后一级目录不会真正进入”的问题，并把目录/文件/不存在的判断下沉到 SFTP 后端

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“SFTP 后端目标路径判定 refinement”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` 窗口框架，`gpui_component` UI 组件，`tokio` 运行时，`russh` SSH 后端
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/init.rs`
- 本轮代码入口：`src/sftp.rs`，`src/app/actions/sftp.rs`
- 依赖统一策略：本轮不新增 Rust 依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/sftp.rs`，`src/app/actions/sftp.rs`，`docs/project-implementation-tracker/project-map.md`

## 测试环境

- 测试框架：Rust 编译检查、Rust 单元测试、tracking docs validator
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 src/sftp.rs src/app/actions/sftp.rs`，`cargo check`，`cargo test --quiet reveal_target_directory -- --nocapture`，`cargo test --quiet terminal::highlight -- --nocapture`，`git diff --check`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 运行 `cargo check --all-targets` 和 `cargo test --all`
- 外部依赖：真实行为仍依赖 SSH / SFTP 会话、服务端 `metadata()` 返回和 GUI 中的 Command/Ctrl+单击交互；本轮本机验证不依赖联网
- 工具可用性：本机可执行 `rustfmt`、`cargo check`、`cargo test`、`git diff --check` 与 tracking docs validator；`cargo fmt` 子命令未安装
- 证据文件：`Cargo.toml`，`.github/workflows/release.yml`
- 本轮验证结果：`rustfmt` 通过；`cargo check` 通过；定向测试 `reveal_target_directory` 与 `terminal::highlight` 均通过；`git diff --check` 通过；tracking docs validator 通过

## 环境变化检查

- 是否发现变化：是
- 变化摘要：当前环境主体未变，但 `current.md` 语境从“终端路径识别 + SFTP 跳转”切换到“SFTP 后端目标路径判定 refinement”；验证重点切换到目录存在时直进、文件/不存在时回父目录
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：项目边界清晰，运行环境稳定，本轮不依赖新增依赖；问题已定位为前端过早回退父目录，导致最后一级目录不会真正进入
- 开工前动作：已复查 `src/sftp.rs`、`src/app/actions/sftp.rs` 与现有 tracking 文档
- 开工前动作：已确认无需联网、不使用多 agent；项目地图已覆盖当前路径，无需结构性刷新
- 完成后动作：已执行 `cargo check`、定向单元测试和 `git diff --check`；GUI 中真实 SSH / SFTP 点击验证仍需在实际机器上确认
