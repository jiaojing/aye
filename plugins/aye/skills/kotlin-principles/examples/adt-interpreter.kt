/**
 * 标准范例:基于 safeguard 简化的 ADT + Type Class 模式落地。
 *
 * 演示 `kotlin-principles` 的核心设计哲学——数据 / 行为接口 / 解释器实现 三层分离。
 *
 * 覆盖维度 1 / 5 / 6 / 7 / 10 / 12 / 15;协程 / 集合 / 委托见 SKILL.md。
 */
package aye.example.adt

// ==== 数据(ADT 层) ====

/** 监控指标的状态。sealed interface 闭合;data class 子类承载 message。 */
sealed interface IndicatorState {
    val message: String
    data class Normal(override val message: String) : IndicatorState
    data class Alerting(override val message: String, val severity: Severity) : IndicatorState
}

enum class Severity { LOW, HIGH }

/** 领域 newtype——防 String 误传。 */
@JvmInline value class IndicatorName(val raw: String)

/** 复合数据:Indicator = 名字 + 当前状态。 */
data class Indicator(val name: IndicatorName, val state: IndicatorState)

/** 解释器输出之一:领域结果(sealed result,维度 5)。 */
sealed interface Outcome {
    data object Continue : Outcome
    data class Halt(val reason: String) : Outcome
}

// ==== 行为接口(Type Class 层) ====

/** "对 Indicator 求值"的抽象接口——`fun interface` 让单方法 SAM 可由 lambda 实例化(维度 1 / 14)。 */
fun interface IndicatorEvaluator<R> {
    fun evaluate(i: Indicator): R
}

// ==== 解释器实现(具体 instance) ====
// 对照:有配置 → `class`(可构造多实例);无状态 → `object`(全局单例)。维度 1。

/** 解释器 1:带前缀的可读消息。需要 prefix 配置 → `class`。 */
class PrefixedMessageReporter(private val prefix: String) : IndicatorEvaluator<String> {
    override fun evaluate(i: Indicator): String = when (val s = i.state) {
        is IndicatorState.Normal -> "[$prefix·OK] ${i.name.raw}: ${s.message}"
        is IndicatorState.Alerting -> "[$prefix·${s.severity}] ${i.name.raw}: ${s.message}"
    }
}

/** 解释器 2:ADT → Outcome(sealed result,维度 5)。无状态 → `object`。 */
object OutcomeMapper : IndicatorEvaluator<Outcome> {
    override fun evaluate(i: Indicator): Outcome = when (val s = i.state) {
        is IndicatorState.Normal -> Outcome.Continue
        is IndicatorState.Alerting -> Outcome.Halt(s.message)
    }
}

// ==== 顶层扩展(解释器层入口 + 派生谓词) ====

/** 派生谓词:无副作用,纯解读 ADT。维度 7 的"对 ADT 加额外能力"正例。 */
val Indicator.isAlerting: Boolean get() = state is IndicatorState.Alerting

/** 顶层扩展作快捷入口,封装默认 evaluator——业务调用方零感知具体实现。 */
fun Indicator.report(prefix: String = "aye"): String = PrefixedMessageReporter(prefix).evaluate(this)
fun Indicator.outcome(): Outcome = OutcomeMapper.evaluate(this)
