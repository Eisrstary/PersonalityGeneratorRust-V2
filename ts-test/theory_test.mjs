// PAPS WASM 理论验证测试
import init, { wasm_generate, wasm_generate_random, wasm_leaf_param_count } from '../crates/paps-wasm/pkg/paps_wasm.js';
import { readFile } from 'node:fs/promises';

const wasmBuffer = await readFile('../crates/paps-wasm/pkg/paps_wasm_bg.wasm');
await init(wasmBuffer);

console.log("PAPS WASM 理论验证");
console.log("=".repeat(50));

// 测试1: 确定性
console.log("\n[测试1] 确定性");
const a = JSON.parse(wasm_generate(42n));
const b = JSON.parse(wasm_generate(42n));
const same = a.ai_text === b.ai_text && a.stats.active === b.stats.active;
console.log(`  同种子一致: ${same ? 'PASS' : 'FAIL'}`);

// 测试2: 参数数量
console.log("\n[测试2] 参数数量");
const leafCount = wasm_leaf_param_count();
console.log(`  叶子参数: ${leafCount} (预期196): ${leafCount === 196 ? 'PASS' : 'FAIL'}`);

// 测试3: 数据结构完整性
console.log("\n[测试3] 数据结构");
const d = JSON.parse(wasm_generate(1n));
const checks = [
    ['seed', typeof d.seed === 'number'],
    ['ai_text', typeof d.ai_text === 'string' && d.ai_text.length > 100],
    ['summary', typeof d.summary === 'string' && d.summary.length > 10],
    ['stats', d.stats && d.stats.total === 196],
    ['traits', Array.isArray(d.traits)],
    ['domains', Array.isArray(d.domains) && d.domains.length === 8],
    ['dormant_ids', Array.isArray(d.dormant_ids)],
];
let allOk = true;
for (const [name, ok] of checks) {
    console.log(`  ${name}: ${ok ? 'PASS' : 'FAIL'}`);
    if (!ok) allOk = false;
}

// 测试4: 随机生成
console.log("\n[测试4] 随机生成");
const rand = JSON.parse(wasm_generate_random());
console.log(`  种子: ${rand.seed} (非0): ${rand.seed !== 0 ? 'PASS' : 'FAIL'}`);
console.log(`  结构完整: ${rand.ai_text && rand.stats ? 'PASS' : 'FAIL'}`);

// 测试5: 蒙特卡洛 (1K样本, WASM较慢)
console.log("\n[测试5] 蒙特卡洛 1K样本");
let sumAct = 0, minAct = 999, maxAct = 0;
for (let i = 0; i < 1000; i++) {
    const p = JSON.parse(wasm_generate_random());
    const act = p.stats.active;
    sumAct += act;
    if (act < minAct) minAct = act;
    if (act > maxAct) maxAct = act;
}
const avg = sumAct / 1000;
console.log(`  激活: avg=${avg.toFixed(1)} range=${minAct}-${maxAct}`);
console.log(`  激活率: ${(avg/196*100).toFixed(1)}%`);
const rateOk = avg/196 > 0.70 && avg/196 < 0.82;
console.log(`  激活率合理: ${rateOk ? 'PASS' : 'FAIL'}`);

// 测试6: AI文本格式
console.log("\n[测试6] AI文本格式");
const text = d.ai_text;
const hasSeed = text.startsWith('SEED ');
const hasTraits = text.includes('TRAITS');
const hasDomain = text.includes('[信息摄入]');
const hasDormant = text.includes('DORMANT');
const hasActivation = text.includes('ACTIVATION');
const hasNarrative = text.includes('NARRATIVE');
console.log(`  SEED行: ${hasSeed ? 'PASS' : 'FAIL'}`);
console.log(`  TRAITS行: ${hasTraits ? 'PASS' : 'FAIL'}`);
console.log(`  领域分组: ${hasDomain ? 'PASS' : 'FAIL'}`);
console.log(`  DORMANT: ${hasDormant ? 'PASS' : 'FAIL'}`);
console.log(`  ACTIVATION: ${hasActivation ? 'PASS' : 'FAIL'}`);
console.log(`  NARRATIVE: ${hasNarrative ? 'PASS' : 'FAIL'}`);

console.log("\n" + "=".repeat(50));
console.log("WASM 理论验证完成");
