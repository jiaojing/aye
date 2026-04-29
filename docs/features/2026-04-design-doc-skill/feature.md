# 新增 aye:design-doc skill

## Feature

aye 的 Phase 2 chain 在 `acceptance` 之后有一个分支:大功能(改公开 API / 跨 crate / 多方案纠结)需要先产出 design 文档再实施。这个分支节点目前是空的——`flow` 标 "design-doc(NEW,待新增)",10 处引用都说"待新增"。

不补这个 skill,大功能场景下 AI 会跳过 design 直接写代码,或者临时即兴写 design 没有约定结构,失去复用价值。

### Users

- 自己(白爷,处理大功能时)
- aye plugin 用户——大功能场景下需要 design 仪式

### Scope

**In**

- 新增 `skills/design-doc/SKILL.md`,定义触发判据 / design.md 模板 / 仪式
- 与 `design-review` 边界文档化(产出 vs 评价)
- 更新 `flow` "待新增 1 个" → 0,skill 总数描述同步
- 更新 `acceptance` / `feature` 等 skill 中"待新增"引用
- 更新 README "design-doc *(planned)*" → 去掉 planned
- design.md 模板:problem-recap + 方案空间 + 决策 + 实施细节(选填:风险/回滚/Open questions)

**Out**

- 不改 `design-review` 内核(它是判据,本 skill 是仪式)
- 不改其他 skill 内核
- 不强制 design.md 模板(参考,非闸门)
- 不支持单 feature 多份 design.md(本次 scope 单 design.md;多子系统后续)
- 不改 chain 拓扑(只在 acceptance 后大功能分支触发)
- 不为本 skill 自身的 SKILL.md 写作再写 design.md(避免元循环)

### Constraints

- 与 `design-review` 概念正交不重叠——5 维度判据归 design-review,产出 design.md 归本 skill
- design.md 落地路径:`docs/features/<slug>/design.md`(沿用 feature 强制目录约定)
- 触发关键词不能跟 `feature` / `scope` / `acceptance` 重叠(那些是闸门 0/1/2,本 skill 是 acceptance 之后的可选分支)

### Decisions(原 Open questions,已敲定)

- Q1 ✅ **四段式**:问题 + 方案空间 + 决策 + 实施细节(选填:风险/回滚/Open questions)。理由:跟 feature.md 风格连续,ADR 偏 retrospective,本场景偏 forward-looking
- Q2 ✅ **触发关键词清单**(白话 + 术语混合):
  - 白话:"得出个方案 / 先设计一下 / 得想想怎么做 / 怎么搞 / 拿不准用啥"
  - 术语:"大功能 / 改公开 API / schema 改 / 跨 crate / 多方案纠结 / 需要画图 / 架构选型 / 设计文档"
  - 边界:不和 `aye:feature` 撞("这个怎么搞" 已被 feature 占,design-doc 用"得想想怎么做"——前者是 0→1,后者是 acceptance 后纠结怎么实现)

## Acceptance

- [x] AC-1: `skills/design/SKILL.md` 存在,符合 SKILL.md 规范(name + description + body)
- [x] AC-2: 触发判据明确——任意命中即触发(改公开 API / 持久化 schema / 跨 crate / 多方案纠结 / 需要架构图)
- [x] AC-3: design.md 模板可直接 copy 使用
- [x] AC-4: 与 `design-review` 边界写在 SKILL.md 末尾"与其他 skill 的关系"段
- [x] AC-5: `flow/SKILL.md` 的"待新增"引用清零,skill 数量描述同步
- [x] AC-6: `acceptance` / `feature` 等 skill 中"待新增"引用清零
- [x] AC-7: README.md 中 `*(planned)*` 标记清掉
- [x] AC-8: `grep -rn "待新增" skills/ README.md` 命中清零
- [x] AC-9: commit + push,plugin reload 后 `/aye:design` 可触发

## Tasks

- [x] T1: 写 `skills/design/SKILL.md`(原 design-doc,中途改名 design)
- [x] T2: 改 `flow/SKILL.md` 的"待新增"引用 + skill 数量描述
- [x] T3: 改 `acceptance/SKILL.md` description 和正文里"待新增"引用
- [x] T4: 改 `feature/SKILL.md` 末尾"待新增"标去掉
- [x] T5: 改 README.md 第 27 行 `*(planned)*` 去掉
- [x] T6: 中途 cleanup——8 个 SKILL.md 加 `name:` frontmatter 字段(主流惯例)
- [x] T7: dogfood——更新本 feature.md Tasks + Notes
- [x] T8: commit-review + push(单 commit:新 skill + rename + frontmatter 统一)

## Notes

关键设计决策追踪:

1. **design-doc vs design-review 正交**:design-review 是 5 维度评价判据(types/builder/trait/naming/refactoring),design-doc 是产出 design.md 仪式。两者非互斥——大 feature 写 design.md 时可中途调用 design-review 评价某段设计选择。

2. **触发位置在 acceptance 之后**(沿用现有 flow chain):scope 已锁、acceptance 已钉,但实施方案还没敲定时进入 design-doc。不改 chain 拓扑。

3. **不强制模板**:design.md 模板是参考,不是闸门。AI 按场景增减章节(架构图大不大、数据流要不要、状态机有没有)。

4. **避免元循环**:本 skill 自身 SKILL.md 写作不走 design-doc 流程,按"明确小任务"路径——否则陷入"写 SKILL.md 前先写 design.md;design.md 怎么设计?再来一份..."的无限递归。

5. **单 commit deliverable**:T1-T8 是同一个 deliverable(design skill 上线 + 引用清理 + frontmatter 统一),按 feature 5 判据应单 commit。回滚原子性强(回滚一半会留 dangling 引用)。

6. **中途演化:design-doc → design**:T1 写完后讨论中决定改名 design-doc → design。理由:跟 feature 同风格(1 词)、文件名 `design.md` 跟 skill 名一致、触发更顺("先 design 一下")。目录 `2026-04-design-doc-skill` **保留原名**作历史档案,反映演化轨迹。

7. **保留 design-review,不砍**:讨论过砍 design-review 把 5 维度并入 design + principles,白爷直觉拒绝("挪到其他地方感觉不对")。判据是独立 skill 比散到模板 + 哲学层更好发现、更完整;砍 skill 是 over-optimization。

8. **frontmatter 统一**:扫描发现 agent-skills / rust-skills 主流 plugin 100% 写 `name:` 字段;aye 9 个里 8 个漏写。本 commit 顺手统一(8 行 trivial 替换),降低未来维护成本。属于"plugin 自洽"的同一 deliverable。
