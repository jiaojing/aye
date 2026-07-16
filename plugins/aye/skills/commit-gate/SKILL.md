---
name: commit-gate
description: 'commit + push 闸门(Phase 2 末尾)。触发关键词:"测试绿了 / commit / 提交 / push / 推送 / 可以提交了 / 可以推了"。任何 git commit/push 前先展示 diff 和验证结果，等待用户明确授权；大改动、公开 API 或持久化变更先调用 review。push 后回写 feature task/acceptance，不自动生成 handoff。'
---

# Commit Gate

测试和 DoD 通过后，先准备可 review 的 diff；只有用户明确授权才执行 commit 或 push。两项授权彼此独立，不从之前一次批准中推断。

## 闸门

```text
实施 → tests/DoD → 准备 diff → 用户 review
     → 明确 commit 授权 → commit
     → 明确 push 授权   → push + feature 回写
```

大改动（>200 行、公开 API、持久化）先用 `review` 检查设计和破面，再展示 diff。

## Diff 展示

| 改动量 | 展示 |
|---|---|
| <50 行 | 直接展示关键改动 |
| 50–200 行 | stat + 关键 hunk 摘要 |
| >200 行 | 文件清单，逐文件说明关键 diff |

不要只给 stat 让用户自己寻找变化；说明每个文件为何存在、风险在哪里。

## 自检

- [ ] 编译/测试和本次 Acceptance 全部通过。
- [ ] 公开 API、schema、config 或 toolchain 变更已运行对应兼容检查。
- [ ] diff 没有越出已确认 scope。
- [ ] 没有调试代码、无意 TODO、生成物或秘密。
- [ ] 用户已有修改没有被覆盖或混进本次意图。

然后等待明确的 `commit` 指令；需要拆 commit 时先提出拆分依据。不要默认 direct。

## Commit Message

- 遵循仓库语言；本项目默认中文。
- 标题不超过 15 字，只概括核心动作。
- 正文只解释 why，不复述文件和技术步骤。
- 不把文档同步、配置同步等配套动作列成主题。
- 不添加 `Co-Authored-By` 或 AI 标注。

```text
<核心动作> (<可选 issue>)

<一句话说明为什么需要这次改动>
```

## Push 后

1. 报告远端和提交范围。
2. 找到本轮 `docs/features/<slug>/feature.md`，把对应 task 标为 `done @ <sha>`，完成的 acceptance 一并勾选。
3. 不主动宣布 session 结束，不自动生成摘要。

用户说“今天到这 / 暂停 / handoff”时才进入独立 `handoff` skill。
