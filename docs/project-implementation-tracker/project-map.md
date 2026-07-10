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
| `src/app/` | 应用壳实现目录，启动/事件/输入/主题/同步/终端搜索/工作区分别落到功能子目录 | 调整应用显示名、启动日志、crash hook、SAVED 侧栏入口、Custom 页面、theme list、字体下拉、主题应用逻辑、原生菜单或工作区动作时 | 真实实现分布在 `lifecycle/`、`input/`、`theme/`、`syncing/`、`terminal/`、`workspace/`、`state/`、`actions/`、`views/`、`dialogs/` |
| `src/app/actions.rs` | 应用动作层入口 | 改 actions 模块导出时 | 子模块在 `src/app/actions/`；由原 `session/mod.rs`、`session/pane.rs`、`session/saved_sessions.rs`、`sftp/ops.rs`、`terminal/input.rs` 迁入 |
| `src/app/actions/` | 应用动作层实现，集中承载直接操作 `AxShell` 的会话、pane、SFTP UI、本地文件浏览和终端输入动作 | 改 `open_local`、`connect_ssh`、pane split/focus、saved session 分组/重命名、SFTP UI 操作、terminal key/mouse/IME/scroll 行为时 | 入口为 `src/app/actions.rs` |
| `src/app/theme.rs` | app 视觉系统、主题注册、custom theme 和字体加载 | 改内置主题资源、用户主题加载、custom theme 保存/应用、Maple Mono 内置字体加载或 ThemeRegistry 接线时 | 直接暴露为 `crate::app::theme`，资源 include 路径按当前文件使用 `../../assets/...` |
| `src/app/core/` | app 常量和共享类型 | 改尺寸常量、快捷键 context、仓库 URL、版本展示、Pane/Tab/SFTP UI 类型或 workspace tab 描述时 | 通过 `#[path]` 继续暴露为 `crate::app::constants` 与内部 `types` |
| `src/app/input/` | 原生菜单和快捷键录制/绑定 | 改 App menu、Quit shutdown、workspace keybindings、设置页按键录制、冲突检测或 keybinding 展示时 | 通过 `#[path]` 继续暴露为 `crate::app::app_menu` 与 `crate::app::keybinding_recorder` |
| `src/app/lifecycle/` | 启动、窗口打开、日志/crash hook、`AxShell::new` 和事件泵 | 改启动顺序、日志目录、crash report、主窗口 options、非 macOS 窗口图标、runtime event pump、输入事件分发或初始化默认状态时 | 通过 `#[path]` 继续暴露为 `crate::app::startup`，内部 `init` / `event_loop` 仍挂到 `AxShell` impl |
| `src/app/syncing/` | app 层同步动作 | 改同步表单取值、WebDAV/S3 上传下载触发、sync status 或 SyncFinished 事件接线时 | 通过 `#[path]` 继续暴露为 `crate::app::config_sync` |
| `src/app/terminal/` | app 层 terminal 周边功能 | 改终端搜索栏、搜索匹配跳转、搜索 highlight map 或搜索输入焦点时 | 通过 `#[path]` 继续暴露为 `crate::app::search`，不要和根级 `src/terminal/` 终端模型混淆 |
| `src/app/workspace/` | app 工作区页面与 tab 生命周期 | 改 workspace page route、terminal/SFTP/settings tab 切换、当前 tab 自动可见、SFTP 页面打开/关闭、连接进度重试或布局持久化时 | 通过 `#[path]` 继续挂为内部 `workspace` 模块；根目录无 `workspace.rs` 文件 |
| `src/app/state/` | `AxShell` 子状态聚合 | 改 appearance、search、monitoring、runtime/event channel 这类高内聚状态字段时 | `appearance.rs`、`search.rs`、`monitoring.rs`、`runtime.rs` 分组降低 `AxShell` 字段噪声 |
| `src/config/` | 配置文件模型、配置路径、`ConfigStore` 和 proxy/X11 配置 helper | 改配置持久化、旧目录迁移、sync 默认对象名、proxy 默认值、custom theme draft 或本地 X server 配置时 | `store.rs` 是真实实现；旧 `src/session/config.rs` 只做兼容 re-export |
| `src/session/` | 会话和外观配置模型，以及旧 `session::config` 路径兼容层 | 改 `Session`、`AuthMethod`、`SshConnectionMode`、窗口 bounds、标题栏/光标/custom theme 模型时 | `model.rs` 承载可序列化模型；`config.rs` re-export `config::store` 与 `session::model`，避免旧调用一次性全改 |
| `src/backend.rs` | backend 领域入口 | 改 backend 模块导出时 | 现代 Rust 具名入口；子模块为 `src/backend/auth.rs`、`src/backend/local.rs`、`src/backend/ssh.rs` 和 `src/backend/ssh/` |
| `src/backend/` | 本地/SSH 后端连接、认证 helper、远程系统采样和 PTY/SSH 事件桥接 | 改 SSH 连接、private key 解析、legacy 算法 fallback、本地 shell、后台事件输出或 backend shutdown 时 | `auth.rs` 管 SSH/SFTP 共用 key 解析；`local.rs` 管 PTY child/thread reaper；`ssh.rs` 管 SSH 主 task、query `JoinSet` 和有界 shutdown；`ssh/` 还包含连接认证、legacy 算法、远程系统探针和 X11 relay |
| `src/sftp.rs` | SFTP 命令循环、文件传输、预览、远程删除和 archive 下载入口 | 改 SFTP worker、上传/下载、远程删除、预览、编辑远程文件或 archive 解包时 | 现代 Rust 具名入口；子模块 `src/sftp/auth.rs` 管 SFTP SSH 认证，`src/sftp/path.rs` 管路径/格式化 helper；UI 侧操作已迁到 `src/app/actions/sftp.rs` |
| `src/sftp/` | SFTP 子模块目录 | 改 SFTP 认证或远程路径 helper 时 | 入口为 `src/sftp.rs` |
| `src/terminal/` | 终端模型、渲染、颜色和字体 metrics | custom theme brightness、终端颜色语义、字体间距、PTY resize、render snapshot 或 terminal model helper 时 | 输入/鼠标/IME/scroll action 已迁到 `src/app/actions/terminal.rs`；`element.rs` 仍是渲染热点 |
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
| `src/config/store.rs` | 本地配置文件模型、路径、getter/setter、proxy/X11 helper 和 `ConfigStore` 实现 | `ConfigFile`，`ConfigStore::load/save`，`config_root_dir_path`，config path helpers，`connect_proxy`，`active_proxy` | 改配置目录、旧目录迁移、sync 默认对象名、custom theme draft、registry file 路径、proxy 或 X11 配置时 |
| `src/session/config.rs` | 旧配置导入路径兼容层 | `pub use crate::config::store::*`，`pub use crate::session::model::*` | 需要保持旧 `crate::session::config::*` 调用兼容时；不要在这里新增真实配置逻辑 |
| `src/session/model.rs` | 会话、SSH 连接模式、窗口 bounds、标题栏、光标和 custom theme 序列化模型 | `Session`，`AuthMethod`，`SshConnectionMode`，`ordered_ssh_connection_modes`，`SavedWindowBounds`，`TitleBarStyle`，`CursorStyle`，`CustomThemeConfig` | 改配置文件里的模型字段、serde 默认值或同步 payload 模型时 |
| `src/app/state/` | `AxShell` 子状态模块 | `AppearanceState`，`SearchState`，`MonitoringState`，`RuntimeState` | 改应用外观、搜索、系统监控或 runtime/event channel 状态分组时 |
| `src/app/actions/session.rs` | 会话连接、SSH 表单、tab 生命周期和 active session 查询 action | `open_local`，`connect_ssh`，`open_ssh_session`，`shutdown_all_backends`，`handle_tab_close`，`active_snapshot` | 改本地/SSH tab 创建、SSH 表单加载/重置、当前 workspace tab 可见性、断线重试、关闭 tab/group、窗口退出或 active session 查询时 |
| `src/app/actions/pane.rs` | pane tree 操作和 group activation action | `split_current_pane`，`focus_adjacent_pane`，`activate_group_page`，`focus_pane_with_id`，`sync_system_tab_to_active_group` | 改 split pane、pane focus、splitter drag、active group + 页面联动切换、当前 workspace tab 自动可见或监控 tab 跟随 group 时 |
| `src/app/actions/saved_sessions.rs` | session selector 和 saved group action | `selector_entries`，`on_selector_key_down`，`saved_session_groups`，`commit_saved_group_rename` | 改选择器键盘行为、saved session 分组、组名展示或重命名时 |
| `src/app/actions/sftp.rs` | UI 侧 SFTP 与本地文件浏览 action | `ensure_sftp_handle_for_group`，`release_sftp_handle_for_group`，`sweep_idle_sftp_connections` | 改按需建连、pin-aware group worker 回收、深睡断连、远端目录点击或传输入口时 |
| `src/app/actions/terminal.rs` | terminal 键盘、鼠标、滚动和 IME action | `on_terminal_key_down`，`terminal_grid_point_and_side`，`on_terminal_scroll`，IME helpers | 鼠标命中、选择、滚动行高、快捷键、粘贴或 IME 候选框位置与终端网格不一致时 |
| `src/app/theme.rs` | 主题注册、当前主题应用和 custom theme 逻辑 | `load_embedded_themes`，`load_user_themes`，`apply_theme_preferences`，`save_custom_appearance` | app 视觉系统入口；通过 `crate::app::theme` 暴露 |
| `src/app/lifecycle/startup.rs` | 启动辅助、日志初始化和窗口打开 | `init_logging`，`runtime_log_dir`，`crash_report_dir`，`open_main_window`，window close callback | 窗口关闭先发起 backend shutdown，再保存布局；通过 `crate::app::startup` 兼容导出 |
| `assets/icons/terminal_icon_all_formats/terminal_icon_256.png` | 非 macOS runtime 窗口图标和 Linux/Debian 256px 图标资源 | PNG 资源文件 | 改 `include_bytes!`、Debian asset 或 Linux release icon 路径时确认存在性 |
| `src/app/input/app_menu.rs` | GPUI 原生应用菜单注册 | `install`，`app_menus`，`Quit` | `Quit` 会先关闭全部 backend；原 `src/app/app_menu.rs` 迁入；通过 `crate::app::app_menu` 兼容导出 |
| `src/app.rs` | 全局 UI 状态结构和 app 子模块出口 | `AxShell` fields，type re-exports | 新增/调整应用级状态字段、输入实体、scroll handle、runtime/event channel 或跨模块共享类型时 |
| `src/app/lifecycle/init.rs` | `AxShell` 初始化和默认状态装配 | `AxShell::new` | 原 `src/app/lifecycle/init.rs` 迁入；新增输入框、默认配置读取、初始 theme/font/system 状态、订阅或 event pump 启动时 |
| `src/app/lifecycle/event_loop.rs` | 输入事件、后台事件分发、系统采样和主题同步 | `on_input_event`，`start_event_pump`，`drain_backend_events`，`sample_system_if_due` | 原 `src/app/event_loop.rs` 迁入；改 backend event 处理、SFTP event 更新、connection progress、system monitor sampling 或 follow-system theme 同步时 |
| `src/app/core/types.rs` | app/session/UI 共享类型 | `PaneLayout`，`TabGroup`，`SftpSortColumn`，`SftpTransferTab`，`TerminalScrollbarHandle`，`WorkspacePage` | 原 `src/app/types.rs` 迁入；改 pane tree 类型、tab group、SFTP UI 状态、terminal scrollbar 或工作区页面枚举时 |
| `src/app/workspace/workspace.rs` | 工作区页面、连接进度、远程采样请求、workspace tab 切换和布局持久化辅助 | `workspace_tabs`，`active_workspace_tab_index`，`ensure_active_workspace_tab_visible`，`set_workspace_page`，`request_active_system_snapshot` | 改 workspace tab 渲染/索引映射、当前 tab 自动可见、SFTP 页面关闭确认、二次快捷键、连接重试或监控可见性采样时 |
| `src/app/dialogs/` | 弹窗和设置页渲染目录模块 | `ssh.rs`，`selector.rs`，`transfers.rs`，`delete_confirm.rs`，`sftp_close_confirm.rs`，`settings/` | 改 SSH 弹窗、session selector、传输关闭确认、transfer history、delete confirm、设置页和 About 页面时；入口为 `src/app/dialogs.rs` |
| `src/app/dialogs.rs` | dialogs 目录模块入口和共享 imports | 子模块声明，`crate::app::dialogs` 路由 | 改 dialogs 模块可见性、共享 imports 或新增 dialog 子文件时 |
| `src/app/dialogs/settings/` | 设置页子页面目录 | `appearance.rs`，`font_page.rs`，`terminal.rs`，`workspace.rs`，`monitoring.rs`，`language.rs`，`custom.rs`，`shell.rs`，`fonts.rs`，`about.rs`，`help.rs`，`keybindings.rs`，`sync.rs`，`proxy.rs` | 改 Settings 页面分组、Custom 页、字体列表、About 页日志目录入口、Help 页、Keybindings 页、Sync 页或 Proxy/X11 页时；入口为 `src/app/dialogs/settings.rs` |
| `src/app/dialogs/settings/appearance.rs` | Settings 外观页 | `settings_appearance_page` | 改主题模式、light/dark theme 或标题栏样式时 |
| `src/app/dialogs/settings/font_page.rs` | Settings 字体页 | `settings_fonts_page` | 改 UI/terminal 字体大小、字体族或光标样式时 |
| `src/app/dialogs/settings/terminal.rs` | Settings 终端行为页 | `settings_terminal_page` | 改右键复制粘贴、关键词高亮、SSH 重试或传输中关闭 SFTP 的默认行为时 |
| `src/app/dialogs/sftp_close_confirm.rs` | 活跃 SFTP 传输的页面关闭确认弹窗 | `show_sftp_transfer_close_dialog` | 改首次确认、group 绑定、二次快捷键、保持页面、后台继续、取消断开或记住动作时 |
| `src/app/dialogs/settings/workspace.rs` | Settings 工作区页 | `settings_workspace_page` | 改布局锁定或 reset layout 入口时 |
| `src/app/dialogs/settings/monitoring.rs` | Settings 监控页 | `settings_monitoring_page` | 改监控仪表盘显示或监控位置设置时 |
| `src/app/dialogs/settings/language.rs` | Settings 语言页 | `settings_language_page` | 改显示语言选择和 locale 保存行为时 |
| `src/app/dialogs/settings/custom.rs` | Custom theme 设置页 | `settings_custom_page` | 改 custom theme name/base theme/override 输入和保存/重置入口时 |
| `src/app/dialogs/settings/shell.rs` | 设置页外层交互壳 | `settings_page_shell` | 改设置页内 keybinding 录制、Prev/NextTab 捕获或失焦取消录制时 |
| `src/app/views/` | 主工作区视图目录模块，按渲染区域拆分 SFTP、监控、侧栏、顶部标签、终端 pane 和整体布局 | `helpers.rs`，`layout.rs`，`monitoring.rs`，`sftp_panel.rs`，`sidebar.rs`，`tab_bar.rs`，`terminal_panel.rs` | 增加固定 Local Terminal 入口、调整 SFTP 双列面板、saved session 分组、侧栏折叠态、顶部标签、监控面板或主布局时 |
| `src/app/views.rs` | views 目录模块入口和共享 imports | 子模块声明，`crate::app::views` 路由 | 改 views 模块可见性、共享 imports 或新增 views 子文件时 |
| `src/app/views/layout.rs` | `Render for AxShell` 和顶层菜单/workspace/body 布局 | `render`，platform menu row，workspace page route，resizable panels，overlays | 改 Windows/Linux 全宽菜单、主布局、集成标题栏、workspace/body split、SFTP 页面接线或全局 overlays 时 |
| `src/app/views/sftp_panel.rs` | SFTP 双列页面主体 | `render_sftp_panel` | 改远端/本地文件列表、表头、上传下载按钮、隐藏文件开关、SFTP 右键菜单或双栏布局时 |
| `src/app/views/sftp_panel/sort.rs` | SFTP 远端/本地列表排序 helper | `sort_sftp_entries`，`SftpSortableEntry` | 改名称/大小/修改时间排序规则、目录优先或本地/远端排序一致性时 |
| `src/app/views/sftp_panel/transfer_panel.rs` | SFTP 页面传输标签和传输行渲染 | `render_sftp_transfer_panel`，`render_sftp_transfer_row`，`sftp_transfer_status_text` | 改传输列表分组、进度条、暂停/恢复/取消/移除按钮或传输状态文案时 |
| `src/app/views/sidebar.rs` | 展开/收起侧栏和 saved session entry 渲染 | `sidebar`，`render_collapsed_sidebar`，saved local terminal entries | 改 SAVED 列表、分组展开/重命名、折叠态入口或本地终端固定入口时 |
| `src/app/views/monitoring.rs` | 底部/侧栏监控面板 | `render_monitoring_panel`，`render_sidebar_monitoring_panel` | 改 CPU/MEM/NET/DISK 展示、sparkline、监控位置或滚动条时 |
| `src/app/views/tab_bar.rs` | 顶部 tab bar 和 split/search 操作按钮 | `render_tab_bar`，`tabs_scroll_handle` | 改编号 terminal/SFTP 标签、当前 tab 自动可见、SFTP 标签关闭、tab 选择/关闭、settings tab、split pane 按钮或 tab bar 搜索按钮时 |
| `src/app/views/terminal_panel.rs` | 终端工作区、SFTP 页面、settings 页面承载和 pane tree 渲染 | `render_terminal_panel`，`render_pane_tree`，terminal scrollbar gutter | 改终端 focus/key/mouse、右侧滚动槽、pane splitter、disconnect overlay、SFTP 页面挂载或 settings 页面承载时 |
| `src/app/views/helpers.rs` | views 内部小 helper | `bind_titlebar_drag`，`collapsed_sidebar_abbrev`，`render_home_page` | 改集成标题栏拖动、折叠侧栏简称或空首页时 |
| `src/session.rs` | session 领域模块入口 | `pub mod config`，`pub mod model` | 改 session 领域模块导出时；真实 app action 已迁到 `src/app/actions/` |
| `src/backend/auth.rs` | SSH / SFTP 共用私钥解析和 public key 算法 fallback helper | `load_session_private_key`，`private_keys_with_algs` | 改 inline key、key path、passphrase 或 RSA SHA512/SHA256/none fallback 顺序时 |
| `src/backend/local.rs` | 本地 PTY 后端 | `LocalBackendShutdown`，`spawn_local_terminal` | 改本地 shell、PTY resize、child kill、reader/writer reaper 或本地 backend event 输出时 |
| `src/backend/ssh.rs` | SSH 终端运行循环、PTY/shell 生命周期和 channel handler 接线 | `SshBackendShutdown`，`spawn_ssh_terminal`，`run_ssh`，`cancel_ssh_child_tasks`，`ClientHandler` | 改 SSH 命令循环、PTY 请求、shell 生命周期、远程采样/CWD task、关闭语义或 handler 接线时 |
| `src/backend/ssh/connection.rs` | SSH TCP/proxy 连接、认证和 default/legacy 模式选择 | `connect_and_authenticate`，`connect_with_mode_priority`，`connect_with_mode`，`key_source_label` | 改 SSH 密码/密钥认证、proxy 连接、连接模式 fallback、认证状态上报或 resolved mode 写回事件时 |
| `src/backend/ssh/legacy.rs` | SSH legacy 算法配置和协商错误摘要 | `ssh_client_config`，`negotiation_error_details`，`negotiation_error_short_reason` | 改老服务器算法兼容、`No common algorithm` 诊断或默认/legacy 模式配置时 |
| `src/backend/ssh/system_probe.rs` | SSH 远程系统采样脚本和输出解析入口 | `sample_remote_system_with_handle`，`REMOTE_SYSTEM_PROBE` | 改远程 CPU/MEM/SWAP/NET/DISK 采样命令、Linux/Darwin 兼容或采样 session 错误处理时 |
| `src/backend/ssh/x11.rs` | SSH X11 forwarding 配置解析、cookie 校验和本地 relay | `X11ForwardingState`，`handle_x11_channel` | 改 X11 DISPLAY 选择、cookie 替换、本地 Unix/TCP 连接或 X11 channel relay 时 |
| `src/sftp.rs` | SFTP 命令循环、worker、工作 pin 和子 task 生命周期 | `SftpHandle`，`SftpWorkPin`，`spawn_sftp`，`run_sftp`，`download_path_impl`，`upload_paths_impl` | 改 pin-aware command 入口、关闭/取消、传输进度、远程编辑、递归删除、archive 下载或预览时 |
| `src/sftp/auth.rs` | SFTP 连接认证 | `connect_and_authenticate`，`SftpClientHandler` | 改 SFTP SSH 认证主流程或 server key 策略时；private key 解析改 `src/backend/auth.rs` |
| `src/sftp/path.rs` | SFTP 远程路径和格式化 helper | `join_remote`，`parent_dir`，`format_mtime`，`shell_quote` | 改远程路径拼接、父目录解析、mtime 展示、shell quote 或文件大小格式化时 |
| `src/terminal/element.rs` | terminal 前景色、高亮、字体 metrics 测量、等宽字体保护、光标对比度与网格渲染 | `TerminalElement`，`terminal_font_is_monospace`，`terminal_monospace_font_family`，`layout_grid`，`cell_run_style`，`cursor_layout` | 终端文本、背景块、光标颜色/形状、PTY resize、比例字体 fallback 或字体间距问题 |
| `src/terminal.rs` | terminal 核心模型、事件和 render snapshot | `BackendShutdown`，`BackendTx`，`TerminalTab`，`BackendEvent`，`BackendCommand`，`Transfer`，`SftpUiState`，`TerminalMouseTrackingMode` | 改 terminal 数据模型、backend event、backend replacement/shutdown、transfer state、SFTP UI state 或给 action 层增加安全访问 API 时 |
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
- `rg -n 'SftpHandle|spawn_sftp|run_sftp|JoinSet|connect_and_authenticate|join_remote|parent_dir' src/sftp`
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

- 刷新触发：项目命名、Cargo 包/二进制名、配置目录、同步默认文件名、启动初始化、日志/crash hook、非 macOS runtime 图标资源、release workflow、tag/version 映射规则、manifest/lock 临时同步、macOS/Linux 打包元数据、仓库级 agent 指令、Rust 模块布局约束、SAVED 侧栏入口、custom theme 持久化模型、theme file 注册策略、设置页字段分组、Settings 子页面新增/删除/改名、theme list 行为、terminal 亮度语义、终端字体 metrics、窗口激活/后台/深睡状态、workspace page / tab 模型、terminal backend shutdown controller、SFTP 按需页面/标签关闭/快捷键焦点、SFTP worker/task 关闭所有权、SFTP 列表排序/传输标签面板、SFTP 目录导航失败恢复、SSH 连接认证/legacy/远程系统探针/X11 relay、settings Custom/shell 拆分、app/backend 根目录收拢、app/actions/state/config/session/sftp/backend/ui/dialogs 模块拆分或用户文档范围发生变化时刷新
- 最近依据：`AGENTS.md`，`Cargo.toml`，`src/app.rs`，`src/app/theme.rs`，`src/app/core/constants.rs`，`src/app/core/types.rs`，`src/app/input/app_menu.rs`，`src/app/input/keybinding_recorder.rs`，`src/app/lifecycle/startup.rs`，`src/app/lifecycle/init.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/syncing/config_sync.rs`，`src/app/terminal/search.rs`，`src/app/workspace/workspace.rs`，`src/app/state.rs`，`src/app/actions.rs`，`src/config.rs`，`src/config/store.rs`，`src/session.rs`，`src/session/model.rs`，`src/session/config.rs`，`src/app/dialogs.rs`，`src/app/dialogs/settings.rs`，`src/app/dialogs/settings/appearance.rs`，`src/app/dialogs/settings/font_page.rs`，`src/app/dialogs/settings/terminal.rs`，`src/app/dialogs/settings/workspace.rs`，`src/app/dialogs/settings/monitoring.rs`，`src/app/dialogs/settings/language.rs`，`src/app/dialogs/settings/custom.rs`，`src/app/dialogs/settings/shell.rs`，`src/app/views.rs`，`src/app/views/tab_bar.rs`，`src/app/views/layout.rs`，`src/app/views/terminal_panel.rs`，`src/app/views/sftp_panel.rs`，`src/app/views/sftp_panel/sort.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`src/backend.rs`，`src/backend/auth.rs`，`src/backend/local.rs`，`src/backend/ssh.rs`，`src/backend/ssh/connection.rs`，`src/backend/ssh/legacy.rs`，`src/backend/ssh/system_probe.rs`，`src/backend/ssh/x11.rs`，`src/sftp.rs`，`src/terminal.rs`，`docs/project-env-audit/current.md`

## 最后更新时间

- 2026-07-10 14:37 +0800
