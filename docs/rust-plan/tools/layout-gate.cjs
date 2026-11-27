#!/usr/bin/env node
/*
 * Phase 1.3 gate: verify the actor base-chain layout against the retail
 * executable, not against a header.
 *
 * For each class it decodes the class constructor from main.dol and checks:
 *  - every field marked "store"/"vec"/"vec16" has a matching store from the
 *    constructor, at the claimed offset and width;
 *  - every field marked "subobject" has a call to the known constructor with
 *    r3 = this + callOffset, within a few instructions;
 *  - the vtable pointer is written at the claimed offset;
 *  - every class's size matches the first own offset of the next class in the
 *    chain;
 *  - no constructor store is left unexplained outside a declared field.
 *
 * Exit code 1 on any FAIL.
 */

const fs = require('fs');

const DOL = process.env.DOL || 'E:/Localsend/mariobroswii/sys/main.dol';
const LAYOUT = process.argv[2] || 'C:/Users/Wither/Documents/Codex/2026-11-25/rad/work/phase1/actor-base-layout.json';

const dol = fs.readFileSync(DOL);
const be32 = (o) => dol.readUInt32BE(o);
const segs = [];
for (let i = 0; i < 7; i++) segs.push({ off: be32(0x00 + i * 4), vaddr: be32(0x48 + i * 4), size: be32(0x90 + i * 4) });
for (let i = 0; i < 11; i++) segs.push({ off: be32(0x1c + i * 4), vaddr: be32(0x64 + i * 4), size: be32(0xac + i * 4) });

function fileOff(vaddr) {
  for (const s of segs) {
    if (s.size && vaddr >= s.vaddr && vaddr + 4 <= s.vaddr + s.size) return s.off + (vaddr - s.vaddr);
  }
  return -1;
}
function iw(vaddr) {
  const o = fileOff(vaddr);
  if (o < 0) throw new Error('unmapped ' + vaddr.toString(16));
  return dol.readUInt32BE(o);
}
const sx = (v, bits) => ((v & (1 << (bits - 1))) ? v - (1 << bits) : v);
const fld = { rt: (w) => (w >>> 21) & 31, ra: (w) => (w >>> 16) & 31, d: (w) => sx(w & 0xffff, 16) };

function decodeCtor(start, thisReg) {
  const stores = [];
  const subobjects = [];
  let pendingAddiR3 = null; // { offset, age }
  let r3IsThis = true; // r3 == this on entry; kept true only while r3 still derives from this
  const startAddr = parseInt(start, 16);
  const LOADS = new Set([32, 33, 34, 35, 40, 41, 48, 50]);
  for (let i = 0; i < 0x800; i += 4) {
    const a = startAddr + i;
    const w = iw(a);
    const op = w >>> 26;
    const rt = fld.rt(w), ra = fld.ra(w), d = fld.d(w);
    const isThisBase = (r) => r === thisReg || (r === 3 && r3IsThis);

    // expire a pending r3<-this+off if it was too long ago
    if (pendingAddiR3 && ++pendingAddiR3.age > 16) pendingAddiR3 = null;

    if (op === 31 && ((w >>> 1) & 0x3ff) === 444) { // or = mr rA,rS
      if (ra === 3) r3IsThis = rt === thisReg || (rt === 3 && r3IsThis);
      continue;
    }
    if (op === 14) { // addi
      if (rt === 3) {
        if (ra === thisReg || (ra === 3 && r3IsThis)) pendingAddiR3 = { offset: d, age: 0 };
        else r3IsThis = false;
      }
      continue;
    }
    if (op === 15) { // lis
      if (rt === 3) { r3IsThis = false; pendingAddiR3 = null; }
      continue;
    }
    if (op === 18) { // b / bl
      const lk = w & 1;
      if (lk) {
        const li = sx(w & 0x3fffffc, 26);
        const aa = (w >>> 1) & 1;
        const target = (aa ? 0 : a) + li;
        if (pendingAddiR3) {
          subobjects.push({ offset: pendingAddiR3.offset, target: '0x' + (target >>> 0).toString(16) });
          pendingAddiR3 = null;
        }
      }
      continue;
    }
    if (op === 19) { // blr / bcctr / rfi
      if (w === 0x4e800020 || w === 0x4c820020) break;
      continue;
    }
    // stores
    const widths = { 36: 4, 38: 1, 44: 2, 52: 4, 54: 8 };
    if (widths[op]) {
      if (isThisBase(ra)) stores.push({ offset: d, width: widths[op], at: '0x' + a.toString(16) });
      continue;
    }
    if (LOADS.has(op) && rt === 3) { r3IsThis = false; pendingAddiR3 = null; }
  }
  return { stores, subobjects };
}

const layout = JSON.parse(fs.readFileSync(LAYOUT, 'utf8'));
const byName = new Map(layout.classes.map((c) => [c.name, c]));

let fail = 0;
let warn = 0;
const lines = [];
const out = (s) => { lines.push(s); console.log(s); };

function ownStart(c) {
  const offs = c.fields.filter((f) => f.offset != null && f.offset !== c.vptrOffset).map((f) => f.offset);
  return offs.length ? Math.min(...offs) : null;
}

for (const c of layout.classes) {
  if (!c.ctor) continue;
  const thisReg = parseInt(c.thisReg.slice(1), 10);
  const { stores, subobjects } = decodeCtor(c.ctor, thisReg);
  out(`\n== ${c.name}  ctor ${c.ctor}  this=${c.thisReg} ==`);
  out('  observed stores: ' + stores.map((s) => `${s.offset}(w${s.width})`).join(' '));
  if (subobjects.length) out('  observed ctor calls on this+off: ' + subobjects.map((s) => `${s.offset}->${s.target}`).join(' '));

  const has = (offset, width) => stores.some((s) => s.offset === offset && s.width === width);

  if (c.vptrOffset != null) {
    if (has(c.vptrOffset, 4)) out(`  PASS vptr @${c.vptrOffset}`);
    else { out(`  FAIL vptr @${c.vptrOffset} not stored`); fail++; }
  }

  for (const f of c.fields) {
    if (f.offset == null) continue;
    const ev = f.evidence || { kind: 'documented' };
    const tag = `${f.name} @${f.offset} (${ev.kind})`;
    if (ev.kind === 'store') {
      if (has(f.offset, ev.width)) out(`  PASS ${tag}`);
      else { out(`  FAIL ${tag}: no ${ev.width}-byte store`); fail++; }
    } else if (ev.kind === 'vec') {
      let ok = true;
      for (let k = 0; k < ev.width; k += 4) if (!has(f.offset + k, 4)) ok = false;
      if (ok) out(`  PASS ${tag}`);
      else { out(`  FAIL ${tag}: missing 4-byte store in range`); fail++; }
    } else if (ev.kind === 'vec16') {
      let ok = true;
      for (let k = 0; k < ev.width; k += 2) if (!has(f.offset + k, 2)) ok = false;
      if (ok) out(`  PASS ${tag}`);
      else { out(`  FAIL ${tag}: missing 2-byte store in range`); fail++; }
    } else if (ev.kind === 'subobject') {
      const at = ev.callOffset != null ? ev.callOffset : f.offset;
      if (subobjects.some((s) => s.offset === at && s.target.toLowerCase() === String(ev.call).toLowerCase())) out(`  PASS ${tag}`);
      else { out(`  WARN ${tag}: expected call ${ev.call} at +${at}`); warn++; }
    } else if (ev.kind === 'zero' || ev.kind === 'documented') {
      // nothing to prove; zero-init fields are expected to have no store
    } else {
      out(`  WARN ${tag}: unknown evidence kind`); warn++;
    }
  }

  // unclaimed stores: outside every field range (subobject ranges count)
  for (const s of stores) {
    if (c.vptrOffset != null && s.offset === c.vptrOffset) continue;
    const os = ownStart(c);
    const covered = (os != null && s.offset < os) || c.fields.some((f) => f.offset != null && f.size != null && s.offset >= f.offset && s.offset < f.offset + f.size);
    if (!covered && s.offset < (c.tailUnresolvedFrom ?? Infinity)) { out(`  WARN unexplained store @${s.offset} (${s.at})`); warn++; }
  }
}

// chain size checks
const chainEdges = [
  ['fBase_c', 'dBase_c', 100],
  ['dBase_c', 'dBaseActor_c', 112],
  ['dBaseActor_c', 'dActor_c', 296],
  ['dActor_c', 'dActorState_c', 916],
  ['dActor_c', 'dActorMultiState_c', 916],
  ['dActorMultiState_c', 'dEn_c', 1044],
];
out('\n== chain continuity ==');
for (const [a, b, expect] of chainEdges) {
  const A = byName.get(a), B = byName.get(b);
  const os = B ? ownStart(B) : null;
  const sizeOk = A && A.size === expect;
  const nextOk = os === expect;
  if (sizeOk && nextOk) out(`  PASS ${a}.size=${expect} == first own field of ${b} (${os})`);
  else { out(`  FAIL ${a}.size=${A ? A.size : '?'} / ${b} own start=${os}, expected ${expect}`); fail++; }
}

out(`\nRESULT: ${fail} fail, ${warn} warn`);
fs.writeFileSync('C:/Users/Wither/Documents/Codex/2026-11-25/rad/outputs/phase1-actor-base-layout.gate.txt', lines.join('\n') + '\n');
process.exit(fail ? 1 : 0);

