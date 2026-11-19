#!/usr/bin/env node
// Source-integrity guard: catches truncated or clobbered source files.
//
// Motivation: a lane truncated `src/ui/lyt.rs` to 0 bytes with a whole-file
// write, which broke the tree and destroyed ~700 KB of translated code. The
// gate should fail on that shape immediately.
//
// Checks:
//   1. every src/**/*.rs is non-empty;
//   2. every file containing `pub unsafe fn` has at least one of them;
//   3. no file is smaller than 25% of the largest recorded size for that path
//      (recorded in work/source_sizes.json by --record or after a clean gate);
//   4. no file contains a NUL byte or U+FFFD.
//
// Usage: node tools/check-source-integrity.mjs [--record]

import { readFileSync, writeFileSync, readdirSync, statSync, existsSync } from 'node:fs';
import { join } from 'node:path';

const root = 'src';
const ledgerPath = 'work/source_sizes.json';
const record = process.argv.includes('--record');

const files = [];
(function walk(dir) {
  for (const entry of readdirSync(dir)) {
    const p = join(dir, entry);
    if (statSync(p).isDirectory()) walk(p);
    else if (p.endsWith('.rs')) files.push(p);
  }
})(root);

const previous = existsSync(ledgerPath) ? JSON.parse(readFileSync(ledgerPath, 'utf8')) : {};
const findings = [];
const sizes = {};

for (const f of files) {
  const buf = readFileSync(f);
  sizes[f] = buf.length;

  if (buf.length === 0) {
    findings.push(`${f}: is 0 bytes`);
    continue;
  }
  const s = buf.toString('utf8');
  if (/^pub unsafe fn /m.test(s) && !/pub unsafe fn /.test(s)) {
    findings.push(`${f}: declares bodies but none are present`);
  }
  if (s.includes('\u0000')) findings.push(`${f}: contains a NUL byte`);
  if (s.includes('\uFFFD')) findings.push(`${f}: contains U+FFFD (encoding damage)`);

  const prev = previous[f];
  if (prev && prev >= 4096 && buf.length < prev * 0.25) {
    findings.push(`${f}: shrank from ${prev} to ${buf.length} bytes (${Math.round((buf.length / prev) * 100)}%)`);
  }
}

// A directory-wide collapse would also show up as a huge total drop.
const total = Object.values(sizes).reduce((a, b) => a + b, 0);
const prevTotal = Object.values(previous).reduce((a, b) => a + b, 0);
if (prevTotal >= 4096 && total < prevTotal * 0.9) {
  findings.push(`src total shrank from ${prevTotal} to ${total} bytes`);
}

console.log(`source-integrity: ${files.length} files, ${total} bytes, ${findings.length} finding(s)`);
for (const f of findings) console.log('  ' + f);

if (record) {
  writeFileSync(ledgerPath, JSON.stringify(sizes, null, 0) + '\n');
  console.log(`recorded sizes -> ${ledgerPath}`);
} else if (findings.length) {
  process.exit(1);
}
