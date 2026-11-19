#!/usr/bin/env node
// Page-offset checker: catches `addi`/`lwzu` sign-extension mis-decodes.
//
// The recurring bug class in this tree is a 16-bit `addi rX,rX,K` immediate
// with K negative being applied as an unsigned page offset, e.g.
//
//   lis  r4,0x8030
//   addi r4,r4,-11880        ; -0x2E68
//
// resolves to 0x802FD198, not 0x8030D198. Every constant is documented with
// the instruction pair it came from, so this tool re-derives the address from
// the doc block and compares it with the declared value.
//
// The second check catches the mirror-image bug: a constant whose *name*
// encodes an address that contradicts its declared value, e.g.
//
//   const E252_FLOAT_TABLE_8043_C1B0: usize = 0x8042_C1B0;
//
// Here the name says page 0x8043, the value (and its doc block) say 0x8042.
// The pair check above cannot see this because the instruction pair and the
// value agree; only the name is stale. The check re-extracts address-shaped
// hex tokens from the name, normalises them (a name may split a 4+4 token,
// `8043_C1B0`) and compares page and offset against the declared value.
//
// Name tokens are 4-8 hex digits. To keep ordinary numbers (`F32_ONE_1`,
// `_1500`), sentinels (`_DEAD`, `_FACE`) and wave tags (`_E252`, `_C243`)
// out, only 7-8 digit tokens starting with `8` and 4-digit pages in
// 0x8000..0x8045 are treated as addresses. A token whose low 16 bits equal
// the value's low 16 bits but whose page differs is a hard mismatch (the
// classic dropped/transposed digit). Tokens that differ more than that are
// usually provenance names (a symbol's original address recorded next to a
// relocated value) or thunk targets, so they are reported as ambiguous and
// never fail the build.
//
// Usage: node tools/check-page-offsets.mjs [--report-only]

import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';

const root = 'src';
const files = [];
(function walk(dir) {
  for (const entry of readdirSync(dir)) {
    const p = join(dir, entry);
    if (statSync(p).isDirectory()) walk(p);
    else if (p.endsWith('.rs')) files.push(p);
  }
})(root);

function parseImm(text) {
  const t = text.trim();
  const neg = t.startsWith('-');
  const body = neg ? t.slice(1) : t;
  let v;
  if (/^0x/i.test(body)) v = parseInt(body.replace(/_/g, ''), 16);
  else v = Number(body);
  if (!Number.isFinite(v)) return null;
  if (neg) return -v;
  // A 32-bit sign-extended form (0xFFFFFF78) is negative in 16 bits.
  if (v > 0x7fff && v <= 0xffffffff) {
    const s16 = v & 0xffff;
    if (s16 > 0x7fff) return s16 - 0x10000;
  }
  return v;
}

function constValue(text) {
  let t = text.trim().replace(/_/g, '');
  if (/^0x/i.test(t)) return parseInt(t.slice(2), 16);
  if (/^\d+$/.test(t)) return Number(t);
  return null;
}

const LIS = /lis\s+r(\d+)\s*,\s*(0x[0-9a-fA-F]+)/i;
const ADDI = (reg) =>
  new RegExp(`addi\\s+r${reg}\\s*,\\s*r${reg}\\s*,\\s*(-?(?:0x[0-9a-fA-F]+|\\d+))`, 'i');
const LWZU = (reg) =>
  new RegExp(`lwzu\\s+r${reg}\\s*,\\s*(-?(?:0x[0-9a-fA-F]+|\\d+))\\s*\\(\\s*r${reg}\\s*\\)`, 'i');

const findings = [];
let checked = 0;
let ambiguous = 0;

for (const file of files) {
  const lines = readFileSync(file, 'utf8').split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const m = /^\s*(?:pub\s+)?const\s+([A-Za-z0-9_]+)\s*:\s*usize\s*=\s*([^;]+);/.exec(lines[i]);
    if (!m) continue;
    const [, name, rawValue] = m;
    const value = constValue(rawValue);
    if (value === null) continue;

    // Collect the contiguous doc block above the declaration.
    let start = i - 1;
    while (start >= 0 && /^\s*\/\/\//.test(lines[start])) start--;
    const doc = lines.slice(start + 1, i).join('\n');
    if (!doc) continue;

    const lis = LIS.exec(doc);
    if (!lis) continue;
    const page = parseInt(lis[2].replace(/_/g, ''), 16);
    const reg = lis[1];

    // Only the first addi/lwzu for the same register, after the lis.
    const tail = doc.slice(lis.index + lis[0].length);
    const add = ADDI(reg).exec(tail) || LWZU(reg).exec(tail);
    if (!add) continue;
    const imm = parseImm(add[1]);
    if (imm === null) continue;

    checked++;
    // A positive 16-bit immediate cannot be mis-sign-extended; skip.
    if (imm >= 0) continue;
    ambiguous++;

    const expected = ((page << 16) + imm) >>> 0;
    // If the doc names the resolved base explicitly, the pair documents that
    // base and the constant may be a documented field inside it
    // (e.g. "`0x80359470 + 0xC4` -> `0x80359534`").
    const baseHex = expected.toString(16);
    const docMentionsBase = new RegExp(`0x0*${baseHex}`, 'i').test(doc);
    if (expected !== value && !docMentionsBase) {
      findings.push({
        file,
        line: i + 1,
        name,
        docPage: `0x${page.toString(16)}`,
        imm: imm.toString(10),
        expected: `0x${expected.toString(16)}`,
        actual: `0x${value.toString(16)}`,
      });
    }
  }
}

const enforce = !process.argv.includes('--report-only');

// --- Name-encoded address check -------------------------------------------

const NAME_HEX = /^[0-9A-Fa-f]+$/;

// Split a constant name into normalised address tokens. Consecutive hex
// segments are joined when a name splits a 4+4 address across underscores
// (`8043_C1B0`); a trailing 3-digit group is allowed so a truncated 7-digit
// token (`D_8029_BB8`) is still visible.
function addressTokens(name) {
  const parts = name.split('_');
  const out = [];
  for (let i = 0; i < parts.length; i++) {
    const first = parts[i];
    if (!NAME_HEX.test(first) || first.length < 4 || first.length > 8) continue;
    let joined = first.toUpperCase();
    let j = i + 1;
    while (
      j < parts.length &&
      joined.length < 8 &&
      NAME_HEX.test(parts[j]) &&
      parts[j].length >= 3 &&
      parts[j].length <= 4 &&
      joined.length + parts[j].length <= 8
    ) {
      joined += parts[j].toUpperCase();
      j++;
    }
    out.push(joined);
    i = j - 1;
  }
  return out;
}

const nameMismatches = [];
const nameAmbiguous = [];
let nameTokens = 0;

for (const file of files) {
  const lines = readFileSync(file, 'utf8').split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const m = /^\s*(?:pub\s+)?const\s+([A-Za-z0-9_]+)\s*:\s*usize\s*=\s*([^;]+);/.exec(lines[i]);
    if (!m) continue;
    const [, name, rawValue] = m;
    const value = constValue(rawValue);
    if (value === null || value < 0x80000000 || value > 0x8fffffff) continue;

    for (const text of addressTokens(name)) {
      const numeric = parseInt(text, 16);
      let kind;
      if (text.length === 8 && text.startsWith('8')) kind = 'addr';
      else if (text.length === 7 && text.startsWith('8')) kind = 'truncated';
      else if (text.length === 4 && numeric >= 0x8000 && numeric <= 0x8045) kind = 'page';
      else continue;
      nameTokens++;

      const page = value >>> 16;
      const tokenPage = text.length === 4 ? numeric : numeric >>> 16;
      const low16 = (a) => a & 0xffff;
      let verdict = null;
      let reason = '';
      if (kind === 'addr') {
        if (numeric === value) continue;
        if (low16(numeric) === low16(value)) {
          verdict = 'mismatch';
          reason = `name page 0x${tokenPage.toString(16)} but value page 0x${page.toString(16)}`;
        } else if (tokenPage === page) {
          verdict = 'ambiguous';
          reason = `same page 0x${page.toString(16)} but name offset 0x${low16(numeric).toString(16)} != value offset 0x${low16(value).toString(16)}`;
        } else {
          verdict = 'ambiguous';
          reason = `name 0x${text} and value 0x${value.toString(16)} disagree (provenance or target name)`;
        }
      } else if (kind === 'truncated') {
        if (low16(numeric) === low16(value)) {
          verdict = 'mismatch';
          reason = `7-digit name token 0x${text} vs value page 0x${page.toString(16)} (dropped hex digit?)`;
        } else {
          verdict = 'ambiguous';
          reason = `7-digit name token 0x${text} does not match value 0x${value.toString(16)}`;
        }
      } else {
        if (numeric === page) continue;
        verdict = 'ambiguous';
        reason = `name page 0x${numeric.toString(16)} but value page 0x${page.toString(16)} (page-only token)`;
      }

      const finding = {
        file,
        line: i + 1,
        name,
        token: `0x${text}`,
        value: `0x${value.toString(16)}`,
        reason,
      };
      if (verdict === 'mismatch') nameMismatches.push(finding);
      else nameAmbiguous.push(finding);
    }
  }
}

console.log(
  `page-offset: ${checked} documented lis/addi pairs, ${ambiguous} with a negative immediate, ` +
    `${findings.length} mismatch${findings.length === 1 ? '' : 'es'}`,
);
for (const f of findings) {
  console.log(
    `  ${f.file}:${f.line} ${f.name}: lis page ${f.docPage} + addi ${f.imm} -> ${f.expected}, declared ${f.actual}`,
  );
}
console.log(
  `name-address: ${nameTokens} address-shaped name tokens, ${nameMismatches.length} mismatch` +
    `${nameMismatches.length === 1 ? '' : 'es'}, ${nameAmbiguous.length} ambiguous (report only)`,
);
for (const f of nameMismatches) {
  console.log(`  ${f.file}:${f.line} ${f.name}: name token ${f.token} vs value ${f.value} - ${f.reason}`);
}
for (const f of nameAmbiguous) {
  console.log(`  (report-only) ${f.file}:${f.line} ${f.name}: name token ${f.token} vs value ${f.value} - ${f.reason}`);
}
if ((findings.length || nameMismatches.length) && enforce) process.exit(1);
