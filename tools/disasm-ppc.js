const fs = require('fs');
const path = process.argv[2];
const addrs = process.argv.slice(3).map(s => parseInt(s, 16));
const buf = fs.readFileSync(path);
const phoff = buf.readUInt32BE(0x1c);
const phentsize = buf.readUInt16BE(0x2a);
const phnum = buf.readUInt16BE(0x2c);
const segs = [];
for (let i = 0; i < phnum; i++) {
  const o = phoff + i * phentsize;
  if (buf.readUInt32BE(o) !== 1) continue;
  segs.push({ off: buf.readUInt32BE(o + 4), vaddr: buf.readUInt32BE(o + 8), filesz: buf.readUInt32BE(o + 16) });
}
function readAt(vaddr, len) {
  for (const s of segs) {
    if (vaddr >= s.vaddr && vaddr + len <= s.vaddr + s.filesz) {
      return buf.subarray(s.off + (vaddr - s.vaddr), s.off + (vaddr - s.vaddr) + len);
    }
  }
  throw new Error('unmapped ' + vaddr.toString(16));
}
const rnames = ['r0','r1','r2','r3','r4','r5','r6','r7','r8','r9','r10','r11','r12','r13','r14','r15','r16','r17','r18','r19','r20','r21','r22','r23','r24','r25','r26','r27','r28','r29','r30','r31'];
const sprnames = {1:'xer',8:'lr',9:'ctr',18:'dsisr',19:'dar',22:'dec',25:'sdr1',26:'srr0',27:'srr1',272:'sprg0',273:'sprg1',274:'sprg2',275:'sprg3',282:'ear',284:'tb',285:'tbu',287:'pvr',512:'gqr0',513:'gqr1',514:'gqr2',515:'gqr3',516:'gqr4',517:'gqr5',518:'gqr6',519:'gqr7',912:'hid0',913:'hid1',914:'iabr',915:'dabr',920:'dabr',921:'dabrx',936:'mmcr0',937:'pmc1',938:'pmc2',939:'sia',940:'mmcr1',941:'pmc3',942:'pmc4',943:'sda',952:'mmcr2',956:'sprg4',957:'sprg5',958:'sprg6',959:'sprg7'};
const cond = {12:'ge',4:'lt',2:'gt',8:'le',0:'ne',1:'beq',2:'bne',3:'blt',4:'bge',5:'bgt',6:'ble',7:'bso',8:'bns',9:'bun',10:'bnu',11:'bs',12:'bns?',13:'bso?',14:'bnl',15:'bnl?',16:'bdnz',17:'bdz',18:'bdnzt',19:'bdnzf',20:'bdz',21:'bdzt',22:'bdzf'};
function sx(v, bits) { const m = 1 << (bits - 1); return (v & m) ? v - (1 << bits) : v; }
function dec(w, addr) {
  const op = w >>> 26;
  const rt = (w >>> 21) & 31, ra = (w >>> 16) & 31, rb = (w >>> 11) & 31;
  const imm = sx(w & 0xffff, 16);
  const d = sx(w & 0xffff, 16);
  const ea = (ra === 0 ? '' : `+0x${(d >>> 0).toString(16)}`) ;
  const mem = ra === 0 ? `0x${(d >>> 0).toString(16)}` : `0x${(d >>> 0).toString(16)}(${rnames[ra]})`;
  switch (op) {
    case 3: return `twi 0x${((w>>>16)&31).toString(16)},${rnames[ra]},${imm}`;
    case 7: return `mulli ${rnames[rt]},${rnames[ra]},${imm}`;
    case 8: return `subfic ${rnames[rt]},${rnames[ra]},${imm}`;
    case 10: return `cmpli cr${(w>>>23)&7},${(w>>>16)&31},${rnames[ra]},0x${(w&0xffff).toString(16)}`;
    case 11: return `cmpi cr${(w>>>23)&7},${(w>>>16)&31},${rnames[ra]},${imm}`;
    case 12: return `addic ${rnames[rt]},${rnames[ra]},${imm}`;
    case 13: return `addic. ${rnames[rt]},${rnames[ra]},${imm}`;
    case 14: return ra === 0 ? `li ${rnames[rt]},${imm}` : `addi ${rnames[rt]},${rnames[ra]},${imm}`;
    case 15: return ra === 0 ? `lis ${rnames[rt]},0x${((w & 0xffff) >>> 0).toString(16)}` : `addis ${rnames[rt]},${rnames[ra]},0x${((w & 0xffff) >>> 0).toString(16)}`;
    case 16: {
      const bo = (w >>> 21) & 31, bi = (w >>> 16) & 31, bd = sx(w & 0xfffc, 16);
      return `bc ${bo},${bi},0x${(addr + bd).toString(16)}`;
    }
    case 18: {
      const li = sx(w & 0x3fffffc, 26); const aa = (w >>> 1) & 1; const lk = w & 1;
      return `${lk ? 'bl' : 'b'}${aa ? 'a' : ''} 0x${((aa ? 0 : addr) + li).toString(16)}`;
    }
    case 19: {
      const xo = (w >>> 1) & 0x3ff;
      if (xo === 16) return `bclr ${(w>>>21)&31},${(w>>>16)&31}`;
      if (xo === 528) return `bcctr ${(w>>>21)&31},${(w>>>16)&31}`;
      if (xo === 50) return 'rfi';
      // XL-form condition-register ops: BT,BA,BB are bits 21-25, 16-20,
      // 11-15 - same field slots as bclr's BO/BI.  No Rc bit in this form.
      {
        const bt = (w>>>21)&31, ba = (w>>>16)&31, bb = (w>>>11)&31;
        const crops = { 33: 'crnor', 129: 'crandc', 193: 'crxor', 225: 'crnand',
                        257: 'crand', 289: 'creqv', 417: 'crorc', 449: 'cror' };
        if (crops[xo]) return `${crops[xo]} ${bt},${ba},${bb}`;
      }
      return `xl${xo}`;
    }
    case 20: { const sh=(w>>>11)&31, mb=(w>>>6)&31, me=(w>>>1)&31; return `rlwimi ${rnames[ra]},${rnames[rt]},${sh},${mb},${me}`; }
    case 21: { const sh=(w>>>11)&31, mb=(w>>>6)&31, me=(w>>>1)&31; return `rlwinm${w&1?' rc':''} ${rnames[ra]},${rnames[rt]},${sh},${mb},${me}`; }
    case 23: return `rlwnm ${rnames[ra]},${rnames[rt]},${rnames[rb]},${(w>>>6)&31},${(w>>>1)&31}`;
    case 24: return `ori ${rnames[ra]},${rnames[rt]},0x${(w&0xffff).toString(16)}`;
    case 25: return `oris ${rnames[ra]},${rnames[rt]},0x${(w&0xffff).toString(16)}`;
    case 26: return `xori ${rnames[ra]},${rnames[rt]},0x${(w&0xffff).toString(16)}`;
    case 27: return `xoris ${rnames[ra]},${rnames[rt]},0x${(w&0xffff).toString(16)}`;
    case 28: return `andi. ${rnames[ra]},${rnames[rt]},0x${(w&0xffff).toString(16)}`;
    case 29: return `andis. ${rnames[ra]},${rnames[rt]},0x${(w&0xffff).toString(16)}`;
    case 31: {
      const xo = (w >>> 1) & 0x3ff;
      switch (xo) {
        case 0: return `cmp cr${(w>>>23)&7},${rnames[ra]},${rnames[rb]}`;
        case 32: return `cmpl cr${(w>>>23)&7},${rnames[ra]},${rnames[rb]}`;
        case 40: return `subf ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 266: return `add${w&1?' rc':''} ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 444: return rt === rb ? `mr ${rnames[ra]},${rnames[rt]}` : `or ${rnames[ra]},${rnames[rt]},${rnames[rb]}`;
        case 28: return `and ${rnames[ra]},${rnames[rt]},${rnames[rb]}`;
        case 60: return `andc ${rnames[ra]},${rnames[rt]},${rnames[rb]}`;
        case 124: return `nor ${rnames[ra]},${rnames[rt]},${rnames[rb]}`;
        case 284: return `xor ${rnames[ra]},${rnames[rt]},${rnames[rb]}`;
        case 316: return `xnor ${rnames[ra]},${rnames[rt]},${rnames[rb]}`;
        case 235: return `mullw ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 491: return `divw ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 459: return `divwu ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 23: return `lwzx ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 151: return `stwx ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 87: return `lbzx ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 215: return `stbx ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 279: return `lhzx ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 407: return `sthx ${rnames[rt]},${rnames[ra]},${rnames[rb]}`;
        case 535: return `lfsx f${rt},${rnames[ra]},${rnames[rb]}`;
        case 599: return `lfdx f${rt},${rnames[ra]},${rnames[rb]}`;
        case 663: return `stfsx f${rt},${rnames[ra]},${rnames[rb]}`;
        case 727: return `stfdx f${rt},${rnames[ra]},${rnames[rb]}`;
        case 339: {
          const spr = (((w >>> 16) & 31) << 5) | ((w >>> 11) & 31);
          return `mfspr ${rnames[rt]},${sprnames[spr] || spr}`;
        }
        case 467: {
          const spr = (((w >>> 16) & 31) << 5) | ((w >>> 11) & 31);
          return `mtspr ${sprnames[spr] || spr},${rnames[rt]}`;
        }
        case 19: return `mfcr ${rnames[rt]}`;
        case 144: return `mtcrf 0x${(((w>>>12)&0xff)).toString(16)},${rnames[rt]}`;
        case 26: return `cntlzw ${rnames[ra]},${rnames[rt]}`;
        case 24: return `slw ${rnames[ra]},${rnames[rt]},${rnames[rb]}`;
        case 536: return `srw ${rnames[ra]},${rnames[rt]},${rnames[rb]}`;
        case 792: return `sraw ${rnames[ra]},${rnames[rt]},${rnames[rb]}`;
        case 824: return `srawi ${rnames[ra]},${rnames[rt]},${(w>>>11)&31}`;
        case 598: return 'sync';
        case 854: return 'eieio';
        case 306: return 'tlbie';
        default: return `X31_${xo}`;
      }
    }
    case 32: return `lwz ${rnames[rt]},${mem}`;
    case 33: return `lwzu ${rnames[rt]},${mem}`;
    case 34: return `lbz ${rnames[rt]},${mem}`;
    case 35: return `lbzu ${rnames[rt]},${mem}`;
    case 36: return `stw ${rnames[rt]},${mem}`;
    case 37: return `stwu ${rnames[rt]},${mem}`;
    case 38: return `stb ${rnames[rt]},${mem}`;
    case 39: return `stbu ${rnames[rt]},${mem}`;
    case 40: return `lhz ${rnames[rt]},${mem}`;
    case 41: return `lhzu ${rnames[rt]},${mem}`;
    case 42: return `lha ${rnames[rt]},${mem}`;
    case 43: return `lhau ${rnames[rt]},${mem}`;
    case 44: return `sth ${rnames[rt]},${mem}`;
    case 45: return `sthu ${rnames[rt]},${mem}`;
    case 46: return `lmw ${rnames[rt]},${mem}`;
    case 47: return `stmw ${rnames[rt]},${mem}`;
    case 48: return `lfs ${rnames[rt]},${mem}`;
    case 49: return `lfsu ${rnames[rt]},${mem}`;
    case 50: return `lfd ${rnames[rt]},${mem}`;
    case 51: return `lfdu ${rnames[rt]},${mem}`;
    case 52: return `stfs ${rnames[rt]},${mem}`;
    case 53: return `stfsu ${rnames[rt]},${mem}`;
    case 54: return `stfd ${rnames[rt]},${mem}`;
    case 55: return `stfdu ${rnames[rt]},${mem}`;
    case 56: case 57: { const fd=(w>>>21)&31, fa=(w>>>16)&31, dq=sx(w&0xfff,12), gqr=(w>>>12)&7, wbit=(w>>>15)&1; return `${op===56?'psq_l':'psq_lu'} f${fd},0x${(dq>>>0).toString(16)}(${rnames[fa]}),${wbit},${gqr}`; }
    case 60: case 61: { const fs=(w>>>21)&31, fa=(w>>>16)&31, dq=sx(w&0xfff,12), gqr=(w>>>12)&7, wbit=(w>>>15)&1; return `${op===60?'psq_st':'psq_stu'} f${fs},0x${(dq>>>0).toString(16)}(${rnames[fa]}),${wbit},${gqr}`; }
    case 17: return 'sc';
    case 59: {
      // A-form: two-operand arithmetic (fadds/fsubs/fdivs/...) uses FRB
      // (bits 16-20), but fmuls uses FRC (bits 21-25). Four-operand forms use
      // fD,fA,fC,fB.
      const xo = (w >>> 1) & 0x1f;
      const fd = (w >>> 21) & 31, fa = (w >>> 16) & 31, fb = (w >>> 11) & 31, fc = (w >>> 6) & 31;
      const two = { 18: 'fdivs', 20: 'fsubs', 21: 'fadds', 22: 'fsqrts', 24: 'fres', 26: 'frsqrts' };
      const four = { 28: 'fmsubs', 29: 'fmadds', 30: 'fnmsubs', 31: 'fnmadds' };
      if (two[xo]) return `${two[xo]} f${fd},f${fa},f${fb}`;
      if (xo === 25) return `fmuls f${fd},f${fa},f${fc}`;
      if (four[xo]) return `${four[xo]} f${fd},f${fa},f${fc},f${fb}`;
      return `F59_${xo} f${fd},f${fa},f${fb}`;
    }
    case 63: {
      // Opcode 63 mixes A-form (5-bit XO) and X-form (10-bit XO: fmr, fneg,
      // fabs, fnabs, frsp, fctiwz, ...).
      const xo10 = (w >>> 1) & 0x3ff;
      const fd = (w >>> 21) & 31, fa = (w >>> 16) & 31, fb = (w >>> 11) & 31, fc = (w >>> 6) & 31;
      // X-form compares: crfD occupies bits 6-8 (the top three bits of the
      // "fd" field, so fd itself is garbage here), and the two compared FPRs
      // are FRA (bits 11-15) then FRB (bits 16-20) - i.e. fa then fb.  Do not
      // fall through to the arithmetic fallback, which has no crfD slot.
      if (xo10 === 0 || xo10 === 32) {
        return `${xo10 === 32 ? 'fcmpo' : 'fcmpu'} cr${(w >>> 23) & 7},f${fa},f${fb}`;
      }
      const xform = { 12: 'frsp', 14: 'fctiw', 15: 'fctiwz', 40: 'fneg', 72: 'fmr', 136: 'fnabs', 264: 'fabs' };
      if (xform[xo10]) return `${xform[xo10]} f${fd},f${fb}`;
      const two = { 18: 'fdiv', 20: 'fsub', 21: 'fadd', 22: 'fsqrt', 26: 'frsqrts' };
      const four = { 28: 'fmsub', 29: 'fmadd', 30: 'fnmsub', 31: 'fnmadd' };
      if (two[xo10]) return `${two[xo10]} f${fd},f${fa},f${fb}`;
      if (xo10 === 25) return `fmul f${fd},f${fa},f${fc}`;
      if (four[xo10]) return `${four[xo10]} f${fd},f${fa},f${fc},f${fb}`;
      return `F63_${xo10} f${fd},f${fa},f${fb}`;
    }
    default: return `OP_${op}`;
  }
}
for (const a of addrs) {
  console.log('===== ' + a.toString(16));
  let n = 0;
  // read up to 1400 bytes, stop after blr/bctr with depth heuristic (just dump 64 words)
  const b = readAt(a, 0x500);
  const maxw = parseInt(process.env.LEN || '60', 10);
  for (let i = 0; i < 320; i += 4) {
    const w = b.readUInt32BE(i);
    const text = dec(w, a + i);
    console.log((a + i).toString(16) + ': ' + w.toString(16).padStart(8, '0') + '  ' + text);
    n++;
    if (n >= maxw) break;
    const op = w >>> 26;
    if (process.env.STOP === '0') continue;
    if (w === 0x4e800020 || w === 0x4c820020) break; // blr
    if (op === 19 && ((w >>> 1) & 0x3ff) === 528) break; // bcctr
    if (op === 18 && (w & 1) === 0) break; // tail b
  }
}
