# 当前项目实施记录

## 当前目标

- 目标：支持已保存和新建的串口、Telnet 会话；新建串口会话时自动枚举当前已连接端口。
- 交付物：兼容会话模型、串口与 Telnet 后端、统一会话表单、端口自动检测、非 SSH 能力边界、测试与验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`Cargo.toml`、`Cargo.lock`、`src/session.rs`、`src/backend/`、`src/terminal/`、`src/app/`、`locales/`、`docs/project-env-audit/`、`docs/project-implementation-tracker/`。
- 不在本轮范围内：SSH/SFTP 协议变更、Telnet 身份认证自动化、串口设备驱动安装、跨进程端口占用恢复。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 审查现有会话/终端架构与参考项目 | 源码、依赖与参考实现核对 | AxShell 事件总线可复用；采用 `serialport` 同步 I/O 与 TCP Telnet 路径 |
| P2 | completed | 扩展会话模型与实现 Serial/Telnet 后端 | 单元测试、`cargo check` | Serial 采用阻塞设备线程；Telnet 采用最小 RFC 854 协商 |
| P3 | completed | 新建/编辑表单、端口检测和非 SSH UI 边界 | 静态审计、全量测试 | 串口仅在表单打开、切换到串口或手动刷新时枚举；SFTP 仅限 SSH |
| P4 | completed | 完成格式化、回归、跟踪记录与验证 | `cargo test --quiet`、差异检查、tracking validator | 保留实体串口与 Telnet server 手工验收 |

## 已完成

- `SessionKind` 增加 `Ssh`、`Serial`、`Telnet`，旧保存会话和 v1 share 文件自动回退到 SSH；串口参数包含端口、波特率、数据位、校验、停止位和流控。
- 新增 `serialport` 后端：设备打开与读写均在非 UI 线程，50ms 读取超时允许可控关闭；表单打开、切换到串口或点击刷新时才枚举当前端口。
- 新增 Telnet 后端：通过现有直连/SOCKS5/HTTP 代理通道连接，支持 IAC 转义、NAWS、SGA 和保守的 RFC 854 选项回应；空端口默认 23。
- 统一连接表单、会话选择器、侧栏、会话分屏、断线重连和无凭据 share JSON；非 SSH 会话不创建 SFTP 页面、不参与 SSH 密码提示、远程监控或连接健康检查。
- 连接进度失败后的“重试”按 Serial/Telnet/SSH 分派，复用保留 terminal scrollback 的后端重建路径；仅 SSH 重启关联 SFTP。
- 串口端口下拉使用共享 `fast_menu` 惰性候选构造，保存会话选择列表保持 `uniform_list` 虚拟渲染。

## 验证

- 已完成：受影响 Rust 文件 `rustfmt --edition 2024`；`cargo test --quiet`（220 项）；`cargo check`；`git diff --check`；fast hover/list 静态审计；tracking docs validator。
- 未完成：在 macOS、Windows、Linux 分别以真实已连接串口测试自动枚举、收发、设备拔出和重连；以可访问 Telnet server 测试登录、协商、窗口大小与断线重试。

## 风险与阻塞

- 无代码阻塞；串口权限、设备驱动、其他进程占用和 Telnet 服务端协商差异依赖目标系统与服务端环境。

## 下一步

- 执行三平台实体串口与 Telnet GUI 验收，并依据实际设备/服务端兼容结果补充诊断或协议选项。

## 最后更新时间

- 2026-07-16 17:00 +0800
