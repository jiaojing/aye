---
name: kotlin-principles
description: 'Kotlin 的领域类型、协程生命周期、值语义与公共 API 判据。用于 Kotlin 设计、实现或审查；需要具体陷阱时按需读取 reference。'
---

# Kotlin Principles

聚焦 Kotlin 特化；通用领域边界与工程取舍见 `principles`，审查见 `review`。

## 数据与行为

- 根据合法状态选择 sealed、enum、data/value class。成员、扩展、领域顶层函数与 interface 解释器都可承载行为，按真实封装和替换需求选择。
- 扩展函数不拥有 receiver 的私有状态；不能靠它保护可任意构造或修改的数据。避免隐藏 Service locator 和 I/O。
- `object` 可表达无状态策略或确有进程生命周期的实例；有状态服务的 owner、构造和关闭应明确。
- Ktor、Spring、Android、Compose 类型留在 adapter/lifecycle 边界，核心接收领域输入。
- ADT + interface 的可选范例见 [examples/adt-interpreter.kt](examples/adt-interpreter.kt)，不要求每个操作增加解释器。

## 按风险检查

| 风险 | 判据 |
|---|---|
| 非法领域值 | 检查构造、copy、可变字段和输入转换能否绕过约束 |
| 协程生命周期 | scope owner、取消传播、timeout、失败与 shutdown |
| 错误语义 | 领域 outcome、异常和缺值区分；catch/runCatching 不吞 cancellation |
| 值语义 | 只读不等于不可变；Array 相等性、浅 copy 和 constructor/body 字段差异 |
| Public API | source/binary/toolchain 兼容；platform type、默认参数与 inline 实现泄漏 |

具体语义与例子按需读取 [reference.md](reference.md)。普通语法选择遵循项目惯例，复杂度数字只提示检查职责，不能代替上下文判断。

性能或 inline/集合策略变化按实际调用、版本与测量判断，不从语法外观推断收益。
