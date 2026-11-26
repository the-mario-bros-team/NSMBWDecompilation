#!/usr/bin/env node
// Convert a Wii REL module into a loadable ELF32 big-endian EM_PPC image.
//
// The REL is already decompressed; this tool does not touch LZ11.  It uses
// synthetic, documented section bases (docs/REL.md) because the real runtime
// bases are chosen by the game's loader and are not stored in the REL.
//
// Usage:
//   node tools/rel2elf.mjs --all
//   node tools/rel2elf.mjs <module.rel> [out.elf]
//   node tools/rel2elf.mjs --resolve-address <summary.json> <0xaddress>...
//   node tools/rel2elf.mjs --address-layout <module.rel|module.rel.LZ>...
//
// --all converts the four NSMBW RELs in work/rels/ and writes
// work/rels/elf/summary.json.  A single-file conversion prints its JSON
// summary and writes only the requested ELF.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { decodeLz11 } from './lz11.mjs';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(HERE, '..');
const REL_DIR = path.join(ROOT, 'work', 'rels');
const ELF_DIR = path.join(REL_DIR, 'elf');

const EHDR_SIZE = 52;
const PHDR_SIZE = 32;
const PT_LOAD = 1;
const PF_X = 1;
const PF_W = 2;
const PF_R = 4;
const PAGE_ALIGN = 0x1000;

// Canonical analysis slots, from docs/REL.md.  These are not runtime bases.
const CANONICAL_MODULES = {
  1: { name: 'd_profileNP', base: 0x80500000 },
  2: { name: 'd_basesNP', base: 0x80600000 },
  3: { name: 'd_enemiesNP', base: 0x80a00000 },
  4: { name: 'd_en_bossNP', base: 0x80c00000 },
};
const MODULE_ORDER = [1, 2, 3, 4];

// PPC ELF relocation types used by Wii RELs.  Types 8/9 and 12/13 are the
// branch-prediction variants; the REL format stores the same 14-bit field, so
// they are patched like ADDR14/REL14 without synthesising prediction hints.
const RELOC_TYPES = {
  1: { name: 'ADDR32', width: 4 },
  2: { name: 'ADDR24', width: 4 },
  3: { name: 'ADDR16', width: 2 },
  4: { name: 'ADDR16_LO', width: 2 },
  5: { name: 'ADDR16_HI', width: 2 },
  6: { name: 'ADDR16_HA', width: 2 },
  7: { name: 'ADDR14', width: 4 },
  8: { name: 'ADDR14_BRTAKEN', width: 4 },
  9: { name: 'ADDR14_BRNTAKEN', width: 4 },
  10: { name: 'REL24', width: 4 },
  11: { name: 'REL14', width: 4 },
  12: { name: 'REL14_BRTAKEN', width: 4 },
  13: { name: 'REL14_BRNTAKEN', width: 4 },
};

const hex = (value, digits = 8) =>
  `0x${(value >>> 0).toString(16).padStart(digits, '0')}`;

const alignUp = (value, alignment) =>
  Math.ceil(value / alignment) * alignment;

function readHeader(buffer) {
  if (buffer.length < 0x4c) {
    throw new Error(`REL is too short for a Wii REL header (${buffer.length} bytes)`);
  }
  return {
    moduleId: buffer.readUInt32BE(0x00),
    next: buffer.readUInt32BE(0x04),
    prev: buffer.readUInt32BE(0x08),
    numSections: buffer.readUInt32BE(0x0c),
    sectionInfoOffset: buffer.readUInt32BE(0x10),
    nameOffset: buffer.readUInt32BE(0x14),
    nameSize: buffer.readUInt32BE(0x18),
    version: buffer.readUInt32BE(0x1c),
    bssSize: buffer.readUInt32BE(0x20),
    relOffset: buffer.readUInt32BE(0x24),
    impOffset: buffer.readUInt32BE(0x28),
    impSize: buffer.readUInt32BE(0x2c),
    prologSection: buffer[0x30],
    epilogSection: buffer[0x31],
    unresolvedSection: buffer[0x32],
    bssSection: buffer[0x33],
    prolog: buffer.readUInt32BE(0x34),
    epilog: buffer.readUInt32BE(0x38),
    unresolved: buffer.readUInt32BE(0x3c),
    align: buffer.readUInt32BE(0x40),
    bssAlign: buffer.readUInt32BE(0x44),
    fixSize: buffer.readUInt32BE(0x48),
  };
}

function parseRel(buffer, label) {
  const header = readHeader(buffer);
  if (header.numSections > 0x1000) {
    throw new Error(`${label}: implausible section count ${header.numSections}`);
  }
  if (header.sectionInfoOffset + header.numSections * 8 > buffer.length) {
    throw new Error(`${label}: section table exceeds file`);
  }
  if (header.impOffset + header.impSize > buffer.length) {
    throw new Error(`${label}: import table exceeds file`);
  }

  const sections = [];
  for (let i = 0; i < header.numSections; i++) {
    const entry = header.sectionInfoOffset + i * 8;
    const rawOffset = buffer.readUInt32BE(entry);
    const offset = (rawOffset & 0xfffffffe) >>> 0;
    sections.push({
      index: i,
      rawOffset,
      offset,
      size: buffer.readUInt32BE(entry + 4),
      executable: (rawOffset & 1) === 1,
      isBss: false,
      canonicalOffset: offset,
      elfOffset: null,
    });
  }

  // Wii REL: an entry with file offset 0 and size > 0 is the BSS section.
  const bssCandidates = sections.filter(
    (s) => s.index !== 0 && s.offset === 0 && s.size > 0,
  );
  if (bssCandidates.length > 1) {
    throw new Error(`${label}: multiple BSS-like sections (${bssCandidates.map((s) => s.index).join(', ')})`);
  }
  const bssIndex = bssCandidates.length ? bssCandidates[0].index : -1;
  if (bssIndex >= 0) sections[bssIndex].isBss = true;

  const warnings = [];
  let maxEnd = 0;
  for (const s of sections) {
    if (s.index === 0 || s.isBss || s.size === 0) continue;
    if (s.offset + s.size > buffer.length) {
      throw new Error(
        `${label}: section ${s.index} exceeds file ` +
        `(0x${s.offset.toString(16)}+0x${s.size.toString(16)} > 0x${buffer.length.toString(16)})`,
      );
    }
    maxEnd = Math.max(maxEnd, s.offset + s.size);
  }

  // Check section data ranges do not overlap.  BSS is placed separately.
  const live = sections
    .filter((s) => s.index !== 0 && s.size > 0 && !s.isBss)
    .sort((a, b) => a.offset - b.offset);
  for (let i = 1; i < live.length; i++) {
    const prev = live[i - 1];
    const cur = live[i];
    if (cur.offset < prev.offset + prev.size) {
      throw new Error(
        `${label}: sections ${prev.index} and ${cur.index} overlap ` +
        `(0x${prev.offset.toString(16)}+0x${prev.size.toString(16)} vs 0x${cur.offset.toString(16)})`,
      );
    }
  }

  if (bssIndex >= 0 && header.bssSize !== sections[bssIndex].size) {
    warnings.push(
      `BSS table size 0x${sections[bssIndex].size.toString(16)} differs from ` +
      `header bssSize 0x${header.bssSize.toString(16)}; using header bssSize`,
    );
  }
  if (header.bssAlign > 30) {
    throw new Error(`${label}: unsupported bssAlign ${header.bssAlign}`);
  }
  const bssAlign = 1 << header.bssAlign;
  const bssOffset = header.bssSize > 0 ? alignUp(maxEnd, bssAlign) : 0;
  if (bssIndex >= 0) sections[bssIndex].canonicalOffset = bssOffset;
  if (header.bssSize > 0 && bssOffset < maxEnd) {
    throw new Error(`${label}: BSS placement overlaps initialized sections`);
  }

  const bss = {
    sectionIndex: bssIndex >= 0 ? bssIndex : null,
    fileOffset: 0,
    offset: bssOffset,
    size: header.bssSize,
    align: bssAlign,
  };

  return {
    label,
    size: buffer.length,
    header,
    sections,
    bss,
    maxEnd,
    warnings,
    buffer,
  };
}

// Cache only canonical module metadata.  Each conversion parses its own input
// file again, so a single-file conversion never depends on a stale cache entry.
const canonicalCache = new Map();

function loadCanonicalModule(moduleId) {
  if (moduleId === 0) return { meta: null };
  const def = CANONICAL_MODULES[moduleId];
  if (!def) return { error: `unknown module id ${moduleId}` };
  if (canonicalCache.has(moduleId)) return canonicalCache.get(moduleId);

  let result;
  try {
    const file = path.join(REL_DIR, `${def.name}.rel`);
    const buf = fs.readFileSync(file);
    const meta = parseRel(buf, file);
    if (meta.header.moduleId !== moduleId) {
      throw new Error(
        `${file}: header module id ${meta.header.moduleId}, expected ${moduleId}`,
      );
    }
    meta.name = def.name;
    meta.base = def.base;
    meta.file = file;
    result = { meta };
  } catch (error) {
    result = { error: error.message };
  }
  canonicalCache.set(moduleId, result);
  return result;
}

function resolveSymbol(moduleId, sectionIndex, addend, current) {
  if (moduleId === 0) {
    return { address: addend >>> 0, module: 'main.dol', section: sectionIndex };
  }
  let meta;
  if (moduleId === current.header.moduleId) {
    meta = current;
  } else {
    const loaded = loadCanonicalModule(moduleId);
    if (loaded.error) return { error: loaded.error };
    meta = loaded.meta;
  }
  const section = meta.sections[sectionIndex];
  if (!section || section.size === 0) {
    return { error: `symbol section ${sectionIndex} is invalid in module ${moduleId}` };
  }
  const sectionSize = section.isBss ? meta.bss.size : section.size;
  if (sectionSize === 0) {
    return { error: `symbol section ${sectionIndex} is empty in module ${moduleId}` };
  }
  const address = meta.base + section.canonicalOffset + addend;
  if (address > 0xffffffff) {
    return { error: `symbol address overflows 32 bits for module ${moduleId}` };
  }
  return {
    address: address >>> 0,
    module: meta.name ?? `module${moduleId}`,
    section: sectionIndex,
    sectionSize,
    isBss: section.isBss,
  };
}

function writeRelocation(elf, offset, type, symbolAddress, targetAddress) {
  const old =
    type === 2 || type === 7 || type === 8 || type === 9 ||
    type === 10 || type === 11 || type === 12 || type === 13
      ? elf.readUInt32BE(offset)
      : 0;

  switch (type) {
    case 1: // ADDR32
      elf.writeUInt32BE(symbolAddress >>> 0, offset);
      return;
    case 2: // ADDR24
      elf.writeUInt32BE(
        ((old & 0xfc000003) | (symbolAddress & 0x03fffffc)) >>> 0,
        offset,
      );
      return;
    case 3: // ADDR16
    case 4: // ADDR16_LO
      elf.writeUInt16BE(symbolAddress & 0xffff, offset);
      return;
    case 5: // ADDR16_HI
      elf.writeUInt16BE((symbolAddress >>> 16) & 0xffff, offset);
      return;
    case 6: // ADDR16_HA
      elf.writeUInt16BE(
        (((symbolAddress >>> 16) + ((symbolAddress & 0x8000) !== 0 ? 1 : 0)) & 0xffff),
        offset,
      );
      return;
    case 7: // ADDR14
    case 8: // ADDR14_BRTAKEN
    case 9: // ADDR14_BRNTAKEN
      elf.writeUInt32BE(
        ((old & 0xffff0003) | (symbolAddress & 0x0000fffc)) >>> 0,
        offset,
      );
      return;
    case 10: { // REL24
      const delta = (symbolAddress - targetAddress) >>> 0;
      elf.writeUInt32BE(((old & 0xfc000003) | (delta & 0x03fffffc)) >>> 0, offset);
      return;
    }
    case 11: // REL14
    case 12: // REL14_BRTAKEN
    case 13: { // REL14_BRNTAKEN
      const delta = (symbolAddress - targetAddress) >>> 0;
      elf.writeUInt32BE(((old & 0xffff0003) | (delta & 0x0000fffc)) >>> 0, offset);
      return;
    }
    default:
      throw new Error(`internal error: no writer for relocation type ${type}`);
  }
}

function relocationFitError(type, symbolAddress, targetAddress) {
  switch (type) {
    case 2: { // ADDR24: (S >> 2) must fit the 24-bit field
      if ((symbolAddress & 3) !== 0) return 'ADDR24 symbol is not word-aligned';
      if (symbolAddress > 0x03fffffc) {
        return `ADDR24 symbol ${hex(symbolAddress)} exceeds 0x03fffffc`;
      }
      return null;
    }
    case 3: // ADDR16
      if (symbolAddress > 0xffff) {
        return `ADDR16 symbol ${hex(symbolAddress)} exceeds 0xffff`;
      }
      return null;
    case 7: // ADDR14
    case 8: // ADDR14_BRTAKEN
    case 9: { // ADDR14_BRNTAKEN
      if ((symbolAddress & 3) !== 0) return 'ADDR14 symbol is not word-aligned';
      if (symbolAddress > 0x0000fffc) {
        return `ADDR14 symbol ${hex(symbolAddress)} exceeds 0x0000fffc`;
      }
      return null;
    }
    case 10: { // REL24
      const delta = (symbolAddress - targetAddress) >>> 0;
      const low = delta & 0xffffff;
      const signed = low & 0x800000 ? low - 0x1000000 : low;
      if (signed < -0x800000 || signed > 0x7fffff) {
        return `REL24 displacement ${signed} is outside the signed 24-bit range`;
      }
      return null;
    }
    case 11: // REL14
    case 12: // REL14_BRTAKEN
    case 13: { // REL14_BRNTAKEN
      const delta = (symbolAddress - targetAddress) >>> 0;
      const low = delta & 0x3fff;
      const signed = low & 0x2000 ? low - 0x4000 : low;
      if (signed < -0x2000 || signed > 0x1fff) {
        return `REL14 displacement ${signed} is outside the signed 14-bit range`;
      }
      return null;
    }
    default:
      return null;
  }
}

function createRelocationStats() {
  return {
    streams: 0,
    entries: 0, // every 8-byte entry, including control entries
    relocationEntries: 0, // entries with a real/unknown relocation type
    applied: 0,
    failed: 0,
    unknown: 0,
    appliedByType: {},
    attemptedByType: {},
    failedByType: {},
    unknownTypes: {},
    controls: {
      type0: 0,
      none201: 0,
      section202: 0,
      stop203: 0,
      noStopStreams: 0,
      section202WithDelta: 0,
    },
    warnings: {
      symbolAddendOutsideSection: 0,
      module0AddendOutsideMain: 0,
    },
    module0AddendMin: null,
    module0AddendMax: null,
    failures: [],
    imports: [],
  };
}

function addCount(object, key) {
  object[key] = (object[key] ?? 0) + 1;
}

function applyRelocations(elf, mod) {
  const stats = createRelocationStats();
  const buffer = mod.buffer;
  const impOffset = mod.header.impOffset;
  const impSize = mod.header.impSize;

  for (let p = impOffset; p < impOffset + impSize; p += 8) {
    const moduleId = buffer.readUInt32BE(p);
    const streamOffset = buffer.readUInt32BE(p + 4);
    const importStats = {
      moduleId,
      relOffset: streamOffset,
      entries: 0,
      relocationEntries: 0,
      applied: 0,
      failed: 0,
      unknown: 0,
      targetSections: [],
    };
    const targetSections = new Set();
    stats.streams++;
    stats.imports.push(importStats);

    let q = streamOffset;
    let cursor = 0;
    let targetSection = -1;
    let stopped = false;

    while (q + 8 <= buffer.length) {
      const delta = buffer.readUInt16BE(q);
      const type = buffer[q + 2];
      const symbolSection = buffer[q + 3];
      const addend = buffer.readUInt32BE(q + 4);
      const entryOffset = q;
      q += 8;

      stats.entries++;
      importStats.entries++;

      if (type === 203) { // R_RVL_STOP
        stats.controls.stop203++;
        stopped = true;
        break;
      }
      if (type === 202) { // R_RVL_SECT: switch target section, reset cursor
        stats.controls.section202++;
        if (delta !== 0) stats.controls.section202WithDelta++;
        targetSection = symbolSection;
        targetSections.add(symbolSection);
        cursor = 0;
        continue;
      }
      if (type === 201) { // R_RVL_NONE: advance the offset accumulator
        stats.controls.none201++;
        cursor += delta;
        continue;
      }
      if (type === 0) { // R_PPC_NONE
        stats.controls.type0++;
        cursor += delta;
        continue;
      }

      cursor += delta;
      stats.relocationEntries++;
      importStats.relocationEntries++;
      addCount(stats.attemptedByType, type);
      const typeInfo = RELOC_TYPES[type];

      if (!typeInfo) {
        stats.unknown++;
        importStats.unknown++;
        addCount(stats.unknownTypes, type);
        continue;
      }

      const fail = (reason, detail) => {
        stats.failed++;
        importStats.failed++;
        addCount(stats.failedByType, type);
        if (stats.failures.length < 100) {
          stats.failures.push({
            moduleId,
            streamOffset,
            entryOffset,
            type,
            typeName: typeInfo.name,
            symbolSection,
            targetSection,
            targetOffset: cursor,
            reason,
            ...(detail ? { detail } : {}),
          });
        }
      };

      if (moduleId === 0) {
        if (stats.module0AddendMin === null || addend < stats.module0AddendMin) {
          stats.module0AddendMin = addend;
        }
        if (stats.module0AddendMax === null || addend > stats.module0AddendMax) {
          stats.module0AddendMax = addend;
        }
        // main.dol has no REL section table in this workspace.  A rough sanity
        // check catches an accidentally section-relative value without
        // rejecting legitimate main.dol BSS addresses.
        if (addend !== 0 && (addend < 0x80000000 || addend >= 0x81800000)) {
          stats.warnings.module0AddendOutsideMain++;
        }
      }

      const target = mod.sections[targetSection];
      if (!target || target.size === 0) {
        fail('target-section-invalid', `target section ${targetSection}`);
        continue;
      }
      if (target.isBss || target.elfOffset === null) {
        fail('target-section-bss', `target section ${targetSection} has no file bytes`);
        continue;
      }
      if (cursor < 0 || cursor + typeInfo.width > target.size) {
        fail(
          'target-out-of-bounds',
          `offset ${cursor} + ${typeInfo.width} > section ${targetSection} size ${target.size}`,
        );
        continue;
      }
      const targetAddress = mod.base + target.canonicalOffset + cursor;
      if (targetAddress > 0xffffffff) {
        fail('target-address-overflow', `0x${targetAddress.toString(16)}`);
        continue;
      }

      const symbol = resolveSymbol(moduleId, symbolSection, addend, mod);
      if (symbol.error) {
        fail('symbol-unresolved', symbol.error);
        continue;
      }
      if (moduleId !== 0 && symbol.sectionSize !== undefined &&
          addend >= symbol.sectionSize) {
        stats.warnings.symbolAddendOutsideSection++;
      }

      const fitError = relocationFitError(type, symbol.address, targetAddress);
      if (fitError) {
        fail('relocation-does-not-fit', fitError);
        continue;
      }

      const writeOffset = target.elfOffset + cursor;
      writeRelocation(
        elf,
        writeOffset,
        type,
        symbol.address,
        targetAddress >>> 0,
      );
      stats.applied++;
      importStats.applied++;
      addCount(stats.appliedByType, type);
    }

    if (!stopped) {
      stats.controls.noStopStreams++;
    }
    importStats.targetSections = [...targetSections].sort((a, b) => a - b);
  }

  return stats;
}

function sectionRole(section) {
  if (section.index === 0) return 'null';
  if (section.isBss) return 'bss';
  if (section.size === 0) return 'empty';
  return section.executable ? 'text' : 'data';
}

function computeEntry(mod) {
  const { prologSection, prolog } = mod.header;
  const section = mod.sections[prologSection];
  if (
    section &&
    section.size > 0 &&
    !section.isBss &&
    section.executable &&
    prolog < section.size
  ) {
    return {
      entry: (mod.base + section.canonicalOffset + prolog) >>> 0,
      fromProlog: true,
      reason: null,
    };
  }

  const text = mod.sections.find(
    (s) => s.index !== 0 && s.executable && !s.isBss && s.size > 0,
  );
  if (text) {
    return {
      entry: (mod.base + text.canonicalOffset) >>> 0,
      fromProlog: false,
      reason: `invalid prolog (section ${prologSection}, offset 0x${prolog.toString(16)})`,
    };
  }
  return {
    entry: mod.base >>> 0,
    fromProlog: false,
    reason: 'no executable section',
  };
}

function congruentFileOffset(cursor, vaddr, alignment) {
  const skew = ((vaddr % alignment) - (cursor % alignment) + alignment) % alignment;
  return cursor + skew;
}

function describeSections(mod) {
  return mod.sections
    .filter((section) => section.size > 0)
    .map((section) => {
      const isBss = section.isBss;
      const size = isBss ? mod.bss.size : section.size;
      return {
        index: section.index,
        role: sectionRole(section),
        fileOffset: isBss ? 0 : section.offset,
        size,
        executable: section.executable,
        canonicalOffset: section.canonicalOffset,
        vaddr: hex(mod.base + section.canonicalOffset),
      };
    });
}

function buildElf(relBuffer, { input, output, name }) {
  const mod = parseRel(relBuffer, input || name || '<buffer>');
  const def = CANONICAL_MODULES[mod.header.moduleId];
  if (!def) {
    throw new Error(
      `${input}: module id ${mod.header.moduleId} has no canonical base`,
    );
  }
  mod.name = name ?? def.name;
  mod.base = def.base;
  mod.input = input;
  mod.output = output;

  // Build one PT_LOAD per non-empty REL section.  Per-section segments keep
  // section VAs exactly base + file offset and make overlap checking trivial.
  const segments = [];
  for (const section of mod.sections) {
    if (section.index === 0 || section.size === 0 || section.isBss) continue;
    const vaddr = (mod.base + section.canonicalOffset) >>> 0;
    segments.push({
      kind: section.executable ? 'text' : 'data',
      sectionIndex: section.index,
      vaddr,
      memsz: section.size,
      filesz: section.size,
      flags: PF_R | (section.executable ? PF_X : PF_W),
      sourceOffset: section.offset,
      sourceSize: section.size,
      outOff: null,
      align: PAGE_ALIGN,
    });
    section.elfOffset = null; // assigned below
  }
  if (mod.bss.size > 0) {
    segments.push({
      kind: 'bss',
      sectionIndex: mod.bss.sectionIndex,
      vaddr: (mod.base + mod.bss.offset) >>> 0,
      memsz: mod.bss.size,
      filesz: 0,
      flags: PF_R | PF_W,
      sourceOffset: 0,
      sourceSize: 0,
      outOff: 0,
      align: PAGE_ALIGN,
    });
  }

  segments.sort((a, b) => a.vaddr - b.vaddr);
  // Memory ranges must not overlap.  (The section table was already checked
  // for initialized-section overlap; this also covers BSS placement.)
  for (let i = 1; i < segments.length; i++) {
    const prev = segments[i - 1];
    const cur = segments[i];
    if (cur.vaddr < prev.vaddr + prev.memsz) {
      throw new Error(
        `${input}: PT_LOAD ranges overlap at ${hex(prev.vaddr)}+${hex(prev.memsz)} ` +
        `and ${hex(cur.vaddr)}`,
      );
    }
    if (cur.vaddr + cur.memsz > 0x100000000) {
      throw new Error(`${input}: segment exceeds 32-bit address space`);
    }
  }

  const phnum = segments.length;
  const headerBytes = EHDR_SIZE + PHDR_SIZE * phnum;
  let cursor = headerBytes;
  for (const segment of segments) {
    if (segment.filesz === 0) {
      segment.outOff = 0;
      continue;
    }
    segment.outOff = congruentFileOffset(cursor, segment.vaddr, PAGE_ALIGN);
    cursor = segment.outOff + segment.filesz;
  }
  const totalSize = cursor;
  const elf = Buffer.alloc(totalSize);

  const entryInfo = computeEntry(mod);

  // ELF header.
  elf[0] = 0x7f;
  elf[1] = 0x45; // E
  elf[2] = 0x4c; // L
  elf[3] = 0x46; // F
  elf[4] = 1; // ELFCLASS32
  elf[5] = 2; // ELFDATA2MSB
  elf[6] = 1; // EV_CURRENT
  elf.writeUInt16BE(2, 16); // ET_EXEC
  elf.writeUInt16BE(20, 18); // EM_PPC
  elf.writeUInt32BE(1, 20); // e_version
  elf.writeUInt32BE(entryInfo.entry >>> 0, 24);
  elf.writeUInt32BE(EHDR_SIZE, 28); // e_phoff
  elf.writeUInt32BE(0, 32); // e_shoff
  elf.writeUInt32BE(0, 36); // e_flags
  elf.writeUInt16BE(EHDR_SIZE, 40);
  elf.writeUInt16BE(PHDR_SIZE, 42);
  elf.writeUInt16BE(phnum, 44);
  elf.writeUInt16BE(40, 46); // e_shentsize (no section headers are emitted)
  elf.writeUInt16BE(0, 48);
  elf.writeUInt16BE(0, 50);

  // Program headers.
  let ph = EHDR_SIZE;
  for (const segment of segments) {
    elf.writeUInt32BE(PT_LOAD, ph + 0);
    elf.writeUInt32BE(segment.outOff, ph + 4);
    elf.writeUInt32BE(segment.vaddr >>> 0, ph + 8);
    elf.writeUInt32BE(segment.vaddr >>> 0, ph + 12);
    elf.writeUInt32BE(segment.filesz, ph + 16);
    elf.writeUInt32BE(segment.memsz, ph + 20);
    elf.writeUInt32BE(segment.flags, ph + 24);
    elf.writeUInt32BE(segment.align, ph + 28);
    ph += PHDR_SIZE;
  }

  // Copy initialized section bytes, then patch relocations directly in the
  // ELF image so every target is also checked against the final file layout.
  for (const segment of segments) {
    if (segment.filesz === 0) continue;
    const section = mod.sections[segment.sectionIndex];
    if (segment.sourceOffset + segment.sourceSize > relBuffer.length) {
      throw new Error(`${input}: section ${section.index} exceeds REL file`);
    }
    relBuffer.copy(
      elf,
      segment.outOff,
      segment.sourceOffset,
      segment.sourceOffset + segment.sourceSize,
    );
    section.elfOffset = segment.outOff;
  }

  const relocations = applyRelocations(elf, mod);

  // Summary-facing section list.  Empty/null sections are counted separately.
  const sections = describeSections(mod);
  const emptySections = mod.sections
    .filter((section) => section.index !== 0 && !section.isBss && section.size === 0)
    .map((section) => section.index);

  const segmentSummary = segments.map((segment) => ({
    kind: segment.kind,
    sectionIndex: segment.sectionIndex,
    vaddr: hex(segment.vaddr),
    memsz: segment.memsz,
    filesz: segment.filesz,
    flags: segment.flags,
    fileOff: hex(segment.outOff),
    align: hex(segment.align, 3),
  }));

  const validation = validateElf(elf, segments, mod, entryInfo);

  fs.mkdirSync(path.dirname(output), { recursive: true });
  fs.writeFileSync(output, elf);

  return {
    name: mod.name,
    moduleId: mod.header.moduleId,
    input,
    output,
    relBytes: relBuffer.length,
    elfBytes: elf.length,
    sha256: crypto.createHash('sha256').update(elf).digest('hex'),
    base: hex(mod.base),
    header: {
      version: mod.header.version,
      numSections: mod.header.numSections,
      bssSize: mod.header.bssSize,
      bssAlign: mod.header.bssAlign,
      prologSection: mod.header.prologSection,
      prolog: hex(mod.header.prolog),
      impOffset: hex(mod.header.impOffset),
      impSize: mod.header.impSize,
      relOffset: hex(mod.header.relOffset),
      fixSize: mod.header.fixSize,
    },
    entry: hex(entryInfo.entry),
    entryFromProlog: entryInfo.fromProlog,
    entryFallbackReason: entryInfo.reason,
    sections,
    emptySections,
    bss: {
      sectionIndex: mod.bss.sectionIndex,
      size: mod.bss.size,
      offset: mod.bss.offset,
      vaddr: hex(mod.base + mod.bss.offset),
      align: mod.bss.align,
    },
    segments: segmentSummary,
    relocations,
    warnings: mod.warnings,
    validation,
  };
}

function validateElf(elf, expectedSegments, mod, entryInfo) {
  const errors = [];
  const warnings = [];
  const memoryOverlaps = [];
  const fileOverlaps = [];

  if (elf.length < EHDR_SIZE) {
    return { ok: false, errors: ['ELF shorter than header'], warnings, memoryOverlaps, fileOverlaps };
  }
  const ident = elf.subarray(0, 4).toString('hex');
  if (ident !== '7f454c46') errors.push('bad ELF magic');
  if (elf[4] !== 1) errors.push('not ELFCLASS32');
  if (elf[5] !== 2) errors.push('not ELFDATA2MSB');
  if (elf[6] !== 1) errors.push('bad ELF version');
  if (elf.readUInt16BE(16) !== 2) errors.push('e_type is not ET_EXEC');
  if (elf.readUInt16BE(18) !== 20) errors.push('e_machine is not EM_PPC');
  if (elf.readUInt32BE(20) !== 1) errors.push('bad e_version');
  if (elf.readUInt16BE(40) !== EHDR_SIZE) errors.push('bad e_ehsize');
  if (elf.readUInt16BE(42) !== PHDR_SIZE) errors.push('bad e_phentsize');
  if (elf.readUInt32BE(32) !== 0) errors.push('unexpected section headers');
  if (elf.readUInt16BE(48) !== 0) errors.push('unexpected e_shnum');

  const ePhnum = elf.readUInt16BE(44);
  const ePhoff = elf.readUInt32BE(28);
  if (ePhoff !== EHDR_SIZE) errors.push(`e_phoff is ${ePhoff}, expected ${EHDR_SIZE}`);
  if (ePhoff + ePhnum * PHDR_SIZE > elf.length) {
    errors.push('program header table exceeds file');
    return { ok: false, errors, warnings, memoryOverlaps, fileOverlaps };
  }
  if (ePhnum !== expectedSegments.length) {
    errors.push(`e_phnum ${ePhnum}, expected ${expectedSegments.length}`);
  }

  const segments = [];
  for (let i = 0; i < ePhnum; i++) {
    const p = ePhoff + i * PHDR_SIZE;
    segments.push({
      type: elf.readUInt32BE(p + 0),
      offset: elf.readUInt32BE(p + 4),
      vaddr: elf.readUInt32BE(p + 8),
      paddr: elf.readUInt32BE(p + 12),
      filesz: elf.readUInt32BE(p + 16),
      memsz: elf.readUInt32BE(p + 20),
      flags: elf.readUInt32BE(p + 24),
      align: elf.readUInt32BE(p + 28),
    });
  }

  for (const s of segments) {
    if (s.type !== PT_LOAD) errors.push(`program header ${hex(s.vaddr)} is not PT_LOAD`);
    if (s.filesz > s.memsz) errors.push(`segment ${hex(s.vaddr)} has filesz > memsz`);
    if (s.offset + s.filesz > elf.length) errors.push(`segment ${hex(s.vaddr)} exceeds file`);
    if (s.vaddr + s.memsz > 0x100000000) errors.push(`segment ${hex(s.vaddr)} wraps 32-bit space`);
    if (s.align === 0 || (s.align & (s.align - 1)) !== 0) {
      errors.push(`segment ${hex(s.vaddr)} has non-power-of-two p_align`);
    }
    if (s.filesz > 0 && s.align !== 0 &&
        ((s.vaddr - s.offset) % s.align + s.align) % s.align !== 0) {
      errors.push(`segment ${hex(s.vaddr)} violates p_vaddr/p_offset congruence`);
    }
  }

  const byMem = [...segments].sort((a, b) => a.vaddr - b.vaddr);
  for (let i = 1; i < byMem.length; i++) {
    const prev = byMem[i - 1];
    const cur = byMem[i];
    if (cur.vaddr < prev.vaddr + prev.memsz) {
      memoryOverlaps.push({
        previous: { vaddr: hex(prev.vaddr), memsz: prev.memsz },
        current: { vaddr: hex(cur.vaddr), memsz: cur.memsz },
      });
    }
  }
  if (memoryOverlaps.length) errors.push('PT_LOAD memory ranges overlap');

  const byFile = segments
    .filter((s) => s.filesz > 0)
    .sort((a, b) => a.offset - b.offset);
  const headerEnd = ePhoff + ePhnum * PHDR_SIZE;
  if (byFile.length && byFile[0].offset < headerEnd) {
    errors.push('first file-backed segment overlaps ELF/program headers');
  }
  for (let i = 1; i < byFile.length; i++) {
    const prev = byFile[i - 1];
    const cur = byFile[i];
    if (cur.offset < prev.offset + prev.filesz) {
      fileOverlaps.push({
        previous: { fileOff: hex(prev.offset), filesz: prev.filesz },
        current: { fileOff: hex(cur.offset), filesz: cur.filesz },
      });
    }
  }
  if (fileOverlaps.length) errors.push('PT_LOAD file ranges overlap');

  const entry = elf.readUInt32BE(24);
  if (entry !== (entryInfo.entry >>> 0)) {
    errors.push(`e_entry ${hex(entry)} differs from planned ${hex(entryInfo.entry)}`);
  }
  const execSegments = segments.filter((s) => (s.flags & PF_X) !== 0);
  const entrySegment = execSegments.find(
    (s) => entry >= s.vaddr && entry < s.vaddr + s.filesz,
  );
  if (!entrySegment) errors.push('e_entry is not inside a file-backed executable segment');
  if (entry % 4 !== 0) warnings.push('e_entry is not 4-byte aligned');

  // Every non-empty REL section must have exactly one matching PT_LOAD.
  const liveSections = mod.sections.filter(
    (s) => s.index !== 0 && s.size > 0 && !s.isBss,
  );
  for (const section of liveSections) {
    const vaddr = mod.base + section.canonicalOffset;
    const match = segments.filter(
      (s) => s.vaddr === (vaddr >>> 0) && s.filesz === section.size,
    );
    if (match.length !== 1) {
      errors.push(`section ${section.index} is not covered by exactly one PT_LOAD`);
    } else {
      const expectedFlags = PF_R | (section.executable ? PF_X : PF_W);
      if (match[0].flags !== expectedFlags) {
        errors.push(`section ${section.index} has unexpected segment flags`);
      }
    }
  }
  if (mod.bss.size > 0) {
    const bssVaddr = (mod.base + mod.bss.offset) >>> 0;
    const match = segments.filter(
      (s) => s.vaddr === bssVaddr && s.filesz === 0 && s.memsz === mod.bss.size,
    );
    if (match.length !== 1) errors.push('BSS section is not covered by exactly one PT_LOAD');
    else if (match[0].flags !== (PF_R | PF_W)) errors.push('BSS segment has unexpected flags');
  }

  // Compare the parsed headers against the layout we intended to emit.
  if (segments.length === expectedSegments.length) {
    for (let i = 0; i < segments.length; i++) {
      const got = segments[i];
      const want = expectedSegments[i];
      if (
        got.vaddr !== (want.vaddr >>> 0) ||
        got.memsz !== want.memsz ||
        got.filesz !== want.filesz ||
        got.flags !== want.flags ||
        got.offset !== want.outOff
      ) {
        errors.push(`program header ${i} differs from planned segment`);
      }
    }
  }

  return {
    ok: errors.length === 0,
    errors,
    warnings,
    memoryOverlaps,
    fileOverlaps,
    phnum: ePhnum,
    entry: hex(entry),
    entryInExecutable: Boolean(entrySegment),
    entryFileBacked: Boolean(entrySegment),
    entryFromProlog: entryInfo.fromProlog,
    segmentCount: segments.length,
  };
}

function convertFile(input, output) {
  const buffer = fs.readFileSync(input);
  const base = path.basename(input).replace(/\.rel$/i, '');
  return buildElf(buffer, {
    input,
    output,
    name: base,
  });
}

function crossModuleRanges(modules) {
  const ranges = modules
    .filter((m) => !m.error)
    .map((m) => {
      let start = Infinity;
      let end = 0;
      for (const segment of m.segments) {
        const startNum = parseInt(segment.vaddr, 16);
        start = Math.min(start, startNum);
        end = Math.max(end, startNum + segment.memsz);
      }
      return { name: m.name, moduleId: m.moduleId, start, end };
    })
    .sort((a, b) => a.start - b.start);

  const overlaps = [];
  for (let i = 1; i < ranges.length; i++) {
    if (ranges[i].start < ranges[i - 1].end) {
      overlaps.push({
        previous: ranges[i - 1].name,
        current: ranges[i].name,
        previousEnd: hex(ranges[i - 1].end),
        currentStart: hex(ranges[i].start),
      });
    }
  }
  return {
    overlap: overlaps.length > 0,
    overlaps,
    ranges: ranges.map((r) => ({
      name: r.name,
      moduleId: r.moduleId,
      start: hex(r.start),
      end: hex(r.end),
    })),
  };
}

function addressLayout(paths) {
  const ids = new Set();
  const ranges = [];
  const modules = paths.map((input) => {
    const source = fs.readFileSync(input);
    const mod = parseRel(/\.lz$/i.test(input) ? decodeLz11(source) : source, input);
    const moduleId = mod.header.moduleId;
    const def = CANONICAL_MODULES[moduleId];
    if (!def) throw new Error(`${input}: module id ${moduleId} has no canonical base`);
    if (ids.has(moduleId)) throw new Error(`module ${moduleId} was supplied twice`);
    ids.add(moduleId);
    mod.base = def.base;
    const sections = describeSections(mod);
    for (const section of sections) {
      if (section.index === 0 || section.size === 0) continue;
      const start = mod.base + section.canonicalOffset;
      const end = start + section.size;
      if (end > 0x1_0000_0000) {
        throw new Error(`module ${moduleId} section ${section.index} overflows the address space`);
      }
      ranges.push({ start, end, moduleId, section: section.index });
    }
    return { name: def.name, moduleId, base: hex(mod.base), sections, warnings: mod.warnings };
  });
  ranges.sort((a, b) => a.start - b.start);
  for (let i = 1; i < ranges.length; i++) {
    const previous = ranges[i - 1];
    const current = ranges[i];
    if (previous.end > current.start) {
      throw new Error(
        `module ${previous.moduleId} section ${previous.section} overlaps ` +
        `module ${current.moduleId} section ${current.section}`,
      );
    }
  }
  return { format: 'REL address layout', modules, validation: { ok: true } };
}

function resolveAnalysisAddresses(summary, addresses) {
  if (summary?.validation?.ok !== true) {
    throw new Error('expected a validated rel2elf summary');
  }
  const sections = (summary.modules ?? [summary]).flatMap((module) =>
    module.sections
      .filter((section) => section.index !== 0 && section.size > 0)
      .map((section) => ({
        module: module.moduleId,
        section: section.index,
        start: Number(section.vaddr),
        size: section.size,
        executable: section.executable,
      })),
  );

  return addresses.map((input) => {
    if (!/^0x[0-9a-f]{1,8}$/i.test(input)) {
      throw new Error(`invalid 32-bit hexadecimal address: ${input}`);
    }
    const address = Number(input);
    const matches = sections.filter(({ start, size }) =>
      address >= start && address - start < size,
    );
    if (matches.length !== 1) {
      throw new Error(`${hex(address)} matches ${matches.length} REL sections`);
    }
    const [{ module, section, start, executable }] = matches;
    return { address: hex(address), module, section, offset: address - start, executable };
  });
}

function main() {
  const args = process.argv.slice(2);
  if (args[0] === '--help' || args[0] === '-h') {
    console.log(
      'usage:\n' +
      '  node tools/rel2elf.mjs --all\n' +
      '  node tools/rel2elf.mjs <module.rel> [out.elf]\n' +
      '  node tools/rel2elf.mjs --resolve-address <summary.json> <0xaddress>...\n' +
      '  node tools/rel2elf.mjs --address-layout <module.rel|module.rel.LZ>...',
    );
    return;
  }

  if (args[0] === '--address-layout') {
    if (args.length < 2) {
      console.error('usage: node tools/rel2elf.mjs --address-layout <module.rel|module.rel.LZ>...');
      process.exitCode = 2;
      return;
    }
    try {
      console.log(JSON.stringify(addressLayout(args.slice(1)), null, 2));
    } catch (error) {
      console.error(`rel2elf: ${error.message}`);
      process.exitCode = 1;
    }
    return;
  }

  if (args[0] === '--resolve-address') {
    const [, summaryPath, ...addresses] = args;
    if (!summaryPath || addresses.length === 0) {
      console.error('usage: node tools/rel2elf.mjs --resolve-address <summary.json> <0xaddress>...');
      process.exitCode = 2;
      return;
    }
    try {
      const summary = JSON.parse(fs.readFileSync(summaryPath, 'utf8'));
      console.log(JSON.stringify(resolveAnalysisAddresses(summary, addresses), null, 2));
    } catch (error) {
      console.error(`rel2elf: ${error.message}`);
      process.exitCode = 1;
    }
    return;
  }

  const all = args.length === 0 || args[0] === '--all';
  if (all) {
    if (args.length > 1) {
      console.error('rel2elf: --all takes no further arguments');
      process.exit(2);
    }
    fs.mkdirSync(ELF_DIR, { recursive: true });
    const modules = [];
    let failedModules = 0;
    for (const moduleId of MODULE_ORDER) {
      const def = CANONICAL_MODULES[moduleId];
      const input = path.join(REL_DIR, `${def.name}.rel`);
      const output = path.join(ELF_DIR, `${def.name}.elf`);
      try {
        modules.push(convertFile(input, output));
      } catch (error) {
        failedModules++;
        modules.push({
          name: def.name,
          moduleId,
          input,
          output,
          error: error.message,
        });
      }
    }

    const totals = {
      modules: modules.length,
      converted: modules.filter((m) => !m.error).length,
      failedModules,
      relocationsApplied: modules.reduce(
        (n, m) => n + (m.relocations?.applied ?? 0), 0),
      relocationsFailed: modules.reduce(
        (n, m) => n + (m.relocations?.failed ?? 0), 0),
      relocationsUnknown: modules.reduce(
        (n, m) => n + (m.relocations?.unknown ?? 0), 0),
      validationOk: modules.filter((m) => m.validation?.ok).length,
    };
    const unresolvedTypes = {};
    for (const m of modules) {
      for (const [type, count] of Object.entries(m.relocations?.unknownTypes ?? {})) {
        unresolvedTypes[type] = (unresolvedTypes[type] ?? 0) + count;
      }
    }
    const ranges = crossModuleRanges(modules);
    const summary = {
      tool: 'rel2elf',
      format: 'ELF32 big-endian EM_PPC (ET_EXEC, PT_LOAD only)',
      generatedAt: new Date().toISOString(),
      canonicalBases: Object.fromEntries(
        MODULE_ORDER.map((id) => [CANONICAL_MODULES[id].name, hex(CANONICAL_MODULES[id].base)]),
      ),
      modules,
      totals,
      unresolvedTypes,
      crossModuleRanges: ranges,
      validation: {
        ok: failedModules === 0 && totals.validationOk === modules.length &&
          !ranges.overlap && totals.relocationsFailed === 0 &&
          totals.relocationsUnknown === 0,
        modulesConverted: totals.converted,
        modulesValidated: totals.validationOk,
        noOverlappingSegments: modules
          .filter((m) => !m.error)
          .every((m) => m.validation.memoryOverlaps.length === 0 &&
            m.validation.fileOverlaps.length === 0),
        noCrossModuleOverlap: !ranges.overlap,
      },
    };
    fs.writeFileSync(
      path.join(ELF_DIR, 'summary.json'),
      `${JSON.stringify(summary, null, 2)}\n`,
    );
    console.log(JSON.stringify(summary, null, 2));
    if (!summary.validation.ok) process.exitCode = 1;
    return;
  }

  if (args.length > 2) {
    console.error('usage: node tools/rel2elf.mjs <module.rel> [out.elf]');
    process.exit(2);
  }
  const input = args[0];
  const base = path.basename(input).replace(/\.rel$/i, '');
  const output = args[1] ?? path.join(ELF_DIR, `${base}.elf`);
  try {
    const result = convertFile(input, output);
    console.log(JSON.stringify(result, null, 2));
    if (!result.validation.ok) process.exitCode = 1;
  } catch (error) {
    console.error(`rel2elf: ${error.message}`);
    process.exit(1);
  }
}

main();
