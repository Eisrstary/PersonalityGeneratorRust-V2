use crate::types::{ParameterSpec, Domain, ValueRange, Granularity};
/// 领域D：行为执行
pub fn domain_d_params() -> Vec<ParameterSpec> { vec![
ParameterSpec { id: "D039".into(), name: "行为蓄能时间".into(), domain: Domain::D, definition: "从决定行动到实际开始行动之间的时间延迟".into(), value_range: ValueRange::FloatMs{min:0.0,max:86400000.0}, granularity: Granularity::Splittable, is_leaf: false, parent_id: None, sub_param_ids: vec!["D039a".into(),"D039b".into(),"D039c".into(),"D039d".into()], couplings: vec![], collapse_conditions: vec!["截止日期临近时D039可能从∞跳变到0(截止日期效应)".into()], drift_patterns: vec![], reversal_conditions: vec![], age_drift_rate: 1000.0, activation_probability: 0.95, is_activated: true },
ParameterSpec::atomic("D039a","对愉快任务的蓄能",Domain::D,"决定做愉快任务到实际开始的延迟(ms)",ValueRange::FloatMs{min:0.0,max:3600000.0}).with_activation(0.95).with_age_drift(500.0),
ParameterSpec::atomic("D039b","对不愉快任务的蓄能",Domain::D,"决定做不愉快任务到实际开始的延迟(ms)",ValueRange::FloatMs{min:0.0,max:86400000.0}).with_activation(0.95).with_coupling(vec!["B015"],"D039b→∞ + B015↑","拖延-内疚循环").with_age_drift(2000.0),
ParameterSpec::atomic("D039c","对道德任务的蓄能",Domain::D,"决定做道德行动到实际开始的延迟(ms)",ValueRange::FloatMs{min:0.0,max:3600000.0}).with_activation(0.80).with_age_drift(300.0),
ParameterSpec::atomic("D039d","对危险任务的蓄能",Domain::D,"决定做危险行动到实际开始的延迟(ms)",ValueRange::FloatMs{min:0.0,max:86400000.0}).with_activation(0.75).with_age_drift(800.0),
ParameterSpec { id: "D040".into(), name: "攻击行为基线".into(), domain: Domain::D, definition: "系统在无挑衅情况下发起攻击行为的概率".into(), value_range: ValueRange::Float01, granularity: Granularity::Splittable, is_leaf: false, parent_id: None, sub_param_ids: vec!["D040a".into(),"D040b".into(),"D040c".into(),"D040d".into(),"D040e".into()], couplings: vec![], collapse_conditions: vec!["威胁情境下D040可能从零跳变到极高(防御性攻击)".into()], drift_patterns: vec!["长期处于暴力环境中D040通常上升".into()], reversal_conditions: vec![], age_drift_rate: -0.001, activation_probability: 0.60, is_activated: true },
ParameterSpec::atomic("D040a","对内群体的攻击",Domain::D,"对内群体成员发起攻击的概率",ValueRange::Float01).with_activation(0.30).with_age_drift(-0.001),
ParameterSpec::atomic("D040b","对外群体的攻击",Domain::D,"对外群体成员发起攻击的概率",ValueRange::Float01).with_activation(0.45).with_coupling(vec!["B015b"],"D040b↑ + B015b↓","冷酷型").with_coupling(vec!["B015b"],"D040b↑ + B015b↑","迫不得已型"),
ParameterSpec::atomic("D040c","言语攻击",Domain::D,"发起言语攻击的概率",ValueRange::Float01).with_activation(0.75).with_age_drift(-0.001),
ParameterSpec::atomic("D040d","身体攻击",Domain::D,"发起身体攻击的概率",ValueRange::Float01).with_activation(0.15).with_age_drift(-0.002),
ParameterSpec::atomic("D040e","制度性攻击",Domain::D,"通过制度/规则伤害他人的概率",ValueRange::Float01).with_activation(0.25).with_age_drift(0.001),
ParameterSpec { id: "D041".into(), name: "规则遵循度".into(), domain: Domain::D, definition: "系统遵守外部规则(法律/规范/命令)的默认程度".into(), value_range: ValueRange::Float01, granularity: Granularity::Splittable, is_leaf: false, parent_id: None, sub_param_ids: vec!["D041a".into(),"D041b".into(),"D041c".into(),"D041d".into()], couplings: vec![], collapse_conditions: vec!["当规则与E051(使命感)冲突时：D041可能跳变".into()], drift_patterns: vec![], reversal_conditions: vec!["当规则制定者背叛系统时：D041a可能从高→低(规则信任崩塌)".into()], age_drift_rate: 0.001, activation_probability: 0.85, is_activated: true },
ParameterSpec::atomic("D041a","对法律规则的遵循",Domain::D,"遵守法律的默认程度",ValueRange::Float01).with_activation(0.85).with_age_drift(0.002),
ParameterSpec::atomic("D041b","对社会规范的遵循",Domain::D,"遵守社会规范的默认程度",ValueRange::Float01).with_activation(0.85).with_age_drift(0.001),
ParameterSpec::atomic("D041c","对权威命令的遵循",Domain::D,"遵守权威命令的默认程度",ValueRange::Float01).with_activation(0.80).with_coupling(vec!["C028"],"D041c↑ + C028↓","盲从型").with_age_drift(-0.001),
ParameterSpec::atomic("D041d","对自己制定的规则的遵循",Domain::D,"遵守自己制定的道德准则的默认程度",ValueRange::Float01).with_activation(0.75).with_age_drift(0.002),
ParameterSpec::atomic("D042","行为灵活性",Domain::D,"原计划受阻时切换到替代方案的速度",ValueRange::Float01).with_activation(0.95).with_coupling(vec!["E051"],"D042↓ + E051↑","撞了南墙也不回头").with_coupling(vec!["C037"],"D042↑ + C037↓","随风倒型").with_collapse("压力下D042可能急剧下降(认知僵化)").with_drift("随年龄通常下降(习惯固化)").with_age_drift(-0.002),
] }
