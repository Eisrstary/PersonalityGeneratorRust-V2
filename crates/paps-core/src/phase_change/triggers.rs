use std::collections::HashMap;

/// 相变事件类型 —— 触发的六种重大事件
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseEventType {
    /// 背叛：被信任者严重伤害
    Betrayal,
    /// 丧失：失去重要他人
    Loss,
    /// 羞辱：公开被羞辱
    Humiliation,
    /// 权力：突然获得权力
    PowerGain,
    /// 原谅：被受害者原谅
    Forgiveness,
    /// 见证：目睹极端事件
    WitnessTrauma,
}

/// 获取相变跳变规则
///
/// 返回 (参数ID → 跳变量) 的映射
/// threshold_factor: 0=全跳变，1=无跳变
pub fn get_phase_jumps(event: &PhaseEventType, threshold_factor: f64) -> HashMap<String, f64> {
    let factor = threshold_factor.clamp(0.0, 1.0);
    let mut jumps = HashMap::new();

    match event {
        PhaseEventType::Betrayal => {
            // 被信任者严重伤害 → F061↓(信任崩塌) + A008↑(威胁放大) + B022可能跳变
            jumps.insert("F061".into(), -0.4 * factor);
            jumps.insert("F061a".into(), -0.5 * factor);
            jumps.insert("F061b".into(), -0.3 * factor);
            jumps.insert("A008".into(), 0.3 * factor);
            jumps.insert("A008b".into(), 0.4 * factor);
            jumps.insert("F062".into(), 0.3 * factor);
            jumps.insert("B022".into(), 0.3 * factor);
            jumps.insert("B022a".into(), 0.4 * factor);
            jumps.insert("C033".into(), -0.3 * factor);
            jumps.insert("C033a".into(), -0.4 * factor);
        }
        PhaseEventType::Loss => {
            // 失去重要他人 → C026↑(意义寻求) + E051可能↑或↓
            jumps.insert("C026".into(), 0.3 * factor);
            jumps.insert("C026a".into(), 0.25 * factor);
            jumps.insert("C026c".into(), 0.35 * factor);
            jumps.insert("B011_sadness".into(), -0.2 * factor); // 更易悲伤
            jumps.insert("E051".into(), -0.2 * factor); // 使命感受到冲击
        }
        PhaseEventType::Humiliation => {
            // 公开被羞辱 → B017可能↑或↓ + E046↓
            jumps.insert("E046".into(), -0.3 * factor);
            jumps.insert("E046a".into(), -0.4 * factor);
            jumps.insert("B017a".into(), 0.3 * factor);
            jumps.insert("B017c".into(), 0.35 * factor);
            jumps.insert("E045".into(), -0.2 * factor);
        }
        PhaseEventType::PowerGain => {
            // 突然获得权力 → C031可能↑ + A010可能从+1跳变到-1
            jumps.insert("C031".into(), 0.3 * factor);
            jumps.insert("C031a".into(), 0.4 * factor);
            jumps.insert("C031c".into(), 0.25 * factor);
            jumps.insert("A010".into(), -0.4 * factor); // 从仰视强者→俯视弱者
            jumps.insert("C032".into(), 0.2 * factor);
            jumps.insert("C034".into(), 0.15 * factor);
        }
        PhaseEventType::Forgiveness => {
            // 被受害者原谅 → B015可能从零跳变到极高(延迟内疚涌现)
            jumps.insert("B015".into(), 0.5 * factor);
            jumps.insert("B015a".into(), 0.5 * factor);
            jumps.insert("B015b".into(), 0.4 * factor);
            jumps.insert("B015c".into(), 0.45 * factor);
            jumps.insert("B015f".into(), 0.6 * factor);
            jumps.insert("B022".into(), -0.3 * factor); // 怨恨衰减
            jumps.insert("B022a".into(), -0.4 * factor);
        }
        PhaseEventType::WitnessTrauma => {
            // 目睹极端事件 → 多个参数同时跳变
            jumps.insert("A008".into(), 0.3 * factor);
            jumps.insert("A008a".into(), 0.35 * factor);
            jumps.insert("B011_fear".into(), -0.25 * factor); // 更易恐惧
            jumps.insert("C026".into(), 0.25 * factor);
            jumps.insert("E044".into(), 0.2 * factor);
            jumps.insert("E044a".into(), 0.25 * factor);
            jumps.insert("G064b".into(), -0.15 * factor); // 对负面事件更敏感
            jumps.insert("A003".into(), -0.2 * factor); // 躯体解离风险
        }
    }

    jumps
}
