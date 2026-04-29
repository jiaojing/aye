# 补全 aye 漏掉的 skill + 重构 Phase 1 模型

## Feature

aye plugin 的 `flow` 自报有 11 个 skill,实际只交付了 9 个,2 个标"待新增"(`kanban` / `design-doc`)。`spec` skill 的描述也偷偷提到"kanban 待新增,当前先手动维护 kanban.md"——意味着 Phase 1 链路实际是断的。

更深层的问题:`spec` + `kanban` 的职责拆分是把一个 feature 的需求描述和任务清单硬切两份,让用户在两个 skill 间跳转维护一个本质连续的工作单元。这是**用 SDD 的"项目级 SPEC.md"模板套了 aye 的"需求级 spec"工作流**,层级搬错了。

### Users

- 自己(白爷,aye 的作者 + 第一用户)
- 任何安装 aye plugin 的开发者——使用流程必须自洽,不能让用户去填 plugin 的"待新增"窟窿

### Scope

**In**

- rename `aye:spec` → `aye:feature`,合并 `aye:spec` + 计划中的 `aye:kanban` 为单一 skill
- feature.md 模板:problem + users + scope + acceptance + tasks + status + notes 全在一份文档里
- 强制约定每个 feature 一个目录(`docs/features/<YYYY-MM>-<slug>/`),目录内至少一份 `feature.md`,需要时同目录加 `design.md` / `decisions.md` / `notes.md`
- 同步修改 `flow` / `scope` / `acceptance` / `handoff` / `rust-principles` 中所有 `spec` / `kanban` 引用
- 同步 README.md
- 新增 `aye:design-doc` skill(替补 flow 里仍然待新增的位置)
- dogfood:用本流程产出本 feature.md 自验证

**Out**

- 不改其他 skill 的内部仪式(`principles` / `scope` / `acceptance` / `handoff` / `commit-review` / `design-review` 内核保留,只改它们对 spec/kanban 的引用文字)
- 不引入独立的 `task` skill——task 颗粒由写代码 ↔ commit-review 循环天然产出,不前置规划
- 不引入 `epic` / `chapter` / `mission` 这种"feature 之上的容器"抽象——单层 feature 够用
- 不改 `principles` / `rust-principles` 的内核(只改它们对 spec/kanban 的引用文字)
- 不调整 plugin 分发结构(marketplace.json `source: "./"` 不动)
- 不改 plugin 版本号(还是 0.1.0)

### Constraints

- plugin 安装机制是 `git clone` 整个仓库到 `~/.claude/plugins/cache/`,意味着 `docs/` 会跟着分发到所有用户;已确认接受(OSS 透明,dogfood 档案是活样本)
- `docs/` 不污染 Claude 上下文——Claude 只加载 `skills/` 等特定结构,死文件不被读
- 现有 skill 之间有 auto-invoke chain 引用,改 `aye:spec` → `aye:feature` 必须同步所有引用,否则 chain 断裂

### Open questions

- Q1: `aye:design-doc` 应该长什么样?触发判据(改公开 API / 跨 crate / 多方案纠结)、模板内容、与 `design-review` 的边界——待 T3 启动时回答
- Q2: dogfood 后是否会发现 feature.md 模板有缺项?如果发现,允许在 commit 前加迭代

## Acceptance

- [x] AC-1: `skills/feature/SKILL.md` 存在,描述合并后的 feature 概念,模板覆盖 problem/acceptance/tasks/notes 章节
- [x] AC-2: `skills/spec/` 目录已通过 `git mv` 改名为 `skills/feature/`,git 历史保留
- [x] AC-3: `flow/scope/acceptance/handoff/rust-principles` 中所有 `spec` / `kanban` 引用更新为 `feature` 语义
- [x] AC-4: README.md 的 Phase 1 表格、auto-invoke 例子同步
- [x] AC-5: `feature.md` 文件落地约定明确为"每个 feature 一个目录,无 promote 仪式"
- [ ] AC-6: `aye:design-doc` skill 落地(`skills/design-doc/SKILL.md` 存在 + flow 里"待新增 1 个"清零)
- [x] AC-7: 本 feature.md 自身存在(dogfood 自验证)
- [ ] AC-8: `grep -rn "spec\b\|kanban\b" skills/ README.md` 的命中只剩反面教材引用 / 真实历史 task 描述,无功能性残留
- [ ] AC-9: commit + push 完成,plugin reload 后 `/aye:feature` 可触发,`/aye:spec` 不可触发(已删)

## Tasks

- [x] T1: `git mv skills/spec skills/feature` + 重写 `feature/SKILL.md`(done,本会话)
- [x] T2: 修 `flow` / `scope` / `acceptance` / `handoff` / `rust-principles` 的 spec/kanban 引用(done,本会话)
- [x] T2.5: 重新设计 feature 文件落地——从"自适应单文件 / 目录"改为"强制每 feature 一目录",删除 promote 概念(done,本会话)
- [x] T4: README.md 同步(done,本会话)
- [x] T5: dogfood——创建本 feature.md(done,本会话当前 task)
- [ ] T6: commit-review + push 本波改动(rename + 重构 + dogfood,作为单一 commit)
- [ ] T3: 新增 `aye:design-doc` skill(独立 feature 拆分?或同 commit?待 T6 完成后决定)
- [ ] T7: 验证——push 后 reload plugin,跑 `/aye:feature` 触发测试

## Notes

关键设计决策追踪(dogfood 透明,留给未来回看):

1. **从"补两个待新增 skill"演化为"重构 Phase 1 模型"**:起初任务是补 `kanban` + `design-doc`,讨论中重新审视 `spec` + `kanban` 的拆分,发现这是把 SDD 的项目级 SPEC.md 模板套到 aye 的需求级工作流——层级搬错了。合并成 `feature` 一个 skill 后,问题消失。

2. **拒绝 epic / chapter / mission 这种容器抽象**:Scrum 的 Theme>Epic>Story>Task 四层在 aye 的单人 + AI 配对场景下太重。最终保留单层 feature——它就是工作的原子单位,内含 task list。

3. **从"自适应单文件 / 目录"改为"强制每 feature 一目录"**:第一版设计是简单 feature 单 `.md` 文件,复杂时 promote 成目录。白爷 push back:数据结构本质是"文档集合",目录天然表达,文件不能。改成强制目录消除了 promote 这个特殊路径——好品味是消除特殊情况,不是增加条件判断。这次是用白爷自己的哲学校正了 AI 的"实用主义"偏向。

4. **task 不单独成 skill**:讨论过 spec → tasks → implementation 三阶段(agent-skills 风格),拒绝。task 颗粒小、前置规划成本 > 收益,由写代码 ↔ commit-review 循环天然产出。aye 仪式停在 acceptance 之后、写代码之前,不再细化。

5. **从 agent-skills 借鉴 ASSUMPTIONS 显式化**:feature skill 的"仪式三步"第一步要求 AI 列出对需求 / 架构的假设让用户校正。降低"AI 自己拍脑袋默默推进"的失败率。

6. **plugin 分发代价已知接受**:marketplace `source: "./"` 整库 clone 到用户 cache。docs/ 不进 Claude 上下文(不被加载),但占用户磁盘几 KB。决定接受——OSS 透明,本 feature.md 自身就是 aye 用 aye 的活样本,对推广反而是优点。
