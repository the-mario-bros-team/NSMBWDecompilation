#!/usr/bin/env node
// Generate docs/FUNCTION_INDEX.md from ledger/functions.csv.
// Usage: node tools/index-functions.mjs
import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const csvPath = path.join(root, 'ledger', 'functions.csv');
const outPath = path.join(root, 'docs', 'FUNCTION_INDEX.md');

function parseCsv(text) {
  const rows = [];
  let row = [], field = '', quoted = false;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (quoted) {
      if (c === '"') {
        if (text[i + 1] === '"') { field += '"'; i++; }
        else quoted = false;
      } else field += c;
    } else if (c === '"') quoted = true;
    else if (c === ',') { row.push(field); field = ''; }
    else if (c === '\n') { row.push(field); rows.push(row); row = []; field = ''; }
    else if (c !== '\r') field += c;
  }
  if (field.length || row.length) { row.push(field); rows.push(row); }
  return rows.filter((r) => r.length > 1 || r[0] !== '');
}

const rows = parseCsv(fs.readFileSync(csvPath, 'utf8'));
const header = rows[0];
const idx = Object.fromEntries(header.map((h, i) => [h, i]));
const records = rows.slice(1).map((r) => Object.fromEntries(header.map((h) => [h, r[idx[h]] ?? ''])));

const byModule = new Map();
for (const r of records) {
  if (!byModule.has(r.module)) byModule.set(r.module, []);
  byModule.get(r.module).push(r);
}
for (const list of byModule.values()) list.sort((a, b) => parseInt(a.address, 16) - parseInt(b.address, 16));

const esc = (s) => String(s ?? '')
  .replace(/[\u2013\u2014]/g, '-')
  .replace(/\u2026/g, '...')
  .replace(/[^\x20-\x7E]/g, '')
  .replace(/\|/g, '\\|')
  .replace(/\s+/g, ' ')
  .trim();
const short = (s, n = 110) => {
  const t = esc(s);
  return t.length > n ? t.slice(0, n - 3) + '...' : t;
};

const lines = [
  '# Function index',
  '',
  'Generated from `ledger/functions.csv` by `tools/index-functions.mjs`.',
  'This is the human-readable index; the CSV is the machine-readable source.',
  '',
  'Columns: virtual address, recovered/assigned name, translation status, evidence',
  'strength, Rust file, and the original Ghidra evidence. Addresses are virtual',
  '(image base `0x80004000`); see `docs/READING_GUIDE.md` for the confidence keys.',
  '',
];

for (const [module, list] of [...byModule.entries()].sort((a, b) => a[0].localeCompare(b[0]))) {
  const counts = {};
  for (const r of list) counts[r.status] = (counts[r.status] ?? 0) + 1;
  lines.push(`## ${module} (${list.length})`);
  lines.push('');
  lines.push('Status: ' + Object.entries(counts).map(([k, v]) => `${k} ${v}`).join(', '));
  lines.push('');
  lines.push('| Address | Name | Status | Conf. | Rust | Evidence |');
  lines.push('|---|---|---|---|---|---|');
  for (const r of list) {
    lines.push(`| \`0x${r.address.toUpperCase()}\` | \`${esc(r.name)}\` | ${esc(r.status)} | ${esc(r.confidence)} | \`${esc(r.rust_file)}\` | ${short(r.evidence)} |`);
  }
  lines.push('');
}

lines.push('## How to update');
lines.push('');
lines.push('```powershell');
lines.push('node tools/merge-ledger.mjs   # merge lane CSVs -> ledger/functions.csv');
lines.push('node tools/index-functions.mjs # regenerate this index');
lines.push('node tools/doc-lint.mjs src    # readability gate');
lines.push('```');
lines.push('');

fs.writeFileSync(outPath, lines.join('\n'));
console.log(JSON.stringify({ modules: byModule.size, functions: records.length, out: outPath }, null, 2));
