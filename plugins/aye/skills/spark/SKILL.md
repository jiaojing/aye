---
name: spark
description: 'Feature 之前的可选想法探索(Phase 0.5)。触发关键词:"spark / 展开想想 / 先 brainstorm / 先聊清楚 / 这个值不值得做 / 先出 proposal / 先别写代码 / 还没决定做不做"。从 raw idea 或 inbox 条目展开成 proposal/spec,帮助判断是否值得进入 feature；不承诺、不写 feature.md、不进 scope、不写代码。'
---

# Spark

AI 协作的**可选探索层**:在 `feature` 之前,把一个 raw idea 展开到足够判断"值不值得做 / 是否现在做 / 大概怎么做"。

`spark` 不等于 `feature`。**feature = 已决定要做之后的承诺态**,spark = 还没决定做之前的探索态。

---

## 为什么需要

`inbox` 只 capture,不展开;`feature` 一旦落地就进入承诺态。两者之间有一类真实需求:

- 想法不是一闪而过,值得认真想
- 但还没决定要不要做
- 直接进 `feature` 会把探索误写成承诺
- 直接进 `design` 又太早,因为连"做不做 / 做哪种形态"都没定

`spark` 的职责就是补这段:**探索清楚,然后停住**。

---

## 触发场景

- 用户说"展开想想 / 先 brainstorm / 先聊清楚"
- 用户问"这个值不值得做 / 要不要做 / 方向对不对"
- 用户说"先出个 proposal / 先出 spec / 先别写代码"
- 用户从 inbox 里挑一条,但还没决定正式做
- 一个想法有 2-3 种产品形态,需要先比较

**不触发**:

- 已经决定要做 → 直接 `feature`
- 只是想暂存一句话 → `inbox`
- 已经有 feature.md,只是在决定怎么实现 → `design`
- 正在写代码中审某个抽象 → `review`
- 用户要求行业标准 / 主流 research → `pua`

---

## 工作流位置

`inbox` 和 `spark` 都是 feature 之前的可选层,**没有强制顺序**:

```
raw idea
  ├─ inbox   可选:先存着,不承诺
  ├─ spark   可选:展开判断值不值得做,不承诺
  └─ feature 必经:一旦决定要做,结构化成 feature.md
```

常见路径:

- raw idea → `inbox`:一闪而过,先存
- raw idea → `spark`:值得展开,但还没决定做
- raw idea → `feature`:已经决定做
- `inbox` → `spark`:从暂存想法里挑一条先探索
- `inbox` → `feature`:从暂存想法里挑一条直接承诺
- `spark` → `feature`:探索后用户明确"做这个"
- `spark` → `inbox`:探索后暂时不做,保留成 raw candidate

---

## 产物

默认写到:

```
docs/sparks/<YYYY-MM-DD>-<kebab-slug>.md
```

除非用户明确说"只聊,别落文件"。`docs/sparks/` 是探索记录,不是 backlog / priority 列表,`flow` 起手不主动展开。

---

## Spark.md 模板

```markdown
# Spark: <idea 名,人话>

## Source
<来自用户临时想法 / docs/inbox.md 某条 / 其他上下文>

## Question
<这次探索要回答的核心问题:值不值得做?现在做吗?做哪种形态?>

## Context
<当前已知背景。只写影响判断的事实,不写实现流水账>

## Options

### Option A: <一句话>
- Shape: <做成什么样>
- Upside: <它解决什么>
- Cost: <复杂度 / 维护 / 时间 / 风险>
- Unknowns: <还不确定什么>

### Option B: <一句话>
- Shape: ...
- Upside: ...
- Cost: ...
- Unknowns: ...

(可选 Option C)

## Recommendation
<推荐哪条,为什么。必须说明不推荐其他选项的具体原因>

## Decision
Status: exploring | ready-for-feature | parked | dropped
Next: <停住 / 进入 feature / 回 inbox / 删除>

## Open questions
- <仍需用户拍板的问题,按候选 + trade-off + 推荐 + 理由输出>

## Notes
<对话中产生的关键事实或约束>
```

---

## 仪式三步

### 1. 先界定探索问题

开始时先明确这次 spark 要回答什么,不要泛聊:

- 值不值得做?
- 是否现在做?
- 做轻量版还是完整版本?
- 是产品问题、流程问题,还是工程问题?

如果缺一个阻塞信息,只问**一个**最关键问题。问题必须按 `principles` 的「AI 不当提问机」格式:候选 + trade-off + 推荐 + 理由。

### 2. 展开 2-3 个选项

至少给 2 个候选。每个候选必须有:

- **Shape**:它实际长什么样
- **Upside**:解决什么问题
- **Cost**:代价和风险
- **Unknowns**:需要后续确认什么

不要把技术实现细节写深。spark 阶段比较的是"做哪类东西 / 值不值得做",不是"用哪个函数签名"。

### 3. 写 proposal 然后停住

写 `docs/sparks/<date>-<slug>.md` 后,输出:

- spark 文件路径
- 推荐结论
- 明确的下一步候选:`feature` / `inbox` / `drop` / `stop`

**必须停住**。不要自动 invoke `feature`,不要写 `feature.md`,不要进 `scope`,不要写代码。

---

## Self-review checklist

写完 spark 文件后自查:

- 是否误写了 acceptance / tasks / deadline,把探索态变成承诺态?
- 是否至少列了 2 个真实候选,而不是一个方案拆成两个名字?
- Recommendation 有没有具体反驳其他选项?
- Decision status 是否明确,且默认不是 `ready-for-feature`?
- 是否还有 TBD / TODO / placeholder?
- 如果来自 inbox,是否没有擅自删除 inbox 条目?
- 是否停在 proposal,没有自动进入 feature / scope / code?

---

## 与其他 skill 的关系

- **`inbox`**:同属 feature 上游。inbox 只 capture; spark 展开判断。两者可互相跳,但都不强制。
- **`feature`**:spark 后用户明确"做这个"才进入 feature。feature 是承诺态,会产 `docs/features/<slug>/feature.md`。
- **`design`**:feature / acceptance 之后才处理技术方案。spark 不替代 design。
- **`pua`**:如果用户要求行业标准 / 开源项目 / 专家 research,先用 pua 或在 spark 中明确引用 pua 的结论。
- **`flow`**:spark 是 Phase 0.5 可选探索层。flow 地图展示它,但默认不扫描 `docs/sparks/`。

---

## Auto-invoke / next-step chain

完成后默认 **stop**。

- 用户说"做这个 / 进入 feature" → invoke `feature`
- 用户说"先存着 / 以后再说" → invoke `inbox` 或保留现有 inbox 条目
- 用户说"不要了 / drop" → 只在用户明确要求时删除 inbox 条目或 spark 文件

一句话总结:

**spark = feature 之前的可选探索层:raw idea / inbox 条目 → 2-3 个选项 + 推荐 + proposal 文件 → 停住等用户决定。**
