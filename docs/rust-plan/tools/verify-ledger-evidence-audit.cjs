const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),assert=require('node:assert/strict'),{spawnSync}=require('node:child_process');
const root='C:/Users/Wither/Documents/Codex/2026-11-25/rad/';
const checkout='C:/Users/Wither/.codex/worktrees/rust-guest-memory/decomp';
const paths=['phase1-ledger-evidence.jsonl','phase1-evidence-verification.json'].map(s=>root+'outputs/'+s);
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const initial=paths.map(sha);
let run=spawnSync('node',[root+'work/phase1/ledger-evidence-audit.cjs'],{encoding:'utf8'});
assert.equal(run.status,0,run.stderr);
assert.deepEqual(paths.map(sha),initial);
const s=JSON.parse(fs.readFileSync(paths[1],'utf8'));
for(const item of Object.values(s.inputs)) assert.equal(sha(item.file),item.sha256);
const raw=Object.fromEntries(['ledger','recomp','normalizedE1','reviewedE1'].map(k=>[k,fs.readFileSync(s.inputs[k].file,'utf8').split(/\r?\n/)]));
const columns=Object.fromEntries(raw.reviewedE1[0].split('\t').map((k,i)=>[k,i]));
const live=JSON.parse(fs.readFileSync(root+'work/rust-phase0/phase0-rel-layout-verification.json','utf8'));
for(const rel of s.relCode) {
  const check=live.find(x=>x.module===rel.id)?.sections.find(x=>x.index===rel.section && x.executable);
  assert(check);
  assert.equal(check.canonical,rel.start);
  assert.equal(check.size,rel.end-rel.start);
  assert.equal(rel.mapping.anchors.length,3);
  for (const a of rel.mapping.anchors) {
    const row=raw.reviewedE1[a.reviewedLine-1].split('\t');
    assert.equal(row[columns.name],a.name);
    assert.equal(row[columns.module],rel.module);
    assert(row[columns.evidence].includes(rel.raw.sha256));
    assert.equal(Number(row[columns.address])-a.sectionOffset,rel.mapping.mergedBase);
    assert.equal(Number(row[columns.runtime_address])-a.sectionOffset,rel.mapping.publicBase);
  }
}
const rows=fs.readFileSync(paths[0],'utf8').trim().split('\n').map(JSON.parse);
const seen=new Set();let cites=0;const by={};const placements={};
for(const row of rows) {
  const addr=Number(row.address);
  assert(!seen.has(addr));seen.add(addr);
  assert(raw.ledger[row.ledgerLine-1].toLowerCase().includes(row.address.slice(2)));
  assert(fs.existsSync(path.join(checkout,row.rustFile)));
  const found=[...s.mainRanges,...s.relCode].filter(x=>addr>=x.start&&addr<x.end);
  assert.equal(found.length,1);
  const [section]=found;
  assert.equal(row.location.module,section.module);
  assert.equal(row.location.section,section.section);
  assert.equal(row.location.offset,addr-section.start);
  if (section.id) assert.equal(Number(row.location.capturedRuntime),section.liveStart+row.location.offset);
  placements[row.placement]=(placements[row.placement]??0)+1;
  by[row.location.module]=(by[row.location.module]??0)+1;
  for (const e of row.exports) assert.equal(e.available,fs.existsSync(path.join(checkout,e.path)));
  for(const c of row.candidates) {
    const line=raw[c.source][c.line-1];assert(line);
    if(c.source==='recomp') {
      const [,value,name]=/^\s*([\da-f]{8})\s+(.+?)\s*$/i.exec(line);
      assert.equal(Number('0x'+value),addr);assert.equal(name,c.spelling);
    } else {
      const cells=line.split('\t');
      if(c.source==='normalizedE1') {
        assert.equal(Number('0x'+cells[0]),addr);assert.equal(cells[1],c.spelling);
      }else {
        assert.equal(cells[columns.module],row.location.module);
        assert.equal(cells[columns.name],c.spelling);
        assert.equal(c.signature,cells[columns.source_signature]||undefined);
        const reviewed=Number(cells[columns.address]),runtime=Number(cells[columns.runtime_address]);
        if(section.mapping) {
          assert.equal(reviewed-section.mapping.mergedBase,row.location.offset);
          assert.equal(runtime-section.mapping.publicBase,row.location.offset);
        } else assert.equal(reviewed,addr);
      }
    }
    cites++;
  }
}
for(const rejected of s.rejectedReviewRows) assert(raw.reviewedE1[rejected.line-1]?.split('\t')[columns.module]===rejected.module);
assert.equal(s.rejectedReviewRows.length,Object.values(s.reviewExcluded).reduce((a,b)=>a+b,0));
assert.equal(rows.length,s.totals.rows);
assert.deepEqual(placements,s.totals.placement);
for(const [k,v] of Object.entries(by))assert.equal(v,s.totals.byModule[k].rows);
const ledger=spawnSync('node',['tools/verify-ledger.mjs','--enforce-bodies'],{cwd:checkout,encoding:'utf8'});
const check=JSON.parse(ledger.stdout);
assert.equal(check.rows,rows.length);
assert.equal(check.uniqueAddresses,seen.size);
assert(check.bodies.ok);
assert.equal(check.inventory.missingExports.length,5);
console.log('Verified byte-identical regeneration, '+rows.length+' unique ledger rows, '+cites+' candidate citations, 12 reviewed anchors, and '+s.rejectedReviewRows.length+' rejected-source citations.');
console.log('Existing ledger body check passed. Its five expected inventory exports are missing.');
console.log('Output SHA-256',initial);