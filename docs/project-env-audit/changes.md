## 2026-07-15 高刷新率自适应刷新环境验证

- 时间：2026-07-15 12:10 +0800
- 变化摘要：前台 terminal/UI 变化会启动最多 3 帧的 GPUI animation frame 校准；有效样本将应用层合帧周期限定在 60–120Hz。窗口移动/缩放、失焦和系统恢复会清除样本；没有变化时不保留 animation frame 请求，后台 250ms、深睡 1s 和空闲低频通知不变。
- 受影响文件：`src/app/state/runtime.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/lifecycle/init.rs`，`src/app/views/layout.rs`，`docs/resource-lifecycle*.md`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo 和锁定 GPUI；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。上游 GPUI / WGPU 帧调度资料已联网确认。
- 验证结果：`rustfmt --edition 2024` 通过；帧节奏测试 6 项通过；`cargo check` 通过；完整 `cargo test --quiet` 200 项通过；`git diff --check` 和 tracking validator 通过。仅保留依赖 `block v0.1.6` 的 future-incompat warning。

## 2026-07-14 SFTP 下载任务文件明细预检

- 日期：2026-07-14 19:42 +0800
- 变化摘要：运行时、依赖、manifest/lock 与 CI 配置不变；本轮为 SFTP 下载任务增加可持久化的文件明细和文件清单界面，采用 WinSCP 的批量任务为顶层、文件信息为次级的模型。
- 受影响文件：`src/sftp/model.rs`，`src/events.rs`，`src/sftp/transfer.rs`，`src/sftp/worker/runtime.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/dialogs/transfers.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。
- 验证结果：已确认 WinSCP 官方文档将队列行定义为后台操作而非单文件，并为运行中的多文件任务提供总体进度、当前文件行和完整文件清单；待执行 Rust 格式化、聚焦测试、`cargo check`、完整测试、空白检查和 tracking docs validator。

## 2026-07-14 完成 SFTP 下载任务文件明细环境验证

- 日期：2026-07-14 20:09 +0800
- 变化摘要：SFTP 下载任务现在保留任务级进度、当前文件/已发现文件概览和可虚拟滚动的文件清单；下载文件状态覆盖完成、跳过、失败、取消、重启恢复和 SFTP 断连中断。运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/sftp/model.rs`，`src/events.rs`，`src/sftp/transfer.rs`，`src/sftp/worker/runtime.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/sftp.rs`，`src/app/dialogs.rs`，`src/app/dialogs/transfers.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：受影响 Rust 文件格式化通过；`cargo check` 通过；`cargo test --quiet` 185 项全部通过；SFTP hover/list 静态审计确认文件清单使用 `uniform_list` 和共享 `list_fast_hover_options`；`git diff --check`、tracking docs validator 待本轮最终文档写入后复跑。保留既有 `block v0.1.6` future-incompat warning；真实 SFTP 多选和目录递归下载仍需 GUI 手工确认。

## 2026-07-14 Windows CI tracing 编译修复预检

- 日期：2026-07-14 18:47 +0800
- 变化摘要：运行时、依赖、manifest/lock 和 CI 配置不变；本轮修复 Windows-only 本地 X server 启动日志中 `display = %display` 触发的 `tracing::field::display` 命名冲突。
- 受影响文件：`src/app/lifecycle/startup.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。
- 验证结果：计划执行 `rustfmt --edition 2024 src/app/lifecycle/startup.rs`、`cargo check`、可用时运行 Windows target check、`git diff --check` 和 tracking docs validator；真实 Windows CI 需由 CI 复跑确认。

## 2026-07-14 完成 Windows CI tracing 编译修复环境验证

- 日期：2026-07-14 18:54 +0800
- 变化摘要：Windows-only 本地 X server 启动日志中用于 display 值的局部变量已从 `display` 改为 `resolved_display`，避免 `tracing::info!` 的 `%display` 路径把 formatter 函数本身当成待显示值；运行时、依赖、manifest/lock 和 CI 配置不变。
- 受影响文件：`src/app/lifecycle/startup.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`rustfmt --edition 2024 src/app/lifecycle/startup.rs` 通过；`cargo check` 通过并仅保留既有 `block v0.1.6` future-incompat warning；`git diff --check` 通过；tracking docs validator 通过。`cargo check --target x86_64-pc-windows-msvc` 因本机未安装目标标准库失败，Windows CI 真实编译需由 CI 复跑确认。

## 2026-07-13 SFTP 默认本地目录设置完成验证

- 日期：2026-07-13 17:40 +0800
- 变化摘要：SFTP 本地文件浏览器现在支持全局默认本地目录配置和 Settings 入口；新建或未保存连接会从配置值开始，留空则回退用户主目录；已保存 SSH 会话仍优先恢复各自最后本地目录。运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/sftp.rs`，`src/app/dialogs/settings/proxy.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/features/sftp.md`，`docs/features/sftp.zh.md`，`docs/project-env-audit/current.md`，`docs/project-implementation-tracker/current.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`rustfmt --edition 2024` 通过；`cargo test --quiet local_sftp -- --nocapture` 5 项通过；`cargo check` 通过；`cargo test --quiet` 163 项通过；`cargo build` 通过；`git diff --check` 通过；tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；真实 GUI Settings 目录选择、保存、重置和 SFTP 首次打开目录仍需手工确认。

## 2026-07-13 SFTP 默认本地目录设置预检

- 日期：2026-07-13 17:25 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮新增 SFTP 本地文件浏览器的全局默认本地目录配置和 Settings 入口，同时保留已保存 SSH 会话的每会话本地目录记忆优先级。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/sftp.rs`，`src/app/dialogs/settings/proxy.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/features/sftp.md`，`docs/features/sftp.zh.md`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改 WebDAV/S3 `SyncPayload`。
- 验证结果：相关 Rust 文件格式化通过；`cargo test --quiet local_sftp -- --nocapture` 5 项通过；`cargo check` 通过并仅保留既有 `block v0.1.6` future-incompat warning；完整测试、构建、空白检查和 tracking docs validator 待执行。

## 2026-07-12 终端 URL 中文标点边界识别预检

- 日期：2026-07-12 16:28 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修复终端 URL 识别在 URL 后紧跟中文逗号等中文标点时，会把标点和后续中文文本一起纳入链接的问题。
- 受影响文件：`src/terminal/highlight.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。
- 验证结果：计划执行 `rustfmt --edition 2024 src/terminal/highlight.rs`、聚焦 URL 长度测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；真实终端 hover/open 仍需手工确认。

## 2026-07-12 完成终端 URL 中文标点边界识别环境验证

- 日期：2026-07-12 16:34 +0800
- 变化摘要：终端 URL token 扫描现在会在中文逗号等中文句读标点处停止，避免把 `https://github.com/abbodi1406/vcredist，可以...` 中的逗号和后续中文文本纳入链接；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/terminal/highlight.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`rustfmt --edition 2024 src/terminal/highlight.rs` 通过；`cargo test --quiet find_url_len_stops_at_cjk_sentence_punctuation` 1 项通过；`cargo check` 通过；完整 `cargo test --quiet` 131 项通过；`git diff --check` 通过；tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；真实终端 hover/open 仍需手工确认。

## 2026-07-12 saved SSH 无密钥导入导出预检

- 日期：2026-07-12 11:40 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮新增 saved SSH 会话与分组的无密码、无私钥 share JSON 导出和导入，并在原生菜单栏添加入口。
- 受影响文件：`src/app/input/app_menu.rs`，`src/app/views/layout.rs`，`src/app/actions/saved_sessions.rs`，`src/config/store.rs`，`src/main.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：计划执行受影响 Rust 文件格式化、聚焦测试、`cargo check`、必要的完整测试、`git diff --check` 和 tracking docs validator；真实 GUI 菜单栏文件选择仍需手工确认。

## 2026-07-12 完成 saved SSH 无密钥导入导出环境验证

- 日期：2026-07-12 11:53 +0800
- 变化摘要：saved SSH 会话与分组的 share JSON 导入/导出已实现；导出条目不包含密码、私钥路径、内联私钥、passphrase 或代理密码；File 菜单已添加导入/导出入口；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/actions/saved_sessions.rs`，`src/app/input/keybinding_recorder.rs`，`src/main.rs`，`src/app/input/app_menu.rs`，`src/app/views/layout.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-implementation-tracker/project-map.md`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：受影响 Rust 文件 `rustfmt --edition 2024` 通过；`cargo test --quiet saved_sessions_share -- --nocapture` 3 项通过；`cargo check` 通过；fast hover/context 静态审计通过；完整 `cargo test --quiet` 127 项通过；`git diff --check` 通过；tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；真实 GUI 菜单栏文件选择和导入后列表刷新仍需手工确认。

## 2026-07-12 完成 sidebar saved SSH 右键范围导出环境验证

- 日期：2026-07-12 12:02 +0800
- 变化摘要：sidebar saved session 右键菜单新增单条 SSH 导出；sidebar saved group 展开态和折叠态右键菜单新增分组导出；导出继续复用不含密码、私钥路径、内联私钥、passphrase 或代理密码的 share JSON。运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/saved_sessions.rs`，`src/app/actions/sftp.rs`，`src/app/views/layout.rs`，`src/app/views/sidebar.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-implementation-tracker/project-map.md`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：受影响 Rust 文件 `rustfmt --edition 2024` 通过；`cargo check` 通过；`cargo test --quiet saved_sessions -- --nocapture` 4 项通过；fast hover/context 静态审计通过；完整 `cargo test --quiet` 128 项通过；`git diff --check` 通过；tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；真实 GUI sidebar 文件选择和导出结果仍需手工确认。

## 2026-07-12 MacXServer 本地 X server 支持预检

- 日期：2026-07-12 13:07 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮让 macOS X11 本地 server 同时识别 XQuartz 和 `/Applications/MacXServer.app`，并为 MacXServer 固定使用 TCP display `127.0.0.1:0`，避免误用 XQuartz launchd `DISPLAY`。
- 受影响文件：`src/platform/x_server.rs`，`src/app/lifecycle/startup.rs`，`src/app/actions/session.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`；联网证据来自 MacXServer 官方 README / plan 文档。
- 验证结果：计划执行 `rustfmt --edition 2024`、macOS X server helper 聚焦测试、`cargo check`、`git diff --check` 和 tracking validator；真实 MacXServer GUI 与远端 X11 程序联机仍需手工确认。

## 2026-07-12 完成 MacXServer 本地 X server 支持环境验证

- 日期：2026-07-12 13:15 +0800
- 变化摘要：macOS 本地 X server 配置现可识别 `/Applications/MacXServer.app`；配置为 MacXServer 时固定使用 `127.0.0.1:0`，让现有 X11 relay 连接本机 TCP 6000，避免误用 XQuartz launchd `DISPLAY`。运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/platform/x_server.rs`，`src/app/lifecycle/startup.rs`，`src/app/actions/session.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖；真实 MacXServer / XQuartz GUI 联机仍需手工确认。
- 验证结果：`rustfmt --edition 2024 src/platform/x_server.rs src/app/lifecycle/startup.rs src/app/actions/session.rs` 通过；`cargo test --quiet macxserver` 2 项通过；`cargo check` 通过；`git diff --check` 通过；tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning。

## 2026-07-12 完成 macOS X server Browse 选择器修正环境验证

- 日期：2026-07-12 13:27 +0800
- 变化摘要：macOS X11 本地 X server app Browse 从只允许选择目录改为允许选择文件或目录，以便选择 XQuartz.app / MacXServer.app 这类 `.app` 应用包。运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/actions/session.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖；真实 macOS 文件对话框选择 `.app` 仍需手工确认。
- 验证结果：`rustfmt --edition 2024 src/app/actions/session.rs` 通过；`cargo check` 通过；`git diff --check` 通过；tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning。

## 2026-07-12 新增内置主题预设预检

- 日期：2026-07-12 09:54 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮新增内置 theme JSON 和默认 Theme profile，候选主题来自用户允许的联网检索。
- 受影响文件：`assets/themes/popular.json`，`src/app/theme.rs`，`src/config/model.rs`，`src/config/store.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：计划执行相关 `rustfmt`、主题/profile 聚焦测试、`cargo check`、`git diff --check` 和 tracking docs validator。

## 2026-07-12 完成新增内置主题预设环境验证

- 日期：2026-07-12 10:01 +0800
- 变化摘要：新增 `assets/themes/popular.json` 并注册 embedded themes；默认 Theme profile 增加 Catppuccin、Dracula、Nord、Rose Pine 和组合预设；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`assets/themes/popular.json`，`src/app/theme.rs`，`src/config/model.rs`，`src/config/store.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；theme/profile 聚焦测试通过；`cargo check` 通过；完整 `cargo test --quiet` 117 项通过；`git diff --check` 通过；tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；真实 GUI 主题视觉仍需手工确认。

## 2026-07-12 Base Theme 下拉 lazy fast menu 预检

- 日期：2026-07-12 09:44 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；截图中的 Theme Editor Base Theme 下拉菜单行已走 `fast_menu`，本轮只补齐 lazy candidate builder，避免 Settings 页面 render 时构建主题候选。
- 受影响文件：`src/app/dialogs/settings/custom.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：计划执行 `rustfmt --edition 2024 src/app/dialogs/settings/custom.rs`、`cargo check`、`git diff --check` 和 tracking docs validator。

## 2026-07-12 完成 Base Theme 下拉 lazy fast menu 环境验证

- 日期：2026-07-12 09:47 +0800
- 变化摘要：Theme Editor Base Theme 下拉已切到 `fast_settings_menu_lazy`，菜单行继续使用共享 fast hover；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/dialogs/settings/custom.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`rustfmt --edition 2024 src/app/dialogs/settings/custom.rs` 通过；`cargo check` 通过；fast hover 审计搜索通过；`git diff --check` 通过；tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；真实 GUI Base Theme 下拉 hover 手感仍需手工确认。

## 2026-07-12 项目本地 fast hover skill 预检

- 日期：2026-07-12 09:23 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮只新增 `.agents/skills/ax-ashell-fast-hover/` 项目本地 skill，用于指导后续 agent 复用 `src/app/hover.rs`、Settings `fast_menu` 和长列表 `uniform_list` 规则。
- 受影响文件：`.agents/skills/ax-ashell-fast-hover/`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：计划执行 skill quick validate、`git diff --check` 和 tracking docs validator；不运行 Rust 编译，因为本轮不改 Rust 源码。

## 2026-07-12 完成项目本地 fast hover skill 环境验证

- 日期：2026-07-12 09:29 +0800
- 变化摘要：项目本地 `ax-ashell-fast-hover` skill 已创建并通过校验，`AGENTS.md` 已补充 skill 入口；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`AGENTS.md`，`.agents/skills/ax-ashell-fast-hover/SKILL.md`，`.agents/skills/ax-ashell-fast-hover/agents/openai.yaml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：skill quick validate 通过；`git diff --check` 通过；tracking docs validator 通过；Rust 源码未改，本轮未运行 `cargo check` / `cargo test`。

## 2026-07-12 修正内置主题 registry reload 后回落默认暗色

- 日期：2026-07-12 07:21 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮确认外部 `ThemeRegistry::watch_dir()` reload 会清掉 AxShell embedded themes，导致 Gruvbox/Matrix/Tokyo 等内置 profile 解析失败并 fallback 到 `Default Dark`。修复集中在主题注册/应用链路和 Custom draft/profile 持久化隔离。
- 受影响文件：`src/app/theme.rs`，`src/config/store.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：`rustfmt --edition 2024 src/app/theme.rs src/config/store.rs` 通过；`cargo check` 通过；theme/profile 定向测试共 15 项通过；`cargo test --quiet` 113 项全部通过；`cargo build` 通过；`git diff --check` 通过；tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；GUI profile 连续切换仍需手工确认。

## 2026-07-12 全局快速 hover 接口与长列表虚拟化预检

- 日期：2026-07-12 09:06 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮新增 `src/app/hover.rs` 参数化快速 hover 接口，把 Settings 外剩余 selector、sidebar saved sessions 和 SFTP transfer history 等长列表切到 `gpui::uniform_list` 可见行渲染，并把 SFTP / saved session 右键菜单改为自绘 fast hover 菜单行。
- 受影响文件：`AGENTS.md`，`src/app/hover.rs`，`src/app/dialogs/settings/fast_menu.rs`，`src/app/dialogs/ssh.rs`，`src/app/dialogs/selector.rs`，`src/app/views.rs`，`src/app/views/layout.rs`，`src/app/views/sidebar.rs`，`src/app/views/sftp_panel.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`src/app/actions/saved_sessions.rs`，`src/app/actions/session.rs`，`src/app/actions/sftp.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：受影响 Rust 文件 `rustfmt --edition 2024` 通过；`cargo check` 通过，仅保留既有 `block v0.1.6` future-incompat warning；仍需完整测试、空白检查和 tracking docs validator。

## 2026-07-12 完成全局快速 hover 统一环境验证

- 日期：2026-07-12 09:18 +0800
- 变化摘要：共享快速 hover 接口、selector/sidebar/transfer 虚拟列表和 SFTP/saved session 自绘右键菜单已完成；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：受影响 Rust 文件 `rustfmt --edition 2024` 通过；`cargo check` 通过；`cargo test --quiet` 117 项全部通过；`git diff --check` 通过；tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；真实 GUI hover 手感仍需手工确认。

## 2026-07-12 Settings 长下拉快速 hover 预检

- 日期：2026-07-12 08:09 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮把 Settings 中字体等长下拉菜单切换到与 SFTP 列表同类的 `uniform_list` 虚拟渲染路径，减少全量行渲染导致的 hover 卡顿。
- 受影响文件：`src/app/dialogs/settings/fast_menu.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：确认本轮验证命令收敛为 `rustfmt --edition 2024 src/app/dialogs/settings/fast_menu.rs`、`cargo check`、`git diff --check` 和 tracking docs validator；真实 GUI 下拉 hover 手感仍需手工确认。

## 2026-07-12 Settings 字体下拉候选缓存补充

- 日期：2026-07-12 08:18 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；用户指出 `Terminal Font` 下拉仍卡后，确认除列表渲染外还有候选构建慢路径：terminal 字体下拉会逐字体执行等宽测量过滤。本轮补充缓存一次打开期间的 `FastMenuItem` 列表、系统字体名和按字号过滤后的 terminal 字体候选。
- 受影响文件：`src/app/dialogs/settings/fast_menu.rs`，`src/app/dialogs/settings/font_page.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：`rustfmt --edition 2024 src/app/dialogs/settings/fast_menu.rs src/app/dialogs/settings/font_page.rs` 通过；`cargo check` 通过，仅保留既有 `block v0.1.6` future-incompat warning；仍需完整测试、空白检查和 tracking docs validator。

## 2026-07-11 Settings 交互与主题实时预览修正预检

- 日期：2026-07-11 20:42 +0800
- 变化摘要：本轮从 SFTP hover 修正切换到 Settings 页面交互修正，范围包括设置页主要下拉 hover 反馈、SSH/保存会话入口聚焦和 custom theme 输入实时预览。
- 受影响文件：`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/custom.rs`，`src/app/dialogs/settings/shell.rs`，`src/app/theme.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/session.rs`，`src/app/actions/saved_sessions.rs`，`src/app/dialogs/selector.rs`，`src/app/views/sidebar.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；计划验证为相关 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：确认可在仓库内 UI/action 层修正；不新增依赖、不修改 `Cargo.toml` / `Cargo.lock`、不修改外部 cargo 缓存源码、不需要联网、不使用多 agent。

## 2026-07-11 刷新环境记录到主题 preset 色调修正

- 日期：2026-07-11 19:35 +0800
- 变化摘要：运行时、依赖、工具链和测试入口不变；本轮只调整内置主题资产、默认 theme profile 和已保存默认 profile 的归一化迁移。
- 受影响文件：`assets/themes/tokyonight.json`，`assets/themes/matrix.json`，`src/config/model.rs`，`src/config/store.rs`，`src/app/theme.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；计划验证为相关 `rustfmt`、theme/profile 定向测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：本机工具链满足仓库约束；现有 `ThemeSet` schema 和配置归一化入口足以完成调整，无需新增依赖、联网或多 agent。

## 2026-07-11 完成主题 preset 色调修正环境验证

- 日期：2026-07-11 19:50 +0800
- 变化摘要：Tokyo Night / Storm / Moon 和 Matrix 补充 light companion；默认 preset 改为实际对应色调；旧重复内置 profile 会迁移到 replacement；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`assets/themes/tokyonight.json`，`assets/themes/matrix.json`，`src/config/model.rs`，`src/config/store.rs`，`src/app/theme.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；theme/profile 定向测试 12 项通过；`cargo check` 通过；`cargo test --quiet` 110 项全部通过；`git diff --check` 和 tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；GUI 主题 preset 仍需手工确认。

## 2026-07-11 刷新环境记录到 Settings 布局归属整理

- 日期：2026-07-11 17:56 +0800
- 变化摘要：运行时、依赖、工具链和测试入口不变；本轮只调整 Settings 页面侧栏组织、页面内容归属和双语名称。
- 受影响文件：`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/font_page.rs`，`src/app/dialogs/settings/custom.rs`，`src/app/dialogs/settings/terminal.rs`，`src/app/dialogs/settings/workspace.rs`，`src/app/dialogs/settings/proxy.rs`，`src/app/dialogs/settings/general.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；计划验证为相关 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：本机工具链满足仓库约束；现有 GPUI settings 组件和配置 getter/setter 足以完成调整，无需新增依赖、联网或多 agent。

## 2026-07-11 完成 Settings 布局归属整理环境验证

- 日期：2026-07-11 18:25 +0800
- 变化摘要：Settings 侧栏和页面归属已调整为 General、Appearance & Theme、Theme Editor、Terminal、Workspace Layout、Monitor & Resources、Connections、Settings Sync、Shortcuts、Help、About；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/general.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/font_page.rs`，`src/app/dialogs/settings/custom.rs`，`src/app/dialogs/settings/terminal.rs`，`src/app/dialogs/settings/workspace.rs`，`src/app/dialogs/settings/proxy.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；`cargo check` 通过；`cargo test --quiet` 106 项全部通过；`git diff --check` 和 tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；GUI 设置页仍需手工确认。

## 2026-07-11 主题套装化与设置页简化预检

- 日期：2026-07-11 17:10 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮把主题设置主概念收敛为“主题套装 + 模式”，扩展内置 profile 预设，并把亮/暗变体从 Appearance 主路径移到 Custom 高级编辑语义。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/custom.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；计划运行相关 `rustfmt`、主题/profile 定向测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：确认可复用现有 `ThemeProfileConfig`、GPUI component `ThemeRegistry` 和内置主题 JSON；不新增依赖、不联网、不使用多 agent。

## 2026-07-11 完成主题套装化与设置页简化环境验证

- 日期：2026-07-11 17:19 +0800
- 变化摘要：默认主题套装扩展到 13 个，Appearance 页主路径简化为 Theme Mode + Theme，Custom 页表达为基于当前预设修改并保留高级亮/暗变体；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app/theme.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/custom.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；theme/profile 定向测试 4 项通过；`cargo check` 通过；`cargo test --quiet` 105 项全部通过；`git diff --check` 和 tracking docs validator 通过。GUI 主题套装选择和 Custom 页编辑体验仍需手工确认。

## 2026-07-11 修正 Custom 主题继承值环境记录

- 日期：2026-07-11 17:31 +0800
- 变化摘要：Custom 页不再同时显示 `Light Theme` / `Dark Theme` 两段，只显示当前模式的一组覆盖字段；字段 placeholder 和说明里的继承值改为从当前基础主题动态解析。运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/theme.rs`，`src/app/dialogs/settings/custom.rs`，`src/app/lifecycle/init.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；theme/profile 与 import theme 定向测试共 8 项通过；`cargo check` 通过；`cargo test --quiet` 106 项全部通过；`git diff --check` 和 tracking docs validator 通过。

## 2026-07-11 设置页主题 profile 与自定义保存路径预检

- 触发原因：用户要求按相关 skills 分步实施设置页主题改造，包括多套配置、自定义基于已有主题修改、自定义保存和指定保存位置。
- 执行内容：复查 `Cargo.toml`、`Cargo.lock`、`src/config/model.rs`、`src/config/store.rs`、`src/app/theme.rs`、`src/app/dialogs/settings/appearance.rs`、`src/app/dialogs/settings/custom.rs`、`src/app/lifecycle/init.rs`、`src/app/actions/session.rs`、`locales/en.yml` 和 `locales/zh-CN.yml`；确认主题引擎已支持 `ThemeSet` 多主题和用户 `themes/` 目录，当前限制主要来自 AxShell 配置模型与设置页 light/dark 双槽 UI。
- 影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app/theme.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/custom.rs`，`src/app/lifecycle/init.rs`，`src/app.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：确认本轮不新增依赖、不修改 `Cargo.toml` / `Cargo.lock`、不需要联网、不使用多 agent；本机 `rustc 1.96.1` / `cargo 1.96.1` 可用。
- 对 plan 的更新：允许继续实施“兼容旧 light/dark 字段的 `theme_profiles` 配置；外观页 profile 选择；自定义页基于当前 profile 编辑并保存到默认或指定路径”。

## 2026-07-11 Windows Ctrl 点击跳转修复预检

- 触发原因：用户反馈 Windows 下终端 URL/SFTP Ctrl+点击跳转不工作，实际按 Windows 键才触发。
- 执行内容：复查 `Cargo.toml`、`.github/workflows/ci.yml`、`src/app/actions/terminal.rs`、`src/app/actions/session.rs`、`src/app/terminal.rs` 和 `src/terminal/highlight.rs`；确认项目环境不变，问题范围是 GPUI mouse modifier 判定误用 `platform`。
- 影响文件：`src/app/actions/terminal.rs`，`src/app/actions/session.rs`，`src/app/terminal.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：确认本轮不新增依赖、不修改 `Cargo.toml` / `Cargo.lock`、不需要联网、不使用多 agent；验证命令收敛为 Rust 格式化、相关单元测试、`cargo check`、`git diff --check` 和 tracking docs validator。
- 对 plan 的更新：无

## 2026-07-11 release tag About 版本显示修复预检

- 触发原因：用户反馈 GitHub 发布版本的 About 页面没有跟随 tag 更新，仍显示仓库 `Cargo.toml` 中的 `2026.7.6`。
- 执行内容：复查 `Cargo.toml`、`Cargo.lock`、`build.rs`、`.github/workflows/release.yml`、`scripts/release_version.py`、`src/app/constants.rs` 和 `src/app/dialogs/settings/about.rs`；确认 About 页面只读 `env!("CARGO_PKG_VERSION")`，release workflow 虽然解析了 tag 并导出 `RELEASE_PUBLIC_VERSION`，但二进制 UI 没有显式优先使用该值。
- 影响文件：`build.rs`，`src/app/constants.rs`，`.github/workflows/release.yml`，`scripts/release_version.py`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：确认本轮不新增依赖、不修改 `Cargo.toml` / `Cargo.lock`、不需要联网、不使用多 agent；本机 `rustc 1.96.1` / `cargo 1.96.1` 可用；`python3 scripts/release_version.py env --tag v2026.7.11-1` 可派生 `RELEASE_PUBLIC_VERSION=2026.07.11.1`。
- 对 plan 的更新：允许继续实施“build script 读取 `RELEASE_PUBLIC_VERSION` 并注入 `AXSHELL_PUBLIC_VERSION`；About helper 优先读取注入值，普通构建回退 Cargo 包版本”。

## 2026-07-11 完成 release tag About 版本显示环境验证

- 触发原因：构建期 release public version 注入和 About 版本 helper 修复已完成，需要回写环境记录。
- 执行内容：在 `build.rs` 中监听 `RELEASE_PUBLIC_VERSION` 并注入 `AXSHELL_PUBLIC_VERSION`；在 `src/app/constants.rs` 中让 About 版本优先读取注入值，普通构建回退 Cargo 包版本；执行本机格式化、聚焦测试、编译检查、完整测试、空白检查和 tracking docs validator。
- 影响文件：`build.rs`，`src/app/constants.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 build.rs src/app/constants.rs` 通过；带 `RELEASE_PUBLIC_VERSION=2026.07.11.1` 的定向测试 1 项通过；普通环境 constants 测试 4 项通过；`cargo check` 通过；`cargo test --quiet` 98 项全部通过；`git diff --check` 通过；tracking docs validator 通过；真实 GitHub Actions tag release About 页面仍需后续实跑确认。
- 对 plan 的更新：无

## 2026-07-11 完成 Windows Ctrl 点击跳转环境验证

- 触发原因：终端 URL/SFTP 点击跳转修饰键修复和本机验证已完成，需要回写环境记录。
- 执行内容：在 `src/app/terminal.rs` 增加平台可测的 Command/Ctrl 链接激活 helper；替换 `src/app/actions/terminal.rs` 和 `src/app/actions/session.rs` 中终端 hover、点击打开和缩放滚轮的错误 `platform` 判定；补充单元测试并刷新当前环境记录。
- 影响文件：`src/app.rs`，`src/app/actions/terminal.rs`，`src/app/actions/session.rs`，`src/app/terminal.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 src/app.rs src/app/actions/terminal.rs src/app/actions/session.rs src/app/terminal.rs` 通过；`cargo test --quiet app::terminal::tests::terminal_link_activation -- --nocapture` 通过，2 个测试全部通过；`cargo check` 通过；`cargo test --quiet` 通过，94 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过；真实 Windows GUI 点击仍需手工确认。
- 对 plan 的更新：无

## 2026-07-10 刷新环境记录到终端系统文本导航快捷键

- 触发原因：用户要求终端输入继承各系统文本导航习惯，包括 Windows/Linux `Ctrl+←/→`、macOS `Command+←/→` 和 `Option+←/→`，并明确要求检索
- 执行内容：复查 `Cargo.toml`、`src/app/actions/terminal.rs`、`src/terminal.rs`、`.github/workflows/ci.yml` 和仓库级 `AGENTS.md`；检索 Apple、GNU Readline 和 xterm 官方资料；确认本轮主改动集中在终端按键编码层，不新增依赖、不调整配置 schema
- 影响文件：`src/terminal.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/research.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：确认本轮验证命令收敛为 `rustfmt --edition 2024 src/terminal.rs`、聚焦 `cargo test --quiet terminal::tests::`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；GUI 手工按键验证仍需实机确认
- 对 plan 的更新：允许继续实施“macOS Command/Option 箭头转为 Readline 文本导航序列；Windows/Linux Ctrl 箭头保留 xterm modified cursor”

## 2026-07-10 完成终端系统文本导航快捷键环境验证

- 触发原因：终端平台文本导航映射和测试已实现，需要回写本轮实际验证结果和剩余手工边界
- 执行内容：在 `src/terminal.rs` 增加 macOS 平台文本导航映射，`Command+←/→` 发送 Readline `C-a/C-e`，`Option+←/→` 发送 Readline `M-b/M-f`；保留 Windows/Linux `Ctrl+←/→` 的 xterm modified cursor 行为；补充按键编码单元测试
- 影响文件：`src/terminal.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/research.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 src/terminal.rs` 通过；`cargo test --quiet terminal::tests::` 通过，11 个测试全部通过；`cargo check` 通过；`cargo test --quiet` 通过，50 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过
- 对 plan 的更新：代码侧实现已完成；真实 macOS / Linux / Windows GUI 按键事件和用户 shell 自定义绑定仍需实机确认

## 2026-07-10 刷新环境记录到 SFTP 懒连接

- 触发原因：用户认可资源消耗评估结论，要求先落地“SFTP 懒连接”，避免打开 SSH 会话时立即建立独立 SFTP 连接
- 执行内容：复查 `src/app/actions/session.rs`、`src/app/actions/sftp.rs`、`src/app/actions/pane.rs`、`src/app/workspace/workspace.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/dialogs/` 和 `src/sftp.rs`；确认主技术栈、依赖版本和测试环境未变，问题集中在 app 层过早启动 SFTP worker 和 UI 侧默认句柄已存在的假设
- 影响文件：`src/app/actions/session.rs`，`src/app/actions/sftp.rs`，`src/app/actions/pane.rs`，`src/app/workspace/workspace.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/dialogs/delete_confirm.rs`，`src/app/dialogs/transfers.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：确认本轮验证命令收敛为 `rustfmt --edition 2024` 针对受影响 Rust 文件、`cargo check`、`cargo test --quiet`、`git diff --check` 与 tracking docs validator；无需联网，无需多 agent
- 对 plan 的更新：允许继续实施“group 级统一 ensure handle；移除 SSH 打开和 split pane 时的 SFTP 预启动；SFTP 页面/上传下载/编辑/删除/传输控制入口统一按需建连”

## 2026-07-10 完成 SFTP 懒连接环境验证

- 触发原因：SFTP 按需建连实现和本机验证已完成，需要回写当前环境结论与剩余手工边界
- 执行内容：在 `src/app/actions/sftp.rs` 增加 group 级 `ensure_sftp_handle_for_group()` / `ensure_active_sftp_handle()` / `restart_sftp_handle_for_group()`；移除 `src/app/actions/session.rs` 和 `src/app/actions/pane.rs` 中打开 SSH 与 split pane 时的 `spawn_sftp()` 预启动；让 `src/app/workspace/workspace.rs` 打开 SFTP 页面时按需建连，并让新建目录、删除、编辑、上传下载、传输控制等 UI 入口统一先 ensure handle
- 影响文件：`src/app/actions/session.rs`，`src/app/actions/sftp.rs`，`src/app/actions/pane.rs`，`src/app/workspace/workspace.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/dialogs/delete_confirm.rs`，`src/app/dialogs/transfers.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 src/app/actions/session.rs src/app/actions/sftp.rs src/app/actions/pane.rs src/app/workspace/workspace.rs src/app/lifecycle/event_loop.rs src/app/dialogs/delete_confirm.rs src/app/dialogs/transfers.rs src/app/views/sftp_panel/transfer_panel.rs` 通过；`cargo check` 通过；`cargo test --quiet` 通过，50 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过；仍保留既有 `block v0.1.6` future-incompat warning
- 对 plan 的更新：本轮代码侧实现已完成；下一阶段可继续做 SFTP 空闲断开和自动重连，GUI 手工验证仍需确认“只在打开或首次使用 SFTP 时才建连”

## 2026-07-10 刷新环境记录到 SFTP 空闲断开

- 触发原因：用户确认继续第二阶段，实现空闲时回收 SFTP 连接
- 执行内容：复查 `src/app.rs`、`src/app/lifecycle/init.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/actions/sftp.rs`、`src/sftp.rs` 和 `src/terminal.rs`；确认当前问题可限制在 app 层 group 级 handle 与事件泵；确认 `self.transfers` 中 `Running` / `Paused` 可作为活跃传输保护条件
- 影响文件：`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/sftp.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：确认本轮验证命令收敛为 `rustfmt --edition 2024 src/app.rs src/app/lifecycle/init.rs src/app/lifecycle/event_loop.rs src/app/actions/sftp.rs`、`cargo check`、`cargo test --quiet`、`git diff --check` 与 tracking docs validator；无需联网，无需多 agent
- 对 plan 的更新：允许继续实施“新增 group 级 last_activity；事件泵定期回收无活跃传输且不可见的 SFTP 连接；下一次使用时继续走 ensure 自动重连”

## 2026-07-10 完成 SFTP 空闲断开环境验证

- 触发原因：SFTP 空闲断开实现和本机验证已完成，需要回写当前环境结论与剩余边界
- 执行内容：在 `src/app.rs` / `src/app/lifecycle/init.rs` 增加 group 级 `sftp_last_activity`；在 `src/app/actions/sftp.rs` 增加 `mark_sftp_activity_for_group()`、活跃传输判断和 `sweep_idle_sftp_connections()`，默认 300 秒空闲后回收；在 `src/app/lifecycle/event_loop.rs` 中把 idle sweep 接入事件泵，并在 `SftpEntries` / `SftpStatus` / `SftpHome` / `TransferStarted` 等事件上刷新活跃时间；同步在 group 关闭和 SFTP 页面打开路径清理或刷新活跃状态
- 影响文件：`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/sftp.rs`，`src/app/actions/session.rs`，`src/app/workspace/workspace.rs`，`src/app/dialogs/transfers.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 src/app.rs src/app/lifecycle/init.rs src/app/lifecycle/event_loop.rs src/app/actions/sftp.rs src/app/actions/session.rs src/app/workspace/workspace.rs src/app/dialogs/transfers.rs src/app/views/sftp_panel/transfer_panel.rs` 通过；`cargo check` 通过；`cargo test --quiet` 通过，50 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过；仍保留既有 `block v0.1.6` future-incompat warning
- 对 plan 的更新：第二阶段代码侧实现已完成；下一步如需继续降低占用，可评估为远程编辑 watcher 增加会话 pin/refcount，或继续做 SSH 休眠策略

## 2026-07-10 刷新环境记录到 SSH 可配置传输重试

- 触发原因：用户要求把 SSH 登录时的网络重试做成可配置项，可在设置中配置重试次数与延时，并希望给出主流默认值依据
- 执行内容：复查 `src/backend/ssh/connection.rs`、`src/sftp/auth.rs`、`src/config/store.rs`、`src/app.rs`、`src/app/lifecycle/init.rs`、`src/app/dialogs/settings/terminal.rs` 和本地 tracking 文档；补充一次外部检索，确认 OpenSSH `ConnectionAttempts` 默认更保守，但本轮默认值应先保持 AxShell 现有行为
- 影响文件：`src/backend/ssh/connection.rs`，`src/sftp/auth.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs/settings/terminal.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/research.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：确认本轮验证命令收敛为 `rustfmt --edition 2024 src/backend/ssh/connection.rs src/sftp/auth.rs src/config/store.rs src/app.rs src/app/lifecycle/init.rs src/app/dialogs/settings/terminal.rs`、`cargo check`、`cargo test --quiet`、`git diff --check` 与 tracking docs validator；GUI 手工验证仍需覆盖设置页输入与保存
- 对 plan 的更新：允许继续实施“新增全局 SSH transport retry 配置；SSH/SFTP 复用同一 transport retry helper；设置页暴露重试次数和逗号分隔延时输入”

## 2026-07-09 刷新环境记录到非交互文本可选复制

- 触发原因：用户希望程序中各处文字内容能复制，而不是完全不可选中
- 执行内容：复查 `gpui_component` 的 `TextView::selectable(true)`、窗口级 `selected_text()` 机制，以及当前 `src/app/views/`、`src/app/dialogs/` 中非交互说明、日志和路径文本渲染入口；确认不宜把按钮/菜单/列表交互控件文字改成可选文本，但可优先覆盖连接日志、错误详情、传输详情、路径列表和说明文本
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认本轮不需要新增依赖、不需要联网；验证命令收敛为 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 与 tracking docs validator
- 对 plan 的更新：允许继续实施“非交互文字块使用 selectable TextView；保留按钮/菜单/列表项交互语义”

## 2026-07-09 刷新环境记录到 SSH 瞬时网络失败自动重试

- 触发原因：用户截图显示首次 SSH 连接报 `No route to host (OS error 65)`，但手动 Retry 后可成功，要求判断是否存在问题
- 执行内容：复查 `src/backend/ssh/connection.rs`、`src/config/store.rs` 和连接进度展示链路；确认错误属于 TCP 层网络不可达，不是 SSH 算法协商问题；现有 cached mode 失败后切 legacy 的提示对这类网络错误有误导性，且缺少短退避自动重试
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认本轮不需要新增依赖、不需要联网；验证命令收敛为 `rustfmt`、`cargo check`、定向单元测试、`git diff --check` 与 tracking docs validator
- 对 plan 的更新：允许继续实施“TCP/代理连接短暂网络错误自动重试；网络错误不触发 SSH legacy fallback”

## 2026-07-09 完成 SSH 瞬时网络失败自动重试环境验证

- 触发原因：SSH TCP/代理连接短退避重试和网络错误 fallback 边界已实现，需要回写环境验证结果
- 执行内容：在 `src/backend/ssh/connection.rs` 中增加 transport connect retry helper，对 `No route to host`、`Connection refused`、`Timed out` 等短暂网络错误做 0.5s / 1.5s 自动重试；网络错误不再触发 SSH mode fallback，SSH 协商错误仍保留 legacy fallback；补充错误分类单元测试
- 影响文件：`src/backend/ssh/connection.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 src/backend/ssh/connection.rs` 通过；`cargo check` 通过；`cargo test --quiet retry -- --nocapture` 通过；`cargo test --quiet retries -- --nocapture` 通过；`cargo test --quiet` 通过，45 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过
- 对 plan 的更新：代码侧实现已完成；真实局域网瞬时恢复仍需用户侧或实机环境确认

## 2026-07-09 刷新环境记录到弹窗遮罩与登录取消按钮

- 触发原因：用户反馈弹窗出现后其他区域点击事件应被屏蔽，并希望登录弹窗提供取消按钮
- 执行内容：复查 `src/app/dialogs/ssh.rs`、`src/app/dialogs/selector.rs`、`src/app/dialogs/transfers.rs`、`src/app/dialogs/delete_confirm.rs`、`src/app/views/layout.rs` 和 `gpui_component` Dialog overlay 行为；确认应用内 SSH 表单已有取消按钮，但背景点击会因 `overlay_closable(true)` 关闭弹窗；连接进度浮层只在失败后显示取消入口
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认本轮不需要新增依赖、不需要联网；验证命令收敛为 `rustfmt`、`cargo check`、`git diff --check` 与 tracking docs validator
- 对 plan 的更新：允许继续实施“应用内 Dialog 背景点击只吞事件、不关闭；连接进度浮层始终显示取消按钮”

## 2026-07-09 完成弹窗遮罩与登录取消按钮环境验证

- 触发原因：弹窗遮罩和 SSH 登录连接取消入口已实现，需要回写环境验证结果
- 执行内容：将应用内 Dialog 统一设置为 `overlay_closable(false)`；为连接进度浮层补充遮罩事件屏蔽和连接中可见取消按钮；执行本机格式化、编译检查、diff 空白检查和 tracking docs validator
- 影响文件：`src/app/dialogs/ssh.rs`，`src/app/dialogs/selector.rs`，`src/app/dialogs/transfers.rs`，`src/app/dialogs/delete_confirm.rs`，`src/app/views/layout.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 src/app/dialogs/ssh.rs src/app/dialogs/selector.rs src/app/dialogs/transfers.rs src/app/dialogs/delete_confirm.rs src/app/views/layout.rs` 通过；`cargo check` 通过；`git diff --check` 通过；tracking docs validator 通过；GUI 手工验证未执行
- 对 plan 的更新：本轮代码侧实现已完成；后续需要在 GUI 中确认背景点击不关闭弹窗、不穿透点击，以及连接中取消按钮关闭对应连接进度

## 2026-07-09 刷新环境记录到非 macOS CI 资源路径修复

- 触发原因：用户提供 `windows-x86_64`、`linux-x86_64`、`linux-aarch64` CI 失败日志，要求排查失败原因
- 执行内容：复查 `.github/workflows/ci.yml`、`Cargo.toml`、`src/app/lifecycle/startup.rs` 与图标资源目录；确认三个平台失败原因相同，均为 `include_bytes!` 编译期读取 `terminal_icon_256.png` 的相对路径少退一级目录；将资源引用改为基于 `CARGO_MANIFEST_DIR` 的包根目录路径
- 影响文件：`src/app/lifecycle/startup.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 src/app/lifecycle/startup.rs` 通过；`cargo check` 通过；`git diff --check` 通过；资源存在性检查通过；`cargo check --target x86_64-unknown-linux-gnu` 多次停在 crates.io 下载中断，未进入项目代码编译
- 对 plan 的更新：本轮无需新增依赖或调整 CI；后续由 GitHub Actions matrix 重新验证 Windows/Linux release 构建

## 2026-07-06 初始化环境预检记录

- 触发原因：用户要求先评估实现难度，再进入真实施工
- 执行内容：检查项目根目录、`Cargo.toml`、`README.md`、CI workflow、本机 `rustc` 与 `cargo` 可用性，并初始化环境记忆目录
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认项目为 Rust 桌面应用；`rustc --version` 与 `cargo --version` 可执行；当前 CI 仅构建未跑测试
- 对 plan 的更新：允许进入 `docs/project-implementation-tracker/` 规划阶段

## 2026-07-09 刷新环境记录到 SFTP shell 工作目录同步任务

- 触发原因：用户要求打开 SFTP 页面时默认查询用户 shell 所在位置，并补充要求参考 VS Code 的捕获方法
- 执行内容：复查 `src/terminal.rs`、`src/backend/ssh.rs`、`src/backend/local.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/workspace/workspace.rs`、`src/app/actions/pane.rs` 和项目 tracking 文档；将 current 记录从 SFTP 传输交互修正切换到 SFTP 打开时同步远端 shell CWD 的任务语境
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认主技术栈和依赖版本未变；本轮不新增 Rust 依赖；`cargo check` 已通过；`cargo test` 已通过，30 个测试全部通过；tracking docs validator 已通过
- 对 plan 的更新：允许继续按“捕获 shell integration OSC CWD 序列 + 独立 SSH exec `pwd -P` 兜底”的实现边界收口

## 2026-07-08 刷新环境记录到 SFTP 独立页面任务

- 触发原因：用户要求把 SFTP 改成独立页面，并补充编号标签页和快捷键对已打开 SFTP 的聚焦行为
- 执行内容：复查 `Cargo.toml`、`src/app/types.rs`、`src/app/workspace.rs`、`src/session/pane.rs`、`src/app/ui/layout.rs`、`src/app/ui/tab_bar.rs`、`src/app/ui/terminal_panel.rs` 和 `src/app/ui/sftp_panel.rs`；确认主技术栈和依赖版本未变，只刷新当前任务的页面模型、标签模型和验证重点
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认本轮不需要联网和外部服务；实施验证命令收敛为 `rustfmt`、`cargo check`、`cargo test` 和 tracking docs 校验；GUI 手工验证仍需要覆盖编号标签与 SFTP 页面切换
- 对 plan 的更新：允许继续实施 `WorkspacePage::Sftp`、编号 terminal/SFTP 标签和快捷键聚焦逻辑

## 2026-07-08 刷新环境记录到 SFTP 按需页面收口任务

- 触发原因：用户补充 SFTP 页面应按需打开、SFTP 页面内快捷键应返回对应 SSH、长文本应省略显示，且 SFTP 标签需要关闭按钮
- 执行内容：复查 `src/app/types.rs`、`src/app/workspace.rs`、`src/session/mod.rs`、`src/session/pane.rs`、`src/app/ui/layout.rs`、`src/app/ui/tab_bar.rs`、`src/app/ui/terminal_panel.rs` 和 `src/app/ui/sftp_panel.rs`；确认主技术栈、依赖版本和 CI 事实未变，只刷新当前任务的 UI 状态、快捷键焦点和验证重点
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 手工验证仍需覆盖按需 SFTP 标签、快捷键打开/返回、SFTP 标签关闭和长文本省略
- 对 plan 的更新：允许把本轮结果收口为“不新增依赖、不改 SFTP 后端，只调整工作区页面状态、快捷键路由和 SFTP 页面显示”

## 2026-07-06 刷新当前环境记录到标签栏修复任务

- 触发原因：本轮进入新的真实修复任务，原 `current.md` 停留在上一轮语境
- 执行内容：复查 `Cargo.toml`、`.github/workflows/ci.yml`、`src/app/startup.rs`，并将环境 current 记录刷新为本轮标签栏交互修复的当前态
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认运行环境和 CI 事实未变，主要变化是 current 文档语境与任务边界已同步
- 对 plan 的更新：允许继续按本轮实施计划修改 `src/app/ui.rs`

## 2026-07-06 补充 macOS 标题栏平台行为影响

- 触发原因：用户截图表明问题与 macOS 透明标题栏原生拖动行为直接相关，需要把平台层影响写入环境记忆
- 执行内容：补充 `docs/project-env-audit/current.md` 中关于集成标题栏平台行为的说明
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：环境事实未变，仅补充了影响本轮修复判断的平台层约束
- 对 plan 的更新：明确后续同类问题需要同时检查应用层与 macOS 原生标题栏拖动策略

## 2026-07-06 补充 Linux 标题栏平台行为影响

- 触发原因：用户要求 Linux 也统一行为，需要把 Linux 平台默认窗口拖动策略写入环境记忆
- 执行内容：补充 `docs/project-env-audit/current.md` 中关于 Linux 集成标题栏平台行为的说明
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：环境事实未变，仅扩大了平台层约束说明范围
- 对 plan 的更新：明确后续同类问题需要同时检查 macOS / Linux 的平台默认拖动策略

## 2026-07-06 补充 Windows 原生拖窗依赖约束

- 触发原因：用户要求 Windows 也统一为“标签块不拖、空白区可拖”，需要在环境记忆里补充 Windows 原生拖窗依赖与验证边界
- 执行内容：补充 `docs/project-env-audit/current.md` 中关于 `raw-window-handle`、`windows` 依赖，以及 Windows 目标编译验证未完成的说明
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：环境事实主体未变，仅新增 Windows 平台依赖与验证边界说明
- 对 plan 的更新：明确后续若继续改 Windows 标题栏交互，需要同时验证 GPUI hit-test 行为与原生拖窗 helper

## 2026-07-06 收敛为仅 macOS 集成标题栏

- 触发原因：用户确认非 macOS 更适合直接使用系统原生标题栏，需要同步刷新环境约束说明
- 执行内容：更新 `docs/project-env-audit/current.md`，移除对 Windows 集成标题栏额外拖窗 helper 依赖的当前态描述，改为记录“macOS 集成、非 macOS 原生”的平台策略
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：环境事实主体未变，仅修正当前平台策略与依赖约束描述
- 对 plan 的更新：后续若再讨论非 macOS 集成标题栏，需要重新进行平台习惯和实现成本评估

## 2026-07-06 刷新环境记录到监控仪表盘任务

- 触发原因：本轮真实施工已从标题栏策略修复切换到监控仪表盘可见性和设置持久化，需要同步环境 current 语境
- 执行内容：复查 `Cargo.toml`、`src/app/mod.rs`、`src/app/ui.rs`、`src/session/config.rs`，确认主技术栈与测试环境未变，只刷新当前任务的实现与验证重点
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：运行环境和 CI 事实未变；当前任务主要依赖本机 `cargo check` 与 GUI 手工验证
- 对 plan 的更新：允许继续实施监控仪表盘采样门控与设置项修改

## 2026-07-06 刷新环境记录到 SSH 兼容性任务

- 触发原因：本轮真实施工已切换到 SSH 老服务器兼容性修复，需要同步环境 current 语境
- 执行内容：复查 `Cargo.toml`、`src/backend/ssh.rs`、`src/session/mod.rs`，确认主技术栈与测试环境未变，只刷新当前任务的实现与验证重点
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：运行环境和 CI 事实未变；当前任务主要依赖本机 `cargo check` 和真实 SSH 服务器联机验证
- 对 plan 的更新：允许继续实施 SSH 算法 fallback 与错误诊断增强

## 2026-07-06 刷新环境记录到 ax_ashell 标识迁移任务

- 触发原因：用户要求将项目内 `ashell` 字符全部切换为 `ax_ashell`，需要同步环境 current 语境与验证重点
- 执行内容：复查 `Cargo.toml`、`README.md`、`src/app/startup.rs`、`.github/workflows/release.yml` 与 `assets/` 资源命名，确认主技术栈与测试环境未变，只刷新当前任务的改名范围与验证重点
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：运行环境和 CI 事实未变；当前任务主要依赖本机 `cargo check`、批量文本检索以及资源/脚本路径一致性检查
- 对 plan 的更新：允许继续实施 crate 名、显示名、资源文件名与脚本/CI 引用的统一改名

## 2026-07-06 收敛 release workflow 到无额外密钥模式

- 触发原因：用户要求暂不考虑发布到 cask 或任何需要 token / 外部密钥的平台，需要先停用相关 workflow 步骤
- 执行内容：复查 `.github/workflows/release.yml` 中的发布链路，确认 `cask` job 依赖 `secrets.TAP_GITHUB_TOKEN`，并将 `publish` / `cask` 两个发布 job 整体注释停用，仅保留构建与 artifact 上传
- 影响文件：`.github/workflows/release.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认当前 workflow 中明确依赖外部仓库 token 的是 Homebrew cask 发布；停用后不会再触发该密钥路径
- 对 plan 的更新：后续只需维护本地构建和 artifact 产出逻辑，等需要正式发布时再恢复 `publish` / `cask`

## 2026-07-06 收口 README 与日期版本策略

- 触发原因：用户要求维护双语 README、说明 fork 来源，并改用日期版本规则

## 2026-07-09 刷新环境记录到 Windows X11 display 自动选择

- 目的：在修改 Windows 本地 X server 启动逻辑前，确认当前项目环境、X11 入口和验证边界
- 改动范围：`src/config/store.rs`，`src/app/lifecycle/startup.rs`，`src/backend/ssh/x11.rs`，`src/app/actions/session.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 Windows 本地 X server 默认 display、启动参数和 relay 入口；确认主技术栈与依赖版本未变，本轮不新增依赖、不联网、不使用多 agent；将 current 记录切换到“Windows X11 display 自动避让”任务语境
- 验证结果：确认本轮验证命令收敛为 `rustfmt`、`cargo check`、`cargo test`、`git diff --check` 与 tracking docs validator；Windows GUI / 真实 SSH X11 联机验证仍需用户侧实机确认
- 风险/待办：只对内置识别的 `VcXsrv` / `Xming` 启用自动 display 参数；自定义可执行文件的专用参数不在本轮范围

## 2026-07-09 完成 Windows X11 display 自动选择环境验证

- 目的：在 Windows 本地 X server 自动 display 避让实现后，把实际编译测试结果和剩余实机边界回写到环境记忆
- 改动范围：`src/config/store.rs`，`src/app/lifecycle/startup.rs`，`src/backend/ssh/x11.rs`，`src/app/actions/session.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/user-guide.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：实现 Windows 本地 X server 按 `:0`、`:1`、`:2` 探测空闲 display；让启动函数返回实际 display；让 X11 relay 复用实际 display 进行 cookie 查询和本地 TCP 连接；同步设置页文案与用户文档；执行 `rustfmt`、`cargo check`、`cargo test` 和 `git diff --check`
- 验证结果：`rustfmt --edition 2024 src/config/store.rs src/app/lifecycle/startup.rs src/backend/ssh/x11.rs src/app/actions/session.rs` 通过；`cargo check` 通过；`cargo test` 通过，30 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过
- 风险/待办：Windows GUI 与真实 SSH X11 forwarding 仍需在 `:0` 被占用的实际机器上确认；当前远端 `DISPLAY=localhost:10.0` 语义未改变
- 执行内容：复查 `README.md`、`README.en.md`、`Cargo.toml`、`src/app/dialogs.rs` 与 `scripts/package-macos-app.sh`，确认运行环境未变，仅将验证重点扩展到 README 当前态、版本展示映射与打包元数据一致性
- 影响文件：`README.md`，`README.en.md`，`Cargo.toml`，`Cargo.lock`，`src/app/constants.rs`，`src/app/dialogs.rs`，`scripts/package-macos-app.sh`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：待运行 `cargo check` 与 tracking docs 校验确认版本策略调整未破坏构建
- 对 plan 的更新：后续版本遵循“对外日期版、内部 semver 兼容映射”的实现边界

## 2026-07-06 切换 alacritty_terminal 到官方来源

- 触发原因：用户要求将 `alacritty_terminal` 从 `zed-industries/alacritty` fork 切换到官方来源
- 执行内容：复查 `Cargo.toml`、`Cargo.lock`、`src/terminal/mod.rs`；先在临时副本验证官方 `alacritty_terminal = "0.26.0"` 可通过 `cargo check`，再在真实仓库执行相同切换并运行 `cargo update -p alacritty_terminal` 与 `cargo check`
- 影响文件：`Cargo.toml`，`Cargo.lock`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：真实仓库 `Cargo.lock` 已从 `zed-industries/alacritty` git 源切到官方 registry `alacritty_terminal 0.26.0`；`cargo check` 通过
- 对 plan 的更新：当前仅移除了终端内核的 Zed fork 绑定；若要继续减少 Zed 依赖，需要另行评估 `gpui/gpui_platform/menu`

## 2026-07-06 恢复 GitHub Release 自动发布链路

- 触发原因：用户确认希望在 tag 构建后自动把产物发布到 GitHub Release，而不是只保留 workflow artifact
- 执行内容：复查 `.github/workflows/release.yml` 中现有 build/artifact 流程和被注释的发布链路；保持 `cask` 停用；恢复 `publish` job，用 `actions/download-artifact` 汇总构建产物，再用 GitHub Release action 基于仓库内置 token 附加到 tag 对应的 Release
- 影响文件：`.github/workflows/release.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：待执行 token / secret 路径扫描与 tracking docs 校验
- 对 plan 的更新：当前发布边界固定为“保留 workflow artifact + 自动发布 GitHub Release asset；继续停用 Homebrew cask”

## 2026-07-06 刷新环境记录到设置页标签化任务

- 触发原因：用户确认将设置从弹窗改为主工作区标签页，需要同步当前环境语境与验证重点
- 执行内容：复查 `Cargo.toml`、`src/app/mod.rs`、`src/app/ui.rs`、`src/app/dialogs.rs`、`src/terminal/mod.rs`，确认主技术栈与测试环境未变，只刷新当前任务的实现边界、验证命令与 GUI 风险说明
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：运行环境和 CI 事实未变；当前任务主要依赖本机 `cargo check` 与后续 GUI 手工验证
- 对 plan 的更新：允许继续实施工作区页面态与设置页内容抽取

## 2026-07-06 刷新环境记录到 dev-reload debug 日志任务

- 触发原因：用户要求为 `cargo dev-reload` 增加仅在 debug 模式启用的日志落盘，需要同步当前环境语境与验证重点
- 执行内容：复查 `examples/dev_reload.rs`、`.cargo/config.toml`、`Cargo.toml`，确认主技术栈与测试环境未变，只刷新当前任务的实现边界、验证命令与文件系统/子进程输出转发约束
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：运行环境和 CI 事实未变；当前任务主要依赖本机 `cargo check --example dev_reload`、`cargo test --example dev_reload` 与必要的命令行手工验证
- 对 plan 的更新：允许继续实施 debug-only 日志目录与 stdout/stderr 双写

## 2026-07-06 刷新环境记录到设置页 Custom 配置中心任务

- 触发原因：用户要求将设置页同级 `Custom` 并入 `General`，并把配置文件内容按类别和默认值集中放入 `Custom`
- 执行内容：复查 `Cargo.toml`、`src/app/dialogs.rs`、`src/session/config.rs`、`locales/en.yml`、`locales/zh-CN.yml`，确认主技术栈与测试环境未变，只刷新当前任务的 UI 结构、配置字段和文案验证重点
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：运行环境和 CI 事实未变；当前任务主要依赖 `rustfmt`、`cargo check` 与 GUI 手工验证
- 对 plan 的更新：允许继续实施设置页 `Custom` theme 配置中心；用户后续收窄为只展示 theme 相关配置

## 2026-07-06 刷新环境记录到 russh 依赖升级任务

- 触发原因：用户要求将 `russh` 升级到最新版，需要同步依赖管理、网络 registry 和验证重点
- 执行内容：复查 `Cargo.toml`、`Cargo.lock`、`src/backend/ssh.rs`、`src/sftp/mod.rs`，确认项目仍为 Rust / Cargo 桌面应用；通过 Cargo registry 查询确认 `russh` 最新版本为 `0.62.2`
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：当前依赖为 `russh = "0.49"`、`russh-keys = "0.49"`、`russh-sftp = "2"`；升级需要联网更新 lockfile，并以 `cargo check` 验证 API 兼容性
- 对 plan 的更新：允许继续实施 `russh` 依赖升级和必要 API 适配
## 2026-07-07 刷新环境记录到 XQuartz 设置任务

- 目的：进入 macOS XQuartz / X11 设置入口实现前，确认当前项目环境和验证命令边界
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：读取现有环境记忆、`Cargo.toml`、设置页与 SSH 会话入口；将 current 记录从 `russh` 升级任务切换到 XQuartz 设置接入任务
- 验证结果：确认主技术栈仍为 Rust / GPUI / Tokio / russh；本轮验证命令为 `rustfmt` 与 `cargo check`
- 风险/待办：GUI 手工验证需要本机实际安装 XQuartz；完整 X11 forwarding relay 不在本轮范围

## 2026-07-07 刷新环境记录到 XQuartz 懒启动策略任务

- 目的：根据用户反馈，将 XQuartz 启动从 SSH 前自动调用调整为后续 X11 请求检测时懒启动
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：确认运行环境事实未变；将 current 记录的范围和验证重点改为 `src/session/mod.rs`、`src/app/startup.rs` 与中英文 locale
- 验证结果：本轮依赖本机 `rustfmt` 与 `cargo check`；不涉及联网、CI 或依赖版本变化
- 风险/待办：完整 X11 forwarding relay 未实现前，无法在运行时真正触发“检测到远端 X11 请求后启动 XQuartz”

## 2026-07-07 刷新环境记录到 russh X11 relay 任务

- 目的：进入 SSH 后端 X11 forwarding relay 实现前，确认当前项目环境、russh 版本和运行时外部依赖
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：确认主技术栈仍为 Rust / GPUI / Tokio / russh；将 current 记录的范围和验证重点改为 `src/backend/ssh.rs`、XQuartz、`xauth`、远程 `sshd_config` 与 X11 程序支持
- 验证结果：本轮依赖本机 `rustfmt --edition 2024 --config skip_children=true src/backend/ssh.rs` 与 `cargo check`；不涉及依赖版本变化
- 风险/待办：真实 GUI 验证需要用户本机 XQuartz、远程 SSH server 允许 X11 forwarding，并且远程测试程序本身带 X11 支持

## 2026-07-07 刷新环境记录到 local X server 跨平台适配任务

- 目的：进入 Windows Xming/VcXsrv 与 Linux X11/Wayland 适配前，确认当前项目环境和运行时外部依赖边界
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：确认主技术栈和依赖版本未变；将 current 记录的范围和验证重点改为 `src/session/config.rs`、`src/app/startup.rs`、`src/backend/ssh.rs`、local X server endpoint、`xauth` 与 no-auth fallback
- 验证结果：本轮依赖本机 `rustfmt` 与 `cargo check`；不涉及依赖版本变化
- 风险/待办：Windows/Linux GUI 行为需要在对应平台实机验证；Wayland 场景仍依赖 Xwayland 提供 `DISPLAY`

## 2026-07-07 刷新环境记录到 Windows X11 启动参数修复

- 目的：修正 Windows `VcXsrv/Xming` 自动启动参数与当前 no-auth relay fallback 不一致的问题
- 改动范围：`src/session/config.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：确认当前 Windows relay 在无本机 `xauth` cookie 时会退回 no-auth setup；将自动启动 `VcXsrv/Xming` 的默认参数补为 `:0 -multiwindow -clipboard -ac`；同步更新设置说明与环境记录
- 验证结果：本轮继续使用 `rustfmt`、`cargo check` 与 tracking docs 校验；不涉及依赖版本变化
- 风险/待办：若用户自行预启动了启用 access control 的 X server，本修复不会改变该外部进程的行为，仍需用户提供 `xauth` 或手动关闭 access control

## 2026-07-07 刷新环境记录到 dev-reload 与 release 输入焦点冲突修复

- 目的：修正 macOS 上 release `.app` 与 `cargo dev-reload` 开发实例同时运行时，开发窗口输入仍落到 release 实例的问题
- 改动范围：`examples/dev_reload.rs`，`src/app/startup.rs`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：确认终端后端没有跨进程共享输入；将问题收敛为 macOS 应用身份和前台激活冲突；把 `dev-reload` 启动改为注入独立实例标记，并在 macOS 下生成独立开发 app bundle；同时让开发实例窗口打开时显式激活当前 app 并区分窗口标题
- 验证结果：`rustfmt --edition 2024 examples/dev_reload.rs src/app/startup.rs` 通过；`cargo check --example dev_reload` 通过；`cargo check` 通过；`cargo test --example dev_reload` 通过；`cargo run --example dev_reload` 启动后不再出现 `_LSOpenURLsWithCompletionHandler ... error -10810`；用户确认当前 release 与 dev 实例已经独立隔离；仅保留既有依赖 `block v0.1.6` future-incompat warning
- 风险/待办：若用户直接运行裸二进制而非 `.app`，仍应优先使用当前实例标记和前台激活逻辑

## 2026-07-07 刷新环境记录到终端原生高亮避让任务

- 目的：在进入终端高亮策略修改前，确认当前项目环境、验证命令和本轮外部依赖边界
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 `Cargo.toml`、`.cargo/config.toml`、`src/terminal/element.rs`、`src/terminal/highlight.rs` 与 `src/app/search.rs`；确认主技术栈和依赖版本未变，只将 current 记录的任务语境和验证重点切换到终端渲染层高亮策略
- 验证结果：确认本轮不需要联网和外部服务；实施验证命令收敛为 `rustfmt --edition 2024 src/terminal/element.rs`、`cargo test keyword_highlight` 和 `cargo check`
- 风险/待办：GUI 手工验证如有需要，仅用于目视确认“已有 ANSI 颜色时不再被关键词高亮覆盖”，不影响当前代码级验证路径

## 2026-07-07 完成终端原生高亮避让任务的本机验证

- 目的：在终端高亮策略修改完成后，把本轮实际执行的本机验证结果回写到环境记忆
- 改动范围：`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt --edition 2024 src/terminal/element.rs`、`cargo test keyword_highlight` 与 `cargo check`，确认改动仅发生在终端渲染层，不涉及依赖版本或外部运行时
- 验证结果：格式化、定向单元测试和编译检查均通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：若后续需要把“原生高亮”判定进一步细分到 underline-only 或其他样式，还需补充新的渲染判定和测试

## 2026-07-07 刷新环境记录到终端关键词完整匹配任务

- 目的：在进入关键词边界感知匹配实现前，确认当前项目环境、验证命令和本轮外部依赖边界
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 `Cargo.toml`、`.cargo/config.toml`、`src/terminal/highlight.rs` 与既有 `src/terminal/element.rs` 原生颜色避让逻辑；确认主技术栈和依赖版本未变，只将 current 记录的任务语境切换到 matcher 语义收紧
- 验证结果：确认本轮仍不需要联网和外部服务；实施验证命令收敛为 `rustfmt --edition 2024 src/terminal/highlight.rs`、`cargo test keyword_highlight` 和 `cargo check`
- 风险/待办：本轮默认把 `_` 视为 token 内字符，会停止匹配 `my_ERROR` 这类标识符内部命中；若后续需要恢复这类日志风格匹配，需要单独设计边界策略

## 2026-07-07 完成终端关键词完整匹配任务的本机验证

- 目的：在关键词边界感知匹配修改完成后，把本轮实际执行的本机验证结果回写到环境记忆
- 改动范围：`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt --edition 2024 src/terminal/highlight.rs`、`cargo test keyword_highlight` 与 `cargo check`，确认 matcher 语义收紧未破坏现有编译和上一轮原生颜色避让测试
- 验证结果：格式化、定向单元测试和编译检查均通过；`cargo test keyword_highlight` 共通过 8 个相关测试；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：若后续需要重新支持 `my_ERROR` 这类标识符内部命中，需要在当前完整匹配策略之上再设计细粒度例外规则

## 2026-07-07 刷新环境记录到 SSH 会话分组任务

- 目的：在进入 SAVED 分组改造前，确认当前项目环境、验证命令和本轮外部依赖边界
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 `Cargo.toml`、`src/session/config.rs`、`src/session/mod.rs`、`src/app/mod.rs`、`src/app/dialogs.rs`、`src/app/ui.rs` 与同步链路；确认主技术栈和依赖版本未变，只将 current 记录的任务语境切换到 SSH 会话分组和 SAVED 侧栏改造
- 验证结果：确认本轮不需要联网和外部服务；实施验证命令收敛为 `rustfmt --edition 2024 --config skip_children=true src/session/config.rs src/session/mod.rs src/app/mod.rs src/app/dialogs.rs src/app/ui.rs` 和 `cargo check`
- 风险/待办：最终交互效果仍需 GUI 手工确认；折叠侧栏因空间约束预计继续保留紧凑会话入口而非完整组头结构

## 2026-07-08 刷新环境记录到 SSH 成功连接方法优先级缓存任务

- 目的：在进入 SSH default/legacy 成功模式缓存实现前，确认当前项目环境、验证命令和外部依赖边界
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 `Cargo.toml`、`src/session/config.rs`、`src/backend/ssh.rs`、`src/sftp/auth.rs`、`src/session/mod.rs`、`src/app/event_loop.rs` 与 `src/terminal/mod.rs`；确认主技术栈、依赖版本和 CI 事实未变，只将 current 记录的任务语境切换到 SSH 成功模式优先级缓存
- 验证结果：`rustfmt` 通过；`cargo test ssh_connection_modes` 通过，2 个定向测试全部通过；`cargo check` 通过；`cargo test` 通过，15 个测试全部通过；仍保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：真实 SSH / SFTP 联机效果仍需目标服务器手工验证；代理路径优先级缓存不在本轮范围

## 2026-07-07 完成 SSH 会话分组任务的本机验证

- 目的：在 SAVED 分组、SSH 表单组选项和组重命名实现完成后，把本轮实际验证结果回写到环境记忆
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/session/config.rs src/session/mod.rs src/app/mod.rs src/app/dialogs.rs src/app/ui.rs`、`cargo check`、`cargo test` 与 tracking docs 校验；确认本轮改动只影响会话模型、GPUI 侧栏/弹窗和本地文档，不涉及依赖版本、外部服务或联网步骤
- 验证结果：格式化、编译检查、13 个 Rust 测试和 tracking docs 校验均通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终交互效果仍需本机手工确认；折叠侧栏当前继续保留紧凑平铺入口，如需也改成完整分组结构需另行设计

## 2026-07-07 完成折叠侧栏分组修订的本机验证

- 目的：在折叠侧栏改为先显示组并支持点击展开后，把本轮实际验证结果回写到环境记忆
- 改动范围：`src/app/ui.rs`，`docs/user-guide.md`，`docs/user-guide.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/app/ui.rs`、`cargo check`、`cargo test` 与 tracking docs 校验；确认本轮改动集中在 GPUI 折叠侧栏渲染和本地文档，不涉及依赖版本、外部服务或联网步骤
- 验证结果：格式化、编译检查、13 个 Rust 测试和 tracking docs 校验均通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终交互效果仍需本机手工确认；折叠态当前未增加组重命名入口，避免窄栏过挤

## 2026-07-07 刷新环境记录到 custom 主题注册化任务

- 目的：在进入 custom theme 从运行时 override 迁移到真实 registry/save 流程前，确认当前项目环境、验证命令和本轮外部依赖边界
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`

## 2026-07-08 刷新环境记录到依赖小升级任务

- 目的：在进入 `anyhow` / `open` / `uuid` lockfile 小版本升级前，确认当前项目环境、依赖边界和验证命令
- 改动范围：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 `Cargo.toml`、`Cargo.lock`、`.github/workflows/ci.yml` 与前一轮 `cargo update --dry-run` / `cargo info` 结果；将 current 记录从 SFTP UI 任务切换到依赖小升级语境
- 验证结果：确认主技术栈、`rust-version = 1.85.0` 和 CI 构建入口未变；本轮实施验证命令收敛为 `cargo update -p anyhow -p open -p uuid`、`cargo check` 与 tracking docs 校验
- 风险/待办：更大版本升级与 `zed` git 依赖前移仍受当前 Rust 版本限制，不在本轮范围

## 2026-07-08 完成依赖小升级任务的本机验证

- 目的：在 lockfile 小版本升级完成后，把本轮实际执行的升级结果和本机验证回写到环境记忆
- 改动范围：`Cargo.lock`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 `cargo update -p anyhow -p open -p uuid`，确认 `Cargo.lock` 中 `anyhow`、`open`、`uuid` 已升级；复查额外传递依赖变化；执行 `cargo check` 与 tracking docs 校验
- 验证结果：编译检查通过，tracking docs 校验通过；本轮除目标依赖升级外，仅额外移除了 `pathdiff` 并让 `tempfile` 改用 `getrandom 0.3.4`；仍保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：若继续推进依赖更新，下一步需先决定是否提升 `rust-version`，否则 `zed` 相关 git 依赖和多项较新 crate 仍会被工具链卡住

## 2026-07-08 刷新环境记录到继续升级其他依赖任务

- 目的：在最新 stable Rust 验证通过后，继续尝试升级除 `anyhow` / `open` / `uuid` 之外的其他 Rust 依赖
- 改动范围：`Cargo.toml`，`Cargo.lock`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 `Cargo.toml`、`Cargo.lock`、CI 构建入口和当前未提交 diff；确认主技术栈、`rust-version = 1.85.0` 和本机 `rustc 1.96.1` 事实未变；将 current 记录切换到继续升级其他依赖的预检态
- 验证结果：确认本轮需要访问 Cargo registry / git index 获取最新兼容解；实施验证命令收敛为 `cargo update --dry-run`、`cargo check --locked`、`cargo test --locked` 和 tracking docs 校验
- 风险/待办：当前已有前一轮 lockfile diff；若本轮继续升级成功，最终 `Cargo.lock` diff 会包含前一轮和本轮依赖变化

## 2026-07-08 完成 Rust 依赖集合升级的本机验证

- 目的：在继续升级其他依赖完成后，把实际 MSRV 调整、依赖变更和验证结果回写到环境记忆
- 改动范围：`Cargo.toml`，`Cargo.lock`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：将 `rust-version` 从 `1.85.0` 提升到 `1.88.0`；升级 `directories`、`portable-pty`、`rfd`、`rust-i18n`、`thiserror`、`notify`、`zip`、`reqwest` 等直依赖约束；更新 lockfile；同步开发文档中的 Rust 版本要求
- 验证结果：`cargo check --locked` 通过；`cargo check --examples --locked` 通过；`cargo test --locked` 通过，13 个测试全部通过；仍保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：`chacha20poly1305` / `hmac` / `rand` / `sha2` 新主版本需要源码 API 迁移，未纳入本轮；GUI / 平台手工验证仍需后续执行
- 执行内容：复查 `Cargo.toml`、`src/app/theme.rs`、`src/app/dialogs.rs`、`src/app/mod.rs`、`src/session/config.rs`、`src/terminal/element.rs` 与 `gpui-component` theme registry/schema；确认主技术栈和依赖版本未变，只将 current 记录切换到 custom theme 注册化任务语境
- 验证结果：确认本轮不需要联网和外部服务；实施验证命令收敛为 `rustfmt --edition 2024 --config skip_children=true src/app/theme.rs src/app/mod.rs src/app/dialogs.rs src/session/config.rs src/terminal/element.rs src/main.rs`、`cargo check`、`cargo test` 和 tracking docs 校验
- 风险/待办：GUI 手工验证仍需本机确认；同名 custom theme 保存时需要避免当前会话继续吃旧 registry 缓存

## 2026-07-07 完成 custom 主题注册化任务的本机验证

- 目的：在 custom theme 注册化、可视编辑和 theme list 持久化实现完成后，把本轮实际验证结果回写到环境记忆
- 改动范围：`src/app/theme.rs`，`src/app/mod.rs`，`src/app/dialogs.rs`，`src/session/config.rs`，`src/terminal/element.rs`，`src/main.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/app/theme.rs src/app/mod.rs src/app/dialogs.rs src/session/config.rs src/terminal/element.rs src/main.rs`、`cargo check`、`cargo test` 与 tracking docs 校验；确认本轮改动只影响本地 theme registry / 配置持久化 / 设置页 UI，不涉及依赖版本、外部服务或联网步骤
- 验证结果：格式化、编译检查、13 个 Rust 测试和 tracking docs 校验均通过；`General` 页主题下拉现只依赖 registry 列表；`Custom` 页保存链路已能写出真实 theme file 并立即应用当前 draft；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终交互效果仍需本机手工确认；当前改名保存行为会保留旧 custom theme 作为历史条目，如后续需要“重命名覆盖”语义需再补清理策略

## 2026-07-07 完成设置页焦点修复的本机验证

- 目的：修正设置页快捷键录制焦点逻辑导致普通输入框无法输入的问题
- 改动范围：`src/app/dialogs.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：移除设置页根容器任意鼠标按下时强制聚焦主 `focus_handle` 的逻辑；保留快捷键录制按钮显式聚焦主 `focus_handle`；在设置页根容器 `on_key_down` 中增加焦点校验，只有主 `focus_handle` 当前聚焦时才处理快捷键录制和设置页标签切换
- 验证结果：`rustfmt --edition 2024 --config skip_children=true src/app/dialogs.rs`、`cargo check`、`cargo test` 均通过；13 个 Rust 测试全部通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终交互效果仍需本机手工确认；若后续希望点击设置页空白处也支持快捷键切换标签，需要改成只在非输入控件背景点击时聚焦，而不是恢复全局抢焦点

## 2026-07-07 刷新环境记录到崩溃日志 hook 任务

- 触发原因：用户要求程序崩溃时保存崩溃日志到文件，并提示用户到指定仓库反馈
- 执行内容：复查 `Cargo.toml`、`src/main.rs`、`src/app/startup.rs` 与现有运行日志文档；确认 Rust/GPUI/cargo 环境事实未变，本轮验证重点切换为启动期 panic hook、crash 文件写入和原生错误提示
- 影响文件：`src/main.rs`，`src/app/startup.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`

## 2026-07-07 刷新环境记录到终端字体 metrics 修复任务

- 目的：在进入 Terminal 字体间距修复前，确认当前项目环境、验证命令和本轮外部依赖边界
- 改动范围：`src/app/mod.rs`，`src/app/ui.rs`，`src/session/mod.rs`，`src/terminal/element.rs`，`src/terminal/input.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 `Cargo.toml`、`src/app/ui.rs`、`src/session/mod.rs`、`src/terminal/element.rs`、`src/terminal/input.rs` 与 GPUI `TextSystem` 字体测量 API；确认主技术栈和依赖版本未变，只将 current 记录切换到终端字体 metrics 修复任务语境
- 验证结果：确认本轮不需要联网和外部服务；实施验证命令收敛为 `rustfmt --edition 2024 --config skip_children=true src/app/mod.rs src/app/ui.rs src/session/mod.rs src/terminal/element.rs src/terminal/input.rs`、`cargo check` 和 tracking docs 校验
- 风险/待办：GUI 最终视觉效果仍需本机手工切换不同终端字体确认；非等宽字体只能按代表字符建立终端网格，不能保证每个 glyph 都严格适配

## 2026-07-07 完成终端字体 metrics 修复的本机验证

- 目的：在终端字体 metrics 缓存和实测接入完成后，把本轮实际验证结果回写到环境记忆
- 改动范围：`src/app/mod.rs`，`src/session/mod.rs`，`src/terminal/element.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/app/mod.rs src/app/ui.rs src/session/mod.rs src/terminal/element.rs src/terminal/input.rs`、`cargo check`、`cargo test` 与 tracking docs 校验；确认本轮改动集中在 GPUI terminal metrics / 渲染 / resize 计算，不涉及依赖版本、外部服务或联网步骤
- 验证结果：格式化、编译检查和 13 个 Rust 测试均通过；tracking docs 校验因历史 `changes/2026/07.md` 与 `research.md` 旧记录缺少 `时间：` 字段未通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终视觉效果仍需本机手工切换不同终端字体确认；若用户选择非等宽字体，终端仍只能按代表字符建立等宽网格

## 2026-07-07 完成 Terminal 比例字体保护的本机验证

- 目的：修正 Arial 等比例字体被用于 Terminal 后仍出现网格混乱的问题
- 改动范围：`src/app/dialogs.rs`，`src/terminal/element.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：新增 Terminal 字体等宽检测和比例字体 fallback；设置页 Terminal 字体下拉改为只列等宽字体，UI 字体下拉保持原行为
- 验证结果：`rustfmt --edition 2024 --config skip_children=true src/app/dialogs.rs src/app/mod.rs src/app/ui.rs src/session/mod.rs src/terminal/element.rs src/terminal/input.rs`、`cargo check`、`cargo test` 均通过；13 个 Rust 测试全部通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终效果仍需确认；如果某些系统字体的等宽属性被 advance 检测误判，可将其加入显式 allowlist

## 2026-07-07 刷新环境记录到 SAVED 固定本地终端入口任务

- 目的：在进入 SAVED 区固定 Local Terminal 入口实现前，确认当前项目环境、验证命令和本轮外部依赖边界
- 改动范围：`src/app/ui.rs`，`docs/user-guide.md`，`docs/user-guide.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 `Cargo.toml`、`src/app/ui.rs`、`src/session/mod.rs` 与现有用户文档；确认主技术栈和依赖版本未变，只将 current 记录切换到 SAVED 固定本地终端入口任务语境
- 验证结果：确认本轮不需要联网和外部服务；实施验证命令收敛为 `rustfmt --edition 2024 --config skip_children=true src/app/ui.rs`、`cargo check`、`cargo test` 和 tracking docs 校验
- 风险/待办：GUI 最终点击效果仍需本机手工确认；固定入口语义是“新开本地终端”，不会复用已有本地 tab

## 2026-07-07 完成 SAVED 固定本地终端入口的本机验证

- 目的：在 SAVED 固定本地终端入口实现完成后，把本轮实际验证结果回写到环境记忆
- 改动范围：`src/app/ui.rs`，`docs/user-guide.md`，`docs/user-guide.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/app/ui.rs`、`cargo check`、`cargo test`；确认本轮改动集中在 GPUI 侧栏渲染和用户文档，不涉及依赖版本、会话配置模型、SSH/SFTP 协议或外部服务
- 验证结果：格式化、编译检查和 13 个 Rust 测试均通过；tracking docs 校验因历史 `changes/2026/07.md` 与 `research.md` 旧记录缺少 `时间：` 字段未通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终点击效果仍需本机手工确认；固定入口语义是“新开本地终端”，不会复用已有本地 tab

## 2026-07-07 刷新环境记录到 AxShell 项目改名任务

- 目的：进入项目名称、Cargo 包名、二进制名、配置目录和打包元数据统一改名任务前，确认当前运行环境和验证边界
- 改动范围：`Cargo.toml`，`Cargo.lock`，`assets/ax_shell.desktop`，`assets/ax_ashell.desktop`，`.github/workflows/release.yml`，`scripts/package-macos-app.sh`，`examples/dev_reload.rs`，`src/` 项目标识引用，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/user-guide.md`，`docs/user-guide.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：确认主技术栈仍为 Rust / GPUI / Tokio / russh；将 current 记录切换为 AxShell 改名任务；验证重点扩展到 `ax_shell` 二进制、旧 `ax_ashell` 配置目录迁移、macOS/Linux 打包元数据和文档引用一致性
- 验证结果：运行环境和依赖版本未变；本轮不需要联网、不依赖外部 SSH/X11 服务
- 风险/待办：真实远端仓库重命名和平台安装包展示不在自动验证范围内，需要后续手工确认

## 2026-07-07 完成 AxShell 项目改名的本机验证

- 目的：在项目名称、二进制、配置目录和打包元数据统一改名完成后，把本轮实际验证结果回写到环境记忆
- 改动范围：`Cargo.toml`，`Cargo.lock`，`assets/ax_shell.desktop`，旧 desktop 文件删除，`.github/workflows/release.yml`，`scripts/package-macos-app.sh`，`examples/dev_reload.rs`，`src/` 项目标识引用，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/user-guide.md`，`docs/user-guide.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt`、`cargo check`、`cargo check --example dev_reload`、`cargo test` 和残留旧名检索；确认本轮改动不涉及依赖版本、外部服务或 SSH/SFTP 协议行为
- 验证结果：格式化、编译检查、dev-reload example 编译和 13 个 Rust 测试均通过；仅保留既有 `block v0.1.6` future-incompat warning；非历史区域的旧名只保留在旧配置目录迁移代码和升级说明中
- 风险/待办：真实远端仓库重命名、本地目录名调整和平台安装包展示需要后续手工确认

## 2026-07-07 刷新环境记录到 tag 全链路版本源任务

- 目的：进入“tag 作为唯一发布版本源”的实施前，确认当前项目环境、版本事实和验证边界
- 改动范围：`.github/workflows/release.yml`，`scripts/package-macos-app.sh`，`Cargo.toml`，`Cargo.lock`，`src/app/constants.rs`，`src/app/startup.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 release workflow、macOS 打包脚本、运行时版本显示和 Cargo manifest/lock；确认主技术栈、依赖版本和 CI 运行环境未变化，本轮重点切换为 tag/version 映射、manifest/lock 临时同步和 bundle 版本派生
- 验证结果：确认本轮不需要联网和外部服务；实施验证将收敛为版本脚本样例运行、workflow 静态自检、`rustfmt`、`cargo check` 和 tracking docs 校验
- 风险/待办：若继续支持 `vYYYY.MM.DD.N` 四段 tag，需要把它合法映射到 Cargo semver 与 bundle version，避免 workflow、本地打包和运行时版本各自漂移

## 2026-07-07 完成 tag 全链路版本源的环境收口

- 目的：在发布链路改造完成后，把本轮实际验证边界和少量官方文档检索结果回写到环境记忆
- 改动范围：`.github/workflows/release.yml`，`scripts/release_version.py`，`scripts/package-macos-app.sh`，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行版本脚本样例运行、非法 tag 失败校验、shell 静态检查、release workflow YAML 静态自检、`cargo check` 与 tracking docs 校验；补充一次 Apple 官方文档检索，用于确认 `CFBundleShortVersionString` / `CFBundleVersion` 的格式约束，并据此固定 macOS bundle version 映射
- 验证结果：版本脚本、shell / YAML 自检、编译检查和 tracking docs 校验全部通过；当前发布链路已统一到 `scripts/release_version.py`；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：真实 GitHub Release 执行、macOS Finder/系统信息展示和多平台安装包体验仍需一次远端 tag push / 本机实装验证

## 2026-07-07 收口 tag 与 Cargo 版本格式一致性

- 目的：避免 `Cargo.toml` 和 tag 分别使用 `2026.7.6` 与 `2026.07.06` 两套主版本字符，减少误读
- 改动范围：`scripts/release_version.py`，`.github/workflows/release.yml`，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：用临时 manifest 执行 `cargo metadata`，确认 Cargo 会拒绝 `2026.07.06` / `2026.07.06.1`；据此将规范 tag 收口为 `vYYYY.M.D` / `vYYYY.M.D-N`，让 tag 与 `Cargo.toml` 保持同一套 Cargo 兼容字符，再由脚本派生对外展示版本
- 验证结果：Cargo 版本约束验证通过；共享版本脚本、shell / YAML 自检和 tracking docs 校验继续通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：真实远端 tag push 仍需改用新的 canonical tag 格式；历史旧格式 tag 如需重跑工作流，应按当时脚本版本处理

## 2026-07-07 完成 terminal 左侧留白微调

- 目的：避免 terminal 正文紧贴左侧分隔线，给 terminal 区域增加约半个字符宽度的左留白
- 改动范围：`src/app/ui.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 terminal 容器、渲染和输入命中路径；确认无需改 `src/terminal/element.rs` / `src/terminal/input.rs`；在 `src/app/ui.rs` 的 terminal 容器上增加 `pl(cell_width / 2.)`
- 验证结果：`cargo check` 通过；当前改动只影响 terminal 容器层布局，不影响 PTY 网格、选区或输入命中逻辑；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：最终视觉效果仍需用户本机目视确认；若半字符仍偏近，可继续上调到 `0.75 * cell_width`

## 2026-07-07 刷新环境记录到 canonical-only release tag 任务

- 目的：在收紧 release tag 解析前，确认本轮环境边界和验证方式
- 改动范围：`scripts/release_version.py`，`.github/workflows/release.yml`，`Cargo.toml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查版本脚本、release workflow 注释、Cargo manifest 和 tracking docs；确认运行环境与依赖版本未变，本轮重点是删除 legacy tag 解析，并同步拒绝零填充的 `YYYY.MM.DD` / `YYYY.MM.DD-N`
- 验证结果：确认本轮不需要联网和外部服务；实施验证将收敛为版本脚本正反例、`cargo check` 和 tracking docs 校验
- 风险/待办：历史旧格式 tag 若在当前脚本下重跑 workflow，将按设计直接失败，需要在变更记录中明确这一行为变化

## 2026-07-07 完成 canonical-only release tag 环境收口

- 目的：在 release tag 解析收紧完成后，把实际验证结果回写到环境记忆
- 改动范围：`scripts/release_version.py`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 canonical tag 正反例、Cargo 版本正反例和 `Cargo.toml` 版本读取校验；确认脚本已删除 legacy tag 分支，并额外拒绝零填充的月 / 日版本段；完成 `cargo check`
- 验证结果：`v2026.7.7`、`v2026.7.7-1` 和 manifest 版本读取通过；`v2026.07.07`、`v2026.07.07.1` 与 `2026.07.07` 按预期失败；`cargo check` 与 tracking docs 校验通过
- 风险/待办：历史旧格式 tag 若在当前脚本下重跑 workflow，会直接失败；后续发布只能使用 canonical `vYYYY.M.D` / `vYYYY.M.D-N`

## 2026-07-07 刷新环境记录到 X11 路径显示修复

- 目的：在进入设置页 X11 本地 X server 路径显示回归修复前，确认当前项目环境和验证命令边界
- 改动范围：`src/session/config.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 `Cargo.toml`、`src/session/config.rs`、`src/app/mod.rs`、`src/app/dialogs.rs` 与 `src/session/mod.rs`；确认主技术栈和依赖版本未变；将 current 记录切换到 X11 路径兼容修复任务
- 验证结果：确认本轮不需要联网和外部服务；实施验证命令收敛为 `cargo check` 与 tracking docs 校验
- 风险/待办：若最终问题来自 GPUI 设置组件渲染而不是配置空值兼容，本轮需要额外补做 GUI 手工复核

## 2026-07-07 完成 X11 路径显示修复的本机验证

- 目的：在 X11 路径空值兼容修复完成后，把本轮实际验证结果回写到环境记忆
- 改动范围：`src/session/config.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt --edition 2024 src/session/config.rs`、`cargo check` 与 tracking docs 校验；确认本轮改动集中在配置兼容逻辑，不涉及依赖版本、外部 SSH/X11 服务或联网步骤
- 验证结果：格式化、编译检查和 tracking docs 校验均通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终效果仍需用户重启应用后在设置页手工确认；若仍不显示，应继续排查 GPUI 设置组件渲染链路

## 2026-07-07 完成 X11 输入框布局修复的本机验证

- 目的：在根据用户截图追加 X11 输入框布局修复后，把本轮实际验证结果回写到环境记忆
- 改动范围：`src/app/dialogs.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 `gpui-component` 的 `SettingItem` 与 `Input` 渲染逻辑；在 `src/app/dialogs.rs` 中为 X11 路径项补充宽度和最小宽度约束；执行 `rustfmt --edition 2024 src/app/dialogs.rs` 与 `cargo check`
- 验证结果：格式化和编译检查通过；当前已同时覆盖配置空值兼容和设置项输入框塌缩两条主分支；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终效果仍需用户重启应用后在设置页手工确认；若仍异常，应继续排查窗口内热更新状态或 GPUI `InputState` 运行时渲染

## 2026-07-07 刷新环境记录到 Windows Expand Panel 消失修复

- 目的：在进入 Windows 下 SFTP `Expand Panel` 偶发消失修复前，确认当前项目环境和验证命令边界
- 改动范围：`src/app/ui.rs`，`src/app/resizable/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 `Cargo.toml`、`src/app/ui.rs`、`src/app/mod.rs` 与 `src/app/resizable/mod.rs`；确认主技术栈和依赖版本未变；将 current 记录切换到 Windows 下 SFTP panel 展开恢复修复任务
- 验证结果：确认本轮不需要联网和外部服务；实施验证命令收敛为 `cargo check` 与 tracking docs 校验
- 风险/待办：当前还没有 Windows 实机日志；若本轮代码级修复后仍复现，需要补抓 Windows 实际布局状态或运行时日志

## 2026-07-07 完成 Windows Expand Panel 消失修复的本机验证

- 目的：在 Windows 下 SFTP `Expand Panel` 偶发消失修复完成后，把本轮实际验证结果回写到环境记忆
- 改动范围：`src/app/ui.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt --edition 2024 src/app/ui.rs`、`cargo check` 与 tracking docs 校验；确认本轮改动集中在面板布局恢复逻辑，不涉及依赖版本、外部 SSH/X11 服务或联网步骤
- 验证结果：格式化、编译检查和 tracking docs 校验均通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：Windows GUI 最终效果仍需用户手工确认；若仍复现，应继续记录展开前后的 `body_panels.sizes()`、窗口尺寸和最小化状态

## 2026-07-07 刷新环境记录到双列 SFTP 面板实现

- 目的：在进入“服务器 / 本地”双列 SFTP 面板实现前，确认当前项目环境和验证命令边界
- 改动范围：`src/app/mod.rs`，`src/app/ui.rs`，`src/sftp/ops.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 `Cargo.toml`、`src/app/mod.rs`、`src/app/ui.rs`、`src/sftp/ops.rs` 与 `src/sftp/mod.rs`；确认主技术栈和依赖版本未变；将 current 记录切换到双列 SFTP 面板任务
- 验证结果：确认本轮不需要联网和外部服务；实施验证命令收敛为 `cargo check` 与 tracking docs 校验
- 风险/待办：本轮默认只在前端补本地文件浏览和上传联动；若后续需要把下载目标也改成“当前本地列目录”，需再单独收口下载行为变更

## 2026-07-07 完成双列 SFTP 面板实现的本机验证

- 目的：在双列 SFTP 面板实现完成后，把本轮实际验证结果回写到环境记忆
- 改动范围：`src/app/mod.rs`，`src/app/ui.rs`，`src/sftp/ops.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：执行 `rustfmt --edition 2024 src/app/mod.rs src/app/ui.rs src/sftp/ops.rs`、`cargo check` 与 tracking docs 校验；确认本机无 `cargo fmt` 子命令，因此本轮直接使用 `rustfmt` 二进制；确认改动集中在前端状态、布局和 locale，不涉及依赖版本、外部 SSH 服务或联网步骤
- 验证结果：格式化、编译检查和 tracking docs 校验均通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：Windows / macOS GUI 最终效果仍需用户手工确认；若后续要把下载语义也切到当前本地列目录，需要继续评估远端下载入口和覆盖策略

## 2026-07-08 完成双列 SFTP 布局对齐修复的本机验证

- 目的：在用户截图反馈双列 SFTP 高度不对齐和列表内容塌缩后，把本轮实际验证结果回写到环境记忆
- 改动范围：`src/app/ui.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：复查 `render_sftp_panel()` 的外层双列 `h_flex`、远端 pane 和本地 pane 的高度约束；补齐 `items_stretch()`、`h_full()`、`w_full()` 与 `overflow_hidden()`；执行 `rustfmt --edition 2024 src/app/ui.rs` 与 `cargo check`
- 验证结果：格式化和编译检查通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终效果仍需用户重启或热重载后确认；若列表仍不显示，应继续检查 `UniformList` 父级尺寸或本地目录读取结果

## 2026-07-08 完成双列 SFTP 顶部工具区紧凑化验证

- 目的：在用户截图反馈顶部工具区过高、两侧样式需要一致后，把本轮实际验证结果回写到环境记忆
- 改动范围：`src/app/ui.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：将远端 / 本地功能按钮从独立操作行合并到路径行，改为小号图标按钮加 tooltip；补充 `parent_folder` 中英文文案；执行 `rustfmt --edition 2024 src/app/ui.rs` 与 `cargo check`
- 验证结果：格式化和编译检查通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终效果仍需用户重启或热重载后确认；若半宽下图标按钮仍溢出，应继续收紧按钮间距或隐藏低频动作到菜单

## 2026-07-08 完成 Zed 依赖升级环境验证

- 时间：2026-07-08 09:13 CST
- 触发原因：用户要求评估并继续验证 `zed-industries/zed` 升级代价；在 `cargo dev-reload` 报出 `accesskit` 版本冲突后，需要确认当前环境与工具链是否能完成依赖统一升级
- 执行内容：复查 `Cargo.toml`、`Cargo.lock`、当前 env / tracking 记录与现有未提交 diff；确认根项目 `gpui` / `gpui_platform` / `menu` 需要保持 plain git source 才能和 `gpui-component` 共用同一 Zed source id；在真实仓库统一 `Cargo.lock` 到 `f9c994796ad4341649d7b8664edbdfaae8bebd5d` 后执行 `cargo check --locked`、`cargo test --locked`，并刷新环境 current
- 影响文件：`Cargo.lock`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`Cargo.lock` 中 Zed 源已统一为 `git+https://github.com/zed-industries/zed#f9c994796ad4341649d7b8664edbdfaae8bebd5d`；`cargo check --locked` 通过；`cargo test --locked` 13 个测试全部通过；当前无需源码 API 迁移；仍保留既有 `block v0.1.6` future-incompat warning
- 对 plan 的更新：当前环境已证明本轮升级可在 `rust-version = 1.88.0` 下落地；若后续需要在 manifest 层显式 pin 某个 Zed commit，必须先让 `gpui-component` 与根依赖共享完全相同的 source id
## 2026-07-08 刷新环境记录到 dev-reload Windows 顺序修复

- 触发原因：用户确认继续修复 `cargo dev-reload`，需要确认该命令是否跨平台可用，并把环境/验证边界切换到 Windows 重载顺序问题
- 执行内容：复查 `.cargo/config.toml`、`examples/dev_reload.rs`、`docs/development.md`、`.github/workflows/ci.yml` 与现有 env/tracking 记录；确认 `cargo dev-reload` 是跨平台 alias，问题集中在非 macOS 分支的重载顺序与 Windows `.exe` 占用风险；将当前验证入口刷新为 `dev_reload` example 的局部编译/测试命令
- 受影响文件：`examples/dev_reload.rs`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 更新后的命令或环境：`rustfmt --edition 2024 examples/dev_reload.rs`，`cargo check --example dev_reload`，`cargo test --example dev_reload`，`python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`
- 验证结果：`rustfmt --edition 2024 examples/dev_reload.rs` 通过；`cargo check --example dev_reload` 通过；`cargo test --example dev_reload` 通过，3 个测试全部通过；tracking docs 校验通过；仍未做 Windows 实机热重载回归

## 2026-07-08 刷新环境记录到大文件按功能拆分任务

- 时间：2026-07-08 09:31 CST
- 触发原因：用户要求将项目中的大文件按功能拆分，并在必要时继续细分，需要把环境/验证边界切换到结构重构任务
- 执行内容：复查 `Cargo.toml`、`Cargo.lock`、`src/app/ui.rs`、`src/app/dialogs.rs`、`src/app/mod.rs`、`src/session/mod.rs`、`src/sftp/mod.rs`、`src/backend/ssh.rs` 与现有 env/tracking 记录；确认本轮不涉及依赖更新，重点是本地模块拆分后的全仓编译和测试回归
- 影响文件：`src/app/`，`src/session/`，`src/sftp/`，`src/backend/ssh.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：已确认工具链、依赖锁定和现有 Rust 测试环境可支撑本轮结构重构；当前主要风险来自模块边界与可见性调整，而非外部环境
- 对 plan 的更新：本轮验证入口切换为 `cargo check`、`cargo test` 和 tracking docs 校验；待模块拆分完成后刷新 `project-map.md`

## 2026-07-08 刷新环境记录到公共 helper 抽取任务

- 时间：2026-07-08 11:12 +0800
- 触发原因：用户确认继续把相似/相同代码收敛到公共内容中，需要把环境/验证边界切换到行为保持型重构任务
- 执行内容：复查 `Cargo.toml`、`.github/workflows/ci.yml`、`src/backend/ssh.rs`、`src/sftp/auth.rs`、`src/sftp/mod.rs`、`src/app/constants.rs` 与现有 env/tracking 记录；确认本轮不新增依赖、不需要联网和外部服务
- 影响文件：`src/backend/`，`src/sftp/`，`src/app/constants.rs`，`src/app/dialogs.rs`，`src/app/startup.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，13 个测试全部通过；仍保留既有 `block v0.1.6` future-incompat warning
- 对 plan 的更新：本轮验证入口固定为格式化、全仓编译、全仓测试和 tracking docs 校验；真实 SSH / SFTP 联机验证不在自动验证范围内

## 2026-07-08 刷新环境记录到运行日志可观测性增强

- 时间：2026-07-08 11:34 +0800
- 触发原因：用户确认继续实施日志建议，需要把环境/验证边界切换到运行日志入口、启动摘要和保留策略增强
- 执行内容：复查 `src/app/startup.rs`、`src/app/dialogs.rs`、`src/main.rs`、`docs/development.md` 和现有 env/tracking 记录；确认本轮不新增依赖、不需要联网和外部服务；实现后执行格式化、编译、测试和 tracking docs 校验
- 影响文件：`src/app/startup.rs`，`src/app/dialogs.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，15 个测试全部通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 打开目录行为未手工验证
- 对 plan 的更新：本轮验证入口固定为格式化、全仓编译、全仓测试和 tracking docs 校验；日志目录按钮的实际系统打开行为留作手工验证

## 2026-07-08 刷新环境记录到 dialogs 子模块目录迁移

- 时间：2026-07-08 11:44 +0800
- 触发原因：用户确认将 `src/app/dialogs.rs` 做成子模块目录，需要把环境/验证边界切换到行为保持型结构迁移
- 执行内容：复查 `src/app/mod.rs`、`src/app/dialogs.rs`、项目地图和现有 env/tracking 记录；确认 Rust 会将 `pub mod dialogs;` 解析到 `src/app/dialogs/mod.rs`；实施后执行格式化、编译和全量测试
- 影响文件：`src/app/dialogs.rs`，`src/app/dialogs/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 src/app/dialogs/mod.rs` 通过；`cargo check` 通过；`cargo test` 通过，15 个测试全部通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI dialog 行为未手工验证
- 对 plan 的更新：本轮只完成目录模块迁移；后续可继续按 `ssh.rs`、`selector.rs`、`transfers.rs`、`delete_confirm.rs`、`settings/` 拆分

## 2026-07-08 刷新环境记录到 dialogs 与 ui 目录模块拆分

- 时间：2026-07-08 12:08 +0800
- 触发原因：用户要求继续拆 `src/app/dialogs/`，随后要求继续拆 `src/app/ui.rs`
- 执行内容：复查 `src/app/mod.rs`、`src/app/dialogs/`、`src/app/ui.rs` 和项目地图；确认 `pub mod dialogs;` 与 `pub mod ui;` 均可保持外部路径不变并解析到目录模块；本轮不新增依赖、不联网、不使用多 agent
- 影响文件：`src/app/dialogs.rs`，`src/app/dialogs/`，`src/app/dialogs/settings/`，`src/app/ui.rs`，`src/app/ui/`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，15 个测试全部通过；tracking docs 校验通过；仍保留既有 `block v0.1.6` future-incompat warning
- 对 plan 的更新：本轮验证入口固定为格式化、全仓编译、全仓测试和 tracking docs 校验；GUI 交互仍需手工验证

## 2026-07-08 刷新环境记录到 settings 模块细分

- 时间：2026-07-08 12:40 +0800
- 触发原因：用户点名 `setting/mod.rs`，需要继续降低 settings 主模块体积
- 执行内容：复查 `src/app/dialogs/settings/mod.rs` 的页面分区和旧注释块；确认本轮不新增依赖、不联网、不使用多 agent；删除未编译旧 Custom 注释块，并将 Sync、Proxy/X11 页面迁移到独立子模块
- 影响文件：`src/app/dialogs/settings/mod.rs`，`src/app/dialogs/settings/sync.rs`，`src/app/dialogs/settings/proxy.rs`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，15 个测试全部通过；tracking docs 校验通过；仍保留既有 `block v0.1.6` future-incompat warning
- 对 plan 的更新：本轮验证入口不变；GUI 设置页行为仍需手工验证

## 2026-07-08 刷新环境记录到原生菜单栏接入

- 时间：2026-07-08 13:14 +0800
- 触发原因：用户确认可以使用各系统原生菜单，需要在上一轮提交后进入菜单栏真实实现
- 执行内容：复查 `Cargo.toml`、`src/main.rs`、`src/app/mod.rs`、`src/app/keybinding_recorder.rs`、`src/app/ui/layout.rs` 和 GPUI `set_menus` API；确认本轮不新增依赖、不需要联网和外部服务
- 影响文件：`src/main.rs`，`src/app/mod.rs`，`src/app/app_menu.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：验证入口固定为 `rustfmt`、`cargo check`、`cargo test` 和 tracking docs 校验；原生菜单实际显示需 macOS / Windows / Linux GUI 手工确认
- 对 plan 的更新：菜单栏将作为现有 action 的薄触发层实现，避免复制设置页、会话、pane 和终端复制粘贴逻辑

## 2026-07-08 完成原生菜单栏接入环境验证

- 时间：2026-07-08 13:25 +0800
- 触发原因：GPUI 原生菜单栏代码已接入，需要把实际验证结果回写到环境记忆
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/main.rs src/app/mod.rs src/app/app_menu.rs`、`cargo check`、`cargo test`；确认本轮未新增依赖，未调整 `Cargo.toml` / `Cargo.lock`
- 影响文件：`src/app/app_menu.rs`，`src/app/mod.rs`，`src/main.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：格式化通过；`cargo check` 通过；`cargo test` 通过，15 个测试全部通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 原生菜单显示未手工验证
- 对 plan 的更新：当前环境可支持 GPUI `App::set_menus` 接入；后续平台差异需通过三端 GUI 验证确认

## 2026-07-08 刷新环境记录到菜单快捷键显示补充

- 时间：2026-07-08 13:36 +0800
- 触发原因：用户补充要求菜单栏使用各系统默认快捷键，并在菜单项后显示快捷键
- 执行内容：复查 GPUI macOS / Windows / Linux 平台菜单实现和项目快捷键配置路径；确认 macOS 会从 `keymap` 生成菜单项 key equivalent，Windows / Linux 当前 GPUI 平台层不保证系统菜单右侧快捷键展示；项目侧补齐退出 Settings、恢复 workspace keymap 后的菜单刷新
- 影响文件：`src/app/app_menu.rs`，`src/app/workspace.rs`，`src/app/dialogs/settings/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，15 个测试全部通过；tracking docs 校验通过；GUI 原生菜单快捷键展示仍需实机确认
- 对 plan 的更新：验证入口不变；重点确认 keymap 改动后菜单注册会重新执行

## 2026-07-08 刷新环境记录到终端光标自动反差色

- 时间：2026-07-08 13:47 +0800
- 触发原因：用户确认实现光标颜色自动随背景变化，以提升终端光标可见性
- 执行内容：复查 `Cargo.toml`、`src/terminal/element.rs` 和 `src/terminal/mod.rs`；确认本轮只改终端渲染层，不新增依赖、不调整配置格式、不联网
- 影响文件：`src/terminal/element.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：待执行 `rustfmt`、`cargo check`、`cargo test` 和 tracking docs 校验；终端光标视觉效果仍需 GUI 手工确认
- 对 plan 的更新：验证入口保持不变；本轮默认做自动反差色，不先加入自定义颜色设置

## 2026-07-08 完成终端光标自动反差色环境验证

- 时间：2026-07-08 13:55 +0800
- 触发原因：终端光标自动反差色代码已实现，需要回写实际验证结果
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/terminal/element.rs`、`cargo check`、`cargo test`；确认本轮未新增依赖，未调整配置格式
- 影响文件：`src/terminal/element.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：格式化通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；tracking docs 校验通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 终端光标视觉效果未手工验证
- 对 plan 的更新：本轮自动反差色实现已通过本机静态和单元测试验证；后续如需用户精确控制颜色，可再扩展配置项

## 2026-07-08 刷新环境记录到终端字体亮度作用域和范围收口

- 时间：2026-07-08 15:23 +0800
- 触发原因：用户确认亮度范围使用 0.60-1.20，并要求亮度控制只影响命令行显示部分
- 执行内容：复查 `src/app/theme.rs`、`src/session/config.rs`、`src/terminal/element.rs` 和 `locales/`；确认本轮不新增依赖、不调整配置格式、不联网、不使用多 agent；亮度设置保留在 terminal 前景色渲染路径，移除 theme 生成阶段的全局颜色改写
- 影响文件：`src/app/theme.rs`，`src/session/config.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；tracking docs 校验通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 设置页和终端亮度视觉效果未手工验证
- 对 plan 的更新：验证入口保持不变；后续如需控制非 terminal 页面亮度，应单独设计为页面主题设置而非复用终端字体亮度

## 2026-07-08 刷新环境记录到终端刷新第一阶段优化

- 时间：2026-07-08 15:46 +0800
- 触发原因：用户确认先实现“内容签名比较 + 有选区时降频刷新”的第一阶段方案，减少动态等待输出时无意义整屏刷新
- 执行内容：复查 `src/terminal/mod.rs`、`src/terminal/element.rs`、`src/app/event_loop.rs`、`src/app/mod.rs` 和 `src/app/init.rs`；确认本轮不新增依赖、不调整配置格式、不联网、不使用多 agent；实现路径收敛为 viewport 内容比较与选区期节流，不进入 row cache 或局部绘制
- 影响文件：`src/terminal/mod.rs`，`src/app/event_loop.rs`，`src/app/mod.rs`，`src/app/init.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt` 通过；`cargo check` 通过；`cargo test` 与 tracking docs 校验待执行；GUI 终端选区稳定性未手工验证
- 对 plan 的更新：验证入口保持不变；若第一阶段收益不足，下一阶段再评估 row hash / row layout cache

## 2026-07-08 刷新环境记录到终端交互期输出延迟

- 时间：2026-07-08 16:53 +0800
- 触发原因：用户补充 `Working` 是 Codex 在终端中的流式文本，并指出中文 IME 候选会被刷新打断，需要把任务从“减少刷新”收口到“交互期间延迟应用输出”
- 执行内容：复查 `src/terminal/mod.rs`、`src/terminal/input.rs`、`src/terminal/element.rs`、`src/app/event_loop.rs`、`src/session/pane.rs` 与 `src/session/mod.rs`；确认本轮不新增依赖、不调整配置格式、不联网、不使用多 agent
- 影响文件：`src/terminal/mod.rs`，`src/terminal/input.rs`，`src/app/event_loop.rs`，`src/session/pane.rs`，`src/session/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：待执行 `rustfmt`、`cargo check`、`cargo test` 与 tracking docs 校验；GUI 终端选区与中文 IME 稳定性仍需手工验证
- 对 plan 的更新：验证入口保持不变；实现边界改为活动终端交互锁期间的输出缓存和结束后冲刷

## 2026-07-08 完成终端交互期输出延迟环境验证

- 时间：2026-07-08 17:07 +0800
- 触发原因：终端交互期输出延迟代码已实现，需要回写实际验证结果
- 执行内容：执行 `rustfmt --edition 2024 src/terminal/mod.rs src/terminal/input.rs src/app/event_loop.rs src/session/pane.rs src/session/mod.rs`、`cargo check`、`cargo test`；确认本轮未新增依赖，未调整 `Cargo.toml` / `Cargo.lock`
- 影响文件：`src/terminal/mod.rs`，`src/terminal/input.rs`，`src/app/event_loop.rs`，`src/session/pane.rs`，`src/session/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：格式化通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；tracking docs 校验待执行；仍保留既有 `block v0.1.6` future-incompat warning；GUI 终端选区与中文 IME 稳定性未手工验证
- 对 plan 的更新：当前环境可支持活动终端交互锁期间的输出延迟与结束后冲刷；后续只需补 GUI 手工确认

## 2026-07-08 修正 mouse up 立即冲刷导致的终端选区回归

- 时间：2026-07-08 17:19 +0800
- 触发原因：用户截图反馈底部文本仍“选不上”，复查后确认主因是 `on_terminal_mouse_up` 在选区刚形成时就立即冲刷积压输出
- 执行内容：修改 `src/terminal/input.rs`，移除 `mouse up` 路径中的即时 flush；保留在真正清选区或进入下一段输入交互时再冲刷输出；随后执行 `rustfmt --edition 2024 src/terminal/input.rs`、`cargo check` 和 `cargo test`
- 影响文件：`src/terminal/input.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：格式化通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；tracking docs 校验待执行；GUI 终端选区保持仍需手工确认
- 对 plan 的更新：优先先看“松开鼠标后选区是否保持”；若仍存在最后一行命中偏差，再继续修正 terminal grid bounds 记录

## 2026-07-08 完成 SFTP 独立页面任务的本机验证

- 时间：2026-07-08 18:08 +0800
- 触发原因：SFTP 独立页面、编号标签页和快捷键聚焦逻辑已实现，需要把本轮最终验证结果回写到环境记忆
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/app/types.rs src/app/mod.rs src/app/init.rs src/app/workspace.rs src/session/pane.rs src/session/mod.rs src/session/config.rs src/app/ui/mod.rs src/app/ui/layout.rs src/app/ui/tab_bar.rs src/app/ui/terminal_panel.rs src/app/ui/sftp_panel.rs src/app/app_menu.rs src/app/search.rs`、`cargo check`、`cargo test` 与 `python3 /Users/albertxin/.codex/skills/project-implementation-tracker/scripts/validate_tracking_docs.py .`，并同步刷新当前环境记录
- 影响文件：`src/app/types.rs`，`src/app/mod.rs`，`src/app/init.rs`，`src/app/workspace.rs`，`src/session/pane.rs`，`src/session/mod.rs`，`src/session/config.rs`，`src/app/ui/mod.rs`，`src/app/ui/layout.rs`，`src/app/ui/tab_bar.rs`，`src/app/ui/terminal_panel.rs`，`src/app/ui/sftp_panel.rs`，`src/app/app_menu.rs`，`src/app/search.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：格式化通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；tracking docs 校验通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 手工验证未执行
- 对 plan 的更新：本轮环境验证已完成；后续只需在 GUI 中补看编号标签、SFTP 页面切换和快捷键聚焦体验

## 2026-07-08 刷新环境记录到 SFTP 列表排序与传输面板

- 时间：2026-07-08 21:09 +0800
- 触发原因：用户补充要求 SFTP 页面支持列表表头排序、窄窗口优先压缩 `modified` / `size` 列，并将底部替换为“正在传输/失败/已完成”三标签传输面板
- 执行内容：复查 `Cargo.toml`、`src/app/ui/sftp_panel.rs`、`src/app/types.rs`、`src/app/mod.rs`、`src/app/init.rs`、`src/app/resizable/` 和 `src/app/dialogs/transfers.rs`；确认本轮不新增依赖、不调整配置格式、不联网、不使用多 agent
- 影响文件：`src/app/types.rs`，`src/app/mod.rs`，`src/app/init.rs`，`src/app/ui/mod.rs`，`src/app/ui/sftp_panel.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：待执行 `rustfmt`、`cargo check`、`cargo test` 与 tracking docs 校验；GUI 表头排序、列压缩和面板拖拽仍需手工验证
- 对 plan 的更新：验证入口保持不变；实现限定在 UI/state 层，传输后端和旧传输历史弹窗保留

## 2026-07-08 完成 SFTP 列表排序与传输面板环境验证

- 时间：2026-07-08 21:52 +0800
- 触发原因：SFTP 列表排序、窄窗口列压缩和三标签传输面板代码已完成，需要回写实际本机验证结果
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/app/types.rs src/app/mod.rs src/app/init.rs src/app/ui/mod.rs src/app/ui/sftp_panel.rs src/app/ui/layout.rs src/app/ui/tab_bar.rs src/app/ui/terminal_panel.rs src/app/workspace.rs src/session/mod.rs src/session/pane.rs`、`cargo check` 和 `cargo test`；确认本轮未新增依赖，未调整 `Cargo.toml` / `Cargo.lock`
- 影响文件：`src/app/types.rs`，`src/app/mod.rs`，`src/app/init.rs`，`src/app/ui/mod.rs`，`src/app/ui/sftp_panel.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：格式化通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；tracking docs 校验通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 表头排序、列压缩和面板拖拽仍需手工验证
- 对 plan 的更新：本轮环境验证已完成；后续只需在 GUI 中补看 SFTP 页面排序、列压缩、传输标签和高度拖拽体验

## 2026-07-08 刷新环境记录到 SFTP 目录失败状态恢复

- 时间：2026-07-08 22:05 +0800
- 触发原因：用户反馈远端某个 SFTP 文件夹超时或报错后，服务端列表后续不能继续点击
- 执行内容：复查 `Cargo.toml`、`src/sftp/ops.rs`、`src/sftp/mod.rs`、`src/app/event_loop.rs`、`src/app/ui/sftp_panel.rs` 和 `src/terminal/mod.rs`；确认主技术栈与测试环境未变，本轮不新增依赖、不调整配置格式
- 影响文件：`src/sftp/ops.rs`，`src/app/event_loop.rs`，`src/terminal/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：待执行 `rustfmt`、`cargo check`、`cargo test` 与 tracking docs 校验；真实超时目录交互仍需 GUI 和远端环境手工验证
- 对 plan 的更新：验证入口保持不变；修复限定在 SFTP UI 状态提交和事件回写路径

## 2026-07-08 完成 SFTP 目录失败状态恢复环境验证

- 时间：2026-07-08 22:05 +0800
- 触发原因：SFTP 目录读取失败后的状态恢复代码已完成，需要回写实际本机验证结果
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/sftp/ops.rs src/sftp/mod.rs src/app/event_loop.rs src/terminal/mod.rs`、`cargo check` 和 `cargo test`；确认本轮未新增依赖，未调整 `Cargo.toml` / `Cargo.lock`
- 影响文件：`src/sftp/ops.rs`，`src/sftp/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：格式化通过；`cargo check` 通过；`cargo test` 通过，18 个测试全部通过；tracking docs 校验通过；仍保留既有 `block v0.1.6` future-incompat warning；真实超时目录交互仍需 GUI 和远端环境手工验证
- 对 plan 的更新：本轮环境验证已完成；后续只需在 GUI 中补看坏目录报错后旧目录列表是否仍可点击

## 2026-07-08 刷新环境记录到终端 IME composition 锚点与高亮

- 时间：2026-07-08 22:35 +0800
- 触发原因：用户确认需要执行“像 VS Code 那样，在光标后方维护 composition 内容区域，选定后再提交到终端渲染层，并在刷新后保持高亮不中断”
- 执行内容：复查 `Cargo.toml`、`src/terminal/input.rs`、`src/terminal/element.rs`、`src/terminal/mod.rs`、`src/app/workspace.rs`、`src/app/ui/terminal_panel.rs`、`src/app/event_loop.rs` 以及 GPUI `InputHandler` 本地依赖源码；确认本轮不新增依赖、不调整配置格式、不联网、不使用多 agent
- 影响文件：`src/app/mod.rs`，`src/app/init.rs`，`src/app/workspace.rs`，`src/app/ui/terminal_panel.rs`，`src/terminal/input.rs`，`src/terminal/element.rs`，`src/terminal/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：待执行 `rustfmt`、`cargo check`、`cargo test` 与 tracking docs 校验；GUI 中文 IME 候选框和预编辑高亮仍需手工验证
- 对 plan 的更新：验证入口保持不变；实现限定在现有终端输入状态、IME bounds 和 overlay 绘制层

## 2026-07-08 完成终端 IME composition 锚点与高亮环境验证

- 时间：2026-07-08 22:49 +0800
- 触发原因：终端 IME composition 状态、锚点定位和预编辑选区高亮代码已完成，需要回写实际本机验证结果
- 执行内容：执行 `rustfmt --edition 2024 --config skip_children=true src/app/mod.rs src/app/init.rs src/app/workspace.rs src/app/ui/terminal_panel.rs src/app/event_loop.rs src/terminal/input.rs src/terminal/element.rs src/terminal/mod.rs`、`cargo check` 和 `cargo test`；确认本轮未新增依赖，未调整 `Cargo.toml` / `Cargo.lock`
- 影响文件：`src/app/mod.rs`，`src/app/init.rs`，`src/app/workspace.rs`，`src/app/ui/terminal_panel.rs`，`src/app/event_loop.rs`，`src/terminal/input.rs`，`src/terminal/element.rs`，`src/terminal/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：格式化通过；`cargo check` 通过；`cargo test` 通过，23 个测试全部通过；tracking docs 校验通过；仍保留既有 `block v0.1.6` future-incompat warning；GUI 中文 IME 候选和持续输出刷新场景未手工验证
- 对 plan 的更新：当前环境可支持 composition 锚点与预编辑高亮；后续只需在 GUI 中补看系统输入法候选窗口稳定性

## 2026-07-08 修正 IME composition 仍冻结终端线路

- 时间：2026-07-08 22:54 +0800
- 触发原因：用户反馈底层线路仍被冻结，说明旧的交互期输出延迟策略不应继续覆盖 IME composition
- 执行内容：修改 `src/app/event_loop.rs`，让 backend output 只在真实终端选区/拖选时 defer，IME composition 不再触发输出冻结；修改 `src/terminal/input.rs`，让 `setMarkedText("")` 取消输入时统一走 composition 清理和积压输出冲刷路径
- 影响文件：`src/app/event_loop.rs`，`src/terminal/input.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 --config skip_children=true src/app/event_loop.rs src/terminal/input.rs` 通过；`cargo check` 通过；`cargo test` 通过，23 个测试全部通过；GUI 中文 IME 候选和持续输出刷新场景仍需手工验证
- 对 plan 的更新：策略修正为“鼠标选区继续冻结输出；IME composition 只靠 overlay 锚点和高亮保持稳定，不冻结底层终端线路”

## 2026-07-08 完成终端选区行级冻结环境验证

- 目的：在移除全局输出冻结并改为渲染层只冻结选中行后，把实际验证结果和环境边界回写到环境记忆
- 改动范围：`src/app/mod.rs`，`src/app/init.rs`，`src/app/event_loop.rs`，`src/app/ui/layout.rs`，`src/app/ui/terminal_panel.rs`，`src/app/workspace.rs`，`src/session/mod.rs`，`src/session/pane.rs`，`src/terminal/input.rs`，`src/terminal/element.rs`，`src/terminal/mod.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：删除空的 deferred output 机制；让 backend output 始终进入 `TerminalTab::feed`；新增选区 frozen snapshot 状态、复制优先 frozen text、渲染层 frozen row 覆盖和倒置索引 + live selection 校准；执行 `rustfmt --edition 2024 --config skip_children=true src/app/mod.rs src/app/init.rs src/app/event_loop.rs src/app/ui/terminal_panel.rs src/app/ui/layout.rs src/app/workspace.rs src/session/mod.rs src/session/pane.rs src/terminal/input.rs src/terminal/element.rs src/terminal/mod.rs`、`cargo check` 和 `cargo test`
- 验证结果：格式化通过；`cargo check` 通过；`cargo test` 通过，25 个测试全部通过；新增测试覆盖 UTF-16 composition range、frozen bottom index 随 history 增长上移、history 到上限后跟随 live selection 行偏移；仍保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 手工验证未执行，仍需在真实持续输出和系统中文输入法场景确认只有选中行冻结、其他行刷新、候选框位置和预编辑高亮稳定

## 2026-07-09 刷新环境记录到 release highlights 展示修复

- 目的：在修改 GitHub Release workflow 发布正文生成逻辑前，确认本轮环境边界和验证方式
- 改动范围：`.github/workflows/release.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 release workflow 的 publish job、`Generate release highlights` Bash 片段和版本脚本；确认主技术栈与依赖版本未变，本轮不新增依赖、不调整 Rust 源码、不联网、不使用多 agent
- 验证结果：待执行 release workflow YAML 静态检查、`Generate release highlights` shell 片段静态检查和 tracking docs validator；真实 GitHub Release 页面效果需下次 tag 发布时确认
- 风险/待办：GitHub 平台最终 Markdown 渲染以线上 Release 页面为准；本轮通过显式 commit URL 链接避免依赖自动链接规则

## 2026-07-09 完成 release highlights 展示修复环境验证

- 目的：在 GitHub Release workflow 发布正文生成逻辑修复后，把实际验证结果和剩余风险回写到环境记忆
- 改动范围：`.github/workflows/release.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：修改 release highlights 生成逻辑，使 commit 条目使用完整 hash 构造 URL、短 hash 带 `#` 作为展示文本；移除 release body 中的 Keyword Rules，改写入 GitHub Actions step summary；执行 workflow YAML 解析、Bash 静态检查、`git diff --check`、本地样例生成和 tracking docs validator
- 验证结果：workflow YAML 解析通过；`Generate release highlights` Bash 静态检查通过；`git diff --check` 通过；本地样例生成确认 release body 输出 `[#短hash](.../commit/完整hash)`，且不包含 `Keyword Rules`；tracking docs validator 通过
- 风险/待办：真实 GitHub Release 页面渲染仍需在下次 tag 发布后确认；本轮未运行 Rust 编译/测试，因为未修改 Rust 源码、依赖或构建矩阵

## 2026-07-09 刷新环境记录到 release highlights 格式收敛

- 目的：在进一步调整 GitHub Release workflow 发布正文格式前，确认本轮环境边界和验证方式
- 改动范围：`.github/workflows/release.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 release workflow 的 publish job、`Generate release highlights` Bash 片段和本地模拟输出；确认主技术栈与依赖版本未变，本轮不新增依赖、不调整 Rust 源码、不联网、不使用多 agent
- 验证结果：待执行 workflow YAML 解析、`Generate release highlights` Bash 静态检查、本地样例生成、`git diff --check` 与 tracking docs validator；真实 GitHub Release 页面效果需下次 tag 发布时确认
- 风险/待办：关键词分组仍是启发式规则，真实发布页最终效果仍需 tag 发布后确认

## 2026-07-09 完成 release highlights 格式收敛环境验证

- 目的：在 GitHub Release workflow 发布正文格式收敛后，把实际验证结果和剩余风险回写到环境记忆
- 改动范围：`.github/workflows/release.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：修改 release highlights 输出结构和关键词规则；执行 workflow YAML 解析、Bash 静态检查、`git diff --check`、本地样例生成和 tracking docs validator；确认本轮未修改 Rust 源码、依赖或构建矩阵
- 验证结果：workflow YAML 解析通过；`Generate release highlights` Bash 静态检查通过；`git diff --check` 通过；本地样例生成确认 `Full changelog`、句尾 commit 链接、SFTP 独立分组和关键词误匹配修正生效；tracking docs validator 通过
- 风险/待办：关键词分组仍是启发式规则；真实 GitHub Release 页面渲染仍需在下次 tag 发布后确认

## 2026-07-09 刷新环境记录到发布产物覆盖范围扩展

- 目的：在扩展 GitHub Actions 发布矩阵和产物类型前，确认本轮环境边界和验证方式
- 改动范围：`.github/workflows/ci.yml`，`.github/workflows/release.yml`，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/research.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 CI / Release workflow、Debian metadata、Linux desktop entry 和中英文文档；联网核对 GitHub-hosted runner 当前标签；确认本轮不新增 Rust 依赖、不调整应用源码、不使用多 agent
- 验证结果：待执行 workflow YAML 解析、CI / Release Bash 片段静态检查和 tracking docs validator；Linux ARM64、Linux `.deb` 和 macOS universal 真实产物需 GitHub Actions 运行确认
- 风险/待办：Windows ARM64 runner 仍为 public preview，本轮不纳入主发布矩阵；macOS universal 和 `.deb` 安装体验需发布后实测

## 2026-07-09 完成发布产物覆盖范围扩展环境验证

- 目的：在发布矩阵和产物类型扩展后，把实际静态验证结果和剩余线上验证边界回写到环境记忆
- 改动范围：`.github/workflows/ci.yml`，`.github/workflows/release.yml`，`README.md`，`README.en.md`，`docs/development.md`，`docs/development.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：新增 Linux ARM64 CI/Release matrix；在 Release workflow 中为 Linux 输出 `.tar.gz` 和 `.deb`；新增 macOS universal job 组合双架构 `.app`；同步中英文发布文档；确认本轮未修改 Rust 源码、依赖或 lockfile
- 验证结果：workflow YAML 解析通过；Release workflow 所有 `run` 脚本经本地占位替换后通过 `bash -n`；`git diff --check` 通过；tracking docs validator 通过
- 风险/待办：Linux ARM64 runner、Linux `.deb` 构建安装和 macOS universal app 需要 GitHub Actions 实际发布后下载验证；Windows ARM64 仍因 runner preview 状态留作后续实验项

## 2026-07-09 刷新环境记录到 Windows 菜单栏缺失修复

- 目的：在修复 Windows 中菜单栏不可见问题前，确认当前项目环境、平台依赖实现和验证边界
- 改动范围：`src/app/app_menu.rs`，`src/app/init.rs`，`src/app/mod.rs`，`src/app/ui/layout.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 `Cargo.toml`、`src/main.rs`、`src/app/startup.rs`、`src/app/app_menu.rs`、`src/app/ui/layout.rs` 和 `gpui` / `gpui_component` 依赖实现；确认 Windows/Linux 的 `gpui::set_menus` 只保存菜单数据，不自动显示菜单 UI，本轮需要在应用层显式渲染
- 验证结果：确认主技术栈与依赖版本未变；本轮验证命令收敛为 `rustfmt`、`cargo check`、`cargo check --target x86_64-pc-windows-msvc` 和 tracking docs 校验
- 风险/待办：若后续要求完全原生 Win32 menubar，而不是应用内菜单栏，还需继续评估平台层实现成本

## 2026-07-09 完成 Windows 菜单栏显式渲染环境验证

- 目的：在 Windows 菜单栏显式渲染修复后，把实际编译验证结果和剩余实机边界回写到环境记忆
- 改动范围：`src/app/app_menu.rs`，`src/app/init.rs`，`src/app/mod.rs`，`src/app/ui/layout.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：在 `AxShell` 中接入 Windows `AppMenuBar` 实体；让 `app_menu::refresh()` 同时刷新平台菜单、`gpui_component::GlobalState` 和窗口菜单栏；在 Windows 原生标题栏布局顶部渲染菜单栏；执行 `rustfmt` 和 `cargo check`，并启动 Windows 目标编译验证
- 验证结果：`rustfmt --edition 2024 src/app/mod.rs src/app/init.rs src/app/app_menu.rs src/app/ui/layout.rs` 通过；`cargo check` 通过；`cargo check --target x86_64-pc-windows-msvc` 下载依赖后因本机缺少该 Rust target 而未完成；tracking docs validator 通过；GUI 手工验证尚未执行
- 风险/待办：最终菜单显示与点击行为仍需 Windows 实机确认；若要继续做 Windows target 编译，需要使用带 `rustup` 或预装 `x86_64-pc-windows-msvc` target 的工具链

## 2026-07-09 扩展环境记录到 Linux 菜单栏缺失修复

- 目的：在确认 Linux 菜单栏路径与 Windows 同类后，把 Linux 菜单栏缺失修复纳入当前环境边界
- 改动范围：`src/app/app_menu.rs`，`src/app/init.rs`，`src/app/ui/layout.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 `gpui_linux` 的 `set_menus` 实现，确认其与 Windows 一样只保存菜单数据；将 `AppMenuBar` 初始化、刷新与布局渲染从仅 Windows 泛化到 Windows / Linux 共用路径；执行 `rustfmt`、`cargo check` 和 Linux target 编译验证
- 验证结果：`rustfmt --edition 2024 src/app/init.rs src/app/app_menu.rs src/app/ui/layout.rs` 通过；`cargo check` 通过；`cargo check --target x86_64-unknown-linux-gnu` 下载依赖后因本机缺少该 Rust target 而未完成；Linux GUI 手工验证尚未执行
- 风险/待办：Linux 最终显示与点击行为仍需在实际桌面环境确认；若要继续做 Linux target 编译，需要使用带 `rustup` 或预装 `x86_64-unknown-linux-gnu` target 的工具链

## 2026-07-09 刷新环境记录到 SFTP 交互体验修正

- 目的：在修改 SFTP 页面传输默认位置、传输信息呈现和列表点击习惯前，确认当前项目环境、入口和验证边界
- 改动范围：`src/app/actions/sftp.rs`，`src/app/views/sftp_panel.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 `Cargo.toml`、`.github/workflows/ci.yml`、SFTP action 层和 SFTP 双栏页面；确认主技术栈与依赖版本未变，本轮不新增依赖、不联网、不使用多 agent
- 验证结果：待执行 `rustfmt`、`cargo check`、`cargo test` 和 tracking docs validator；真实 SFTP 双栏交互仍需 GUI 手工验证
- 风险/待办：全局 Transfers 弹窗保留菜单入口，本轮只取消上传/下载启动时的自动弹窗；列表二次点击行为需要实机确认手感

## 2026-07-09 完成 SFTP 交互体验修正环境验证

- 目的：在 SFTP 页面交互修正完成后，把实际编译测试结果和剩余验证边界回写到环境记忆
- 改动范围：`src/app/actions/sftp.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：将下载默认目录改为右侧本地浏览器当前目录；上传保持左侧远端当前目录并取消自动弹出传输历史；将远端/本地列表改为单击选中、再次点击目录打开；将 SFTP 页面底部传输行压缩为单行显示
- 验证结果：`rustfmt --edition 2024 src/app/actions/sftp.rs src/app/views/sftp_panel.rs src/app/views/sftp_panel/transfer_panel.rs` 通过；`cargo check` 通过；`cargo test` 通过，25 个测试全部通过；tracking docs validator 通过
- 风险/待办：GUI 手工验证和真实 SSH/SFTP 连接验证未执行；传输弹窗仍可通过菜单手动打开，本轮只取消自动弹窗

## 2026-07-09 刷新环境记录到终端路径识别与 SFTP 跳转

- 目的：在实现终端路径识别和 Command/Ctrl+单击跳转 SFTP 前，确认当前项目环境、已有 shell 工作目录链路和验证边界
- 改动范围：`src/terminal/highlight.rs`，`src/sftp/path.rs`，`src/sftp.rs`，`src/app/actions/session.rs`，`src/app/actions/terminal.rs`，`src/app/actions/sftp.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/workspace/workspace.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查终端 URL 命中逻辑、shell 工作目录采集、SFTP 列目录返回结构和工作区 SFTP 切页行为；确认主技术栈与依赖版本未变，本轮不新增依赖、不联网、不使用多 agent；将验证命令收敛为 `rustfmt`、`cargo check`、定向单元测试、`cargo test`、`git diff --check` 与 tracking docs validator
- 验证结果：已确认终端点击当前只识别 URL，现有 shell cwd 采集链路可作为相对路径解析基准；SFTP 导航目前缺少统一绝对化与文件自动定位，需要本轮补齐
- 风险/待办：真实 GUI 点击体验和实际 SSH / SFTP 会话行为仍需手工确认；当前路径识别规则需要避免把普通文本误判成路径

## 2026-07-09 完成终端路径识别与 SFTP 跳转环境验证

- 目的：在终端路径识别和 Command/Ctrl+单击 SFTP 跳转实现后，把实际编译测试结果和剩余 GUI / 联机边界回写到环境记忆
- 改动范围：`src/terminal/highlight.rs`，`src/sftp/path.rs`，`src/sftp.rs`，`src/app.rs`，`src/app/core/types.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/workspace/workspace.rs`，`src/app/actions/sftp.rs`，`src/app/actions/terminal.rs`，`src/app/actions/session.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：为远端路径新增归一化和绝对化 helper；扩展终端 hover / click 目标为 URL 或路径，并对路径 token 裁掉尾部标点与 `:行号` / `:列号`；让 Command/Ctrl+单击路径时打开当前 group 的 SFTP 页面，按当前 shell 工作目录把相对路径解析成绝对路径，并在目录列表返回后自动定位到目标目录或文件；同时抑制显式点击跳转被“切到 SFTP 页自动同步 cwd”逻辑覆盖
- 验证结果：`rustfmt --edition 2024 src/sftp/path.rs src/sftp.rs src/app.rs src/app/core/types.rs src/app/lifecycle/init.rs src/app/lifecycle/event_loop.rs src/app/workspace/workspace.rs src/app/actions/sftp.rs src/app/actions/terminal.rs src/app/actions/session.rs src/terminal/highlight.rs` 通过；`cargo check` 通过；`cargo test --quiet terminal::highlight -- --nocapture` 通过；`cargo test --quiet normalize_remote_path -- --nocapture` 通过；`cargo test --quiet resolve_remote_path -- --nocapture` 通过；`cargo test --quiet` 通过，40 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过
- 风险/待办：GUI 手工验证和真实 SSH / SFTP 联机验证未执行；复杂 shell 转义、符号链接大小写差异和更激进的路径启发式仍留待后续迭代

## 2026-07-09 完成非交互文本可选复制环境验证

- 目的：在将主要非交互文本改为可拖选复制后，把编译、测试和文档校验结果回写到环境记忆
- 改动范围：`src/app/views.rs`，`src/app/views/layout.rs`，`src/app/views/helpers.rs`，`src/app/views/monitoring.rs`，`src/app/views/terminal_panel.rs`，`src/app/views/sftp_panel.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`src/app/dialogs.rs`，`src/app/dialogs/selector.rs`，`src/app/dialogs/transfers.rs`，`src/app/dialogs/delete_confirm.rs`，`src/app/dialogs/settings/about.rs`，`src/app/dialogs/settings/sync.rs`，`src/app/dialogs/settings/proxy.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：新增可选文本 helper；将连接日志、错误详情、路径列表、文件列表、传输详情、监控指标和说明文字迁移到 `TextView::selectable(true)`；按钮、菜单、排序表头和输入框 label 保持控件语义不变
- 验证结果：`rustfmt --edition 2024` 覆盖本轮修改 Rust 文件并通过；`cargo check` 通过；`cargo test --quiet` 通过，45 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过；仍保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 手工拖选复制验证未执行；第三方 setting 组件内部 description、按钮/菜单/表头等控件 label 未强行转为可选文本，以避免破坏点击和排序语义

## 2026-07-09 完成终端选区固定 viewport 位置环境验证

- 目的：在修正终端 frozen selection 随输出刷新移动的问题后，把编译、测试和剩余 GUI 验证边界回写到环境记忆
- 改动范围：`src/terminal.rs`，`src/terminal/element.rs`，`src/app/actions/terminal.rs`，`src/app/lifecycle/event_loop.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：将 frozen selection 的 cell/highlight 坐标从 bottom-index/history delta 改为固定 viewport row/col；移除渲染层对 live selection 和 history 变化的重映射；保留选择时文本快照作为复制来源；取消 backend output 自动清理 frozen selection
- 验证结果：`rustfmt --edition 2024 src/terminal.rs src/terminal/element.rs src/app/actions/terminal.rs src/app/lifecycle/event_loop.rs` 通过；`cargo check` 通过；`cargo test --quiet frozen_ -- --nocapture` 通过，3 个相关测试全部通过；`cargo test --quiet` 通过，46 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过；仍保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 手工持续输出拖选验证未执行；窗口 resize 后选区会按当前 rows/cols 裁剪，仍需实机确认视觉体验

## 2026-07-09 刷新环境记录到终端选区去除旧文字冻结

- 目的：在修正 frozen selection 仍覆盖旧文字、选择高亮仍随刷新丢失的问题前，确认当前项目环境和验证边界
- 改动范围：`src/terminal.rs`，`src/terminal/element.rs`，`src/app/actions/terminal.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查终端 frozen selection 数据结构、鼠标选择捕获逻辑和渲染层 `layout_grid()`；确认主技术栈与依赖版本未变，本轮不新增依赖、不联网、不使用多 agent；将验证命令收敛为 `rustfmt`、`cargo check`、定向单元测试、`cargo test`、`git diff --check` 与 tracking docs validator
- 验证结果：已确认根因是渲染层仍用 frozen cell 覆盖 live cell，且 selection 背景依赖 live cell 遍历；修正后需重新执行本机验证
- 风险/待办：GUI 手工持续输出拖选验证仍需实机执行；复制内容将继续来自选择时文本快照，终端画面文字不再冻结

## 2026-07-09 完成终端选区去除旧文字冻结环境验证

- 目的：在去除 frozen cell 覆盖并改为独立 selection 背景后，把实际编译测试结果和剩余 GUI 验证边界回写到环境记忆
- 改动范围：`src/terminal.rs`，`src/terminal/element.rs`，`src/app/actions/terminal.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：将 `TerminalFrozenSelection` 收窄为只保存 tab、viewport selection 和复制文本；选择捕获不再保存旧 terminal cell 或 highlight；渲染层移除 frozen cell/highlight 覆盖，改为按 captured selection 独立绘制背景矩形并继续绘制实时 terminal snapshot
- 验证结果：`rustfmt --edition 2024 src/terminal.rs src/terminal/element.rs src/app/actions/terminal.rs` 通过；`cargo check` 通过；`cargo test --quiet frozen_ -- --nocapture` 通过，3 个相关测试全部通过；`cargo test --quiet` 通过，46 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过；仍保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 手工持续输出拖选验证仍需实机执行；复制内容仍是选择时文本快照，终端画面文字不再冻结

## 2026-07-10 刷新环境记录到仓库级 AGENTS 指令

- 目的：在新增仓库级 agent 指令文件前，确认 Codex 默认加载规则和本项目约束入口
- 改动范围：`AGENTS.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：检索官方 Codex 文档确认默认 repo guidance 文件是 `AGENTS.md`，并确认 `.agent` 不是默认加载文件名；复查项目 release tag 规则、Rust 模块布局和 tracker 记录要求
- 验证结果：已确认本轮不修改 Rust 源码、不新增依赖；后续只需执行 `git diff --check` 与 tracking docs validator
- 风险/待办：如果未来确实要使用 `.agent` 作为文件名，需要在 Codex 配置中加入 fallback 文件名并重启/新开会话

## 2026-07-10 完成仓库级 AGENTS 指令环境验证

- 目的：在新增仓库级 `AGENTS.md` 后，把文档验证结果和剩余使用边界回写到环境记忆
- 改动范围：`AGENTS.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：新增 `AGENTS.md`，写入 Codex 默认指令文件说明、Rust 现代模块布局、项目模块例外、implementation tracking、验证命令、release tag 规则和 git hygiene；刷新项目地图入口
- 验证结果：`git diff --check` 通过；tracking docs validator 通过；本轮未修改 Rust 源码，未运行 `cargo check` 或 `cargo test`
- 风险/待办：`.agent` 仍不是默认 Codex 指令文件名；如需该名字必须配置 fallback

## 2026-07-10 刷新环境记录到 Settings 信息架构整理

- 目的：在整理设置页前，确认当前项目环境、Settings 入口和验证边界
- 改动范围：`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/general.rs`，`src/app/dialogs/settings/`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 `AGENTS.md`、环境记录、项目地图和 Settings 现状；确认 General 页混合外观、字体、终端、工作区、监控、语言和 reset layout 多类设置，本轮将拆为 focused pages / modules；确认不新增依赖、不改配置 schema、不联网、不使用多 agent
- 验证结果：待执行 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；GUI 设置页实际交互需人工确认
- 风险/待办：页面拆分会改变左侧导航结构；Settings 组件 icon 枚举和移动后的闭包捕获需通过编译验证

## 2026-07-10 完成 Settings 信息架构整理环境验证

- 目的：在拆分 Settings General 页后，把编译测试结果和剩余 GUI 验证边界回写到环境记忆
- 改动范围：`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/general.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/font_page.rs`，`src/app/dialogs/settings/terminal.rs`，`src/app/dialogs/settings/workspace.rs`，`src/app/dialogs/settings/monitoring.rs`，`src/app/dialogs/settings/language.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：删除旧的 `settings/general.rs`；新增 Appearance、Fonts、Terminal、Workspace、Monitoring 和 Language focused pages；更新 Settings 左侧页面顺序和中英文页面标题；刷新项目地图中的 settings 子页面索引
- 验证结果：`rustfmt --edition 2024` 覆盖本轮修改 Rust 文件并通过；`cargo check` 通过；`cargo test --quiet` 通过，46 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过；仍保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 设置页手工点击验证未执行；`sync.rs` 与 `proxy.rs` 仍可在后续继续做局部拆分

## 2026-07-10 刷新环境记录到 SFTP 页面快捷键关闭页面

- 目的：在修改 SFTP 页面快捷键 toggle 行为前，确认当前项目环境、SFTP 页面生命周期入口和验证边界
- 改动范围：`src/app/workspace/workspace.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 `AGENTS.md`、环境记录、项目地图、`ToggleSftpZoom` 绑定和 `AxShell::toggle_active_sftp_page()`；确认当前已在 SFTP 页面时只切回终端，未关闭 `sftp_page_open`；本轮不新增依赖、不联网、不使用多 agent
- 验证结果：待执行 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；真实 GUI 快捷键体验仍需人工确认
- 风险/待办：菜单文案仍为 Open SFTP Page，本轮只修改行为；如需文案改为 Toggle/Close 需另行处理

## 2026-07-10 完成 SFTP 页面快捷键关闭页面环境验证

- 目的：在修正 SFTP 页面快捷键 toggle 行为后，把编译测试结果和剩余 GUI 验证边界回写到环境记忆
- 改动范围：`src/app/workspace/workspace.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：将 `AxShell::toggle_active_sftp_page()` 的当前 SFTP 页面分支改为复用 `close_sftp_page()`，因此再次触发 `ToggleSftpZoom` 会关闭当前 group 的 SFTP 页面并返回终端焦点；未改快捷键配置、菜单文案或 SFTP 后端
- 验证结果：`rustfmt --edition 2024 src/app/workspace/workspace.rs` 通过；`cargo check` 通过；`cargo test --quiet` 通过，50 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过；仍保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 手工快捷键验证未执行；菜单文案仍显示 Open SFTP Page，后续如需要可单独改为 Toggle/Close 文案

## 2026-07-10 刷新环境记录到终端历史滚动背景色块残留修复

- 目的：在修复终端滚动历史时 ANSI 背景色块残留问题前，确认当前项目环境、终端渲染入口和验证边界
- 改动范围：`src/terminal/element.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：复查 `src/terminal.rs`、`src/terminal/element.rs` 和 `src/app/actions/terminal.rs`；确认终端 snapshot 会随 `display_offset` 更新，但元素 paint 开始时没有显式清理整个终端 bounds，只叠加绘制显式 ANSI 背景色块和文字；本轮不新增依赖、不联网、不使用多 agent
- 验证结果：待执行 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；真实 ANSI 背景色滚动体验仍需人工确认
- 风险/待办：GUI 手工复现需要运行会输出背景色的命令或程序；本轮只处理绘制残留，不改变终端 buffer 语义

## 2026-07-10 完成终端历史滚动背景色块残留环境验证

- 目的：在修复终端绘制层旧 ANSI 背景色块残留后，把编译测试结果和剩余 GUI 验证边界回写到环境记忆
- 改动范围：`src/terminal/element.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 执行内容：在 `TerminalElement::paint()` 中为每帧先填充当前终端元素 bounds 的主题背景，再叠加本帧 ANSI 背景矩形、文字、下划线、custom block、IME composition 和光标，避免旧帧色块在滚动历史时残留
- 验证结果：`rustfmt --edition 2024 src/terminal/element.rs` 通过；`cargo check` 通过；`cargo test --quiet` 通过，50 个测试全部通过；`git diff --check` 通过；tracking docs validator 通过；仍保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 手工历史滚动复现验证未执行；若后续仍出现残留，需要进一步检查 GPUI 层元素失效或 terminal scrollbar repaint 时序
## 2026-07-10 刷新环境记录到 SFTP 生命周期回收

- 触发原因：用户确认资源回收改造按阶段逐项实施，第一阶段处理 SFTP worker、传输 task 与远程编辑 watcher 的关闭所有权
- 执行内容：复查 `Cargo.toml`、`.github/workflows/ci.yml`、`src/sftp.rs`、`src/app/actions/sftp.rs`、`src/app/actions/session.rs` 和现有环境记录；确认可复用现有 `tokio` 的 `JoinHandle`、`JoinSet` 与 timeout，无需新增依赖或联网
- 影响文件：`src/sftp.rs`，`src/app/actions/sftp.rs`，`src/app/actions/session.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustc 1.96.1` 与 `cargo 1.96.1` 可用；确认默认验证命令为 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 与 tracking docs validator；真实 SFTP 生命周期压测待实现后执行
- 对 plan 的更新：允许实施“主 worker 有界关闭，关闭时由 `JoinSet` 取消并回收传输、远程编辑 watcher 和自动上传子 task”

## 2026-07-10 完成 SFTP 生命周期回收环境验证

- 触发原因：SFTP worker 回收和传输中关闭确认已实现，需要记录实际验证结果与运行时手工边界
- 执行内容：新增 worker 共享关闭所有权与 `JoinSet` 子任务收口；统一 group 释放入口；增加传输中关闭确认、记忆偏好和 Terminal 设置页恢复入口；补充配置归一化和 worker 子任务取消测试
- 影响文件：`src/sftp.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/sftp.rs`，`src/app/actions/session.rs`，`src/app/workspace/workspace.rs`，`src/app/core/types.rs`，`src/app/dialogs.rs`，`src/app/dialogs/sftp_close_confirm.rs`，`src/app/dialogs/settings/terminal.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024`、`cargo check`、`cargo test --quiet`（57 个通过）和 `git diff --check` 均通过；tracking docs validator 待复验；仍保留既有 `block v0.1.6` future-incompat warning
- 对 plan 的更新：真实 SSH/SFTP 服务与 GUI 手工验证需确认关闭弹窗、三种记忆选项、后台传输保护和取消后 watcher/连接停止

## 2026-07-10 刷新环境记录到 SFTP 二次快捷键确认

- 触发原因：用户要求传输中关闭 SFTP 时仍始终显示确认，并让再次按打开 SFTP 的快捷键确认上次记住的选择
- 执行内容：复查 SFTP 关闭弹窗、`ToggleSftpZoom` / `ClosePane` action 路由、Terminal 设置页和 `ConfigStore`；确认当前持久化字段直接绕过确认，需改为仅保存二次 `ToggleSftpZoom` 的默认动作
- 影响文件：`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/views/layout.rs`，`src/app/workspace/workspace.rs`，`src/app/dialogs/sftp_close_confirm.rs`，`src/app/dialogs/settings/terminal.rs`，`src/config/store.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustc 1.96.1` 与 `cargo 1.96.1` 可用；确认本轮验证命令为 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 与 tracking docs validator；GUI 交互验证待实现后执行
- 对 plan 的更新：允许实施“首次关闭永远打开确认；仅确认框打开时，第二次 `ToggleSftpZoom` 根据记住动作确认；无默认动作时保持弹窗等待用户点击”

## 2026-07-10 完成 SFTP 二次快捷键确认环境验证

- 目的：将传输中关闭 SFTP 的已记住动作改为二次快捷键确认，同时保留首次确认弹窗。
- 改动范围：`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/workspace/workspace.rs`，`src/app/dialogs/sftp_close_confirm.rs`，`src/app/dialogs/settings/terminal.rs`，`src/config/store.rs`，`locales/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 执行内容：确认弹窗保存发起关闭的 group；有活跃传输时首次关闭始终打开弹窗；再次按 `ToggleSftpZoom` 仅在匹配弹窗打开时使用已保存动作；`ask` 表示不绑定二次快捷键动作；Terminal 设置页提供该默认动作设置。
- 验证结果：`rustfmt --edition 2024` 覆盖本轮 Rust 修改通过；`cargo check` 通过；`cargo test --quiet` 通过，57 个测试通过；`git diff --check` 通过；tracking docs validator 通过。保留 `block v0.1.6` 的既有 future-incompat warning。
- 风险/待办：尚未在真实 SFTP 传输和 GUI 中手工验证首次弹窗、二次快捷键、`ask` 状态和 `ClosePane` 边界。

## 2026-07-10 初始化深度休眠第一阶段环境预检

- 时间：2026-07-10 12:38 +0800
- 触发原因：用户要求把休眠防线写入项目文档，并开始按阶段实现。
- 执行内容：复查 `Cargo.toml`、环境记录、事件泵、监控、配置入口和当前 GPUI checkout；确认可复用 `Context::observe_window_activation`、现有单一事件泵和 `ConfigStore`，无需新增依赖或联网。
- 影响文件：`src/app.rs`，`src/app/state/`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/dialogs/settings/monitoring.rs`，`src/config/store.rs`，`docs/resource-lifecycle.md`，`docs/resource-lifecycle.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：本机 `rustc 1.96.1`、`cargo 1.96.1` 可用；已确认默认验证命令为 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；代码验证待实施后执行。
- 对 plan 的更新：允许实现“失焦立即停监控/主题/光标，保留低频 backend drain；失焦 5 分钟深睡，可配置关闭/1/5/15/30 分钟”，不在本阶段断开 SSH/PTY/SFTP。

## 2026-07-10 完成深度休眠第一阶段环境验证

- 时间：2026-07-10 12:52 +0800
- 触发原因：第一阶段深睡实现需要回写实际编译、测试与运行时手工边界。
- 执行内容：使用 GPUI `observe_window_activation` 驱动窗口生命周期；新增持久化的深睡阈值归一化、后台事件泵节流、监控/远程 probe/主题/光标降载和 SFTP idle sweep 节流；新增 Monitoring 设置入口及中英文资源生命周期、用户指南说明。
- 影响文件：`src/app.rs`，`src/app/state/`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/workspace/workspace.rs`，`src/app/dialogs/settings/monitoring.rs`，`src/config/store.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/resource-lifecycle.md`，`docs/resource-lifecycle.en.md`，`docs/user-guide.md`，`docs/user-guide.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024`、`cargo check`、定向 lifecycle/config 测试、`cargo test --quiet`（63 个通过）、`git diff --check` 和 tracking docs validator 均通过；保留既有 `block v0.1.6` future-incompat warning。
- 对 plan 的更新：GUI 失焦/恢复与真实 SSH 高输出仍需手工验证；后续 SFTP 深睡回收先实现 pin/refcount，不能以本阶段的“无活跃传输”替代远程编辑保护。

## 2026-07-10 初始化 SFTP pin/refcount 环境预检

- 时间：2026-07-10 13:04 +0800
- 触发原因：用户确认继续实现 SFTP pin/refcount 和深睡按需回收。
- 执行内容：复查 `Cargo.toml`、现有环境记录、`src/sftp.rs`、SFTP UI action、删除确认入口和事件泵；确认项目已有 `Arc`、`Mutex`、`JoinSet`、tokio channel 和 worker 有界关闭，无需新增依赖或联网。
- 影响文件：`src/sftp.rs`，`src/app/actions/sftp.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/dialogs/delete_confirm.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：本机 `rustc 1.96.1`、`cargo 1.96.1` 可用；默认验证命令为 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；代码验证待实施后执行。
- 对 plan 的更新：允许在 worker 内实施“入队 pin、child task pin、watcher pin 和 UI 查询 pin 数”；自动回收只在 pin 为零时执行。

## 2026-07-10 完成 SFTP pin/refcount 环境验证

- 时间：2026-07-10 13:15 +0800
- 触发原因：SFTP worker pin/refcount 与深睡回收实现完成，需要记录实际验证和手工边界。
- 执行内容：在 SFTP worker 中实现 RAII work pin，收口 UI command 发送；将 pin 覆盖 queued command、短操作、传输、自动上传和远程编辑 watcher；普通 idle 与深睡回收统一以 pin 为准，并保持用户显式关闭、取消和重连的强制关闭语义。
- 影响文件：`src/sftp.rs`，`src/app/actions/sftp.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/dialogs/delete_confirm.rs`，`docs/resource-lifecycle.md`，`docs/resource-lifecycle.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024`、`cargo check`、pin/reclaim 定向测试、`cargo test --quiet`（66 个通过）、`git diff --check` 和 tracking docs validator 均通过；保留既有 `block v0.1.6` future-incompat warning。
- 对 plan 的更新：真实 SFTP 传输、远程编辑和深睡回收仍需 GUI 手工验证；下一阶段转向 SSH/local PTY/backend query 的取消所有权与有界 shutdown。
## 2026-07-10 刷新环境记录到 terminal backend shutdown

- 触发原因：SFTP worker pin/refcount 与深睡回收已完成，本轮切换到 SSH/local PTY 后台执行资源的关闭与回收。
- 执行内容：复查 `src/terminal.rs`、`src/backend/ssh.rs`、`src/backend/local.rs`、`src/app/actions/session.rs`、`src/app/workspace/workspace.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/lifecycle/startup.rs` 和 `portable_pty` 的 `ChildKiller` 接口；确认可使用已有 Tokio runtime、`JoinSet` 和 `ChildKiller` 实现有界关闭，不需要新依赖。
- 影响文件：`src/terminal.rs`，`src/backend/ssh.rs`，`src/backend/local.rs`，`src/app/actions/session.rs`，`src/app/workspace/workspace.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/lifecycle/startup.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认本轮验证命令为 `rustfmt --edition 2024` 覆盖变更 Rust 文件、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；完整 GUI 验证仍需要 SSH 主机及本地 shell。
- 对 plan 的更新：允许实施“SSH 主任务 timeout/abort、SSH child task JoinSet、本地 PTY child kill + 后台 join、tab/retry/window shutdown 统一入口”。
## 2026-07-10 完成 terminal backend shutdown 环境验证

- 触发原因：SSH/local PTY 后台资源的统一关闭实现、回归测试和 tracking 收口均已完成。
- 执行内容：实现 `BackendShutdown` 控制器；SSH 使用有界 graceful close、超时 abort 和 `JoinSet` 子任务回收；local PTY 使用 `ChildKiller` 与后台 reader/writer join；窗口关闭、Quit、tab close/retry 和自然关闭事件接入同一控制器，并抑制主动关闭实例的旧事件。
- 影响文件：`src/terminal.rs`，`src/backend/ssh.rs`，`src/backend/local.rs`，`src/app/actions/session.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/lifecycle/startup.rs`，`src/app/input/app_menu.rs`，`docs/resource-lifecycle.md`，`docs/resource-lifecycle.en.md`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024`、`cargo check`、`cargo test --quiet`（69 项通过）、`git diff --check` 和 tracking docs validator 均通过；保留既有 `block v0.1.6` future-incompat warning。
- 对 plan 的更新：本轮环境结论为“可在 UI 不阻塞的前提下有界回收 terminal backend”；真实 SSH 连接、失联 timeout、local shell 派生进程和 OS 强制退出仍需实机验证。
## 2026-07-10 刷新环境记录到 workspace tab 可见性

- 触发原因：用户报告顶部标签栏溢出后，快捷键或 pane focus 切换到隐藏标签不会自动显示当前标签。
- 执行内容：复查 GPUI `ScrollHandle`、`TabBar::track_scroll`、`src/app/workspace/workspace.rs`、`src/app/actions/pane.rs`、`src/app/actions/session.rs` 和 `src/app/views/tab_bar.rs`；确认无需新增依赖，正确实现需要从 `workspace_tabs` 的渲染序列取 index，而不能使用 `self.tabs` 的内部 terminal tab index。
- 影响文件：`src/app/workspace/workspace.rs`，`src/app/actions/pane.rs`，`src/app/actions/session.rs`，`src/app/views/tab_bar.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认本轮验证命令为 `rustfmt --edition 2024` 覆盖变更 Rust 文件、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；GUI 手工验证需要足够多的工作区标签。
- 对 plan 的更新：允许实施“统一当前 workspace tab 可见性 helper；接入 workspace 切换、pane focus、tab 创建和关闭后自动选中；为 SFTP 插入导致的索引偏移添加单元测试”。

## 2026-07-10 完成 workspace tab 可见性环境验证

- 触发原因：顶部标签溢出后的自动可见性修复已完成，需要记录实际验证结果和 GUI 验证边界。
- 执行内容：确认 GPUI `ScrollHandle::scroll_to_item()` 按 child 渲染序列最小滚动；修正内部 terminal tab 索引与顶部 workspace tab 顺序不一致的问题，并覆盖页面/组切换、新建会话、tab 激活和关闭后的自动选中路径。
- 影响文件：`src/app/workspace/workspace.rs`，`src/app/actions/session.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024`、定向单元测试、`cargo check`、`cargo test --quiet`（70 项）、`git diff --check` 均通过；保留既有 `block v0.1.6` future-incompat warning。GUI 手工验证未执行。
- 对 plan 的更新：运行环境和工具链无变化；后续仅需在桌面端验证多标签溢出时的实际视觉行为。

## 2026-07-10 刷新环境记录到终端滚动槽与平台菜单布局

- 触发原因：用户反馈终端滚动条遮挡内容，并要求 Windows/Linux 应用菜单独占顶部。
- 执行内容：复查 `src/app/views/terminal_panel.rs`、`src/app/views/layout.rs`、terminal scrollbar handle 和 GPUI Component `Scrollbar`；确认 16px scrollbar 轨道当前绝对覆盖 terminal pane，且平台菜单位于随 sidebar 宽度变化的 workspace 主列。
- 影响文件：`src/app/views/terminal_panel.rs`，`src/app/views/layout.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：本机 Rust 工具链可用；确认仍以 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator 作为验证命令；GUI 手工验证需要 Windows/Linux 桌面会话。
- 对 plan 的更新：允许实施“固定 16px scrollbar gutter 和全宽菜单根行”，不改终端缓冲区、滚动状态或菜单动作。

## 2026-07-10 完成终端滚动槽与平台菜单布局环境验证

- 时间：2026-07-10 14:37 +0800
- 触发原因：终端滚动槽与平台菜单布局实现完成，需要记录实际本机验证结果。
- 执行内容：terminal pane 的内容和滚动轨道改为独立弹性列与固定 16px gutter；平台菜单提升为 Windows/Linux 根布局的独立全宽行，避免受侧栏布局影响。
- 影响文件：`src/app/views.rs`，`src/app/views/terminal_panel.rs`，`src/app/views/layout.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024`、`cargo check`、`cargo test --quiet`（70 项）、`git diff --check` 和 tracking docs validator 均通过；保留既有 `block v0.1.6` future-incompat warning。
- 对 plan 的更新：运行环境和依赖策略保持不变；真实 Windows/Linux GUI 仍需验证长行末尾、滚动条拖动和侧栏状态。

## 2026-07-10 初始化 terminal tab UI 状态回收环境预检

- 时间：2026-07-10 14:56 +0800
- 触发原因：关闭 terminal tab 后的 UI 状态残留需要修复，避免 tab ID 缓存增长和已关闭 pane 的鼠标命中。
- 执行内容：复查 `Cargo.toml`、CI、`src/app/actions/session.rs`、`src/app/actions/terminal.rs`、`src/app.rs` 和现有环境记录；确认本轮只需复用 `AxShell` 的状态字段与统一关闭入口，无需新增依赖或联网。
- 影响文件：`src/app/actions/session.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：本机 `rustc 1.96.1`、`cargo 1.96.1` 可用；默认验证命令为 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；GUI 手工验证待实现后执行。
- 对 plan 的更新：允许实施“按 tab ID 清除 scrollbar/bounds/hover/IME/frozen selection/progress；整组关闭逐一回收；仅关闭 active tab 时复位全局选择状态”。

## 2026-07-10 完成 terminal tab UI 状态回收环境验证

- 时间：2026-07-10 15:01 +0800
- 触发原因：关闭 terminal tab 后的 UI 状态回收完成，需要记录本机验证与 GUI 验证边界。
- 执行内容：在统一关闭入口按 tab ID 删除 scrollbar/bounds 缓存和关联短期状态；整组关闭逐一回收，避免关闭分支遗漏；保留现有 backend `Close`、pane 选中和 SFTP worker 语义。
- 影响文件：`src/app/actions/session.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 src/app/actions/session.rs`、`cargo check`、`cargo test --quiet`（70 项）、`git diff --check` 和 tracking docs validator 均通过；保留既有 `block v0.1.6` future-incompat warning。
- 对 plan 的更新：环境与依赖策略不变；GUI 仍需验证关闭连接中、hover、IME 预编辑或选择中的 tab。

## 2026-07-10 初始化 backend event queue 有界化环境预检

- 时间：2026-07-10 15:43 +0800
- 触发原因：审查发现 terminal backend、SFTP 和 listener 共用的 UI 事件队列无上限，可能在高输出或 UI 滞后时造成内存无限积压。
- 执行内容：复查 `Cargo.toml`、CI、runtime、event pump、`src/terminal.rs`、local/SSH backend 和 SFTP worker；初步判断标准库 `sync_channel` 可提供固定上限与背压，无需新增依赖或联网。
- 影响文件：`src/terminal.rs`，`src/app/lifecycle/init.rs`，`src/app/state/runtime.rs`，`src/backend/local.rs`，`src/backend/ssh.rs`，`src/backend/ssh/connection.rs`，`src/sftp.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：本机 `rustc 1.96.1`、`cargo 1.96.1` 可用；默认验证命令为 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；GUI 高输出验证待实现后执行。
- 对 plan 的更新：允许实施“固定 256 条 backend event queue；后台生产者在满队列时背压；UI thread 的 CWD/title metadata 用 `try_send` 避免自阻塞”。

## 2026-07-10 完成 backend event queue 有界化环境验证

- 时间：2026-07-10 16:04 +0800
- 触发原因：backend 到 UI 的事件队列有界化实现完成，需要记录实际运行时方案、自动化验证和人工验证边界。
- 执行内容：以现有 Tokio `mpsc::channel(256)` 替代无界标准库 channel；本地 PTY 线程使用 `blocking_send`，SSH、SFTP 和配置同步异步任务使用 `send().await`，UI 内 title/CWD metadata 使用 `try_send`；限制每轮 event pump drain 为 256 条，并将 SFTP 目录读取超时与事件投递等待分离。
- 影响文件：`src/terminal.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/state/runtime.rs`，`src/app/syncing/config_sync.rs`，`src/backend/local.rs`，`src/backend/ssh.rs`，`src/backend/ssh/connection.rs`，`src/sftp.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024`、`cargo check`、`cargo test --quiet`（71 项）、`git diff --check` 和 tracking docs validator 通过；保留既有 `block v0.1.6` future-incompat warning。
- 对 plan 的更新：队列上限为事件数而非字节数，单个大型目录列表或预览仍需单独控制；GUI 侧待验证本地/SSH 高输出、关闭响应和 SFTP 传输进度。

## 2026-07-10 初始化 SFTP 目录浏览内存上限环境预检

- 时间：2026-07-10 16:23 +0800
- 触发原因：有界 backend event queue 后，超大目录仍可在单个 `SftpEntries` 事件和 UI 克隆/排序中形成内存峰值。
- 执行内容：复查 `src/sftp.rs`、event loop、SFTP view 与锁定的 `russh-sftp 2.3.0`；确认高层 `read_dir()` 先累积至 EOF，必须使用公开 `RawSftpSession::opendir/readdir/close` 在项目侧跨批次截断。
- 影响文件：`src/sftp.rs`，`src/terminal.rs`，`src/app/lifecycle/event_loop.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：本机 Rust 工具链和锁定依赖源码可用；无需联网、无需新增依赖、未使用多 agent；实现与回归测试待执行。
- 对 plan 的更新：允许实施“raw SFTP 分批浏览，最多保留 2,000 条或 2 MiB 名称/路径数据，关闭目录句柄并显示截断状态”。

## 2026-07-10 完成 SFTP 目录浏览内存上限环境验证

- 时间：2026-07-10 16:51 +0800
- 触发原因：超大目录和目录预览的受限读取已实现，需要记录 timeout 语义、自动化验证和 GUI 验证边界。
- 执行内容：普通浏览、初始目录和 reveal path 均改为 raw SFTP 按 `READDIR` 批次采集，达到 2,000 条或 2 MiB 名称/路径预算立即关闭目录句柄；目录预览限制为 200 条及 128 KiB 内容预算。临时 raw session 建立和目录读取共用 30 秒 timeout，超时 future 析构会话；异常长 UTF-8 路径也按字符边界截断预览正文。
- 影响文件：`src/sftp.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024 src/sftp.rs`、`cargo check`、`cargo test --quiet sftp::lifecycle_tests`（6 项）、`cargo test --quiet`（75 项）、`git diff --check` 和 tracking docs validator 通过；保留既有 `block v0.1.6` future-incompat warning。GUI 手工验证未执行。
- 对 plan 的更新：环境与依赖策略保持不变；需要真实 SSH/SFTP 目录验证截断提示、普通/超大目录浏览、目录预览、文件预览和传输不回归。

## 2026-07-10 完成 SFTP 目录分页加载环境验证

- 时间：2026-07-10 17:41 +0800
- 触发原因：用户要求超大 SFTP 目录支持动态加载，同时不能取消上一项的总内存上限。
- 执行内容：SFTP worker 使用持久 `BrowseCursor` 保存 raw SFTP session、目录句柄和已收取的待发条目；每次“加载更多”最多追加 250 项。cursor 在 EOF、2,000 项/2 MiB 总预算、路径切换、worker 关闭、空闲 worker 重建和读取失败时关闭；页结果事件携带追加、是否还有更多和预算截断状态。远端列表页脚新增显式加载更多按钮与总上限提示，继续使用虚拟列表渲染。
- 影响文件：`src/sftp.rs`，`src/terminal.rs`，`src/app/actions/session.rs`，`src/app/actions/sftp.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/views/sftp_panel.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024`、`cargo check`、`cargo test --quiet sftp::lifecycle_tests`（7 项）、`cargo test --quiet`（76 项）、`git diff --check` 和 tracking docs validator 通过；保留既有 `block v0.1.6` future-incompat warning。GUI 手工验证未执行。
- 对 plan 的更新：环境与依赖策略保持不变；需要真实 SFTP 目录验证多次加载、EOF 自动隐藏按钮、总上限提示、导航和 idle worker 重建后的 cursor 回收。

## 2026-07-10 刷新环境记录到标签宽度和颜色显示策略

- 时间：2026-07-10 19:23 +0800
- 触发原因：用户要求限制顶部标签宽度，并把非激活标签是否显示状态色做成可配置设置。
- 执行内容：复查 `Cargo.toml`、顶部 tab bar、工作区设置页和配置存储；确认可以复用 GPUI 尺寸 API、主题 token 与现有 Settings `Switch`，无需新增依赖或联网。
- 影响文件：`src/app/core/constants.rs`，`src/app/views/tab_bar.rs`，`src/app/dialogs/settings/workspace.rs`，`src/config/store.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：本机 Rust/Cargo 环境可用；默认验证为 `rustfmt`、配置定向测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；GUI 视觉检查待实现后执行。
- 对 plan 的更新：允许实施标签最大宽度和“为非激活标签着色”持久化开关；主题和 workspace/tab 数据模型保持不变。

## 2026-07-10 完成标签宽度和颜色显示策略环境验证

- 时间：2026-07-10 19:28 +0800
- 触发原因：标签宽度和可配置颜色策略实现完成，需要记录实际环境验证结果。
- 执行内容：所有 workspace 标签增加 220px 最大宽度；新增 `color_inactive_tabs` 配置和工作区设置开关，默认仅激活标签显示状态色，非激活标签使用主题灰色。
- 影响文件：`src/app/core/constants.rs`，`src/app/views.rs`，`src/app/views/tab_bar.rs`，`src/app/dialogs/settings/workspace.rs`，`src/config/store.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：`rustfmt --edition 2024`、2 项配置定向测试、`cargo check`、`cargo test --quiet`（78 项）和 `git diff --check` 均通过；保留既有 `block v0.1.6` future-incompat warning。
- 对 plan 的更新：运行环境和依赖策略保持不变；GUI 仍需验证长标题省略、亮/暗主题状态色和设置切换。

## 2026-07-10 复核标签改动的 AGENTS 约束

- 时间：2026-07-10 19:34 +0800
- 触发原因：用户要求按仓库 `AGENTS.md` 约束本轮代码。
- 执行内容：确认项目仍为 Rust 2024 / GPUI、使用 `cargo`、没有嵌套 `AGENTS.md`、未新增依赖或旧式 `mod.rs`；配置逻辑保留在 `src/config/store.rs`。补齐环境当前态中遗漏的 `src/app/views.rs` 范围记录。
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：源码合规，无需改动；完整规定命令将在记录修正后重跑。
- 对 plan 的更新：运行环境、依赖和模块边界不变；保留 GUI 手工验证要求。

## 2026-07-10 完成 AGENTS 规定环境复验

- 时间：2026-07-10 19:36 +0800
- 触发原因：完成仓库指令合规复核后，需要确认规定 Rust 命令在当前工具链下仍全部通过。
- 执行内容：重跑变更文件格式化、标签配置定向测试、`cargo check` 和完整测试；未改变依赖、manifest、模块树或源码行为。
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：格式化和 2 项定向测试通过；`cargo check` 通过；`cargo test --quiet` 78 项通过；保留既有 `block v0.1.6` future-incompat warning。
- 对 plan 的更新：环境和依赖策略无变化；最终空白检查、tracking docs validator 和 GUI 手工检查边界保持不变。

## 2026-07-10 完成 AGENTS 最终环境校验

- 时间：2026-07-10 19:36 +0800
- 触发原因：Rust 规定命令通过后，需要确认最终差异和跟踪文档符合仓库约束。
- 执行内容：运行 `git diff --check` 和 tracking docs validator，并同步环境当前态的完成状态。
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：空白检查和 tracking docs validator 通过；未发现新增环境、依赖或模块布局风险。
- 对 plan 的更新：自动化环境复核完成，仅保留桌面 GUI 视觉检查。

## 2026-07-10 刷新环境记录到 terminal/SFTP 模块拆分

- 时间：2026-07-10 19:44 +0800
- 触发原因：用户确认拆分 `src/terminal.rs` 和 `src/sftp.rs`，需要切换环境当前态和验证边界。
- 执行内容：确认 Rust 2024、Cargo、GPUI、alacritty terminal、Tokio 和 russh/russh-sftp 环境未变；本轮只新增现代具名子模块并保持 root re-export，不新增依赖或修改 manifest/lock。
- 影响文件：`src/terminal.rs`，`src/terminal/`，`src/sftp.rs`，`src/sftp/`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：本机工具链和现有 78 项测试基线可用；拆分后按 `AGENTS.md` 执行格式化、定向测试、`cargo check`、完整测试、空白检查和 tracking validator。
- 对 plan 的更新：允许开工；重点保护 SFTP 单 worker 所有权和既有 `crate::terminal::*` / `crate::sftp::*` 路径。

## 2026-07-10 完成 terminal/SFTP 模块拆分环境验证

- 时间：2026-07-10 20:21 +0800
- 触发原因：terminal/SFTP 具名子模块迁移完成，需要记录最终工具链和测试结果。
- 执行内容：确认 root entry 收敛、兼容 re-export、最小 `pub(super)` 和单 SFTP runtime 所有权；未新增依赖、`mod.rs` 或 manifest/lock 修改。
- 影响文件：`src/terminal.rs`，`src/terminal/`，`src/sftp.rs`，`src/sftp/`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 计划状态变更：无
- 验证结果：terminal 38 项与 SFTP 14 项定向测试通过；`cargo check` 和 `cargo test --quiet`（78 项）通过；`git diff --check` 和 tracking docs validator 通过；保留既有 `block v0.1.6` future-incompat warning。
- 对 plan 的更新：运行环境、依赖和业务行为不变；允许按功能、重构、文档三个 review unit 提交。

## 2026-07-11 刷新环境记录到 README 与用户文档维护

- 时间：2026-07-11 06:50 +0800
- 触发原因：用户要求维护项目 README，在 `docs/` 增加 README 导航，并按功能拆分文档以便后续添加图片；随后指定默认 `README.md` 使用英文、中文使用 `README.zh.md`。
- 执行内容：复核 README maintenance skill、根 README、双语用户/开发文档、项目地图、manifest 和 release workflow；确定根入口、docs 导航、功能页和截图目录四层文档结构。
- 影响文件：`README.md`，`README.en.md`，`README.zh.md`，`docs/` 用户文档，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 计划状态变更：无
- 验证结果：开工前工作树干净；本轮无需联网、多 agent、Rust/依赖改动或 GUI 验证。
- 对 plan 的更新：允许实施 Markdown-only 文档重构；最终检查相对链接、双语互链、空白和 tracking contract。

## 2026-07-11 完成 README 与用户文档结构验证

- 时间：2026-07-11 07:03 +0800
- 触发原因：双语 README、docs 导航和功能页拆分完成，需要确认活动文档路径和最终跟踪契约。
- 执行内容：检查用户可见 Markdown 相对链接、功能页中英文配对、旧入口残留、标题锚点、空白和 tracking docs；更新环境当前态和项目地图。
- 影响文件：根 README、`docs/` 用户文档、`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 计划状态变更：无
- 验证结果：31 个用户可见 Markdown 文件相对链接目标全部存在；8 组功能页语言配对完整；活动文档无旧入口引用；`git diff --check` 和 tracking docs validator 通过。
- 对 plan 的更新：Markdown-only 交付完成；实际截图留待后续按截图规范添加。

## 2026-07-11 刷新环境记录到全应用日志覆盖

- 时间：2026-07-11 07:25 +0800
- 触发原因：用户要求根据日志覆盖调查，按相关 skills 逐步实现全软件诊断补强。
- 执行内容：复核 Rust/Cargo/CI 环境、日志初始化与轮转、事件总线、SFTP、同步、本地 PTY、SSH、监控、配置保存和敏感字段；确定六阶段实施与验证边界。
- 影响文件：`src/main.rs`，`src/diagnostics.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/config_sync.rs`，`src/backend/`，`src/config/store.rs`，`src/sftp/`，相关 app 保存调用点，跟踪文档
- 计划状态变更：无
- 验证结果：本机 `rustc 1.96.1` / `cargo 1.96.1` 可用；基线 `cargo check` 和 `git diff --check` 通过；保留既有 `block v0.1.6` future-incompat warning。
- 对 plan 的更新：允许在不新增依赖、不改 schema/manifest 的前提下实施日志可靠性、覆盖和脱敏。

## 2026-07-11 验证日志 writer 与脱敏基础

- 时间：2026-07-11 07:42 +0800
- 触发原因：日志基础可靠性和脱敏 helper 实现完成，需要记录阶段环境验证。
- 执行内容：确认仍复用现有 tracing 依赖；改用无损 non-blocking buffer、小时轮转和 7 天文件保留，新增 diagnostics 模块，不修改 manifest/lock。
- 影响文件：`src/main.rs`，`src/diagnostics.rs`，`src/app/lifecycle/startup.rs`，`src/app/actions/`，跟踪文档
- 计划状态变更：无
- 验证结果：4 项定向测试和 `cargo check` 通过；运行环境和依赖策略不变。
- 对 plan 的更新：允许继续核心模块日志覆盖，最终统一重跑完整测试。

## 2026-07-11 验证核心错误路径日志覆盖

- 时间：2026-07-11 07:55 +0800
- 触发原因：SFTP、sync、local backend 和 monitoring 日志补强完成，需要确认现有运行环境与行为测试保持稳定。
- 执行内容：对核心日志变更执行格式化、编译和模块定向测试；确认未新增依赖或配置字段。
- 影响文件：`src/diagnostics.rs`，`src/sftp/`，`src/app/config_sync.rs`，`src/app/lifecycle/event_loop.rs`，`src/backend/local.rs`，`src/app/actions/session.rs`，跟踪文档
- 计划状态变更：无
- 验证结果：阶段 `cargo check`、14 项 SFTP、1 项 local backend 和 5 项 sync 测试通过；运行环境和依赖策略不变。
- 对 plan 的更新：允许继续替换配置保存静默错误，最终运行完整回归。

## 2026-07-11 验证配置保存与远端日志脱敏环境

- 日期：2026-07-11 08:33 +0800
- 变化摘要：配置保存错误统一进入结构化日志；SSH/SFTP/X11 日志和错误链增加敏感值清洗；运行时、依赖、配置 schema、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/diagnostics.rs`，`src/config/store.rs`，相关 app 保存调用点，`src/backend/ssh.rs`，`src/backend/ssh/`，`src/sftp/`，跟踪文档
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo 和现有 tracing 依赖；无需联网、多 agent 或外部服务即可完成静态与单元测试。
- 验证结果：全部变更 Rust 文件格式化通过；阶段 `cargo check` 和 `git diff --check` 通过；未修改 `Cargo.toml` / `Cargo.lock`。

## 2026-07-11 刷新环境记录到快捷键设置完整化

- 日期：2026-07-11 10:07 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮将 Settings 未覆盖但实际生效的应用级快捷键接入现有 keybinding 配置和录制路径。
- 受影响文件：`src/app/input/keybinding_recorder.rs`，`src/app/actions/terminal.rs`，`src/app/views/layout.rs`，`src/main.rs`，`src/app/dialogs/settings/keybindings.rs`，`locales/`，`docs/features/workspace*.md`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；验证命令为相关 `rustfmt --edition 2024`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：本机工具链满足仓库约束；现有 GPUI action/keybinding、settings 录制和 config `key_bindings` 足以完成实施，无需新增依赖、联网或多 agent。工作树中既有 Windows Ctrl 点击改动将原样保留。

## 2026-07-11 完成快捷键设置完整化环境验证

- 日期：2026-07-11 10:22 +0800
- 变化摘要：实际生效的 workspace 与 terminal-focus 应用级快捷键已接入 Settings `Key Bindings`；运行时、依赖、配置 schema、manifest/lock 和 CI 配置保持不变。
- 受影响文件：`src/app/input/keybinding_recorder.rs`，`src/app/actions/terminal.rs`，`src/main.rs`，`locales/`，`docs/features/workspace*.md`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；`cargo check` 通过；`cargo test --quiet` 94 项全部通过；`git diff --check` 和 tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning，真实 GUI 快捷键录制与触发仍需手工确认。

## 2026-07-11 刷新环境记录到主题 JSON 导入

- 日期：2026-07-11 16:20 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮在主题 profile 与自定义保存位置基础上补齐 JSON 导入能力。
- 受影响文件：`src/config/store.rs`，`src/app/theme.rs`，`src/app/dialogs/settings/custom.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；计划运行相关 `rustfmt`、主题/profile 定向测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：确认可复用现有 `rfd`、`serde_json`、GPUI component `ThemeSet` 与用户 themes 目录机制；不新增依赖、不联网、不使用多 agent。

## 2026-07-11 完成主题 JSON 导入环境验证

- 日期：2026-07-11 16:57 +0800
- 变化摘要：主题 JSON 导入、用户 themes 目录持久化、导入 profile 激活和 Custom 设置页导入入口已完成；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/config/store.rs`，`src/app/theme.rs`，`src/app/dialogs/settings/custom.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；theme/profile 定向测试 7 项通过；`cargo check` 通过；`cargo test --quiet` 105 项全部通过；`git diff --check` 和 tracking docs validator 通过。GUI 文件选择和真实主题渲染仍需手工确认。

## 2026-07-11 完成全应用日志覆盖环境验证

- 日期：2026-07-11 08:40 +0800
- 变化摘要：日志覆盖实现与自动化回归完成；运行时、依赖、配置 schema、manifest/lock 和 CI 配置保持不变。
- 受影响文件：本轮日志基础设施、diagnostics、app/backend/SFTP/sync/config 调用点和跟踪文档
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；最终验证为相关 `rustfmt`、定向测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：92 项完整测试全部通过；空白检查和 tracking validator 通过；自动化环境验证完成，真实外部服务、GUI 与日志目录故障注入保留手工验证。

## 2026-07-10 刷新环境记录到源码模块边界治理

- 日期：2026-07-10 20:40 +0800
- 变化摘要：运行时、依赖和工具链未变化；当前实施范围从 terminal/SFTP 大文件拆分切换为全源码模块边界治理，计划逐项迁移全应用事件、SFTP 状态、session/config 类型、proxy/X server 运行时职责和 app 模块入口。
- 受影响文件：`src/`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 更新后的命令或环境：继续使用 Rust 2024、Cargo 和现有锁定依赖；每阶段运行 `rustfmt --edition 2024`、定向测试与 `cargo check`，最终运行 `cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：本机 `rustc 1.96.1`、`cargo 1.96.1` 可用；基线 `cargo check` 与 `git diff --check` 通过；无需联网、不使用多 agent、不修改 `Cargo.toml` / `Cargo.lock`。
## 2026-07-10 完成源码模块边界治理环境验证

- 日期：2026-07-10 21:26 +0800
- 变化摘要：新增全应用事件、proxy transport 和 platform X Server 模块；session/config、terminal/SFTP 和 app 模块所有权完成迁移；`src/system.rs` 更名为 `src/monitoring.rs`；运行时、依赖、manifest/lock 和配置 schema 不变。
- 受影响文件：`src/`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/project-map.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；源码验证为 `rustfmt --edition 2024`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：`cargo check` 无源码告警；`cargo test --quiet` 78 项全部通过；`git diff --check` 通过。Windows 专用 X Server 单元测试在当前 macOS 主机不执行，真实 proxy/X11 联机未验证；保留既有 `block v0.1.6` future-incompat warning。

## 2026-07-10 刷新环境记录到本地 PTY 能力与终端一致性测试

- 日期：2026-07-10 21:53 +0800
- 变化摘要：运行时、依赖和工具链未变化；本轮固定本地 PTY 的 `TERM=xterm-256color`，并新增 CSI `K/J`、DECSTBM 滚动区域和 `?1049` alternate screen 一致性测试。
- 受影响文件：`src/backend/local.rs`，`src/terminal/tab.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 更新后的命令或环境：继续使用 Rust 2024、Cargo 和现有锁定依赖；验证命令为相关 `rustfmt`、backend/terminal 定向测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：本机 `rustc 1.96.1`、`cargo 1.96.1` 可用；工作树基线干净；`portable-pty` 与 `alacritty_terminal` 现有 API 足以覆盖本轮测试，无需新增依赖或联网。

## 2026-07-10 完成本地 PTY 能力与终端一致性测试环境验证

- 日期：2026-07-10 22:08 +0800
- 变化摘要：本地 PTY 新建 shell 统一声明 `TERM=xterm-256color`；新增 1 项命令环境测试和 4 项终端 parser/grid 一致性测试；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/backend/local.rs`，`src/terminal/tab.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；本地新建 PTY 使用固定 `xterm-256color` 能力基线，`COLORTERM=truecolor` 与其他现有环境保持不变。
- 验证结果：相关 `rustfmt` 通过；5 项新增定向测试通过；`cargo check` 通过；`cargo test --quiet` 83 项全部通过；保留既有 `block v0.1.6` future-incompat warning。真实 Codex/类似 TUI 色块复现需在重启并新建本地终端后手工确认。

## 2026-07-10 完成终端兼容性修复最终文档校验

- 日期：2026-07-10 22:10 +0800
- 变化摘要：本轮代码、测试和环境记录均已收口；运行环境和依赖事实没有进一步变化。
- 受影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`git diff --check` 和 tracking docs validator 通过；自动化环境验证完整，剩余仅为新建本地 PTY 后的真实 TUI 手工复现。

## 2026-07-10 刷新环境记录到 Settings 快捷键 toggle

- 日期：2026-07-10 22:17 +0800
- 变化摘要：运行时、依赖、工具链和测试入口不变；本轮只调整 Settings 页面内的快捷键事件路由，使当前设置快捷键可再次关闭页面。
- 受影响文件：`src/app/dialogs/settings/shell.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；验证命令为相关 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：本机 `rustc 1.96.1`、`cargo 1.96.1` 可用；现有 Settings shell 和 `event_matches_action` 足以完成修改，无需新增依赖、联网或多 agent。工作树中已有本地 PTY/终端测试改动将原样保留。

## 2026-07-10 完成 Settings 快捷键 toggle 环境验证

- 日期：2026-07-10 22:22 +0800
- 变化摘要：Settings 页面在非快捷键录制状态下可直接匹配当前配置的 `OpenSettings` 并关闭；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/dialogs/settings/shell.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；`cargo check` 通过；`cargo test --quiet` 83 项全部通过；`git diff --check` 和 tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning，真实 GUI toggle 仍需手工确认。

## 2026-07-10 刷新环境记录到 Settings 关闭确认

- 日期：2026-07-10 22:27 +0800
- 变化摘要：运行时、依赖和工具链不变；本轮新增向后兼容的 Settings 关闭确认偏好、确认 dialog 和记住选择恢复入口。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs.rs`，`src/app/dialogs/settings_close_confirm.rs`，`src/app/dialogs/settings/shell.rs`，`src/app/dialogs/settings/workspace.rs`，`src/app/workspace.rs`，`src/app/views/tab_bar.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；验证命令为相关 `rustfmt`、配置定向测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：本机工具链满足仓库约束；现有 config serde/default、`DialogKind`、`WindowExt::open_dialog` 和 SFTP remember 模式足以完成实施，无需新增依赖、联网或多 agent。

## 2026-07-10 完成 Settings 二次快捷键确认环境验证

- 日期：2026-07-10 22:47 +0800
- 变化摘要：Settings 关闭确认每次都会出现；dialog 打开后第二次当前 Settings 快捷键执行记住的关闭/保持打开动作；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs.rs`，`src/app/dialogs/settings_close_confirm.rs`，`src/app/dialogs/settings/shell.rs`，`src/app/dialogs/settings/workspace.rs`，`src/app/workspace.rs`，`src/app/views/tab_bar.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；配置定向测试 2 项通过；`cargo check` 通过；`cargo test --quiet` 85 项全部通过；`git diff --check` 和 tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning。

## 2026-07-11 刷新环境记录到 SFTP / 菜单 hover 响应修正

- 日期：2026-07-11 20:08 +0800
- 变化摘要：运行时、依赖和工具链不变；本轮修正 SFTP 文件列表和自绘右键菜单 hover 反馈，避免快速移动鼠标时背景色显得滞后。
- 受影响文件：`src/app/views/sftp_panel.rs`，`src/app/views/layout.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；计划验证为相关 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：本机 `rustc 1.96.1`、`cargo 1.96.1` 可用；gpui-component popup menu 只读复核，不修改外部依赖源码；无需新增依赖、联网或多 agent。

## 2026-07-11 刷新环境记录到状态驱动 SFTP hover

- 日期：2026-07-11 20:22 +0800
- 变化摘要：运行时、依赖和工具链不变；SFTP 文件行 hover 从 GPUI 行级 `.hover()` 切换为 `hovered_path` 状态驱动，避免虚拟列表行 hover 与新状态互相覆盖。
- 受影响文件：`src/app/sftp.rs`，`src/app/actions/sftp.rs`，`src/app/actions/session.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/views/sftp_panel.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；已执行相关 `rustfmt` 和 `cargo check`，计划继续执行完整测试、空白检查和 tracking docs validator。
- 验证结果：`cargo check` 通过并仅保留既有 `block v0.1.6` future-incompat warning；无需新增依赖、联网或多 agent。

## 2026-07-11 刷新环境记录到全局 hover 审计与 helper 抽取

- 日期：2026-07-11 20:28 +0800
- 变化摘要：运行时、依赖和工具链不变；按用户要求完成联网检索和全局 `.hover()` 审计，新增 `FastHoverState` 作为虚拟列表快速 hover helper。
- 受影响文件：`src/app.rs`，`src/app/hover.rs`，`src/app/sftp.rs`，`src/app/actions/sftp.rs`，`src/app/actions/session.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/views/sftp_panel.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增运行或测试依赖；计划继续执行完整测试、空白检查和 tracking docs validator。
- 验证结果：`rg` 全局扫描已完成；联网来源记录已写入 `docs/project-implementation-tracker/research.md`；`cargo check` 待 helper 抽取后最终复跑。

## 2026-07-11 完成 SFTP / 菜单 hover 响应修正环境验证

- 日期：2026-07-11 20:30 +0800
- 变化摘要：SFTP 文件行快速 hover helper、状态驱动 hover、SFTP 自绘菜单 hover 统一和全局 hover 审计均已完成；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app.rs`，`src/app/hover.rs`，`src/app/sftp.rs`，`src/app/actions/sftp.rs`，`src/app/actions/session.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/views/sftp_panel.rs`，`src/app/views/layout.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；`cargo check` 通过；`cargo test --quiet` 112 项全部通过；`git diff --check` 和 tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；真实 GUI hover 手感仍需手工确认。

## 2026-07-11 完成 Settings 交互与主题实时预览环境验证

- 日期：2026-07-11 21:34 +0800
- 变化摘要：Settings 下拉菜单、Theme/Appearance 页字体候选构建、Settings 重开状态、SSH/会话入口焦点和 custom theme 实时预览均已完成；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app.rs`，`src/app/actions/session.rs`，`src/app/actions/saved_sessions.rs`，`src/app/dialogs/selector.rs`，`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/`，`src/app/lifecycle/event_loop.rs`，`src/app/lifecycle/init.rs`，`src/app/theme.rs`，`src/app/views/sidebar.rs`，`src/app/views/helpers.rs`，`src/app/workspace.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；`cargo check` 通过；`cargo test --quiet` 110 项全部通过；`git diff --check` 和 tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；真实 GUI Settings hover 手感、SSH 焦点和 custom theme 预览仍需手工确认。

## 2026-07-11 刷新环境记录到 Settings 快速菜单文本回归

- 日期：2026-07-11 21:48 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修复 Settings 快速下拉菜单标签因 flex 内容测量而统一显示为 `...` 的回归。
- 受影响文件：`src/app/dialogs/settings/fast_menu.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；验证命令为相关 `rustfmt`、`cargo check`、`git diff --check` 和 tracking docs validator。
- 验证结果：现有 GPUI `Popover` 和本地 fast menu helper 足以完成修复；不新增依赖、不联网、不使用多 agent。

## 2026-07-11 验证 Settings 快速菜单文本布局实现

- 日期：2026-07-11 21:51 +0800
- 变化摘要：菜单容器和菜单行获得稳定的 `min_width` 实际宽度，普通标签不再在布局测量时收缩为 `...`；超长标签仍由既有 ellipsis 截断。
- 受影响文件：`src/app/dialogs/settings/fast_menu.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；已执行相关 `rustfmt`、`cargo check` 和 `cargo test --quiet`，待执行 `git diff --check` 和 tracking docs validator。
- 验证结果：`cargo check` 通过并仅保留既有 `block v0.1.6` future-incompat warning；完整单测 110 项通过；不新增依赖、联网或多 agent。

## 2026-07-11 完成 Settings 快速菜单文本回归环境验证

- 日期：2026-07-11 21:53 +0800
- 变化摘要：Settings 快速菜单文本布局回归已自动化收口；运行时、依赖、配置 schema、manifest/lock 和 CI 配置不变。
- 受影响文件：`src/app/dialogs/settings/fast_menu.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；无新增运行或测试依赖。
- 验证结果：相关 `rustfmt`、`cargo check`、`cargo test --quiet`（110 项）、`git diff --check` 和 tracking docs validator 全部通过；真实 GUI 文本显示仍需手工确认。

## 2026-07-11 刷新环境记录到 Custom Theme 连续实时预览

- 日期：2026-07-11 23:01 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修复 Custom Theme 连续修改时，外部 ThemeRegistry 因同名主题不覆盖而导致预览停留在首次结果的问题。
- 受影响文件：`src/app/theme.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；计划执行相关 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：现有 Theme 和 ThemeConfig API 足以直接应用每次生成的配置对；不新增依赖、不联网、不使用多 agent。

## 2026-07-11 验证 Custom Theme 连续实时预览实现

- 日期：2026-07-11 23:05 +0800
- 变化摘要：Custom Theme 实时预览改为直接安装每次新构造的 light/dark config，并复用普通主题的 Theme 应用逻辑；不再依赖同名 registry 条目的覆盖。
- 受影响文件：`src/app/theme.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；已执行相关 `rustfmt`、`cargo check`、`cargo test --quiet`，待执行 `git diff --check` 和 tracking docs validator。
- 验证结果：完整单测 110 项通过；`cargo check` 仅保留既有 `block v0.1.6` future-incompat warning；不新增依赖、联网或多 agent。

## 2026-07-11 完成 Custom Theme 连续实时预览环境验证

- 日期：2026-07-11 23:07 +0800
- 变化摘要：Custom Theme 连续实时预览已自动化收口；运行时、依赖、配置 schema、manifest/lock 和 CI 配置不变。
- 受影响文件：`src/app/theme.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；无新增运行或测试依赖。
- 验证结果：相关 `rustfmt`、`cargo check`、`cargo test --quiet`（110 项）、`git diff --check` 和 tracking docs validator 全部通过；真实 GUI 连续编辑预览仍需手工确认。

## 2026-07-11 刷新环境记录到普通 Theme profile 直接应用

- 日期：2026-07-11 23:12 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修复 Appearance 页普通 Theme profile 已写入配置但窗口主题未可靠应用的问题。
- 受影响文件：`src/app/theme.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；计划执行相关 `rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator。
- 验证结果：脱敏配置字段确认 Matrix profile 已持久化；内置 profile JSON 有实际不同色调；不新增依赖、不联网、不使用多 agent。

## 2026-07-11 验证普通 Theme profile 直接应用实现

- 日期：2026-07-11 23:16 +0800
- 变化摘要：普通 profile 直接从 registry 解析并安装明确的 light/dark 配置对，Custom profile 保留动态构造；已生成新 debug 二进制用于关闭旧进程后的 GUI 验证。
- 受影响文件：`src/app/theme.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；已执行相关 `rustfmt`、`cargo check`、`cargo test --quiet` 和 `cargo build`，待执行 `git diff --check` 和 tracking docs validator。
- 验证结果：完整单测 110 项通过；`cargo check` / `cargo build` 仅保留既有 `block v0.1.6` future-incompat warning；不新增依赖、联网或多 agent。

## 2026-07-11 完成普通 Theme profile 直接应用环境验证

- 日期：2026-07-11 23:18 +0800
- 变化摘要：普通 Theme profile 直接应用已自动化收口；运行时、依赖、配置 schema、manifest/lock 和 CI 配置不变。
- 受影响文件：`src/app/theme.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；无新增运行或测试依赖。
- 验证结果：相关 `rustfmt`、`cargo check`、`cargo test --quiet`（110 项）、`cargo build`、`git diff --check` 和 tracking docs validator 全部通过；真实 GUI 连续 profile 切换仍需手工确认。
## 2026-07-12 初始化全局字体亮度拆分环境记录

- 日期：2026-07-12 07:33 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮把 Custom 主题字体亮度迁移为 Theme 设置中的全局 UI / Terminal 亮度，Custom Theme Editor 不再负责亮度。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app/state/appearance.rs`，`src/app/lifecycle/init.rs`，`src/app/theme.rs`，`src/app/actions/session.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/font_page.rs`，`src/app/dialogs/settings/custom.rs`，`src/terminal/element.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：确认本轮验证命令收敛为相关 `rustfmt --edition 2024`、配置/theme 聚焦测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；GUI 视觉亮度仍需手工确认。

## 2026-07-12 完成全局字体亮度拆分环境验证

- 日期：2026-07-12 07:58 +0800
- 变化摘要：全局 `ui_font_brightness` / `terminal_font_brightness` 配置、旧 custom 亮度迁移、Settings 控件、UI 前景色亮度后处理和 terminal 全局亮度读取已完成；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/app/state/appearance.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/theme.rs`，`src/app/actions/session.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/custom.rs`，`src/terminal/element.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：相关 `rustfmt` 通过；theme/profile 聚焦测试 12 项通过；font brightness 设置测试 2 项通过；import theme 测试 5 项通过；`cargo check` 通过；`cargo test --quiet` 117 项全部通过；`git diff --check` 和 tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；GUI 亮度视觉效果仍需手工确认。
## 2026-07-12 新增内置字体资源预检

- 日期：2026-07-12 10:07 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮从用户提供的三个字体压缩包筛选必要字体，新增编译期字体资源、授权和 Settings 内置字体标识。
- 受影响文件：`assets/fonts/`，`src/app/theme.rs`，`src/app/dialogs/settings/font_page.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`；使用本机 `unzip`、`otfinfo` / `fc-scan` 核对字体资源。
- 验证结果：计划执行字体元数据检查、相关 `rustfmt`、字体聚焦测试、`cargo check`、完整 `cargo test --quiet`、`git diff --check` 和 tracking docs validator。

## 2026-07-12 完成新增内置字体资源环境验证

- 日期：2026-07-12 10:25 +0800
- 变化摘要：新增 Iosevka Term、JetBrains Mono 和 Monaspace Neon Var 内置字体资源、授权和 Settings 优先排序；运行时架构、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`assets/fonts/`，`src/app/theme.rs`，`src/app/dialogs/settings/font_page.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖；字体通过编译期 `include_bytes!` 和 GPUI `add_fonts()` 注册。
- 验证结果：字体 metadata、CoreText family 和 variable axes 核对通过；`rustfmt`、3 项字体聚焦测试、`cargo check`、`cargo build`、120 项完整测试、fast hover 审计、`git diff --check` 和 tracking validator 均通过；真实 GUI 字体视觉仍需手工确认。

## 2026-07-12 监控 Bottom 位置修复预检

- 日期：2026-07-12 10:34 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修正 `Monitoring Position = Bottom` 时监控面板实际显示在主内容上方的问题。
- 受影响文件：`src/app/views/layout.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。
- 验证结果：计划执行 `rustfmt --edition 2024 src/app/views/layout.rs`、`cargo check`、`git diff --check` 和 tracking docs validator；真实 GUI 位置仍需手工确认。

## 2026-07-12 完成监控 Bottom 位置修复环境验证

- 日期：2026-07-12 10:40 +0800
- 变化摘要：`Monitoring Position = Bottom` 时监控面板改为在主内容之后渲染；运行时架构、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/views/layout.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`rustfmt`、`cargo check`、完整 `cargo test --quiet`（120 项）、fast hover 路径审计、`git diff --check` 和 tracking validator 均通过；真实 GUI 位置仍需手工确认。

## 2026-07-12 Integrated 标题栏横线修复预检

- 日期：2026-07-12 10:45 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修正 Integrated 标题栏中 `TabBar` 底部分隔线只覆盖中间区域的问题。
- 受影响文件：`src/app/views/layout.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：计划执行 `rustfmt --edition 2024 src/app/views/layout.rs`、`cargo check`、`git diff --check` 和 tracking docs validator；真实 GUI 横线贯穿效果仍需手工确认。

## 2026-07-12 完成 Integrated 标题栏横线修复环境验证

- 日期：2026-07-12 10:52 +0800
- 变化摘要：Integrated 标题栏改为覆盖 `TabBar` 内部局部分隔线，并单独绘制一条全宽 1px 底部分隔线；运行时架构、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/views/layout.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`rustfmt`、`cargo check`、完整 `cargo test --quiet`（120 项）、`git diff --check` 和 tracking validator 均通过；真实 GUI 横线贯穿和无叠线效果仍需手工确认。

## 2026-07-12 saved session 右键菜单检查预检

- 日期：2026-07-12 11:02 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮检查 saved session 自绘右键菜单接线，并修正菜单缺少窗口边界夹取的问题。
- 受影响文件：`src/app/views/layout.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。
- 验证结果：计划执行 `rustfmt --edition 2024 src/app/views/layout.rs`、`cargo check`、完整 `cargo test --quiet`、fast hover/context 审计、`git diff --check` 和 tracking docs validator；真实 GUI 右键菜单仍需手工确认。

## 2026-07-12 完成 saved session 右键菜单环境验证

- 日期：2026-07-12 11:07 +0800
- 变化摘要：saved session 右键菜单 Copy Info / Clone / Edit / Delete 接线复核完成，并新增窗口边界夹取，避免菜单靠右或靠下打开时被裁掉；运行时架构、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/views/layout.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`rustfmt`、`cargo check`、完整 `cargo test --quiet`（120 项）、fast hover/context 审计、`git diff --check` 和 tracking validator 均通过；真实 GUI 右键菜单仍需手工确认。

## 2026-07-12 无密码无私钥 SSH 终端密码输入预检

- 日期：2026-07-12 11:15 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮在 SSH 会话既无密码也无私钥时直接创建未连接 tab 并进入终端内本地密码提示，回车后再用临时密码发起连接。
- 受影响文件：`src/app.rs`，`src/app/session_ui.rs`，`src/app/actions/session.rs`，`src/app/actions/terminal.rs`，`src/app/workspace.rs`，`src/app/lifecycle/event_loop.rs`，`src/terminal/backend.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：计划执行受影响 Rust 文件 `rustfmt --edition 2024`、`cargo check`、聚焦测试或完整 `cargo test --quiet`、`git diff --check` 和 tracking docs validator；真实 GUI 终端密码输入仍需手工确认。

## 2026-07-12 完成无密码无私钥 SSH 终端密码输入环境验证

- 日期：2026-07-12 11:32 +0800
- 变化摘要：无密码无私钥的密码认证 SSH 会话改为打开 tab 后直接显示终端内 `Password: `，输入密码后再发起连接；运行时架构、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app.rs`，`src/app/session_ui.rs`，`src/app/actions/session.rs`，`src/app/actions/terminal.rs`，`src/app/workspace.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/lifecycle/init.rs`，`src/terminal/backend.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖；终端密码为当前 tab 临时值，不保存到配置。
- 验证结果：`rustfmt`、`cargo check`、新增聚焦测试 4 项、完整 `cargo test --quiet`（124 项）、`git diff --check` 和 tracking validator 均通过；真实 GUI 终端密码输入仍需手工确认。

## 2026-07-12 Rocky/Linux 主窗口拖动修复预检

- 日期：2026-07-12 14:43 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修复 Rocky/Linux 编译产物主窗口顶部区域无法拖动的问题。
- 受影响文件：`src/app/lifecycle/startup.rs`，`src/app/views/layout.rs`，`src/app/views/tab_bar.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改 SSH/X11 relay 或本地 X server 配置。
- 验证结果：计划执行 `rustfmt --edition 2024 src/app/lifecycle/startup.rs src/app/views/layout.rs src/app/views/tab_bar.rs`、`cargo check`、完整 `cargo test --quiet`、`git diff --check` 和 tracking docs validator；真实 Rocky/Linux GUI 拖动仍需手工确认。

## 2026-07-12 完成 Rocky/Linux 主窗口拖动环境验证

- 日期：2026-07-12 14:50 +0800
- 变化摘要：Linux 集成标题栏路径不再关闭 `is_movable`；Linux Native 顶部 tab bar 容器和右侧 spacer 复用 `bind_titlebar_drag()` 提供应用内窗口拖动区域；运行时架构、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/lifecycle/startup.rs`，`src/app/views/layout.rs`，`src/app/views/tab_bar.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`rustfmt`、`cargo check`、完整 `cargo test --quiet`（130 项）、`git diff --check` 和 tracking validator 均通过；真实 Rocky/Linux GUI 拖动仍需手工确认。

## 2026-07-12 SSH 弹窗 Enter 提交行为修复预检

- 日期：2026-07-12 15:58 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修复新建/编辑 SSH 连接弹窗中 Enter 默认关闭弹窗并丢失表单内容的问题。
- 受影响文件：`src/app/dialogs/ssh.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改 SSH 协议、认证或配置 schema。
- 验证结果：计划执行 `rustfmt --edition 2024 src/app/dialogs/ssh.rs`、`cargo check`、完整 `cargo test --quiet`、`git diff --check` 和 tracking docs validator；真实 GUI 中不同输入框焦点下的 Enter 行为仍需手工确认。

## 2026-07-12 完成 SSH 弹窗 Enter 提交行为环境验证

- 日期：2026-07-12 16:02 +0800
- 变化摘要：SSH 新建/编辑弹窗显式覆盖 Dialog `on_ok`，Enter 改为调用 `connect_ssh()`；校验失败不走 Dialog 默认关闭路径，保留表单内容；运行时架构、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/dialogs/ssh.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`rustfmt`、`cargo check`、完整 `cargo test --quiet`（130 项）、`git diff --check` 和 tracking validator 均通过；真实 GUI 中不同输入框焦点下的 Enter 行为仍需手工确认。

## 2026-07-12 Ubuntu 图标 app_id 匹配修复预检

- 日期：2026-07-12 16:17 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修复 Ubuntu 下窗口 / Dock 图标无法匹配应用的问题。
- 受影响文件：`src/app/lifecycle/startup.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改图标资源或 Debian 包资产列表。
- 验证结果：计划执行 `rustfmt --edition 2024 src/app/lifecycle/startup.rs`、`cargo check`、完整 `cargo test --quiet`、`git diff --check` 和 tracking docs validator；Ubuntu Dock / App Grid 真实图标显示仍需手工确认。

## 2026-07-12 完成 Ubuntu 图标 app_id 匹配环境验证

- 日期：2026-07-12 16:23 +0800
- 变化摘要：Linux/FreeBSD 普通启动默认设置 `app_id = ax_shell`，与 `ax_shell.desktop`、`Icon=ax_shell` 和安装到 hicolor 的 `ax_shell.png` 对齐；开发重载仍可通过 `AX_SHELL_APP_ID` 覆盖；运行时架构、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`src/app/lifecycle/startup.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖。
- 验证结果：`rustfmt`、`cargo check`、完整 `cargo test --quiet`（130 项）、`git diff --check` 和 tracking validator 均通过；Ubuntu Dock / App Grid 真实图标显示仍需手工确认。

## 2026-07-12 完成 Ubuntu X11 运行库缺失文档环境验证

- 日期：2026-07-12 16:11 +0800
- 变化摘要：Proxy/X11 用户文档新增 Ubuntu 缺少 `libxkbcommon-x11.so.0` 的排查说明和 `libxkbcommon-x11-0` 安装命令；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`docs/features/proxy-x11.md`，`docs/features/proxy-x11.zh.md`，跟踪文档。
- 更新后的命令或环境：无；文档-only，不新增运行或测试依赖。
- 验证结果：`git diff --check` 通过；tracking docs validator 通过；未运行 Rust 编译测试。

## 2026-07-12 完成 Windows Visual C++ 运行库缺失文档环境验证

- 日期：2026-07-12 16:29 +0800
- 变化摘要：Proxy/X11 用户文档新增 Windows 缺少 Visual C++ runtime DLL 的排查说明，提供 Microsoft VC++ Redistributable 和 `abbodi1406/vcredist` AIO 工具作为解决办法；运行时、依赖、manifest/lock 与 CI 配置不变。
- 受影响文件：`docs/features/proxy-x11.md`，`docs/features/proxy-x11.zh.md`，跟踪文档。
- 更新后的命令或环境：无；文档-only，不新增运行或测试依赖。
- 验证结果：`git diff --check` 通过；tracking docs validator 通过；未运行 Rust 编译测试。

## 2026-07-12 Ubuntu client-side 窗口控制修复预检

- 日期：2026-07-12 16:17 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修复 Ubuntu/GNOME Wayland 下 GPUI 回退到 client-side decoration 时 AxShell 缺少关闭、最小化和拖动区域的问题。
- 受影响文件：`src/app/views.rs`，`src/app/views/layout.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，复用现有 `gpui_component::TitleBar`。
- 验证结果：计划执行受影响 Rust 文件 `rustfmt --edition 2024`、`cargo check`、完整 `cargo test --quiet`、`git diff --check` 和 tracking docs validator；真实 Ubuntu/GNOME Wayland GUI 控件仍需手工确认。

## 2026-07-12 完成 Ubuntu client-side 窗口控制修复环境验证

- 日期：2026-07-12 16:23 +0800
- 变化摘要：Linux Native 标题栏样式在 GPUI 实际回退为 `Decorations::Client` 时渲染 `gpui_component::TitleBar`，补齐最小化、最大化/还原、关闭和拖动区域；支持 server-side decorations 的 Linux 桌面仍保留系统标题栏。
- 受影响文件：`src/app/views.rs`，`src/app/views/layout.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖；未修改 `Cargo.toml` / `Cargo.lock`。
- 验证结果：`rustfmt`、`cargo check`、完整 `cargo test --quiet`（130 项）、`git diff --check` 和 tracking validator 均通过；`cargo check` 保留既有 `block v0.1.6` future-incompat warning；真实 Ubuntu/GNOME Wayland GUI 控件仍需手工确认。
## 2026-07-13 Tokio runtime 按需生命周期预检

- 日期：2026-07-13 08:55 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮将把启动时无条件创建的 Tokio multi-thread runtime 改为首次远程工作时创建、所有根任务和 SFTP handle 释放后销毁。
- 受影响文件：`src/app/state/runtime.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/session.rs`，`src/app/actions/pane.rs`，`src/app/actions/sftp.rs`，`src/app/workspace.rs`，`src/app/config_sync.rs`，`src/app/lifecycle/event_loop.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不修改外部 cargo 缓存源码。
- 验证结果：已完成 PID `51129` 线程采样与源码使用点复查；计划运行受影响 Rust 文件 `rustfmt`、定向 lifecycle 测试、`cargo check`、`cargo test --quiet`、`git diff --check` 和 tracking docs validator；真实 GUI 线程回收仍需手工确认。

## 2026-07-13 完成 Tokio runtime 按需生命周期环境验证

- 日期：2026-07-13 09:09 +0800
- 变化摘要：Tokio runtime 现仅在 SSH、SFTP 或配置同步首次执行时创建，固定为 2 worker / 最多 8 blocking worker；所有 SSH/SFTP 根 worker、shutdown reaper、sync future 和 X11 relay 通过 lease 防止提前销毁，最后一项工作结束后释放 runtime。
- 受影响文件：`src/app.rs`，`src/app/state/runtime.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/session.rs`，`src/app/actions/pane.rs`，`src/app/actions/sftp.rs`，`src/app/workspace.rs`，`src/app/config_sync.rs`，`src/backend/ssh.rs`，`src/backend/ssh/x11.rs`，`src/sftp/worker.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增依赖或 manifest/lock 变更；未使用远程功能的独立 dev 进程采样为 17 条总线程且无 `ax-tokio` worker。
- 验证结果：两项 runtime 生命周期测试和完整 133 项 Rust 测试通过；`cargo check`、`cargo build`、`git diff --check` 和 tracking docs validator 通过。保留既有 `block v0.1.6` future-incompat warning；真实 SSH/SFTP/sync/X11 关闭后的 GUI 回收仍需手工确认。

## 2026-07-13 Rayon worker 配置预检

- 日期：2026-07-13 09:22 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮将在启动最早期通过配置设置 `RAYON_NUM_THREADS`，默认值为 2，并在资源设置页提供 4 的吞吐优先选项。
- 受影响文件：`src/main.rs`，`src/app/lifecycle/startup.rs`，`src/config/model.rs`，`src/config/store.rs`，`src/app/dialogs/settings/monitoring.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`；通过 Rayon 已支持的 `RAYON_NUM_THREADS` 环境变量配置传递依赖中的 global pool。
- 验证结果：计划执行受影响 Rust 文件格式化、Rayon 设置聚焦测试、`cargo check`、`cargo test --quiet`、`cargo build`、独立 debug app 线程采样、`git diff --check` 和 tracking docs validator。

## 2026-07-13 完成 Rayon worker 配置环境验证

- 日期：2026-07-13 09:35 +0800
- 变化摘要：新增 `rayon_threads` 持久化设置，默认 2、仅允许 2 或 4；启动在 macOS launch environment 同步后、GPUI 初始化前设置 `RAYON_NUM_THREADS`；资源设置页提供 2（默认）和 4（更高吞吐）下拉选项，并提示修改后重启生效。
- 受影响文件：`src/main.rs`，`src/app/lifecycle/startup.rs`，`src/config/model.rs`，`src/config/store.rs`，`src/app/dialogs/settings/monitoring.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-implementation-tracker/project-map.md`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖，未修改 `Cargo.toml` / `Cargo.lock`。
- 验证结果：受影响 Rust 文件格式化、Rayon 聚焦测试、`cargo check`、完整 `cargo test --quiet`（135 项）、`cargo build`、fast menu 静态审计、`git diff --check` 和 tracking docs validator 均已通过；临时空配置启动的独立 debug app 采样为 2 条 `rayon_core` worker。保留既有 `block v0.1.6` future-incompat warning；4-worker 选项需用户在真实 Settings 中切换并重启后手工采样确认。

## 2026-07-13 终端输出热点优化预检

- 日期：2026-07-13 09:43 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮优化持续 PTY 输出时主线程对同一 tab 的重复解析和每段输出后的可视网格全屏 SipHash。
- 受影响文件：`src/app/lifecycle/event_loop.rs`，`src/terminal/tab.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，复用 256 条有界 backend event queue 和现有刷新节流。
- 验证结果：计划执行受影响 Rust 文件格式化、event loop/terminal 聚焦测试、`cargo check`、完整 `cargo test --quiet`、`cargo build`、同场景 debug `sample`、`git diff --check` 和 tracking docs validator。

## 2026-07-13 完成终端输出热点优化环境验证

- 时间：2026-07-13 10:00 +0800
- 变化摘要：连续 PTY `Output` 在同一连续事件段按 tab 合并后只 feed 一次；终端变更判断改为 O(1) dirty generation，移除了逐段扫描可视网格的 SipHash；关键字高亮缓存用 generation 失效。
- 受影响文件：`src/app/lifecycle/event_loop.rs`，`src/terminal/tab.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`；刷新节流、PTY/SSH 协议和事件顺序保持既有语义。
- 验证结果：受影响 Rust 文件格式化、聚焦 batch/dirty-generation/ANSI 边界测试、`cargo check`、完整 `cargo test --quiet`（139 项）和 `cargo build` 通过；静态 debug `sample` 未持续产生 PTY 输出，确认旧 hash 热点帧为 0，但不能量化真实持续输出时的 CPU 降幅；保留既有 `block v0.1.6` future-incompat warning。

## 2026-07-13 自定义 Rayon worker 设置预检

- 时间：2026-07-13 10:05 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮会把资源页 Rayon worker 从固定 2/4 选项改为用户输入的 `1–64`，默认保持 2，重启后通过已有 `RAYON_NUM_THREADS` 生效。
- 受影响文件：`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/dialogs/settings/monitoring.rs`，`src/config/model.rs`，`src/config/store.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`。
- 验证结果：确认现有 `InputState` 支持回车/失焦提交；计划运行受影响 Rust 文件格式化、配置聚焦测试、`cargo check`、完整 `cargo test --quiet`、`cargo build`、`git diff --check` 和 tracking docs validator。

## 2026-07-13 完成自定义 Rayon worker 设置环境验证

- 时间：2026-07-13 10:13 +0800
- 变化摘要：Rayon worker 配置从仅允许 2/4 改为 `1–64`；资源页改用数值输入，Enter 或失焦时保存并回显规范化值，非法文本保留当前配置；默认仍为 2，仍在应用重启前设置 `RAYON_NUM_THREADS`。
- 受影响文件：`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/dialogs/settings/monitoring.rs`，`src/config/model.rs`，`src/config/store.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`；Rayon global pool 的 worker 数仍只在下次启动生效。
- 验证结果：受影响 Rust 文件 `rustfmt`、Rayon 配置测试、输入解析测试、`cargo check`、完整 `cargo test --quiet`（140 项）和 `cargo build` 通过；保留既有 `block v0.1.6` future-incompat warning；真实 Settings 页面输入和重启后的 worker 数尚未手工确认。

## 2026-07-13 Terminal snapshot 与关键词高亮优化预检

- 时间：2026-07-13 10:25 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮将用 generation 缓存 terminal snapshot、以行级增量方式更新 keyword highlight，并从内存配置传入开关，移除 `render_snapshot()` 内磁盘加载。
- 受影响文件：`src/terminal/tab.rs`，`src/terminal/highlight.rs`，`src/terminal/element.rs`，`src/app/actions/session.rs`，`src/app/actions/terminal.rs`，`src/app/search.rs`，`src/app/views/layout.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，保留前台 16 ms refresh 节流与既有 PTY 协议。
- 验证结果：sample 表明 AxShell reader thread 与原生 Terminal `tty-io` 同为阻塞等待，CPU 热点在 AxShell GPUI draw、snapshot 和 highlighter；计划运行格式化、聚焦测试、`cargo check`、完整测试、构建、空白检查、tracking validator 和持续输出复采样。

## 2026-07-13 完成 Terminal snapshot 与关键词高亮优化环境验证

- 日期：2026-07-13 10:42 +0800
- 变化摘要：terminal snapshot 以 `dirty_generation` 和内存 keyword 开关缓存为共享 `Rc`；可见 cell 不再在同一 generation 的每次 `Window::draw`/交互查询中重新构建。关键词/IP/port 高亮按可见行字符与列复用，URL 保留逻辑行处理；`render_snapshot()` 已无 `ConfigStore::load()`。
- 受影响文件：`src/terminal/tab.rs`，`src/terminal/highlight.rs`，`src/terminal/element.rs`，`src/app/actions/session.rs`，`src/app/actions/terminal.rs`，`src/app/search.rs`，`src/app/views.rs`，`src/app/views/layout.rs`，`src/app/views/terminal_panel.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；没有新增运行或测试依赖，未修改 `Cargo.toml` / `Cargo.lock`，未联网或使用多 agent。
- 验证结果：受影响 Rust 文件 `rustfmt`、snapshot 8 项与 highlight 14 项聚焦测试、`cargo check`、完整 `cargo test --quiet`（144 项）、`cargo build`、`git diff --check` 和 tracking validator 均通过。保留既有 `block v0.1.6` future-incompat warning；真实持续输出 GUI 的 CPU 新 sample 尚未执行，不能从自动化测试推断具体百分比降幅。

## 2026-07-13 TermDamage 增量快照环境预检

- 日期：2026-07-13 10:56 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；已确认锁定的 `alacritty_terminal 0.26.0` 原生提供逐行逐列 `TermDamage`，本轮将以其替代应用层整行内容比较。
- 受影响文件：`src/terminal/tab.rs`，`src/terminal/highlight.rs`，`src/terminal/element.rs`，`docs/project-implementation-tracker/research.md`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`；已完成外部 API 与同类 terminal 实现检索，未使用多 agent。
- 验证结果：已确认 `Term::damage()` / `reset_damage()` 与 `LineDamageBounds` 在本机 `0.26.0` 源码存在；后续将执行受影响 Rust 文件格式化、damage/snapshot/highlight 聚焦测试、`cargo check`、完整测试、构建、空白检查和 tracking validator。

## 2026-07-13 完成 TermDamage 增量快照环境验证

- 时间：2026-07-13 11:26 +0800
- 变化摘要：`alacritty_terminal 0.26.0` 的 `TermDamage` 现作为可视 snapshot 的唯一行失效来源。受损行重建、未变行复用 `Rc<RenderRow>`；高亮关闭期间的行块变化也会在重新启用时失效 keyword 和 URL 缓存，URL 同时使用前后自动换行边界防止残留颜色。
- 受影响文件：`src/terminal/tab.rs`，`src/terminal/highlight.rs`，`src/terminal/element.rs`，`src/app/actions/session.rs`，`src/app/actions/terminal.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`；`TermDamage` 每次读取后在 mutation 路径立即 `reset_damage()`。
- 验证结果：受影响 Rust 文件 `rustfmt`、snapshot 聚焦测试 9 项、highlight 聚焦测试 18 项、`cargo check`、完整 `cargo test --quiet`（149 项）、`cargo build` 和 `git diff --check` 均通过。仅保留既有 `block v0.1.6` future-incompat warning；真实 GUI 持续输出 CPU 降幅未在本轮量化。

## 2026-07-13 完成持续输出高亮限频环境验证

- 时间：2026-07-13 12:05 +0800
- 变化摘要：连续输出的 keyword/IP/port/URL 颜色从每个 terminal draw 重算改为最多每 `125ms` 一次；文字、ANSI 样式和光标继续使用现有 16ms 终端刷新。延迟窗口仅复用可证明未变或按 scrollback 增量平移的颜色，无法证明对应关系时清空，停止输出后由 event pump 自动补刷。
- 受影响文件：`src/terminal/tab.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/actions/terminal.rs`，`src/app/actions/session.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`；高亮限频为固定 `125ms`，不新增用户配置项。
- 验证结果：受影响 Rust 文件 `rustfmt`、tab 聚焦测试 11 项、highlight 聚焦测试 18 项、`cargo check`、完整 `cargo test --quiet`（151 项）、`cargo build` 和 `git diff --check` 均通过。仅保留既有 `block v0.1.6` future-incompat warning；真实 GUI 持续输出 CPU 降幅未在本轮量化。

## 2026-07-13 完成本地 Shell Profile 环境验证

- 时间：2026-07-13 12:51 +0800
- 变化摘要：本地终端新增持久化 `LocalShellProfile`，配置包含名称、程序和逐行 argv 参数；默认 profile 按平台提供 macOS zsh/bash、Windows PowerShell/cmd/Git Bash/WSL、Linux login shell/bash/sh/zsh/fish 候选。Linux 无 `SHELL` 时回退由 `/bin/zsh` 改为 `/bin/sh`。
- 受影响文件：`src/config/model.rs`，`src/config/store.rs`，`src/backend/local.rs`，`src/terminal/tab.rs`，`src/app/actions/session.rs`，`src/app/actions/pane.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs/settings/terminal.rs`，`locales/`，`docs/features/terminal-ssh*.md`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo 和 `portable-pty 0.9`；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`。PTY 使用 `CommandBuilder::args` 逐项传递参数，禁止命令字符串拼接；Windows 不覆盖 `SHELL`，保证 `wsl.exe` 内部环境不被污染。
- 验证结果：`rustfmt` 通过；profile/PTY 聚焦测试 3 项、terminal tab 聚焦测试 12 项、`cargo check`、完整 `cargo test --quiet`（154 项）、`cargo build`、`git diff --check` 和 tracking validator 均通过。`cargo check` / `cargo build` 仅保留既有 `block v0.1.6` future-incompat warning；真实 GUI 下启动各平台 shell 未执行。

## 2026-07-13 README 与文档双语文件名审查预检

- 时间：2026-07-13 13:54 +0800
- 变化摘要：本轮只维护 README 与 docs 的文件名/内容语言一致性及当前链接；不改 Rust 源码、依赖、CI 或发布配置。
- 受影响文件：`README.md`，`README.zh.md`，`docs/README.md`，`docs/README.zh.md`，`docs/development*.md`，`docs/resource-lifecycle*.md`，相关用户文档和跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；验证限制为 Markdown 内链解析、`git diff --check` 和 tracking docs validator，避免在 dirty worktree 写入构建产物。
- 验证结果：已确认文档树的目标约定是英文 `.md`、中文 `.zh.md`；待执行静态验证。

## 2026-07-13 完成 README 与文档双语文件名环境验证

- 时间：2026-07-13 13:57 +0800
- 变化摘要：开发与资源生命周期文档已统一为英文 `.md`、中文 `.zh.md`；当前 README、文档导航和交叉链接均已同步。
- 受影响文件：README、当前 docs 导航/用户页面、`docs/project-implementation-tracker/` 和本环境记录。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`，未运行会产生构建产物的命令。
- 验证结果：忽略代码块、HTML 注释和行内代码后的 Markdown 活动相对链接解析通过；`git diff --check` 与 tracking docs validator 通过。

## 2026-07-13 完成终端行级布局缓存环境验证

- 时间：2026-07-13 14:04 +0800
- 变化摘要：终端预绘制从每帧遍历完整可视 cell 改为稳定 GPUI element state 中的行级布局缓存；`Rc<RenderRow>`、字体/主题和行高亮不变时复用已准备的背景、text runs 和 block glyph。底部滚屏候选行必须逐 cell 对照当前 Alacritty grid 才可复用，延迟 keyword 高亮同样仅迁移已验证行块。
- 受影响文件：`src/terminal/element.rs`，`src/terminal/tab.rs`，`src/terminal/highlight.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`。验证执行 `rustfmt`、terminal focused tests、`cargo check`、完整 `cargo test --quiet`、`cargo build`、`git diff --check`、tracking docs validator 和 macOS `sample`。
- 验证结果：focused tests 47 项、完整测试 158 项、`cargo check`、`cargo build` 和 `git diff --check` 均通过；当前 debug dev PID 的 10 秒空闲 sample 未出现 terminal layout、shape 或 highlighter 热点，物理内存 93.9 MiB。采样进程不在持续输出场景，未量化对既有高压 sample 的 CPU 降幅；仅保留既有 `block v0.1.6` future-incompat warning。

## 2026-07-13 本地 SFTP 会话目录恢复实施记录

- 时间：2026-07-13 16:32 +0800
- 目的：让本地 SFTP 浏览器为每个已保存 SSH 会话恢复最后一次成功打开的目录，同时避免将机器相关路径同步到 WebDAV 或 S3。
- 改动范围：`src/config/model.rs`，`src/config/store.rs`，`src/app/actions/sftp.rs`，`src/app/workspace.rs`，`docs/features/sftp.md`，`docs/features/sftp.zh.md`，跟踪文档。
- 执行内容：在本机 `ConfigFile` 添加按保存 `Session.id` 存储的目录映射；成功导航才保存，临时连接不保存；加载、删除会话和同步下载的会话替换都会清理无效 mapping。恢复目录不可读时回退用户主目录但不修改记录；同步 payload 保持只含 sessions。
- 验证结果：相关 Rust 文件已格式化，`cargo test --quiet local_sftp` 4 项、`cargo check`、完整 `cargo test --quiet` 162 项、`cargo build`、`git diff --check` 和 tracking validator 通过。`cargo check` / `cargo build` 仅保留既有 `block v0.1.6` future-incompat warning。
- 风险/待办：需在真实 GUI 中验证两个保存会话各自恢复目录、删除目录后的 home 回退，以及未保存连接不写入配置。

## 2026-07-13 终端 ShapedLine 行缓存实施记录

- 时间：2026-07-13 22:36 +0800
- 变化摘要：终端 `RowLayout` 现在在 prepaint / 行布局构建阶段生成并保存 `ShapedLine`，paint 阶段直接绘制缓存 shaped line，不再对每个未变 text run 调用 `shape_line_by_hash`。
- 受影响文件：`src/terminal/element.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`。验证执行 `rustfmt`、聚焦测试、`cargo check`、完整 `cargo test --quiet`、`cargo build`、`git diff --check` 和 tracking validator。
- 验证结果：`rustfmt --edition 2024 src/terminal/element.rs`、`cargo test --quiet grid_layout_key_tracks_state_that_invalidates_shaped_rows`、`cargo check`、完整 `cargo test --quiet` 164 项、`cargo build`、`git diff --check` 和 tracking validator 均通过。`cargo check` / `cargo build` 仅保留既有 `block v0.1.6` future-incompat warning。
- 风险/待办：后续需在真实持续输出负载下复采样，量化 `shape_line_by_hash` 热点是否消失；`ShapedLine::paint` 的 glyph 绘制成本仍会存在。

## 2026-07-13 终端重连状态高亮闪烁预检

- 时间：2026-07-13 22:58 +0800
- 变化摘要：运行时、依赖、工具链和 CI 入口不变；本轮修复外部重连状态持续刷新时，AxShell 终端关键词 / URL 高亮因 full damage 重建未变行而在延迟窗口内闪烁。
- 受影响文件：`src/terminal/tab.rs`，必要时 `src/terminal/highlight.rs` / `src/terminal/element.rs`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`；保留 125ms 高亮限频和前台 16ms 终端刷新语义。
- 验证结果：已确认截图文案来自终端内外部 Codex 请求流，AxShell 的相关问题在 `build_visible_rows` 行复用与延迟高亮映射；计划执行 `rustfmt`、聚焦测试、`cargo check`、`git diff --check` 和 tracking docs validator。

## 2026-07-13 完成终端重连状态高亮闪烁环境验证

- 时间：2026-07-13 23:09 +0800
- 变化摘要：`build_visible_rows` 现在会在 full damage 和 dirty rows 重建前逐 cell 验证旧行是否仍等于当前 terminal grid；未变行继续复用旧 `Rc<RenderRow>`，让 125ms 延迟高亮窗口可以保留未变化的关键词 / URL 颜色。
- 受影响文件：`src/terminal/tab.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`；高亮限频和终端刷新语义保持不变。
- 验证结果：`rustfmt --edition 2024 src/terminal/tab.rs` 通过；新增回归测试 1 项、terminal tab 聚焦测试 16 项、`cargo check`、完整 `cargo test --quiet` 165 项、`git diff --check` 和 tracking validator 均通过。`cargo check` 仅保留既有 `block v0.1.6` future-incompat warning；真实 GUI 外部请求重连场景尚未手工观察。
## 2026-07-14 完成 SFTP 批量下载与远端拖放验证

- 日期：2026-07-14 08:56 +0800
- 变化摘要：远端多选下载现在作为单个 `DownloadPaths` 命令处理，只创建一个传输 SFTP session 并顺序处理全部项目；普通失败不阻断其余项目，取消中断整批。右键下载复用已选择集合；远端列表支持拖至本地文件面板后下载。运行时、依赖、manifest/lock 和 CI 配置不变。
- 受影响文件：`src/app/actions/sftp.rs`，`src/app/views.rs`，`src/app/views/sftp_panel.rs`，`src/sftp/transfer.rs`，`src/sftp/worker.rs`，`src/sftp/worker/runtime.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`，未联网，未使用多 agent。
- 验证结果：相关 `rustfmt`、右键选择集测试、worker lifecycle 测试、`cargo check`、完整 `cargo test --quiet`（166 项）、`git diff --check` 和 tracking validator 均通过。仅保留既有 `block v0.1.6` future-incompat warning；真实 SFTP 服务端和应用内拖放仍需手工确认。
## 2026-07-14 会话 SFTP Path 环境验证

- 时间：2026-07-14 09:23 +0800
- 变化摘要：保存 SSH 会话新增可选 `sftp_path`；SSH 新建/编辑和保存会话导入导出都会保留该非敏感字段。SFTP 认证成功后，以服务器 home 解析路径并直接读取该目录；空值保持服务器 home。全局及最近本地目录设置未改变。
- 受影响文件：`src/session.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/session.rs`，`src/app/actions/saved_sessions.rs`，`src/app/actions/sftp.rs`，`src/app/workspace.rs`，`src/app/dialogs/ssh.rs`，`src/sftp/worker/runtime.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/features/sftp.md`，`docs/features/sftp.zh.md`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不联网或使用多 agent。
- 验证结果：`rustfmt --edition 2024`、会话 serde 兼容测试、保存会话导入导出测试、SFTP 首目录解析测试、`cargo check`、完整 `cargo test --quiet`（168 项）、`git diff --check` 和 tracking docs validator 均通过。仅保留既有 `block v0.1.6` future-incompat warning；GUI 首目录行为仍需手工确认。

## 2026-07-14 会话级 X11 forwarding 与本机 X server 提示环境验证

- 日期：2026-07-14 10:57 +0800
- 变化摘要：X11 forwarding 从全局配置迁至保存的 `Session.x11_forwarding`，新建会话默认开启；SSH 表单在未发现 `DISPLAY` 或配置路径的本机 X server 时显示安装提示，但不自动安装或启动服务。Windows VcXsrv/Xming 识别分支、远端 `DISPLAY` 分配和本机 relay 仍使用现有 Rust / Tokio / russh 组件。
- 受影响文件：`src/session.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/actions/session.rs`，`src/app/actions/saved_sessions.rs`，`src/app/dialogs/ssh.rs`，`src/app/dialogs/settings/proxy.rs`，`src/backend/ssh.rs`，`src/backend/ssh/x11.rs`，`src/platform/x_server.rs`，`src/config/model.rs`，`src/config/store.rs`，`locales/`，`docs/features/proxy-x11*.md`，跟踪文档。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`，未联网或使用多 agent。
- 验证结果：相关 `rustfmt`、3 项聚焦测试、`cargo check`、完整 `cargo test --quiet`（171 项）、`git diff --check` 和 tracking docs validator 均通过。仅保留既有 `block v0.1.6` future-incompat warning；Windows X server 和远端 GUI 需手工验证。

## 2026-07-14 SFTP 本地文件外拖环境验证

- 时间：2026-07-14 09:51 +0800
- 变化摘要：右侧本地文件列表支持将已有本机文件或文件夹原生拖到 Finder / Explorer。拖动已勾选项目会携带完整勾选集合；拖动未勾选项目只携带当前项目。远端项目继续只能拖入本地面板触发下载，未落盘前不对外伪造本机路径。
- 受影响文件：`src/app/views/sftp_panel.rs`，`docs/features/sftp.md`，`docs/features/sftp.zh.md`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo 和已有 GPUI 拖放接口；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不联网或使用多 agent。
- 验证结果：`rustfmt --edition 2024 src/app/views/sftp_panel.rs`、`cargo check`、完整 `cargo test --quiet`（168 项）和 `git diff --check` 通过。仅保留既有 `block v0.1.6` future-incompat warning；真实 Finder / Explorer 拖放和远端拖入本地下载需要手工确认。

## 2026-07-14 SFTP 本地文件外拖结论更正

- 时间：2026-07-14 09:58 +0800
- 变化摘要：继续审查锁定 GPUI 的平台层后确认，`ExternalPaths` 支持操作系统文件拖入应用，`.on_drag` 仅维护 GPUI 窗口内拖放；没有应用向 Finder / Explorer 发起文件拖放的 macOS `NSDraggingSession`、Windows OLE 或 Linux 等价实现。已撤回无法跨应用放下的本地列表接线，远端拖入本地面板下载保持不变。
- 受影响文件：`src/app/views/sftp_panel.rs`，`docs/features/sftp.md`，`docs/features/sftp.zh.md`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo 和锁定 GPUI；不新增依赖，不修改 `Cargo.toml` / `Cargo.lock`，不联网或使用多 agent。跨应用外拖需未来扩展平台层。
- 验证结果：撤回无效接线后，`rustfmt --edition 2024 src/app/views/sftp_panel.rs`、`cargo check`、完整 `cargo test --quiet`（168 项）、`git diff --check` 和 tracking validator 通过。仅保留既有 `block v0.1.6` future-incompat warning。
## 2026-07-14 完成 SFTP 递归下载与覆盖确认环境验证

- 时间：2026-07-14 13:55 +0800
- 变化摘要：目录下载改为通过 SFTP 递归列出目录并逐文件写入，保留嵌套相对路径，不再创建远端 tar 或本地解压。已有本地文件时，后台传输通过有界事件与 one-shot 响应暂停，界面提供跳过、替换、本次全部替换和本次启动全部替换；最后一种只保留在 `AxShell` 运行期状态，重启自动重置。
- 受影响文件：`src/sftp.rs`，`src/sftp/archive.rs`（删除），`src/sftp/model.rs`，`src/sftp/path.rs`，`src/sftp/transfer.rs`，`src/sftp/worker/runtime.rs`，`src/events.rs`，`src/app.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs.rs`，`src/app/dialogs/sftp_overwrite_confirm.rs`，`src/app/views/layout.rs`，`locales/en.yml`，`locales/zh-CN.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`，未联网或使用多 agent。
- 验证结果：`rustfmt --edition 2024` 通过；覆盖路径安全与任务内全部替换定向测试各 1 项通过；`cargo check` 通过；完整 `cargo test --quiet` 173 项通过；`git diff --check` 通过。仅保留依赖 `block v0.1.6` 的 future-incompat warning；真实 SFTP 服务器和 GUI 交互仍待手工验收。

## 2026-07-14 完成终端链接快捷键视觉环境验证

- 时间：2026-07-14 14:34 +0800
- 变化摘要：终端 URL 和路径在普通鼠标悬停时不再显示下划线或手型指针；仅在 macOS 按下 Command、其他平台按下 Ctrl 时显示可激活视觉。GPUI `on_modifiers_changed` 使按键按下/松开时即时更新，无需移动鼠标；URL/path hover 从行布局缓存键移除，避免普通 hover 重建文本行。
- 受影响文件：`src/app.rs`，`src/app/terminal.rs`，`src/app/actions/terminal.rs`，`src/app/views/terminal_panel.rs`，`src/terminal/element.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`，未联网或使用多 agent。
- 验证结果：`rustfmt --edition 2024` 通过；终端链接修饰键测试 3 项和布局缓存测试 1 项通过；`cargo check` 通过；完整 `cargo test --quiet` 174 项通过；hover 静态审计、`git diff --check` 通过。仅保留依赖 `block v0.1.6` 的 future-incompat warning；真实 GUI 下 Command/Ctrl 按下和松开时的视觉切换仍待手工验收。

## 2026-07-14 完成 SFTP 传输面板环境验证

- 时间：2026-07-14 15:23 +0800
- 变化摘要：传输记录、状态页计数和批量动作按当前 SFTP group 隔离；持久化传输模型记录开始/结束时间并兼容旧配置；传输面板使用固定表头列、平均速度和列表专用共享 hover。行右键与固定宽度的更多按钮复用同一自绘菜单，下载目录通过既有跨平台 `open` 依赖打开。
- 受影响文件：`src/app.rs`，`src/app/sftp.rs`，`src/app/actions/sftp.rs`，`src/app/actions/saved_sessions.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/views/layout.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`src/sftp.rs`，`src/sftp/model.rs`，`locales/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo、既有 `chrono` / `open` 依赖、GPUI `uniform_list` 和共享 FastHover；未新增依赖，未修改 `Cargo.toml` / `Cargo.lock`，未联网或使用多 agent。
- 验证结果：`rustfmt --edition 2024`、传输聚焦测试、`cargo check`、完整 `cargo test --quiet`（178 项）、hover 静态审计和 `git diff --check` 通过；仅保留依赖 `block v0.1.6` 的 future-incompat warning。真实 GUI 会话切换、列宽和右键操作仍待手工验收。

## 2026-07-14 完成 SFTP 系统文件图标缓存环境验证

- 时间：2026-07-14 17:03 +0800
- 变化摘要：SFTP 本地和远端列表通过独立的 `file-icons.json` 持久化缓存显示本机系统文件类型图标。缓存按目录、通用文件和常用扩展名存储，启动时先加载，缺失、损坏、平台或 Linux 主题变化时后台预热并原子更新；行渲染、滚动、hover 和断联后只读取内存映射。
- 受影响文件：`Cargo.toml`，`Cargo.lock`，`src/config/store.rs`，`src/platform.rs`，`src/platform/file_icons.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/views.rs`，`src/app/views/sftp_panel.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：继续使用 Rust 2024 / Cargo、GPUI `uniform_list` 和共享 FastHover；macOS 使用 `NSWorkspace`，Windows 使用 `SHGetFileInfoW`，Linux 使用 `freedesktop-icons` 和 `mime_guess`。已联网检索 KDE、Nautilus 和 Microsoft Shell 的远端类型图标与缓存边界；未使用多 agent。
- 验证结果：`rustfmt --edition 2024`、`cargo check`、`cargo test --quiet file_icon`（5 项）、完整 `cargo test --quiet`（183 项）、SFTP hover 静态审计、`git diff --check` 和 tracking validator 通过。仅保留依赖 `block v0.1.6` 的 future-incompat warning；真实三端 GUI 图标主题、缩放和回退仍需手工验收。

## 2026-07-15 终端同步增量高亮施工前预检

- 时间：2026-07-15 09:18 +0800
- 目的：消除持续输出中新建行先以普通 ANSI 色绘制、随后才补关键词 / URL 色的跳色，同时保留大范围变更的 125ms CPU 限频。
- 改动范围：`src/terminal/tab.rs`，`src/terminal/highlight.rs`，必要时 `src/terminal/element.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 执行内容：确认本机 `rustc 1.96.1` / `cargo 1.96.1` 满足项目 Rust 2024 和 `rust-version = 1.88.0`；审查可视行的 `Rc<RenderRow>` 逐 cell 复用、按视口行号的高亮缓存、URL 的 `WRAPLINE` 扩展和 event loop 的到期刷新。确定不新增依赖、不修改 manifest/lock、不联网、不使用多 agent。
- 验证结果：已确认 `TermDamage::Full` 在滚屏时不能直接代表全屏高亮失效，且 app event loop 已在高亮到期时安排 UI 刷新。待执行 Rust 修改、聚焦测试、`cargo check`、全量测试、构建、空白检查和 tracking validator。
- 风险/待办：跨 `WRAPLINE` URL 需要在当前与前一帧换行边界间扩展同步识别范围；无换行进度条的按列局部识别留作后续 sample 驱动的第二阶段。

## 2026-07-15 完成终端同步增量高亮环境验证

- 时间：2026-07-15 09:42 +0800
- 目的：让少量终端输出变更在首帧即获得关键词 / URL 颜色，同时保持 resize、alternate screen 等大范围变化的 125ms 保护。
- 改动范围：`src/terminal/tab.rs`，`src/terminal/highlight.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 执行内容：`build_visible_rows` 现在返回真正重建行；最多 4 行重建时同步高亮并在 125ms 后仅校正这些行，大范围重建继续批处理。高亮缓存按 `Rc<RenderRow>` 重排，URL 只构建受影响的 `WRAPLINE` 逻辑行，缓存查找按行指针哈希完成。
- 验证结果：`rustfmt --edition 2024 src/terminal/tab.rs src/terminal/highlight.rs`、terminal tab 聚焦测试 18 项、terminal highlight 聚焦测试 20 项、`cargo check`、完整 `cargo test --quiet`（189 项）、`cargo build`、`git diff --check` 和 tracking validator 通过。仅保留依赖 `block v0.1.6` 的 future-incompat warning。
- 风险/待办：仍需用真实 GUI 持续换行输出、跨行 URL、resize / alternate screen 和无换行 `\r` 进度条做颜色首帧及 CPU sample 验收；仅当后者仍是热点时再保留 `LineDamageBounds` 列范围做第二阶段局部识别。

## 2026-07-15 大范围高亮回退调度加固

- 时间：2026-07-15 09:49 +0800
- 目的：确保 resize、alternate screen 等已排队的大范围高亮校正，不会被随后到来的单行输出意外改为同步路径。
- 改动范围：`src/terminal/tab.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 执行内容：为 `HighlightRefresh` 增加已排队大范围刷新的状态；该状态存在时，小行重建继续复用缓存并等待 125ms 全量校正。新增 resize 后紧接 `\r` 输出的回归测试。
- 验证结果：新增回归测试通过；完整 `cargo test --quiet`（190 项）、`cargo check`、`cargo build`、`git diff --check` 和 tracking validator 通过。仅保留依赖 `block v0.1.6` 的 future-incompat warning。
- 风险/待办：真实 GUI 仍需验证连续 ANSI 刷新和 resize / alternate screen 后的首帧颜色；无换行进度条的列范围优化仍为独立第二阶段。

## 2026-07-15 系统 suspend/resume MVP 施工前预检

- 时间：2026-07-15 10:23 +0800
- 目的：在不改动终端/SFTP 架构、不引入原生平台事件依赖的前提下，实现系统唤醒后的跨平台安全恢复兜底。
- 改动范围：`src/app/state/lifecycle.rs`，`src/app/state/monitoring.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/workspace.rs`，`src/events.rs`，`src/backend/ssh.rs`，`src/app/actions/sftp.rs`，`docs/resource-lifecycle*.md`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 执行内容：确认本机 `rustc 1.96.1` / `cargo 1.96.1` 满足 Rust 2024 与 `rust-version = 1.88.0`；审查窗口 lifecycle、单一 event pump、远程系统监控、SSH child task、SFTP work pin/有界关闭、CI 三平台构建。确定以 10 秒以上的调度间隙做通用 resume fallback，不新增依赖、不修改 `Cargo.toml` / `Cargo.lock`、不联网、不使用多 agent。
- 验证结果：已定位恢复时旧 remote probe、`remote_sample_in_flight`、SFTP server-side handle 与恢复风暴风险；待执行恢复 reducer、事件代次、当前上下文探测、聚焦测试、`cargo check`、完整测试、空白检查和 tracking validator。
- 风险/待办：该兜底不能准确区分系统睡眠、调试暂停和严重主线程阻塞；正式阶段需要 macOS `NSWorkspace`、Windows `WM_POWERBROADCAST`、Linux logind D-Bus 接入与三平台实机睡眠/唤醒验收。SSH 仅标记可能失效并让用户重连，不承诺会话恢复；SFTP 不自动续传。

## 2026-07-15 完成系统 suspend/resume MVP 环境验证

- 时间：2026-07-15 11:09 +0800
- 目的：完成不依赖原生平台事件的恢复安全性 MVP，并验证不引入 SSH 重连风暴或 SFTP 自动续传。
- 改动范围：`src/app/state/lifecycle.rs`，`src/app/state/monitoring.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/workspace.rs`，`src/events.rs`，`src/backend/local.rs`，`src/backend/ssh.rs`，`src/backend/ssh/system_probe.rs`，`src/monitoring.rs`，`src/terminal/backend.rs`，`src/terminal/tab.rs`，`src/app/actions/sftp.rs`，`src/app/sftp.rs`，双语资源与跟踪文档。
- 执行内容：以双时钟 10 秒 event-pump 间隙触发可能恢复；以 monitoring generation 忽略旧采样，以 terminal backend generation 拒绝用户重连后的旧健康检查事件。SSH 健康检查最多 5 秒且只针对当前可见 terminal tab；空闲 SFTP 标记为按需重建，带 work pin 或活动/暂停传输的 worker 不主动处置。未新增依赖，未修改 `Cargo.toml`、`Cargo.lock` 或 CI。
- 验证结果：受影响 Rust 文件 `rustfmt --edition 2024` 通过；`cargo test --quiet resume` 4 项通过；`cargo check` 通过；完整 `cargo test --quiet` 194 项通过；`git diff --check` 与 tracking docs validator 通过。仅保留依赖 `block v0.1.6` 的 future-incompat warning。
- 风险/待办：仍须在 macOS、Windows、Linux 实机验证睡眠、可用时休眠、断网、活动 SSH、空闲 SFTP 与带 pin 传输；随后以 `PowerEvent::Suspend/Resume` 适配原生事件，但保留本轮通用兜底。

## 2026-07-16 串口与 Telnet 会话环境验证

- 时间：2026-07-16 17:00 +0800
- 变化摘要：新增 `serialport 4.9.0` 依赖，支持串口端口枚举和非 UI 线程设备 I/O；Telnet 复用现有 Tokio TCP/SOCKS5/HTTP proxy transport，无额外网络依赖。Linux lockfile 引入 `libudev`/`libudev-sys`，CI 已安装 `libudev-dev`。
- 受影响文件：`Cargo.toml`，`Cargo.lock`，`src/session.rs`，`src/backend/serial.rs`，`src/backend/telnet.rs`，`src/terminal/`，`src/app/`，`locales/`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`。
- 更新后的命令或环境：仓库仍使用 Rust 2024 / Cargo，MSRV 为 Rust 1.88；本机 `rustc 1.96.1` / `cargo 1.96.1`。CI 覆盖 Windows、Linux x86_64/aarch64 与 macOS x86_64/aarch64 release build，Linux runner 的现有 `libudev-dev` 满足新增 crate 编译前提。
- 验证结果：受影响 Rust 文件 `rustfmt --edition 2024` 通过；`cargo test --quiet` 220 项通过；`cargo check` 通过；`git diff --check` 和 tracking docs validator 通过。仅保留依赖 `block v0.1.6` future-incompat warning；实体串口设备、权限/占用、设备拔出、Telnet server 协商和断线重试仍需目标平台 GUI 验收。
