# 外部检索记录

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
