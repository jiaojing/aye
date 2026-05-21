---
name: handoff
description: '迭代终点交接(用户主动触发)。触发关键词:"今天到这 / 收 / 暂停 / handoff / context 满了 / 下次继续"。**双输出**:写文件 `docs/features/handoff-<date>.md`(持久化,下个 session 直接读)+ inline 摘要(本 session 用户可见)。< 10 行,只交事实,不替下家拍决策。散落想法不在本 skill 处理——用户自行喊 `aye:inbox` 关键词 capture。'
---

# Handoff

Session 交接给下一个 AI session(或下次自己)的协议:**只交事实,不替下家拍决策**。

下家应该 fresh eyes 读完代码 + 文档后再开口提决策点和方案——**不该被前任的预设引导**。

---

## 触发

用户明示要关 session 时:

- "今天到这 / 收 / 暂停"
- "handoff / 交接"
- "context 满了 / 下次继续"

`commit-gate` push 完后**默认不触发**摘要。除非用户喊上面的关键词,LLM 不主动摆收尾姿态——避免"每个 commit 都给摘要"的噪音。

---

## 双输出(关键)

handoff 触发时**同时**做两件事:

1. **写文件** `docs/features/handoff-<YYYY-MM-DD>.md`(持久化)
   - 下个 session 起来直接 Read 这个文件,无需翻对话历史
   - 跟 features/ 同目录,domain coherent(handoff 是 feature 工作流的产出物)
   - 同日多次触发 → 覆盖(瞬态状态,git 自带历史)
2. **inline 同样内容**(本 session 用户可见)

下个 session 进入时,AI 应自觉 `ls docs/features/handoff-*.md | tail -1` 找最新 handoff,Read 拉到 context。或在 feature.md "Notes" 段引用最新 handoff 路径。

---

## 摘要模板(< 10 行)

```markdown
## Today
<commits / push 状态 / 1-2 行重点>

## 状态
<test 状态 / worktree 状态>

## 下个 session 起点
<下一条 task 编号 + 一句话标题 + 文件 + 行>

## 注意约定
<最近犯过的错的提醒,1-3 条>
```

**硬上限:10 行**。超了 = 在偷塞决策。

---

## 反模式协议(只交事实)

不交:

- ❌ 决策点 + "我倾向" / "推荐"
- ❌ Trade-off 矩阵
- ❌ 推荐执行顺序("先做 A 再做 B")
- ❌ Leading question("我觉得用 X 更好,你怎么看?")
- ❌ 用词约定 / 流程铁律(交接不是教学,1-2 条提醒就够)

**重复**:已在 commit / PR / `feature.md` / ADR 里的内容,**引用路径或 commit-sha 即可,不复述**。

---

## 何时跳过

- 单 task 内 push 后继续做下条 → 不触发
- 短 commit(typo / doc-only) + 用户没喊 → 不触发
- 用户明示"继续 / 不交接 / 接着做" → 跳过

---

## 散落想法不在本 skill 处理

session 收尾时对话里常冒出**未承诺的散落想法**("以后做"、"还有个想法"、"这块可以...")——既不属于当前 feature,又不该当场升级成新 feature。这些归 `aye:inbox` 管。

**handoff 不自动钩 inbox**:

- 不扫关键词、不抛交互式选择问题引导 capture
- 理由:`inbox` 自己的铁律是"用户主动喊才走"(基于 `principles` 的"AI 不当提问机");handoff 自动钩等于替用户决定"该不该 capture",违反 inbox 自身设计 + handoff "只交事实,不替下家拍决策"主旨
- 用户在 handoff 后想起散落想法,**自行**喊 inbox 关键词("记一下 / 先存着 / 以后做 / inbox / 散落想法 ...")正常走 inbox capture
- 当场没想起 → 想法丢失是 GTD 模型的自洽代价:capture 责任在用户,工具不替他记

---

## 与其他 skill 的关系

- `commit-gate`:本 skill 在 commit-gate push **之后**触发(用户主动喊)。commit-gate 默认行为是短确认 + 回打勾,**不带摘要**
- `feature.md`:摘要"下个 session 起点"指向 feature.md 待办 task
- `inbox`:**不自动 chain**。用户在 handoff 后想 capture 散落想法,自行喊 inbox 关键词

---

## Auto-invoke / next-step chain

handoff 是 chain **终点**,完成后:

1. 输出摘要(< 10 行)
2. session 可关闭——但**不强制**,用户可继续
3. 下次新 session 起来,从 feature.md 拉下条 task,直接进 scope
