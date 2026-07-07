use serde::{Deserialize, Serialize};
use crate::profile::PersonalityProfile;
use crate::params::ParamRegistry;
use crate::error::PapsResult;

mod rules;
mod emergence;

pub use rules::CouplingRule;
pub use emergence::EmergencePattern;

/// 耦合推理报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingReport {
    /// 匹配的耦合规则列表
    pub matched_rules: Vec<MatchedRule>,
    /// 涌现的行为模式
    pub emergence_patterns: Vec<EmergencePattern>,
    /// 矛盾耦合（同时匹配但方向冲突的规则）
    pub contradictory_couplings: Vec<ContradictoryCoupling>,
}

/// 匹配的耦合规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedRule {
    /// 规则ID
    pub rule_id: String,
    /// 耦合类别
    pub category: String,
    /// 条件描述
    pub condition: String,
    /// 现象描述
    pub phenomenon: String,
    /// 匹配置信度 [0,1]
    pub confidence: f64,
}

/// 矛盾耦合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContradictoryCoupling {
    /// 第一条规则ID
    pub rule_a: String,
    /// 第二条规则ID
    pub rule_b: String,
    /// 矛盾描述
    pub description: String,
}

/// 耦合推理引擎
pub struct CouplingInferenceEngine {
    rules: Vec<CouplingRule>,
}

impl CouplingInferenceEngine {
    pub fn new() -> Self {
        CouplingInferenceEngine {
            rules: rules::build_rules(),
        }
    }

    /// 获取所有耦合规则
    pub fn rules(&self) -> &[CouplingRule] {
        &self.rules
    }

    /// 分析人格档案，返回耦合推理报告
    pub fn analyze(&self, profile: &PersonalityProfile) -> PapsResult<CouplingReport> {
        // 1. 规则匹配
        let matched_rules = self.match_rules(profile);

        // 2. 涌现分析
        let emergence_patterns = emergence::analyze_emergence(&matched_rules);

        // 3. 矛盾检测
        let contradictory_couplings = self.detect_contradictions(&matched_rules);

        Ok(CouplingReport {
            matched_rules,
            emergence_patterns,
            contradictory_couplings,
        })
    }

    /// 规则匹配
    fn match_rules(&self, profile: &PersonalityProfile) -> Vec<MatchedRule> {
        let mut matched = Vec::new();

        for rule in &self.rules {
            let mut all_conditions_met = true;
            let mut total_confidence = 0.0;
            let mut condition_count = 0;

            for cond in &rule.conditions {
                if let Some(&value) = profile.values.get(&cond.param_id) {
                    let met = match cond.direction {
                        rules::ParamDirection::High => value > cond.threshold,
                        rules::ParamDirection::Low => value < cond.threshold,
                        rules::ParamDirection::Neutral => {
                            value >= cond.threshold - 0.1 && value <= cond.threshold + 0.1
                        }
                    };
                    if met {
                        // 置信度 = 偏离阈值的程度
                        let deviation = match cond.direction {
                            rules::ParamDirection::High => (value - cond.threshold).max(0.0),
                            rules::ParamDirection::Low => (cond.threshold - value).max(0.0),
                            rules::ParamDirection::Neutral => {
                                1.0 - (value - cond.threshold).abs().min(1.0)
                            }
                        };
                        total_confidence += deviation;
                        condition_count += 1;
                    } else {
                        all_conditions_met = false;
                        break;
                    }
                } else {
                    all_conditions_met = false;
                    break;
                }
            }

            if all_conditions_met && condition_count > 0 {
                let avg_confidence = total_confidence / condition_count as f64;
                matched.push(MatchedRule {
                    rule_id: rule.id.clone(),
                    category: rule.category.to_string(),
                    condition: rule.condition_desc.clone(),
                    phenomenon: rule.phenomenon.clone(),
                    confidence: (avg_confidence * rule.base_confidence).min(1.0),
                });
            }
        }

        // 按置信度降序排列
        matched.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        matched
    }

    /// 矛盾检测
    fn detect_contradictions(&self, matched: &[MatchedRule]) -> Vec<ContradictoryCoupling> {
        let mut contradictions = Vec::new();

        // 预定义的矛盾规则对
        let contradiction_pairs = vec![
            (("high_guilt_high_aggression", "边攻击边内疚（自我撕裂型）"),
             ("low_guilt_high_aggression", "攻击且无内疚（冷酷型）"),
             "同时匹配高内疚+攻击和低内疚+攻击——参数在不同子维度上取值矛盾"),
            (("high_power_low_guilt", "追求权力且不择手段"),
             ("high_power_high_guilt", "追求权力但自我怀疑"),
             "权力动机与内疚感的关系方向矛盾"),
            (("high_empathy_ingroup_low_outgroup", "选择性共情"),
             ("high_empathy_all", "普遍共情"),
             "内群体共情与外群体共情的模式矛盾"),
        ];

        for ((rule_a_id, _), (rule_b_id, _), desc) in &contradiction_pairs {
            let has_a = matched.iter().any(|r| r.rule_id == *rule_a_id);
            let has_b = matched.iter().any(|r| r.rule_id == *rule_b_id);
            if has_a && has_b {
                contradictions.push(ContradictoryCoupling {
                    rule_a: rule_a_id.to_string(),
                    rule_b: rule_b_id.to_string(),
                    description: desc.to_string(),
                });
            }
        }

        contradictions
    }
}

impl Default for CouplingInferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}
