# Kotlin Principles Reference: 15 维度判据详细

> 本文档是 `kotlin-principles` SKILL.md 的详细参考。SKILL.md 留索引骨架,各维度详细正反例 + 决策表在这里。
> LLM 在 review Kotlin 代码命中某维度时,Read 本文档对应段查详细。

每条三段式:**判据 / 正例 / 反例**。阈值和优先级是 review 信号，不代替项目上下文。

## 目录

1. 类型设计
2. Null safety
3. Scope functions
4. 协程基础
5. 错误处理
6. 不可变
7. 扩展函数
8. 集合 API
9. 委托
10. 命名惯例
11. 历史包袱与上下文选择
12. Public API 与兼容性
13. 复杂度信号
14. inline / reified
15. equals / copy

---

## 1. 类型设计(sealed / data / value / object / enum 选用)

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

## 2. Null safety(优先级阶梯)

**判据(5 档)**:

1. **smart cast**:`if (x != null) x.foo()`(编译器自动消除)
2. **`?.let { } / ?:`**:短链 1-2 层
3. **`requireNotNull(x) { "..." }`**:前置条件违反 / 早抛
4. **`as?`**:类型转换可能失败,返回 nullable 自然链 `?:`
5. **避免 `!!`**:它不携带失败原因;优先用能表达缺值或不变量的工具

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
val name = config.name!!                       // ❌ 失败时缺少领域上下文
val s: String = obj as String                  // ❌ 若类型并非已证明不变量,应使用 as?
```

---

## 3. Scope functions(let / run / with / apply / also 五选一)

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

## 4. 协程基础(语言层)

> 范围:`suspend` 语义 / 结构化并发 / `Dispatchers.Default/IO/Main` 边界。**`Flow` / `Channel` 不在本 skill**(深度内容留按需召唤型 skill)。

**判据**:

1. **结构化并发**:`coroutineScope { }` 子协程绑定当前 scope,失败传播
2. **Scope owner**:应用、request、viewModel/service lifecycle 必须明确谁 cancel 和等待结束
3. **`supervisorScope`**:只有子失败不应传染兄弟时使用
4. **`Dispatchers` 边界**:CPU bound → `Default`;阻塞 IO → `IO`;UI → `Main`
5. **Timeout/shutdown**:外部 I/O 和后台工作明确 deadline、cleanup 和失败传播
6. **禁 `GlobalScope.launch`**:没有 lifecycle owner
7. **禁 `runBlocking` 在 `suspend fun` 内**:可能阻塞线程池
8. **禁吞 `CancellationException`**:`runCatching` / `catch (Exception)` 必须透传 cancel
9. **确定性测试**:使用 `runTest`、test dispatcher/virtual time 或显式同步,不用 `Thread.sleep`

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

## 5. 错误处理(exception vs `Result<T>` vs sealed result)

**判据**:

1. 有多个调用方必须区分的领域 outcome:使用 sealed result,获得穷举检查
2. 跨边界需要把异常作为值传递:可使用 `Result<T>`,协程内仍要透传 cancellation
3. 编程错误或不变量违反:抛出带上下文的 exception
4. Nullable 只表达"无值";不要让 null 同时表示 not found、validation fail 和 I/O fail

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

## 6. 不可变(`val` / `var` / `List` vs `MutableList` / `copy()` 陷阱)

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

## 7. 扩展函数(vs 成员函数 vs 顶层函数)

**判据**:

1. 类型自己维护的不变量和固有行为:使用成员
2. ADT + interface 解释器:核心行为由 interface/实现类承载,可用薄扩展提供类型化入口
3. stdlib/第三方类型的额外能力:扩展函数是自然选择
4. 无明确 receiver 的算法:使用有领域归属的顶层函数,避免 `utils` 垃圾桶
5. 扩展函数保持无状态,不隐藏 I/O、Service locator 或昂贵副作用

**正例**:

```kotlin
class Order { fun submit(): Outcome { ... } }  // 成员维护 Order 不变量

fun Instant.toSeoul(): ZonedDateTime = atZone(ZoneId.of("Asia/Seoul"))

val Expr.isLeaf: Boolean get() = this is Expr.Num

fun Expr.evaluateWith(eval: Eval<Int>): Int = eval.eval(this) // 薄解释器入口
```

**反例**:

```kotlin
fun Order.submit(): Outcome = GlobalServices.orders.submit(this) // ❌ 隐藏 service lookup 和 I/O
```

---

## 8. 集合 API(`Sequence` vs `List` 切换)

**判据**:

1. **默认 List**:`map / filter / fold` 等,小数据或结果本来就要物化
2. **评估 Sequence**:数据量大、多步中间集合昂贵,或短路场景(`first` / `any` / `take`)
3. 单步后立即 `toList()` 通常没有收益;性能敏感时用 benchmark 判断

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

## 9. 委托(`by lazy` / `Delegates.X` / 自定义)

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

## 10. 命名惯例(`toX` / `asX` / `xOrNull` / `xOrElse`)

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

## 11. 历史包袱与上下文选择

以下用法是 review 信号,不是脱离上下文的语法禁令。

| 信号 | 默认替代 | 允许例外 |
|---|---|---|
| `!!` | `?:` / `requireNotNull` / `checkNotNull` | 极窄的测试夹具;仍应给失败上下文 |
| `GlobalScope.launch` | 明确 owner 的 `CoroutineScope` | 基础设施级进程 scope,且有显式 shutdown |
| suspend 内 `runBlocking` | 直接 suspend/await | 无 |
| `lateinit` | 构造器注入 / nullable state / `lazy` | framework lifecycle 或测试注入要求时 |
| `Array<T>` | `List<T>` / primitive array | Java interop、固定缓冲区或性能证据 |
| companion 只放 const | top-level `const val` | 常量需要类型命名空间时 |
| `@JvmStatic` | 普通 Kotlin API | Java consumer 明确需要静态入口 |

---

## 12. Public API 与兼容性

**判据**:

1. 库/模块 public API 显式标返回类型,避免泄漏 platform type 或具体实现
2. 私有/file-private 可使用推导
3. 库启用 `-Xexplicit-api=strict`
4. public interface 新增 abstract member、改变 JVM signature、可见性或默认参数可能破坏 source/binary consumer
5. 提高 Kotlin/JVM/Gradle 最低版本是兼容性变更;发布前使用 binary compatibility validator 等机械检查

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

## 13. 复杂度阈值(函数行数 / 嵌套深度 / 参数个数)

**判据**:以下数字是 review 信号,不是机械失败线。

- 函数约 50 行以上:检查是否包含多个可命名职责
- 嵌套超过 3 层:优先早返回、拆函数或重建数据形态
- 参数超过 5 个:检查是否存在领域 command/config 对象
- 圈复杂度超过团队 detekt 阈值:检查状态建模和分支归属

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

## 14. inline / reified / crossinline(高阶函数关键决策)

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

## 15. equals / hashCode / data class copy 陷阱

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
