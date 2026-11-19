#!/usr/bin/env node
// Extract every U8 .arc under a tree into work/arcs/extracted.
// Usage: node tools/extract-arcs.mjs <files-root> <out-root>
import fs from 'node:fs';
import path from 'node:path';
import { parseU8, magicOf } from './u8.mjs';

const [root, outRoot] = process.argv.slice(2);
if (!root || !outRoot) {
  console.error('usage: node extract-arcs.mjs <files-root> <out-root>');
  process.exit(2);
}

let arcs = 0, files = 0, bytes = 0;
const errors = [];
const types = {};

function walk(dir) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.name.toLowerCase().endsWith('.arc')) extract(p);
  }
}

function extract(p) {
  const rel = path.relative(root, p).replace(/\\/g, '/');
  try {
    const buf = fs.readFileSync(p);
    const u8 = parseU8(buf, rel);
    const dest = path.join(outRoot, rel.replace(/\.arc$/i, '').replace(/[\\/]/g, path.sep));
    for (const f of u8.files) {
      const target = path.join(dest, ...f.path.split('/'));
      fs.mkdirSync(path.dirname(target), { recursive: true });
      fs.writeFileSync(target, buf.subarray(f.offset, f.offset + f.size));
      files++; bytes += f.size;
      const t = magicOf(buf, f.offset);
      types[t] = (types[t] ?? 0) + 1;
    }
    arcs++;
  } catch (err) {
    errors.push({ path: rel, error: String(err.message ?? err) });
  }
}

walk(root);
fs.mkdirSync(outRoot, { recursive: true });
fs.writeFileSync(path.join(outRoot, 'summary.json'), JSON.stringify({
  arcs, files, bytes, types, errors,
}, null, 2));
console.log(JSON.stringify({ arcs, files, bytes, types, errors: errors.length, outRoot }, null, 2));
