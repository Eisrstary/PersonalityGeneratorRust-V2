use std::collections::HashMap;
use crate::params::ParamRegistry;

/// 经验类型 —— 重复暴露的刺激类型
#[derive(Debug, Clone)]
pub enum ExperienceType {
    /// 反复暴露于他人痛苦（→共情疲劳）
    RepeatedExposureToSuffering,
    /// 长期处于威胁环境（→适应性警觉）
    ChronicThreatExposure,
    /// 长期处于安全环境（→威胁敏感度下降）
    ChronicSafetyExposure,
    /// 反复成功经验（→趋近偏向增加）
    RepeatedSuccess,
    /// 反复失败/创伤（→回避偏向增加）
    RepeatedFailure,
    /// 反复说谎（→欺骗舒适度上升）
    RepeatedDeception,
    /// 反复伤害同一对象（→内疚下降）
    RepeatedHarm,
    /// 长期与外群体接触并建立关系（→外群体共情上升）
    OutgroupContact,
    /// 长期与外群体冲突（→外群体共情下降/施虐上升）
    OutgroupConflict,
    /// 长期孤独（→社会性线索优先级下降或亲和动机变化）
    ChronicLoneliness,
    /// 反复公开羞辱（→羞耻疲劳）
    RepeatedHumiliation,
    /// 长期暴力环境（→攻击基线上升）
    ChronicViolenceExposure,
}

/// 习惯化/敏化配置
#[derive(Debug, Clone)]
pub struct HabituationConfig {
    /// 经验类型
    pub experience_type: ExperienceType,
    /// 暴露次数
    pub exposure_count: f64,
    /// 影响幅度（最终变化的最大值）
    pub amplitude: f64,
    /// 衰减/增长常数 λ（越大越快达到饱和）
    pub lambda: f64,
}

impl Default for HabituationConfig {
    fn default() -> Self {
        HabituationConfig {
            experience_type: ExperienceType::RepeatedExposureToSuffering,
            exposure_count: 10.0,
            amplitude: 0.3,
            lambda: 0.3,
        }
    }
}

/// 计算习惯化/敏化效应
///
/// 使用指数模型：ΔP = amplitude × (1 - e^(-λ × exposure_count))
/// 正 amplitude = 参数上升（敏化），负 amplitude = 参数下降（习惯化）
pub fn compute_habituation(
    registry: &ParamRegistry,
    _current_values: &HashMap<String, f64>,
    config: &HabituationConfig,
) -> HashMap<String, f64> {
    let mut deltas = HashMap::new();

    // 指数衰减/增长因子
    let factor = 1.0 - (-config.lambda * config.exposure_count).exp();
    let delta = config.amplitude * factor;

    if delta.abs() < 1e-10 {
        return deltas;
    }

    // 根据经验类型确定影响的参数
    let affected_params: &[&str] = match config.experience_type {
        ExperienceType::RepeatedExposureToSuffering => {
            &["A009", "A009a", "A009b", "A009c"]
        }
        ExperienceType::ChronicThreatExposure => {
            &["A008", "A008a", "A008b", "A008c"]
        }
        ExperienceType::ChronicSafetyExposure => {
            &["A008", "A008a", "A008b", "A008c"]
        }
        ExperienceType::RepeatedSuccess => {
            &["C025", "C025a", "C025b", "C025c"]
        }
        ExperienceType::RepeatedFailure => {
            &["C025", "C025a", "C025b", "C025c"]
        }
        ExperienceType::RepeatedDeception => {
            &["F059", "F059a", "F059b", "C036"]
        }
        ExperienceType::RepeatedHarm => {
            &["B015", "B015a", "B015b", "B015c", "B015f"]
        }
        ExperienceType::OutgroupContact => {
            &["A009b", "B016b", "B021b"]
        }
        ExperienceType::OutgroupConflict => {
            &["A009b", "B016b", "B021b"]
        }
        ExperienceType::ChronicLoneliness => {
            &["A004", "C033", "C033a", "C033b"]
        }
        ExperienceType::RepeatedHumiliation => {
            &["B017", "B017a", "B017c"]
        }
        ExperienceType::ChronicViolenceExposure => {
            &["D040", "D040a", "D040b", "D040c", "D040d"]
        }
    };

    // 确定漂移方向（正=上升，负=下降）
    let direction: f64 = match config.experience_type {
        ExperienceType::RepeatedExposureToSuffering => -1.0,   // 共情疲劳→下降
        ExperienceType::ChronicThreatExposure => 1.0,           // 适应性警觉→上升
        ExperienceType::ChronicSafetyExposure => -1.0,          // 威胁敏感度下降
        ExperienceType::RepeatedSuccess => 1.0,                 // 趋近偏向增加
        ExperienceType::RepeatedFailure => -1.0,                // 回避偏向增加
        ExperienceType::RepeatedDeception => 1.0,               // 欺骗舒适度上升
        ExperienceType::RepeatedHarm => -1.0,                   // 内疚下降
        ExperienceType::OutgroupContact => 1.0,                 // 外群体共情上升
        ExperienceType::OutgroupConflict => -1.0,               // 外群体共情下降
        ExperienceType::ChronicLoneliness => -1.0,              // 社会性线索优先级下降
        ExperienceType::RepeatedHumiliation => -1.0,            // 羞耻疲劳→下降
        ExperienceType::ChronicViolenceExposure => 1.0,         // 攻击基线上升
    };

    for param_id in affected_params {
        if registry.get(param_id).is_some() {
            deltas.insert(param_id.to_string(), delta * direction);
        }
    }

    deltas
}
