const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const root = 'C:/Users/Wither/.codex/worktrees/rust-guest-memory/decomp';
const output = 'C:/Users/Wither/Documents/Codex/2026-11-25/rad/outputs';
const disc = 'E:/Localsend/mariobroswii/sys';
const inputs = {
  ledger: path.join(root, 'ledger/functions.csv'),
  recomp: 'E:/NSMBWRecomp/wiicompiled/projects/nsmbw/MAP.txt',
  normalizedE1: 'E:/NSMBWRecomp/maps/nsmbw_e1_symbols.tsv',
  reviewedE1: 'E:/NSMBWDecompCodex/analysis/symbols/symbols_E1_merged_v13.tsv',
};

function prefix(file, length) {
  const fd = fs.openSync(file, 'r');
  try {
    const buffer = Buffer.alloc(length);
    if (fs.readSync(fd, buffer, 0, length, 0) !== length) throw new Error(file + ' is truncated');
    return buffer;
  } finally {
    fs.closeSync(fd);
  }
}

function address(raw) {
  if (!/^(?:0x)?[0-9a-f]{8}$/i.test(raw)) throw new Error('Invalid address ' + raw);
  return Number.parseInt(raw.replace(/^0x/i, ''), 16);
}

function hex(value) {
  return '0x' + value.toString(16).padStart(8, '0');
}

function isPlaceholder(name) {
  return /^(?:(?:fn|fun|sub)_(?:0x)?[0-9a-f]{8}|0x[0-9a-f]{8})(?:__.*)?$/i.test(name);
}

function csv(text) {
  const rows = [];
  let row = [], field = '', quoted = false, line = 1, rowStart = 1;
  for (let i = 0; i < text.length; i += 1) {
    const ch = text[i];
    if (quoted && ch === '"') {
      if (text[i + 1] === '"') {
        field += '"';
        i += 1;
      } else quoted = false;
    } else if (!quoted && ch === '"') quoted = true;
    else if (!quoted && ch === ',') {
      row.push(field);
      field = '';
    } else if (!quoted && ch === '\n') {
      rows.push({ line: rowStart, fields: [...row, field] });
      row = [];
      field = '';
      rowStart = line + 1;
    } else if (ch !== '\r') field += ch;
    if (ch === '\n') line += 1;
  }
  if (quoted) throw new Error('Unterminated ledger field');
  if (field || row.length) rows.push({ line: rowStart, fields: [...row, field] });
  return rows;
}

const boot = prefix(path.join(disc, 'boot.bin'), 8);
const revision = boot.subarray(0, 6).toString('ascii') + ' rev ' + boot[7];
if (revision !== 'SMNE01 rev 1') throw new Error('Unexpected executable revision ' + revision);
const header = prefix(path.join(disc, 'main.dol'), 256);
const ranges = Array.from({ length: 7 }, (_, i) => {
  const start = header.readUInt32BE(0x48 + 4 * i);
  return { start, end: start + header.readUInt32BE(0x90 + 4 * i) };
}).filter(({ start, end }) => start < end);

const payloads = Object.fromEntries(Object.entries(inputs).map(([key, file]) => [key, fs.readFileSync(file)]));
const fingerprints = Object.fromEntries(Object.entries(inputs).map(([key, file]) => [
  key, { file, sha256: crypto.createHash('sha256').update(payloads[key]).digest('hex') },
]));
const text = (key) => payloads[key].toString('utf8').replace(/^\uFEFF/, '');
const rows = csv(text('ledger'));
const columns = rows.shift().fields;
const indexes = Object.fromEntries(columns.map((name, index) => [name, index]));
const entries = new Map();
let outsideText = 0;
for (const row of rows) {
  if (row.fields.length !== columns.length) throw new Error('Ledger column count at line ' + row.line);
  const read = (key) => row.fields[indexes[key]];
  const original = read('evidence').match(/work[\\/]ghidra[\\/]main-gb[\\/]export[\\/][^\s;,)"']+/i)?.[0];
  if (!original) continue;
  const value = address(read('address'));
  if (!ranges.some(({ start, end }) => value >= start && value < end)) {
    outsideText += 1;
    continue;
  }
  if (entries.has(value)) throw new Error('Duplicate main ledger address ' + hex(value));
  entries.set(value, {
    address: hex(value),
    current: { name: read('name'), file: read('rust_file'), ledgerLine: row.line },
    export: { path: original.replaceAll('\\', '/'), available: fs.existsSync(path.join(root, original)) },
    candidates: [],
  });
}

function add(raw, candidate) {
  const row = entries.get(address(raw));
  if (row && candidate.spelling) row.candidates.push(candidate);
}
for (const [index, line] of text('recomp').split(/\r?\n/).entries()) {
  if (!line.trim()) continue;
  const match = /^\s*([0-9a-f]{8})\s+(.+?)\s*$/i.exec(line);
  if (!match) throw new Error('Invalid recomp map line ' + (index + 1));
  const [, raw, spelling] = match;
  add(raw, { source: 'recomp', line: index + 1, spelling });
}
for (const [index, line] of text('normalizedE1').split(/\r?\n/).entries()) {
  if (!line.trim()) continue;
  const [raw, spelling, display] = line.split('\t');
  add(raw, { source: 'normalizedE1', line: index + 1, spelling, display });
}
const [reviewColumns, ...reviews] = text('reviewedE1').split(/\r?\n/);
const reviewIndexes = Object.fromEntries(reviewColumns.split('\t').map((name, index) => [name, index]));
for (const key of ['address', 'kind', 'name', 'module', 'confidence', 'display_name', 'source_signature']) {
  if (!(key in reviewIndexes)) throw new Error('Missing reviewed column ' + key);
}
for (const [index, line] of reviews.entries()) {
  if (!line) continue;
  const fields = line.split('\t');
  const read = (key) => fields[reviewIndexes[key]];
  if (read('module') !== 'main' || read('kind') !== 'function') continue;
  add(read('address'), {
    source: 'reviewedE1', line: index + 2, spelling: read('name'),
    display: read('display_name'), signature: read('source_signature') || undefined,
    sourceLabel: read('source'), confidence: read('confidence'),
  });
}

const tally = { rows: 0, placeholders: 0, agreement: 0, singleSource: 0, conflicts: 0, noName: 0, availableExports: 0, outsideText };
const sorted = [...entries.values()].sort((a, b) => address(a.address) - address(b.address));
for (const row of sorted) {
  tally.rows += 1;
  if (row.export.available) tally.availableExports += 1;
  row.current.placeholder = isPlaceholder(row.current.name);
  if (row.current.placeholder) tally.placeholders += 1;
  const candidates = row.candidates.filter((c) => !isPlaceholder(c.spelling) && !c.confidence?.includes('unknown'));
  const spellings = new Set(candidates.map((c) => c.spelling));
  row.assessment = spellings.size === 0 ? 'noName'
    : spellings.size > 1 ? 'conflicts'
    : new Set(candidates.map((c) => c.source)).size > 1 ? 'agreement' : 'singleSource';
  if (row.current.placeholder) tally[row.assessment] += 1;
}
if (tally.agreement + tally.singleSource + tally.conflicts + tally.noName !== tally.placeholders) {
  throw new Error('Placeholder assessment is incomplete');
}

fs.mkdirSync(output, { recursive: true });
const inventory = path.join(output, 'phase1-main-symbol-evidence.jsonl');
fs.writeFileSync(inventory, sorted.map((row) => JSON.stringify(row)).join('\n') + '\n');
const metadata = path.join(output, 'phase1-main-symbol-summary.json');
fs.writeFileSync(metadata, JSON.stringify({
  revision, ranges: ranges.map(({ start, end }) => ({ start: hex(start), end: hex(end) })),
  inputs: fingerprints, tally,
  notes: [
    'Agreement means exact spelling at the same address across input files. These sources can share provenance.',
    'Disagreements are preserved as candidates. No source names were applied.',
    'Inventory targets cite main-program exports in the existing ledger and fall inside the local executable text ranges.',
    'Unversioned raw symbol listings are omitted.',
  ],
}, null, 2) + '\n');
console.log(JSON.stringify({ inventory, metadata, revision, tally }));
