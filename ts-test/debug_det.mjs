import init, { wasm_generate } from '../crates/paps-wasm/pkg/paps_wasm.js';
import { readFile } from 'node:fs/promises';

const wasmBuffer = await readFile('../crates/paps-wasm/pkg/paps_wasm_bg.wasm');
await init(wasmBuffer);

const a = JSON.parse(wasm_generate(42n));
const b = JSON.parse(wasm_generate(42n));

console.log('ai_text same:', a.ai_text === b.ai_text);
console.log('stats same:', JSON.stringify(a.stats) === JSON.stringify(b.stats));
console.log('traits same:', JSON.stringify(a.traits) === JSON.stringify(b.traits));
console.log('domains same:', JSON.stringify(a.domains) === JSON.stringify(b.domains));
console.log('dormant same:', JSON.stringify(a.dormant_ids) === JSON.stringify(b.dormant_ids));

if (a.ai_text !== b.ai_text) {
    for (let i = 0; i < Math.min(a.ai_text.length, b.ai_text.length); i++) {
        if (a.ai_text[i] !== b.ai_text[i]) {
            console.log('First diff at', i);
            console.log('A:', JSON.stringify(a.ai_text.substring(i-5, i+15)));
            console.log('B:', JSON.stringify(b.ai_text.substring(i-5, i+15)));
            break;
        }
    }
}
