---
description: 迭代终点交接(Phase 2 自动 + 手动 fallback)。**主**:commit-review push 完成后**自动**触发,迭代结束 = session 结束 = 自然交接。**fallback**:用户说"今天到这 / 收 / 暂停 / 下次继续 / context 满了 / handoff" 时手动触发。输出当前 commit 状态 + 下条 backlog 起点 + 注意事项,< 10 行。规则:只交事实,不预设方案,不替下个 session 拍决策。
---

# Handoff

Session 交接给下一个 AI session(或下次自己)的规则:**只交事实,不替下家拍决策**。

---

## 为什么需要

交接是**信息不够时拒绝拍板**。错误的交接两种:

1. **信息不够还推荐方案** = sycophancy(讨好式补全)。下家不知道你信息不足,以为推荐有依据,被引导接受错误方案。
2. **trade-off 矩阵 / 决策点 / 我倾向** = 替下家做了脑力工作,下家本应是 fresh eyes 自己评估。

下家应该读完代码 + 文档后再开口提决策点和方案。**不是被前任的预设引导**。

---

## 触发场景

- Context 即将耗尽(预警信号:工具响应变慢 / 能感知到 context 已大)
- 用户主动说"我们今天到这,下次继续"
- 长任务跨多个 session(比如 epic 跨多 PR,每 PR 一 session)
- 切换 AI 实例(主线程 → 子 agent)

**不触发**:对话内的轻量切换(从设计讨论切到写代码,同 session 内不用 handoff)。

---

## 应该交什么(事实)

```markdown
## Today

- 完成 commit:abcd123 / def4567
- 已 push:abcd123 到 origin/feature-x
- 未 push:def4567(本地待 review)

## 状态

- cargo test --workspace --lib:绿
- worktree:clean / 有未 stage 改动

## 下个 session 起点

- 下一条 backlog:F2 端到端 integration test
- 详细见 docs/kanban.md 第 30 行
- 相关文件:src/shield-pricing/tests/(待新建 end_to_end.rs)

## 注意约定

- 用词:近期约定"sprint" 不用"刀"字相关
- 流程:commit / push 前等用户点头(详见 commit-review skill)
```

**< 10 行**。

---

## 不应该交什么(预设)

### ❌ 决策点 + 我倾向

```
✗ 决策点 1:Story 1+2 同一 PR 还是分开?
   我倾向:同一 PR(理由 …)
✗ 决策点 2:校准链路放本 sprint 还是下个?
   推荐:下个(理由 …)
```

下家应该读完代码 + 上下文后**自己**评估这些决策点,前任的"倾向"是污染。

### ❌ Trade-off 矩阵

```
✗ 方案 A:优点 X,缺点 Y
   方案 B:优点 P,缺点 Q
   我推荐 A
```

如果信息足够拍板,前任 session 自己应该已经动手了。如果还在交接 = 信息不足。**信息不足时排矩阵 = 拍脑袋假权威**。

### ❌ Leading question

```
✗ "我觉得用 newtype 封装更好,你怎么看?"
```

引导式提问,下家容易"嗯,你的看法对"附和。应该:**陈述事实,让下家提问**。

### ❌ 推荐执行顺序("先做 A 再做 B")

下家有自己的判断。前任陈述事实即可,不替下家排期。

---

## 模板

```markdown
## Today
<commits / push 状态 / 1-2 行重点>

## 状态
<test 状态 / worktree 状态>

## 下个 session 起点
<下一条 backlog 编号 + 一句话标题 + 详细位置(文件 + 行)>

## 注意约定
<最近犯过的错的提醒,1-3 条>
```

**硬上限:10 行**。超了 = 在偷塞决策。

---

## 反模式合集

| 反模式 | 为什么有害 |
|--------|----------|
| 包揽分析 + 推荐 | 信息不够还推荐 = sycophancy |
| 列 4 个决策点 + 我倾向 | 替下家做了脑力工作 |
| Trade-off 矩阵 | 制造"前任已分析过"的虚假权威 |
| 推荐执行顺序 | 越权,下家自有判断 |
| 重复用词约定 / 流程铁律 | 交接不是教学,放 1-2 条提醒就够 |

---

## 与其他 skill 的关系

- **`flow`**:地图。handoff 是横切元规则,任意一步都可能触发交接。
- **`commit-review`**:交接前必须确认 commit / push 状态,本 skill 直接引用 commit-review 的"测试绿 + diff 摆出 + 等点头"循环结果。
- **`scope` / `acceptance`**:交接时如果 scope / DoD 已对齐过,**只引用不复述**(下家会去看原文)。

---

## Auto-invoke chain

handoff 是 chain **终点**,完成后:

1. 输出交接信息(< 10 行)
2. session 可关闭
3. 下次新 session 起来,从 handoff 信息恢复上下文,继续从 Phase 1(看板选下条)或 Phase 2(spec 已写好的 story)开始

handoff 本身**不 chain 到任何 skill**——它是 session 自然终点。
