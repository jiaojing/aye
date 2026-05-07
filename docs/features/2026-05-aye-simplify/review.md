# aye 复杂度 review v2：精修而非剧砍

> v1 是没读完 SKILL.md 的草率推断（"砍 4 / 瘦 3"）。深度阅读全 11 skill 后修订为本 v2。
> v1 反思：dogfood 暴露 AI 在没读全资料时容易给"砍"答案——简化偏见 + sycophancy 反应。已沉淀为 `feature` 反模式 8（短信号下不包揽）。

---

## 1. 单 skill 评估（深度阅读后）

| skill | 行 | 标签 | 判据 |
|---|---:|---|---|
| `principles` | 180 | **留** | 4 哲学密度高;ADT + Type Class 模式跨语言判据,无替代 |
| `flow` | 185 | **留 + 瘦身**（删哲学论证） | 入口角色必备;但内含 2 段哲学论证（5 硬约束 / Scrum 累赘对照）应挪 `principles` |
| `feature` | 362 | **留 + 大瘦身** | 三要素 + 5 硬约束 + 5 拆判据 + 7 反模式 + 总结 → AI token 噪音;但反模式有故事(颗粒度铁律 / Open Q 自己拍) 都要留 |
| `scope` | 144 | **留** | 3 反例真实(横向越界 / 纵向越界 / 静默扩),跟 feature 边界清(闸门 1 vs 闸门 0) |
| `acceptance` | 172 | **留 + 加跳过逃生口** | DoD 三要素实在,但**假设每个 task 都有 test/command** → 文档型/简单 task 空跑 |
| `design` | 249 | **留** | 跟 design-review 边界表清晰,4 段式模板实用,大功能场景必备 |
| `design-review` | 434 | **留**（不大动） | 5 维度 = 判据库 = 长度合理,不是入口 |
| `commit-review` | 154 | **留 + 收尾增强**（吸 handoff） | 6 硬规则紧凑;吸纳 handoff 模板让 push 后摘要协议进 LLM 思考流程 |
| `handoff` | 149 | **内嵌 commit-review,删 skill** | auto-invoke 时 LLM 不读 SKILL.md,反模式协议失效;模板 < 10 行可直接进 commit-review push 后规范段 |
| `rust-principles` | 167 | **留** | Rust 特化,长度紧凑 |
| `kotlin-principles` | 597 | **留 + 拆 reference.md** | 597 太长,SKILL.md 留判据骨架,15 维度详细 + checklist 移 reference |

总览：11 → **10 skill**（handoff 内嵌）。约 2900 行 → ~2400 行。

---

## 2. 5 条结构性发现

| # | 问题 | 性质 |
|---|---|---|
| **S1** | `acceptance` 在文档型/简单 task 上空跑 | 闸门链不自适应 |
| **S2** | `handoff` auto-invoke 时 LLM 不真读 SKILL.md,协议失效 | skill 形态错配 |
| **S3** | "5 维度"撞名（`principles` 决策框架 vs `design-review`） | 命名碰撞 |
| **S4** | 哲学论证散落 `flow`（违反入口锐利） | 信号位置错 |
| **S5** | LLM 在短确认信号下容易包揽 Open questions（本 session 真踩） | 反模式漏 |

---

## 3. dogfood 痛点回应

| # | 痛点 | 修复 |
|---|---|---|
| 1 | 闸门成本 × task 数量 = 交互爆炸 | **颗粒度铁律已治**(`feature` 已加,task 数量从源头降) |
| 2 | commit 完不回 `feature.md` 打勾 | `commit-review` 加"回打勾"步骤 |
| 3 | plugin cache 跟不上 version bump | `README` 加"改 SKILL.md 必须 bump plugin.json"备忘 |
| 4 | 聊半天不干活(过度确认) | `feature` 反模式 8 + `acceptance` 跳过逃生口 |
| 5 | SKILL.md 自身臃肿 | `feature` / `kotlin-principles` 瘦身,哲学论证整合 |

---

## 4. 执行计划（6 commit）

按颗粒度铁律 1 task 1 commit + 同质必合：

| # | commit | 内容 |
|---|---|---|
| 1 | `commit-review 收尾增强,handoff 内嵌` | + 回 feature.md 打勾;+ handoff 模板;删 `skills/handoff/`;改 cross-ref |
| 2 | `acceptance 加跳过逃生口` | 加"何时跳过"段(文档型/0 command DoD/已 feature.AC 细化) |
| 3 | `feature 瘦身 + 反模式 8` | 合并三要素 + 硬约束推导;加反模式 8(短信号不包揽 Open Q) |
| 4 | `kotlin-principles 拆 reference` | SKILL.md 留骨架;15 维度 + checklist → `reference.md` |
| 5 | `flow 哲学论证挪 principles` | flow 删 5 硬约束 + Scrum 累赘表;principles 加 AI 协作篇 |
| 6 | `aye-simplify 收尾` | README bump 备忘;principles "5 维度"重命名;feature.md 打勾;bump 0.3.0 |

bump major(0.2.0→0.3.0):删 handoff skill = 破坏性,major 合理。

---

## 5. 精简后形态

```
横切（4）:
  principles         哲学底座 + AI 协作篇(吸 flow 论证)
  rust-principles    Rust 特化
  kotlin-principles  Kotlin 特化(SKILL.md 骨架 + reference.md 详细)
  design-review      5 维度判据(改名避撞)

入口（1）:
  flow  工作流地图(删哲学论证,留地图本体)

实施（5）:
  feature       闸门 0 problem 结构化(瘦身)
  scope         闸门 1 文件级(加跳过场景)
  acceptance    闸门 2 DoD(加跳过逃生口)
  design        闸门 2.5(可选,大功能)
  commit-review 闸门 3 commit + push + 回打勾 + handoff 摘要(收口)
```

10 skill,chain 步数不变（feature → scope → acceptance → [design] → 写代码 ↔ design-review → commit-review）,但闸门有逃生口（简单 task 跳过 acceptance/design）。

---

## 6. 一句话总结

aye 不是"11 → 7"的剧烈精简,是 **11 → 10 + 4 处精修**:闸门链加自适应（acceptance 跳过、handoff 吸合）、协议进 LLM 思考流程、内文瘦身、哲学论证归位。**瑞士军刀 = 刀刃锋利,不是刀片少**。
