# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为 Rust 2024 / GPUI 桌面应用；本轮维护双语 README，并把用户功能说明拆分到可扩展的 `docs/` 导航与功能页面。

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：已读取现有记录，并刷新为“README 与用户文档结构维护”任务语境。

## 运行环境

- 主技术栈：Rust 桌面应用，`gpui` / `gpui_component` UI，Markdown 用户与开发文档。
- 版本约束：仓库声明 `rust-version = 1.88.0`、edition `2024`；本轮不修改 Rust 源码或工具链。
- 包管理器：`cargo`
- 构建 / 运行入口：`src/main.rs`，`src/app.rs`
- 本轮文档入口：`README.md`，`README.zh.md`，`docs/README.md`，`docs/README.zh.md`，`docs/user-guide*.md`，`docs/features/`，`docs/images/`。
- 依赖策略：不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`；内部链接使用仓库相对路径。

## 测试环境

- 测试框架：Markdown 链接检查、`git diff --check`、tracking docs validator。
- 默认测试命令：Rust 改动时使用 `cargo check`、`cargo test --quiet`；本轮为 Markdown-only，不重复运行 Rust 测试。
- CI 测试命令：`.github/workflows/ci.yml` 执行多平台 release build，未声明独立文档 job。
- 当前实施验证命令：检查全部相对 Markdown 链接目标、双语入口互链、文档导航覆盖、`git diff --check` 和 tracking docs validator。
- 外部依赖：无；无需联网、外部服务或 GUI。
- 证据文件：`AGENTS.md`，`README.md`，`README.zh.md`，`docs/README*.md`，`docs/user-guide*.md`，`docs/features/`，`docs/development*.md`，`docs/project-implementation-tracker/project-map.md`。

## 环境变化检查

- 是否发现变化：是
- 变化摘要：用户指定新的语言入口约定，默认 `README.md` 使用英文，中文使用 `README.zh.md`；详细用户文档将由单篇指南迁为导航和独立功能页。
- 受影响文件：根 README、`docs/` 用户文档、环境审计和实施跟踪文档。
- 是否需要更新 `current.md` / `changes.md`：是；README 入口、文档结构和项目地图均发生变化。

## 开工判定

- 状态：允许开工
- 原因：现有双语用户指南提供了可追溯内容基础，可在不改变产品行为的前提下拆分；根工作树开工前干净。
- 开工前动作：已读取 `AGENTS.md`、README maintenance skill、环境记录、实施记录、项目地图、根 README、双语用户指南、开发文档和 release/manifest 证据。
- 完成后动作：双语入口、功能页、截图说明和项目地图已更新；31 个用户可见 Markdown 文件链接检查、语言配对、空白检查与 tracking docs validator 均通过。

## 最后确认时间

- 2026-07-11 07:03 +0800
