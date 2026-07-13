[English](local-data-troubleshooting.md) · [文档导航](../README.zh.md)

# 本地数据与故障排查

## 配置文件

已保存会话和应用偏好写入平台配置目录。在常见 Unix 类系统中，主要会话文件为：

```text
~/.config/ax_shell/sessions.json
```

自定义主题保存在相邻的 `themes/` 目录。

当前配置不存在时，AxShell 可以从旧 `ax_ashell` 目录复制数据。迁移不会删除旧文件。

## 日志和崩溃报告

运行日志写入：

```text
~/.config/ax_shell/log
```

崩溃报告写入：

```text
~/.config/ax_shell/crash/ax_shell-crash-*.log
```

不同平台的配置根目录可能不同，可以优先使用设置页中的日志目录操作入口。

## 反馈问题

1. 使用最小受影响工作流复现问题。
2. 记录平台、AxShell 版本，以及问题发生在本地终端还是 SSH 会话。
3. 存在崩溃报告时一并提供。
4. 附上最近的相关运行日志，并先移除主机、用户名、路径和凭据信息。
5. 在 <https://github.com/xinalbert/axshell/issues> 创建 issue。

构建失败还应提供执行命令和编译器输出。开发日志说明见[开发与打包](../development.zh.md)。

<!-- 截图目标：../images/features/local-data-log-settings.png -->
