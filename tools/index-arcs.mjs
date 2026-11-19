#!/usr/bin/env node
// Index every U8 .arc under a tree into one JSON inventory.
// Usage: node index-arcs.mjs <files-root> <out.json>
import fs from 'node:fs';
import path from 'node:path';
import { parseU8, magicOf } from './u8.mjs';

const [root, out] = process.argv.slice(2);
if (!root || !out) {
  console.error('usage: node index-arcs.mjs <files-root> <out.json>');
  process.exit(2);
}

const arcs = [];
const errors = [];
let totalMembers = 0;
let totalMemberBytes = 0;

function walk(dir) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.name.toLowerCase().endsWith('.arc')) addArc(p);
  }
}

function addArc(p) {
  const rel = path.relative(root, p).replace(/\\/g, '/');
  try {
    const buf = fs.readFileSync(p);
    const u8 = parseU8(buf, rel);
    const members = u8.files.map((f) => ({
      path: f.path,
      size: f.size,
      magic: magicOf(buf, f.offset),
    }));
    totalMembers += members.length;
    totalMemberBytes += members.reduce((a, m) => a + m.size, 0);
    arcs.push({ path: rel, bytes: buf.length, nodes: u8.nodeCount, members });
  } catch (err) {
    errors.push({ path: rel, error: String(err.message ?? err) });
  }
}

walk(root);

const byMagic = {};
for (const a of arcs) for (const m of a.members) byMagic[m.magic] = (byMagic[m.magic] ?? 0) + 1;

const result = {
  generated: new Date().toISOString(),
  root,
  arc_count: arcs.length,
  member_count: totalMembers,
  member_bytes: totalMemberBytes,
  member_types: byMagic,
  errors,
  arcs,
};
fs.mkdirSync(path.dirname(out), { recursive: true });
fs.writeFileSync(out, JSON.stringify(result, null, 1));
console.log(JSON.stringify({
  arcs: arcs.length,
  members: totalMembers,
  memberMB: +(totalMemberBytes / 1048576).toFixed(1),
  types: byMagic,
  errors: errors.length,
  out,
}, null, 2));
