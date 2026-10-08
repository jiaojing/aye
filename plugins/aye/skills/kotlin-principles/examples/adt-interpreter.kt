/**
 * 可选范例:ADT + 能力接口，与有领域归属的普通函数共存。
 *
 * 有真实替换需求时用接口；简单领域规则直接用显式输入/结果的函数。
 *
 * 协程、相等性和公共 API 的边界判据见 SKILL.md / reference.md。
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

/** 领域规则输出:显式的 sealed result。 */
sealed interface Outcome {
    data object Continue : Outcome
    data class Halt(val reason: String) : Outcome
}

// ==== 行为接口(Type Class 层) ====

/** "对 Indicator 求值"的抽象接口——`fun interface` 让单方法 SAM 可由 lambda 实例化。 */
fun interface IndicatorEvaluator<R> {
    fun evaluate(i: Indicator): R
}

// ==== 解释器实现(具体 instance) ====

/** 带前缀的可读消息解释器。prefix 配置由实例持有。 */
class PrefixedMessageReporter(private val prefix: String) : IndicatorEvaluator<String> {
    override fun evaluate(i: Indicator): String = when (val s = i.state) {
        is IndicatorState.Normal -> "[$prefix·OK] ${i.name.raw}: ${s.message}"
        is IndicatorState.Alerting -> "[$prefix·${s.severity}] ${i.name.raw}: ${s.message}"
    }
}

/** 领域规则:无需替换能力时，模块内的纯函数足够。 */
fun determineOutcome(i: Indicator): Outcome = when (val s = i.state) {
    is IndicatorState.Normal -> Outcome.Continue
    is IndicatorState.Alerting -> Outcome.Halt(s.message)
}

// ==== 顶层扩展(解释器层入口 + 派生谓词) ====

/** 派生谓词:无副作用，显式解读领域状态。 */
val Indicator.isAlerting: Boolean get() = state is IndicatorState.Alerting

/** 顶层扩展提供便捷入口；report 使用解释器，outcome 使用领域函数。 */
fun Indicator.report(prefix: String = "aye"): String = PrefixedMessageReporter(prefix).evaluate(this)
fun Indicator.outcome(): Outcome = determineOutcome(this)
