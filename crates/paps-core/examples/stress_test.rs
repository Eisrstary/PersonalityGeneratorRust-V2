//! PAPS 压力测试 — 10M确定性 + 10M随机，每1M汇报
use paps_core::*;
use std::time::Instant;

const TOTAL: u64 = 10_000_000;
const INTERVAL: u64 = 1_000_000;

fn main() {
    println!("PAPS 压力测试 — {}M 次 x2", TOTAL / 1_000_000);
    println!("============================================================");

    let gen = PersonalityGenerator::new();
    let reg = ParamRegistry::global();
    let leaf_ids: Vec<String> = reg.leaf_param_ids().iter().cloned().collect();

    // ===== 测试1: 确定性 =====
    println!("\n[测试1] 确定性验证");
    let seed = SeedCode::from(42u64);
    let baseline = gen.generate(seed);
    let baseline_vals: Vec<f64> = leaf_ids.iter().map(|id| baseline.values[id]).collect();

    let start = Instant::now();
    let mut fail_count = 0u64;

    for i in 1..=TOTAL {
        let p = gen.generate(seed);
        let mut ok = true;
        for j in 0..leaf_ids.len() {
            if p.values[&leaf_ids[j]] != baseline_vals[j] { ok = false; break; }
        }
        if !ok { fail_count += 1; }
        if i % INTERVAL == 0 {
            let e = start.elapsed().as_secs_f64();
            println!("  {:>3}M | {:.1}s | {:.0}/s | 失败: {}",
                i / 1_000_000, e, i as f64 / e, fail_count);
        }
    }
    let t1 = start.elapsed();
    println!("  完成: {:.1}s {:.0}/s 失败:{}",
        t1.as_secs_f64(), TOTAL as f64 / t1.as_secs_f64(), fail_count);

    // ===== 测试2: 随机 =====
    println!("\n[测试2] 随机穷举");
    let start = Instant::now();
    let mut sum_act = 0u64;
    let mut min_act = usize::MAX;
    let mut max_act = 0usize;

    for i in 1..=TOTAL {
        let (_, p) = gen.generate_random();
        let a = p.activated_params.len();
        sum_act += a as u64;
        if a < min_act { min_act = a; }
        if a > max_act { max_act = a; }
        if i % INTERVAL == 0 {
            let e = start.elapsed().as_secs_f64();
            println!("  {:>3}M | {:.1}s | {:.0}/s | 激活avg:{:.1} range:{}-{}",
                i / 1_000_000, e, i as f64 / e,
                sum_act as f64 / i as f64, min_act, max_act);
        }
    }
    let t2 = start.elapsed();
    let avg = sum_act as f64 / TOTAL as f64;
    println!("  完成: {:.1}s {:.0}/s 激活avg:{:.1} range:{}-{}",
        t2.as_secs_f64(), TOTAL as f64 / t2.as_secs_f64(), avg, min_act, max_act);

    println!("\n============================================================");
    println!("  总耗时: {:.1}s", (t1 + t2).as_secs_f64());
    println!("  确定性: {}", if fail_count == 0 { "PASS" } else { "FAIL" });
    println!("  激活率: {:.1}%", avg / leaf_ids.len() as f64 * 100.0);
    println!("============================================================");
}
