# aye

Lightweight AI-pair workflow skills for Claude Code.

Designed for **solo dev + AI pair programming**, not team Scrum. Skills focus on the rituals that an AI partner most easily skips:

- **scope alignment** — AI proposes scope, human approves, only then implement.
- **acceptance criteria** — define "done" before writing code, not after.
- **pull-based iteration** — one item at a time, green tests before next.

## Skills

Two-phase workflow: **Phase 1 = feature clarification (cross-session)** → **Phase 2 = task iteration (single-session, ends with handoff)**. Skills auto-invoke each other via chains; `/aye:flow` shows the full map.

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
| `/aye:commit-review` | Gate 3 | Tests green — show diff, wait for explicit "commit" / "push" before touching git |
| `/aye:handoff` | Iteration end | Auto-invoked after `commit-review` push — hand off facts only, ready to close session |

### Cross-cutting (always-on)

| Skill | When |
|-------|------|
| `/aye:flow` | Lost / want the workflow map / unsure which skill applies |
| `/aye:principles` | Stuck on a multi-option choice — decision framework (good taste / don't break consumer / pragmatism / simplicity) |
| `/aye:rust-principles` | Auto-invoked in Rust projects (Cargo.toml / .rs / cargo) — type triage, lifetime preference, error-handling style |

## Install

**Via marketplace** (recommended, repo must be public):

```
/plugin marketplace add jiaojing/aye
/plugin install aye@dongbai
```

- `jiaojing/aye` is the GitHub repo path (where the marketplace lives).
- `dongbai` is the marketplace name — author/brand namespace, can host multiple plugins in the future.
- `aye` is the plugin name within the marketplace.

**Or local development** (clone + point Claude at the directory):

```bash
git clone https://github.com/jiaojing/aye ~/Documents/projects/aye
# In ~/.zshrc:
alias claude='claude --plugin-dir ~/Documents/projects/aye'
```

## Verify

After install, `/plugin` should list `aye` as enabled. Then try:

```
/aye:flow
```

You should see the workflow map. If you say something like *"I want to add a search feature"*, the LLM should auto-invoke `/aye:feature` based on the description match.

## License

MIT
