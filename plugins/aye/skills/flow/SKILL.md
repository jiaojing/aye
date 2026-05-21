---
name: flow
description: '工作流地图(导航层)。触发关键词:"该做什么 / 怎么开始 / 整体流程 / 哪个 skill / 现在到哪一步 / 不知如何展开"。**新 session 起手强制先扫 docs/features/handoff-*.md 作为接力点 + 顺带扫 docs/inbox.md 数量摘要**,再展示三段论(Phase 0 可选 inbox capture / Phase 1 feature 明确 / Phase 2 围绕 task 迭代)+ 14 个 skill 位置 + chain 关系。为单人 + AI 配对设计,Kanban 拉式,不是教科书 Scrum。'
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

## 新 session 起手强制动作(铁律)

flow 触发时,AI **必须先做**:

1. `ls docs/features/handoff-*.md`(项目根目录下)
2. 存在 → 读最新一份(文件名 `handoff-YYYY-MM-DD.md` 按日期降序取头一份)
3. 把"接力点"作为**地图第一节**抛给用户(状态 + 起点候选)
4. **顺带扫** `docs/inbox.md`(存在则 `wc -l` 算 `## Inbox` 段条目数)
   - 抛一行摘要:"📥 `docs/inbox.md` inbox 有 N 项,要看吗?"
   - 用户喊"看"才 Read,默认不主动展开(零 context 浪费)
   - 不存在 → 不提
5. 再列三段论 / chain map
6. handoff 不存在 + inbox 不存在 → 按现状直接抛地图

**理由**:session 无记忆是 aye 的 Phase 2 闸门设计前提,handoff 文件就是为接力服务的 ground truth,inbox 文件是 feature 上游 capture 层的 ground truth。新 session 喊 `/aye:flow`(Claude Code)或显式说"用 aye flow"(Codex)几乎等价于"我刚开 session,告诉我从哪接 + 还有啥候选要做"——直接抛地图让用户从零选,等于让用户自己背 context,反 self-contained。

**反模式**:看到 flow 触发就抛通用地图,忽视 handoff / inbox 文件存在。上一任写 handoff / capture inbox 是为了让下一任无痛接力,跳过等于浪费上游工作。

---

> **为什么 aye 长这样**:5 条 AI 协作硬约束推导出"两段论 + 闸门链 + 横切判据"的形状,详见 `principles` 的 AI 协作篇。本 skill 是地图,只画工作流形状,不重复论证。

---

## 三段论

整个工作流分三段(Phase 0 可选):

```
┌────────────────────────────────────────────┐
│ Phase 0(可选):inbox capture               │
│                                            │
│   散落想法 ──→ inbox(只 capture,不承诺)  │
│                                            │
│   产出:docs/inbox.md(raw bullet,无 status │
│         无承诺;handoff 时也会引导写入)     │
│                                            │
│   想法成熟可跳过 inbox,直接进 Phase 1      │
└────────────────────────────────────────────┘
                    │
                    ▼ 用户挑一条 extract(或新想法直接进)
┌────────────────────────────────────────────┐
│ Phase 1:feature 明确(可跨多 session)      │
│                                            │
│   想法 ──→ feature(对话补充细节)          │
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

(可选回顾)多个完成 feature 凑一组主题
              ──→ docs/epic-<slug>.md
              (retrospective 聚合,用户主动喊才走)
```

**关键洞察**:
- **session = task = PR**,三位一体——AI 没记忆从约束**变成 feature**,迭代天然 self-contained
- **Phase 0 可选**(项目早期 / 想法成熟可直接 Phase 1),**Phase 1 跨 session**(feature.md 持久化),**Phase 2 单 session**(自然闭环)
- 一个 feature 内的多个 task 各自走 Phase 2,不重新走 Phase 1
- 交接摘要是 commit-gate push 后自带步骤,不再独立 skill
- epic 聚合是 retrospective 视图(看完成态归纳),永远手动触发,不预先规划

---

## Chain Map(完整自动跳转关系)

```
inbox(可选 Phase 0)──→ feature(extract 一条做)
   │
   └→ 也可独立 capture,不进 feature(等以后)

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
                          └─ 喊"今天到这 / 收 / 暂停" → invoke handoff
                                                       │
                                                       ▼ 摘要 < 10 行(不自动钩 inbox)
                                                       └─ 想 capture 散落想法?用户自行喊 inbox 关键词

(可选回顾)多个完成 feature ──→ epic-<slug>.md(用户喊"总结")

横切判据(任意阶段调用):
  principles        — 哲学底座
  rust-principles   — Rust 项目特化
  kotlin-principles — Kotlin 项目特化
  review            — 5 维度判据(写代码中 / commit 前)
  pua               — 跳出代码 / 站领域专家 / research(用户主动喊)
  pick              — 决策点强制走交互式选择工具;不可用则文本 fallback(用户主动喊)
```

每个 skill 自己的 SKILL.md 末尾有"Auto-invoke / next-step chain"段说明完成后下一步。

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
| `inbox` | Phase 0(可选)— feature 上游 capture 层。**用户主动喊**"记一下 / 先存着 / 以后做 / inbox / 散落想法 ..."才走(handoff 不自动钩);想法成熟可跳过直接进 feature |
| `design` | 闸门 2.5 — 大功能 / 多方案 / 跨 crate / 改公开 API / 改持久化 → 走;否则跳 |
| `handoff` | 用户主动 — 喊"今天到这 / 收 / 暂停 / handoff" |
| `pua` | 用户主动 — 喊"以终为始 / 看行业标准 / 不要捡简单的" |
| `pick` | 用户主动 — 喊"pick / 拍板 / 选一个 / 哪条 / 让我选",强制下一次决策提问走交互式选择工具;不可用则文本 fallback |

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

**全 14 个 skill**(Gate 4 + Triggered 5 + Reference 4 + Nav 1)。

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

**Phase 0**(可选 inbox)capture 未承诺的散落想法,**Phase 1**(feature)产出 **feature.md**(承载需求 + acceptance + tasks + status);**Phase 2**(scope → acceptance → [design] → 写代码 + review → commit-gate:含 push + 回打勾 + 摘要)围绕单个 task 单 session 闭环。

**session = task = PR**——AI 无记忆从约束变 feature。

横切判据:`principles` / `rust-principles` / `kotlin-principles` / `review` 任意阶段调用。

---

## 与其他 skill 的关系

本 skill 是**地图入口**,不替代任何具体仪式 skill:
- 落到 Phase 0 → `inbox`(可选 capture)
- 落到 Phase 1 → `feature`
- 落到 Phase 2 → `scope` / `acceptance` / `design` / `review` / `commit-gate`
- 想要哲学底座 → `principles` / `rust-principles`(Rust 项目) / `kotlin-principles`(Kotlin 项目)

如果 AI 在新 feature 不知如何下手,**先调本 skill 看地图**,再点对应 step skill。
