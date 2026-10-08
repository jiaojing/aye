# Inbox

aye 项目的未承诺需求暂存。raw bullet,无 status,无承诺。

保存或探索想法 → 按用户请求使用 `aye:spark`；
想法决定做 → 由 `aye:feature` 或现有任务承接。handoff 负责交接当前工作。

详见 [spark/SKILL.md](../plugins/aye/skills/spark/SKILL.md)。

---

## Inbox

- shield-rs 项目 `docs/sprint.md` 改造,从手动维护 ad-hoc kanban → 走 aye:spark 的暂存与承接模式 [#dogfood]
- `aye:feature` SKILL.md 瘦身,当前模板 + 反模式 + 颗粒度铁律已经过长,可考虑拆 `feature` 主体 + `feature-rules` 引用 [#refactor]
- `aye:feature` 的 Open Questions 段可用宿主交互选择工具一体化呈现,比 markdown 选项列表交互更顺 [#ux]
- 项目 feature 量 > 50 时再加 `aye:archive` skill 处理机械归档(done feature 移目录),跟 inbox / epic 解耦 [#future]
- handoff 散落想法关键词清单未来可扩展(实战发现新触发词时回头加) [#follow-up]

## Epics

(暂无 — 等多个相关 feature 完成后,用户主动喊"总结一下"才产出)
