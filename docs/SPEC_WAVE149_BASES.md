# Wave 149 - `game::bases` - the next eight (0.73-0.69 KB band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified. The only file written is this one. No merge or
check tool was run. The throwaway decoder and the ELF/literal readers live under
`%TEMP%\opencode\w149` (`ppc.js`, `cand.js`, `each.js`, `verify.js`, `refs.js`,
`dumpelf.js`, `dumprange.js`, `strs.js`), never in the repository.

Waves 128-148 walked this bucket from the top in size order; the last row claimed
by `docs/SPEC_WAVE148_BASES.md` is 764 bytes. This wave continues one rung
lower, **744 bytes down to 708 bytes**, which is the eight-row set
`docs/SPEC_WAVE148_BASES.md:112-113` predicted verbatim. Every row is decoded
instruction by instruction, gets a field map, a numbered behaviour list, a
constant table, its callees and an exact signature.

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x8071DCC0` | 744 | 186 | `8071dcc0_FUN_8071dcc0.c` | ready (high) |
| 2 | `0x80798F10` | 744 | 186 | `80798f10_FUN_80798f10.c` | ready (high) |
| 3 | `0x8069C780` | 740 | 185 | `8069c780_FUN_8069c780.c` | ready (high) |
| 4 | `0x806E3090` | 728 | 182 | `806e3090_FUN_806e3090.c` | ready (high) |
| 5 | `0x80796DC0` | 728 | 182 | `80796dc0_FUN_80796dc0.c` | ready (high) |
| 6 | `0x806D29C0` | 724 | 181 | `806d29c0_FUN_806d29c0.c` | ready (high) |
| 7 | `0x8079A040` | 716 | 179 | `8079a040_FUN_8079a040.c` | ready (high) |
| 8 | `0x8064F0A0` | 708 | 177 | `8064f0a0_FUN_8064f0a0.c` | ready (high) |

**Ready: 8. Conditional: 0.**

Every `bytes` value is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim and `decompiled` is
`true` for all eight. In all eight the size is the true instruction span: the
decoder found exactly one `blr`, at `address + size - 4`
(`0x8071DFA4`, `0x807991F4`, `0x8069CA60`, `0x806E3364`, `0x80797094`,
`0x806D2C90`, `0x8079A308`, `0x8064F360`), and no earlier one. There is no
padding tail, no `halt_baddata` cut and no unconditional branch out of the
extent: **2** of the 1,458 words in the eight bodies are outside the throwaway
decoder's table - `divw` at `0x8071DD48` (row 1, `XO = 491`) and the double
`fsub` at `0x806D2A78` (row 6, `XO = 20` with `A = 0`) - both hand-decoded
below - and every `b`/`bc` target is inside its own body.

### Why all eight are ready

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image, every callee is named by address (or by its ledger name), and
  the body can be written as code with no assumed calling convention beyond the
  standard PowerPC EABI. Data in `main.dol` BSS (`0x8035xxxx`, `0x8037xxxx`,
  `0x8042xxxx`), in this REL's BSS (`0x80822900..0x80834D84`), or in another
  REL is an *input*, named by address.
* **conditional** - exactly one named input would still be missing.

### The two GQR scares retired

Neither of the two rows rejected by earlier lanes has any data-side `psq`:

* **`0x806D29C0`** was rejected by `docs/COVERAGE_WAVE28_BASES.md:88,191`
  ("paired-single reconstruction (`unaff_GQR0`, `ldexpf`)") and left
  untranslated. The raw body has **no** `psq` word at all, no `mfspr` on any
  `gqr`, and one `stfd f31`/`stfdu`-free scalar save. The Ghidra
  `unaff_GQR0`/`ldexpf` noise at `0x806D2A78` is the double `fsub`
  `f0 = f0 - f4` mis-printed by the decompiler; the word is `0xFC002028`
  (`D = 0`, `A = 0`, `B = 4`, `C = 0`, `XO = 20`), i.e. `fsub f0, f0, f4`.
  This is the same rejection class as `0x806DA520` in wave 148.
* **`0x806E3090`** was deferred by `docs/COVERAGE_WAVE80_BASES.md:142-144`
  ("the same paired-single idiom plus `CONCAT44(0x43300000, ...)` integer
  conversions"). The raw body contains no `psq`, no `stfd`, no `lfd` pair and
  no `mfspr gqr`; the `CONCAT44(0x43300000, ...)` is the ordinary unsigned
  `u32 -> f64` ladder around `subf`/`xoris`, and both uses are transcribed
  below. Nothing about the float lanes is missing.

### This is the list wave 148 predicted, exactly

Re-deriving the candidate set from the current tree gives **1,885** rows: the
5,615 `functions.tsv` rows, filtered against the 16,974 distinct addresses in
`ledger/functions.csv` and the 1,552 distinct `0x`/bare addresses that already
appear in a `#{1,6} ` heading of `docs/*.md`, and against `decompiled=true`.
Wave 148 published 1,901 before the wave-148/149 merges; the 16-row difference
is the addresses that the translation lane has since claimed. The first eight
are byte-for-byte `docs/SPEC_WAVE148_BASES.md:112-113`:

```
0x8071DCC0 744 | 0x80798F10 744 | 0x8069C780 740 | 0x806E3090 728
0x80796DC0 728 | 0x806D29C0 724 | 0x8079A040 716 | 0x8064F0A0 708
```

The 744-byte tie is broken by address ascending (`0x8071DCC0` before
`0x80798F10`), and the 728-byte tie is ordered `0x806E3090` then `0x80796DC0`,
exactly as wave 148 said they would be. The 708-byte tie with
`0x80775D00` is broken by address ascending, so `0x8064F0A0` is row 8. Each of
the eight was re-tested against `ledger/functions.csv` (exact `address` field),
against every `#{1,6} ` heading in `docs/*.md` (including the bare and lowercase
spellings), and against `src/game/bases.rs` (any `fn_<addr>` token absent). All
eight are absent from all three, and each has a `decompiled=true` TSV row with a
matching `.c` export on disk. SHA-1 of the eight bodies against every same-size
`d_basesNP` row: **no duplicate**.

Applying the same filter after this wave, the next eight are:
`0x80775D00` (708), `0x80626DB0` (704), `0x8070F370` (692), `0x806D1EF0` (684),
`0x807BC310` (684), `0x8078FC80` (680), `0x807A6BE0` (680), `0x80680FD0` (676).

## Method

**Address mapping.** Re-derived from the program headers of
`work/rels/elf/d_basesNP.elf` (also visible in `work/rels/elf/summary.json`),
not copied:

```
PT_LOAD  p_offset  p_vaddr     filesz    memsz
  0      0x0010F0  0x806000F0  0x1C6004  0x1C6004
  1      0x1C70F4  0x807C60F4  0x0004FC  0x0004FC
  2      0x1C75F0  0x807C65F0  0x00000C  0x00000C
  3      0x1C7600  0x807C6600  0x00A5FC  0x00A5FC
  4      0x1D1C00  0x807D0C00  0x051CB8  0x051CB8
  5      0x000000  0x80822900  0x000000  0x012484 (zero fill)
```

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) +
0x1000` for every code address in this document, as `docs/REL_ELF_MAPPING.md`
requires. Literal pools in `0x807Cxxxx`/`0x807Dxxxx` resolve through segments
1-4 and are read out verbatim below. `0x8082xxxx` below `0x80822900` is real
data; `0x80822900..0x80834D84` is this REL's BSS and reads as zero.

`main.dol`-resident tables (`0x8031D634`, `0x8031D39C`, the `0x8031D3xx`
parameter block) are read from `work/dol/main.elf`, whose program headers map
`0x802FE3A0..0x80351640` contiguously at `p_offset 0x2FA900`; the reader applies
that segment rule directly rather than a `-base` shortcut.

**Decoder.** Reused the wave-146 throwaway decoder
(`%TEMP%\opencode\w149\ppc.js`) and re-validated it against the facts
`docs/REL_ELF_MAPPING.md` pins: branch `LK` is bit 0 and `AA` bit 1;
conditional branches use the 14-bit `BD`; the `SPR` field is
`((word >> 11) & 0x1F) << 5 | ((word >> 16) & 0x1F)`; `psq_l` = 56 and
`psq_st` = 60 with the quantiser in bits 12-14 and `W` in bit 15; single-float
`fmuls`/`fdivs` use `FRC`. Two decoder notes for this pass:

* `XO = 491` (`opcode 31`) is `divw`. Row 1 prints `x31_unk 0x1eb` at
  `0x8071DD48`; the word is `0x7C041BD6`, i.e. `divw r0, r4, r3`
  (`rD = 0`, `rA = 4`, `rB = 3`), confirmed bit by bit. It computes the
  remainder `r4 - (r4 / r3) * r3` in the next two instructions.
* `fc002028` (row 6) is the double `fsub f0, f0, f4`; the field-`A` value 0 is
  the register `f0`, not a literal, for floating-point X-form opcodes. There
  are no other gaps.

**CodeWarrior helpers.** Rows 2, 5 and 7 save GPRs through the
`0x802DCD40..0x802DCD74` stubs and restore through `0x802DCD8C..0x802DCDC0`.
The mapping used here (stride 4 from `_savegpr_14` at `0x802DCD40`) is:

| row | prologue | helper | registers saved | `this` recovered into |
|--:|---|---|---|---|
| 2 | `0x80798F20` | `_savegpr_27` = `0x802DCD74` | `r27..r31` | `r30` |
| 5 | `0x80796DD0` | `_savegpr_27` = `0x802DCD74` | `r27..r31` | `r31` |
| 7 | `0x8079A050` | `_savegpr_26` = `0x802DCD70` | `r26..r31` | not kept (`r3` unused) |

The matching epilogues are `0x802DCDC0`, `0x802DCDC0` and `0x802DCDBC`. The
helpers write through `r11` and never touch `r3`, so the incoming `this`
survives. Rows 1, 3, 4, 6 and 8 have no saved GPR helper and take `r3` directly.

**Class attribution.** Six of the eight can be named from data in the produced
image that no earlier document used:

* `0x80809840..0x808098C0` is a 15-record `{0, -1, handler}` state table; row 1's
  handler sits at record `0x80809864` and row 2's sibling `0x8071E130` at
  `0x808098B4`. The adjacent names at `0x808098C4..0x80809950` are
  `daObjWendy_c::StateID_{Wait,Fall,Rise,Fill,Drop}` and the parameter block
  `0x80809620` is the same one wave 141/148 used for `0x8071D500`/`0x8071E130`.
  Rows 1 and 2 are therefore bodies of that same world-9 water object. Wave 141
  already named the `+0x1A488`/`+0x1A59C` pair; row 1 uses both.
* `0x807F18E4..0x807F1920` is the state table whose handlers include
  `0x8069C2B0`, `0x8069C2C0`, `0x8069C170` and `0x8069C780`; the names
  immediately after it at `0x807F1928..0x807F1978` are
  `daJrClownB_c::StateID_DemoWait`, `..._DemoAwake`, `..._Move`. Row 3 is a
  `daJrClownB_c` body (the same family as wave 146 §6's `0x8069C2C0`).
* `0x807FCD14..0x807FCD3C` is a state table whose handlers include
  `0x806E2F50`, `0x806E3080`, `0x806E3090`, `0x806E2FD0`, followed at
  `0x807FCD40` by `daLiftWaterWheel_c::StateID_NoDisp` / `..._Move`. Row 4 is
  the `daLiftWaterWheel_c` per-frame driver, exactly as wave 10/80 guessed.
* `0x807FAC40..0x807FAC60` is a vtable whose first word is `0x806D29C0`; the
  string `Wm_ob_cmnsparkloop` sits at `0x807FAC24`. Row 6 is that world-map
  object's initialiser.
* `0x807E4C30..0x807E4C70` is the state table whose handlers include
  `0x8064ED70`, `0x8064F090`, `0x8064F0A0`, `0x8064F080`, `0x8064F380`, followed
  at `0x807E4D14` by `daBgmInterlockingDummyBlock_c::StateID_WaitForLeader` and
  the other five `..._WaitFor*/Move*` names. Row 8 is that class's animation
  attach/release body.

Rows 5 and 7 are named from their own dispatch tables and callees (below).

**Unexported callees.** `0x800B2EF0`, `0x80100200`, `0x800FDC80`,
`0x80100A50`, `0x80100970`, `0x800FAD60`, `0x800F9370`, `0x800C1510`,
`0x800C1230`, `0x800C12B0`, `0x800C12D0`, `0x800C1560`, `0x800C1580`,
`0x80043CA0`, `0x800DB180`, `0x800DAE80`, `0x801995A0`, `0x800F9BA0`,
`0x800FAE40`, `0x800FD2E0`, `0x800FB350` and `0x800F57A0` have no
`functions.tsv` row in either `rel-d_basesNP` or `main-gb` (or are exported
under a different name), and no `.c` export in this tree. Each was checked
against the exported extent that precedes it: every one starts inside a larger
un-analysed `main.dol` gap, and every one is called with registers the compiler
has just set up. They are named by address only, exactly as
`docs/SPEC_WAVE128_BASES.md` and `docs/SPEC_WAVE148_BASES.md` do; where a
documented precedent exists it is cited.

**Call notation.** Throughout this document `callN(addr, a, b, ...)` lists the
integer argument registers `r3, r4, ...` from left to right and treats the
callee as returning `f32` in `f1` when the value is used as a float. Where a
callee takes a float in `f1` in addition to (or instead of) `r3`/`r4`, the call
is written `addr(f1 = ..., r3 = ...)` explicitly. This matters for
`0x8065C210` (a `t` in `f1` plus two pointers), `0x800C10F0`/`0x800C1560`
(a pointer plus a `f32` pair) and `0x800DB180` (four `f64` in `f1..f4` plus
`r3..r7`).

---

## 1. `0x8071DCC0` - water-screen fill/hold arm of the world-9 water object

**Evidence.** `work/ghidra/rel-d_basesNP/export/8071dcc0_FUN_8071dcc0.c`,
744 bytes, 186 words, synthetic `0x8071DCC0`. Body `0x8071DCC0..0x8071DFA8`
(ends `blr`). Ghidra's five `unreachable block` warnings
(`0x8071DF64..0x8071DF84`) are the five `psq_l f31/f30/f29/f28/f27` restores.

**What it is.** A two-phase body of the same large water object whose sequence
function `0x8071D500` is specified in `docs/SPEC_WAVE141_BASES.md:541-647` and
whose `0x8071E130` drop/floor handler is `docs/SPEC_WAVE148_BASES.md:391-519`.
The phase word is `+0x1A59C` (`0x6967` words in), which is the same word wave
141 used as its `0..5` state. This body drives the water's height `+0xB0` from
the shared curve block `0x80809620`, plays the per-frame water cue `0x553`,
spawns effect slot `0x1C` every 15 frames, and posts `0x80830100` (mode 3) or
`0x808300C0` (every other mode) when the rise completes.

### Signature

```rust
pub unsafe fn fn_8071dcc0(this: usize)
```

`r3` is the water object (`r29 = r3` at `0x8071DD10`). No other argument is
read. Returns nothing. The object is at least `0x1A5A0` bytes; `r6 = this +
0x20000` is the base used for the two big offsets.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x000` | `u32` | actor id, passed as the sound tag |
| `+0x0AC`/`+0x0B0`/`+0x0B4` | `f32` | world position; `+0xB0` is the driven height |
| `+0x394` | interface | state-message sink; vtable slot `+0x18` takes `(this+0x394, msg)` |
| `+0x1A488` | `u32` | rise timer (0..180), read as `this[0x6922]` |
| `+0x1A59C` | `u32` | phase word, `0` = fill, `1` = hold |

Globals: `0x80429E00` (render/camera object; `+0x38` and `+0x40` are the two
floats used for the effect position), `0x80429DD0` (effect-slot manager),
`0x80429D60` (audio manager), `0x8082FFE4` (`f32` water base height),
`0x8082FFEC` (`s32` mode), `0x808300C0`/`0x80830100` (BSS message blocks).
Curve block `0x80809620`: `+0x08` = `0.0f`, `+0x1C` = `216.0f`, `+0x24` = `75`
(`u32`), `+0x7C` = `8.0f`, `+0x80` = `180` (`u32`), `+0x84` = `0.1f`,
`+0x88` = `15` (`u32`). Pool: `0x807CD020` = `1.0f`,
`0x807CD028` = `f64 0x4330000080000000` (`2^52 + 2^31`, the biased
`u32 -> f64` ladder constant).

### Exact behaviour

Prologue `0x8071DCC0..0x8071DD18`: frame `0x90`; save LR, `f31..f27`
(`stfd` + `psq_st ...,qr0,0` pairs), `r31..r28`; `r29 = this`;
`r31 = 0x8082FFE0`; `r6 = this + 0x20000`; `r0 = [this+0x1A59C]`.

1. `0x8071DD20..0x8071DD2C` - `if [this+0x1A59C] != 0 { goto step 6 }`.
2. `0x8071DD2C..0x8071DD44` - load the curve block and camera:
   ```rust
   let p = 0x80809620usize;
   let period = read_u32(p + 0x88);          // 15
   let span   = read_u32(p + 0x24);          // 75
   let c216   = read_f32(p + 0x1C);          // 216.0
   let c8     = read_f32(p + 0x7C);          // 8.0
   let c0     = read_f32(p + 0x08);          // 0.0
   let cam    = *(0x80429E00 as *const usize);
   let cam_y  = read_f32(cam + 0x38);        // f31
   let cam_x  = read_f32(cam + 0x40);        // f30
   ```
3. `0x8071DD48..0x8071DD6C` - every `period` frames, spawn cue `0x1C`:
   ```rust
   let timer  = read_u32(this + 0x1A488);
   let quot   = timer / period;              // divw r0, r4, r3
   let rem    = timer - quot * period;       // mullw + subf.
   if rem == 0 {
       let t = call0_f32(0x800B2EF0);        // f1
       // 0x80078410(f1 = cam_y + cam_x*t, f2 = height, r3 = mgr, r4 = 0x1C)
       call4_f32(0x80078410,
                 cam_y + cam_x * t,
                 read_f32(this + 0xB0),
                 *(0x80429DD0 as *const u32),
                 0x1C);
   }
   ```
4. `0x8071DD8C..0x8071DDE0` - normalised fill weight:
   ```rust
   let mut w = 1.0f32;                       // f3
   if span > 0 {
       let d = span as i32 - timer as i32;   // subf r3,r4,r30
       let num = (d * d) as u32;
       let den = span * span;
       let ratio = (num as f64 - MAGIC) / (den as f64 - MAGIC);
       w = 1.0 - ratio as f32;
   }
   let height = read_f32(0x8082FFE4) + c0 + w * (c216 + c8);
   write_f32(this + 0xB0, height);
   ```
   (`MAGIC` is the `f64 2^52 + 2^31` at `0x807CD028`; the two `xoris 0x8000`
   stores at `0x8071DDB8`/`0x8071DDC4` build the biased low words.)
5. `0x8071DDFC..0x8071DE60` - per-frame cue and phase advance:
   ```rust
   let y = call0_f32(0x800B30E0);            // f31
   let x = call0_f32(0x800B30C0);            // f1
   let mut pos = [x, y];
   let mut out = [0u8; 8];
   call2(0x8006A3F0, &mut out, &pos);        // main.dol two-float copy
   call5(0x80199390, *(0x80429D60 as *const u32), 0x553,
         read_u32(this + 0x000), &out, 0);
   if read_u32(this + 0x1A488) >= span {
       write_u32(this + 0x1A488, 0xFFFF_FFFF);   // li r3,-1
       write_u32(this + 0x1A59C, 1);
   }
   ```
   The `timer >= 75` test is `cmpw r0,r30` + `blt` at `0x8071DE48`; the two
   stores are literal `li -1` / `li 1`.
6. `0x8071DE64..0x8071DE8C` - `if [this+0x1A59C] != 1 { goto epilogue }`; in
   state 1, `if [0x8082FFEC] == 3`:
   ```rust
   vcall(this + 0x394, 0x18, 0x80830100);
   ```
   (`lwzu r12,916(r3)` at `0x8071DE78` loads the interface and advances `r3`.)
7. `0x8071DE90..0x8071DF30` - the non-mode-3 hold arm:
   ```rust
   let p = 0x80809620usize;
   let span = read_u32(p + 0x24);            // 75
   let c216 = read_f32(p + 0x1C);            // 216.0
   let c8   = read_f32(p + 0x7C);            // 8.0
   let c0   = read_f32(p + 0x08);            // 0.0
   let limit= read_u32(p + 0x80);            // 180
   let mut w = 1.0f32;                       // f7
   if span > 0 {
       let t = read_u32(this + 0x1A488);
       let num = t * t;
       let den = span * span;
       w = 1.0 - ((num as f64 - MAGIC) / (den as f64 - MAGIC)) as f32;
   }
   let mut target = c216;
   let v = w * (c216 + c8);
   if c216 < v { target = v; }               // fcmpo + fmr = max(v, 216)
   let mut dst = read_f32(0x8082FFE4) + c0 + target;
   write_f32(this + 0xB0,
             call3_f32(0x8065C210, 0.1, this + 0xB0, &dst)); // t in f1
   if read_u32(this + 0x1A488) >= limit {
       vcall(this + 0x394, 0x18, 0x808300C0);
   }
   ```
   The `max` is implemented as `fcmpo cr0, f4, f2` + `bge` + `fmr f2,f4`, so
   the result is `max(v, 216.0)`; `0.1` comes from `0x808096A4`.
8. `0x8071DF60..0x8071DFA8` - epilogue: `psq_l`/`lfd` for `f31..f27`, restore
   `r31..r28`, LR, `blr`.

`vcall(this, 0x18, msg)` is `(*(this+0x394)->vtable->slot_0x18)(this+0x394,
msg)`; the load pair is `lwz r12,0x394(rX); lwz r12,0x18(r12); mtctr; bctrl`
(`0x8071DE78`, `0x8071DF48`).

### Constants

| symbol | value | use |
|---|---|---|
| `0x8080963C` | `216.0f` | fill amplitude |
| `0x8080969C` | `8.0f` | fill amplitude bias |
| `0x80809628` | `0.0f` | height offset |
| `0x80809644` | `75` | full-rise frame count |
| `0x808096A0` | `180` | hold completion frame count |
| `0x808096A8` | `15` | cue period (divisor) |
| `0x808096A4` | `0.1f` | `0x8065C210` blend factor |
| `0x807CD020` | `1.0f` | weight base |
| `0x807CD028` | `f64 4503601774854144` (`2^52+2^31`) | biased `u32 -> f64` |
| `0x8082FFE4` | `f32` | water base height |
| `0x8082FFEC` | `s32` | mode; `3` skips the hold arm |

Cue ids: audio `0x553` (`0x80199390`), effect slot `0x1C` (`0x80078410`).
Messages: `0x80830100` (mode 3) and `0x808300C0` (otherwise).

**Verdict: ready (high).** Both messages are BSS in this REL, every curve
constant is literal, and every callee is exported or named by address.
This confirms wave 148's retirement of the wave-94 "paired-single" rejection
for this family: row 1 has five `qr0` save pairs and no data-side `psq`.

---

## 2. `0x80798F10` - world-map pose / state scheduler

**Evidence.** `work/ghidra/rel-d_basesNP/export/80798f10_FUN_80798f10.c`,
744 bytes, 186 words, synthetic `0x80798F10`. Body `0x80798F10..0x807991F8`
(ends `blr`). The single `blr` is the only terminator; no Ghidra warnings.

**What it is.** The per-frame pose selector of a world-map object (its caller is
`0x80799210`, which also calls `0x80798D40`/`0x80798E20` and stores the return
value at `+0x2E0`). It resolves a 12-byte pose record from the `main.dol` table
`0x8031D634` (the same table `0x80799960` reads its two blend floats from),
optionally overrides the record by a two-anchor height test, fires a map
effect when the record is `5`, transitions the object through `0x80799960`,
then re-attaches one of the animation nodes `wrist_l1` / `wrist_r1` when the
pose index is `6`. It is the closest sibling of `0x807807E0`
(`docs/SPEC_WAVE129_BASES.md:1249-1341`), with `+0x22C`/`+0x230`/`+0x1D0` in
place of `+0x1F4`/`+0x200`/`+0x1CC` and the global record table in place of
`*(this+0x294)`.

### Signature

```rust
pub unsafe fn fn_80798f10(this: usize, slot: u32) -> u32
```

`r3` = object (`r30 = r3` at `0x80798F28`), `r4` = slot/selection id, forwarded
to `0x80775030` as its third argument (`or r5,r4,r4` at `0x80798F30`). The
return value is `r31`, the resolved pose index (`or r3,r31,r31` at
`0x807991E0`); the caller stores it at `+0x2E0`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x22C` | `u32` | zone/anchor id A (passed to `0x80100200` and the effect record) |
| `+0x230` | `u32` | zone/anchor id B |
| `+0x24C` | `u32` | transition counter, cleared when the state changes |
| `+0x1AC` | `s32` | animation handle A (wrist attach when `idx == 6`) |
| `+0x1B0` | `s32` | animation handle B |
| `+0x100` | object | attach node used by the `wrist_*` names |
| `+0x2E4` | object | attach node used by the non-6 name |
| `+0x1D0` | object | sub-object; `+0x151`/`+0x152` are two bytes, `+0x154` the current pose id |
| `+0x2E0` | `u32` | (written by the caller) returned pose index |

Globals: `0x8042A18C` (world manager, `mgr`), `0x80429F7C` (global object whose
`+0x380` byte selects `idx 2` vs `4` at index `0x0E`), `0x8031D634` (record
table, `{u32 id; f32 a; f32 b}` stride 12). `0x807D0160` = `0.0f`,
`0x807D0188` = `-10.0f`, `0x8031D3B0` = `12.0f`, `0x8031D3BA` = `s16 0`.
Strings `0x8081B834` = `"wrist_l1"`, `0x8081B840` = `"wrist_r1"`.

### Exact behaviour

Prologue `0x80798F10..0x80798F58`: frame `0x80`; save LR, `r27..r31` via
`_savegpr_27`; `r30 = this`; `r27 = *(0x8042A18C)`; `r5 = slot`;
`r4 = [this+0x22C]`.

1. `0x80798F3C..0x80798F5C` - pose index:
   ```rust
   let idx = call3_u32(0x80775030, mgr, read_u32(this + 0x22C), slot);
   let mut pose = read_u32(0x8031D634 + idx * 12);   // r28
   let mode = call0_u32(0x80100F70);                 // f1-free predicate
   ```
2. `0x80798F60..0x80798FDC` - mode non-zero override:
   ```rust
   if mode != 0 {
       let mut a = [0f32; 3]; let mut b = [0f32; 3];
       call3(0x80100200, &mut a, mgr, read_u32(this + 0x22C));
       call3(0x80100200, &mut b, mgr, read_u32(this + 0x230));
       if a[1] > b[1] {                       // A.y > B.y
           let sub = read_u32(this + 0x1D0);
           pose = if read_u8(sub + 0x152) == 5 { 0x8B } else { 0x16 };
       }
   }
   ```
   The two `0x80100200` calls write three `f32` each (precedent:
   `docs/SPEC_WAVE130_BASES.md:751-760`). The comparison is
   `fcmpo cr0, f0, f2` with `f0 = a[1]`, `f2 = b[1]`, `ble` skipping the
   override (`0x80798FAC..0x80798FBC`).
3. `0x80798FE0..0x80799014` - index `0x0E` remap (only when `mode == 0`):
   ```rust
   if mode == 0 && idx == 0x0E {
       let mut k = 4u32;
       if read_u8(*(0x80429F7C as *const u32) + 0x380) != 0 { k = 2; }
       pose = read_u32(0x8031D634 + k * 12);
   }
   ```
4. `0x80799018..0x807990D4` - pose `5` effect:
   ```rust
   if pose == 5 {
       let mut a = [0f32; 3]; let mut b = [0f32; 3];
       call3(0x80100200, &mut a, mgr, read_u32(this + 0x22C));
       call3(0x80100200, &mut b, mgr, read_u32(this + 0x230));
       let sub  = read_u32(this + 0x1D0);
       call4(0x80104FD0, *(0x8042A27C as *const u32), 0,
             read_u8(sub + 0x151), read_u8(sub + 0x152));
       // 0x80105060(f1 = 0.0, r3 = [0x8042A27C], r4 = 1, r5/r6 = the two bytes)
       call5_f32(0x80105060, 0.0, *(0x8042A27C as *const u32), 1,
                 read_u8(sub + 0x151), read_u8(sub + 0x152));
       call6(0x800F6110, /*f1*/ 12.0, /*f2*/ -10.0, this,
             &a, &b, 0);
   }
   ```
   `0x800F6110` is the five-float/pointer effect spawn documented in
   `docs/COVERAGE_WAVE91_BASES.md:24`; the last argument is
   `(s16)[0x8031D3BA] = 0`, loaded as `lha r6,30(r4)` from `0x8031D39C`.
5. `0x807990D4..0x80799100` - state transition:
   ```rust
   let cur = read_u32(read_u32(this + 0x1D0) + 0x154);
   if pose != cur {
       write_u32(this + 0x24C, 0);
       call3(0x80799960, this, idx, pose);       // state setter
   }
   call1(0x800FDC80, this);
   ```
   `0x80799960(this, idx, state)` is the 108-byte state setter: it stores
   `state` at `+0x250`, and when the state changes calls vtable `+0x5C` on
   `+0x1D0` with the record's two floats (`0x8031D638 + idx*12`,
   `0x8031D63C + idx*12`), or `3.5f`/`5.0f` when `state == 2`
   (`0x8031D650`/`0x8031D654`). Its own export is
   `work/ghidra/rel-d_basesNP/export/80799960_FUN_80799960.c`.
6. `0x80799104..0x80799184` - `idx == 6` wrist pair:
   ```rust
   if idx == 6 {
       let s = vcall(this + 0x1D0, 0x28);       // model/anim accessor
       write_u32(this + 0x1AC,
           call6(0x80103410, *(0x8042A26C as *const u32), 10, s,
                 "wrist_l1", this + 0x100, 0));
       let s = vcall(this + 0x1D0, 0x28);
       write_u32(this + 0x1B0,
           call6(0x80103410, *(0x8042A26C as *const u32), 10, s,
                 "wrist_r1", this + 0x100, 0));
   }
   ```
7. `0x80799184..0x807991DC` - all other indices:
   ```rust
   let sub = read_u32(this + 0x1D0);
   let s   = vcall(sub, 0x28);
   let a1  = call1_u32(0x80100A50, read_u8(sub + 0x151));
   let a2  = call1_u32(0x80100970, idx);
   write_u32(this + 0x1AC,
       call6(0x80103410, *(0x8042A26C as *const u32), a2, s, a1, 0,
             this + 0x2E4));
   ```
   The two forms differ by one argument position, exactly as
   `docs/SPEC_WAVE129_BASES.md:1336-1339` notes for the sibling.
8. `0x807991DC..0x807991F8` - epilogue: `or r3,r31,r31`, restore
   `r27..r31`, LR, `blr`.

The virtual calls are `lwz r12,0(r3); lwz r12,40(r12); mtctr; bctrl`
(`0x80799110`/`0x8079914C`/`0x8079918C`), i.e. slot `+0x28` with `r3 = sub`;
the two `0x80103410` attach calls are the world-map anim-node API already
translated in `src/game/worldmap.rs`.

### Constants

| symbol | value | use |
|---|---|---|
| `0x8031D634` | 12-byte record table | `{state id, f32, f32}`, index from `0x80775030` |
| `0x8031D3B0` | `12.0f` | `0x800F6110` first argument |
| `0x8031D3BA` | `s16 0` | `0x800F6110` last argument |
| `0x807D0160` | `0.0f` | `0x80105060` position weight |
| `0x807D0188` | `-10.0f` | `0x800F6110` second argument |
| `0x8081B834` | `"wrist_l1"` | attach node name |
| `0x8081B840` | `"wrist_r1"` | attach node name |
| `0x8042A18C` | world manager | `0x80775030`/`0x80100200` |
| `0x8042A26C` | anim-node allocator | `0x80103410` |
| `0x8042A27C` | effect-record manager | `0x80104FD0`/`0x80105060` |

Record samples read from `main.dol` (index -> state id): `0 -> 0xA9`,
`1 -> 2`, `2 -> 2`, `3 -> 0x35`, `4 -> 5`, `5 -> 6`, `6 -> 0x38`,
`10 -> 0x7F`, `14 -> 2`.

**Verdict: ready (high).** The record table and its two floats are read from
`main.dol`; both `0x80799960` consumers and both strings are pinned; the only
unexported helpers are named by address (`0x80100200`, `0x80100A50`,
`0x80100970`, `0x800FDC80`).

---

## 3. `0x8069C780` - `daJrClownB_c` per-frame body (no-timer variant)

**Evidence.** `work/ghidra/rel-d_basesNP/export/8069c780_FUN_8069c780.c`,
740 bytes, 185 words, synthetic `0x8069C780`. Body `0x8069C780..0x8069CA64`
(ends `blr`). Ghidra's single `unreachable block` warning
(`ram,0x8069CA44`) is the `psq_l f31` restore.

**What it is.** A third variant of the carried-object / clown-car per-frame
family already specified at `0x8069B750` (`docs/SPEC_WAVE128_BASES.md`) and
`0x8069C2C0` (`docs/SPEC_WAVE146_BASES.md:926-1071`). It runs the same tail
call set in the same order as `0x8069C2C0` minus that sibling's `+0x360`
launcher state machine: it runs the `+0x544` rider tick, the two clip-idle
predicates, publishes the four `+0xD30..+0xD34` flags, fires `0x8069ACA0`
whenever `+0x500` is zero, and then the shared animation/collision tail. The
state table `0x807F18E4` and its `daJrClownB_c::StateID_*` strings make the
class attribution direct.

### Signature

```rust
pub unsafe fn fn_8069c780(this: usize)
```

`r3` = actor (`r29 = r3` at `0x8069C7A0`). Returns nothing. Frame `0x40`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x60` | `void*` | class vtable |
| `+0x0AC`/`+0x0B0`/`+0x0B4` | `f32` | position |
| `+0x0EC` | `f32` | vertical speed, clamped to `0.0` by the tail |
| `+0x114`,`+0x11C` | object | two orientation holders (`0x8015F470`) |
| `+0x1EC` | object | collision/motion container |
| `+0x348` | `u8` | `(x_saved - x) < 0` |
| `+0x500` | `u16` | launch/tail timer; `0` fires `0x8069ACA0` |
| `+0x544` | object | rider sub-object (slot `+0x1C` is its tick) |
| `+0x5F8` | object | model object reached through slot `+0x2DC` |
| `+0x768`/`+0x76C`/`+0x770` | `f32` | saved/target position used by the sector test |
| `+0xD31`..`+0xD34` | `u8` | written flags (see below) |

### Exact behaviour

Prologue `0x8069C780..0x8069C7A0`: save LR, `f31` + `psq_st f31,qr0,0`,
`r31`, `r30`, `r29`; `r29 = this`. Epilogue `0x8069CA40..0x8069CA64`.

1. `0x8069C7A4..0x8069C7B4` - `vcall1(this + 0x544, 0x1C)` (rider tick).
2. `0x8069C7B8..0x8069C82C` - clip-idle gate:
   ```rust
   let model = vcall1(this, 0x2DC);                 // *(this+0x60) + 0x2DC
   let clip  = read_u32(model + 0x5F8);
   let mut both = 1u32;                             // r30
   if vcall0(clip, 0x0C) == 0 && vcall0(clip, 0x10) == 0 { both = 0; }
   if both != 0 { vcall1(this, 0x380); }
   ```
3. `0x8069C82C..0x8069C8A0` - signed-x and vertical-sector flags:
   ```rust
   let dx = read_f32(this + 0x768) - read_f32(this + 0xAC);
   let dy = read_f32(this + 0x76C) - read_f32(this + 0xB0);
   let dz = read_f32(this + 0x770) - read_f32(this + 0xB4);
   let neg = if dx < 0.0 { 1u8 } else { 0u8 };
   write_u8(this + 0x348, neg);
   if neg == 1 { write_u8(this + 0xD31, 1); }
   else        { write_u8(this + 0xD32, 1); }
   let s = call1_u32(0x8069A170, this);
   if s == 0 { write_u8(this + 0xD33, 1); }
   else if s == 2 { write_u8(this + 0xD34, 1); }
   ```
   `dx`, `dy`, `dz` are stored to `r1+8`, `r1+0x10`, `r1+0xC`; only `dx` is
   used by the branch (the other two stores are dead in this body but literal).
   `0x8069A170` is the +/-pi/8 sector test (wave 135 §3).
4. `0x8069C8CC..0x8069C8E4` - `if read_u16(this + 0x500) == 0 { fn_8069ACA0(this); }`
   (no `+0x360` guard, unlike the wave-146 sibling).
5. `0x8069C8E4..0x8069C9CC` - shared tail, part 1:
   ```rust
   call1(0x800A4E90, this);
   vcall1(this, 0x33C); vcall1(this, 0x340);
   call1(0x8006CD90, this); call1(0x8006CDE0, this); call1(0x800955F0, this);
   let a = vcall_f64(this, 0x350) as f32;           // f31
   let b = vcall_f64(this, 0x34C) as f32;
   call3_f32(0x8015F470, this + 0x114, b, a);       // dest, f1, f2
   let c = vcall_f64(this, 0x358) as f32;
   let d = vcall_f64(this, 0x354) as f32;
   call3_f32(0x8015F470, this + 0x11C, d, c);
   ```
6. `0x8069C9AC..0x8069C9CC` - `vcall1(this, 0x344)`;
   `let s = vcall_s16(this, 0x374); call2(0x800A4480, this, s);`.
7. `0x8069C9D0..0x8069CA2C` - the two `+0x1EC` clamps:
   ```rust
   if call1(0x80072BC0, this + 0x1EC) != 0 && read_f32(this + 0xEC) < 0.0 {
       write_f32(this + 0xEC, 0.0);
   }
   if call1(0x8006EF40, this + 0x1EC, 0) != 0 && 0.0 < read_f32(this + 0xEC) {
       write_f32(this + 0xEC, 0.0);
   }
   call2(0x8006FA40, this + 0x1EC, 0);
   ```
8. `0x8069CA34..0x8069CA3C` - `fn_80699E30(this); fn_8069A230(this);`.
9. `0x8069CA40..0x8069CA64` - epilogue.

### Constants

| symbol | value | use |
|---|---|---|
| `0x807CA08C` | `0.0f` | signed-x compare |
| `0x807F13C0` | `30` | (not used by this variant; the wave-146 sibling seeds `+0x500`) |

### Callees

| target | shape | note |
|---|---|---|
| `+0x544` -> `+0x1C` | `(obj)` | rider tick |
| `this` -> `+0x2DC` | `(this) -> obj` | model accessor |
| `obj+0x5F8` -> `+0x0C`,`+0x10` | `() -> u32` | clip-idle predicates |
| `this` -> `+0x380` | `(this)` | both-clips-idle reaction |
| `this` -> `+0x33C`,`+0x340`,`+0x344` | `(this)` | per-frame phases |
| `this` -> `+0x34C`,`+0x350`,`+0x354`,`+0x358` | `(this) -> f64` | orientation pairs |
| `this` -> `+0x374` | `(this) -> s16` | animation/index selector |
| `0x8069A170` | `fn_8069a170` | sector test (`1` band, `0` above, `2` below) |
| `0x8069ACA0` | `fn_8069aca0` | halfword stamp + `+0xD4` tail call |
| `0x800A4E90`, `0x8006CD90`, `0x8006CDE0`, `0x800955F0` | unexported `main.dol` | `(this)` |
| `0x8015F470` | `thunk_FUN_8015f480` | `(dest, f1, f2)` |
| `0x800A4480` | unexported | `(this, s16)` |
| `0x80072BC0`,`0x8006EF40`,`0x8006FA40` | unexported | `(this+0x1EC[, 0]) -> u32` |
| `0x80699E30`, `0x8069A230` | `fn_80699e30`, `fn_8069a230` | shared tail |

**Verdict: ready (high).** The tail call set, every slot offset and every flag
match the already-transcribed wave-128/146 siblings; the only difference from
`0x8069C2C0` is the missing `+0x360` gate, and the whole body decodes with no
`unaff_GQR0` beyond the single `f31` spill.

---

## 4. `0x806E3090` - `daLiftWaterWheel_c` per-frame driver

**Evidence.** `work/ghidra/rel-d_basesNP/export/806e3090_FUN_806e3090.c`,
728 bytes, 182 words, synthetic `0x806E3090`. Body `0x806E3090..0x806E3368`
(ends `blr`). Ghidra's only warning is the overlapping-globals note; there are
no unreachable-block warnings and no `psq`.

**What it is.** The per-frame body of the `daLiftWaterWheel_c` water-wheel lift
(`g3d/obj_waterscreen.brres`, `waterPlate_W4boss`; class strings at
`0x807FCD40`). It gates on the two-word rider mask, advances an angle
`+0x8C4` by a clamped signed delta `+0x8C6`, either integrates the actor's x
from that delta or reads the wheel's x/y back from the `+0x7BC` geometry object,
detects a collision through `0x806E2B10` and restores the pre-move position,
plays cue `0x2B4` on the first blocked frame and `0x2B3` on every moving frame,
and posts `0x8082E0B0` when the mask test says no rider is aboard.

### Signature

```rust
pub unsafe fn fn_806e3090(this: usize)
```

`r3` = actor (`r30 = r3` at `0x806E30AC`). Returns nothing. Frame `0x50`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x000` | `u32` | actor id (sound tag) |
| `+0x0AC`/`+0x0B0`/`+0x0B4` | `f32` | position |
| `+0x358`/`+0x35C` | `u32` | rider mask halves |
| `+0x394` | interface | state-message sink, vtable slot `+0x18` |
| `+0x7BC` | object | wheel geometry/path state |
| `+0x8B4` | `f32` | previous delta (blocks the `0x2B4` repeat) |
| `+0x8B8` | `u32` | "blocked last frame" latch |
| `+0x8BC` | `u32` | rider gate (0 = skip the mask test) |
| `+0x8C0` | `u32` | cue cooldown, seeded with 16 |
| `+0x8C4` | `s16` | wheel angle |
| `+0x8C6` | `s16` | signed angle delta, clamped to `[-256, 256]` |
| `+0x8C8` | `u8` | 1 = read x/y from the geometry object |

Globals: `0x8042A078` (`u32*`, two-word rider mask), `0x80429D60` (audio
manager). Constant block `0x807CBB98`: `+0x38` = `0.0f`, `+0x3C` = `8.0f`,
`+0x5C` = `0.5f`, `+0x60` = `2048.0f`, `+0x68` = `f64 2^52+2^31`
(`0x807CBC00`). Sound ids `0x2B3` = 691, `0x2B4` = 692; message
`0x8082E0B0`.

### Exact behaviour

Prologue `0x806E3090..0x806E30B8`: frame `0x50`; save LR, `r31..r28`;
`r30 = this`; `r31 = 0x807CBB98`.

1. `0x806E30B8..0x806E3104` - rider gate:
   ```rust
   if read_u32(this + 0x8BC) != 0 {
       let mask = *(0x8042A078 as *const *const u32);
       let hit = (read_u32(this + 0x358) & *mask)
               | (read_u32(this + 0x35C) & *mask.add(1));
       if hit == 0 {
           vcall(this + 0x394, 0x18, 0x8082E0B0);
           return;
       }
   }
   ```
2. `0x806E3108..0x806E312C` - save state and pre-move position:
   ```rust
   let saved = [read_f32(this + 0xAC), read_f32(this + 0xB0),
                read_f32(this + 0xB4)];      // r1+0x28..0x30
   let angle0 = read_s16(this + 0x8C4);      // r28
   let geo    = read_u8(this + 0x8C8);       // r29
   call1(0x806E2E40, this);                  // rider/pre-move step
   ```
3. `0x806E3130..0x806E3160` - clamp the delta:
   ```rust
   let mut d = read_s16(this + 0x8C6);
   if d < -256 { write_s16(this + 0x8C6, -256); d = -256; }
   else if d > 256 { write_s16(this + 0x8C6, 256); d = 256; }
   ```
   The two stores are `li r0,-256` / `li r0,256`; the printed
   `addi r0,r0,-256` is the `rA = 0` alias, not an arithmetic update.
4. `0x806E3164..0x806E31A4` - angle and x step:
   ```rust
   let angle = read_s16(this + 0x8C4) + d;
   write_s16(this + 0x8C4, angle);
   let step = -(d as f32) / 512.0f32;        // 0.5 * 8 * (-d) / 2048
   if geo != 0 {
       call1(0x800C1510, this + 0x7BC);
       call1(0x800C1230, this + 0x7BC);
       let mut v = [0f32; 3];
       call2(0x800C12B0, &mut v, this + 0x7BC);
       write_f32(this + 0xAC, v[0]);
       write_f32(this + 0xB0, v[1]);
   } else {
       write_f32(this + 0xAC, read_f32(this + 0xAC) + step);
   }
   ```
   The `f64` is `(-d)` via `neg r3,r4` + `xoris 0x8000`, then
   `- 0x807CBC00`; the `f32` rounding that follows makes the result
   `-d / 512`.
5. `0x806E31E4..0x806E3204` - cooldown and collision:
   ```rust
   if read_u32(this + 0x8C0) != 0 { write_u32(this + 0x8C0, read_u32(this + 0x8C0) - 1); }
   call1(0x806E26B0, this);
   let blocked = call1_u32(0x806E2B10, this) != 0;
   ```
6. `0x806E3208..0x806E32CC` - blocked arm:
   ```rust
   if blocked {
       write_s16(this + 0x8C4, angle0);
       write_s16(this + 0x8C6, 0);
       write_f32(this + 0xAC, saved[0]);
       write_f32(this + 0xB0, saved[1]);
       write_f32(this + 0xB4, saved[2]);
       call1(0x806E26B0, this);
       if geo != 0 {
           let mut v = [saved[0], saved[1]];
           call2(0x800C12D0, this + 0x7BC, &v);
       }
       if read_u32(this + 0x8C0) == 0
          && read_u32(this + 0x8B8) == 0
          && read_f32(this + 0x8B4) != 0.0
       {
           write_u32(this + 0x8C0, 16);
           let mut out = [0u8; 12];
           call2(0x8006A4B0, &mut out, this + 0xAC);   // three-float copy
           call4(0x80198C30, *(0x80429D60 as *const u32), 0x2B4, &out, 0);
       }
       write_u32(this + 0x8B8, 1);
   } else {
       write_u32(this + 0x8B8, 0);
   }
   ```
7. `0x806E32CC..0x806E3338` - delta publish and moving cue:
   ```rust
   let d = read_s16(this + 0x8C6);
   write_f32(this + 0x8B4, (d as f64 - MAGIC) as f32);
   if d != 0 && read_u32(this + 0x8B8) == 0 {
       let mut out = [0u8; 12];
       call2(0x8006A4B0, &mut out, this + 0xAC);
       call6(0x801995A0, *(0x80429D60 as *const u32), 0x2B3,
             read_u32(this + 0x000), &out, d, 0);
   }
   ```
8. `0x806E3338..0x806E3368` - `fn_806E2BF0(this); fn_806E27A0(this);` and
   epilogue.

### Constants

| symbol | value | use |
|---|---|---|
| `0x807CBBD0` | `0.0f` | "blocked cue already played" test |
| `0x807CBBD4` | `8.0f` | step scale |
| `0x807CBBF4` | `0.5f` | step scale |
| `0x807CBBF8` | `2048.0f` | step divisor |
| `0x807CBC00` | `f64 2^52+2^31` | biased signed `s16 -> f64` |
| `0x8042A078` | two-word mask | rider gate |
| `0x8082E0B0` | BSS message | "no rider" |

**Verdict: ready (high).** No `psq` and no GQR use anywhere; the two
`0x43300000` ladders are the standard biased conversions. `0x806E2B10`,
`0x806E2430` (its probe) and the `0x800Cxxxx` geometry API are named by
address.

---

## 5. `0x80796DC0` - world-map course-type message dispatcher

**Evidence.** `work/ghidra/rel-d_basesNP/export/80796dc0_FUN_80796dc0.c`,
728 bytes, 182 words, synthetic `0x80796DC0`. Body `0x80796DC0..0x80797098`
(ends `blr`); the switch is a jump table, not a `bcctr` tail.

**What it is.** A world-map per-frame dispatcher. It reads the world byte
`0x8042A24D` and the current zone id, maps the zone through `0x800F9840` to a
course-type index `0..10`, and posts one of the engine messages `4`, `8`,
`0xC`, `0x1F`, `0x21`, `0x12`, `0x13` (or skips) depending on that index and
two completion predicates. It is called from `0x80796A5C` and `0x80796CA0`
inside the world-map route code.

### Signature

```rust
pub unsafe fn fn_80796dc0(this: usize)
```

`r3` = world-map object (`r31 = r3` at `0x80796DD8`). Returns nothing. Frame
`0x20`; saves `r27..r31`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x22C` | `u32` | current zone/route id |
| `+0x29C` | object | audio/effect handle (passed to `0x80C56270`) |

Globals: `0x8042A18C` (world manager; its `+0x338C` is the current world
index), `0x8042A1AC` (engine message context), `0x8042A24D` (`u8` world id,
returned by `0x807714F0`). Jump table `0x8081B808` (11 `u32` entries, read from
the image):
`0x80796F64, 0x80796ED8, 0x80796E24, 0x80796E60, 0x80796F84, 0x80797004,
0x80796F14, 0x80797004, 0x80796F34, 0x80796FC4, 0x80796E9C`.

### Exact behaviour

Prologue `0x80796DC0..0x80796E04`: frame `0x20`; save LR, `r27..r31`
(`_savegpr_27`); `r31 = this`; `r29 = *(0x8042A18C)`;
`r28 = call0_u32(0x807714F0) & 0xFF`;
`r27 = call2_u32(0x80775100, r29, read_u32(this + 0x22C))`;
`r30 = call1_u32(0x800F9840, r27)`; `call0(0x8076B0A0)`.

`0x800F9840(zone)` is the course-type classifier read from `main.dol`:
`11` for negative, `0..10` for the zone-range boundaries, so `r30` is the
dispatch index. `0x8076B0A0` clears the cue handles.

1. `0x80796E04..0x80796E24` - `if r30 > 10 { goto default }`; else
   `pc = jump_table[r30]`.
2. `0x80796E24` (case 2) and `0x80796E60` (case 3) and `0x80796E9C` (case 10)
   and `0x80796ED8` (case 1) - "post a numeric id with an effect argument":
   ```rust
   // case 2 -> msg 4; case 3 -> msg 8; case 10 -> msg 8; case 1 -> msg 0xC
   let arg = call2_u32(0x800F9BA0, r28, r27);
   call5(0x801016B0, *(0x8042A1AC as *const u32), MSG, this, arg, 0x80);
   call3(0x80C56270, read_u32(this + 0x29C), 0x10, 0);
   ```
   Cases:
   | index | message | effect reset |
   |---:|---:|---|
   | 1 | `0x0C` | yes |
   | 2 | `0x04` | yes |
   | 3 | `0x08` | yes |
   | 10 | `0x08` | yes |
3. `0x80796F14` (case 6), `0x80796F34` (case 8), `0x80796F64` (case 0) -
   fixed messages with no effect argument:
   - case 0 and case 6: `0x801016B0(ctx, 0x1F, 0, 0, 0x80)`.
   - case 8: `0x801016B0(ctx, 0x21, 0, 0, 0x80)` then
     `0x80C56270(this+0x29C, 0x10, 0)`.
4. `0x80796F84` (case 4) - conditional `0x1F`:
   ```rust
   let a = call2_u32(0x800FAE40, r28, r27);
   if a == 0 || call0_u32(0x800FD2E0) != 0 {
       call5(0x801016B0, ctx, 0x1F, 0, 0, 0x80);
   }
   ```
5. `0x80796FC4` (case 9) - opposite conditional `0x1F`:
   ```rust
   if call2_u32(0x800FAE40, r28, r27) == 0
      && (call0_u32(0x800FB350) & 0xFF) != 0 {
       call5(0x801016B0, ctx, 0x1F, 0, 0, 0x80);
   }
   ```
6. `0x80797004` (default, and cases 5/7) - route-table probe:
   ```rust
   let w = read_u32(0x8042A18C + 0x338C);
   let off = w * 0x6518;
   let rec = call3_u32(0x800F57A0, 0x8042A18C + off + 0x3394,
                       read_u32(this + 0x22C), off + 0x400);
   if rec != 0 {
       let hit = call2_u32(0x8079A4B0, this, read_u32(this + 0x22C)) != 0;
       call5(0x801016B0, ctx, if hit { 0x13 } else { 0x12 }, 0, 0, 0x80);
   }
   ```
   `0x800F57A0` is the route-table mask probe documented in
   `docs/SPEC_WAVE141_BASES.md:819-824`.
7. `0x80797080..0x80797098` - epilogue (`_restgpr_27`, LR, `blr`).

### Constants

| symbol | value | use |
|---|---|---|
| `0x8081B808` | 11-entry table | `r30 -> handler` |
| `0x801016B0` | message API | `(ctx, id, arg3, arg4, 0x80)` |
| `0x8042A1AC` | message context | `r3` of `0x801016B0` |
| `0x8042A24D` | `u8` world id | from `0x807714F0` |
| `0x8042A18C` | world manager | `+0x338C` world index |
| `0x6518` | record stride | `0x8042D520 + w*0x6518` |
| `0x400` | probe mask | last argument to `0x800F57A0` |

Message ids: `0x04`, `0x08`, `0x0C`, `0x12`, `0x13`, `0x1F`, `0x21`.

**Verdict: ready (high).** The 11-entry jump table, every message id and every
predicate branch are read from the image; `0x80C56270` is a translated
`d_en_bossNP` helper and the remaining `0x800Fxxxx` helpers are named by
address (the same family used in waves 141/146/147).

---

## 6. `0x806D29C0` - `Wm_ob_cmnsparkloop` initialiser

**Evidence.** `work/ghidra/rel-d_basesNP/export/806d29c0_FUN_806d29c0.c`,
724 bytes, 181 words, synthetic `0x806D29C0`. Body `0x806D29C0..0x806D2C94`
(ends `blr`). Ghidra's `Removing unreachable block (ram,0x806d2c70)` is the
`psq_l f31` restore.

**What it is.** The constructor/initialiser of the world-map object whose vtable
is `0x807FAC40` and whose name string is `Wm_ob_cmnsparkloop`
(`0x807FAC24`). It stamps a spawned actor's packed parameter word (`+0x04`)
into the object: phases, a loop scale (`+0x774`), two variants selected by
fields `4..5` and `12..13`, a `+0x766/+0x768` halfword pair, a 3D loop built
through `0x800C10F0`/`0x800C1560`/`0x800C1580`, and one of two BSS state
messages. It returns `1` on success and `2` when a negative global at
`[0x80429F7C]+0xAF4` vetoes the spawn.

### Signature

```rust
pub unsafe fn fn_806d29c0(this: usize) -> u32
```

`r3` = object (`r29 = r3` at `0x806D29E8`). Returns `1` (r3 at
`0x806D2C68`), or `2` from the abort arm (`0x806D2AC4`). Frame `0x40`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | packed spawn word (see bit map below) |
| `+0x0AC`/`+0x0B0`/`+0x0B4` | `f32` | position |
| `+0x0DC`/`+0x0E0`/`+0x0E4` | `f32` | `1.0` phase/scale seeds |
| `+0x104` | `u16` | cleared |
| `+0x318` | `f32` | `410.0` (r31+0x6C) |
| `+0x31C` | `f32` | `16.0` (r31+0x4C) |
| `+0x320`/`+0x324` | `f32` | `0.0` |
| `+0x394` | interface | state-message sink, vtable slot `+0x18` |
| `+0x3D0` | object | animation/phase container |
| `+0x41A` | `u8` | copy of `0x80429D28` |
| `+0x430` | `s8` | `-1` |
| `+0x434` | `u32` | `1` |
| `+0x438` | `f32` | copy of `+0x774` |
| `+0x43C` | `u32` | `0` |
| `+0x65C` | object | 3D loop container (`0x800Cxxxx`) |
| `+0x748` | `u32` | bit 8 of `+0x04` |
| `+0x766`/`+0x768` | `u16` | `(bits 16..19 ?: 5) << 6` |
| `+0x76A` | `u8` | variant (bits 4..5) |
| `+0x76B` | `u8` | variant (bits 12..13) |
| `+0x76C` | `u8` | second variant copy (`1` or `[0x80429CA0]`) |
| `+0x76D`/`+0x76E`/`+0x76F` | `u8` | cleared |
| `+0x770` | `u8` | bit flags, `0x80` cleared (`& 0x7F`) |
| `+0x774` | `f32` | loop scale |

Bit map of `+0x004` (Ghidra's `rlwinm` extraction, MSB indexing):

| field | extraction | use |
|---|---|---|
| bits 0..3 | `packed & 0xF` | loop count -> `f31` |
| bits 4..5 | `rlwinm rS,28,30,31` = `(packed >> 4) & 3` | `+0x76A`, and message choice (`== 2`) |
| bit 8 | `rlwinm rS,24,31,31` = `(packed >> 8) & 1` | `+0x748` |
| bits 12..13 | `rlwinm rS,20,30,31` = `(packed >> 12) & 3` | `+0x76B`, `+0x76C` (`== 2` / `== 3`) |
| bits 16..19 | `rlwinm rS,16,28,31` = `(packed >> 16) & 0xF` | `+0x766`/`+0x768`, `0 -> 5` |
| bit 20 | `rlwinm rS,12,31,31` = `(packed >> 20) & 1` | veto check |

### Exact behaviour

Prologue `0x806D29C0..0x806D29F0`: frame `0x40`; save LR, `f31` + `psq_st`,
`r31..r28`; `r29 = this`; `r31 = 0x807CB560`.

1. `0x806D29F0..0x806D29FC` - `call1(0x806D31B0, this); call1(0x80043CA0, this);`
   then `packed = read_u32(this + 4)`.
2. `0x806D2A00..0x806D2A68` - loop scale:
   ```rust
   let count = (packed & 0xF) as f64;                 // biased ladder
   write_f32(this + 0xDC, 1.0); write_f32(this + 0xE0, 1.0);
   write_f32(this + 0xE4, 1.0);
   write_u32(this + 0x748, 0);
   let f31 = 16.0f32 * (7.0f32 + count as f32);       // 112..352
   let v   = 0.5f64 * (-(f31 as f64)) - 4.0f64;       // f64
   write_f32(this + 0x774, v as f32);
   write_u8(this + 0x41A, read_u8(0x80429D28));
   ```
   The `f64` constants are `0x807CB5B0 = 0.5` and `0x807CB5B8 = 4.0`.
3. `0x806D2A90..0x806D2AA4` - phase seed:
   ```rust
   call9_f64(0x800DB180, /*f1*/ 0.0, /*f2*/ 0.0, /*f3*/ 8.0,
             /*f4*/ -(v as f64), this + 0x3D0, this, 0, 1, 0);
   write_f32(this + 0x438, read_f32(this + 0x774));
   write_u32(this + 0x434, 1);
   ```
   Argument order is the raw one: `f1..f4` first, then `r3 = this+0x3D0`,
   `r4 = this`, `r5 = 0`, `r6 = 1`, `r7 = 0`. The `0x800DB180` shape
   (four `f64` + five words) is the precedent recorded in
   `docs/SPEC_WAVE129_BASES.md:1234-1239`.
4. `0x806D2AA8..0x806D2AC8` - veto:
   ```rust
   if (packed & 0x100000) != 0 {
       if read_i32(*(0x80429F7C as *const u32) + 0xAF4) < 0 { return 2; }
   }
   ```
5. `0x806D2AC8..0x806D2B00` - fixed state:
   ```rust
   write_u8(this + 0x430, 0xFF);
   write_u32(this + 0x43C, 0);
   write_f32(this + 0x320, 0.0); write_f32(this + 0x324, 0.0);
   write_f32(this + 0x318, 410.0); write_f32(this + 0x31C, 16.0);
   write_u16(this + 0x104, 0);
   call1(0x800DAE80, this + 0x3D0);
   ```
6. `0x806D2B00..0x806D2B38` - variant A:
   ```rust
   write_u8(this + 0x770, read_u8(this + 0x770) & 0x7F);   // clears 0x80
   let a = (packed >> 4) & 3;
   if a == 0 { write_u8(this + 0x76A, 1); }
   else if a == 1 { write_u8(this + 0x76A, read_u8(0x80429CA0)); }
   ```
7. `0x806D2B38..0x806D2B7C` - variant B:
   ```rust
   write_u32(this + 0x748, (packed >> 8) & 1);
   write_u8(this + 0x76E, 0);
   let b = (packed >> 12) & 3;
   write_u8(this + 0x76B, b as u8);
   if b == 2 { write_u8(this + 0x76C, 1); }
   else if b == 3 { write_u8(this + 0x76C, read_u8(0x80429CA0)); }
   ```
8. `0x806D2B7C..0x806D2BC4` - timer halfwords and loop seed:
   ```rust
   let mut n = (packed >> 16) & 0xF;
   if n == 0 { n = 5; }
   let h = (n << 6) as u16;
   write_u16(this + 0x766, h); write_u16(this + 0x768, h);
   write_u8(this + 0x76D, 0); write_u8(this + 0x76F, 0);
   let mut p = [read_f32(this + 0xAC), read_f32(this + 0xB0)];
   // 0x800C10F0(r3 = this+0x65C, r4 = &p, r5 = 1, r6 = 0, f1 = 0.0)
   call5_f32(0x800C10F0, this + 0x65C, &p, 1, 0, /*f1*/ 0.0);
   ```
9. `0x806D2BC8..0x806D2C20` - the two loop arcs:
   ```rust
   call3(0x800C1560, this + 0x65C, &[0.0, f31 * 0.5 - 4.0], 1);
   call3(0x800C1560, this + 0x65C, &[0.0, -f31 * 0.5 + 4.0], 2);
   call1(0x800C1580, this + 0x65C);
   ```
10. `0x806D2C24..0x806D2C68` - state message and return:
    ```rust
    if a == 2 { vcall(this + 0x394, 0x18, 0x8082D610); }
    else      { vcall(this + 0x394, 0x18, 0x8082D5D0); }
    return 1;
    ```
11. `0x806D2C6C..0x806D2C94` - epilogue (`psq_l f31`, `lfd f31`, restores,
    LR, `blr`).

### Constants

| symbol | value | use |
|---|---|---|
| `0x807CB5C0` | `1.0f` | `+0xDC..+0xE4` seeds |
| `0x807CB5A8` | `7.0f` | loop base |
| `0x807CB5AC` | `16.0f` | loop scale |
| `0x807CB5B0` | `f64 0.5` | loop scale |
| `0x807CB5B8` | `f64 4.0` | loop offset |
| `0x807CB5C4` | `8.0f` | `0x800DB180` third float |
| `0x807CB5C8` | `0.0f` | `+0x320/+0x324`, arc x |
| `0x807CB5CC` | `410.0f` | `+0x318` |
| `0x807CB5D0` | `0.5f` | arc scale |
| `0x807CB5D4` | `4.0f` | arc offset |
| `0x807CB5D8` | `f64 2^52` | unsigned `u32 -> f64` |
| `0x80429CA0` | `u8` | variant copy source |
| `0x80429D28` | `u8` | `+0x41A` |
| `0x80429F7C` | global object | `+0xAF4` veto |
| `0x8082D5D0`/`0x8082D610` | BSS messages | variant A/B |

**Verdict: ready (high).** Retires the wave-28 "paired-single reconstruction"
rejection: there is no `psq` word at all, and `fsub f0,f0,f4` is the double
subtract hand-decoded above. The `0x800Cxxxx` geometry API is named by address.

---

## 7. `0x8079A040` - world-map course-id selector

**Evidence.** `work/ghidra/rel-d_basesNP/export/8079a040_FUN_8079a040.c`,
716 bytes, 179 words, synthetic `0x8079A040`. Body `0x8079A040..0x8079A30C`
(ends `blr`); the 8-way switch is a jump table (`0x8081B854`).

**What it is.** The world-map "which course is the player standing on" resolver.
It runs once, only while `[0x80429F7C]+0x48 == -1` (no course selected yet),
switches on the world/map node byte `0x8043A24D`, builds a five-character course
name with `0x800F9370` (or uses one of the fixed strings `W1X0`, `F001`,
`F101`), resolves it to a course id with `0x801002C0`, and stores the result at
`+0x48`. Any value other than `-1` is preserved on later frames.

### Signature

```rust
pub unsafe fn fn_8079a040(this: usize)
```

`r3` = world-map object (`r30 = *(0x8042A18C)` is the lookup manager; the
argument itself is not dereferenced). Returns nothing. Frame `0x30`; saves
`r26..r31`.

### Field map

| symbol | meaning |
|---|---|
| `0x80429F7C` -> `+0x48` | `s32` selected course id, `-1` = unset |
| `0x8042A18C` | world manager passed as `r3` to `0x801002C0` |
| `0x8043A24D` | `u8` map node / world selector |
| `0x8083273C` | `u32` global used by case 5 (`1..3` -> `"F101"`, `0` -> generated name) |
| `0x8081B854` | 8-entry jump table (`0x8079A09C, A0C0, A134, A158, A17C, A254, A2AC, A2D0`) |
| `0x8081B84C` | `"F101"` |
| `0x8081B874` | `"W1X0"` |
| `0x8081B87C` | `"F001"` |

### Exact behaviour

Prologue `0x8079A040..0x8079A070`: frame `0x30`; save LR, `r26..r31`
(`_savegpr_26`); `r29 = *(0x80429F7C)`; `r30 = *(0x8042A18C)`.

1. `0x8079A064..0x8079A070` - `if read_i32(r29 + 0x48) != -1 { goto epilogue }`.
2. `0x8079A070..0x8079A098` - `let sel = read_u8(0x8043A24D); r28 = -1;
   if sel > 7 { goto store } else { pc = table[sel] }`.
3. `0x8079A09C` (case 0) - `name = 0x800F9370(0, 3, &buf);
   r28 = 0x801002C0(mgr, &buf)`.
4. `0x8079A0C0` (case 1) - two-stage:
   ```rust
   let a = call2_u32(0x800FAD60, 1, 4);          // r31
   let b = call2_u32(0x800FAD60, 1, 5);
   if b != 0 {
       call3(0x800F9370, 1, 5, &buf);
       r28 = call2_u32(0x801002C0, mgr, &buf);
   } else if a != 0 {
       call3(0x800F9370, 1, 4, &buf);
       r28 = call2_u32(0x801002C0, mgr, &buf);
   }
   ```
5. `0x8079A134` (case 2) - `0x800F9370(2, 1, &buf); r28 = 0x801002C0(mgr, &buf)`.
6. `0x8079A158` (case 3) - `0x800F9370(3, 0, &buf); r28 = 0x801002C0(mgr, &buf)`.
7. `0x8079A17C` (case 4) - actor scan:
   ```rust
   let mut found = -1i32;                        // r27
   for i in 0..4u32 {                            // r26
       let a = call1_u32(0x80793BC0, i);
       if a != 0 {
           let v = vcall_u32(a, 0x70);           // *(a+0x60)->+0x70
           if v == read_u32(*(0x8042A1A0 as *const u32) + 0x22C) {
               found = i as i32;
               break;
           }
       }
   }
   if found == 0 {
       call3(0x800F9370, 4, 0, &buf);
       r28 = call2_u32(0x801002C0, mgr, &buf);
   } else if found == 1 {
       call3(0x800F9370, 4, 0x14, &buf);
       r28 = call2_u32(0x801002C0, mgr, &buf);
   } else if found == 2 {
       call3(0x800F9370, 4, 4, &buf);
       r28 = call2_u32(0x801002C0, mgr, &buf);
   }
   ```
   `0x80793BC0(i)` is the "first actor whose kind low nibble == i" scan
   (export `80793bc0_FUN_80793bc0.c`); the virtual call is
   `lwz r12,0x60(a); lwz r12,0x70(r12); mtctr; bctrl` with `r3 = a`.
8. `0x8079A254` (case 5):
   ```rust
   let v = read_u32(0x8083273C);
   if v.wrapping_sub(1) <= 2 {                  // v in 1..3
       r28 = call2_u32(0x801002C0, mgr, 0x8081B84C);   // "F101"
   } else if v == 0 {
       call3(0x800F9370, 5, 2, &buf);
       r28 = call2_u32(0x801002C0, mgr, &buf);
   }
   ```
   The `cmplwi r0,2` at `0x8079A260` is unsigned, so `v == 0` is handled by the
   second test (`cmpwi r3,0; bne store`).
9. `0x8079A2AC` (case 6) - `0x800F9370(6, 0x14, &buf); ...`.
10. `0x8079A2D0` (case 7) - `0x800F9370(7, 2, &buf); ...`.
11. `0x8079A2F0..0x8079A30C` - `write_i32(r29 + 0x48, r28)` and epilogue.

`0x800F9370(level, area, buf)` writes `buf[0] = 'W'`, `buf[1] = level + '1'`
(or `'A'` for level 9), then two digits chosen from `0x800F9840(area)`; it is
the NSMBW course-code builder (export
`work/ghidra/main-gb/export/800f9370_FUN_800f9370.c`). `0x801002C0` is the
`0x800F53A0` tail call at the `+0x3394` world offset whose `r3` is the course
id (export is `void`; the caller keeps `r3` - precedent
`docs/SPEC_WAVE142_BASES.md:425-433`).

### Constants

| symbol | value | use |
|---|---|---|
| `0x8081B854` | 8-entry table | case dispatch |
| `0x8081B84C` | `"F101"` | case 5 fixed name |
| `0x8081B874` | `"W1X0"` | (already used at `docs/SPEC_WAVE140_BASES.md:221`) |
| `0x8081B87C` | `"F001"` | alternate fixed name |
| `0x8043A24D` | `u8` | map node selector 0..7 |
| `0x8083273C` | `u32` | case-5 variant |
| `0x8042A1A0` | global object | `+0x22C` actor id for the case-4 scan |

**Verdict: ready (high).** All eight case targets, all three fixed strings and
the generated-name ranges are read from the image; `0x800F9370`/`0x801002C0`
are the already-specified course-name/course-id pair, and `0x80793BC0` is
exported.

---

## 8. `0x8064F0A0` - `daBgmInterlockingDummyBlock_c` animation attach / clip poll

**Evidence.** `work/ghidra/rel-d_basesNP/export/8064f0a0_FUN_8064f0a0.c`,
708 bytes, 177 words, synthetic `0x8064F0A0`. Body `0x8064F0A0..0x8064F364`
(ends `blr`). No Ghidra warnings, no `psq`, no `stfd`.

**What it is.** The sub-state body of the interlocking dummy block
(`daBgmInterlockingDummyBlock_c`, strings at `0x807E4D14`). While its state
word `+0x360` is `1` it binds the player's animation into one of two block
handles (`+0x584` or `+0x5BC`) according to the player's two handedness bytes,
then posts a BSS state message through the class vtable slot `+0xD4`; while the
state is `2` it polls the same handle's clip-finished predicate
`0x80169020` and discards the result. The "player" is the global pointer
`0x808283E8`; its `+0x3E4..+0x3F9` block supplies the blend weight, the
"holding" flags and the two hand selectors.

### Signature

```rust
pub unsafe fn fn_8064f0a0(this: usize)
```

`r3` = block actor (`r30 = r3` at `0x8064F0B8`). Returns nothing. Frame `0x20`;
saves `r30`, `r31`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x360` | `u32` | state: `1` = attach, `2` = poll clip |
| `+0x544` | object | model/animation owner (target of the blend) |
| `+0x584` | object | handle A (right-hand pair) |
| `+0x5BC` | object | handle B (left-hand pair) |
| `+0x5F4` | `u32` | animation id for A |
| `+0x5F8` | `u32` | animation id for B |
| `+0x5FC` | `f32` | copy of the player's `+0x3EC`/`+0x3F0` value |
| `+0x600` | `u32` | sub-state: `0` = first pair, `1` = second pair |

Player globals: `0x808283E8` (`u32` pointer), its `+0x3E4`/`+0x3E8` (blend
`f32`), `+0x3EC`/`+0x3F0` (speed `f32`), `+0x3F4`/`+0x3F5` (hand bytes for
state 2), `+0x3F8`/`+0x3F9` (both-hands-held bytes for state 1). Constants:
`0x807C8708` = `0.0f`. BSS messages `0x808282F8` and `0x80828338`.

### Exact behaviour

Prologue `0x8064F0A0..0x8064F0C4`: frame `0x20`; save LR, `r31`, `r30`;
`r30 = this`; `r31 = *(0x808283E8)`.

1. `0x8064F0BC..0x8064F0D4` - dispatch:
   ```rust
   match read_u32(this + 0x360) {
       1 => step 2,
       2 => step 6,
       _ => return,
   }
   ```
2. `0x8064F0D8..0x8064F104` - state 1, both-hands gate:
   ```rust
   if r31 == 0 { return; }
   let mut both = read_u8(ptr + 0x3F8) != 0;
   if both { both = read_u8(ptr + 0x3F9) != 0; }
   if !both { return; }
   ```
   The two tests are `lbz`/`cmpwi`/`beq` pairs; any zero short-circuits.
3. `0x8064F108..0x8064F1FC` - state 1, first sub-state
   (`read_u32(this+0x600) == 0`):
   ```rust
   if r31 == 0 { return; }
   if read_u8(ptr + 0x3F4) == 0 {
       attach(this + 0x5BC, this + 0x544, &read_u32(this + 0x5F8));
       blend(this + 0x544, this + 0x5BC, /*f1*/ 0.0);
       weight(this + 0x5BC, /*f1*/ read_f32(ptr + 0x3E4));
       write_f32(this + 0x5FC, read_f32(ptr + 0x3EC));
       commit(this + 0x5BC);
       vcall(this, 0x00D4, 0x80828338);
   } else {
       attach(this + 0x584, this + 0x544, &read_u32(this + 0x5F4));
       blend(this + 0x544, this + 0x584, /*f1*/ 0.0);
       weight(this + 0x584, /*f1*/ read_f32(ptr + 0x3E4));
       write_f32(this + 0x5FC, read_f32(ptr + 0x3EC));
       commit(this + 0x584);
       vcall(this, 0x00D4, 0x808282F8);
   }
   ```
   The helper shapes are
   `0x801651F0(target, model, &anim_id, 1)` (load),
   `0x80169F80(model, target, f1 = 0.0)` (blend start),
   `0x80168FE0(target, f1)` (stores `f1` at `target+0x30`),
   `0x801680E0(target)` (commit/tick), all from
   `work/ghidra/main-gb/export/`.
4. `0x8064F200..0x8064F2F0` - state 1, second sub-state
   (`read_u32(this+0x600) == 1`):
   ```rust
   if r31 == 0 { return; }
   if read_u8(ptr + 0x3F5) != 0 {
       attach(this + 0x5BC, this + 0x544, &read_u32(this + 0x5F8));
       blend(this + 0x544, this + 0x5BC, 0.0);
       weight(this + 0x5BC, /*f1*/ read_f32(ptr + 0x3E8));
       write_f32(this + 0x5FC, read_f32(ptr + 0x3F0));
       commit(this + 0x5BC);
       vcall(this, 0x00D4, 0x80828338);
   } else {
       attach(this + 0x584, this + 0x544, &read_u32(this + 0x5F4));
       blend(this + 0x544, this + 0x584, 0.0);
       weight(this + 0x584, /*f1*/ read_f32(ptr + 0x3E8));
       write_f32(this + 0x5FC, read_f32(ptr + 0x3F0));
       commit(this + 0x584);
       vcall(this, 0x00D4, 0x808282F8);
   }
   ```
5. `0x8064F2F4..0x8064F34C` - state 2, clip poll:
   ```rust
   if read_u32(this + 0x600) == 0 {
       let h = if read_u8(ptr + 0x3F4) != 0 { this + 0x5BC }
               else { this + 0x584 };
       call1(0x80169020, h);                    // bool result discarded
   } else if read_u32(this + 0x600) == 1 {
       let h = if read_u8(ptr + 0x3F5) != 0 { this + 0x5BC }
               else { this + 0x584 };
       call1(0x80169020, h);
   }
   ```
   `0x80169020` is the clip-finished predicate (it compares the clip frame
   against `unaff_r2-0x53d0`/`-0x53c8` and the `+0x34` mode byte); this body
   ignores its return. Unlike steps 3-4 there is **no** null test on `r31`
   before the two `lbz` reads in this arm; if `0x808283E8` were zero this path
   would fault. It is reproduced literally and flagged, not "fixed".
6. Any other `+0x360` value falls through to the epilogue
   (`0x8064F0D4`/`0x8064F34C`).
7. `0x8064F34C..0x8064F364` - epilogue.

`vcall(this, 0x00D4, msg)` is `(*(this+0x60)->slot_0xD4)(this, msg)`; the load
pair is `lwz r12,96(r30); lwz r12,212(r12); mtctr r12; bctrl`
(`0x8064F174`, `0x8064F268`).

### Constants

| symbol | value | use |
|---|---|---|
| `0x807C8708` | `0.0f` | `0x80169F80` blend start |
| `0x808283E8` | `u32` pointer | player object |
| `0x808282F8` | BSS message | posted with the `+0x584` arm (byte `0x3F4`/`0x3F5` clear) |
| `0x80828338` | BSS message | posted with the `+0x5BC` arm (byte `0x3F4`/`0x3F5` set) |

### Callees

| target | shape | note |
|---|---|---|
| `0x801651F0` | `(target, model, &anim_id, 1)` | bind animation |
| `0x80169F80` | `(model, target, f1)` | start blend |
| `0x80168FE0` | `(target, f1)` | store `f1` at `target+0x30` |
| `0x801680E0` | `(target)` | commit/tick |
| `0x80169020` | `(target) -> bool` | clip-finished poll (return discarded) |
| `+0x60` -> `+0xD4` | `(this, msg)` | class state message |

**Verdict: ready (high).** Every handle offset, weight source and message is
literal; the five animation helpers are exported in `main-gb`; the one
null-check asymmetry is reproduced and documented.

---

## Cross-checks and re-reads

* **Row 1 vs `docs/SPEC_WAVE141_BASES.md:541-647`.** Same object, same
  `+0x1A488`/`+0x1A59C` pair, same `0x80809620` curve block, same
  `0x8082FFE4`/`0x8082FFEC` globals, same `0x808300C0` family. Row 1 adds the
  per-frame fill weight (`75`/`216`/`8`) and the `0x553` cue.
* **Row 2 vs `docs/SPEC_WAVE129_BASES.md:1249-1341`.** The same pose-record
  selector shape with the global table `0x8031D634`; the `0x80103410` argument
  forms match item 7 of that section (the four integer slots shift by one).
  Row 2 also confirms the table's two floats are exactly what `0x80799960`
  feeds to the `+0x5C` transition call.
* **Row 3 vs `docs/SPEC_WAVE146_BASES.md:926-1071`.** Same tail call set in the
  same order; this variant is the one without the `+0x360` gate and therefore
  calls `0x8069ACA0` whenever `+0x500` is zero.
* **Row 4 vs `docs/COVERAGE_WAVE10_BASES.md:190` and
  `docs/COVERAGE_WAVE80_BASES.md:142-144`.** The class attribution is
  confirmed and the float-lane deferral is retired.
* **Row 6 vs `docs/COVERAGE_WAVE28_BASES.md:88,191`.** The rejection is
  retired: no `psq`, no GQR.
* **Row 7 vs `docs/SPEC_WAVE140_BASES.md:221`.** Both use the same
  `0x801002C0(mgr, name)` course-id lookup; row 7 is the one-shot selector that
  fills `[0x80429F7C]+0x48`, while that section's `0xA0` block re-selects
  `+0x22C`.

## Verification summary

* 8/8 rows: TSV `decompiled=true`, `size` matches the decoded span, exactly one
  `blr` at `address + size - 4`, every branch target inside the body.
* 8/8 rows: `.c` export exists at the TSV `file` name.
* 8/8 rows: absent from `ledger/functions.csv`, from every `#{1,6} ` heading in
  `docs/*.md`, and from `src/game/bases.rs` (`fn_<addr>` token).
* 8/8 bodies: SHA-1 unique among all same-size `d_basesNP` rows.
* 0 conditional rows; 2 hand-decoded instruction words (`divw`, double `fsub`),
  both documented above.
* Next eight re-derived after this wave:
  `0x80775D00` (708), `0x80626DB0` (704), `0x8070F370` (692),
  `0x806D1EF0` (684), `0x807BC310` (684), `0x8078FC80` (680),
  `0x807A6BE0` (680), `0x80680FD0` (676).
