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
