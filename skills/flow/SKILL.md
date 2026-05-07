---
name: flow
description: 工作流地图(导航层)。触发关键词:"该做什么 / 怎么开始 / 整体流程 / 哪个 skill / 现在到哪一步 / 不知如何展开"。展示两段论(Phase 1 feature 明确 / Phase 2 围绕 task 迭代)+ 10 个 skill 位置 + chain 关系。为单人 + AI 配对设计,Kanban 拉式,不是教科书 Scrum。
---

# Flow

AI 协作的工作流地图。**不是教科书 Scrum**——为单人 + AI 实时配对编程设计,去掉团队协作仪式,补上 AI 协作专属闸门。

---

## 触发场景

- 开始一个新的 feature
- 接到陌生任务,不知如何展开
- 想看整体流程地图,确认现在该用哪个 skill
- 想理解 aye 全体 skill 怎么编排成一条线

**不触发**:改一行 typo / 改 commit message / 已经在某一步执行中(那时直接调对应 skill,不绕回 flow)。

---

## 5 条 AI 协作硬约束(为什么这套和教科书 Scrum 不同)

教科书 Scrum 假设"团队协作 + 标准化降低沟通成本"。AI 协作场景**完全不同**:

1. **AI 没持久记忆** → 任务必须自描述,不能靠"上次我们说过"。**优势化**:session = 迭代 = task = PR,自然边界
2. **AI 写快人 review 慢** → 瓶颈是人的 review 带宽,不是 AI 产出速度
3. **AI 容易越界** → 给个修 bug,顺手"清理"周边代码;需要显式 scope 纪律
4. **AI 的理解会偏** → 把模糊需求误解成另一种 deliverable,只能人类校正
5. **PR 是天然 mental scope** → 一个 PR ≈ AI 一个 conversation 容量;PR 大了 AI 自己也会丢线

这 5 条约束推导出**两段论 + 4 闸门 + 横切判据**的工作流形状。

---

## 两段论

整个工作流分两段:

```
┌────────────────────────────────────────────┐
│ Phase 1:feature 明确(可跨多 session)      │
│                                            │
│   模糊想法 ──→ feature(对话补充细节)      │
│                                            │
│   产出:feature.md(单文档承载 problem +    │
│         users + scope + acceptance + tasks │
│         + status + notes)                  │
└────────────────────────────────────────────┘
                    │
                    ▼ 拉一个 task
┌────────────────────────────────────────────┐
│ Phase 2:围绕 task 迭代(单 session 闭环)   │
│                                            │
│   scope ──→ acceptance ──→ design?     │
│   ──→ [写代码 ↔ design-review N 次]        │
│   ──→ commit-review (含 push + 打勾 + 摘要) │
│                                            │
│   产出:1 个 PR + feature.md task 打勾      │
│         + 1 次交接摘要(给"想关 session"     │
│         的无痛出口,不强制关)               │
└────────────────────────────────────────────┘
                    │
                    ▼ 新 session 接力
              拉下一个 task,回 Phase 2
              (feature 已经明确,不再回 Phase 1)
```

**关键洞察**:
- **session = task = PR**,三位一体——AI 没记忆从约束**变成 feature**,迭代天然 self-contained
- **Phase 1 跨 session**(feature.md 持久化),**Phase 2 单 session**(自然闭环)
- 一个 feature 内的多个 task 各自走 Phase 2,不重新走 Phase 1
- 交接摘要是 commit-review push 后自带步骤,不再独立 skill

---

## Chain Map(完整自动跳转关系)

```
feature ────→ scope(进 PR 级实施)

scope ──────→ acceptance

acceptance ──┬─→ design(大功能,可选闸门 2.5)
             └─→ [写代码]

design ─→ [写代码]

[写代码 + design-review N 次]
              │
              ▼ 测试绿
commit-review ─→ push ─→ 回 feature.md 打勾 + 交接摘要(自带)
                          │
                          ▼ 用户选:
                          ├─ 关 session  → 摘要已就位,无痛
                          └─ 拉下条 task → 直接进 scope,不绕回 Phase 1

横切判据(任意阶段调用):
  principles        — 哲学底座
  rust-principles   — Rust 项目特化
  kotlin-principles — Kotlin 项目特化
  design-review     — 5 维度判据(写代码中 / commit 前)
```

每个 skill 自己的 SKILL.md 末尾有"Auto-invoke chain"段说明完成后下一步。

---

## Skill 10 个 + 三层结构

```
┌─ 横切层(常驻判据 / 元规则)──────────────────────┐
│  principles        工程哲学(好品味 / 不破坏 consumer│
│                    / 实用主义 / 简洁)              │
│  rust-principles   Rust idiomatic 设计原则         │
│  kotlin-principles Kotlin idiomatic 设计原则       │
│  flow              工作流地图(本 skill)            │
└──────────────────────────────────────────────────┘

┌─ Phase 1:feature 明确 ──────────────────────────┐
│  feature  单文档承载需求 + acceptance + tasks    │
│           + status(闸门 0)                       │
└──────────────────────────────────────────────────┘

┌─ Phase 2:围绕 task 迭代 ────────────────────────┐
│  scope          PR 级 scope 对齐(闸门 1)        │
│  acceptance     钉死可执行 DoD(闸门 2)          │
│  design         大功能设计文档(可选闸门 2.5)    │
│  design-review  5 维度判据(横向 N 次)           │
│  commit-review  commit + push + 回打勾 + 摘要   │
│                 (闸门 3,含原 handoff 内容)      │
└──────────────────────────────────────────────────┘
```

**全 10 个 skill 全部交付**(横切 4 + Phase 1 的 feature + Phase 2 五个)。

---

## feature.md 落地约定

**强制约定**:每个 feature 一个目录,目录内至少一份 `feature.md`(单一形态,不区分简单 / 复杂)。

```
docs/features/
  <YYYY-MM>-<kebab-slug>/
    feature.md        # 必须,主文档
    design.md         # 可选,需要技术设计时加
    decisions.md      # 可选,长期 ADR
    notes.md          # 可选
```

默认只有 `feature.md` 一份;需要时同目录下直接加 `design.md` / `decisions.md` 等,无 promote 仪式。详见 `aye:feature`。

---

## 不要做的事(教科书 Scrum 的累赘)

| 累赘 | 为什么不要 |
|------|----------|
| Sprint planning(批量规划)| Kanban 拉式即可,完一项拉一项 |
| Daily standup | 对话本身就是 standup |
| Estimate / 故事点 / velocity | AI 速度方差大,velocity 概念失效 |
| 固定 sprint 周期 | 节拍器对 AI 没用,完了就拉下一个 |
| 周期性 retro | 用户一句"这样不对"就是即时 retro |
| Spec / story / kanban 多份文档 | 一份 `feature.md` 起步,需要时同目录加辅助文档 |

---

## 一句话总结

**Phase 1**(feature)产出 **feature.md**(承载需求 + acceptance + tasks + status);**Phase 2**(scope → acceptance → [design] → 写代码 + design-review → commit-review:含 push + 回打勾 + 摘要)围绕单个 task 单 session 闭环。

**session = task = PR**——AI 无记忆从约束变 feature。

横切判据:`principles` / `rust-principles` / `kotlin-principles` / `design-review` 任意阶段调用。

---

## 与其他 skill 的关系

本 skill 是**地图入口**,不替代任何具体仪式 skill:
- 落到 Phase 1 → `feature`
- 落到 Phase 2 → `scope` / `acceptance` / `design` / `design-review` / `commit-review`
- 想要哲学底座 → `principles` / `rust-principles`(Rust 项目) / `kotlin-principles`(Kotlin 项目)

如果 AI 在新 feature 不知如何下手,**先调本 skill 看地图**,再点对应 step skill。
