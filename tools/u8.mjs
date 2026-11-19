#!/usr/bin/env node
// U8 (Wii .arc) archive parser: list, index, extract.
// Usage: node u8.mjs <archive.arc> [--json] [--extract <dir>]
import fs from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const MAGIC = 0x55aa382d;

export function parseU8(buf, label = '') {
  if (buf.length < 0x20 || buf.readUInt32BE(0) !== MAGIC) {
    throw new Error(`${label}: not a U8 archive`);
  }
  const rootOff = buf.readUInt32BE(4);
  const headerSize = buf.readUInt32BE(8);
  const dataOff = buf.readUInt32BE(12);
  const rootLast = buf.readUInt32BE(rootOff + 8);
  const nodeCount = rootLast; // root.size = 1-based index of last node = node count
  if (nodeCount === 0 || nodeCount > 200000) throw new Error(`${label}: bad node count ${nodeCount}`);
  if (rootOff + nodeCount * 12 > buf.length) throw new Error(`${label}: node table out of range`);

  const nodes = [];
  for (let i = 0; i < nodeCount; i++) {
    const o = rootOff + i * 12;
    const word = buf.readUInt32BE(o);
    nodes.push({
      type: word >>> 24,
      nameOff: word & 0xffffff,
      dataOff: buf.readUInt32BE(o + 4),
      size: buf.readUInt32BE(o + 8),
    });
  }
  const stringBase = rootOff + nodeCount * 12;
  const readName = (n) => {
    let e = stringBase + n.nameOff;
    if (e >= buf.length) return `<bad-name@${n.nameOff}>`;
    let z = e;
    while (z < buf.length && buf[z] !== 0) z++;
    return buf.toString('utf8', e, z);
  };

  const files = [];
  const dirs = [];
  const walk = (index, prefix) => {
    const n = nodes[index];
    const name = readName(n);
    if (n.type === 1) {
      const full = prefix + (name ? name + '/' : '');
      dirs.push(full || '/');
      let child = index + 1;
      while (child < n.size) child = walk(child, full);
      return n.size;
    }
    files.push({ path: prefix + name, offset: n.dataOff, size: n.size });
    return index + 1;
  };
  walk(0, '');
  return { rootOff, headerSize, dataOff, nodeCount, rootLast, nodes, dirs, files };
}

export function magicOf(buf, off) {
  if (off + 4 > buf.length) return '';
  const b = buf.subarray(off, off + 4);
  const hex = b.toString('hex');
  const ascii = b.toString('latin1');
  const known = {
    '55aa382d': 'U8',
    bres: 'BRRES',
    RSAR: 'BRSAR',
    RSTM: 'BRSTM',
    BCSV: 'BCSV',
    RLYT: 'BRLYT',
    RFNT: 'BRFNT',
    REFF: 'BREFF',
    REFT: 'BREFT',
    '0020af30': 'TPL',
  };
  if (known[ascii]) return known[ascii];
  if (known[hex]) return known[hex];
  if (b[0] === 0x10) return 'LZ10?';
  if (b[0] === 0x11) return 'LZ11?';
  if (ascii === 'MDL0' || ascii === 'TEX0' || ascii === 'CHR0') return 'BRRES-sub';
  return hex;
}

if (import.meta.url === pathToFileURL(process.argv[1] ?? '').href) {
  const [file, outDir] = process.argv.slice(2);
  if (!file) {
    console.error('usage: node u8.mjs <archive.arc> [--json] [--extract <dir>]');
    process.exit(2);
  }
  const buf = fs.readFileSync(file);
  const u8 = parseU8(buf, file);
  const json = process.argv.includes('--json');
  const extractIdx = process.argv.indexOf('--extract');

  if (extractIdx !== -1) {
    const dest = process.argv[extractIdx + 1];
    if (!dest) throw new Error('--extract needs a directory');
    for (const f of u8.files) {
      const target = path.join(dest, ...f.path.split('/'));
      fs.mkdirSync(path.dirname(target), { recursive: true });
      fs.writeFileSync(target, buf.subarray(f.offset, f.offset + f.size));
    }
    console.log(`extracted ${u8.files.length} files to ${dest}`);
  } else if (json) {
    console.log(JSON.stringify({
      file,
      bytes: buf.length,
      nodes: u8.nodeCount,
      files: u8.files.map((f) => ({
        path: f.path,
        offset: f.offset,
        size: f.size,
        magic: magicOf(buf, f.offset),
      })),
    }, null, 2));
  } else {
    console.log(`${file}: ${u8.files.length} files, ${u8.nodeCount} nodes`);
    for (const f of u8.files) console.log(`  ${String(f.size).padStart(9)} ${magicOf(buf, f.offset).padEnd(8)} ${f.path}`);
  }
}
