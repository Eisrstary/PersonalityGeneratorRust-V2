use crate::profile::PersonalityProfile;
use crate::error::PapsResult;

/// 反转引擎 —— 检测并标记参数意义反转
///
/// 反转不是值变化，而是参数功能表现的翻转。
/// 例如：A008(威胁放大)在极度恐惧时→完全麻木(冻结)
pub struct ReversalEngine;

/// 反转条件
#[derive(Debug, Clone)]
pub enum ReversalCondition {
    /// A008 极度恐惧→冻结
    ExtremeFearFreeze,
    /// C026 极度抑郁→意义否定
    SevereDepressionNihilism,
    /// C032 权力导致重大负面后果→权力厌恶
    PowerAversion,
    /// B023 地位上升→反向嫉妒
    StatusReversalEnvy,
    /// E043 过度自我监控→行为瘫痪
    HyperSelfMonitorParalysis,
    /// E054 极端情况→现实否认
    ExtremeRealityDenial,
    /// D041 规则制定者背叛→规则信任崩塌
    RuleMakerBetrayal,
}

impl ReversalEngine {
    pub fn new() -> Self {
        ReversalEngine
    }

    /// 检测并标记反转参数
    ///
    /// 返回更新后的 profile（含反转标记）
    pub fn detect_and_mark(
        &self,
        profile: &PersonalityProfile,
        conditions: &[ReversalCondition],
    ) -> PapsResult<PersonalityProfile> {
        let mut new_profile = profile.clone();
        let mut reversed: Vec<String> = profile.reversed_params.clone();

        for condition in conditions {
            let param_id = match condition {
                ReversalCondition::ExtremeFearFreeze => "A008",
                ReversalCondition::SevereDepressionNihilism => "C026",
                ReversalCondition::PowerAversion => "C032",
                ReversalCondition::StatusReversalEnvy => "B023",
                ReversalCondition::HyperSelfMonitorParalysis => "E043",
                ReversalCondition::ExtremeRealityDenial => "E054",
                ReversalCondition::RuleMakerBetrayal => "D041",
            };
            if !reversed.contains(&param_id.to_string()) {
                reversed.push(param_id.to_string());
            }
        }

        new_profile.reversed_params = reversed;
        Ok(new_profile)
    }

    /// 检查参数是否处于反转状态
    pub fn is_reversed(profile: &PersonalityProfile, param_id: &str) -> bool {
        profile.reversed_params.iter().any(|p| p == param_id)
    }
}

impl Default for ReversalEngine {
    fn default() -> Self {
        Self::new()
    }
}
