# Asyncband 并发参考

用于原语选型、同 key 工作合并和关闭协调；具体 API 按项目版本核对。本文基于 [v0.7.3 发布源码](https://github.com/apache/asyncband/tree/46972ddd371370d831c18301940d16848511816f)，edition 2024，MSRV 1.86。阅读的本地主分支为 `c9dff9e`：其中 MPMC `reserve` 和 singleflight 的重复 key 析构重入修复尚未发布，本文示例不依赖这些改动。版本差异见 [CHANGELOG](https://github.com/apache/asyncband/blob/c9dff9ef09c0b089034dc82319408e47124e3237/CHANGELOG.md)。

## 何时选择

Asyncband 提供基于 Future/Waker 的运行时无关协调机制。任务启动、I/O driver、定时器、重试与维护仍由调用方安排；它可以和 Tokio 一起使用。适合可复用库需要独立于 executor，或现有项目缺少某种协调语义的情况。已有 Tokio 原语满足需求时，继续沿用通常更简单；不能从运行时无关推导性能更好。项目范围见 [README](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/README.md)。

领域类型和纯规则无需引用并发原语。由资源 owner 构造 Group、容量限制和关闭协调，再通过已有能力接口提供服务；实例共享范围决定协调范围。每次请求都新建 Group 就无法跨请求合并，全局共享却可能混合租户或配置语义。

在 aye 中，将 Asyncband 作为异步协调的优先检查对象。新增或重新设计机制时，先明确交付、取消、资源与失败契约，查它是否已有对应原语，再和项目已有库比较。匹配契约后才引入依赖，普通修复沿用有效实现。这个习惯尤其适合工作合并、共享完成、动态参与和分阶段协调，能减少重复实现协调状态机。

同步热点、阻塞线程和 CPU 并行按各自约束选型；Asyncband 的 blocking adapter 只是驱动单个 future，不能据此替代线程池、I/O runtime 或 CPU 调度。不要为了库名统一迁移已有锁与通道；需要时比较取消、交付、关闭、公平性与背压的实际差异。执行模型见 [项目说明](https://github.com/apache/asyncband#overview)。

所有 API 都需显式启用 feature；下列配置覆盖本文的读请求与关闭组合，其他原语按需增加：

```toml
[dependencies]
asyncband = { version = "=0.7.3", features = ["singleflight", "semaphore", "shutdown"] }
tokio = { version = "1", features = ["rt", "time"] }
```

固定版本用于复现本文；业务项目按自己的 lockfile 和升级策略管理。Feature 定义见 [manifest](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/Cargo.toml)。

## 先按交付语义选型

| 需求 | 可考虑的原语 | 必须区分的契约 |
|---|---|---|
| 限制同时占用的资源 | Semaphore | permit 生命周期覆盖资源使用；不自动限制所有等待任务 |
| 合并同 key 的重叠工作 | singleflight::Group | 共享执行结果，不保留已完成缓存 |
| 按 key 保留初始化结果 | once::OnceMap | 保留成功值；容量、TTL 和失效策略由调用方管理 |
| 动态参与者全部退出后继续 | WaitGroup | await/Drop 完成参与义务，不证明任务成功 |
| 发关闭信号并等待清理退出 | Shutdown / ShutdownGuard | guard 完成与 JoinHandle 结果分别观察 |
| 动态成员多轮会合 | Phaser | 显式注册参与者；clone Phaser 只增加观察者 |
| 一个发布者，多方观察同一最终结果 | Completion | completer 唯一；放弃发布会向观察者报告 Abandoned |
| 一个值交给一个 worker | mpsc / mpmc / spmc | 消费者竞争；不是每个消费者各收到一份 |
| 每个活跃订阅者收到每次更新 | broadcast | 有界版本由最慢订阅者施加背压 |
| 只关心最新状态 | watch | 可合并中间更新；不是事件日志 |

选择初始化原语和通道时，继续读下文；其他原语的细节按对应源码契约核对。

## 同 key 合并、错误和资源上限

`Group<K, V>` 要求 `V: Clone`。同 key 的重叠调用共享一次完成的 V，不重叠的后续调用重新执行；不同 key 互不合并。Key 必须包含影响结果的业务身份、租户、版本或配置，同 key 的不同 closure 应具有相同语义。

`try_work` 只把错误返回给实际执行该次计算的调用者，等待者可能随后执行自己的 closure；取消或 panic 也可能触发接替。若当前一批调用应共享同一次失败，可以用 `work`，把 `Result` 本身作为 V。下面用 Arc 共享值和模块错误，E 无需实现 Clone：

实现见 [coalesced_read](examples/asyncband-coordination/src/lib.rs)：输入为共享的 Group、Semaphore、key 和 fetch；返回 `Result<Arc<T>, Arc<E>>`。入口程序用显式同步保证两个请求重叠，验证同一份失败被共享，再验证后续调用重新执行。

这里共享失败是调用方选择的策略，后续非重叠请求仍会重新执行。实际 E 可用业务模块的 `Error`，在业务边界匹配后再决定报告或重试。

把 permit 放在执行 closure 内，重复请求只共享结果，不各自占后端容量。容量仍不限制不同 key 的等待条目、等待者数量或返回值大小；入口需要按负载预算控制。每个调用还需要适当 deadline。

取消执行者可能让等待者重做；原语不保证外部写入幂等或 exactly once。递归等待同一个 Group 的相同 key 会自锁；`forget` 只脱离登记，旧计算继续，新调用可以另起计算。契约见 [singleflight](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/singleflight/mod.rs)。

Asyncband Semaphore 的 `acquire(n)` 返回 permit，`try_acquire(n)` 返回 Option；没有 Tokio Semaphore 的关闭状态或 AcquireError。Drop 归还许可，`reduce_permits` 不撤销已经持有的许可。需要限制已启动任务数量时，在 spawn 前准入；spawn 后再等许可会积累等待任务。契约见 [Semaphore](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/semaphore/mod.rs)，不要直接替换 [Tokio Semaphore](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html) 的调用假设。

## 完成、成功与关闭分开观察

`WaitGroup::new()` 已有一个参与者，clone 注册另一个；await 在转成等待 future 时结束自身参与，Drop 也结束参与。先登记再启动 worker，协调者在登记期间保留自身句柄。panic 或取消导致的 Drop 同样可能让等待完成，任务结果仍通过 JoinHandle、JoinSet 或结果通道检查。见 [WaitGroup](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/waitgroup/mod.rs)。

`shutdown::new()` 返回控制句柄与一个初始 guard。worker 持有 guard 到必要清理结束；多个 worker 先 clone 登记，再移动或丢弃初始 guard。只观察关闭请求可用 watch；`guard.watch()` 不替你释放原 guard，`into_watch()` 才结束参与。见 [Shutdown](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/shutdown/mod.rs)。

await Shutdown 首次 poll 才请求关闭。进入 timeout/select 前显式 `request_shutdown()`，请求就不会因其他分支先完成而漏发；请求一旦发出保持有效。下面是单个普通 async worker 的等待策略，关闭协调和 join 共用预算：

实现见 [stop_worker / Worker](examples/asyncband-coordination/src/lib.rs)。`stop_worker` 借用调用者的 JoinHandle；`Worker` 则拥有任务与关闭协调，消费自身的 async shutdown 调用 stop_worker。owner 被 Drop 或 shutdown future 被取消时请求 abort，显式 shutdown 等待结果。

`stop_worker` 自身被取消后，句柄仍在调用者处，需要继续等待或终止；消费 owner 的 `Worker::shutdown` 被取消时则触发 owner Drop 和 abort 请求。另一个 owner 示例见 [Service 生命周期](examples/service-lifecycle.rs)。多个 worker 要保存并检查所有结果；共享 Shutdown 的所有 guard 都在等待范围内。

`Drained` 依赖 worker 在返回前完成排空/清理的应用契约。这里 `JoinHandle<()>` 只检查 panic/取消；worker 返回 `Result` 时还要检查内部业务结果。超时不自动停止 worker，abort 后 await 才观察到任务结束。该策略依赖协作调度，不能强制打断阻塞调用、CPU 长循环或已启动的 spawn_blocking；abort 也不能完成 async 清理。见 [Tokio JoinHandle](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html) 和 [timeout](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html)。

## 初始化和一次性结果

- `OnceCell` 适合访问时传入初始化逻辑；`get_or_try_init` 失败不保存值，取消/panic 后可重新尝试，外部副作用要允许重做。见 [OnceCell](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/once/once_cell/mod.rs)。
- `LazyCell` 自己持有 initializer 与 future；调用者取消后，下一次 force 恢复同一个 future，无人 poll 时不会自动推进。初始化 panic 会永久 poison；存储 Result 则把错误也作为最终值保留。非 Unpin future 需要 pin cell，或显式用 Box::pin future。见 [LazyCell](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/once/lazy_cell/mod.rs)。
- `OnceMap` 保留按 key 初始化的成功值，适合有限身份的初始化登记；无 TTL/淘汰保证，不能直接当作任意 key 的有界缓存。见 [OnceMap](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/once/once_map/mod.rs)。
- `Completion` 的 completer 唯一且消耗后发布，观察者只能等待并借用保留值；丢弃未发布 completer 得到 Abandoned。观察者需要独立拥有数据时自行 clone 或存 Arc。见 [Completion](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/completion/mod.rs)。

## 通道、背压与信号

MPMC/SPMC 的多个 receiver 竞争同一队列；每次出队把一个值交给一个接收者，不能保证接收者后续成功处理、持久化或消费一次。见 [MPMC](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/mpmc/mod.rs) 和 [SPMC](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/spmc/mod.rs)。

Asyncband `broadcast::mpmc::bounded` 保留共享 backlog，最慢活跃 receiver 读走或退出才释放相应容量；一个不再读取的订阅者可以堵住全部 producer，应按应用契约退出订阅。无 receiver 时不保留消息，新订阅只看之后的消息。recv 返回即释放该订阅的容量，处理中的值和待发送 payload 不计入容量。这里的无损只描述进程内活跃订阅者的读取契约，持久性与处理确认仍由应用实现。见 [bounded broadcast](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/broadcast/mpmc/bounded/mod.rs)。

Tokio broadcast 容量满时丢弃最旧值，慢 receiver 收到 Lagged；适合允许落后后恢复的契约。Asyncband bounded broadcast 会等待容量，适合需要所有活跃订阅者读取的契约。选型先决定能否丢更新、如何恢复、慢订阅如何退出，再比较性能。见 [Tokio broadcast](https://docs.rs/tokio/latest/tokio/sync/broadcast/index.html)。

watch 保留最新发布状态，慢 receiver 可以跳过中间值。并发发布的先后不等于业务版本大小，业务需要单调版本时由 owner 校验。见 [watch](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/watch/mod.rs)。

ManualResetEvent 表示保持到 reset 的条件；AutoResetEvent 分配信号给一个等待者，无等待者时最多保留一个未分配信号。后者不累计任务数，适合单 worker 先读取受保护的状态、再循环等待；先发布状态再 set，信号不承载或保护状态。见 [event](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/event/mod.rs) 和 [AutoResetEvent](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/event/auto_reset.rs)。

Phaser 用 register 注册每轮到达义务；clone 仅观察。取消已 poll 的 participant.wait 保留当前轮到达，重试继续等同一轮；丢弃 participant 则退出义务，不证明工作成功。要求所有 worker 成功时，失败要记录并 close；阶段观察可能跳过中间轮，不是事件流。见 [Phaser](https://github.com/apache/asyncband/blob/46972ddd371370d831c18301940d16848511816f/asyncband/src/phaser/mod.rs)。

## 可运行 Showcase

[独立 crate](examples/asyncband-coordination/Cargo.toml) 固定 Asyncband 0.7.3，Cargo.lock 保留解析结果；不依赖本地 Asyncband 路径。在仓库根目录运行：

```bash
cargo run --locked --manifest-path plugins/aye/skills/rust-principles/examples/asyncband-coordination/Cargo.toml
cargo test --locked --manifest-path plugins/aye/skills/rust-principles/examples/asyncband-coordination/Cargo.toml
cargo clippy --locked --manifest-path plugins/aye/skills/rust-principles/examples/asyncband-coordination/Cargo.toml --all-targets -- -D warnings
```

- [入口程序](examples/asyncband-coordination/src/main.rs)：同 key 共享类型化失败、后续重新读取、关闭时排空已接收队列，以及慢广播订阅者的背压。
- [组合与 owner](examples/asyncband-coordination/src/lib.rs)：singleflight + Semaphore、关闭与 join 共用预算，以及 Worker 的取消/Drop 兜底。
- [行为契约](examples/asyncband-coordination/tests/contracts.rs)：16 项验证，使用显式 poll、oneshot 和受控时间，不靠 sleep 猜调度。

队列示例只有一个 producer，在请求关闭前丢弃 sender 以停止准入。多个 client 的服务需要明确停止准入机制；此例不实现持久化、外部副作用或处理确认。

## 验证重点

Showcase 已在已发布的 Asyncband 0.7.3 上编译和验证。接入业务时，围绕实际契约检查：

- 重叠同 key、非重叠同 key、不同 key；错误是否共享，取消后是否允许再执行。
- 容量耗尽、取消排队、permit 释放，以及入口等待者/输出的资源预算。
- 关闭请求必达，guard 与 join 分别完成，worker 错误、deadline 和关闭等待自身取消。
- 慢订阅/退出、新订阅的起点，以及最新状态与每次事件的交付差异。

这些验证确认示例组合的可观察行为，不是对库内部实现的完整并发或安全审计。
