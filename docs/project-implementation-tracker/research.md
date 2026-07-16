# 外部检索记录

## 2026-07-15 SFTP 初始目录与终端目录关联

- 时间：2026-07-15 16:10 +0800
- 检索问题：SFTP 文件浏览器应如何在固定起始路径、上次目录、服务端 home 和 SSH 终端当前目录之间选择，避免打开时出现短暂跳转。
- 检索原因：AxShell 当前在进入 SFTP 页面时会同时加载 home 和终端 CWD；需要以主流产品的明确配置和动作边界决定修复策略。
- 来源列表：WinSCP Directories <https://winscp.net/eng/docs/ui_login_directories>；Cyberduck SFTP <https://docs.cyberduck.io/protocols/sftp/>；VS Code Remote SSH <https://code.visualstudio.com/docs/remote/ssh>。
- 关键结论：WinSCP 支持每站点的初始远端目录以及“记住上次目录”，未指定时通常使用服务端 home；Cyberduck 将当前文件浏览目录传入终端作为显式“Open in Terminal”动作；VS Code Remote SSH 让用户显式打开远端工作目录。三者均不支持普通打开时先展示一个目录、再后台跳到另一个目录的模式。
- 对实施计划的影响：AxShell 普通 SFTP 打开改为一次性确定初始路径，优先 `Session.sftp_path`，其次保存会话的最后远端目录，最后服务端 home。移除终端 CWD 的自动同步，在远端路径栏提供显式跳转按钮。
- 未解决问题：远端初始目录被删除或无权限时的自动 home 回退不在本轮范围，保持当前的 SFTP 错误提示语义。

## 2026-07-15 高刷新率与自适应帧率调度

- 时间：2026-07-15 11:58 +0800
- 检索问题：AxShell 是否应自行读取 120Hz / VRR 显示器刷新率，如何在不增加空闲资源消耗的前提下让活动终端刷新适配实际帧率。
- 检索原因：实现依赖锁定 GPUI 对平台帧回调、VRR 和热压力的实际语义，不能按通用 GUI 框架假设设计。
- 来源列表：Zed / GPUI `window.rs` <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs>；Zed `gpui_wgpu/wgpu_renderer.rs` <https://github.com/zed-industries/zed/blob/main/crates/gpui_wgpu/src/wgpu_renderer.rs>；Zed macOS window backend <https://github.com/zed-industries/zed/blob/main/crates/gpui_macos/src/window.rs>；Zed Linux X11 client <https://github.com/zed-industries/zed/blob/main/crates/gpui_linux/src/linux/x11/client.rs>；WGPU `PresentMode` <https://wgpu.rs/doc/wgpu/enum.PresentMode.html>。
- 关键结论：GPUI 上游仅在窗口 dirty 时重建场景，并在高频输入时选择性保持 presentation 以支持 VRR；非活跃窗口约限 30Hz，严重/临界热压力约限 60Hz。WGPU 默认 FIFO VSync；macOS 帧源跟随窗口当前屏幕，Linux X11 读取当前 mode 的刷新周期。GPUI 没有适合 AxShell 直接跨平台读取并持久化显示器 Hz 的公开 API。
- 对实施计划的影响：保留 GPUI / WGPU 的平台帧源和保护策略。AxShell 只在新前台活动 burst 中请求三个 animation frame 采样实际间隔；采样有效后将终端/UI 合帧间隔限制在 8.333ms 至 33ms。无效、陈旧或无活动时回退当前 16ms，不创建常驻 animation loop。
- 未解决问题：锁定依赖是 Git 提交而不是版本化稳定 API；跨显示器移动、不同 VRR 策略和 Windows 组合器行为需要三平台实机帧率、CPU、GPU 与功耗采样确认。

## 2026-07-14 WinSCP 多文件传输队列模型

- 时间：2026-07-14 19:42 +0800
- 检索问题：主流 SFTP 客户端在多选或目录递归下载时，传输列表应按批量任务还是单文件呈现，如何同时保留总进度和完整文件明细。
- 检索原因：用户要求检索其他软件的处理方式，并确认以 WinSCP 为参考实现 AxShell 的下载记录体验。
- 来源列表：WinSCP 官方 Transfer Queue / Background Operations <https://winscp.net/eng/docs/transfer_queue>；WinSCP 官方 Background Operations Queue List <https://winscp.net/eng/docs/ui_queue>；FileZilla 官方 usage guide <https://wiki.filezilla-project.org/Using#Transferring_files>。
- 关键结论：WinSCP 明确规定顶层队列条目是后台操作而非单文件；运行中的多文件任务首行显示批量总体进度，后续行显示当前传输文件；用户可打开任务的完整文件清单，已传输、跳过和当前文件均保留状态。FileZilla 也支持将单文件、目录和多选文件加入队列，但公开文档未明确其队列的行粒度。WinSCP 的两层模型同时避免目录任务淹没主队列，并满足文件级追踪。
- 对实施计划的影响：AxShell 保持现有 `Transfer` 为任务级暂停、恢复、取消和历史单位；下载 worker 为每个实际文件上报明细，SFTP 面板显示当前文件/已发现文件数，并以单独文件清单对话框展示所有文件。目录不预扫描，明细随递归下载动态累积。
- 未解决问题：递归目录的总文件数在完成扫描前不可知；本轮不预计算总数，也不为上传任务建立文件级清单。

## 2026-07-13 GPUI 持续输出的终端网格布局缓存

- 时间：2026-07-13 13:18 +0800
- 检索问题：在 AxShell 已通过 `TermDamage` 和 125ms 关键词限频降低 snapshot/highlight 成本后，如何安全继续降低持续输出时的 GPUI `paint` 路径成本。
- 检索原因：用户确认继续优化并要求检索；新 sample 显示 `Window::draw_roots -> paint` 已成为主路径。
- 来源列表：锁定 GPUI revision `f9c994796ad4341649d7b8664edbdfaae8bebd5d` 的 `Window::use_keyed_state` / `with_element_state` <https://github.com/zed-industries/zed/blob/f9c994796ad4341649d7b8664edbdfaae8bebd5d/crates/gpui/src/window.rs>；同 revision 的 Zed `terminal_element.rs` <https://github.com/zed-industries/zed/blob/f9c994796ad4341649d7b8664edbdfaae8bebd5d/crates/terminal_view/src/terminal_element.rs>；xterm.js `RenderDebouncer` <https://github.com/xtermjs/xterm.js/blob/master/src/browser/RenderDebouncer.ts>；WezTerm terminal render cache <https://github.com/wezterm/wezterm/blob/main/wezterm-gui/src/termwindow/render/mod.rs>。
- 关键结论：GPUI 可通过稳定 element key 保存连续帧状态，但未提供适用于本任务的公开 display-list 缓存接口。Zed 当前 terminal element 仍在 `prepaint` 中生成 grid layout；xterm.js 合并最小行范围更新；WezTerm 以行 shape hash 缓存 shaping。当前风险最低的下一步是缓存 AxShell 自己的 `LayoutRect` / `BatchedTextRun` / custom block / underline 准备结果，以 snapshot 身份、字体/主题、搜索高亮、选择和 hover 状态为失效键；光标与 IME 保持每帧绘制。
- 对实施计划的影响：P2 实现稳定 element state 的布局缓存；不修改 renderer、Metal、glyph atlas 或 PTY flow control。由于底部滚动通常使所有可视行内容变化，预期先减少无内容变化的重复 `prepaint`，再根据新 sample 决定是否需要增量行级 layout cache。
- 未解决问题：缓存后的文本 `shape_line` / glyph paint 是否仍占主要 CPU，需要以相同工作负载重新 sample；如果是，后续才评估可缓存的 shaped line 或 renderer 级实例化。

## 2026-07-13 Terminal dirty-region 渲染路径

- 时间：2026-07-13 10:56 +0800
- 检索问题：AxShell 持续 PTY 输出时，是否有比应用层整行内容比较更准确且低风险的 snapshot / keyword highlight 增量更新方案。
- 检索原因：用户明确要求联网检索优化方案，并确认实施首选方案。
- 来源列表：`alacritty_terminal 0.26.0` 的 `TermDamage` <https://docs.rs/alacritty_terminal/0.26.0/alacritty_terminal/term/enum.TermDamage.html>、`LineDamageBounds` <https://docs.rs/alacritty_terminal/0.26.0/alacritty_terminal/term/struct.LineDamageBounds.html>、`Term` API <https://docs.rs/alacritty_terminal/0.26.0/alacritty_terminal/term/struct.Term.html>；xterm.js `RenderDebouncer` <https://github.com/xtermjs/xterm.js/blob/master/src/browser/RenderDebouncer.ts> 与 `RenderService` <https://github.com/xtermjs/xterm.js/blob/master/src/browser/services/RenderService.ts>；WezTerm render cache <https://github.com/wezterm/wezterm/blob/main/wezterm-gui/src/termwindow/render/mod.rs>；Alacritty instanced renderer <https://github.com/Alacritty/alacritty/blob/master/alacritty/src/renderer/text/glsl3.rs>；xterm.js flow control <https://xtermjs.org/docs/guides/flowcontrol/>。
- 关键结论：当前已锁定的 `alacritty_terminal 0.26.0` 提供 `TermDamage::Full/Partial` 和每行 `LineDamageBounds { line, left, right }`；读取 damage 后必须调用 `reset_damage()`。xterm.js 将多次更新合并到下一帧的最小行范围；WezTerm 分层缓存行 shaping 和 quad；Alacritty 的 glyph atlas / instanced rendering 是更高成本的 renderer 重写路径。逐 chunk 暂停/恢复 PTY 会增加上下文切换，不适合当前目标。
- 对实施计划的影响：本轮以 `TermDamage` 为唯一内容 dirty 来源，在 `feed` / resize / scroll 后累计可视损伤；snapshot 用共享行块替换受损行，keyword/IP/port 重算受损行及 URL 自动换行相邻行。选择、配置、cursor 等 UI 状态另行失效，不将其混入 terminal damage。
- 未解决问题：GPUI 文本 layout 仍可能是后续热点；只有新 sample 显示 snapshot/highlight 已非主要成本时，才评估行级 GPUI layout cache 或 glyph atlas 重写。

## 2026-07-12 主流主题 palette 来源

- 时间：2026-07-12 09:54 +0800
- 检索问题：AxShell 新增哪些内置主题预设更符合主流终端/编辑器用户预期，并且有可追溯 palette 来源。
- 检索原因：用户明确允许联网检索；新增主题不应只凭印象命名或拍脑袋配色。
- 来源列表：Catppuccin palette JSON <https://raw.githubusercontent.com/catppuccin/palette/main/palette.json>；Dracula README palette <https://github.com/dracula/dracula-theme#color-palette-oss>；Nord colors and palettes <https://www.nordtheme.com/docs/colors-and-palettes>；Rosé Pine palette JSON <https://raw.githubusercontent.com/rose-pine/palette/main/palette.json>。
- 关键结论：Catppuccin 提供 Latte/Mocha 等官方 light/dark palette；Dracula 提供 Dracula dark 和 Alucard light palette；Nord 官方文档明确 16 色 palette 及 terminal-friendly 命名；Rosé Pine 提供 Dawn/Main/Moon palette。它们覆盖暖色、冷色、柔和 pastel 和高对比暗色，适合作为内置预设补充。
- 对实施计划的影响：新增 `assets/themes/popular.json`，包含 Catppuccin Latte/Mocha、Dracula Alucard/Dracula、Nord Light/Nord、Rosé Pine Dawn/Main/Moon；默认 profile 增加纯主题和少量跨主题组合。
- 未解决问题：真实 GUI 中的 Settings 表单、SFTP 列表、终端正文和 hover 对比度仍需手工确认；部分 palette 需要按 AxShell ThemeConfig token 做工程映射，不能逐字段等同于原项目所有 UI token。

## 2026-07-11 GPUI hover 响应路径

- 时间：2026-07-11 20:28 +0800
- 检索问题：GPUI 中 `.hover()`、`on_hover`、`on_mouse_move` 和虚拟列表/菜单 hover 的适用边界是什么，是否应为高频列表抽出状态驱动的快速 hover helper
- 检索原因：用户明确要求联网检索；实现路径需要区分普通稳定元素的样式 hover 和虚拟列表行的显式鼠标跟随状态
- 来源列表：Zed GPUI source `window.rs` <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/window.rs>；longbridge gpui-component `ListItem` source <https://github.com/longbridge/gpui-component/blob/main/crates/ui/src/list/list_item.rs>；longbridge gpui-component `MenuItemElement` source <https://github.com/longbridge/gpui-component/blob/main/crates/ui/src/menu/menu_item.rs>
- 关键结论：普通稳定元素继续使用 `.hover()` 更简单；虚拟列表行如果需要“跟手”的背景反馈，应使用稳定元素 id，并通过 `on_mouse_move` / `on_hover` 更新应用状态，再由渲染背景读取该状态，避免和行级 `.hover()` 两套机制互相覆盖。
- 对实施计划的影响：新增 `src/app/hover.rs` 的 `FastHoverState`；SFTP 远端/本地文件行移除旧 `.hover()`，改为状态驱动 hover；表头、侧边栏、selector、splitter、搜索按钮和普通菜单保留 `.hover()`。
- 未解决问题：真实 GUI 鼠标快速扫过时的手感仍需手工确认；若后续出现新的虚拟列表，应复用 `FastHoverState` 而不是直接复制 SFTP 逻辑。

## 2026-07-12 MacXServer display 兼容性

- 时间：2026-07-12 13:07 +0800
- 检索问题：macOS MacXServer 的本地 X server 是否使用标准 X11 display / port，能否被当前 SSH X11 relay 作为本地目标
- 检索原因：用户询问 AxShell 当前 X11 能否同时支持 XQuartz 和 MacXServer，并给出本机安装路径 `/Applications/MacXServer.app`；display/port 行为会影响实现计划
- 来源列表：MacXServer README <https://github.com/toddvernon/MacXServer/blob/main/README.md>；MacXServer product plan <https://github.com/toddvernon/MacXServer/blob/main/PRODUCT_2_SERVER.md>
- 关键结论：MacXServer 是 macOS 上的 X11 server；其 quick start 说明 server 运行后 port 6000 映射到 display `:0`，X client 可使用 `<mac-ip>:0`；产品计划也记录 server listens on `:6000`。
- 对实施计划的影响：AxShell 在配置路径为 `MacXServer.app` 时应固定本地 relay display 为 `127.0.0.1:0`，让 `local_x11_endpoints()` 直接走 TCP 6000，而不是优先采用可能来自 XQuartz 的 launchd `DISPLAY` socket。
- 未解决问题：MacXServer 的 Xauthority / no-auth 行为和远端具体 X11 程序兼容性仍需真实 GUI + SSH X11 forwarding 联机验证。

## 2026-07-10 终端系统文本导航快捷键

- 时间：2026-07-10 07:54 +0800
- 检索问题：终端输入是否应按平台习惯支持 `Ctrl+←/→`、macOS `Command+←/→` 和 `Option+←/→`，这些按键应该编码成什么
- 检索原因：用户明确要求检索；实现路径依赖 macOS 文本导航习惯、Readline 控制序列和 xterm modified cursor 序列的兼容边界
- 来源列表：Apple Support `Keyboard shortcuts in Terminal on Mac` <https://support.apple.com/guide/terminal/keyboard-shortcuts-trmlshtcts/mac>；Apple Support `Text tool keyboard shortcuts in Motion on Mac` <https://support.apple.com/guide/motion/text-tool-keyboard-shortcuts-motn192e4990/mac>；GNU Bash Manual `Commands For Moving` <https://www.gnu.org/software/bash/manual/html_node/Commands-For-Moving.html>；XTerm Control Sequences <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
- 关键结论：macOS 文本输入习惯中 `Command+←/→` 对应移动到行首/行尾，`Option+←/→` 对应按词移动；Readline 常见序列为 `C-a` / `C-e` 和 `M-b` / `M-f`；Windows/Linux 终端中的 `Ctrl+←/→` 通常走 xterm modified cursor，例如 `CSI 1;5D` / `CSI 1;5C`
- 对实施计划的影响：在 `src/terminal.rs` 增加平台文本导航别名；macOS 只对 `Command+Arrow` 和 `Option+Arrow` 特判，不全局启用 `option_as_meta`，避免影响 Option 输入字符；现有非 macOS `Ctrl+Arrow` modified cursor 逻辑保留
- 未解决问题：真实 shell 可能自定义 keybind；GUI 层实际键盘事件仍需要在真实平台手工确认

## 2026-07-09 VS Code 终端工作目录捕获方法

- 时间：2026-07-09 13:57 +0800
- 检索问题：VS Code terminal shell integration 如何捕获 shell 当前工作目录
- 检索原因：用户明确要求参考 VS Code 的捕获方法；该实现决策影响是否向交互 shell 注入可见命令
- 来源列表：VS Code Docs `Terminal Shell Integration`
- 关键结论：VS Code 依赖 shell integration 发出的 OSC 序列传递当前工作目录；本轮采用 `OSC 633;P;Cwd=...` 作为主兼容路径，同时兼容 iTerm2 风格 `OSC 1337;CurrentDir=...` 和通用 `OSC 7;file://...`
- 对实施计划的影响：终端输出流中解析 CWD escape sequence 并缓存到 SSH tab；没有缓存时用独立 SSH exec session 执行 `pwd -P` 兜底，避免污染用户正在看的交互 shell
- 未解决问题：远端 shell 若没有启用 shell integration，不会自动输出实时 `cd` 后的 CWD；兜底查询只能提供独立 session 的目录信息，需要真实 SSH/SFTP 场景手工确认体验

## 2026-07-06 russh 依赖版本

- 时间：2026-07-07 07:57 +0800
- 检索问题：`russh`、`russh-keys`、`russh-sftp` 在 crates.io / Cargo registry 的当前版本是什么
- 检索原因：用户要求将 `russh` 升级到最新版，版本信息会随时间变化，必须查询当前 registry
- 来源列表：Cargo registry / crates.io via `cargo search russh --limit 5`；Cargo registry / crates.io via `cargo search russh-keys --limit 5`；Cargo registry / crates.io via `cargo search russh-sftp --limit 5`
- 关键结论：`russh = "0.62.2"`；`russh-keys = "0.50.0-beta.7"`；`russh-sftp = "2.3.0"`
- 对实施计划的影响：本轮目标版本定为 `russh 0.62.2`；`russh-sftp` 升级到 `2.3.0`；`russh-keys` 没有与 `russh 0.62.2` 同步的稳定线，且项目没有直接使用其 API，因此移除直接依赖并使用 `russh::keys`
- 未解决问题：未做 upstream changelog 深入分析；真实 SSH/SFTP 服务器兼容性需后续联机验证

## 2026-07-07 GitHub Release 描述生成能力

- 时间：2026-07-07 07:57 +0800
- 检索问题：GitHub Release workflow 能否同时使用自动生成 release notes 和自定义 release body
- 检索原因：用户希望发布流程自动把提交记录中的重大改动放进 Release 描述
- 来源列表：GitHub Docs `Automatically generated release notes`；`softprops/action-gh-release` README
- 关键结论：GitHub 支持自动生成 release notes；`softprops/action-gh-release` 支持 `generate_release_notes`，也支持用 `body_path` 从文件读取自定义 Release body
- 对实施计划的影响：保留 `generate_release_notes: true`，同时在 publish job 中从 git tag range 生成 `release/body.md`，再通过 `body_path: release/body.md` 注入自定义 Highlights
- 未解决问题：未在真实 tag push 后执行 GitHub Release 发布演练；最终页面拼接效果需发布时确认

## 2026-07-07 X11 forwarding cookie 替换策略

- 时间：2026-07-07 07:57 +0800
- 检索问题：SSH X11 forwarding 是否可以把远端 X11 setup 直接透明转发给本机 X server，还是必须替换 fake cookie
- 检索原因：用户询问能否不处理 cookie 直接转发；该决策影响 X11 relay 的安全边界和能否被 XQuartz 接受
- 来源列表：RFC 4254 Section 6.3.1 `x11-req`；OpenSSH portable `channels.c`
- 关键结论：`x11-req` 中的 authentication cookie 应为 fake random cookie；收到 X11 connection 后，客户端应检查 fake cookie 并替换成本机 X server 的 real cookie；把 fake cookie 原样转发给 XQuartz 通常会被拒绝，把 real cookie 直接发给远端则暴露本机 X 授权凭据
- 对实施计划的影响：`src/backend/ssh.rs` 必须实现 X11 setup packet 解析、fake cookie 校验、real cookie 替换，再进入透明双向 relay；cookie 不匹配或解析失败时关闭该 X11 channel
- 未解决问题：不同远端 sshd 对 display 编号和临时 xauth 文件的实现可能有差异，仍需真实远端联机验证

## 2026-07-07 macOS bundle version 格式约束

- 时间：2026-07-07 21:29 +0800
- 检索问题：`CFBundleShortVersionString` 和 `CFBundleVersion` 是否允许直接使用四段日期版本，例如 `2026.07.06.1`
- 检索原因：本轮要把 Git tag 做成唯一发布版本源，但同日补发 tag `vYYYY.MM.DD.N` 如果直接写入 plist，可能违反 Apple 对 bundle version 的格式要求
- 来源列表：Apple Developer Documentation `CFBundleShortVersionString`；Apple Developer Glossary `version number`；Apple Developer Glossary `build version number`
- 关键结论：`CFBundleShortVersionString` 应保持三段数字版本；`CFBundleVersion` 也必须保持纯数字、最多三段的 build version 形式，不适合直接写入四段日期 tag
- 对实施计划的影响：共享版本脚本将 `CFBundleShortVersionString` 固定为 `YYYY.MM.DD`，将 `CFBundleVersion` 改为 `YYYYMMDD` 或 `YYYYMMDD.N`，避免 tag 后缀直接进入四段 plist 版本
- 未解决问题：真实 GitHub Release 产物下载后的 Finder / 系统信息展示仍需通过一次实机安装确认

## 2026-07-09 GitHub Actions 发布 runner 覆盖

- 时间：2026-07-09 07:56 +0800
- 检索问题：当前 GitHub-hosted runners 是否支持 Linux ARM64、macOS Intel / ARM64 和 Windows ARM64 标签
- 检索原因：用户要求增加发布软件的不同系统版本，runner 标签可用性会随 GitHub Actions 平台变化，需要以官方文档为准
- 来源列表：GitHub Docs `GitHub-hosted runners reference`
- 关键结论：标准 runner 列表包含 `ubuntu-22.04-arm` / `ubuntu-24.04-arm` Linux ARM64 标签、`macos-15-intel` Intel macOS 标签、`macos-14` / `macos-15` ARM64 macOS 标签；Windows ARM64 以 `windows-11-arm` 等标签提供，但标注为 public preview
- 对实施计划的影响：本轮纳入稳定收益更高的 Linux ARM64、Linux `.deb` 和 macOS universal 产物；Windows ARM64 不并入主发布矩阵，留作后续 experimental workflow 或手动验证
- 未解决问题：Linux ARM64、`.deb` 安装体验和 macOS universal app 仍需 GitHub Actions 实际运行与下载验证

## 2026-07-10 SSH 连接重试默认值依据

- 时间：2026-07-10 10:48 +0800
- 检索问题：SSH 客户端的连接重试默认值是否存在统一主流做法，AxShell 的可配置重试默认值应如何选择
- 检索原因：用户要求把 SSH 登录网络重试做成可配置，并希望给出“主流软件的重复次数”为默认值依据；该信息可能随软件版本或文档更新变化，需要核实
- 来源列表：OpenSSH `ssh_config(5)` 文档，`ConnectionAttempts` 默认值为 1 次尝试 <https://man7.org/linux/man-pages/man5/ssh_config.5.html>
- 关键结论：OpenSSH 官方默认相对保守，`ConnectionAttempts` 为 1；不同 GUI SSH 客户端对自动重连/连接重试的默认策略并不统一，且不少产品把“断线自动重连”和“首次连接重试”分开定义；在缺少稳定统一官方对照的前提下，本轮默认值应优先保持 AxShell 当前已上线行为，即额外 2 次 transport retry，延时 0.5s / 1.5s
- 对实施计划的影响：设置页说明里明确“默认值保持当前产品行为”；配置 schema 允许用户自定义重试次数与延时；不把 OpenSSH 的 1 次尝试直接强推为新默认，以避免回退已有用户体验
- 未解决问题：未找到足够稳定且一致的多家 GUI SSH 客户端“首次连接重试”官方默认值对照；如用户后续明确指定对标某一产品，可再补充定向检索

## 2026-07-14 系统文件图标平台方案

- 时间：2026-07-14 15:45 +0800
- 检索问题：Linux 如何在不依赖 GTK 运行时的条件下，按当前 Freedesktop 图标主题查找 SFTP 文件类型图标。
- 检索原因：项目需在 macOS、Windows 和 Linux 呈现各自系统风格的文件图标；Linux 的图标主题路径和继承链没有单一跨桌面原生 API。
- 来源列表：`https://crates.io/crates/freedesktop-icons`，`cargo info freedesktop-icons`，本机下载的 `freedesktop-icons 0.4.0` crate 源码。
- 关键结论：`freedesktop-icons 0.4.0` 可按主题名、尺寸和图标名称查找 PNG/SVG 资源，并处理主题目录及继承关系；应用可用 MIME 推断扩展名对应的主题图标候选，再回退到通用文件/文件夹图标。
- 对实施计划的影响：Linux 后端采用该 crate，避免引入 GTK/GIO；macOS 保持 `NSWorkspace`，Windows 保持 Shell `SHGetFileInfoW`。所有平台输出统一的 GPUI 缓存图像。
- 未解决问题：需在 GNOME、KDE 等真实桌面环境确认主题检测和缺失资源回退的视觉效果。

## 2026-07-14 文件管理器类型图标与缓存边界

- 时间：2026-07-14 16:34 +0800
- 检索问题：主流文件管理器和 Windows Shell 如何为远端/慢速目录选择类型图标，以及系统图标查询是否适合在 UI 线程执行。
- 检索原因：用户要求联网对标，以验证 SFTP 图标是否应以路径为缓存键，以及启动预热/离线缓存策略是否合理。
- 来源列表：KDE KIO `KFileItem` <https://github.com/KDE/kio/blob/master/src/core/kfileitem.cpp>；GNOME Nautilus `nautilus-file.c` <https://github.com/GNOME/nautilus/blob/main/src/nautilus-file.c>；Microsoft `SHGetFileInfoW` <https://learn.microsoft.com/windows/win32/api/shellapi/nf-shellapi-shgetfileinfow>。
- 关键结论：KDE 对慢速 URL 以文件名扩展名推断 MIME，再使用 MIME 图标名缓存；Nautilus 将已获得的 `GIcon` 保存在文件对象上，并把自定义/缩略图作为额外分支；Windows Shell 支持虚拟扩展名加 `SHGFI_USEFILEATTRIBUTES`，无需访问真实文件，并明确建议从后台线程调用。三者均不要求为远端路径同步读取本地文件系统。
- 对实施计划的影响：AxShell 缓存固定为目录、通用文件和受控扩展名，并序列化图像数据到独立配置文件；缓存缺失、损坏、平台或 Linux 主题变更时才在启动阶段预热。SFTP 行渲染只查询内存映射。
- 未解决问题：KDE/GNOME 会处理本地自定义目录图标和缩略图；AxShell 本轮故意不支持这两类路径相关资源，需在真实三端 GUI 验收系统主题、缩放和回退视觉效果。

## 2026-07-16 xiaoxingshell 串口与 Telnet 参考实现

- 时间：2026-07-16 16:30 +0800
- 检索问题：参考 `xiaoxinghaha/xiaoxingshell`，确定 AxShell 串口与 Telnet 会话的最小后端模型、会话参数和新建表单端口检测时机。
- 检索原因：用户明确要求以该项目为参考，并要求新建串口会话自动检测当前已连接端口。
- 来源列表：xiaoxingshell repository tree <https://github.com/xiaoxinghaha/xiaoxingshell>；串口实现 <https://github.com/xiaoxinghaha/xiaoxingshell/blob/main/src/serial.rs>；Telnet 实现 <https://github.com/xiaoxinghaha/xiaoxingshell/blob/main/src/telnet.rs>；会话表单 <https://github.com/xiaoxinghaha/xiaoxingshell/blob/main/ui/session_dialog.slint>。
- 关键结论：参考项目将串口作为独立同步 I/O 路径，配置波特率和基本线路参数；Telnet 作为 TCP 字节流处理并处理 Telnet 控制字节。端口枚举适合在会话创建/编辑流程中触发，而不是持续运行于 UI 列表渲染。
- 对实施计划的影响：AxShell 使用 `serialport` worker 线程并复用既有 backend event/terminal byte 渲染；Telnet 复用 Tokio 与代理 transport，加入 IAC 转义、NAWS、SGA 和保守 RFC 854 协商。串口端口枚举限制为表单打开、切换到串口和手动刷新；长候选下拉复用本地 lazy `fast_menu`。
- 未解决问题：参考项目与目标服务器/串口设备的具体兼容范围不能替代 AxShell 的实际三平台验收；串口权限、占用、设备拔出与 Telnet 服务端私有协商仍待手工验证。
