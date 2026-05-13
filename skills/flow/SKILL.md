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

> **为什么 aye 长这样**:5 条 AI 协作硬约束推导出"两段论 + 闸门链 + 横切判据"的形状,详见 `principles` 的 AI 协作篇。本 skill 是地图,只画工作流形状,不重复论证。

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
│   ──→ [写代码 ↔ review N 次]        │
│   ──→ commit-gate (含 push + 打勾 + 摘要) │
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
- 交接摘要是 commit-gate push 后自带步骤,不再独立 skill

---

## Chain Map(完整自动跳转关系)

```
feature ────→ scope(进 PR 级实施)

scope ──────→ acceptance

acceptance ──┬─→ design(大功能,可选闸门 2.5)
             └─→ [写代码]

design ─→ [写代码]

[写代码 + review N 次]
              │
              ▼ 测试绿
commit-gate ─→ push ─→ 短确认 + 回 feature.md 打勾(默认,不主动给摘要)
                          │
                          ▼ 用户选:
                          ├─ 拉下条 task → 直接进 scope
                          └─ 喊"今天到这 / 收 / 暂停" → invoke handoff(写文件 + inline)

横切判据(任意阶段调用):
  principles        — 哲学底座
  rust-principles   — Rust 项目特化
  kotlin-principles — Kotlin 项目特化
  review     — 5 维度判据(写代码中 / commit 前)
  pua               — 跳出代码 / 站领域专家 / research(用户主动喊)
```

每个 skill 自己的 SKILL.md 末尾有"Auto-invoke chain"段说明完成后下一步。

---

## Skill 分层(4 类)

按**性质**而非 lifecycle 分:

### Gate(必走,AI 不能跳)

| skill | 何时进入 |
|---|---|
| `feature` | 闸门 0 — 模糊需求结构化,所有新工作的起点 |
| `scope` | 闸门 1 — 改 ≥ 2 文件 / 改公开 API / 任务模糊 → 走;typo / 单文件显然 → 跳 |
| `acceptance` | 闸门 2 — 有 test command DoD / 改公开 API → 走;文档型 / 简单 → 跳 |
| `commit-gate` | 闸门 3 — 动 git 前必经 |

### Triggered(条件 / 用户主动喊)

| skill | 何时触发 |
|---|---|
| `design` | 闸门 2.5 — 大功能 / 多方案 / 跨 crate / 改公开 API / 改持久化 → 走;否则跳 |
| `handoff` | 用户主动 — 喊"今天到这 / 收 / 暂停 / handoff" |
| `pua` | 用户主动 — 喊"以终为始 / 看行业标准 / 不要捡简单的" |

### Reference(横切判据,任意阶段查)

| skill | 何时调用 |
|---|---|
| `principles` | 多方案纠结 / 决策框架 / 取舍 — 哲学底座 + AI 协作准则 |
| `review` | 写代码中 / commit 前 — 5 维度设计判据 |
| `rust-principles` | Rust 项目自动 active — 类型三分法 / 生命周期 / 错误处理 |
| `kotlin-principles` | Kotlin 项目自动 active — ADT / scope functions / coroutine |

### Nav

| skill | 何时调用 |
|---|---|
| `flow` | 新 session / 不知道用哪个 skill — 本 skill 给地图 |

**全 12 个 skill**(Gate 4 + Triggered 3 + Reference 4 + Nav 1)。

**Phase 2 chain 是 conditional 不是 strict**——简单 task 可只走 `feature` → `commit-gate`,复杂才全套。

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

## 一句话总结

**Phase 1**(feature)产出 **feature.md**(承载需求 + acceptance + tasks + status);**Phase 2**(scope → acceptance → [design] → 写代码 + review → commit-gate:含 push + 回打勾 + 摘要)围绕单个 task 单 session 闭环。

**session = task = PR**——AI 无记忆从约束变 feature。

横切判据:`principles` / `rust-principles` / `kotlin-principles` / `review` 任意阶段调用。

---

## 与其他 skill 的关系

本 skill 是**地图入口**,不替代任何具体仪式 skill:
- 落到 Phase 1 → `feature`
- 落到 Phase 2 → `scope` / `acceptance` / `design` / `review` / `commit-gate`
- 想要哲学底座 → `principles` / `rust-principles`(Rust 项目) / `kotlin-principles`(Kotlin 项目)

如果 AI 在新 feature 不知如何下手,**先调本 skill 看地图**,再点对应 step skill。
