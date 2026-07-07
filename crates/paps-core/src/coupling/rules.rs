use serde::{Deserialize, Serialize};

/// 参数方向
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ParamDirection {
    /// 高于阈值
    High,
    /// 低于阈值
    Low,
    /// 接近阈值（中性）
    Neutral,
}

/// 参数条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamCondition {
    /// 参数ID
    pub param_id: String,
    /// 方向
    pub direction: ParamDirection,
    /// 阈值
    pub threshold: f64,
}

/// 耦合类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CouplingCategory {
    PerceptionEmotion,
    EmotionBehavior,
    MotivationBehavior,
    MetacognitionEmotion,
    SocialMetacognition,
}

impl CouplingCategory {
    pub fn to_string(&self) -> String {
        match self {
            CouplingCategory::PerceptionEmotion => "感知-情绪耦合".into(),
            CouplingCategory::EmotionBehavior => "情绪-行为耦合".into(),
            CouplingCategory::MotivationBehavior => "动机-行为耦合".into(),
            CouplingCategory::MetacognitionEmotion => "元认知-情绪耦合".into(),
            CouplingCategory::SocialMetacognition => "社交-元认知耦合".into(),
        }
    }
}

/// 耦合规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingRule {
    /// 规则ID
    pub id: String,
    /// 耦合类别
    pub category: CouplingCategory,
    /// 条件列表（全部满足才匹配）
    pub conditions: Vec<ParamCondition>,
    /// 条件描述文本
    pub condition_desc: String,
    /// 现象描述
    pub phenomenon: String,
    /// 基础置信度 [0,1]
    pub base_confidence: f64,
}

/// 构建所有耦合规则
pub fn build_rules() -> Vec<CouplingRule> {
    vec![
        // ===== 感知-情绪耦合 =====
        CouplingRule {
            id: "selective_empathy".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "A009a".into(), direction: ParamDirection::High, threshold: 0.7 },
                ParamCondition { param_id: "A009b".into(), direction: ParamDirection::Low, threshold: 0.3 },
            ],
            condition_desc: "A009a↑ + A009b↓".into(),
            phenomenon: "对内群体痛苦高度敏感，对外群体痛苦相对麻木——选择性共情".into(),
            base_confidence: 0.85,
        },
        CouplingRule {
            id: "self_torment_after_harm".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "A009c".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "B015".into(), direction: ParamDirection::High, threshold: 0.6 },
            ],
            condition_desc: "A009c↑ + B015↑".into(),
            phenomenon: "伤害他人后能清晰感知对方痛苦并产生强烈内疚——自我折磨型".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "cold_after_harm".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "A009c".into(), direction: ParamDirection::Low, threshold: 0.3 },
                ParamCondition { param_id: "B015".into(), direction: ParamDirection::Low, threshold: 0.3 },
            ],
            condition_desc: "A009c↓ + B015↓".into(),
            phenomenon: "伤害他人后既不感知对方痛苦也不感到内疚——冷酷型".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "sadistic_perception".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "A009".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "B016".into(), direction: ParamDirection::High, threshold: 0.5 },
            ],
            condition_desc: "A009↑ + B016→+1".into(),
            phenomenon: "能清晰感知他人痛苦，且痛苦引发自身愉悦——施虐型感知".into(),
            base_confidence: 0.75,
        },
        CouplingRule {
            id: "threat_anger_chain".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "A008".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "B019".into(), direction: ParamDirection::High, threshold: 0.6 },
            ],
            condition_desc: "A008↑ + B019↑".into(),
            phenomenon: "感知到威胁后迅速产生愤怒——敌意归因→愤怒→攻击链".into(),
            base_confidence: 0.85,
        },

        // ===== 情绪-行为耦合 =====
        CouplingRule {
            id: "high_guilt_high_aggression".into(),
            category: CouplingCategory::EmotionBehavior,
            conditions: vec![
                ParamCondition { param_id: "B015".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "D040".into(), direction: ParamDirection::High, threshold: 0.5 },
            ],
            condition_desc: "B015↑ + D040↑".into(),
            phenomenon: "边攻击边内疚——自我撕裂型暴力".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "low_guilt_high_aggression".into(),
            category: CouplingCategory::EmotionBehavior,
            conditions: vec![
                ParamCondition { param_id: "B015".into(), direction: ParamDirection::Low, threshold: 0.3 },
                ParamCondition { param_id: "D040".into(), direction: ParamDirection::High, threshold: 0.5 },
            ],
            condition_desc: "B015↓ + D040↑".into(),
            phenomenon: "攻击且无内疚——冷酷型暴力".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "sadistic_aggression".into(),
            category: CouplingCategory::EmotionBehavior,
            conditions: vec![
                ParamCondition { param_id: "B016".into(), direction: ParamDirection::High, threshold: 0.5 },
                ParamCondition { param_id: "D040".into(), direction: ParamDirection::High, threshold: 0.5 },
            ],
            condition_desc: "B016→+1 + D040↑".into(),
            phenomenon: "攻击行为伴随享受——施虐型攻击".into(),
            base_confidence: 0.75,
        },
        CouplingRule {
            id: "impulsive_anger".into(),
            category: CouplingCategory::EmotionBehavior,
            conditions: vec![
                ParamCondition { param_id: "B019".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "C030".into(), direction: ParamDirection::Low, threshold: 0.3 },
            ],
            condition_desc: "B019↑ + C030↓".into(),
            phenomenon: "愤怒→攻击几乎没有缓冲——冲动型暴力".into(),
            base_confidence: 0.85,
        },
        CouplingRule {
            id: "premeditated_anger".into(),
            category: CouplingCategory::EmotionBehavior,
            conditions: vec![
                ParamCondition { param_id: "B019".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "C030".into(), direction: ParamDirection::High, threshold: 0.6 },
            ],
            condition_desc: "B019↑ + C030↑".into(),
            phenomenon: "愤怒后延迟攻击——预谋型暴力".into(),
            base_confidence: 0.80,
        },

        // ===== 动机-行为耦合 =====
        CouplingRule {
            id: "high_power_low_guilt".into(),
            category: CouplingCategory::MotivationBehavior,
            conditions: vec![
                ParamCondition { param_id: "C032".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "B015".into(), direction: ParamDirection::Low, threshold: 0.3 },
            ],
            condition_desc: "C032↑ + B015↓".into(),
            phenomenon: "追求权力且不择手段——马基雅维利型".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "high_power_high_guilt".into(),
            category: CouplingCategory::MotivationBehavior,
            conditions: vec![
                ParamCondition { param_id: "C032".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "B015".into(), direction: ParamDirection::High, threshold: 0.6 },
            ],
            condition_desc: "C032↑ + B015↑".into(),
            phenomenon: "追求权力但自我怀疑——边掌权边内疚".into(),
            base_confidence: 0.75,
        },
        CouplingRule {
            id: "dominate_down_submit_up".into(),
            category: CouplingCategory::MotivationBehavior,
            conditions: vec![
                ParamCondition { param_id: "C031a".into(), direction: ParamDirection::High, threshold: 0.5 },
                ParamCondition { param_id: "C031b".into(), direction: ParamDirection::Low, threshold: -0.3 },
            ],
            condition_desc: "C031a→+1 + C031b→-1".into(),
            phenomenon: "对下支配对上顺从——权力情境依赖型".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "mission_fighter".into(),
            category: CouplingCategory::MotivationBehavior,
            conditions: vec![
                ParamCondition { param_id: "E051".into(), direction: ParamDirection::High, threshold: 0.7 },
                ParamCondition { param_id: "D040".into(), direction: ParamDirection::High, threshold: 0.5 },
            ],
            condition_desc: "E051↑ + D040↑".into(),
            phenomenon: "为使命而战——信念驱动型攻击".into(),
            base_confidence: 0.75,
        },
        CouplingRule {
            id: "mission_ends_justify".into(),
            category: CouplingCategory::MotivationBehavior,
            conditions: vec![
                ParamCondition { param_id: "E051".into(), direction: ParamDirection::High, threshold: 0.7 },
                ParamCondition { param_id: "B015".into(), direction: ParamDirection::Low, threshold: 0.3 },
            ],
            condition_desc: "E051↑ + B015↓".into(),
            phenomenon: "为使命不择手段——目的证明手段正当".into(),
            base_confidence: 0.75,
        },

        // ===== 元认知-情绪耦合 =====
        CouplingRule {
            id: "perfect_self_rationalization".into(),
            category: CouplingCategory::MetacognitionEmotion,
            conditions: vec![
                ParamCondition { param_id: "E048".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "B015".into(), direction: ParamDirection::Low, threshold: 0.3 },
            ],
            condition_desc: "E048↑ + B015↓".into(),
            phenomenon: "完美的自我合理化——任何行为都能被自己接受".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "sincere_grandiosity".into(),
            category: CouplingCategory::MetacognitionEmotion,
            conditions: vec![
                ParamCondition { param_id: "E055".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "E046".into(), direction: ParamDirection::High, threshold: 0.6 },
            ],
            condition_desc: "E055↑ + E046↑".into(),
            phenomenon: "真诚地相信自己是伟大的人——自我欺骗支撑的高外显自尊".into(),
            base_confidence: 0.75,
        },
        CouplingRule {
            id: "rumination_resentment_loop".into(),
            category: CouplingCategory::MetacognitionEmotion,
            conditions: vec![
                ParamCondition { param_id: "E044".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "B022".into(), direction: ParamDirection::High, threshold: 0.7 },
            ],
            condition_desc: "E044↑ + B022→∞".into(),
            phenomenon: "反刍-怨恨循环——反复咀嚼怨恨，怨恨永不消散".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "self_tearing_action".into(),
            category: CouplingCategory::MetacognitionEmotion,
            conditions: vec![
                ParamCondition { param_id: "E053".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "B015".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "D040".into(), direction: ParamDirection::High, threshold: 0.5 },
            ],
            condition_desc: "E053↑ + B015↑ + D040↑".into(),
            phenomenon: "自我撕裂但继续行动——明知不对但停不下来".into(),
            base_confidence: 0.70,
        },

        // ===== 社交-元认知耦合 =====
        CouplingRule {
            id: "performer_self_deception".into(),
            category: CouplingCategory::SocialMetacognition,
            conditions: vec![
                ParamCondition { param_id: "F060a".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "F060b".into(), direction: ParamDirection::Low, threshold: 0.3 },
                ParamCondition { param_id: "E055".into(), direction: ParamDirection::High, threshold: 0.5 },
            ],
            condition_desc: "F060a↑ + F060b↓ + E055↑".into(),
            phenomenon: "公开表演私下随意+自我欺骗——'表演的我就是真正的我'".into(),
            base_confidence: 0.70,
        },
        CouplingRule {
            id: "natural_liar".into(),
            category: CouplingCategory::SocialMetacognition,
            conditions: vec![
                ParamCondition { param_id: "F059".into(), direction: ParamDirection::High, threshold: 0.7 },
                ParamCondition { param_id: "C036".into(), direction: ParamDirection::High, threshold: 0.6 },
            ],
            condition_desc: "F059→1 + C036↑".into(),
            phenomenon: "天生的说谎者——说谎时生理完全平稳且认为欺骗合理".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "self_deception_loop".into(),
            category: CouplingCategory::SocialMetacognition,
            conditions: vec![
                ParamCondition { param_id: "C036c".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "E055".into(), direction: ParamDirection::High, threshold: 0.6 },
            ],
            condition_desc: "C036c↑ + E055↑".into(),
            phenomenon: "自我欺骗的完美闭环——接受自我欺骗且成功欺骗了自己".into(),
            base_confidence: 0.75,
        },

        // ===== 额外耦合规则 =====
        CouplingRule {
            id: "high_empathy_ingroup_low_outgroup".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "A009a".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "A009b".into(), direction: ParamDirection::Low, threshold: 0.4 },
            ],
            condition_desc: "A009a↑ + A009b↓".into(),
            phenomenon: "对内群体痛苦敏感，对外群体痛苦麻木——选择性共情（最普遍的共情模式）".into(),
            base_confidence: 0.85,
        },
        CouplingRule {
            id: "high_empathy_all".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "A009a".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "A009b".into(), direction: ParamDirection::High, threshold: 0.6 },
            ],
            condition_desc: "A009a↑ + A009b↑".into(),
            phenomenon: "对内群体和外群体痛苦都高度敏感——普遍共情".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "observer_type".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "A004".into(), direction: ParamDirection::High, threshold: 0.6 },
                ParamCondition { param_id: "B021".into(), direction: ParamDirection::Low, threshold: 0.3 },
            ],
            condition_desc: "A004↑ + B021↓".into(),
            phenomenon: "高度关注人但不被情绪感染——观察者型".into(),
            base_confidence: 0.75,
        },
        CouplingRule {
            id: "paranoid_processing".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "A008".into(), direction: ParamDirection::High, threshold: 0.7 },
                ParamCondition { param_id: "F062".into(), direction: ParamDirection::High, threshold: 0.7 },
            ],
            condition_desc: "A008↑ + F062↑".into(),
            phenomenon: "偏执型信息处理——高威胁放大+高背叛检测=一切都是针对我的".into(),
            base_confidence: 0.80,
        },
        CouplingRule {
            id: "somatic_risk".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "A003".into(), direction: ParamDirection::Low, threshold: 0.3 },
                ParamCondition { param_id: "B015".into(), direction: ParamDirection::High, threshold: 0.6 },
            ],
            condition_desc: "A003↓ + B015↑".into(),
            phenomenon: "躯体化风险——情绪通过身体表达但意识不到".into(),
            base_confidence: 0.70,
        },
        CouplingRule {
            id: "anhedonia".into(),
            category: CouplingCategory::PerceptionEmotion,
            conditions: vec![
                ParamCondition { param_id: "B011_joy".into(), direction: ParamDirection::High, threshold: 0.7 },
                ParamCondition { param_id: "B018".into(), direction: ParamDirection::Low, threshold: 0.3 },
            ],
            condition_desc: "喜阈↑ + B018↓".into(),
            phenomenon: "快感缺失——难以快乐且快乐转瞬即逝".into(),
            base_confidence: 0.80,
        },
    ]
}
