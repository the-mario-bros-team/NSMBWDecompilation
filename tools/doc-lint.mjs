#!/usr/bin/env node
// Readability linter for the reconstructed Rust tree.
// Checks: module doc, per-function doc, evidence block, unexplained raw code.
// Usage: node tools/doc-lint.mjs [srcDir]
import fs from 'node:fs';
import path from 'node:path';

const root = process.argv[2] ?? 'src';
const violations = [];
let fileCount = 0;

function walk(dir) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.name.endsWith('.rs')) check(p);
  }
}

function check(file) {
  fileCount++;
  const text = fs.readFileSync(file, 'utf8');
  const rel = path.relative(root, file);
  if (!text.trimStart().startsWith('//!')) {
    violations.push({ file: rel, line: 1, rule: 'module-doc', detail: 'file must start with a //! module header' });
  }
  const lines = text.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(/^\s*pub\s+(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)/);
    if (!m) continue;
    let j = i - 1;
    let doc = '';
    while (j >= 0 && (lines[j].trim().startsWith('///') || lines[j].trim().startsWith('#[') || lines[j].trim() === '')) {
      if (lines[j].trim().startsWith('///')) doc = lines[j] + '\n' + doc;
      if (lines[j].trim() === '' && doc.length > 0) break;
      j--;
    }
    if (!doc) {
      violations.push({ file: rel, line: i + 1, rule: 'fn-doc', detail: `${m[1]}: missing /// doc comment` });
      continue;
    }
    if (!doc.includes('# Original')) {
      violations.push({ file: rel, line: i + 1, rule: 'fn-evidence', detail: `${m[1]}: doc lacks "# Original" evidence block` });
    }
    if (!/# (Confidence|Safety)/.test(doc)) {
      violations.push({ file: rel, line: i + 1, rule: 'fn-confidence', detail: `${m[1]}: doc lacks "# Confidence" or "# Safety"` });
    }
  }
  const bare = text.match(/\/\/\s*TODO(?!:)/g);
  if (bare) violations.push({ file: rel, line: 1, rule: 'todo', detail: 'use "// unknown:" or "// partial:" instead of bare TODO' });
}

walk(root);
const summary = { files: fileCount, violations: violations.length, byRule: {} };
for (const v of violations) summary.byRule[v.rule] = (summary.byRule[v.rule] ?? 0) + 1;

if (violations.length === 0) {
  console.log(JSON.stringify({ ok: true, ...summary }, null, 2));
} else {
  console.log(JSON.stringify({ ok: false, ...summary, violations: violations.slice(0, 200) }, null, 2));
  process.exitCode = 1;
}
