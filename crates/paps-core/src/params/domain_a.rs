use crate::types::{ParameterSpec, Domain, ValueRange, Granularity};

/// 领域A：信息摄入 —— 世界如何进入这个系统
pub fn domain_a_params() -> Vec<ParameterSpec> {
    vec![
        ParameterSpec {
            id: "A001".into(), name: "视觉采样率".into(), domain: Domain::A,
            definition: "单位时间内视觉注意点的切换频率".into(),
            value_range: ValueRange::FloatHz { min: 1.0, max: 10.0 },
            granularity: Granularity::Splittable, is_leaf: false,
            parent_id: None, sub_param_ids: vec!["A001a".into(), "A001b".into()],
            couplings: vec![], collapse_conditions: vec!["威胁情境下：锁定(隧道视觉)或暴涨(过度警觉)——方向取决于A008".into()],
            drift_patterns: vec!["随年龄缓慢下降".into(), "创伤后可能出现永久偏移".into()],
            reversal_conditions: vec!["极度疲劳时：高→零(认知崩溃)".into()],
            age_drift_rate: -0.05, activation_probability: 0.85, is_activated: true,
        },
        ParameterSpec::atomic("A001a", "社交场景采样率", Domain::A, "社交场景中的视觉注意点切换频率", ValueRange::FloatHz { min: 1.0, max: 10.0 }).with_activation(0.85).with_age_drift(-0.04),
        ParameterSpec::atomic("A001b", "非社交场景采样率", Domain::A, "非社交场景中的视觉注意点切换频率", ValueRange::FloatHz { min: 1.0, max: 10.0 }).with_activation(0.95).with_age_drift(-0.06),

        ParameterSpec {
            id: "A002".into(), name: "听觉歧义容忍窗口".into(), domain: Domain::A,
            definition: "对模糊语音/语调保持多解而不急于消歧的持续时间".into(),
            value_range: ValueRange::FloatMs { min: 0.0, max: 10000.0 },
            granularity: Granularity::Splittable, is_leaf: false,
            parent_id: None, sub_param_ids: vec!["A002a".into(), "A002b".into(), "A002c".into()],
            couplings: vec![], collapse_conditions: vec!["被信任者背叛后：A002b/c可能从高永久跳变到零".into()],
            drift_patterns: vec!["随年龄通常上升(经验积累)".into(), "反复背叛后永久下降".into()],
            reversal_conditions: vec![],
            age_drift_rate: 30.0, activation_probability: 0.75, is_activated: true,
        },
        ParameterSpec::atomic("A002a", "对亲近者的容忍", Domain::A, "对亲近者模糊语音/语调的容忍持续时间", ValueRange::FloatMs { min: 0.0, max: 10000.0 }).with_activation(0.85).with_age_drift(35.0),
        ParameterSpec::atomic("A002b", "对陌生人的容忍", Domain::A, "对陌生人模糊语音/语调的容忍持续时间", ValueRange::FloatMs { min: 0.0, max: 10000.0 }).with_activation(0.70).with_age_drift(25.0),
        ParameterSpec::atomic("A002c", "对敌对者的容忍", Domain::A, "对敌对者模糊语音/语调的容忍持续时间", ValueRange::FloatMs { min: 0.0, max: 10000.0 }).with_activation(0.50).with_age_drift(30.0),

        ParameterSpec::atomic("A003", "内感受分辨率", Domain::A, "对自身躯体信号(心跳、呼吸、胃紧、肌肉张力)的觉察精度", ValueRange::Float01)
            .with_activation(0.95).with_coupling(vec!["B020"], "A003↑ + B020↓", "身体知道但无法命名")
            .with_coupling(vec!["B020"], "A003↑ + B020↑", "高情绪颗粒度").with_coupling(vec!["B015"], "A003↓ + B015↑", "躯体化")
            .with_collapse("创伤后：高→零(躯体解离)").with_drift("可通过正念训练提升").with_drift("慢性压力下缓慢下降").with_age_drift(0.0),

        ParameterSpec {
            id: "A004".into(), name: "社会性线索优先级".into(), domain: Domain::A,
            definition: "面孔/注视方向/身体朝向相对于非社会性物体的注意优先级".into(),
            value_range: ValueRange::Float01, granularity: Granularity::Splittable, is_leaf: false,
            parent_id: None, sub_param_ids: vec!["A004a".into(), "A004b".into(), "A004c".into()],
            couplings: vec![], collapse_conditions: vec!["被群体驱逐后：A004a可能崩塌(内群体面孔变为威胁信号)".into()],
            drift_patterns: vec!["孤独长期化后缓慢下降".into()], reversal_conditions: vec![],
            age_drift_rate: -0.002, activation_probability: 0.80, is_activated: true,
        },
        ParameterSpec::atomic("A004a", "对内群体的优先级", Domain::A, "对内群体成员面孔/注视/身体的注意优先级", ValueRange::Float01).with_activation(0.85).with_age_drift(-0.001),
        ParameterSpec::atomic("A004b", "对外群体的优先级", Domain::A, "对外群体成员面孔/注视/身体的注意优先级", ValueRange::Float01).with_activation(0.70).with_age_drift(-0.003),
        ParameterSpec::atomic("A004c", "对威胁面孔的优先级", Domain::A, "对威胁面孔的注意优先级", ValueRange::Float01).with_activation(0.80).with_age_drift(0.001),

        ParameterSpec {
            id: "A005".into(), name: "新异刺激打断阈值".into(), domain: Domain::A,
            definition: "意外刺激使当前注意焦点发生偏移的最小强度".into(),
            value_range: ValueRange::Float01, granularity: Granularity::Splittable, is_leaf: false,
            parent_id: None, sub_param_ids: vec!["A005a".into(), "A005b".into(), "A005c".into()],
            couplings: vec![], collapse_conditions: vec!["安全→威胁切换时：阈值可能从高跳变到极低".into()],
            drift_patterns: vec!["随年龄通常上升(更不容易被打断)".into(), "睡眠剥夺后急剧下降".into()],
            reversal_conditions: vec![], age_drift_rate: 0.003, activation_probability: 0.95, is_activated: true,
        },
        ParameterSpec::atomic("A005a", "听觉打断阈值", Domain::A, "听觉意外刺激打断注意的最小dB强度", ValueRange::FloatDecibel { min: 20.0, max: 80.0 }).with_activation(0.95).with_age_drift(0.15),
        ParameterSpec::atomic("A005b", "视觉打断阈值", Domain::A, "视觉意外刺激打断注意的最小对比度", ValueRange::FloatPercent { min: 5.0, max: 80.0 }).with_activation(0.95).with_age_drift(0.2),
        ParameterSpec::atomic("A005c", "触觉打断阈值", Domain::A, "触觉意外刺激打断注意的最小强度", ValueRange::Float01).with_activation(0.95).with_age_drift(0.002),

        ParameterSpec::atomic("A006", "背景-前景分离效率", Domain::A, "在多声源/多刺激环境中提取目标信息的速度", ValueRange::Float01)
            .with_activation(0.98).with_coupling(vec!["A004"], "A006↑ + A004↑", "在人群中精准锁定一个人的声音")
            .with_coupling(vec!["A002"], "A006↓ + A002↓", "在嘈杂环境中完全无法交流")
            .with_collapse("疲劳时效率急剧下降").with_drift("随年龄缓慢下降").with_drift("音乐训练可提升").with_age_drift(-0.002),

        ParameterSpec {
            id: "A007".into(), name: "预期违背消耗".into(), domain: Domain::A,
            definition: "处理不符合预期的信息时消耗的认知资源比例".into(),
            value_range: ValueRange::FloatPercent { min: 0.0, max: 100.0 },
            granularity: Granularity::Splittable, is_leaf: false,
            parent_id: None, sub_param_ids: vec!["A007a".into(), "A007b".into(), "A007c".into()],
            couplings: vec![], collapse_conditions: vec!["威胁情境下：A007b可能从高跳变到零".into()],
            drift_patterns: vec!["反复经历同类违背后缓慢下降(习惯化)".into()], reversal_conditions: vec![],
            age_drift_rate: -0.1, activation_probability: 0.80, is_activated: true,
        },
        ParameterSpec::atomic("A007a", "对物理世界的预期违背", Domain::A, "物理世界预期违背的认知资源消耗比例", ValueRange::FloatPercent { min: 0.0, max: 100.0 }).with_activation(0.90).with_age_drift(-0.12),
        ParameterSpec::atomic("A007b", "对社交脚本的预期违背", Domain::A, "社交脚本预期违背的认知资源消耗比例", ValueRange::FloatPercent { min: 0.0, max: 100.0 }).with_activation(0.80).with_age_drift(-0.08),
        ParameterSpec::atomic("A007c", "对自我概念的预期违背", Domain::A, "自我概念预期违背的认知资源消耗比例", ValueRange::FloatPercent { min: 0.0, max: 100.0 }).with_activation(0.70).with_coupling(vec!["E051"], "A007c↑ + E051↑", "自我概念受到挑战时认知资源急剧消耗").with_age_drift(-0.05),

        ParameterSpec {
            id: "A008".into(), name: "威胁线索放大系数".into(), domain: Domain::A,
            definition: "将模糊/中性刺激解读为威胁信号的倾向强度".into(),
            value_range: ValueRange::Float01, granularity: Granularity::Splittable, is_leaf: false,
            parent_id: None, sub_param_ids: vec!["A008a".into(), "A008b".into(), "A008c".into()],
            couplings: vec![], collapse_conditions: vec!["安全→威胁切换时：A008可能从低跳变到极高".into()],
            drift_patterns: vec!["长期暴露于真实威胁环境中：A008永久升高".into(), "长期安全环境中：A008缓慢下降".into()],
            reversal_conditions: vec!["在极度恐惧时可能反转：威胁→完全麻木(冻结反应)".into()],
            age_drift_rate: -0.001, activation_probability: 0.80, is_activated: true,
        },
        ParameterSpec::atomic("A008a", "对物理威胁的放大", Domain::A, "将模糊物理刺激解读为威胁的倾向", ValueRange::Float01).with_activation(0.85).with_age_drift(-0.002),
        ParameterSpec::atomic("A008b", "对社交威胁的放大", Domain::A, "将模糊社交信号解读为威胁的倾向", ValueRange::Float01).with_activation(0.80).with_age_drift(0.0),
        ParameterSpec::atomic("A008c", "对符号威胁的放大", Domain::A, "将模糊符号/概念解读为威胁的倾向", ValueRange::Float01).with_activation(0.60).with_age_drift(0.001),

        ParameterSpec {
            id: "A009".into(), name: "痛苦线索敏感度".into(), domain: Domain::A,
            definition: "对他人痛苦表情/声音/姿态的注意捕获强度".into(),
            value_range: ValueRange::Float01, granularity: Granularity::Splittable, is_leaf: false,
            parent_id: None, sub_param_ids: vec!["A009a".into(), "A009b".into(), "A009c".into()],
            couplings: vec![], collapse_conditions: vec!["长期施害后A009c可能从高跳变到零(内疚疲劳→共情麻木)".into()],
            drift_patterns: vec!["反复暴露于他人痛苦而不采取行动：A009缓慢下降(共情疲劳)".into()],
            reversal_conditions: vec!["被受害者反抗时：A009c可能从高→低(愤怒替代共情)".into()],
            age_drift_rate: 0.001, activation_probability: 0.75, is_activated: true,
        },
        ParameterSpec::atomic("A009a", "对内群体痛苦的敏感度", Domain::A, "对内群体成员痛苦线索的注意捕获强度", ValueRange::Float01).with_activation(0.85).with_coupling(vec!["A009b"], "A009a↑ + A009b↓", "选择性共情").with_age_drift(0.002),
        ParameterSpec::atomic("A009b", "对外群体痛苦的敏感度", Domain::A, "对外群体成员痛苦线索的注意捕获强度", ValueRange::Float01).with_activation(0.50).with_age_drift(-0.001),
        ParameterSpec::atomic("A009c", "对施害对象痛苦的敏感度", Domain::A, "对被我伤害的人的痛苦线索的注意捕获强度", ValueRange::Float01).with_activation(0.35).with_coupling(vec!["B015"], "A009c↑ + B015↑", "伤害他人后自我折磨").with_coupling(vec!["B015"], "A009c↓ + B015↓", "伤害他人后无感").with_age_drift(-0.003),

        ParameterSpec::atomic("A010", "猎物/捕食者注意偏向", Domain::A, "注意资源自动流向弱者(猎物)还是强者(捕食者)的倾向", ValueRange::FloatNeg1Pos1)
            .with_activation(0.70).with_coupling(vec!["C031"], "A010→-1 + C031↑", "寻找可保护对象(保护者型)")
            .with_coupling(vec!["C031","C032"], "A010→-1 + C031↓ + C032↑", "寻找可支配对象(掠夺者型)")
            .with_coupling(vec!["B019"], "A010→+1 + B019↑", "在强者面前自卑，在弱者面前发泄")
            .with_coupling(vec!["C034"], "A010→+1 + C034↑", "崇拜强者+渴望成为强者")
            .with_collapse("权力变化时：获得权力后A010可能从+1跳变到-1")
            .with_drift("社会地位上升时缓慢偏向-1").with_drift("社会地位下降时缓慢偏向+1").with_age_drift(0.0),
    ]
}
