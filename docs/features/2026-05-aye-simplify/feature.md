# 2026-05-aye-simplify：优化 aye skill 集，逼向"瑞士军刀"

## Feature

本 session dogfood 暴露 aye 多处摩擦信号——闸门成本固定让用户主动合并 task 规避、commit 完不回 feature.md 打勾、plugin cache 跟不上版本、11 skill 之间疑似有重叠和同质。但用户对 aye 的标尺是"简单强大瑞士军刀"——一个工具，一眼会用，刀刃锋利。当前 aye 是否已经过度复杂，缺乏系统盘整，只有零散痛感。需要先 review 找问题，再按结论修剪。

### Users

- 用户本人（aye 主用户，频繁触碰 skill）
- 未来引入 aye 的其他工程师（skill 越多 → 学习成本越高，越像 Scrum 累赘的反面）
- 配对的 AI（skill 越多 → 每次 reminder 越长 → token 噪音越大，触发判断越分支）

### Scope

**In**

- T1 出 `review.md`：全 11 skill 评估（行数 / 职责单一性 / 触发清晰度 / 与其他 skill 重叠）
- T1 横向看 chain：闸门跳转是否有冗余 / 哪些 skill 是同质能合
- T1 汇总本 session 真实 dogfood 痛点（交互成本 / 打勾漏 / cache / 颗粒度等）
- T1 输出"砍 / 合 / 留"清单 + 判据 + "精简后 aye 形态草图"
- 用户对齐 review 结论后，**按结论实施修剪**（改 SKILL.md / 删 skill / 合并 skill / 调 chain）
- 实施完同步更新 `flow.md` / `README.md` / `plugin.json`（版本号 bump）

**Out**（钉死防越界）

- ❌ 改 plugin marketplace / cache 行为（平台层，不在 aye 仓库职责）
- ❌ 加新 skill（本 feature 只做精简）
- ❌ 调整 `kotlin-principles` / `rust-principles` 内文深度（看的是 skill 颗粒，不是各 skill 内容）
- ❌ `examples/` 子目录（skill 内文，不是 skill 本身）
- ❌ `plugin.json` / `marketplace.json` / `README.md` 内容评估（meta 层不进 review；只在实施阶段同步引用）
- ❌ 跟 `agent-skills` / claude code marketplace 其他 plugin 横切对照

### Constraints

- review 标尺："瑞士军刀"是主观判断，**不量化**——但每条砍/合/留必须带判据（同质 / 重叠 / 低使用频率 / 跟其他 skill 重复职责）
- 实施按 review 结论，**不在实施阶段改方向**——若中途想改，回来更新 review 再继续

### Open questions（已答，留 trail）

- ✅ Q1: 不量化，主观判断"是否太复杂"
- ✅ Q2: 输出 = `review.md`（用户确认后动手实施）
- ✅ Q3: 只看 aye 项目内的 `skills/` + `flow` 设计，不含 meta 配置
- ✅ Q4（推断自 Q2）：信任 review 报告，确认即动手（动手是本 feature 的 T2+，不是 sub-feature）
- ✅ Q5: 不跟 `agent-skills` 横切对照

## Acceptance

- [ ] AC-1: `docs/features/2026-05-aye-simplify/review.md` 存在，含全 11 skill 评估 + 砍合留清单 + 简化形态草图
- [ ] AC-2: review 结论里标"砍 / 合"的项，**全部完成实施**（SKILL.md 改完 / skill 删干净 / 引用同步）
- [ ] AC-3: 实施后，`flow.md` / `README.md` / `plugin.json` 引用与新形态一致
- [ ] AC-4: 实施后 bump 版本号（minor 或 major 视改动幅度），push origin
- [ ] AC-5: dogfood 痛点至少 3 个被 review 回应或修复

## Tasks

- [x] T1: 写 `review.md`（单 skill 评估 + 横向重叠 + dogfood 回应 + 砍合留清单 + 简化形态草图，一次过）（v1 草率，v2 深度阅读后修订）
- [ ] T2: `commit-review` 收尾增强,handoff 内嵌
- [ ] T3: `acceptance` 加跳过逃生口
- [ ] T4: `feature` 瘦身 + 反模式 8
- [ ] T5: `kotlin-principles` 拆 reference.md
- [ ] T6: `flow` 哲学论证挪 `principles`
- [ ] T7: 收尾(README bump 备忘 / "5 维度"重命名 / feature 打勾 / bump 0.3.0 / push)

## Notes

### 设计决策（对话产出）

- **本 feature 边界纠偏**：最初草稿把"动手砍/合/重构"放进 Out，是把 feature 缩成纯 review。用户纠正：本 feature 应是"优化调整 aye skill 集"——review 只是 T1，T2+ 是实施。已修订。
- **T2+ 故意半开放**：具体几个 task / 改哪些 skill，要 review 完成 + 用户对齐结论才能填。这跟"feature 阶段穷尽 task list"有张力——但本 feature 就是"先看再改"性质，提前虚构 task 反而违反颗粒度铁律 3（零新决策）。
- **不量化"瑞士军刀"**：用户明确"看感觉"。但要求"砍/合/留"带判据——避免变成纯凭感觉拍。

### 关键时间线

- 2026-05-07: feature 启动，Q1-Q5 回答完毕，Q4 自 Q2 推断
