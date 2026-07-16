---
name: review
description: '5 维度设计判据(Phase 2 横向辅助，可调 N 次)。触发关键词:"该不该 newtype / 抽 trait / 上 Builder / review 设计 / 审一下 / 审视 / 这个抽象合理吗 / 命名是否好 / 大破面变更 / 重构合理吗"。按类型抽象、Builder、能力接口、组织边界、重构兼容五维审视代码或设计；大改动 commit 前由 commit-gate 调用。'
---

# Design Review

这是设计判据，不是 lint 或提交闸门。先扫触发信号，只深入命中的维度；只报告会影响正确性、边界、consumer 或长期维护的问题。

## 开场扫描

```text
[ ] raw primitive 是否在表达有名字、有不变量的领域概念？
[ ] 构造调用是否出现参数谜题、伪可选或重复拼装？
[ ] interface/trait 是 consumer 真需求，还是实现结构偶合？
[ ] 行为和模块是否有清楚的领域归属？
[ ] 框架对象、全局状态或内部字段是否泄漏到核心/consumer？
[ ] 后台任务是否有 owner、取消、timeout 和 join/shutdown？
[ ] 变更会破坏编译、行为、配置、数据或最低工具链吗？
[ ] 判断错了，撤销代价有多高？
```

## 维度 1：类型抽象

### 触发信号

- `String` 表达 ID、状态、受信/未受信输入；
- 数字、tuple、map 表达有名字的业务量；
- 多个 nullable/bool 拼出互斥状态；
- 同一公式、校验或查询在多处重复；
- type alias 假装 newtype。

### 判断

Newtype/ADT 必须至少带来一项收益：

1. 阻止误传或非法组合；
2. 在构造时维护不变量；
3. 表达信任级别、单位或状态；
4. 让 compiler 穷举所有 case。

答不上来就不要加。类型已经存在时应在边界一致使用，避免一半 typed、一半重新解构成 raw。

警惕两端：所有 primitive 都包装会增加噪音；完全用 primitive 则把领域规则推给记忆和注释。

## 维度 2：Builder / 构造

### 触发信号

- 构造器参数多、含多个同类型值或 optional；
- `new_with_xxx` 工厂排列组合；
- 调用方重复拼装同一配置；
- Builder 把必填字段伪装成可选 setter。

### 判断

- 必填项在构造入口明确提供；可选项才走 setter/default。
- 参数全部必填时，优先命名参数对象或直接构造，不为模式而 Builder。
- setter 名称必须匹配 replace/append 语义。
- 常见业务预设可放在 Builder 上，并允许调用方继续微调。
- 一个 optional 字段通常用 `with_xxx` 即可。

默认值必须是语义上有效且安全的状态；如果所有 consumer 都覆盖某个默认，它其实是必填项。

## 维度 3：能力接口 / Trait

### 触发信号

- 多个类型有相同签名但没有共同语义；
- trait/interface 方法过多；
- 只有 implementor 在使用接口，consumer 并不需要替换；
- 泛型或动态分发只增加 solver/cast 成本；
- 修改 public interface 的方法集合。

### 判断

1. **Consumer 驱动**：接口由调用方需要的最小能力定义。
2. **领域理由**：能用一句业务语言说明为什么这些实现属于同一能力。
3. **小接口**：consumer 不需要的方法拆开组合。
4. **构造与行为分开**：构造通常不是领域能力；使用语言标准转换协议。
5. **可逆性**：实现数量不是硬门槛；真实 boundary 可支持单实现接口，偶合的三个实现也不值得抽象。

公共 trait/interface 一旦允许外部实现，新增必需方法通常是 breaking change；设计和 review 时必须查 consumer。

## 维度 4：组织、命名与边界

### 触发信号

- `service` / `repository` / `utils` / `common` 等技术层或垃圾桶模块；
- framework request/config/context 进入领域核心；
- `main`/composition root 之外的可变全局 singleton；
- adapter 同时解析协议并做业务决策；
- 大函数、三层以上嵌套、what 注释、隐藏 trick；
- consumer 连续访问 `.field.field.field`。

### 判断

- 按领域职责组织，composition root 只负责接线。
- 框架停在 adapter；进入核心前转换成领域类型和强类型配置。
- 行为有命名归属：方法、trait/interface 或解释器；避免无主的 public 自由函数。
- 能命名的步骤提取为函数/绑定，让名字代替 what 注释。
- 展开隐藏语义；读者不应跨多个函数猜边界 case 如何补齐。
- 深穿字段意味着封装缺口；让类型提供稳定能力。
- 使用框架 API 前从 manifest/lockfile 确认版本，禁止混用 API 代际。

并发/资源边界额外检查：谁启动、取消、等待，timeout 在哪里，错误由谁处理。不能回答就阻断合并。

## 维度 5：重构与兼容

### 触发信号

- 改 public API、enum case、trait、配置 key、错误分类或序列化字段；
- 提高 Rust MSRV、JVM/Kotlin/Gradle 等最低版本；
- 收紧曾经合法的输入；
- 一个修改引发大量文件连锁；
- 涉及 deprecated、migration 或 backwards compatibility。

### 判断

1. 比较“现在撤销”与“继续背债”的成本，不拿沉没成本当保留理由。
2. internal 结构通常较可逆；public API 半可逆；持久化格式接近不可逆。
3. 先查定义、所有调用点、外部 consumer 和 regression net，再改。
4. 优先加法演进；deprecation 必须给 replacement、migration 和保留周期。
5. 使用机械兼容检查，不靠肉眼判断。
6. 一个 PR 只承载一个可独立 review 的交付；不要把无关清理混进来。

被质疑时只调整证据支持的部分；不因沉没成本死守，也不因用户质疑无条件推倒全部判断。

## 输出

```markdown
## Review 结论

### 必须改
- `<位置>`：<问题> —— <实际影响和最小修复>

### 建议改
- `<位置>`：<改进及 trade-off>

### 已确认无问题
- <仅列用户可能关心的关键边界，不做礼貌性表扬清单>

### 设计判断
- <选择、依据、剩余风险>
```

纯格式和 lint 问题交给工具。报告必须校准：真实阻断才放“必须改”，不为显得严格制造问题。

review 完成后回到原流程；大改动在 `commit-gate` 展示 diff 前必须完成本审视。
