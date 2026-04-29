---
description: 5 维度设计判断标准 — 类型抽象 / Builder / Trait / 代码组织命名 / 重构推进。不是 lint 清单,是"先看触发信号,命中再深入"的判据。即将做大破面变更 / review 已写代码或他人 PR / 多方案纠结("该不该 newtype/抽 trait/上 Builder")时调用。
---

# Design Review

按 5 个维度审视代码和设计决策。**不是 lint 清单,是一组"先看触发信号、命中再深入"的判断标准**。每个维度先看"触发信号"——只有命中的才深入审,没命中的略过。

---

## 触发场景

- 用户说 `/review` / "审一下" / "code review" / "审视设计"
- 即将做大破面变更前
- 几个方案纠结时("该不该抽 trait / Builder / newtype?")
- review 已写代码或他人 PR

**不触发场景**:写新代码的过程中(写代码用 TDD / incremental 即可,不用本 skill 自审);纯文档;纯 bug fix(用 debugging skill)。

---

## 维度 1: 类型抽象

### 触发信号

- 看到 `f64` / `Vec<f64>` / `(f64, f64)` / `HashMap<String, f64>` 表达**有名字的业务概念**
- 看到 `String` 表达 ID、分类、状态
- 看到 `bool` 表达多种语义状态(如 `is_valid: bool` 实际有 4 种状态)
- 看到同一表达式在多个地方重复(`order.total + order.tax + order.shipping` 出现 3 次)
- 类型别名 `type X = f64` 假装 newtype

### 审视清单

1. **有名字的概念有没有 type?**
   行内人能叫出名字的东西在代码里**必须有一席之地**。这是 DDD Ubiquitous Language。
   反过来:**有专用术语 + 只有 raw 表达 = 建模缺失信号**。

2. **Newtype 正当性测试**
   不是"有名字就必 newtype"。问三个问题:
   - 能阻止什么误操作?(比如 `OrderId + OrderId` 编译期拒绝、`UserId == ProductId` 编译期拒绝)
   - 能表达什么不变量?(如 `Email::new(s) -> Option<Email>` 强制格式校验)
   - 能携带什么元数据?(如 `RawInput` vs `Sanitized` 的语义差——同样是 String,信任级别不同)

   答不上来不加。

3. **重复计算 = 类型缺失信号**
   同样公式 / 同样查询出现 N 次 → 缺一个对象。把计算结果包成 type,一次构造多处消费。
   *性能提升是副产品,主要论证是**消除纠缠**(decomplect)*。

4. **边界泄漏**
   `OrderId(String)` 出了模块边界 `.0` 解构变回 String → newtype 防护失效。
   原则:**newtype at boundary, raw at core**——边界处强制 typed,核心热循环允许解构。

5. **半做状态比完全没做更危险**
   类型存在但**部分调用方不用**——读者切换心智。一致性优先。

### 通用案例

```rust
// ❌ 反模式:业务概念裸 f64 / String
struct Order {
    customer: String,   // 是邮箱、用户名、还是 ID?
    price: f64,         // 是单价、总价、还是结算价?
    quantity: f64,      // 是数量、重量、还是体积?
    discount: f64,      // 是绝对值还是百分比?
}

// ✅ 改进:让类型携带语义
struct Order {
    customer: CustomerId,
    price: UnitPrice,
    quantity: Quantity,
    discount: DiscountBps,
}

// 进一步:派生量也 typed
let total: Notional = price.into_notional(quantity);  // 编译期保证维度
```

### 反模式合集

- **"为性能不做 type"**——性能是副产品,可读性是主要论证
- **"贯彻到所有 String / f64"**——newtype at boundary, raw at core
- **`type Email = String` alias** 冒充 newtype——alias 不防误用
- **过度运算符重载** 让 newtype 比 raw 还难读

---

## 维度 2: Builder 模式

### 触发信号

- 看到 `pub fn new(...)` 5+ 个参数
- 看到 `Option<T>` 在参数里
- 看到多个 `::new_with_xxx(...)` 静态工厂排列组合
- 调用方反复写"拼参数再 new"的样板

### 审视清单

1. **Builder 解决什么?**
   - **可选 + 必填混合**——可选走 setter 不传不调
   - **参数 holder**——构造期多步私有 helper 共享字段,避免透传
   - **链式语义标签**——比位置参数更可读

   **不是"参数多"**——参数全必填只是命名问题,Builder 改不了根本

2. **必填位置参数 + 可选 setter**
   全部字段都设 setter 是反模式(伪可选——让必填看起来可不传)。必填上来就给:
   ```rust
   HttpClient::builder(host, port)        // 必填位置
       .timeout(Duration::from_secs(30))  // 可选 setter
       .build()
   ```

3. **Setter 语义看名字**
   - `field(x)` / 名词单数 / `set_field(x)` → **替换语义**(后者覆盖前者)
   - `add_field(x)` / `push_field(x)` / `with_field(x)` → **累积语义**(多次调用累加)

   **别混用**。命名 = 替换 但实现 = `extend`,是埋雷。

4. **业务预设 = builder 上的方法**
   常见组合("standard" / "aggressive" / "conservative")作为 builder 的预设方法,**不独立静态工厂**。原因:预设后还要能链式覆盖。
   ```rust
   HttpClient::builder(...).production().timeout(覆盖默认).build()  // 预设 + 微调 同条链
   ```

5. **不要为模式而模式**
   只有 1 个可选字段 → linear-with(`.with_xxx()`)够用,不上 full Builder。
   全必填 → 直接 `new(...)` 位置参数。

### 通用案例

```rust
// ❌ 反模式 1:参数顺序谜
HttpClient::new("localhost", 8080, 30, true, false, None, "v1")

// ❌ 反模式 2:所有字段 setter,必填字段也"伪可选"
HttpClient::builder().host("?").port(?).build()  // 忘传 host 默默用了空字符串

// ❌ 反模式 3:extend setter 无人用累积
fn headers(mut self, hs: Vec<Header>) -> Self {
    self.headers.extend(hs);  // 实际所有调用方都只调一次
    self
}

// ✅ 改进
HttpClient::builder("localhost", 8080)   // 必填位置
    .timeout(Duration::from_secs(30))    // 可选
    .production()                        // 业务预设
    .timeout_override(custom)            // 预设后微调
    .build()
```

### 反模式合集

- **默认值是"伪可选"**(实际所有调用方都传同一个) → 该是必填
- **`extend` setter 但无人调多次** → 是 replace 穿了 append 衣服,改 `=`
- **静态工厂 `::production()`** 切断微调链 → 改 builder 上的预设方法
- **Builder 包了 1 字段** → linear-with 足够,不要重型

---

## 维度 3: Trait 设计

### 触发信号

- 同名同签名方法在多个 struct 里独立 impl(隐含契约)
- trait 方法 5+ 个(God trait 嫌疑)
- 看到 `&dyn Trait` 但 trait 不该 dyn-safe

### 审视清单

1. **真共性 vs 偶合**
   能否 articulate **一句领域理由**说明 trait 必须存在?
   - "X 必须能做到 Y,因为业务上 X 的职责是 Z" → 真共性
   - "这几个 struct 都有这个方法" → 偶合(结构相似的巧合)

   测试:能想象将来有个**符合 X 概念但不实现 Y** 的实现吗?想象不出 → 真共性;能想出 → 慎抽。

2. **小 trait > God trait**(ISP 接口分离原则)
   trait 越大,违反它的未来场景越多。看到 5+ 方法的 trait,问:
   - 调用方真需要全部吗?
   - 能否拆成可组合小 trait?

3. **构造器不入 trait**
   `new` / `from_xxx` 是 inherent,trait 放**行为**。要统一构造接口用 `From` / `TryFrom`。

4. **签名发散就不入 trait**
   方法名相同但参数 / 返回不同(即使 GAT 能搞定返回,参数差异搞不定),强抽是把语义差掩盖在泛型障碍下。

5. **抽 trait 不是永恒承诺**
   关键判断不是"会不会错"而是"**错了改起来贵不贵**"——可逆性优先。
   trait 的 consumer 越少 / impl 越少 → 错了越好改。

### 通用案例

```rust
// ❌ 反模式:偶合抽 trait
// 4 个 struct 都有 fn id() -> String,于是抽 trait HasId
// 但语义上 "Order::id" 是订单号、"User::id" 是用户号、"Invoice::id" 是发票号
// 它们之间没有任何业务关联——这是结构相似的巧合
trait HasId { fn id(&self) -> String; }   // 没领域理由

// ✅ 真共性
// 订阅事件、定时任务、SLA 阶段 都有"关键时间点"的概念
// 因为消费方(scheduler / monitor / alert)需要精确命中这些点而非插值
// 这条领域理由立得住
trait Schedulable {
    /// 业务关键时间点:消费者必须精确处理,不能跳过
    fn key_timestamps(&self) -> &[Timestamp];
}

// ❌ God trait
trait Order {
    fn total(&self) -> Money;
    fn discount(&self) -> Money;
    fn tax(&self) -> Money;
    fn shipping(&self) -> Money;
    fn refundable_amount(&self) -> Money;
    fn cancellation_fee(&self) -> Money;
    fn loyalty_points(&self) -> u64;
    fn invoice_pdf(&self) -> Vec<u8>;
    // ... 还有 5 个
}

// ✅ 拆成可组合小 trait
trait Priced       { fn total(&self) -> Money; }
trait Refundable   { fn refundable(&self) -> Money; fn cancel_fee(&self) -> Money; }
trait LoyaltyEarning { fn points(&self) -> u64; }
trait Invoiceable  { fn invoice_pdf(&self) -> Vec<u8>; }
// 调用方按需 bound: <T: Priced + Refundable>
```

### 反模式合集

- **看到 N 个同签名方法就抽 trait** → 先问领域理由
- **抽完不可逆**(trait 渗透全代码)→ 抽前评估"如果证伪,撤销有多难"
- **trait 名叫 `Common` / `Shared`** → 没业务角色就不是真 trait

---

## 维度 4: 代码组织 / 命名

### 触发信号

- 函数 100+ 行
- 嵌套 3+ 层
- 注释解释 "what does"(这段代码做什么)而不是 "why does"(为什么这样做)
- 复杂表达式 inline 在大函数体里没命名
- 看到 `.field.field.field` 链式深穿

### 审视清单

1. **能命名的步骤就提函数**
   想写注释前先问:能不能提取函数 / 闭包 / let binding,让**名字替代注释**?
   函数命名当注释——好名字是最好的文档。

2. **行为跟类型走**
   方法放 struct 的 impl,**不做 pub 自由函数**。
   `obj.method(...)` > `module::method(&obj, ...)`

3. **消除特殊情况**("好品味")
   if/else 处理边界 case → 重写让边界变成正常流程。
   - 10 行带 if 判断 → 4 行无条件分支
   - 空集合通过哨兵元素退化成正常情况,不需要单独 if

4. **不藏 trick**
   隐式语义(左闭右开靠下段接力补 / 对称性消去等)必须**摊开或写在显眼位置**。
   读者不该靠"读三段代码 + 跨函数推理"才能确认正确性。

5. **不过度设计**
   `impl IntoIterator<Item = T>` vs `Vec<T>` —— 按调用方实际需要选。所有调用方都传 `Vec`,泛型只是给 trait solver 添工。

6. **封装 > 逻辑方便**
   连续 `.field.field.field` = 信号,缺一个方法。问"调用方需要知道多少内部细节?"

### 通用案例

```rust
// ❌ 反模式:隐式 trick + inline 公式
fn process(items: &[Item]) -> Vec<Output> {
    let mut out = Vec::new();
    for i in 0..items.len() - 1 {  // -1 隐含"跳过最后一个"
        let curr = &items[i];
        let next = &items[i + 1];
        // 公式直接 inline,语义靠注释
        // a · curr + (1-a) · next, where a 是 weighting factor
        let mixed = 0.7 * curr.value + 0.3 * next.value;
        out.push(Output { v: mixed });
    }
    out  // 最后一个 item 怎么处理?读者要扫整个函数才知道
}

// ✅ 改进
fn process(items: &[Item]) -> Vec<Output> {
    items.windows(2)                      // 显式:相邻对
        .map(|pair| weighted_blend(pair)) // 函数名替代注释
        .collect()
}

fn weighted_blend(pair: &[Item]) -> Output {
    const SELF_WEIGHT: f64 = 0.7;
    let &[curr, next] = pair else { unreachable!() };
    Output { v: SELF_WEIGHT * curr.value + (1.0 - SELF_WEIGHT) * next.value }
}
```

### 反模式合集

- **过早抽象** —— 三次重复才考虑抽象(rule of three)
- **过度注释 "what"** —— 读代码就能看出做了什么;只在 "why" 非 obvious 时写
- **大函数 + 内部分章注释** —— 章节就是函数边界,提出来
- **跨函数语义接力** —— "这里返回 N-1 个,最后一个由 caller 补" → 包成一个函数

---

## 维度 5: 重构 / 推进决策

### 触发信号

- 即将做大破面变更(改 trait / 改 enum 变体 / 重构 fields)
- 多种方案选择犹豫
- 有 "deprecated"、"migration"、"backwards compat" 议题
- 改一个文件触发了 10+ 文件的连锁

### 审视清单

1. **沉没成本 ≠ 归零,但不是保留理由**
   "已经写了这么多" 不构成 "不能改" 的理由。错的设计要敢改。
   评估:撤销代价 vs 留着的持续债务,取小者。

2. **可逆性优先**
   关键问题不是 "会不会错",是 "**错了改起来贵不贵**"。
   - 改 internal trait/struct → 一般可逆
   - 改 public API → 半可逆
   - 改持久化数据格式 → 几乎不可逆

   不可逆决策需要更高的确定性门槛。

3. **不反向 yes-machine**
   被质疑时不立刻退让一大片。**反向 sycophancy 也是失败**——"什么都赞成"和"什么都退让"是同一种谄媚。
   原始判断如果有依据,被挑战只该针对**质疑指出的具体点**调整,不应整体翻盘。

4. **单点推进,不批量**
   一次一条 backlog item,做完测试绿才进下一条。
   一个 PR 改 5 件事 = review 不能做 = 出 bug 难定位。

5. **先验证不破坏测试再改**
   - 不动代码先 `grep` / LSP find-references 看影响面
   - 看测试覆盖率(这块有没有 regression net)
   - 改前先**让测试可见**(如果没有,先补一个再动)

### 通用案例

**场景**:你写了一个 trait,3 处用,现在发现接口设计不好,想改。

```
不该问:           "我已经投了这么多精力做这个 trait 了..."
该问:             "如果我现在改,撤销代价是什么?(3 处 impl 各改一次 + 1 处 trait 定义)
                    如果我留着,未来 3 个月要付什么持续债务?(读者每次困惑 + 错误使用风险)"

如果撤销 << 持续债务 → 改
如果撤销 >> 持续债务 → 留 + 文档说明
```

### 反模式合集

- **"现在要不要改架构?" → "等以后"** —— 永远不来。不可逆决策延后是债
- **一次 PR 改 N 件事** —— 每件独立 commit
- **被质疑就推倒重来** —— 是 sycophancy 的另一种表现
- **不写测试就改公开 API** —— 静默 break 调用方

---

## 通用 Review Checklist(开场 6 问)

被触发后,先用这 6 问扫一遍,命中了再深入对应维度:

```
[ ] 1. 这段代码有没有用 f64 / Vec<f64> / String / bool 表达**有名字的业务概念**?  → 维度 1
[ ] 2. 这个构造器是否参数过多 / 含 Option / 有多个 ::new_with_xxx?              → 维度 2
[ ] 3. 这里 trait 设计是真共性还是结构相似的偶合?                               → 维度 3
[ ] 4. 有没有重复出现的公式 / 重复查询?                                          → 维度 1 + 4
[ ] 5. 函数命名能传达意图,是不是有藏的隐式语义?                                 → 维度 4
[ ] 6. 这个变更可逆吗?错了改起来贵不贵?                                         → 维度 5
```

---

## 输出格式建议

review 完成后给用户的报告结构:

```markdown
## Review 结论

### 🔴 必须改(阻断 merge)
- [具体位置] 问题 / 修复建议

### 🟡 建议改(不阻断,但应跟进)
- ...

### 🟢 已经做得好的地方
- ...

### 设计判断(如果有)
- 关于 X 的设计选择,我的看法是 Y,理由 Z
- 但你最终决定,我接受任何 informed 决定
```

不要列一大堆纯风格细节(缩进 / 命名一致性等小事)—— 那是 lint 工具的事。本 skill 关注**判断与设计**。

---

## 与其他 skill 的关系

- **`flow`**:地图。本 skill 是"拉式执行"步骤的横向辅助,做大破面变更 / review 时调用。
- **`scope-align`**:scope 对齐时如果涉及破面,可同步触发本 skill 做 5 维度判断。
- **`commit-review`**:改动量大或破面时,commit-review 阶段可触发本 skill 做更深判断。
- **`principles`**:本 skill 的判据建立在 principles 4 条哲学之上(好品味 / 不破坏 consumer / 实用主义 / 简洁)。
- **`agent-skills:code-review-and-quality`**:那个跑五轴 review(correctness / readability / architecture / security / performance)是 review 的**框架**;本 skill 提供**判断标准**,可以被那个 skill 引用。两者并行不冲突。
