# 当前项目实施记录

## 当前目标

- 目标：把根 README 收敛为英文默认入口和中文切换页，在 `docs/` 建立双语导航，并把用户功能说明拆成便于后续补图的独立 Markdown 页面。
- 交付物：`README.md` / `README.zh.md`、`docs/README.md` / `docs/README.zh.md`、双语功能文档、截图目录约定、精简兼容用户指南、更新后的项目地图和链接验证。

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`README.md`，`README.en.md`，`README.zh.md`，`docs/README*.md`，`docs/user-guide*.md`，`docs/features/`，`docs/images/`，`docs/development*.md`，跟踪文档。
- 不在本轮范围内：修改应用功能、Rust 源码、配置 schema、依赖、manifest/lock、发布 workflow 或生成实际产品截图。

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | README skill、现有双语文档、项目地图和环境预检 | 工作树、现有链接、文档标题和产品证据复核 | 用户指定默认英文 README |
| P2 | completed | 根 README 双语入口与 `docs/` 双语总导航 | 结构对齐、语言互链、相对链接 | 根 README 只保留概览和快速开始 |
| P3 | completed | terminal/SSH、workspace、SFTP、settings、sync、proxy/X11、monitoring 等双语功能页 | 功能覆盖和交叉链接检查 | 每页预留截图位置说明 |
| P4 | completed | 精简兼容用户指南、截图目录说明和项目地图刷新 | 旧入口可继续导航到新页面 | 不保留重复长文 |
| P5 | completed | Markdown-only 最终验证和实施记录收口 | 链接检查、`git diff --check`、tracking validator | 不运行无关 Rust 测试 |

## 已完成

- 已读取 README maintenance skill 及参考规范，确认根 README 应保持简短并把详细说明移入 `docs/`。
- 已确认项目现有语言约定是中文 `README.md`、英文 `README.en.md`，用户要求改为英文 `README.md`、中文 `README.zh.md`。
- 已确认现有 `docs/user-guide*.md` 聚合了终端、SSH、SFTP、工作区、设置、同步、代理、X11、监控和日志内容，适合按功能拆分。
- 已确认 `preview.png` 可继续作为根 README 首屏预览；后续功能截图应放入 `docs/images/`，不创建缺失图片链接。
- 已完成施工前环境预检；无需联网、多 agent、依赖或 Rust 改动。
- 已将英文内容写入默认 `README.md`，新增中文 `README.zh.md`，并删除旧 `README.en.md` 入口；两个根页面结构对齐并在顶部互链。
- 已新增 `docs/README.md` 和 `docs/README.zh.md`，集中导航快速入门、功能指南、开发文档、资源生命周期和截图说明。
- 已新增双语快速入门及 8 组独立功能页，覆盖 terminal/SSH、workspace、SFTP、appearance/settings、sync、proxy/X11、monitoring/lifecycle 和本地数据/故障排查。
- 已将原单篇 `user-guide` 收敛为兼容索引，并修复中英文开发文档回链。
- 已新增 `docs/images/` 截图规范和 `docs/images/features/` 实际目录说明；功能页使用 HTML 注释预留图片目标，不产生破损图片。
- 已更新项目地图中的根 README、docs 导航、功能页和截图目录职责，并把常用定位命令切换到 `README.zh.md`。
- 已检查 31 个用户可见 Markdown 文件的相对链接目标，全部存在；8 组功能页中英文配对完整，活动文档不再引用删除的 `README.en.md` 或 `user-guide.en.md`。
- 已完成 `git diff --check` 和 tracking docs validator。

## 验证

- 已完成：工作树、README、双语用户/开发文档、项目地图、manifest/release 证据复核；31 个用户可见 Markdown 文件链接检查；功能页语言配对；旧活动链接检索；`git diff --check` 和 tracking docs validator。
- 未完成：实际功能截图尚未提供，因此仅完成路径和插入位置准备；本轮无 Rust 改动，未运行无关编译测试。

## 风险与阻塞

- 剩余风险一：外部网站如果直接链接旧 `README.en.md` 或 `docs/user-guide.en.md`，仓库内无法自动改写；仓库活动文档已全部切换到新入口。
- 剩余风险二：后续添加截图时需要按 `docs/images/README*.md` 清理敏感信息，并把功能页注释替换为真实图片链接。
- 无阻塞。

## 下一步

- 后续可按功能页逐步加入截图；README 与 docs 结构维护已完成。

## 最后更新时间

- 2026-07-11 07:03 +0800
