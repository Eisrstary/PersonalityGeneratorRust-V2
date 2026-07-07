use crate::types::{ParameterSpec, Domain, ValueRange, Granularity};
/// 领域G：时间性与发展
pub fn domain_g_params() -> Vec<ParameterSpec> { vec![
ParameterSpec::atomic("G063","参数漂移速率",Domain::G,"各参数随时间定向变化的速度(每个参数独立)。这是关于漂移的漂移——元漂移。",ValueRange::Float01).with_activation(0.95).with_collapse("重大事件(创伤/皈依/成功/丧失)可导致参数跳变(非连续变化)").with_drift("漂移方向可能反转(如B015内疚感在长期施害后从上升反转为下降)").with_age_drift(0.0),
ParameterSpec { id: "G064".into(), name: "重大事件相变阈值".into(), domain: Domain::G, definition: "引起参数永久偏移所需的最小事件冲击强度（1=什么事都改不了我，0=小事也能改变我）".into(), value_range: ValueRange::Float01, granularity: Granularity::Splittable, is_leaf: false, parent_id: None, sub_param_ids: vec!["G064a".into(),"G064b".into()], couplings: vec![], collapse_conditions: vec!["相变本身就是崩塌".into()], drift_patterns: vec!["随年龄G064通常上升(人格越来越稳定)".into()], reversal_conditions: vec![], age_drift_rate: 0.003, activation_probability: 0.95, is_activated: true },
ParameterSpec::atomic("G064a","对正面事件的相变阈值",Domain::G,"正面事件引起永久偏移所需的最小冲击强度",ValueRange::Float01).with_activation(0.90).with_age_drift(0.004),
ParameterSpec::atomic("G064b","对负面事件的相变阈值",Domain::G,"负面事件引起永久偏移所需的最小冲击强度",ValueRange::Float01).with_activation(0.95).with_coupling(vec!["B022"],"G064b↓ + B022→∞","一次背叛改变终身").with_age_drift(0.002),
ParameterSpec::atomic("G065","情境人格切换幅度",Domain::G,"同一参数在不同情境(家庭/职场/独处/社交)间的取值差异幅度",ValueRange::FloatPercent{min:0.0,max:100.0}).with_activation(0.90).with_coupling(vec!["E055"],"G065↑ + E055↑","真诚地相信每个情境的自己都是'真正的自己'").with_coupling(vec!["B015e","B015f"],"G065↑ + B015e↑ + B015f↓","虚伪型切换").with_collapse("当两个情境发生碰撞(如家人出现在职场)：切换可能崩溃").with_age_drift(-0.05),
ParameterSpec::atomic("G066","身份叙事更新速率",Domain::G,"系统的自我定义在新经历后更新的速度（0=持续重写，∞天=固定不变）",ValueRange::FloatDays{min:0.0,max:36500.0}).with_activation(0.90).with_coupling(vec!["G064"],"G066→∞ + G064↑","我一直都是这样的人").with_coupling(vec!["C037"],"G066→0 + C037↓","今天的我和昨天完全不同").with_drift("随年龄G066通常下降(自我叙事越来越固定)").with_age_drift(30.0),
] }
