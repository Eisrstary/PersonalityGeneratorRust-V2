//! PAPS 人格生成演示程序
//!
//! 展示：种子码生成 → 漂移模拟 → 相变事件 → 关系坍缩 → 耦合推理

use paps_core::*;
use paps_core::drift::DriftConfig;
use paps_core::phase_change::PhaseEventType;
use std::collections::HashMap;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     PAPS 人格原子参数系统 —— 生成演示                          ║");
    println!("║     Personality Atomic Parameter System Demo                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ===== 1. 基本信息 =====
    let reg = ParamRegistry::global();
    println!("【系统信息】");
    println!("  顶层参数: {} 个", reg.top_level_param_ids().len());
    println!("  叶子参数: {} 个", reg.leaf_param_count());
    println!("  领域分布:");
    for d in &[Domain::A, Domain::B, Domain::C, Domain::D, Domain::E, Domain::F, Domain::G, Domain::H] {
        println!("    领域{:?}: {} 个参数", d, reg.by_domain(*d).len());
    }

    // ===== 2. 种子码生成 =====
    let seed = SeedCode::from(42u64);
    println!("\n【种子码】{}", seed.as_u64());

    let gen = PersonalityGenerator::new();
    let profile = gen.generate(seed);

    // ===== 3. 激活统计 =====
    let activated_count = profile.activated_params.len();
    let total = profile.values.len();
    println!("\n【激活统计】");
    println!("  已激活参数: {}/{} ({:.1}%)",
        activated_count, total,
        activated_count as f64 / total as f64 * 100.0);
    println!("  休眠参数: {} (保持中性值)", total - activated_count);

    // ===== 4. 各领域参数概览 =====
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              领域A: 信息摄入                                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    print_domain_summary(&profile, Domain::A);

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              领域B: 情绪生成与调节                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    print_domain_summary(&profile, Domain::B);

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              领域C: 动机与价值                                ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    print_domain_summary(&profile, Domain::C);

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              领域D: 行为执行                                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    print_domain_summary(&profile, Domain::D);

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              领域E: 元认知与自我                                ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    print_domain_summary(&profile, Domain::E);

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              领域F: 社交信号                                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    print_domain_summary(&profile, Domain::F);

    // ===== 5. 漂移模拟 =====
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              漂移模拟: 10年后                                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    let drift_engine = DriftEngine::new();
    let drift_config = DriftConfig {
        years: 10.0,
        ..Default::default()
    };
    let aged_profile = drift_engine.apply_drift(&profile, &drift_config).unwrap();

    // 显示几个关键参数的漂移
    let key_params = vec![
        ("A006", "背景-前景分离效率"),
        ("B011_joy", "喜悦唤醒阈值"),
        ("C030a", "攻击冲动缓冲"),
        ("D042", "行为灵活性"),
        ("B018", "积极情绪维持能力"),
        ("G064b", "负面事件相变阈值"),
    ];
    println!("  {:<20} {:>10} {:>10} {:>10}", "参数", "原始值", "10年后", "变化");
    println!("  {:-<20} {:-<10} {:-<10} {:-<10}", "", "", "", "");
    for (id, name) in &key_params {
        let orig = profile.get(id).unwrap_or(0.0);
        let aged = aged_profile.get(id).unwrap_or(0.0);
        let delta = aged - orig;
        println!("  {:<20} {:>10.4} {:>10.4} {:>+10.4}", name, orig, aged, delta);
    }

    // ===== 6. 相变事件 =====
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              相变事件: 经历背叛后                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    let phase_engine = PhaseChangeEngine::new();
    let betrayed_profile = phase_engine.apply_event(&profile, PhaseEventType::Betrayal).unwrap();

    let betrayal_params = vec![
        ("F061", "信任默认值"),
        ("F061a", "对同类初始信任"),
        ("A008b", "社交威胁放大"),
        ("F062", "背叛检测灵敏度"),
        ("B022a", "对亲近者怨恨半衰期"),
        ("C033a", "深度关系动机"),
    ];
    println!("  {:<20} {:>10} {:>10} {:>10}", "参数", "原始值", "背叛后", "变化");
    println!("  {:-<20} {:-<10} {:-<10} {:-<10}", "", "", "", "");
    for (id, name) in &betrayal_params {
        let orig = profile.get(id).unwrap_or(0.0);
        let after = betrayed_profile.get(id).unwrap_or(0.0);
        let delta = after - orig;
        println!("  {:<20} {:>10.4} {:>10.4} {:>+10.4}", name, orig, after, delta);
    }

    // ===== 7. 关系坍缩 =====
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              关系坍缩: 不同关系中的参数表现                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    let collapse_engine = RelationCollapseEngine::new();
    let collapsed = collapse_engine.collapse(&profile).unwrap();

    let rel_params = vec!["A009a", "B015a", "B021a", "F061", "A008b", "D040b"];
    let rel_names = vec!["内群体痛苦敏感", "内群体内疚", "内群体情绪传染", "信任默认值", "社交威胁放大", "外群体攻击"];

    // 表头
    print!("  {:<16}", "参数");
    for rt in RelationType::all() {
        print!(" {:>8}", rt.name());
    }
    println!();
    print!("  {:-<16}", "");
    for _ in RelationType::all() { print!(" {:-<8}", ""); }
    println!();

    for (i, pid) in rel_params.iter().enumerate() {
        print!("  {:<16}", rel_names[i]);
        for rt in RelationType::all() {
            if let Some(rel_map) = collapsed.relations.get(rt.id()) {
                let val = rel_map.get(*pid).copied().unwrap_or(0.0);
                print!(" {:>8.3}", val);
            }
        }
        println!();
    }

    // ===== 8. 耦合推理 =====
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              耦合推理: 匹配的耦合规则                            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    let coupling_engine = CouplingInferenceEngine::new();
    let report = coupling_engine.analyze(&profile).unwrap();

    println!("  匹配规则数: {}", report.matched_rules.len());
    for (i, rule) in report.matched_rules.iter().enumerate() {
        if i >= 8 { println!("  ... 还有 {} 条", report.matched_rules.len() - 8); break; }
        println!("  [{:.2}] {} → {}", rule.confidence, rule.condition, rule.phenomenon);
    }

    if !report.emergence_patterns.is_empty() {
        println!("\n  【涌现模式】");
        for pattern in &report.emergence_patterns {
            println!("  ◆ {} (置信度: {:.2})", pattern.name, pattern.confidence);
            println!("    {}", pattern.description);
        }
    }

    if !report.contradictory_couplings.is_empty() {
        println!("\n  【矛盾耦合】");
        for cc in &report.contradictory_couplings {
            println!("  ⚠ {} ↔ {} : {}", cc.rule_a, cc.rule_b, cc.description);
        }
    }

    // ===== 9. 随机生成对比 =====
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              随机生成: 3个不同人格的关键参数对比                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    let compare_params = vec![
        ("A003", "内感受分辨率"),
        ("A009a", "内群体痛苦敏感"),
        ("B015a", "内群体内疚"),
        ("B018", "积极情绪维持"),
        ("C032a", "人际权力动机"),
        ("C033a", "深度关系动机"),
        ("E045", "内隐自尊"),
        ("E051", "使命感清晰度"),
        ("F061", "信任默认值"),
        ("D042", "行为灵活性"),
    ];

    print!("  {:<16}", "参数");
    for i in 1..=3 { print!("  {:>10}", format!("人格{}", i)); }
    println!();
    print!("  {:-<16}", "");
    for _ in 1..=3 { print!("  {:-<10}", ""); }
    println!();

    let mut profiles_3 = Vec::new();
    for _ in 0..3 {
        let (s, p) = gen.generate_random();
        profiles_3.push((s, p));
    }

    for (pid, name) in &compare_params {
        print!("  {:<16}", name);
        for (_s, p) in &profiles_3 {
            let val = p.get(pid).unwrap_or(0.0);
            let activated = p.activated_params.contains(&pid.to_string());
            if activated {
                print!("  {:>10.4}", val);
            } else {
                print!("  {:>10}", "【休眠】");
            }
        }
        println!();
    }

    println!("\n  3个人格的种子码: {}, {}, {}",
        profiles_3[0].0.as_u64(), profiles_3[1].0.as_u64(), profiles_3[2].0.as_u64());

    // ===== 10. 预设参数演示 =====
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              部分预设: 固定某些参数，其余随机                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    let mut presets = HashMap::new();
    presets.insert("A009a".to_string(), 0.95); // 对内群体痛苦高度敏感
    presets.insert("B015a".to_string(), 0.10); // 对内群体伤害低内疚
    presets.insert("E051".to_string(), 0.90);  // 高使命感
    presets.insert("C032a".to_string(), 0.85); // 高人际权力动机

    let preset_profile = gen.generate_with_presets(SeedCode::from(99u64), &presets).unwrap();

    println!("  预设参数:");
    for (pid, val) in &presets {
        let spec = reg.get(pid).unwrap();
        println!("    {} ({}): 预设={:.2}, 实际={:.4}", pid, spec.name, val, preset_profile.get(pid).unwrap());
    }

    println!("\n  预设组合的耦合推理:");
    let preset_report = coupling_engine.analyze(&preset_profile).unwrap();
    for rule in &preset_report.matched_rules {
        println!("    [{:.2}] {} → {}", rule.confidence, rule.condition, rule.phenomenon);
    }
    for pattern in &preset_report.emergence_patterns {
        println!("    ◆ {} : {}", pattern.name, pattern.description);
    }

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    演示结束                                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}

fn print_domain_summary(profile: &PersonalityProfile, domain: Domain) {
    let reg = ParamRegistry::global();
    let domain_params = reg.by_domain(domain);
    let mut shown = 0;
    for pid in domain_params {
        if let Some(spec) = reg.get(pid) {
            if spec.is_leaf {
                if let Some(&val) = profile.values.get(pid) {
                    let activated = profile.activated_params.contains(pid);
                    let status = if activated { "激活" } else { "休眠" };
                    let bar = if activated {
                        let pct = (val - spec.value_range.min()) / (spec.value_range.max() - spec.value_range.min());
                        let filled = (pct * 20.0) as usize;
                        format!("[{}{}]", "█".repeat(filled), "░".repeat(20 - filled))
                    } else {
                        "······················".to_string()
                    };
                    println!("  {:>6} {:<20} {:>8.4} {} {}",
                        pid, spec.name, val, bar, status);
                    shown += 1;
                    if shown >= 8 { break; }
                }
            }
        }
    }
}
