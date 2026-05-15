# 2026-05-aye-inbox-layer：feature 之上的 optional inbox 层 + 回顾性 epic 归档

## Feature

aye 当前只有 per-feature 工作流（`feature.md → scope → acceptance → build → commit-gate`），缺 feature **上游** 的 capture 层。solo + AI 配对节奏下出现两个真痛点：

- **杂乱想法没暂存处**：对话里冒出"这个以后也该做"类未结构化、未承诺的需求，要么塞进当前 feature.md 的 Notes 污染主线，要么口头说说下次忘掉。
- **多个完成 feature 没主题聚合视图**：dogfood 实证 aye repo 已有 5 个 feature，回看时只能靠目录名猜，没有"哪几个属于同一主题"的归纳层。

受影响人：aye 主用户本人（dogfood 一号场）+ 任何 solo + AI 配对走 aye 工作流的人（shield-rs 项目作旁证，本 feature 范围外作 follow-up）。

需要：引入 **GTD inbox-process 心智模型** 的第三层，与现有 feature / handoff 三维度正交：

- `inbox.md` = inbox（未结构化、未承诺的 raw 需求暂存）
- `feature.md` = project（已结构化、已承诺的工作单元，保持现状不动）
- `epic-<slug>.md` = retrospective archive（回顾时手动产出的主题聚合，非预先规划）

### Users

- **aye 主用户本人**（dogfood 一号场）：本 feature 落地后立即把当前对话散落想法填进新建 `docs/inbox.md` 验证 capture/extract 工作流
- **配对的 AI**：行为变化 — `aye:handoff` 触发时检测散落想法关键词 → 引导写 inbox；`aye:flow` 起手扫 handoff 后顺带扫 `docs/inbox.md` 数量摘要
- **shield-rs 项目**（dogfood 二号场，旁证）：当前手动维护 `docs/sprint.md` 干类似事，本 feature 落地后作 follow-up feature 改造，不进本 scope

### Scope

**In**

- 新增 `aye:inbox` skill（中文，覆盖：inbox 模型 + 4 项能力 + 反模式 + chain 接入）
- 改 `aye:handoff` SKILL.md：加 "检测对话散落想法关键词 → `AskUserQuestion` 引导写入 `docs/inbox.md`" chain 段
- 改 `aye:flow` SKILL.md：起手扫 handoff 后**顺带**扫 `docs/inbox.md` 数量摘要（"`docs/inbox.md` inbox 有 N 项，要看吗？"）
- 改 `aye:flow` 的 chain map：新增 Phase 0 (可选 inbox capture)
- 新增 `docs/inbox.md` 文件（dogfood 一号场，把本对话散落想法填进去）
- `README.md` + `plugin.json` bump 0.7.0（新增独立 skill，minor bump，0.x 实验期）

**Out**（钉死防越界）

- ❌ 不引入 sprint / iteration / 时间周期切分（背叛 Kanban 拉式定位）
- ❌ 不在 `feature.md` frontmatter 加 status 字段（status 唯一存于 feature.md 内部，inbox 不碰）
- ❌ 不引入机械 archive 机制（done feature 自动移目录）— YAGNI，>50 feature 真痛了再加
- ❌ 不预先规划 epic（epic 永远 retrospective，不 prospective）
- ❌ 不按 feature 完成数自动触发 epic 总结（避免数量驱动凑数）
- ❌ 不在 inbox.md 维护 active focus 段（handoff-\*.md 已经在管，不重复）
- ❌ 不改现有 `docs/features/` 目录结构（feature.md 工作流零影响）
- ❌ 不引入 priority tier / P0-P3 分类（candidate 优先级用人脑判断，不进 schema）
- ❌ 不在 inbox 条目里写 acceptance / tasks / 时间承诺（那是 feature 的活，inbox 不替 feature 做承诺级的事）
- ❌ 不强制每个 feature 必走 inbox（inbox 是**可选起点**，想法成熟可直接 `aye:feature` 跳过 inbox）
- ❌ 不 dogfood 改造 shield-rs `docs/sprint.md`（本 feature 范围外，作 follow-up）
- ❌ 不重构 handoff / flow 的核心逻辑（只加 chain 段，不动主体；handoff 双输出 + flow 起手扫 handoff 都保留）

### Constraints

- 所有新内容中文（保留英文术语：`inbox` / `epic` / `retrospective` / `capture` / `extract`）
- 新增 skill 文件位置 `skills/inbox/SKILL.md`（与现有 skill 同构）
- `inbox.md` 位置 `docs/inbox.md`（docs 根下，不进 `docs/features/`，因为 inbox 装的是未结构化 raw，跟 features/ 装的"已 process actionable"边界要清晰）
- `epic-<slug>.md` 位置 `docs/epic-<slug>.md`（与 inbox.md 同级；不带 inbox 前缀因为 epic 跟 inbox 是两类不同生命周期产物，不强绑同前缀）
- bump 版本：0.6.2 → 0.7.0（minor）
- chain 接入只能加 hook 不破现有行为

### Open questions（已答，留 trail）

- ✅ Q1: skill 命名 = **`aye:inbox`**（语义准 + GTD 标准术语 + 长期零歧义；否决 `aye:kanban` 因为已删 status 流转，看板语义不再适用）
- ✅ Q2: `aye:flow` 起手扫 inbox 展示 = **数量摘要**（"inbox 有 N 项，要看吗？"），不主动 Read，零 context 浪费
- ✅ Q3: handoff 检测散落想法 = **关键词触发**（"以后做 / 之后再说 / 还有个想法 / 记一下 / 这块可以..." 等），不每次都问，零唠叨
- ✅ Q4: epic 归档命名 = **`docs/epic-<slug>.md`**（不带 inbox 前缀，因 epic 跟 inbox 是不同生命周期产物）
- ✅ Q5: inbox 条目格式 = **建议格式**`- <一句话需求> [#tag]`（tag 可选，不强制 schema — inbox 性质本来就 raw）
- ✅ Q6: dogfood 范围 = **只 aye repo**（shield-rs 作 follow-up，保持 feature 颗粒度纯净）

## Acceptance

- [x] AC-1: `skills/inbox/SKILL.md` 存在，中文，覆盖 inbox 模型 + 4 项能力（inbox 维护 / inbox→feature 提取引导 / 多 feature→epic 总结 / 不管 active focus）+ chain 接入 + 反模式协议
- [x] AC-2: `skills/handoff/SKILL.md` 已加 "检测散落想法关键词 → `AskUserQuestion` 引导写入 inbox" chain 段（关键词清单 + 仪式步骤）
- [x] AC-3: `skills/flow/SKILL.md` 起手扫描段已加 inbox 数量摘要（与 handoff 检测并列），chain map 新增 Phase 0
- [x] AC-4: `docs/inbox.md` 在 aye repo 自身已建，含本对话产出的散落想法（实际填 5 条 ≥ 3）
- [x] AC-5: `README.md` + `plugin.json` 已 bump（0.7.0，列出新增 skill）
- [ ] AC-6: 手动跑一遍 chain 验证可闭环 — 留作下个 session 真接续时自然验证（mock 跑收益低）
- [ ] AC-7: push origin main

## Tasks

按颗粒度铁律（一 task = 一 commit / 同质必合 / 零新决策 / 软线 5-7）：

- [x] T1: 写 `skills/inbox/SKILL.md`（中文极简，覆盖 inbox 模型 + 4 项能力 + chain + 反模式 + 与其他 skill 关系）
- [x] T2: 改 `skills/handoff/SKILL.md` 加散落想法捕获 chain（关键词触发 + AskUserQuestion + 写入 inbox）
- [x] T3: 改 `skills/flow/SKILL.md` 起手扫描段加 inbox 数量摘要 + chain map 加 Phase 0
- [x] T4: dogfood — 在 aye repo 建 `docs/inbox.md`，把本对话产出的散落想法填进去（实际填 5 条）
- [x] T5: `README.md` + `plugin.json` bump 0.7.0（push 单独走 commit-gate）

## Notes

### 设计来源（前置对话拷问产出，4 轮迭代定版）

- **第 1 轮**：候选三段（candidate/active/done）+ Tier 1/2/3 升级路径 → 被否，状态机太重
- **第 2 轮**：单 kanban.md 板 + epic 维度拆分 → 被否，epic 边界定不准
- **第 3 轮**：白爷自己提 GTD **inbox + retrospective epic** 模型 → 定版，概念极简，3 个根本问题一并解
- **第 4 轮**：naming 从 `kanban` 改 `inbox`（语义准、GTD 标准、零长期歧义）

关键判据沉淀：

- **status 唯一存于 feature.md**，避免 single source of truth 漂移
- **epic 是 retrospective**（回顾性发现），不是 prospective（预先规划）— 边界定不准的问题转嫁
- **archive 永远手动**，不按 feature 完成数自动触发（数量驱动 = 退化成 prospective）
- **inbox 是可选起点**，不强制每个 feature 必走（想法成熟可直接 `aye:feature` 跳过）

### 两层接力关系（不是替代）

```
脑中想法
    ↓ capture (aye:inbox)
docs/inbox.md          ← raw bullet，无 status，无承诺
    ↓ extract（用户挑一条决定要做）
docs/features/<slug>/feature.md   ← 结构化 + 承诺
    ↓ scope / AC / build / commit-gate
完成
    ↓ (可选回顾) 多个相关 feature 凑一组
docs/epic-<slug>.md   ← retrospective 主题聚合
```

inbox 不偷 feature 的活（不写 acceptance / tasks / 时间承诺），feature 不偷 inbox 的活（不存未承诺的想法）。

### 与现有 skill 的边界

| 文件 | 维度 | 生命周期 | 职责 |
|------|------|---------|------|
| `docs/inbox.md` | project | 持久 | inbox：raw 需求暂存（未承诺）|
| `docs/features/<slug>/feature.md` | feature | 持久 | 单工作单元生命周期（含 status，已承诺）|
| `docs/features/handoff-*.md` | session | 瞬态（每日覆盖）| session 接力点 |
| `docs/epic-<slug>.md` | project | 持久（归档）| 回顾性主题聚合（手动产出）|

### aye 工作流图变化

```
Phase 0 (新增，可选): inbox capture        ← aye:inbox
Phase 1: feature 结构化                    ← aye:feature（现状）
Phase 2: scope / AC / build / commit-gate  ← 现状
```

### dogfood 二号场后续

shield-rs 项目当前有 `docs/sprint.md`（手动维护 ad-hoc kanban）。本 feature 落地后**作 follow-up feature** 改造（不进本 feature scope，保持颗粒度）。

### 关键时间线

- 2026-05-15: feature 启动，前置对话 4 轮拷问定版（心智模型 + 目录结构 + naming）
