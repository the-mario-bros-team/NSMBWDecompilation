#!/usr/bin/env node
// Generate docs/COVERAGE.md from ledger/functions.csv.
// Usage: node tools/coverage-doc.mjs
import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const csvPath = path.join(root, 'ledger', 'functions.csv');
const outPath = path.join(root, 'docs', 'COVERAGE.md');

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

const groups = new Map();
for (const r of records) {
  const key = r.module || 'unknown';
  if (!groups.has(key)) groups.set(key, { total: 0, translated: 0, partial: 0, stub: 0, high: 0, medium: 0, low: 0 });
  const g = groups.get(key);
  g.total++;
  if (g[r.status] !== undefined) g[r.status]++;
  if (g[r.confidence] !== undefined) g[r.confidence]++;
}

const staging = [...groups.entries()].filter(([m]) => m.startsWith('subsystems::')).sort((a, b) => a[0].localeCompare(b[0]));
const semantic = [...groups.entries()].filter(([m]) => !m.startsWith('subsystems::')).sort((a, b) => a[0].localeCompare(b[0]));
const sum = (list) => list.reduce((a, [, g]) => ({
  total: a.total + g.total,
  translated: a.translated + g.translated,
  partial: a.partial + g.partial,
  stub: a.stub + g.stub,
}), { total: 0, translated: 0, partial: 0, stub: 0 });
const semanticSum = sum(semantic);
const stagingSum = sum(staging);

const row = (m, g) => `| ${m} | ${g.total} | ${g.translated} | ${g.partial} | ${g.stub} |`;
const table = (list) => [
  '| Module | Functions | Translated | Partial | Stub |',
  '|---|---:|---:|---:|---:|',
  ...list.map(([m, g]) => row(m, g)),
];

const lines = [
  '# Coverage snapshot',
  '',
  'Generated from `ledger/functions.csv` by `tools/coverage-doc.mjs`.',
  'Do not edit by hand; regenerate after merging lane ledgers.',
  '',
  `Ledger total: **${records.length} functions** - ${semanticSum.total} semantic, ${stagingSum.total} staging.`,
  '',
  '## Semantic modules (reader-facing)',
  '',
  ...table(semantic),
  '',
  `Semantic total: **${semanticSum.total}** (${semanticSum.translated} translated, ${semanticSum.partial} partial, ${semanticSum.stub} stub).`,
  '',
  '## Staging ranges (not reader-facing)',
  '',
  ...table(staging),
  '',
  `Staging total: **${stagingSum.total}**. These shrink as functions are promoted; see docs/PROMOTION.md.`,
  '',
  '## Context',
  '',
  '- `main.dol` has **10,276** decompiled functions; the four RELs add **9,976** (36 + 5,615 + 3,555 + 770).',
  '- The ledger counts only functions that have been translated at least once; it is not a progress bar for all 20,252.',
  '- "Translated" means the recorded control flow is represented; inferred field names may still be unproven.',
  '- Gates: `cargo check` must be 0 and `node tools/doc-lint.mjs src` must be 0 before a lane is merged.',
  '',
  '## Refresh',
  '',
  '```powershell',
  'node tools/merge-ledger.mjs',
  'node tools/index-functions.mjs',
  'node tools/coverage-doc.mjs',
  '```',
  '',
];
fs.writeFileSync(outPath, lines.join('\n'));
console.log(JSON.stringify({ ledger: records.length, semantic: semanticSum, staging: stagingSum, out: outPath }, null, 2));
