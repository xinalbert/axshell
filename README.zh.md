[English](README.md)

# AxShell

![AxShell 工作区预览](preview.png)

AxShell 是一个基于 Rust 和 GPUI 的桌面终端工作区，用于统一管理本地 Shell、SSH 会话、SFTP 文件与日常远程运维操作。

项目 Fork 自 <https://github.com/rust-kotlin/ashell.git>，当前仓库为 <https://github.com/xinalbert/axshell>。

## 主要能力

- 本地终端和已保存 SSH 会话，支持密码与私钥认证
- 多标签、多 Pane 工作区，支持快捷键配置和终端搜索
- 内置 SFTP 浏览、传输控制、远程编辑和超大目录分页
- 主题、字体、标签颜色、监控和工作区偏好设置
- 通过 WebDAV 或 S3 兼容存储加密同步会话配置
- 全局/单会话代理与 SSH X11 转发

## 快速开始

AxShell 需要 Rust `1.88.0` 或更高版本。

```bash
cargo run --release
```

开发期使用重启式自动重载：

```bash
cargo dev-reload
```

## 文档

- [文档导航](docs/README.zh.md)
- [快速入门](docs/getting-started.zh.md)
- [功能指南](docs/README.zh.md#功能指南)
- [内置字体](docs/features/bundled-fonts.zh.md)
- [开发与打包](docs/development.zh.md)

## 项目说明

- Release tag 使用 `vYYYY.M.D` 或 `vYYYY.M.D-N`。
- 发布流程构建 Windows x86_64、Linux x86_64/aarch64，以及 macOS 分架构和 universal 安装包。
- 需要迁移时，旧 `ax_ashell` 配置会复制到当前 `ax_shell` 配置目录，旧目录不会被删除。

## 参与和反馈

请通过 [GitHub Issues](https://github.com/xinalbert/axshell/issues) 提交问题和功能建议。准备代码或发布改动前，请先阅读[开发与打包](docs/development.zh.md)。

## 许可证

本项目采用 [GPL-3.0-or-later](LICENSE)。
