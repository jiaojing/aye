# Inbox

aye 项目的未承诺需求暂存。raw bullet,无 status,无承诺。

想法成熟 → 用户挑一条喊 `aye:feature` extract;
散落想法 → handoff 触发时也会引导回这里。

详见 `aye:inbox` SKILL.md。

---

## Inbox

- shield-rs 项目 `docs/sprint.md` 改造,从手动维护 ad-hoc kanban → 走 aye:inbox 模式 [#dogfood]
- `aye:feature` SKILL.md 瘦身,当前模板 + 反模式 + 颗粒度铁律已经过长,可考虑拆 `feature` 主体 + `feature-rules` 引用 [#refactor]
- `aye:feature` 的 Open Questions 段可借用 `pick` 的 AskUserQuestion 工具一体化呈现,比 markdown 选项列表交互更顺 [#ux]
- 项目 feature 量 > 50 时再加 `aye:archive` skill 处理机械归档(done feature 移目录),跟 inbox / epic 解耦 [#future]
- handoff 散落想法关键词清单未来可扩展(实战发现新触发词时回头加) [#follow-up]

## Epics

(暂无 — 等多个相关 feature 完成后,用户主动喊"总结一下"才产出)
