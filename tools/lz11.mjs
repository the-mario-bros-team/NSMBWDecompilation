#!/usr/bin/env node
// LZ11 (Nintendo) decompressor.
// Usage: node lz11.mjs <input.LZ> <output> [--check]
import fs from 'node:fs';
import { fileURLToPath } from 'node:url';

export function decodeLz11(src) {
  if (src.length < 4 || src[0] !== 0x11) {
    throw new Error(`not LZ11 (first byte 0x${src[0]?.toString(16) ?? '??'})`);
  }

  let size = src[1] | (src[2] << 8) | (src[3] << 16);
  let p = 4;
  const next = () => {
    if (p >= src.length) throw new Error('truncated LZ11 token');
    return src[p++];
  };
  if (size === 0) {
    if (src.length < 8) throw new Error('truncated LZ11 header');
    size = src.readUInt32LE(4);
    p = 8;
  }
  if (size === 0 || size > 0x4000000) {
    throw new Error(`implausible decompressed size ${size}`);
  }

  const out = Buffer.allocUnsafe(size);
  let o = 0;
  let flags = 0;
  let mask = 0;

  while (o < size) {
    if (mask === 0) {
      flags = next();
      mask = 0x80;
    }
    if (flags & mask) {
      const b1 = next();
      let len = b1 >> 4;
      let disp;
      if (len === 0) {
        const b2 = next();
        const b3 = next();
        len = (((b1 & 0x0f) << 4) | (b2 >> 4)) + 0x11;
        disp = (((b2 & 0x0f) << 8) | b3) + 1;
      } else if (len === 1) {
        const b2 = next();
        const b3 = next();
        const b4 = next();
        len = (((b1 & 0x0f) << 12) | (b2 << 4) | (b3 >> 4)) + 0x111;
        disp = (((b3 & 0x0f) << 8) | b4) + 1;
      } else {
        const b2 = next();
        len = (b1 >> 4) + 1;
        disp = (((b1 & 0x0f) << 8) | b2) + 1;
      }
      if (disp > o) throw new Error(`back-reference ${disp} > written ${o} at src 0x${p.toString(16)}`);
      for (let i = 0; i < len && o < size; i++) {
        out[o] = out[o - disp];
        o++;
      }
    } else {
      out[o++] = next();
    }
    mask >>= 1;
  }
  return out;
}

function main() {
  const [input, output] = process.argv.slice(2);
  if (!input || !output) {
    console.error('usage: node lz11.mjs <input> <output>');
    process.exitCode = 2;
    return;
  }
  try {
    const src = fs.readFileSync(input);
    const out = decodeLz11(src);
    fs.writeFileSync(output, out);
    console.log(JSON.stringify({
      input, output,
      compressed: src.length,
      decompressed: out.length,
      ratio: +(src.length / out.length).toFixed(3),
    }));
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}

const entry = process.argv[1];
if (entry && fs.existsSync(entry) && fs.realpathSync(entry) === fileURLToPath(import.meta.url)) {
  main();
}
