use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rand::Rng;
use rand_chacha::ChaCha20Rng;

use crate::params::ParamRegistry;
use crate::seed::SeedCode;
use crate::error::{PapsError, PapsResult};

/// 人格参数档案 —— 所有叶子参数在某一时刻的取值快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityProfile {
    /// 种子码
    pub seed: u64,
    /// 生成时间戳 (Unix epoch ms)
    pub created_at_ms: i64,
    /// 叶子参数值映射：参数ID → 值
    pub values: HashMap<String, f64>,
    /// 已激活的参数ID集合（其余为中性/休眠参数）
    pub activated_params: Vec<String>,
    /// 漂移历史（可选，漂移模拟后填充）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_history: Option<Vec<DriftRecord>>,
    /// 反转标记的参数列表
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub reversed_params: Vec<String>,
    /// 已触发的相变事件列表
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub phase_events: Vec<String>,
}

/// 漂移记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftRecord {
    /// 漂移发生的时间 (Unix epoch ms)
    pub timestamp_ms: i64,
    /// 漂移量：参数ID → 变化量
    pub deltas: HashMap<String, f64>,
    /// 漂移原因
    pub reason: String,
}

impl PersonalityProfile {
    /// 获取指定参数的值
    pub fn get(&self, param_id: &str) -> Option<f64> {
        self.values.get(param_id).copied()
    }

    /// 按领域获取参数值
    pub fn get_domain(&self, domain: crate::types::Domain) -> HashMap<String, f64> {
        let reg = ParamRegistry::global();
        let domain_params = reg.by_domain(domain);
        let mut result = HashMap::new();
        for pid in domain_params {
            if let Some(&val) = self.values.get(pid) {
                result.insert(pid.clone(), val);
            }
        }
        result
    }

    /// 获取顶层参数值（从子参数聚合）
    pub fn get_top_level(&self) -> HashMap<String, f64> {
        let reg = ParamRegistry::global();
        let mut result = HashMap::new();
        for pid in reg.top_level_param_ids() {
            if let Some(param) = reg.get(pid) {
                if param.sub_param_ids.is_empty() {
                    // 本身就是叶子参数
                    if let Some(&val) = self.values.get(pid) {
                        result.insert(pid.clone(), val);
                    }
                } else {
                    // 从子参数聚合
                    let mut sum = 0.0;
                    let mut count = 0;
                    for sub_id in &param.sub_param_ids {
                        if let Some(&val) = self.values.get(sub_id) {
                            sum += val;
                            count += 1;
                        }
                    }
                    if count > 0 {
                        result.insert(pid.clone(), sum / count as f64);
                    }
                }
            }
        }
        result
    }

    /// 序列化为 JSON 字符串
    pub fn to_json(&self) -> PapsResult<String> {
        serde_json::to_string(self).map_err(PapsError::JsonError)
    }

    /// 序列化为格式化的 JSON 字符串
    pub fn to_json_pretty(&self) -> PapsResult<String> {
        serde_json::to_string_pretty(self).map_err(PapsError::JsonError)
    }

    /// 从 JSON 字符串反序列化
    pub fn from_json(json: &str) -> PapsResult<Self> {
        serde_json::from_str(json).map_err(PapsError::JsonError)
    }

    /// 获取参数总数
    pub fn param_count(&self) -> usize {
        self.values.len()
    }

    /// 生成AI优化的结构化档案
    ///
    /// 返回 AiProfile：所有参数归一化到0-1，带语义标签，极端值优先排序，
    /// 包含推断的人格特征和自然语言描述。这是对AI最友好的输出格式。
    pub fn to_ai_profile(&self) -> crate::report::AiProfile {
        crate::report::build_ai_profile(
            self.seed,
            self.created_at_ms,
            &self.values,
            &self.activated_params,
            self.drift_history.clone(),
            &self.phase_events,
            &self.reversed_params,
        )
    }

    /// 生成AI优化档案的 JSON 字符串
    pub fn to_ai_json(&self) -> PapsResult<String> {
        let report = self.to_ai_profile();
        serde_json::to_string_pretty(&report).map_err(PapsError::JsonError)
    }

    /// 生成AI最优阅读格式：高密度结构化自然语言
    pub fn to_ai_text(&self) -> String {
        self.to_ai_profile().to_ai_text()
    }
}

/// 预计算的叶子参数元数据（避免每次生成时查HashMap）
struct LeafMeta {
    id: String,
    activation_prob: f64,
    range_min: f64,
    range_max: f64,
    neutral: f64,
}

/// 人格生成器
pub struct PersonalityGenerator {
    registry: &'static ParamRegistry,
    /// 预计算的叶子参数元数据（按 leaf_param_ids 顺序）
    leaf_metas: Vec<LeafMeta>,
}

impl PersonalityGenerator {
    /// 创建生成器
    pub fn new() -> Self {
        let registry = ParamRegistry::global();
        let leaf_metas: Vec<LeafMeta> = registry.leaf_param_ids().iter().map(|id| {
            let spec = registry.get(id).unwrap();
            LeafMeta {
                id: id.clone(),
                activation_prob: spec.activation_probability,
                range_min: spec.value_range.min(),
                range_max: spec.value_range.max(),
                neutral: spec.value_range.neutral_value(),
            }
        }).collect();
        PersonalityGenerator { registry, leaf_metas }
    }

    /// 使用种子码确定性生成人格
    ///
    /// 同一种子码保证生成完全相同的人格参数。
    pub fn generate(&self, seed: SeedCode) -> PersonalityProfile {
        let mut rng = seed.create_rng();
        self.generate_from_rng(&mut rng, seed.as_u64(), None)
    }

    /// 使用种子码生成人格，部分参数使用预设值
    ///
    /// presets: 参数ID → 预设值。未指定的参数由RNG随机生成。
    /// 预设值会在值域内被 clamp。
    pub fn generate_with_presets(
        &self,
        seed: SeedCode,
        presets: &HashMap<String, f64>,
    ) -> PapsResult<PersonalityProfile> {
        // 验证预设值
        for (param_id, &value) in presets {
            if let Some(spec) = self.registry.get(param_id) {
                if !spec.value_range.contains(value) {
                    return Err(PapsError::ValueOutOfRange(
                        param_id.clone(),
                        value,
                        spec.value_range.min(),
                        spec.value_range.max(),
                    ));
                }
            } else {
                return Err(PapsError::ParamNotFound(param_id.clone()));
            }
        }

        let mut rng = seed.create_rng();
        Ok(self.generate_from_rng(&mut rng, seed.as_u64(), Some(presets)))
    }

    /// 完全随机生成人格（使用系统熵源）
    ///
    /// 返回 (种子码, 人格档案)
    pub fn generate_random(&self) -> (SeedCode, PersonalityProfile) {
        use rand::SeedableRng;
        // 使用系统熵源生成随机种子
        let mut sys_rng = rand::thread_rng();
        let random_seed: u64 = sys_rng.gen();
        let seed = SeedCode::from(random_seed);
        let profile = self.generate(seed);
        (seed, profile)
    }

    /// 核心生成逻辑 —— 使用预计算元数据，零HashMap查找
    fn generate_from_rng(
        &self,
        rng: &mut ChaCha20Rng,
        seed: u64,
        presets: Option<&HashMap<String, f64>>,
    ) -> PersonalityProfile {
        use rand::Rng;

        let n = self.leaf_metas.len();
        let mut values: HashMap<String, f64> = HashMap::with_capacity(n);
        let mut activated_params: Vec<String> = Vec::with_capacity(n);

        for meta in &self.leaf_metas {
            let has_preset = presets.map_or(false, |p| p.contains_key(&meta.id));

            let is_activated = if has_preset {
                true
            } else {
                rng.gen::<f64>() < meta.activation_prob
            };

            let value = if has_preset {
                presets.unwrap().get(&meta.id).copied().unwrap()
            } else if is_activated {
                let factor: f64 = rng.gen();
                meta.range_min + factor * (meta.range_max - meta.range_min)
            } else {
                meta.neutral
            };

            if is_activated {
                activated_params.push(meta.id.clone());
            }
            values.insert(meta.id.clone(), value);
        }

        PersonalityProfile {
            seed,
            created_at_ms: chrono::Utc::now().timestamp_millis(),
            values,
            activated_params,
            drift_history: None,
            reversed_params: vec![],
            phase_events: vec![],
        }
    }
}

impl Default for PersonalityGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_generation() {
        let gen = PersonalityGenerator::new();
        let seed = SeedCode::from(42u64);

        let profile1 = gen.generate(seed);
        let profile2 = gen.generate(seed);

        assert_eq!(profile1.values, profile2.values,
            "Same seed must produce identical profiles");
    }

    #[test]
    fn test_different_seeds_different_profiles() {
        let gen = PersonalityGenerator::new();
        let p1 = gen.generate(SeedCode::from(1u64));
        let p2 = gen.generate(SeedCode::from(2u64));

        // 极大概率不同（浮点比较允许微小概率碰撞）
        let mut diff_count = 0;
        for (k, v1) in &p1.values {
            if let Some(v2) = p2.values.get(k) {
                if (v1 - v2).abs() > 1e-10 {
                    diff_count += 1;
                }
            }
        }
        assert!(diff_count > 0, "Different seeds should produce different values");
    }

    #[test]
    fn test_presets_applied() {
        let gen = PersonalityGenerator::new();
        let seed = SeedCode::from(12345u64);

        let mut presets = HashMap::new();
        presets.insert("A003".to_string(), 0.75);
        presets.insert("B018".to_string(), 0.25);

        let profile = gen.generate_with_presets(seed, &presets).unwrap();

        assert!((profile.get("A003").unwrap() - 0.75).abs() < 1e-10);
        assert!((profile.get("B018").unwrap() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn test_preset_out_of_range() {
        let gen = PersonalityGenerator::new();
        let seed = SeedCode::from(12345u64);

        let mut presets = HashMap::new();
        presets.insert("A003".to_string(), 1.5); // out of [0,1]

        let result = gen.generate_with_presets(seed, &presets);
        assert!(result.is_err());
    }

    #[test]
    fn test_preset_nonexistent_param() {
        let gen = PersonalityGenerator::new();
        let seed = SeedCode::from(12345u64);

        let mut presets = HashMap::new();
        presets.insert("NONEXISTENT".to_string(), 0.5);

        let result = gen.generate_with_presets(seed, &presets);
        assert!(result.is_err());
    }

    #[test]
    fn test_random_generation() {
        let gen = PersonalityGenerator::new();
        let (seed1, _profile1) = gen.generate_random();
        let (seed2, _profile2) = gen.generate_random();

        assert_ne!(seed1.as_u64(), seed2.as_u64(),
            "Random seeds should almost certainly differ");
    }

    #[test]
    fn test_json_roundtrip() {
        let gen = PersonalityGenerator::new();
        let profile = gen.generate(SeedCode::from(42u64));

        let json = profile.to_json().unwrap();
        let restored = PersonalityProfile::from_json(&json).unwrap();

        // Compare key fields
        assert_eq!(profile.seed, restored.seed);
        assert_eq!(profile.values.len(), restored.values.len());
        for (k, v1) in &profile.values {
            let v2 = restored.values.get(k).expect("key should exist");
            assert!((v1 - v2).abs() < 1e-10, "Value mismatch for {}: {} vs {}", k, v1, v2);
        }
    }

    #[test]
    fn test_get_top_level() {
        let gen = PersonalityGenerator::new();
        let profile = gen.generate(SeedCode::from(42u64));

        let top = profile.get_top_level();
        // Should have top-level params
        assert!(top.contains_key("A001"));
        assert!(top.contains_key("B015"));
    }

    #[test]
    fn test_param_count() {
        let gen = PersonalityGenerator::new();
        let profile = gen.generate(SeedCode::from(42u64));

        let reg = ParamRegistry::global();
        assert_eq!(profile.param_count(), reg.leaf_param_count(),
            "Profile should contain all leaf params");
    }
}
