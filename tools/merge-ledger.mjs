#!/usr/bin/env node
// Merge every ledger/agents/*.csv into ledger/functions.csv and write a
// human-readable ledger/SUMMARY.md. Deduplicates by module+address.
// Usage: node tools/merge-ledger.mjs
import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const agentsDir = path.join(root, 'ledger', 'agents');
const outCsv = path.join(root, 'ledger', 'functions.csv');
const outMd = path.join(root, 'ledger', 'SUMMARY.md');

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

function toCsvField(v) {
  const s = String(v ?? '');
  return /[",\n]/.test(s) ? '"' + s.replace(/"/g, '""') + '"' : s;
}

const header = ['module', 'address', 'name', 'status', 'confidence', 'evidence', 'rust_file'];
const byKey = new Map();
const sources = [];
const skipped = [];

if (fs.existsSync(agentsDir)) {
  for (const f of fs.readdirSync(agentsDir).filter((n) => n.endsWith('.csv')).sort()) {
    const full = path.join(agentsDir, f);
    const rows = parseCsv(fs.readFileSync(full, 'utf8'));
    if (!rows.length) continue;
    const cols = rows[0].map((x) => x.trim());
    const idx = Object.fromEntries(header.map((h) => [h, cols.indexOf(h)]));
    if (Object.values(idx).some((i) => i < 0)) {
      skipped.push(f);
      continue;
    }
    let n = 0;
    for (const r of rows.slice(1)) {
      const rec = Object.fromEntries(header.map((h) => [h, (r[idx[h]] ?? '').trim()]));
      if (!rec.address) continue;
      // Normalize before deduplicating: lanes disagree on 0x prefixes, and
      // some write a module id that differs from the rust_file-derived one.
      rec.address = String(rec.address)
        .toLowerCase()
        .split('-')
        .map((part) => part.trim().replace(/^0x/, ''))
        .join('-');
      rec.module = moduleOf(rec);
      const key = `${rec.module}|${rec.address}`;
      const prev = byKey.get(key);
      if (!prev) { byKey.set(key, rec); n++; }
      else {
        const rank = { translated: 3, partial: 2, stub: 1 };
        if ((rank[rec.status] ?? 0) > (rank[prev.status] ?? 0)) byKey.set(key, rec);
        n++;
      }
    }
    sources.push({ file: f, rows: n });
  }
}

function moduleOf(rec) {
  const rf = String(rec.rust_file ?? '').replace(/\\/g, '/');
  const m = rf.match(/^src\/(.+)\.rs$/);
  return m ? m[1].replace(/\//g, '::') : (rec.module || 'unknown');
}

const records = [...byKey.values()];
for (const r of records) r.module = moduleOf(r);
records.sort((a, b) => {
  if (a.module !== b.module) return a.module.localeCompare(b.module);
  return parseInt(a.address, 16) - parseInt(b.address, 16);
});

fs.writeFileSync(outCsv, header.join(',') + '\n' + records.map((r) => header.map((h) => toCsvField(r[h])).join(',')).join('\n') + '\n');

const byModule = {};
for (const r of records) {
  byModule[r.module] ??= { total: 0, translated: 0, partial: 0, stub: 0, high: 0, medium: 0, low: 0 };
  const m = byModule[r.module];
  m.total++;
  if (m[r.status] !== undefined) m[r.status]++;
  if (m[r.confidence] !== undefined) m[r.confidence]++;
}
const md = [
  '# Function ledger summary',
  '',
  `Generated from ${sources.length} lane ledger(s); ${records.length} unique functions.`,
  '',
  '| Module | Functions | Translated | Partial | Stub | High | Medium | Low |',
  '|---|---:|---:|---:|---:|---:|---:|---:|',
  ...Object.entries(byModule).map(([k, v]) => `| ${k} | ${v.total} | ${v.translated} | ${v.partial} | ${v.stub} | ${v.high} | ${v.medium} | ${v.low} |`),
  '',
  '## Sources',
  '',
  ...sources.map((s) => `- \`ledger/agents/${s.file}\` (${s.rows} rows)`),
  ...(skipped.length ? ['', 'Skipped (non-ledger CSV): ' + skipped.map((s) => `\`${s}\``).join(', ')] : []),
  '',
  '`ledger/functions.csv` is the machine-readable index; this file is the readable summary.',
  '',
].join('\n');
fs.writeFileSync(outMd, md);

console.log(JSON.stringify({ records: records.length, sources, skipped, byModule }, null, 2));
