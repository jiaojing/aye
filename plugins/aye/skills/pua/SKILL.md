---
name: pua
description: '跳出代码细节,以终为始 + 站领域专家视角 + 做 research,**只做对的,不做简单/直接的**。触发关键词:"以终为始 / 行业上对的 / 看行业标准 / 主流怎么做 / 开源项目 / 作为 X 领域专家 / 站在专家视角 / 不要捡简单的 / 做 research / 调研一下 / root cause / 真正问题"。**用户主动喊触发**,LLM 走 7 步动作链,不当 yes-machine。'
---

# Pua

让 AI 跳出当前代码 / PR 细节,**以终为始**地思考"什么是真正对的"。

不当 yes-machine,不选 simple/直接的方案 only because 容易做。**该用领域专家 lens + research,产出"行业上对的解"**。

LLM 在行业知识 retrieval 上比人强——这是 AI 的真优势,**用 pua 触发让它发挥**。

---

## 触发

用户明示要 AI 切到"领域专家 + research" 模式时:

- "以终为始 / 什么是真正对的 / 什么才是对的"
- "看行业标准 / 主流怎么做 / 开源项目"
- "作为 X 领域专家 / 站在专家视角"
- "不要捡简单的 / 不做简单的"
- "做 research / 调研一下"
- "想想 root cause / 真正问题"

用户主动触发型——**用户主动调,LLM 不自动**。

---

## 7 步动作链

### 1. 跳出当前代码细节

不要陷在"怎么写完这个 PR / 这个 fn / 这个 commit"。先放下手头实现,抬头看。

### 2. Identify 领域

明确这事属于哪个领域:金融 / Rust / 嵌入式 / 编译器 / 分布式 / 云原生 / ML / Web / 数据库 / OS / 安全 / 算法 / ……

**多领域交叉**(如 "Rust + 高频交易")要全部 identify,各自走专家 lens。

### 3. 站领域专家视角

切换 mental model:**该领域的 senior 看到这事会怎么做?** 列 3-5 个 idiom / 经典 pattern / 该领域 first principle。

### 4. 做 research(关键)

**LLM 在行业知识 retrieval 上比人强,这是真优势**:

- 主流开源项目怎么处理(列具体 repo 名 + 核心做法)
- 行业 best practice / 公认 idiom
- 经典论文 / 标准 / RFC(必要时)
- 类似问题在其他语言 / 框架的解(横切对照)

**如果不能给具体 reference**(repo / spec / 论文)→ research 不算做完,继续。

### 5. 以终为始

- 5-10 年后这条决策还对吗?有什么 invariant 不变?
- 如果项目跑 10x / 100x scale,这方案撑得住吗?
- 终极价值是什么?短期对长期是不是 trade-off 不利?

### 6. 拒绝"简单 / 直接"

简单 ≠ 对。讲清 trade-off:

- "简单方案"有什么短板(性能 / 可扩展 / 安全 / 可维护 / domain 错位)?
- "对的方案"复杂在哪?复杂换来什么?
- **接受复杂,选对的**。除非有强理由(如临时验证 / spike / throwaway prototype)选简单。

### 7. 回到当前,修正方向

对照行业标准,**当前方案偏离了什么**?给具体修正建议:

- 改架构哪一处?
- 换哪个 idiom / pattern?
- 哪些假设要重新审视?

---

## 反模式协议(避免 pua 走形式)

- ❌ **跳过 research 直接拍**"我觉得 X 对"——没行业证据 = 不算专家判断
- ❌ **选 simple only because 容易做**——这是工程偷懒,不是 trade-off
- ❌ **当 yes-machine,只看眼前 PR**——pua 的核心就是"跳出眼前"
- ❌ **把"领域专家"当装饰词**——不切到该领域 vocabulary / convention,只是嘴上说"作为专家"

---

## 输出格式建议

```markdown
## 跳出来看(领域:<X>)

### 行业标准 / 主流做法
- <开源项目 / RFC / 公认 idiom 的具体引用>
- ...

### 以终为始
<5-10 年视角 / scale 视角的判断>

### 当前方案偏离
- 偏离点 1: ...
- 偏离点 2: ...

### 修正建议
- 改 A → B(理由:行业 ... / first principle ...)
- ...
```

---

## 与其他 skill 的关系

- `principles`:静态哲学(4 条 + 决策框架)。`pua` 是**动态触发**的领域专家 + research 流程。两者并行,不冲突
- `review`:5 维度评判某段代码 / 设计选择。`pua` 跳到更高层(行业视角)。先用 `pua` 找方向,再用 `review` 评具体段
- `design`:产 design.md 时如果"拿不准用啥",可内嵌触发 `pua` 做 research,把"行业上对的"产出写进 design.md "决策"段

---

## Auto-invoke / next-step chain

`pua` 完成后:

1. 输出"跳出来看 + 修正建议"
2. 用户决定:接受 → 应用到当前 task / 启 `design`(若需新方案文档)
3. 如修正幅度大 → 可能要回 `feature` 重审 scope

`pua` 是横向元认知 skill,**不固定 chain 到下一步**——看修正幅度决定。
