---
name: flow
description: '解释 aye 的工作流并定位下一步。用于用户请求流程导航、恢复任务或不知道如何展开时；按目标、风险与既有授权选择需要的 skills。'
---

# Flow

这是流程关系的统一入口。具体判据、文档格式和授权细节由各 skill 维护，不在这里重复定义。

## 恢复任务

请求恢复工作时优先读取相关 feature/任务与最新 handoff，核对当前代码和工作区。当前请求与历史任务无关时，不把旧范围套入新任务。

需要待选事项概览时，可报告 `docs/inbox.md` 中 `## Inbox` 的条目数量；用户需要内容时再展开。不为每个新 session 自动扫描所有文档。

## 工作流

```text
未承诺的想法 → spark（按请求保存或探索）
已决定做、目标模糊 → feature（澄清）
目标与授权明确 → 按需要补 scope / acceptance / design
               → implementation ↔ review（按风险）
               → 验收结果与交付说明

请求 commit / push → commit-gate（准备结果并执行授权的操作）
请求暂停 / 接力    → handoff
```

这是按需要选择能力的地图，不是每个任务都必须经过的闸门链：

- **明确任务**：复用已有范围、验收与方案，直接完成必要实施和验证。
- **有歧义**：在受影响的目标或范围上对齐，不重复确认已经决定的内容。
- **重要设计变化**：明确验收含义，用 design 收敛边界、契约、并发或持久化取舍；设计与验收可以迭代。
- **需要接力**：更新已有任务与决策记录，不机械创建全套文档。
- **用户限定阶段**：只探索、规划、review 或先确认时，停在对应产物；阶段切换不扩张授权。

## Skill 职责

| 类型 | Skill | 职责 |
|---|---|---|
| Clarification | `feature` | 已决定要做的需求与交付拆分 |
| Clarification | `scope` | 本次修改范围与真实未知项 |
| Clarification | `acceptance` | 可执行验收与验证缺口 |
| Design | `design` | 有实质取舍的技术方案 |
| Git | `commit-gate` | 可 review 的提交结果与操作授权 |
| Exploration | `spark` | 未承诺想法的保存、探索与承接 |
| On request | `handoff` | 事实与已有决策的接力 |
| On request | `cross-review` | 多 agent 审查轮次与当前索引 |
| On request | `pua` | 领域证据与方向校正 |
| Reference | `principles` | 跨语言工程取舍与领域边界 |
| Reference | `review` | 有证据的设计审查 |
| Reference | `rust-principles` | Rust 特化判据 |
| Reference | `kotlin-principles` | Kotlin 特化判据 |
| Navigation | `flow` | 当前工作流 |

## 交付节奏

围绕一个可独立验收和 review 的结果推进；task 不绑定 session 或 commit 数量。已有决策足够时继续工作，只对真正影响目标或授权的事项提问。

项目可以采用已有的 issue、ADR、backlog 和文档布局。aye 不要求维护 sprint 或额外状态系统。
