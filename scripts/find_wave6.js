const fs = require('fs');
const base = 'C:\\Users\\WitherULTRA7\\wii-decomp\\nsmbw';
const p = base + '\\work\\ghidra\\main-gb\\export\\functions.tsv';
const ledger = base + '\\ledger\\functions.csv';
const dvd = base + '\\src\\runtime\\dvd.rs';
const translated = new Set();
for (const l of fs.readFileSync(ledger, 'utf8').split(/\r?\n/)) {
  if (l.includes('runtime::dvd')) {
    const parts = l.split(',');
    if (parts[1]) translated.add(parts[1].toLowerCase());
  }
}
const ds = fs.readFileSync(dvd, 'utf8');
const m = ds.match(/0x801[dD][0-9a-fA-F]+/g);
if (m) {
  for (const x of m) translated.add(x.slice(2).toLowerCase());
}
const out = [];
for (const l of fs.readFileSync(p, 'utf8').split(/\r?\n/)) {
  if (!l.trim()) continue;
  const parts = l.split('\t');
  const a = parseInt(parts[0], 16);
  if (a >= 0x801D2D90 && a < 0x801D4000) {
    const sz = parseInt(parts[2]);
    if (sz >= 16 && !translated.has(parts[0].toLowerCase())) {
      out.push({addr: parts[0], name: parts[1], sz});
    }
  }
}
console.log(out.length);
for (const o of out) console.log(o.addr, o.name, o.sz);
