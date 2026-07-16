---
name: inbox
description: 'feature 上游的可选 capture 层。触发关键词:"记一下 / 先存着 / 以后做 / 想到个事 / inbox / 散落想法 / 灵感 / 暂存 / 候选"。维护 docs/inbox.md 的未结构化、未承诺 raw idea；可由用户主动展开成 spark、提取成 feature，或把已完成 feature 回顾性聚合成 epic。'
---

# Inbox

保存未结构化、未承诺的 raw idea。Inbox 不维护 acceptance、tasks、priority、deadline 或 active focus。

```text
inbox（未结构化、未承诺）
  ├─ explore → spark（已展开、未承诺）
  └─ commit  → feature（已结构化、已承诺）
完成后的多个 feature ──用户主动回顾→ epic
```

想法成熟时可跳过 inbox，直接进入 `spark` 或 `feature`。

## Schema

```markdown
# Inbox

## Inbox

- <一句话想法> [#optional-tag]

## Epics

- [<主题>](epic-<slug>.md) — <完成时间/摘要>
```

只维护一个纯 bullet 的 `## Inbox` 段。已处理条目直接删除，不留 status 或 strikethrough；trail 在 git 和下游文档中。

## 动作

| 用户意图 | 动作 | Inbox 原条目 |
|---|---|---|
| “记一下 X” | 追加 `- X [#tag]`；文件不存在则创建 | 保留 |
| “展开这条” | 调用 `spark`，生成 proposal | 保留，直到决定做或删除 |
| “做这条” | 调用 `feature`；feature.md 落地后删除原条目 | 删除 |
| “删掉这条” | 直接删除 | 删除 |
| “把这些完成项归成 epic” | 用户确认主题后聚合已完成 feature | 不影响 raw inbox |

Capture 完立即回原对话，不主动推动用户承诺。

## Epic 规则

- Epic 永远是 retrospective：从已完成 feature 总结共同主题，不预先规划未来 feature。
- 永远由用户主动触发，不按 feature 数量提示或自动创建。
- 保留源 feature.md；epic 是聚合视图，不是替代品。
- 写 `docs/epic-<slug>.md`，并在 `## Epics` 增加索引。

## 与 Handoff 的边界

`handoff` 只记录当前事实，不自动扫描或引导 inbox capture。用户想到散落事项时主动说“记一下”。

`flow` 可在新 session 只报告 inbox 条目数量；用户要求查看时才展开内容。

## 禁止

- 在 inbox 条目下写 acceptance、tasks、status、priority 或时间承诺。
- 维护 `Active` 段；当前焦点由 handoff/feature 状态表达。
- extract 后保留划线历史。
- 根据数量自动创建 epic。
