#!/usr/bin/env node
// Plan the next wave: the 5 disjoint lanes with the most remaining functions.
//
// Reads ledger/functions.csv plus every Ghidra functions.tsv, computes remaining
// counts per 64 KiB bucket, attributes each bucket to its dominant Rust file
// (from the ledger), and prints lanes whose file is not excluded.
//
// Usage:
//   node tools/plan-next-wave.mjs
//   node tools/plan-next-wave.mjs --exclude src/game/bases.rs,src/game/enemies.rs
//   node tools/plan-next-wave.mjs --json
//
// The tool only plans. Launching subagents is done by the coordinator after
// running this command; see docs/RESET_POLICY.md.

import fs from "node:fs";
import path from "node:path";

const args = process.argv.slice(2);
function take(flag) {
  const i = args.indexOf(flag);
  return i === -1 ? null : args[i + 1];
}
const excludeArg = take("--exclude") || "";
const json = args.includes("--json");

const EXPORTS = [
  "main-gb",
  "rel-d_basesNP",
  "rel-d_enemiesNP",
  "rel-d_en_bossNP",
  "rel-d_profileNP",
];

const ledgerPath = "ledger/functions.csv";
if (!fs.existsSync(ledgerPath)) {
  console.error(`missing ${ledgerPath} (run from the repository root)`);
  process.exit(2);
}

// Ledger rows: address is field 2, rust_file is the last field. Both are
// unquoted in the generated ledger.
const ledger = new Map();
const byFile = new Map();
for (const line of fs.readFileSync(ledgerPath, "utf8").split(/\r?\n/).slice(1)) {
  if (!line) continue;
  const m = line.match(/^([^,]*),([^,]*),/);
  if (!m) continue;
  ledger.set(m[2].trim().toLowerCase().replace(/^0x/, ""), m[1]);
  const file = line.split(",").pop().trim().replace(/\r$/, "");
  const a = m[2].trim().toLowerCase().replace(/^0x/, "");
  if (!byFile.has(file)) byFile.set(file, []);
  byFile.get(file).push(a);
}

const exclude = new Set(
  excludeArg
    .split(",")
    .map((s) => s.trim().replace(/\\/g, "/"))
    .filter(Boolean)
);

const buckets = [];
for (const exp of EXPORTS) {
  const p = path.join("work/ghidra", exp, "export", "functions.tsv");
  if (!fs.existsSync(p)) continue;
  const per = new Map();
  for (const line of fs.readFileSync(p, "utf8").split(/\r?\n/).slice(1)) {
    if (!line.trim()) continue;
    const a = line.split("\t")[0].trim().toLowerCase().replace(/^0x/, "");
    if (!a || a.length !== 8) continue;
    const key = a.slice(0, 4);
    if (!per.has(key)) per.set(key, { total: 0, done: 0, files: new Map() });
    const b = per.get(key);
    b.total++;
    if (ledger.has(a)) b.done++;
  }
  // Attribute translated addresses to their Rust files for bucket ownership.
  const owners = new Map();
  for (const line of fs.readFileSync(ledgerPath, "utf8").split(/\r?\n/).slice(1)) {
    if (!line) continue;
    const m = line.match(/^([^,]*),([^,]*),.*,(src\/[^,]+)$/);
    if (!m) continue;
    const a = m[2].trim().toLowerCase().replace(/^0x/, "");
    owners.set(a, m[3].trim());
  }
  for (const line of fs.readFileSync(p, "utf8").split(/\r?\n/).slice(1)) {
    if (!line.trim()) continue;
    const a = line.split("\t")[0].trim().toLowerCase().replace(/^0x/, "");
    if (!a || a.length !== 8) continue;
    const key = a.slice(0, 4);
    const b = per.get(key);
    if (ledger.has(a)) {
      const f = owners.get(a);
      if (f) b.files.set(f, (b.files.get(f) || 0) + 1);
    }
  }
  for (const [key, b] of per) {
    const left = b.total - b.done;
    if (left <= 0) continue;
    const dominant = [...b.files.entries()]
      .sort((x, y) => y[1] - x[1])
      .map(([f, n]) => ({ file: f, count: n }));
    buckets.push({
      export: exp,
      prefix: key,
      bucket: `0x${key}0000`,
      remaining: left,
      total: b.total,
      translated: b.done,
      dominant,
    });
  }
}

buckets.sort((a, b) => b.remaining - a.remaining);

// Count bodies per file for context.
const bodyCounts = new Map();
for (const f of byFile.keys()) {
  if (fs.existsSync(f)) {
    const src = fs.readFileSync(f, "utf8");
    bodyCounts.set(f, (src.match(/pub unsafe fn /g) || []).length);
  }
}

const usedFiles = new Set(exclude);
const plan = [];
for (const b of buckets) {
  if (plan.length >= 5) break;
  const owner = b.dominant[0];
  if (!owner) continue; // bucket never translated: needs naming first
  if (usedFiles.has(owner.file)) continue;
  if (exclude.has(owner.file)) continue;
  usedFiles.add(owner.file);
  const fileExists = fs.existsSync(owner.file);
  plan.push({
    module: owner.file.slice(4, -3).replace(/\//g, "::"),
    file: owner.file,
    fileExists,
    bodies: bodyCounts.get(owner.file) ?? 0,
    export: b.export,
    bucket: b.bucket,
    remaining: b.remaining,
    total: b.total,
    translated: b.translated,
    runnersUp: b.dominant.slice(1, 3),
  });
}

if (json) {
  console.log(JSON.stringify(plan, null, 2));
  process.exit(0);
}

console.log("Next-wave plan (5 disjoint lanes, biggest remaining buckets):");
console.log("");
console.log("| # | module | file | bodies | export | bucket | remaining | total |");
console.log("|---:|---|---|---:|---|---|---:|---:|");
plan.forEach((p, i) => {
  console.log(
    `| ${i + 1} | \`${p.module}\` | \`${p.file}\` | ${p.bodies} | \`${p.export}\` | \`${p.bucket}\` | ${p.remaining} | ${p.total} |`
  );
});
console.log("");
console.log("Exclusions in effect:", [...exclude].join(", ") || "(none)");
console.log("");
console.log("Notes:");
console.log("- Buckets with no translated owner need a module named first (run");
console.log("  tools/audit-unledgered.mjs --all once a file exists for them).");
console.log("- Launch subagents with docs/LANE_BRIEF_WAVE<N>.md; see");
console.log("  docs/RESET_POLICY.md for when to launch automatically.");
