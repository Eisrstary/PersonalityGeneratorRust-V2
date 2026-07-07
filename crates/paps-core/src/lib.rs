//! # PAPS Core — 人格原子参数系统核心库
//!
//! 本库实现了"人格碎片生成器.txt"规范的全部功能：
//! - 参数定义（A001-H084，含子参数展开）
//! - 种子码确定性人格生成 + 随机生成 + 部分预设
//! - 动态漂移模拟（年龄/习惯化/训练）
//! - 相变引擎（背叛/丧失/羞辱/权力/原谅/见证）
//! - 参数反转标记
//! - 关系坍缩计算（8种关系类型下的人格光谱投射）
//! - 耦合推理引擎（规则匹配 + 涌现分析）

pub mod params;
pub mod types;
pub mod error;
pub mod seed;
pub mod generator;
pub mod profile;
pub mod report;
pub mod drift;
pub mod phase_change;
pub mod reversal;
pub mod relation_collapse;
pub mod coupling;

// 便捷 re-export
pub use types::{ParameterSpec, ValueRange, Granularity, ParamId, Domain};
pub use error::{PapsError, PapsResult};
pub use seed::SeedCode;
pub use generator::PersonalityGenerator;
pub use profile::PersonalityProfile;
pub use report::{AiProfile, DomainSnap, ParamSnap, TraitSummary, ProfileStats, SpectrumLabel};
pub use drift::DriftEngine;
pub use phase_change::PhaseChangeEngine;
pub use reversal::ReversalEngine;
pub use relation_collapse::{RelationCollapseEngine, RelationType, CollapsedProfile};
pub use coupling::{CouplingInferenceEngine, CouplingReport, EmergencePattern};
pub use params::ParamRegistry;
