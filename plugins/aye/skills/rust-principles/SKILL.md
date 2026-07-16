---
name: rust-principles
description: 'Rust 项目专属设计原则(横切)。在 Cargo.toml、.rs、cargo/rustc、Rust API 或所有权设计上下文触发。覆盖运行时 Service/Context/Value 分类、ADT + Trait 解释器、owned/borrowed 取舍、封装、错误、任务生命周期、框架边界、Default、命名、public API/MSRV 和 review checklist。'
---

# Rust Principles

只讲 Rust 特化；语言无关的品味和工作流分别走 `principles`、`review`、`flow`。

## 运行时三分法

先判断类型寿命和所有权，再选择引用、move、Clone 或 Arc：

| 分类 | 寿命 | 默认传递 | 例子 |
|---|---|---|---|
| Service | 长生命周期，由 composition root 构造 | `&T`；确有共享所有权时 `Arc<T>` | client、pool、配置快照 |
| Context | pipeline 中间产物 | owned move | model、engine、request context |
| Value | 小型领域值 | by value / borrow | newtype、money、timestamp |

`Arc` 表达共享所有权，不表达“全局”。不要把 Service 自动做成 singleton，也不要让 Value 为了省一次复制感染整条 lifetime 链。

## 数据与行为

- ADT 用 `enum` / `struct` 承载状态；能力用 `trait`，解释器用 `impl Trait for Type`。
- 行为必须有类型归属：inherent `impl`、trait 或解释器；避免无主的 public 自由函数。
- 多个 `impl` 块可按职责拆分，同一类型仍保持自然的 `obj.method()` 调用。
- 扩展 trait 与便捷 inherent method 可以作为入口，但核心解释逻辑不要重复。

## Ownership / Lifetime

- 应用层优先 owned 数据，避免为了局部借用给整条对象图添加 lifetime。
- 库 API、零拷贝热路径和明确的临时视图可以且应该借用；不要机械 Clone/Arc。
- Clone 必须表达合理的复制语义；无理由的 `.clone()` 是所有权设计信号。
- 锁不能跨 `.await`；先缩小 guard 作用域或重构状态所有权。

## 封装与 Default

- 不暴露破坏不变量的 `pub` field；连续 `.field.field` 是缺能力方法的信号。
- 只有语义上有效、安全且无需隐藏 I/O 的状态才实现 `Default`。
- 不用空字符串、零 ID 或未连接 client 伪造“可默认”的半初始化对象。

## 错误

- 可恢复失败返回 `Result<T, E>`；库/领域错误优先 `thiserror`，应用边界聚合可用 `anyhow`。
- 能传播就用 `?`；`expect` 只用于已由不变量证明不可能失败的位置，并写清原因。
- 错误在调用栈中补上下文，只在系统边界记录一次；不要 log 后又原样 return。

## 并发与资源生命周期

- 每个 `tokio::spawn`/线程必须有 owner、取消路径和回收策略；不要丢弃需要观察的 `JoinHandle`。
- 外部 I/O 设置 timeout；shutdown 使用独立 deadline，不能复用已取消 context。
- 无界输入必须限制并发；先使用 runtime/库提供的结构化原语，不手写固定 worker pool。
- async 测试使用显式同步或暂停/推进虚拟时间，不靠 sleep 猜调度。

## Frameworks at the Edge

- clap、axum、tonic、tauri 等类型停在 adapter。
- adapter 转换成领域 command/config/value 后调用核心；核心不解析 flags、HTTP request 或 GUI event。
- 在 `main`/composition root 构造依赖并接线，避免可变全局状态。

## 抽象

- trait 由 consumer 的最小能力驱动；实现数量是信号，不是“三个才抽”的硬门槛。
- 同签名不代表同语义；抽 trait 前说明领域理由、替换需求或测试边界。
- 泛型用于复用真实算法和静态能力，不为可能永远不会出现的类型层级预制复杂度。

## 命名

| 前缀 | 语义 |
|---|---|
| `as_` | 廉价借用/视图 |
| `to_` | 复制或计算得到新值 |
| `into_` | 消费 self、转移所有权 |

同时遵守 `iter` / `iter_mut` / `into_iter`，不用 `get_` 前缀；newtype 用于阻止真实误用或维护不变量。

## 当前生态与兼容性

- 优先标准库已经稳定提供的能力，如 `OnceLock` / `LazyLock`；第三方替代必须有具体收益。
- `std::sync::Mutex` 与 `parking_lot::Mutex` 按 poisoning、性能、依赖和运行环境选择，不设全局默认。
- public trait 新增必需方法通常破坏外部 implementor。
- MSRV 是 public contract；提高前先确认 consumer，并在 release notes 说明。
- 发布库使用机械检查（如 `cargo semver-checks`）验证兼容性。

## Review Checklist

```text
[ ] 没有无理由 Clone/Arc 或不必要的 lifetime 扩散
[ ] 可恢复错误没有 unwrap；expect 写明已证明的不变量
[ ] 没有 pub field 泄漏不变量
[ ] unsafe 块有 SAFETY 注释
[ ] 没有 lock guard 跨 await
[ ] spawn/thread 有 owner、cancel、timeout、join/shutdown
[ ] framework 类型没有进入领域核心
[ ] Default 表达有效安全状态
[ ] public API / trait / MSRV 变化已做 consumer 兼容检查
[ ] 大函数和深嵌套已按命名职责拆分
```
