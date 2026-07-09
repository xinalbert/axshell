# 当前项目实施记录

## 当前目标

- 目标：修正终端路径跳转里“最后一级目录不会真正进入”的问题；将目录/文件/不存在的判断下沉到 SFTP 侧，目录存在就直接进入，文件或不存在才退到上一级
- 交付物：SFTP 后端目标路径判定与 reveal 命令；前端改为直接传绝对目标路径；目录直进、文件/不存在回父目录的一致行为

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal/highlight.rs`，`src/sftp/path.rs`，`src/sftp.rs`，`src/app.rs`，`src/app/core/types.rs`，`src/app/actions/session.rs`，`src/app/actions/terminal.rs`，`src/app/actions/sftp.rs`，`src/app/lifecycle/event_loop.rs`，`src/app/lifecycle/init.rs`，`src/app/workspace/workspace.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：本地文件路径跳转、非 SFTP 页面中的额外 UI 提示文案、Windows GUI / Linux GUI 手工验收、远端路径不存在时的专门错误对话框、路径中空格的 shell 语义推断增强

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 复查“最后一级目录不进入”的当前根因，并确认应把判定从前端下沉到 SFTP 后端 | 源码检查 | 项目地图已覆盖 `src/sftp.rs` 与相关 action 路径，无需刷新 |
| P2 | completed | SFTP 后端 `reveal` 语义：目录直进，文件/不存在回父目录 | `cargo test --quiet reveal_target_directory -- --nocapture`，`cargo check` | 不再由前端提前猜父目录 |
| P3 | completed | 前端改为直接传绝对目标路径，复用 SFTP 后端判定结果 | `cargo test --quiet terminal::highlight -- --nocapture`，`cargo check` | URL 与路径分流逻辑保持不变 |
| P4 | completed | 验证与 tracking 收口 | `rustfmt`，`cargo check`，`git diff --check`，tracking docs validator | GUI 手工验收仍未执行 |

## 已完成

- 已完成施工前环境预检，确认项目仍为 Rust / GPUI 桌面应用，且本轮不需要联网、不使用多 agent
- 已确认之前的问题在前端：`open_sftp_and_reveal_path()` 在点击前就直接把目标退到了父目录，所以最后一级目录永远不会真正进入
- 已在 `src/sftp.rs` 新增 SFTP 后端 `RevealPath` 路径揭示语义，让后端自己通过 `metadata()` 判断目标是目录、文件还是不存在
- 已将目标目录决策固定为：目录存在就直接进入；文件或不存在则回退到父目录
- 已将前端 `src/app/actions/sftp.rs` 改为直接把绝对目标路径交给 SFTP，不再提前猜父目录

## 验证

- 已完成：`rustfmt --edition 2024 src/sftp.rs src/app/actions/sftp.rs`
- 已完成：`cargo check`
- 已完成：`cargo test --quiet reveal_target_directory -- --nocapture`
- 已完成：`cargo test --quiet terminal::highlight -- --nocapture`
- 已完成：`git diff --check`
- 已完成：tracking docs validator
- 未完成：GUI 手工验证，真实 SSH / SFTP 联机点击验证

## 风险与阻塞

- 风险一：目录/文件判定现在依赖 SFTP `metadata()`；若某些服务端对权限、符号链接或不存在路径的错误返回特殊，仍需实机确认
- 风险二：当前路径识别规则本身未变，复杂 shell 转义、嵌套引号和更激进的 token 形态仍留待后续迭代
- 风险三：当前只实现远端 SFTP 跳转；本地终端路径单击打开本地文件浏览器不在本轮范围内

## 下一步

- 在真实 SSH / SFTP 会话里手工验证以下场景：存在的目录路径会直接进入；存在的文件路径会回到父目录并选中文件；不存在路径会回到父目录
- 如需要，再继续扩展更复杂的路径 token 识别和本地路径跳转

## 最后更新时间

- 2026-07-09 16:53 +0800
