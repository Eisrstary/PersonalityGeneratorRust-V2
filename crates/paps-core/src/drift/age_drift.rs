use std::collections::HashMap;
use crate::params::ParamRegistry;

/// 年龄漂移配置
#[derive(Debug, Clone)]
pub struct AgeDriftConfig {
    /// 是否启用年龄漂移
    pub enabled: bool,
    /// 全局漂移倍率（1.0=标准速率）
    pub global_multiplier: f64,
}

impl Default for AgeDriftConfig {
    fn default() -> Self {
        AgeDriftConfig {
            enabled: true,
            global_multiplier: 1.0,
        }
    }
}

/// 计算年龄相关漂移
///
/// 每个参数的漂移量 = age_drift_rate × years × global_multiplier
/// age_drift_rate 定义在 ParameterSpec 中，正=随年龄上升，负=随年龄下降
pub fn compute_age_drift(
    registry: &ParamRegistry,
    _current_values: &HashMap<String, f64>,
    years: f64,
    config: &AgeDriftConfig,
) -> HashMap<String, f64> {
    let mut deltas = HashMap::new();

    for param_id in registry.leaf_param_ids() {
        if let Some(spec) = registry.get(param_id) {
            let drift = spec.age_drift_rate * years * config.global_multiplier;
            if drift.abs() > 1e-10 {
                deltas.insert(param_id.clone(), drift);
            }
        }
    }

    deltas
}
