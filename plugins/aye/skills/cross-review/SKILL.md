---
name: cross-review
description: '多 agent 交叉审查协议(Phase 2 横向协作)。触发关键词:"交叉 review / ping-pong / round review / cc 和你互审 / codex review round / 多 agent 审查 / review.md 索引 / reviews 目录 / 下一轮 review"。用于 feature.md / design.md / PR 方案需要 CC/Codex/Claude 多轮互审时,维护 `review.md` 当前真相索引 + `reviews/NNN-*.md` 历史轮次,控制 token,每轮只关 open item。审查判据仍用 `review`,本 skill 管文件协议和状态收敛。'
---

# Cross Review

多 agent ping-pong review 的文件协议。它解决的不是"怎么看设计",而是"多轮互审怎么不丢状态、不爆 token、不让后来者读完整聊天记录"。

**边界**:
- `review` = 审查判据(类型抽象 / Builder / Trait / 命名 / 重构)
- `cross-review` = 多轮审查的状态管理与文件协议

两者可叠加:本 skill 管轮次文件,具体 finding 可引用 `review` 的判据。

---

## 触发场景

- 用户说"让 CC 和你 ping-pong 一下 / 交叉 review / round review / 下一轮 review"
- 已有 `feature.md` / `design.md`,需要另一个 agent 回应上一轮 review
- 目录里出现 `review.md` + `reviews/NNN-*.md`
- 用户要求"看下一轮 / 回应上一轮 / 把结论写入 round 文件"

**不触发**:
- 单次代码审查 / 设计审查,没有多 agent 轮次需求 → `review`
- commit 前摆 diff 等用户点头 → `commit-gate`
- feature 还没写清楚 → `feature`

---

## 文件布局

默认落在被审对象同目录:

```text
docs/features/<slug>/
  feature.md                 # 被审对象 / source of truth
  review.md                  # 当前真相索引,每轮更新
  reviews/
    001-codex.md             # 历史轮次,append-only
    002-cc-response.md
    003-codex.md
```

如果被审对象是 `design.md`,仍放同目录。不要在项目根散落 review 文件。

---

## review.md 结构

`review.md` 只放当前真相,不放完整历史:

```markdown
# Review Index: <topic>

Last updated: <date> (after Round N <agent>)

## Decision Log

| # | Decision | Source |
|---|---|---|
| D1 | ... | R1 codex |

## Open Items

| ID | Item | Status |
|---|---|---|
| R2-Q1 | ... | pending CC |

## Latest Round Summary

<5-10 bullets,只摘要最近一轮>

## History

| Round | File | Source | Date |
|---|---|---|---|
| 1 | `reviews/001-codex.md` | Codex | 2026-05-25 |
```

**铁律**:
- `Decision Log` 只放已拍板结论,不要放争论过程。
- `Open Items` 只放未关闭问题,每轮必须更新状态。
- 历史 round 文件 append-only,旧轮次不要改。
- 下一轮默认只读 `feature.md` + `review.md` + 最新 response;需要追溯才读具体文件。

---

## 每轮流程

### 1. 找上下文

1. 定位被审对象:`feature.md` / `design.md` / 用户点名文件。
2. 读同目录 `review.md`(若存在)。
3. 读最新一份 `reviews/NNN-*.md` 或用户点名的 response。
4. **不要默认读全 `reviews/` 历史**。只在 `review.md` 摘要不够或 open item 需要追溯时读具体文件。

### 2. 产出本轮文件

文件名规则:

```text
reviews/<NNN>-<agent>.md
reviews/<NNN>-<agent>-response.md
```

示例:

```text
reviews/005-codex.md
reviews/006-cc-response.md
```

`NNN` = 当前最大编号 + 1,三位补零。`agent` 用 `codex` / `cc` / `claude` / `human`。

本轮文件结构:

```markdown
# Round N — <Agent> Review

Reviewer: <Agent>
Date: <date>
Inputs: `feature.md`, `review.md`, `reviews/00X-...md`

## Verdict

<先给结论:能不能关 open items / 是否还有 blocker>

## Findings / Responses

<按严重度或 open item 顺序>

## Decision Updates

<哪些应进 Decision Log>

## Open Items

<剩什么给下一轮>
```

### 3. 更新 review.md

每轮必须同步更新:

- `Last updated`
- `Decision Log` 新增已拍板项
- `Open Items` 关闭 / 新增 / 改状态
- `Latest Round Summary`
- `History` 新增本轮文件

如果 open item 清空,明确写:

```text
Open Items 已清空。可以进入 <next step>。
```

### 4. 必要时同步 feature.md

只有当某项已经拍板,并且会影响实施 scope / AC / tasks,才 patch `feature.md`。

同步方式:
- 在 `feature.md` 标注引用 Decision ID,如 `(D12)`。
- 不把完整争论搬进 `feature.md`。
- 不偷改未拍板问题;未拍板只进 `Open Items`。

---

## Token 控制规则

- 默认读:`feature.md` + `review.md` + 最新一轮。
- 禁止默认读全历史。
- 如果要追溯,只读被 `Open Items` / `History` 指到的具体文件。
- 每轮 `Latest Round Summary` 控制在 10 bullets 内。
- 每条 Decision 一句话,不要塞论证。

---

## Finding 写法

优先按这个顺序:

1. **Blocking / High Risk**:不解决会实现错或验收不可执行
2. **Design Suggestions**:更稳的设计或边界修正
3. **Acceptance Changes**:验收缺口
4. **Open Questions**:需要下一轮拍板的问题

每条 finding 要有:
- 具体位置 / 具体文案 / 具体代码引用
- 为什么有风险
- 建议怎么改
- 是否需要进入 `Decision Log` 或 `Open Items`

不要写泛泛的"可以优化 / 注意一下"。

---

## 与其他 skill 的关系

- **`review`**:本 skill 的 finding 内容可调用它的 5 维度判据。
- **`design`**:如果交叉 review 的对象是设计方案,`design.md` 仍是方案 source of truth;本 skill 只管 review thread。
- **`feature`**:如果 review 后改变 scope / AC / tasks,同步写回 `feature.md`。
- **`commit-gate`**:大改动 commit 前可以要求 cross-review 已关 open items,但 commit-gate 仍是动 git 闸门。

---

## 完成后输出

对用户只报:

- 新增的 round 文件路径
- `review.md` 是否已更新
- `feature.md` 是否同步改动
- 剩余 open items / 或 "已清空,可进入 X"

不要把整份 review 再贴一遍。

