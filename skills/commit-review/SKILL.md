---
name: commit-review
description: commit + push 闸门(Phase 2 末尾)。触发关键词:"测试绿了 / 完成了 / commit / 提交 / push / 推送 / 准备好了 / 可以了 / 改完了"。摆 diff → 等明确点头才动 git。大改动(>200 行 / 改公开 API / 改持久化数据)时先内嵌调用 design-review 做 5 维度审查。包含 commit message 规范(中文 ≤15 字 / 只讲 why / 不带 Co-Authored-By / 副作用改动不进 message)。push 完 auto-invoke: handoff.
---

# Commit Review

AI 协作的最后一道闸门:**改完代码不直接 commit,先把 diff 摆给用户做 code review,得到明确"commit / push"指令才动 git**。

测试绿不是 review 替代品。测试验证逻辑正确,review 验证设计与意图。两者都要。

---

## 触发场景

任何以下情况触发本 skill:

- 改完一处代码,`cargo check` / `npm test` 等已绿
- 即将运行 `git commit` / `git push`
- 文档同步、housekeeping、"小改动"——**任何动 git 的意图**
- 用户明确说"commit"或"push"之前

**反例(常见失守)**:
- 测试绿就 commit
- "顺手 doc 同步"自己 commit
- 单个 PR 内之前 commit 批准过一次,以为后续都默认批准
- 把"我已经测试过了"当 review 替代品

---

## 标准工作循环

```
读代码 → 改代码 → 跑测试(cargo test / npm test / pytest …)
        ↓ 测试绿
    停下来,把 diff 摆给用户
        ↓
    等用户 review + 明确说"commit"或"push"
        ↓
    才动 git
```

每次 commit 和 push **独立征求同意**。一次一件事,不批量授权。

---

## 展示 diff 的方式(按改动量选)

| 改动量 | 展示方式 |
|--------|---------|
| < 50 行 | 直接贴改动前后的 code block,关键段加注释 |
| 50–200 行 | `git diff --stat` 列文件 + 关键 hunk 摘要 |
| > 200 行 | 先列文件清单 + 单文件 `git diff` 逐个过,不要一次倾倒 |

**别只贴 stat 让用户自己 git diff**——你做 review preparation 的工作,不是把活推给用户。

---

## Commit Message 规范

### 6 条硬规则

1. **中文写作**(国际团队按团队约定调整)
2. **标题 ≤ 15 字**:核心动作一句话,可选带 backlog ID(如 `(B-01)` / `(#42)`)
3. **正文只讲 why**:这个 commit **为什么存在** / 解决什么问题
4. **不列技术细节**:不写"删了 X 参数"、"把 Y 迁到 Z"、"改了 N 个文件"——看 diff 就知道
5. **副作用改动不进 message**:文档更新、配置同步、memory 等配套改动**不写进 commit message**
6. **不带 `Co-Authored-By`** 或类似 AI 标注行(覆盖 IDE / 工具的默认行为)

### 模板

```
<核心动作> (<可选 backlog ID>)

<一句话 why:这个 commit 为什么存在>
```

### 正确示例

```
解耦 auth 与 user model (#42)

Auth 应该独立验证 token,不该依赖 user model 字段 ——
session 层不再持有 user 业务状态。
```

### 错误示例

```
✗ 把 UserService 从 api 模块迁到 service 模块    # 说技术细节
✗ 删除 createUser 上未使用的 _legacy_flag 参数   # 说技术细节
✗ 同时:README 更新;CHANGELOG 同步;TODO 调整    # 副作用进 message
✗ Co-Authored-By: Claude ...                    # 不要带
```

---

## 绝不做的事

- ❌ `cargo test` 绿就 `git commit`(忽略 review 闸门)
- ❌ doc 同步 / housekeeping / "小改动"任何理由自作主张 commit
- ❌ 一个 PR 内之前批准过就以为后续 commit 默认批准(每次独立)
- ❌ 顺便 git push(push = share 给他人,更要谨慎)
- ❌ 把"我已经测试过了"当 review 替代品

---

## 输出格式建议

改动完测试绿后,给用户的报告结构:

```markdown
## Diff 摘要

**改动量**:< 50 行 / 50–200 行 / > 200 行
**文件**:foo/bar.rs, foo/baz.rs

[按改动量贴 code block / stat / 文件清单]

## 自检

- [ ] 编译 / 测试绿
- [ ] 没有越界改 scope 之外的文件
- [ ] 没有调试代码 / TODO 残留

## 等指示(默认 [1] review)

- [1] **review**  — 摆 diff 逐文件过,我等你 commit / push 明确指令
- [2] **direct**  — 跳 review,直接 commit + push 本波改动(信任快路径)
- [3] **hold**    — 暂停,继续改不 commit
- [4] **split**   — 本波多 deliverable,先拆多个 commit 再选

回复数字 / 关键字 / 具体修改意见。

**AI 不许自己默认 [2]**——只有用户明确选了才走 direct。
```

---

## 与其他 skill 的关系

- **`scope`**:动手前对齐 scope。本 skill 是动手**后**、commit 前的闸门。前者防越界,后者防自作主张。
- **`design-review`**:5 维度设计审查。改动量大或破面时,review 阶段可触发它做更深判断。
- **`agent-skills:git-workflow-and-versioning`**:git 操作的通用流程参考。本 skill 强调"等用户点头才动"这一刚性边界,是它的补充。

---

## Auto-invoke chain

push 完成后,LLM **自动 invoke**: `handoff`(迭代终点交接)。

如果用户明确说"继续下一条 / 不交接 / 接着做",则跳过 handoff,继续 Phase 1(看板选下条)。

大改动(>200 行 / 改公开 API / 改持久化)进入 commit-review 时,**先内嵌调用** `design-review` 做 5 维度审查,再摆 diff。
