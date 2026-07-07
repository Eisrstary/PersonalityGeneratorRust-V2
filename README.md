# PAPS — 人格原子参数系统

**Personality Atomic Parameter System**

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![WASM](https://img.shields.io/badge/wasm-supported-purple.svg)](https://webassembly.org)

> 这里没有"人格类型"。没有"好人"与"坏人"。没有"原型"。
> 这里只有参数。参数在关系中坍缩。参数在时间里漂移。参数在情境中撕裂。
> 每一个具体的人 = 所有参数在特定历史/关系/情境下的唯一一次取值。

---

## 概述

PAPS 是一个**人格参数化引擎**，将人格分解为 196 个独立的原子参数，通过种子码确定性生成完整的人格光谱。它不是人格分类系统——不输出"类型"或"标签"，只输出参数取值及其相互关系。

### 核心理念

- **参数 ≠ 特质**：特质假设跨情境一致性，参数承认情境崩塌
- **激活概率**：不是每个参数对每个人都"激活"——未经历相关情境的参数保持中性值
- **自然生长**：无伦理预设，无价值导向，参数在值域内自由分布
- **同种子 = 同人格**：`ChaCha20Rng` 确定性 PRNG，跨平台一致

---

## 架构

```
paps-core/          Rust 核心库
├── params/         参数定义 (8领域, 196叶子参数)
├── generator/      人格生成器 (种子码/随机/预设)
├── drift/          动态漂移引擎 (年龄/习惯化/训练)
├── phase_change/   相变引擎 (6种事件)
├── reversal/       参数反转标记
├── relation_collapse/  关系坍缩 (8种关系类型)
├── coupling/       耦合推理 (25条规则 + 涌现分析)
├── report/         AI优化输出 (AiProfile)
└── profile.rs      PersonalityProfile

paps-wasm/          WASM 绑定层 (9个导出函数)

ts-test/            TypeScript 测试 + 报告生成
```

---

## 快速开始

### Rust 原生

```rust
use paps_core::*;

let gen = PersonalityGenerator::new();

// 种子码生成
let profile = gen.generate(SeedCode::from(42u64));

// AI优化输出
let ai = profile.to_ai_profile();
println!("{}", ai.to_ai_text());

// 随机生成
let (seed, profile) = gen.generate_random();

// 部分预设
let mut presets = HashMap::new();
presets.insert("E051".to_string(), 0.95);
presets.insert("A009a".to_string(), 0.10);
let profile = gen.generate_with_presets(SeedCode::from(99u64), &presets)?;
```

### WASM / JavaScript

```javascript
import init, { wasm_generate } from './pkg/paps_wasm.js';
await init();

const data = JSON.parse(wasm_generate(42n));
console.log(data.ai_text);   // AI 紧凑文本
console.log(data.traits);    // 推断特征
console.log(data.domains);   // 领域详细参数
```

---

## 参数体系

### 8 个领域

| 领域 | 名称 | 参数数 |
|------|------|--------|
| A | 信息摄入 | 30 |
| B | 情绪生成与调节 | 63 |
| C | 动机与价值 | 61 |
| D | 行为执行 | 17 |
| E | 元认知与自我 | 30 |
| F | 社交信号 | 21 |
| G | 时间性与发展 | 6 |
| H | 身体-环境耦合 | 18 |

### 激活机制

每个参数有 `activation_probability` (0.05 ~ 1.0)：

- **0.95-1.0**：人人都有（基础感知、情绪、身体耦合）
- **0.70-0.90**：大多数人（社交模式、道德情绪）
- **0.40-0.60**：取决于经历（外群体共情、对陌生人内疚）
- **0.05-0.30**：特定/罕见情境

未激活的参数取**中性值**（值域中点），已激活的在值域内随机分布。

---

## 引擎

### 漂移引擎

模拟参数随时间变化：

- **年龄漂移**：每个参数有独立 `age_drift_rate`（每年变化量）
- **习惯化/敏化**：指数模型 `ΔP = amplitude × (1 - e^(-λ × exposure_count))`
- **训练干预**：12 种训练类型（正念/CBT/愤怒管理/共情训练等）

```rust
let engine = DriftEngine::new();
let aged = engine.apply_drift(&profile, &DriftConfig {
    years: 10.0,
    ..Default::default()
})?;
```

### 相变引擎

6 种触发事件导致的参数非连续跳变：

| 事件 | 主要影响 |
|------|---------|
| 背叛 | 信任↓ 威胁放大↑ 背叛检测↑ |
| 丧失 | 意义寻求↑ 使命感受冲击 |
| 羞辱 | 外显自尊↓ 公开羞耻↑ |
| 权力获得 | 支配倾向↑ 注意偏向反转 |
| 被原谅 | 内疚涌现↑ 怨恨衰减 |
| 见证创伤 | 威胁放大↑ 恐惧阈值↓ |

跳变幅度受 `G064`（相变阈值）调控。

### 关系坍缩

同一人格在 8 种关系类型下的参数光谱投射：

亲密 / 陌生 / 敌对 / 权力上位 / 权力下位 / 依赖 / 内群体 / 外群体

每种关系有默认 modifier，如内群体共情 ×1.2，敌对信任 ×0.3。

### 耦合推理

25 条结构化规则 + 8 种涌现模式：

- 规则匹配：如 `A009a↑ + A009b↓ → 选择性共情`
- 涌现分析：多规则交互产生高阶模式（先发制人型暴力、冷酷权力追求者等）
- 矛盾检测：同时匹配冲突规则时标记

---

## 输出格式

### AiProfile (完整数据包)

```json
{
  "seed": 42,
  "ai_text": "SEED 42 | 196参数 147激活(75%) 49休眠\nTRAITS | 使命驱动:71\n...",
  "summary": "种子码42。196个参数中147个激活(75%)...",
  "stats": { "total": 196, "active": 147, "dormant": 49, "activation_rate": 0.75 },
  "traits": [{ "name": "使命驱动", "intensity": 0.71, "evidence": ["E051"] }],
  "domains": [{ "domain": "A", "name": "信息摄入", "params": [...] }],
  "dormant_ids": ["A002b", "A007c", ...]
}
```

### AI 文本 (to_ai_text)

```
SEED 42 | 196参数 147激活(75%) 49休眠
TRAITS | 使命驱动:71
[信息摄入] 猎物/捕食者注意偏向=↑↑ 对内群体的优先级=↑↑ 对敌对者的容忍=↓↓ ...
[情绪生成与调节] 对愤怒的传染=↓↓ 对亲近者面前的羞耻=↓↓ ...
DORMANT(49) A002b A007c A009b ...
ACTIVATION A:83% B:71% C:77% D:64% E:70% F:67% G:60% H:100%
NARRATIVE 种子码42。196个参数中147个激活(75%)。核心特征：使命驱动（强度71%）。高激活领域：A、H。
```

---

## 性能

| 指标 | 数值 |
|------|------|
| 确定性生成 | 52,000 次/秒 (release, 单核) |
| 随机生成 | 56,000 次/秒 |
| 10M 次确定性验证 | 0 失败 |
| WASM 体积 | ~200KB (gzip ~60KB) |

---

## 测试

### Native (Rust) — 8 项理论验证

| # | 测试 | 状态 |
|---|------|------|
| 1 | 确定性 (10M次) | PASS |
| 2 | 激活机制 (休眠=中性值) | PASS |
| 3 | 预设参数 | PASS |
| 4 | 漂移方向 (5/5) | PASS |
| 5 | 相变方向 (4/4) | PASS |
| 6 | 关系坍缩 (5/5) | PASS |
| 7 | 耦合推理 | PASS |
| 8 | 蒙特卡洛 (100K) | PASS |

### WASM (JS) — 6 项理论验证

全部 PASS，包括确定性（ai_text 完全一致）。

---

## 项目结构

```
PersonalityGeneratorRust-V2/
├── Cargo.toml              workspace
├── crates/
│   ├── paps-core/           核心库
│   │   ├── src/
│   │   │   ├── params/      8个领域参数定义
│   │   │   ├── drift/       漂移引擎
│   │   │   ├── phase_change/ 相变引擎
│   │   │   ├── relation_collapse/ 关系坍缩
│   │   │   ├── coupling/    耦合推理
│   │   │   └── report.rs    AI优化输出
│   │   └── examples/
│   │       ├── stress_test.rs   压力测试
│   │       ├── theory_test.rs   理论验证
│   │       └── demo.rs          演示
│   └── paps-wasm/           WASM 绑定
│       └── src/lib.rs
├── ts-test/                 TypeScript 测试
│   ├── theory_test.mjs       WASM 理论验证
│   ├── report_clean.mjs      报告生成器
│   └── personality_report.txt  示例输出
├── 人格碎片生成器.txt        原始规范文档
└── README.md
```

---

## 构建

```bash
# Rust 原生
cargo build --release -p paps-core

# 运行测试
cargo test -p paps-core
cargo run --release --example theory_test

# WASM
wasm-pack build --target web crates/paps-wasm

# TypeScript 测试
cd ts-test && node theory_test.mjs
```

---

## 许可

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

本项目采用 Apache License, Version 2.0。详见 [LICENSE](LICENSE) 文件。

## 作者

[Eisrstary](https://github.com/Eisrstary)
