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

**判据(决策表)**:

| 场景 | 首选 |
|---|---|
| 多分支闭合状态(子类有字段差异) | `sealed interface`(优于 `sealed class`) |
| 纯字段记录(equals / hashCode / copy 自动) | `data class` |
| 单字段领域类型(newtype) | `@JvmInline value class` |
| 闭合 + 所有子类无字段 | `enum class`(比 sealed 更轻) |
| 单例 / 标记对象 | `data object`(2.0+) > `object` |

**判据补充**:

- `sealed interface` > `sealed class`:子类不被单继承占死,`value class` 可实现 sealed interface(反之不行)
- 不要用 `class` 包字段(无 `equals`)——除非真需引用语义

**正例**:

```kotlin
sealed interface OrderStatus {
    data object Pending : OrderStatus            // 单例标记
    data class Filled(val price: Price) : OrderStatus
    data class Cancelled(val reason: String) : OrderStatus
}
@JvmInline value class Price(val raw: BigDecimal)  // newtype
@JvmInline value class OrderId(val raw: Long)
```

**反例**:

```kotlin
class OrderStatus(
    val tag: String,                // ❌ 字符串标记
    val price: BigDecimal? = null,  // ❌ 散落 nullable
    val reason: String? = null,
)
```

**用 string + 多 nullable 模拟 ADT** 是 sealed 的反义词——`when` 编译器不能穷举,失修代价大。

---

### 2. Null safety(优先级阶梯)

**判据(5 档)**:

1. **smart cast**:`if (x != null) x.foo()`(编译器自动消除)
2. **`?.let { } / ?:`**:短链 1-2 层
3. **`requireNotNull(x) { "..." }`**:前置条件违反 / 早抛
4. **`as?`**:类型转换可能失败,返回 nullable 自然链 `?:`
5. **禁 `!!`**:**任何场景**都有上面之一更好

**正例**:

```kotlin
val x: String? = ...
if (x != null) println(x.length)               // smart cast
user?.let { logger.info("login: $it") }        // ?.let
val name = config.name ?: "unknown"            // ?:
val token = requireNotNull(req.token) { "token required" }  // 前置条件
val str = obj as? String ?: return null        // as? + ?:
```

**反例**:

```kotlin
val name = config.name!!                       // ❌ 禁
val s: String = obj as String                  // ❌ 应用 as?,失败抛 ClassCastException
```

---

### 3. Scope functions(let / run / with / apply / also 五选一)

**判据(决策表)**:

| 函数 | 内部访问 | 返回 | 用途 |
|---|---|---|---|
| `let` | `it` | lambda 结果 | 非空对象 → 转换 / 链式 |
| `run` | `this` | lambda 结果 | 临时 receiver scope + 计算结果 |
| `with` | `this` | lambda 结果 | 多次访问同一对象 |
| `apply` | `this` | this | **配置链式**(返回原对象) |
| `also` | `it` | this | 副作用(log / 校验)+ 不破坏链 |

**判据补充**:

- 不要嵌套 scope 函数(Kotlin Coding Conventions 原话)
- 不要长链(>3 层)

**正例**:

```kotlin
val req = Request().apply {                    // apply: 配置
    method = "POST"
    body = json
}

nullableUser?.let { it.toDto() }               // let: 非空 → 转换

list.filter { it.active }
    .also { logger.info("active: ${it.size}") }  // also: 副作用,不破坏链
    .map { it.id }
```

**反例**:

```kotlin
config.apply { user.apply { ... } }            // ❌ 嵌套
val name = user.apply { logger.log(this) }     // ❌ apply 当 let 用 — name = user 不是日志
```

---

### 4. 协程基础(语言层)

> 范围:`suspend` 语义 / 结构化并发 / `Dispatchers.Default/IO/Main` 边界。**`Flow` / `Channel` 不在本 skill**(深度内容留按需召唤型 skill)。

**判据**:

1. **结构化并发**:`coroutineScope { }` 子协程绑当前作用域,失败传播
2. **`supervisorScope`**:子失败不传染兄弟时
3. **`Dispatchers` 边界**:CPU bound → `Default`;IO(网络/磁盘)→ `IO`;UI → `Main`
4. **禁 `GlobalScope.launch`**(`@DelicateCoroutinesApi`,无 lifecycle / 资源泄漏 / 无 ExceptionHandler)
5. **禁 `runBlocking` 在 `suspend fun` 内**(redundant + thread starvation)
6. **禁吞 `CancellationException`**:`runCatching` / `catch (e: Exception)` 在协程内必须透传 cancel

**正例**:

```kotlin
suspend fun fetchAll() = coroutineScope {
    val a = async(Dispatchers.IO) { fetchA() }
    val b = async(Dispatchers.IO) { fetchB() }
    a.await() to b.await()
}

try {
    work()
} catch (e: CancellationException) {
    throw e                                    // 透传 cancel
} catch (e: Exception) {
    logger.error(e) { "fail" }
}
```

**反例**:

```kotlin
GlobalScope.launch { ... }                     // ❌
runBlocking { suspendFun() }                   // ❌ 在 suspend fun 内
runCatching { work() }.onFailure { logger.error(it) }  // ❌ 协程内吞 CancellationException
```

---

### 5. 错误处理(exception vs `Result<T>` vs sealed result)

**判据(优先级阶梯)**:

1. **领域结果用 sealed result**:类型安全,业务流程显式,`when` 穷举
2. **跨边界 / 调用方做异常处理**:`Result<T>` + `runCatching` 包外部 API(注意维度 4 协程内透传 cancel)
3. **真错误 / 不变量违反**:`throw` exception(`IllegalStateException` / 自定义 `class XxxException : RuntimeException`)
4. **禁** 用 nullable 表"操作失败"——nullable 表"无值",不表"出错"

**正例**:

```kotlin
sealed interface PaymentResult {
    data class Success(val txId: TxId) : PaymentResult
    data class Insufficient(val balance: Money) : PaymentResult
    data class Declined(val reason: DeclineReason) : PaymentResult
}

when (val r = pay(amount)) {
    is Success -> notify(r.txId)
    is Insufficient -> ask(r.balance)
    is Declined -> showError(r.reason)
}
```

**反例**:

```kotlin
fun pay(amount: Money): TxId? = ...            // ❌ null 是"成功 or 失败 or 没找着"?歧义
fun pay(amount: Money): Pair<Boolean, TxId?> = ...  // ❌ 反 ADT
```

---

### 6. 不可变(`val` / `var` / `List` vs `MutableList` / `copy()` 陷阱)

**判据**:

1. **`val` 优先于 `var`**——可变时显式说明
2. **`List` 优先于 `MutableList`**——接口暴露 read-only,实现可以是 mutable
3. **修改 data class → `copy()`**,不要 mutable field
4. **`copy()` 是浅拷贝**——嵌套 mutable 引用共享

**正例**:

```kotlin
data class Order(val id: OrderId, val items: List<Item>)  // val + List

fun process(o: Order, x: Item): Order = o.copy(items = o.items + x)

class Repo {
    private val _orders = mutableListOf<Order>()       // 内部可变
    val orders: List<Order> get() = _orders.toList()   // 暴露不可变
}
```

**反例**:

```kotlin
data class Order(var id: OrderId)              // ❌ var 在 data class → equals/hashCode 跟着变
class Repo { val orders: MutableList<Order> = ... }    // ❌ 暴露 mutable
```

---

### 7. 扩展函数(vs 成员函数 vs 顶层函数)

**判据(优先级阶梯)**:

1. **成员函数**(首选):行为属于类型自身
2. **顶层函数**:无类型归属的工具,放 utils/
3. **扩展函数**(谨慎):**仅当**无法改源类(stdlib / 第三方) + 行为是"对该类型的额外能力" + 不是核心领域行为
4. **禁** 领域行为写成顶层扩展(应是 Service interface + 实现)

**正例**:

```kotlin
class Order { fun submit(): Outcome { ... } }  // 成员: 领域行为

fun Instant.toSeoul(): ZonedDateTime = atZone(ZoneId.of("Asia/Seoul"))  // stdlib 加能力

val Expr.isLeaf: Boolean get() = this is Expr.Num  // ADT 派生谓词(三分法解释器层)
```

**反例**:

```kotlin
fun Order.submit(): Outcome = OrderService.submit(this)  // ❌ 领域行为漂浮成扩展
```

---

### 8. 集合 API(`Sequence` vs `List` 切换)

**判据**:

1. **默认 List**:`map / filter / fold` 等,小数据 / 单步
2. **切 Sequence**:链 ≥ 3 步 + 数据量大,**或**短路场景(`first` / `any` / `take`)
3. **禁** Sequence 滥用——单步 + 立即终结时 Sequence 比 List 慢(开销)

**正例**:

```kotlin
users.filter { it.active }                     // 单步: List

hugeList.asSequence()                          // 多步 + 短路 + 大数据: Sequence
    .filter { it.active }
    .map { it.toDto() }
    .first { it.country == "CN" }
```

**反例**:

```kotlin
list.asSequence().map { it.id }.toList()       // ❌ 单步 + 立即终结,Sequence 没价值
```

---

### 9. 委托(`by lazy` / `Delegates.X` / 自定义)

**判据(优先级阶梯)**:

1. **派生属性懒计算 → `by lazy`**(默认线程安全)
2. **观察 / 校验 set → `Delegates.observable / vetoable`**
3. **接口实现委托 → `class A(impl: B) : B by impl`**(避免手写一堆 forward)
4. **自定义 `ReadWriteProperty`**:**3 处重复再做**(rule of three)

**正例**:

```kotlin
class Indicator {
    val summary: String by lazy { computeExpensive() }   // 懒
}

class CompositeRepo(delegate: UserRepo) : UserRepo by delegate   // 接口委托
```

**反例**:

```kotlin
class A : B {                                  // ❌ 手写 forward,该用 by delegate
    override fun foo() = b.foo()
    override fun bar() = b.bar()
}
```

---

### 10. 命名惯例(`toX` / `asX` / `xOrNull` / `xOrElse`)

**判据**:

| 前缀 | 语义 |
|---|---|
| `toX` | 转换(可能贵 / 复制):`toString` / `toList` |
| `asX` | 视图 / 廉价转换(共享底层):`asSequence` / `asStateFlow` |
| `xOrNull` | 失败返回 null:`getOrNull` / `firstOrNull` |
| `xOrElse` | 失败返回默认值:`getOrElse` |
| `xOrThrow` | 失败抛异常(默认即抛) |

**其他硬规则**:

- 不用 `get_` 前缀:`name` 不是 `getName`
- Boolean 用 `is/has/can`:`isEmpty` / `hasNext` / `canSubmit`

**正例**:

```kotlin
fun User.toDto(): UserDto = ...
fun MutableList<T>.asReadOnly(): List<T> = this
val isAlerting: Boolean get() = ...
```

**反例**:

```kotlin
fun getName(): String = name                   // ❌ 应是 val name
fun checkValid(): Boolean = ...                // ❌ check 不像谓词,应 isValid
```

---

### 11. 弃用替换(`!!` / `GlobalScope` / `lateinit` / `Array` 等历史包袱)

**判据(直接禁用清单)**:

| 别用 | 改用 | 原因 |
|---|---|---|
| `!!` | `?.let` / `?:` / `requireNotNull` | 维度 2 |
| `GlobalScope.launch` | 显式 `CoroutineScope` | 维度 4 |
| `runBlocking` 在 suspend 内 | 直接 `await` / 改 suspend | 维度 4 |
| `lateinit var x: T` | 构造器注入 / `by lazy` | 暴露未初始化态 + var |
| `Array<T>` | `List<T>` / 原始数组(`IntArray` 等) | `Array<T>` 是 boxed,equals 引用比较 |
| `companion object` 单纯放 const | top-level `const val` | 顶层 const 更简洁,不需 import companion |
| `@JvmStatic` 滥用 | 仅 Java 互操作必要时 | Kotlin 内调用不需要 |

每条用法即反例。

---

### 12. Public API explicit return type(库 / 模块边界)

**判据**:

1. **库代码 / 模块 public API:必须显式标返回类型**——避免 inferred type 漏 platform type / 暴露内部实现
2. **私有 / file-private 可省略**:推导 OK
3. **强制化**:`-Xexplicit-api=strict` 编译选项

**正例**:

```kotlin
fun getUser(id: UserId): User = ...                         // 显式
val cache: ConcurrentMap<UserId, User> = ConcurrentHashMap()  // 暴露接口而非实现

private fun helper() = computeStuff()          // 私有:推导 OK
```

**反例**:

```kotlin
val cache = ConcurrentHashMap<UserId, User>()  // ❌ 暴露 ConcurrentHashMap 实现细节
fun load() = readJson(file)                    // ❌ 推导出啥?String? Map?
```

---

### 13. 复杂度阈值(函数行数 / 嵌套深度 / 参数个数)

**判据**:

- **函数行数 < 50**(超了考虑拆)
- **嵌套深度 ≤ 3**(参 Linus 内核风格)
- **参数 ≤ 5**(超了用 data class 包装)
- **圈复杂度 ≤ 10**(detekt 默认)

**补充**:

- 嵌套深度过高 → smart cast 链断,可读性骤降
- 参数过多 → builder / data class

**正例**:

```kotlin
data class CreateOrder(                        // 包装多参数
    val userId: UserId,
    val items: List<Item>,
    val coupon: Coupon?,
    val deliveryAt: Instant,
    val notes: String,
)
fun create(req: CreateOrder): Order = ...
```

**反例**:

```kotlin
fun create(                                    // ❌ 7 个参数
    userId: UserId, items: List<Item>, coupon: Coupon?,
    deliveryAt: Instant, notes: String, source: Source, retry: Int,
): Order = ...
```

---

### 14. inline / reified / crossinline(高阶函数关键决策)

**判据**:

1. **`inline`**:函数体嵌入调用点,**仅当**接收 lambda + lambda 体短(避免代码膨胀),**或**需要 `reified` 类型参数,**或**需要 non-local return
2. **`reified <T>`**:运行时拿泛型(必须 `inline`)
3. **`crossinline`**:lambda 不能 non-local return
4. **`noinline`**:某 lambda 参数不嵌入

**判据补充**:不是所有高阶函数都加 `inline`——库 API 才加,业务代码不必。

**正例**:

```kotlin
inline fun <reified T> Json.parse(s: String): T = ...      // reified

inline fun forEach(action: (Item) -> Unit) {               // non-local return
    for (i in items) action(i)
}
```

**反例**:

```kotlin
inline fun add(a: Int, b: Int): Int = a + b                // ❌ 没 lambda,inline 没意义
```

---

### 15. equals / hashCode / data class copy 陷阱

**判据**:

1. **data class equals/hashCode 仅基于 constructor 字段**——body 里 `val x = ...` 不参与
2. **不要 data class 含 `Array` 字段**:`equals` 引用比较,默认就出错
3. **`copy()` 是浅拷贝**——嵌套 mutable 引用共享(维度 6 已说)
4. **`val x by lazy` 在 data class**:`copy()` 时 lazy 重置——理解就 OK
5. **不要 data class 继承普通 class**(2.0 后允许但 equals 语义混乱)

**正例**:

```kotlin
data class Order(val id: OrderId, val items: List<Item>)
val o1 = Order(id, listOf(a, b))
val o2 = o1.copy()
o1 == o2         // true(结构相等)
```

**反例**:

```kotlin
data class Buf(val data: ByteArray)            // ❌ Array 字段:Buf(byteArrayOf(1)) != Buf(byteArrayOf(1))
data class Mut(val items: MutableList<Int>)    // ❌ mutable 字段:copy 后共享 list,改一个动两个
```

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
