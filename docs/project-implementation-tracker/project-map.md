# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/init.rs`，`src/app/actions.rs`，`src/app/views.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`AGENTS.md`，`.agents/skills/`，`src/app/`，`src/session/`，`src/sftp/`，`src/terminal/`，`src/sync/`，`locales/`，`docs/`，`Cargo.toml`，`Cargo.lock`，`build.rs`，`.github/workflows/`，`scripts/`，`assets/*.desktop`
- 排除：`.git/`，`target/`，`assets/` 批量图标/字体资源，构建产物与外部依赖缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `AGENTS.md` | Codex 仓库级持久指令 | 改 agent 默认约束、Rust 模块布局规则、Settings 下拉/hover 性能规则、验证/提交/tag 习惯或长期项目工作约定时 | Codex 默认读取 `AGENTS.md`；`.agent` 不是默认加载文件名，除非客户端配置了 fallback |
| `.agents/skills/` | 项目本地 agent skill 目录 | 需要给后续 agent 固化项目专用工作流、交互规范或可复用检查清单时 | 当前包含 `ax-ashell-fast-hover`，用于统一 AxShell 下拉、长列表和菜单行快速 hover 规则 |
| `src/app.rs` | 应用壳入口，声明 app 子模块、`AxShell` 状态结构和 type re-export | 新增/调整应用级状态字段、输入实体、scroll handle、runtime/event channel、Settings generation、saved context menu state、模块出口或跨模块共享类型时 | 现代 Rust 具名入口；不再使用 `src/app/mod.rs` |
| `src/app/` | 应用壳、功能状态、动作、视图、对话框、输入和生命周期实现 | 调整 AxShell 状态、工作区、SFTP UI、terminal UI、搜索、同步、菜单、启动、共享 hover、Settings 快速菜单或事件泵时 | `input.rs` / `lifecycle.rs` 是真实父模块入口；单文件功能直接位于 `app/` |
| `src/events.rs` | backend、SFTP、监控和同步共用的有界应用事件总线 | 改事件载荷、发送端类型、队列容量或 app event loop 接线时 | 256 条 Tokio channel；远程监控和 SSH 恢复检查事件携带 generation；SFTP 下载文件以 `TransferFileStarted` / `TransferFileFinished` 附着于批量 transfer；terminal `Output` 由 event loop 在连续输出段中批处理 |
| `src/diagnostics.rs` | 跨模块日志脱敏与诊断 helper | 改主机、用户、路径、错误链已知敏感值脱敏或统一诊断字段时 | 不得记录密码、私钥内容、token 或终端输入输出 |
| `src/monitoring.rs` | 本地系统采样、远端采样模型和格式化 | 改 CPU/MEM/NET/DISK 采样、远端 key/value 解析或字节格式化时 | 原 `src/system.rs`；内容限定为监控领域 |
| `src/app/sftp.rs` | app 层每个连接组的 SFTP 页面状态 | 改当前远端路径、首次成功列举标记、分页状态、选择集、预览或 home dir 状态时 | `has_opened_directory` 区分初始占位值与已成功打开的远端目录，供空闲连接重建时恢复；`connection_may_be_stale` 让空闲连接在系统恢复后按需重建；与 `src/sftp/` 协议/传输实现分离 |
| `src/app/actions.rs` | 应用动作层入口 | 改 actions 模块导出时 | 子模块在 `src/app/actions/`；由原 `session/mod.rs`、`session/pane.rs`、`session/saved_sessions.rs`、`sftp/ops.rs`、`terminal/input.rs` 迁入 |
| `src/app/actions/` | 应用动作层实现，集中承载直接操作 `AxShell` 的会话、pane、SFTP UI、本地文件浏览和终端输入动作 | 改 `open_local`、`connect_ssh`、pane split/focus、saved session 分组/重命名、SFTP UI 操作、terminal key/mouse/IME/scroll 行为时 | 入口为 `src/app/actions.rs` |
| `src/app/theme.rs` | app 视觉系统、主题注册、theme profile、custom theme 导入/保存和字体加载 | 改内置主题资源、用户主题加载、theme profile 应用、custom theme 导入/保存/应用/保存路径、Maple Mono 内置字体加载或 ThemeRegistry 接线时 | 直接暴露为 `crate::app::theme`，资源 include 路径按当前文件使用 `../../assets/...` |
| `src/app/hover.rs` | app 内下拉、菜单行和长列表快速 hover 共享接口 | 改 `.fast_hover(cx)` 默认 token、`FastHoverOptions` 参数、预计算 hover tokens 或长列表 hover 语义时 | 新增列表/菜单 hover 时优先复用该接口；不要在 feature 模块重复手写 hover token |
| `src/app/input/` | 原生菜单和快捷键录制/绑定 | 改 App menu、saved SSH 导入/导出菜单、Quit shutdown、workspace/session keybindings、设置页或 SSH 表单按键录制、冲突检测或 keybinding 展示时 | 会话动态快捷键由 `keybinding_recorder` 解析/冲突检测并在工作区事件中分发；父模块入口为 `src/app/input.rs` |
| `src/app/lifecycle/` | 启动、窗口打开、日志/crash hook、`AxShell::new` 和事件泵 | 改启动顺序、日志目录、主窗口 options、runtime event pump、窗口激活、恢复兜底、输入事件或初始化状态时 | 父模块入口为 `src/app/lifecycle.rs`；`startup` 由 app 兼容导出给 main |
| `src/app/state/` | `AxShell` 子状态聚合 | 改 appearance、monitoring、runtime/event channel、窗口生命周期或系统恢复代次时 | search 状态已并入 `src/app/search.rs` |
| `src/config/` | 配置文件模型、默认值、规范化规则和 `ConfigStore` | 改配置 schema/serde 默认、窗口/光标/theme profile、保存会话的本地 SFTP 目录、旧远端目录字段迁移、sync 默认对象名或 custom theme draft/save path 时 | `model.rs` 承载 `ConfigFile` 和值类型，`store.rs` 只做持久化、迁移、归一化和访问器；远端初始目录由会话 `sftp_path` 或服务器 home 决定 |
| `src/platform/` | 平台相关本地集成 | 改本地 X Server、系统文件图标或目标专属原生 API 时 | 入口为 `src/platform.rs`；子模块包括 `x_server.rs` 和 `file_icons.rs` |
| `src/session.rs` | SSH 会话领域模型 | 改 `Session`、`AuthMethod`、`SshConnectionMode`、会话级 `sftp_path` / `shortcut` 或连接模式优先级时 | `shortcut` 是同步配置的一部分，旧会话自动回退为空；类型直接由 `crate::session` 导出；无兼容 config 子模块 |
| `src/backend.rs` | backend 领域入口 | 改 backend 模块导出时 | 子模块为 `auth`、`local`、`proxy`、`ssh` |
| `src/backend/` | 本地/SSH 后端、共享认证、proxy transport、远程系统采样和 PTY/SSH 事件桥接 | 改 SSH 连接、private key、proxy、legacy fallback、本地 shell、后台事件或 backend shutdown 时 | `proxy.rs` 负责 session/env/global proxy 解析和网络连接；config 不再执行 transport |
| `src/sftp.rs` | SFTP 模块入口和兼容 re-export | 改 SFTP 子模块声明、`RemoteEntry` / `PreviewData` / `SftpHandle` / path helper 出口时 | 现代 Rust 具名入口；真实实现位于 `src/sftp/`，UI action 位于 `src/app/actions/sftp.rs` |
| `src/sftp/` | SFTP auth、model、session、browse、preview、transfer、operations 和 worker 实现 | 改 SFTP 连接、分页 cursor、预览、传输、删除或 worker 生命周期时 | `worker.rs` 管 handle/command/pin，`worker/runtime.rs` 单一持有 cursor、active transfer 和 `JoinSet` |
| `src/terminal/` | terminal backend、tab、listener、按键、CWD、transfer、渲染和高亮实现 | 改 backend 协议、terminal tab、输入编码、OSC/CWD、传输模型、render 或 highlight 时 | `src/terminal.rs` 只做入口/re-export；输入 action 仍位于 `src/app/actions/terminal.rs` |
| `src/sync/` | 会话配置加密同步 payload | 判断新增会话字段是否会自动进入同步上传/下载时 | 本轮预计不改传输逻辑，只依赖 `Session` 序列化扩展 |
| `src/main.rs` | 应用初始化入口 | `main`，crash hook，macOS 环境同步，Rayon worker 配置 | 改全局初始化或必须在 GPUI 前生效的进程环境变量时 |
| `locales/` | 中英文界面文案 | 新增 custom theme 分组、提示、保存说明、日志入口和错误消息时 | 需要同步 `en.yml` 和 `zh-CN.yml` |
| `.github/workflows/` | CI / Release 构建和打包元数据 | 改二进制名、artifact 名、macOS bundle Info.plist 或发布路径时 | `release.yml` 手工组装 `.app`，需要与 Cargo 包名一致 |
| `scripts/` | 本地开发/打包脚本与发布辅助脚本 | 改 macOS `.app` 名称、bundle id、图标文件名、签名逻辑、tag/version 映射或发布前 manifest 同步时 | `package-macos-app.sh` 会运行 `cargo build --release` 并组装 bundle；本轮将新增共享版本脚本 |
| `assets/themes/` | 内置 GPUI 主题 JSON 资源 | 改内置主题变体、默认 preset 可引用的 theme 名称、light/dark companion 或主题色调时 | `src/app/theme.rs` 通过 `include_str!("../../assets/themes/*.json")` 注册；`popular.json` 承载 Catppuccin、Dracula/Alucard、Nord、Rose Pine；新增/改名主题需同步 `src/config/model.rs` 默认 profile 和配置归一化测试 |
| `assets/fonts/` | 内置字体二进制、授权与精简清单 | 增删内置 UI/Terminal 字体、字重、变量字体或授权信息时 | `README.md` 记录 family/version/选取范围；`docs/features/bundled-fonts*.md` 记录面向用户的上游仓库和授权入口；`src/app/theme.rs` 通过统一 embedded font family 表注册 |
| `assets/*.desktop` | Linux desktop entry | 改应用显示名、Exec、Icon、StartupWMClass 或 Debian metadata 时 | 当前 desktop 文件为 `assets/ax_shell.desktop` |
| `assets/icons/terminal_icon_all_formats/` | 应用图标资源目录 | 改 `build.rs` Windows icon、macOS bundle icon、Linux desktop/deb icon、非 macOS runtime window icon 或 release 打包图标路径时 | 批量图标不逐项索引；`terminal_icon_256.png` 是 `startup.rs` 非 macOS runtime window icon 的编译期资源 |
| `README.md` / `README.zh.md` | 英文默认项目入口与中文入口 | 改项目定位、快速开始、文档入口、贡献或支持信息时 | 保持简短并在顶部互链；详细功能放入 `docs/` |
| `docs/README.md` / `docs/README.zh.md` | 英文默认文档导航与中文文档导航 | 新增、删除、移动用户/开发/设计文档时 | 用户功能页位于 `docs/features/`，两种语言结构对齐 |
| `docs/features/` | 按功能拆分的双语用户文档 | 改终端/SSH、工作区、SFTP、设置、内置字体、同步、代理/X11、监控或本地数据行为时 | 默认英文 `.md`，中文 `.zh.md`；`bundled-fonts*.md` 记录内置字体上游仓库、版本、样式和授权；截图建议路径记录在各页和 `docs/images/README.md` |
| `docs/` | 用户/开发文档、资源生命周期设计、环境审计和实施跟踪 | 改项目名称、配置目录、同步文件名、休眠策略、打包命令或验证边界时 | 根导航为 `docs/README*.md`；英语 `resource-lifecycle.md` 与中文 `resource-lifecycle.zh.md` 记录深睡分期与资源边界 |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `AGENTS.md` | Codex agents 默认读取的项目约束文件 | Rust module layout，Settings dropdown and hover performance，verification，release tags，git hygiene | 新增模块、拆分文件、Settings 下拉/长列表 hover 性能修正、准备 release tag、提交或判断项目级 agent 行为时 |
| `.agents/skills/ax-ashell-fast-hover/SKILL.md` | 项目本地 AxShell 快速 hover skill | `FastHoverExt` / `FastHoverOptions` 使用规则，Settings `fast_menu`，`uniform_list`，禁用旧 hover 路径，验证清单 | 后续修改 hover_list、Settings 下拉、UI/Terminal Font 菜单、SFTP/sidebar/selector 长列表或自绘 context menu hover 时 |
| `docs/resource-lifecycle.md` | English resource lifecycle, deep-sleep and resume MVP design | State machine, phases, resource policy, resume fallback, verification | Keep English documentation aligned with the Chinese lifecycle design |
| `docs/resource-lifecycle.zh.md` | 中文资源生命周期、深度休眠与恢复 MVP 设计 | 状态机、阶段路线、资源策略、恢复兜底、验证边界 | 实现或评审后台降载、SFTP pin、backend shutdown 与系统睡眠恢复时 |
| `src/config/model.rs` | 配置文件与值模型、默认值和规范化规则 | `ConfigFile`，`LocalShellProfile`，global SFTP local-directory setting，`default_local_shell_profiles`，`default_rayon_threads`，theme/profile/window types | 改配置 serde、local shell profile/argv、全局 SFTP 本机目录、Rayon `1–64` 范围、默认值、theme profile、窗口/标题栏/光标或 custom theme 模型时 |
| `src/config/store.rs` | 本地配置路径、迁移、getter/setter 和 `ConfigStore` | `ConfigStore::load/save`，`file_icons_path`，`normalize_local_shell_profiles`，global SFTP local-directory accessor，`rayon_threads`，`normalize_theme_profiles` | 改配置目录、独立文件图标缓存路径、旧目录迁移、local shell profile、全局 SFTP 本机目录、Rayon worker 设置、Settings 二次快捷键动作、sync 默认对象名、theme profile 或 custom theme draft 时 |
| `src/backend/proxy.rs` | SSH/SFTP transport proxy | `ProxyStream`，`ENV_PROXY`，`connect`，`active` | 改 SOCKS5/HTTP/direct 连接、环境代理或 session/global proxy 优先级时 |
| `src/platform/x_server.rs` | 本地 X Server 平台 helper | `default_app_path`，`local_x_server_available`，`default_display`，`resolve_display`，`launch_args` | 改 X server 缺失提示、macOS XQuartz、Windows VcXsrv/Xming、DISPLAY 或手动启动参数时 |
| `src/platform/file_icons.rs` | 跨平台系统文件类型图标缓存、持久化和解析 | `FileIconCache`，`StoredFileIconCache`，`start_file_icon_cache_refresh`，macOS/Windows/Linux icon loader | 改 SFTP 文件图标来源、`file-icons.json` schema、启动预热、主题失效、原子写入或平台图标转换时 |
| `src/app/state/` | `AxShell` 子状态模块 | `AppearanceState`，`LifecycleState`，`MonitoringState`，`RuntimeState` | 改应用外观、窗口生命周期、系统恢复检测/代次、系统监控或按需 Tokio runtime / backend event channel 状态分组时 |
| `src/app/actions/session.rs` | 会话连接、SSH 表单、local shell profile 和 tab 生命周期 action | `save_ssh_session_from_form`，`record_session_shortcut`，`connect_session_shortcut_if_matched`，`connect_ssh`，`open_saved_session_sftp_only`，`shutdown_all_backends`，`active_snapshot` | 改本地/SSH tab 创建、会话 `sftp_path` / `x11_forwarding` / `shortcut` 保存、saved SSH 只开 SFTP、SSH 表单加载/重置、动态快捷键连接、tab UI 状态回收、断线重试、关闭 tab/group、窗口退出或 active session 查询时 |
| `src/app/actions/pane.rs` | pane tree 操作和 group activation action | `split_current_pane`，`focus_adjacent_pane`，`activate_group_page`，`focus_pane_with_id`，`sync_system_tab_to_active_group` | Local split 必须复制 source tab 的 `LocalShellProfile`；改 split pane、pane focus、splitter drag、active group + 页面联动切换时 |
| `src/app/actions/saved_sessions.rs` | session selector、saved group、saved session 菜单和 share 导入/导出 action | `selector_entries`，`on_selector_key_down`，`saved_session_groups`，`session_tooltip_info`，`copy_saved_session_json`，`import_ssh_session_from_clipboard`，`export_saved_sessions_share_file`，`export_saved_group_share_file`，`export_saved_session_share_file`，`import_saved_sessions_share_file`，`open_saved_session_context_menu`，`commit_saved_group_rename` | 改选择器键盘行为、saved session 分组、侧栏 tooltip 信息、无凭据文件或剪贴板导入/导出、组名展示、右键菜单或重命名时 |
| `src/app/actions/sftp.rs` | UI 侧 SFTP 与本地文件浏览 action | `ensure_sftp_handle_for_group`，`sftp_worker_initial_request`，`sftp_worker_restore_path`，`open_sftp_at_terminal_working_dir`，`open_sftp_and_reveal_path`，`release_sftp_handle_for_group`，`pause_sftp_transfers_in_tab`，`open_sftp_transfer_context_menu`，`trigger_sftp_transfer_context_*`，`download_targets_for_context`，`sweep_idle_sftp_connections`，`mark_idle_sftp_connections_stale` | 改按需建连、明确路径直达、会话 `sftp_path` / home 初始目录、已打开页面重连恢复、显式终端目录跳转、全局本机目录、分页加载、当前 group 的批量传输动作、传输右键菜单、worker 回收/深睡断连或系统恢复时 |
| `src/app/actions/terminal.rs` | terminal 键盘、鼠标、修饰键、滚动和 IME action | `on_terminal_modifiers_changed`，`on_terminal_key_down`，`connect_session_shortcut_if_matched`，`terminal_grid_point_and_side`，`on_terminal_scroll` | 固定 workspace/terminal 快捷键优先于保存 SSH 的动态连接快捷键；改 URL/路径激活快捷键、鼠标命中、选择、滚动、快捷键、粘贴或 IME 候选框位置时 |
| `src/app/theme.rs` | 主题、内置字体注册、当前 theme profile 和 custom theme 逻辑 | `EMBEDDED_FONT_FAMILIES`，`BUILT_IN_FONT_FAMILIES`，`load_fonts`，`load_embedded_themes`，`apply_theme_profile` | app 视觉系统入口；增删字体需同步 `assets/fonts/README.md` 与 Settings 字体排序 |
| `assets/fonts/README.md` | 内置字体 family、版本、样式和用途清单 | Bundled Fonts 表、排除范围、授权入口 | 判断字体包中哪些文件需要编译进应用或核对内部 family 名时 |
| `src/app/lifecycle/startup.rs` | 启动辅助、进程环境、日志 writer/轮转、crash hook 和窗口打开 | `configure_rayon_threads`，`init_logging`，`runtime_log_dir`，`crash_report_dir`，`open_main_window` | 启动期配置必须在 GPUI 初始化前应用；日志 writer 必须保留进程期 guard |
| `assets/icons/terminal_icon_all_formats/terminal_icon_256.png` | 非 macOS runtime 窗口图标和 Linux/Debian 256px 图标资源 | PNG 资源文件 | 改 `include_bytes!`、Debian asset 或 Linux release icon 路径时确认存在性 |
| `src/app/input/app_menu.rs` | GPUI 原生应用菜单注册 | `install`，`app_menus`，`Quit`，About，saved SSH import/export menu items | `Quit` 会先关闭全部 backend；AxShell 菜单可打开设置内 About；File 菜单承载 saved SSH 导入/导出 action；原 `src/app/app_menu.rs` 迁入；通过 `crate::app::app_menu` 兼容导出 |
| `src/app.rs` | 全局 UI 状态结构和 app 子模块出口 | `AxShell` fields，`FileIconCache`，rayon_threads_input，local shell profile inputs，type re-exports，saved session/group context menu state | 新增/调整应用级状态字段、图标缓存、输入实体、scroll handle、runtime/event channel、Settings generation 或 saved sidebar 右键菜单状态时 |
| `src/app/lifecycle/init.rs` | `AxShell` 初始化和默认状态装配 | `AxShell::new`，`FileIconCache::load`，`start_file_icon_cache_refresh`，rayon_threads_input，transfer history recovery，`backend_event_channel` | 配置缓存命中时立即装入类型图标；不命中时启动预热；使用 `src/events.rs` 构造 256 条 Tokio backend event queue |
| `src/app/lifecycle/event_loop.rs` | 输入事件、后台事件分发、系统采样、主题同步和恢复兜底 | `on_input_event`，`start_event_pump`，`detect_system_resume`，`handle_system_resume`，`drain_backend_events`，`TransferStarted` / `TransferProgress`，`flush_terminal_output` | 连续 `Output` 段按 tab 聚合；不执行文件图标解析；长时间未调度仅恢复当前上下文，不自动重连 |
| `src/app/constants.rs` | app 尺寸、快捷键 context、仓库 URL 和版本展示 | layout constants，`TERMINAL_KEY_CONTEXT`，`public_version_label` | 改 UI 固定尺寸、入口链接或公开版本文案时 |
| `src/app/pane.rs` | pane tree 数据模型 | `PaneLayout` | 改 split tree、tab 查找、替换、删除或 pane 统计时 |
| `src/app/workspace.rs` | 工作区页面、tab/group 模型、连接进度、远程采样和布局持久化 | `TabGroup`，`WorkspacePage`，`workspace_tabs`，`set_workspace_page`，`open_settings_page`，`open_about_page`，`request_active_ssh_resume_health_check` | 改 terminal/SFTP/settings tab、SFTP-only group 可见性和关闭回退、当前 tab 可见性、Settings 重开状态重置或指定分页跳转、页面关闭、连接重试、恢复健康检查或监控采样时 |
| `src/app/sftp.rs` | app 层 SFTP 页面、本地浏览、排序和右键菜单状态 | `SftpUiState`，`LocalFileBrowserState`，`SftpContextMenuState`，`SftpTransferContextMenuState` | 改 SFTP UI 状态模型、文件/传输右键菜单而非协议 worker 时 |
| `src/app/terminal.rs` | app 层 terminal 字体/滚动条/链接视觉状态 | `TerminalFontMetrics`，`TerminalScrollbarHandle`，`HoveredUrl`，`terminal_link_visual_active` | 改 terminal UI metrics、scrollbar adapter、URL/path hover 或 Command/Ctrl 激活提示时 |
| `src/app/session_ui.rs` | session selector 与连接进度 UI 模型 | `SelectorEntry`，`ConnectionProgress` | 改 session selector 条目或连接进度遮罩状态时 |
| `src/app/search.rs` | terminal 搜索状态与行为 | `SearchState`，`perform_search`，`search_highlight_map` | 改搜索输入、全缓冲区匹配、跳转或高亮时 |
| `src/app/config_sync.rs` | app 层配置同步动作 | sync credentials，upload/download action，`SyncFinished` | 改 WebDAV/S3 表单取值、同步触发或状态接线时 |
| `src/app/dialogs/` | 弹窗和设置页渲染目录模块 | `ssh.rs`，`selector.rs`，`transfers.rs`，`delete_confirm.rs`，`sftp_close_confirm.rs`，`settings_close_confirm.rs`，`settings/` | 改 SSH 弹窗、session selector、关闭确认、transfer history、下载文件清单、delete confirm、设置页和 About 页面时；入口为 `src/app/dialogs.rs` |
| `src/app/dialogs.rs` | dialogs 目录模块入口和共享 imports | 子模块声明，`crate::app::dialogs` 路由 | 改 dialogs 模块可见性、共享 imports 或新增 dialog 子文件时 |
| `src/app/dialogs/settings/` | 设置页子页面目录 | `general.rs`，`appearance.rs`，`font_page.rs`，`custom.rs`，`fast_menu.rs`，`terminal.rs`，`workspace.rs`，`monitoring.rs`，`sync.rs`，`proxy.rs`，`keybindings.rs`，`shell.rs`，`about.rs`，`help.rs` | 入口为 `src/app/dialogs/settings.rs`；侧栏按 General、Appearance & Theme、Theme Editor、Terminal、Workspace、Monitor & Resources、Connections、Sync、Shortcuts、Help、About 装配；Settings 下拉统一走本地 fast menu |
| `src/app/dialogs/settings/general.rs` | Settings 通用页 | `settings_general_page` | 改显示语言或 Settings 页面自身行为，例如第二次 Settings 快捷键动作时 |
| `src/app/dialogs/settings/appearance.rs` | Settings 外观与主题页 | `settings_appearance_page` | 改主题模式、主题套装选择、标题栏样式、字体/光标分组挂载或外观页命名时 |
| `src/app/dialogs/settings/font_page.rs` | Settings 字体与光标分组 helper | `settings_font_group`，`take_built_in_font_names` | 改 UI/terminal 字体、monospace 过滤、内置字体优先顺序或标签时；候选列表需保持 lazy/cache，避免 hover 时扫描系统字体 |
| `src/app/dialogs/settings/fast_menu.rs` | Settings 专用轻量下拉菜单 helper | `FastMenuItem`，`fast_settings_menu`，`fast_settings_menu_lazy`，`fast_settings_menu_disabled` | 改 Settings 内下拉 hover 反馈、避免 `PopupMenu` hover selected-state 重绘、延迟构建昂贵菜单候选或新增简单 Settings 菜单时 |
| `src/app/dialogs/settings/terminal.rs` | Settings 终端行为与 local shell profile 页 | `settings_terminal_page`，shared `fast_settings_menu` | 改右键复制/粘贴、关键词高亮、默认 shell 选择、profile 程序/逐行 argv 编辑时；不得绕过 shared fast menu |
| `src/app/dialogs/sftp_close_confirm.rs` | 活跃 SFTP 传输的页面关闭确认弹窗 | `show_sftp_transfer_close_dialog` | 改首次确认、group 绑定、二次快捷键、保持页面、后台继续、取消断开或记住动作时 |
| `src/app/dialogs/sftp_overwrite_confirm.rs` | SFTP 本地同名文件覆盖确认弹窗 | `show_next_sftp_overwrite_dialog`，`approve_queued_sftp_overwrites` | 改跳过、替换、任务内全部替换、进程内全部替换、排队或失效请求处理时 |
| `src/app/dialogs/settings_close_confirm.rs` | Settings 页面关闭确认弹窗 | `show_settings_close_confirm_dialog` | 改确认关闭、保持页面、记住第二次快捷键动作或关闭提示文案时 |
| `src/app/dialogs/settings/workspace.rs` | Settings 工作区布局页 | `settings_workspace_page` | 改布局锁定、非激活标签状态色或 reset layout 入口时 |
| `src/app/dialogs/settings/monitoring.rs` | Settings 监控与资源页 | `settings_monitoring_page`，deep sleep，Rayon worker numeric input | 改监控仪表盘、资源策略、Rayon worker 或监控位置设置时 |
| `src/app/dialogs/settings/custom.rs` | Custom theme 设置页 | `settings_custom_page` | 改 custom theme name、基于预设修改语义、亮/暗高级变体、override 输入、保存路径、浏览目录、导入主题和保存/重置入口时 |
| `src/app/dialogs/settings/proxy.rs` | Settings 连接与网络页 | `settings_connection_page` | 改 SSH 重试、SFTP 默认本地目录、SFTP 传输关闭策略、全局代理或 X11 转发设置时 |
| `src/app/dialogs/settings/shell.rs` | 设置页外层交互壳 | `settings_page_shell` | 改设置页内 keybinding 录制、关闭确认、OpenSession/NewSsh/Prev/NextTab 捕获或失焦取消录制时 |
| `src/app/views/` | 主工作区视图目录模块，按渲染区域拆分 SFTP、监控、侧栏、顶部标签、终端 pane 和整体布局 | `helpers.rs`，`layout.rs`，`monitoring.rs`，`sftp_panel.rs`，`sidebar.rs`，`tab_bar.rs`，`terminal_panel.rs` | 增加固定 Local Terminal 入口、调整 SFTP 双列面板、saved session 分组、侧栏折叠态、顶部标签、监控面板或主布局时 |
| `src/app/views.rs` | views 目录模块入口和共享 imports | 子模块声明，`crate::app::views` 路由 | 改 views 模块可见性、共享 imports 或新增 views 子文件时 |
| `src/app/views/layout.rs` | `Render for AxShell` 和顶层菜单/workspace/body 布局 | `render`，platform menu row，workspace page route，resizable panels，overlays | 改 Windows/Linux 全宽菜单、主布局、SFTP 页面接线、自绘文件/传输/saved session/saved group 右键菜单或全局 overlays 时 |
| `src/app/views/sftp_panel.rs` | SFTP 双列页面主体 | `render_sftp_panel`，`RemoteSftpDrag`，`FileIconCache` 接线，远端加载更多页脚 | 改远端/本地文件列表、系统文件图标、远端到本地面板内部拖放下载、分页加载按钮、表头、上传下载按钮、隐藏文件开关、SFTP 右键菜单或双栏布局时 |
| `src/app/views/sftp_panel/sort.rs` | SFTP 远端/本地列表排序 helper | `sort_sftp_entries`，`SftpSortableEntry` | 改名称/大小/修改时间排序规则、目录优先或本地/远端排序一致性时 |
| `src/app/views/sftp_panel/transfer_panel.rs` | SFTP 页面传输标签、固定列表列和传输行渲染 | `render_sftp_transfer_panel`，`render_sftp_transfer_header`，`render_sftp_transfer_row`，任务当前文件/文件数、时间/速度 helper | 改当前 group 传输隔离、状态页计数、固定列、下载文件清单入口、列表专用 hover、更多/右键操作或传输状态文案时 |
| `src/app/views/sidebar.rs` | 展开/收起侧栏和 saved session entry 渲染 | `sidebar`，`render_collapsed_sidebar`，saved sidebar row renderers | 展开/折叠会话行分别缓存 tooltip 和右键复制信息；改 SAVED 列表、`uniform_list` 可见行、分组展开/重命名、分组/单条 SSH 右键入口、折叠态入口或本地终端固定入口时 |
| `src/app/views/monitoring.rs` | 底部/侧栏监控面板 | `render_monitoring_panel`，`render_sidebar_monitoring_panel` | 改 CPU/MEM/NET/DISK 展示、sparkline、监控位置或滚动条时 |
| `src/app/views/tab_bar.rs` | 顶部 tab bar 和 split/search 操作按钮 | `render_tab_bar`，`tabs_scroll_handle` | 改编号 terminal/SFTP 标签、当前 tab 自动可见、SFTP 标签关闭、tab 选择/关闭、settings tab、split pane 按钮或 tab bar 搜索按钮时 |
| `src/app/views/terminal_panel.rs` | 终端工作区、SFTP 页面、settings 页面承载和 pane tree 渲染 | `render_terminal_panel`，`render_pane_tree`，terminal link cursor gate | SFTP 页面复用保存 SSH 的动态连接快捷键；改终端 focus/key/mouse、Command/Ctrl 链接手型、右侧滚动槽、pane splitter、disconnect overlay、SFTP 页面挂载或 settings 页面承载时 |
| `src/app/views/helpers.rs` | views 内部小 helper | `bind_titlebar_drag`，`collapsed_sidebar_abbrev`，`render_home_page` | 改集成标题栏拖动、折叠侧栏简称或空首页时 |
| `src/backend/auth.rs` | SSH / SFTP 共用私钥解析和 public key 算法 fallback helper | `load_session_private_key`，`private_keys_with_algs` | 改 inline key、key path、passphrase 或 RSA SHA512/SHA256/none fallback 顺序时 |
| `src/backend/local.rs` | 本地 PTY 后端 | `LocalBackendShutdown`，`spawn_local_terminal`，`local_shell_command` | 使用 `LocalShellProfile` 的程序与逐项 argv 启动；Unix 设置 `SHELL` 为目标程序，Windows 不覆盖以兼容 WSL；改本地 shell、PTY resize、child kill、reader/writer reaper 或 backend event 输出时 |
| `src/backend/ssh.rs` | SSH 终端运行循环、PTY/shell 生命周期、结构化诊断和 channel handler 接线 | `SshBackendShutdown`，`spawn_ssh_terminal`，`run_ssh`，`cancel_ssh_child_tasks`，`ClientHandler` | 改 SSH 命令循环、会话级 X11 request、PTY 请求、shell 生命周期、远程采样/CWD task、关闭语义、错误日志或 handler 接线时 |
| `src/backend/ssh/connection.rs` | SSH TCP/proxy 连接、认证、敏感字段日志和 default/legacy 模式选择 | `connect_and_authenticate`，`connect_with_mode_priority`，`connect_with_mode`，`key_source_label` | 改 SSH 密码/密钥认证、proxy 连接、连接模式 fallback、认证状态上报、日志脱敏或 resolved mode 写回事件时 |
| `src/backend/ssh/legacy.rs` | SSH legacy 算法配置和协商错误摘要 | `ssh_client_config`，`negotiation_error_details`，`negotiation_error_short_reason` | 改老服务器算法兼容、`No common algorithm` 诊断或默认/legacy 模式配置时 |
| `src/backend/ssh/system_probe.rs` | SSH 远程系统采样脚本和输出解析入口 | `sample_remote_system_with_handle`，`REMOTE_SYSTEM_PROBE` | 改远程 CPU/MEM/SWAP/NET/DISK 采样命令、Linux/Darwin 兼容或采样 session 错误处理时 |
| `src/backend/ssh/x11.rs` | SSH X11 forwarding 状态、cookie 校验、本地 relay 和脱敏诊断 | `X11ForwardingState::for_session`，`handle_x11_channel` | 改会话级 X11 开关、DISPLAY 选择、cookie 替换、本地 Unix/TCP 连接、来源地址脱敏或 X11 channel relay 时 |
| `src/sftp.rs` | SFTP 模块入口和兼容出口 | module declarations，`RemoteEntry`，`PreviewData`，`SftpHandle`，path re-export | 改 SFTP 模块树或既有 `crate::sftp::*` 路径时 |
| `src/sftp/auth.rs` | SFTP 连接认证 | `connect_and_authenticate`，`SftpClientHandler` | 改 SFTP SSH 认证主流程或 server key 策略时；private key 解析改 `src/backend/auth.rs` |
| `src/sftp/model.rs` | SFTP 公共数据、覆盖决策与持久化传输模型 | `RemoteEntry`，`SftpOverwriteRequest`，`Transfer`，`TransferFile`，`TransferState`，`TransferFileState`，`unix_timestamp_secs` | 改目录条目、覆盖请求/响应、任务或文件状态、传输开始/结束时间、旧 `Cancelled` 或旧历史配置兼容时 |
| `src/sftp/path.rs` | SFTP 远程路径和格式化 helper | `join_remote`，`parent_dir`，`format_mtime` | 改远程路径拼接、父目录解析、mtime 展示或文件大小格式化时 |
| `src/sftp/session.rs` | SFTP channel/session 构造和 timeout 常量 | `open_sftp_session`，`open_browse_sftp_session`，`open_transfer_sftp_session` | 改普通、raw browse 或 transfer SFTP channel 建立时 |
| `src/sftp/browse.rs` | 目录 cursor、分页预算、reveal 和目录事件 | `BrowseCursor`，`DirectoryPage`，`open_and_emit_browser_page`，`read_next_browser_page` | 改分页、EOF、目录上限、cursor 关闭或 reveal path 时 |
| `src/sftp/preview.rs` | 文件与受限目录预览 | `preview_impl`，`directory_preview_body` | 改二进制判断、预览字节预算或目录截断提示时 |
| `src/sftp/transfer.rs` | 传输 flag、递归下载、覆盖决策和上传实现 | `TransferStateFlag`，`DownloadOverwritePolicy`，`download_path_impl`，`download_file_impl`，下载文件开始/结束事件，`upload_paths_impl` | 改暂停/恢复/取消、递归目录下载、文件明细状态、冲突响应、批量完成事件、进度或并发上传时 |
| `src/sftp/operations.rs` | SFTP 通用远程操作 | `recursive_delete` | 改递归删除行为时 |
| `src/sftp/worker.rs` | SFTP handle、command、初始浏览/定位请求、work pin 和 shutdown controller | `SftpHandle`，`SftpInitialRequest`，`SftpCommand::DownloadPaths`，`SftpWorkPin`，`spawn_sftp` | 改批量下载 command API、首次直达路径或普通目录浏览、pin 计数、worker 创建或有界关闭时 |
| `src/sftp/worker/runtime.rs` | SFTP 单一命令循环和 child task 所有权 | `run_sftp`，`SftpInitialRequest`，`SftpCommand::DownloadPaths`，`cancel_sftp_child_tasks` | 改服务器 home 获取、首次显式定位或会话默认远端目录的首次列目录、单 session 批量下载调度、下载文件明细、cursor/transfer state、remote edit watcher 或 child task 回收时 |
| `src/terminal/element.rs` | terminal 行级 `ShapedLine` 缓存、字体 metrics、等宽保护与实时覆盖层绘制 | `TerminalElement`，`GridLayoutCache`，`GridLayoutKey`，`hovered_url_underlines`，`RowLayout` | 改终端 text/background/block 准备缓存、搜索/keyword 色彩失效键、Command/Ctrl 下的 URL/path 下划线、光标/选择/IME 或字体间距问题时 |
| `src/terminal.rs` | terminal 模块入口和公开出口 | module declarations，backend/tab/input exports | 改 terminal 模块树或既有 `crate::terminal::*` 路径时 |
| `src/terminal/backend.rs` | terminal backend command、sender 和 shutdown controller | `BackendCommand`，`BackendTx`，`BackendShutdown` | 改本地/SSH terminal 命令协议或关闭控制时；全应用事件在 `src/events.rs` |
| `src/terminal/tab.rs` | terminal tab、共享行块 snapshot、滚屏复用、selection 和高亮节流 | `TerminalTab`，`RenderSnapshot`，`RenderRow`，`build_visible_rows`，`render_row_matches_term`，`SnapshotCache`，`HighlightRefresh`，`TermDamage` | `feed` / resize / scroll 累积并 reset damage；底部滚屏仅在逐 cell 对照当前 grid 后复用行；关键词颜色最多每 125ms 刷新且只随已验证行移动 |
| `src/terminal/highlight.rs` | keyword、HTTP/IP/port 与 URL/path 终端识别及行级高亮缓存 | `HighlightCache`，`highlight_rows_incremental`，`highlight_row`，`apply_url_highlights`，`find_terminal_target_at_cell` | 改关键词性能或 URL/path 命中时；行块身份变更才重算 keyword，URL 以当前/前一帧自动换行边界扩展逻辑行 |
| `src/terminal/listener.rs` | alacritty terminal listener 和 dimensions | `TerminalListener`，`TerminalSize`，`new_term` | 改 PTY write/title event 或 terminal 初始化尺寸时 |
| `src/terminal/key_encoding.rs` | 跨平台终端按键编码 | `encode_key`，`TerminalModifiers` | 改 Ctrl/Alt/Command、cursor mode 或 readline 导航编码时 |
| `src/terminal/cwd.rs` | OSC shell working directory 解析 | `extract_shell_working_directory`，`parse_working_directory_osc` | 改 OSC 7/633/1337、URI 或 percent decode 时 |
| `src/main.rs` | 应用启动初始化顺序 | `main()` | 新增用户 theme 文件初始加载和 watch 入口 |
| `Cargo.toml` | Cargo 包、依赖、Debian metadata | `[package]`，target-specific system icon dependencies，`[package.metadata.deb]` | 改 crate/package name、二进制名、deb assets、平台图标接口或依赖时 |
| `Cargo.lock` | 根包与依赖锁文件 | `[[package]] name = "ax_shell"` | 若发布时临时同步 root package version，需要确认 lock 中 root package 条目一起更新 |
| `build.rs` | Cargo build script | `main`，compile-time env，Windows icon resource | 改构建期环境变量、About/日志版本注入、Windows exe 图标或 resource 编译时 |
| `.github/workflows/release.yml` | 多平台 release 构建与 GitHub Release 发布 | `build`，`publish`，macOS bundle heredoc | 改 release artifact、bundle display name、binary copy path 或 cask 注释模板时 |
| `scripts/package-macos-app.sh` | 本地 macOS bundle 打包脚本 | `APP_NAME`，`DISPLAY_NAME`，`BUNDLE_ID`，`Info.plist` | 改 macOS 本地打包输出和显示名时 |
| `scripts/release_version.py` | 共享发布版本规则脚本 | tag 解析、Cargo semver/public version/bundle version 派生、manifest/lock 同步 | 改 tag 作为唯一版本源的全链路规则时 |
| `examples/dev_reload.rs` | restart-based 开发重载 | `build_app`，`prepare_macos_app_bundle`，env constants | 改 dev 二进制名、bundle id、开发 app 显示名或日志文件名时 |

## 常用定位

- `rg -n 'RuntimeState|ensure_runtime|release_runtime|runtime_handle|runtime_state\\.runtime' src/app`
- `rg -n 'TerminalOutputBatch|flush_terminal_output|dirty_generation|feed_advances_dirty_generation' src/app/lifecycle/event_loop.rs src/terminal/tab.rs`
- `rg -n 'render_snapshot|SnapshotCache|HighlightCache|highlight_cells_incremental|keyword_highlight' src/terminal src/app`
- `rg -n 'TermDamage|damage\(|reset_damage|LineDamageBounds|dirty_rows' src/terminal`
- `rg -n 'AxAshell|ax_ashell|AX_ASHELL|AxShell|ax_shell|AX_SHELL' Cargo.toml Cargo.lock src examples scripts .github assets README.md README.zh.md docs`
- `rg -n 'GITHUB_REF_NAME|refs/tags|CFBundleShortVersionString|RELEASE_PUBLIC_VERSION|CARGO_PKG_VERSION|version = ' .github/workflows scripts src build.rs Cargo.toml Cargo.lock`
- `rg -n 'render_sftp_panel|render_monitoring_panel|sidebar\\(|render_collapsed_sidebar|render_tab_bar|render_terminal_panel|render_pane_tree|WorkspacePage' src/app/views src/app`
- `rg -n 'sidebar\\(|render_collapsed_sidebar|saved_session_groups|open_local' src/app/views src/app/actions`
- `rg -n 'split_current_pane|focus_adjacent_pane|activate_group|activate_group_page|sync_system_tab_to_active_group' src/app/actions`
- `rg -n 'workspace_tabs|active_workspace_tab_index|tabs_scroll_handle|scroll_to_item' src/app`
- `rg -n 'SftpHandle|BrowseCursor|spawn_sftp|run_sftp|open_and_emit_browser_page|read_next_browser_page|JoinSet|connect_and_authenticate|join_remote|parent_dir' src/sftp`
- `rg -n 'sftp_session|open_saved_session_sftp_only|group_has_terminal_tab|open_sftp_page' src/app locales`
- `rg -n 'BackendShutdown|shutdown_all_backends|SshBackendShutdown|LocalBackendShutdown|cancel_ssh_child_tasks' src`
- `rg -n 'load_session_private_key|private_keys_with_algs|key_source_label' src/backend src/sftp`
- `rg -n 'custom_theme|ThemeRegistry|load_embedded_themes|apply_theme_preferences|save_custom' src/app src/session src/main.rs`
- `rg -n '\\.hover\\(|\\.on_hover\\(|\\.on_mouse_move\\(|fast_hover|uniform_list\\(' src/app src/terminal`
- `rg -n 'terminal_font_metrics|terminal_font_is_monospace|terminal_cell_width|terminal_line_height|layout_grid' src/app src/config src/session src/terminal`
- `rg -n 'observe_window_activation|WindowLifecycleState|deep_sleep_after_minutes|sample_system_if_due|sync_theme_if_due' src/app src/config`
- `rg -n 'observe_event_pump_tick|detect_system_resume|handle_system_resume|remote_sample_generation|CheckConnection|ConnectionHealthy|mark_idle_sftp_connections_stale' src`
- `rg -n 'RAYON_NUM_THREADS|rayon_threads|configure_rayon_threads' src/main.rs src/app/lifecycle/startup.rs src/config src/app/dialogs/settings`
- `rg -n 'rayon_threads_input|commit_rayon_threads_input|RAYON_THREADS_(MIN|MAX)' src/app src/config`
- `rg -n 'LocalShellProfile|local_shell_profiles|default_local_shell_profile|spawn_local_terminal' src/config src/backend/local.rs src/terminal/tab.rs src/app`
- `rg -n 'x11_forwarding|local_x_server_available|request_x11|resolve_display|windows_x_server_kind' src/session.rs src/app src/backend src/platform`
- `rg -n 'sftp_path|default_local_sftp_path|last_(local|remote)_sftp_paths|configured_default_local_browser_dir|restore_active_local_sftp_path|open_sftp_at_terminal_working_dir' src/session.rs src/config src/app src/sftp`
- `rg -n 'session_shortcut|shortcut|record_session_shortcut|connect_session_shortcut_if_matched' src/session.rs src/app locales docs/features`
- `rg -n 'OpenAbout|open_about_page|copy_saved_session_json|import_ssh_session_from_clipboard' src/app src/main.rs locales`
- `rg -n 'FileIconCache|file_icons_path|file-icons.json|start_file_icon_cache_refresh' src/config src/platform src/app`
- `rg -n 'ThemeConfig|ThemeSet|try_parse_color|watch_dir|default_light_theme|default_dark_theme' ~/.cargo/git/checkouts/gpui-component-*`
- `cargo check`

## 忽略与未索引

- `assets/icons/` 批量图标、`assets/fonts/*.ttf`、`target/` 未逐项索引：字体二进制由 `assets/fonts/README.md` 汇总，构建产物不展开

## 刷新规则

- 刷新触发：项目命名、Cargo 包/二进制名、构建脚本、配置目录、同步默认文件名、启动初始化、独立文件图标缓存、local shell profile/PTY argv、Rayon worker 配置或自定义值范围、日志/crash hook、Tokio runtime 生命周期、terminal Output batching / dirty generation / `TermDamage` / snapshot / highlight cache / URL-path modifier visuals、非 macOS runtime 图标资源、release workflow、tag/version 映射规则、manifest/lock 临时同步、macOS/Linux 打包元数据、仓库级 agent 指令、项目本地 agent skill、Rust 模块布局约束、共享快速 hover 接口、Settings 下拉/长列表 hover 性能规则、内置字体 family/字重/授权/排序、SAVED 侧栏入口、theme profile 默认套装、theme 设置页主路径、custom theme 持久化模型、custom theme 导入/保存路径、custom theme 实时预览、theme file 注册策略、内置 theme JSON、设置页字段分组、Settings 下拉菜单、Settings 关闭确认偏好/dialog、theme list 行为、terminal 亮度语义、终端字体 metrics、窗口激活/后台/深睡状态、系统恢复兜底/远程监控代次/SSH 健康检查、workspace page / tab 模型、Settings About 菜单入口、terminal backend shutdown controller、会话 `sftp_path` / `x11_forwarding` / `shortcut`、无凭据 session JSON 剪贴板往返、本机 X server 检测、SFTP 按需页面/标签关闭/快捷键焦点、SFTP worker/task 关闭所有权、SFTP 分页或受限目录浏览/预览、SFTP 递归下载/覆盖确认/传输标签面板、SSH 连接认证/legacy/远程系统探针/X11 relay、settings Custom/shell 拆分、app/backend 根目录收拢、app/actions/state/config/session/sftp/backend/ui/dialogs 模块拆分或用户文档范围发生变化时刷新
- 最近依据：`AGENTS.md`，`.agents/skills/ax-ashell-fast-hover/SKILL.md`，`src/session.rs`，`src/app/actions/saved_sessions.rs`，`src/app/input/app_menu.rs`，`src/app/views/sidebar.rs`，`docs/project-env-audit/current.md`

## 最后更新时间

- 2026-07-16 11:30 +0800
