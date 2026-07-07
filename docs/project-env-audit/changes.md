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

## 2026-07-07 完成设置页焦点修复的本机验证

- 目的：修正设置页快捷键录制焦点逻辑导致普通输入框无法输入的问题
- 改动范围：`src/app/dialogs.rs`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`
- 执行内容：移除设置页根容器任意鼠标按下时强制聚焦主 `focus_handle` 的逻辑；保留快捷键录制按钮显式聚焦主 `focus_handle`；在设置页根容器 `on_key_down` 中增加焦点校验，只有主 `focus_handle` 当前聚焦时才处理快捷键录制和设置页标签切换
- 验证结果：`rustfmt --edition 2024 --config skip_children=true src/app/dialogs.rs`、`cargo check`、`cargo test` 均通过；13 个 Rust 测试全部通过；仅保留既有 `block v0.1.6` future-incompat warning
- 风险/待办：GUI 最终交互效果仍需本机手工确认；若后续希望点击设置页空白处也支持快捷键切换标签，需要改成只在非输入控件背景点击时聚焦，而不是恢复全局抢焦点
