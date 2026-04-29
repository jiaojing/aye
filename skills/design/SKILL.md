---
name: design
description: 大功能技术设计敲定(Phase 2 闸门 2.5,可选)。触发关键词:"得出个方案 / 先设计一下 / 得想想怎么做 / 怎么搞 / 拿不准用啥 / 大功能 / 改公开 API / schema 改 / 跨 crate / 多方案纠结 / 需要画图 / 架构选型 / 设计文档"。在 acceptance 之后、写代码之前,把"怎么实现"敲定成 design.md(问题 + 方案空间 + 决策 + 实施细节四段)。Auto-invoke: 写代码(实施)。
---

# Design

AI 协作的**闸门 2.5**(可选,大功能才触发):scope 锁了、acceptance 钉了,但**实施方案还没定**——先产 design.md,多轮敲定再写代码。

不写 design,大功能场景下 AI 容易"先冲再说",写到一半发现方案不对回头大改;或者临时即兴定方案没有结构,后续没法复用。

---

## 触发场景(任意命中即触发)

- **改公开 API**:对外接口签名 / 协议 / 错误码——破面成本高,要先设计
- **持久化 schema 改**:数据库 / 配置文件 / 序列化格式——演进路径要想清
- **跨 crate / 跨模块**:边界要先画清,实施再切片
- **多方案纠结**:有 2 种以上合理选法,需要拉出来对比
- **需要架构图 / 数据流图 / 状态机**:文字讲不清的设计
- **拿不准用啥**:trait 还是 enum?Builder 还是直接 new?Channel 还是 Mutex?
- **用户白话**:"得出个方案 / 先设计一下 / 得想想怎么做 / 大功能 / 架构选型"

**不触发**:

- 单文件 < 100 行的修改
- bug fix / typo / 局部重命名
- 实施方案显然(只有一种合理选法)
- 已有 design.md 的小修(直接更新现有文档)

---

## 与 `design-review` 的边界(关键)

| | `design` | `design-review` |
|---|---|---|
| 性质 | **产出仪式** | **评价判据** |
| 产物 | `design.md` 文档 | 5 维度评判结论 |
| 触发 | 大功能,acceptance 之后 | 任意阶段,审视某段设计选择 |
| 时机 | 写代码**前** | 写代码**中** / commit **前** |

**两者非互斥**:写 design.md 时遇到拿不准的设计选择(类型抽象 / Builder / Trait / 命名 / 重构),内嵌调用 `design-review` 做 5 维度评估,结果写进 design.md 的"决策"章节。

---

## design.md 四段式模板

```markdown
# Design: <feature 名>

> Linked feature: ../feature.md(可选,如果 feature 在同目录)

## 问题

<technical problem,不是 user problem——user problem 在 feature.md>
<例:"feature.md 说要支持 X,但现在的 Y 数据结构装不下,因为 Z">
<不要重复 feature.md 的 problem——这里讲的是"为什么需要做技术决策">

## 方案空间

至少列 2 个候选方案。**每个方案独立成段**,内含:
- **形态**:核心数据结构 / 接口签名 / 流程伪代码
- **优点**:相对其他方案的优势
- **缺点**:已知短板 / 风险
- **试剂量**:大概 diff 量级 / 涉及几个模块

### 方案 A: <一句话概括>
<形态 / 优点 / 缺点 / 试剂量>

### 方案 B: <一句话概括>
<形态 / 优点 / 缺点 / 试剂量>

(可选 方案 C: ...)

## 决策

**选 方案 X**。

判据(按优先级):
1. <第一维度 + 为什么 X 在这里赢>
2. <第二维度 + ...>
3. ...

**反方向反驳**:为什么不选其他方案?
- 不选 A:<具体短板,不是泛泛说"复杂"——要说"复杂在哪个具体点上">
- 不选 C:...

(如某段决策内嵌了 design-review 的 5 维度判据,在这里引用)

## 实施细节

具体到能照着写代码的程度:

- **架构图 / 数据流**(如适用):ASCII art / mermaid
- **关键 API 签名**:函数名 + 参数 + 返回值 + 错误类型
- **数据 schema**:struct 定义 / 表结构 / 协议字段
- **状态机**(如适用):状态 + 转移条件
- **关键算法步骤**(如适用):伪代码

例:

\`\`\`
┌─────────┐    request    ┌──────────┐
│ Client  ├──────────────>│ Handler  │
└─────────┘               └────┬─────┘
                               │ validate
                               ▼
                          ┌──────────┐
                          │ Service  │
                          └────┬─────┘
                               │ persist
                               ▼
                          ┌──────────┐
                          │   DB     │
                          └──────────┘
\`\`\`

## 风险与回滚(可选)

- **R1**:<风险描述> → <缓解 / 监控点>
- **回滚条件**:满足什么条件触发回滚?(数据丢失 / 性能退化 X% / ...)

## Open questions(可选)

- Q1: <还没敲定的子决策,等讨论>
```

---

## 文件落地

```
docs/features/<YYYY-MM>-<slug>/
  feature.md         # 闸门 0 产出
  design.md          # 本 skill 产出(大功能时存在)
```

**一份 feature 一份 design.md**(本次 scope)。多子系统 → 多 design 暂未支持。

---

## 仪式三步

### 1. AI 提议 design.md 草稿

按四段式模板输出**完整草稿**:

- 问题章节明确技术 problem(不重复 feature.md)
- 方案空间至少 2 个候选,各方案独立
- 决策给推荐 + 判据 + 反方向反驳
- 实施细节具体到能照写

**显式列 ASSUMPTIONS**:

```
ASSUMPTIONS I'M MAKING:
1. <对架构 / 性能 / 兼容性的假设>
2. <对方案空间完备性的假设——还有没有遗漏的候选?>
→ 请校正,否则按这些前提推进。
```

### 2. 用户 review / 多轮迭代

可能:

- ✅ 整体 ok → 落地 design.md → 进写代码
- 🟡 调整某段(改方案空间 / 改决策 / 改实施细节)→ AI 改后再确认
- 🔄 加候选方案(用户提出第 3 个选项)→ AI 补 + 重评决策
- ❌ 方向错了 → 重提

**design 阶段允许多轮反复**——这是它存在的意义。一轮就过的设计往往是没认真想。

### 3. design.md 落地 → 写代码

写到 `docs/features/<slug>/design.md`,然后才开始改源码。**不能跳过这步直接 fork 进 src**。

---

## 反模式

### 反模式 1:方案空间只列一个候选

```
❌ ## 方案空间
   ### 方案 A: 用 trait
   ...
```

只列 A 等于没设计——你已经决定了,只是补程序而已。**至少 2 个**,哪怕 B 显然差也要列出来,反方向反驳才有内容。

### 反模式 2:决策只写"选 A,因为 A 好"

```
❌ 决策:选方案 A,A 比 B 好
```

判据要**具体到维度**:可测试性?性能?可逆性?简洁性?**对每个维度说为什么 A 在这里赢**。这是 design-review 5 维度的应用场景。

### 反模式 3:实施细节抽象空话

```
❌ 实施细节:用合适的数据结构,做必要的错误处理
```

这是没设计。**实施细节要具体到能照着写代码**:接口签名 / struct 字段 / 错误类型 / 关键流程。如果还没具体,就还没 ready,继续讨论。

### 反模式 4:design.md = feature.md

把 user-facing problem / acceptance criteria / out-of-scope 抄一遍——**不要**。design.md 讲技术 how,feature.md 讲 user-facing what。重复了说明边界没分清。

design.md 顶部用一行链回 feature.md 即可。

### 反模式 5:写完 design 直接动手不等点头

```
❌ "design 写完了,我开始改 src/foo.rs..."
```

design 是闸门——必须等用户 review 点头。多轮迭代是 feature。

### 反模式 6:design 里写 code

design 是设计,不是实现。**伪代码 / 接口签名 / 流程图 OK**,**完整可编译代码块 not OK**——那是写代码阶段干的事。design 太具体反而失去"快速对齐方向"的功能。

---

## 与其他 skill 的关系

- **`flow`**:地图。本 skill 是闸门 2.5(acceptance 之后,写代码之前的可选分支)。
- **`feature`**:闸门 0,产出 feature.md(user-facing what)。本 skill 是 feature 的下游可选步骤,产出 design.md(technical how)。两者文件并列在 `docs/features/<slug>/` 目录。
- **`scope`**:闸门 1。scope 决定改哪些文件,本 skill 决定**怎么改**。先 scope 后 design。
- **`acceptance`**:闸门 2。acceptance 钉死"完成 = 什么",本 skill 钉死"实现 = 怎么"。先 acceptance 后 design——知道"完成的样子"才能讨论"如何到达"。
- **`design-review`**:5 维度判据。本 skill 是产出仪式,design-review 是评价判据。**两者非互斥**:写 design.md 内嵌调用 design-review 做 5 维度评估,结果写进决策章节。

---

## 一句话总结

**Design-doc 是闸门 2.5(可选):大功能时,把"怎么实现"敲定成 design.md(问题 + 方案空间 + 决策 + 实施细节四段)→ 用户 review 多轮 → 才进写代码**。

它不是 feature 重复,不是 design-review 替代;它是 acceptance 已钉死后,实施前的方案空间收敛仪式。

---

## Auto-invoke chain

完成本 skill 后,LLM **自动 invoke**:开始写代码(实施),期间可调 `design-review` N 次。

写代码完 + 测试绿 → `commit-review`(闸门 3)。
