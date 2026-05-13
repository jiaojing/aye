---
name: acceptance
description: 钉死可执行 DoD(Phase 2 第二步)。触发关键词:"DoD / 验收 / acceptance / 怎么验证 / 确认目标 / 验收标准"。在 scope 对齐之后、写代码之前,把 feature.acceptance(用户视角粗版)细化成带 command 的 checklist。Auto-invoke: design(大功能可选闸门 2.5)或开始写代码.
---

# Acceptance

AI 协作的**闸门 2**:动手前钉死"什么算完"。

测试绿不等于完成。"看起来 work" 不等于完成。**完成的定义必须在写代码前显式钉死**,不是事后看心情。

---

## 为什么需要

AI 容易"看起来对就交付":
- 跑了几个 happy path → 觉得 work 了
- 实现了一个变体 → 忘了边界 case
- 改了主路径 → 忘了相关迁移 / 文档

人也会犯这种错,但 AI 更甚——因为 AI 没有"这个改动会影响什么"的长期工作记忆,需要**显式 acceptance criteria 当外置 checklist**。

---

## 何时必走 / 何时跳过(硬清单)

**必走**(任一命中):

- 改公开 API / 持久化 schema / 外部接口
- 跨多个变体 / 边界 case
- DoD 描述含模糊词("性能不退化" / "兼容老调用方")
- 多人 review 流程(PR 描述需要可执行 DoD 给 reviewer)

**跳过**(任一命中即可直接进写代码):

- 文档型 task:写 / 改 markdown,DoD = "内容齐全,结构对",没 command 可跑
- `feature.md` AC 已细化到 command:三要素已满足,直接当 DoD 用,不重复
- bug fix 已有失败 test 复现:让 test 绿就是 DoD
- 简单重构(< 50 行,语义不变,有覆盖测试):DoD = 测试仍绿
- 1 行 typo / 局部重命名:DoD 显然 = "改完编译过"

跳过 ≠ 没 DoD。心里仍要清楚"完成 = 什么",只是不必走全套仪式。

---

## 触发场景(用户口语)

`scope` 完成后 conditional 触发本 skill,或用户直接喊:

- "DoD / 验收 / 什么算完 / acceptance / 怎么验证"
- "确认目标 / 完成标准"

但**先看"何时必走 / 跳过"硬清单**——简单 task 即使触发也跳过仪式。

---

## DoD 三要素

每条 DoD 必须满足:

### 1. 具体

| ❌ 模糊 | ✅ 具体 |
|---------|---------|
| "代码 work" | "POST /users 在 email 重复时返回 409,body 含 error_code: EMAIL_TAKEN" |
| "性能不退化" | "p99 latency 不增加 > 10%(基线 50ms)" |
| "测试通过" | "tests/auth_flow.rs 新增 3 个 case 全绿" |

### 2. 可测试

DoD 写完,问:**怎么验证它满足了?**
- 自动化(测试 / 命令断言)→ 最好
- 手动操作(curl 一下 / UI 点一下)→ 可以,但写下命令
- "感觉对了就 OK" → ❌ 不算 DoD

### 3. 可验证(give me the command)

最好附上**怎么跑出来**:

```
DoD-1: cargo test --test auth_flow 全绿
DoD-2: curl -X POST /users -d '{"email":"a@b.com"}' 两次,第二次返回 409
DoD-3: 文档 docs/auth.md 同步更新
```

---

## 写在哪

按场景选合适位置:

| 场景 | 写在 |
|------|------|
| 项目内部 backlog | feature.md 的 Tasks 条目下 |
| PR 流程 | PR description 顶部 "Acceptance" section |
| 对话内 ad-hoc 任务 | 对话开头,scope 对齐后下一段 |
| 长任务 | 任务列表(每个子任务自己的 DoD) |

**不要**只放在脑子里——AI session 切换 / 自己几小时后回来,都会忘。

---

## DoD 模板

### 通用模板

```markdown
## Acceptance Criteria

- [ ] DoD-1: <具体行为/产出> | 验证:<command 或操作>
- [ ] DoD-2: <具体行为/产出> | 验证:<command 或操作>
- [ ] DoD-3: <具体行为/产出> | 验证:<command 或操作>

## 已知 out-of-scope(不进 DoD)

- <主动排除的相关项,留给独立任务>
```

### 实例(integration test 类任务)

```markdown
## Acceptance Criteria

- [ ] DoD-1: tests/end_to_end.rs 新建,3 个 case 全绿
      | 验证:cargo test --test end_to_end
- [ ] DoD-2: 不修改 src/ 下任何文件(纯外部 consumer 视角测试)
      | 验证:git diff --stat src/ 为空
- [ ] DoD-3: README "Quick Start" 段落引用其中一个 case 作 demo
      | 验证:grep "tests/end_to_end" README.md

## Out-of-scope

- 性能基准(独立任务,见 backlog 中 perf-baseline)
- 校准链路 case(独立任务)
```

---

## DoD 不确定点

DoD 写到中途遇到边界拿不准——例:这个测试覆盖到哪一层?这条 ACR 算 must-have 还是可拖下一轮?返回值边界 case 怎么定?——**按 `principles` 的「AI 不当提问机」准则抛**:候选 + trade-off + 推荐 + 理由。环境支持 `AskUserQuestion` 时优先用工具收拍板,答完写回 DoD。

**反模式**:DoD 模糊段写"待定 / TBD" → 等于把决策推到实施阶段才暴露,成本最高。

---

## 反模式

### 反模式 1:DoD = "做完 X"

```
❌ DoD: 实现 user registration 功能
```

"实现"不是 DoD,是任务描述。DoD 必须答"怎么知道实现完了"。

### 反模式 2:事后凑 DoD

```
代码改完 → "我们的 DoD 是什么来着" → 现编一个让自己通过
```

DoD **必须前置**。事后凑 = 没 DoD。

### 反模式 3:DoD 全是"代码 work"

```
❌ - 代码编译过
   - 测试不挂
   - 看起来正常
```

这是**编程基本功**,不是 DoD。DoD 应该说**这次改动特有的**验收点。

### 反模式 4:DoD 太多导致工作量爆炸

```
❌ 12 条 DoD,做完发现工作量 = 3 天
```

DoD 多 = scope 太大,**回去拆 task**,不是硬上。一个 task 通常 3-5 条 DoD。

---

## 与其他 skill 的关系

- **`flow`**:地图。本 skill 是闸门 2(scope 对齐**之后**)。
- **`scope`**:闸门 1。先 scope 后 DoD——scope 定边界,DoD 定完成度。顺序不能反。
- **`commit-gate`**:闸门 3(commit 前)。commit 前回头核对 DoD 是否全勾掉。
- **`feature`** 的 Acceptance 章节(用户视角粗版)是本 skill 的输入,本 skill 把它做成可验证的带 command checklist。

---

## Auto-invoke chain

完成本 skill 后,LLM 决定下一步:

- **大功能**(改公开 API / 持久化 / 跨 crate / 多方案纠结)→ `design`(产 design.md + 多轮敲定)
- **正常实施** → 开始写代码,中途可调 `review` N 次

写代码完 + 测试绿 → `commit-gate`。
