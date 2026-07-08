## 2026-07-06 初始化环境预检记录

- 触发原因：用户要求先评估实现难度，再进入真实施工
- 执行内容：检查项目根目录、`Cargo.toml`、`README.md`、CI workflow、本机 `rustc` 与 `cargo` 可用性，并初始化环境记忆目录
- 影响文件：`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 计划状态变更：无
- 验证结果：确认项目为 Rust 桌面应用；`rustc --version` 与 `cargo --version` 可执行；当前 CI 仅构建未跑测试
- 对 plan 的更新：允许进入 `docs/project-implementation-tracker/` 规划阶段

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
