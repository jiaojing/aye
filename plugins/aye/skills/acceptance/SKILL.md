---
name: acceptance
description: '钉死可执行 DoD(Phase 2 闸门 2)。触发关键词:"DoD / 验收 / acceptance / 怎么验证 / 确认目标 / 验收标准"。在 scope 后、写代码前，把 feature 的用户视角 acceptance 细化成具体且带 command/操作的 checklist；公开 API、持久化、外部接口、多个边界或模糊兼容要求必须使用。'
---

# Acceptance

定义“什么算完成”。测试绿只是基础，不自动证明用户行为、兼容性和迁移都完成。

## 何时必走

- 改公开 API、持久化 schema、外部协议或配置契约；
- 跨多个变体或边界 case；
- DoD 含“性能不退化 / 兼容老调用方 / 正常工作”等模糊表述；
- reviewer 需要可执行的 PR 验收清单。

已有失败测试的 bug fix、AC 已带 command、语义不变的小重构、typo 和纯文档任务可跳过完整仪式，但仍要明确验证方式。

## DoD 质量

每条必须回答：

1. **具体行为**：输入、边界和预期输出是什么？
2. **观察方式**：自动测试、命令还是明确手动步骤？
3. **通过条件**：怎样判定成功，而不是“看起来正常”？

```markdown
## Acceptance Criteria

- [ ] DoD-1: <具体行为/产出> | 验证：<command 或步骤>
- [ ] DoD-2: <边界/兼容要求> | 验证：<command 或步骤>

## Out of scope
- <不参与本次验收的真实相关项>
```

只写本次改动特有的验收点，不拿“能编译 / 普通测试绿”填数量。DoD 数量暴涨是 scope 过大的信号，不规定机械数量。

## 按风险补 DoD

| 改动 | 至少覆盖 |
|---|---|
| 公开 API | 外部 consumer 编译/兼容检查、deprecation/migration |
| 持久化或 config | 旧数据/旧配置读取、迁移、回滚 |
| 后台任务 | 取消、timeout、graceful shutdown |
| CLI/HTTP/GUI adapter | 从真实入口做内存或集成测试 |
| 并发代码 | 确定性同步；不靠 sleep 猜时序 |
| 不可信输入 | containment、权限和非法输入 |
| 版本敏感框架 | manifest 版本与 API 代际一致 |

遇到会改变 DoD 的未知项时按 `pick` 协议让用户拍板，回答后写回清单；不要保留 TBD。

## 下一步

- 大功能、多方案、公开 API、schema 或跨模块设计：进入 `design`。
- 其他任务：开始实施。
- 实施完成后逐条验证 DoD，再进入 `commit-gate`。
