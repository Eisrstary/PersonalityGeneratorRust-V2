use wasm_bindgen::prelude::*;
use paps_core::*;
use std::collections::HashMap;

fn build_full_output(seed: u64, profile: &PersonalityProfile) -> String {
    let ai = profile.to_ai_profile();
    let output = serde_json::json!({
        "seed": seed,
        "ai_text": ai.to_ai_text(),
        "summary": ai.narrative,
        "stats": {
            "total": ai.stats.total_params,
            "active": ai.stats.active_params,
            "dormant": ai.stats.dormant_params,
            "activation_rate": ai.stats.activation_rate,
            "domain_activation": ai.stats.domain_activation,
        },
        "traits": ai.traits.iter().map(|t| serde_json::json!({
            "name": t.trait_name,
            "intensity": t.intensity,
            "evidence": t.evidence,
            "description": t.description,
        })).collect::<Vec<_>>(),
        "domains": ai.domains.iter().map(|d| serde_json::json!({
            "domain": d.domain,
            "name": d.name,
            "params": d.params.iter().map(|p| serde_json::json!({
                "id": p.id,
                "name": p.name,
                "norm": p.norm,
                "label": format!("{:?}", p.label),
                "low_means": p.low_means,
                "high_means": p.high_means,
            })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "dormant_ids": ai.dormant_params,
    });
    serde_json::to_string_pretty(&output).unwrap_or_else(|e| format!("{{\"error\":\"{}\"}}", e))
}

#[wasm_bindgen]
pub fn wasm_generate(seed: u64) -> String {
    let gen = PersonalityGenerator::new();
    let profile = gen.generate(SeedCode::from(seed));
    build_full_output(seed, &profile)
}

#[wasm_bindgen]
pub fn wasm_generate_random() -> String {
    let gen = PersonalityGenerator::new();
    let (seed, profile) = gen.generate_random();
    build_full_output(seed.as_u64(), &profile)
}

#[wasm_bindgen]
pub fn wasm_generate_with_presets(seed: u64, presets_json: &str) -> String {
    let gen = PersonalityGenerator::new();
    let presets: HashMap<String, f64> = match serde_json::from_str(presets_json) {
        Ok(p) => p, Err(e) => return format!("{{\"error\":\"{}\"}}", e),
    };
    match gen.generate_with_presets(SeedCode::from(seed), &presets) {
        Ok(profile) => build_full_output(seed, &profile),
        Err(e) => format!("{{\"error\":\"{}\"}}", e),
    }
}

#[wasm_bindgen] pub fn wasm_leaf_param_count() -> usize { ParamRegistry::global().leaf_param_count() }
