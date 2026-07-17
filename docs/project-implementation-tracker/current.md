# 当前项目实施记录

## 当前目标

- 目标：降低 AxShell 在 macOS、Windows 和 Linux 上的独立窗口常驻内存与空闲渲染开销，同时保留终端窗口的现有交互和跨窗口迁移语义。
- 交付物：按需、无进程表的本地系统采样；轻量独立窗口初始化；跨平台渲染资源边界与验证记录。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`src/monitoring.rs`、`src/app/state/monitoring.rs`、`src/app/lifecycle/event_loop.rs`、`src/app/workspace.rs`、`src/app/lifecycle/init.rs`、`src/app/lifecycle/startup.rs`、必要的 `src/app.rs`、`docs/project-implementation-tracker/`、`worker.md`。
- 不在本轮范围内：修改或 vendoring GPUI/Zed renderer、更新 `Cargo.toml` / `Cargo.lock`、重做原生窗口 surface、改变终端/SFTP 协议或发布安装包。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：是，已完成
- 多 agent：已结束

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 环境、当前内存 profile、GPUI 多窗口资源边界与实施方案 | 运行时 `vmmap`、上游源码/Apple 文档审查 | 不修改依赖；每个可见原生窗口仍需自己的 surface/drawable |
| P2 | completed | 按需且不保留全进程表的跨平台系统采样 | 单元测试、`cargo check` | 仅采集当前 UI 需要的 CPU/RAM/网络/磁盘；构造与采样后均不保留进程表 |
| P3 | completed | 独立终端窗口跳过不需要的监控、文件图标和本地目录预热 | 完整 Rust 测试、`cargo check` | 保留转移的 `TabGroup`、runtime 和关闭/回迁语义；主窗口关闭时释放全局强引用 |
| P4 | completed | 整合跨平台窗口策略与 renderer 边界 | 锁定 GPUI 源码和平台 API 审阅 | 共享设备/上下文而非 native surface；不试图跨窗口共享 `CAMetalLayer` / swapchain / `wgpu::Surface` |
| P5 | completed | 格式化、测试、tracking 校验和手工验收说明 | `cargo test --quiet`、`git diff --check`、tracking validator | 自动化与文档校验完成；三平台实机 GUI / memory profile 清单已交付，尚待目标平台执行 |

## 已完成

- 运行时确认 debug sample 的 macOS physical footprint 为约 245.7 MiB，其中 `IOSurface` 约 101.2 MiB、malloc 分配约 79.8 MiB。
- 确认每个窗口无条件建立 `sysinfo::System::new_all()` 并常驻完整进程表；当前界面不读取该进程表。
- `SystemSampler` 改为 CPU/内存选择性刷新，局部采样器仅在可见主窗口监控面板首次需要时构造；系统恢复只重置已构造的采样器。
- 独立窗口在构造期即进入 `Detached` 模式：不加载/刷新文件图标缓存，不验证或枚举本地 SFTP 目录，也不会因全局监控配置创建采样器。
- 主窗口关闭时移除匹配的 `MainWorkspace` global，避免 detached 窗口存活期间保留整套不可见主窗口状态。
- 联网核对 GPUI/Zed 三端后端：每个窗口必须持有独立的 `CAMetalLayer` / DXGI swapchain / `wgpu::Surface`；不把原生 surface 共享列入本轮实现。
- 三个低重叠 worker 已完成交付；本地集成复核确认独立窗口的实际监控可见性路径不会重新创建 sampler。

## 验证

- 已完成：项目环境 quick scan、实施记录/项目地图/研究记录审阅、当前 memory profile、GPUI/Zed 与平台 API 核对；`rustfmt --edition 2024`、针对性进程表测试、`cargo check`、`cargo test --quiet`（223 项）、`git diff --check`、tracking validator。
- 未完成：macOS / Windows / Linux 目标机上的 GUI memory profile 与独立窗口手工验收；这不阻塞已验证的跨平台 Rust 逻辑交付。

## 风险与阻塞

- macOS `IOSurface`、Windows swapchain、Linux WGPU surface 是每个可见原生窗口的必要基线；本轮只能减少应用状态和避免不必要重绘，不能承诺消除该部分。
- 锁定 GPUI 是 Git 依赖；renderer 级共享资源需要上游或依赖 patch，超出本轮低风险范围。
- Windows 和 Linux target 的实际 GUI/内存 profile 需要对应平台 CI 或实机验证。

## 下一步

- 在各目标平台按以下清单验收：用同一 release、窗口尺寸和 tab 内容，逐步打开主窗口/两个 detached 窗口并静置 30–60 秒；测试输入、resize、焦点、迁移、回迁、关闭主窗口后的返回提示；macOS 用 `vmmap -summary` 区分 MALLOC 与 IOSurface，Windows 用 VMMap / Task Manager 记录 Private Bytes 与 GPU memory，Linux 用 `smem` 或 `smaps_rollup` 加相应 GPU 工具记录稳定增量。若 renderer buffer 仍主导，再单独评估上游 patch。

## 最后更新时间

- 2026-07-17 10:25 +0800
