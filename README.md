# aye

> *aye aye, captain. ⚓*

Lightweight AI-pair workflow skills for Claude Code.

Designed for **solo dev + AI pair programming**, not team Scrum. Skills focus on the rituals that an AI partner most easily skips:

- **scope alignment** — AI proposes scope, human approves, only then implement.
- **acceptance criteria** — define "done" before writing code, not after.
- **pull-based iteration** — one item at a time, green tests before next.

## Skills

Two-phase workflow: **Phase 1 = feature clarification (cross-session)** → **Phase 2 = task iteration (single-session, ends with commit-gate push + summary)**. Skills auto-invoke each other via chains.

14 个 skill 按性质分 4 类:

### Gate(必走,AI 不能跳)

| Skill | Role | When |
|-------|------|------|
| `/aye:feature` | Gate 0(Phase 1) | 模糊需求 → 单份 `feature.md`(problem + users + scope + constraints + open questions + acceptance + tasks) |
| `/aye:scope` | Gate 1(Phase 2) | 动代码前 — AI 提议改哪些文件 + 不确定项,你点头才走 |
| `/aye:acceptance` | Gate 2(Phase 2) | scope 锁定后 — DoD checklist 钉死"完成" |
| `/aye:commit-gate` | Gate 3(Phase 2 末尾) | 测试绿 — 摆 diff,等明确"commit / push" |

### Triggered(条件 / 用户喊)

| Skill | When |
|-------|------|
| `/aye:inbox` | Phase 0(可选)— feature 上游 capture 层。喊"记一下 / 先存着 / 以后做"或 handoff 检测到散落想法 → 写 `docs/inbox.md`(GTD inbox-process) |
| `/aye:design` | Gate 2.5(可选)— 大功能 / 跨 crate / 改公开 API / 改持久化 → 写 design.md 才写代码 |
| `/aye:handoff` | 用户喊"今天到这 / 收 / 暂停" — 写持久化交接 + inline 摘要 |
| `/aye:pua` | 用户喊"以终为始 / 看行业标准 / 不要捡简单的" — 跳出代码做行业 research |
| `/aye:pick` | 用户喊"pick / 拍板 / 选一个 / 哪条 / 让我选" — 强制下一次决策提问走 `AskUserQuestion` 工具 |

### Reference(横切判据,任意阶段查)

| Skill | When |
|-------|------|
| `/aye:principles` | 多方案纠结 / 决策框架 / 取舍 — 4 条工程哲学 + 决策框架 + AI 协作准则(含 AI 不当提问机) |
| `/aye:review` | 写代码中 / commit 前 — 5 维度判据(类型抽象 / Builder / Trait / 命名 / 重构) |
| `/aye:rust-principles` | Rust 项目自动 active — 类型三分法 / 生命周期 / 错误处理 |
| `/aye:kotlin-principles` | Kotlin 项目自动 active — ADT / scope functions / coroutine |

### Nav

| Skill | When |
|-------|------|
| `/aye:flow` | 新 session / 不知如何展开 — 工作流地图 |

**Lifecycle 顺序**:
- Phase 0(可选,跨 session): `inbox` capture 散落想法 → 等想法成熟 extract 进 Phase 1
- Phase 1(跨 session): `feature` → 拉一个 task
- Phase 2(单 session): `scope` → `acceptance` → [`design`?] → [code ↔ `review` N 次] → `commit-gate`(commit + push + 回打勾)
- (可选回顾): 多个完成 feature → `docs/epic-<slug>.md` retrospective 主题聚合(用户主动喊)

## Usage

### Two trigger modes

1. **Natural language (primary)** — say a keyword in chat, the LLM auto-invokes the matching skill via the `description` field in each `SKILL.md`. Examples:
   - *"我想加个搜索功能"* / *"加 X 功能"* → `feature`
   - *"开始改 / 准备写代码"* → `scope`
   - *"测试绿了 / 改完了"* → `commit-gate`
   - *"今天到这 / context 满了"* → `commit-gate` (push 后自带交接摘要)

   Each skill's full keyword list lives in its `SKILL.md` frontmatter.

2. **Explicit slash (override)** — `/aye:<skill>` forces invocation. Use when:
   - **Resuming a session** — first thing in a new session, type `/aye:flow` to re-orient.
   - **Auto-invoke missed** — your phrasing didn't match any keyword but you want the skill anyway.
   - **Re-entry** — already invoked once, want to re-run with fresh context (e.g. revisit `acceptance` after scope drift).

### A full feature, end to end

You say *"I want to add search to X"*:

**Phase 1 — feature clarification (cross-session):**
- LLM auto-invokes `/aye:feature` → produces a single `feature.md` (problem + users + scope + acceptance + tasks + status + notes)
- You approve

**Phase 2 — one task at a time (single session, ends with commit-gate + summary):**
- *"do tasks[0] — implement query parsing"*
- → `/aye:scope` proposes which files change + open questions; you approve
- → `/aye:acceptance` pins the DoD checklist (each item executable)
- *(optional, big design)* → `/aye:design` writes `design.md` (problem + options + decision + impl) before coding
- You write code; invoke `/aye:review` for 5-axis judgment whenever you want
- *"tests green"* → `/aye:commit-gate` shows the diff, waits for explicit "commit / push"
- → `commit-gate` push 后自动产出 next-task pointer + caveats summary（< 10 行,只交事实）。You then choose: close the session (summary makes it painless) or pull next task.

Next session: open `feature.md`, pick the next unchecked task, repeat Phase 2. Cross-session memory lives in `feature.md`, not in chat history.

## Install

Repo must be public, then:

```
/plugin marketplace add jiaojing/aye
/plugin install aye@dongbai
```

- `jiaojing/aye` is the GitHub repo path (where the marketplace lives).
- `dongbai` is the marketplace name — author/brand namespace, can host multiple plugins in the future.
- `aye` is the plugin name within the marketplace.

## For maintainers

When editing any `skills/*/SKILL.md`, bump `version` in `.claude-plugin/plugin.json` (semver) and push. Otherwise `/plugin update aye` shows no-op despite new commits — marketplace caches by manifest version, not by commit sha.

After bump + push, **users must run BOTH `/plugin update aye` AND `/reload-plugins`** to switch their running cache to the new version. `/plugin update` alone fetches the new manifest but doesn't reload skills already loaded in the current Claude Code session.

## Verify

After install, `/plugin` should list `aye` as enabled. Then try:

```
/aye:flow
```

You should see the workflow map. If you say something like *"I want to add a search feature"*, the LLM should auto-invoke `/aye:feature` based on the description match.

## Changelog

### 0.7.0

**`inbox` skill 加入**(feature 上游可选 capture 层,GTD inbox-process):
- 新增 `aye:inbox`:承接未结构化、未承诺的 raw 需求,只 capture 不承诺
- 4 项能力:inbox 维护 / extract 成 feature / 回顾性 epic 总结 / 不管 active focus
- 铁律:epic 永远 retrospective + 永远手动触发(不按数量自动);inbox 不替 feature 做承诺
- 文件落 `docs/inbox.md`,epic 归档落 `docs/epic-<slug>.md`(同 docs 根下,不进 features/)
- **chain 交集**:
  - `handoff` 触发时若检测到散落想法关键词("以后做 / 还有个想法 / TODO" 等) → 抛 AskUserQuestion 引导写入 inbox
  - `flow` 新 session 起手扫 handoff 后顺带扫 inbox 数量摘要(不主动展开,零 context 浪费)
- 工作流升级为**三段论**:Phase 0(可选 inbox)→ Phase 1(feature)→ Phase 2(task 迭代)
- `flow` chain map 同步更新,全 skill 数 13 → 14

### 0.6.1

**`scope` 二次确认 bug fix**:
- AskUserQuestion 答完 = 已点头,AI 不该再追问"说'动手'我才开干"——冗余二次确认
- 仪式第 2 步改成 conditional 双路径:工具路径直接动手 / 纯文字路径才等点头
- 输出格式段同步重写

### 0.6.0

**`pick` skill 加入**:
- 用户主动喊"pick / 拍板 / 选一个 / 哪条 / 让我选 / 二选一"
- 强制下一次决策提问走 `AskUserQuestion` 工具(原约束在 `principles` § AI 不当提问机 + 4 个 step skill 里横切,但实战经常没触发)
- `commit-gate` 等指示段同步改写为选项化呈现
- `flow` 收录(Triggered 表 + Chain Map),全 skill 数 12 → 13

### 0.5.0 (breaking)

**Rename**:
- `design-review` → `review`(去掉冗余前缀,跟 `commit-gate` 形成 reference vs gate 对比)
- `commit-review` → `commit-gate`(命名明示闸门性质,避免与 `review` 视觉混淆)
- 老 slash 命令 `/aye:design-review` / `/aye:commit-review` 失效;改用 `/aye:review` / `/aye:commit-gate`

**「AI 不当提问机」准则**:
- `principles` 加横切准则段:抛 Open Question 必须 候选 + trade-off + 推荐 + 理由
- `AskUserQuestion` 工具优先(用户键盘选,零打字);文本 fallback 模板备用
- `feature` / `scope` / `design` / `acceptance` 各 SKILL.md 引用 + 加输出模板

**关键字降冲突**(只动 description):
- "多方案纠结" 三抢 → 只留 `design`
- "审视/对不对/反思" 三抢 → `review` 保留,`principles` / `pua` 让路
- "怎么搞" 三抢 → `feature` / `scope` / `design` 各自换更尖锐触发
- "判据" 双抢 → `principles` 改"哲学/取舍",`review` 保留"5 维度判据"
- "完成" 双抢 → `acceptance` 改"DoD/验收标准",`commit-gate` 改"测试绿了/可以提交"

**分层呈现**:
- README + `flow` skill 按 Gate / Triggered / Reference / Nav 4 类重排(替代原 Phase 1 / Phase 2 / Cross-cutting 视图)

**spec-kit 借鉴**:
- `feature` 的 `### Constraints` 字段加项目级不可变约束示例(替 spec-kit `constitution.md`)
- `design` 实施细节段加条件分支:破公开 API → `contracts.md`,改持久化 → `schema.md`(替 spec-kit `contracts/` + `data-model.md`)

## License

MIT
