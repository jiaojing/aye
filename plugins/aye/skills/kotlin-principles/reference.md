# Kotlin 语义与边界参考

按 `SKILL.md` 命中的风险读取相关部分。保留不易从语法外观判断的语义；版本敏感行为以项目配置和官方文档为准。

## 领域类型与受控构造

sealed/enum 表达闭合状态，data class 提供值操作，value class 表达有语义的包装；这些机制不自动保证业务有效性。

- 检查 constructor、工厂、生成的 `copy()`、反序列化和可变字段是否允许绕过规则。
- 工厂返回的“已校验值”只有在其他构造入口受控时才能作为信任边界。copy 的可见性与项目编译器设置一起检查。
- 同一领域状态有互斥字段时优先表达合法组合；不要因为存在 nullable 就自动判错。
- 成员适合需要私有状态的不变量；领域顶层函数适合显式输入/结果的规则，真实替换需求才引入 interface。
- 扩展函数是静态解析的；不要将扩展的同名调用误当成成员多态。
- `data object` 在 Kotlin 1.9 稳定；data class 自 Kotlin 1.1 起可继承其他类。是否采用由状态与相等性语义决定，不设普遍继承禁令。

来源：[Kotlin 1.9 data objects](https://kotlinlang.org/docs/whatsnew19.html#stable-data-objects-for-symmetry-with-data-classes)、[Kotlin 1.1 data class inheritance](https://kotlinlang.org/docs/whatsnew11.html#sealed-and-data-classes)、[扩展的静态解析](https://kotlinlang.org/docs/extensions.html#extension-or-member-functions)。

## 协程与取消

- scope 的 owner 决定何时取消、等待和释放资源。需要独立生命周期时显式构造与持有 scope，不从便利调用隐式创建无主工作。
- `coroutineScope` 与 `supervisorScope` 的选择取决于子失败是否应传播；supervisor 不等于错误可以忽略。
- 阻塞调用需要合适的调度/隔离；`suspend` 不保证函数体不会阻塞线程。在 suspend 路径中使用 `runBlocking` 是需要调查的阻塞信号。
- 外部 I/O、后台工作和 shutdown 明确 deadline、cleanup 与失败传播。
- `runCatching` 会捕获 Throwable。泛化 catch 或转换成结果时，不把取消转换为普通业务失败。

```kotlin
try {
    work()
} catch (e: CancellationException) {
    throw e
} catch (e: Exception) {
    handleFailure(e)
}
```

避免重复抄入已被下层库正确处理的逻辑；检查实际 catch 边界。异步测试使用 runTest、test dispatcher/受控时间或显式同步，避免依赖真实 sleep 的调度猜测。

来源：[协程取消](https://kotlinlang.org/docs/coroutines-cancellation.html)、[runCatching](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/run-catching.html)。

## 缺值与失败

nullable 表达缺值，异常表达约定的失败或编程错误，sealed result 表达调用方需要区分的领域 outcome。沿用项目契约，不为统一形式改写所有错误路径。

如果 null 同时代表“未找到”“校验失败”和“I/O 失败”，应拆清调用方真正需要的结果语义。`!!` 的关键问题是未证明的假设与缺少失败上下文，不将所有语法使用自动视为缺陷。

## 相等性、只读与 copy

- data class 默认生成的 equals/hashCode、copy 和 componentN 基于主构造器属性；body 属性不自动参与。
- Array 的默认 equals 是引用语义，若领域需要内容相等则显式实现或改变表示。
- `val` 固定引用，`List` 限制接口上的修改能力，均不保证底层对象不可变；consumer 可能观察到别处的写入。
- `copy()` 是浅拷贝，嵌套可变对象会共享；body 属性/初始化逻辑会在新实例重新执行。
- 可变 hashCode 参与 Map/Set key 时可能破坏查找；检查身份与值语义，而非机械要求所有对象成为 data class。
- 继承中的 final equals/hashCode 会影响生成行为；核对整个层级的相等性契约。

```kotlin
data class Snapshot(val items: MutableList<String>)
val first = Snapshot(mutableListOf("a"))
val second = first.copy()
second.items.add("b") // first 也会观察到修改
```

来源：[Data classes](https://kotlinlang.org/docs/data-classes.html)、[集合类型](https://kotlinlang.org/docs/collections-overview.html)。

## Public API 与 JVM 边界

- public API 明确语义和返回类型，避免推导出 platform type 或具体可变实现；已有 explicit API 模式继续使用。
- source compatibility 与 binary compatibility 分别检查；interface 方法、JVM signature、可见性、默认参数或工具链变化可能影响不同 consumer。
- public inline 的代码进入调用方，注意实现/可见性与 ABI 演进；reified 等能力按实际需求使用。
- Java interop、Array、lateinit、@JvmStatic 等按真实 consumer 或 framework lifecycle 判断，不设脱离用途的语法禁令。
- 公共契约变化使用项目已有的兼容检查；缺少证据时明确说明，不只用当前源码编译通过证明兼容。

来源：[库 API 兼容指南](https://kotlinlang.org/docs/api-guidelines-backward-compatibility.html)、[Inline functions](https://kotlinlang.org/docs/inline-functions.html)。
