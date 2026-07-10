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
