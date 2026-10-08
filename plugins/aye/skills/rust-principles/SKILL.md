---
name: rust-principles
description: 'Rust 的领域类型、所有权、错误恢复、异步资源与 unsafe 判据。用于 Rust 设计、实现或审查；语义细节和可运行示例按需读取。'
---

# Rust Principles

聚焦会影响语义与正确性的 Rust 特化。通用设计见 `principles`，审查与流程分别见 `review`、`flow`。

## 运行时角色与所有权

| 角色 | 默认考虑 | 例子 |
|---|---|---|
| 资源 Service | 生命周期 owner 构造；借用，确有共享所有权时 Arc | client、pool、长期状态拥有者 |
| Context | pipeline 中间产物，通常 owned move | request/model/计算上下文 |
| Value | 值传递或局部借用 | newtype、money、timestamp |

这些是寿命与传递的观察方式；运行时 Context 不等于 DDD bounded context。执行协议中的 Service 可以无状态，不一定是资源 owner；`Arc` 表达共享所有权，不自动表达全局 singleton。

- 应用数据优先考虑 owned；库 API、零拷贝路径和临时视图按真实需求借用，避免不必要的 lifetime 扩散。
- Clone/Arc 有明确复制或共享语义；不为了绕过编译错误机械添加。
- 纯领域规则与可变状态 owner 分开说明；有状态 Service 需要明确访问与并发策略。

修复编译错误时先核对机制与实际数据流：E0382 检查后续需要借用、转移、复制还是共享；lifetime 错误检查 owner 与引用的存活关系；Send/Sync 错误检查真实跨线程需求及跨 await 保留的值。普通局部修复足够时直接修复，反复冲突或边界错位才回看设计。

## 类型、行为与封装

- `enum` / `struct` 表达状态；trait 用于真实能力需求，解释器是可选组织方式。
- inherent 方法、trait/解释器和领域模块内的函数都可承载行为；检查规则归属，不强制所有操作归于类型。
- 构造和状态转换保护不变量；限制会绕过约束的字段或写入入口，深穿字段只是调查信号。
- 动态状态通常由 enum 与受控转换表达；调用顺序确需编译期限制时考虑 typestate，不从状态数量机械推导泛型结构。
- `Default` 表达真实有效状态，不用零 ID、空凭据或未连接 client 伪造完成初始化。
- 框架输入在 adapter 转成领域输入；依赖按明确 owner 构造与接线。
- 小业务单元可用文件模块；目录模块通过私有子模块和 `pub use` 发布公开契约。文件拆分不自动产生新业务边界，consumer 不依赖内部文件路径。

## 操作、执行与组合

- 需要统一 dispatch、替换执行实现或组合处理阶段时，鼓励 **Operation/Request + Service 执行单元 + Layer/Pipeline 组合**。操作数据表达意图，Service 解释执行；闭合操作集合用 enum，具体请求也可直接用 struct，按业务边界拥有各自的协议。
- Layer 包装或构造 Service，Pipeline 串接具有明确输入输出的阶段；业务准入、风险校验、提交也能参与组合。纯规则可保留为值与函数，由执行单元调用；规定必要顺序及状态 owner，不把领域不变量变成可随意重排的配置。
- Service 可以使用领域 trait 或 Tower 协议，依实际调用和组合需求选择。Tower 的 readiness 必须传递给随后调用的同一个实例；业务拒绝、执行失败与结果未知按调用契约区分。简单函数或类型化方法已足够时，直接使用。

## 错误与资源

- 独立业务模块默认拥有自己的 `Error` 枚举和 `Result<T>` 别名，固定本模块错误类型；跨模块使用 `payments::Result<T>` 等限定名。同一模块内复用，不按文件拆分错误类型。默认用 `thiserror` 实现，已有等价模式可沿用。
- 调用方先按模块错误做业务决策；只需传播与报告的编排使用 `anyhow` 添加上下文，保留原始 cause。已证明不可能失败的位置才使用 `expect` 并说明依据。
- 错误粒度取决于 consumer 的处理动作；重试由了解幂等性、预算与结果是否确定的边界决定，不把 transport timeout 自动当作业务未执行。
- 在引入新业务信息的边界补充操作、对象标识等上下文，避免每个 helper 重复包装；在合适边界记录，避免重复 log 后原样传播。
- 后台任务/线程有可观察的生命周期、取消或终止路径与回收策略；按用途决定是否持有并等待 JoinHandle。
- 外部 I/O 与 shutdown 有合适的 deadline；不要把已取消的工作 token 当作清理预算。
- 无界输入按资源预算限制并发；优先现有结构化原语，定制调度需要具体收益。
- 并发设计先明确异步协调、同步共享或 CPU 并行的需求。异步协调选型先检查 [Asyncband](asyncband.md) 是否有契约合适的原语，再结合项目现有库、运行时和版本约束选择；局部修复沿用有效实现，不自动触发依赖迁移。
- 同步/阻塞锁的 guard 避免跨 `.await`，以免阻塞或死锁。异步锁可按资源需求跨 await 持有，但应检查持锁范围、竞争和取消语义。
- 异步测试用显式同步或受控时间，不用 sleep 猜调度。

## 按需深入

遇到错误映射、多项失败与部分进度、取消/关闭、unsafe/FFI 或性能问题时，读取 [reference.md](reference.md) 对应部分。性能调整先说明目标、代表性负载、瓶颈证据与收益；unsafe 检查实际安全不变量和调用方责任，注释或测试通过不能单独证明 soundness。

- [模块与错误完整示例](examples/module-boundary/lib.rs)：销售文件模块、支付目录门面、私有子模块、公开契约、adapter 映射，以及 CLI 的 thiserror + anyhow 分工；文件导航见 reference。
- [Operation + Service + Layer/Pipeline 参考](operation-dsl.md)：统一执行入口或阶段组合时读取；包含 Toasty 操作协议、Cauldron 业务 pipeline、Tower 契约，以及可运行的键值协议与批次准入/提交/执行示例。
- [Service 生命周期示例](examples/service-lifecycle.rs)：有界队列、独立 client、关闭排空、超时 abort/等待和取消 shutdown 时的 owner 清理。
- [Asyncband 并发参考](asyncband.md)：需要运行时独立协调、同 key 合并、动态完成或通道选型时读取；包含 [可运行 showcase](examples/asyncband-coordination/src/main.rs)、任务 owner 和行为测试，不作为默认依赖。

示例独立于业务项目，运行与行为验证命令见对应参考；按当前问题读取，不把示例形式当作统一模板。

## Public API

- trait/interface 从 consumer 的最小能力发现，不按实现数量设门槛。
- public trait 新增必需方法、enum case、错误或所有权签名变化，检查外部 consumer/implementor。
- MSRV、配置和持久化格式也是契约；重要变化提供约定和迁移，适用时使用 semver 等机械检查。
- 使用项目版本支持的标准库和依赖 API。锁与执行器等选择按实际语义、性能和现有模式决定；错误处理默认组合见上文。

关键检查聚焦不变量、unsafe 的安全依据、任务清理、锁与 await、公开兼容和实际验证缺口。
