#!/usr/bin/env node
// Gate: no modelled CodeWarrior register-save helper calls may survive in src/.
//
// The band 0x802DCD40..0x802DCDD4 is the image's `_savegpr_14.._savegpr_31` /
// `_restgpr_14.._restgpr_31` stub block (8 words each, `blr`-terminated). Those
// are compiler prologue/epilogue artefacts, never real callees, so a Rust body
// must not model them as calls. This scanner finds them by *value*, not name:
//
//   * any numeric literal in the band, upper/lower case, with or without `_`
//     separators (`0x802DCD74`, `0x802d_cd74`, `0x802D_CD74` all match);
//   * any use of a `const`/`static` whose declared value is in the band, even
//     when the declaration is in another file;
//   * comments and string literals are masked first, so documentation that
//     mentions an address is not a hit;
//   * each hit is attributed to its enclosing function by brace span, never by
//     its line text, so identical lines in different bodies count separately.
//
// Measurement bugs this prevents (both hid the os.rs residue):
//
//   1. The wave-289 census used a case-sensitive token regex
//      `/0x802D_?CD[0-9A-Fa-f]{2}/`. `src/runtime/os.rs` spells its addresses
//      lower-case (`0x802d_cd70`), so 203 real references in 89 bodies were
//      reported as zero.
//   2. That census and `work/wave289/d_residue_bodies.mjs` attributed a site
//      with `masked.indexOf(lineText)` - the first line in the file with the
//      same text. Bodies sharing an identical helper line were all credited to
//      the first body, turning 167 bodies into 89 (and `fn_801f4880`'s real 2
//      sites into a false 18). This scanner uses the line index and the brace
//      span instead.
//
// Usage:
//   node tools/check-helper-calls.mjs [--report-only] [--verbose] [--json]
//
// Exit status: 1 when live references remain (0 under --report-only), 0 when
// the only findings are dead declarations. A dead declaration (a band-valued
// constant with no references anywhere) is listed, but does not fail the gate.
import fs from "node:fs";
import path from "node:path";

const LO = 0x802dcd40, HI = 0x802dcdd4;
const args = new Set(process.argv.slice(2));
const REPORT_ONLY = args.has("--report-only") || process.env.HELPER_CALLS_REPORT_ONLY === "1";
const VERBOSE = args.has("--verbose");
const JSON_OUT = args.has("--json");

export function maskCommentsAndStrings(source) {
  let out = "", i = 0;
  const n = source.length;
  while (i < n) {
    const c = source[i], d = source[i + 1];
    if (c === "/" && d === "/") {
      while (i < n && source[i] !== "\n") { out += " "; i++; }
      continue;
    }
    if (c === "/" && d === "*") {
      out += "  "; i += 2;
      while (i < n && !(source[i] === "*" && source[i + 1] === "/")) { out += source[i] === "\n" ? "\n" : " "; i++; }
      if (i < n) { out += "  "; i += 2; }
      continue;
    }
    if (c === '"') {
      out += " "; i++;
      while (i < n) {
        if (source[i] === "\\") { out += "  "; i += 2; continue; }
        if (source[i] === '"') { out += " "; i++; break; }
        out += source[i] === "\n" ? "\n" : " "; i++;
      }
      continue;
    }
    out += c; i++;
  }
  return out;
}

const NUM = /^(?:0[xX]([0-9A-Fa-f_]+)|([0-9]+))$/;
function parseNum(expr) {
  const m = NUM.exec(expr.trim());
  if (!m) return null;
  if (m[1] !== undefined) return parseInt(m[1].replace(/_/g, ""), 16);
  return parseInt(m[2].replace(/_/g, ""), 10);
}
const inBand = (v) => v !== null && v >= LO && v <= HI;
const fmt = (v) => "0x" + v.toString(16).toUpperCase().padStart(8, "0");

function walk(dir, out = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name).replace(/\\/g, "/");
    if (e.isDirectory()) walk(p, out);
    else if (p.endsWith(".rs")) out.push(p);
  }
  return out;
}

const files = walk("src").sort();
const sources = new Map();
for (const f of files) sources.set(f, fs.readFileSync(f, "utf8"));

// ---- pass 1: band-valued constants (direct value or alias to one) ----
const DECL = /^\s*(?:pub(?:\([^)]*\))?\s+)?(?:const|static)\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*[^=;]+\s*=\s*([^;]+);/;
const rawDecls = new Map(); // name -> {file,line,expr}
for (const f of files) {
  const masked = maskCommentsAndStrings(sources.get(f));
  const lines = masked.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const m = DECL.exec(lines[i]);
    if (!m) continue;
    rawDecls.set(m[1], { file: f, line: i + 1, expr: m[2].trim() });
  }
}
const consts = new Map(); // name -> {file,line,value}
for (let changed = true; changed;) {
  changed = false;
  for (const [name, d] of rawDecls) {
    if (consts.has(name)) continue;
    let v = parseNum(d.expr);
    if (v === null && /^[A-Za-z_][A-Za-z0-9_]*$/.test(d.expr)) {
      const ref = consts.get(d.expr);
      if (ref) v = ref.value;
    }
    if (v === null) continue;
    consts.set(name, { ...d, value: v });
    changed = true;
  }
}
const bandConsts = new Map([...consts].filter(([, d]) => inBand(d.value)));

// ---- pass 2: brace-span function bodies ----
const bodiesByFile = new Map();
for (const f of files) {
  const masked = maskCommentsAndStrings(sources.get(f));
  const lines = masked.split(/\r?\n/);
  const stack = [];
  const bodies = [];
  let depth = 0, carriedName = null;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const fnm = /(?:^|\s)(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:const\s+)?(?:extern\s+"[^"]*"\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)/.exec(line);
    let pendingName = fnm ? fnm[1] : carriedName;
    if (fnm) carriedName = fnm[1];
    if (!fnm && /;\s*$/.test(line)) carriedName = null;
    for (const ch of line) {
      if (ch === "{") {
        depth++;
        if (pendingName) { stack.push({ name: pendingName, start: i + 1, depth }); pendingName = null; carriedName = null; }
      } else if (ch === "}") {
        if (stack.length && stack[stack.length - 1].depth === depth) {
          const b = stack.pop();
          b.end = i + 1;
          bodies.push(b);
        }
        depth--;
      }
    }
  }
  bodiesByFile.set(f, bodies);
}
function ownerOf(file, line) {
  let best = null;
  for (const b of bodiesByFile.get(file) || []) {
    if (b.start <= line && line <= b.end && (!best || b.start > best.start)) best = b;
  }
  return best ? best.name : "<top-level>";
}

// ---- pass 3: references ----
const TOKEN = /0x[0-9A-Fa-f_]+|\b\d[\d_]*\b/g;
const refs = [];
const usedConsts = new Set();
for (const f of files) {
  const masked = maskCommentsAndStrings(sources.get(f));
  const lines = masked.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const isDecl = DECL.test(line);
    if (isDecl) continue; // the declaration itself is not a use
    for (const m of line.matchAll(TOKEN)) {
      const v = parseNum(m[0]);
      if (!inBand(v)) continue;
      refs.push({ file: f, line: i + 1, kind: "literal", token: m[0], value: v, body: ownerOf(f, i + 1) });
    }
    for (const [name, d] of bandConsts) {
      const re = new RegExp(`\\b${name}\\b`, "g");
      let m;
      while ((m = re.exec(line))) {
        refs.push({ file: f, line: i + 1, kind: "const", token: name, value: d.value, body: ownerOf(f, i + 1) });
        usedConsts.add(name);
      }
    }
  }
}

// ---- report ----
const perFile = new Map();
for (const f of files) perFile.set(f, { refs: 0, bodies: new Set(), decls: 0, dead: [] });
for (const r of refs) {
  const s = perFile.get(r.file);
  s.refs++;
  if (r.body !== "<top-level>") s.bodies.add(r.body);
}
for (const [name, d] of bandConsts) {
  const s = perFile.get(d.file);
  s.decls++;
  if (!usedConsts.has(name)) s.dead.push(name);
}
const dead = [...bandConsts].filter(([name]) => !usedConsts.has(name))
  .map(([name, d]) => ({ name, file: d.file, line: d.line, value: d.value }));
const bodyKeys = new Set(refs.map((r) => `${r.file}\t${r.body}`));
const totalBodies = new Set(refs.map((r) => r.body !== "<top-level>" ? `${r.file}\t${r.body}` : null).filter(Boolean)).size;

if (JSON_OUT) {
  console.log(JSON.stringify({
    band: [fmt(LO), fmt(HI)],
    files: files.length,
    references: refs.length,
    bodies: totalBodies,
    declarations: bandConsts.size,
    deadDeclarations: dead,
    perFile: [...perFile].filter(([, s]) => s.refs || s.decls).map(([file, s]) => ({
      file, refs: s.refs, bodies: s.bodies.size, declarations: s.decls, dead: s.dead,
    })),
    reportOnly: REPORT_ONLY,
  }, null, 2));
} else {
  console.log(`helper-call gate: band ${fmt(LO)}..${fmt(HI)}`);
  console.log(`files=${files.length} refs=${refs.length} bodies=${totalBodies} declarations=${bandConsts.size} dead=${dead.length}`);
  for (const [file, s] of [...perFile].filter(([, s]) => s.refs || s.decls).sort((a, b) => b[1].refs - a[1].refs || a[0].localeCompare(b[0]))) {
    console.log(`  ${file}\trefs=${s.refs}\tbodies=${s.bodies.size}\tdecls=${s.decls}${s.dead.length ? "\tdead=" + s.dead.join(",") : ""}`);
  }
  if (dead.length) {
    console.log(`\ndead declarations (band-valued constant with zero references; not a gate failure): ${dead.length}`);
    for (const d of dead) console.log(`  DEAD ${d.file}:${d.line}\t${d.name}\t${fmt(d.value)}`);
  }
  if (refs.length) {
    console.log(`\nlive references: ${refs.length} in ${totalBodies} bodies`);
    if (VERBOSE) for (const r of refs) console.log(`  REF ${r.file}:${r.line}\t${r.kind}=${r.token}\tbody=${r.body}`);
    else console.log("  (re-run with --verbose for every site)");
  }
  console.log("");
  if (refs.length && !REPORT_ONLY) console.log(`FAIL: ${refs.length} live helper reference(s) remain; remove the calls and rebind their values to real arguments.`);
  else if (refs.length && REPORT_ONLY) console.log(`REPORT-ONLY: ${refs.length} live helper reference(s) remain (gate not enforced).`);
  else console.log("OK: no live helper references remain.");
}
process.exit(refs.length && !REPORT_ONLY ? 1 : 0);
