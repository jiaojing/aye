---
description: Rust 项目专属设计原则 — 类型三分法(Service/Context/Value)、行为跟类型走、生命周期偏好、错误处理风格(thiserror/anyhow)、命名惯例(as_/to_/into_)、弃用替换(LazyLock/parking_lot)、Review checklist、Crate 成熟度模板。仅在 Rust 项目(Cargo.toml / .rs / cargo / rustc / 类型三分法 上下文)触发。新开 Rust 项目可作为 project CLAUDE.md 起点。
---

# Rust Principles

Rust 项目专属设计原则与 idiomatic 选择。**语言无关的代码品味**走 `principles` / `design-review`,本 skill 只讲 Rust 特化的部分。

---

## 触发场景

- 在 Rust 项目里(检测到 `Cargo.toml` / `.rs` 文件 / `cargo` 命令)
- 讨论 Rust 设计选择("用 newtype 还是 type alias?" / "Model 该不该 owned by Engine?")
- 新开 Rust 项目,需要 project CLAUDE.md 的设计原则起点
- Review Rust 代码,关注 idiomatic / 风格层

**不触发**:
- 非 Rust 项目(零干扰)
- 具体编译错误 / 借用检查 / Send/Sync / ownership 问题 → 走 `rust-skills` 插件的 `m0x` 系列(各自有错误码 trigger)
- 语言无关的代码品味问题 → 走 yebai 的 `principles` / `design-review`

---

## 核心设计原则

### 1. 类型三分法

动手前先分类每个类型,决定生命周期和传递方式:

| 分类 | 寿命 | 传递 | 例子 |
|------|------|------|------|
| **Service** | 长生命周期,全局 / 单例 | `Arc<T>` / 引用 | `DataSource`, `Config`, `ConnectionPool` |
| **Context** | Pipeline 中间产物,用完即弃 | owned move | `Model`, `Engine`, `Builder` |
| **Value** | 纯值,Copy/Clone | by value | `f64`, 领域 newtype, `Greeks`, `Email` |

**先判分类,再决传递方式**——不分类就开始写,容易把 Value 写成借用、把 Context 写成 Arc,白白绕生命周期。

### 2. 行为跟类型走

- 方法放 struct 的 `impl`,**不做 `pub` 自由函数**
- `obj.method(...)` > `module::method(&obj, ...)`
- 文件太大按职责拆多个 `impl` 块(Rust 同 crate 内允许多个 impl)

### 3. 生命周期偏好

- **不引入 `'a` 除非必要**,偏好 owned 数据
- `Model owned by Engine` 比引用更自然
- 需要共享时用 `Clone` 或 `Arc`,不用引用

理由:lifetime 是病毒——一处加 `'a`,所有调用链都要传染。owned + `Arc` 看起来"不优雅",但**写起来 / 读起来 / 维护起来都更便宜**。

### 4. 封装优先(Rust 表现层)

语言无关的"封装 > 逻辑方便"在 Rust 表现为:
- 不暴露 `pub` field 给调用方直接访问
- `mesher.axes[0].locations` → `mesher.locations(0)`
- 调用方连续 `.field.field` = 缺方法的信号

### 5. 错误处理风格

- 可恢复错误用 `Result<T, E>`,custom error 用 **`thiserror`**
- 应用层汇总错误用 **`anyhow`**
- 不滥用 `.unwrap()` / `.expect()`,**生产代码只在"逻辑上不可能"的地方用**(且必须 `expect("...")` 带原因)
- 函数能返回 `Result` 就别 `unwrap`

### 6. 不预先抽象

- 两个实现不值得抽 trait,**等第三个出现再考虑**(rule of three)
- 不追求多对多的统一 trait
- 抽 trait 前问 `design-review` 维度 3 的"真共性 vs 偶合"

---

## 命名惯例(Rust-specific,非显而易见)

来自 Rust API Guidelines + 社区共识:

| 前缀 | 语义 | 举例 |
|------|------|------|
| `as_` | 廉价转换,返回引用 | `str::as_bytes()` |
| `to_` | 昂贵转换,复制或计算 | `Path::to_string_lossy()` |
| `into_` | 消费 self,所有权转移 | `String::into_bytes(self)` |

其他硬规则:
- **不用 `get_` 前缀**: `fn name()` not `fn get_name()`
- 迭代器三兄弟: `iter()` / `iter_mut()` / `into_iter()`
- 用 newtype 表达领域语义: `struct Email(String)` 不要裸 `String`(具体取舍见 `design-review` 维度 1)

---

## 弃用 / 替换(优先用新的)

| 别用 | 改用 | 原因 |
|------|------|------|
| `lazy_static!` | `std::sync::OnceLock` | 标准库已支持(Rust 1.70+) |
| `once_cell::Lazy` | `std::sync::LazyLock` | 标准库已支持(Rust 1.80+) |
| `std::sync::Mutex` | `parking_lot::Mutex` | 更快,无毒化 |
| `failure` / `error-chain` | `thiserror` / `anyhow` | 主流选择 |
| `for i in 0..v.len()` | `for (i, x) in v.iter().enumerate()` | 地道 |

---

## Review Checklist(常驻)

Rust 代码写完 / review 前快速扫:

```
[ ] 没有无理由的 .clone()(可能是所有权设计问题)
[ ] 库代码没有 .unwrap()(用 ? 或 expect 带原因)
[ ] 没有 pub field 泄漏不变量
[ ] 没有 String 能用 &str 的地方
[ ] 没有 .unwrap() 在能返回 Result 的函数里
[ ] unsafe 块必须有 // SAFETY: 注释
[ ] 函数长度 < 50 行(超了就想拆)
[ ] 没有 hold lock across .await
```

**和 `design-review` 5 维度的关系**:本 checklist 是 Rust 实现层信号(具体到 keyword 级);`design-review` 是设计判据(抽象到决策级)。两者都跑,不重复。

---

## 可选:Crate 成熟度表(多 crate workspace 模板)

如果项目是 cargo workspace,建议在 project CLAUDE.md 标注各 crate 状态,**防止 AI 对 WIP / placeholder 层做过度 review**:

```markdown
## Crate 成熟度

| Crate | 状态 | 说明 |
|-------|------|------|
| crate-a | ✅ stable | production-ready |
| crate-b | 🟡 WIP | 部分实现 |
| crate-c | 🟢 placeholder | 骨架,等实装 |
```

规则:
- ✅ stable 层做完整 5 维度 review
- 🟡 WIP 层结构性 bug 先修,细节等实装
- 🟢 placeholder 层不深度 review,只关注架构骨架是否合理

---

## 用法:作为新 Rust 项目 CLAUDE.md 起点

新开 Rust 项目时:
1. 创建 project `CLAUDE.md`
2. 引用本 skill:"设计原则见 `yebai/rust-principles` skill (LLM 自动 invoke)"
3. 在 CLAUDE.md 里只写**项目特定**的部分(架构、crate 列表、build command),**不重复本 skill 的通用原则**

这样:
- 通用 Rust 设计原则一处维护,所有项目自动跟进(改 skill 即可)
- project CLAUDE.md 只放项目特殊内容,体积小,信号强
- 新 Rust 项目零拷贝 bootstrap

---

## 与其他 skill 的关系

- **`principles`**(yebai 语言无关版):本 skill 是它的 Rust 特化扩展,sibling 关系。语言无关层在 `principles`,Rust 特化层在本 skill。
- **`design-review`**:5 维度判据是语言无关的;本 skill 的 review checklist 是 Rust 实现层信号。两者并行不冲突——design-review 给设计决策,本 skill 给具体编码 / review 检查。
- **`rust-skills` 插件的 m0x 系列**(独立 plugin):本 skill 讲**风格 / 设计选择**(高层);m0x 讲**具体编译错误 / 语义问题 / 错误码**(低层)。错位互补,不重复。
  - 例:本 skill 说"用 thiserror 写 custom error";`m06-error-handling` 说"如何用 ? 操作符传播 / when to panic vs Result / 具体错误码 E0277"。
- **`flow` / `spec` / `scope-align` / `acceptance` / `commit-review`**(yebai 仪式层):正交关系,本 skill 是知识库,它们是流程。
