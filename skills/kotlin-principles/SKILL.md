---
name: kotlin-principles
description: Kotlin 项目专属设计原则(横切,语言特化)。仅在 Kotlin 项目(*.kt / build.gradle.kts / settings.gradle.kts / kotlinx / Ktor / Spring Boot Kotlin / Android Kotlin 上下文)触发,**非 Kotlin 项目零干扰**。基于 `principles` 的 ADT + Type Class 模式落地到 Kotlin(数据=sealed/data/value;行为=interface;解释器=class 实现 + 顶层扩展)。覆盖 15 维度判据:类型设计 / null safety / scope functions / 协程语言层 / 错误处理 / 不可变 / 扩展函数 / 集合 / 委托 / 命名 / 弃用替换 / public API 显式 / 复杂度阈值 / inline-reified / equals-copy 陷阱;末尾 Review checklist。强立场判定 + 优先级阶梯。新 Kotlin 项目可作为 project CLAUDE.md 起点。
---

# Kotlin Principles

Kotlin 项目专属设计原则与 idiomatic 选择。**语言无关的代码品味**走 `principles` / `design-review`,本 skill 只讲 Kotlin 特化的部分。

---

## 触发场景

- 在 Kotlin 项目里(检测到 `*.kt` / `build.gradle.kts` / `kotlinx` 依赖 / Ktor / Spring Boot Kotlin)
- 讨论 Kotlin 设计选择("用 sealed interface 还是 enum?" / "scope function 该选哪个?")
- 新开 Kotlin 项目,需要 project CLAUDE.md 的设计原则起点
- Review Kotlin 代码,关注 idiomatic / 风格层

**不触发**:

- 非 Kotlin 项目(零干扰)
- 框架特定问题(Ktor 路由 / Spring 注解 / Android lifecycle / Compose)→ 走对应框架的资源
- Kotlin 编译错误 / 类型推导问题 → IDE 即时反馈足够,本 skill 不重复
- 语言无关的代码品味问题 → 走 aye 的 `principles` / `design-review`

---

## 顶层设计哲学:ADT + Type Class 模式落地到 Kotlin

> 跨语言通用判据见 `principles` 的"ADT + Type Class 模式"。本节讲 Kotlin 怎么落地。

| 层 | Kotlin 工具 |
|---|---|
| **数据 (ADT)** | `sealed interface` / `sealed class` 表 sum type;`data class` 表 product type;`value class @JvmInline` 当 newtype |
| **行为接口** | `interface` / `fun interface`(单方法 SAM) |
| **解释器实现** | 实现类(`class : Interpreter`)+ 顶层扩展函数(`fun X.evaluate()`)+ 派生谓词(`val X.isFoo: Boolean`) |

### 最小例

```kotlin
// 数据
sealed interface Expr {
    data class Num(val v: Int) : Expr
    data class Add(val l: Expr, val r: Expr) : Expr
}

// 行为接口
interface Eval<R> { fun eval(e: Expr): R }

// 解释器实现 + 顶层扩展入口
class IntEval : Eval<Int> {
    override fun eval(e: Expr): Int = when (e) {
        is Expr.Num -> e.v
        is Expr.Add -> eval(e.l) + eval(e.r)
    }
}
fun Expr.evaluate(): Int = IntEval().eval(this)
```

### 为什么这样落地(Kotlin 特殊取舍)

- **`sealed interface` 优于 `sealed class`**——子类不被单继承占死,且 `value class` 可以实现 sealed interface(反之不行)
- **`value class` 当 newtype 用**——只支持单字段,等同于 Rust 的 `struct X(u32)`(详见维度 1)
- **解释器层允许实现类 + 顶层扩展函数双形态**——Kotlin 没有 Haskell `instance` 那样的独立概念,顶层扩展是天然替代
- **行为不写在 `data class` 内**——避免数据 + 行为耦合,违背三分法的"双向膨胀"反例

完整范例见 [`examples/adt-interpreter.kt`](examples/adt-interpreter.kt)(T6 后填充)。

---

## 15 维度判据(索引)

每条维度的判据骨架。**详细正反例 + 决策表 + 案例**见 [`reference.md`](reference.md)——SKILL.md 不重载 token,需要时 Read reference 对应段。

| # | 维度 | 核心判据 |
|---|---|---|
| 1 | 类型设计 | sealed interface > sealed class;data class 纯字段;value class 当 newtype;data object 单例 |
| 2 | Null safety | smart cast > ?.let > requireNotNull > as?;**禁 !!** |
| 3 | Scope functions | apply 配置;let 非空转换;also 副作用;不嵌套 |
| 4 | 协程基础 | coroutineScope 结构化并发;Dispatchers.IO/Default/Main 边界;**禁 GlobalScope / 协程内吞 cancel** |
| 5 | 错误处理 | sealed result(领域)> Result(跨边界)> exception(真错误);**禁 nullable 表"操作失败"** |
| 6 | 不可变 | val > var;List > MutableList;copy() 是浅拷贝 |
| 7 | 扩展函数 | 成员 > 顶层 > 扩展(慎);**领域行为不写顶层扩展** |
| 8 | 集合 API | 默认 List;多步 + 大数据 / 短路 → asSequence |
| 9 | 委托 | by lazy 懒;Delegates.observable 校验;by impl 接口委托 |
| 10 | 命名惯例 | toX 转换 / asX 视图 / xOrNull / xOrElse;is/has/can boolean;**禁 get_ 前缀** |
| 11 | 弃用替换 | 禁:!! / GlobalScope / lateinit / Array<T> / companion-object-放-const / @JvmStatic 滥用 |
| 12 | Public API explicit | 库 public 必须显式标返回类型 + 暴露接口而非实现 |
| 13 | 复杂度阈值 | 函数 < 50 行 / 嵌套 ≤ 3 / 参数 ≤ 5 |
| 14 | inline / reified | 仅当 lambda + 体短 / 需 reified / 需 non-local return |
| 15 | equals/copy 陷阱 | data class 不含 Array / mutable 引用;copy 是浅拷贝 |

Review 时按"触发关键词"定位维度,再 Read reference 看具体判据。

---

## Review Checklist

> 末尾 checklist,可被 `design-review` / `commit-review` 引用。

Kotlin 代码写完 / review 前快速扫:

```
[ ] 没有 !!(用 ?.let / ?: / requireNotNull)            ── 维度 2 / 11
[ ] 没有 GlobalScope.launch / runBlocking 在 suspend 内 ── 维度 4
[ ] 协程内 runCatching / catch 必须透传 CancellationException ── 维度 4
[ ] 没有 lateinit var(用构造器注入 / by lazy)            ── 维度 11
[ ] data class 没有 var / Array / mutable 引用字段       ── 维度 6 / 15
[ ] 没有暴露 MutableList / MutableMap(暴露 List / Map)   ── 维度 6
[ ] 没有嵌套 scope 函数;apply 不当 let 用                ── 维度 3
[ ] 没有用 nullable 表"操作失败"(用 sealed result)        ── 维度 5
[ ] 库 public 函数显式标返回类型,不暴露 ConcurrentHashMap 等实现类 ── 维度 12
[ ] 没有 getX() / checkX()(用 val x / isX)               ── 维度 10
[ ] 函数 < 50 行 / 嵌套 ≤ 3 / 参数 ≤ 5                    ── 维度 13
```

**和 `design-review` 5 维度的关系**:本 checklist 是 Kotlin 实现层信号(具体到 keyword 级);`design-review` 是设计判据(抽象到决策级)。两者都跑,不重复。

---

## 与其他 skill 的关系

- **`principles`**(aye 语言无关版):本 skill 是它的 Kotlin 特化扩展。"ADT + Type Class 模式"在 `principles`,Kotlin 落地映射在本 skill。
- **`rust-principles`**(姊妹):同为语言特化 sibling。`rust-principles` 的"运行时三分法"(Service / Context / Value)是运行时角度,与本 skill 的设计角度正交,不重叠。
- **`design-review`**:5 维度判据是语言无关的;本 skill 的 Review checklist 是 Kotlin 实现层信号。两者并行不冲突。
- **`flow` / `feature` / `scope` / `acceptance` / `commit-review`**(aye 仪式层):正交关系,本 skill 是知识库,它们是流程。

---

## 用法:作为新 Kotlin 项目 CLAUDE.md 起点

新开 Kotlin 项目时:

1. 创建 project `CLAUDE.md`
2. 引用本 skill:"设计原则见 `aye/kotlin-principles` skill(LLM 自动 invoke)"
3. 在 CLAUDE.md 里只写**项目特定**的部分(架构、模块列表、build command),**不重复本 skill 的通用原则**

这样:

- 通用 Kotlin 设计原则一处维护,所有项目自动跟进(改 skill 即可)
- project CLAUDE.md 只放项目特殊内容,体积小,信号强
- 新 Kotlin 项目零拷贝 bootstrap
