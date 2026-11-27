#!/usr/bin/env node
/*
 * Phase 1.1 + 1.2 generator.
 *
 * Input : outputs/phase1-ledger-evidence.jsonl  (the verified evidence inventory)
 *         E:/NSMBWDecompCodex/analysis/symbols/symbols_E1_merged_v13.tsv (cross-check)
 * Output: outputs/phase1-name-table.tsv
 *         outputs/phase1-name-table.jsonl
 *         outputs/phase1-class-list.tsv
 *         outputs/phase1-summary.json
 *
 * The evidence inventory already joined every ledger row to its name candidates.
 * This script turns that inventory into (a) one chosen name per row, with
 * provenance and confidence, and (b) a demangled class list. It applies nothing
 * to the Rust sources: a name candidate is not proof of identity.
 */

const fs = require('fs');
const path = require('path');

const OUT = 'C:/Users/Wither/Documents/Codex/2026-11-25/rad/outputs';
const EVIDENCE = path.join(OUT, 'phase1-ledger-evidence.jsonl');
const V13 = 'E:/NSMBWDecompCodex/analysis/symbols/symbols_E1_merged_v13.tsv';

const STRUCTURAL = /^_(prolog|epilog|unresolved|start|init|text|data|bss|ctors|dtors|header|entry|end)$/i;

function normAddr(a) {
  if (!a) return '';
  let s = String(a).trim().toLowerCase();
  if (!s.startsWith('0x')) s = '0x' + s.replace(/^0+/, '');
  return s;
}

function candidateRank(c) {
  // reviewedE1 is the curated table, normalizedE1 the clean E1 map, recomp the MAP.txt.
  return ({ reviewedE1: 0, normalizedE1: 1, recomp: 2 })[c.source] ?? 9;
}

function usable(c) {
  return c && c.spelling && !STRUCTURAL.test(c.spelling);
}

function classifyOwner(owner, declaredClass) {
  if (!owner) return { kind: 'free', confidence: 'none' };
  if (declaredClass) return { kind: 'class', confidence: 'declared' };
  const last = owner.split('::').pop();
  if (/_c$/.test(last)) return { kind: 'class', confidence: 'naming-convention' };
  if (/^[A-Z]/.test(last)) return { kind: 'class', confidence: 'naming-convention' };
  return { kind: 'namespace', confidence: 'inferred' };
}

function splitDisplay(display) {
  if (!display) return { class: '', method: '', params: '' };
  const s = String(display).trim();
  const open = s.indexOf('(');
  const head = open >= 0 ? s.slice(0, open) : s;
  const params = open >= 0 ? s.slice(open) : '';
  const lastSep = head.lastIndexOf('::');
  if (lastSep < 0) return { class: '', method: head, params };
  return { class: head.slice(0, lastSep), method: head.slice(lastSep + 2), params };
}

function csvCell(v) {
  const s = v == null ? '' : String(v);
  return /[",\n\r\t]/.test(s) ? '"' + s.replace(/"/g, '""') + '"' : s;
}

function main() {
  const raw = fs.readFileSync(EVIDENCE, 'utf8').split(/\r?\n/).filter(Boolean);
  const rows = raw.map((l) => JSON.parse(l));
  if (rows.length !== 20250) throw new Error(`expected 20250 evidence rows, got ${rows.length}`);

  // Cross-check table: main-program functions from the curated v13 table, keyed by address.
  const v13ByAddr = new Map();
  const v13Lines = fs.readFileSync(V13, 'utf8').split(/\r?\n/);
  const header = v13Lines[0].split('\t');
  const col = Object.fromEntries(header.map((h, i) => [h, i]));
  for (let i = 1; i < v13Lines.length; i++) {
    const f = v13Lines[i].split('\t');
    if (f.length < header.length) continue;
    if (f[col.kind] !== 'function') continue;
    v13ByAddr.set(normAddr(f[col.address]), {
      name: f[col.name],
      declaredClass: (f[col.declared_class] || '').trim(),
    });
  }

  const tsv = [];
  const jsonl = [];
  const classMap = new Map();
  const tally = {
    rows: rows.length,
    chosen: 0,
    agreement: 0,
    singleSource: 0,
    conflicts: 0,
    noName: 0,
    structuralOnly: 0,
    withDisplay: 0,
    withClass: 0,
    classes: 0,
    mainAddressMatches: 0,
    mainAddressChecked: 0,
    nameMatchesV13: 0,
    nameDiffersV13: 0,
  };
  const conflicts = [];

  for (const r of rows) {
    const cands = (r.candidates || []).slice().sort((a, b) => candidateRank(a) - candidateRank(b));
    const usableCands = cands.filter(usable);
    const displays = usableCands.filter((c) => c.display && c.display !== c.spelling);
    const displayCand = displays[0] || usableCands.find((c) => c.display) || null;
    const display = displayCand ? (displayCand.display || displayCand.spelling) : '';
    const parts = splitDisplay(display);
    if (display) tally.withDisplay++;

    let chosenName = null;
    let chosenSource = null;
    let chosenConfidence = null;
    let chosenProof = null;

    if (r.assessment === 'agreement' && usableCands.length) {
      chosenName = usableCands[0].spelling;
      chosenSource = 'agreement:' + usableCands.map((c) => c.source).join('+');
      chosenConfidence = usableCands[0].confidence || r.confidence;
      tally.agreement++;
    } else if (r.assessment === 'singleSource' && usableCands.length) {
      chosenName = usableCands[0].spelling;
      chosenSource = 'single:' + usableCands[0].source;
      chosenConfidence = usableCands[0].confidence || r.confidence;
      tally.singleSource++;
    } else if (r.assessment === 'conflicts') {
      tally.conflicts++;
      conflicts.push({
        ledgerLine: r.ledgerLine,
        address: r.address,
        module: r.location && r.location.module,
        spellings: usableCands.map((c) => c.source + '=' + c.spelling),
      });
    } else {
      tally.noName++;
      if ((r.candidates || []).some((c) => c.spelling)) tally.structuralOnly++;
    }
    if (chosenName) {
      tally.chosen++;
      const srcCand = usableCands[0];
      chosenProof = [
        srcCand.sourceLabel,
        srcCand.source + (srcCand.line ? ':' + srcCand.line : ''),
      ].filter(Boolean).join(' | ');
    }

    // Cross-check main-program rows against the curated table.
    const addr = normAddr(r.address);
    const v13 = r.location && r.location.module === 'main' ? v13ByAddr.get(addr) : null;
    if (v13) {
      tally.mainAddressChecked++;
      tally.mainAddressMatches++;
      if (chosenName && chosenName === v13.name) tally.nameMatchesV13++;
      else if (chosenName) tally.nameDiffersV13++;
    }
    const owner = classifyOwner(parts.class, v13 ? v13.declaredClass : '');

    const row = {
      ledgerLine: r.ledgerLine,
      module: r.location ? r.location.module : '',
      section: r.location ? r.location.section : null,
      offset: r.location ? r.location.offset : null,
      address: addr,
      runtimeAddress: r.location && r.location.capturedRuntime ? normAddr(r.location.capturedRuntime) : '',
      rustFile: r.rustFile,
      currentName: r.currentName,
      chosenName,
      chosenDisplay: display,
      className: parts.class,
      ownerKind: owner.kind,
      methodName: parts.method,
      parameters: parts.params,
      assessment: r.assessment,
      confidence: chosenConfidence,
      provenance: chosenSource,
      proof: chosenProof,
      candidates: cands.map((c) => ({ source: c.source, spelling: c.spelling, display: c.display, confidence: c.confidence })),
    };
    jsonl.push(JSON.stringify(row));

    tsv.push([
      row.ledgerLine, row.module, row.address, row.runtimeAddress, row.rustFile,
      row.currentName, row.chosenName || '', row.chosenDisplay, row.className,
      row.ownerKind, row.methodName, row.assessment, row.confidence || '', row.provenance || '', row.proof || '',
    ].map(csvCell).join('\t'));

    if (row.className && row.ownerKind !== 'free') {
      tally.withClass++;
      let e = classMap.get(row.className);
      if (!e) {
        const nsParts = row.className.split('::');
        e = {
          className: row.className,
          namespace: nsParts.slice(0, -1).join('::'),
          simpleName: nsParts[nsParts.length - 1],
          kind: owner.kind,
          modules: new Set(),
          methods: [],
          resolved: 0,
          unresolved: 0,
          sample: [],
        };
        classMap.set(row.className, e);
      }
      if (owner.kind === 'class') e.kind = 'class';
      e.modules.add(row.module);
      e.methods.push(row.methodName);
      if (row.chosenName) e.resolved++; else e.unresolved++;
      if (e.sample.length < 3) e.sample.push(row.address);
    }
  }

  const classOnly = [...classMap.values()].filter((c) => c.kind === 'class');
  const nsOnly = [...classMap.values()].filter((c) => c.kind !== 'class');
  tally.classes = classOnly.length;
  tally.namespaces = nsOnly.length;

  const classTsv = ['class\tnamespace\tsimpleName\tmethodCount\tresolvedNames\tunresolvedNames\tmodules\tsampleAddresses'];
  const classes = classOnly.sort((a, b) => b.methods.length - a.methods.length);
  for (const c of classes) {
    classTsv.push([
      c.className, c.namespace, c.simpleName, c.methods.length, c.resolved, c.unresolved,
      [...c.modules].join(','), c.sample.join(','),
    ].map(csvCell).join('\t'));
  }

  const nameTsvHeader = ['ledgerLine\tmodule\taddress\truntimeAddress\trustFile\tcurrentName\tchosenName\tchosenDisplay\towner\townerKind\tmethod\tassessment\tconfidence\tprovenance\tproof'];

  const nsTsv = ['owner\tmethodCount\tresolvedNames\tunresolvedNames\tmodules\tsampleAddresses'];
  for (const c of nsOnly.sort((a, b) => b.methods.length - a.methods.length)) {
    nsTsv.push([
      c.className, c.methods.length, c.resolved, c.unresolved,
      [...c.modules].join(','), c.sample.join(','),
    ].map(csvCell).join('\t'));
  }

  fs.writeFileSync(path.join(OUT, 'phase1-name-table.tsv'), nameTsvHeader.concat(tsv).join('\n') + '\n');
  fs.writeFileSync(path.join(OUT, 'phase1-name-table.jsonl'), jsonl.join('\n') + '\n');
  fs.writeFileSync(path.join(OUT, 'phase1-class-list.tsv'), classTsv.join('\n') + '\n');
  fs.writeFileSync(path.join(OUT, 'phase1-namespace-list.tsv'), nsTsv.join('\n') + '\n');
  fs.writeFileSync(path.join(OUT, 'phase1-summary.json'), JSON.stringify({
    generated: new Date().toISOString(),
    inputs: { evidence: EVIDENCE, crossCheck: V13 },
    tally,
    conflicts,
    topClasses: classes.slice(0, 25).map((c) => ({ class: c.className, methods: c.methods.length, resolved: c.resolved })),
  }, null, 2) + '\n');

  console.log(JSON.stringify(tally, null, 2));
  if (tally.conflicts) console.log('conflicts:', JSON.stringify(conflicts, null, 2));
}

main();
