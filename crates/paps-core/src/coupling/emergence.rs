use serde::{Deserialize, Serialize};
use super::MatchedRule;

/// 涌现行为模式 —— 多个耦合规则交互产生的高阶模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencePattern {
    /// 模式名称
    pub name: String,
    /// 模式描述
    pub description: String,
    /// 支撑该模式的规则ID列表
    pub supporting_rules: Vec<String>,
    /// 涌现置信度 [0,1]
    pub confidence: f64,
    /// 模式类别
    pub category: EmergenceCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergenceCategory {
    /// 暴力模式
    ViolencePattern,
    /// 权力模式
    PowerPattern,
    /// 共情模式
    EmpathyPattern,
    /// 自我欺骗模式
    SelfDeceptionPattern,
    /// 矛盾模式（自我撕裂）
    ContradictionPattern,
}

impl EmergenceCategory {
    pub fn to_string(&self) -> String {
        match self {
            EmergenceCategory::ViolencePattern => "暴力模式".into(),
            EmergenceCategory::PowerPattern => "权力模式".into(),
            EmergenceCategory::EmpathyPattern => "共情模式".into(),
            EmergenceCategory::SelfDeceptionPattern => "自我欺骗模式".into(),
            EmergenceCategory::ContradictionPattern => "矛盾模式".into(),
        }
    }
}

/// 分析涌现模式
///
/// 当多个耦合规则同时匹配时，分析是否存在高阶行为模式涌现。
pub fn analyze_emergence(matched_rules: &[MatchedRule]) -> Vec<EmergencePattern> {
    let mut patterns = Vec::new();
    let rule_ids: Vec<&str> = matched_rules.iter().map(|r| r.rule_id.as_str()).collect();

    // 模式1: 先发制人型暴力
    // A008↑ + B019↑ (威胁→愤怒链) + B019↑ + C030↓ (冲动暴力)
    if rule_ids.contains(&"threat_anger_chain") && rule_ids.contains(&"impulsive_anger") {
        patterns.push(EmergencePattern {
            name: "先发制人型暴力".into(),
            description: "感知威胁后几乎立即产生愤怒并转化为攻击行为——威胁→愤怒→攻击链几乎没有缓冲。在模糊情境中容易过度反应。".into(),
            supporting_rules: vec!["threat_anger_chain".into(), "impulsive_anger".into()],
            confidence: 0.85,
            category: EmergenceCategory::ViolencePattern,
        });
    }

    // 模式2: 冷酷权力追求者
    // C032↑ + B015↓ (不择手段) + B015↓ + D040↑ (冷酷攻击)
    if rule_ids.contains(&"high_power_low_guilt") && rule_ids.contains(&"low_guilt_high_aggression") {
        patterns.push(EmergencePattern {
            name: "冷酷权力追求者".into(),
            description: "追求权力过程中不择手段，攻击行为不伴随内疚——马基雅维利型人格光谱。".into(),
            supporting_rules: vec!["high_power_low_guilt".into(), "low_guilt_high_aggression".into()],
            confidence: 0.80,
            category: EmergenceCategory::PowerPattern,
        });
    }

    // 模式3: 自我撕裂的行动者
    // B015↑ + D040↑ (边攻击边内疚) + E053↑ + B015↑ + D040↑ (自我撕裂)
    if rule_ids.contains(&"high_guilt_high_aggression") && rule_ids.contains(&"self_tearing_action") {
        patterns.push(EmergencePattern {
            name: "自我撕裂的行动者".into(),
            description: "明知行为会造成伤害且深感内疚，但仍然继续行动——内在的道德冲突与行为持续性并存。".into(),
            supporting_rules: vec!["high_guilt_high_aggression".into(), "self_tearing_action".into()],
            confidence: 0.75,
            category: EmergenceCategory::ContradictionPattern,
        });
    }

    // 模式4: 真诚的自恋者
    // E055↑ + E046↑ (真诚地相信自己是伟大的) + C036c↑ + E055↑ (自我欺骗闭环)
    if rule_ids.contains(&"sincere_grandiosity") && rule_ids.contains(&"self_deception_loop") {
        patterns.push(EmergencePattern {
            name: "真诚的自恋者".into(),
            description: "通过自我欺骗维持高外显自尊——真诚地相信自己的伟大，不接受相反证据。自我欺骗形成完美闭环。".into(),
            supporting_rules: vec!["sincere_grandiosity".into(), "self_deception_loop".into()],
            confidence: 0.80,
            category: EmergenceCategory::SelfDeceptionPattern,
        });
    }

    // 模式5: 选择性共情者
    // selective_empathy + 相关规则
    if rule_ids.contains(&"selective_empathy") || rule_ids.contains(&"high_empathy_ingroup_low_outgroup") {
        patterns.push(EmergencePattern {
            name: "选择性共情者".into(),
            description: "对内群体成员高度共情，对外群体成员共情显著降低——这是最普遍的共情模式，内群体偏好是人类默认设置。".into(),
            supporting_rules: vec!["selective_empathy".into(), "high_empathy_ingroup_low_outgroup".into()],
            confidence: 0.85,
            category: EmergenceCategory::EmpathyPattern,
        });
    }

    // 模式6: 使命驱动型极端主义者
    // E051↑ + D040↑ (为使命而战) + E051↑ + B015↓ (为使命不择手段)
    if rule_ids.contains(&"mission_fighter") && rule_ids.contains(&"mission_ends_justify") {
        patterns.push(EmergencePattern {
            name: "使命驱动型极端主义者".into(),
            description: "强烈的使命感驱动攻击行为，且为达成使命可以忽视道德约束——信念成为暴力的合法性来源。".into(),
            supporting_rules: vec!["mission_fighter".into(), "mission_ends_justify".into()],
            confidence: 0.75,
            category: EmergenceCategory::ViolencePattern,
        });
    }

    // 模式7: 偏执型警觉者
    // paranoid_processing + threat_anger_chain
    if rule_ids.contains(&"paranoid_processing") && rule_ids.contains(&"threat_anger_chain") {
        patterns.push(EmergencePattern {
            name: "偏执型警觉者".into(),
            description: "高度警觉威胁线索+高背叛检测+威胁快速转化为愤怒——世界被感知为充满敌意的地方，随时准备反击。".into(),
            supporting_rules: vec!["paranoid_processing".into(), "threat_anger_chain".into()],
            confidence: 0.80,
            category: EmergenceCategory::ViolencePattern,
        });
    }

    // 模式8: 施虐型人格光谱
    // sadistic_perception + sadistic_aggression
    if rule_ids.contains(&"sadistic_perception") && rule_ids.contains(&"sadistic_aggression") {
        patterns.push(EmergencePattern {
            name: "施虐型人格光谱".into(),
            description: "能清晰感知他人痛苦，且痛苦引发愉悦——攻击行为伴随享受。这不是'类型'，而是参数在特定区间的涌现结果。".into(),
            supporting_rules: vec!["sadistic_perception".into(), "sadistic_aggression".into()],
            confidence: 0.70,
            category: EmergenceCategory::ViolencePattern,
        });
    }

    patterns
}
