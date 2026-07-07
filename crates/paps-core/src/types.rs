use serde::{Deserialize, Serialize};

/// 参数ID类型
pub type ParamId = String;

/// 领域标识
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Domain {
    A, // 信息摄入
    B, // 情绪生成与调节
    C, // 动机与价值
    D, // 行为执行
    E, // 元认知与自我
    F, // 社交信号
    G, // 时间性与发展
    H, // 身体-环境耦合
}

impl Domain {
    pub fn name(&self) -> &'static str {
        match self {
            Domain::A => "信息摄入 —— 世界如何进入这个系统",
            Domain::B => "情绪生成与调节 —— 系统如何生成和调控情感状态",
            Domain::C => "动机与价值 —— 什么驱动系统采取行动",
            Domain::D => "行为执行 —— 系统如何将意图转化为行动",
            Domain::E => "元认知与自我 —— 系统如何观察和定义自己",
            Domain::F => "社交信号 —— 系统如何发送和接收人际信息",
            Domain::G => "时间性与发展 —— 参数如何随时间变化",
            Domain::H => "身体-环境耦合 —— 身体与环境如何交互影响",
        }
    }
}

/// 参数粒度
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Granularity {
    /// 原子级 — 不可再拆
    Atomic,
    /// 分子级 — 多个独立维度但存在耦合（如B011四维情绪阈值）
    Molecular,
    /// 可拆 — 可进一步拆分为子参数
    Splittable,
}

/// 参数值域定义 — 保留原始量纲
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueRange {
    /// [0.0, 1.0] 归一化范围
    Float01,
    /// [-1.0, 1.0] 双向范围
    FloatNeg1Pos1,
    /// 频率 Hz [min, max]
    FloatHz { min: f64, max: f64 },
    /// 时长 ms [min, max]
    FloatMs { min: f64, max: f64 },
    /// 天数 [min, max]
    FloatDays { min: f64, max: f64 },
    /// 分贝 dB [min, max]
    FloatDecibel { min: f64, max: f64 },
    /// 百分比 % [min, max]
    FloatPercent { min: f64, max: f64 },
    /// 计数 [min, max]
    FloatCount { min: f64, max: f64 },
    /// 整数范围 [min, max]
    Int { min: i64, max: i64 },
}

impl ValueRange {
    /// 在值域内生成随机值（由RNG提供随机因子 [0,1)）
    pub fn random_value(&self, rng_factor: f64) -> f64 {
        match self {
            ValueRange::Float01 => rng_factor,
            ValueRange::FloatNeg1Pos1 => rng_factor * 2.0 - 1.0,
            ValueRange::FloatHz { min, max } => min + rng_factor * (max - min),
            ValueRange::FloatMs { min, max } => min + rng_factor * (max - min),
            ValueRange::FloatDays { min, max } => min + rng_factor * (max - min),
            ValueRange::FloatDecibel { min, max } => min + rng_factor * (max - min),
            ValueRange::FloatPercent { min, max } => min + rng_factor * (max - min),
            ValueRange::FloatCount { min, max } => min + rng_factor * (max - min),
            ValueRange::Int { min, max } => {
                let range = (*max - *min) as f64;
                (*min as f64 + rng_factor * range).round()
            }
        }
    }

    /// 检查值是否在值域内
    pub fn contains(&self, value: f64) -> bool {
        match self {
            ValueRange::Float01 => (0.0..=1.0).contains(&value),
            ValueRange::FloatNeg1Pos1 => (-1.0..=1.0).contains(&value),
            ValueRange::FloatHz { min, max } => (*min..=*max).contains(&value),
            ValueRange::FloatMs { min, max } => (*min..=*max).contains(&value),
            ValueRange::FloatDays { min, max } => (*min..=*max).contains(&value),
            ValueRange::FloatDecibel { min, max } => (*min..=*max).contains(&value),
            ValueRange::FloatPercent { min, max } => (*min..=*max).contains(&value),
            ValueRange::FloatCount { min, max } => (*min..=*max).contains(&value),
            ValueRange::Int { min, max } => {
                value >= *min as f64 && value <= *max as f64 && value.fract() == 0.0
            }
        }
    }

    pub fn min(&self) -> f64 {
        match self {
            ValueRange::Float01 => 0.0,
            ValueRange::FloatNeg1Pos1 => -1.0,
            ValueRange::FloatHz { min, .. } => *min,
            ValueRange::FloatMs { min, .. } => *min,
            ValueRange::FloatDays { min, .. } => *min,
            ValueRange::FloatDecibel { min, .. } => *min,
            ValueRange::FloatPercent { min, .. } => *min,
            ValueRange::FloatCount { min, .. } => *min,
            ValueRange::Int { min, .. } => *min as f64,
        }
    }

    pub fn max(&self) -> f64 {
        match self {
            ValueRange::Float01 => 1.0,
            ValueRange::FloatNeg1Pos1 => 1.0,
            ValueRange::FloatHz { max, .. } => *max,
            ValueRange::FloatMs { max, .. } => *max,
            ValueRange::FloatDays { max, .. } => *max,
            ValueRange::FloatDecibel { max, .. } => *max,
            ValueRange::FloatPercent { max, .. } => *max,
            ValueRange::FloatCount { max, .. } => *max,
            ValueRange::Int { max, .. } => *max as f64,
        }
    }

    /// 中性值 —— 值域的中点，用于未激活参数的默认值
    pub fn neutral_value(&self) -> f64 {
        (self.min() + self.max()) / 2.0
    }
}

/// 耦合关系描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingInfo {
    /// 关联的参数ID列表
    pub related_params: Vec<ParamId>,
    /// 耦合条件描述（如 "A009a↑ + A009b↓"）
    pub condition: String,
    /// 现象描述
    pub phenomenon: String,
}

/// 参数规范 — 完整的参数元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterSpec {
    /// 参数ID（如 "A001", "A001a"）
    pub id: ParamId,
    /// 中文名称
    pub name: String,
    /// 所属领域
    pub domain: Domain,
    /// 参数定义
    pub definition: String,
    /// 值域
    pub value_range: ValueRange,
    /// 粒度
    pub granularity: Granularity,
    /// 是否为叶子参数（不可再拆）
    pub is_leaf: bool,
    /// 父参数ID（如果是子参数）
    pub parent_id: Option<ParamId>,
    /// 子参数ID列表（如果是可拆参数）
    pub sub_param_ids: Vec<ParamId>,
    /// 耦合关系
    pub couplings: Vec<CouplingInfo>,
    /// 崩塌条件描述
    pub collapse_conditions: Vec<String>,
    /// 漂移模式描述
    pub drift_patterns: Vec<String>,
    /// 反转条件描述
    pub reversal_conditions: Vec<String>,
    /// 年龄漂移速率（每年变化量，正=上升，负=下降，0=不变）
    pub age_drift_rate: f64,
    /// 激活概率 [0.0, 1.0] —— 该参数在此人格中被"激活"的概率
    ///
    /// 不是每个人都经历过触发该参数的情境。
    /// 1.0 = 几乎所有人都会激活（如基础情绪唤醒阈值）
    /// 0.5 = 约一半人会激活（如对施害对象痛苦的敏感度——取决于是否伤害过别人）
    /// 0.0 = 极少数人会激活（如施虐型愉悦——需要特殊情境）
    /// 未激活的参数取中性值（值域中点），已激活的在值域内随机分布。
    pub activation_probability: f64,
    /// 该参数是否已被激活（运行时状态，不参与序列化比较）
    #[serde(skip)]
    pub is_activated: bool,
}

impl ParameterSpec {
    /// 创建原子级叶子参数
    pub fn atomic(id: &str, name: &str, domain: Domain, definition: &str, value_range: ValueRange) -> Self {
        ParameterSpec {
            id: id.to_string(),
            name: name.to_string(),
            domain,
            definition: definition.to_string(),
            value_range,
            granularity: Granularity::Atomic,
            is_leaf: true,
            parent_id: None,
            sub_param_ids: vec![],
            couplings: vec![],
            collapse_conditions: vec![],
            drift_patterns: vec![],
            reversal_conditions: vec![],
            age_drift_rate: 0.0,
            activation_probability: 1.0,
            is_activated: true,
        }
    }

    /// 设置激活概率（builder模式）
    pub fn with_activation(mut self, prob: f64) -> Self {
        self.activation_probability = prob.clamp(0.0, 1.0);
        self
    }

    /// 添加耦合信息（builder模式）
    pub fn with_coupling(mut self, related_params: Vec<&str>, condition: &str, phenomenon: &str) -> Self {
        self.couplings.push(CouplingInfo {
            related_params: related_params.iter().map(|s| s.to_string()).collect(),
            condition: condition.to_string(),
            phenomenon: phenomenon.to_string(),
        });
        self
    }

    /// 添加崩塌条件
    pub fn with_collapse(mut self, desc: &str) -> Self {
        self.collapse_conditions.push(desc.to_string());
        self
    }

    /// 添加漂移模式
    pub fn with_drift(mut self, desc: &str) -> Self {
        self.drift_patterns.push(desc.to_string());
        self
    }

    /// 添加反转条件
    pub fn with_reversal(mut self, desc: &str) -> Self {
        self.reversal_conditions.push(desc.to_string());
        self
    }

    /// 设置年龄漂移速率
    pub fn with_age_drift(mut self, rate: f64) -> Self {
        self.age_drift_rate = rate;
        self
    }
}
