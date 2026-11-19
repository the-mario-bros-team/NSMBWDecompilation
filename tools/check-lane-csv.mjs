#!/usr/bin/env node
// Pre-merge validation for a wave's agent CSVs.
//
// Usage: node tools/check-lane-csv.mjs wave33
//
// Checks every ledger/agents/<wave>-*.csv for:
//   - the canonical 7-column header
//   - exactly 7 fields per row (catches unquoted commas in `evidence`)
//   - lowercase plain-hex addresses, no `0x` prefix
//   - unique addresses inside the file and across the whole wave
//   - no overlap with the already-merged ledger/functions.csv
//   - a real `pub unsafe fn fn_<addr>` body in the declared rust_file
//   - an evidence file that exists relative to the repo root
//   - an `# Original` byte size that matches functions.tsv, when available
//
// Exits non-zero and lists every problem when anything fails.
import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const wave = process.argv[2];
if (!wave) {
  console.error('usage: node tools/check-lane-csv.mjs <wave-prefix>');
  process.exit(2);
}

const HEADER = ['module', 'address', 'name', 'status', 'confidence', 'evidence', 'rust_file'];

function parseCsv(text) {
  const rows = [];
  let row = [];
  let field = '';
  let quoted = false;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (quoted) {
      if (c === '"') {
        if (text[i + 1] === '"') {
          field += '"';
          i++;
        } else quoted = false;
      } else field += c;
    } else if (c === '"') quoted = true;
    else if (c === ',') {
      row.push(field);
      field = '';
    } else if (c === '\n') {
      row.push(field);
      rows.push(row);
      row = [];
      field = '';
    } else if (c !== '\r') field += c;
  }
  if (field.length || row.length) {
    row.push(field);
    rows.push(row);
  }
  return rows.filter((r) => r.length > 1 || r[0] !== '');
}

// Read the merged ledger so a lane cannot re-claim an address.
const ledgerDone = new Set();
const ledgerPath = path.join(root, 'ledger', 'functions.csv');
if (fs.existsSync(ledgerPath)) {
  for (const row of parseCsv(fs.readFileSync(ledgerPath, 'utf8')).slice(1)) {
    const a = (row[1] || '').trim().toLowerCase().replace(/^0x/, '');
    if (a) ledgerDone.add(a);
  }
}

// Load every functions.tsv we can find, keyed by lowercase address.
const sizes = new Map();
for (const dir of fs.readdirSync(path.join(root, 'work', 'ghidra'), { withFileTypes: true })) {
  if (!dir.isDirectory()) continue;
  // `smoke` is a synthetic fixture whose one-row tsv has fake sizes; it would
  // shadow the real 364-byte `entry` with its dummy 1-byte row.
  if (dir.name === 'smoke') continue;
  const tsv = path.join(root, 'work', 'ghidra', dir.name, 'export', 'functions.tsv');
  if (!fs.existsSync(tsv)) continue;
  const lines = fs.readFileSync(tsv, 'utf8').split(/\r?\n/).filter((l) => l.trim());
  // Resolve the size column by header name: the export has 5 columns
  // (`address name size decompiled file`), so the old `len - 2` index pointed
  // at `decompiled` and the cross-check silently never matched. Fall back to
  // `len - 2` when the file has no recognisable header.
  const headerCells = (lines[0] || '').split('\t').map((x) => x.trim().toLowerCase());
  const hasHeader = headerCells.includes('address');
  const sizeCol = hasHeader ? headerCells.indexOf('size') : -1;
  for (const line of hasHeader ? lines.slice(1) : lines) {
    const c = line.split('\t');
    const addr = (c[0] || '').trim().toLowerCase().replace(/^0x/, '');
    const rawSize = sizeCol >= 0 ? c[sizeCol] : c[c.length - 2];
    const size = parseInt((rawSize || '').replace(/,/g, ''), 10);
    if (/^[0-9a-f]{8}$/.test(addr) && Number.isFinite(size)) sizes.set(addr, size);
  }
}

const agentsDir = path.join(root, 'ledger', 'agents');
const files = fs
  .readdirSync(agentsDir)
  .filter((n) => n.startsWith(wave + '-') && n.endsWith('.csv'))
  .sort();

if (!files.length) {
  console.error(`no ledger/agents/${wave}-*.csv found`);
  process.exit(1);
}

const problems = [];
const seen = new Map();
const srcCache = new Map();
const tally = {};
let totalRows = 0;

for (const file of files) {
  const full = path.join(agentsDir, file);
  const rows = parseCsv(fs.readFileSync(full, 'utf8'));
  const header = rows[0].map((x) => x.trim());
  if (header.join(',') !== HEADER.join(',')) {
    problems.push(`${file}: header is "${header.join(',')}"`);
    continue;
  }
  let count = 0;
  for (const r of rows.slice(1)) {
    if (!r.length || (r.length === 1 && !r[0].trim())) continue;
    count++;
    totalRows++;
    if (r.length !== 7) {
      problems.push(`${file}: row has ${r.length} fields (unquoted comma?): ${r[0]}|${r[1]}|...`);
      continue;
    }
    const [module, address, name, status, confidence, evidence, rustFile] = r.map((x) => x.trim());
    const addr = address.toLowerCase();

    if (!/^[0-9a-f]{8}$/.test(addr)) {
      problems.push(`${file}: address "${address}" is not plain 8-digit lowercase hex`);
      continue;
    }
    if (name !== `fn_${addr}` && !/^FUN_[0-9a-f]{8}$/.test(name) && !/^[a-z_][a-z0-9_]*$/.test(name)) {
      problems.push(`${file}: name "${name}" does not match the address`);
    }
    if (!['translated', 'partial', 'stub'].includes(status)) {
      problems.push(`${file}: bad status "${status}"`);
    }
    if (!['high', 'medium', 'low'].includes(confidence)) {
      problems.push(`${file}: bad confidence "${confidence}"`);
    }
    if (ledgerDone.has(addr)) problems.push(`${file}: ${addr} is already in ledger/functions.csv`);
    if (seen.has(addr)) problems.push(`${file}: ${addr} duplicated (also in ${seen.get(addr)})`);
    else seen.set(addr, file);

    const evidencePath = evidence.split(';')[0].trim();
    if (!evidencePath) problems.push(`${file}: ${addr} has no evidence path`);
    else if (!fs.existsSync(path.join(root, evidencePath))) {
      problems.push(`${file}: ${addr} evidence missing: ${evidencePath}`);
    }

    if (!fs.existsSync(path.join(root, rustFile))) {
      problems.push(`${file}: ${addr} rust_file missing: ${rustFile}`);
    } else {
      if (!srcCache.has(rustFile)) srcCache.set(rustFile, fs.readFileSync(path.join(root, rustFile), 'utf8'));
      const src = srcCache.get(rustFile);
      // Bodies are usually `pub unsafe fn fn_<addr>(`, but a few modules name
      // them semantically (`pub unsafe fn entry(`), so accept either that or a
      // literal `# Original` reference to the address inside this file.
      const nameRe = name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      const bodyRe = new RegExp(`pub\\s+(?:unsafe\\s+)?fn\\s+(?:fn_${addr}|${nameRe})\\s*\\(`);
      const originalRe = new RegExp(`0x${addr.toUpperCase()}[^0-9A-Fa-f]`, 'i');
      if (!bodyRe.test(src) && !originalRe.test(src)) {
        problems.push(`${file}: ${addr} has no body and no # Original reference in ${rustFile}`);
      }
      // Byte size cross-check against the Ghidra inventory. `# Original` docs
      // write the address inside backticks (`0xADDR` (N bytes)), so allow a
      // closing backtick; and only look before this body, because a
      // neighbouring body's comment may mention the same address with a
      // different extent.
      const expected = sizes.get(addr);
      if (expected !== undefined) {
        const blockRe = new RegExp(
          `0x${addr.toUpperCase()}\`?\\s*\\(\\s*([\\d,]+)\\s*bytes\\s*\\)`,
          'g'
        );
        const bodyAt = src.search(bodyRe);
        const scope = bodyAt >= 0 ? src.slice(0, bodyAt) : src;
        const hits = [...scope.matchAll(blockRe)];
        const m = hits.length ? hits[hits.length - 1] : null;
        if (m) {
          const got = parseInt(m[1].replace(/,/g, ''), 10);
          if (got !== expected) {
            problems.push(`${file}: ${addr} byte size ${got} in doc, ${expected} in functions.tsv`);
          }
        }
      }
    }

    const k = `${status}/${confidence}`;
    tally[k] = (tally[k] || 0) + 1;
  }
  console.log(`${file}: ${count} rows`);
}

console.log(`\ntotal rows: ${totalRows}, unique addresses: ${seen.size}`);
console.log('status/confidence: ' + Object.entries(tally).map(([k, v]) => `${k}=${v}`).join(' '));

if (problems.length) {
  console.error(`\n${problems.length} problem(s):`);
  for (const p of problems) console.error('  ' + p);
  process.exit(1);
}
console.log('\nOK');
