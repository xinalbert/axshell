# 当前项目实施记录

## 当前目标

- 目标：每次通过快捷键或 Settings 标签关闭按钮关闭设置页时都弹出确认；弹窗打开后，再按一次 Settings 快捷键执行上次记住的“关闭”或“保持打开”动作。
- 交付物：向后兼容的第二次快捷键动作偏好、始终出现的确认弹窗、记住选择持久化、Workspace 动作开关、快捷键/标签按钮接线、中英文文案和完整验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/config/model.rs`，`src/config/store.rs`，`src/app.rs`，`src/app/lifecycle/init.rs`，`src/app/dialogs.rs`，`src/app/dialogs/settings_close_confirm.rs`，`src/app/dialogs/settings/shell.rs`，`src/app/dialogs/settings/workspace.rs`，`src/app/workspace.rs`，`src/app/views/tab_bar.rs`，`locales/en.yml`，`locales/zh-CN.yml`，跟踪文档。
- 不在本轮范围内：把所有 Settings 表单改造成统一草稿/撤销事务、自动提交带显式 Save 按钮的输入、调整复制粘贴键位、修改依赖或 manifest/lock。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P14 | completed | 环境预检、现有确认模式复核、项目地图刷新和本轮计划 | 工具链、manifest、CI、配置、dialog、workspace 与工作树复核 | 保留现有未提交终端改动 |
| P15 | completed | 默认关闭页面且向后兼容的第二次 Settings 快捷键动作偏好 | 配置默认值、getter/setter 与 serde/default 定向测试 | remember 可保存关闭或保持打开 |
| P16 | completed | Settings 关闭确认弹窗、确认/取消和 remember 状态 | 源码差异、编译检查 | 文案明确即时保存与显式 Save 的边界 |
| P17 | completed | 快捷键、标签关闭按钮、Workspace 恢复开关和中英文文案 | `cargo check`、调用点复核 | 所有显式关闭入口走同一 request 方法 |
| P18 | completed | 首版“记住后跳过弹窗”实现的格式化和回归 | `cargo test --quiet`、`git diff --check`、tracking validator | 用户已纠正语义，本步骤结果仅作为中间验证 |
| P19 | completed | 改为每次弹窗，第二次 Settings 快捷键执行记住动作 | 配置测试、`cargo check`、调用点复核 | 默认第二次快捷键确认关闭 |
| P20 | completed | 修正后完整测试、空白检查和文档校验 | `cargo test --quiet`、`git diff --check`、tracking validator | GUI 行为保留手工验证边界 |

## 已完成

- 已确认 Settings 大多数控件在修改时立即保存，部分表单仍明确要求点击 Save，因此不能承诺关闭时统一提交全部输入。
- 已确认 SFTP 关闭确认已提供 `DialogKind`、不可点击遮罩、remember checkbox 和配置持久化参考模式。
- 已确定弹窗语义为“已应用的设置自动保存，确认关闭”；确认弹窗每次都出现。
- 已确定 remember 保存的是“弹窗打开后再次按 Settings 快捷键执行关闭或保持打开”，而不是跳过后续弹窗。
- 已完成施工前环境预检；不新增依赖、不联网、不使用多 agent，并刷新项目地图覆盖新增确认模块和配置偏好。
- 已新增默认执行关闭的 `settings_close_shortcut_confirms` 配置字段、getter/setter 和默认值；旧配置缺少字段时默认让第二次快捷键确认关闭。
- 已完成配置 Rust 格式化和 2 项设置关闭确认定向测试。
- 已新增独立 `settings_close_confirm.rs`，支持确认、取消、remember checkbox 和确认偏好持久化。
- 已新增 `request_close_settings_page`，快捷键与 Settings 标签关闭按钮统一打开确认弹窗。
- 已在 Workspace 设置页加入“第二次设置快捷键关闭页面”开关，并补充中英文 dialog、checkbox、按钮和提示文案。
- 已将配置偏好改为 `settings_close_shortcut_confirms`：默认第二次快捷键关闭，remember 可保存“关闭”或“保持打开”。
- 已让 close request 始终打开确认框，并为 dialog content 建立独立焦点和当前 Settings 快捷键监听；第二次按键执行已记住动作。
- 已将 Workspace 开关和中英文文案修正为“第二次 Settings 快捷键动作”，不再存在跳过后续弹窗的路径。
- 已完成修正后的 Rust 格式化、2 项配置定向测试和 `cargo check`。
- 已完成修正后的 85 项完整测试、`git diff --check` 和 tracking docs validator；项目地图已覆盖新增 dialog 与配置偏好。

## 验证

- 已完成：环境、工作树、配置持久化、dialog 模式、Settings 快捷键和标签关闭调用点复核；语义修正后的实现、Rust 格式化、2 项配置定向测试、`cargo check`、85 项完整测试、`git diff --check` 和 tracking docs validator。
- 未完成：真实 GUI 中验证每次关闭均弹窗、第二次当前 Settings 快捷键执行记住动作，以及 remember 保存关闭/保持打开。

## 风险与阻塞

- 风险一：确认弹窗必须避免声称会自动提交仍处于输入框中的显式 Save 表单。
- 风险二：remember 只能改变弹窗内第二次快捷键动作，不能改变“每次都弹窗”的规则。
- 风险三：快捷键和标签关闭按钮需要统一走 request 方法，实际关闭方法仍保留为内部确认后的最终动作。
- 剩余风险：自动化检查无法替代真实窗口中确认、取消、记住与重新启用确认的交互验证。
- 无阻塞。

## 下一步

- 功能和自动化验证完成，可按独立 review unit 提交并创建合规 release tag。

## 最后更新时间

- 2026-07-10 22:47 +0800
