---
name: design
description: '大功能技术设计敲定(Phase 2 闸门 2.5，可选)。触发关键词:"得出个方案 / 先设计一下 / 拿不准用啥 / 多方案纠结 / 大功能 / 改公开 API / schema 改 / 跨 crate / 架构选型 / 技术选型 / 实现方案 / 需要画图 / 设计文档"。在 acceptance 后、写代码前产出 design.md，收敛技术问题、真实方案空间、决策和可实施细节。'
---

# Design

在 acceptance 已明确后决定技术 **how**。只在高破面或确有方案选择时使用，避免为显然的小改制造仪式。

## 何时必走

- 改公开 API、协议、错误码或持久化 schema；
- 跨 crate/模块边界；
- 存在两个以上真实可行方案；
- 需要架构图、数据流或状态机才能讲清；
- trait/enum、Builder、并发模型、依赖选择等仍未定。

局部 bug、单文件小改、方案显然或已有 design.md 的小修可跳过。

## Step 0：理解现状

设计前先检查：

- 模块/crate 结构和 composition root；
- 现有配置、错误、client、trait 和测试模式；
- 新能力的注册/接线位置；
- manifest/lockfile 中的语言、工具链和框架版本；
- 公开 consumer、持久化格式和兼容约束。

复用现有模式；不要在 spec 中重新发明已有能力。

## design.md 模板

```markdown
# Design: <feature>

> Linked feature: <path>

## 问题
<需要技术决策的原因，不重复 user problem>

## 方案空间
### 方案 A: <形态>
- 优点：
- 缺点/风险：
- 试剂量：

### 方案 B: <仅在它是真实可行候选时存在>
...

## 决策
**选择：<方案>**

- 判据：可测试性 / 可读性 / 一致性 / 简洁性 / 可逆性
- 排除其他方向的具体原因：
- Assumptions：

## 实施细节
- 模块边界和 composition root：
- 关键 API / 数据类型 / 错误：
- 数据流、状态机或算法：
- 测试入口：

## 风险与回滚
- 风险：
- 缓解：
- 回滚条件：
```

只有一个合理方案时，说明排除条件即可，不制造明显劣质的 B。小决策按 `pick` 协议收敛并写回“决策”。

## 条件化设计要求

| 命中场景 | 必须回答 |
|---|---|
| 后台任务/并发 | 谁启动、取消、等待；timeout 和失败传播 |
| 外部 I/O | timeout、重试边界、错误映射、shutdown |
| 框架接入 | adapter 边界、进入核心的领域类型 |
| 公开 API | consumer、兼容策略、机械检查命令、migration |
| 持久化/schema | migration、旧数据、回填、回滚 |
| 缓存 | freshness、失效和一致性 |
| 不稳定框架 | 精确版本、升级策略、禁止混用的 API 代际 |

公开 API 和 schema 细节较大时分别拆到 `contracts.md` / `schema.md`，design.md 只保留决策和链接。

## 执行

1. 基于 Step 0 提出完整草稿，列出真实 assumptions。
2. 用户 review 并收敛必要决策；一次通过是允许的。
3. 用户确认后写入 `docs/features/<slug>/design.md`，才开始实施。

伪代码、接口签名和流程图可以进入 design；完整可编译实现留到编码阶段。

## 下一步

design.md 确认并落地后开始实施；过程中可按需调用 `review`，完成并验证 acceptance 后进入 `commit-gate`。
