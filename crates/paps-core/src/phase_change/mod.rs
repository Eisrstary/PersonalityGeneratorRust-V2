use crate::profile::PersonalityProfile;
use crate::params::ParamRegistry;
use crate::error::PapsResult;

mod triggers;
pub use triggers::PhaseEventType;

/// 相变引擎 —— 模拟参数的阶段性非连续跳变
pub struct PhaseChangeEngine {
    registry: &'static ParamRegistry,
}

impl PhaseChangeEngine {
    pub fn new() -> Self {
        PhaseChangeEngine {
            registry: ParamRegistry::global(),
        }
    }

    /// 对人格档案应用相变事件
    ///
    /// 相变是非连续的参数跳变，与漂移不同，发生在短时间内。
    /// 跳变幅度受 G064（重大事件相变阈值）影响：G064 越高，跳变幅度越小。
    pub fn apply_event(
        &self,
        profile: &PersonalityProfile,
        event: PhaseEventType,
    ) -> PapsResult<PersonalityProfile> {
        let mut new_profile = profile.clone();

        // 获取相变阈值（G064 或 G064a/G064b）
        let threshold = self.get_threshold(profile, &event);

        // 获取该事件的跳变规则
        let jumps = triggers::get_phase_jumps(&event, threshold);

        // 应用跳变（clamp到值域）
        for (param_id, delta) in &jumps {
            if let Some(spec) = self.registry.get(param_id) {
                if let Some(current) = new_profile.values.get(param_id) {
                    let new_val = (*current + delta).clamp(spec.value_range.min(), spec.value_range.max());
                    new_profile.values.insert(param_id.clone(), new_val);
                }
            }
        }

        // 记录相变事件
        new_profile.phase_events.push(format!("{:?}", event));

        Ok(new_profile)
    }

    /// 获取相变阈值（G064 越高 → 跳变越小）
    fn get_threshold(&self, profile: &PersonalityProfile, event: &PhaseEventType) -> f64 {
        let base_id = match event {
            PhaseEventType::Betrayal | PhaseEventType::Loss | PhaseEventType::Humiliation
            | PhaseEventType::WitnessTrauma => "G064b", // 负面事件
            PhaseEventType::PowerGain | PhaseEventType::Forgiveness => "G064a", // 正面事件
        };
        // G064 值越高，阈值越大，跳变越小
        // 阈值因子 = 1.0 - G064（G064=0 → 因子=1 全跳变；G064=1 → 因子=0 无跳变）
        let g064 = profile.get(base_id).unwrap_or(0.5);
        1.0 - g064
    }
}

impl Default for PhaseChangeEngine {
    fn default() -> Self {
        Self::new()
    }
}
