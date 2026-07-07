use std::collections::HashMap;
use crate::profile::PersonalityProfile;
use crate::params::ParamRegistry;
use crate::error::PapsResult;

mod age_drift;
mod habituation;
mod training;

pub use age_drift::AgeDriftConfig;
pub use habituation::{HabituationConfig, ExperienceType};
pub use training::{TrainingConfig, TrainingType};

/// 漂移引擎 —— 模拟参数随时间的变化
pub struct DriftEngine {
    registry: &'static ParamRegistry,
}

/// 漂移配置
#[derive(Debug, Clone)]
pub struct DriftConfig {
    /// 模拟的年数
    pub years: f64,
    /// 年龄漂移配置
    pub age_drift: AgeDriftConfig,
    /// 习惯化/敏化配置
    pub habituation: Option<HabituationConfig>,
    /// 训练干预配置
    pub training: Option<Vec<TrainingConfig>>,
}

impl Default for DriftConfig {
    fn default() -> Self {
        DriftConfig {
            years: 1.0,
            age_drift: AgeDriftConfig::default(),
            habituation: None,
            training: None,
        }
    }
}

impl DriftEngine {
    pub fn new() -> Self {
        DriftEngine {
            registry: ParamRegistry::global(),
        }
    }

    /// 对人格档案应用漂移模拟
    ///
    /// 返回漂移后的新档案（含漂移历史记录）
    pub fn apply_drift(
        &self,
        profile: &PersonalityProfile,
        config: &DriftConfig,
    ) -> PapsResult<PersonalityProfile> {
        let mut new_profile = profile.clone();

        // 初始化漂移历史
        let mut drift_records: Vec<crate::profile::DriftRecord> =
            profile.drift_history.clone().unwrap_or_default();

        let now_ms = chrono::Utc::now().timestamp_millis();
        let mut deltas: HashMap<String, f64> = HashMap::new();
        let mut reasons: Vec<String> = Vec::new();

        // 1. 年龄漂移
        if config.age_drift.enabled {
            let age_deltas = age_drift::compute_age_drift(
                self.registry,
                &new_profile.values,
                config.years,
                &config.age_drift,
            );
            for (param_id, delta) in &age_deltas {
                *deltas.entry(param_id.clone()).or_insert(0.0) += delta;
            }
            if !age_deltas.is_empty() {
                reasons.push(format!("年龄漂移 ({}年)", config.years));
            }
        }

        // 2. 习惯化/敏化
        if let Some(ref hab_config) = config.habituation {
            let hab_deltas = habituation::compute_habituation(
                self.registry,
                &new_profile.values,
                hab_config,
            );
            for (param_id, delta) in &hab_deltas {
                *deltas.entry(param_id.clone()).or_insert(0.0) += delta;
            }
            if !hab_deltas.is_empty() {
                reasons.push(format!("习惯化/敏化 ({:?})", hab_config.experience_type));
            }
        }

        // 3. 训练干预
        if let Some(ref trainings) = config.training {
            for training_config in trainings {
                let train_deltas = training::compute_training_effect(
                    self.registry,
                    &new_profile.values,
                    training_config,
                );
                for (param_id, delta) in &train_deltas {
                    *deltas.entry(param_id.clone()).or_insert(0.0) += delta;
                }
                if !train_deltas.is_empty() {
                    reasons.push(format!("训练干预 ({:?})", training_config.training_type));
                }
            }
        }

        // 应用所有漂移量（clamp到值域）
        for (param_id, delta) in &deltas {
            if let Some(spec) = self.registry.get(param_id) {
                if let Some(current) = new_profile.values.get(param_id) {
                    let new_val = (current + delta).clamp(spec.value_range.min(), spec.value_range.max());
                    new_profile.values.insert(param_id.clone(), new_val);
                }
            }
        }

        // 记录漂移历史
        if !deltas.is_empty() {
            drift_records.push(crate::profile::DriftRecord {
                timestamp_ms: now_ms,
                deltas,
                reason: reasons.join("; "),
            });
        }

        new_profile.drift_history = Some(drift_records);
        Ok(new_profile)
    }
}

impl Default for DriftEngine {
    fn default() -> Self {
        Self::new()
    }
}
