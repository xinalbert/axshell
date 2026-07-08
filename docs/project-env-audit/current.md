# 项目施工前预检

## 项目边界

- 类型：独立项目
- 根目录：`<repo-root>`
- 结论：当前为独立 Rust / GPUI 桌面应用中的 GitHub Release workflow 小范围修复；本轮目标是在已有 commit 链接修复基础上进一步收敛 highlights 发布正文格式，减少误分组和工程化说明

## 环境记忆目录

- 目录：`docs/project-env-audit/`
- current.md：存在
- changes.md：存在
- 处理动作：读取现有记录并刷新为“release highlights 格式收敛”任务语境

## 运行环境

- 主技术栈：Rust 桌面应用，GitHub Actions Release workflow，Bash 发布辅助脚本段
- 版本约束：`rust-version = 1.88.0`
- 包管理器：`cargo`
- 构建 / 运行入口：`.github/workflows/release.yml` 中 `cargo build --release --target ...`
- 本轮代码入口：`.github/workflows/release.yml` 的 `Generate release highlights` step
- 发布说明依据：publish job 通过 `git log` 读取 tag range 内 commit，按关键词生成 `release/body.md`，再由 `softprops/action-gh-release@v3` 使用 `body_path` 和 `generate_release_notes: true` 发布；本轮只调整 highlights 文本结构和匹配规则
- 依赖统一策略：本轮不新增依赖，不调整 `Cargo.toml` / `Cargo.lock`
- 证据文件：`.github/workflows/release.yml`，`scripts/release_version.py`，`docs/project-implementation-tracker/project-map.md`

## 测试环境

- 测试框架：workflow YAML / shell 静态检查、tracking docs validator
- 默认测试命令：`cargo test`
- 当前实施验证命令：release workflow YAML 静态检查，`Generate release highlights` shell 片段静态检查，tracking docs validator
- CI 测试命令：`.github/workflows/ci.yml` 当前执行 `cargo build --release --target ...`
- 外部依赖：本轮不需要联网或外部服务；真实 GitHub Release 页面渲染需下次 tag 发布时确认
- 工具可用性：本机已可读取 workflow 和脚本；Rust 编译链路本轮不涉及
- 证据文件：`.github/workflows/release.yml`，`.github/workflows/ci.yml`
- 本轮验证结果：workflow YAML 解析通过；`Generate release highlights` Bash 静态检查通过；`git diff --check` 通过；本地样例生成确认 `Full changelog`、句尾 commit 链接、SFTP 独立分组和关键词误匹配修正生效；tracking docs validator 通过

## 环境变化检查

- 是否发现变化：是
- 变化摘要：本轮任务延续 GitHub Release workflow 发布说明展示修复，重点从 commit 链接可达性推进到 release body 格式和分组规则；运行环境和依赖不变
- 受影响文件：`.github/workflows/release.yml`，`docs/project-env-audit/current.md`，`docs/project-env-audit/changes.md`，`docs/project-implementation-tracker/current.md`，`docs/project-implementation-tracker/changes/2026/07.md`
- 是否需要更新 `current.md` / `changes.md`：是

## 开工判定

- 状态：允许开工
- 原因：问题限定在现有 release highlights Bash 生成逻辑内，可通过 Markdown 输出结构、compare 链接和关键词边界规则修复，不需要调整构建、打包或版本解析链路
- 开工前动作：已复查 `.github/workflows/release.yml` 的 publish job、`Generate release highlights` step、项目地图和当前环境记录；已确认不需要联网、不使用多 agent
