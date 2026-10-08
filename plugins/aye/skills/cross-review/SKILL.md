---
name: cross-review
description: '管理用户要求的多 agent 审查轮次。维护 review.md 当前索引和 reviews 历史，跟进 open items，并检查本轮相关变更；设计判据由 review 提供。'
---

# Cross Review

维护审查当前事实与可追溯历史。轮次协议服务于用户请求的协作，不自动制造新轮次或扩大任务。

## 文件协议

沿用已有位置；默认：

```text
docs/features/<slug>/
  feature.md / design.md     # 存在时作为目标与决策来源
  review.md                  # 当前索引
  reviews/NNN-agent.md        # 历史轮次
```

`review.md` 维护已决定事项、Open Items、最新结论与 History；历史 round 文件 append-only，不回改。沿用项目已有 schema，没有索引时可采用：

```markdown
# Review Index: <topic>

Last updated: <date / round>

## Decision Log
| ID | Decision | Source |
|---|---|---|

## Open Items
| ID | Item | Status |
|---|---|---|

## Latest Round Summary
<简短结论与剩余问题>

## History
| Round | File | Source | Date |
|---|---|---|---|
```

## 每轮

- 读取目标/决策来源、当前索引和最新 response，以及本轮 diff、相关实现、契约、consumer 与验证证据。
- 优先跟进 open items，同时检查修复回归和本轮新增的实质问题；历史按追溯需要读取，不把“少读历史”当作限制源码阅读。
- 使用当前最大编号 +1 写新 round，记录输入、结论、finding/response、决策变化和剩余事项。
- 同步更新当前索引及 History，避免 round 与索引描述不同事实。
- 只有已决定且影响范围、验收或任务的变化写回相关文档；提议保留为 open item。

Finding 按 `review` 给位置、触发条件、风险、证据和最小修复。记录“已修复”时核对当前实现与验证，不能只凭 response 声明关闭。

## 对用户报告

说明本轮结论、新增文件、索引变化与剩余事项，按影响提供必要证据。问题解决后明确收敛，不为满足轮次数量继续审查。
