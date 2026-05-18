---
name: inbox
description: feature 上游的可选 capture 层(GTD inbox-process)。触发关键词:"记一下 / 先存着 / 以后做 / 想到个事 / inbox / 散落想法 / 灵感 / 还没想清楚 / 暂存 / 候选"。承载未结构化、未承诺的 raw 需求,只 capture 不承诺。**4 项能力**:inbox 维护 / 提取成 feature / 多 feature 回顾成 epic / 不管 active focus(handoff 在管)。**铁律**:epic 永远 retrospective + 永远手动触发,不按数量。文件落 `docs/inbox.md`,epic 归档落 `docs/epic-<slug>.md`。
---

# Inbox

AI 协作的**可选 capture 层**——feature 上游,承接还没结构化、还没承诺的 raw 需求。

**不替 feature 做承诺**:inbox 条目只 capture,不写 acceptance / tasks / 时间承诺——承诺是 feature 的活。

---

## 心智模型(GTD inbox-process)

```
脑中想法
    ↓ capture(本 skill)
docs/inbox.md          ← raw bullet,无 status,无承诺
    ↓ extract(用户挑一条决定要做)
docs/features/<slug>/feature.md   ← 结构化 + 承诺
    ↓ scope / acceptance / build / commit-gate
完成
    ↓ (可选回顾) 多个相关 feature 凑一组
docs/epic-<slug>.md   ← retrospective 主题聚合
```

**两层接力,不是替代**:

- **inbox** = 未结构化、未承诺("哪天可能做")
- **feature** = 已结构化、已承诺("这次就做这条")

inbox 不偷 feature 的活,feature 不偷 inbox 的活。

**inbox 是可选起点,不强制**:

- 想法已经成熟 → 直接喊 `aye:feature`,**跳过 inbox**
- 想法还模糊 / 一闪而过想存档 → `aye:inbox` 先 capture,后续再 extract
- handoff 收尾时想起散落想法 → **用户自行**喊 inbox 关键词 capture(handoff 不自动钩 inbox,详见与 handoff 的边界)

---

## 触发场景

- 对话里冒出"这个以后也该做"、"记一下"、"先存着"类未承诺想法
- 用户主动喊"看下 inbox" / "inbox 有啥" — 摆 inbox 内容
- 用户挑一条 inbox 条目说"做这个" — 走 inbox → feature 提取

**不自动从 handoff 钩进来**:handoff 收尾不扫关键词、不抛 AskUserQuestion。用户在 handoff 之后(或任何时候)想起散落想法,自行喊本 skill 关键词触发,详见「与 handoff 的边界」。

**不触发**:

- 想法已经明确成形 → 直接喊 `aye:feature`,跳过 inbox
- 当前 feature 内的实现细节 / task 调整 → 写 feature.md 的 Notes 或 tasks,不进 inbox
- bug fix / typo / 一行修改 → 直接动手,不走 inbox

---

## 4 项核心能力

### 1. inbox 维护(增删条目)

新增条目:追加到 `## Inbox` 段尾,建议格式 `- <一句话需求> [#tag]`(tag 可选)。

划掉过时条目:直接删除(不留 ~~strikethrough~~,保持 inbox 始终是"未处理"的真实视图)。

### 2. inbox → feature 提取引导

用户选定一条 inbox 条目要做时:

1. AI **invoke `aye:feature`**,把该条目作为 feature.md 草稿起点
2. feature.md 落地后,**清掉 inbox 里对应那行**(不留痕,extract 是单向迁移)
3. 提示用户:"已从 inbox 提取 → `docs/features/<slug>/feature.md`,inbox 该行已删"

### 3. 多 feature → epic 回顾性总结(**纯手动**)

用户主动喊"总结一下这一组 feature" / "把这几个归一个 epic" 时:

1. AI 列出候选 feature(从 `docs/features/` 找 status=done 的)
2. 用户确认主题 + slug
3. 写 `docs/epic-<slug>.md`,聚合多个 feature.md 的核心信息(problem 浓缩 / 关键决策 / 完成时间线 / 跨 feature 学到的判据)
4. **源 feature.md 保留**(epic 是聚合视图,不是替代;feature.md 仍是原始 record)

### 4. 不管 active focus

"当前在做什么 feature" 这个 cross-feature 视图归 `handoff-*.md` + `docs/features/` 最新 mtime 管,inbox 不重复维护。

---

## inbox.md schema

**最小可用**:

```markdown
# Inbox

## Inbox

- 想法 A [#tag]
- 想法 B [#auth]
- 想法 C

## Epics(可选,只在已产出 epic 时维护)

- [auth-rewrite](epic-auth-rewrite.md) — 2026-Q2 完成,3 feature 聚合
- [observability-pass](epic-observability-pass.md) — 2026-Q3 完成,2 feature 聚合
```

**约定**:

- 只一个 `## Inbox` 段,纯 bullet list
- tag 用 `[#xxx]` 方括号格式,便于未来 epic 总结时检索
- `## Epics` 段是 epic 归档索引(可选,有 epic 才加)
- 不维护 status / priority / 时间字段(那些是 feature 的活)

---

## 触发判据(建议建 inbox.md 的条件)

任一命中 → 建议建 `docs/inbox.md`:

- inbox 累积 ≥ 3 个 candidate(脑里 / 对话里散落想法多到记不住)
- 跨 feature 依赖明显(完成 feature A 后想起来 "B 应该也做")
- 已 ship feature ≥ 3,需要主题聚合视图

不命中 → 保持 per-feature,跳过 inbox(直接喊 `aye:feature`)。

**新项目第一周通常不需要 inbox**——直接喊 `aye:feature` 进结构化流程。inbox 是项目活到出现散落想法时的产物。

---

## Epic 总结铁律

### 1. 永远 retrospective,不 prospective

epic 是**完成多个 feature 后回头发现"这几个是一组"**,不是预先规划"这个 epic 包含哪些 feature"。

理由:prospective epic 需要预判 feature 边界,而 feature 边界本身就是会变的(scope 调整、需求漂移)。retrospective epic 看完成态聚合,边界已定,零判断负担。

### 2. 永远手动触发,不按数量自动

不按 feature 完成数自动触发(如"完成 5 个就提示总结")。

理由:数量触发隐含"每 N 个 feature 构成一个 epic"的错误假设。epic 本质是**主题聚合**,不是**数量聚合**。5 个互不相干的 feature 强行打包 = 凑数,污染 epic 语义。

### 3. 源 feature.md 保留

epic.md 是聚合视图,feature.md 是原始 record,二者不互斥。回顾时看 epic.md 拿全局,深挖时跳 feature.md 拿细节。

---

## 与 handoff 的边界(不自动 chain)

`aye:handoff` 触发时**不**扫关键词、**不**抛 AskUserQuestion 引导 capture。理由两条:

1. **本 skill 自身铁律**:capture / extract / epic 全部"用户主动喊才走",建立在 `principles` 的"AI 不当提问机"之上。handoff 自动钩等于替用户决定"该不该 capture",违反本 skill 设计
2. **handoff 主体铁律**:"只交事实,不替下家拍决策"。自动启 capture 仪式属于"替下家拍 capture 决策"的变体

**正确流程**:用户在 handoff 收尾(或任何时候)想起散落想法,**自行**喊本 skill 关键词("记一下 / 先存着 / 以后做 / inbox / 散落想法 / 灵感 ..."),正常走 capture 仪式。

**Trade-off**:用户当场没想起 → 想法可能丢失。这是 GTD inbox 模型的自洽代价——capture 责任在用户,工具不替他记。比起"AI 每次 handoff 都问一遍"的噪音 + 错误捕获(关键词命中但用户其实不想 capture),丢失少量未承诺想法是更小的代价。

---

## 与 flow 的 chain(起手扫描)

`aye:flow` 新 session 起手时,在扫 `docs/features/handoff-*.md` 之后**顺带**:

1. `ls docs/inbox.md` — 存在则 `wc -l` 算 inbox 条目数
2. 抛一行摘要:"📥 `docs/inbox.md` inbox 有 N 项,要看吗?"
3. 用户喊"看"才 Read,默认不主动展开(零 context 浪费)

不存在 inbox.md → 不提,按现状走 handoff 接力点。

---

## 框架边界:aye 不管 feature 之前的 prioritize

inbox 只是 raw capture(可选),**故意不承载 prioritize 视图**。

prioritize 视图(跨 feature 候选主题归类 / priority 排序 / "已想清楚但等业务触发" 等待区 / WIP / sprint 等)= **项目特定 + 团队偏好 + 业务节奏**,使用者自加任意文件即可(`kanban.md` / `backlog.md` / `_deferred/` 目录 ...),aye 不替你拍——框架强加 schema 必然漂移成 Jira。

aye 框架本分:**inbox(可选)→ feature(承诺)→ ship**,中间不扩宽。

---

## 反模式

### 反模式 1:inbox 条目写承诺级内容

```
❌
## Inbox
- 重写 auth 模块
  - acceptance: 通过所有现有测试
  - tasks:
    - [ ] T1: 调研 oauth2 crate
    - [ ] T2: 写 migration
  - 6 月底前完成
```

acceptance / tasks / 时间承诺**是 feature 的活**,不该在 inbox。写成上面这样 = 在 inbox 偷偷搞 feature,违背"未承诺"本质。

```
✅
## Inbox
- 重写 auth 模块 [#auth]
```

需要承诺 → 走 `aye:feature` extract。

### 反模式 2:在 inbox 维护 status

```
❌
## Inbox
- [doing] 想法 A
- [todo] 想法 B
- [done] 想法 C
```

inbox 不带 status。承诺 = 进 feature,完成 = feature.md 打勾 / commit-gate 收口。inbox 里的条目要么在(未承诺),要么没了(已 extract 或已划掉)。

### 反模式 3:prospective epic

```
❌ 用户:"开个 auth-rewrite epic,先列出来要做哪几个 feature"
   AI:写 epic-auth-rewrite.md,列 [feature-1: ... / feature-2: ... / feature-3: ...]
```

epic 永远 retrospective。想预先规划一组工作 → 在 inbox 里用 tag 聚合(`[#auth]`),等真做完几个再回头聚合成 epic。

### 反模式 4:按数量自动触发 epic

```
❌ AI 检测到完成第 5 个 feature → 主动建议 "要不要总结成 epic?"
```

epic 是主题驱动,不是数量驱动。每完成 N 个就提示 = 退化成 prospective(用户被推着凑数)。**用户主动喊才走**。

### 反模式 5:在 inbox.md 维护 "active focus"

```
❌
## Active
- 当前在做:2026-05-auth-rewrite

## Inbox
- ...
```

active focus 归 `handoff-*.md` + features 目录 mtime 管,inbox 不重复。重复维护必漂移。

### 反模式 6:extract 后留 strikethrough

```
❌
## Inbox
- ~~想法 A~~ (已 extract → features/2026-05-xxx/)
- 想法 B
```

extract = 单向迁移。inbox 始终是"未处理"的真实视图,留 strikethrough 等于让 inbox 长尾累积。直接删,trail 在 git log + features/ 目录里。

---

## 仪式三步

### 1. capture(新增条目)

用户喊"记一下 X" / 主动触发本 skill 任一关键词:

- AI Read `docs/inbox.md`(不存在则 Write 新建)
- 追加条目 `- <一句话需求> [#tag]` 到 `## Inbox` 段尾
- 不动其他段

### 2. extract(挑一条做)

用户选定 inbox 条目"做这个":

- AI invoke `aye:feature`,把该条目作为 feature.md 草稿起点
- feature.md 落地后,Edit `docs/inbox.md` 删除对应那行
- 提示:"已从 inbox 提取 → `docs/features/<slug>/feature.md`,inbox 该行已删"

### 3. epic 总结(**用户主动喊才走**)

用户喊"总结一下这一组 feature" / "归一个 epic":

- AI 列候选 feature(`docs/features/` 下 status=done 的)
- 用户确认主题 + slug
- Write `docs/epic-<slug>.md`
- inbox.md 的 `## Epics` 段追加一行索引

---

## 与其他 skill 的关系

- **`feature`**:本 skill 上游。inbox extract 后 chain 进 `feature`。feature 是"承诺",inbox 是"未承诺"。
- **`flow`**:本 skill 是 flow chain map 的 **Phase 0(可选)**——feature 之前的 capture 层。flow 起手扫 handoff 后顺带扫 inbox 数量。
- **`handoff`**:**不自动 chain**。handoff 主体只交事实,散落想法 capture 由用户在 handoff 后自行喊本 skill 关键词触发。详见「与 handoff 的边界」。
- **`principles`**:本 skill 的 epic retrospective 铁律 + 反模式协议建立在 `principles` 的"AI 不当提问机"(用户主动喊才走 epic)之上。

---

## Auto-invoke chain

完成本 skill 后,根据动作不同:

- **capture 完** → 不 chain,回原对话(用户继续干当前事)
- **extract 完** → invoke `feature`(把 extract 出的需求结构化)
- **epic 总结完** → 不 chain,回原对话

inbox 是 capture 工具,不是工作流入口——大多数时候 capture 完就完事,不主动推下一步。

---

## 一句话总结

**inbox = feature 上游的可选 capture 层**:raw bullet 暂存未承诺想法 → 用户挑一条 extract 成 feature → 多个完成 feature 可手动总结成 retrospective epic。

文件 `docs/inbox.md`,epic 归档 `docs/epic-<slug>.md`。

铁律:不替 feature 做承诺 / epic 永远 retrospective + 手动 / 不在 inbox 维护 active focus / extract 后直接删行不留痕。
