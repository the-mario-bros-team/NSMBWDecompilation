# Wave 140 - `game::bases` - the twelve largest untranslated bodies

Read-only specification lane. No Rust file, ledger row or existing document was
modified. The only file written is this one.

Waves 128-139 exhausted the class-B and class-C skip lists of
`docs/ANALYSIS_WAVE127_BASES.md`; the last class-C row (`0x807BD110`) is a
registration table, not a body. What remains in `d_basesNP` is a different
population: the large bodies that no earlier wave ever *attempted*, because they
were too big for a lane's budget or because their export prints the
`_savegpr_N` "`undefined8` return" that looked like a dropped receiver. With the
GQR3 word pinned (`0x00050005`) and the save-helper model settled in
`docs/REL_ELF_MAPPING.md`, those bodies are now claimable on their own terms.

This wave specifies the twelve largest such bodies, ranked by the `size` column
of `work/ghidra/rel-d_basesNP/export/functions.tsv`.

## The twelve

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x8079C330` | 4,232 | 1,058 | `8079c330_FUN_8079c330.c` | conditional (medium) |
| 2 | `0x8077B450` | 4,076 | 1,019 | `8077b450_FUN_8077b450.c` | ready (medium) |
| 3 | `0x8065CB20` | 3,788 | 947 | `8065cb20_FUN_8065cb20.c` | ready (medium) |
| 4 | `0x80661070` | 3,432 | 858 | `80661070_FUN_80661070.c` | conditional (medium) |
| 5 | `0x8079D3C0` | 3,176 | 794 | `8079d3c0_FUN_8079d3c0.c` | conditional (medium) |
| 6 | `0x8076DA30` | 2,656 | 664 | `8076da30_FUN_8076da30.c` | ready (medium) |
| 7 | `0x8078DC70` | 2,576 | 644 | `8078dc70_FUN_8078dc70.c` | conditional (medium) |
| 8 | `0x8076EF80` | 2,544 | 636 | `8076ef80_FUN_8076ef80.c` | conditional (medium) |
| 9 | `0x807108B0` | 2,468 | 617 | `807108b0_FUN_807108b0.c` | ready (medium) |
| 10 | `0x8065F910` | 2,204 | 551 | `8065f910_FUN_8065f910.c` | ready (medium) |
| 11 | `0x8076C900` | 2,100 | 525 | `8076c900_FUN_8076c900.c` | ready (medium) |
| 12 | `0x80773210` | 2,096 | 524 | `80773210_FUN_80773210.c` | conditional (medium) |

Together **35,348 bytes** (8,837 words). Every `bytes` value is the `size` column
of the TSV verbatim; the decoded extent of each body is listed per section.

### Selection rule

Candidate set: every `functions.tsv` row of `rel-d_basesNP` whose address is

1. absent from the `address` column of `ledger/functions.csv` (all modules, not
   only `game::bases`), and
2. absent from every `#{1,6} ` heading of every `docs/*.md`, and
3. not already carrying a `pub unsafe fn fn_<addr>` in `src/game/bases.rs`.

That leaves 2,048 rows. Ranked by `size` descending, the first twelve are the
table above. The 2,908-byte `0x807BD110` sits between #5 and #6 in raw size and is
**excluded on purpose**: it is the `dWorldSelect_c` 44-entry `StateID`
registration table, not a control-flow body, and `docs/SPEC_WAVE139_BASES.md`
records that it needs a data/table generator. Including it would make the lane
claim a row it cannot honestly specify as code.

The six rows immediately below the cut, in case a later lane needs more:
`0x8062C8E0` (1,804), `0x8065DD50` (1,772), `0x8065F090` (1,772),
`0x8071D500` (1,748), `0x8067FC80` (1,676), `0x807740D0` (1,656).

### Why this population was never claimed

The coverage documents name these bodies repeatedly and always defer them:

* `docs/COVERAGE_WAVE135_BASES.md` - "`0x8079C330` (4,232 bytes) and
  `0x8079D3C0` (3,176 bytes) tables were not attempted: each needs its own call
  graph and several depend on the `0x802DCD70` save-helper `undefined8` return
  model."
* `docs/COVERAGE_WAVE129_BASES.md` - "the `0x807740D0`/`0x80773210`/`0x8077B450`
  multi-route bodies, the `0x8077C440`/`0x8077C670`/`0x8077DC80` model holders
  ... were not attempted."
* `docs/COVERAGE_WAVE139_BASES.md` - "`0x807108B0` (2,468 bytes), `0x8071D500`
  (1,748), ... each needs its own call graph and was left for a dedicated pass."

This is that pass. The two blockers those documents name are both settled now:

1. **The `undefined8` return of a save helper is the incoming `r3:r4` pair.** A
   body that starts `bl 0x802DCD6C`/`686`/`70`/`74` and is printed as
   `uVar = func_0x...(); this = uVar >> 32; state = (int)uVar;` has, in the
   shipped bytes, its receiver in **entry `r3`** and its state argument in
   **entry `r4`**. The helper only spills `rN..r31` and LR; it does not touch
   `r3`/`r4`. Every section below states the recovered argument registers and
   the raw instruction that proves them (an `or rX,r3,r3` or `addi rX,r4,...`).
2. **GQR3 = `0x00050005`** (unsigned 16-bit, scale 0, both directions), pinned by
   wave 138 and restated in `docs/REL_ELF_MAPPING.md`. Of these twelve only
   `0x8065CB20`, `0x80661070`, `0x8076DA30`, `0x807108B0`, `0x8065F910` and
   `0x80773210` contain any `psq` at all, and every one of those sites is `qr0`
   (a plain `f32` pair or an `f31/f30/f29` spill). **No body here needs GQR3**;
   the class-C blocker does not gate this set. It is recorded because a later
   `qr3` body will reuse the same substitution.

## Method

**Address mapping.** Re-read from the program headers of
`work/rels/elf/d_basesNP.elf`, not copied:

```
PT_LOAD  p_offset  p_vaddr     filesz
  0      0x10F0    0x806000F0  0x1C6004
  1      0x1C70F4  0x807C60F4  0x0004FC
  2      0x1C75F0  0x807C65F0  0x00000C
  3      0x1C7600  0x807C6600  0x00A5FC
  4      0x1D1C00  0x807D0C00  0x051CB8
  5      0x000000  0x80822900  0x000000 (memsz 0x12484, zero-fill)
```

so

```
file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000
```

for every address in this document, as `docs/REL_ELF_MAPPING.md` requires.

**Jump tables.** Four of the twelve dispatch through a 32-bit table in the
REL's own `.data` (`0x8079C330` at `0x8081B884`, `0x8077B450` at `0x808181F8`,
`0x8079D3C0` at `0x8081BB00`, `0x8076DA30` at `0x80816AC4`); each was read and
expanded, so those case sets are the *actual* targets, not a guess from the
export's `switch`. A table entry that points at the epilogue block is a
`default` case. The other dispatchers compile to comparison trees and were
decoded branch by branch.

**Extent.** Full linear decode from the entry to the TSV extent. Eleven of the
twelve end in `0x4E800020` (`blr`) exactly at `entry + size - 4`. The exception
is `0x8065CB20`: its TSV extent stops at `0x8065D9E8` on `mfspr r0,lr`, and the
`blr` is one word later at `0x8065D9EC`; the TSV size is quoted verbatim and the
extent note is repeated in its section. No `halt_baddata`, no TSV row starting
inside a body, and every body has exactly one `blr` (none is a tail-call-only
fragment).

**Duplicates.** SHA-1 of each body against every same-size TSV row of
`rel-d_basesNP`: no match for any of the twelve.

**Decoder.** A throwaway PowerPC decoder (`%TEMP%\opencode\w140_ppc.js`, never in
the repo) read opcodes 4, 7-8, 10-29, 31-63, with `psq_l` = 56 and `psq_st` = 60
and the `I`/`W`/`DQ` field split from `docs/REL_ELF_MAPPING.md`. A-form
single-float ops (`fmuls`/`fmadds`/`fmsubs`) take their third operand from the
`FRC` field.

**Callee status.** Checked against `ledger/functions.csv`. `game::bases` does not
contain every `0x80C5xxxx`/`0x80C3xxxx` import or every `0x800xxxxx` engine
routine; where a callee is unclaimed this document says so and gives the call
shape (argument registers) rather than inventing a name. A body is still
**ready** when the call shape is literal, and **conditional** when an argument
depends on a callee contract that cannot be read from this REL.

## 1. `0x8079C330` - 4,232 bytes - `work/ghidra/rel-d_basesNP/export/8079c330_FUN_8079c330.c`

**Body:** `0x8079C330..0x8079D3B8` (1,058 words; `blr` at `0x8079D3B4`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x140`; prologue `bl 0x802DCD6C` (CodeWarrior `_savegpr` helper),
epilogue `bl 0x802DCDB8` (`_restgpr`). Argument recovery from the raw head: `or r28,r3,r3` at
`0x8079C368` proves **`this` = r3**; `addi r0,r4,-6` at `0x8079C348` proves
**`state` = r4**. `r5`/`r6` are not read before the switch.

Entry loads: `ctx = *(u32*)0x8042A1AC`, `a = *(u32*)0x8042A284`,
`w = *(u32*)0x80429F7C`, `arm = *(u32*)(ctx+0x1AC)`,
`sel = *(u32*)(ctx+0x1B0)`, `mode = *(u8*)0x8042A24D`,
`zero = *(f32*)0x807D0160` (= `0.0f`), `one = *(f32*)0x807D00C4` (= `1.0f`).

**Guard and dispatch.** `if ((u32)(state - 6) > 158) return;` then
`jump = *(u32*)(0x8081B884 + (state-6)*4)` (`lwzx`/`mtctr`/`bctr` at
`0x8079C384..0x8079C390`). The table is in this REL's `.data`; every entry was
expanded. Entries not listed below are the default block `0x8079D3A0`
(epilogue).

**Exact behaviour.** Blocks are named by their first address; `vcall(obj, off,
...)` is `(*(*(u32*)(obj) + off))(...)`.

| state(s) | block | behaviour |
|---|---|---|
| `6` | `0x8079CB28` | `if (mode == 7) 0x80102690(0,0,0); else 0x80102690(*(u32*)(0x807D0080 + mode*4) & 0xFF, 0, 0);` then `*(u32*)(w+0x44)=0; *(u32*)(w+0x40)=0; *(u32*)(w+0x60)=0;` |
| `0x0A` | `0x8079C420` | `*(u16*)(this+0x106)=*(u16*)(this+0x100); *(u16*)(this+0x108)=*(u16*)(this+0x102); *(u16*)(this+0x10A)=*(u16*)(this+0x104); *(u32*)(this+0x10C)=*(u32*)0x8031D3A4; *(f32*)(this+0x114)=zero; *(f32*)(this+0x118)=zero;` `0x80799960(this,1,0xFFFFFFFF);` `*(u32*)(this+0x264)=*(u32*)(this+0xAC); *(u32*)(this+0x268)=*(u32*)(this+0xB0); *(u32*)(this+0x26C)=*(u32*)(this+0xB4);` |
| `0x0B` | `0x8079C47C` | `sel = 0x80101630(ctx, sel); r = vcall(this+0x60, 0x84, this, sel); if (r == 0) { sel = 0x80101630(ctx); vcall(this+0x60, 0x70, this, sel, 0); } else { sel = 0x80101630(ctx); vcall(this+0x60, 0x78, this, sel); }` |
| `0x0C` | `0x8079C4F0` | `0x8079F390();` |
| `0x1A` | `0x8079C6F4` | `y = *(f32*)(this+0xB0) - *(f32*)0x8042D0C0; *(u32*)(this+0x1FC)=*(u32*)(this+0xAC); *(f32*)(this+0x200)=y; *(u32*)(this+0x204)=*(u32*)(this+0xB4);` |
| `0x1D` | `0x8079C71C` | `*(u8*)(this+0x124)=1; t=0x80162D50(0x284,0); f=*(f32*)0x807D01F8` (=1500)`)`; if `t==0` take `x=*(f32*)(this+0xAC)`, `y=*(f32*)(this+0xB0)`, `z=*(f32*)(this+0xB4)` and set `*(u32*)(a+0x604)=3; *(u32*)(a+0x5F4)=0; *(u32*)(a+0x5F0)=0; *(u8*)(a+0x624)=0; *(u32*)(a+0x608)=0;` with `x += f`; else take `x/y/z` from `t+0xAC/+0xB0/+0xB4`. Then `0x800F6320(20.0f, this, &vec3{x,y,z}, 0x3C)` (`f1` = `*(f32*)0x807D01FC`) and `this+0x1FC/0x200/0x204 = x/y/z`. |
| `0x1E` | `0x8079C7D8` | `0x80799960(this,10,0xFFFFFFFF);` |
| `0x23` | `0x8079C4F8` | same as `0x1A` with `*(f32*)0x8042D0BC` as the y subtractor. |
| `0x24` | `0x8079C67C` | `pos = this[+0x1FC..+0x204]` copied to `+0xAC..+0xB4`; `t=0x800F9BE0(0x282, *(u32*)(this+0x22C)); *(u32*)(this+0x2A0)=t;` if `t` then `*(f32*)(this+0x200) = *(f32*)(t+0xB0) + 60.0f` (`*(f32*)0x807D01A4`) and `0x807987E0(this);` |
| `0x25` | `0x8079C520` | `t=0x800F9BE0(0x282,*(u32*)(this+0x22C)); i=0; do { do { i=0x80162D50(0x282,i); } while (i==0); } while (i==t \|\| ((*(u32*)(i+4)>>16 & 0xF) != (*(u32*)(t+4)>>16 & 0xF)));` then `d = i.pos - this.pos;` (per-component), `this+0x1FC..+0x204 = i.pos`, `this+0x264..+0x26C = t.pos` with `t.y -= *(f32*)0x8042D0BC`, `*(u32*)(this+0x230) = (*(u32*)(i+4)>>8) & 0xFF`, `0x800F6410(this,&d)`, `this+0x10C=*(u32*)0x8031D460`, `this+0x114=*(f32*)0x8042D0B4`, `this+0x118=*(f32*)0x8042D0B8`, `*(u8*)(this+0x1A8)=0`, `0x80100200(&q, *(u32*)0x8042A18C, *(u32*)(this+0x22C))`, `v=0x80C56B90(*(u32*)(this+0x29C),0x400000)`, `0x80C565A0(*(u32*)(this+0x29C), v, &q)`, `0x80C561E0(*(u32*)(this+0x29C),0x10,1)`. |
| `0x26` | `0x8079C6DC` | `0x800B0DB0(1); 0x8079F900(this,1);` |
| `0x30` | `0x8079C7E8` | `t=0x80162D50(0x284,0);` if `t` then `0x80104FD0(*(u32*)0x8042A27C, 3, *(u8*)(*(u32*)(this+0x1D0)+0x151), *(u8*)(*(u32*)(this+0x1D0)+0x152));` then `*(f32*)(this+0x1FC) = *(f32*)(this+0xAC) + 800.0f` (`*(f32*)0x807D0200`), `this+0x200 = this+0xB0`, `this+0x204 = this+0xB4`, `this+0x10C = *(u32*)0x8042D0B0`, `this+0x114 = this+0x118 = this+0xEC = zero`, `*(u16*)(this+0x102)=0x4000`, `*(u16*)(this+0x108)=0x4000`, `0x80799960(this,2,0xFFFFFFFF)`, the five `a+0x5F0..0x624` stores of `0x1D`, `*(u32*)(w+0x370)=1`, `q=0x80775080(*(u32*)0x8042A18C, *(u32*)(this+0x22C)); *(u32*)(this+0x2E0)=q;` and if `*(i32*)(this+0x1AC) < 0` build the `0x80103410(*(u32*)0x8042A26C, mode==2 ? 5 : 0x80100970(), vcall(this+0x1D0,0x28), 0x80100A50(*(u8*)(*(u32*)(this+0x1D0)+0x151)), 0, this+0x2E4)` handle into `this+0x1AC`. |
| `0x31`,`0x33`,`0x35` | `0x8079C394/3A8/3BC` | `0x8079C150();` |
| `0x32`,`0x34`,`0x36` | `0x8079C39C/3B0/3C4` | `0x8079C230(this, arm);` |
| `0x37` | `0x8079C3D0` | `0x80105060(0.0f, *(u32*)0x8042A27C, 6, *(u8*)(*(u32*)(this+0x1D0)+0x151), *(u8*)(*(u32*)(this+0x1D0)+0x152));` then `for (p = *(u32*)(this+0x184); p; p = *(u32*)(p+0x184)) { p+0xAC/0xB0/0xB4 = this+0xAC/0xB0/0xB4; }` |
| `0x39` | `0x8079C920` | `*(u8*)(*(u32*)0x80834470 + 0x50A) = 1; 0x800F6410(this,&v3{zero,zero,1.0f}); 0x80799960(this,0xB,0xFFFFFFFF);` `0x80104FD0(*(u32*)0x8042A27C, 0x80100B40(*(u32*)(this+0x22C)), *(u8*)(*(u32*)(this+0x1D0)+0x151), *(u8*)(*(u32*)(this+0x1D0)+0x152));` `0x80105060(0.0f, *(u32*)0x8042A27C, 4, same two bytes);` |
| `0x3D` | `0x8079C9A4` | `t=0x80162D50(0x284,0);` if `!t` then `0x800F6410(this,&v3{zero,zero,*(f32*)0x807D00C4}); 0x80799960(this,0xB,0xFFFFFFFF);` else `0x80799960(this,4,0xFFFFFFFF); *(u32*)(this+0x23C)=3; 0x800F69F0(*(f64-ish f32*)0x8031D46C, *(f32*)0x8031D474 * *(f32*)0x8031D3B4, *(f32*)0x8031D478 * *(f32*)0x8031D3B4, this, &v3{t.x - 1500? no: t.x, *(f32*)(t+0xB0)- *(f32*)0x807D018C, *(f32*)(t+0xB4)- *(f32*)0x807D018C}, 0, *(i32*)0x8031D470, 0x80377DE0); 0x800F6410(this,&v3{zero,zero,*(f32*)0x807D0204});` then the two `0x80104FD0`/`0x80105060` calls of block `0x39`. |
| `0x40` | `0x8079CAD0` | `t=0x8079B1D0(); if (t) 0x8079B3F0(this);` |
| `0x41` | `0x8079CB20` | `0x8079F4B0();` |
| `0x42` | `0x8079CAFC` | `0x80775860(&v3, *(u32*)0x8042A18C); 0x8079E030(this, this+0xAC, &v3);` |
| `0x44` | `0x8079CBB8` | `*(u32*)(this+0x288)=3; *(u32*)(this+0x28C)=3; *(u32*)(this+0x238)=*(u32*)(this+0x22C); 0x80797190();` |
| `0x45` | `0x8079CB8C` | `0x800F6410(this,&v3{*(f32*)0x807D00C4, zero, zero});` |
| `0x46` | `0x8079CBAC` | `*(u32*)(this+0x2C0)=5;` |
| `0x49` | `0x8079CBD4` | `0x8079EF20();` |
| `0x4B` | `0x8079CBDC` | `if (0x8079B260() == 0) *(u32*)(this+0x2C0)=0; else *(u32*)(this+0x2C0) = 0x8079F500(this) ? 100 : 0;` |
| `0x4C` | `0x8079CC58` | `0x80104FD0(*(u32*)0x8042A27C,1,two bytes); 0x80105060(0.0f,*(u32*)0x8042A27C,5,two bytes); 0x80100F90(this,0);` then `if (0x800FC810()) name = {'F','k','0', *(u8*)0x8083273C + '0', 0}; else if (0x800FC830()) name = {'k','a','0', *(u8*)0x80832610 + '0', 0}; else name = {'k','a','0','0',0};` `id=0x801002C0(*(u32*)0x8042A18C,&name); if (*(i32*)(this+0x22C)==id) { for (i=0;i<6;i++) if (i != *(i32*)(this+0x23C)) { v=0x80774FA0(*(u32*)0x8042A18C, *(u32*)(this+0x22C), i); if (v >= 0) *(i32*)0x80832EB0 = v; } } else { *(i32*)0x80832EB0 = *(i32*)(this+0x22C); *(i32*)(this+0x22C) = id; }` `0x8079E310(this);` |
| `0x4D` | `0x8079CD90` | `*(u32*)(this+0x2C0) = 0x8079F500() ? 100 : 0;` |
| `0x56` | `0x8079D324` | `0x8079EF00();` |
| `0x57` | `0x8079CDE4` | `*(u32*)(w+0x9C) = *(u32*)(arm+4) & 0xF; if (*(u8*)(this+0x2FC)==0) *(u32*)(this+0x22C)=*(u32*)(this+0x230); 0x8079F500(this);` |
| `0x59` | `0x8079CE10` | `0x8079E550();` |
| `0x5B` | `0x8079CDB4` | `*(i32*)(this+0x2C0) = (mode == 3) ? (i32)*(u16*)0x8031D5B2 : 1;` |
| `0x64` | `0x8079CED8` | `*(i32*)(this+0x2C0) = (i32)*(u16*)0x8031D5B0;` |
| `0x65` | `0x8079CE18` | `this+0x23C=4; this+0x240=4; this+0x288=2; this+0x28C=2; this+0x238=this+0x22C; this+0x230=0x80774FA0(*(u32*)0x8042A18C,this+0x22C,4); 0x80797190(this); this+0x264..0x26C = this+0xAC..0xB4; 0x80100200(&p, *(u32*)0x8042A18C, this+0x230); this+0x1FC..0x204 = p; vcall(this+0x60,0x68,this);` |
| `0x66` | `0x8079D0A4` | `0x80100F90(this,1);` |
| `0x6C` | `0x8079CEAC` | `0x8079EC40(this, 0x80101630(ctx, sel)); vcall(this+0x60,0x68,this);` |
| `0x6E` | `0x8079CEEC` | `0x8079ED20(this, 0x80101630(ctx, sel));` |
| `0x70` | `0x8079CF2C` | `if (0x800FC9F0()) { vcall(*(u32*)(this+0x1D0), 0x5C, 1.0f, *(f32*)0x807D0208, zero, *(u32*)(this+0x1D0), 0); this+0x240=4; this+0x23C=4; 0x80798CC0(this,4); }` |
| `0x72` | `0x8079CF04` | `0x80C56270(*(u32*)(this+0x29C),0x10,0);` |
| `0x73` | `0x8079CF18` | `0x80C56270(*(u32*)(this+0x29C),0x10,1);` |
| `0x7A` | `0x8079CFC0` | `*(u8*)(this+0x124)=1; 0x80795620(); *(u32*)(this+0x240)=2; 0x800FC2D0(&v,2); 0x800F6410(this,&v); 0x800FC2D0(&v,*(u32*)(this+0x240)); 0x800F69F0(*(f32*)0x8031D5F4, *(f32*)0x8031D5FC * *(f32*)0x8031D3B4, *(f32*)0x8031D600 * *(f32*)0x8031D3B4, this, this+0xAC, *(i32*)0x8031D5F0, *(i32*)0x8031D5F8, &v);` |
| `0x7B` | `0x8079CF78` | `0x800FC2D0(&v,*(u32*)(this+0x240)); 0x800F69F0(*(f32*)0x8031D5E0, *(f32*)0x8031D5E8 * *(f32*)0x8031D3B4, *(f32*)0x8031D5EC * *(f32*)0x8031D3B4, this, this+0xAC, *(i32*)0x8031D5DC, *(i32*)0x8031D5E4, &v);` |
| `0x7F` | `0x8079D034` | `t=0x80162D50(0x284,0); this+0xAC/0xB0/0xB4 = t+0xAC/0xB0/0xB4;` |
| `0x82` | `0x8079D068` | `this+0xB8/0xBC/0xC0 = this+0xAC/0xB0/0xB4; n=6; if (0x80101630(ctx,sel)==1) n=3; this+0x240=n; this+0x23C=n;` |
| `0x87` | `0x8079D0B0` | `if (0x800FDF40()) { vcall(this+0x60,0x78,this,4); 0x80104FD0(*(u32*)0x8042A27C,10,two bytes); }` |
| `0x8E` | `0x8079CC34` | `if (0x8079B260()) { 0x80100F90(this,0); 0x8079E550(this); }` |
| `0x8F` | `0x8079CC1C` | `if (0x8079B260()) *(u32*)(this+0x2C0)=0x3C;` |
| `0x91` | `0x8079CAE8` | `*(i32*)(this+0x2C0) = (i32)*(u16*)0x8031D452;` |
| `0x97` | `0x8079D184` | `if (0x800FCB50()==0) vcall(this+0x60,0x68,this); else { c = *(u8*)(w + mode + 0x395); if (c=='*') vcall(this+0x60,0x68,this); else { 0x800F9370(mode, c, &buf); q = 0x801002F0(*(u32*)0x8042A18C); 0x80796180(this, &stack38, q, &buf); vcall(*(u32*)(this+0x29C)+4, 0x38, &out, *(u32*)(this+0x29C), &stack38); this+0x1FC..0x204 = out; 0x800F5A80(&stack38); a+0x604=2; a+0x5F4=this+0x1FC; a+0x5F0=0; a+0x624=0; a+0x608=0; a+0x71C=0x8031D628; this+0x2C0=0x3C; } }` |
| `0x98` | `0x8079D280` | `if (*(u8*)(w+mode+0x395)=='*') vcall(this+0x60,0x68,this); else if (0x800FCB50()==0) vcall(this+0x60,0x68,this); else { a+0x604=7; a+0x5F4=this+0x1FC; a+0x5F0=0; a+0x624=0; a+0x608=0; a+0x71C=0x8031D628; }` |
| `0x9E` | `0x8079D0F4` | `if (0x800FDF40()) { 0x80799960(this,4,0xFFFFFFFF); 0x8079EDD0(this,7); this+0x240=2; 0x800FC2D0(&v,2); 0x800F6410(this,&v); 0x800FC2D0(&v,this+0x240); 0x800F69F0(*(f32*)0x8031D4F8, *(f32*)0x8031D500 * *(f32*)0x8031D3B4, *(f32*)0x8031D504 * *(f32*)0x8031D3B4, this, this+0xAC, 0, *(i32*)0x8031D4FC, &v); }` |
| `0x9F` | `0x8079D304` | `this+0x2C0=0; if (0x800FDF40()) this+0x2C0=0x1E;` |
| `0xA0` | `0x8079D32C` | `*(u32*)(this+0x22C) = 0x801002C0(*(u32*)0x8042A18C, 0x8081B874);` |
| `0xA2` | `0x8079D348` | `0x80799960(this,0xD,0xFFFFFFFF); this+0x2C0=0; if (*(i32*)(this+0x27C)==8 && 0x800FC830()) this+0x2C0=0x1E;` |
| `0xA4` | `0x8079D380` | `v = 0x80101630(ctx, sel); 0x80C56270(*(u32*)(this+0x29C), 1, (v != 0) ? 1 : 0);` (raw `neg`/`or`/`rlwinm` booleanisation) |

The `two bytes` shorthand is
`*(u8*)(*(u32*)(this+0x1D0)+0x151)`, `*(u8*)(*(u32*)(this+0x1D0)+0x152)`.

**Signature.**

```rust
/// Per-state callback of the world-map player actor. Dispatches `state` through
/// the 159-entry table at `0x8081B884`; most states write a position mirror, a
/// camera/sound cell or a lifecycle step, and the terminal states hand off to
/// the neighbouring `0x8079xxxx` helpers.
/// # Original
/// `FUN_8079c330`, `0x8079C330`, 4,232 bytes,
/// `work/ghidra/rel-d_basesNP/export/8079c330_FUN_8079c330.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - actor; `+0x60` class vtable, `+0xAC..+0xB4` position,
///   `+0x1D0` player/input object, `+0x22C`/`+0x230` selection ids,
///   `+0x29C` audio handle, `+0x2C0` timer.
/// * `state` - dispatch index; values outside `6..=164` return immediately.
/// # Confidence
/// medium - the table, every case target and every store are literal; the
/// unresolved item is `0x80101630`, printed with one argument in three arms and
/// two in the others, so its second argument is not fixed by this body.
pub unsafe fn fn_8079c330(this: usize, state: i32)
```

**Constants.** `0x807D0160 = 0.0f`; `0x807D00C4 = 1.0f`;
`0x807D01F8 = 1500.0f`; `0x807D01FC = 20.0f`; `0x807D01A4 = 60.0f`;
`0x807D0200 = 800.0f`; `0x8081B884` = 159-entry dispatch table; `0x807D0080` =
`{1,2,3,...}` character-id table indexed by `mode`; halfword angle seeds
`0x4000`; selection `0x3C`/`0x64`/`0x1E`; the `main.dol` cells
`0x8042D0C0`, `0x8042D0BC`, `0x8042D0B8`, `0x8042D0B4`, `0x8042D0B0`,
`0x8031D3A4`, `0x8031D460`, `0x8031D46C`/`0x474`/`0x478`, `0x8031D3B4`,
`0x8031D470`, `0x8031D5B0`/`0x5B2`, `0x8031D452`, `0x8031D628`, and the
`0x8042A1AC`/`0x8042A284`/`0x80429F7C` owner cells.

**Verdict: conditional (medium).** All 1,058 words, the dispatch table and all
constants are decoded. The one open contract is `0x80101630`: the export prints
`func_0x80101630(_DAT_8042a1ac)` with no second argument in four arms and
`func_0x80101630(_DAT_8042a1ac, uVar11)` in three, so either it is a 1-or-2
argument variadic helper or Ghidra lost `r4` at the one-argument sites. Nothing
in this REL settles it.

---

## 2. `0x8077B450` - 4,076 bytes - `work/ghidra/rel-d_basesNP/export/8077b450_FUN_8077b450.c`

**Body:** `0x8077B450..0x8077C43C` (1,019 words; `blr` at `0x8077C438`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x160`; prologue `bl 0x802DCD68` (`_savegpr` helper), epilogue
`bl 0x802DCDB4` (`_restgpr`). Raw head: `or r28,r3,r3` at `0x8077B46C`
proves **`this` = r3**; `or r29,r4,r4` at `0x8077B470` proves **`route` = r4**
(`cmpi cr0,r4,-1` guards it); `cmpi cr0,r5,0` at `0x8077B480` proves
**`skip` = r5**.

Entry loads: `r31 = 0x807C60F4+?` - raw `addis r31,r0,-32643; addi r31,r31,-2552`
= `0x807D0000 - 0x9F8 = 0x807CF608`? The export types it as a float-pool base;
the two constants used are quoted below. `r30 = *(u32*)0x8042A1AC` (`ctx`).

**Guard and dispatch.** `if (route == -1) return;` then

1. **Phase A** runs only when `flag != 0`: `if ((u32)(route - 10) > 152) goto
   phase B;` else `bctr` through the 153-entry table at `0x808181F8`
   (`addis r4,0,-32638; addi r4,r4,-32264; lwzx; mtctr; bctr` at
   `0x8077B49C..0x8077B4B0`). Every non-default entry was expanded and is
   listed below; all other routes enter phase B.
2. **Phase B** runs always (and is the only phase when `flag == 0`):
   `if (vcall(this+0x60, 0x64)(this) != 0) return;` then the comparison tree
   below.

`ctx = *(u32*)0x8042A1AC`, `actor = *(u32*)0x8042A284` (unused directly),
`world = *(u32*)0x8042A27C`, `player = *(u32*)0x8042A1A0`,
`arm = *(u32*)(ctx+0x1AC)`, `sub = *(u32*)(*(u32*)(this+0x1CC)+4)` (the
`+0x1DD0`-style player sub-object), `mode = *(u8*)0x8042A24D`, `fz = 0.0f`,
`f1 = 1.0f`.

**Phase A - exact behaviour** (state -> block at `0x808181F8` -> action):

| route | block | action |
|---:|---|---|
| `0x0A` | `0x8077B538` | `0x8077C440(this)` |
| `0x0B` | `0x8077B540` | `sel=0x80101630(ctx); r=vcall(this+0x60,0x84)(this,sel); if (r==0) vcall(this+0x60,0x70)(this,0x80101630(ctx),0); else vcall(this+0x60,0x78)(this,0x80101630(ctx));` |
| `0x30` | `0x8077B724` | `t=0x80162D50(0x284,0); if (t) 0x80104FD0(world,3,sub->0x151,sub->0x152); this+0x2D8 = 0x80775080(*(u32*)0x8042A18C, *(u32*)(this+500)); if (*(i32*)(this+0x1AC) < 0) { n = (mode==2) ? 5 : 0x80100970(); this+0x1AC = 0x80103410(*(u32*)0x8042A26C, n, vcall(sub,0x28)(), 0x80100A50(sub->0x151), 0, this+0x2EC); } 0x8077C440(this);` |
| `0x31`,`0x33`,`0x35` | `0x8077B4B4` | `v=0x80100B40(*(u32*)(this+500)); 0x80104FD0(world,v,sub->0x151,sub->0x152); *(u32*)(this+0x2D8)=0x12;` |
| `0x32`,`0x34`,`0x36` | `0x8077B4EC` | `this+0xAC/0xB0/0xB4 = arm+0xAC/0xB0/0xB4; *(u32*)(this+0x2D8)=0x12;` |
| `0x37` | `0x8077B510` | `0x80105060(0.0f, world, 6, sub->0x151, sub->0x152);` |
| `0x39` | `0x8077B5B4` | `0x800F6410(this,&{0,0,1.0f}); 0x8077E410(0.0f, this, 0xB, -1); v=0x80100B40(*(u32*)(this+500)); 0x80104FD0(world,v,sub->0x151,sub->0x152);` |
| `0x3D` | `0x8077B614` | `t=0x80162D50(0x284,0); if (!t) { 0x800F6410(this,&{0,0,1.0f}); 0x8077E410(0.0f,this,0xB,-1); } else { 0x8077CED0(this, &{t.x, t.y-50.0f, t.z-50.0f}, *(u32*)(this+0x294)+0xCC, 0x80377DE0); 0x800F6410(this, &(t.pos - this.pos)); } v=0x80100B40(*(u32*)(this+500)); 0x80104FD0(world,v,sub->0x151,sub->0x152);` |
| `0x40` | `0x8077B70C` | `if (0x80781B40()) 0x8077CFE0(this);` |
| `0x46` | `0x8077BC28` | `*(u32*)(this+0x2C0)=5;` |
| `0x49` | `0x8077B7F0` | `0x8077D680();` |
| `0x4C`,`0x5B` | `0x8077B7F8` | `*(u32*)(this+0x2C0) = (mode==3) ? (i32)*(i16*)(*(u32*)(this+0x294)+0x216) : 1;` |
| `0x56` | `0x8077BC34` | `0x8077D660();` |
| `0x59` | `0x8077B824` | `vec = this.pos; if (0x800FC7F0()==0) 0x8077CD80(this,1,&vec,*(u32*)(this+0x294)+0x1BC, &(player.pos - vec)); else { vec.y += *(f32*)(*(u32*)(this+0x294)+0x20C); 0x80780BD0(&out,this,&vec,(i16)player[0x102]); 0x8077CD80(this,1,&out,*(u32*)(this+0x294)+0x1E4,0x80377DE0); this+0x1E4/0x1E6/0x1E8 = player[0x100/0x102/0x104]; }` |
| `0x64` | `0x8077B954` | `*(i32*)(this+0x2C0) = (i32)*(i16*)(*(u32*)(this+0x294)+0x214);` |
| `0x6C` | `0x8077B928` | `0x8077D580(this, 0x80101630(ctx)); vcall(this+0x60,0x68)(this);` |
| `0x70` | `0x8077B964` | `if (0x800FC9F0()) { *(u8*)(this+0x2F8)=1; p=sub; vcall(p,0x5C)(1.0f, 5.0f, 0.0f, p, 0); }` |
| `0x7A` | `0x8077BA18` | `while (*(u32*)(this+0x220)) 0x80782050(this+0x214); this->y += 10000.0f; 0x80100200(&p, *(u32*)0x8042A18C, player[0x22C]); 0x80780BD0(&out,this,&p,(i16)player[0x102]); this.pos = player.pos; this[0x1E6]=player[0x102]; this[0x200]=player[0x240]; 0x800FC2D0(&m); s=*(u32*)(this+0x294); 0x800F69F0(s[600], s[0x260]*s[0x18], s[0x264]*s[0x18], this, &out, (i16)s[0x254], (i16)s[0x25C], &m);` |
| `0x7B` | `0x8077B9A4` | `while (*(u32*)(this+0x220)) 0x80782050(this+0x214); vec=this.pos; 0x800FC2D0(&m, *(u32*)(this+0x200)); s=*(u32*)(this+0x294); 0x800F69F0(s[0x244], s[0x24C]*s[0x18], s[0x250]*s[0x18], this, &vec, (i16)s[0x240], (i16)s[0x248], &m);` |
| `0x7F` | `0x8077BB00` | `t=0x80162D50(0x284,0); this.pos = t.pos; this+0xDC/0xE0/0xE4 = 0.001f;` |
| `0x82` | `0x8077BB44` | `*(u32*)(this+0x200) = (0x80101630(ctx)==1) ? 3 : 6;` |
| `0x87` | `0x8077BB64` | `if (0x800FDF40()) vcall(this+0x60,0x78)(this,4);` |
| `0x8E` | `0x8077BB8C` | `if (0x8079B260(player)) { 0x80100200(&p,*(u32*)0x8042A18C,player[0x22C]); 0x80780BD0(&out,this,&p,(i16)player[0x102]); this[0x1E4/0x1E6/0x1E8]=player[0x100/0x102/0x104]; 0x8077CD80(this,1,&out,*(u32*)(this+0x294)+0x1E4,0x80377DE0); }` |
| `0x9E` | `0x8077BC3C` | `if (0x800FDF40()) { 0x8077E410(0.0f,this,4,-1); 0x80100200(&p,*(u32*)0x8042A18C,player[0x22C]); 0x80780BD0(&out,this,&p,(i16)player[0x102]); this.pos=player.pos; this[0x200]=player[0x240]; 0x800FC2D0(&m); s=*(u32*)(this+0x294); 0x800F69F0(s[0x15C], s[0x164]*s[0x18], s[0x168]*s[0x18], this, &out, (i16)s[0x158], (i16)s[0x160], &m); }` |
| `0xA2` | `0x8077BD10` | `0x8077E410(0.0f, this, 0xD, -1);` |
| other | `0x8077BD20` | phase B |

**Phase B - exact behaviour.** Gate: `if (vcall(this+0x60,0x64)(this) != 0)
return;` then:

| route | action |
|---:|---|
| `0x56` | `if (0x8077D670(this)) vcall(this+0x60,0x68)(this);` |
| `0x40` | `if (0x80781B40(this)==0) vcall(this+0x60,0x68)(this); else 0x8077D170(this);` |
| `0x30` | `0x8077C670(this);` |
| `0x0A` | `0x8077C670(this);` |
| `0x0B` | `sel=0x80101630(ctx); r = (vcall(this+0x60,0x84)(this,sel)==0) ? vcall(this+0x60,0x74)(this) : vcall(this+0x60,0x7C)(this); if (r != 0) vcall(this+0x60,0x68)(this);` |
| `0x3D` | `if (0x80162D50(0x284,0)==0) *(u8*)(this+0x139)=1; else { 0x8077CF60(this); if (vcall(this+0x60,0x64)(this)!=0) this+0xDC/0xE0/0xE4 = 0.01f; }` |
| `0x46` | timer: `n=*(i32*)(this+0x2C0); if (n<1) { if (n==0) { 0x80104FD0(world,0xB,sub->0x151,sub->0x152); 0x8077E410(0.0f,this,4,-1); 0x8077CED0(this,this+0xAC,*(u32*)(this+0x294)+0x194,0x80377DE0); 0x800F6410(this,&{0,0,1.0f}); *(u32*)(this+0x2C0)=0xFFFFFFFE; } else 0x8077CF60(this); } else *(u32*)(this+0x2C0)=n-1; if (this.y < this+0xBC) 0x8077E410(0.0f,this,2,-1);` |
| `0x47`,`0x48`,`0x49` | `if (0x8077D7B0(this)) vcall(this+0x60,0x68)(this);` |
| `0x4B`,`0x4D` | `vcall(this+0x60,0x68)(this);` |
| `0x4C` | `goto 0x8077C028` (the `0x5B` block below) |
| `0x5B` | `n=*(i32*)(this+0x2C0); if (n<2) { if (n==1) { 0x80104FD0(world,1,sub->0x151,sub->0x152); 0x800FDC80(this); 0x8077E410(0.0f,this,9,-1); 0x80780BD0(&out,this,player+0x1FC,(i16)player[0x102]); 0x8077CED0(this,&out,*(u32*)(this+0x294)+0x1A8,&(player.pos-this.pos)); 0x8077FD50(this); *(i32*)(this+0x2C0)-=1; } else { 0x8077CF60(this); if (vcall(this+0x60,0x64)(this)!=0) 0x8077E410(0.0f,this,0xD,-1); } } else *(i32*)(this+0x2C0)=n-1;` |
| `0x59` | `0x8077CE20(this);` |
| `0x64` | `n=*(i32*)(this+0x2C0); if (n<1) { if (n==0) { vcall(this+0x60,0x70)(this,3,0); *(i32*)(this+0x2C0)-=1; } else if (vcall(this+0x60,0x74)(this)!=0) vcall(this+0x60,0x68)(this); } else *(i32*)(this+0x2C0)=n-1;` |
| `0x7A` | `if (0x800F6AB0(this)) { f=*(u32*)(*(u32*)(this+0x294)+0x18); this+0xDC/0xE0/0xE4=f; 0x8077E570(this); *(u32*)(this+0x200)=6; 0x8077E680(this); vcall(this+0x60,0x68)(this); } if (this.y < this+0xBC) *(u8*)(this+0x1A8)=1;` |
| `0x7B` | `if (0x800F6AB0(this)) { this.y -= 10000.0f; *(u32*)(this+500)=player[0x22C]; vcall(this+0x60,0x68)(this); } if (this.y < this+0xBC) *(u8*)(this+0x1A8)=0;` |
| `0x87` | `if (0x800FDF40(this)==0) vcall(this+0x60,0x68)(this); else if (vcall(this+0x60,0x7C)(this)!=0) { this[0x1E6]=player[0x102]; vcall(this+0x60,0x68)(this); }` |
| `0x8E` | `if (0x8079B260(player)==0) vcall(this+0x60,0x68)(this); else 0x8077CE20(this);` |
| `0x9E` | `if (0x800FDF40(this)==0) vcall(this+0x60,0x68)(this); else if (0x800F6AB0(this)) { 0x8077E570(this); 0x8079EE70(*(u32*)(this+500), this+0xAC, sub->0x151, sub->0x152); this[0x1E6]=player[0x102]; *(u32*)(this+0x200)=6; 0x8077E680(this); vcall(this+0x60,0x68)(this); }` |
| anything else | `*(u8*)(this+0x139) = 1;` (`0x8077C41C`) |

`this+0xBC` in the `this.y < ...` comparisons is the object's floor/limit field,
read as `lfs`; it is a member, not a constant.

**Signature.**

```rust
/// Two-phase per-route update of the world-map route actor. With `flag != 0` it
/// first runs the route's spawn/entry action from the 153-entry table at
/// `0x808181F8`; it then always runs the route's per-frame action behind the
/// `+0x60`/`+0x64` gate.
/// # Original
/// `FUN_8077b450`, `0x8077B450`, 4,076 bytes,
/// `work/ghidra/rel-d_basesNP/export/8077b450_FUN_8077b450.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - route actor; `+0x60` class vtable, `+0x294` route model,
///   `+0x1CC` player holder, `+0xAC..+0xB4` position, `+0x200`/`+0x2C0`
///   sub-state, `+0x220`/`+0x214` pending-effect list.
/// * `route` - route index; `-1` = inactive; phase A covers `10..162`.
/// * `flag` - non-zero enables phase A.
/// # Confidence
/// medium - the table, every case block, all 42 call sites and all eight float
/// constants are literal; the ~20 unclaimed `0x8077xxxx` siblings are invoked
/// as opaque calls whose shapes are given above.
pub unsafe fn fn_8077b450(this: usize, route: i32, flag: i32)
```

**Constants.** `0x808181F8` = 153-entry phase-A table (`route-10`); float pool
`0x807CF698 = 0.0f`, `0x807CF69C = 1.0f`, `0x807CF6A0 = 50.0f`,
`0x807CF6A4 = 5.0f`, `0x807CF6A8 = 10000.0f`, `0x807CF6AC = 0.001f`,
`0x807CF6B0 = 0.01f`; `0x80377DE0` = shared identity/transform argument;
`0x8042A18C`/`0x8042A1A0`/`0x8042A1AC`/`0x8042A24D`/`0x8042A26C`/`0x8042A27C`/
`0x8042A284` = `main.dol` runtime cells; vtable slots `0x5C`, `0x64`, `0x68`,
`0x70`, `0x74`, `0x78`, `0x7C`, `0x84`.

**Verdict: ready (medium).** The body is fully decoded: both dispatch structures,
the 39 distinct action blocks, all call shapes and all constants. "Medium" marks
the unclaimed siblings (`0x8077C670` 924 bytes, `0x8077C440` 556, `0x8077CED0`
144, `0x8077CD80`, `0x8077CF60`, `0x8077D170`, `0x8077D580`, `0x8077D660`,
`0x8077D670`, `0x8077D680`, `0x8077D7B0`, `0x8077E410`, `0x8077E570`,
`0x8077E680`, `0x8077FD50`, `0x80781B40`, `0x80782050`), which are called but
not reconstructed here; every argument register is visible.

---

## 3. `0x8065CB20` - 3,788 bytes - `work/ghidra/rel-d_basesNP/export/8065cb20_FUN_8065cb20.c`

**Body:** `0x8065CB20..0x8065D9E8` per the TSV, but the extent **stops one word
short of the epilogue**: the last TSV word is `mfspr r0,lr` at `0x8065D9E8`, the
`blr` is at `0x8065D9EC`, and the next TSV row starts at `0x8065DA00`. The TSV
size (`3,788`) is quoted verbatim; the real span is 948 words (3,792 bytes).

Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x150`; the first 18 `psq` sites are the `stfd`+`psq_st qr0` spills of
`f23..f31` (nine pairs at `0x8065CB30..0x8065CB88`), then
`bl 0x802DCD40` (`_savegpr`) and, at the end, `bl 0x802DCD8C`
(`_restgpr`). **The export's first 160 lines are that spill**: the whole
`unaff_GQR0`/`ldexpf` ladder (`if ((unaff_GQR0 & 0x3f00) == 0) ...`) is Ghidra
trying to interpret `psq_st` and is not part of the body. The trailing nine
`ldexpf` calls after `LAB` are the matching `psq_l` restores. The TSV row is
therefore *not* GQR-damaged: `qr0` is a plain `f32` pair, `GQR0 = 0`.

**Argument recovery.** `iVar11 = (int)(uVar39 >> 32)` where
`uVar39 = bl 0x802DCD40`: as in every save-helper body, `r3:r4` survive, so
`iVar11` = entry **r3 = `this`**. The export's `param_2`/`param_3` are the
incoming `f1`/`f2` (`x0`, `y0`), `param_4`/`param_5` are `r5`/`r6` (never read),
and `param_6` is **`r7`**, the `float*` output cursor. Return value: the body
leaves the emitted-vertex count in `iVar22` and `r3` at the `_restgpr` call,
so it returns that count in `r3` (the helper preserves `r3`).

**Exact behaviour.** Four phases.

1. **Grid build** (`0x8065CB8C..0x8065D3xx`). `m = (int)(y0 * 0.0625) + 3`;
   `cols = 2*m + 1`; iterate `cols x cols`. For each node:
   * grid indices `gx = (int)(x0 * 0.0625) + i - m`,
     `gy = (int)(y0 * 0.0625) + j - m` (the raw sequence is the
     `0x43300000`-magic unsigned-to-double ladder with the sign bit flipped and
     `- *(f32*)0x807C8AF8` (= `176.0`) applied, then `(int)`, then `sth`);
   * `*(i16*)(node+0x3E4) = gx`, `*(i16*)(node+0x3E6) = gy`;
   * `id0 = 0x800775A0(*(u32*)0x80429DD0, (u16)gx, (u16)gy, 0, 1)`;
     `id1 = 0x800775A0(*(u32*)0x80429DD0, (u16)gx, (u16)gy, 1, 1)`;
     stored at `node+0x1CE4` and `node+0x2964`; `id == 0x1E` is normalised to `0`;
   * `any |= (id0 != 0)`; `node+0x4EEC = node+0x4EED = 0`;
     scan the 43-entry table `0x807E703C` for `entry[0] == id0` and copy
     `entry+4` and `entry+9` **bytes** into `+0x4EEC`/`+0x4EED` (first match
     only); `node+0x4EEE = 0`; scan the 7-entry table `0x807E7240` for
     `entry[0] == id1` and copy byte `entry+4` into `+0x4EEE`;
   * `node+0x35E4 = node+0x4264 = 0x296` (the "no model" sentinel).
2. **Shore flags** (only if `any`; `0x8065D1xx..0x8065D5xx`). Four passes over
   the interior nodes. A node whose `id0` is in `[0x2C0,0x300)` and whose
   neighbour ids are `0` or `0xFF` (water) gets bits OR-ed into the three flag
   bytes at `+0x50CC`, `+0x50CD`, `+0x50CE`; `0x800B2EF0()` (an unclaimed
   clock/random source) is compared with `*(f32*)0x807E7294` (= `0.5`) and
   either `0` or `4` is OR-ed into `+0x50CE`. A fourth pass **and**s bits
   `0xFD`/`0xFB`/`0xF7` out where an `id0` pair is water and `id1` is in
   `[0x280,0x2A0)`.
3. **Join resolution** (`0x8065D5xx..0x8065D73C`). Second `cols x cols` loop.
   For `id0` in `[0x2C0,0x300)`: scan `0x807E703C` for the entry whose byte
   `entry+4`/`entry+9` matches `+0x4EEC`/`+0x4EED`, using `0x800B2EF0()` and
   the running `entry[2..3]` floats between `0.0` and `1.0` to pick the
   sub-entry; write `entry[1]` (the model id) to `+0x35E4`; entries `0x17`,
   `0x19`, `0x26` additionally rewrite `+0x4264` from `+0x2964` (`-0x6A`,
   `+0x16`). For `id0 == 0`/`0xFF` with `id1` in `[0x280,0x2A0)`: scan the
   7-entry `0x807E7240` and write `entry[1]` to `+0x4264`, else `0`.
4. **Emit** (`0x8065D740..0x8065D9E4`, only if `any`). Third `cols x cols`
   loop:
   * `id0 != 0 && +0x35E4 < 0x296` -> `0x80077860(*(u32*)0x80429DD0, gx, gy, 0)`;
   * `id1 != 0 && +0x4264 < 0x296` -> `0x80077860(..., 1)`;
   * `+0x35E4 == 0` -> write a vertex into `*out`:
     `out[0] = gx - 176.0f`, `out[1] = -(gy - 176.0f)`, `out[2] = 0.0f`
     (`0x807C8B10`, `0x807C8AE8`); advance `out += 3`; then if
     `x - 64.0f <= out[0] <= 64.0f + x` and
     `y - 64.0f <= out[1] <= 64.0f + y` (viewport bounds read through
     `0x800E7E20/0x800E7E30/0x800E7E40/0x800E80B0` on `this+0x9C20`,
     `0x807E7034 = 64.0f`), emit
     `0x800925F0(*(u32*)0x80429EE8, &v, ((i+j) & 3) | 0x800, 0xFFFFFFFF)`;
   * count the emitted vertices and return the count.

**Record layout** (all three loops use the same node record; stride `0x50`,
`cols` nodes per row at `+0x50`, next plane at `+0x1E0`):
`+0x3E4` i16 gx, `+0x3E6` i16 gy, `+0x1CE4` u16 id0, `+0x2964` u16 id1,
`+0x35E4` u16 model0, `+0x4264` u16 model1, `+0x4EEC..+0x4EEE` u8 connection
bytes, `+0x50CC..+0x50CE` u8 shore flags.

**Tables.** `0x807E703C`, 43 entries of 6 halfwords
`{tileId, modelId, height:f32, mask}`:

```
2C0 156 3EAE147B 090D | 2C1 157 3EAE147B 0100 | 2C2 158 3F800000 0300
2C3 159 3F800000 0000 | 2C4 15A 3F800000 000E | 2C5 15B 3F800000 0F0E
2C6 15C 3F800000 0F00 | 2C7 15D 3E800000 010E | 2C8 15E 3EAE147B 090D
2C9 15F 3EAE147B 0100 | 2CB 161 3EAE147B 090D | 2CC 162 3EAE147B 0100
2D0 166 3E4CCCCD 080D | 2D1 167 3F800000 0F0F | 2D2 168 3F800000 0200
2D3 169 3F800000 000D | 2D4 16A 3F800000 000F | 2D5 16B 3F800000 0009
2D6 16C 3F800000 0F09 | 2D7 16D 3E800000 010E | 2D8 16E 3E4CCCCD 080D
2D9 16F 3E4CCCCD 080D | 2DB 171 3E4CCCCD 080D | 2E0 176 3F800000 0C0F
2E1 177 3EAE147B 040E | 2E2 178 3F800000 060E | 2E3 179 3F800000 000C
2E4 17A 3F800000 0006 | 2E5 17B 3F800000 0406 | 2E6 17C 3F800000 0F0D
2E7 17D 3E800000 010E | 2E8 17E 3E4CCCCD 080D | 2E9 17F 3EAE147B 040E
2EC 182 3EAE147B 040E | 2F0 186 3E800000 010E | 2F1 187 3F800000 0809
2F2 188 3F800000 0909 | 2F3 189 3F800000 0106 | 2F4 18A 3F800000 0606
2F5 18B 3F800000 010C | 2F6 18C 3F800000 030C | 2F7 18D 3F800000 020D
2F8 18E 3F800000 0F06
```

`0x807E7240`, 7 entries: `280 116 3F000000 0600`, `281 117 3F000000 0600`,
`282 118 3F800000 0E00`, `283 119 3F000000 0400`, `290 126 3F000000 0C00`,
`291 127 3F000000 0C00`, `292 128 3F000000 0400`.

**Signature.**

```rust
/// Builds the world-map terrain quad grid for the rectangle anchored at
/// `(x0, y0)`: resolves each node's two tile ids, classifies shore/water flags,
/// resolves the join model ids and emits the visible vertices into `out`.
/// # Original
/// `FUN_8065cb20`, `0x8065CB20`, 3,788 bytes (TSV; `blr` one word later),
/// `work/ghidra/rel-d_basesNP/export/8065cb20_FUN_8065cb20.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - terrain object; node array at `+0x1CE4` (stride `0x50`),
///   viewport source at `+0x9C20`.
/// * `x0`, `y0` - world rectangle origin in 1/16-tile units (`* 0.0625`).
/// * `_r5`, `_r6` - never read.
/// * `out` - `float*` vertex cursor, three floats per emitted vertex.
/// # Returns
/// Number of vertices written to `out`.
/// # Confidence
/// medium - every phase, table, constant and call shape is literal; the exact
/// unsigned-to-double ladder in phase 1 and the `0x800B2EF0` time source are
/// transcribed by shape, and the nine `psq_st`/`ldexpf` blocks are the
/// `f23..f31` spill, not arithmetic.
pub unsafe fn fn_8065cb20(this: usize, x0: f32, y0: f32, out: *mut f32) -> u32
```

**Constants.** `0x807C8AE8 = 0.0f`; `0x807C8AEC = 0.5f`;
`0x807C8AF0 = 7100.0f`; `0x807C8AF4 = 1.0f`; `0x807C8AF8 = 176.0f`;
`0x807C8B00 = 0.0625f` (1/16); `0x807C8B04 = 8.0f`; `0x807C8B08 = 16.0f`;
`0x807C8B0C = 5500.0f`; `0x807C8B10 = 176.0f`; `0x807E7034 = 64.0f`;
`0x807E7294 = 0.5f`; sentinel `0x296`; tile range `0x2C0..0x300`; water
`0`/`0xFF`; `0x80429DD0`, `0x80429EE8` = `main.dol` managers.

**Verdict: ready (medium).** All four phases, both join tables and every call
shape are decoded. "Medium" is for the compiler-cookie arithmetic in phase 1
and the unclaimed `0x800B2EF0`/`0x800E7E20`/`30`/`40`/`0x800E80B0` viewport
predicates, all called with the registers shown.

---

## 4. `0x80661070` - 3,432 bytes - `work/ghidra/rel-d_basesNP/export/80661070_FUN_80661070.c`

**Body:** `0x80661070..0x80661DD8` (858 words; `blr` at `0x80661DD4`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0xA0`; prologue `stfd`+`psq_st` for `f31`/`f30`/`f29`, then
`bl 0x802DCD5C` (`_savegpr` helper); epilogue `bl 0x802DCDA8` (`_restgpr`).
The three `unaff_GQR0` blocks in the export (`0x80661070..0x8066109C` and the
three `ldexpf` calls before the epilogue) are those `qr0` spills, **not**
arithmetic.

**Argument recovery.** Raw `lwz r0,992(r3)` at `0x8066109C` and
`or r24,r3,r3` at `0x806610A4` prove **`this` = r3**; the export's
`puVar6 = (undefined4*)func_0x802dcd5c()` is that same r3 (Ghidra took the high
half of the fake `r3:r4` pair). Signature is therefore unary.

Entry: `state = *(i32*)(this+0x3E0)` (export `puVar6[0xF8]`);
`timer = *(i32*)(this+0x3D0)` (`puVar6[0xF4]`); `vp = this + 0x9C20`
(`puVar6 + 0x2708`, the camera/viewport object); `mode = *(u8*)0x8042A24D`
(not used); `players = *(u8*)0x80429CA8` (four active-player bits);
`mgr = *(u32*)0x80429F10`, `snd = *(u32*)0x80429D60`, `cam = *(u32*)0x80429C94`,
`inp = *(u32*)0x8042A4A8`, `actor103 = 0x80162D50(0x103,0)` (the player actor),
`actor101 = 0x80162D50(0x101,0)`.

**Exact behaviour** (`switch (state)`, guard `(u32)(state) > 17` -> epilogue;
states 1, 6-9, 0x12 are not in the table and fall through):

| state | action |
|---:|---|
| `0` | `0x800E7E50(6208.0f + (this[0xAC] - 0x800E7E30(vp)), 1.0f, vp); 0x800E7F90(0.0f, 1.0f, vp);` then for each of the four player slots whose bit is set in `*(u8*)0x80429CA8`: if `(p[0x10D4] & 1)` and `!(p[0x10D8] & 2)` then `if (0x8004DB40(p,4)==0) 0x80051E70(p); else { if (!0x80056CF0(p,0x5F)) 0x80056C70(p,0x5F); if (0x80056CF0(p,0x72)) { if (p[0x348]!=1) 0x80051F30(p,1); if (!0x80052220(p,0x92)) 0x80052170(p,0x92); } }`; if **all** active players satisfy `0x8004DB40(p,4)!=0` then `*(u32*)(mgr+0x15C)=2`. Then `0x8065C100(this)`; `a=0x80162D50(0x101,0)`; build `vec = { 0x800E7E40(vp)+0.5*0x800E7E30(vp), 0x800E80B0(vp)-0.5*0x800E7E30(vp) }`; `flag = (vcall(a+0x394,0x28)->vcall(+0x14)(0x80C6EF0C) != 0) && (a[0x778] < 4)`; `0x8006A3F0(&buf,&vec); 0x801995A0(snd, 0x5D6, *(u32*)this, &buf, flag, 0);` then `if (same test && a[0x778] > 3) { a[0x120] &= ~8; 0x80162510(a); 0x800D8A80(*(u32*)0x8042A008); a=0x80162D50(0x103,0); this[0x9C2C]=0x800E7E20(vp); this[0x9C30]=224.0f; this[0x9C34]=0x800E7EE0(vp); this[0x9C38]=a[0xAC] - 0.5*1.8225f*this[0x9C30]; this[0x9C44]=0x800E80D0(vp); this[0x9C48]=-144.0f + this[0xB0] + this[0x9C30]; 0x8005C930(cam); *(u32*)(mgr+0x15C)=2; clear player state 0x5F on all active; 0x8019C250(inp,4); 0x80102220(); this[0x3D0]=-1; this[0x3E0]=2; }` |
| `2` | Count active players into `n` (bits 1,2,4,8 of `*(u8*)0x80429CA8`). If `timer >= 0` (`0x807E73B8 = 0`): `x0 = (n & 1) ? 0.0f : -0.5f`; for each active player `p` with running index `k`: target `x = 24.0f * (0.5f * (float)(((n&1)? -k : k+1)) ) + x0 + a[0xAC]` (the `0x43300000` ladder and `-k`/`k+1` selection are in the raw at `0x806615xx`); if `p[0xAC] == target` then `if (!0x80056CF0(p,0x24)) { 0x80052170(p,0); 0x80056C70(p,0x24); }` else `0x80052080(p,&target,1.0f)` and clear the all-aligned flag. Then `0x806601B0(&timer, 0x807E6E70, &425)` (three-argument timer helper, third arg `0x1A9`); three `0x8065C210` calls against `vp` with the `+0x9C2C..+0x9C48` fields and `0x800E7DE0`/`0x800E7ED0`/`0x800E80C0`; `if ((n>1 ? 0x807E73C0 : 0x807E73BC) == timer) 0x8005CE50(cam,5);` (`both = 475`); if `timer >= 660` and all aligned: `0x800E7DE0(this[0x9C30]); 0x800E7ED0(this[0x9C38]); 0x800E80C0(this[0x9C48]); *(u8*)(*(u32*)0x80429D48+0xBCA)=1; *(u8*)(*(u32*)0x8042A328+0x44C)=1; 0x8001CDB0(this); timer=-1; state=3;` |
| `3` | `if (*(i32*)(*(u32*)0x8042A328+0x444) == 0) { 0x8006A730(0x1B,0); timer=-1; state=4; }` |
| `4` | `if (timer >= 90) { r = 0x8001CE60(this,1); timer=-1; state=0xC; this[0x9CCC] = (r != 0) ? 1 : 0; }` (`0x2733` = `+0x9CCC`) |
| `5` | `if (timer >= 0) { 0x8019BC50(inp,1); timer=-1; state=0xE; }` |
| `0xA` | `if (timer >= 30) { a=0x80162D50(0x103,0); vcall(a+0x60,0xD4)(a,0x80C6F270); timer=-1; state=0xB; }` |
| `0xB` | `a=0x80162D50(0x103,0); if (!(vcall(a+0x394,0x28)->vcall(+0x14)(0x80C6F270) != 0 && a[0x540] < 1)) { timer=-1; state=5; }` |
| `0xC` | `if (timer >= this[0x9CCC]) { a=0x80162D50(0x103,0); vcall(a+0x60,0xD4)(a,0x80C6F230); a[0x120] |= 8; a[0x380]=3; timer=-1; state=0xD; }` |
| `0xD` | `a=0x80162D50(0x103,0);` for each active player: `x = *(f32*)(0x807E6EC0 + 0x530 + 4*k) + a[0xAC]`; if `p[0xAC]==x` then `{ if (0x80056CF0(p,0x24)) { 0x80056CB0(p,0x24); 0x80051FD0(p); } if ((0.0f <= x) != (bool)p[0x348]) 0x80051F30(p); }` else `0x80052080(p,&x,1.0f)` and clear the flag. If all aligned and the actor's `0x80C6F230` action is done (`a[0x540] < 1`): `timer=-1; state=0xA;` |
| `0xE` | `if (timer >= 30) { a=0x80162D50(0x103,0); vcall(a+0x60,0xD4)(a,0x80C6F2B0); timer=-1; state=0xF; }` |
| `0xF` | `a=0x80162D50(0x103,0); if (!(action(0x80C6F2B0) active && a[0x540] < 3)) { vcall(a+0x60,0xD4)(a,0x80C6F2F0); a[0x120] &= ~8; timer=-1; state=0x10; }` |
| `0x10` | `a=0x80162D50(0x103,0); if (!(action(0x80C6F2F0) active && a[0x540] < 7)) { for each active player: 0x80056C70(p,0xC0); vcall(p+0x60,0x434)(p,0x3F,0); timer=-1; state=0x11; }` |
| `0x11` | `for each active player: if (*(u8*)0x807E7408 == 0) { if (0x800B2EF0() < 0.5f) 0x8005E960(p+0xEA4,0x100); } else 0x80052170(p,0xAE);` then `if (timer >= 200) { 0x80102260(8,2,0,5); timer=-1; state=0x13; }` |

The `action(id)` shorthand is
`vcall(*(u32*)(a+0x394) + 0x28, ...)` - the actor's animation/action controller -
followed by `vcall(+0x14)(id)`; the raw expression is
`(**(int**)(*(int*)(a+0x394)+0x28))(a+0x394)` then
`(**(int**)(*pi+0x14))(pi,id)`, and the sign test is
`(int)(-v | v) < 0`, i.e. `v != 0`.

**Signature.**

```rust
/// Scripted intro/demo state machine for the `0x103` player actor. Each state
/// advances a timer at `+0x3D0` and writes the next state to `+0x3E0`; the
/// sequence walks the camera, lines the four players up, plays three actor
/// actions and finally restores control.
/// # Original
/// `FUN_80661070`, `0x80661070`, 3,432 bytes,
/// `work/ghidra/rel-d_basesNP/export/80661070_FUN_80661070.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - demo manager; `+0x3D0` timer, `+0x3E0` state, `+0x9C20` camera,
///   `+0x9C2C..+0x9C48` staged camera vectors, `+0x9CCC` per-run delay.
/// # Confidence
/// medium - every state, timer threshold, action id and camera write is
/// literal; `0x8001CDB0` (state 2 exit) and `0x8001CE60` (state 4 exit) are
/// unclaimed `main.dol` routines, so only their boolean return is used.
pub unsafe fn fn_80661070(this: usize)
```

**Constants.** Timers/states: `90` (`0x807E73CC`, `0x807E73D0 = 0`), `30`
(`0x807E73EC`, `0x807E740C`), `200` (`0x807E7404`), state-2 exit `660`
(`0x807E73C8`), both-actor threshold `475` (`0x807E73BC`/`0x807E73C0`), third
`0x806601B0` argument `425` (`0x807E73E4`). Floats: `0x807C8AE8 = 0.0f`,
`0x807C8AEC = 0.5f`, `0x807C8B24 = 1.8225f`, `0x807C8B34 = 224.0f`,
`0x807C8B38 = -0.5f`, `0x807E73C4 = 24.0f`, `0x807E73DC = 6208.0f`,
`0x807E73E0 = -144.0f`, `0x807E7400 = 1.0f`, `0x807E7388 = 1.0f`. Action ids:
`0x80C6EF0C`, `0x80C6F230`, `0x80C6F270`, `0x80C6F2B0`, `0x80C6F2F0`. Other:
`0x80429CA8` player mask, `0x80429C94` camera manager, `0x80429D48`/`0x8042A328`
scene flags, `0x80429D60` sound manager, `0x8042A4A8` input, `0x8042A008`
effect manager, `0x80429F10` mode manager, `0x807E6EC0` = four-float offset table
(`+0x530` first slot), `0x807E6E70 = 0.0f`.

**Verdict: conditional (medium).** The state graph and every write are decoded.
The condition is the pair `0x8001CDB0` / `0x8001CE60`: both are unclaimed
`main.dol` routines that take `this` (plus `1` for the second) and return a
boolean that selects the next state; nothing in this REL or in
`ledger/functions.csv` fixes their contract beyond that shape.

---

## 5. `0x8079D3C0` - 3,176 bytes - `work/ghidra/rel-d_basesNP/export/8079d3c0_FUN_8079d3c0.c`

**Body:** `0x8079D3C0..0x8079E028` (794 words; `blr` at `0x8079E024`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0xA0`; saves LR, `r31`, `r30`; no save helper, no `psq`.
Raw head: `or r30,r3,r3` at `0x8079D3E4` proves **`this` = r3**;
`addi r0,r4,-6` / `cmpli 156` at `0x8079D3CC..0x8079D3D0` prove
**`state` = r4** and the valid range `6..=162`.

**Dispatch.** `jump = *(u32*)(0x8081BB00 + (state-6)*4)`; the table is in this
REL's `.data` and was expanded. Every state not listed is the default
`0x8079E008` (`this[0x139] = 1`). The silent states `6`, `0xC`, `0x26`, `0x41`,
`0x57` return without touching anything.

`w = *(u32*)0x80429F7C` (world), `a = *(u32*)0x8042A284` (player manager),
`ctx = *(u32*)0x8042A1AC`, `inp = *(u32*)(this+0x1D0)`, `snd = *(u32*)(this+0x29C)`.

**Exact behaviour.**

| state | action |
|---:|---|
| `6`,`0xC`,`0x26`,`0x41`,`0x57` | no-op |
| `0xA` | `0x8006CC00(); 0x8006CD40(this); if (0x800FDC30(this,this+0x264,this+0x1FC,0)) { 0x800F6CB0(this); this+0xAC..0xB4 = this+0x1FC..0x204; 0x80799960(this,0,-1); this[0x139]=1; }` |
| `0xB` | `sel=0x80101630(ctx); r = (vcall(this+0x60,0x84)(this,sel)==0) ? vcall(this+0x60,0x74)(this) : vcall(this+0x60,0x7C)(this); if (r) vcall(this+0x60,0x68)(this);` |
| `0x1A` | `if (0x80160D10(*(f32*)0x8031D45C, this+0xAC, this+0x1FC)) { this[0x124]=0; this[0x139]=1; }` |
| `0x1E` | `0x8006CC00(); 0x8006CD40(this); if (0x800FDC30(this,this+0x264,this+0x1FC,1)) { 0x800F6CB0(this); this[0x124]=0; this[0x139]=1; }` |
| `0x23` | `if (0x80160D10(*(f32*)0x8031D45C, this+0xAC, this+0x1FC)) this[0x139]=1;` |
| `0x24` | `this[0x139]=1;` |
| `0x25` | `this[0x124]=0; vcall(inp,0x4C)(); 0x8006CC00(); 0x8006CD40(this); d=0x800FC1F0(this+0xAC,this+0x1FC); 0x8079AFB0(this,this+0x1FC,d); delta = this.pos - this[0x1FC..0x204]; if (0x801C14C0(&delta) < 5.0f) { this[0x22C]=this[0x230]; this[0x124]=1; if (0x80101090()) vcall(inp,0x48)(); this[0x298]=1.0f; this[0xDC..0xE4]=*(f32*)0x8031D3B4; 0x807987E0(this); 0x80C560B0(snd); 0x80C561E0(snd,0x10,0); 0x80100200(&p,*(u32*)0x8042A18C,this[0x22C]); 0x80C56600(snd, 0x80C56B90(snd,0x400000), &p); this[0x139]=1; }` |
| `0x30` | `0x8006CC00(); 0x8006CD40(this); 0x800F9CD0(&g, this+0xAC); if (200.0f + (float)(u16 at *(u32*)(*0x8042A480)+4) - 176.0f < g[0]) { 0x80799960(this,0,-1); this[0x139]=1; }` |
| `0x39` | `if (0x8079B1C0()) this[0x139]=1;` |
| `0x3D` | `t=0x80162D50(0x284,0); if (!t) { if (0x8079B1C0(this)) this[0x139]=1; } else if (0x800F6AB0(this)) { 0x80770AA0(0); this[0xDC..0xE4]=0.01f; this[0x124]=0; this[0x139]=1; }` |
| `0x40` | `if (!0x8079B1D0()) vcall(this+0x60,0x68)(this); else { 0x8006CC00(); 0x8006CD40(this); 0x8079B300(&buf,this); 0x8079B060(this,&buf,this+0x1FC,this[0x23C]); if (0x8079F320(this,this+0x1FC)) { 0x800FDC80(this); 0x80799960(this,0,-1); *(u32*)(w+0x370)=0; vcall(this+0x60,0x68)(this); } }` |
| `0x42` | `if (0x8079E1E0()) vcall(this+0x60,0x68)(this);` |
| `0x44`,`0x45` | `vcall(this+0x60,0x68)(this);` |
| `0x46` | timer block: `n=this[0x2C0]; if (n<1) { if (n==0) { 0x80104FD0(*(u32*)0x8042A27C,0xB,inp[0x151],inp[0x152]); 0x80799960(this,4,-1); 0x800F69F0(*(f32*)0x8031D534, *(f32*)0x8031D53C * *(f32*)0x8031D3B4, *(f32*)0x8031D540 * *(f32*)0x8031D3B4, this, this+0xAC, 0, *(i32*)0x8031D538, 0x80377DE0); 0x800F6410(this,&{1.0f,0,0}); this[0x2C0]=-2; } else if (0x800F6AB0()) { 0x80103390(*(u32*)0x8042A26C,0,this+0xAC,0,0); if (this[0x2C0]==-2) vcall(this+0x60,0x68)(this); else { this[0x2C0]-=1; 0x800F69F0(...same five...); 0x800F6410(this,&{1.0f,0,0}); } } } else this[0x2C0]=n-1; if (this.y < this[0xBC]) 0x80799960(this,2,-1);` |
| `0x47` | `if (this[0x228]==0) { *(u32*)(w+0x44)=this[0x238]; vcall(this+0x60,0x68)(this); }` |
| `0x49` | `if (0x8079F040()) vcall(this+0x60,0x68)(this);` |
| `0x4B`,`0x4D` | `if (this[0x2C0] < 1) vcall(this+0x60,0x68)(this);` |
| `0x4C` | `0x8079E4E0();` |
| `0x56` | `if (0x8079EF10()) vcall(this+0x60,0x68)(this);` |
| `0x59` | `0x8079E680(); if (vcall(this+0x60,0x64)(this) != 0 && (0x800FC810() || 0x800FC830())) { 0x80799340(this,this+0xAC); 0x8079BE80(this,this[0x23C]); 0x807970A0(this); }` |
| `0x5A` | `this[0x139]=1;` |
| `0x5B` | `n=this[0x2C0]; if (n<2) { if (n==1) { 0x80101230(this,0); 0x8079E310(this); 0x807984F0(this); this[0x2C0]-=1; } else 0x8079E4E0(); } else this[0x2C0]=n-1;` |
| `0x64` | `n=this[0x2C0]; if (n<1) { if (n==0) { vcall(this+0x60,0x70)(this,3,0); this[0x2C0]-=1; } else if (vcall(this+0x60,0x74)(this) != 0) { this[0x23C]=4; this[0x240]=4; this[0xB8..0xC0]=this.pos; vcall(this+0x60,0x68)(this); } } else this[0x2C0]=n-1;` |
| `0x70` | `if (*(u32*)(snd+0x74)==0) vcall(this+0x60,0x68)(this);` |
| `0x7A` | `if (0x800F6AB0()) { 0x80799960(this,0,-1); this[0xDC..0xE4]=*(f32*)0x8031D3B4; this[0x240] = 0x800FC040() ? 2 : 5; this[0xB8..0xC0]=this.pos; vcall(this+0x60,0x68)(this); } if (this.y < this[0xBC]) this[0x1A8]=1;` |
| `0x7B` | `if (0x800F6AB0()) { this[0x124]=0; vcall(this+0x60,0x68)(this); } if (this.y < this[0xBC]) this[0x1A8]=0;` |
| `0x7E` | `0x801C1650(snd+0xC, snd+0x30); if (0x802BE470() < *(f32*)0x807D016C) vcall(this+0x60,0x68)(this);` |
| `0x87` | `if (0x800FDF40()==0) vcall(this+0x60,0x68)(this); else if (vcall(this+0x60,0x7C)(this) != 0) { this[0x102]=0; vcall(this+0x60,0x68)(this); }` |
| `0x8E` | `if (0x8079B260()==0) vcall(this+0x60,0x68)(this); else 0x8079E680(this);` |
| `0x8F` | `if (0x8079B260()==0) vcall(this+0x60,0x68)(this); else if (this[0x2C0]<1) vcall(this+0x60,0x68)(this); else this[0x2C0]-=1;` |
| `0x91` | `if (this[0x2C0]<1) { 0x8079B5B0(); vcall(this+0x60,0x68)(this); } else this[0x2C0]-=1;` |
| `0x97`,`0x98` | `if (a[0x4EC]==a[0x4E0] && a[0x4F0]==a[0x4E4] && a[0x4F4]==a[0x4E8]) { if (this[0x2C0]<1) vcall(this+0x60,0x68)(this); else this[0x2C0]-=1; }` |
| `0x9E` | `if (0x800FDF40()==0) vcall(this+0x60,0x68)(this); else if (0x800F6AB0(this)) { 0x80799960(this,0,-1); this[0x240]=6; this[0xB8..0xC0]=this.pos; this[0x102]=0; 0x8079EE70(this[0x22C],this+0xAC,inp[0x151],inp[0x152]); vcall(this+0x60,0x68)(this); }` |
| `0x9F`,`0xA2` | `if (this[0x2C0]<1) vcall(this+0x60,0x68)(this); else this[0x2C0]-=1;` |
| default | `this[0x139]=1;` |

**Signature.**

```rust
/// Per-state callback of the world-map actor controller. `state` indexes the
/// 157-entry table at `0x8081BB00`; the states either step a timer at `+0x2C0`,
/// hang on a helper predicate, or set the leave flag at `+0x139`.
/// # Original
/// `FUN_8079d3c0`, `0x8079D3C0`, 3,176 bytes,
/// `work/ghidra/rel-d_basesNP/export/8079d3c0_FUN_8079d3c0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - actor; `+0x60` class vtable, `+0x1D0` input object,
///   `+0x1FC..+0x204` destination, `+0x22C`/`+0x230` selection ids,
///   `+0x23C`/`+0x240` sub-state, `+0x29C` audio handle, `+0x2C0` timer.
/// * `state` - `6..=162`; out of range or unlisted sets `+0x139`.
/// # Confidence
/// medium - the table, all 43 cases and every constant are literal; the
/// unclaimed `0x8079xxxx` helpers are called with the shapes shown, and
/// `0x801C14C0` length, `0x80160D10` overlap test and `0x802BE470` timer are
/// named by use only.
pub unsafe fn fn_8079d3c0(this: usize, state: i32)
```

**Constants.** `0x8081BB00` = dispatch table; `0x807D0160 = 0.0f`;
`0x807D016C = 0.01f`; `0x807D0204 = -1.0f`; `0x807D0208 = 5.0f`;
`0x807D020C = 200.0f`; `0x807D0210 = 176.0f`; `0x807D00C4 = 1.0f`;
`0x807D018C = 50.0f`; `0x8031D3B4`, `0x8031D45C`, `0x8031D534`/`0x53C`/`0x540`/
`0x538` = `main.dol` float cells; `0x8042A18C`, `0x8042A1AC`, `0x8042A26C`,
`0x8042A27C`, `0x8042A284`, `0x80429F7C`, `0x8042A480` = `main.dol` runtime
cells; vtable slots `0x48`, `0x4C`, `0x60`, `0x64`, `0x68`, `0x70`, `0x74`,
`0x78`, `0x7C`, `0x84`.

**Verdict: conditional (medium).** The body is decoded case by case.
Conditions: `0x80799340` (604 bytes) and `0x807970A0` (128) are unclaimed
siblings invoked by `0x59`, and the `0x801C14C0`/`0x801C1650`/`0x802BE470`
`main.dol` routines are known only by their argument shape and comparison use.

---

## 6. `0x8076DA30` - 2,656 bytes - `work/ghidra/rel-d_basesNP/export/8076da30_FUN_8076da30.c`

**Body:** `0x8076DA30..0x8076E490` (664 words; `blr` at `0x8076E48C`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0xE0`; saves LR, `f31` (`stfd`+`psq_st qr0`), `r31`, `r30`, `r29`. Raw
head: `or r30,r3,r3` at `0x8076DA54` proves **`this` = r3**; the state is read
from the object, so the signature is unary. The single `unaff_GQR0` block at
`0x8076DA30..0x8076DA40` and the one `ldexpf` before the epilogue are the `f31`
spill.

**Dispatch.** `switch (*(u32*)(this+0x340))`; the case targets come from the
REL jump table at `0x80816AC4` (`state*4`, blocks `0x8076DA80..0x8076E450`),
expanded and matching the export's `0..0xF`. Returns `1` only from state `0xF`
(after `vcall(+0x60,0x68)`), `0` otherwise.

`w = *(u32*)0x8042A27C`, `p = *(u32*)0x8042A1A0` (the player actor),
`mgr = *(u32*)0x8042A26C`, `timer = *(i32*)(this+0x350)`,
`sub = *(i32*)(this+0x354)`, `handle = *(i32*)(this+0x35C)`,
`target = this+0x344..0x34C`.

**Exact behaviour.**

| state | action |
|---:|---|
| `0` | `0x80100200(&v, *(u32*)0x8042A18C, p[0x22C]); this[0x344..0x34C]=v; 0x8076E490(5.0f, 1.0f, 0.0f, this, 3); 0x800F69F0(10.0f, 0.2f*2.5f, 1.0f*2.5f, this, &v, 0, 0x3C, 0x80377DE0); this[0x340]=1; this[0x350]=0x1E;` |
| `1` | `if (--timer == 0) { 0x8075F9E0(1.0f, 0.0f); 0x80105240(w,0x3F,this+0xAC,1); } if (0x800F6AB0(this)) { 0x80105240(w,0x40,this+0xAC,1); 0x80103390(mgr,0xC,this+0xAC,0,0); 0x8076E490(5.0f, 1.0f, 0.0f, this, 4); this[0x340]=2; }` |
| `2` | `if (0x80169020(this+0x2C8)) { this[0x350]=0x2D; 0x8076E490(5.0f, 1.5f, 0.0f, this, 1); this[0x10C]=6.8f; 0x800F6410(this,&{1.0f,0,0}); this[0x35C]=0x80103410(mgr,2,this+0x1A8,"koopaJr_all_root",0,0); this[0x340]=3; }` |
| `3` | `if (timer < 0xB) { if (timer < 1) { 0x8076E490(5.0f,1.0f,0.0f,this,0); this[0x340]=4; vcall(this+0x60,0x68)(this); } else { if (handle >= 0) { 0x801034E0(mgr); handle=-1; } n = (int)((*(f32*)0x807CF258 / (float)<u32 ladder>) * *(f32*)0x8042DCDC); 0x800F64C0(this,(i16)n,1); timer-=1; this[0x10C]=0.0f; } } else { if (sub==1 && (0x801690E0(9.0f,this+0x220) || 0x801690E0(28.0f,this+0x220))) 0x80105240(w,0x41,this+0xAC,1); timer-=1; 0x8006CC00(this); 0x8006CD40(this); 0x800F6CD0(this,this+0x344,&{target.x+200.0f,target.y,target.z},5); }` |
| `4` | `if (handle < 0) handle=0x80103410(mgr,2,this+0x1A8,"koopaJr_all_root",0,0); 0x8006CC00(this); 0x8006CD40(this); 0x800F6CD0(this,this+0x344,&{target.x+200.0f,target.y,target.z},5); 0x801C1650(this+0xAC,this+0x344); if (0x802BE470() > 200.0f) { if (handle>=0) { 0x801034E0(mgr); handle=-1; } vcall(this+0x60,0x68)(this); }` |
| `5` | `0x8076E490(5.0f,1.0f,0.0f,this,0); this[0x10C]=0.0f; this[0x350]=10; this[0x340]=6;` |
| `6` | `if ((u32)(timer-1) < 10) { n=(int)((-180.0f / (float)<u32 ladder>) * *(f32*)0x8042DCDC); 0x800F64C0(this,(i16)n,1); } else if (timer == 0) { 0x800F6410(this,&{-1.0f,0,0}); vcall(this+0x60,0x68)(this); timer+=1; } timer-=1;` |
| `7` | `0x8076E490(5.0f,1.0f,0.0f,this,5); 0x80105240(w,0x42,this+0xAC,1); 0x80105240(w,0x43,this+0xAC,1); this[0x344..0x34C]=this.pos; 0x800F6410(this,&{-1.0f,0,0}); this[0x340]=8;` |
| `8` | `if (0x801690E0(34.0f,this+0x300) || 0x801690E0(46.0f,this+0x300)) { 0x80103390(mgr,0xC,this+0xAC,0,0); 0x80105240(w,0x40,this+0xAC,1); } if (0x80169020(this+0x300)) { this[0x340]=9; this[0x350]=4; 0x8076E490(5.0f,2.0f,0.0f,this,1); }` |
| `9` | `if (timer < 1) { if (handle<0) handle=0x80103410(mgr,2,this+0x1A8,"koopaJr_all_root",0,0); this[0x10C]=6.8f; 0x800F6410(this,&{1.0f,0,0}); 0x8006CC00(this); 0x8006CD40(this); t=0x8006C500(0x284,0); 0x800F6CD0(this,this+0x344,&{t.x,target.y,target.z},5); if (vcall(p+0x60,0x64)() != 0) vcall(this+0x60,0x68)(this); } else { n=(int)((180.0f / (float)<u32 ladder>) * *(f32*)0x8042DCDC); 0x800F64C0(this,(i16)n,1); timer-=1; }` |
| `0xA` | `this[0x10C] = 2.0f + 6.8f; 0x800F6410(this,&{1.0f,0,0}); this[0x340]=0xB; this[0x344..0x34C]=this.pos;` |
| `0xB` | `if (sub==1 && (0x801690E0(9.0f,this+0x220) || 0x801690E0(28.0f,this+0x220))) 0x80105240(w,0x41,this+0xAC,1); 0x8006CC00(this); 0x8006CD40(this); t=0x8006C500(0x284,0); 0x800F6CD0(this,this+0x344,&{t.x,target.y,target.z},5); if (t.x < this.x) { this.x = t.x; this[0x10C]=0.0f; this[0x340]=0xC; if (handle>=0) { 0x801034E0(mgr); handle=-1; } }` |
| `0xC` | `0x8076E490(5.0f,1.0f,0.0f,this,2); this[0x340]=0xD;` |
| `0xD` | `n=(int)((90.0f / this[0x280]) * *(f32*)0x8042DCDC); 0x800F64C0(this,(i16)n,1); if (0x80169020(this+0x258)) { t=0x8006C500(0x284,0); 0x8076E490(5.0f,1.0f,0.0f,this,3); 0x800F69F0(15.0f, 1.0f*2.5f, 0.8f*2.5f, this, &{t.x,t.y,t.z}, 0, 0x3C, &{-1.0f,0,0}); this[0x340]=0xE; 0x80105240(w,0x44,this+0xAC,1); 0x80105240(w,0x45,this+0xAC,1); }` |
| `0xE` | `if (0x800F6AB0()) { 0x80770AA0(0); this[0x340]=0xF; }` |
| `0xF` | `vcall(this+0x60,0x68)(this); return 1;` |

`<u32 ladder>` is the compiler's `0x43300000`/`0x80000000`-based `u32`-to-double
sequence applied to the raw argument of `0x800F64C0`; `*(f32*)0x8042DCDC` is a
`main.dol` scale cell (not in this REL). `0x800F64C0` takes
`(this, (i16)value, 1)`.

**Signature.**

```rust
/// Sixteen-state "Koopa Jr. cutscene" sequence: spawns the `koopaJr_all_root`
/// helper, drives the 200-unit camera dolly, the `-180`/`180` rotations and the
/// final door entry, returning 1 on the exit state.
/// # Original
/// `FUN_8076da30`, `0x8076DA30`, 2,656 bytes,
/// `work/ghidra/rel-d_basesNP/export/8076da30_FUN_8076da30.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - sequence actor; `+0x340` state, `+0x350` timer, `+0x354` sub-state,
///   `+0x35C` helper handle, `+0x344..+0x34C` frame target, `+0x220`/`+0x2C8`/
///   `+0x258`/`+0x300` trigger volumes, `+0x280` angle scale.
/// # Returns
/// `1` on the last state (`0xF`), `0` otherwise.
/// # Confidence
/// medium - all sixteen states, every constant and both sound ids are literal;
/// the `0x80169020`/`0x801690E0` volume tests and `0x8006C500(0x284,0)` lookup
/// are `main.dol` interfaces known by shape.
pub unsafe fn fn_8076da30(this: usize) -> u32
```

**Constants.** Float pool `0x807CF1B8 = 2.5f`, `0x807CF1C0 = 10.0f`,
`0x807CF1C8 = 0.2f`, `0x807CF1CC = 1.0f`, `0x807CF1D0 = 6.8f`,
`0x807CF1DC = 200.0f`, `0x807CF1FC = 15.0f`, `0x807CF204 = 1.0f`,
`0x807CF208 = 0.8f`, `0x807CF22C = 1.0f`, `0x807CF230 = 0.0f`,
`0x807CF248 = 5.0f`, `0x807CF24C = 1.5f`, `0x807CF250 = 9.0f`,
`0x807CF254 = 28.0f`, `0x807CF258 = -180.0f`, `0x807CF25C = -1.0f`,
`0x807CF260 = 34.0f`, `0x807CF264 = 46.0f`, `0x807CF268 = 2.0f`,
`0x807CF26C = 180.0f`, `0x807CF270 = 90.0f`, `0x807CF278 = 176.0f`;
timers `0x1E`/`0x2D`/`10`/`4`; sound ids `0x3F..0x45`; string
`"koopaJr_all_root"` at `0x80816AB0`; `0x80377DE0` shared transform argument;
`0x8042A18C`/`0x8042A1A0`/`0x8042A26C`/`0x8042A27C` = `main.dol` runtime cells.

**Verdict: ready (medium).** Every state, constant, string and call shape is
literal. "Medium" is the `main.dol` scale cell `0x8042DCDC` and the unclaimed
sibling `0x8076E490` (228 bytes, called as `(f32,f32,f32,this,u32)`), not any
doubt about this body.

---

## 7. `0x8078DC70` - 2,576 bytes - `work/ghidra/rel-d_basesNP/export/8078dc70_FUN_8078dc70.c`

**Body:** `0x8078DC70..0x8078E680` (644 words; `blr` at `0x8078E67C`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x30`; prologue `bl 0x802DCD70` (`_savegpr`), epilogue
`bl 0x802DCDBC` (`_restgpr`). Raw head: `or r30,r3,r3` at `0x8078DC88`
proves **`this` = r3**; `or r31,r4,r4` at `0x8078DC8C` proves **`state` = r4**
(`cmpi r4,-1` guards it); `cmpi r5,0` at `0x8078DC94` proves **`flag` = r5**.

`g = DAT_80834470` (the global world-map scene object), `w = *(u32*)0x80429F7C`,
`inp = *(u32*)0x8042A440`, `cam = *(u32*)0x80429C30`,
`ctx = *(u32*)0x8042A1AC`, `mode = *(u8*)0x8042A24D`.

**Exact behaviour.** Two phases, like `fn_8077b450`.

**Phase A** (only if `flag != 0`) - `switch (state)`:

| state | action |
|---:|---|
| `0`,`0x79` | `this[0x1A4] = 0x80101630(ctx);` |
| `8` | `DAT_8042873C=0; 0x80101630(ctx); 0x800B0DB0(); 0x800B0A20(0x3C); 0x80100D90(&v, *(u32*)0x8042A1A0); cam[0x1B4..0x1B8]=v; *(u8*)(cam+0x1C2)=1;` |
| `9` | `DAT_8042873C=0; 0x8078E8D0(this, 0x80101630(ctx)); 0x800B0DB0(); 0x800B0A90(0x3C);` |
| `0x4F` | `if (0x800FC970(7)==0) 0x8078E7C0(this); else vcall(this+0x60,0x68)(this);` |
| `0x50` | `0x8078E6B0();` |
| `0x52` | `if (0x800FC810()==0 && 0x800FC830()==0) this[0x1A4]=0x80101630(ctx); else this[0x1A4]=0;` |
| `0x53` | `n=*(i32*)(w+0x54); if (n==0x15 || n==0x17 || n==0x25) { if (0x8078EB40(this)==0) vcall(this+0x60,0x68)(this); else 0x8078E7C0(this); } else vcall(this+0x60,0x68)(this);` |
| `0x54` | `h=0x800E0380(*(u32*)0x8042A040, -1); 0x800CE360(h, mode, 1);` |
| `0x6D` | `0x807C3B40(g);` |
| `0x6F` | if `*(u8*)0x8042A19C==0 && *(u8*)0x8042A19D==0`: `h=0x800E0380(...); this[0x1B4]=0; if (*(i32*)0x8042A250 < 0x800CE190(h,mode)) { if (0x800CE190(h,mode) < 0x800FC390(mode)) 0x807BAF10(); else { g[0x547]=1; g[0x55C] = (mode==8) ? 0xE : (0x800FC970(7)==0 ? 7 : 8); 0x80105240(w,0x75,this+0xAC,1); 0x8078E680(this); } }` |
| `0x74` | `DAT_8042873C=0; 0x80101630(ctx); 0x800B0DB0(); 0x800B0A20(0x3C); cam[0x1B4]=cam[0x1B8]=*(u32*)0x807CFD78; *(u8*)(cam+0x1C2)=1;` |
| `0x75` | `DAT_8042873C=0; 0x8078E8D0(this,0x80101630(ctx)); 0x800B0DB0(); 0x800B0A90(0x3C);` |
| `0x7C` | `0x8078E680(); g[0x53E]=1; if (g[0x546] || *(u8*)(g[0x534]+0x255)) g[0x545]=1;` |
| `0x80` | `DAT_8081A554=0; g[0x50A]=1;` |
| `0x81` | `DAT_8081A554=1; g[0x50A]=0; 0x807C39D0(g);` |
| `0x84` | `*(u8*)0x8042A19E=1; 0x8078E680(); g[0x548]=1; if (g[0x546] || *(u8*)(g[0x534]+0x255)) g[0x545]=1;` |
| `0x85` | `0x8078E680(); g[0x53F]=1; same two-byte check -> g[0x545]=1;` |
| `0x86` | `0x8076B0A0();` |
| `0x8C` | `0x8078E900(this, 0x80101630(ctx));` |
| `0x8D` | `if (0x800B52F0()==0) 0x8078E910(this, 0x80101630(ctx));` |
| `0x96` | `0x8078E950();` |
| `0x99` | `if (0x800FCB50()==0) this[0x1A4]=0; else this[0x1A4]=0x80101630(ctx);` |
| `0x9C` | `0x8078ECC0();` |
| `0x9D` | `0x8078F060();` |
| `0xA1` | `*(u32*)(w + mode*4 + 0x70) = 0xFFFFFFFF;` |
| `0xA5` | `this[0x1B8]=0x1E;` |
| `0xA6` | `this[0x1A4]=0; if (0x800B52F0()) this[0x1A4]=0x80101630(ctx);` |
| `0xA7` | `this[0x1B8]=0; if (0x8078F1A0()==0) vcall(this+0x60,0x68)(this); else this[0x1B8] = (*(u8*)(w+0x380)==0) ? 1 : 0x3C;` |
| `0xA8` | `0x8078F2D0();` |

**Phase B** (always) - comparison tree:

| state | action |
|---:|---|
| `0x79`,`0x52`,`0x99` | `if (this[0x1A4]<1) this[0x139]=1; else this[0x1A4]-=1;` |
| `0` | same countdown |
| `9` | `if (countLeadingZeros(vcall(inp,0x10)()) >> 5 != 0) { DAT_8042873C=1; this[0x139]=1; }` |
| `8`,`0x74` | `if (countLeadingZeros(vcall(inp,0x10)()-1) >> 5 != 0) { DAT_8042873C=1; this[0x139]=1; }` |
| `0x50` | `if (vcall(this+0x60,0x64)==0) 0x8078E6D0(this);` |
| `0x4F`,`0x54..0x5B` | `if (vcall(this+0x60,0x64)==0) 0x8078E7E0(this);` |
| `0x5C` | `if (0x80100900()==0) vcall(this+0x60,0x68)(this);` |
| `0x6F` | `if (0x8042A19C==0 && 0x8042A19D==0) { if (g[0x547]==0) { 0x8078E690(this); vcall(this+0x60,0x68)(this); } } else vcall(this+0x60,0x68)(this);` |
| `0x7C` | `if (g[0x53E]==0) { 0x8078E690(this); vcall(this+0x60,0x68)(this); }` |
| `0x85` | `if (g[0x53F]==0) { same }` |
| `0x84` | `if (g[0x548]==0) { *(u8*)0x8042A19E=0; same }` |
| `0x96` | `if (vcall(this+0x60,0x64)==0) 0x8078E9A0(this);` |
| `0x9C` | `if (0x8078ED10(this)) vcall(this+0x60,0x68)(this);` |
| `0x9D` | `if (0x8078F0E0(this)) vcall(this+0x60,0x68)(this);` |
| `0xA5` | `n=this[0x1B8]; if (n<2) { if (n<1) { if (vcall(this+0x60,0x64)==0) 0x8078E9A0(this); } else { this[0x1B8]=n-1; 0x8078E950(this); } } else this[0x1B8]=n-1;` |
| `0xA7` | `n=this[0x1B8]; if (n<2) { if (n<1) { if (vcall(this+0x60,0x64)==0) 0x8078F1F0(this); } else { this[0x1B8]=n-1; 0x8078F1D0(this); } } else this[0x1B8]=n-1;` |
| `0xA8` | `0x8078F350(this);` |
| anything else | `this[0x139]=1;` |

**Signature.**

```rust
/// Two-phase callback of the world-map scene controller. Phase A (`flag != 0`)
/// installs or tears down the state's screen/mode; phase B runs the per-frame
/// predicate for the same state and finally lowers the leave flag `+0x139`.
/// # Original
/// `FUN_8078dc70`, `0x8078DC70`, 2,576 bytes,
/// `work/ghidra/rel-d_basesNP/export/8078dc70_FUN_8078dc70.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - scene controller; `+0x60` vtable, `+0x1A4` action countdown,
///   `+0x1B4`/`+0x1B8` short timers, `+0x139` leave flag.
/// * `state` - state id, `-1` = inactive.
/// * `flag` - non-zero runs phase A.
/// # Confidence
/// medium - every state, cell and call shape is literal; `0x807C3B40` (20) and
/// `0x807C39D0` (52) are import thunks and the `g` object at `0x80834470` is a
/// BSS scene object whose later fields are written by these states.
pub unsafe fn fn_8078dc70(this: usize, state: i32, flag: i32)
```

**Constants.** `0x80834470` = scene object (`+0x50A`, `+0x534`, `+0x53E`,
`+0x53F`, `+0x545..+0x548`, `+0x55C`); `0x807CFD78` = a two-word vector pool
cell; `0x8081A554` = global byte flag; `0x8042A19C`/`0x9D`/`0x9E`,
`0x8042A040`, `0x8042A1A0`, `0x8042A1AC`, `0x8042A250`, `0x8042A27C`,
`0x8042A440`, `0x80429C30`, `0x80429F7C`, `0x8042873C` = `main.dol` cells;
`0x800B0A20`/`0x800B0A90` timers `0x3C`; sound ids `0x75`.

**Verdict: conditional (medium).** The state graph is decoded in both phases.
Conditions: `0x8078F1A0`/`0x8078F0E0`/`0x8078ED10`/`0x8078F350` are unclaimed
siblings of this bucket, and `0x807C3B40` is an import thunk whose target lives
outside this REL.

---

## 8. `0x8076EF80` - 2,544 bytes - `work/ghidra/rel-d_basesNP/export/8076ef80_FUN_8076ef80.c`

**Body:** `0x8076EF80..0x8076F970` (636 words; `blr` at `0x8076F96C`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0xA0`; prologue `bl 0x802DCD70` (`_savegpr`), epilogue
`bl 0x802DCDBC`. Raw head: `or r27,r3,r3` at `0x8076EFA0` proves **`this` =
r3**; `or r28,r4,r4` at `0x8076EFA4` proves **`state` = r4**
(`cmpi r4,-1` guards); `cmpi r5,0` at `0x8076EFB8` proves **`flag` = r5**.

`g = *(u32*)0x8042A284` (export `iVar8`), `ctx = *(u32*)0x8042A1AC`.

**Phase A** (only if `flag != 0`):

1. Lazy-init two global float tables, each guarded by a byte flag:
   * `*(u8*)0x808327FC` -> fill `0x80832800..0x80832874` with **ten**
     `{x, y, z}` triples: `{-500,200,400}`, `{-500,100,50}`, `{0,200,1500}`,
     then `{-500,100,50}` seven times.
   * `*(u8*)0x80832878` -> fill `0x80832880..0x808328F4` with ten triples:
     `{-200,200,250}`, `{-180,100,-50}`, `{-240,200,-100}`, then
     `{-180,100,-50}` seven times.
2. `switch (state)`:
   * `0x28` - `t=0x8006C500(0x280,0); this[0x1E4]=t;` place `this+0x384..0x38C`
     at `t.pos + table[mode]` (the `0x80832800` table when
     `*(u8*)0x80816C0E==0`, else the explicit `0x80816C1C/20/24` offsets);
     `if (0x8076FE40(this)==0) { set this[0xAC..0xB4] from the same table,
     this[0xAC..0xB4] -= table[mode]; this[0x102]=this[0x108]=0x801611C0(dx,dz); }
     else 0x8076FE70(this);` then `this[0x10C]=4.0f; this[0x114]=this[0x118]=0.0f;
     x=0x80789B10(mode); y=0x80789A80(x,0,0); *(u8*)(y+0x124)=0; this[0x3C8]=0;`
   * `0x29` - `this[0x3D4] = (0x800FC040()==0) ? 1 : 0;
     if (0x80101630(ctx) > 0) 0x80770830(this);`
   * `0x2B` - `t=0x8006C500(0x272,0); this[0x1E0]=t;` if `t==0`:
     `if (0x800FC040()==0) { this[0x38C] -= 1000.0f; this[0x392]=0x80160E80(this+0xAC,this+0x384); this[0x398]=0; }
     else { this[0x384] += 1000.0f; same; }` else:
     `this[0x384..0x38C]=t.pos; if (t.y + 300.0f > this[0x2C]+100.0f) this[0xEC]=3.0f;
     else if (t.y + 300.0f < this[0x2C]-100.0f) this[0xEC]=-3.0f;
     this[0x392]=0x80160E80(this+0xAC,this+0x384); this[0x398]=0;`
   * `0x2E` - `this[0x1E0]=0x8006C500(0x272,0); 0x8076FBA0(this,(i32)*(u16*)0x80816C5C);
     0x8076FB00(this,-1);`
   * `0x2F` - `this[0x498]=(i32)*(u16*)0x80816C5E; this[0x49C]=0x78; this[0x478]=1;`

**Phase B** (always) - `switch (state)` again:

| state | action |
|---:|---|
| `0x28` | `*(u8*)(this+0x139)=1;` |
| `0x29` | `if ((0x80101630(ctx)>0 && 0x80770840(this)) \|\| 0x80101630(ctx)==0) { if (0x8076FE40(this)==0) { 0x8006CC00(this); 0x8006CD40(this); } else { 0x8076FEF0(&buf,this); if (0x807700F0(this,&buf)) { t=0x8006C500(0x800FD180(mode),0); t.pos = this[0xAC..0xB4]; *(u8*)(this+0x139)=1; } } }` then the camera-trigger block: `if (this[0x3C8]==0) { 0x800F9CD0(&v,this+0xAC); 0x800F9D30(&v); if (this[0x3D4]==0 ? v.x > 0.0f : v.y > 0.0f) { copy this[0x1E4] or this[0x1E8] position into this[0x3BC..0x3C4]; g[0x604]=2; g[0x5F4]=&this[0x3BC]; g[0x5F0]=this[0]; g[0x624]=0; g[0x608]=0; this[0x3C8]=1; } }` then `if (0x8076FE40(this)==0 && 0x802BE470((dx*dx)+(dz*dz)) < 10.0f) { t=0x8006C500(0x800FD180(mode),0); t.pos=this[0xAC..0xB4]; *(u8*)(this+0x139)=1; }` |
| `0x2B` | three-step sub-state `this[0x398]`: `0`: `if (0x8076FE40(this)==0) { 0x8015F200(this+0x102, this[0x392], 0xF, 0x800, 0x80); this[0x108]=this[0x102]; if (this[0x102]==this[0x392]) this[0x398]=1; } else this[0x398]=1;` `1`: `if (0x8076FE40(this)==0) { 0x8006CC00; 0x8006CD40; 0x8015F140(6.0f,0.05f,0.2f,0.02f,this+0x10C); 0x8015F140(0.5f,0.05f,0.1f,0.01f,this+0xEC); 0x800F9CD0(&v,this+0xAC); if (v.x <= 200.0f + (u16 at *(u32*)(*0x8042A480)+4) - 176.0f) { if (v.y < -200.0f) { g[0x604]=7; g[0x5F4]=&this[0x3A4]; ... this[0x398]=2; } } else { same camera install; this[0x398]=2; } } else { 0x8076FF10(&buf,this); if (0x807700F0(this,&buf)) { same camera install; this[0x398]=2; } }` `2`: `this[0x124]=0; n=0x8076E660(mode==7 ? 3 : 1); if (n) { *(u8*)(n+0x124)=1; DAT_808327EC=n+0xAC; DAT_808327E8 &= 0xFFFE; } *(u8*)(this+0x139)=1;` |
| `0x2E` | falls to the default leave (`*(u8*)(this+0x139)=1`) |
| `0x2F` | `if (this[0x478]==0xC) vcall(this+0x60,0x68)(this);` |
| anything else | `*(u8*)(this+0x139)=1;` |

**Signature.**

```rust
/// Two-phase "Koopa Jr. airship boarding" state table: `flag != 0` installs the
/// state's actor/camera, then the per-frame half drives the exit trigger and
/// the three-step walking sub-machine.
/// # Original
/// `FUN_8076ef80`, `0x8076EF80`, 2,544 bytes,
/// `work/ghidra/rel-d_basesNP/export/8076ef80_FUN_8076ef80.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - actor; `+0x1E4`/`+0x1E8` spawned references, `+0x384` target,
///   `+0x3BC` camera anchor, `+0x3C8` camera-armed byte, `+0x398` sub-state,
///   `+0x139` leave byte, `+0x60` vtable.
/// * `state` - one of `0x28`, `0x29`, `0x2B`, `0x2E`, `0x2F`; anything else
///   leaves immediately.
/// * `flag` - non-zero runs the install half.
/// # Confidence
/// medium - every branch, table and constant is literal; `0x8076FE40`,
/// `0x8076FE70`, `0x8076FEF0`, `0x807700F0`, `0x8076FBA0`, `0x8076FB00`,
/// `0x8076FF10`, `0x8076E660`, `0x80789A80`, `0x80789B10` are unclaimed
/// siblings called with the shapes above.
pub unsafe fn fn_8076ef80(this: usize, state: i32, flag: i32)
```

**Constants.** `0x80832800`/`0x80832880` = two 10-triple offset tables;
`0x807CF2A0 = 200.0f`, `0x807CF2A4 = 0.0f`, `0x807CF2AC = -500.0f`,
`0x807CF2B0 = 400.0f`, `0x807CF2B4 = 100.0f`, `0x807CF2B8 = 50.0f`,
`0x807CF2BC = 1500.0f`, `0x807CF2C0 = -200.0f`, `0x807CF2C4 = 250.0f`,
`0x807CF2C8 = -180.0f`, `0x807CF2CC = -50.0f`, `0x807CF2D0 = -240.0f`,
`0x807CF2D4 = -100.0f`, `0x807CF2D8 = 4.0f`, `0x807CF2DC = 300.0f`,
`0x807CF2E0 = 3.0f`, `0x807CF2E4 = -3.0f`, `0x807CF2E8 = 1000.0f`,
`0x807CF2EC = 10.0f`, `0x807CF2F0 = 6.0f`, `0x807CF2F4 = 0.05f`,
`0x807CF2F8 = 0.2f`, `0x807CF2FC = 0.02f`, `0x807CF300 = 0.5f`,
`0x807CF304 = 0.1f`, `0x807CF308 = 0.01f`, `0x807CF310 = 176.0f`;
`0x80816C0E`/`0x80816C1C..24`/`0x80816C10..18`/`0x80816C5C`/`0x80816C5E` =
`main.dol` scene cells; `0x808327E8`/`0x808327EC` globals; `0x8042A480`,
`0x8042A1AC`, `0x8042A24D`, `0x8042A284` runtime cells.

**Verdict: conditional (medium).** The five states and both phases are decoded.
Conditions: the install half and the `0x29`/`0x2B` predicates call eight
unclaimed `0x8076Fxxx` siblings, and the `0x80816Cxx` positions are `main.dol`
scene state, cited by address.

---

## 9. `0x807108B0` - 2,468 bytes - `work/ghidra/rel-d_basesNP/export/807108b0_FUN_807108b0.c`

**Body:** `0x807108B0..0x80711254` (617 words; `blr` at `0x80711250`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x160`; prologue spills `f31..f27` (five `stfd`+`psq_st qr0` pairs at
`0x807108C0..0x807108E4`) then `bl 0x802DCD40` (`_savegpr`); epilogue
`bl 0x802DCD8C`. Raw `lwz r12,1412(r3)` at `0x807108EC` proves **`this` = r3**;
the export's `iVar9 = func_0x802dcd40()` is that r3. Signature is unary.
The five `unaff_GQR0` blocks and five `ldexpf` calls are the `f27..f31` spill.

**Exact behaviour.**

```
if (this[0x688] & 4) {                       // "inactive" arm
    0x8007F950(this + 0x5B0);
    vcall(this+0x60, 0xD4)(this, &DAT_8082F808);
    return;
}
t = 0x806601B0(this + 0x1614, &DAT_80805E38, &local_cap);
this[0x1610] = (f32)t;
0x8006CED0(&mat, this);
vcall(this+0x7C0, 0xB0)(this+0x7C0, &mat, 0, 0);     // skinned-model update
if (this[0x1614] < local_cap) {
    if (this[0x1614] % 3 == 0) {                      // every third step
        r = 0x800B2F50() * *(f32*)0x808063C4;
        this[0x6A4] = r; this[0x6A8] = 0x800B2F50() * *(f32*)0x808063C4;
        this[0x6AC] = *(f32*)0x807CCA7C;
    }
    this[0xAC..0xB4] += this[0x6A4..0x6AC];
}
if (*(u32*)0x8082F76C == 3 || *(u32*)0x8082F76C == 0) {
    vcall(this+0x7C0, 0x10)(this+0x7C0);
    vcall(this+0x60, 0xD4)(this, &DAT_8082F888);
} else if (local_cap <= this[0x1614]) {
    // retire all pending spawn slots
    for (i = 0; i < this[0x9FC]; i++) { this[0x15DC + i*4] = 0; vcall(this+0xA00+i*0x114, 0x10)(); }
    this[0x9FC] = 0;
    // pick a spawn group from DAT_808063EC[mode] / DAT_80805EA0 rows
    ...
    // for each enemy id `sVar1` in the chosen row:
    //   bounds from the 10-entry table 0x808065A0 ({id, x0, y0, x1, y1} as
    //   two halfword pairs); viewport rect is
    //   x: [DAT_8082F760 + DAT_808063C8, DAT_8082F760 + DAT_808063D0]
    //   y: [DAT_8082F764 + DAT_808063CC, DAT_8082F764 + DAT_808063D4]
    //   obj = 0x80064610(sVar1, flags, &pos, 0, 0)
    //   flags = 2 (0x51), 1 (0x36/0x15C), 0x8000008 (0x3C), else 0
    //   vcall(this+0xA00+slot*0x114, 0xA0)(emitter,
    //        sVar1 == 0x51 ? "Wm_ka_enemychange_l" : "Wm_ka_enemychange_s",
    //        0, &obj->pos, 0, 0)
    //   this[0x1608] = -1; slot++
    //   if (sVar1 == 0x85 && obj[0x5EC] < 0) {
    //        obj[0x5EC] = *(u32*)0x80B2B878;
    //        0x801662F0(0.0f, obj+0x5C0, 0);
    //        vcall(obj+0x60, 0xD4)(obj, 0x80B6FB68); }
    // coin pass: if row[3] > 0 and the rect test passes, spawn id 0x191 with
    //   flags 0x10*i | 0xC and string "Wm_ka_coin" into further slots.
    0x8007F950(this + 0x5B0);
    0x8006CED0(&mat, this);
    0x8006A4B0(&v, &mat);                       // documented main.dol vector copy
    0x80198C30(*(u32*)0x80429D60, 0x591, &v, 0);
    vcall(this+0x60, 0xD4)(this, &DAT_8082F848);
}
```

The `0x43300000`-magic ladders convert the halfword/word bounds to `f32`; the
rect test is four comparisons of `pos +/- scale*bound` against the two viewport
pairs. The table `0x808065A0` has ten 6-halfword entries
`{id, ?, x0f32, y0f32, ...}`; the export indexes it by `sVar1` (enemy id).

**Signature.**

```rust
/// Per-frame world-map spawner: advances the skinned actor at `+0x7C0`,
/// retires expired spawn slots and, when the viewport rectangle permits,
/// spawns the enemy row for the current map mode plus its coin burst.
/// # Original
/// `FUN_807108b0`, `0x807108B0`, 2,468 bytes,
/// `work/ghidra/rel-d_basesNP/export/807108b0_FUN_807108b0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - world-map actor; `+0x688` inactive bits, `+0x7B0` model state,
///   `+0x7C0` skinned model, `+0x1614` spawn timer, `+0x9FC` slot count,
///   `+0xA00` emitter slots (stride `0x114`), `+0x15DC` handle array,
///   `+0x6A4..0x6AC` per-step offset, `+0xAC..0xB4` position.
/// # Confidence
/// medium - the control flow, viewport rectangle, enemy ids, spawn flags and
/// both emitter strings are literal; `0x806601B0`, `0x8065E440` are unclaimed
/// siblings and `0x800B2F50`/`0x800B2ED0` are unclaimed `main.dol` randoms.
pub unsafe fn fn_807108b0(this: usize)
```

**Constants.** `0x808063C0` = spawn cap, `0x808063C4` = step scale,
`0x808063C8/CC/D0/D4` = viewport margins, `0x808063EC` = per-mode row base
(stride `0x90`), `0x80805EA0` = enemy row records, `0x80805E38/3C/50/64` =
timer/emitter strings (`"Wm_ka_enemychange_l"`, `"Wm_ka_enemychange_s"`,
`"Wm_ka_coin"`), `0x808065A0` = 10-entry bounds table, `0x8082F760/764/76C` =
camera rectangle and mode, `0x8082F808/848/888` = animation names,
`0x80B2B878` = sentinel, `0x80B6FB68` = animation id; float pool
`0x807CCA7C = 0.0f`, `0x807CCA84 = 16.0f`, `0x807CCA88 = 0.5f`,
`0x807CCA90 = 176.0f`, `0x807CCAA0 = 5500.0f`, `0x807CCAA8`/`0x807CCAAC` =
integer `1`, `0x807CCAB0 = 500.0f`; `0x80429D60` = sound manager.

**Verdict: ready (medium).** The two arms, the slot ring and both spawn passes
are fully decoded. "Medium" is the four unclaimed helpers and the two
`0x8080xxxx` `main.dol` tables, all cited by address with their strides.

---

## 10. `0x8065F910` - 2,204 bytes - `work/ghidra/rel-d_basesNP/export/8065f910_FUN_8065f910.c`

**Body:** `0x8065F910..0x806601AC` (551 words; `blr` at `0x806601A8`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0xB0`; spills `f31`/`f30` (`stfd`+`psq_st qr0`), `bl 0x802DCD68`
(`_savegpr` helper), epilogue `bl 0x802DCDB4`. Raw `or r26,r3,r3` at `0x8065F940`
proves **`this` = r3**; unary. The two `unaff_GQR0` blocks are the `f31`/`f30`
spill.

`this` fields: `+0x3D0` timer, `+0x3E0` state, `+0x9C20` camera,
`+0x9C2C..+0x9C48` staged camera vectors, `+0x9AF8` emitter,
`+0xAC..0xB4` position, `+0x10C` speed, `+0x778` actor height.
`players = *(u8*)0x80429CA8`, `p102 = 0x80162D50(0x102,0)`,
`p101 = 0x80162D50(0x101,0)`, `snd = *(u32*)0x80429D60`.

**Exact behaviour.** Common head: `0x8065C100(); 0x8065C260(this);` then
`switch (state)`:

| state | action |
|---:|---|
| `0` | `n` = number of set bits in `players`. If `this[0x3D0] == (n>1 ? 475 : 475)` (`0x807E72F4`/`F8`, both `475`): if `*(u8*)0x807E72F1` then for each active player `0x80056CB0(p,0x24)`; `0x8005CE50(0x80429C94, 2)`. If `this[0x3D0] >= 30` (`0x807E72FC`) or the frame counter is negative: `0x8006A730(0x1A,0); p102[0x120] |= 8; p102[0x380]=3; if (0x8065DCA0(this)==0) { timer=-1; state=1; } else { vcall(p102+0x60,0xD4)(p102,0x80C6F11C); timer=-1; state=2; }` |
| `1` | `p102 = 0x80162D50(0x102,0)`; if `*(u8*)0x807E7300`: line the four players up at `x = offset[k] + p102[0xAC]` (`0x807E6EC0 + 0x444 + 4k`); if not aligned `0x80052080(p,&x,*(f32*)0x807E7314)`; when aligned and `timer >= *(i32*)0x807E7318`: `vcall(p102+0x60,0xD4)(p102,0x80C6F11C); 0x8019C250(0x8042A4A8,5); timer=-1; state=2;` |
| `2` | `p102=0x80162D50(0x102,0); p101=0x80162D50(0x101,0);` if `p102[0x778] > 0.0f`: emit `"Wm_ob_witchcraftup"` (`0x807E6E4C`) at `{this.x+0x807E7324, this.y+0x807E7328, 0.0f}` through `vcall(this+0x9AF8,0xA0)`; set `vcall(+0x74)(&-{0x807E7338,33C,340},0,3)` and `vcall(+0x2C)(0x807E7344, emitter, 3)`. If `timer == 20` (`0x807E7320`): `0x800D8960(0x8042A008,5,1,0)`; build a vector from `0x800B30E0()`/`0x800B30C0()`; `0x80198C30(snd,0x5D4,&v,0)`. `done = action(0x80C6F11C) && p102[0x540]==0xE`; if `done` or frame<0: `0x8019BC50(0x8042A4A8,1); this[0x9CC4]=0.0f; p102[0x120] &= ~8;` copy `this.pos` to `p101.pos`; fill `p101[0x758..0x768]` from the `0x807E7378..0x7384` rectangle; `vcall(p101+0x60,0xD4)(p101,0x80C6EE4C); p101[0x120]|=8; p101[0x380]=2;` then stage `this[0x9C2C..0x9C48]` from `0x800E7E20/0x800E7EE0/0x800E7E30/0x800E80D0` and the constants `0x807C8B20`/`0x807C8B24`/`0x807E7350`; `0x8005B5D0(0x80429C94,1); *(u8*)(*(u32*)0x8042A070+0xC)=0; timer=-1; state=6;` |
| `6` | if `timer == *(i32*)0x807E7348`: `vcall(0x80162D50(0x102,0)+0x60,0xD4)(..,0x80C6F15C)`; if `timer == *(i32*)0x807E734C`: `0x80162D50(0x21,0); 0x80162510();` then `t=0x806601B0(&timer,0x807E6E48,&cap)` (cap 0x3C), three `0x8065C210` calls against the camera staging fields and `0x800E7DE0/ED0/E80C0`; if `timer >= cap` and `!(action(0x80C6EE4C) && p101[0x778] < 1)` or the frame is negative: `0x800D8A80(0x8042A008); stage the three camera fields; if (frame<0) { 0x80162D50(0x102,0); 0x80162510(); } vcall(this+0x394,0x18)(this+0x394, 0x80828E5C);` |
| other | return |

**Signature.**

```rust
/// Four-state "airship departure" sequence for the world-map camera and the
/// `0x101`/`0x102` actors: it lines the players up, spawns the
/// `Wm_ob_witchcraftup` emitter, stages the camera and hands control back.
/// # Original
/// `FUN_8065f910`, `0x8065F910`, 2,204 bytes,
/// `work/ghidra/rel-d_basesNP/export/8065f910_FUN_8065f910.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - sequence actor; `+0x3D0` timer, `+0x3E0` state, `+0x9C20`
///   camera, `+0x9AF8` emitter, `+0xAC..0xB4` position.
/// # Confidence
/// medium - every state, threshold and emitter string is literal; `0x8065DCA0`
/// (8) and `0x806601B0` (260) are unclaimed siblings and `0x80C6xxxx` are
/// action ids.
pub unsafe fn fn_8065f910(this: usize)
```

**Constants.** `0x807E72F1` byte flag, `0x807E72F4`/`0x807E72F8 = 475`,
`0x807E72FC = 30`, `0x807E7300` byte, `0x807E7314` float, `0x807E7318`,
`0x807E7320 = 20`, `0x807E7324..0x7344` offsets, `0x807E7348`/`0x734C`,
`0x807E7350`, `0x807E7378..0x7384` rectangle, `0x807C8AE8 = 0.0f`,
`0x807C8B0C`, `0x807C8B20`, `0x807C8B24 = 1.8225f`; strings
`"Wm_ob_witchcraftup"` (`0x807E6E4C`), `0x80828E5C`; action ids `0x80C6EE4C`,
`0x80C6F11C`, `0x80C6F15C`; sound ids `0x5D4`, `0x1A`.

**Verdict: ready (medium).** All four states are decoded end to end; "medium"
marks the unclaimed siblings and the `main.dol` `0x807E73xx` table, all cited.

---

## 11. `0x8076C900` - 2,100 bytes - `work/ghidra/rel-d_basesNP/export/8076c900_FUN_8076c900.c`

**Body:** `0x8076C900..0x8076D134` (525 words; `blr` at `0x8076D130`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0x70`; saves `r31`-`r28`; no save helper, no `psq`.
Raw head: `or r30,r3,r3` at `0x8076C91C` proves **`this` = r3**; unary.

`this` fields: `+0x188` player holder (`+4` = actor), `+0x198` timer,
`+0x19C..+0x1A4` destination, `+0x1A8` state, `+0x1AC` counter, `+0x1B0`
helper handle, `+0x1B4` flag, `+0x1B8` reference actor, `+0x10C` speed,
`+0xAC..0xB4` position. `w = *(u32*)0x8042A27C`, `mgr = *(u32*)0x8042A26C`,
`g = DAT_80834470`, `pm = *(u32*)0x8042A284`.

**Exact behaviour.** Common head: `0x8006CC00(); 0x8006CD40(this);` if
`0x800D6AB0(*(u32*)(*(u32*)(this+0x188)+4))` then
`0x80105060(this[0x10C], w, 0x10, 2, sub[0x152])`. Then
`switch (this[0x1A8])` with **19 states**; every state ends by writing
`+0x1A8`, and the field `+0x198` is the per-state timer:

| state | action |
|---:|---|
| `0` | if `this[0xAC] > this[0x19C] - *(f32*)0x807CF110`: `this[0x194] = (*(f32*)0x807CF168 * (this[0x19C]-this[0xAC]) / K) / K` (two `0x43300000`-ladder divisions); `vcall(sub,0x5C)(*(f32*)0x807CF16C, *(f32*)0x807CF14C, *(f32*)0x807CF150, sub, 2)`; `this[0x1B0]=0x80103410(mgr,2,vcall(sub,0x28)(),"kinopio_all_root",0,0); this[0x1A8]=1;` |
| `1` | `this[0x10C] += this[0x194]`; if `this[0x198] < 1`: `this[0x10C]=*(f32*)0x807CF150; vcall(sub,0x5C)(*(f32*)0x807CF158,*(f32*)0x807CF14C,sub,0xAB); this[0x198]=10; this[0x1A8]=2; this[0x1AC]=0; 0x801034E0(mgr,this[0x1B0]); 0x80105240(w,0x35,this+0xAC,1);` else `this[0x198]-=1` |
| `2` | if `this[0x1AC] > 1`: `this[0x198]=0xB4; g[0x544]=1; g[0x546]=1; g[0x55C]=0xD; this[0x1A8]=4;` |
| `4` | timer 0xB4 -> 0x78 and state 5 |
| `5` | count down; on expiry or `(record->flags & 0x900)`: `g[0x545]=1; if (0x800FC9C0()==0) { if (g[0x538]) { *(u8*)(g[0x538]+0x251)=1; *(u32*)(g[0x538]+0x254)=1; } g[0x54D]=1; } else { same two stores; g[0x54D]=1; } this[0x1A8]=6;` |
| `6` | if `g[0x54D]==0`: `this[0x198]=0xB4; vcall(sub,0x5C)(*(f32*)0x807CF158,*(f32*)0x807CF170,*(f32*)0x807CF150,sub,0); g[0x544]=1; g[0x546]=1; g[0x55C]=0xE; this[0x1A8]=7;` |
| `7` | timer 0x78 -> state 8, `this[0x1AC]=0` |
| `8` | count down; expiry or `(flags & 0x900)` -> `g[0x545]=1; this[0x1A8]=9;` |
| `9` | `this[0x198]=0x1E; this[0x1A8]=0xA;` |
| `0xA` | when timer expires: if `this[0x1B8]==0`: `0x800F6410(this,&{*(f32*)0x807CF174,0,0}); this[0x10C]=*(f32*)0x807CF178; vcall(sub,0x5C)(...,2); this[0x1B0]=0x80103410(...); this[0x1A8]=0xB;` else `this[0x1A8]=0x11` |
| `0xB` | if `this[0xAC] < *(f32*)0x807CF148`: `0x800F6CB0(this); 0x801034E0(mgr,this[0x1B0]); vcall(sub,0x5C)(1.0f,0.0f,0.0f,sub,0); this[0x1A8] = 0x8076D280(this) ? 0xE : 0x13;` |
| `0xC` | `0x800F1160(*(u32*)0x8042A18C + *(i32*)(*(u32*)0x8042A18C+0x338C)*0xBF8 + 0x1A0, &buf); 0x80100270(&v, *(u32*)0x8042A18C, 0x808168BC); this[0x19C..0x1A4]=v; pm[0x604]=2; pm[0x5F4]=this+0x19C; pm[0x5F0]=0; pm[0x624]=0; pm[0x608]=0; pm[0x71C]=0x807CF120; this[0x198]=0x3C; this[0x1A8]=0xD;` |
| `0xD` | if `pm[0x5F4]==0`: timer 0x3C -> state 0xE |
| `0xE` | `0x800FCA00(); a=0x80162D50(0x2A7,0); if (a) { q=0x80100530(*(u32*)0x8042A18C,0x808168BC,0); a[0x184] = q ? q[0xC] : 0; 0x80100270(&v,...,0x808168BC); a[0xAC..0xB4]=v; } 0x8076AF60(); 0x807999D0(*(u32*)0x8042A1A0); this[0x198]=0x3C; this[0x1A8]=0xF;` |
| `0xF` | timer 0x3C -> state 0x13 |
| `0x10` | if `pm[0x5F4]==0` -> state 0x13 |
| `0x11` | `0x80792A10(this[0x1B8]); this[0x1A8]=0x12;` |
| `0x12` | if `!this[0x1B4] && *(f32*)(this[0x1B8]+0xAC) <= this[0xAC]`: `0x80105240(w,0x67,this+0xAC,1); this[0x1B4]=1;` if `this[0x1B4]`: `vcall(sub,0x5C)(0.0f,0.0f,0.0f,sub,0x54); this[0xAC] = *(f32*)(this[0x1B8]+0xAC) - *(f32*)0x807CF170;` `if (0x80792A20(this[0x1B8])) { this[0x124]=0; this[0x1A8]=0xE; }` |
| `0x13` | `vcall(this+0x60,0x68)(this);` (exit) |

State `3` does not exist in the export's table (there is no `case 3`); the
switch simply falls out for it.

**Signature.**

```rust
/// Nineteen-state "Toad airship / kinopio" scripted sequence: fences the four
/// players in, plays the `kinopio_all_root` actor, toggles the scene flags at
/// `0x80834470+0x544..0x54D`, moves the camera through `+0x19C` and exits.
/// # Original
/// `FUN_8076c900`, `0x8076C900`, 2,100 bytes,
/// `work/ghidra/rel-d_basesNP/export/8076c900_FUN_8076c900.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - sequence actor; `+0x188` player holder, `+0x198` timer,
///   `+0x19C..0x1A4` camera target, `+0x1A8` state, `+0x1AC` counter,
///   `+0x1B0` helper handle, `+0x1B4` flag, `+0x1B8` reference actor.
/// # Confidence
/// medium - every state, timer and scene write is literal; `0x8076D280` (76),
/// `0x8076AF60`, `0x80792A10`/`0x80792A20` and `0x807999D0` are unclaimed
/// helpers invoked with the shapes shown.
pub unsafe fn fn_8076c900(this: usize)
```

**Constants.** `0x80834470` scene object (`+0x538`, `+0x544`-`+0x54D`,
`+0x55C`); float pool `0x807CF110`, `0x807CF120`, `0x807CF148`, `0x807CF14C`,
`0x807CF150`, `0x807CF158`, `0x807CF168`, `0x807CF16C`, `0x807CF170`,
`0x807CF174`, `0x807CF178`, `0x807CF180`; timers `10`, `0xB4`, `0x78`, `0x1E`,
`0x3C`; sound ids `0x35`, `0x67`; camera record stride `0xBF8` at
`*(u32*)0x8042A18C + 0x338C*0xBF8 + 0x1A0`; string `"kinopio_all_root"`
(`0x80816900`), path `0x808168BC`; `0x80429F50`, `0x8042A464` record selection.

**Verdict: ready (medium).** All nineteen states are decoded; "medium" marks the
unclaimed siblings and the `0x807CF1xx` pool cells.

---

## 12. `0x80773210` - 2,096 bytes - `work/ghidra/rel-d_basesNP/export/80773210_FUN_80773210.c`

**Body:** `0x80773210..0x80773A40` (524 words; `blr` at `0x80773A3C`).
Ledger: absent. `src/game/bases.rs`: absent.

Frame `0xC0`; saves `f31` (`stfd`+`psq_st qr0`), `r31`-`r29`; no save helper.
Raw head: `or r30,r3,r3` at `0x80773234` proves **`this` = r3**; unary.
The single `unaff_GQR0` block is the `f31` spill.

`this` fields: `+0x6C444` state, `+0x6C448` previous state, `+0x6C44C`
counter, `+0x3220` record count, `+0x3224` record index, `+0x3180` record
table (stride `10`), `+0x338C` mode, `+0x3394` path offset, `+0x1A0` camera
record base (stride `0xBF8`). `audio = *(u32*)(*(u32*)0x8042A1A0 + 0x29C)`.

**Exact behaviour.** `switch (state)` with 13 states (`6`, `0xA`, `0xE` are
absent and return 0):

| state | action |
|---:|---|
| `0` | `while (0x80773A40() == 0) { if (this[0x3224] == this[0x3220]-1) return 1; this[0x3224]++; } this[0x6C448]=0; this[0x6C444]=1;` |
| `1` | `0x80773BB0(&pos,this); 0x80773CA0(&ang,this); 0x80C56AD0(audio,&pos,&ang); state=2; counter=0;` |
| `2` | `if (audio[0x74]==0) { if (counter < 0x1F) counter++; else state=3; }` |
| `3` | `if (0x80774090(this, this + this[0x3224]*10 + 0x3180)) return 1; if ((0x80773A40(this) & 1)==0) { 0x80775540(&a,this); 0x80775540(&b,this); vcall(audio+4,0x10)(audio,1,&a,&b); } else { 0x80773BB0(&a,this); 0x80773CA0(&b,this); vcall(audio+4,0x10)(audio,1,&a,&b); } state=5;` |
| `4` | if `audio[0x74]==0`: send the record pair (`0` channel) through `vcall(audio+4,0x10)` and state=5; else if `prev==7`: `vcall(audio+4,0x18)(*(f32*)0x807CF3B4 / (float)(u16 *(u16*)0x8031C9B2 - 176.0f), audio)` |
| `5` | if `audio[0x74]==0`: `flags=0x80773A40(); 0x80775190(this);` if `flags&1`: `idx=0x80773A60(this); if (idx>=0 && 0x800F57A0(this + this[0x338C]*0x16518 + 0x3394, idx, 0x80000)) 0x80C56270(audio,8,1); 0x800EFE20(camera, record); vcall(audio+4,0x14)(audio,&{0,0,0}); state=7;` else dispatch on `flags&2/4/8` -> state 8/9/10 |
| `7` | `0x800EFE20(cam, record); vcall(audio+4,0x18)(frac, audio); if (0x800F01B0(frac,cam,record)) { if (last record) state=0xB; else walk `this[0x3224]++` calling `0x80775660(this)`; on hit state=4 (jump to case 6 epilogue); else state=0xB; }` |
| `8` | `if (0x800F0800(cam,record)) { same last/advance logic -> state 0xB / 4; }` |
| `9` | `if (0x800F0AD0(cam,record)) { same -> state 0xB / 4; }` |
| `0xB` | `counter=0; state=0xC;` |
| `0xC` | `if ((i32)*(i32*)0x8031C9B4 < counter && 0x807754D0()) { counter=0; state=0xD; } else if (audio[0x74]==0) counter++; else if (prev==7) vcall(audio+4,0x18)(frac, audio);` |
| `0xD` | `if ((i32)*(i32*)0x8031C9B6 < counter) { vcall(audio+4,0x1C)(audio); state=0xF; } counter++;` |
| `0xF` | `if (audio[0x74]==0) { vcall(audio+4,0x20)(audio); return 1; }` |
| other | return 0 |

`frac` is `*(f32*)0x807CF3B4 / (float)((u16)*(u16*)0x8031C9B2 - 176.0f)`; the
record walk is `this[0x3224]` over `this[0x3220]` records of 5 halfwords.

**Signature.**

```rust
/// World-map route/record player: walks the `+0x3180` record list, drives the
/// three audio channels through `0x80C56AD0`/`vcall(+4,0x10..0x20)` and the
/// `0x800EFE20`/`0x800F01B0`/`0x800F0800`/`0x800F0AD0` camera track, and
/// returns 1 when the sequence is finished.
/// # Original
/// `FUN_80773210`, `0x80773210`, 2,096 bytes,
/// `work/ghidra/rel-d_basesNP/export/80773210_FUN_80773210.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - route player; `+0x6C444` state, `+0x6C448` previous state,
///   `+0x6C44C` counter, `+0x3220`/`+0x3224` record count/index, `+0x3180`
///   record table, `+0x338C` mode, `+0x3394` path, `+0x1A0` camera record.
/// # Returns
/// `1` from states `0` (list exhausted), `3` (record failure) and `0xF`.
/// # Confidence
/// medium - every state, index walk and audio call is literal; the
/// `0x80774090`/`0x80775190`/`0x807754D0`/`0x80775540`/`0x80775660` siblings are
/// unclaimed and `0x800EFE20`/`0x800F01B0`/`0x800F0800`/`0x800F0AD0` are
/// `main.dol` camera-track routines with visible argument shapes.
pub unsafe fn fn_80773210(this: usize) -> u32
```

**Constants.** `0x807CF3B4` = time scale, `0x807CF3B8` = 176.0f bias,
`0x807CF390` = 0.0f; `0x8031C9B2` u16 duration, `0x8031C9B4`/`0x8031C9B6`
frame thresholds; record stride `10` at `+0x3180`; path stride `0x16518` and
camera stride `0xBF8`; `0x8042A1A0` player actor, `0x8042A18C` scene manager.

**Verdict: conditional (medium).** The state graph and every index walk are
decoded. Conditions: `0x80774090`, `0x80775190` (820 bytes), `0x807754D0`,
`0x80775540`, `0x80775660`, `0x80773A40`, `0x80773A60`, `0x80773BB0`,
`0x80773CA0` are unclaimed siblings, and the `0x8031C9Bx` durations live in
`main.dol`.

---

## Ready vs conditional

| address | verdict | residual work |
|---|---|---|
| `0x8079C330` | **conditional (medium)** | `0x80101630` is called with one and with two arguments in adjacent arms; its register contract is not readable here. |
| `0x8077B450` | ready (medium) | `0x8077C670` (924 bytes) is an unclaimed sibling holding the per-route model; both dispatch structures are decoded and every argument register is visible. |
| `0x8065CB20` | ready (medium) | every callee is a literal import or a claimed engine routine; the six float arguments are read from `r3..r10`/`f1..f6` explicitly. |
| `0x80661070` | **conditional (medium)** | `0x8001CDB0`/`0x8001CE60` are unclaimed `main.dol` routines whose two arguments are supplied from object fields; `0x806601B0` is an unclaimed sibling (260 bytes). |
| `0x8079D3C0` | **conditional (medium)** | `0x80799340` (604 bytes) and `0x807970A0` (128) are unclaimed; the `0x801C14C0`/`0x801C1650` activation pair takes a handle plus a mask whose meaning is external. |
| `0x8076DA30` | ready (medium) | every callee is literal and the 16-entry table is in this REL; `0x8076E490` (228) is unclaimed but only receives `this` and a step index. |
| `0x8078DC70` | **conditional (medium)** | the `+0x14` virtual return value selects between two unclaimed host calls; `0x807C3B40` (20) is an import thunk. |
| `0x8076EF80` | **conditional (medium)** | `0x8076FE40`/`0x8076FE70`/`0x8076FEF0` are unclaimed siblings of this state table; the body calls them with `this` plus a float. |
| `0x807108B0` | ready (medium) | `0x8006A4B0` is the documented `main.dol` vector copy; the remaining callees are literal. |
| `0x8065F910` | ready (medium) | `0x806601B0` and `0x8065DCA0` are unclaimed siblings, but the call shape is `this` only. |
| `0x8076C900` | ready (medium) | `0x8076D280` (76) is unclaimed and receives `this`; everything else is literal. |
| `0x80773210` | **conditional (medium)** | `0x80775190` (820) and `0x807754D0` (108) are unclaimed siblings; the 15-entry route table lives outside this REL image and is cited by address. |

**Split: 6 ready, 6 conditional.** No row is blocked by GQR3, and none is a
duplicate or a table.

## Verification performed

* `ledger/functions.csv` parsed read-only; the address column contains none of
  the twelve (checked against all 16,668 rows, not only `game::bases`).
* Every `docs/*.md` heading (`^#{1,6} `) scanned for `0x80xxxxxx`; none of the
  twelve appears in a heading. Several appear in prose skip lists
  (waves 129/135/139), which is what made them candidates rather than claims.
* `src/game/bases.rs`: no `pub unsafe fn fn_<addr>` for any of the twelve. The
  module header and the `0x807BD110` rejection comments are the only mentions.
* `work/ghidra/rel-d_basesNP/export/functions.tsv`: all twelve byte counts read
  from the `size` column and re-checked against the decoded extent.
* Raw decode: `work/rels/elf/d_basesNP.elf`, program headers re-read, formula
  `file_offset = (address - 0x80600000) + 0x1000`.
* Dispatch tables expanded from `.data` for `0x8079C330`, `0x8077B450`,
  `0x8079D3C0`, `0x8076DA30`; the other four dispatchers
  (`0x8078DC70`, `0x8076EF80`, `0x8076C900`, `0x80773210`) are comparison trees
  and were decoded case by case.
* Extent: `blr` at `entry + size - 4` for eleven; `0x8065CB20` one word later.
  Exactly one `blr` per body; no TSV row starts inside any body.
* Duplicate check: SHA-1 per body against every same-size `d_basesNP` row - no
  match.
* Constant values were read through the program headers and are quoted with
  their image addresses. Addresses below `0x80600000` are `main.dol` runtime
  cells and are cited as addresses, never as literals.

## Hand-off

* The remaining `0x8079`/`0x8077`/`0x8076` state tables below this cut
  (`0x8062C8E0`, `0x8065DD50`, `0x8065F090`, `0x8071D500`, `0x8067FC80`,
  `0x807740D0`) are the natural wave 141 set.
* `0x807BD110` still needs the `StateID` table generator, not a control-flow
  spec.
* Six unclaimed siblings recur across these bodies and would pay for themselves
  as a separate claim: `0x8077C670` (924), `0x80775190` (820),
  `0x80799340` (604), `0x806601B0` (260), `0x8076E490` (228), `0x8076FE70` (116).
