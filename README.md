# yebai

Lightweight AI-pair workflow skills for Claude Code.

Designed for **solo dev + AI pair programming**, not team Scrum. Skills focus on the rituals that an AI partner most easily skips:

- **scope alignment** — AI proposes scope, human approves, only then implement.
- **acceptance criteria** — define "done" before writing code, not after.
- **pull-based iteration** — one item at a time, green tests before next.

## Skills

| Skill | When |
|-------|------|
| `/yebai:flow` | Picking up a non-trivial story; want a process map |
| `/yebai:scope-align` | About to dive into code; scope feels fuzzy |
| `/yebai:acceptance` | Story accepted, before any implementation |

Plus optional language-specific principles, loaded on-demand by description match:

| Skill | When |
|-------|------|
| `/yebai:rust-principles` | Auto-invoked in Rust projects (Cargo.toml / .rs / cargo) — type triage, lifetime preference, error-handling style, naming conventions |

## Install

**Local development:**

```bash
git clone https://github.com/jiaojing/yebai ~/Documents/projects/yebai
# In ~/.zshrc:
alias claude='claude --plugin-dir ~/Documents/projects/yebai'
```

**Or via marketplace** (after public push):

```
/plugin marketplace add jiaojing/yebai
```

## License

MIT
