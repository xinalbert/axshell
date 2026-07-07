[English](README.en.md)

# AxAshell

![Preview](preview.png)

AxAshell 是一个基于 Rust、GPUI 和 `alacritty_terminal` 的桌面终端工作区，提供本地 Shell、SSH 远程会话和内置 SFTP 文件管理。

Forked from <https://github.com/rust-kotlin/ashell.git>

仓库地址：<https://github.com/xinalbert/ax_ashell>

## 主要功能

- 本地终端标签页与 SSH 会话管理，支持密码和私钥认证
- 内置 SFTP 面板，支持浏览、上传、下载、删除、断点控制和远程文件编辑
- 多标签、多 Pane 工作区，支持快速搜索、拆分、聚焦和关闭面板
- 设置页内置主题、字体、快捷键、监控面板和标题栏样式配置
- 通过 WebDAV 或 S3 做会话配置同步，上传内容端到端加密
- 全局代理和 SSH X11 转发设置，适合远程运维和图形程序转发场景

## 快速开始

直接运行桌面应用：

```bash
cargo run --release
```

开发期自动重编译并重启：

```bash
cargo dev-reload
```

更完整的运行、开发和打包说明见 [开发与打包](docs/development.md)。

## 文档

- [使用指南](docs/user-guide.md)：界面结构、SSH / SFTP、同步、代理和 X11 的具体用法
- [开发与打包](docs/development.md)：开发命令、`cargo dev-reload`、macOS `.app` 和 Debian `.deb` 打包

## 当前状态

- 运行时与打包图标统一使用 `assets/icons/terminal_icon_all_formats`
- 当前保留 GitHub Actions 构建与 artifact 上传链路
- 自动发布 GitHub Release、Homebrew cask 等依赖外部密钥的发布流程暂未启用

## 许可证

本项目采用 [GPL-3.0-or-later](LICENSE)。
