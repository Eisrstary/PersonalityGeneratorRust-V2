// PAPS 完整人格报告 —— AI最优 + 人类简洁
import init, { wasm_generate } from '../crates/paps-wasm/pkg/paps_wasm.js';
import { readFile, writeFile } from 'node:fs/promises';

const wasmBuffer = await readFile('../crates/paps-wasm/pkg/paps_wasm_bg.wasm');
await init(wasmBuffer);

const SEED = 42n;
const data = JSON.parse(wasm_generate(SEED));
const s = data.stats;

// ============================================================
// 构建报告
// ============================================================
let r = '';

// === 头部 ===
r += `PAPS 人格报告 | 种子 ${data.seed}\n`;
r += `${s.total}参数 ${s.active}激活(${pct(s.activation_rate)}) ${s.dormant}休眠\n`;

// === 特征 ===
if (data.traits.length > 0) {
    const traits = data.traits.map(t => `${t.name} ${pct(t.intensity)}`).join(' | ');
    r += `特征: ${traits}\n`;
}

// === 领域激活 ===
const domAct = Object.entries(s.domain_activation)
    .map(([d, v]) => `${d} ${pct(v)}`)
    .join(' ');
r += `领域激活: ${domAct}\n`;

// === 休眠 ===
r += `休眠: ${data.dormant_ids.join(' ')}\n`;

// === 分隔 ===
r += `\n${'─'.repeat(70)}\n`;
r += `详细参数 (norm=归一化值 0-100, 休眠参数已省略)\n`;
r += `${'─'.repeat(70)}\n`;

// === 每个领域：表格 ===
for (const domain of data.domains) {
    const active = domain.params.filter(p => p.label !== 'Dormant');
    const dormant = domain.params.filter(p => p.label === 'Dormant');
    if (active.length === 0) continue;

    r += `\n${domain.name} (${domain.domain})`;
    if (dormant.length > 0) r += `  休眠${dormant.length}: ${dormant.map(p => p.id).join(' ')}`;
    r += '\n';

    // 表头
    r += `  ${pad('参数', 20)} ${pad('值', 5)} ${pad('标签', 6)} 光谱\n`;
    r += `  ${'─'.repeat(20)} ${'─'.repeat(5)} ${'─'.repeat(6)} ${'─'.repeat(40)}\n`;

    for (const p of active) {
        const val = pct(p.norm);
        const label = labelShort(p.label);
        const spectrum = `${p.low_means} ← ${val} → ${p.high_means}`;
        r += `  ${pad(p.name, 20)} ${pad(val, 5)} ${pad(label, 6)} ${spectrum}\n`;
    }
}

// === 摘要 ===
r += `\n${'─'.repeat(70)}\n`;
r += `${data.summary}\n`;

// ============================================================
// 写文件
// ============================================================
const outPath = 'personality_report.txt';
await writeFile(outPath, r, 'utf-8');
console.log(`报告: ${outPath} (${r.length} 字符)`);
console.log(r);

// ============================================================
function pct(n) { return Math.round(n * 100) + '%'; }
function labelShort(l) {
    const m = { VeryHigh: '极高', High: '偏高', Neutral: '中等', Low: '偏低', VeryLow: '极低' };
    return m[l] || l;
}
function pad(s, n) {
    let len = 0;
    for (const c of s) { len += c.charCodeAt(0) > 255 ? 2 : 1; }
    return s + ' '.repeat(Math.max(1, n - len));
}
