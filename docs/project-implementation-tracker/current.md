# 当前项目实施记录

## 当前目标

- 目标：修正 SFTP 页面传输默认位置、传输信息呈现和文件列表点击习惯
- 交付物：SFTP 下载默认落到右侧本地浏览器当前目录；上传默认发到左侧远端当前目录；上传/下载启动后不弹出传输历史窗口，只在页面底部 Active 传输列表单行显示；远端和本地列表改为单击选中、再次点击目录才打开

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/app/actions/sftp.rs`，`src/app/views/sftp_panel.rs`，`src/app/views/sftp_panel/transfer_panel.rs`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：SFTP 后端协议重写、真实 SSH/SFTP 联机验证、传输队列持久化策略、全局传输弹窗功能删除

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 环境预检和实施计划切换到 SFTP 交互修正 | tracking docs validator | 已确认本轮不依赖联网和新增依赖 |
| P2 | completed | SFTP 上传/下载默认路径和传输提示行为修正 | `cargo check`，源码检查 | 上传仍可用 picker 选择源，下载不再要求选择目录 |
| P3 | completed | 远端/本地列表单击选中、再次点击目录打开 | `cargo check`，源码检查 | 本地文件再次点击用系统默认应用打开 |
| P4 | completed | 格式化、编译测试和文档收口 | `rustfmt`，`cargo check`，`cargo test`，tracking docs validator | GUI 手工验证仍需用户本机确认 |

## 已完成

- 已完成施工前环境预检，确认项目仍为 Rust / GPUI 桌面应用
- 已定位 SFTP 行为入口：`src/app/actions/sftp.rs` 与 `src/app/views/sftp_panel.rs`
- 已将远端下载默认目录改为右侧本地浏览器当前目录
- 已将本地上传默认远端目录保持为左侧远端当前目录，并取消传输历史自动弹窗
- 已将远端/本地列表改为单击选中、再次点击目录打开；本地文件再次点击用系统默认应用打开
- 已将 SFTP 页面底部传输行压缩为单行显示

## 验证

- 已完成：`rustfmt --edition 2024 src/app/actions/sftp.rs src/app/views/sftp_panel.rs src/app/views/sftp_panel/transfer_panel.rs`
- 已完成：`cargo check` 通过
- 已完成：`cargo test` 通过，25 个测试全部通过
- 已完成：tracking docs validator 通过
- 未完成：GUI 手工验证、真实 SSH/SFTP 连接验证

## 风险与阻塞

- 风险一：GUI 单击/再次点击行为需要真实应用中确认手感；本轮以源码行为和编译测试验证为主
- 风险二：全局 Transfers 弹窗仍保留菜单入口，本轮只取消 SFTP 启动传输后的自动弹窗

## 下一步

- 在真实 SFTP 连接中确认双栏默认目录、Active 单行传输显示和二次点击打开目录的手感

## 最后更新时间

- 2026-07-09 12:45 +0800
