# 项目地图

## 项目概览

- 用途：基于 Rust 和 GPUI 的 SSH / 本地终端桌面客户端
- 主要入口：`src/main.rs`，`src/app/startup.rs`，`src/app/mod.rs`，`src/app/init.rs`，`src/app/ui.rs`，`src/session/mod.rs`

## 索引范围

- 根目录：`<repo-root>`
- 覆盖：`src/app/`，`src/session/`，`src/sftp/`，`src/terminal/`，`src/sync/`，`locales/`，`docs/`，`Cargo.toml`，`Cargo.lock`，`.github/workflows/`，`scripts/`，`assets/*.desktop`
- 排除：`.git/`，`target/`，`assets/` 批量图标/字体资源，构建产物与外部依赖缓存

## 目录地图

| Path | Purpose | Open When | Notes |
| --- | --- | --- | --- |
| `src/session/` | 配置持久化、会话模型、本地终端动作、saved session 分组和 pane 状态管理 | 改 config root、会话连接、saved session 分组/重命名、pane split/focus、tab/group 生命周期或本地配置兼容逻辑时 | `mod.rs` 保留会话连接和 tab 生命周期；`pane.rs` 管 pane tree；`saved_sessions.rs` 管 selector 与 saved group |
| `src/app/` | 启动、日志、应用状态、事件泵、侧栏、设置页、弹窗、主题、字体选择、原生菜单和工作区 UI | 调整应用显示名、启动日志、crash hook、AxShell 状态字段、SAVED 侧栏入口、Custom 页面、theme list、字体下拉、主题应用逻辑、原生菜单或工作区动作时 | `mod.rs` 保留状态结构；`app_menu.rs` 管 GPUI 原生菜单注册；`init.rs` 负责 `AxShell::new`；`event_loop.rs` 负责后台事件泵；`ui/` 和 `dialogs/settings/mod.rs` 仍是主要渲染热点 |
| `src/backend/` | 本地/SSH 后端连接、认证 helper、远程系统采样和 PTY/SSH 事件桥接 | 改 SSH 连接、private key 解析、legacy 算法 fallback、本地 shell 或后台事件输出时 | `auth.rs` 管 SSH/SFTP 共用 key 解析；`ssh.rs` 管终端 SSH 主流程；`local.rs` 管本地终端 |
| `src/sftp/` | SFTP 命令循环、认证、远程路径 helper、文件传输、预览和本地文件操作 | 改 SFTP 连接认证、远程路径拼接、上传/下载、远程删除、预览、编辑远程文件或本地文件浏览时 | `mod.rs` 保留命令循环、传输实现和事件发送 helper；`auth.rs` 管 SFTP SSH 认证；`path.rs` 管路径/格式化 helper；`ops.rs` 管 UI 侧 SFTP/local file 操作 |
| `src/terminal/` | 终端渲染、颜色、字体 metrics 和交互 | custom theme brightness、终端颜色语义、字体间距、PTY resize 或鼠标命中需要联动时 | 本轮改 `element.rs` 与 `input.rs`，让字体实测 metrics 统一驱动渲染和输入命中 |
| `src/sync/` | 会话配置加密同步 payload | 判断新增会话字段是否会自动进入同步上传/下载时 | 本轮预计不改传输逻辑，只依赖 `Session` 序列化扩展 |
| `src/main.rs` | 应用初始化入口 | 增加全局初始化、custom theme watch/load、补入口初始化顺序时 | 本轮在 `main()` 第一行注册 panic hook，保证早期启动 panic 可落 crash 文件 |
| `locales/` | 中英文界面文案 | 新增 custom theme 分组、提示、保存说明、日志入口和错误消息时 | 需要同步 `en.yml` 和 `zh-CN.yml` |
| `.github/workflows/` | CI / Release 构建和打包元数据 | 改二进制名、artifact 名、macOS bundle Info.plist 或发布路径时 | `release.yml` 手工组装 `.app`，需要与 Cargo 包名一致 |
| `scripts/` | 本地开发/打包脚本与发布辅助脚本 | 改 macOS `.app` 名称、bundle id、图标文件名、签名逻辑、tag/version 映射或发布前 manifest 同步时 | `package-macos-app.sh` 会运行 `cargo build --release` 并组装 bundle；本轮将新增共享版本脚本 |
| `assets/*.desktop` | Linux desktop entry | 改应用显示名、Exec、Icon、StartupWMClass 或 Debian metadata 时 | 当前 desktop 文件为 `assets/ax_shell.desktop` |
| `docs/` | README、用户/开发文档、环境审计和实施跟踪 | 改项目名称、配置目录、同步文件名、打包命令或验证边界时 | 本轮需同步 README、user-guide、development、env audit 与 project tracker |

## 关键文件

| Path | Role | Key Symbols / Sections | Read For |
| --- | --- | --- | --- |
| `src/session/config.rs` | 本地配置文件模型、路径和 getter/setter | `ConfigFile`，`ConfigStore::load/save`，`config_root_dir_path`，config path helpers | 改配置目录、旧目录迁移、sync 默认对象名、custom theme draft 和 registry file 路径 |
| `src/app/theme.rs` | 主题注册、当前主题应用和 custom theme 逻辑 | `load_embedded_themes`，`load_user_themes`，`apply_theme_preferences`，`save_custom_appearance` | 本轮已改成“真实 ThemeConfig + theme file 持久化 + registry 即时应用/监听” |
| `src/app/startup.rs` | 启动辅助、日志初始化和窗口打开 | `init_logging`，`runtime_log_dir`，`crash_report_dir`，`open_main_window`，platform launch helpers | 增加运行日志、crash 日志、panic hook、窗口打开错误记录、日志目录入口或启动期诊断时 |
| `src/app/app_menu.rs` | GPUI 原生应用菜单注册 | `install`，`app_menus`，`Quit` | 增加/调整系统菜单栏、菜单动作、OS copy/paste 语义或应用级 Quit 行为时 |
| `src/app/mod.rs` | 全局 UI 状态结构和 app 子模块出口 | `AxShell` fields，type re-exports | 新增/调整应用级状态字段、输入实体、scroll handle、runtime/event channel 或跨模块共享类型时 |
| `src/app/init.rs` | `AxShell` 初始化和默认状态装配 | `AxShell::new` | 新增输入框、默认配置读取、初始 theme/font/system 状态、订阅或 event pump 启动时 |
| `src/app/event_loop.rs` | 输入事件、后台事件分发、系统采样和主题同步 | `on_input_event`，`start_event_pump`，`drain_backend_events`，`sample_system_if_due` | 改 backend event 处理、SFTP event 更新、connection progress、system monitor sampling 或 follow-system theme 同步时 |
| `src/app/types.rs` | app/session/UI 共享类型 | `PaneLayout`，`TabGroup`，`TerminalScrollbarHandle`，`WorkspacePage` | 改 pane tree 类型、tab group、terminal scrollbar 或工作区页面枚举时 |
| `src/app/workspace.rs` | 工作区页面、连接进度、远程采样请求、workspace tab 切换和布局持久化辅助 | `set_workspace_page`，`switch_workspace_tab`，`request_active_system_snapshot`，`retry_connection_progress`，`save_layout_state` | 改设置页 / SFTP 页面生命周期、workspace tab 顺序、监控可见性采样、连接重试或窗口布局保存时 |
| `src/app/dialogs/` | 弹窗和设置页渲染目录模块 | `mod.rs`，`ssh.rs`，`selector.rs`，`transfers.rs`，`delete_confirm.rs`，`settings/` | 改 SSH 弹窗、session selector、transfer history、delete confirm、设置页和 About 页面时 |
| `src/app/dialogs/mod.rs` | dialogs 目录模块入口和共享 imports | 子模块声明，`crate::app::dialogs` 路由 | 改 dialogs 模块可见性、共享 imports 或新增 dialog 子文件时 |
| `src/app/dialogs/settings/` | 设置页目录模块 | `mod.rs`，`fonts.rs`，`about.rs`，`help.rs`，`keybindings.rs`，`sync.rs`，`proxy.rs` | 改设置页字体列表、About 页日志目录入口、Help 页、Keybindings 页、Sync 页或 Proxy/X11 页时；General/Custom 仍主要在 `mod.rs` |
| `src/app/ui/` | 主 UI 目录模块，按渲染区域拆分 SFTP、监控、侧栏、顶部标签、终端 pane 和整体布局 | `mod.rs`，`helpers.rs`，`layout.rs`，`monitoring.rs`，`sftp_panel.rs`，`sidebar.rs`，`tab_bar.rs`，`terminal_panel.rs` | 增加固定 Local Terminal 入口、调整 SFTP 双列面板、saved session 分组、侧栏折叠态、顶部标签、监控面板或主布局时 |
| `src/app/ui/mod.rs` | UI 目录模块入口和共享 imports | 子模块声明，`crate::app::ui` 路由 | 改 UI 模块可见性、共享 imports 或新增 UI 子文件时 |
| `src/app/ui/layout.rs` | `Render for AxShell` 和顶层 workspace/body 布局 | `render`，workspace page route，resizable workspace/body panels，dialog/sheet/context menu overlays，connection progress overlay | 改主布局、集成标题栏、workspace/body split、SFTP 独立页面接线或全局 overlays 时 |
| `src/app/ui/sftp_panel.rs` | SFTP 双列页面主体和传输摘要 | `render_sftp_panel` | 改远端/本地文件列表、上传下载按钮、隐藏文件开关、SFTP 右键菜单或 SFTP 页面 footer 时 |
| `src/app/ui/sidebar.rs` | 展开/收起侧栏和 saved session entry 渲染 | `sidebar`，`render_collapsed_sidebar`，saved local terminal entries | 改 SAVED 列表、分组展开/重命名、折叠态入口或本地终端固定入口时 |
| `src/app/ui/monitoring.rs` | 底部/侧栏监控面板 | `render_monitoring_panel`，`render_sidebar_monitoring_panel` | 改 CPU/MEM/NET/DISK 展示、sparkline、监控位置或滚动条时 |
| `src/app/ui/tab_bar.rs` | 顶部 tab bar 和 split/search 操作按钮 | `render_tab_bar` | 改编号 terminal/SFTP 标签、tab 选择/关闭、settings tab、split pane 按钮或 tab bar 搜索按钮时 |
| `src/app/ui/terminal_panel.rs` | 终端工作区、SFTP 页面、settings 页面承载和 pane tree 渲染 | `render_terminal_panel`，`render_pane_tree` | 改终端 focus/key/mouse 事件绑定、pane splitter、disconnect overlay、SFTP 页面挂载或 settings 页面承载时 |
| `src/app/ui/helpers.rs` | UI 内部小 helper | `bind_titlebar_drag`，`collapsed_sidebar_abbrev`，`render_home_page` | 改集成标题栏拖动、折叠侧栏简称或空首页时 |
| `src/session/mod.rs` | 会话连接、SSH 表单、tab 生命周期和 active session 查询 | `open_local`，`connect_ssh`，`open_ssh_session`，`handle_tab_close`，`active_snapshot` | 改本地/SSH tab 创建、SSH 表单加载/重置、断线重试、关闭 tab/group 或 active session 查询时 |
| `src/session/pane.rs` | pane tree 操作和 group activation | `split_current_pane`，`focus_adjacent_pane`，`activate_group`，`activate_group_page`，`sync_system_tab_to_active_group` | 改 split pane、pane focus、splitter drag、active group + 页面联动切换或监控 tab 跟随 group 时 |
| `src/session/saved_sessions.rs` | session selector 和 saved group 管理 | `selector_entries`，`on_selector_key_down`，`saved_session_groups`，`commit_saved_group_rename` | 改选择器键盘行为、saved session 分组、组名展示或重命名时 |
| `src/backend/auth.rs` | SSH / SFTP 共用私钥解析和 public key 算法 fallback helper | `load_session_private_key`，`private_keys_with_algs` | 改 inline key、key path、passphrase 或 RSA SHA512/SHA256/none fallback 顺序时 |
| `src/backend/ssh.rs` | SSH 终端连接、legacy 算法兼容、远程系统采样和 X11 转发 | `spawn_ssh_terminal`，`connect_and_authenticate`，`legacy_client_config`，`key_source_label` | 改 SSH 终端认证、算法协商、状态上报、远程系统探针或 X11 转发时 |
| `src/sftp/mod.rs` | SFTP 命令循环、上传/下载/预览/删除实现 | `spawn_sftp`，`run_sftp`，`download_path_impl`，`upload_paths_impl`，`recursive_delete` | 改 SFTP runtime 命令、传输进度、远程编辑、递归删除、archive 下载或预览实现时 |
| `src/sftp/auth.rs` | SFTP 连接认证 | `connect_and_authenticate`，`SftpClientHandler` | 改 SFTP SSH 认证主流程或 server key 策略时；private key 解析改 `src/backend/auth.rs` |
| `src/sftp/path.rs` | SFTP 远程路径和格式化 helper | `join_remote`，`parent_dir`，`format_mtime`，`shell_quote` | 改远程路径拼接、父目录解析、mtime 展示、shell quote 或文件大小格式化时 |
| `src/terminal/element.rs` | terminal 前景色、高亮、字体 metrics 测量、等宽字体保护、光标对比度与网格渲染 | `TerminalElement`，`terminal_font_is_monospace`，`terminal_monospace_font_family`，`layout_grid`，`cell_run_style`，`cursor_layout` | 终端文本、背景块、光标颜色/形状、PTY resize、比例字体 fallback 或字体间距问题 |
| `src/terminal/input.rs` | terminal 键盘、鼠标、滚动和 IME 输入 | `terminal_grid_point_and_side`，`on_terminal_scroll` | 鼠标命中、选择、滚动行高或 IME 候选框位置与终端网格不一致时 |
| `src/main.rs` | 应用启动初始化顺序 | `main()` | 新增用户 theme 文件初始加载和 watch 入口 |
| `Cargo.toml` | Cargo 包、依赖、Debian metadata | `[package]`，`[package.metadata.deb]` | 改 crate/package name、二进制名、deb assets 或依赖时 |
| `Cargo.lock` | 根包与依赖锁文件 | `[[package]] name = "ax_shell"` | 若发布时临时同步 root package version，需要确认 lock 中 root package 条目一起更新 |
| `.github/workflows/release.yml` | 多平台 release 构建与 GitHub Release 发布 | `build`，`publish`，macOS bundle heredoc | 改 release artifact、bundle display name、binary copy path 或 cask 注释模板时 |
| `scripts/package-macos-app.sh` | 本地 macOS bundle 打包脚本 | `APP_NAME`，`DISPLAY_NAME`，`BUNDLE_ID`，`Info.plist` | 改 macOS 本地打包输出和显示名时 |
| `scripts/release_version.py` | 共享发布版本规则脚本 | tag 解析、Cargo semver/public version/bundle version 派生、manifest/lock 同步 | 改 tag 作为唯一版本源的全链路规则时 |
| `examples/dev_reload.rs` | restart-based 开发重载 | `build_app`，`prepare_macos_app_bundle`，env constants | 改 dev 二进制名、bundle id、开发 app 显示名或日志文件名时 |

## 常用定位

- `rg -n 'init_logging|runtime_log_dir|crash_report_dir|panic|crash|open_main_window|current_window_title' src/main.rs src/app/startup.rs src/app/dialogs`
- `rg -n 'AxAshell|ax_ashell|AX_ASHELL|AxShell|ax_shell|AX_SHELL' Cargo.toml Cargo.lock src examples scripts .github assets README.md README.en.md docs`
- `rg -n 'GITHUB_REF_NAME|refs/tags|CFBundleShortVersionString|CARGO_PKG_VERSION|version = ' .github/workflows scripts src Cargo.toml Cargo.lock`
- `rg -n 'render_sftp_panel|render_monitoring_panel|sidebar\\(|render_collapsed_sidebar|render_tab_bar|render_terminal_panel|render_pane_tree|WorkspacePage' src/app/ui src/app`
- `rg -n 'sidebar\\(|render_collapsed_sidebar|saved_session_groups|open_local' src/app/ui src/session/mod.rs`
- `rg -n 'split_current_pane|focus_adjacent_pane|activate_group|activate_group_page|sync_system_tab_to_active_group' src/session`
- `rg -n 'spawn_sftp|run_sftp|connect_and_authenticate|join_remote|parent_dir' src/sftp`
- `rg -n 'load_session_private_key|private_keys_with_algs|key_source_label' src/backend src/sftp`
- `rg -n 'custom_theme|ThemeRegistry|load_embedded_themes|apply_theme_preferences|save_custom' src/app src/session src/main.rs`
- `rg -n 'terminal_font_metrics|terminal_font_is_monospace|terminal_cell_width|terminal_line_height|layout_grid' src/app src/session src/terminal`
- `rg -n 'ThemeConfig|ThemeSet|try_parse_color|watch_dir|default_light_theme|default_dark_theme' ~/.cargo/git/checkouts/gpui-component-*`
- `cargo check`

## 忽略与未索引

- `assets/icons/`、`assets/fonts/`、`target/` 未索引：本轮不涉及批量图标/字体资源或构建产物

## 刷新规则

- 刷新触发：项目命名、Cargo 包/二进制名、配置目录、同步默认文件名、启动初始化、日志/crash hook、release workflow、tag/version 映射规则、manifest/lock 临时同步、macOS/Linux 打包元数据、SAVED 侧栏入口、custom theme 持久化模型、theme file 注册策略、设置页字段分组、theme list 行为、terminal 亮度语义、终端字体 metrics、workspace page / tab 模型、app/session/sftp/backend/ui/dialogs 模块拆分或用户文档范围发生变化时刷新
- 最近依据：`Cargo.toml`，`src/app/types.rs`，`src/app/workspace.rs`，`src/session/pane.rs`，`src/app/ui/tab_bar.rs`，`src/app/ui/layout.rs`，`src/app/ui/sftp_panel.rs`，`docs/project-env-audit/current.md`

## 最后更新时间

- 2026-07-08 17:33 +0800
