---
name: pick
description: 决策点提问交互化(用户主动触发)。触发关键词:"pick / 拍板 / 选一个 / 哪条 / 二选一 / 你定 / 给我选 / 让我选 / 出选项"。横切 skill,**用户主动喊**:强制 AI 下一次抛决策点时调用 `AskUserQuestion` 工具呈现 2-4 个候选,不要文字罗列让用户敲键盘回答。候选构造遵循 `principles` 的「AI 不当提问机」(候选 + trade-off + 推荐 + 理由)。
---

# Pick

让 AI **用工具问决策点**,不要把"1. xxx 2. xxx 点头哪条"贴成文字让用户手打"1"。

`AskUserQuestion` 工具能弹原生选项 UI,用户键盘 / 鼠标点一下完事。约束已经写在 `principles` § AI 不当提问机和 `feature` / `scope` / `acceptance` / `design` 里——**但横切约束不自动加载**,实战经常漏触发。

`pick` 是兜底:**用户主动喊一句,AI 接下来的决策提问必须走工具**。

---

## 触发

用户明示要把决策点工具化呈现时:

- "pick / 拍板 / 让我拍 / 你定"
- "选一个 / 选一条 / 哪条 / 哪个"
- "二选一 / 三选一"
- "给我选 / 让我选 / 出选项"
- "用工具问 / 弹选项 / 别让我打字"

`disable-model-invocation` 类型——**用户主动喊,LLM 不自动**。喊一次,作用于**接下来这一次提问**;后续提问如还需工具化,再喊一次(或在 CLAUDE.md 写硬规则)。

---

## 强制动作链

### 1. 锁定决策点

识别当前要让用户拍的**是不是真决策点**:

- ✅ **2-4 个互斥候选,选一个推进** — 工具化(典型:选实现方案 A/B、起 T7 还是 T3、改 X 还是改 Y)
- ❌ **开放问题** — 不工具化(如"接下来想干啥?")
- ❌ **信息确认** — 简短文字即可(如"我理解对了吗?")
- ❌ **>4 候选 或 候选不互斥** — 先拆分 / 收敛,再工具化

边界含糊就**优先工具化**——多一次点击 < 多一次打字。

### 2. 构造候选(引用 principles)

每个候选必须四件套,不抄,见 `principles` § AI 不当提问机:

- **候选** — 一句话方案
- **trade-off** — 短板 / 代价
- **推荐项** — 第一个,label 末尾加 ` (Recommended)`
- **理由** — 一行说明为什么推荐

工具字段映射:

| 工具字段 | 内容 |
|---|---|
| `header` | 决策点标题(< 20 字) |
| `question` | 一句话问题 |
| `multiSelect` | `false`(互斥决策)/ `true`(可多选,如清单勾选) |
| `options[].label` | 候选短名,推荐项末尾 ` (Recommended)` |
| `options[].description` | trade-off + 一行理由 |

### 3. 调 `AskUserQuestion` 工具

工具 schema 不在默认 prompt 里,先 `ToolSearch` 拉:

```
ToolSearch(query: "select:AskUserQuestion", max_results: 1)
```

拉到 schema 后调一次,等用户选完再继续。

### 4. 收到答复后写回

- 决策落点写回对应文档(feature.md / scope / acceptance.md / design.md / 当前 PR notes)
- 不要"用户选完就忘"——决策要留痕,见 `principles` § 决策记录

---

## 反模式

| ❌ | 为什么不行 |
|---|---|
| 文字罗列 "1. xxx 2. xxx 点头哪条" | 用户截图就是这个症状——pick 存在的理由 |
| 候选无 trade-off | 用户拍不动,等于裸问 |
| 候选 > 4 个 | 工具上限,且选择疲劳——先收敛 |
| 候选不互斥(如 "做 A / 同时做 A 和 B / 只做 B") | 应该拆成 2 个独立问题 |
| 用户已经表态偏好后还工具化追问 | 啰嗦,直接执行 |
| 单次 pick 后默认"所有后续提问都用工具" | pick 作用于**这一次**,别越界 |

---

## 与其他 skill 的关系

- `principles` § AI 不当提问机 — 候选构造的**判据来源**,pick 不重写,只引用 + 强制工具化
- `feature` / `scope` / `acceptance` / `design` / `commit-gate` — 这些 skill 各自的"等用户拍板"环节已经写了"环境支持时优先用 AskUserQuestion"——pick 是**兜底触发器**,在它们没自动加载 / 没触发时由用户喊一句强制走
- `handoff` / `pua` — 同属用户主动触发型,但语义不同:`handoff` 收 session、`pua` 切专家视角、`pick` 强制工具化提问

---

## Auto-invoke chain

`pick` 是一次性触发,**无固定下游**:

1. 用户喊触发词
2. AI 下一次抛决策点 → 走 `AskUserQuestion` 工具
3. 用户选完 → 回到原 skill 流(scope / acceptance / design / commit-gate / 写代码 …)继续
4. pick 不接管后续提问节奏——如需持续工具化,用户再喊一次

**不写文件、不持久化**——`pick` 只改变提问形式,不产 artifact。
