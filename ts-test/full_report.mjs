// PAPS 完整人格报告 —— 同一种子一次生成，AI文本+详细数据
import init, { wasm_generate } from '../crates/paps-wasm/pkg/paps_wasm.js';
import { readFile, writeFile } from 'node:fs/promises';

const wasmBuffer = await readFile('../crates/paps-wasm/pkg/paps_wasm_bg.wasm');
await init(wasmBuffer);

const SEED = 42n;
const data = JSON.parse(wasm_generate(SEED));

// ============================================================
// 构建报告
// ============================================================
let r = '';

r += '═'.repeat(70) + '\n';
r += `  PAPS 人格原子参数系统 —— 完整报告\n`;
r += `  种子码: ${data.seed}\n`;
r += '═'.repeat(70) + '\n\n';

// --- 统计 ---
const s = data.stats;
r += '┌─ 统计 ─────────────────────────────────────────────┐\n';
r += `│ 总参数: ${String(s.total).padStart(4)}  激活: ${String(s.active).padStart(4)} (${Math.round(s.activation_rate*100)}%)  休眠: ${String(s.dormant).padStart(4)}\n`;
r += '├────────────────────────────────────────────────────┤\n';
r += '│ 领域激活率: ';
for (const [d, v] of Object.entries(s.domain_activation)) {
    r += `${d}:${Math.round(v*100)}% `;
}
r += '\n└────────────────────────────────────────────────────┘\n\n';

// --- 特征 ---
if (data.traits.length > 0) {
    r += '┌─ 推断特征 ─────────────────────────────────────────┐\n';
    for (const t of data.traits) {
        r += `│ ◆ ${t.name} (强度 ${Math.round(t.intensity*100)}%)\n`;
        r += `│   ${t.description}\n`;
        r += `│   证据: ${t.evidence.join(', ')}\n`;
    }
    r += '└────────────────────────────────────────────────────┘\n\n';
}

// --- AI 文本 ---
r += '┌─ AI 紧凑视图 ──────────────────────────────────────┐\n';
const aiLines = data.ai_text.split('\n').filter(l => l.trim());
for (const line of aiLines) {
    if (line.length <= 60) {
        r += `│ ${line}\n`;
    } else {
        // 折行
        let remain = line;
        while (remain.length > 0) {
            const chunk = remain.substring(0, 57);
            r += `│ ${chunk}\n`;
            remain = remain.substring(57);
        }
    }
}
r += '└────────────────────────────────────────────────────┘\n\n';

// --- 详细参数 (每个领域一个表格) ---
for (const domain of data.domains) {
    const active = domain.params.filter(p => p.label !== 'Dormant');
    const dormant = domain.params.filter(p => p.label === 'Dormant');
    if (active.length === 0 && dormant.length === 0) continue;

    r += `┌─ ${domain.name} (${domain.domain}) ─${'─'.repeat(48 - domain.name.length)}┐\n`;

    // 激活的参数
    for (const p of active) {
        const bar = makeBar(p.norm);
        const label = labelName(p.label);
        r += `│ ${p.id} ${padR(p.name, 18)} ${bar} ${padR(label, 8)} ${padL(p.norm.toFixed(2), 5)}  ${p.low_means} ↔ ${p.high_means}\n`;
    }

    // 休眠参数
    if (dormant.length > 0) {
        const dormantIds = dormant.map(p => p.id).join(' ');
        r += `│ ── 休眠(${dormant.length}): ${dormantIds}\n`;
    }

    r += `└${'─'.repeat(56)}┘\n\n`;
}

// --- 休眠汇总 ---
r += `休眠参数汇总 (${data.dormant_ids.length}): ${data.dormant_ids.join(' ')}\n\n`;

// --- 摘要 ---
r += `摘要: ${data.summary}\n`;

// ============================================================
// 写文件
// ============================================================
const outPath = 'personality_full_report.txt';
await writeFile(outPath, r, 'utf-8');
console.log(`报告已写入: ${outPath} (${r.length} 字符)`);
console.log(r.substring(0, 2500));

// ============================================================
// 辅助
// ============================================================
function makeBar(norm) {
    const filled = Math.round(norm * 10);
    return '█'.repeat(filled) + '░'.repeat(10 - filled);
}
function labelName(l) {
    const map = { VeryHigh: '极高↑↑', High: '偏高↑', Neutral: '中等→', Low: '偏低↓', VeryLow: '极低↓↓', Dormant: '休眠' };
    return map[l] || l;
}
function padR(s, n) {
    let len = 0;
    for (const c of s) { len += c.charCodeAt(0) > 255 ? 2 : 1; }
    return s + ' '.repeat(Math.max(0, n - len));
}
function padL(s, n) { return ' '.repeat(Math.max(0, n - s.length)) + s; }
