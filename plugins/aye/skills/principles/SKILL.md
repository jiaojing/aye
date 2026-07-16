---
name: principles
description: '工程哲学底座(横切)。触发关键词:"决策框架 / 该用哪种 / 取舍 / 简洁性 / 哲学判据 / 会破坏什么"。提供好品味、consumer 兼容、实用主义、简洁，以及 ADT + Type Class、consumer 驱动抽象、领域边界、框架隔离、生命周期所有权和多方案决策判据。'
---

# Principles

工程哲学底座。具体语言规则走 `rust-principles` / `kotlin-principles`，具体设计审视走 `review`。

## 4 条核心哲学

### Good Taste：消除特殊情况

- 换一个建模角度，让边界情况退化成正常情况。
- 优先消除分支，而不是继续堆条件判断。

### Never Break Consumer

- 任何导致现有调用方无预警失效的改动都是 bug。
- 兼容不只指编译：行为、配置、错误分类、持久化格式和最低工具链也是契约。
- 优先加法演进；弃用必须给 replacement 和 migration。

### 实用主义

- 解决已经存在或可验证的问题，不为假想未来预制复杂度。
- 理论上完美但现实中更难测试、阅读和回滚的方案不是更好方案。

### 简洁执念

- 函数只做一件事；happy path 保持线性。
- 嵌套超过 3 层是重构信号，不是必须突破的挑战。
- 清楚胜过聪明；无收益的抽象直接删除。

## 设计原则

### ADT + Type Class

数据建模时，把类型分成三个角色：

| 层 | 角色 | Rust | Kotlin |
|---|---|---|---|
| 数据 ADT | 承载状态 | `enum` / `struct` | `sealed interface` / `data class` / `value class` |
| 行为签名 | 定义能力 | `trait` | `interface` / `fun interface` |
| 解释器 | 实现能力 | `impl Trait for Type` | 实现类 / `object` |

数据与行为在**建模职责**上分离；行为实现仍归属于 trait/interface 或解释器类型。方法、扩展函数和多个 `impl` 块可以作为组织与调用入口，不等于把状态和行为重新耦合。

为什么这样建模：

- 加数据 case：穷举检查指出所有需要适配的解释器。
- 加新行为：增加解释器，数据结构不动。
- 原始类型升级为 newtype/value class 时，行为边界帮助调用方平滑迁移。

警惕：

- 在数据类型里堆所有领域操作，导致数据 case 与行为双向膨胀。
- Service 持有领域集合等可变业务状态。
- 行为漂成无命名归属的自由函数。

### 抽象从 Consumer 中发现

- 先写具体实现；接口由 consumer 的真实需求定义，不由 implementor 对未来的想象预制。
- 抽象必须对应真实替换、测试边界或领域能力；实现数量只是信号，不是硬门槛。
- 接口保持最小，只暴露 consumer 实际使用的能力。

### Domains over Layers

- 模块按领域职责命名：`billing` / `auth` / `jobs`，警惕 `service` / `repository` / `utils` / `common`。
- composition root 负责接线；领域模块不横向穿透彼此内部。
- 目录深浅由语言和项目决定，“领域所有权清楚”比机械扁平更重要。

### Frameworks at the Edge

- CLI、HTTP、GUI、ORM、配置框架都是 adapter。
- 框架对象停在边界，先转换成领域类型和强类型配置，再进入核心。
- adapter 只做解析、校验、路由、错误映射和输出；业务决策留在核心。

### Lifecycle Ownership

- 每个异步任务、线程、连接和后台服务都必须有明确 owner。
- 启动前回答：谁取消、何时超时、谁等待结束、失败传到哪里。
- 不能说明如何停止的任务，不应启动。

## 元规则

开始工作前问三件事：

1. 这是真问题还是臆想出来的？
2. 有更简单的方法吗？
3. 会破坏哪些 consumer 或边界？

### 多方案决策框架

按以下顺序评估真实可行的候选：

| 优先级 | 判据 | 问题 |
|---|---|---|
| 1 | 可测试性 | 能否可靠、低成本地验证？ |
| 2 | 可读性 | 六个月后还能快速理解吗？ |
| 3 | 一致性 | 符合项目现有模式吗？ |
| 4 | 简洁性 | 是满足需求的最小方案吗？ |
| 5 | 可逆性 | 判断错了，撤销有多贵？ |

不要为了凑“方案空间”制造明显劣质候选。只有一个合理方案时，说明排除条件即可。

## 工作纪律

- 改公开 API 前先查定义、所有调用点和外部 consumer。
- 使用框架 API 前先从 manifest/lockfile 判断版本，禁止混用不同 major/version 的示例。
- 优先结构化输出和机械检查，不靠肉眼解析彩色日志或判断兼容性。
- 遇到 placeholder/WIP，先确认成熟度，不对空骨架做深度设计 review。
- 调试时先验证实际编译输入、调用路径和第二调用点，再怀疑工具链。

## AI 协作约束

- AI 没有可靠的跨 session 记忆：任务和决策必须自描述、可落盘。
- AI 写得快、人 review 慢：限制 diff 和 PR mental scope。
- AI 容易越界：先锁 scope，再实施；扩 scope 必须重新对齐。
- AI 会误解模糊需求：feature、acceptance 和 design 分别校正 what、done、how。
- 一次只推进一个可 review 的交付，避免批量生产超过人的 review 带宽。

具体工作流由 `flow` 编排，本 skill 不重复 Scrum 或 chain 说明。

## 决策点提问质量

需要用户拍板时，必须提供：

- 2–4 个真实、互斥候选；
- 每项的 trade-off；
- 推荐项；
- 推荐理由。

没有真实决策就不要提问；信息确认不包装成选择题。工具呈现、host 差异和文本 fallback 统一走 `pick`。

## 关系

- `review`：把本原则落到类型、Builder、Trait、组织和重构判断。
- `flow`：编排 gate、triggered 和 reference skills。
- `rust-principles` / `kotlin-principles`：语言特化落地。
