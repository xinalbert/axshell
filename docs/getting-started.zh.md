[English](getting-started.md) · [文档导航](README.zh.md)

# 快速入门

## 环境要求

- Rust `1.88.0` 或更高版本
- Cargo
- macOS、Linux 或 Windows 桌面环境

## 运行 AxShell

```bash
cargo run --release
```

开发期使用重启式自动重载：

```bash
cargo dev-reload
```

打包依赖和各平台命令见[开发与打包](development.zh.md)。

## 主工作区

- **已保存会话：** 打开本地终端、按组管理 SSH 连接并新建连接。
- **终端工作区：** 使用标签、拆分 Pane、终端搜索和可配置快捷键。
- **SFTP 页面：** 从 SSH 会话浏览远程文件并管理传输。
- **系统监控：** 查看本地或远端 CPU、内存、网络、磁盘和系统信息。
- **设置：** 配置外观、终端行为、工作区规则、同步、代理和 X11。

## 下一步

- 连接远程主机：[终端与 SSH](features/terminal-ssh.zh.md)
- 了解工作区操作：[工作区](features/workspace.zh.md)
- 传输远程文件：[SFTP](features/sftp.zh.md)
- 查找配置和日志：[本地数据与故障排查](features/local-data-troubleshooting.zh.md)

<!-- 截图目标：images/features/getting-started-workspace.png -->
