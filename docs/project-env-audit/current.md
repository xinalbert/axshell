# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用的真实功能改动；本轮目标是实现终端刷新第一阶段优化，通过 viewport 内容比较和选区期节流减少动态等待输出时的无效整屏刷新。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“终端刷新第一阶段优化”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GPUI / gpui-component / rust-i18n / alacritty_terminal
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`cargo run --release`
- 调试辅助入口：`cargo dev-reload`
- 本轮代码入口：`src/terminal/mod.rs`，`src/app/event_loop.rs`，`src/app/mod.rs`，`src/app/init.rs`
- 渲染依据：终端 snapshot 与绘制由 `src/terminal/mod.rs` 和 `src/terminal/element.rs` 驱动；后台输出与 blink/空闲刷新由 `src/app/event_loop.rs` 统一调度
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`Cargo.toml`，`src/terminal/mod.rs`，`src/terminal/element.rs`，`src/app/event_loop.rs`，`src/app/mod.rs`，`src/app/init.rs`

## 测试环境

- 测试框架：Rust 内置测试与静态编译检查
- 默认测试命令：`cargo test`
- 当前实施验证命令：`rustfmt --edition 2024 src/terminal/mod.rs src/app/mod.rs src/app/init.rs src/app/event_loop.rs`，`cargo check`，`cargo test`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；终端选区稳定性和动态输出体感仍需要 GUI 手工验证
- 工具可用性：本机 `cargo` 可正常执行；当前工程已有 Rust 测试可用于基础回归
- 证据文件：`Cargo.toml`，`.github/workflows/ci.yml`，`src/terminal/mod.rs`，`src/app/event_loop.rs`
- 本轮验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；tracking docs 校验通过；GUI 终端选区体验未手工验证

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务从终端字体亮度作用域收口切换到终端刷新第一阶段优化；运行环境不变，验证入口仍为格式化、全仓编译、全仓测试和 tracking docs 校验
- 受影响文件：`src/terminal/mod.rs`，`src/app/event_loop.rs`，`src/app/mod.rs`，`src/app/init.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：第一阶段实现只需在现有终端 snapshot 和事件泵路径上增加轻量比较与节流，不需要新增依赖或改绘制模型
- 开工前动作：已复查终端输出事件、blink/空闲 notify、selection 状态与 terminal snapshot 路径；已确认不需要联网、不使用多 agent
