# kotlin-principles：沉淀团队 Kotlin idiom 判据 + 升级跨语言"类型三分法"原则

## Feature

团队后端项目用 Kotlin + Ktor 开发，但 AI 协作 + 人类 review 时缺少统一的 Kotlin idiom 判据——
该 sealed interface 写成 enum，该 `?.let` 写成 if 判空，scope functions 五个混用没章法，协程作用域归
属不清，Result vs exception 凭老人感觉。代码风格不收敛，新人学不到地道写法，review 也只能"我觉得这
样更好"凭手感拍。

aye 已有 `principles`（语言无关）+ `rust-principles`（Rust 特化），Kotlin 缺一块对应件。本次 feature
不只是补 `kotlin-principles`，还顺带把"**类型三分法**：数据 / 行为 / 解释器"作为**跨语言通用判据**
升进 `principles` 横切层（Haskell ADT + type class、OCaml variant + functor、Rust enum + trait、
Kotlin sealed + interface 本质都是同一件事），让三个 skill 在判据层一致。

### Users

- 写 Kotlin 的工程师（写代码 / review 时引判据）
- 给 Kotlin 项目配对的 AI（写代码时遵守，review 时援引）
- design-review / commit-review 仪式（被引用作为判据来源）
- 写 Rust 的工程师（间接受益于 `rust-principles` 与 `principles` 措辞对齐）

### Scope

**In**

- 升级 `skills/principles/SKILL.md`：增加"**类型三分法**：数据 / 行为 / 解释器"作为跨语言通用设计原则
- 对齐 `skills/rust-principles/SKILL.md`：把已有的 Service/Context/Value 措辞与新增的"数据 / 行为 / 解释器"对齐
- 新增 `skills/kotlin-principles/SKILL.md`：纯 Kotlin **语言层**判据
  - **顶层设计哲学**：类型三分法落地到 Kotlin
    - 数据 = `sealed interface` + `data class` + `value class`(newtype)
    - 行为 = `interface`（type class 风格，多实现）
    - 解释器 = 接口实现 + 顶层扩展函数 + 派生谓词（涵盖式定义，不再分第四层）
  - **16 个判据维度**（每条三段式：判据 + 正例 + 反例）：
    1. 类型设计（sealed / data / value / object / enum 选用）
    2. null safety（优先级阶梯：smart cast > `?.let`/`?:` > `requireNotNull` > `as?`，禁 `!!`）
    3. scope functions（let/run/with/apply/also 五选一阶梯）
    4. 协程基础（语言层）：`suspend` / 结构化并发 / `Dispatchers.Default/IO/Main` 边界，**不含** `Flow`/`Channel`
    5. 错误处理（exception vs `Result<T>` vs sealed result class）
    6. 不可变（`val`/`var` / `List` vs `MutableList` / `copy()` 陷阱）
    7. 扩展函数（vs 成员函数 vs 顶层函数 优先级阶梯）
    8. 集合 API（`Sequence` vs `List` 切换、操作链）
    9. 委托（`by lazy` / `Delegates.X` / 自定义）
    10. 命名惯例（`toX` / `asX` / `xOrNull` / `xOrElse`）
    11. 弃用替换（`!!` 禁 / `companion object const` vs top-level const / `@JvmStatic` 边界）
    12. **public API explicit return type**（库 / 模块边界）
    13. **复杂度阈值**（函数行数 / 嵌套深度 / 参数个数）
    14. **inline / reified / crossinline**（高阶函数关键决策）
    15. **equals / hashCode / data class copy 陷阱**
    16. Review checklist（被 design-review / commit-review 引用）
- 全维度采用**优先级阶梯式**呈现（首选 X / 次选 Y / 例外用 Z / 禁用 W），不写"二元 vs"
- 风格立场：**强立场判定**（首选 / 反模式 / 禁用），灰区给阶梯 + 逃生口
- 触发关键词：Kotlin 项目上下文 + 设计判据 / idiom / review；非 Kotlin 项目零干扰
- 新增 `skills/kotlin-principles/examples/adt-interpreter.kt`（≤ 100 行，基于 demeter/safeguard 简化），
  完整体现类型三分法 + ≥ 6 个判据维度，作为 SKILL.md 末尾的"标准范例"
- 更新 `flow.md` 横切层，补 `kotlin-principles` 引用，与 `rust-principles` 对称
- 更新 `README.md` 同步引用

**Out**（钉死防越界）

- ❌ **框架使用指南**：Ktor / Spring / Exposed / koin / kotlinx.serialization 不进
- ❌ **Android 特定**：lifecycle / Compose / ViewModel / `viewModelScope` 不进
- ❌ **KMP 多平台**：expect/actual / 平台差异不进
- ❌ **工具链配置**：Gradle / detekt / ktlint 规则配置不进（detekt rule ID 可作为反模式佐证引用，但不写配置）
- ❌ **Java 互操作细节**：`@JvmStatic` / `@JvmName` 边界除外，其余不进
- ❌ **测试 / 性能 / 调试**：全不进
- ❌ **教程性质**：不解释 "什么是 sealed class"，只讲"何时用 / 何时别用"
- ❌ **Flow / Channel 深度**：协程板块只到 `Dispatchers.X` 边界，深度内容（Flow cold/hot / SharedFlow / StateFlow / Channel 选用）留给未来"按需召唤"型 skill，本 skill 不背
- ❌ **多字段 value class**：今天没 stable，不写"未来可能"，value class 只当 newtype 用
- ❌ **本次不做**：用 safeguard 真实代码做反向 review（放 future backlog）

### Constraints

- skill 形态对齐 `rust-principles`（SKILL.md 主文件，可加 examples 子目录）
- 触发要精准：Rust 项目 / 通用项目零干扰（参 `rust-principles` 触发条件写法）
- 中文为主，代码示例用真实可编译的 Kotlin 2.x
- 新增"类型三分法"在 `principles` 中的描述与 `rust-principles` / `kotlin-principles` 各自的语言映射**三方一致**
- Effective Kotlin / detekt rule ID 等非官方一手出处的引用，下笔前必须二次核对（前期调研基于训练记忆）

### Open questions

（feature 阶段已穷尽，进 scope 阶段如有新疑问再补）

## Acceptance

- [x] AC-1: aye plugin 列表里出现 `aye:kotlin-principles`，Kotlin 项目上下文能触发，Rust / 通用项目不触发
- [x] AC-2: `skills/kotlin-principles/SKILL.md` 顶层有"类型三分法落地到 Kotlin"章节，16 个维度判据每条三段式（判据 + 正例 + 反例），灰区维度采用优先级阶梯式呈现 (done @ c482764；实际 15 维度 + review checklist 单独成章，总信号一致)
- [x] AC-3: 末尾 Review checklist 可被 `design-review` / `commit-review` 引用 (done @ 0f86918)
- [x] AC-4: 产出 `skills/kotlin-principles/examples/adt-interpreter.kt`（≤ 100 行，基于 safeguard 简化），完整体现类型三分法 + ≥ 6 个判据维度，SKILL.md 中作为"标准范例"引用 (done @ b0d8620；66 行，覆盖 7 维度)
- [x] AC-5: `skills/principles/SKILL.md` 在"4 条核心哲学"和"元规则"之间新增"### 设计原则"区块，加"**ADT + Type Class 模式**"为跨语言通用设计原则（顶层用行业通用名；内文展开为：数据 = ADT；行为 = Type Class，再拆接口签名 + 解释器实现两步） (done @ 24ecdc0)
- [x] AC-6: `skills/rust-principles/SKILL.md` 把现有"类型三分法（Service/Context/Value）"改名为"**运行时三分法**"以避开 `principles` 新加条的"ADT + Type Class 模式"撞名，并在开头交叉引用 `principles` 的设计层条目（两者维度正交：设计 vs 运行时） (done @ 24ecdc0)
- [x] AC-7: `flow.md` 横切层补 `kotlin-principles` 引用，与 `rust-principles` 描述对称
- [x] AC-8: README.md 同步

## Tasks

- [x] T0a: 升级 `skills/principles/SKILL.md`，加"类型三分法：数据 / 行为 / 解释器"为跨语言通用原则 (done @ 24ecdc0)
- [x] T0b: 对齐 `skills/rust-principles/SKILL.md` 措辞，与 `principles` 新增条三方一致 (done @ 24ecdc0)
- [x] T1: 调研 `rust-principles` 骨架，定 `kotlin-principles/SKILL.md` 模板结构（顶层三分法 + 16 维度框架） (done @ 92985e5)
- [x] T2-T5: 15 维度判据填充 (done @ c482764；实际推进时 T2-T5 同模板同质合一刀，这正是 `feature` 反模式 7 的真实例证，已沉淀为铁律)
- [x] T6: 写 `examples/adt-interpreter.kt`（基于 safeguard 简化的 ADT 解释器范例），SKILL.md 末尾引用 (done @ b0d8620)
- [x] T7: 写 Review checklist（kotlin-principles 末尾），可被 design-review / commit-review 引用 (done @ 0f86918)
- [x] T8: 更新 `flow.md` + `README.md` 引用
- [~] T9: 自检：用本 skill 把 `examples/adt-interpreter.kt` 跑一遍判据，确认每条都对得上 (跳过；过程性活，按 `feature` 颗粒度铁律应并入 commit-review，不单立 task)

## Notes

### 设计决策（对话产出）

- **类型三分法是跨语言通用判据，不是 Kotlin 专有**：Haskell / OCaml / Rust / Kotlin 同源。所以本次 feature
  的核心改动**升 `principles`**，三个 skill 的判据层一致，避免 kotlin-principles 引用不存在的 principles 条
  导致中间态。
- **顶层命名定为"ADT + Type Class 模式"**（scope 阶段决策更新；最初提的"数据 / 行为 / 解释器"作为内文心智模型保留）：
  - 顶层用行业通用名优于自创名：`ADT + Type Class` 在 Haskell / Scala / Rust trait / Kotlin interface 圈共通认知，程序员搜得到、读论文对得上
  - "解释器"作为独立顶层有些勉强——本质是 Type Class 的 instance + 顶层扩展，不是和"数据/行为"并列的第三层
  - 内文仍保留三段心智："数据 = ADT" + "行为 = Type Class（接口签名 + 解释器实现两步）"
  - 备选方案被 push back：自创"数据 / 行为 / 解释器"搜不到，学术名"Object Algebras"过重
  - **rust-principles** 现有"类型三分法（Service/Context/Value）"改名"**运行时三分法**"以避撞——两者维度正交（设计 vs 运行时）
- **value class 当 Rust newtype 用**：今天 stable 仅单字段。设计哲学(行为不绑数据)让"data class → value class"未来变成机械替换——
  这是为什么三分法值得立的论据，但 skill 现在不赌"多字段 value class"路线图。
- **强立场判据有用——老练 Kotlin 程序员也会踩 `runCatching` 吞 `CancellationException`**：
  demeter/safeguard 代码里有两处 `runCatching {} .onFailure {}` 在协程内会吞取消信号
  （`SafeguardAgent.kt:35`、`SafeGuarder.kt:31`、`SafeguardEngine.kt:74` 的 `catch (Exception)` 同理）。
  这条进 skill 协程板块顶上，**强立场禁用**。
- **优先级阶梯式呈现 > 二元判据**：Kotlin 语言能力冗余（多种方式做同件事），单点判据写不出价值。
  全 16 维度采用"首选 X / 次选 Y / 例外 Z / 禁用 W"的阶梯。
- **互操作历史包袱要钉死**：Kotlin 为兼容 Java 留了 `var` / `lateinit` / `!!` / `Array<T>` / 平台类型 等
  "标准库里有但不该常规用"的特性。这批是强立场判定的天然弹药。

### dogfood 决策

- **dogfood 形态**：从"review 真实代码"切换为"**产出正例展示**"——基于 safeguard 核心 ADT
  （`Indicator.State` / `Context` / `Event`）简化重写为标准 ADT 解释器范例，≤ 100 行，
  作为 skill 末尾的标杆样本。
- **safeguard 是好素材**：水平超过 skill 当前覆盖范围（sealed + atomicfu + Flow + `with` 模拟 context receiver +
  `& Any` definitely-non-null），但本身复杂度不适合直接作为教学样本，需要**简化抽象**。
- **Arrow.kt 退到二级参考**：library 风格 + 重度 FP，作为"高水准对照镜"用，**不当样板**——skill 立场服务普通后端业务代码。

### 调研留下的待核对项

- Effective Kotlin 具体 item 编号（Item 1 / 3 / 4 / 7 等）：调研基于训练记忆，**下笔前需二次核对原书**
- detekt 具体 rule ID（`UnsafeCallOnNullableType` / `GlobalCoroutineUsage` 等）：基于训练记忆，**下笔前到 detekt.dev 核对**
- claude-code marketplace 撞车：**不需要扫**——plugin 各带命名空间（`aye:kotlin-principles`），不会冲突

### 关键决策时间线

- 2026-05-06: feature 启动，对话敲定上述决策
