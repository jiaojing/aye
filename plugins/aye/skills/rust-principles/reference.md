# Rust 语义与边界参考

按 `SKILL.md` 命中的风险读取相关部分。API 以项目 manifest、lockfile、features、edition 和 MSRV 为准。

## 领域值、状态与能力

- Newtype 的有效性来自受控构造及写入入口；同时检查反序列化、公开字段和可变访问能否绕过校验。外部输入的动态约束仍需要运行时验证。
- enum 表达运行时状态，typestate 限制编译期操作顺序。选择取决于 consumer、状态是否来自存储/协议及转换复杂度。
- trait 描述 consumer 需要的能力。闭合实现集合可以用 enum，开放替换边界可以用 trait；泛型与 dyn 按运行时选择、代码体积和实际性能决定。
- Entity/Value Object 是业务语义，owned/borrowed/Arc 是传递与所有权选择，两者没有固定一一映射。Arc 不自动保证内部不可变或线程安全。
- 不可变快照和纯转换不替代持久化的并发控制。跨上下文写入需要项目明确事务、版本冲突、协调及恢复责任。

示例：[sales.rs](examples/module-boundary/sales.rs) 拥有订单规则及纯转换，[payments/request.rs](examples/module-boundary/payments/request.rs) 拥有支付请求约束；adapter 连接双方的公开契约，PaymentGateway 承载外部能力。

来源：[Arc 的共享与线程安全](https://doc.rust-lang.org/std/sync/struct.Arc.html#thread-safety)、[Rust API Guidelines](https://rust-lang.github.io/api-guidelines/type-safety.html)。

## 操作、执行与组合

统一 dispatch、替换执行实现或组合业务阶段时，读取 [Operation + Service + Layer/Pipeline 参考](operation-dsl.md)。操作协议按领域边界拥有，Service 解释执行，Layer 包装服务，Pipeline 串接阶段；包含 Toasty/Cauldron 分析、Tower readiness 契约和两个可运行示例。

## 文件模块与目录模块

文件和目录是 Rust 模块的组织方式，业务边界取决于规则、契约及演进责任：

- 小而完整的业务单元可以放在一个文件，拥有自己的值、规则和 Error。
- 实现增多时，目录入口声明私有子模块，再用 `pub use` 导出公开契约；consumer 使用目录门面，内部可以按职责移动文件。
- 同一业务单元拆成多个文件，仍可共用一个 Error；目录汇集多个独立能力时，各能力可以分别拥有错误契约，在更高层按语义组合。
- 跨业务模块通过公开事实、请求和能力接口协作；内部 State、字段和实现路径保持受控，不因同处一个 crate 就共享内部模型。

完整 showcase 的结构：

```text
module-boundary/
├── lib.rs                # 导出业务模块；CLI 和测试是外部 consumer
├── sales.rs              # 文件业务模块：订单值、纯规则、sales::Error
├── payments/
│   ├── mod.rs            # 私有子模块 + pub use；支付公开门面
│   ├── error.rs          # payments::Error / Result<T>，构造与调用共用
│   ├── request.rs        # 私有字段、受控构造
│   └── gateway.rs        # PaymentGateway 能力接口
├── adapter.rs            # 销售事实 → 支付请求；传输失败 → 支付错误
├── main.rs               # CLI 分类处理，再添加 anyhow 上下文
└── tests/contracts.rs    # 通过公开门面验证领域与跨模块行为
```

按问题读取：[库入口](examples/module-boundary/lib.rs)、[销售模块](examples/module-boundary/sales.rs)、[支付门面](examples/module-boundary/payments/mod.rs)、[adapter](examples/module-boundary/adapter.rs)、[CLI](examples/module-boundary/main.rs) 和 [契约测试](examples/module-boundary/tests/contracts.rs)。CLI 是独立 binary target，和测试一样通过库的公开 API 消费模块，不内联复制领域定义。

## 错误与恢复责任

- 区分正常缺值、领域拒绝、基础设施失败和程序不变量破坏；按调用方需要的动作保留可匹配的结果。
- 一个独立业务模块默认拥有一个 `Error` 枚举；构造、校验与调用可以共用，签名返回 `Result<T, ModuleError>`。错误粒度随业务边界，不按文件或每个 helper 新建包装枚举。
- 模块内定义 `pub type Result<T> = std::result::Result<T, Error>`，目录门面同时导出 Error 和 Result。内部签名用 `Result<T>`，跨模块用 `payments::Result<T>` 表明错误归属；别名是同一个标准 Result，不引入新包装或自动错误转换。`TryFrom` 等涉及关联错误的 trait 实现可显式使用 `std::result::Result<Self, Self::Error>`。
- adapter 将外部失败映射到公开契约，同时保留诊断原因；领域核心不需要依赖 HTTP status 或某个数据库错误类型。
- 默认用 `thiserror` 实现模块错误，调用方分类处理后，在只需要传播与报告的编排使用 `anyhow::Context`。应用内部业务模块也发布类型化错误；已有等价错误模式可沿用。
- 上下文说明正在执行的操作、对象标识及相关阶段，例如“确认订单 42 时读取库存失败”。调用方需要匹配的标识、字段或进度放入类型化字段；用于定位的诊断信息可用 `with_context` 添加。在引入新业务信息的边界补充，不按每个 helper 或 `?` 新建包装错误或重复上下文。
- 用 `#[source]` 保留原因，多个来源映射到相同语义时按操作显式转换，不为省 `map_err` 强行使用 `#[from]`。若 source 链已携带底层错误，外层 Display 只描述本层语义，避免重复渲染；不要格式化原始错误后仅保存字符串。
- `anyhow` 添加上下文后仍能 downcast 到模块错误；完整底层原因可沿 `chain()` 查看。常规业务决策在类型化边界完成，不依赖错误文案匹配。
- 需要一次返回多项校验或并发失败时，用带字段或项目标识的类型化错误集合表达；允许短路的操作仍可返回首个错误。`source()` 描述单条原因链，不替代调用方需要的多项失败契约。
- 批处理可能部分完成时，按实际可观察状态记录成功、失败、未执行或结果未知的项目。部分成功属于正常业务结果时用 `BatchOutcome` 等结果类型表达；整个操作失败时，由模块错误携带相关进度。超时或取消后不能把结果未知的项目直接标记为未执行。
- 重试需要判断失败是否可恢复、操作是否幂等、总 deadline/次数预算及取消；临时故障不单独构成安全重试条件。transport timeout 可能意味着结果未知；有外部写入时先按契约查询、去重或协调，不能直接认定未执行。
- 一个错误可以同时有 audience、recoverability 和 source；它们是不同维度，避免把“内部/临时/用户可见”强塞进互斥分类。

示例：[payments/error.rs](examples/module-boundary/payments/error.rs) 定义唯一支付业务 Error；[adapter](examples/module-boundary/adapter.rs) 保留传输失败和 I/O cause，[CLI](examples/module-boundary/main.rs) 先处理拒绝，对结果未知提示查询，再为需要报告的失败添加上下文。传输阶段由模拟底层明确提供，示例不实现真实支付、幂等或对账。

来源：[thiserror](https://docs.rs/thiserror/latest/thiserror/)、[anyhow Context](https://docs.rs/anyhow/latest/anyhow/trait.Context.html)、[标准库 Error/source](https://doc.rust-lang.org/std/error/trait.Error.html)。补充阅读：[Stop Forwarding Errors, Start Designing Them](https://fast.github.io/blog/stop-forwarding-errors-start-designing-them/)；借鉴操作上下文与多项失败的设计，业务错误归属和重试责任仍按以上判据处理。

## 异步、取消与关闭

- Send/Sync 检查实际线程与 executor 契约、任务捕获值，以及跨 await 留存的值。单线程执行和跨线程执行可以采用不同的所有权设计。
- `select!`/timeout 会放弃未完成的 future；检查其中的部分进度、资源和外部副作用。`recv` 等方法有取消安全保证，`read_exact`/`write_all` 等不能照搬同一假设。
- 丢弃 Tokio JoinHandle 会 detach 任务。owner 明确持有、等待或终止策略；abort 是请求，完成清理还需观察任务结束。已启动的 spawn_blocking 工作不能靠 abort 停止。
- 同步锁避免跨 await；异步 Mutex 可以按 I/O 协作需要跨 await 持有，检查锁范围、竞争与取消后的状态。
- mailbox 有界并不等于全部资源有界；还需考虑同时等待发送的请求、排队/运行中的工作、输出缓存及 payload 大小。
- 关闭明确停止接收、已接收工作排空或放弃、在途 I/O 和等待预算。关闭 mpsc receiver 后排空到 None；已获得的 permit 仍可能发送，要按实际 API 处理。
- `Drop` 只能做同步清理或发出终止请求。需要等待/可失败的关闭提供显式 async 方法，并定义该方法自身被取消时的清理责任。
- timeout 依赖执行器获得调度机会；不让 CPU 长循环或阻塞调用占住 worker，再宣称具备硬截止时间。

示例：[service-lifecycle.rs](examples/service-lifecycle.rs)。正常关闭排空已接受工作；deadline 到达后放弃剩余工作并 abort/等待，失败与完成状态向 owner 报告。调用者必须 await shutdown 才能获得排空结果。

需要运行时独立协调或通道选型时，读取 [Asyncband 参考](asyncband.md)；其独立 showcase 覆盖同 key 合并、后端并发限制、关闭/取消与广播背压，运行命令在该参考中。

来源：[select 取消安全](https://docs.rs/tokio/latest/tokio/macro.select.html#cancellation-safety)、[JoinHandle](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html)、[Mutex](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)、[Receiver::close](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html#method.close)、[timeout](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html)。

## Unsafe 与 FFI

先判断 safe API 是否足够。新增 unsafe abstraction 时，列出所有安全入口、unsafe 操作、需要成立的不变量及证明来源：

- 指针的 allocation/provenance、范围、对齐、初始化和存活时间；非空本身不足以证明有效。
- 引用的别名与访问权限、类型有效值和并发同步；手写 Send/Sync 必须证明整个可达 API 的线程安全。
- 错误、panic 和部分构造时的状态、资源释放及 Drop；安全调用方不能使 wrapper 违反内部约束。
- FFI 的 ABI/layout、缓冲区长度与编码、初始化保证、分配/释放配对、回调生命周期和 unwind 约定。

`// SAFETY:` 解释调用点为何满足条件；public unsafe API 的 `# Safety` 说明调用方必须保证什么。不要靠一条 debug_assert 或存在注释就关闭审查。

按风险使用边界测试、Miri 或其他适合的工具；工具覆盖不到的 FFI/平台假设如实记录，测试通过不等于普遍 soundness 证明。

来源：[未定义行为](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)、[from_raw_parts 的前置条件](https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html#safety)、[Miri 的能力与限制](https://github.com/rust-lang/miri#readme)。

## 性能证据

先确定 latency/throughput/内存等目标与代表性负载，区分算法、I/O、分配、锁竞争和调度瓶颈。用匹配交付环境的优化构建、profile 和 benchmark 比较收益，并确认语义及资源预算仍成立。

泛型、inline、零拷贝、RwLock 或某个容器名称不足以证明更快；引用也可能延长大对象的存活时间。记录实际测量及复杂度代价，不设固定优化优先级或收益倍数。

## 运行与验证

在仓库根目录执行：

```bash
cargo run --locked --manifest-path plugins/aye/skills/rust-principles/examples/Cargo.toml --bin module-boundary
cargo run --locked --manifest-path plugins/aye/skills/rust-principles/examples/Cargo.toml --bin service-lifecycle
cargo run --locked --manifest-path plugins/aye/skills/rust-principles/examples/Cargo.toml --bin operation-dsl
cargo run --locked --manifest-path plugins/aye/skills/rust-principles/examples/Cargo.toml --bin service-pipeline
cargo test --locked --manifest-path plugins/aye/skills/rust-principles/examples/Cargo.toml
```

模块 showcase 依赖 thiserror + anyhow，契约测试作为外部 consumer 验证受控构造、纯转换和跨模块映射；CLI 测试验证调用分类、cause 保留和结果未知不盲目重试。生命周期示例使用 Tokio 的 oneshot、Notify 和虚拟时间验证排空、背压、超时、worker 失败及 shutdown 被取消后的清理。操作协议与 Tower pipeline 的运行和行为契约见对应参考。

## 借鉴来源

判据组织参考 [actionbook/rust-skills 的 5c40d3a 快照](https://github.com/actionbook/rust-skills/tree/5c40d3ad785193231b7d0dbfb8e1eb447e5edd94)：m01/m05/m06/m07/m12/m13、unsafe-checker 与 m10。此处示例为 aye 独立实现；具体 Rust/Tokio 语义以以上一手文档和可执行验证为准。
