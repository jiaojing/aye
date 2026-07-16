---
name: feature
description: '决定要做后的模糊需求结构化(Phase 1 闸门 0)。触发关键词:"加 X 功能 / 想做 Y / 优化下 Z / 整理一下 W / 新需求 / 新 feature / 模糊需求 / 还没明确"。通过对话产出 feature.md(problem + users + scope + constraints + acceptance + tasks + notes)；未决定是否做时先用 spark，raw idea 暂存用 inbox。完成后进入 scope。'
---

# Feature

在进入 PR scope 前，把已经决定要做的模糊需求结构化成一份可 handoff 的 `feature.md`。它回答 **what / why**，不回答技术 **how**。

## 何时跳过

- 还没决定做不做：走 `spark`。
- 只想暂存 raw idea：走 `inbox`。
- bug fix、typo、局部重命名、已有 feature.md 的明确小任务：直接进入对应实施流程。

## Feature.md 模板

```markdown
# <feature 名：动宾短语>

## Feature
<当前状态、真实问题、影响；不掺实现方案>

### Users
<谁使用或受影响>

### Scope
**In**
- <本 feature 要交付什么>

**Out**
- <真正可能跑偏、但本 feature 明确不做的方向>

### Constraints
<技术、业务、时间、兼容性等不可变约束>

### Open questions
- <只列影响 scope / acceptance / 方案的真实未知项；没有则省略>

## Acceptance
- [ ] AC-1: <用户视角可验证事实>

## Tasks
- [ ] T1: <一个可独立验收和 review 的 PR 级交付>

## Notes
<关键讨论和短期决策；长期 ADR 可放 decisions.md>
```

## 文件落地

每个 feature 使用单一目录形态，路径稳定为：

```text
docs/features/<YYYY-MM>-<kebab-slug>/
  feature.md       # 必须
  design.md        # 可选：技术方案
  decisions.md     # 可选：长期 ADR
  notes.md         # 可选：长讨论
```

默认只创建 `feature.md`；需要时再增加辅助文件。

## 判断标准

### Problem 不写 Solution

先说明用户或系统目前为什么受阻。技术选型、类型签名、索引和框架属于 design/implementation。

### Out 只写真实漂移方向

不要留空掩盖已知 non-goal，也不要为了凑数量制造无关排除项。

### Open Questions 不制造问题

只问会改变 scope、acceptance 或方案的事项。按 `pick` 协议提供真实候选并记录用户选择；没有未知项就省略。

### 一个 Feature 的边界

以下信号支持拆分：

- 两部分能独立验收、独立回滚；
- acceptance 自然分成不同用户/子系统；
- 一个 session/PR 无法形成可 review 的 mental scope。

不要按文件、前后端或技术层机械拆分同一用户交付。

### Task 粒度

- 一 task = 一个可独立验收、可独立 review 的 PR 级交付。
- task 内不应残留新的方向性决策；有则先补 feature/design。
- 同模板、同验证标准、同一交付的机械工作合并，避免重复跑 gate。
- commit 数量由实现自然决定，不拿 commit 数量切 task。

状态：`[ ]` 待办、`[>]` 进行中、`[x] ... (done @ <sha>)` 完成。

## 执行

1. 阅读代码库和现有文档，提出完整草稿并显式列出影响判断的 assumptions。
2. 等用户校正 problem、scope、acceptance 和真实 open questions；不要替用户拍板。
3. 用户确认后写入 `docs/features/<slug>/feature.md`，再进入 `scope`。

短信号如“OK”只确认已经摆出的内容，不自动回答仍待拍板的问题。

## 禁止

- 把实现方案塞进 Acceptance。
- 把 feature 写成 design doc。
- 用文件级动作充当 task。
- feature 草稿落地后绕过 scope 直接改源码。

## 下一步

完成并落地 feature 后进入 `scope`；用户明确要求只存档时停止，不自动实施。
