# 2026-05-aye-flexible-chain：chain 松绑 + 复活 handoff + 加 pua skill

## Feature

aye 0.3.x 是 strict chain auto-invoke——`feature → scope → acceptance → ... → commit-review` 每步 push 下一步。本 session dogfood 暴露多个卡点：

- 简单 task 上闸门空跑（用户主动合并 task 规避，文档型 task 走全套是噪音）
- 不同 LLM session 行为不一致（同事的目录结构跟我们不同——SKILL.md 是 prompt 不是机制）
- handoff 内嵌 commit-review 后产生"每个 commit 摆收尾姿态"副作用（0.3.1 双模式分流修了，但绕弯路）
- 缺一条**用户高频用、扩写真复杂、口语难替代**的元认知 skill（用户经常喊"以终为始 / 看行业标准 / 站领域专家 / 不要捡简单的"——本 session 实证 8 处）

需要：
- 把 chain 从 strict 松绑成 **必走 + conditional** 两层（必走只留 `feature` 起 + `commit-review` 收，其他可选闸门按场景触发）
- 复活 `handoff` 为极简独立 skill（中文，inline 摘要）
- 新增 `aye:pua`：动态调用领域专家 + research + 以终为始

### Users

- 用户本人（aye 主用户）
- 同事（团队 dogfood）
- 配对的 AI（chain 松绑后行为更灵活，pua 提供元认知触发点）

### Scope

**In**

- chain 改 strict → conditional：`scope` / `acceptance` / `design` SKILL.md 加"何时必走 / 何时跳过"硬清单段
- 各可选闸门 description / auto-invoke chain 段同步改造（场景化触发）
- **复活 `handoff`** 独立 skill（中文极简版，~30-50 行，inline 摘要）
- **新增 `aye:pua`**：以终为始 + 站领域专家 + 做 research + 拒绝"简单 / 直接"
- `commit-review` 砍 0.3.1 双模式段（handoff 接管摘要）
- `flow` chain map 保持不变；skill 列表加文字表格标"必走 / 可选"
- 同步 `README` / `plugin.json` bump 0.4.0

**Out**（钉死防越界）

- ❌ `aye:debug` / `aye:prototype` / `aye:zoom-out`（dogfood 拷问后跳过——用户实际不会用 / 内容不够复杂 / 口语已覆盖）
- ❌ `design-review` 加 deletion test（dogfood 实证我自己连续误判两次,LLM 用必误判,放下）
- ❌ 改 `principles` / `rust-principles` / `kotlin-principles` 内容（只可能加 cross-ref）
- ❌ `feature` SKILL.md 瘦身（留 follow-up）
- ❌ 改 plugin marketplace / cache 行为（平台层）
- ❌ 改 `commit-review` / `feature` 必走地位（动 git 边界保护必留）
- ❌ 重新发明 mattpocock 整套（实证大部分对 aye 价值低，**只取 deletion test 一条判据**）

### Constraints

- 所有新内容用中文（保留英文术语：`feedback loop` / `deletion test` / `pua` 等）
- bump 0.4.0（minor，0.x 实验期 anything goes）
- mattpocock 借鉴层只取**核心判据**（deletion test），不照搬动作型 skill

### Open questions（已答，留 trail）

- ✅ Q1: handoff 形态 = **inline 摘要**（对话内，不留文件）
- ✅ Q2: `aye:debug` = **跳过**（场景模糊 + AI 行为难改 + 双重失效）
- ✅ Q3: `aye:prototype` = **跳过**（用户实际不会用）
- ✅ Q4: `aye:zoom-out` = **跳过**（口语已覆盖，扩写冗余）
- ✅ Q5: bump = **0.4.0**
- ✅ Q6: chain map = **(d) 不动图，skill 列表表格标必走/可选**
- ✅ Q7: `aye:pua` 触发词 = **全留**（以终为始 / 什么是真正对的 / 看行业标准 / 主流怎么做 / 开源项目 / 作为 X 领域专家 / 站在专家视角 / 不要捡简单的 / 做 research / 调研一下）

## Acceptance

- [ ] AC-1: `skills/handoff/SKILL.md` 复活（中文极简版，~30-50 行，inline 摘要协议）
- [ ] AC-2: `skills/pua/SKILL.md` 新建（中文，覆盖：以终为始 / 领域专家 lens / research / 拒绝简单 / 反模式协议）
- [ ] AC-3: `scope` / `acceptance` / `design` SKILL.md 头部加"何时必走 / 何时跳过"硬清单段
- [ ] AC-4: 各可选闸门 auto-invoke 段从 strict 改 conditional；`commit-review` 砍 0.3.1 双模式段
- [ ] AC-5: `flow` skill 列表加表格标"必走 / 可选" + 加 `handoff` / `pua` 引用；`README` 同步；`plugin.json` bump 0.4.0
- [ ] AC-6: push origin

## Tasks（按颗粒度铁律切，软线 5-7）

- [ ] T1: 复活 `handoff`（中文极简）+ `commit-review` 砍 0.3.1 双模式段（同一动作的两面：handoff 接管 = commit-review 不再背）
- [ ] T2: 各可选闸门（`scope` / `acceptance` / `design`）加"何时必走 / 跳过"段 + auto-invoke conditional + description 场景化（同质合一刀）
- [ ] T3: 写 `aye:pua` skill（中文）
- [ ] T4: `flow` skill 列表 + 加 handoff/pua 引用 + `README` 同步 + `plugin.json` bump 0.4.0
- [ ] T5: 收尾（push + feature.md 打勾）

## Notes

### 设计来源 + dogfood 拷问

- **mattpocock/skills 仓库**：实证 `handoff` 独立 skill 站得住 + `deletion test` 是好判据
- **本 session 用户视角拷问**：debug / prototype / zoom-out 三条跳过——用户视角 dogfood 比内容判断更狠
  - debug 触发场景模糊 + AI 内化纪律是 model-level 的事
  - prototype 用户用不到（不写需要 prototype 的代码）
  - zoom-out 用户口语已经覆盖（"我对这块不熟 + 站领域专家"），skill 扩写是冗余包装
- **白爷反向给的真 skill**：`aye:pua`——用户本 session 真用过 8+ 次的元认知 prompt，扩写真复杂，口语难替代
- **判据沉淀**：skill 价值 = 用户真口语习惯 × skill 扩写复杂度 的交集

### 0.3.0 / 0.3.1 反思

- 0.3.0 砍 handoff 内嵌 commit-review 是过度精简（动作型 → 双模式分流绕弯）
- 0.3.1 双模式 fix 本质是"让 commit-review 行为像 handoff 不存在"——绕回原点
- 本 feature 0.4.0 复活 handoff 独立 = **承认 0.3.0 决策错，反向修复**

### 关键时间线

- 2026-05-08: feature 启动，dogfood 拷问完毕，所有 Open Q 答完
