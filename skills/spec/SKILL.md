---
description: 模糊需求结构化(Phase 1 闸门 0)。触发关键词:"加 X 功能 / 想做 Y / 优化下 Z / 整理一下 W / 新需求 / 新 feature / 新 epic / 这个怎么搞"。引导写 spec(problem / users / scope / acceptance / non-goals / open-questions),用户点头才进下一步。Auto-invoke: kanban(若 backlog 多 story 排队,待新增)或 scope(直接进 PR 级)。
---

# Spec

AI 协作的**闸门 0**:进 scope 之前,先把模糊需求结构化成 spec。

不写 spec 直接进 scope,是在"我以为你要的是 X"的错误前提上对齐 PR scope。**前提错了,后面再严的闸门也救不回来**。

---

## 为什么需要

5 条 AI 协作硬约束推导:

- AI **没持久记忆** → spec 是 self-describing 工件,新 session 拿来就能用
- AI **理解会偏** → spec 是显式契约,避免"我以为你要的是 X"
- AI **容易越界** → spec 钉死 non-goals,scope 才有所本
- AI **PR ≈ context 容量** → spec 帮人判断"这能一个 PR 装下吗",不能就拆 epic

`flow` 把"Epic / Story 拆分"曾列为"人判断不需 skill"——错了。这恰好是 AI 协作最需要 skill 把守的位置。

---

## 触发场景

- 用户带模糊需求来("加个搜索功能" / "优化下登录" / "整理一下 X")
- 新 epic / feature 启动
- 改公开 API / 持久化数据 / 跨多个 PR 的工作
- "这听起来不止一个 PR 能搞定"
- 多种合理解读冲突,不知道用户真要哪个

**不触发**:
- bug fix(走 debugging,不需要写 spec)
- typo / 局部重命名 / 一行修改
- 已经有 spec 的小改(直接 scope)
- 明确的小任务("把 readme 里这句改了")

---

## Spec 模板

```markdown
## Problem
<一段话:当前是什么状态?为什么有问题?谁受影响?>
<不能掺 solution——讲 problem 不讲怎么改>

## Users
<谁会用这个能力 / 受这次改动影响?>

## Scope
### In
- <这次要做什么>
- ...
### Out
- <这次明确不做什么——比 In 更重要,钉死 non-goals 防 AI 越界>
- ...

## Acceptance
- AC-1: <用户视角能验证的事实>
- AC-2: ...
（细化到带 command 的可执行 DoD,由 acceptance skill 在动手前做）

## Constraints
<已知技术 / 业务 / 时间 / 兼容性约束>

## Open questions
<我不知道的、需要用户回答的——必须列,不许空>
- Q1: ...
- Q2: ...
```

---

## Spec 三要素

### 1. Problem ≠ solution

❌ "添加 search bar 功能"——这是 solution
✅ "用户找不到自己上传的旧文档,只能靠浏览,平均 30s 才找到"——这是 problem

写 spec 第一题:**能不能把 problem 单独讲清楚,完全不提 solution?**
讲不清楚 = 还没想清楚,不要往下走。

### 2. Out-of-scope 比 in-scope 更重要

In-scope 用户自己能想出来。**Out-of-scope 是 AI 容易跑偏的地方**——必须显式钉死。

例(给"用户能搜自己的文档"这个 spec):

```
Out-of-scope:
- 不做全文索引(LIKE 查询即可,数据量 < 10k 文档)
- 不做 fuzzy match
- 不做搜索建议 / autocomplete
- 不做权限过滤(已有 ACL 层负责)
- 不做跨用户搜索
```

每条都是**真有可能跑偏的方向**,不是形式主义凑数。

### 3. Open questions 不能空

AI 写 spec 容易把"我不知道的"自己拍掉,变成"我假设 X"。**Open questions 强制 AI 显式列出不确定项**,等用户回答。

如果你写不出任何 open question → 八成是没认真想过,再过一遍。

**反例**:Open questions 下面立刻写"我倾向 X"——这是 sycophancy。Open questions 是**等用户回答**的,不是"我帮你想了答案"。

---

## 仪式三步

### 1. AI 提议 spec

按上面模板输出**完整 spec**,然后等。**不动手实施,不开 scope**。

### 2. 用户 review / 纠偏

可能:
- ✅ 整体 ok → 进 scope
- 🟡 调整某节(改 problem / 改 scope / 增减 acceptance)→ AI 改后再确认
- ❌ 偏了 → 重提

**Open questions 必须等用户回答**,不许自己拍。

### 3. spec 落地 → 进 scope

spec 写到合适位置(项目 backlog / PR description / 对话内任务条),作为后续 trail。然后才进 scope("基于这个 spec,本次 PR 改哪些文件")。

---

## 反模式

### 反模式 1:Spec 偷偷塞 implementation

```
❌
Acceptance:
  - 用 PostgreSQL FTS 实现搜索
  - 加 trigram index 加速
```

"用 PostgreSQL FTS"是 implementation,不是 acceptance。Acceptance 写**用户视角能验证的事实**:

```
✅
Acceptance:
  - AC-1: 用户在搜索框输入关键词,< 200ms 内看到匹配结果
  - AC-2: 没匹配时显示"无结果",不报错
```

技术选型留给实施阶段(scope / design-review)。

### 反模式 2:Out-of-scope 空白

```
❌ Out-of-scope: (略)
```

等于没写。强制列 3-5 条**真有可能跑偏的方向**,这是 spec 的安全带。

### 反模式 3:Open questions 自己拍

```
❌ Open questions:
   - Q1: 是否要 fuzzy match? 我倾向不做。
```

引导式提问,用户容易"嗯,你看着办"附和。应该:**陈述疑问,等用户答**。

### 反模式 4:Spec = design doc

Spec 讲 **what + why**,design doc 讲 **how**。混在一起 → spec 变成 200 行设计稿,失去"快速对齐"的功能。

**Spec ≤ 1 页**。超出就该拆 epic。

### 反模式 5:Spec 做完直接动手

```
❌ "spec 写完了,我开始改 src/search.rs..."
```

Spec 只解决"做什么对吗",不解决"这次 PR 改哪些文件"。spec 之后**还要走 scope**(改文件清单)+ **acceptance**(可执行 DoD)。三道仪式不重叠,不替代。

---

## 与其他 skill 的关系

- **`flow`**:地图。本 skill 是流程入口的**闸门 0**(在所有实施仪式之前)。
- **`scope`**:闸门 1。spec 决定"做什么对吗",scope 决定"这次 PR 改哪些文件"——颗粒比 spec 小一档。先 spec 后 scope。
- **`acceptance`**:闸门 2。spec 里 acceptance criteria 是用户视角(较粗),acceptance skill 把它做成带 command 的可执行 checklist。两者不重复。
- **`design-review`**:spec 阶段如果涉及大破面 / 多方案纠结(技术选型),可同步触发 design-review 做 5 维度判据。
- **`principles`**:写 spec 时回去找判据(实用主义 / 不破坏 consumer / 简洁)——尤其在 out-of-scope 的取舍上。

---

## 输出格式建议

spec 给用户时,**等待提示明确**:

```markdown
[完整 spec 如上]

---

我**不进 scope**直到收到指示。请 review 后:
- "ok" → 我进 scope(本次 PR 改文件清单)
- "改 problem/scope/acceptance: ..." → 我改后再确认
- "Open questions 我答了: Q1 ..., Q2 ..." → 我合入 spec 再确认
- "重来" → 重新提议
```

---

## 一句话总结

**Spec 是闸门 0:模糊需求 → problem + users + scope + acceptance + non-goals + open questions → 用户点头 → 才进 scope**。

它不是 design doc,不讲 how;它是 AI 协作的"做什么对吗"契约,前提对了,后面三道闸门才有意义。

---

## Auto-invoke chain

完成本 skill 后,LLM 自动 invoke 下一步:

- **多 story 排队** → `kanban`(看板维护,选下一条 story)— *待新增,当前先手动维护 kanban.md*
- **直接进 PR 级实施** → `scope`

如用户说"先不动,放着",则不 chain。
