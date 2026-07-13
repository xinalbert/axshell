[English](monitoring-lifecycle.md) · [文档导航](../README.zh.md)

# 监控与生命周期

## 系统监控

监控页面可以显示本机或当前 SSH 会话的系统信息，包括 CPU、内存、Swap、网络、磁盘和平台信息。设置中可以显示或隐藏监控，并选择放在侧边栏或底部区域。

## 窗口进入后台

窗口失去焦点后：

- 系统监控刷新、主题轮询和光标闪烁会立即停止；
- 终端进程、SSH 命令和 SFTP 传输继续运行。

## 深度休眠

可配置延迟为关闭、1、5、15 或 30 分钟，默认 5 分钟。

进入深度休眠后，AxShell 只保留低频 backend 事件处理。深睡不会断开本地终端或 SSH 会话；窗口重新获得焦点后，会立即恢复渲染、监控、主题更新和当前页面。

实现边界和生命周期设计见[资源生命周期](../resource-lifecycle.zh.md)。

<!-- 截图目标：../images/features/monitoring-dashboard.png -->
<!-- 截图目标：../images/features/monitoring-lifecycle-settings.png -->
