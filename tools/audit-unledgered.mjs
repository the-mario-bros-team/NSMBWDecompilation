#!/usr/bin/env node
// Audit a lane's Rust file for translated bodies that never reached the ledger.
//
// The recurring failure mode in this project is a subagent that writes real
// `pub unsafe fn` bodies and then dies before writing its CSV. This tool finds
// those bodies, verifies each against the Ghidra inventory and its evidence
// file, and can emit a ready-to-merge salvage CSV.
//
// Usage:
//   node tools/audit-unledgered.mjs --file src/game/bases.rs
//   node tools/audit-unledgered.mjs --file src/game/bases.rs --csv ledger/agents/wave35-bases-salvage.csv
//   node tools/audit-unledgered.mjs --file src/runtime/input.rs --all
//
// Exit code is 1 only when a body fails verification (unknown address, byte
// size mismatch, missing evidence). Merely being unledgered is reported but is
// not an error: that is the condition the tool exists to find.

import fs from "node:fs";
import path from "node:path";

const args = process.argv.slice(2);
function take(flag, fallback = null) {
  const i = args.indexOf(flag);
  if (i === -1) return fallback;
  const v = args[i + 1];
  if (!v || v.startsWith("--")) {
    console.error(`${flag} needs a value`);
    process.exit(2);
  }
  return v;
}

const file = take("--file");
const csvOut = take("--csv");
const moduleArg = take("--module");
const exportArg = take("--export");
const auditAll = args.includes("--all");

if (!file) {
  console.error("usage: node tools/audit-unledgered.mjs --file <rust-file> [--csv <out>] [--all]");
  process.exit(2);
}
if (!fs.existsSync(file)) {
  console.error(`no such file: ${file}`);
  process.exit(2);
}

const posix = file.split(path.sep).join("/");
if (!posix.startsWith("src/") || !posix.endsWith(".rs")) {
  console.error(`expected a path under src/: ${file}`);
  process.exit(2);
}
const moduleName = moduleArg || posix.slice(4, -3).replace(/\//g, "::");

const EXPORTS = [
  [/^game\/bases\.rs$/, "rel-d_basesNP"],
  [/^game\/enemies\.rs$/, "rel-d_enemiesNP"],
  [/^game\/bosses\.rs$/, "rel-d_en_bossNP"],
];
const relPath = posix.slice(4);
const primaryExport =
  exportArg || (EXPORTS.find(([re]) => re.test(relPath)) || [null, "main-gb"])[1];

// Some semantic files mix main.dol and REL addresses (player.rs is the main
// example), so build a union of every inventory. Synthetic REL bases are
// disjoint, which makes an address lookup unambiguous.
const ALL_EXPORTS = [
  primaryExport,
  "main-gb",
  "rel-d_basesNP",
  "rel-d_enemiesNP",
  "rel-d_en_bossNP",
  "rel-d_profileNP",
].filter((v, i, a) => a.indexOf(v) === i);
const inventories = new Map();
for (const name of ALL_EXPORTS) {
  const p = `work/ghidra/${name}/export/functions.tsv`;
  if (!fs.existsSync(p)) continue;
  const map = new Map();
  for (const line of fs.readFileSync(p, "utf8").split(/\r?\n/).slice(1)) {
    if (!line.trim()) continue;
    const c = line.split("\t");
    const a = (c[0] || "").trim().toLowerCase().replace(/^0x/, "");
    if (!a) continue;
    map.set(a, {
      name: (c[1] || "").trim(),
      size: Number.parseInt(c[2], 10),
      file: (c[4] || "").trim(),
    });
  }
  inventories.set(name, map);
}
function findInventory(addr) {
  for (const name of ALL_EXPORTS) {
    const row = inventories.get(name)?.get(addr);
    if (row) return { export: name, row };
  }
  return null;
}

const ledgerPath = "ledger/functions.csv";
if (!fs.existsSync(ledgerPath)) {
  console.error(`missing ${ledgerPath} (run from the repository root)`);
  process.exit(2);
}
const ledger = new Map(); // address -> row
for (const line of fs.readFileSync(ledgerPath, "utf8").split(/\r?\n/).slice(1)) {
  if (!line) continue;
  // The generated ledger is well-formed CSV; address is field 2 and has no
  // quotes. A full parser is unnecessary for this one column.
  const cols = line.match(/^([^,]*),([^,]*),/);
  if (!cols) continue;
  const a = cols[2].trim().toLowerCase().replace(/^0x/, "");
  if (a) ledger.set(a, cols[1]);
}

const lines = fs.readFileSync(file, "utf8").split(/\r?\n/);
const bodies = [];

// A translated function is a top-level `pub unsafe fn` with a `# Original`
// doc block. Both `fn_xxxxxxxx` names and semantic names (`entry`) occur.
for (let i = 0; i < lines.length; i++) {
  const m = lines[i].match(/^\s*pub unsafe fn\s+([A-Za-z0-9_]+)\s*[(<]/);
  if (!m) continue;
  const fnName = m[1];

  let start = i;
  while (start > 0 && /^\s*\/\/\//.test(lines[start - 1])) start--;
  const doc = lines.slice(start, i);

  // Two doc formats exist in this repository:
  //   - Ghidra: `FUN_x` at `0xADDR` (N bytes)
  //   - Evidence: `work/ghidra/.../x.c`
  // and the older compact form:
  //   `name`, `0xADDR`, N bytes,
  //   `work/ghidra/.../x.c`.
  // Parse the whole `# Original` section instead of one line so both work.
  const headings = doc
    .map((l, idx) => ({ idx, m: l.match(/^\/\/\/\s*#\s+(.+?)\s*$/) }))
    .filter((x) => x.m);
  const originalHeading = headings.find((x) => x.m[1].trim() === "Original");
  if (!originalHeading) continue; // helper, not a translation
  const nextHeading = headings.find((x) => x.idx > originalHeading.idx);
  const section = doc
    .slice(originalHeading.idx + 1, nextHeading ? nextHeading.idx : doc.length)
    .join("\n");

  const orig = section.match(/\bat\s+`?0x([0-9A-Fa-f]{8})`?/);
  const addrWord = section.match(/\baddress\s+`?0x([0-9A-Fa-f]{8})`?/);
  const evMatch = section.match(/(work\/ghidra\/[^`\s)]+\.c)/);
  const docEvidence = evMatch ? evMatch[1] : null;
  // A real translation cites one evidence file. Helper functions that record
  // instruction encodings (os::sync/isync/syscall) or combine several exports
  // (os::clear_performance_monitor, link::recover_base) have an `# Original`
  // section but no single evidence file; they are not ledger rows.
  if (!docEvidence) continue;
  let addr = null;
  if (orig) addr = orig[1];
  else if (addrWord) addr = addrWord[1];
  else if (docEvidence) {
    const fromPath = docEvidence.match(/\/([0-9a-f]{8})_/i);
    if (fromPath) addr = fromPath[1];
  }
  if (!addr) {
    const any = section.match(/0x([0-9A-Fa-f]{8})/);
    if (!any) continue;
    addr = any[1];
  }
  addr = addr.toLowerCase();

  const sizeMatch = section.match(/([\d,]+)\s+bytes/i);
  const docSize = sizeMatch ? Number.parseInt(sizeMatch[1].replace(/,/g, ""), 10) : null;

  const confHeading = headings.find((x) => x.m[1].trim() === "Confidence");
  let confidence = "";
  if (confHeading) {
    for (let j = confHeading.idx + 1; j < doc.length; j++) {
      const v = doc[j].match(/^\s*\/\/\/\s*(high|medium|low)\b/);
      if (v) {
        confidence = v[1];
        break;
      }
    }
  }

  // Body extent: brace depth returns to zero after at least one line.
  let depth = 0;
  let end = i;
  for (; end < lines.length; end++) {
    for (const ch of lines[end]) {
      if (ch === "{") depth++;
      else if (ch === "}") depth--;
    }
    if (depth === 0) break;
  }

  bodies.push({
    addr,
    fnName,
    line: i + 1,
    endLine: end + 1,
    docSize,
    docEvidence,
    confidence,
    ledgered: ledger.has(addr),
  });
}

const problems = [];
const salvage = [];

for (const b of bodies) {
  if (!auditAll && b.ledgered) continue;

  const hit = findInventory(b.addr);
  const t = hit ? hit.row : null;
  const expectedEvidence =
    hit && t.file ? path.join("work/ghidra", hit.export, "export", t.file).split(path.sep).join("/") : null;

  const issues = [];
  if (!t) {
    issues.push("address not in functions.tsv");
  } else {
    if (b.docSize === null) issues.push("doc block has no byte count");
    else if (b.docSize !== t.size) issues.push(`byte size doc=${b.docSize} tsv=${t.size}`);
  }
  if (!b.docEvidence) {
    issues.push("doc block has no evidence path");
  } else {
    if (!fs.existsSync(b.docEvidence)) issues.push(`evidence missing on disk: ${b.docEvidence}`);
    if (expectedEvidence && b.docEvidence !== expectedEvidence) {
      issues.push(`evidence path differs from inventory: ${b.docEvidence} != ${expectedEvidence}`);
    }
  }
  if (!b.confidence) issues.push("doc block has no confidence line");

  if (issues.length) {
    problems.push({ ...b, issues });
    continue;
  }

  if (!b.ledgered) {
    salvage.push(b);
  }
}

const total = bodies.length;
const ledgered = bodies.filter((b) => b.ledgered).length;
const unledgered = total - ledgered;

console.log(`file:        ${file}`);
console.log(`module:      ${moduleName}`);
console.log(`export:      ${primaryExport} (+ union fallback)`);
console.log(`bodies:      ${total}`);
console.log(`ledgered:    ${ledgered}`);
console.log(`unledgered:  ${unledgered}`);
console.log(`verified:    ${salvage.length}`);
console.log(`problems:    ${problems.length}`);

if (problems.length) {
  console.log("");
  console.log("addr      line      issue");
  for (const p of problems) {
    for (const issue of p.issues) {
      console.log(`${p.addr}  ${String(p.line).padEnd(8)}  ${issue}`);
    }
  }
}

if (salvage.length) {
  console.log("");
  console.log(`salvageable (${salvage.length}): ${salvage.map((b) => b.addr).join(" ")}`);
}

if (csvOut && salvage.length) {
  const quote = (v) => (/[",\n]/.test(v) ? `"${v.replace(/"/g, '""')}"` : v);
  const rows = ["module,address,name,status,confidence,evidence,rust_file"];
  for (const b of salvage) {
    rows.push(
      [
        moduleName,
        b.addr,
        b.fnName,
        "translated",
        b.confidence,
        `${b.docEvidence}; recovered by audit-unledgered; verified against functions.tsv and the export`,
        file.split(path.sep).join("/"),
      ]
        .map(quote)
        .join(",")
    );
  }
  fs.mkdirSync(path.dirname(csvOut), { recursive: true });
  fs.writeFileSync(csvOut, rows.join("\n") + "\n");
  console.log(`\nwrote ${csvOut} (${salvage.length} rows)`);
}

process.exit(problems.length ? 1 : 0);
