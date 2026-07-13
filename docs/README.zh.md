[English](README.md) · [项目首页](../README.zh.md)

# AxShell 文档导航

这里集中放置按任务拆分的用户指南、开发说明和设计资料。建议先阅读快速入门，再进入当前操作对应的功能页面。

## 从这里开始

- [快速入门](getting-started.zh.md)：环境要求、启动命令和主界面区域
- [使用指南索引](user-guide.zh.md)：原单篇使用指南的兼容入口
- [开发与打包](development.zh.md)：本地开发、安装包和发布流程

## 功能指南

| 功能 | 文档 |
| --- | --- |
| 本地终端和 SSH 会话 | [终端与 SSH](features/terminal-ssh.zh.md) |
| 标签、Pane、搜索和快捷键 | [工作区](features/workspace.zh.md) |
| 远程文件和传输 | [SFTP](features/sftp.zh.md) |
| 主题、字体、标签颜色和设置行为 | [外观与设置](features/appearance-settings.zh.md) |
| 内置字体家族和上游仓库 | [内置字体](features/bundled-fonts.zh.md) |
| WebDAV/S3 加密会话同步 | [配置同步](features/configuration-sync.zh.md) |
| SOCKS5/HTTP 代理和 X11 转发 | [代理与 X11](features/proxy-x11.zh.md) |
| 监控、后台状态和深度休眠 | [监控与生命周期](features/monitoring-lifecycle.zh.md) |
| 配置、日志、迁移和问题反馈 | [本地数据与故障排查](features/local-data-troubleshooting.zh.md) |

## 设计与维护

- [资源生命周期](resource-lifecycle.zh.md)：后台资源策略和深度休眠设计
- [截图说明](images/README.zh.md)：后续文档图片的命名和放置方式
- [项目实施记录](project-implementation-tracker/current.md)：当前仓库实施状态

## 文档约定

- 根 README、开发/设计资料和功能页使用英文 `name.md`、中文 `name.zh.md`。
- 双语页面保持相同结构，并使用相对链接。
- 功能截图放在 `docs/images/features/`；各功能页已用注释预留插入位置。
