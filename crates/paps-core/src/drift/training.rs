use std::collections::HashMap;
use crate::params::ParamRegistry;

/// 训练类型
#[derive(Debug, Clone)]
pub enum TrainingType {
    /// 正念/冥想训练 → 提升A003, E043, B014, 降低E044
    Mindfulness,
    /// CBT认知行为训练 → 降低B013a, E044
    CBT,
    /// 愤怒管理训练 → 降低B019
    AngerManagement,
    /// 情绪词汇训练 → 提升B012, B020
    EmotionVocabulary,
    /// 感恩练习/积极心理学 → 提升B018
    Gratitude,
    /// 心理治疗(长期) → 提升E045, 调节E050
    Psychotherapy,
    /// 辩证思维训练 → 提升E053
    DialecticalThinking,
    /// 认知训练 → 提升E054
    CognitiveTraining,
    /// 社交技能训练 → 提升F060, 降低F058
    SocialSkills,
    /// 共情训练 → 提升A009b, B021b
    EmpathyTraining,
    /// 冲动控制训练 → 提升C030
    ImpulseControl,
}

/// 训练配置
#[derive(Debug, Clone)]
pub struct TrainingConfig {
    /// 训练类型
    pub training_type: TrainingType,
    /// 训练强度 (0.0-1.0)
    pub intensity: f64,
    /// 训练持续时间（年）
    pub duration_years: f64,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        TrainingConfig {
            training_type: TrainingType::Mindfulness,
            intensity: 0.5,
            duration_years: 1.0,
        }
    }
}

/// 计算训练干预效应
///
/// 训练效应 = intensity × duration_years × per_year_effect
/// 效应在参数值域内被 clamp
pub fn compute_training_effect(
    registry: &ParamRegistry,
    _current_values: &HashMap<String, f64>,
    config: &TrainingConfig,
) -> HashMap<String, f64> {
    let mut deltas = HashMap::new();
    let effect_magnitude = config.intensity * config.duration_years;

    if effect_magnitude < 1e-10 {
        return deltas;
    }

    // 根据训练类型确定影响的参数和方向
    let affected: Vec<(&str, f64)> = match config.training_type {
        TrainingType::Mindfulness => vec![
            ("A003", 0.05),
            ("E043", 0.05),
            ("B014", 0.03),
            ("B014a", 0.02),
            ("E044", -0.04),
            ("E044a", -0.03),
            ("E044b", -0.03),
        ],
        TrainingType::CBT => vec![
            ("B013a", -0.06),
            ("E044", -0.05),
            ("E044a", -0.04),
            ("E044b", -0.04),
            ("B014a", 0.03),
        ],
        TrainingType::AngerManagement => vec![
            ("B019", -0.05),
            ("B019a", -0.04),
            ("B019b", -0.03),
            ("B019c", -0.04),
            ("B019d", -0.05),
            ("C030a", 0.03),
        ],
        TrainingType::EmotionVocabulary => vec![
            ("B012", 0.04),
            ("B012a", 0.02),
            ("B012b", 0.02),
            ("B012c", 0.03),
            ("B012d", 0.02),
            ("B020", -0.04),
        ],
        TrainingType::Gratitude => vec![
            ("B018", 0.04),
            ("B013b", 0.02),
            ("B011_joy", -0.02),
        ],
        TrainingType::Psychotherapy => vec![
            ("E045", 0.03),
            ("E050", -0.02),
            ("E050a", -0.02),
            ("E050b", -0.02),
            ("E044", -0.03),
        ],
        TrainingType::DialecticalThinking => vec![
            ("E053", 0.04),
            ("E053a", 0.03),
            ("E053b", 0.03),
            ("E053c", 0.02),
        ],
        TrainingType::CognitiveTraining => vec![
            ("E054", 0.04),
            ("D042", 0.02),
        ],
        TrainingType::SocialSkills => vec![
            ("F060", 0.03),
            ("F060a", 0.02),
            ("F058", -0.03),
            ("F058a", -0.02),
            ("F058b", -0.02),
        ],
        TrainingType::EmpathyTraining => vec![
            ("A009b", 0.04),
            ("B021b", 0.03),
            ("B016b", -0.03),
            ("A009", 0.02),
        ],
        TrainingType::ImpulseControl => vec![
            ("C030", 0.05),
            ("C030a", 0.04),
            ("C030b", 0.03),
            ("C030c", 0.04),
            ("C030d", 0.03),
        ],
    };

    for (param_id, per_year_effect) in &affected {
        if registry.get(param_id).is_some() {
            let delta = per_year_effect * effect_magnitude;
            deltas.insert(param_id.to_string(), delta);
        }
    }

    deltas
}
