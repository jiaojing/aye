---
name: kotlin-principles
description: 'Kotlin 项目专属设计原则(横切)。在 *.kt、Gradle Kotlin DSL、kotlinx、Ktor、Spring、Android 或 Compose 上下文触发。覆盖 ADT + interface 解释器、null safety、scope functions、协程生命周期、错误、不可变、扩展函数、集合、委托、命名、public/binary API、复杂度、inline 和 data class 陷阱。'
---

# Kotlin Principles

只讲 Kotlin 特化；语言无关判据走 `principles` / `review`。详细正反例按需读取 [`reference.md`](reference.md)。

## ADT + 行为解释器

| 层 | Kotlin |
|---|---|
| 数据 ADT | `sealed interface` / `data class` / `data object` / `@JvmInline value class` |
| 行为签名 | `interface` / `fun interface` |
| 解释器 | 实现类或无状态 `object` |
| 类型化入口 | 成员或薄的顶层扩展函数，委托给解释器 |

`sealed interface` 通常比 `sealed class` 保留更多继承自由；无字段闭合状态优先 enum。扩展函数可以提供 `expr.evaluate()` 的自然入口，但核心行为所有权仍归 interface/解释器，扩展保持薄且无状态。

完整示例见 [`examples/adt-interpreter.kt`](examples/adt-interpreter.kt)。

## 15 维度索引

| # | 维度 | 核心判断 |
|---|---|---|
| 1 | 类型设计 | sealed/data/value/enum 按状态形态选择 |
| 2 | Null safety | smart cast / `?.` / `?:` / `requireNotNull`；避免 `!!` |
| 3 | Scope functions | 按 receiver/返回值选；不嵌套、不写长链 |
| 4 | 协程 | structured concurrency、scope owner、cancel/timeout/shutdown |
| 5 | 错误 | 领域结果、异常和 nullable 各自表达清楚语义 |
| 6 | 不可变 | `val`、只读集合；理解 `copy()` 是浅拷贝 |
| 7 | 扩展函数 | 可做薄类型化入口；不隐藏状态、I/O 或 Service locator |
| 8 | 集合 | 默认 eager；大数据、多步或短路时评估 Sequence |
| 9 | 委托 | lazy、observable/vetoable、接口委托按真实重复使用 |
| 10 | 命名 | `toX` / `asX` / `xOrNull` / boolean `is/has/can` |
| 11 | 历史包袱 | 对 `!!` / GlobalScope / lateinit / Array / JVM interop 做上下文判断 |
| 12 | Public API | explicit API、接口而非实现、source/binary/toolchain 兼容 |
| 13 | 复杂度 | 行数/嵌套/参数/圈复杂度都是重构信号，不是机械失败线 |
| 14 | inline/reified | 只在 lambda、reified 或 non-local return 有收益时使用 |
| 15 | equals/copy | data class constructor、Array、mutable 引用和浅拷贝语义 |

命中某维度时读取 reference 对应段，不要每次加载全部细节。

## Frameworks at the Edge

- Ktor、Spring、Android、Compose 类型停在 adapter/lifecycle 边界。
- 核心接收领域 command/config/value，不读取 ApplicationContext、HTTP call 或 UI state。
- 有状态 service 用构造器创建，由 composition root/lifecycle owner 持有；`object` 只适合无状态行为或真正进程级常量。

## Review Checklist

```text
[ ] 没有无理由的 !!；失败语义没有用含糊 nullable 代替
[ ] CoroutineScope 有 owner；没有 GlobalScope 或 suspend 内 runBlocking
[ ] catch/runCatching 没有吞 CancellationException
[ ] 后台工作有 cancel、timeout 和 shutdown；测试用 runTest/虚拟时间而非 sleep
[ ] data class 没有意外的 mutable/Array 引用语义；copy 浅拷贝已明确
[ ] 没有向 consumer 暴露 MutableList/MutableMap 或具体并发实现
[ ] scope function 不嵌套，长链已命名拆分
[ ] 扩展函数是薄入口，不隐藏 service lookup、状态或 I/O
[ ] framework 类型没有进入领域核心
[ ] public API 显式且已检查 source/binary/toolchain 兼容
[ ] 复杂度阈值只作为信号，拆分后职责更清楚
```
