use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::params::ParamRegistry;
use crate::types::Domain;

use crate::profile::DriftRecord;

// ============================================================
// AI优化输出格式
// 设计原则：
//   1. 每个参数只有关键字段（id, 中文名, 0-1归一化值, 语义标签, 激活状态）
//   2. 按领域分组，领域内按光谱极端程度排序（AI最关心极端值）
//   3. 提供 flattened 视图：所有参数扁平化为一个数组
//   4. 语义标签用枚举值，方便AI做条件判断
//   5. 顶层提供统计摘要和推断特征
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpectrumLabel {
    #[serde(rename = "very_low")] VeryLow,
    #[serde(rename = "low")] Low,
    #[serde(rename = "neutral")] Neutral,
    #[serde(rename = "high")] High,
    #[serde(rename = "very_high")] VeryHigh,
    #[serde(rename = "dormant")] Dormant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamSnap {
    pub id: String,
    pub name: String,
    /// 归一化值 [0.0, 1.0]，休眠参数为0.5
    pub norm: f64,
    pub label: SpectrumLabel,
    /// 光谱低端含义
    pub low_means: String,
    /// 光谱高端含义
    pub high_means: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainSnap {
    pub domain: String,
    pub name: String,
    /// 按 |norm-0.5| 降序排列（极端值在前）
    pub params: Vec<ParamSnap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitSummary {
    pub trait_name: String,
    /// 强度 0-1
    pub intensity: f64,
    pub evidence: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProfile {
    pub seed: u64,
    pub created_at: String,
    pub stats: ProfileStats,
    /// 按领域分组（极端值优先）
    pub domains: Vec<DomainSnap>,
    /// 所有参数扁平化
    pub all_params: Vec<ParamSnap>,
    /// 极端参数（|norm-0.5| > 0.35）
    pub extreme_params: Vec<ParamSnap>,
    /// 休眠参数ID列表
    pub dormant_params: Vec<String>,
    /// 推断的人格特征
    pub traits: Vec<TraitSummary>,
    /// 自然语言描述
    pub narrative: String,
}

impl AiProfile {
    /// 生成AI最优阅读格式：高密度结构化自然语言
    ///
    /// 设计原则：
    ///   - 无JSON语法噪音，纯文本
    ///   - 信息按重要性降序排列
    ///   - 极端值用简短标签标注，中性值不列出
    ///   - 休眠参数仅列出ID，不占篇幅
    ///   - 每行一个信息点，AI可逐行扫描
    pub fn to_ai_text(&self) -> String {
        let mut out = String::new();

        // === 头部：一句话摘要 ===
        out.push_str(&format!("SEED {} | {}参数 {}激活({:.0}%) {}休眠\n",
            self.seed, self.stats.total_params, self.stats.active_params,
            self.stats.activation_rate * 100.0, self.stats.dormant_params));

        // === 特征 ===
        if !self.traits.is_empty() {
            out.push_str("TRAITS");
            for t in &self.traits {
                out.push_str(&format!(" | {}:{}", t.trait_name, (t.intensity * 100.0) as u8));
            }
            out.push('\n');
        }

        // === 各领域极端参数（只列偏离中性的） ===
        for domain in &self.domains {
            let significant: Vec<&ParamSnap> = domain.params.iter()
                .filter(|p| p.label != SpectrumLabel::Neutral && p.label != SpectrumLabel::Dormant)
                .collect();
            if significant.is_empty() { continue; }

            out.push_str(&format!("[{}]", domain.name));
            for p in &significant {
                out.push_str(&format!(" {}={}", p.name, label_short(p.label)));
            }
            out.push('\n');
        }

        // === 休眠参数（紧凑列表，排序确保确定性） ===
        if !self.dormant_params.is_empty() {
            let mut sorted_dormant = self.dormant_params.clone();
            sorted_dormant.sort();
            out.push_str(&format!("DORMANT({})", sorted_dormant.len()));
            for id in sorted_dormant.iter().take(20) {
                out.push(' ');
                out.push_str(id);
            }
            if sorted_dormant.len() > 20 {
                out.push_str(&format!(" ...+{}", sorted_dormant.len() - 20));
            }
            out.push('\n');
        }

        // === 领域激活率 (按字母序排列确保确定性) ===
        out.push_str("ACTIVATION");
        let mut domains_sorted: Vec<(&String, &f64)> = self.stats.domain_activation.iter().collect();
        domains_sorted.sort_by(|a, b| a.0.cmp(b.0));
        for (domain, rate) in &domains_sorted {
            out.push_str(&format!(" {}:{:.0}%", domain, *rate * 100.0));
        }
        out.push('\n');

        // === 叙事 ===
        out.push_str(&format!("NARRATIVE {}\n", self.narrative));

        out
    }
}

/// 简短标签：↓↓ ↓ → ↑ ↑↑ ⊘
fn label_short(label: SpectrumLabel) -> &'static str {
    match label {
        SpectrumLabel::VeryLow => "↓↓",
        SpectrumLabel::Low => "↓",
        SpectrumLabel::Neutral => "→",
        SpectrumLabel::High => "↑",
        SpectrumLabel::VeryHigh => "↑↑",
        SpectrumLabel::Dormant => "⊘",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileStats {
    pub total_params: usize,
    pub active_params: usize,
    pub dormant_params: usize,
    pub activation_rate: f64,
    pub domain_activation: HashMap<String, f64>,
}

// ============================================================
// 构建
// ============================================================

pub fn build_ai_profile(
    seed: u64,
    created_at_ms: i64,
    values: &HashMap<String, f64>,
    activated_params: &[String],
    _drift_history: Option<Vec<DriftRecord>>,
    _phase_events: &[String],
    _reversed_params: &[String],
) -> AiProfile {
    let reg = ParamRegistry::global();
    let activated_set: std::collections::HashSet<&str> =
        activated_params.iter().map(|s| s.as_str()).collect();

    let all_domains = vec![
        (Domain::A, "A", "信息摄入"),
        (Domain::B, "B", "情绪生成与调节"),
        (Domain::C, "C", "动机与价值"),
        (Domain::D, "D", "行为执行"),
        (Domain::E, "E", "元认知与自我"),
        (Domain::F, "F", "社交信号"),
        (Domain::G, "G", "时间性与发展"),
        (Domain::H, "H", "身体-环境耦合"),
    ];

    let mut all_params: Vec<ParamSnap> = Vec::new();
    let mut domains: Vec<DomainSnap> = Vec::new();
    let mut domain_activation: HashMap<String, f64> = HashMap::new();

    for (domain, code, name) in &all_domains {
        let domain_param_ids = reg.by_domain(*domain);
        let mut snaps: Vec<ParamSnap> = Vec::new();
        let mut active_count = 0;
        let mut total_count = 0;

        for pid in domain_param_ids {
            if let Some(spec) = reg.get(pid) {
                if !spec.is_leaf { continue; }
                total_count += 1;
                if let Some(&value) = values.get(pid) {
                    let is_active = activated_set.contains(pid.as_str());
                    if is_active { active_count += 1; }

                    let (norm, label) = if !is_active {
                        (0.5, SpectrumLabel::Dormant)
                    } else {
                        let n = normalize(value, spec.value_range.min(), spec.value_range.max());
                        let l = classify(n);
                        (n, l)
                    };

                    let (low_means, high_means) = parse_spectrum_labels(&spec.definition);

                    snaps.push(ParamSnap {
                        id: pid.clone(), name: spec.name.clone(), norm, label, low_means, high_means,
                    });
                }
            }
        }

        snaps.sort_by(|a, b| {
            let ea = (a.norm - 0.5).abs();
            let eb = (b.norm - 0.5).abs();
            eb.partial_cmp(&ea).unwrap_or(std::cmp::Ordering::Equal)
        });

        if total_count > 0 {
            domain_activation.insert(code.to_string(), active_count as f64 / total_count as f64);
        }

        all_params.extend(snaps.clone());
        domains.push(DomainSnap { domain: code.to_string(), name: name.to_string(), params: snaps });
    }

    let extreme_params: Vec<ParamSnap> = all_params.iter()
        .filter(|p| (p.norm - 0.5).abs() > 0.35)
        .cloned().collect();

    let dormant_params: Vec<String> = all_params.iter()
        .filter(|p| p.label == SpectrumLabel::Dormant)
        .map(|p| p.id.clone()).collect();

    let total_params = all_params.len();
    let active_params_count = total_params - dormant_params.len();
    let stats = ProfileStats {
        total_params,
        active_params: active_params_count,
        dormant_params: dormant_params.len(),
        activation_rate: if total_params > 0 { active_params_count as f64 / total_params as f64 } else { 0.0 },
        domain_activation,
    };

    let traits = infer_traits(&all_params);
    let narrative = build_narrative(&traits, &stats, seed);

    let created_at = chrono::DateTime::from_timestamp_millis(created_at_ms)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "unknown".to_string());

    AiProfile { seed, created_at, stats, domains, all_params, extreme_params, dormant_params, traits, narrative }
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    if (max - min).abs() < 1e-10 { return 0.5; }
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}

fn classify(norm: f64) -> SpectrumLabel {
    if norm < 0.15 { SpectrumLabel::VeryLow }
    else if norm < 0.35 { SpectrumLabel::Low }
    else if norm < 0.65 { SpectrumLabel::Neutral }
    else if norm < 0.85 { SpectrumLabel::High }
    else { SpectrumLabel::VeryHigh }
}

fn parse_spectrum_labels(definition: &str) -> (String, String) {
    // 尝试从定义中提取 "A ←→ B" 格式的光谱
    if let Some(pos) = definition.find("←→") {
        let left = definition[..pos].trim();
        let right = definition[pos + "←→".len()..].trim();
        // 清理：去掉开头的括号内容如 "(0)" "(1)" "(-1)" "(+1)"
        let clean = |s: &str| -> String {
            let s = s.trim();
            // 去掉末尾的括号说明
            let s = if let Some(p) = s.rfind('(') {
                s[..p].trim()
            } else { s };
            s.to_string()
        };
        return (clean(left), clean(right));
    }
    ("低".to_string(), "高".to_string())
}

fn infer_traits(all_params: &[ParamSnap]) -> Vec<TraitSummary> {
    let mut traits = Vec::new();
    let find = |id: &str| -> Option<&ParamSnap> { all_params.iter().find(|p| p.id == id) };

    // 共情模式
    let in_emp = find("A009a").map(|p| p.norm).unwrap_or(0.5);
    let out_emp = find("A009b").map(|p| p.norm).unwrap_or(0.5);
    if in_emp > 0.6 && out_emp < 0.4 {
        traits.push(TraitSummary { trait_name: "选择性共情".into(), intensity: (in_emp - out_emp).clamp(0.0, 1.0), evidence: vec!["A009a".into(),"A009b".into()], description: "对内群体痛苦敏感，对外群体痛苦麻木——人类最普遍的共情模式".into() });
    } else if in_emp > 0.6 && out_emp > 0.6 {
        traits.push(TraitSummary { trait_name: "普遍共情".into(), intensity: (in_emp + out_emp) / 2.0, evidence: vec!["A009a".into(),"A009b".into()], description: "对内群体和外群体痛苦都高度敏感".into() });
    }

    // 威胁敏感
    let threat = find("A008b").map(|p| p.norm).unwrap_or(0.5);
    let betrayal = find("F062").map(|p| p.norm).unwrap_or(0.5);
    if threat > 0.65 && betrayal > 0.65 {
        traits.push(TraitSummary { trait_name: "偏执型警觉".into(), intensity: (threat + betrayal) / 2.0, evidence: vec!["A008b".into(),"F062".into()], description: "高度警觉社交威胁+高背叛检测".into() });
    } else if threat < 0.35 && betrayal < 0.35 {
        traits.push(TraitSummary { trait_name: "社交安全感".into(), intensity: 1.0 - (threat + betrayal) / 2.0, evidence: vec!["A008b".into(),"F062".into()], description: "低威胁感知+低背叛检测——社交中感到安全".into() });
    }

    // 冲动控制
    let anger_act = find("B019c").map(|p| p.norm).unwrap_or(0.5);
    let impulse_buf = find("C030c").map(|p| p.norm).unwrap_or(0.5);
    if anger_act > 0.65 && impulse_buf < 0.35 {
        traits.push(TraitSummary { trait_name: "言语冲动型".into(), intensity: (anger_act + (1.0 - impulse_buf)) / 2.0, evidence: vec!["B019c".into(),"C030c".into()], description: "愤怒时容易说出伤人的话，言语冲动缓冲不足".into() });
    }

    // 自我认知
    let implicit_self = find("E045").map(|p| p.norm).unwrap_or(0.5);
    let self_deception = find("E055").map(|p| p.norm).unwrap_or(0.5);
    if implicit_self > 0.7 && self_deception > 0.6 {
        traits.push(TraitSummary { trait_name: "真诚的自恋倾向".into(), intensity: (implicit_self + self_deception) / 2.0, evidence: vec!["E045".into(),"E055".into()], description: "高内隐自尊+高自我欺骗——真诚地相信自己很优秀".into() });
    }

    // 权力动机
    let power = find("C032a").map(|p| p.norm).unwrap_or(0.5);
    let dominance = find("C031a").map(|p| p.norm).unwrap_or(0.5);
    if power > 0.65 && dominance > 0.65 {
        traits.push(TraitSummary { trait_name: "支配型权力驱动".into(), intensity: (power + dominance) / 2.0, evidence: vec!["C032a".into(),"C031a".into()], description: "渴望影响他人+对下位者支配倾向".into() });
    }

    // 使命感
    let mission = find("E051").map(|p| p.norm).unwrap_or(0.5);
    if mission > 0.7 {
        traits.push(TraitSummary { trait_name: "使命驱动".into(), intensity: mission, evidence: vec!["E051".into()], description: "对'为何而活'有清晰答案".into() });
    } else if mission < 0.25 {
        traits.push(TraitSummary { trait_name: "存在性迷茫".into(), intensity: 1.0 - mission, evidence: vec!["E051".into()], description: "对人生意义缺乏清晰答案".into() });
    }

    // 社交倾向
    let social_approach = find("C025b").map(|p| p.norm).unwrap_or(0.5);
    let intimacy = find("C033a").map(|p| p.norm).unwrap_or(0.5);
    if social_approach > 0.65 && intimacy > 0.65 {
        traits.push(TraitSummary { trait_name: "社交亲和型".into(), intensity: (social_approach + intimacy) / 2.0, evidence: vec!["C025b".into(),"C033a".into()], description: "主动趋近社交+渴望深度关系".into() });
    } else if social_approach < 0.35 && intimacy < 0.35 {
        traits.push(TraitSummary { trait_name: "社交疏离型".into(), intensity: 1.0 - (social_approach + intimacy) / 2.0, evidence: vec!["C025b".into(),"C033a".into()], description: "回避社交+低深度关系动机——独处更舒适".into() });
    }

    traits.sort_by(|a, b| b.intensity.partial_cmp(&a.intensity).unwrap());
    traits
}

fn build_narrative(traits: &[TraitSummary], stats: &ProfileStats, seed: u64) -> String {
    let mut n = format!(
        "种子码{}。{}个参数中{}个激活({:.0}%)。",
        seed, stats.total_params, stats.active_params, stats.activation_rate * 100.0
    );
    if traits.is_empty() {
        n.push_str("该人格光谱较为均衡，无明显极端特征。");
    } else {
        n.push_str("核心特征：");
        let names: Vec<String> = traits.iter().take(5).map(|t| format!("{}（强度{:.0}%）", t.trait_name, t.intensity * 100.0)).collect();
        n.push_str(&names.join("；"));
        n.push('。');
    }
    // 排序确保确定性
    let mut high: Vec<_> = stats.domain_activation.iter().filter(|(_,&r)| r > 0.8).map(|(d,_)| d.clone()).collect();
    high.sort();
    let mut low: Vec<_> = stats.domain_activation.iter().filter(|(_,&r)| r < 0.5).map(|(d,_)| d.clone()).collect();
    low.sort();
    if !high.is_empty() { n.push_str(&format!("高激活领域：{}。", high.join("、"))); }
    if !low.is_empty() { n.push_str(&format!("低激活领域：{}。", low.join("、"))); }
    n
}
