#!/usr/bin/env node
// Body-level duplicate gate for `fn_<addr>` and semantic-named definitions.
//
// Name-level uniqueness is not enough: an address body can be written twice
// under the same `fn_<addr>` name in two modules and `cargo check` will never
// notice, because each module only sees its own item. This gate walks the whole
// tree, groups every function definition whose name is exactly `fn_<8 hex>`,
// and rejects any address that is implemented more than once.
//
// Semantic names are checked too, because they hide the same defect: a body
// translated as `dvd_report3` and another as `sdk_panic` can both claim
// `0x801AD4E0` in their `# Original` block without sharing a Rust name. The
// second pass therefore groups every definition by the *primary* address its
// `# Original` evidence claims (`- Ghidra: FUN_x at 0xADDR` - all addresses on
// a merged multi-body line - the untabled `- Ghidra: none ... 0xADDR` shape, or
// `- Address: 0xADDR`), and rejects every address claimed by more than one body
// unless the group is accounted for below. Evidence paths, "inlined into
// FUN_x" notes and host-side-helper prose are provenance for a slice of another
// body, not a competing implementation, and are not claims.
//
// A duplicate group is admissible only when one of these holds:
//
//   1. Delegation: every occurrence except one is a one-line delegation. A
//      delegation is a body that carries the `canonical:` marker and whose code
//      (non-comment) content is at most a single expression, e.g.
//
//          pub unsafe fn fn_801622d0() {
//              // canonical: crate::game::worldmap::fn_801622d0 owns this address.
//              crate::game::worldmap::fn_801622d0();
//          }
//
//   2. Adapter: every non-delegation occurrence that is not the single real
//      body carries the `adapter:` marker and is a thin forwarder (at most two
//      code lines). This covers the documented arity adapters that call one
//      external routine with different argument counts; the marker records why
//      the second name exists, e.g.
//
//          unsafe fn dvd_report4(ctx: usize, line: u32, fmt: u32, arg: u32) {
//              // adapter: arity forwarder to FUN_801AD4E0 (call4 adds %s).
//              call4(DVD_REPORT, ctx as u32, line, fmt, arg);
//          }
//
//      A group made up entirely of adapters is allowed: the address has
//      documented forwarders but no translated body (a `stub` ledger row).
//
//   3. Arch split: every real (non-delegation, non-adapter) occurrence is
//      guarded by a `#[cfg(...)]` attribute naming `target_arch`, and the
//      guards differ (so `powerpc` and `not(powerpc)` are fine, two identical
//      guards are a copy-paste bug that would not compile).
//
// Anything else is a hard failure: the same body exists twice, so the ledger
// would have two competing canonical homes for one address.
//
// Usage: node tools/check-dupe-bodies.mjs [--src <dir>] [--report-only]
//   --src <dir>     tree to scan (default: src)
//   --report-only   print findings, always exit 0 (for use before arming)
//
// Exit code 1 when a duplicate group is unaccounted for.

import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const argv = process.argv.slice(2);
const srcArg = argv.indexOf('--src');
const srcRoot = path.join(root, srcArg >= 0 ? argv[srcArg + 1] : 'src');
const reportOnly = argv.includes('--report-only');

const FN_RE = new RegExp(
  '^\\s*(?:#\\[[^\\]]*\\]\\s*)*(?:pub(?:\\([^)]*\\))?\\s+)?(?:default\\s+)?(?:const\\s+)?(?:async\\s+)?' +
  '(?:unsafe\\s+)?(?:extern\\s+"[^"]*"\\s+)?fn\\s+([A-Za-z_][A-Za-z0-9_]*)\\b',
);
const ATTR_RE = /^#\[.*\](?:\s*\/\/.*)?$/;
const CANONICAL_RE = /\bcanonical\s*:/;
const ADAPTER_RE = /\badapter\s*:/;
const ADDR_NAME_RE = /^fn_[0-9a-f]{6,8}$/i;

function walkRust(dir, out = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walkRust(p, out);
    else if (e.name.endsWith('.rs')) out.push(p);
  }
  return out;
}

// Blank out comments and string/char literals so brace counting cannot be
// fooled by `{` inside a message or a `// }` note. Newlines are preserved so
// offsets stay line-aligned.
function maskLiterals(source) {
  let out = '';
  let i = 0;
  const n = source.length;
  while (i < n) {
    const c = source[i];
    const d = source[i + 1];
    if (c === '/' && d === '/') {
      while (i < n && source[i] !== '\n') { out += ' '; i++; }
      continue;
    }
    if (c === '/' && d === '*') {
      out += '  ';
      i += 2;
      while (i < n && !(source[i] === '*' && source[i + 1] === '/')) {
        out += source[i] === '\n' ? '\n' : ' ';
        i++;
      }
      if (i < n) { out += '  '; i += 2; }
      continue;
    }
    if (c === 'r' && (d === '"' || d === '#')) {
      // Raw string: r"..." or r##"..."##. Emit spaces, keep newlines.
      let j = i + 1;
      let hashes = 0;
      while (source[j] === '#') { hashes++; j++; }
      if (source[j] === '"') {
        out += ' '.repeat(j - i + 1);
        j++;
        const close = '"' + '#'.repeat(hashes);
        while (j < n && !source.startsWith(close, j)) {
          out += source[j] === '\n' ? '\n' : ' ';
          j++;
        }
        const end = Math.min(n, j + close.length);
        out += ' '.repeat(end - j);
        i = end;
        continue;
      }
    }
    if (c === '"' || (c === 'b' && d === '"')) {
      if (c === 'b') { out += ' '; i++; }
      out += ' ';
      i++;
      while (i < n) {
        if (source[i] === '\\') {
          out += ' ';
          if (i + 1 < n) out += source[i + 1] === '\n' ? '\n' : ' ';
          i += 2;
          continue;
        }
        if (source[i] === '"') { out += ' '; i++; break; }
        out += source[i] === '\n' ? '\n' : ' ';
        i++;
      }
      continue;
    }
    if (c === "'") {
      // Char literal or lifetime. Treat `'x'` / `'\n'` as a literal; leave
      // lifetimes (`'a`) alone, they contain no braces anyway.
      const m = /^'(?:\\(?:u\{[0-9a-fA-F]+\}|.)|[^'\\\n])'/.exec(source.slice(i));
      if (m) {
        out += ' '.repeat(m[0].length);
        i += m[0].length;
        continue;
      }
    }
    out += c;
    i++;
  }
  return out;
}

function bodyOf(source, openIndex, endIndex) {
  // Strip the outer braces; slice the *source* (not the masked copy) so the
  // body keeps its comments and formatting.
  return source.slice(openIndex + 1, endIndex);
}

function analyseFile(file) {
  const source = fs.readFileSync(file, 'utf8');
  const masked = maskLiterals(source);
  const lines = source.split(/\r?\n/);
  const defs = [];

  // Offset of each source line start, for O(log n) offset -> line lookups.
  const lineStarts = [0];
  for (let p = 0; p < masked.length; p++) if (masked[p] === '\n') lineStarts.push(p + 1);
  const lineOf = (offset) => {
    let lo = 0;
    let hi = lineStarts.length - 1;
    while (lo < hi) {
      const mid = (lo + hi + 1) >> 1;
      if (lineStarts[mid] <= offset) lo = mid; else hi = mid - 1;
    }
    return lo + 1;
  };

  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(FN_RE);
    if (!m) continue;

    // Attribute + doc run above the definition, tolerating doc lines
    // interleaved with attributes (`#[cfg(..)]` above `/// doc` above `fn`).
    // Only real attribute lines are kept for the arch guard: prose in a doc
    // comment must never be read as a `target_arch` guard.
    const attrLines = [];
    const docLines = [];
    for (let k = i - 1; k >= 0; k--) {
      const t = lines[k].trim();
      if (ATTR_RE.test(t)) { attrLines.unshift(t); continue; }
      if (t.startsWith('///')) {
        docLines.unshift(t.replace(/^\/\/\/\s?/, ''));
        continue;
      }
      if (t.startsWith('//!') || t.startsWith('//') || t.startsWith('/*')) continue;
      break;
    }
    const attrs = attrLines.join('\n');

    // Find the body: the first `{` at or after this line in the masked text.
    const open = masked.indexOf('{', lineStarts[i]);
    if (open < 0) continue;
    let depth = 0;
    let end = -1;
    for (let p = open; p < masked.length; p++) {
      if (masked[p] === '{') depth++;
      else if (masked[p] === '}') {
        depth--;
        if (depth === 0) { end = p; break; }
      }
    }
    if (end < 0) continue;
    const body = bodyOf(source, open, end);
    const endLine = lineOf(end);
    const codeLines = codeLineCount(body);
    defs.push({
      name: m[1],
      file: path.relative(root, file).replace(/\\/g, '/'),
      line: i + 1,
      endLine,
      docLines,
      attrs,
      body,
      codeLines,
      addrName: ADDR_NAME_RE.test(m[1]),
      delegation: CANONICAL_RE.test(body) && codeLines <= 1,
      adapter: ADAPTER_RE.test(body) && codeLines <= 2,
      archCfg: /target_arch/.test(attrs) ? attrs.replace(/\s+/g, ' ').trim() : null,
    });
  }
  return defs;
}

// The `# Original` section of a definition's doc block, one entry per line.
function originalSection(docLines) {
  const start = docLines.findIndex((l) => /^#\s*Original\b/.test(l.trim()));
  if (start < 0) return null;
  const out = [];
  for (let i = start + 1; i < docLines.length; i++) {
    const t = docLines[i].trim();
    if (/^#\s/.test(t)) break;
    out.push(t);
  }
  return out;
}

// Addresses whose *own* body a definition's `# Original` evidence claims.
// Only the canonical "this function is FUN_<addr> at <addr>" shape counts:
// `- Evidence:` paths, "host-side helper" notes and callee mentions are
// provenance for a shape or a slice, not a competing implementation.
function claimedAddresses(def) {
  const orig = originalSection(def.docLines);
  const out = new Set();
  if (!orig) return out;
  for (const t of orig) {
    // A Ghidra line may name several merged bodies: `FUN_a at 0xA and FUN_b
    // at 0xB`. Every one of them is a body claim.
    const gh = t.match(/^-\s*Ghidra:\s*(.*)$/i);
    if (gh && !/^inlined\b/i.test(gh[1])) {
      let m;
      const re = /`?FUN_[0-9a-fA-F]{6,8}`?\s+at\s+`?(?:0x)?([0-9a-fA-F]{6,8})`?/gi;
      let found = false;
      while ((m = re.exec(gh[1]))) {
        out.add(normalizeAddr(m[1]));
        found = true;
      }
      if (found) continue;
      if (/^none\b/i.test(gh[1])) {
        // Untabled bodies have no FUN name, so their Ghidra line names the
        // slot address itself: `- Ghidra: none - no row for 0x80123450.`
        const own = gh[1].match(/(?<![0-9a-fA-F])(?:0x)?([0-9a-fA-F]{8})(?![0-9a-fA-F])/);
        if (own) out.add(normalizeAddr(own[1]));
      }
      continue;
    }
    let m;
    if ((m = t.match(/^-\s*Address:\s*`?(?:0x)?([0-9a-fA-F]{6,8})`?/i))) {
      out.add(normalizeAddr(m[1]));
    }
  }
  return out;
}

function normalizeAddr(a) {
  let s = a.toLowerCase().replace(/^0x/, '').trim();
  return s.length < 8 ? s.padStart(8, '0') : s;
}

// Code content of a body: non-empty lines that are not comments. A delegation
// is allowed a marker comment plus one call.
function codeLineCount(body) {
  return body
    .split('\n')
    .map((l) => l.trim())
    .filter((l) => l !== '' && !l.startsWith('//') && !l.startsWith('/*') && !l.startsWith('*'))
    .length;
}

const defs = [];
for (const file of walkRust(srcRoot)) defs.push(...analyseFile(file));

function judge(list) {
  const real = list.filter((d) => !d.delegation && !d.adapter);
  const archGuards = new Set(real.map((d) => d.archCfg).filter(Boolean));
  if (real.length <= 1) {
    return list.some((d) => d.adapter) ? 'single-body-with-delegations-or-adapters' : 'single-body-with-delegations';
  }
  if (real.every((d) => d.archCfg) && archGuards.size === real.length) return 'cfg-target-arch-split';
  return null;
}

function occurrence(d) {
  return {
    file: d.file,
    line: d.line,
    endLine: d.endLine,
    name: d.name,
    kind: d.delegation ? 'delegation' : d.adapter ? 'adapter' : 'body',
    codeLines: d.codeLines,
    archCfg: d.archCfg,
  };
}

// Pass 1: `fn_<addr>` names must not implement one address twice.
const fnDefs = defs.filter((d) => d.addrName);
const groups = new Map();
for (const d of fnDefs) {
  const key = d.name.toLowerCase();
  if (!groups.has(key)) groups.set(key, []);
  groups.get(key).push(d);
}

const duplicates = [];
const allowed = [];

for (const [name, list] of groups) {
  if (list.length < 2) continue;
  const reason = judge(list);
  const entry = { name, occurrences: list.map(occurrence) };
  if (reason) allowed.push({ ...entry, reason });
  else duplicates.push(entry);
}

// Pass 2: semantic names must not claim an address already claimed by another
// body. Groups with no semantic name are pass 1's business and are skipped, so
// a plain `fn_<addr>` duplicate is reported once.
const claims = new Map(); // addr -> defs claiming it
for (const d of defs) {
  for (const addr of claimedAddresses(d)) {
    if (!claims.has(addr)) claims.set(addr, []);
    claims.get(addr).push(d);
  }
}

const semanticDuplicates = [];
const semanticAllowed = [];

for (const [addr, list] of [...claims].sort((a, b) => a[0].localeCompare(b[0]))) {
  if (list.length < 2) continue;
  // A group of identically named `fn_<addr>` definitions is pass 1's business.
  if (list.every((d) => d.addrName) && new Set(list.map((d) => d.name)).size < 2) continue;
  const reason = judge(list);
  const entry = { address: addr, occurrences: list.map(occurrence) };
  if (reason) semanticAllowed.push({ ...entry, reason });
  else semanticDuplicates.push(entry);
}

if (reportOnly) {
  console.log(JSON.stringify({
    scanned: fnDefs.length,
    duplicateNames: duplicates.length + allowed.length,
    unaccounted: duplicates,
    allowed,
    semanticScanned: claims.size,
    semanticDuplicateAddresses: semanticDuplicates.length + semanticAllowed.length,
    semanticUnaccounted: semanticDuplicates,
    semanticAllowed,
  }, null, 2));
} else if (argv.includes('--list')) {
  for (const d of fnDefs) console.log(`${d.file}:${d.line} fn_${d.name}${d.delegation ? ' delegation' : ''}`);
} else {
  console.log(
    `scanned ${fnDefs.length} fn_<addr> definitions; ` +
    `${duplicates.length + allowed.length} duplicated names ` +
    `(${allowed.length} allowed, ${duplicates.length} failing)`,
  );
  for (const d of duplicates) {
    console.log(`\nFAIL fn_${d.name} defined ${d.occurrences.length}x:`);
    for (const o of d.occurrences) {
      const guard = o.archCfg ? `  [${o.archCfg}]` : '';
      console.log(`  ${o.file}:${o.line}-${o.endLine}  ${o.kind} (${o.codeLines} code lines)${guard}`);
    }
  }
  console.log(
    `semantic: ${claims.size} claimed addresses; ` +
    `${semanticDuplicates.length + semanticAllowed.length} claimed twice ` +
    `(${semanticAllowed.length} allowed, ${semanticDuplicates.length} failing)`,
  );
  for (const d of semanticDuplicates) {
    console.log(`\nFAIL 0x${d.address} claimed by ${d.occurrences.length} bodies:`);
    for (const o of d.occurrences) {
      const guard = o.archCfg ? `  [${o.archCfg}]` : '';
      console.log(`  ${o.file}:${o.line}-${o.endLine}  ${o.name} ${o.kind} (${o.codeLines} code lines)${guard}`);
    }
  }
}

if (!reportOnly && (duplicates.length || semanticDuplicates.length)) process.exit(1);
