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

## 15 维度判据(强立场 + 优先级阶梯)

每条三段式:**判据 / 正例 / 反例**。灰区给优先级阶梯("首选 X / 次选 Y / 例外 Z / 禁用 W"),黑白区直接判定。

### 1. 类型设计(sealed / data / value / object / enum 选用)

(展开见后续)

### 2. Null safety(优先级阶梯)

(展开见后续)

### 3. Scope functions(let / run / with / apply / also 五选一)

(展开见后续)

### 4. 协程基础(语言层)

> 范围:`suspend` 语义 / 结构化并发 / `Dispatchers.Default/IO/Main` 边界。**`Flow` / `Channel` 不在本 skill**(深度内容留按需召唤型 skill)。

(展开见后续)

### 5. 错误处理(exception vs `Result<T>` vs sealed result)

(展开见后续)

### 6. 不可变(`val` / `var` / `List` vs `MutableList` / `copy()` 陷阱)

(展开见后续)

### 7. 扩展函数(vs 成员函数 vs 顶层函数)

(展开见后续)

### 8. 集合 API(`Sequence` vs `List` 切换)

(展开见后续)

### 9. 委托(`by lazy` / `Delegates.X` / 自定义)

(展开见后续)

### 10. 命名惯例(`toX` / `asX` / `xOrNull` / `xOrElse`)

(展开见后续)

### 11. 弃用替换(`!!` / `GlobalScope` / `lateinit` / `Array` 等历史包袱)

(展开见后续)

### 12. Public API explicit return type(库 / 模块边界)

(展开见后续)

### 13. 复杂度阈值(函数行数 / 嵌套深度 / 参数个数)

(展开见后续)

### 14. inline / reified / crossinline(高阶函数关键决策)

(展开见后续)

### 15. equals / hashCode / data class copy 陷阱

(展开见后续)

---

## Review Checklist

> 末尾 checklist,可被 `design-review` / `commit-review` 引用。

(展开见后续 T7)

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
