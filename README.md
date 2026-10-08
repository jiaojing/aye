# aye

> *aye aye, captain. ⚓*

Lightweight AI-pair workflow skills for Claude Code and Codex.

Designed for **solo dev + AI pair programming**. Aye focuses on decisions and evidence that make a delivery reviewable:

- **scope and authorization** — clarify real ambiguity, reuse decisions and authorization, then complete the necessary work.
- **domain boundaries** — make business rules, state ownership and cross-context contracts explicit.
- **acceptance evidence** — define observable results and verify the risks that matter.
- **delivery rhythm** — keep changes understandable and avoid unrelated work.

## Skills

14 个 skill 按实际需求使用。目标与授权明确时直接实施；歧义、重要设计取舍或验证缺口出现时，再补对应能力。

| 类型 | Skill | 职责 |
|---|---|---|
| Clarification | `feature` | 已决定要做的模糊需求 → 目标、范围、验收与任务 |
| Clarification | `scope` | 一次交付的修改范围、领域归属与真实未知项 |
| Clarification | `acceptance` | 可执行验收；复用有效测试并补验证缺口 |
| Design | `design` | 领域边界、重要契约、持久化或并发的实质取舍 |
| Git | `commit-gate` | 准备可 review 的结果并执行已授权的 commit/push |
| Exploration | `spark` | 未承诺想法的保存、探索与承接；按需记录 proposal |
| On request | `handoff` | 暂停/接力时保存事实、已有决策与恢复位置 |
| On request | `cross-review` | 多 agent 审查的当前索引与历史轮次 |
| On request | `pua` | 用户要求的领域证据、research 与方向校正 |
| Reference | `principles` | 工程取舍、bounded context、数据/行为与资源所有权 |
| Reference | `review` | 类型、构造、能力、领域边界与兼容审查 |
| Reference | `rust-principles` | Rust 特化语义、异步资源与 API 兼容 |
| Reference | `kotlin-principles` | Kotlin 类型、协程、值语义与 API 兼容 |
| Navigation | `flow` | 按目标、风险与授权定位下一步 |

## Workflow

`flow` 是流程关系的统一入口，具体格式与判据由各 skill 维护：

```text
想法 → spark（按请求保存或探索）
决定做但目标模糊 → feature（澄清）
目标与授权明确 → 按需补 scope / acceptance / design
               → implementation ↔ review（按风险）
               → 验收结果与交付说明

请求 commit / push → commit-gate
请求暂停 / 接力    → handoff
```

这是一套按需使用的能力，不是固定闸门链。多文件、跨 crate、新增辅助文件或存在普通实现选项，本身不触发额外确认。真实目标歧义、范围扩张、重要未决取舍和权限不足才需要对齐。

- 验收目标在实现前明确；已有测试、计划和文档足够时直接复用。
- 新建或调整业务边界时，说明规则 owner、局部业务语言、公开契约、依赖方向与必要的语义转换。
- 数据与行为可以分开，领域函数与接口解释器都可使用；共享模型需要共同语义与演进约定。
- 兼容性、迁移、并发一致性、任务取消和 shutdown 按实际风险检查。
- 用户明确要求先探索、规划或 review 时停在该阶段；已授权实施的任务不因切换 skill 重复确认。
- 项目已有 issue、ADR 或文档布局优先；默认路径继续保留，不为小任务制造全套文档。
- Git 操作按授权执行，验收通过不自动授权 commit/push。同一任务的明确授权可复用，合并指令可以一次覆盖两项操作。

## Usage

### Natural language

Skills 的 description 描述能力与适用情境；按请求语义选择，不依赖长关键词列表。例如：

- “修复这个明确的错误” → 直接定位、实施与验证；缺少关键事实时再对齐。
- “想加搜索功能，先明确目标” → `feature`。
- “记一下，以后再做” → `spark` 保存；“这个想法值不值得做” → `spark` 探索，可以直接对话。
- “给我选项拍板” → 在当前任务中给真实候选与推荐，使用宿主适合该问题的交互工具。
- “调整计费和支付的职责边界” → 按需要使用 `scope` / `design` / `review`。
- “让 CC 和你下一轮互审” → `cross-review`。
- “测试绿了” → 报告结果；“检查并 commit + push” → `commit-gate`。
- “今天到这，留个接力点” → `handoff`。

### Explicit invocation

Claude Code 使用 `/aye:<skill>`；Codex 可说 `use aye <skill>` 或“用 aye 的 <skill>”。例如 `use aye flow`、`use aye review`、`use aye cross-review`。

从 0.13 起，`/aye:inbox` 并入 `/aye:spark`（Codex 使用 `use aye spark`），保存和探索按请求分别处理；独立的 `/aye:pick` 命令移除，直接在当前任务中要求出选项或拍板。已有 `docs/inbox.md` 和提案文档继续复用。

明确任务无需先调用 flow。需要恢复工作时，按相关任务、最新 handoff 与当前代码核对接力事实。

### A delivery

“实现已有 feature 的查询解析任务”：

1. 读取目标、相关实现、契约与验证，复用已有范围和决策。
2. 目标明确时直接实施；发现关键歧义或重要设计变化时补相应对齐。
3. 按风险检查领域归属、兼容性和行为，完成验收并报告结果。
4. 用户请求 commit/push 时，准备可 review 的摘要并执行已授权操作。
5. 更新已有任务与决策记录；用户请求接力时再写 handoff。

跨 session 的重要事实落到现有项目文档，避免依赖聊天记忆。

## Install

### Claude Code

Repo must be public, then:

```
/plugin marketplace add jiaojing/aye
/plugin install aye@dongbai
```

- `jiaojing/aye` is the GitHub repo path (where the marketplace lives).
- `dongbai` is the marketplace name — author/brand namespace, can host multiple plugins in the future.
- `aye` is the plugin name within the marketplace.

### Codex

Repo must be public, then:

```bash
codex plugin marketplace add jiaojing/aye
codex plugin add aye@dongbai
```

For local development:

```bash
codex plugin marketplace add /path/to/aye
codex plugin add aye@dongbai
```

Both marketplaces point at `plugins/aye/`. Codex reads `.agents/plugins/marketplace.json` plus `plugins/aye/.codex-plugin/plugin.json`; Claude Code reads `.claude-plugin/marketplace.json` plus `plugins/aye/.claude-plugin/plugin.json`. Both hosts share the same `plugins/aye/skills/` directory.

## For maintainers

When changing plugin skills or supporting references, bump `version` in both `plugins/aye/.claude-plugin/plugin.json` and `plugins/aye/.codex-plugin/plugin.json` (semver). Commit/push according to the user's authorization when publishing the update. A version bump helps marketplace clients discover a changed plugin snapshot.

Claude Code users must run BOTH `/plugin update aye` AND `/reload-plugins` to switch their running cache to the new version. `/plugin update` alone fetches the new manifest but doesn't reload skills already loaded in the current Claude Code session.

Codex users can refresh the marketplace snapshot and reinstall/update the plugin with:

```bash
codex plugin marketplace upgrade dongbai
codex plugin add aye@dongbai
```

## Verify

After Claude Code install, `/plugin` should list `aye` as enabled. Then try:

```
/aye:flow
```

After Codex install, try:

```text
use aye flow
```

You should see the workflow map. If you say something like *"I want to add a search feature"*, the LLM should invoke `feature` based on the description match.

## Changelog

### 0.13.5

**Rust Operation + Service + Layer/Pipeline 组合设计**：

- 在主指南推荐操作协议、执行单元与阶段组合，区分资源 owner 和执行 Service，适用于业务准入、提交及外部执行。
- 扩展操作参考，结合 Toasty 与 Cauldron 区分 Layer 包装和 Pipeline 串接，说明 Tower readiness、组合顺序及恢复责任。
- 新增使用 `tower-service` / `tower-layer` 的可运行示例和 7 项行为测试，验证批次累计准入、业务拒绝、提交先于执行、结果未知及 readiness 传递；两份插件清单同步升至 `0.13.5`。

### 0.13.4

**Rust 操作 DSL + 解释器参考设计**：

- 将 Toasty 的 Operation 分析沉淀到按需参考，区分内部计划、后端操作协议、能力差异和资源 owner。
- 新增可运行键值存储示例，展示 Operation/payload、解释器、能力预检和类型化 consumer 门面；覆盖只读约束、响应错误及无自动重试。
- 明确操作数据化与事务、重放、幂等之间的责任边界；两份插件清单同步升至 `0.13.4`。

### 0.13.3

**Rust 错误上下文与部分失败契约**：

- 上下文说明操作与对象；调用方需匹配的信息保留为类型化字段，只在引入新业务信息的边界补充。
- 按需表达多项校验失败和批处理进度；正常的部分成功用业务结果表达，超时或取消后的未知结果保持明确。
- 延续模块级 `Error` / `Result<T>` 与 thiserror + anyhow 分工，明确临时故障不单独保证安全重试；两份插件清单同步升至 `0.13.3`。

### 0.13.2

**Asyncband 可运行 showcase**：

- Rust 异步协调选型先检查 Asyncband 的现成原语，按实际契约、现有实现和版本约束决定是否使用；普通修复不触发依赖迁移。
- 新增独立 Rust crate、锁文件与入口程序，演示共享读取失败、队列排空和广播背压；固定已发布的 Asyncband 0.7.3。
- 保存读请求/关闭组合和 Worker owner，覆盖显式 shutdown、任务结束、超时 abort，以及 owner/关闭 future 的取消。
- 临时验证沉淀为 16 项行为契约；指南链接实际实现并提供运行、测试和 Clippy 命令，减少重复代码。
- 两份插件清单同步升至 `0.13.2`。

### 0.13.1

**Asyncband 按需并发参考**：

- `rust-principles` 新增基于 Asyncband v0.7.3 的原语选型与组合指南，区分运行时协调机制和应用策略。
- 提供 singleflight + Semaphore 共享结果示例，以及关闭协调 + join 共同预算的 worker 示例；解释错误、取消、背压和初始化语义。
- 核对发布版与主分支差异；保留现有 Tokio 示例和 14 个 skill 入口，两份插件清单同步升至 `0.13.1`。

### 0.13.0 (breaking)

**合并想法入口，移除独立选择 skill**：

- `inbox` 并入 `spark`，按当前意图保存、探索或承接；保留 raw idea、提案和 retrospective epic 的已有能力。
- 删除 `pick`，真实候选、推荐和宿主交互工具的简短判据归回 `principles`；已有决定不重问。
- 保留独立 `handoff`，交接和想法管理按用户请求分别使用。
- 独立入口从 16 个降到 14 个；导航、feature 分流、Codex 提示和两份插件清单同步更新。
- 现有 `docs/inbox.md` 和 `docs/sparks/` 等提案位置继续复用；更新当前 inbox 的入口说明。

### 0.12.1

**Rust 判据与可运行示例**：

- `rust-principles` 补编译错误诊断、enum/typestate 取舍、错误恢复责任、性能证据与 unsafe 审查入口。
- 按需参考覆盖领域构造、异步取消/关闭、FFI 不变量和性能判断；判据组织参考 actionbook/rust-skills，具体语义核对官方文档。
- 新增完整模块 showcase：销售文件模块、支付目录门面与私有子模块、受控值和纯规则、adapter 契约/错误映射、thiserror + anyhow，以及通过公开 API 验证的 CLI 和契约测试。
- 保留独立 Service 示例：有界队列、排空、超时、worker 失败与取消清理。
- 两份插件清单同步升至 `0.12.1`，现有 skill 入口保持不变。

### 0.12.0

**按目标、风险与授权使用 skills，补齐领域边界判据**：

- 保留 16 个入口，description 改为能力与适用情境；`flow` 集中维护按需流程。
- `feature` / `scope` / `acceptance` / `design` 复用已有决策、授权与验证，减少重复确认和强制文档；沿既有契约跨模块或新增辅助文件不自动触发闸门。
- `principles` / `review` / `design` 补 bounded context、局部业务语言、规则 owner、跨边界契约、共享模型演进与一致性。
- 数据与行为允许按领域模块组织，普通函数、方法和解释器按真实需要选择；有状态服务明确状态与生命周期 owner。
- `commit-gate` 保留 Git 授权与验证，复用适用授权，删除固定行数与标题长度要求；交付不自动要求 commit/push。
- `spark` 普通探索可直接对话；handoff 以准确接力为准；cross-review 同时核对当前实现、回归与本轮新问题。
- Rust 区分阻塞锁和异步锁；Kotlin reference 精简为关键语义，并纠正 data object / data class inheritance 的版本说明。
- 命令名、16 个 skill 目录与默认产物路径保留；用户依赖旧的逐阶段确认方式时，可明确要求“先确认再实施”。

### 0.11.0

**Skill set distilled and tightened**:
- 16 个 SKILL.md 统一删除重复触发段、关系说明、伪候选和数量型仪式，总行数显著下降
- `principles` 增加 consumer-driven abstraction、domain boundaries、framework edge、lifecycle ownership
- 修复 chain 矛盾:`scope → acceptance → design?`，`commit-gate` 不再冒充 `handoff`
- `review` 收敛为类型 / 构造 / 能力接口 / 组织边界 / 兼容重构五维触发式判据
- Rust 增加 task lifecycle、MSRV/public trait compatibility；删除 parking_lot 和“三实现才抽 trait”绝对规则
- Kotlin 扩展函数恢复为合法解释器入口，并补 coroutine owner、binary compatibility 和虚拟时间测试
- `spark` 不再强制伪造第二方案；`pua` 不再把复杂度当作专业性的代理

### 0.10.0

**`cross-review` skill added**:
- 新增 `aye:cross-review`:多 agent ping-pong review 的文件协议,用于 CC / Codex / Claude / human 多轮互审
- 约定 `review.md` 只放当前真相(Decision Log / Open Items / Latest Round / History),历史轮次落 `reviews/NNN-*.md`
- 默认每轮只读 `feature.md` + `review.md` + 最新 response,禁止默认读全历史,控制 token
- 明确边界:`review` 提供审查判据,`cross-review` 管轮次状态和文件收敛
- 工作流地图升级为 16 个 skill:Gate 4 + Triggered 7 + Reference 4 + Nav 1

### 0.9.0

**`spark` skill added**:
- 新增 `aye:spark`:feature 之前的可选想法探索层,从 raw idea / inbox 条目展开成 proposal/spec
- 产物默认落 `docs/sparks/<YYYY-MM-DD>-<slug>.md`
- 明确边界:不承诺、不写 feature.md、不进 scope、不写代码;完成后默认 stop
- 工作流地图升级为 15 个 skill:Gate 4 + Triggered 6 + Reference 4 + Nav 1
- `flow` 同步说明 Phase 0 可选层:`inbox` capture 与 `spark` exploration 没有强制顺序,都可跳过

### 0.8.0

**Codex compatibility added**:
- Added `plugins/aye/.codex-plugin/plugin.json` and `.agents/plugins/marketplace.json`
- Moved the plugin body under `plugins/aye/` so Claude Code and Codex marketplaces resolve the same skills
- Kept the Claude Code marketplace intact so Claude Code continues to use `/aye:<skill>`
- Reworded host-specific interaction rules as Claude Code `AskUserQuestion` + Codex interactive-tool/text fallback
- Replaced `CLAUDE.md`-only project guidance with host-neutral project agent instructions examples
- Renamed skill tail sections from "Auto-invoke chain" to "Auto-invoke / next-step chain" for dual-host semantics

### 0.7.2

**`inbox/SKILL.md` 加 "框架边界" 段**(明示 aye 不管 feature 之前的 prioritize):
- inbox 只是 raw capture(可选),故意不承载 prioritize 视图(跨 feature 候选主题归类 / priority 排序 / "已想清楚但等业务触发" 等待区 / WIP / sprint)
- 使用者要这类视图 → 自加 `docs/kanban.md` / `backlog.md` / `_deferred/` 目录皆可,aye 不替你拍
- 理由:prioritize = 项目特定 + 团队偏好 + 业务节奏,框架强加 schema 必然漂移成 Jira
- aye 框架本分:**inbox(可选)→ feature(承诺)→ ship**,中间不扩宽
- 触发场景:dogfood 项目(shield-rs)出现 kanban.md backlog 扩段提议,审视后判定越界,只动文档明示边界,零 schema 改动

### 0.7.1

**handoff → inbox 自动 chain 去除**(修正 v0.7.0 设计矛盾):
- `handoff` 不再扫散落想法关键词,不再抛 AskUserQuestion 引导写入 inbox
- 理由:`inbox` 自身铁律是"用户主动喊才走"(基于 `principles` 的"AI 不当提问机");handoff 自动钩等于替用户决定"该不该 capture",违反 inbox 自身设计 + handoff "只交事实,不替下家拍决策"主旨
- 用户在 handoff 后想 capture 散落想法,**自行**喊 inbox 关键词("记一下 / 先存着 / 以后做 / inbox / 散落想法 ...")
- Trade-off:当场没想起 → 想法可能丢失,但这是 GTD 模型自洽代价(capture 责任在用户,工具不替他记),小于"每次 handoff 都问一遍"的噪音 + 误捕获代价
- 同步更新:`handoff/SKILL.md` 删自动钩逻辑改为立场段、`inbox/SKILL.md` 「与 handoff 的 chain」段重写为「与 handoff 的边界」、`flow/SKILL.md` chain map 图示 + inbox 行说明

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
