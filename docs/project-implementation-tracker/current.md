# 当前项目实施记录

## 当前目标

- 目标：消除高 RTT SSH 本地输入 overlay 的本地绘制卡顿，不改变 P8 的输入语义和安全回退。
- 交付物：不把 composition 文本变化纳入终端行布局缓存 key、缓存行为回归测试和完整 Rust 验证。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/terminal/element.rs`、`docs/project-env-audit/`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：改变 SSH 协议或引入 Mosh 服务端、P8 的输入语义/会话配置/回退条件、已确认终端 buffer、IME/selection 绘制语义、默认开启本地回显，以及 Telnet/串口输入优化。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | SSH 与 SFTP 共享的主机密钥确认、持久化与失配拒绝机制 | 主机信任存储单元测试；`cargo check`；真实 SSH/SFTP 首连、重连和密钥变更手工验收 | 未知或变更密钥默认拒绝；首次连接需确认 |
| P2 | completed | 将 legacy SSH 算法改为会话级显式 opt-in，默认只用安全算法 | 会话策略单元测试；`cargo check`；`cargo test --quiet` | 不自动回退，也不再根据连接结果写回算法模式 |
| P3 | completed | 同步 endpoint 强制 HTTPS，并限制 WebDAV/S3 响应大小 | URL/响应限额单元测试；`cargo check` | HTTP 在保存和发起请求前均被拒绝 |
| P4 | completed | 更新可修复的 RustSec 依赖并在 CI 执行锁文件审计 | `cargo audit`；`cargo check`；CI YAML 审阅 | 3 项可修复公告已消除；3 条无可用兼容修复的公告在 CI 中明确暂缓 |
| P5 | completed | 更新双语安全行为文档、环境/实施记录并完成收口验证 | tracking validator；`git diff --check`；完整 `cargo test --quiet` | 自动化收口完成，保留实机验收清单 |
| P6 | completed | 修复主机密钥确认框与连接进度遮罩的层级和点击冲突 | `rustfmt`；`cargo check`；`cargo test --quiet`；真实首次连接确认点击 | 主机密钥确认是唯一可交互模态；等待确认时不显示连接进度遮罩 |
| P7 | completed | 建立 SSH 输入到远端输出的匿名反馈延迟基线 | `TerminalTab` 单元测试；`cargo check`；`cargo test --quiet` | 仅记录时间与聚合值；不记录按键内容，不改变 backend 写入顺序 |
| P8 | completed | 默认关闭的 SSH 会话级本地输入 overlay | 定向单元测试；`rustfmt`；`cargo check`；`cargo test --quiet`；`git diff --check` | 仅主屏、底部、已连接 SSH；不支持的输入先按顺序 flush 再直通 |
| P9 | completed | 本地输入 overlay 不失效终端行布局缓存 | `TerminalElement` 定向测试；`rustfmt`；`cargo check`；`cargo test --quiet`；`git diff --check` | composition 独立绘制；只有实际影响 row shape 的状态可失效 cache |

## 已完成

- 已完成源码与 RustSec 审计：确认 SSH/SFTP callback 无条件接受服务器密钥、默认自动尝试 SHA-1/DSA/CBC/3DES legacy 模式、同步允许 HTTP 且无响应大小限制；SFTP 本地路径穿越和预览/浏览限额已有保护。
- 已完成本轮环境预检、项目地图刷新和联网研究记录；基线 `cargo test --quiet` 为 225 passed。
- P1 已完成：SSH 与 SFTP 在握手时使用同一份本地主机密钥信任记录；首次发现和密钥失配都需要用户比对 SHA-256 指纹并明确确认，超时或拒绝时连接失败。
- P2 已完成：legacy 算法仅可在单个 SSH 会话的高级选项中明确开启；终端和 SFTP 均只使用所选算法集，历史自动降级字段和连接成功后的模式回写已移除。
- P3 已完成：同步仅接受 HTTPS endpoint，WebDAV/S3 成功和错误响应均以流式读取限制在 8 MiB；无效 endpoint 不会写入本地配置。
- P4 已完成：`crossbeam-epoch`、`quinn-proto` 和 `memmap2` 已升级到修复版本，CI 新增 RustSec 审计。其余 `rsa` 和 `quick-xml` 公告受无上游补丁或当前上游版本约束影响，已在 CI 命令中以公告编号及理由显式暂缓。
- P6 已完成：对话框层移到所有应用内遮罩之后；存在活动对话框时不渲染连接进度遮罩，因此主机密钥确认成为唯一可见、可点击的模态交互。
- P7 已完成方案研究：`xiaoxingshell` 使用提示符感知的本地行缓冲和远端回显去重；Mosh 使用有 ACK/过期语义的预测 overlay。两者均表明预测层必须与确认终端状态分离，本项目先测量现有 SSH 输入反馈再修改交互语义。
- P7 已完成实现：SSH 键盘和 IME 输入会启动匿名反馈样本，首个后续输出更新最近值与平均值；连续输入合并为一个样本，30 秒无反馈的样本被丢弃，重连时清空待确认状态。日志不含按键或远端输出内容。
- P8 已完成：SSH 高级选项新增默认关闭的会话级开关。启用后，`LocalInputBuffer` 只在已连接 SSH 主屏、滚动到底部和可见 cursor 时预测单行 ASCII、退格和左右键；Enter 发送整行并等待首个远端输出清理。粘贴、IME、Tab/Ctrl/Alt、鼠标选择、滚动和工作区迁移会先 flush；未提交行遇异步输出也会先发送，避免输入丢失。预测层不写入 Alacritty 确认 buffer。
- P9 已完成定位：`TerminalElement::cached_grid_rows` 的 `GridLayoutKey` 包含 `TerminalComposition`，但 `layout_row` 不读取 composition。每次本地键入改变 overlay 文本都会拒绝所有可见行的 `GridLayoutCache`，触发完整行 shape；composition 实际只在独立 paint 阶段使用。
- P9 已完成上游核对：Zed 在 `prepaint` 中布局确认 terminal cell，并在 `paint` 中单独 shape / 覆盖 IME marked text；其分层与本地 cache-key 收窄一致，来源记录于 `docs/project-implementation-tracker/research.md`。
- P9 已完成实现：`GridLayoutKey` 只保留 `layout_row` 实际消费的 style 和 selection；本地输入或 IME composition 文本继续在 paint 阶段独立重绘，不再使已确认可见行重新 shape。回归测试明确限定 cache key 只跟踪会改变 shaped row 的状态。

## 验证

- 已完成：安全代码审阅、RustSec 官方公告数据库审计、依赖链初步定位、基线 `cargo test --quiet`（225 passed）；P1 的 `cargo test --quiet host_key`（6 passed）、P2 的 `cargo test --quiet legacy_ssh`（1 passed）、P3 的 `cargo test --quiet sync`（7 passed）；P7 的 `cargo test --quiet input_feedback`（3 passed）；P8 的 `cargo test --quiet local_input`（3 passed）、`cargo test --quiet session::tests::new_session_fields_default_when_loading_existing_sessions`（1 passed）、`cargo test --quiet local_input_overlay_requires_opt_in_and_primary_screen`（1 passed）；各步骤的 `cargo check`；P8/P9 完整 `cargo test --quiet`（238 passed）、`rustfmt`、`git diff --check` 和 tracking docs validator；P9 的 `cargo test --quiet grid_layout_key`（1 passed）。
- 未完成：100/250/500 ms RTT SSH 服务上的 P7/P8/P9 手工采样与交互验收；主机密钥确认点击的实机验收、CI 实跑，以及 macOS/Windows/Linux 的真实 SSH/SFTP/同步服务验收。

## 风险与阻塞

- 主机密钥首次信任需要 UI 明确确认，不能静默 TOFU；SSH 与 SFTP 必须使用同一份持久化信任记录。真实服务上的首连、重连、失配和关闭确认仍待手工验收。
- `rsa` Marvin 公告没有可用上游补丁，需通过关闭弱降级、主机认证和依赖上游跟进降低暴露面。
- 依赖升级可能牵动 GPUI Git 依赖，必须与认证行为修复分离验证。
- RustSec 仍报告 `rsa` 和 `quick-xml` 的已知公告，但 CI 只暂缓无可用兼容修复的三个公告 ID；任何新的漏洞公告仍会使 CI 失败。
- 主机密钥确认必须保留明确的“拒绝/信任”选择；问题在于两个模态层同时存在而非确认本身多余。
- 远端输出不保证是对输入的逐字回显；P7 仅测量从本地输入到首个后续远端输出的反馈时间，不能把它作为严格网络 RTT。对无输出或全屏应用必须超时清理，不能阻塞输入。
- P8 不自动识别 shell prompt，因此用户只能在普通、单行 shell 提示符处启用；密码提示、REPL、文本编辑器和全屏程序必须依赖回退路径，不能承诺预测显示正确。
- P8 仍不能把首个远端输出当作逐字回显确认；它只把该输出作为显示层失效信号。用户输入内容不会进入日志或 metrics，但会在启用模式下短暂保留于进程内内存，直至提交、flush 或清理。
- P9 保留 selection、字体、terminal snapshot、highlight 和颜色变化的布局失效；只移除了未被 `layout_row` 消费的 composition 依赖。GUI frame-time 改善仍须通过真实长 scrollback 和高频输入验收。

## 下一步

- 在 100/250/500 ms RTT 的 SSH 链路上验证长 scrollback 下的快速连续输入、IME、selection、字体变更和 P8 的所有回退路径；记录输入路径的 GUI frame-time 采样。

## 最后更新时间

- 2026-07-18 15:48 +0800
