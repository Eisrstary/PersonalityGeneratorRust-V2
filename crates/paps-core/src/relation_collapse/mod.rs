use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::profile::PersonalityProfile;
use crate::params::ParamRegistry;
use crate::error::PapsResult;

mod relation_types;
pub use relation_types::RelationType;

/// 关系坍缩后的参数档案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapsedProfile {
    /// 原始种子码
    pub seed: u64,
    /// 基准参数值（无关系上下文）
    pub baseline: HashMap<String, f64>,
    /// 各关系类型下的参数取值：关系类型 → (参数ID → 值)
    pub relations: HashMap<String, HashMap<String, f64>>,
}

/// 关系坍缩引擎
///
/// 核心理念：人格参数在不同关系中取不同值——
/// 这是人格光谱在不同关系维度的投射。
/// 不是"用户与周围人的关系网"，而是"这个人格在面对不同类型对象时的参数表现"。
pub struct RelationCollapseEngine {
    registry: &'static ParamRegistry,
}

impl RelationCollapseEngine {
    pub fn new() -> Self {
        RelationCollapseEngine {
            registry: ParamRegistry::global(),
        }
    }

    /// 对人格档案进行关系坍缩计算
    ///
    /// 返回所有关系类型下的参数取值矩阵
    pub fn collapse(&self, profile: &PersonalityProfile) -> PapsResult<CollapsedProfile> {
        let relation_types = RelationType::all();
        let mut relations: HashMap<String, HashMap<String, f64>> = HashMap::new();

        for rel_type in &relation_types {
            let modifier_map = rel_type.default_modifiers();
            let mut rel_values: HashMap<String, f64> = HashMap::new();

            for (param_id, &base_value) in &profile.values {
                let modifier = modifier_map.get(param_id).copied().unwrap_or(1.0);
                if let Some(spec) = self.registry.get(param_id) {
                    let collapsed = (base_value * modifier)
                        .clamp(spec.value_range.min(), spec.value_range.max());
                    rel_values.insert(param_id.clone(), collapsed);
                } else {
                    rel_values.insert(param_id.clone(), base_value * modifier);
                }
            }

            relations.insert(rel_type.id().to_string(), rel_values);
        }

        Ok(CollapsedProfile {
            seed: profile.seed,
            baseline: profile.values.clone(),
            relations,
        })
    }
}

impl Default for RelationCollapseEngine {
    fn default() -> Self {
        Self::new()
    }
}
