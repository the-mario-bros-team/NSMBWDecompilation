#!/usr/bin/env node
// Verify the merged ledger against the Ghidra function indexes and the tree.
//
// Checks:
//   1. Every ledger row's rust_file exists under src/.
//   2. No analysis address is recorded in two different modules.
//   3. Inventory invariant: every ledger address exists as a table entry in at
//      least one functions.tsv and the row count never exceeds the number of
//      distinct addresses the inventory holds. Ghidra also reports bodies it
//      only reached by following a branch (`0x8004AAC0` class); those are not
//      table entries and must never be ledgered.
//   4. Every ledger row has a body in its rust_file:
//        - a `fn_<addr>` definition,
//        - a semantic-named function whose `# Original` evidence canonically
//          cites the address (or whose name matches the ledger name),
//        - or an inline documentation note (attached `///` line) whose subject
//          is the address.
//      Rows that only appear as call sites / plain comments are reported as
//      real gaps (the `0x806E7F50` class: name mentioned, body never written).
//
// Usage: node tools/verify-ledger.mjs [--enforce-bodies]
//   --enforce-bodies  exit 1 when any row lacks a body (used by the gate).
// Exit code 1 on any hard failure (a missing inventory export, an address
// outside the inventory, a row count above the inventory, duplicate address,
// missing file, and, with --enforce-bodies, a row without a body).

import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const ledgerPath = path.join(root, 'ledger', 'functions.csv');
const srcRoot = path.join(root, 'src');
const enforceBodies = process.argv.includes('--enforce-bodies');
const exports = [
  'work/ghidra/main-gb/export/functions.tsv',
  'work/ghidra/rel-d_profileNP/export/functions.tsv',
  'work/ghidra/rel-d_basesNP/export/functions.tsv',
  'work/ghidra/rel-d_enemiesNP/export/functions.tsv',
  'work/ghidra/rel-d_en_bossNP/export/functions.tsv',
];

function parseCsv(text) {
  const rows = [];
  let row = [];
  let field = '';
  let quoted = false;
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

const header = ['module', 'address', 'name', 'status', 'confidence', 'evidence', 'rust_file'];
const rows = parseCsv(fs.readFileSync(ledgerPath, 'utf8'));
const idx = Object.fromEntries(header.map((h) => [h, rows[0].indexOf(h)]));
const records = rows.slice(1).map((r) => Object.fromEntries(header.map((h) => [h, (r[idx[h]] ?? '').trim()])));

// --- inventory ------------------------------------------------------------
//
// The union of the Ghidra function exports is the *only* legitimate source of
// ledger rows: an address has to be a table entry in some functions.tsv. Rows
// for bodies that Ghidra merely disassembled (branch targets such as
// `0x8004AAC0`) or for synthesized bases have no table entry here and are
// rejected; the total row count is bounded by the inventory size for the same
// reason.

const inventory = new Set();
const inventoryByExport = [];
const missingExports = [];

for (const rel of exports) {
  const file = path.join(root, rel);
  if (!fs.existsSync(file)) {
    missingExports.push(rel);
    continue;
  }
  const lines = fs.readFileSync(file, 'utf8').trim().split(/\r?\n/).slice(1);
  const addrs = new Set();
  for (const line of lines) {
    const addr = line.split('\t')[0].trim();
    if (addr) addrs.add(normalizeAddr(addr));
  }
  for (const addr of addrs) inventory.add(addr);
  inventoryByExport.push({ export: rel, rows: lines.length, unique: addrs.size });
}

// --- body index -----------------------------------------------------------

const FN_RE = /^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:unsafe\s+)?(?:extern\s+"[^"]*"\s+)?fn\s+([A-Za-z0-9_]+)/;
const ATTR_RE = /^#\[.*?\](?:\s*\/\/.*)?$/;
const HEX_RE = /(?<![0-9a-fA-F])(?:0x)?([0-9a-fA-F]{6,8})(?![0-9a-fA-F])/g;

function normalizeAddr(a) {
  let s = a.toLowerCase().replace(/^0x/, '').split('-')[0].trim();
  return s.length < 8 ? s.padStart(8, '0') : s;
}

function hexTokens(text) {
  const out = [];
  for (const m of text.matchAll(HEX_RE)) out.push(normalizeAddr(m[1]));
  return out;
}

function isDocLine(line) {
  const t = line.trimStart();
  return t.startsWith('///') || t.startsWith('//!');
}

function docText(line) {
  return line.trimStart().replace(/^\/\/[\/!]\s?/, '');
}

// The primary address a function's evidence block claims. Accept only the
// canonical shapes used by the tree so that callee mentions inside
// `# Original` ("calls 0x...") never count as a body for the callee row.
function primaryAddresses(orig) {
  const out = new Set();
  for (const raw of orig.split('\n')) {
    const t = raw.trim();
    let m;
    if ((m = t.match(/^-\s*Ghidra:\s*`?FUN_([0-9a-fA-F]{6,8})`?\s+at\s+`?(?:0x)?([0-9a-fA-F]{6,8})`?/i))) {
      out.add(normalizeAddr(m[2]));
    } else if ((m = t.match(/^-\s*(?:Evidence|C file):\s*`?(?:[^`\s]*\/)?([0-9a-fA-F]{6,8})_/i))) {
      out.add(normalizeAddr(m[1]));
    } else if ((m = t.match(/^(?:-\s*)?Synthetic base\s+`?(?:0x)?([0-9a-fA-F]{6,8})`?/i))) {
      out.add(normalizeAddr(m[1]));
    } else if ((m = t.match(/^`?\/\/\s*synthetic base:\s*(?:0x)?([0-9a-fA-F]{6,8})`?/i))) {
      out.add(normalizeAddr(m[1]));
    }
  }
  return out;
}

// True when the doc line is *about* the address (subject position), e.g.
// "`0x800C9F60` - ...", "| `0x800C9F60` | ... |", rather than a passing
// mention such as "calls `0x801B1140`".
function subjectAddresses(line) {
  const t = line
    .trim()
    .replace(/^[|>*+\-\s]+/, '')
    .replace(/^[`*_]+/, '');
  const m = t.match(/^(?:0x)?([0-9a-fA-F]{8})(?![0-9a-fA-F])/);
  return m ? [normalizeAddr(m[1])] : [];
}

const fileInfo = new Map();

function scanRustFile(file) {
  if (fileInfo.has(file)) return fileInfo.get(file);
  const info = {
    exact: new Map(),      // addr -> line
    semantic: new Map(),   // addr -> { name, line } (primary evidence)
    inline: new Map(),     // addr -> { name, line } (attached /// note)
    mentions: new Map(),   // addr -> line (any other occurrence)
  };
  if (!fs.existsSync(file)) {
    fileInfo.set(file, info);
    return info;
  }
  const lines = fs.readFileSync(file, 'utf8').split(/\r?\n/);
  const noteHint = (map, addr, line) => { if (!map.has(addr)) map.set(addr, line); };
  const docLinesOf = new Set(); // 1-based line numbers that belong to a fn doc
  const inlineHits = [];        // { addr, name, line }

  for (let i = 0; i < lines.length; i++) {
    const fnm = lines[i].match(FN_RE);
    if (!fnm) continue;
    // Collect the contiguous /// doc block immediately above the definition.
    const docLines = [];
    let first = i;
    for (let j = i - 1; j >= 0; j--) {
      const t = lines[j].trim();
      if (t.startsWith('///')) { docLines.unshift(docText(lines[j])); first = j; continue; }
      if (ATTR_RE.test(t)) { first = j; continue; }
      break;
    }
    const doc = docLines.join('\n');
    const name = fnm[1];
    const named = name.match(/^fn_([0-9a-fA-F]{6,8})$/);
    if (named) noteHint(info.exact, normalizeAddr(named[1]), i + 1);
    const primary = doc.includes('# Original')
      ? primaryAddresses(doc.split(/\n(?=#\s)/).find((p) => p.startsWith('# Original')) ?? '')
      : new Set();
    if (doc.includes('# Original')) {
      const orig = doc.split(/\n(?=#\s)/).find((p) => p.startsWith('# Original')) ?? '';
      for (const addr of hexTokens(orig)) {
        if (named && addr === normalizeAddr(named[1])) continue;
        const canonical = primary.has(addr);
        const prev = info.semantic.get(addr);
        if (!prev || (canonical && !prev.canonical)) {
          info.semantic.set(addr, { name, line: i + 1, canonical });
        }
        if (!canonical) noteHint(info.mentions, addr, i + 1);
      }
    }
    for (let j = first; j < i; j++) {
      docLinesOf.add(j + 1);
      const line = lines[j];
      if (!line.trim().startsWith('///')) continue;
      for (const addr of subjectAddresses(docText(line))) {
        if (primary.has(addr)) continue;
        inlineHits.push({ addr, name, line: j + 1 });
      }
    }
  }

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const trimmed = line.trimStart();
    if (isDocLine(line)) {
      if (!trimmed.startsWith('//!') && docLinesOf.has(i + 1)) {
        // Already classified through the attached function doc above.
        continue;
      }
      for (const addr of hexTokens(docText(line))) noteHint(info.mentions, addr, i + 1);
    } else {
      // Any remaining occurrence: call site, plain `//` comment, or constant.
      for (const addr of hexTokens(line)) noteHint(info.mentions, addr, i + 1);
    }
  }

  for (const hit of inlineHits) {
    if (!info.inline.has(hit.addr)) info.inline.set(hit.addr, { name: hit.name, line: hit.line });
  }
  fileInfo.set(file, info);
  return info;
}

function walkRust(dir, out = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walkRust(p, out);
    else if (e.name.endsWith('.rs')) out.push(p);
  }
  return out;
}

const byAddress = new Map();
const missingFiles = new Set();
const missingAddresses = [];
const bodyChecks = [];

for (const r of records) {
  const addr = r.address.toLowerCase().replace(/^0x/, '');
  const start = addr.split('-')[0].trim();
  if (!byAddress.has(start)) byAddress.set(start, []);
  byAddress.get(start).push(r);
  const rust = r.rust_file.replace(/\\/g, '/');
  if (!fs.existsSync(path.join(root, rust))) missingFiles.add(rust);
  if (!inventory.has(normalizeAddr(r.address))) missingAddresses.push(r.address + ' (' + rust + ')');
  bodyChecks.push({ row: r, rust, addr: normalizeAddr(r.address) });
}

// Any two ledger rows for the same normalized entry address are duplicates,
// even when they claim the same module (different address spellings used to
// slip through as separate rows).
const duplicates = [...byAddress.entries()].filter(([, list]) => list.length > 1);

// --- inventory invariant --------------------------------------------------
//
// A ledger row is only admissible when the address is a table entry in some
// functions.tsv, and there can never be more rows than distinct inventory
// addresses. This is the guard that keeps the `0x8004AAC0` class (bodies with
// no table entry) out of the ledger.

const inventoryCheck = {
  ok: missingExports.length === 0 && missingAddresses.length === 0 && records.length <= inventory.size,
  ledgerRows: records.length,
  inventoryAddresses: inventory.size,
  headroom: inventory.size - records.length,
  exports: inventoryByExport,
  missingExports,
  rowsOutsideInventory: missingAddresses.length,
  rowsOutsideInventorySamples: missingAddresses.slice(0, 100),
};

// --- body presence --------------------------------------------------------

const bodyResults = { exact: [], semantic: [], inline: [], gaps: [] };
for (const { row, rust, addr } of bodyChecks) {
  const info = scanRustFile(path.join(root, rust));
  if (info.exact.has(addr)) bodyResults.exact.push({ row, rust });
  else if (info.semantic.has(addr)
    && (info.semantic.get(addr).canonical || info.semantic.get(addr).name.toLowerCase() === row.name.toLowerCase())) {
    bodyResults.semantic.push({ row, rust, fn: info.semantic.get(addr) });
  } else if (info.inline.has(addr)) {
    bodyResults.inline.push({ row, rust, line: info.inline.get(addr).line });
  } else {
    bodyResults.gaps.push({
      row,
      rust,
      addr,
      kind: info.mentions.has(addr) ? 'mention-only' : 'missing',
      line: info.mentions.get(addr) ?? null,
    });
  }
}

// For real gaps, look across the whole tree: a body in a sibling module is a
// wiring mistake, not missing work.
if (bodyResults.gaps.length) {
  const elsewhere = new Map();
  for (const file of walkRust(srcRoot)) {
    const info = scanRustFile(file);
    const rel = path.relative(root, file).replace(/\\/g, '/');
    for (const addr of info.exact.keys()) elsewhere.set(addr, rel);
    for (const [addr, s] of info.semantic) if (s.canonical && !elsewhere.has(addr)) elsewhere.set(addr, rel);
  }
  for (const gap of bodyResults.gaps) {
    const other = elsewhere.get(gap.addr);
    if (other && other !== gap.rust) {
      gap.kind = 'body-elsewhere';
      gap.bodyIn = other;
    }
  }
}

const gapReport = bodyResults.gaps.map((g) => ({
  address: g.row.address,
  module: g.row.module,
  name: g.row.name,
  rustFile: g.rust,
  kind: g.kind,
  line: g.line,
  ...(g.bodyIn ? { bodyIn: g.bodyIn } : {}),
}));

const inlineReport = bodyResults.inline.slice(0, 200).map((i) => ({
  address: i.row.address,
  module: i.row.module,
  name: i.row.name,
  rustFile: i.rust,
  line: i.line,
}));

const semanticReport = bodyResults.semantic.filter((s) => !s.fn.canonical).slice(0, 200).map((s) => ({
  address: s.row.address,
  module: s.row.module,
  name: s.row.name,
  rustFile: s.rust,
  function: s.fn.name,
  line: s.fn.line,
}));

console.log(JSON.stringify({
  rows: records.length,
  uniqueAddresses: byAddress.size,
  duplicateAcrossModules: duplicates.map(([addr, list]) => ({
    address: addr,
    modules: [...new Set(list.map((r) => r.module))],
  })),
  missingRustFiles: [...missingFiles],
  missingFromExports: missingAddresses,
  inventory: inventoryCheck,
  bodies: {
    ok: gapReport.length === 0,
    fnBodies: bodyResults.exact.length,
    semanticBodies: bodyResults.semantic.length,
    inlineDocumentation: bodyResults.inline.length,
    gaps: gapReport.length,
    gapKinds: gapReport.reduce((acc, g) => {
      acc[g.kind] = (acc[g.kind] ?? 0) + 1;
      return acc;
    }, {}),
    gapSamples: gapReport.slice(0, 200),
    inlineSamples: inlineReport,
    nameMatchedSamples: semanticReport,
  },
}, null, 2));

if (duplicates.length || missingFiles.size || !inventoryCheck.ok) process.exit(1);
if (enforceBodies && bodyResults.gaps.length) process.exit(1);
