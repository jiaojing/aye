---
description: 模糊需求结构化(Phase 1 闸门 0)。触发关键词:"加 X 功能 / 想做 Y / 优化下 Z / 整理一下 W / 新需求 / 新 feature / 这个怎么搞"。引导对话补充细节,产出单份 feature.md(承载 problem + users + scope + acceptance + tasks + status + notes)。后续围绕 task 打勾推进。Auto-invoke: scope(进 PR 级实施)。
---

# Feature

AI 协作的**闸门 0**:进 scope 之前,先把模糊需求结构化成 **feature.md**——单份文档承载需求描述 + 验收标准 + task 清单 + 状态跟踪。

不写 feature 直接进 scope,是在"我以为你要的是 X"的错误前提上对齐 PR scope。**前提错了,后面再严的闸门也救不回来**。

---

## 为什么需要

5 条 AI 协作硬约束推导:

- AI **没持久记忆** → feature.md 是 self-describing 工件,新 session 拿来就能用
- AI **理解会偏** → feature.md 是显式契约,避免"我以为你要的是 X"
- AI **容易越界** → feature.md 钉死 non-goals,scope 才有所本
- AI **PR ≈ context 容量** → feature.md 帮人判断"这个 feature 一次能搞定吗",大了拆 task

---

## 触发场景

- 用户带模糊需求来("加个搜索功能" / "优化下登录" / "整理一下 X")
- 新 feature 启动
- 改公开 API / 持久化数据 / 跨多个 PR 的工作
- 多种合理解读冲突,不知道用户真要哪个
- 用户希望对话补充细节后落地成档案

**不触发**:

- bug fix(走 debugging,不需要写 feature)
- typo / 局部重命名 / 一行修改
- 已有 feature.md 的小改(直接 scope)
- 明确小任务("把 readme 里这句改了")

---

## Feature.md 模板

```markdown
# <feature 名:动宾短语,人话>

## Feature
<一段话:当前是什么状态?为什么有问题?谁受影响?>
<不能掺 solution——讲 problem 不讲怎么改>

### Users
<谁会用 / 受这次改动影响?>

### Scope
**In**
- <这次要做什么>
- ...

**Out**
- <这次明确不做什么——比 In 更重要,钉死 non-goals 防 AI 越界>
- ...

### Constraints
<已知技术 / 业务 / 时间 / 兼容性约束>

### Open questions
<必须列,不许空>
- Q1: ...
- Q2: ...

## Acceptance
- [ ] AC-1: <用户视角能验证的事实>
- [ ] AC-2: ...
（细化到带 command 的 checklist 由 acceptance skill 做）

## Tasks
- [ ] T1: <粗到中粒的实施步骤>
- [ ] T2: ...
（推进时打勾:`- [x] T1: xxx (done @ commit-sha)`,进行中:`- [>] T2: xxx`）

## Notes
<对话产生的关键决策 / 待解疑问 / 临时讨论>
（重要决策稳定后,加同目录下 `decisions.md`）
```

---

## 文件落地

**强制约定**:**每个 feature 一个目录,目录内至少一份 `feature.md`**。不区分简单 / 复杂——单一形态,消除特殊情况。

```
docs/features/
  <YYYY-MM>-<kebab-slug>/
    feature.md          # 必须,主文档(problem + acceptance + tasks + status + notes)
    design.md           # 可选,需要技术设计时加(架构图 / 数据流 / 多方案对比)
    decisions.md        # 可选,需要长期留存 ADR 时加
    notes.md            # 可选,对话历史 > 100 行时单独抽
    tasks.md            # 可选,task > 30 条 / 多 sub-feature 时单独抽
```

**为什么强制目录**:

1. **数据结构本质**:feature 是"一组相关文档的集合"——哪怕只有一份,也是"集合 of 一"。目录天然表达集合,文件不能。
2. **消除特殊情况**:不再有"文件 vs 目录"两种形态,工具 / skill 实现零分支判断。
3. **加辅助文档零仪式**:需要 design.md 时直接 `touch <feature>/design.md`,不需要"先升级目录再加文件"。
4. **路径稳定**:feature.md 永远在 `docs/features/<slug>/feature.md`,不变。

**例**:

```
docs/features/
  2026-04-aye-skill-completion/
    feature.md
  2026-05-search-feature/
    feature.md
    design.md            # 涉及索引技术选型
  2026-06-billing-redesign/
    feature.md
    design.md
    decisions.md         # 多个 ADR 长期留存
```

只有一份 feature.md 是最常见的形态,**这不是过度设计——它为后续扩展留了平滑路径**。

---

## Feature 三要素

### 1. Problem ≠ solution

❌ "添加 search bar 功能"——这是 solution
✅ "用户找不到自己上传的旧文档,只能靠浏览,平均 30s 才找到"——这是 problem

写 feature 第一题:**能不能把 problem 单独讲清楚,完全不提 solution?**
讲不清楚 = 还没想清楚,不要往下走。

### 2. Out-of-scope 比 In-scope 更重要

In-scope 用户自己能想出来。**Out-of-scope 是 AI 容易跑偏的地方**——必须显式钉死。

例(给"用户能搜自己的文档"这个 feature):

```
Out:
- 不做全文索引(LIKE 查询即可,数据量 < 10k 文档)
- 不做 fuzzy match
- 不做搜索建议 / autocomplete
- 不做权限过滤(已有 ACL 层负责)
- 不做跨用户搜索
```

每条都是**真有可能跑偏的方向**,不是形式主义凑数。

### 3. Open questions 不能空

AI 写 feature 容易把"我不知道的"自己拍掉,变成"我假设 X"。**Open questions 强制 AI 显式列出不确定项**,等用户回答。

写不出任何 open question → 八成是没认真想过,再过一遍。

**反例**:Open questions 下面立刻写"我倾向 X"——这是 sycophancy。Open questions 是**等用户回答**的,不是"我帮你想了答案"。

---

## 怎么界定"一个 feature"(不是两个)

5 条硬判据:

1. **独立验收**:能写出 acceptance 单独判定"完成"。A 的验收必须假设 B 已存在 → 合并或 B 是前置依赖。
2. **单一 deliverable**:一个 feature = 一个用户/系统可见产出物。"新增 X" + "修 Y 的引用" 如果只为 X 生效,是同一 deliverable,不拆。
3. **PR 边界**:超过 ~500 行 diff、跨 3+ 模块、一个 session 写不完 → 拆。
4. **回滚原子性**:回滚 A 一部分会让另一部分悬空 → 是一个 feature;回滚 A 完全不影响 B → 是两个。
5. **Acceptance 自然分裂**:写 acceptance 时发现"这条针对 X 子系统、那条针对 Y" → 强信号,该拆。

**不该用来拆的反例**:

- ❌ 文件粒度("这个文件改 / 那个文件改")→ 是 task 粒度
- ❌ 前端 / 后端切(同一 deliverable 应该垂直切片)
- ❌ 用户可见 / 内部重构切(重构通常归到 user-facing feature 内)

---

## 仪式三步

### 1. AI 提议初稿 + 列假设

按模板输出**完整 feature.md 草稿**,**显式列 ASSUMPTIONS**:

```
ASSUMPTIONS I'M MAKING:
1. <对需求的假设>
2. <对架构 / 范围的假设>
→ 请校正,否则我按这些前提推进。
```

不动手实施,不开 scope。

### 2. 对话补充细节

用户 review,可能:

- ✅ 整体 ok → 落地 feature.md → 进 scope
- 🟡 调整某节(改 problem / 改 scope / 增减 acceptance / 加 task)→ AI 改后再确认
- ❌ 偏了 → 重提

**Open questions 必须等用户回答**,不许自己拍。

### 3. 落地 feature.md → 进 scope

建目录 `docs/features/<YYYY-MM>-<slug>/`,写到 `feature.md`,作为后续 trail。然后才进 scope("基于这个 feature,本次 task 改哪些文件")。

---

## Tasks 章节怎么写

Tasks 是**粗到中粒的实施步骤**,不是 commit 级 task(commit 级在写代码时即兴拆)。

颗粒判据:每个 task 应该:

- 能在一个 session 内完成
- 对应一个或几个相关 commit
- 有清晰的"完成 = 什么"(心里有数即可,不必写出 acceptance)

例(给"补全 aye 漏掉的 skill"这个 feature):

```markdown
## Tasks
- [x] T1: rename aye:spec → aye:feature, 重写 SKILL.md (done @ abc1234)
- [>] T2: 修 flow/scope/acceptance/handoff/rust-principles 的引用
- [ ] T3: 新增 aye:design-doc skill
- [ ] T4: README 同步
- [ ] T5: dogfood 跑一圈真实 feature 验证
```

状态标记:
- `[ ]` 待办
- `[>]` 进行中
- `[x] ... (done @ <commit-sha>)` 已完成

---

## 反模式

### 反模式 1:feature 偷偷塞 implementation

```
❌
Acceptance:
  - 用 PostgreSQL FTS 实现搜索
  - 加 trigram index 加速
```

"用 PostgreSQL FTS"是 implementation。Acceptance 写**用户视角能验证的事实**:

```
✅
Acceptance:
  - AC-1: 用户在搜索框输入关键词,< 200ms 内看到匹配结果
  - AC-2: 没匹配时显示"无结果",不报错
```

技术选型留给实施阶段(scope / design-review)。

### 反模式 2:Out-of-scope 空白

```
❌ Out: (略)
```

等于没写。强制列 3-5 条**真有可能跑偏的方向**——feature 的安全带。

### 反模式 3:Open questions 自己拍

```
❌ Open questions:
   - Q1: 是否要 fuzzy match? 我倾向不做。
```

引导式提问,用户容易"嗯,你看着办"附和。应该:**陈述疑问,等用户答**。

### 反模式 4:feature.md = design doc

Feature 讲 **what + why + 粗 task**,design doc 讲 **how**(实现细节、架构图、数据流)。混在一起 → 文档膨胀,失去"快速对齐"功能。

需要 design 时同目录下加 `design.md`。

### 反模式 5:Tasks 写成 commit 级

```
❌ Tasks:
   - T1: 改 src/foo.rs 第 30 行
   - T2: 加一行 import
```

太细。Tasks 粒度是"一次小迭代"(一个 session / 几个 commit)。commit 级在写代码时实时切。

### 反模式 6:Feature 写完直接动手

```
❌ "feature 写完了,我开始改 src/search.rs..."
```

Feature 解决"做什么对吗",不解决"这次 task 改哪些文件"。Feature 之后**还要走 scope**(改文件清单)+ **acceptance**(可执行 DoD)。三道仪式不重叠,不替代。

---

## 与其他 skill 的关系

- **`flow`**:地图。本 skill 是流程入口的**闸门 0**(在所有实施仪式之前)。
- **`scope`**:闸门 1。feature 决定"做什么对吗",scope 决定"这次 task 改哪些文件"——颗粒小一档。先 feature 后 scope。
- **`acceptance`**:闸门 2。feature 里 acceptance 是用户视角(较粗),acceptance skill 把它做成带 command 的 checklist。两者不重复。
- **`design-review`**:feature 阶段如涉及大破面 / 多方案纠结(技术选型),可同步触发 design-review 做 5 维度判据。
- **`design-doc`**:大功能添加 `design.md` 时,由 design-doc skill 维护(待新增)。
- **`principles`**:写 feature 时回去找判据——尤其在 out-of-scope 取舍上。

---

## 一句话总结

**Feature 是闸门 0:模糊需求 → problem + users + scope + acceptance + tasks + open questions → 用户点头 → 落地 `docs/features/<slug>/feature.md` → 进 scope**。

每个 feature 一个目录,默认只有 `feature.md` 一份;需要时同目录加 `design.md` / `decisions.md` / `notes.md`。后续每次小迭代围绕 task 打勾推进,状态跟踪天然在文件里。

---

## Auto-invoke chain

完成本 skill 后,LLM 自动 invoke 下一步:

- **直接进 PR 级实施** → `scope`

如用户说"先不动,放着",则不 chain。
