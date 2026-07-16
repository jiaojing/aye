---
name: cross-review
description: '多 agent 交叉审查协议(Phase 2 横向协作)。触发关键词:"交叉 review / ping-pong / round review / cc 和你互审 / codex review round / 多 agent 审查 / review.md 索引 / reviews 目录 / 下一轮 review"。维护 review.md 当前真相索引和 reviews/NNN-*.md 历史轮次；每轮只处理 open items，具体设计判据使用 review。'
---

# Cross Review

管理多 agent review 的状态和文件协议；`review` 负责 finding 判据，本 skill 负责不丢状态、不重复读取全历史。

## 文件布局

```text
docs/features/<slug>/
  feature.md / design.md      # source of truth
  review.md                   # 当前决策和 open items
  reviews/
    001-codex.md              # append-only 历史轮次
    002-cc-response.md
```

## review.md

```markdown
# Review Index: <topic>

Last updated: <date> (after Round N <agent>)

## Decision Log
| ID | Decision | Source |

## Open Items
| ID | Item | Status |

## Latest Round Summary
<最多 10 bullets>

## History
| Round | File | Source | Date |
```

- Decision Log 只放已拍板结论。
- Open Items 只放未关闭事项，每轮必须更新状态。
- 历史 round 文件 append-only，不回改旧轮次。

## 每轮流程

1. 读取 source of truth、`review.md` 和最新 response；只有 open item 需要追溯时才读对应历史文件。
2. 取当前最大编号 +1，写 `reviews/<NNN>-<agent>[-response].md`。
3. 同步更新 review.md 的更新时间、Decision Log、Open Items、最新摘要和 History。
4. 只有已拍板且会改变 scope/AC/tasks 的事项才写回 feature/design；未拍板内容只留在 Open Items。

Round 文件：

```markdown
# Round N — <Agent> Review

Reviewer: <Agent>
Date: <date>
Inputs: <files>

## Verdict
<能否关闭 open items / 是否有 blocker>

## Findings / Responses
<按 blocker 和 open item 顺序>

## Decision Updates
<准备写入 Decision Log 的结论>

## Open Items
<留给下一轮的事项>
```

Finding 必须给具体位置、风险、最小修复，以及是否进入 Decision Log/Open Items；不要写“可以优化”。

## Token 纪律

- 默认只读 source、review.md 和最新一轮。
- 不默认扫描整个 reviews/。
- Decision 一句话，Latest Round Summary 最多 10 bullets。
- Open Items 清空时明确写“可进入下一步”，不要继续制造轮次。

## 对用户输出

只报告新增 round 路径、review.md/feature.md 是否更新，以及剩余 open items；不把整份 review 再贴一遍。
