// PAPS WASM Node.js 测试
import init, { wasm_generate, wasm_generate_random, wasm_generate_with_presets, wasm_leaf_param_count } from '../crates/paps-wasm/pkg/paps_wasm.js';
import { readFile } from 'node:fs/promises';

const wasmBuffer = await readFile('../crates/paps-wasm/pkg/paps_wasm_bg.wasm');
await init(wasmBuffer);

console.log("=".repeat(60));
console.log("PAPS WASM 测试");
console.log("=".repeat(60));

console.log(`\n叶子参数总数: ${wasm_leaf_param_count()}`);

// 种子码 42
console.log("\n--- 种子码 42 ---");
console.log(wasm_generate(42n));

// 确定性
const a = wasm_generate(12345n);
const b = wasm_generate(12345n);
console.log(`\n确定性: ${a === b ? "OK" : "FAIL"}`);

// 种子 1
console.log("\n--- 种子 1 ---");
console.log(wasm_generate(1n));

// 种子 999
console.log("\n--- 种子 999 ---");
console.log(wasm_generate(999n));

// 随机
console.log("\n--- 随机生成 ---");
const rand = wasm_generate_random();
const randObj = JSON.parse(rand);
console.log(`种子: ${randObj.seed}`);
console.log(randObj.profile.substring(0, 500) + "...");

// 预设
console.log("\n--- 预设 E051=0.95 C032a=0.90 A009a=0.10 ---");
console.log(wasm_generate_with_presets(7777n, JSON.stringify({ "E051": 0.95, "C032a": 0.90, "A009a": 0.10 })));

console.log("\n" + "=".repeat(60));
console.log("完成");
