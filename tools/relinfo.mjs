#!/usr/bin/env node
// Inspect a Wii REL: header, sections, imports, relocation counts.
// Usage: node relinfo.mjs <module.rel> [out.json]
import fs from 'node:fs';

const [file, out] = process.argv.slice(2);
if (!file) {
  console.error('usage: node relinfo.mjs <module.rel> [out.json]');
  process.exit(2);
}
const b = fs.readFileSync(file);
const u32 = (o) => b.readUInt32BE(o);
const u16 = (o) => b.readUInt16BE(o);

const h = {
  moduleId: u32(0x00),
  next: u32(0x04),
  prev: u32(0x08),
  numSections: u32(0x0c),
  sectionInfoOffset: u32(0x10),
  nameOffset: u32(0x14),
  nameSize: u32(0x18),
  version: u32(0x1c),
  bssSize: u32(0x20),
  relOffset: u32(0x24),
  impOffset: u32(0x28),
  impSize: u32(0x2c),
  prologSection: b[0x30],
  epilogSection: b[0x31],
  unresolvedSection: b[0x32],
  bssSection: b[0x33],
  prolog: u32(0x34),
  epilog: u32(0x38),
  unresolved: u32(0x3c),
  align: u32(0x40),
  bssAlign: u32(0x44),
  fixSize: u32(0x48),
};

const sections = [];
for (let i = 0; i < h.numSections; i++) {
  const o = h.sectionInfoOffset + i * 8;
  const rawOffset = u32(o);
  sections.push({
    index: i,
    rawOffset,
    offset: rawOffset & ~1,
    size: u32(o + 4),
    executable: (rawOffset & 1) === 1,
  });
}

// nameOffset refers to the split .str debug file, not the REL itself.
const name = null;

const relocTypes = {};
let relocTotal = 0;
let relocEntries = 0;
const imports = [];
for (let p = h.impOffset; p < h.impOffset + h.impSize; p += 8) {
  const moduleId = u32(p);
  const relOffset = u32(p + 4);
  let q = relOffset;
  let count = 0;
  let targetSection = 0;
  let cursor = 0;
  while (q + 8 <= b.length) {
    const delta = u16(q);
    const type = b[q + 2];
    const section = b[q + 3];
    const addend = u32(q + 4);
    if (type === 203) break; // R_RVL_STOP
    if (type === 202) { // R_RVL_SECT
      targetSection = section;
      cursor = 0;
    } else if (type === 201) { // R_RVL_NONE (offset accumulator)
      cursor += delta;
    } else if (type !== 0) {
      cursor += delta;
      relocTypes[type] = (relocTypes[type] ?? 0) + 1;
      count++; relocTotal++; relocEntries++;
    } else {
      cursor += delta;
      relocEntries++;
    }
    q += 8;
  }
  imports.push({ moduleId, relOffset, relocations: count, targetSection });
}

const info = {
  file,
  bytes: b.length,
  name,
  header: h,
  sections: sections.map((s) => ({
    ...s,
    role: s.index === 0 ? 'null' : (s.executable ? 'text' : 'data'),
  })),
  imports,
  relocTypes,
  relocTotal,
  relocEntries,
};

const json = JSON.stringify(info, null, 2);
if (out) fs.writeFileSync(out, json);
console.log(json);
