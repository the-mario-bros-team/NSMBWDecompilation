#!/usr/bin/env node
// Extract printable ASCII strings with file offsets.
// Usage: node strings.mjs <file> <out.txt> [minLen]
import fs from 'node:fs';

const [file, out, minArg] = process.argv.slice(2);
if (!file || !out) {
  console.error('usage: node strings.mjs <file> <out.txt> [minLen]');
  process.exit(2);
}
const min = Number(minArg ?? 5);
const buf = fs.readFileSync(file);
const lines = [];
let start = -1;
for (let i = 0; i <= buf.length; i++) {
  const c = i < buf.length ? buf[i] : 0;
  const printable = c >= 0x20 && c < 0x7f;
  if (printable) {
    if (start < 0) start = i;
  } else {
    if (start >= 0) {
      const len = i - start;
      if (len >= min) lines.push(`${start.toString(16).padStart(8, '0')} ${buf.toString('latin1', start, i)}`);
      start = -1;
    }
  }
}
fs.writeFileSync(out, lines.join('\n'));
console.log(JSON.stringify({ file, out, strings: lines.length, min }, null, 2));
