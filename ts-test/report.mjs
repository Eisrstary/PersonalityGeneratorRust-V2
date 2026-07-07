// PAPS 完整人格报告生成器
// 输出：AI文本摘要 + 人类可读详细表格 + 完整JSON数据
import init, { wasm_generate, wasm_generate_random, wasm_leaf_param_count } from '../crates/paps-wasm/pkg/paps_wasm.js';
import { readFile, writeFile } from 'node:fs/promises';

const wasmBuffer = await readFile('../crates/paps-wasm/pkg/paps_wasm_bg.wasm');
await init(wasmBuffer);

// ============================================================
// 生成3份不同的人格报告
// ============================================================
const seeds = [42n, 1n, 999n];
const reports = [];

for (const seed of seeds) {
    const aiText = wasm_generate(seed);
    reports.push({ seed: Number(seed), text: aiText });
}

// 加一份随机
const randJson = wasm_generate_random();
const rand = JSON.parse(randJson);
console.log('随机返回类型:', typeof rand.profile, rand.profile?.substring?.(0, 80));
reports.push({ seed: Number(rand.seed), text: String(rand.profile) });

// ============================================================
// 输出：AI可读 + 人类可读
// ============================================================
let output = '';

for (const r of reports) {
    output += `\n${'═'.repeat(70)}\n`;
    output += `种子码: ${r.seed}\n`;
    output += `${'═'.repeat(70)}\n\n`;

    // 解析 AI 文本的各部分
    const lines = r.text.split('\n').filter(l => l.trim());

    for (const line of lines) {
        if (line.startsWith('SEED ')) {
            const m = line.match(/SEED (\d+) \| (\d+)参数 (\d+)激活\((\d+)%\) (\d+)休眠/);
            if (m) {
                output += `┌─ 概览 ─────────────────────────────────────────────┐\n`;
                output += `│ 总参数: ${m[2].padStart(4)}  激活: ${m[3].padStart(4)} (${m[4]}%)  休眠: ${m[5].padStart(4)}\n`;
                output += `└────────────────────────────────────────────────────┘\n\n`;
            }
        } else if (line.startsWith('TRAITS ')) {
            const traits = line.replace('TRAITS | ', '');
            output += `◆ 推断特征: ${traits}\n\n`;
        } else if (line.startsWith('[')) {
            // 领域标题
            const domain = line.match(/\[(.+?)\]/)?.[1] || '';
            const params = line.replace(/\[.+?\]\s*/, '');
            output += `┌─ ${domain} ─${'─'.repeat(55 - domain.length)}┐\n`;

            // 解析每个参数: 名称=标签
            const entries = params.split(' ');
            let row = '';
            for (const e of entries) {
                if (!e) continue;
                const eq = e.indexOf('=');
                if (eq < 0) continue;
                const name = e.substring(0, eq);
                const label = e.substring(eq + 1);
                const item = `${name}:${label}`;
                if (row && (row.length + item.length > 55)) {
                    output += `│ ${row.trim()}\n`;
                    row = '';
                }
                row += item + ' ';
            }
            if (row.trim()) output += `│ ${row.trim()}\n`;
            output += `└${'─'.repeat(56)}┘\n\n`;

        } else if (line.startsWith('DORMANT(')) {
            output += `◇ ${line}\n\n`;
        } else if (line.startsWith('ACTIVATION ')) {
            const acts = line.replace('ACTIVATION ', '');
            output += `◆ 领域激活率: ${acts}\n\n`;
        } else if (line.startsWith('NARRATIVE ')) {
            output += `◆ ${line.replace('NARRATIVE ', '')}\n\n`;
        }
    }
}

// ============================================================
// 写文件
// ============================================================
const outPath = 'personality_report.txt';
await writeFile(outPath, output, 'utf-8');
console.log(`报告已写入: ${outPath}`);
console.log(`共 ${reports.length} 个人格`);
console.log(output.substring(0, 2000));
