---
name: spark
description: 'Feature 之前的可选想法探索(Phase 0.5)。触发关键词:"spark / 展开想想 / 先 brainstorm / 先聊清楚 / 这个值不值得做 / 先出 proposal / 先别写代码 / 还没决定做不做"。把 raw idea 或 inbox 条目展开成 proposal，帮助决定做、停、存或放弃；不承诺、不进入 scope、不写代码。'
---

# Spark

探索一个尚未承诺的想法，回答“值不值得做、是否现在做、做成哪种形态”。`inbox` 只 capture，`feature` 表示已经决定做，`design` 处理承诺后的技术 how。

## 产物

默认写 `docs/sparks/<YYYY-MM-DD>-<slug>.md`；用户明确“只聊不落文件”时例外。

```markdown
# Spark: <idea>

## Source
<来源>

## Question
<本次探索要回答的核心问题>

## Context
<只写影响判断的事实>

## Options
### Option A: <真实可行形态>
- Shape:
- Upside:
- Cost/Risk:
- Unknowns:

### Option B: <存在第二个真实候选时才写>
...

## Recommendation
<推荐、依据、为何不选其他真实方向>

## Decision
Status: exploring | ready-for-feature | parked | dropped
Next: stop | feature | inbox | drop

## Open questions
<只列会改变判断的真实未知项；没有则省略>
```

“不做/暂存”可以是一个真实候选；不要为了满足模板把一个方案拆成两个名字。

## 执行

1. 先锁定一个探索问题；缺阻塞信息时按 `pick` 协议只问最关键的决策。
2. 比较真实产品/流程形态，不深入函数签名和实现细节。
3. 写 proposal，给推荐和状态，然后停止。

不得自动进入 feature、scope 或编码。只有用户明确“做这个”才调用 `feature`；“先存着”进入/保留 `inbox`；删除文件或条目必须由用户明确要求。

## 自检

- 没有 acceptance、tasks、deadline 等承诺态内容。
- 候选真实，Recommendation 有证据和 trade-off。
- Decision status 明确，默认不替用户设为 ready-for-feature。
- 来自 inbox 时未擅自删除原条目。
- 已停在 proposal。
