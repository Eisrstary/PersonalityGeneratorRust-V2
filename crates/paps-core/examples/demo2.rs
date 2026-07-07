use paps_core::*;

fn main() {
    let gen = PersonalityGenerator::new();

    // 种子 42 的 AI 文本
    let p = gen.generate(SeedCode::from(42u64));
    let ai = p.to_ai_profile();
    println!("{}", ai.to_ai_text());

    // 确定性
    let a = gen.generate(SeedCode::from(12345u64)).to_ai_text();
    let b = gen.generate(SeedCode::from(12345u64)).to_ai_text();
    println!("\n确定性: {}", if a == b { "OK" } else { "FAIL" });

    // 不同种子
    println!("\n--- 种子 1 ---\n{}", gen.generate(SeedCode::from(1u64)).to_ai_text());
    println!("\n--- 种子 999 ---\n{}", gen.generate(SeedCode::from(999u64)).to_ai_text());

    // 随机
    let (seed, rp) = gen.generate_random();
    println!("\n--- 随机 种子 {} ---\n{}", seed.as_u64(), rp.to_ai_text());

    // 预设
    let mut presets = std::collections::HashMap::new();
    presets.insert("E051".to_string(), 0.95);
    presets.insert("C032a".to_string(), 0.90);
    presets.insert("A009a".to_string(), 0.10);
    let pp = gen.generate_with_presets(SeedCode::from(7777u64), &presets).unwrap();
    println!("\n--- 预设 ---\n{}", pp.to_ai_text());
}
