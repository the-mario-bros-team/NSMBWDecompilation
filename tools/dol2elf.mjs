#!/usr/bin/env node
// Convert a Wii DOL (PowerPC big-endian) into a loadable ELF32 BE (EM_PPC)
// so Ghidra/REA can analyze it.
// Usage: node dol2elf.mjs <main.dol> <out.elf>
import fs from 'node:fs';

const [input, output] = process.argv.slice(2);
if (!input || !output) {
  console.error('usage: node dol2elf.mjs <main.dol> <out.elf>');
  process.exit(2);
}

const dol = fs.readFileSync(input);
const be32 = (o) => dol.readUInt32BE(o);

const textOff = [], dataOff = [], textAddr = [], dataAddr = [], textSize = [], dataSize = [];
for (let i = 0; i < 7; i++) {
  textOff.push(be32(0x00 + i * 4));
  textAddr.push(be32(0x48 + i * 4));
  textSize.push(be32(0x90 + i * 4));
}
for (let i = 0; i < 11; i++) {
  dataOff.push(be32(0x1c + i * 4));
  dataAddr.push(be32(0x64 + i * 4));
  dataSize.push(be32(0xac + i * 4));
}
const bssAddr = be32(0xd8);
const bssSize = be32(0xdc);
const entry = be32(0xe0);

const segments = [];
for (let i = 0; i < 7; i++) {
  if (textSize[i]) segments.push({ kind: 'text', index: i, fileOff: textOff[i], addr: textAddr[i], size: textSize[i], flags: 5 });
}
for (let i = 0; i < 11; i++) {
  if (dataSize[i]) segments.push({ kind: 'data', index: i, fileOff: dataOff[i], addr: dataAddr[i], size: dataSize[i], flags: 6 });
}
if (bssSize) {
  // Some DOLs (NSMBW included) place initialized data sections inside the
  // nominal BSS range. Split BSS around them so ELF segments never overlap.
  const start = bssAddr >>> 0;
  const end = (bssAddr + bssSize) >>> 0;
  const covered = segments
    .filter((s) => s.addr < end && s.addr + s.size > start)
    .sort((a, b) => a.addr - b.addr);
  let cursor = start;
  let frag = 0;
  for (const s of covered) {
    if (s.addr > cursor) segments.push({ kind: 'bss', index: frag++, fileOff: 0, addr: cursor, size: s.addr - cursor, flags: 6 });
    cursor = Math.max(cursor, s.addr + s.size);
  }
  if (cursor < end) segments.push({ kind: 'bss', index: frag++, fileOff: 0, addr: cursor, size: end - cursor, flags: 6 });
}

// sanity: source ranges must fit the DOL
for (const s of segments) {
  if (s.kind !== 'bss' && s.fileOff + s.size > dol.length) {
    throw new Error(`${s.kind}${s.index} exceeds DOL: 0x${s.fileOff.toString(16)}+0x${s.size.toString(16)}`);
  }
}
segments.sort((a, b) => a.addr - b.addr);

const ehsize = 52;
const phentsize = 32;
const phnum = segments.length;
const headerBytes = ehsize + phentsize * phnum;
const headerSize = Math.ceil(headerBytes / 0x100) * 0x100;

// assign output offsets
let cursor = headerSize;
for (const s of segments) {
  if (s.kind === 'bss') { s.outOff = 0; continue; }
  cursor = Math.ceil(cursor / 0x100) * 0x100;
  s.outOff = cursor;
  cursor += s.size;
}
const totalSize = cursor;
const elf = Buffer.alloc(totalSize);

// ELF header (ELF32, big-endian)
elf[0] = 0x7f; elf[1] = 0x45; elf[2] = 0x4c; elf[3] = 0x46; // \x7fELF
elf[4] = 1; // ELFCLASS32
elf[5] = 2; // ELFDATA2MSB
elf[6] = 1; // EV_CURRENT
elf.writeUInt16BE(2, 16); // ET_EXEC
elf.writeUInt16BE(20, 18); // EM_PPC
elf.writeUInt32BE(1, 20); // e_version
elf.writeUInt32BE(entry >>> 0, 24);
elf.writeUInt32BE(ehsize, 28); // e_phoff
elf.writeUInt32BE(0, 32); // e_shoff
elf.writeUInt32BE(0, 36); // e_flags
elf.writeUInt16BE(ehsize, 40);
elf.writeUInt16BE(phentsize, 42);
elf.writeUInt16BE(phnum, 44);
elf.writeUInt16BE(40, 46);
elf.writeUInt16BE(0, 48);
elf.writeUInt16BE(0, 50);

let ph = ehsize;
for (const s of segments) {
  elf.writeUInt32BE(1, ph); // PT_LOAD
  elf.writeUInt32BE(s.outOff, ph + 4);
  elf.writeUInt32BE(s.addr >>> 0, ph + 8);
  elf.writeUInt32BE(s.addr >>> 0, ph + 12);
  elf.writeUInt32BE(s.kind === 'bss' ? 0 : s.size, ph + 16);
  elf.writeUInt32BE(s.size, ph + 20);
  elf.writeUInt32BE(s.flags, ph + 24);
  elf.writeUInt32BE(0x100, ph + 28);
  ph += phentsize;
}

for (const s of segments) {
  if (s.kind === 'bss') continue;
  dol.copy(elf, s.outOff, s.fileOff, s.fileOff + s.size);
}
fs.writeFileSync(output, elf);

console.log(JSON.stringify({
  input, output,
  dol_bytes: dol.length,
  elf_bytes: totalSize,
  entry: `0x${(entry >>> 0).toString(16)}`,
  segments: segments.map((s) => ({
    kind: `${s.kind}${s.kind === 'bss' ? '' : s.index}`,
    vaddr: `0x${(s.addr >>> 0).toString(16)}`,
    size: s.size,
    fileOff: `0x${s.outOff.toString(16)}`,
    flags: s.flags,
  })),
}, null, 2));
