#!/usr/bin/env node
// Render the verified actor base-chain layout as Markdown.
const fs = require('fs');
const LAYOUT = 'C:/Users/Wither/Documents/Codex/2026-11-25/rad/work/phase1/actor-base-layout.json';
const OUT = 'C:/Users/Wither/Documents/Codex/2026-11-25/rad/outputs/phase1-actor-base-layout.md';
const j = JSON.parse(fs.readFileSync(LAYOUT, 'utf8'));

const hx = (n) => '0x' + n.toString(16).toUpperCase().padStart(2, '0');
const ev = (e) => {
  switch (e.kind) {
    case 'store': return `ctor store (${e.width}B)`;
    case 'vec': return `ctor stores (${e.width}B)`;
    case 'vec16': return `ctor stores (${e.width}B, 2B units)`;
    case 'subobject': return `ctor call ${e.call}${e.callOffset != null ? ' at +' + e.callOffset : ''}`;
    case 'zero': return 'zero-init (no ctor store)';
    case 'documented': return 'retail header only (unverified here)';
    default: return e.kind;
  }
};

let md = '';
md += '# Phase 1.3: shared actor base-chain layout\n\n';
md += `Revision: ${j.revision}. Source of truth: constructors decoded from \`${j.dol}\`, not a header. `;
md += 'Every offset marked "ctor store"/"ctor call" is proven by the class constructor; see the gate output.\n\n';
md += '## Chain and a correction\n\n';
md += '```\n';
md += 'fBase_c -> dBase_c -> dBaseActor_c -> dActor_c\n';
md += 'dActor_c -> dActorState_c          (single-state branch)\n';
md += 'dActor_c -> dActorMultiState_c -> dEn_c   (multi-state branch)\n';
md += '```\n\n';
md += '**Correction to the plan:** `dEn_c` does **not** derive from `dActorState_c`. It derives from `dActorMultiState_c`, ' +
  'which is a sibling of `dActorState_c` under `dActor_c`. The constructors prove this: `dEn_c` builds the multi-state ' +
  'manager (two 0x40 vtable blocks at 0x394 and 0x3D4) and its first own field is at 0x414, not after `dActorState_c`.\n\n';

for (const c of j.classes) {
  md += `## ${c.name}\n\n`;
  md += `- Constructor: \`${c.ctor || 'none (inline in dEn_c)'}\``;
  if (c.thisReg) md += `, this = ${c.thisReg}`;
  md += `\n- Object size: ${c.size == null ? 'not proven in this slice' : hx(c.size) + ` (${c.size})`}\n`;
  md += `- vtable pointer: ${c.vptrOffset == null ? 'n/a' : hx(c.vptrOffset)}\n\n`;
  if (c.unresolved) md += `> ${c.unresolved}\n\n`;
  md += '| Offset | Size | Type | Name | Evidence |\n|---|---|---|---|---|\n';
  for (const f of c.fields) {
    const off = f.offset == null ? '?' : hx(f.offset);
    const size = f.size == null ? '?' : hx(f.size);
    md += `| ${off} | ${size} | \`${f.type}\` | ${f.name} | ${ev(f.evidence || { kind: 'documented' })} |\n`;
  }
  md += '\n';
}

md += '## How this was checked\n\n';
md += '`work/phase1/layout-gate.cjs` decodes each constructor from `main.dol`, collects every store whose base register ' +
  'is `this`, and every constructor call whose `r3 = this + offset`, then checks:\n\n';
md += '1. each claimed field has a store of the claimed width at the claimed offset (vectors: every element);\n';
md += '2. each sub-object is constructed at the claimed offset;\n';
md += '3. the vtable pointer is written at the claimed offset;\n';
md += '4. each class size equals the first own field of the next class in the chain;\n';
md += '5. no constructor store is left unexplained before the unresolved tail.\n\n';
md += 'Result: **0 fail, 0 warn**. The `dEn_c` tail from 0x47C on follows the retail `d_enemy.hpp` order but is not ' +
  'claimed as proven here.\n';

fs.writeFileSync(OUT, md);
console.log('wrote', OUT, md.length, 'bytes');
