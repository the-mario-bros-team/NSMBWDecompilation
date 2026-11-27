const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const root = 'C:/Users/Wither/.codex/worktrees/rust-guest-memory/decomp';
const output = 'C:/Users/Wither/Documents/Codex/2026-11-25/rad/outputs';
const disc = 'E:/Localsend/mariobroswii/sys';
const inputs = {
  ledger: path.join(root, 'ledger/functions.csv'),
  recomp: 'E:/NSMBWRecomp/wiicompiled/projects/nsmbw/MAP.txt',
  normalizedE1: 'E:/NSMBWRecomp/maps/nsmbw_e1_symbols.tsv',
  reviewedE1: 'E:/NSMBWDecompCodex/analysis/symbols/symbols_E1_merged_v13.tsv',
};

function prefix(file, length) {
  const fd = fs.openSync(file, 'r');
  try {
    const buffer = Buffer.alloc(length);
    if (fs.readSync(fd, buffer, 0, length, 0) !== length) throw new Error(file + ' is truncated');
    return buffer;
  } finally {
    fs.closeSync(fd);
  }
}

function address(raw) {
  if (!/^(?:0x)?[0-9a-f]{8}$/i.test(raw)) throw new Error('Invalid address ' + raw);
  return Number.parseInt(raw.replace(/^0x/i, ''), 16);
}

function hex(value) {
  return '0x' + value.toString(16).padStart(8, '0');
}

function isPlaceholder(name) {
  return /^(?:(?:fn|fun|sub)_(?:0x)?[0-9a-f]{8}|0x[0-9a-f]{8})(?:__.*)?$/i.test(name);
}

function csv(text) {
  const rows = [];
  let row = [], field = '', quoted = false, line = 1, rowStart = 1;
  for (let i = 0; i < text.length; i += 1) {
    const ch = text[i];
    if (quoted && ch === '"') {
      if (text[i + 1] === '"') {
        field += '"';
        i += 1;
      } else quoted = false;
    } else if (!quoted && ch === '"') quoted = true;
    else if (!quoted && ch === ',') {
      row.push(field);
      field = '';
    } else if (!quoted && ch === '\n') {
      rows.push({ line: rowStart, fields: [...row, field] });
      row = [];
      field = '';
      rowStart = line + 1;
    } else if (ch !== '\r') field += ch;
    if (ch === '\n') line += 1;
  }
  if (quoted) throw new Error('Unterminated ledger field');
  if (field || row.length) rows.push({ line: rowStart, fields: [...row, field] });
  return rows;
}

const { pathToFileURL } = require('node:url');
const { spawnSync } = require('node:child_process');
const capture = 'C:/Users/Wither/Documents/Codex/2026-11-25/rad/work/rust-phase0/1-1-rust-memcpy.wBbOls/rel-ram.bin';
const relDir = 'E:/Localsend/mariobroswii/files/rels';
const reviewedReadme = 'E:/NSMBWDecompCodex/analysis/symbols/README.md';
const sha = (b) => crypto.createHash('sha256').update(b).digest('hex');

async function audit() {
  const boot = prefix(path.join(disc, 'boot.bin'), 8);
  const revision = boot.subarray(0, 6).toString('ascii') + ' rev ' + boot[7];
  if (revision !== 'SMNE01 rev 1') throw Error('unexpected executable revision ' + revision);
  const dolBytes = fs.readFileSync(path.join(disc, 'main.dol'));
  const dolSha = sha(dolBytes);
  const readme = fs.readFileSync(reviewedReadme, 'utf8');
  if (!readme.includes(dolSha)) throw Error('reviewed table executable digest does not match');
  const main = Array.from({length: 7}, (_, i) => {
    const start = dolBytes.readUInt32BE(0x48 + 4 * i);
    return {module: 'main', section: i, start, end: start + dolBytes.readUInt32BE(0x90 + 4 * i)};
  }).filter(s => s.end > s.start);
  const relFiles = fs.readdirSync(relDir).filter(n => /\.rel\.lz$/i.test(n)).sort().map(n => path.join(relDir,n));
  const r = spawnSync('node', [path.join(root, 'tools/rel2elf.mjs'), '--address-layout', ...relFiles], {encoding: 'utf8'});
  if (r.status !== 0) throw Error(r.stderr);
  const layout = JSON.parse(r.stdout);
  const {decodeLz11} = await import(pathToFileURL(path.join(root, 'tools/lz11.mjs')).href);
  const rawById = new Map();
  for (const file of relFiles) {
    const b = decodeLz11(fs.readFileSync(file));
    rawById.set(b.readUInt32BE(0), {file, sha256: sha(b), fields: [0,1,2].map(i => ({
      headerOffset: 0x34 + 4 * i, section: b[0x30 + i], offset: b.readUInt32BE(0x34 + 4 * i),
    }))});
  }
  const tables = new Map();
  const ram = fs.openSync(capture, 'r');
  const mem1 = 24 << 20, mem2 = 64 << 20;
  const pos = (a,n) => {
    if (a >= 0x80000000 && a + n <= 0x80000000 + mem1) return a - 0x80000000;
    if (a >= 0x90000000 && a + n <= 0x90000000 + mem2) return mem1 + a - 0x90000000;
    throw Error('address outside captured RAM: ' + hex(a));
  };
  const word = (a) => {
    const b = Buffer.alloc(4);
    if (fs.readSync(ram,b,0,4,pos(a,4)) !== 4) throw Error('truncated RAM capture');
    return b.readUInt32BE(0);
  };
  try {
    let prev=0;
    const seen=new Set();
    for(let h=word(0x800030c8);h;h=word(h+4)) {
      if(seen.has(h)||word(h+8)!==prev) throw Error('invalid live module list');
      seen.add(h);
      const id=word(h), count=word(h+12), table=word(h+16);
      if(tables.has(id)) throw Error('duplicate live module');
      tables.set(id, Array.from({length:count},(_,i)=>({ref:word(table+8*i),size:word(table+8*i+4)})));
      prev=h;
    }
  } finally {fs.closeSync(ram);}
  if (layout.modules.length !== 4 || rawById.size !== 4 || tables.size !== 4) throw Error('incomplete REL layout');
  const rel=[];
  for(const m of layout.modules) {
    const raw=rawById.get(m.moduleId), live=tables.get(m.moduleId);
    if(!raw || !live) throw Error('missing REL evidence '+m.name);
    for(const s of m.sections) {
      const l=live[s.index];
      if(!l || l.size!==s.size || Boolean(l.ref&1)!==s.executable) throw Error('REL section mismatch in '+m.name);
      const liveStart=(l.ref&~1)>>>0;
      pos(liveStart,s.size);
      if(s.executable) rel.push({module:m.name,id:m.moduleId,section:s.index,start:address(s.vaddr),
        end:address(s.vaddr)+s.size,liveStart,raw,mapping:null});
    }
  }
  const tableText = fs.readFileSync(inputs.reviewedE1,'utf8');
  const [header,...content] = tableText.split(/\r?\n/);
  const cols=Object.fromEntries(header.split('\t').map((n,i)=>[n,i]));
  const reviewed=content.flatMap((line,i)=>{
    if(!line) return [];
    const fields=line.split('\t'), at=(k)=>fields[cols[k]];
    return at('kind')!=='function'?[]:[{line:i+2,module:at('module'),addr:at('address'),runtime:at('runtime_address'),
      spelling:at('name'),display:at('display_name'),signature:at('source_signature')||undefined,
      confidence:at('confidence'),sourceVersion:at('source_version'),evidence:at('evidence'),
      region:at('section'),sourceLabel:at('source')}];
  });
  for(const s of rel) {
    const anchors=[];
    for(const [i,name] of ['_prolog','_epilog','_unresolved'].entries()) {
      const raw=s.raw.fields[i];
      if(raw.section!==s.section || raw.offset>=s.end-s.start) throw Error('header anchor outside code');
      const matches=reviewed.filter(x=>x.module===s.module && x.spelling===name &&
        x.sourceVersion==='E1' && x.evidence?.includes('SHA256='+s.raw.sha256) &&
        x.evidence.includes('header+0x'+raw.headerOffset.toString(16)));
      if(matches.length!==1) throw Error('missing or ambiguous reviewed anchor '+s.module+' '+name);
      const m=matches[0];
      anchors.push({name,sectionOffset:raw.offset,reviewedLine:m.line,mergedBase:address(m.addr)-raw.offset,
        publicBase:address(m.runtime)-raw.offset});
    }
    if(!anchors.every(a=>a.mergedBase===anchors[0].mergedBase && a.publicBase===anchors[0].publicBase)) {
      throw Error('non-uniform REL review mapping '+s.module);
    }
    s.mapping={mergedBase:anchors[0].mergedBase,publicBase:anchors[0].publicBase,anchors};
  }

  const sections=[...main,...rel];
  const find=(value)=>sections.filter(s=>value>=s.start&&value<s.end);
  const rows=csv(fs.readFileSync(inputs.ledger,'utf8').replace(/^\uFEFF/,''));
  const ledgerHead=rows.shift().fields;
  const colsLedger=Object.fromEntries(ledgerHead.map((n,i)=>[n,i]));
  const records=[];
  const byAddress=new Map();
  for(const item of rows) {
    if(item.fields.length!==ledgerHead.length) throw Error('invalid ledger row '+item.line);
    const read=(k)=>item.fields[colsLedger[k]];
    const value=address(read('address'));
    if(byAddress.has(value)) throw Error('duplicate ledger address '+hex(value));
    const paths=[...new Set([...read('evidence').matchAll(/work[\\/]ghidra[\\/](?:main(?:-gb)?|rel-d_[A-Za-z_]+)[\\/]export[\\/][^\s;,)"']+/gi)]
      .map(m=>m[0].replaceAll('\\','/')))];
    const claims=[...new Set(paths.map(p=>p.split('/')[2]).map(p=>p.startsWith('rel-')?p.slice(4):'main'))];
    const matches=find(value);
    const segment=matches[0];
    const placement = matches.length===0?'outside-executable-ranges':matches.length>1?'ambiguous-range'
      :claims.some(c=>c!==segment.module)?'evidence-module-conflict'
      :claims.length===0?'range-only':'module-and-range';
    const location=matches.length===1?{
      module:segment.module, id:segment.id, section:segment.section, offset:value-segment.start,
      capturedRuntime:segment.liveStart===undefined?undefined:hex(segment.liveStart+value-segment.start),
    }:null;
    const row={ledgerLine:item.line,address:hex(value),rustFile:read('rust_file'),currentName:read('name'),
      status:read('status'),confidence:read('confidence'),placeholder:isPlaceholder(read('name')),
      placement,location,exports:paths.map(p=>({path:p,available:fs.existsSync(path.join(root,p))})),candidates:[]};
    records.push(row);
    byAddress.set(value,row);
  }
  function add(value,module,candidate) {
    const row=byAddress.get(value);
    if(row?.location?.module===module && !['evidence-module-conflict','ambiguous-range'].includes(row.placement) &&
      candidate.spelling) row.candidates.push(candidate);
  }
  for(const [i,line] of fs.readFileSync(inputs.recomp,'utf8').split(/\r?\n/).entries()) {
    if(!line.trim())continue;
    const m=/^\s*([0-9a-f]{8})\s+(.+?)\s*$/i.exec(line);
    if(!m)throw Error('invalid recomp row '+(i+1));
    add(address(m[1]),'main',{source:'recomp',line:i+1,spelling:m[2]});
  }
  for(const [i,line] of fs.readFileSync(inputs.normalizedE1,'utf8').split(/\r?\n/).entries()) {
    if(!line.trim())continue;
    const [value,spelling,display]=line.split('\t');
    add(address(value),'main',{source:'normalizedE1',line:i+1,spelling,display});
  }
  const reviewExcluded={noExecutableModule:0,coordinateMismatch:0};
  const rejectedReviewRows=[];
  for(const candidate of reviewed) {
    const {module,addr,runtime,region,line,spelling,display,signature,confidence,sourceLabel,sourceVersion}=candidate;
    if(module==='main') {
      add(address(addr),'main',{source:'reviewedE1',line,spelling,display,signature,confidence,sourceLabel,sourceVersion});
      continue;
    }
    const s=rel.find(x=>x.module===module);
    if(!s || !addr || !runtime) {
      reviewExcluded.noExecutableModule++;
      const reason = s ? "missing reviewed coordinates" : "module outside the four verified executable RELs";
      rejectedReviewRows.push({line,module,addr,runtime,reason});
      continue;
    }
    const offset=address(addr)-s.mapping.mergedBase;
    if(region!=='.text'||offset<0||offset>=s.end-s.start||address(runtime)!==s.mapping.publicBase+offset) {
      reviewExcluded.coordinateMismatch++;
      const reason = region !== ".text" ? "reviewed section is not text"
        : offset < 0 || offset >= s.end-s.start ? "merged address outside executable section"
        : "merged and public runtime offsets disagree";
      rejectedReviewRows.push({line,module,addr,runtime,reason});
      continue;
    }
    add(s.start+offset,module,{source:'reviewedE1',line,spelling,display,signature,confidence,sourceLabel,sourceVersion,
      mergedAddress:addr,publicRuntime:runtime});
  }

  const totals={rows:records.length,placement:{},byModule:{},candidateRows:0,signatureRows:0,exportsAvailableRows:0,
    exportCitationRows:0,conflicts:0,placeholder:{agreement:0,singleSource:0,conflicts:0,noName:0}};
  for(const row of records) {
    totals.placement[row.placement]=(totals.placement[row.placement]||0)+1;
    const module=row.location?.module??'unresolved';
    const m=totals.byModule[module]??={rows:0,candidateRows:0,signatureRows:0,placeholder:0};
    m.rows++;
    if(row.placeholder)m.placeholder++;
    if(row.exports.length)totals.exportCitationRows++;
    if(row.exports.some(x=>x.available))totals.exportsAvailableRows++;
    const relevant=row.candidates.filter(c=>!isPlaceholder(c.spelling)&&!c.confidence?.includes('unknown'));
    const distinct=new Set(relevant.map(c=>c.spelling)).size;
    if(distinct){totals.candidateRows++;m.candidateRows++;}
    if(relevant.some(c=>c.signature)){totals.signatureRows++;m.signatureRows++;}
    row.assessment=distinct===0?'noName':distinct>1?'conflicts'
      :new Set(relevant.map(c=>c.source)).size>1?'agreement':'singleSource';
    if(row.assessment==='conflicts')totals.conflicts++;
    if(row.placeholder)totals.placeholder[row.assessment]++;
  }
  const inputsWithProof={...Object.fromEntries(Object.entries(inputs).map(([key,file])=>[key,{file,sha256:sha(fs.readFileSync(file))}])),
    dol:{file:path.join(disc,'main.dol'),sha256:dolSha},reviewedReadme:{file:reviewedReadme,sha256:sha(fs.readFileSync(reviewedReadme))},
    capture:{file:capture,sha256:sha(fs.readFileSync(capture))},relTool:{file:path.join(root,'tools/rel2elf.mjs'),sha256:sha(fs.readFileSync(path.join(root,'tools/rel2elf.mjs')))}};
  const verification={revision,inputs:inputsWithProof,mainRanges:main,relCode:rel,totals,reviewExcluded,rejectedReviewRows,
    notes:['Executable range placement alone does not establish a function boundary or implementation.',
      'REL names are joined by module, executable section and offset, using three header anchors and the exact module digest.',
      'The captured runtime addresses apply to the recorded RAM capture only. Reviewed public runtime addresses use a separate layout.',
      'Candidate agreement can reflect shared upstream provenance. Existing source signatures are quoted as metadata and no new ABI types or layouts are inferred.']};
  const inventory=path.join(output,'phase1-ledger-evidence.jsonl'), summary=path.join(output,'phase1-evidence-verification.json');
  fs.writeFileSync(inventory,records.map(row=>JSON.stringify(row)).join('\n')+'\n');
  fs.writeFileSync(summary,JSON.stringify(verification,null,2)+'\n');
  console.log(JSON.stringify({inventory,summary,totals,reviewExcluded}));
}
audit().catch(err=>{console.error(err.message);process.exitCode=1;});
