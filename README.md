# aye

> *aye aye, captain. ⚓*

Lightweight AI-pair workflow skills for Claude Code.

Designed for **solo dev + AI pair programming**, not team Scrum. Skills focus on the rituals that an AI partner most easily skips:

- **scope alignment** — AI proposes scope, human approves, only then implement.
- **acceptance criteria** — define "done" before writing code, not after.
- **pull-based iteration** — one item at a time, green tests before next.

## Skills

Two-phase workflow: **Phase 1 = feature clarification (cross-session)** → **Phase 2 = task iteration (single-session, ends with commit-review push + summary)**. Skills auto-invoke each other via chains.

### Navigation (start here)

| Skill | When |
|-------|------|
| `/aye:flow` | New session / lost / unsure where you are — show the workflow map |

### Phase 1 — Feature clarification

| Skill | Role | When |
|-------|------|------|
| `/aye:feature` | Gate 0 | Vague requirement / new feature — produce a single `feature.md` carrying problem + users + scope + acceptance + tasks + status + notes |

### Phase 2 — Task iteration (single session)

| Skill | Role | When |
|-------|------|------|
| `/aye:scope` | Gate 1 | About to dive into code — AI proposes which files change, you approve, only then implement |
| `/aye:acceptance` | Gate 2 | Scope locked — pin down "done" with executable DoD checklist |
| `/aye:design` | Big-design (Gate 2.5, optional) | Cross-crate / public API / persistence change / multi-option dilemma — produce `design.md` (problem + options + decision + impl) before coding |
| `/aye:design-review` | Cross-cut | While coding / before commit — 5-axis judgment (types / builder / trait / naming / refactoring) |
| `/aye:commit-review` | Gate 3 | Tests green — show diff, wait for explicit "commit" / "push" before touching git. Push 后自带"回 feature.md 打勾 + 交接摘要" |

### Cross-cutting (always-on)

| Skill | When |
|-------|------|
| `/aye:principles` | Stuck on a multi-option choice — decision framework (good taste / don't break consumer / pragmatism / simplicity) |
| `/aye:rust-principles` | Auto-invoked in Rust projects (Cargo.toml / .rs / cargo) — type triage, lifetime preference, error-handling style |
| `/aye:kotlin-principles` | Auto-invoked in Kotlin projects (*.kt / build.gradle.kts) — ADT + type class layout, scope functions, coroutine boundaries |

## Usage

### Two trigger modes

1. **Natural language (primary)** — say a keyword in chat, the LLM auto-invokes the matching skill via the `description` field in each `SKILL.md`. Examples:
   - *"我想加个搜索功能"* / *"加 X 功能"* → `feature`
   - *"开始改 / 准备写代码"* → `scope`
   - *"测试绿了 / 改完了"* → `commit-review`
   - *"今天到这 / context 满了"* → `commit-review` (push 后自带交接摘要)

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

**Phase 2 — one task at a time (single session, ends with commit-review + summary):**
- *"do tasks[0] — implement query parsing"*
- → `/aye:scope` proposes which files change + open questions; you approve
- → `/aye:acceptance` pins the DoD checklist (each item executable)
- *(optional, big design)* → `/aye:design` writes `design.md` (problem + options + decision + impl) before coding
- You write code; invoke `/aye:design-review` for 5-axis judgment whenever you want
- *"tests green"* → `/aye:commit-review` shows the diff, waits for explicit "commit / push"
- → `commit-review` push 后自动产出 next-task pointer + caveats summary（< 10 行,只交事实）。You then choose: close the session (summary makes it painless) or pull next task.

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

## Verify

After install, `/plugin` should list `aye` as enabled. Then try:

```
/aye:flow
```

You should see the workflow map. If you say something like *"I want to add a search feature"*, the LLM should auto-invoke `/aye:feature` based on the description match.

## License

MIT
