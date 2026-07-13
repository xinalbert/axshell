[English](resource-lifecycle.md)

# 资源生命周期与深度休眠

## 目标

在窗口不活跃时降低 AxShell 的 CPU、网络和重绘开销，同时不打断用户正在运行的 SSH 命令、本地 PTY 或 SFTP 传输。该机制不创建一个轮询焦点的额外线程；窗口激活/失活由 GPUI 的系统窗口事件驱动。

## 状态机

```text
前台 --窗口失焦--> 后台 --超过阈值--> 深度休眠
  ^                  |                    |
  +------窗口激活----+--------------------+
```

- 前台：按正常频率刷新终端、光标、监控和主题。
- 后台：立即停止远程/本地系统监控、主题轮询和光标闪烁；仍读取 backend 事件，并以低频合并刷新，避免 SSH 输出堆积在内存或 channel 中。
- 深度休眠：保持同一个低频事件泵以收集 backend 事件和执行必要的 SFTP 空闲回收；不轮询窗口焦点。系统窗口激活事件会立即恢复前台。

默认在窗口失焦 5 分钟后进入深度休眠。设置项允许选择：关闭、1、5、15 或 30 分钟。关闭只禁用深度休眠，窗口失焦后的后台降载仍然生效。

## 第一阶段：安全降载

本阶段不关闭 SSH、PTY 或 SFTP worker，包含：

- 持久化深度休眠时间设置，默认 5 分钟。
- 使用 `observe_window_activation` 接收系统窗口激活状态，维护 `Foreground / Background / DeepSleep` 状态。
- 后台和深度休眠停止监控采样、主题轮询和光标闪烁。
- 后台继续 drain 所有 backend 事件；终端和普通 UI 改为合并、低频刷新。
- 将 SFTP 空闲回收检查从事件泵的高频路径节流到定时检查，保留既有的 5 分钟空闲语义。

不会做的事：暂停远端命令、关闭本地 shell、断开 SSH 或暂停/取消 SFTP 传输。本阶段不会新增任何基于深睡的 SFTP 关闭规则；既有 SFTP 空闲回收保持原样，远程编辑 watcher 的 pin/refcount 保护留待第二阶段补齐。

## 后续防线

### 第二阶段：SFTP 引用保护和深睡回收

为每个 SFTP group 建立显式 pin/refcount。传输、远程编辑 watcher、同步、目录操作和预览下载持有 pin；只有 pin 为零、非当前聚焦 group 且已达到深睡条件时，才允许关闭 worker。恢复焦点时按需建连，不能批量重连全部页面。

实施语义：pin 在命令入队前取得，因此等待 worker 接收的操作也受保护；短操作完成后释放，传输和自动上传在 child task 结束后释放，远程编辑 watcher 在用户关闭编辑器或 worker 强制关闭后释放。用户显式关闭、取消传输或重连仍可强制结束 worker。普通后台的 5 分钟空闲回收保持原有规则；深睡时立即回收无 pin、非当前 group 的 worker。

### 第三阶段：SSH、PTY 和查询任务所有权

已实现。terminal backend 现在同时保存 command channel 和非阻塞 shutdown 控制器；现有的 tab 关闭、重连和自然 `Closed` 事件都经由该控制器收口。SSH 主任务的 `JoinHandle` 会先接收 `Close`，最多等待 2 秒，随后才 `abort`；远程监控和 CWD 查询改由主会话内的 `JoinSet` 托管，主会话退出时统一 abort/join。Local PTY 关闭时先 kill shell，再由后台 reaper join reader/writer 线程，避免阻塞 UI。

窗口关闭和应用菜单 Quit 都会调用 `shutdown_all_backends()`，同时关闭 SFTP handle；布局保存仍在关闭请求内同步执行。该阶段不递归终止 shell 自行派生的后台进程树，也不保证在 OS 强制杀进程时完成两秒 graceful window。

### 第四阶段：进程退出和系统睡眠

实现应用级 `shutdown_all()`：先停止新任务，再取消并 join 后端任务，最后保存布局。操作系统睡眠/恢复后先重新判定连接健康度，按当前聚焦页面恢复监控，避免同时向所有 SSH 发采样或重连。

## 资源策略

| 资源 | 后台 | 深度休眠 | 恢复 |
| --- | --- | --- | --- |
| SSH terminal / 本地 PTY | 保持，不中断命令 | 保持，不中断命令 | 原样继续；关闭/重连时有界回收 |
| Backend event 接收 | 低频 drain | 低频 drain | 立即合并刷新 |
| 终端渲染 / 光标 | 合并刷新，停闪烁 | 更低频刷新，停闪烁 | 恢复正常刷新 |
| 系统监控 / 远程 probe | 停止新采样 | 停止新采样 | 仅当前页面立即采样 |
| 跟随系统主题 | 停止轮询 | 停止轮询 | 恢复时同步一次 |
| SFTP 传输 | 继续 | 继续 | 不需要重连 |
| 空闲 SFTP worker | 保持既有超时回收 | 第二阶段按 pin 决定回收 | 按需重连 |

## 验证边界

- 单元测试：状态转换、超时、禁用深睡和配置归一化。
- 本机验证：`rustfmt`、`cargo check`、`cargo test --quiet`、`git diff --check`。
- GUI 手工验证：失焦后监控停止；5 分钟后进入深睡；重新聚焦立即恢复监控和终端刷新；高频 SSH 输出在后台没有持续无界堆积。
- 联机手工验证：SSH 连接中、远程 probe/CWD 查询中关闭 tab 或窗口，应在 2 秒内退出或记录 abort；local shell 关闭和重连后 reader/writer 不应遗留。
