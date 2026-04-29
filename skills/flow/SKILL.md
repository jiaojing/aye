---
description: AI 协作敏捷工作流的导航地图。开新 epic / 接陌生任务 / 不知如何展开 / 想看整体流程时调用。讲清三层 skill 结构 + 7 步流 + 5 条 AI 协作硬约束 + 哪一步对应哪个 skill。为单人 + AI 配对设计,不是教科书 Scrum。
---

# Flow

AI 协作的工作流地图。**不是教科书 Scrum**——为单人 + AI 实时配对编程设计,去掉团队协作仪式,补上 AI 协作专属闸门。

---

## 触发场景

- 开始一个新的 epic / feature / story
- 接到陌生任务,不知如何展开
- 想看整体流程地图,确认现在该用哪个 skill
- 想理解 yebai 全体 skill 怎么编排成一条线

**不触发**:改一行 typo / 改 commit message / 已经在某一步执行中(那时直接调对应 skill,不绕回 flow)。

---

## 5 条 AI 协作硬约束(为什么这套和教科书 Scrum 不同)

教科书 Scrum 假设"团队协作 + 标准化降低沟通成本"。AI 协作场景**完全不同**:

1. **AI 没持久记忆** → 任务必须自描述,不能靠"上次我们说过"
2. **AI 写快人 review 慢** → 瓶颈是人的 review 带宽,不是 AI 产出速度
3. **AI 容易越界** → 给个修 bug,顺手"清理"周边代码;需要显式 scope 纪律
4. **AI 的理解会偏** → 把 case 误命名 story 这种事,只能人类校正
5. **PR 是天然 mental scope** → 一个 PR ≈ AI 一个 conversation 容量;PR 大了 AI 自己也会丢线

这 5 条约束推导出本工作流的特殊形状。

---

## Skill 三层结构

```
┌─ 底座层(横切,永远 active)──────────────────────┐
│  principles   工程哲学(好品味/不破坏 consumer/  │
│               实用主义/简洁)— 纠结时回这找判据 │
│  handoff      session 切换时的交接规则           │
└──────────────────────────────────────────────────┘

┌─ 导航层(找不到方向时进入)─────────────────────┐
│  flow         工作流地图,告诉你"现在该用哪个"  │
└──────────────────────────────────────────────────┘

┌─ 仪式层(按时间点强制触发)─────────────────────┐
│  spec           动手前 · 闸门 0(把模糊需求结构化)│
│  scope-align    动手前 · 闸门 1(对齐本次 PR scope)│
│  acceptance     动手前 · 闸门 2(钉可执行 DoD)    │
│  design-review  动手中 · 横向辅助(5 维度判据)    │
│  commit-review  动手后 · 闸门 3(diff 摆出等点头)│
└──────────────────────────────────────────────────┘
```

**4 道闸门**(spec / scope-align / acceptance / commit-review)对应 5 条硬约束的不同切面。**design-review** 是横切判据,在大破面 / 多方案纠结 / review 已写代码时调用。

---

## 七步流程图

```
Epic ──→ Spec ──→ 排优先级 ──→ Scope 对齐 ──→ Acceptance ──→ 拉式执行 ──→ PR 验收
 │        │         │            │              │              │             │
 │        ▼         │            ▼              ▼              ▼             ▼
人判断   spec     人判断     scope-align    acceptance     design-review   commit-review
       (闸门 0)              (闸门 1)        (闸门 2)       (横向辅助)     (闸门 3)
```

**关键洞察**:7 步里**人判断只 2 步**(Epic 边界 / 优先级);**容易失守的 5 步**都有 skill 把守。"4 道闸门 + 1 道横切"是 AI 协作的最小有效集。

---

## 每步详解

### 1. Epic 定位(人判断)

业务能力的最大颗粒。例:"用户能登录"、"支付通道集成"。

判据:跨多 PR、多 weeks、有明确业务价值边界。

### 2. Spec ←→ `spec`

**AI 协作专属闸门 0**。模糊需求 / 新 feature 进来,先结构化:**problem + users + scope(in/out)+ acceptance + non-goals + open questions** → 用户点头,才进 scope-align。

不写 spec 直接进 scope-align,是在"我以为你要的是 X"的错误前提上对齐。详见 `spec` skill。

**特别提醒**:flow 早期版本曾把"Story 拆分"列为"人判断不需 skill"——错了。这恰好是 AI 容易理解偏的位置,必须有 skill 把守。

### 3. 排优先级(人判断)

判据写下来(下次新 session 没记忆)。常用判据:
- ROI(产出 / 投入)
- 阻塞链(谁阻塞谁)
- 风险(早做风险大的,fail fast)

### 4. Scope 对齐 ←→ `scope-align`

**AI 协作专属闸门 1**。AI 准备动手前,**强制提议 scope → 人点头 → 才动手**循环。

防止 AI 越界 / 自作主张扩展 scope。详见 `scope-align` skill。

### 5. Acceptance 预定 ←→ `acceptance`

**AI 协作专属闸门 2**。写代码前钉死 DoD,不是事后看心情。详见 `acceptance` skill。

### 6. 拉式执行 ←→ `design-review`(横向辅助)

实施代码。**Kanban 拉式**(完一项拉下一项),不是 Scrum 批式(sprint planning 拎一批)。

遇到大破面变更 / 多方案纠结时,调 `design-review` 做 5 维度判据审视。

### 7. PR 验收 ←→ `commit-review`

**AI 协作专属闸门 3**。改完代码 / 测试绿后,**展示 diff → 等用户明确 commit/push 指令 → 才动 git**。详见 `commit-review` skill。

---

## 横向 / 元层 skill(底座层)

| Skill | 角色 |
|-------|------|
| `principles` | 工程哲学底座(好品味 / 不破坏 consumer / 实用主义 / 简洁)。永远 active,纠结时回去找判据 |
| `handoff` | session 即将结束时,只交事实不预设方案 |

---

## 不要做的事(教科书 Scrum 的累赘)

| 累赘 | 为什么不要 |
|------|----------|
| Sprint planning(批量规划)| Kanban 拉式即可,完一项拉一项 |
| Daily standup | 对话本身就是 standup |
| Estimate / 故事点 / velocity | AI 速度方差大,velocity 概念失效 |
| 固定 sprint 周期 | 节拍器对 AI 没用,完了就拉下一个 |
| 周期性 retro | 用户一句"这样不对"就是即时 retro |

---

## 一句话总结

**Epic → 闸门 0(spec)→ 优先级 → 闸门 1(scope-align)→ 闸门 2(acceptance)→ 拉式执行(design-review 辅助)→ 闸门 3(commit-review)**。

横切支撑:`principles` 底座、`handoff` 交接。

---

## 与其他 skill 的关系

本 skill 是**地图入口**,不替代任何具体仪式 skill:
- 落到具体步骤 → 调对应专项 skill(spec / scope-align / acceptance / commit-review / design-review)
- 想要哲学底座 → `principles`
- session 即将结束 → `handoff`

如果 AI 在新 epic 不知如何下手,**先调本 skill 看地图**,再点对应 step skill。
