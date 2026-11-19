#!/usr/bin/env node
// LZ11 (Nintendo) decompressor.
// Usage: node lz11.mjs <input.LZ> <output> [--check]
import fs from 'node:fs';

const [input, output] = process.argv.slice(2);
if (!input || !output) {
  console.error('usage: node lz11.mjs <input> <output>');
  process.exit(2);
}

const src = fs.readFileSync(input);
if (src.length < 4 || src[0] !== 0x11) {
  console.error(`not LZ11 (first byte 0x${src[0]?.toString(16) ?? '??'})`);
  process.exit(1);
}

let size = src[1] | (src[2] << 8) | (src[3] << 16);
let p = 4;
if (size === 0) {
  size = src.readUInt32LE(4);
  p = 8;
}
if (size === 0 || size > 0x4000000) {
  console.error(`implausible decompressed size ${size}`);
  process.exit(1);
}

const out = Buffer.allocUnsafe(size);
let o = 0;
let flags = 0;
let mask = 0;

while (o < size) {
  if (mask === 0) {
    flags = src[p++];
    mask = 0x80;
    if (flags === undefined) throw new Error('truncated flag byte');
  }
  if (flags & mask) {
    const b1 = src[p++];
    let len = b1 >> 4;
    let disp;
    if (len === 0) {
      const b2 = src[p++];
      const b3 = src[p++];
      len = (((b1 & 0x0f) << 4) | (b2 >> 4)) + 0x11;
      disp = (((b2 & 0x0f) << 8) | b3) + 1;
    } else if (len === 1) {
      const b2 = src[p++];
      const b3 = src[p++];
      const b4 = src[p++];
      len = (((b1 & 0x0f) << 12) | (b2 << 4) | (b3 >> 4)) + 0x111;
      disp = (((b3 & 0x0f) << 8) | b4) + 1;
    } else {
      const b2 = src[p++];
      len = (b1 >> 4) + 1;
      disp = (((b1 & 0x0f) << 8) | b2) + 1;
    }
    if (disp > o) throw new Error(`back-reference ${disp} > written ${o} at src 0x${p.toString(16)}`);
    for (let i = 0; i < len && o < size; i++) {
      out[o] = out[o - disp];
      o++;
    }
  } else {
    out[o++] = src[p++];
  }
  mask >>= 1;
}

fs.writeFileSync(output, out);
console.log(JSON.stringify({
  input, output,
  compressed: src.length,
  decompressed: size,
  ratio: +(src.length / size).toFixed(3),
}));
