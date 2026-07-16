---
name: handoff
description: '迭代终点交接(用户主动触发)。触发关键词:"今天到这 / 收 / 暂停 / handoff / context 满了 / 下次继续"。同时写 docs/features/handoff-日期.md 和 inline 摘要；摘要少于 10 行，只交事实，不替下个 session 拍决策，也不自动 capture inbox。'
---

# Handoff

给下一个 session 留一份短小、事实化的接力点。只有用户主动表示暂停/结束时触发；commit/push 后不自动运行。

## 双输出

1. 写 `docs/features/handoff-<YYYY-MM-DD>.md`；同日再次触发覆盖当前瞬态状态，git 保留历史。
2. 在当前对话 inline 输出相同内容。

```markdown
## Today
<commit/push 和本轮结果>

## 状态
<tests、worktree、阻塞事实>

## 下个 session 起点
<feature/task + 文件位置>

## 注意约定
<最多 1–3 条近期事实>
```

全文少于 10 行。已有信息引用 feature、ADR 或 commit，不复述。

## 只交事实

不写推荐、trade-off、leading question、执行顺序或新决策。下一 session 用 fresh eyes 读取代码和文档后再判断。

散落想法属于 `inbox`：handoff 不扫描关键词、不主动询问 capture；用户明确说“记一下”时再调用 inbox。

完成后停止。用户仍可继续对话，但本 skill 不自动进入下一 task。
