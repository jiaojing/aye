---
name: flow
description: '工作流地图(导航层)。触发关键词:"该做什么 / 怎么开始 / 整体流程 / 哪个 skill / 现在到哪一步 / 不知如何展开"。新 session 先读取最新 docs/features/handoff-*.md，并只报告 docs/inbox.md 条目数量；随后展示 pre-feature、feature 和 task/PR 三阶段以及 16 个 skill 的位置。'
---

# Flow

为单人 + AI 配对编程提供导航。它只画地图，不替代具体 gate 或 reference skill。

## 新 Session 起手

flow 触发时先做：

1. 查找 `docs/features/handoff-*.md`，读取日期最新的一份并报告接力点。
2. 若 `docs/inbox.md` 存在，只统计 `## Inbox` 下的 bullet 数量并报告；用户要求查看时才展开。
3. 两者都不存在则按仓库现状直接展示地图。

不要用整个文件行数冒充 inbox 条目数，也不要跳过已有 handoff 直接让用户从零回忆。

## 工作流

```text
Phase 0（可选，未承诺）
  raw idea ─┬→ inbox：只 capture
            ├→ spark：展开判断
            └→ feature：已经决定做

Phase 1（承诺）
  feature → feature.md（what / why / scope / AC / tasks）

Phase 2（围绕一个可 review 交付）
  scope → acceptance? → design? → implementation ↔ review
        → tests + DoD → commit-gate → push → feature task/AC 打勾

用户主动说“暂停 / 今天到这 / handoff”
  → handoff（独立 skill，不由 commit-gate 自动触发）
```

`?` 表示按条件触发，不是每个小任务都跑完整仪式。

## Chain

- `inbox` → `spark`（展开但未承诺）或 `feature`（决定要做）。
- `spark` → `feature`、回 inbox 或停止。
- `feature` → `scope`。
- `scope` → 命中条件时 `acceptance`，否则实施。
- `acceptance` → 命中条件时 `design`，否则实施。
- `design` → 用户确认后实施。
- 实施中按需 `review` / `cross-review`；DoD 通过后进 `commit-gate`。
- `commit-gate` 只负责 diff review、commit/push 授权和 feature 回写。
- `handoff` 仅由用户主动触发；它不自动 capture inbox。

## Skill 分层

| 类型 | Skill | 职责 |
|---|---|---|
| Gate | `feature` | 模糊需求 → 承诺态 feature.md |
| Gate | `scope` | 锁定本次 PR 的 in/out |
| Gate | `acceptance` | 生成可执行 DoD |
| Gate | `commit-gate` | diff review 后再 commit/push |
| Conditional | `inbox` | raw idea capture |
| Conditional | `spark` | 未承诺的探索 |
| Conditional | `design` | 高破面技术方案 |
| Conditional | `handoff` | 用户主动结束/暂停时交接 |
| Conditional | `cross-review` | 多 agent 轮次审查协议 |
| Conditional | `pua` | 用户主动要求专家 research |
| Conditional | `pick` | 用户主动要求决策工具化 |
| Reference | `principles` | 工程哲学和跨语言边界 |
| Reference | `review` | 五维设计判断 |
| Reference | `rust-principles` | Rust 特化原则 |
| Reference | `kotlin-principles` | Kotlin 特化原则 |
| Navigation | `flow` | 当前地图 |

## Pull 规则

- Phase 0 可完全跳过；想法成熟时直接进入 feature。
- 一个 feature 可跨 session；每次只拉一个 task 进入 Phase 2。
- task 是可独立验收、可独立 review 的 PR 级交付，不绑定固定 commit 数。
- 简单修改按各 gate 的 skip 条件缩短链路。
- 工作流不维护 sprint、velocity 或 prospective epic；项目可自行增加优先级视图。

具体文件 schema 由 `inbox`、`spark`、`feature`、`design` 和 `handoff` 各自维护，flow 不重复定义。
