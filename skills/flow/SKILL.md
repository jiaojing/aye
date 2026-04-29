---
description: 工作流地图(导航层)。触发关键词:"该做什么 / 怎么开始 / 整体流程 / 哪个 skill / 看板状态 / 现在到哪一步 / 不知如何展开"。展示两段论(Phase 1 需求明确 / Phase 2 围绕 story 迭代)+ 11 个 skill 位置 + chain 关系。为单人 + AI 配对设计,Kanban 拉式,不是教科书 Scrum。
---

# Flow

AI 协作的工作流地图。**不是教科书 Scrum**——为单人 + AI 实时配对编程设计,去掉团队协作仪式,补上 AI 协作专属闸门。

---

## 触发场景

- 开始一个新的 epic / feature / story
- 接到陌生任务,不知如何展开
- 想看整体流程地图,确认现在该用哪个 skill
- 想理解 aye 全体 skill 怎么编排成一条线

**不触发**:改一行 typo / 改 commit message / 已经在某一步执行中(那时直接调对应 skill,不绕回 flow)。

---

## 5 条 AI 协作硬约束(为什么这套和教科书 Scrum 不同)

教科书 Scrum 假设"团队协作 + 标准化降低沟通成本"。AI 协作场景**完全不同**:

1. **AI 没持久记忆** → 任务必须自描述,不能靠"上次我们说过"。**优势化**:session = 迭代 = PR,自然边界
2. **AI 写快人 review 慢** → 瓶颈是人的 review 带宽,不是 AI 产出速度
3. **AI 容易越界** → 给个修 bug,顺手"清理"周边代码;需要显式 scope 纪律
4. **AI 的理解会偏** → 把 case 误命名 story 这种事,只能人类校正
5. **PR 是天然 mental scope** → 一个 PR ≈ AI 一个 conversation 容量;PR 大了 AI 自己也会丢线

这 5 条约束推导出**两段论 + 4 闸门 + 横切判据**的工作流形状。

---

## 两段论

整个工作流分两段:

```
┌────────────────────────────────────────────┐
│ Phase 1:需求明确(可跨多 session)          │
│                                            │
│   模糊想法 ──→ spec(单 story 结构化)──→   │
│   kanban(看板维护,选下一条) ──→ next      │
│                                            │
│   产出:kanban.md(持久化,跨 session)       │
└────────────────────────────────────────────┘
                    │
                    ▼ 选中一个 story
┌────────────────────────────────────────────┐
│ Phase 2:围绕 story 迭代(单 session 闭环)  │
│                                            │
│   scope ──→ acceptance ──→ design-doc?     │
│   ──→ [写代码 ↔ design-review N 次]        │
│   ──→ commit-review ──→ push ──→ handoff   │
│                                            │
│   产出:1 个 PR + 1 次 handoff(session 关) │
└────────────────────────────────────────────┘
                    │
                    ▼ 新 session 接力
              回 Phase 1 选下条
```

**关键洞察**:
- **session = 迭代 = PR**,三位一体——AI 没记忆从约束**变成 feature**,迭代天然 self-contained
- **Phase 1 跨 session**(看板持久化),**Phase 2 单 session**(自然闭环)
- handoff 是 Phase 2 终点,**不是横切元规则**

---

## Chain Map(完整自动跳转关系)

```
spec ─────────┬─→ kanban(多 story 排队,待新增)
              └─→ scope(直接进 PR)

kanban ────→ scope

scope ────→ acceptance

acceptance ──┬─→ design-doc(大功能,待新增)
             └─→ [写代码]

design-doc ─→ [写代码]

[写代码 + design-review N 次]
              │
              ▼ 测试绿
commit-review ─→ push ─→ handoff(自动)
                          │
                          ▼ session 关
                     [新 session: 回 Phase 1]

横切判据(任意阶段调用):
  principles       — 哲学底座
  rust-principles  — Rust 项目特化
  design-review    — 5 维度判据(写代码中 / commit 前)
```

每个 skill 自己的 SKILL.md 末尾有"Auto-invoke chain"段说明完成后下一步。

---

## Skill 11 个 + 三层结构

```
┌─ 横切层(常驻判据 / 元规则)──────────────────────┐
│  principles      工程哲学(好品味 / 不破坏 consumer│
│                  / 实用主义 / 简洁)              │
│  rust-principles Rust idiomatic 设计原则         │
│  flow            工作流地图(本 skill)            │
└──────────────────────────────────────────────────┘

┌─ Phase 1:需求明确 ──────────────────────────────┐
│  spec     单 story 结构化(闸门 0)               │
│  kanban   看板维护(NEW,待新增)                  │
└──────────────────────────────────────────────────┘

┌─ Phase 2:围绕 story 迭代 ───────────────────────┐
│  scope          PR 级 scope 对齐(闸门 1)        │
│  acceptance     钉死可执行 DoD(闸门 2)          │
│  design-doc     大功能设计文档(NEW,待新增)      │
│  design-review  5 维度判据(横向 N 次)           │
│  commit-review  commit + push 闸门(闸门 3)      │
│  handoff        迭代终点交接(自动)              │
└──────────────────────────────────────────────────┘
```

**当前已有 9 个**(横切 3 + Phase 1 的 spec + Phase 2 五个);**待新增 2 个**:`kanban` / `design-doc`。

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

**Phase 1**(spec → kanban)产出 **kanban.md**;**Phase 2**(scope → acceptance → [design-doc] → 写代码 + design-review → commit-review → push → handoff)单 session 闭环。

**session = 迭代 = PR**——AI 无记忆从约束变 feature。

横切判据:`principles` / `rust-principles` / `design-review` 任意阶段调用。

---

## 与其他 skill 的关系

本 skill 是**地图入口**,不替代任何具体仪式 skill:
- 落到 Phase 1 → `spec` / `kanban`
- 落到 Phase 2 → `scope` / `acceptance` / `design-doc` / `design-review` / `commit-review` / `handoff`
- 想要哲学底座 → `principles` / `rust-principles`(Rust 项目)

如果 AI 在新 epic 不知如何下手,**先调本 skill 看地图**,再点对应 step skill。
