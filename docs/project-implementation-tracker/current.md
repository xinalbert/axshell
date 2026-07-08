# 当前项目实施记录

## 当前目标

- 目标：进一步收敛 GitHub Release highlights 的发布正文格式，减少误分组和工程化说明，让发布页更适合用户阅读
- 交付物：更新后的 `.github/workflows/release.yml` highlights 分组、关键词匹配、commit 链接展示格式，以及同步后的实施记录和环境记录

## 项目边界

- 根目录：`<repo-root>`
- 当前范围：`.github/workflows/release.yml`，`docs/project-env-audit/`，`docs/project-implementation-tracker/`
- 不在本轮范围内：构建矩阵、产物打包、tag/version 解析、Homebrew cask 发布、Rust 应用源码

## 当前状态

- 阶段：已完成
- 开工判定：允许开工
- 是否需要联网：否
- 多 agent：未使用

## 活动计划

| Step | Status | Deliverable | Verification | Notes |
| --- | --- | --- | --- | --- |
| P1 | completed | 复查当前 release highlights 格式和真实提交模拟输出 | 源码检查，本地模拟输出检查 | 项目地图已覆盖本轮范围 |
| P2 | completed | 调整 release body 为用户优先格式，修正关键词误匹配和分组标题 | workflow shell 静态检查，本地样例生成检查 | 保留 `generate_release_notes: true` |
| P3 | completed | 收口验证并刷新跟踪文档 | workflow YAML 静态检查，tracking docs validator | 真实 GitHub Release 页面需下次 tag 发布时确认 |

## 已完成

- 已读取 `project-map.md`、当前实施记录和环境记录，确认本轮仍只涉及 `.github/workflows/release.yml` 的 release highlights 生成段
- 已用当前提交历史模拟 release body，发现 `mouse release` 会误入 `Release and packaging`、`Windows dev-reload` 会因 `window` 子串误入 UI、SFTP 内容集中进入 `SSH and compatibility`
- 已确认发布正文中 `_Automatically selected..._` 偏工程化，适合改为 `Full changelog` 链接
- 已把 release body 顶部说明改为 `Full changelog` compare 链接
- 已把 commit 链接移到条目末尾，格式为 `Subject ([#短hash](commit URL))`
- 已增加关键词边界规则，避免 `mouse release` 误入 Packaging、`Windows` 误入 UI
- 已将 SFTP 独立成 `SFTP` 分组，并让 UI 分组优先匹配菜单/可见性等界面类 commit
- 已扩展 tracking commit 过滤规则，减少维护记录进入发布正文
- 已确认本轮不需要联网、不使用多 agent、不新增依赖

## 验证

- 已完成：源码检查和本地模拟输出检查
- 已完成：`.github/workflows/release.yml` YAML 解析通过
- 已完成：`Generate release highlights` Bash 静态检查通过
- 已完成：`git diff --check` 通过
- 已完成：本地样例生成通过，确认 `Full changelog`、句尾 commit 链接、SFTP 独立分组和关键词误匹配修正生效
- 已完成：tracking docs validator 通过
- 未完成：真实 tag 发布页面确认

## 风险与阻塞

- 阻塞：无
- 风险一：GitHub Release 页面最终渲染效果只能在实际 tag 发布后确认；本轮通过本地模拟生成检查降低风险
- 风险二：关键词规则仍是启发式选择，不会等同人工 changelog；GitHub 自动生成 release notes 仍由 `generate_release_notes: true` 追加

## 下一步

- 下次 tag 发布后检查 GitHub Release 页面：Highlights 应显示 `Full changelog`、按用户可读分组列出条目，且正文不出现 Keyword Rules

## 最后更新时间

- 2026-07-09 07:42 +0800
