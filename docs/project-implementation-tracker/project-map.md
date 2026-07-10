# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/init.rs`，`src/app/actions.rs`，`src/app/views.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`AGENTS.md`，`src/app/`，`src/session/`，`src/sftp/`，`src/terminal/`，`src/sync/`，`locales/`，`docs/`，`Cargo.toml`，`Cargo.lock`，`.github/workflows/`，`scripts/`，`assets/*.desktop`
- 排除：`.git/`，`target/`，`assets/` 批量图标/字体资源，构建产物与外部依赖缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `AGENTS.md` | Codex 仓库级持久指令 | 改 agent 默认约束、Rust 模块布局规则、验证/提交/tag 习惯或长期项目工作约定时 | Codex 默认读取 `AGENTS.md`；`.agent` 不是默认加载文件名，除非客户端配置了 fallback |
| `src/app.rs` | 应用壳入口，声明 app 子模块、`AxShell` 状态结构和 type re-export | 新增/调整应用级状态字段、输入实体、scroll handle、runtime/event channel、模块出口或跨模块共享类型时 | 现代 Rust 具名入口；不再使用 `src/app.rs` |
| `src/app/` | 应用壳、功能状态、动作、视图、对话框、输入和生命周期实现 | 调整 AxShell 状态、工作区、SFTP UI、terminal UI、搜索、同步、菜单、启动或事件泵时 | `input.rs` / `lifecycle.rs` 是真实父模块入口；单文件功能直接位于 `app/` |
| `src/events.rs` | backend、SFTP、监控和同步共用的有界应用事件总线 | 改事件载荷、发送端类型、队列容量或 app event loop 接线时 | 256 条 Tokio channel；不属于 terminal 领域 |
| `src/monitoring.rs` | 本地系统采样、远端采样模型和格式化 | 改 CPU/MEM/NET/DISK 采样、远端 key/value 解析或字节格式化时 | 原 `src/system.rs`；内容限定为监控领域 |
| `src/app/sftp.rs` | app 层每个连接组的 SFTP 页面状态 | 改当前远端路径、分页状态、选择集、预览或 home dir 状态时 | 与 `src/sftp/` 协议/传输实现分离 |
| `src/app/actions.rs` | 应用动作层入口 | 改 actions 模块导出时 | 子模块在 `src/app/actions/`；由原 `session/mod.rs`、`session/pane.rs`、`session/saved_sessions.rs`、`sftp/ops.rs`、`terminal/input.rs` 迁入 |
| `src/app/actions/` | 应用动作层实现，集中承载直接操作 `AxShell` 的会话、pane、SFTP UI、本地文件浏览和终端输入动作 | 改 `open_local`、`connect_ssh`、pane split/focus、saved session 分组/重命名、SFTP UI 操作、terminal key/mouse/IME/scroll 行为时 | 入口为 `src/app/actions.rs` |
| `src/app/theme.rs` | app 视觉系统、主题注册、custom theme 和字体加载 | 改内置主题资源、用户主题加载、custom theme 保存/应用、Maple Mono 内置字体加载或 ThemeRegistry 接线时 | 直接暴露为 `crate::app::theme`，资源 include 路径按当前文件使用 `../../assets/...` |
| `src/app/input/` | 原生菜单和快捷键录制/绑定 | 改 App menu、Quit shutdown、workspace keybindings、设置页按键录制、冲突检测或 keybinding 展示时 | 父模块入口为 `src/app/input.rs` |
| `src/app/lifecycle/` | 启动、窗口打开、日志/crash hook、`AxShell::new` 和事件泵 | 改启动顺序、日志目录、主窗口 options、runtime event pump、输入事件或初始化状态时 | 父模块入口为 `src/app/lifecycle.rs`；`startup` 由 app 兼容导出给 main |
| `src/app/state/` | `AxShell` 子状态聚合 | 改 appearance、monitoring、runtime/event channel 或窗口生命周期状态时 | search 状态已并入 `src/app/search.rs` |
| `src/config/` | 配置文件模型、默认值、规范化规则和 `ConfigStore` | 改配置 schema/serde 默认、窗口/光标/主题模型、旧目录迁移、sync 默认对象名或 custom theme draft 时 | `model.rs` 承载 `ConfigFile` 和值类型，`store.rs` 只做持久化、迁移和访问器 |
| `src/platform/` | 平台相关本地集成 | 改本地 X Server 路径发现、DISPLAY 选择或 Windows 启动参数时 | 入口为 `src/platform.rs`；当前子模块为 `x_server.rs` |
| `src/session.rs` | SSH 会话领域模型 | 改 `Session`、`AuthMethod`、`SshConnectionMode` 或连接模式优先级时 | 类型直接由 `crate::session` 导出；无兼容 config 子模块 |
| `src/backend.rs` | backend 领域入口 | 改 backend 模块导出时 | 子模块为 `auth`、`local`、`proxy`、`ssh` |
| `src/backend/` | 本地/SSH 后端、共享认证、proxy transport、远程系统采样和 PTY/SSH 事件桥接 | 改 SSH 连接、private key、proxy、legacy fallback、本地 shell、后台事件或 backend shutdown 时 | `proxy.rs` 负责 session/env/global proxy 解析和网络连接；config 不再执行 transport |
| `src/sftp.rs` | SFTP 模块入口和兼容 re-export | 改 SFTP 子模块声明、`RemoteEntry` / `PreviewData` / `SftpHandle` / path helper 出口时 | 现代 Rust 具名入口；真实实现位于 `src/sftp/`，UI action 位于 `src/app/actions/sftp.rs` |
| `src/sftp/` | SFTP auth、model、session、browse、preview、transfer、archive、operations 和 worker 实现 | 改 SFTP 连接、分页 cursor、预览、传输、归档、删除或 worker 生命周期时 | `worker.rs` 管 handle/command/pin，`worker/runtime.rs` 单一持有 cursor、active transfer 和 `JoinSet` |
| `src/terminal/` | terminal backend、tab、listener、按键、CWD、transfer、渲染和高亮实现 | 改 backend 协议、terminal tab、输入编码、OSC/CWD、传输模型、render 或 highlight 时 | `src/terminal.rs` 只做入口/re-export；输入 action 仍位于 `src/app/actions/terminal.rs` |
| `src/sync/` | 会话配置加密同步 payload | 判断新增会话字段是否会自动进入同步上传/下载时 | 本轮预计不改传输逻辑，只依赖 `Session` 序列化扩展 |
| `src/main.rs` | 应用初始化入口 | 增加全局初始化、custom theme watch/load、补入口初始化顺序时 | 本轮在 `main()` 第一行注册 panic hook，保证早期启动 panic 可落 crash 文件 |
| `locales/` | 中英文界面文案 | 新增 custom theme 分组、提示、保存说明、日志入口和错误消息时 | 需要同步 `en.yml` 和 `zh-CN.yml` |
| `.github/workflows/` | CI / Release 构建和打包元数据 | 改二进制名、artifact 名、macOS bundle Info.plist 或发布路径时 | `release.yml` 手工组装 `.app`，需要与 Cargo 包名一致 |
| `scripts/` | 本地开发/打包脚本与发布辅助脚本 | 改 macOS `.app` 名称、bundle id、图标文件名、签名逻辑、tag/version 映射或发布前 manifest 同步时 | `package-macos-app.sh` 会运行 `cargo build --release` 并组装 bundle；本轮将新增共享版本脚本 |
| `assets/*.desktop` | Linux desktop entry | 改应用显示名、Exec、Icon、StartupWMClass 或 Debian metadata 时 | 当前 desktop 文件为 `assets/ax_shell.desktop` |
| `assets/icons/terminal_icon_all_formats/` | 应用图标资源目录 | 改 `build.rs` Windows icon、macOS bundle icon、Linux desktop/deb icon、非 macOS runtime window icon 或 release 打包图标路径时 | 批量图标不逐项索引；`terminal_icon_256.png` 是 `startup.rs` 非 macOS runtime window icon 的编译期资源 |
| `docs/` | README、用户/开发文档、资源生命周期设计、环境审计和实施跟踪 | 改项目名称、配置目录、同步文件名、休眠策略、打包命令或验证边界时 | `resource-lifecycle*.md` 记录深睡分期与资源边界 |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `AGENTS.md` | Codex agents 默认读取的项目约束文件 | Rust module layout，verification，release tags，git hygiene | 新增模块、拆分文件、准备 release tag、提交或判断项目级 agent 行为时 |
| `docs/resource-lifecycle.md` | 中文资源生命周期与深度休眠设计 | 状态机、阶段路线、资源策略、验证边界 | 实现或评审后台降载、SFTP pin、backend shutdown 与系统睡眠恢复时 |
| `docs/resource-lifecycle.en.md` | English resource lifecycle and deep-sleep design | State machine, phases, resource policy, verification | Keep English documentation aligned with the Chinese lifecycle design |
| `src/config/model.rs` | 配置文件与值模型、默认值和规范化规则 | `ConfigFile`，`SavedWindowBounds`，`TitleBarStyle`，`CursorStyle`，`CustomThemeConfig` | 改配置 serde、默认值、输入规范化、Settings 二次快捷键动作、窗口/标题栏/光标或 custom theme 模型时 |
| `src/config/store.rs` | 本地配置路径、迁移、getter/setter 和 `ConfigStore` | `ConfigStore::load/save`，`config_root_dir_path`，config path helpers | 改配置目录、旧目录迁移、Settings 二次快捷键动作、sync 默认对象名、custom theme draft 或 registry file 路径时 |
| `src/backend/proxy.rs` | SSH/SFTP transport proxy | `ProxyStream`，`ENV_PROXY`，`connect`，`active` | 改 SOCKS5/HTTP/direct 连接、环境代理或 session/global proxy 优先级时 |
| `src/platform/x_server.rs` | 本地 X Server 平台 helper | `default_app_path`，`default_display`，`resolve_display`，`launch_args` | 改 macOS XQuartz、Windows VcXsrv/Xming、DISPLAY 或空闲 display 选择时 |
| `src/app/state/` | `AxShell` 子状态模块 | `AppearanceState`，`LifecycleState`，`MonitoringState`，`RuntimeState` | 改应用外观、窗口生命周期、系统监控或 Tokio backend event channel 状态分组时 |
| `src/app/actions/session.rs` | 会话连接、SSH 表单、tab 生命周期和 active session 查询 action | `open_local`，`connect_ssh`，`open_ssh_session`，`shutdown_all_backends`，`clear_tab_ui_state`，`handle_tab_close`，`active_snapshot` | 改本地/SSH tab 创建、SSH 表单加载/重置、当前 workspace tab 可见性、tab UI 状态回收、断线重试、关闭 tab/group、窗口退出或 active session 查询时 |
| `src/app/actions/pane.rs` | pane tree 操作和 group activation action | `split_current_pane`，`focus_adjacent_pane`，`activate_group_page`，`focus_pane_with_id`，`sync_system_tab_to_active_group` | 改 split pane、pane focus、splitter drag、active group + 页面联动切换、当前 workspace tab 自动可见或监控 tab 跟随 group 时 |
| `src/app/actions/saved_sessions.rs` | session selector 和 saved group action | `selector_entries`，`on_selector_key_down`，`saved_session_groups`，`commit_saved_group_rename` | 改选择器键盘行为、saved session 分组、组名展示或重命名时 |
| `src/app/actions/sftp.rs` | UI 侧 SFTP 与本地文件浏览 action | `ensure_sftp_handle_for_group`，`release_sftp_handle_for_group`，`load_more_sftp_entries`，`sweep_idle_sftp_connections` | 改按需建连、分页加载操作、pin-aware group worker 回收、深睡断连、远端目录点击或传输入口时 |
| `src/app/actions/terminal.rs` | terminal 键盘、鼠标、滚动和 IME action | `on_terminal_key_down`，`terminal_grid_point_and_side`，`on_terminal_scroll`，IME helpers | 鼠标命中、选择、滚动行高、快捷键、粘贴或 IME 候选框位置与终端网格不一致时 |
| `src/app/theme.rs` | 主题注册、当前主题应用和 custom theme 逻辑 | `load_embedded_themes`，`load_user_themes`，`apply_theme_preferences`，`save_custom_appearance` | app 视觉系统入口；通过 `crate::app::theme` 暴露 |
| `src/app/lifecycle/startup.rs` | 启动辅助、日志初始化和窗口打开 | `init_logging`，`runtime_log_dir`，`crash_report_dir`，`open_main_window`，window close callback | 窗口关闭先发起 backend shutdown，再保存布局；通过 `crate::app::startup` 兼容导出 |
| `assets/icons/terminal_icon_all_formats/terminal_icon_256.png` | 非 macOS runtime 窗口图标和 Linux/Debian 256px 图标资源 | PNG 资源文件 | 改 `include_bytes!`、Debian asset 或 Linux release icon 路径时确认存在性 |
| `src/app/input/app_menu.rs` | GPUI 原生应用菜单注册 | `install`，`app_menus`，`Quit` | `Quit` 会先关闭全部 backend；原 `src/app/app_menu.rs` 迁入；通过 `crate::app::app_menu` 兼容导出 |
| `src/app.rs` | 全局 UI 状态结构和 app 子模块出口 | `AxShell` fields，type re-exports | 新增/调整应用级状态字段、输入实体、scroll handle、runtime/event channel 或跨模块共享类型时 |
| `src/app/lifecycle/init.rs` | `AxShell` 初始化和默认状态装配 | `AxShell::new`，`backend_event_channel` | 使用 `src/events.rs` 构造 256 条 Tokio backend event queue；新增输入框、默认配置读取、初始 theme/font/system 状态、订阅或 event pump 启动时 |
| `src/app/lifecycle/event_loop.rs` | 输入事件、后台事件分发、系统采样和主题同步 | `on_input_event`，`start_event_pump`，`drain_backend_events`，`sample_system_if_due` | 原 `src/app/event_loop.rs` 迁入；单次 drain 上限与 queue 容量一致，改 backend event 处理、SFTP event 更新、connection progress、system monitor sampling 或 follow-system theme 同步时 |
| `src/app/constants.rs` | app 尺寸、快捷键 context、仓库 URL 和版本展示 | layout constants，`TERMINAL_KEY_CONTEXT`，`public_version_label` | 改 UI 固定尺寸、入口链接或公开版本文案时 |
| `src/app/pane.rs` | pane tree 数据模型 | `PaneLayout` | 改 split tree、tab 查找、替换、删除或 pane 统计时 |
| `src/app/workspace.rs` | 工作区页面、tab/group 模型、连接进度、远程采样和布局持久化 | `TabGroup`，`WorkspacePage`，`workspace_tabs`，`set_workspace_page` | 改 terminal/SFTP/settings tab、当前 tab 可见性、页面关闭、连接重试或监控采样时 |
| `src/app/sftp.rs` | app 层 SFTP 页面、本地浏览、排序和右键菜单状态 | `SftpUiState`，`LocalFileBrowserState`，`SftpSortColumn`，`SftpContextMenuState` | 改 SFTP UI 状态模型而非协议 worker 时 |
| `src/app/terminal.rs` | app 层 terminal 字体/滚动条/hover 状态 | `TerminalFontMetrics`，`TerminalScrollbarHandle`，`HoveredUrl` | 改 terminal UI metrics、scrollbar adapter 或 URL/path hover 时 |
| `src/app/session_ui.rs` | session selector 与连接进度 UI 模型 | `SelectorEntry`，`ConnectionProgress` | 改 session selector 条目或连接进度遮罩状态时 |
| `src/app/search.rs` | terminal 搜索状态与行为 | `SearchState`，`perform_search`，`search_highlight_map` | 改搜索输入、全缓冲区匹配、跳转或高亮时 |
| `src/app/config_sync.rs` | app 层配置同步动作 | sync credentials，upload/download action，`SyncFinished` | 改 WebDAV/S3 表单取值、同步触发或状态接线时 |
| `src/app/dialogs/` | 弹窗和设置页渲染目录模块 | `ssh.rs`，`selector.rs`，`transfers.rs`，`delete_confirm.rs`，`sftp_close_confirm.rs`，`settings_close_confirm.rs`，`settings/` | 改 SSH 弹窗、session selector、关闭确认、transfer history、delete confirm、设置页和 About 页面时；入口为 `src/app/dialogs.rs` |
| `src/app/dialogs.rs` | dialogs 目录模块入口和共享 imports | 子模块声明，`crate::app::dialogs` 路由 | 改 dialogs 模块可见性、共享 imports 或新增 dialog 子文件时 |
| `src/app/dialogs/settings/` | 设置页子页面目录 | `appearance.rs`，`font_page.rs`，`terminal.rs`，`workspace.rs`，`monitoring.rs`，`language.rs`，`custom.rs`，`shell.rs`，`about.rs`，`help.rs`，`keybindings.rs`，`sync.rs`，`proxy.rs` | 字体枚举 helper 已并入 `font_page.rs`；入口为 `src/app/dialogs/settings.rs` |
| `src/app/dialogs/settings/appearance.rs` | Settings 外观页 | `settings_appearance_page` | 改主题模式、light/dark theme 或标题栏样式时 |
| `src/app/dialogs/settings/font_page.rs` | Settings 字体页 | `settings_fonts_page` | 改 UI/terminal 字体大小、字体族或光标样式时 |
| `src/app/dialogs/settings/terminal.rs` | Settings 终端行为页 | `settings_terminal_page` | 改右键复制粘贴、关键词高亮、SSH 重试或传输中关闭 SFTP 的默认行为时 |
| `src/app/dialogs/sftp_close_confirm.rs` | 活跃 SFTP 传输的页面关闭确认弹窗 | `show_sftp_transfer_close_dialog` | 改首次确认、group 绑定、二次快捷键、保持页面、后台继续、取消断开或记住动作时 |
| `src/app/dialogs/settings_close_confirm.rs` | Settings 页面关闭确认弹窗 | `show_settings_close_confirm_dialog` | 改确认关闭、保持页面、记住第二次快捷键动作或关闭提示文案时 |
| `src/app/dialogs/settings/workspace.rs` | Settings 工作区页 | `settings_workspace_page` | 改布局锁定、Settings 二次快捷键动作偏好或 reset layout 入口时 |
| `src/app/dialogs/settings/monitoring.rs` | Settings 监控页 | `settings_monitoring_page` | 改监控仪表盘显示或监控位置设置时 |
| `src/app/dialogs/settings/language.rs` | Settings 语言页 | `settings_language_page` | 改显示语言选择和 locale 保存行为时 |
| `src/app/dialogs/settings/custom.rs` | Custom theme 设置页 | `settings_custom_page` | 改 custom theme name/base theme/override 输入和保存/重置入口时 |
| `src/app/dialogs/settings/shell.rs` | 设置页外层交互壳 | `settings_page_shell` | 改设置页内 keybinding 录制、关闭确认、Prev/NextTab 捕获或失焦取消录制时 |
| `src/app/views/` | 主工作区视图目录模块，按渲染区域拆分 SFTP、监控、侧栏、顶部标签、终端 pane 和整体布局 | `helpers.rs`，`layout.rs`，`monitoring.rs`，`sftp_panel.rs`，`sidebar.rs`，`tab_bar.rs`，`terminal_panel.rs` | 增加固定 Local Terminal 入口、调整 SFTP 双列面板、saved session 分组、侧栏折叠态、顶部标签、监控面板或主布局时 |
| `src/app/views.rs` | views 目录模块入口和共享 imports | 子模块声明，`crate::app::views` 路由 | 改 views 模块可见性、共享 imports 或新增 views 子文件时 |
| `src/app/views/layout.rs` | `Render for AxShell` 和顶层菜单/workspace/body 布局 | `render`，platform menu row，workspace page route，resizable panels，overlays | 改 Windows/Linux 全宽菜单、主布局、集成标题栏、workspace/body split、SFTP 页面接线或全局 overlays 时 |
| `src/app/views/sftp_panel.rs` | SFTP 双列页面主体 | `render_sftp_panel`，远端加载更多页脚 | 改远端/本地文件列表、分页加载按钮、表头、上传下载按钮、隐藏文件开关、SFTP 右键菜单或双栏布局时 |
| `src/app/views/sftp_panel/sort.rs` | SFTP 远端/本地列表排序 helper | `sort_sftp_entries`，`SftpSortableEntry` | 改名称/大小/修改时间排序规则、目录优先或本地/远端排序一致性时 |
| `src/app/views/sftp_panel/transfer_panel.rs` | SFTP 页面传输标签和传输行渲染 | `render_sftp_transfer_panel`，`render_sftp_transfer_row`，`sftp_transfer_status_text` | 改传输列表分组、进度条、暂停/恢复/取消/移除按钮或传输状态文案时 |
| `src/app/views/sidebar.rs` | 展开/收起侧栏和 saved session entry 渲染 | `sidebar`，`render_collapsed_sidebar`，saved local terminal entries | 改 SAVED 列表、分组展开/重命名、折叠态入口或本地终端固定入口时 |
| `src/app/views/monitoring.rs` | 底部/侧栏监控面板 | `render_monitoring_panel`，`render_sidebar_monitoring_panel` | 改 CPU/MEM/NET/DISK 展示、sparkline、监控位置或滚动条时 |
| `src/app/views/tab_bar.rs` | 顶部 tab bar 和 split/search 操作按钮 | `render_tab_bar`，`tabs_scroll_handle` | 改编号 terminal/SFTP 标签、当前 tab 自动可见、SFTP 标签关闭、tab 选择/关闭、settings tab、split pane 按钮或 tab bar 搜索按钮时 |
| `src/app/views/terminal_panel.rs` | 终端工作区、SFTP 页面、settings 页面承载和 pane tree 渲染 | `render_terminal_panel`，`render_pane_tree`，terminal scrollbar gutter | 改终端 focus/key/mouse、右侧滚动槽、pane splitter、disconnect overlay、SFTP 页面挂载或 settings 页面承载时 |
| `src/app/views/helpers.rs` | views 内部小 helper | `bind_titlebar_drag`，`collapsed_sidebar_abbrev`，`render_home_page` | 改集成标题栏拖动、折叠侧栏简称或空首页时 |
| `src/backend/auth.rs` | SSH / SFTP 共用私钥解析和 public key 算法 fallback helper | `load_session_private_key`，`private_keys_with_algs` | 改 inline key、key path、passphrase 或 RSA SHA512/SHA256/none fallback 顺序时 |
| `src/backend/local.rs` | 本地 PTY 后端 | `LocalBackendShutdown`，`spawn_local_terminal` | 改本地 shell、PTY resize、child kill、reader/writer reaper 或本地 backend event 输出时 |
| `src/backend/ssh.rs` | SSH 终端运行循环、PTY/shell 生命周期和 channel handler 接线 | `SshBackendShutdown`，`spawn_ssh_terminal`，`run_ssh`，`cancel_ssh_child_tasks`，`ClientHandler` | 改 SSH 命令循环、PTY 请求、shell 生命周期、远程采样/CWD task、关闭语义或 handler 接线时 |
| `src/backend/ssh/connection.rs` | SSH TCP/proxy 连接、认证和 default/legacy 模式选择 | `connect_and_authenticate`，`connect_with_mode_priority`，`connect_with_mode`，`key_source_label` | 改 SSH 密码/密钥认证、proxy 连接、连接模式 fallback、认证状态上报或 resolved mode 写回事件时 |
| `src/backend/ssh/legacy.rs` | SSH legacy 算法配置和协商错误摘要 | `ssh_client_config`，`negotiation_error_details`，`negotiation_error_short_reason` | 改老服务器算法兼容、`No common algorithm` 诊断或默认/legacy 模式配置时 |
| `src/backend/ssh/system_probe.rs` | SSH 远程系统采样脚本和输出解析入口 | `sample_remote_system_with_handle`，`REMOTE_SYSTEM_PROBE` | 改远程 CPU/MEM/SWAP/NET/DISK 采样命令、Linux/Darwin 兼容或采样 session 错误处理时 |
| `src/backend/ssh/x11.rs` | SSH X11 forwarding 配置解析、cookie 校验和本地 relay | `X11ForwardingState`，`handle_x11_channel` | 改 X11 DISPLAY 选择、cookie 替换、本地 Unix/TCP 连接或 X11 channel relay 时 |
| `src/sftp.rs` | SFTP 模块入口和兼容出口 | module declarations，`RemoteEntry`，`PreviewData`，`SftpHandle`，path re-export | 改 SFTP 模块树或既有 `crate::sftp::*` 路径时 |
| `src/sftp/auth.rs` | SFTP 连接认证 | `connect_and_authenticate`，`SftpClientHandler` | 改 SFTP SSH 认证主流程或 server key 策略时；private key 解析改 `src/backend/auth.rs` |
| `src/sftp/model.rs` | SFTP 公共数据与持久化传输模型 | `RemoteEntry`，`PreviewData`，`Transfer`，`TransferState` | 改目录条目、预览载荷、传输序列化或旧 `Cancelled` 兼容时 |
| `src/sftp/path.rs` | SFTP 远程路径和格式化 helper | `join_remote`，`parent_dir`，`format_mtime`，`shell_quote` | 改远程路径拼接、父目录解析、mtime 展示、shell quote 或文件大小格式化时 |
| `src/sftp/session.rs` | SFTP channel/session 构造和 timeout 常量 | `open_sftp_session`，`open_browse_sftp_session`，`open_transfer_sftp_session` | 改普通、raw browse 或 transfer SFTP channel 建立时 |
| `src/sftp/browse.rs` | 目录 cursor、分页预算、reveal 和目录事件 | `BrowseCursor`，`DirectoryPage`，`open_and_emit_browser_page`，`read_next_browser_page` | 改分页、EOF、目录上限、cursor 关闭或 reveal path 时 |
| `src/sftp/preview.rs` | 文件与受限目录预览 | `preview_impl`，`directory_preview_body` | 改二进制判断、预览字节预算或目录截断提示时 |
| `src/sftp/transfer.rs` | 传输 flag、进度事件和上传/下载实现 | `TransferStateFlag`，`download_path_impl`，`upload_paths_impl`，`upload_file_impl` | 改暂停/恢复/取消、传输进度、并发上传或目录下载时 |
| `src/sftp/archive.rs` | 远端临时归档和本地解压 | `create_remote_archive`，`remove_remote_path`，`extract_archive_to` | 改目录打包下载、远端清理或 zip/tar 解压时 |
| `src/sftp/operations.rs` | SFTP 通用远程操作 | `recursive_delete` | 改递归删除行为时 |
| `src/sftp/worker.rs` | SFTP handle、command、work pin 和 shutdown controller | `SftpHandle`，`SftpCommand`，`SftpWorkPin`，`spawn_sftp` | 改 command API、pin 计数、worker 创建或有界关闭时 |
| `src/sftp/worker/runtime.rs` | SFTP 单一命令循环和 child task 所有权 | `run_sftp`，`cancel_sftp_child_tasks` | 改命令调度、cursor/transfer state、remote edit watcher 或 child task 回收时 |
| `src/terminal/element.rs` | terminal 前景色、高亮、字体 metrics 测量、等宽字体保护、光标对比度与网格渲染 | `TerminalElement`，`terminal_font_is_monospace`，`terminal_monospace_font_family`，`layout_grid`，`cell_run_style`，`cursor_layout` | 终端文本、背景块、光标颜色/形状、PTY resize、比例字体 fallback 或字体间距问题 |
| `src/terminal.rs` | terminal 模块入口和公开出口 | module declarations，backend/tab/input exports | 改 terminal 模块树或既有 `crate::terminal::*` 路径时 |
| `src/terminal/backend.rs` | terminal backend command、sender 和 shutdown controller | `BackendCommand`，`BackendTx`，`BackendShutdown` | 改本地/SSH terminal 命令协议或关闭控制时；全应用事件在 `src/events.rs` |
| `src/terminal/tab.rs` | terminal tab、render snapshot、scroll 和 selection | `TerminalTab`，`RenderSnapshot`，`ViewportSelection`，`SftpUiState` | 改 terminal model、feed/resize、snapshot、选择或 tab 内 SFTP UI state 时 |
| `src/terminal/listener.rs` | alacritty terminal listener 和 dimensions | `TerminalListener`，`TerminalSize`，`new_term` | 改 PTY write/title event 或 terminal 初始化尺寸时 |
| `src/terminal/key_encoding.rs` | 跨平台终端按键编码 | `encode_key`，`TerminalModifiers` | 改 Ctrl/Alt/Command、cursor mode 或 readline 导航编码时 |
| `src/terminal/cwd.rs` | OSC shell working directory 解析 | `extract_shell_working_directory`，`parse_working_directory_osc` | 改 OSC 7/633/1337、URI 或 percent decode 时 |
| `src/main.rs` | 应用启动初始化顺序 | `main()` | 新增用户 theme 文件初始加载和 watch 入口 |
| `Cargo.toml` | Cargo 包、依赖、Debian metadata | `[package]`，`[package.metadata.deb]` | 改 crate/package name、二进制名、deb assets 或依赖时 |
| `Cargo.lock` | 根包与依赖锁文件 | `[[package]] name = "ax_shell"` | 若发布时临时同步 root package version，需要确认 lock 中 root package 条目一起更新 |
| `.github/workflows/release.yml` | 多平台 release 构建与 GitHub Release 发布 | `build`，`publish`，macOS bundle heredoc | 改 release artifact、bundle display name、binary copy path 或 cask 注释模板时 |
| `scripts/package-macos-app.sh` | 本地 macOS bundle 打包脚本 | `APP_NAME`，`DISPLAY_NAME`，`BUNDLE_ID`，`Info.plist` | 改 macOS 本地打包输出和显示名时 |
| `scripts/release_version.py` | 共享发布版本规则脚本 | tag 解析、Cargo semver/public version/bundle version 派生、manifest/lock 同步 | 改 tag 作为唯一版本源的全链路规则时 |
| `examples/dev_reload.rs` | restart-based 开发重载 | `build_app`，`prepare_macos_app_bundle`，env constants | 改 dev 二进制名、bundle id、开发 app 显示名或日志文件名时 |

## 常用定位

- `rg -n 'init_logging|runtime_log_dir|crash_report_dir|panic|crash|open_main_window|current_window_title' src/main.rs src/app/lifecycle/startup.rs src/app/dialogs`
- `rg -n 'AxAshell|ax_ashell|AX_ASHELL|AxShell|ax_shell|AX_SHELL' Cargo.toml Cargo.lock src examples scripts .github assets README.md README.en.md docs`
- `rg -n 'GITHUB_REF_NAME|refs/tags|CFBundleShortVersionString|CARGO_PKG_VERSION|version = ' .github/workflows scripts src Cargo.toml Cargo.lock`
- `rg -n 'render_sftp_panel|render_monitoring_panel|sidebar\\(|render_collapsed_sidebar|render_tab_bar|render_terminal_panel|render_pane_tree|WorkspacePage' src/app/views src/app`
- `rg -n 'sidebar\\(|render_collapsed_sidebar|saved_session_groups|open_local' src/app/views src/app/actions`
- `rg -n 'split_current_pane|focus_adjacent_pane|activate_group|activate_group_page|sync_system_tab_to_active_group' src/app/actions`
- `rg -n 'workspace_tabs|active_workspace_tab_index|tabs_scroll_handle|scroll_to_item' src/app`
- `rg -n 'SftpHandle|BrowseCursor|spawn_sftp|run_sftp|open_and_emit_browser_page|read_next_browser_page|JoinSet|connect_and_authenticate|join_remote|parent_dir' src/sftp`
- `rg -n 'BackendShutdown|shutdown_all_backends|SshBackendShutdown|LocalBackendShutdown|cancel_ssh_child_tasks' src`
- `rg -n 'load_session_private_key|private_keys_with_algs|key_source_label' src/backend src/sftp`
- `rg -n 'custom_theme|ThemeRegistry|load_embedded_themes|apply_theme_preferences|save_custom' src/app src/session src/main.rs`
- `rg -n 'terminal_font_metrics|terminal_font_is_monospace|terminal_cell_width|terminal_line_height|layout_grid' src/app src/config src/session src/terminal`
- `rg -n 'observe_window_activation|WindowLifecycleState|deep_sleep_after_minutes|sample_system_if_due|sync_theme_if_due' src/app src/config`
- `rg -n 'ThemeConfig|ThemeSet|try_parse_color|watch_dir|default_light_theme|default_dark_theme' ~/.cargo/git/checkouts/gpui-component-*`
- `cargo check`

## 忽略与未索引

- `assets/icons/` 批量图标、`assets/fonts/`、`target/` 未逐项索引：仅记录当前构建/打包会直接引用的关键图标入口，字体资源和构建产物不逐项展开

## 刷新规则

- 刷新触发：项目命名、Cargo 包/二进制名、配置目录、同步默认文件名、启动初始化、日志/crash hook、非 macOS runtime 图标资源、release workflow、tag/version 映射规则、manifest/lock 临时同步、macOS/Linux 打包元数据、仓库级 agent 指令、Rust 模块布局约束、SAVED 侧栏入口、custom theme 持久化模型、theme file 注册策略、设置页字段分组、Settings 子页面新增/删除/改名、Settings 关闭确认偏好/dialog、theme list 行为、terminal 亮度语义、终端字体 metrics、窗口激活/后台/深睡状态、workspace page / tab 模型、terminal tab UI 状态回收、terminal backend shutdown controller、SFTP 按需页面/标签关闭/快捷键焦点、SFTP worker/task 关闭所有权、SFTP 分页或受限目录浏览/预览、SFTP 列表排序/传输标签面板、SFTP 目录导航失败恢复、SSH 连接认证/legacy/远程系统探针/X11 relay、settings Custom/shell 拆分、app/backend 根目录收拢、app/actions/state/config/session/sftp/backend/ui/dialogs 模块拆分或用户文档范围发生变化时刷新
- 最近依据：`AGENTS.md`，`Cargo.toml`，`src/app.rs`，`src/app/`，`src/backend.rs`，`src/backend/`，`src/config.rs`，`src/config/`，`src/session.rs`，`src/session/`，`src/sftp.rs`，`src/sftp/`，`src/terminal.rs`，`src/terminal/`，`docs/project-env-audit/current.md`

## 最后更新时间

- 2026-07-10 22:40 +0800
