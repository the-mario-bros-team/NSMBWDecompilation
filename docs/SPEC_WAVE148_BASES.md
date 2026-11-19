# Wave 148 - `game::bases` - the next eight (0.75-0.80 KB band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified. The only file written is this one. No merge or
check tool was run. The throwaway decoder and the ELF/literal readers live under
`%TEMP%\opencode\w148` (`ppc.js`, `cand.js`, `dis.js`, `data.js`, `tables.js`,
`tbl2.js`, `tbl3.js`, `scanstr.js`, `scanstr2.js`, `verify.js`), never in the
repository.

Waves 128-147 walked this bucket from the top in size order; the last row
claimed by `docs/SPEC_WAVE147_BASES.md` is 812 bytes. This wave continues one
rung lower, **804 bytes down to 764 bytes**, which is the eight-row set
`docs/SPEC_WAVE147_BASES.md:88-90` predicted verbatim. Every row is decoded
instruction by instruction, gets a field map, a numbered behaviour list, a
constant table, its callees and an exact signature.

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x806DA520` | 804 | 201 | `806da520_FUN_806da520.c` | ready (high) |
| 2 | `0x8071E130` | 788 | 197 | `8071e130_FUN_8071e130.c` | ready (high) |
| 3 | `0x80788CB0` | 788 | 197 | `80788cb0_FUN_80788cb0.c` | ready (high) |
| 4 | `0x80763290` | 780 | 195 | `80763290_FUN_80763290.c` | ready (high) |
| 5 | `0x80794C00` | 780 | 195 | `80794c00_FUN_80794c00.c` | ready (high) |
| 6 | `0x8075F270` | 768 | 192 | `8075f270_FUN_8075f270.c` | ready (high) |
| 7 | `0x8066DDC0` | 764 | 191 | `8066ddc0_FUN_8066ddc0.c` | ready (high) |
| 8 | `0x807727D0` | 764 | 191 | `807727d0_FUN_807727d0.c` | ready (high) |

**Ready: 8. Conditional: 0.**

Every `bytes` value is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim and `decompiled` is
`true` for all eight. In all eight the size is the true instruction span: the
decoder found exactly one `blr`, at `address + size - 4`
(`0x806DA840`, `0x8071E440`, `0x80788FC0`, `0x80763598`, `0x80794F08`,
`0x8075F56C`, `0x8066E0B8`, `0x80772AC8`), and no earlier one. There is no
padding tail, no `halt_baddata` cut and no unconditional branch out of the
extent: **1** of the 1,559 words in the eight bodies is outside the throwaway
decoder's table - the `divw` at `0x80772944` (row 8, `XO = 491`), hand-decoded
below - and every `b` target is inside its own body.

### Why all eight are ready

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image, every callee is named by address (or by its ledger name), and
  the body can be written as code with no assumed calling convention beyond the
  standard PowerPC EABI. Data in `main.dol` BSS (`0x8035xxxx`, `0x8037xxxx`,
  `0x8042xxxx`), in this REL's BSS (`0x80822900..0x80834D84`), or in another
  REL is an *input*, named by address.
* **conditional** - exactly one named input would still be missing.

Six of the eight needed the wave-127 GQR note checked. The raw stream answers it
for all six:

* row 1 has one `psq_st f31,72(r1),qr0,0` / `psq_l` pair - the plain `f31`
  spill - and no data-side `psq` at all.
* row 2 has two `stfd`/`psq_st ...,qr0,0` pairs (`f31`, `f30`) and two matching
  restores.
* row 3 has one `f31` pair.
* row 4 has three pairs (`f31`, `f30`, `f29`) and three restores; row 5 is the
  same shape.
* row 7 has **seven** `stfd`/`psq_st` pairs (`f31` down to `f25`) and seven
  restores - the full `f25..f31` float save - and **no** data-side `psq` word.
* rows 6 and 8 contain no `psq` word at all, so their saves are GPR-only.

`qr0` is type 0 (plain `f32` pair) per `docs/REL_ELF_MAPPING.md:53-55`, so every
one of these brackets is a register save, not a quantised load.

### Two earlier rejections retired

* **`0x806DA520`** was rejected by `docs/COVERAGE_WAVE94_BASES.md:115-121`
  ("paired-single reconstruction ... the register class and the float lanes
  cannot be proven") and left uncalled. The raw body has exactly one `psq` pair,
  it is the `f31` prologue save, `qr0` is type 0, and the Ghidra
  "unreachable block" is `0x806DA828`, the `psq_l` restore half. The body is
  ordinary scalar single-precision code and is specified in full below.
* **`0x8066DDC0`** was rejected by `docs/COVERAGE_WAVE126_BASES.md:112-118`
  ("`unaff_GQR0`/`unaff_GQR3`, `ldexpf` and `CONCAT13`/`CONCAT22` lane splits")
  together with `0x8066E0C0`. Row 7 contains seven `qr0` save pairs and no
  `GQR3` use at all: there is no `mfspr` on any `gqr`, no `psq` with `qr3`, and
  no data-side `psq`. Same rejection class as `0x807373B0` in wave 147.

### This is the list wave 147 predicted, exactly

Re-deriving the candidate set from the current tree gives **1,901** rows: the
5,615 `functions.tsv` rows, filtered against the 16,941 distinct addresses in
`ledger/functions.csv` and the 1,529 distinct `0x`/bare addresses that already
appear in a `#{1,6} ` heading of `docs/*.md`, and against `decompiled=true`.
Wave 147 published 1,928 before the wave-147/148 merges; the 27-row difference
is the addresses that translation lane has since claimed. The first eight are
byte-for-byte `docs/SPEC_WAVE147_BASES.md:88-90`:

```
0x806DA520 804 | 0x8071E130 788 | 0x80788CB0 788 | 0x80763290 780
0x80794C00 780 | 0x8075F270 768 | 0x8066DDC0 764 | 0x807727D0 764
```

The 788-byte tie is broken by address ascending (`0x8071E130` before
`0x80788CB0`), and the two 764-byte rows are ordered `0x8066DDC0` then
`0x807727D0`, exactly as wave 147 said they would be. Each of the eight was
re-tested against `ledger/functions.csv` (exact `address` field), against every
`#{1,6} ` heading in `docs/*.md` (including the bare and lowercase spellings),
and against `src/game/bases.rs` (`pub unsafe fn fn_<addr>` / any `fn_<addr>`
token absent). All eight are absent from all three, and each has a
`decompiled=true` TSV row with a matching `.c` export on disk. SHA-1 of the
eight bodies against every same-size `d_basesNP` row: **no duplicate**.

Applying the same filter after this wave, the next eight are:
`0x8071DCC0` (744), `0x80798F10` (744), `0x8069C780` (740), `0x806E3090` (728),
`0x80796DC0` (728), `0x806D29C0` (724), `0x8079A040` (716), `0x8064F0A0` (708).

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

**Decoder.** Reused the wave-146 throwaway decoder
(`%TEMP%\opencode\w148\ppc.js`) and re-validated it against the facts
`docs/REL_ELF_MAPPING.md` pins: branch `LK` is bit 0 and `AA` bit 1;
conditional branches use the 14-bit `BD`; the `SPR` field is
`((word >> 11) & 0x1F) << 5 | ((word >> 16) & 0x1F)`; `psq_l` = 56 and
`psq_st` = 60 with the quantiser in bits 12-14 and `W` in bit 15; single-float
`fmuls`/`fdivs` use `FRC`. Two decoder notes for this pass:

* `mtctr` is `mtspr 9, rS` (`opcode 31`, `XO = 467`); every virtual call in
  these eight appears as `lwz r12,slot(r3); lwz r12,off(r12); mtctr r12; bctrl`,
  and the slot offsets below are read from those two loads.
* `XO = 491` (`opcode 31`) is `divw`. The reused table does not carry it, so row
  8 printed `x31_unk 0x1eb` at `0x80772944`; the word is `0x7C03B3D6`, i.e.
  `divw r0, r3, r22` (rD = 0, rA = 3, rB = 22, `XO = 0x1EB`), confirmed bit by
  bit. There are no other gaps.
* Several listings print `addi rX,r0,imm`; when the `rA` field is 0 that is
  `li rX,imm`. Rows 2, 3 and 7 rely on this in the state assignments.

**Receiver recovery.** Rows 1, 2, 7 and 8 open with a `bl` to one of the
CodeWarrior `_savegpr_N` helpers. Those helpers are 4-byte stubs in `main.dol`
and the exports of `0x802DCD40..0x802DCD74` prove which registers each one
saves (`_savegpr_21` starts at `0x802DCD5C`, `_savegpr_22` at `0x802DCD60`,
`_savegpr_25` at `0x802DCD6C`, `_savegpr_26` at `0x802DCD70`; the stride is 4
bytes from `_savegpr_14` at `0x802DCD40`). They write through `r11` and never
touch `r3`, so the incoming `this` survives:

| row | prologue | helper | registers saved | `this` recovered into |
|--:|---|---|---|---|
| 1 | `0x806DA538` | `_savegpr_25` | `r25..r31` | `r30` |
| 2 | `0x8071E150` | `_savegpr_26` | `r26..r31` | `r30` |
| 7 | `0x8066DE08` | `_savegpr_22` | `r22..r31` | `r23` |
| 8 | `0x807727E0` | `_savegpr_21` | `r21..r31` | `r24` |

The epilogues confirm it (`_restgpr_25` = `0x802DCDB8`, `_restgpr_26` =
`0x802DCDBC`, `_restgpr_22` = `0x802DCDAC`, `_restgpr_21` = `0x802DCDA8`). Rows
3, 4, 5 and 6 have no saved GPRs and take `r3` directly.

**Class attribution (new evidence this wave).** Three of the eight can now be
named from string blobs in the image that no earlier document used:

* `0x807FB960..0x807FBA40` holds
  `"daLiftScales_c::StateID_NoDisp"`, `..._HoldStop`, `..._HoldStopMove`,
  `..._HoldMove`, `..._Still`, `..._Release`, immediately after the
  `{0, -1, handler}` record table at `0x807FB8C0..0x807FB95C` that contains
  `0x806DA520` at `0x807FB910`. Row 1 is therefore a `daLiftScales_c` body - the
  `lift_tenbin` balance lift whose models are named at `0x807FB734`
  (`g3d/lift_tenbin.brres`).
* `0x808098C4..0x80809948` holds `"daObjWendy_c::StateID_Wait"`, `..._Fall`,
  `..._Rise`, `..._Fill`, `..._Drop`, immediately after the record table at
  `0x8080987C..0x808098C4` that contains `0x8071E130` at `0x808098B4`. Row 2 is
  a `daObjWendy_c` body.
* `0x807E9657..0x807E9738` holds the seven
  `"daCastleBossKey_c::StateID_Demo*"` names, and the allocator of the 0x590
  object that owns row 7 uses vtable `0x807E9910` (wave 126). Row 7 is the
  boss-key body.

Rows 4, 5 and 6 are named from their own tables and callers instead (below).

**Unexported callees.** `0x801625D0`, `0x800FC0A0`, `0x800F2A00`,
`0x800FC370`, `0x800F7400`, `0x800FC140`, `0x800FC190`, `0x800F9840`,
`0x800FC7F0`, `0x800FD8A0`, `0x800FD820`, `0x800FD880`, `0x800FD920`,
`0x800F6390` and `0x8006A3F0`/`0x8006A4B0` have no `functions.tsv` row and no
`.c` export. Each was checked against the exported extent that precedes it:
every one starts inside a larger un-analysed `main.dol` gap, and every one is
called with registers the compiler has just set up. They are named by address
only, exactly as `docs/SPEC_WAVE128_BASES.md` and `docs/SPEC_WAVE147_BASES.md`
do.

**Call notation.** Throughout this document `callN(addr, a, b, ...)` lists the
integer argument registers `r3, r4, ...` from left to right and treats the
callee as returning `f32` in `f1` when the value is used as a float. Where a
callee takes a float in `f1` in addition to (or instead of) `r3`/`r4`, the call
is written `addr(f1 = ..., r3 = ...)` explicitly. This matters for
`0x80168FE0` (a `f64` in `f1` plus an element pointer in `r3`) and
`0x8075F9E0` (two `f64`s in `f1`/`f2`), whose Ghidra exports fold the float
into the first C parameter. Where the Ghidra export drops an argument that is
live in the register stream (a recurring defect for the `0x800Fxxxx` and
`0x8016xxxx` families), the argument is recovered from the instruction stream,
written out with the call, and called out in the section.

**Cross-checks against earlier specifications.** Three rows are re-reads of an
already-documented sibling, and this pass matches them:

* row 1 vs `docs/COVERAGE_WAVE94_BASES.md:36-48`: the field map
  (`+0x5A4` offset, `+0x5A8` ceiling, `+0x5AC` current, `+0x5B0` rate, `+0x5B4`
  presence, `+0x5B8/+0x5BC` in the siblings, per-player flag bytes at `+0x584`,
  `+0x590` rider counter) is that document's map for this lift. Row 1 owns the
  `+0x5A4..+0x5B4` quartet plus `+0x5BD`.
* row 2 vs `docs/SPEC_WAVE143_BASES.md:359` (the `0x16518`/`0x3F08` stride
  family): no shared field; the only overlap is the `0x8082FFE4`/`0x8082FFEC`
  pair, which is *this REL's* BSS in both.
* row 8 vs `docs/SPEC_WAVE147_BASES.md:771-775` (the world-map route player):
  row 8 uses the same `+0x338C` mode selector and the same `0x800F2A00`
  message channel, but on the `0x5C7F4` forest manager of
  `docs/COVERAGE_WAVE75_BASES.md:56-66` instead of the route player's
  `+0x3180`/`+0x3220`/`+0x3224` tables.

---

## 1. `0x806DA520` - rider-tilt integrator of `daLiftScales_c`

**Evidence.** `work/ghidra/rel-d_basesNP/export/806da520_FUN_806da520.c`,
804 bytes, 201 words, synthetic `0x806DA520`. Body `0x806DA520..0x806DA844`
(ends `blr`). Ghidra's single `/* WARNING: Removing unreachable block
(ram,0x806da828) */` is the `psq_l f31` restore.

**What it is.** The per-frame rider-arm update of `daLiftScales_c`, the
`lift_tenbin` balance lift. It integrates a signed platform rate (`+0x5B0`)
from the rider load reported by `0x806DA120`, clamps it to `+-3.5`, adds it to
the accumulated tilt (`+0x5AC`) against the ceiling (`+0x5A8`), plays sound
`0x2B5` on every non-zero step, and posts one of the three state messages
(`0x8082D9E8`, `0x8082DA28`, `0x8082DAE8`) through the actor interface at
`+0x394`. When the tilt hits a stop it also selects one of the lift's four
rider slots into `+0x5BD` and copies that rider's relative position into
`+0x530..+0x538`.

### Signature

```rust
pub unsafe fn fn_806da520(this: usize)
```

`r3` is the `daLiftScales_c` actor (recovered through `_savegpr_25` at
`0x802DCD6C`; `r30 = r3` at `0x806DA548`). No other argument is read. Returns
nothing.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x000` | `u32` | actor id, passed as the sound tag |
| `+0x0AC`/`+0x0B0`/`+0x0B4` | `f32` | world position |
| `+0x358` | `u32` | rider mask, low half |
| `+0x35C` | `u32` | rider mask, high half |
| `+0x394` | interface | state-message sink; vtable slot `+0x18` takes `(this+0x394, msg)` |
| `+0x530`/`+0x534`/`+0x538` | `f32` | selected rider position relative to `this` |
| `+0x5A4` | `f32` | sound-cue forward offset (multiplied by 0.5) |
| `+0x5A8` | `f32` | tilt ceiling |
| `+0x5AC` | `f32` | accumulated tilt |
| `+0x5B0` | `f32` | platform rate (state word in the siblings) |
| `+0x5B4` | `u32` | rider presence/count; 0 disables the mask test |
| `+0x5BD` | `s8` | selected rider slot, `0xFF` = none |

Globals: `0x8042A078` (`u32*` in `main.dol` BSS, a two-word rider mask),
`0x80429D60` (audio-manager pointer), `0x8082D9D8` (base of this REL's BSS
message block; the three messages are `+0x10`, `+0x50`, `+0x110`).

### Exact behaviour

Prologue `0x806DA520..0x806DA558`: frame `0x50`; save `r0`, `f31`;
`_savegpr_25`; `r30 = this`; `r31 = 0x8082D9D8`; `r28 = 0x807CB850` (pool base
for `+0x38` = `0x807CB888`); `r0 = [this+0x5B4]`; `r29 = 0`.

1. `0x806DA54C..0x806DA580` - gate:
   ```rust
   if [this+0x5B4] != 0 {
       let mask = *(u32*)0x8042A078 as *const u32;
       let hit = ([this+0x358] & *mask) | ([this+0x35C] & *mask.add(1));
       if hit == 0 { /* step 2 */ } else { /* step 3 */ }
   } else { /* step 3 */ }
   ```
2. `0x806DA584..0x806DA5A0` - idle reset: `fn_806d9f70(this)`, then
   `interface(this+0x394, 0x8082D9E8)`, then jump to the epilogue.
   `fn_806d9f70` is the documented rider drop/tick pair's first half
   (`docs/COVERAGE_WAVE94_BASES.md:44-45`).
3. `0x806DA5A4..0x806DA618` - rate integration:
   ```rust
   let load = fn_806da120(this);              // 0/1/2 rider-load classifier
   let mut rate = [this+0x5B0];
   if load == 0 { rate -= 0.020751953125; }
   else if load == 2 { rate += 0.020751953125; }
   else if rate < 0.0 {
       rate += 0.078125;
       if rate > 0.0 { rate = 0.0; }
   } else {
       rate -= 0.078125;
       if rate < 0.0 { rate = 0.0; }
   }
   ```
4. `0x806DA618..0x806DA63C` - clamp: `rate = clamp(rate, -3.5, 3.5)`.
5. `0x806DA63C..0x806DA66C` - zero-crossing suppression against the previous
   rate:
   ```rust
   if rate > 0.0 && [this+0x5B0] < 0.0 { rate = 0.0; }
   if rate < 0.0 && [this+0x5B0] > 0.0 { rate = 0.0; }
   ```
6. `0x806DA670..0x806DA6AC` - accumulate:
   ```rust
   let sum = [this+0x5AC] + rate;
   [this+0x5AC] = sum;
   let mut stop = 0u32;                        // r25
   if sum > [this+0x5A8] { [this+0x5AC] = [this+0x5A8]; stop = load & 2; rate = 0.0; }
   else if sum < 0.0     { [this+0x5AC] = 0.0;            stop = load & 1; rate = 0.0; }
   ```
7. `0x806DA6B0..0x806DA714` - sound on a non-zero step (`fcmpu rate, 0.0`
   equality skip):
   ```rust
   if rate != 0.0 {
       let pos = [[this+0x0AC], [this+0x0B0], [this+0x0B4]];
       let mut cue = pos;
       cue[0] = pos[0] + 0.5 * [this+0x5A4];
       let mut out = [0u8; 8];
       call2(0x8006A4B0, &mut out, &cue);      // main.dol position copy
       call5(0x80199390, *(u32*)0x80429D60, 0x2B5, [this+0x000], &out, 0);
   }
   ```
8. `0x806DA718..0x806DA720` - `[this+0x5B0] = rate; fn_806d9fc0(this);`.
   `fn_806d9fc0` is the rider-height pair documented in
   `docs/COVERAGE_WAVE122_BASES.md:90`.
9. `0x806DA724..0x806DA7F0` - stop handling:
   ```rust
   if stop != 0 {
       [this+0x530] = 0.0; [this+0x534] = 0.0; [this+0x538] = 0.0;
       *(u8*)(this+0x5BD) = 0xFF;
       let mut node = call1(0x801625B0, this);      // [[this+0x14]+0x10]
       while node != 0 {
           for i in 0..4usize {
               if *(u8*)(node + 0x584 + i) != 0 {
                   let p = [ [node+0x0AC], [node+0x0B0], [node+0x0B4] ];
                   [this+0x530] = p[0]; [this+0x534] = p[1];
                   [this+0x530] = p[0] - [this+0x0AC];
                   [this+0x534] = p[1] - [this+0x0B0];
                   [this+0x538] = p[2] - [this+0x0B4];
                   *(u8*)(this+0x5BD) = i as u8;
                   break;
               }
           }
           if *(i8*)(this+0x5BD) >= 0 { break; }
           node = call1(0x801625D0, node);          // r3 still holds the node
       }
       interface(this+0x394, 0x8082DAE8);
   } else if [this+0x5B0] == 0.0 && load == 0 {
       interface(this+0x394, 0x8082DA28);
   }
   ```
   The two absolute stores before the relative ones are in the image
   (`0x806DA77C`, `0x806DA788`) and are overwritten two instructions later; they
   are kept here so the transcription is literal. The `addic. r4,r3,172` null
   test at `0x806DA770` always compares `node + 0xAC` against zero and is
   therefore never taken; it is an artefact of the C `if (p != 0)` and no arm is
   needed in Rust.
10. `0x806DA824..0x806DA844` - epilogue: `psq_l f31`, `lfd f31`, `_restgpr_25`,
    `blr`.

`interface(receiver, msg)` is `(*(this+0x394)->vtable->slot_0x18)(this+0x394, msg)`;
the load pair is `lwz r12,0x394(r30); lwz r12,0x18(r12); mtctr r12; bctrl`
(`0x806DA588`, `0x806DA7D8`, `0x806DA80C`).

**Verdict: ready (high).** All three messages are static BSS addresses in this
REL, the sound tag is `[this+0]`, the two floating thresholds are literal, and
the three callees are already translated or named by address.

---

## 2. `0x8071E130` - `daObjWendy_c` rail/height handler

**Evidence.** `work/ghidra/rel-d_basesNP/export/8071e130_FUN_8071e130.c`,
788 bytes, 197 words, synthetic `0x8071E130`. Body `0x8071E130..0x8071E444`
(ends `blr`). Ghidra's two `unreachable block` warnings (`0x8071E420`,
`0x8071E428`) are the `psq_l f31`/`psq_l f30` restores.

**What it is.** The per-frame body of `daObjWendy_c` that keeps a scrolling
height (`+0xB0` of the model holder) in step with a "rail" counter pair
(`+0x1A488` lower bound, `+0x1AC98` upper bound) and pushes the result into the
actor's transform at `+0xB0`. It publishes a positional sound every frame
(`0x554`, tag `[this+0]`, cue built from the two stage camera offsets), then
branches on the global mode `0x8082FFEC == 3` and ends by posting
`0x80830000` (mode 3) or `0x80830040` (any other mode) when the player has
fallen past the screen floor.

### Signature

```rust
pub unsafe fn fn_8071e130(this: usize)
```

`r3` is the `daObjWendy_c` actor/manager (recovered through `_savegpr_26` at
`0x802DCD70`; `r30 = r3` at `0x8071E168`). Returns nothing.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x000` | `u32` | actor id, the sound tag |
| `+0x0B0` | `f32` | smoothed rail height published to the model |
| `+0x394` | interface | state-message sink, vtable slot `+0x18` |
| `+0x1A488` | `u32` | rail lower bound (0x6922 words in) |
| `+0x1AC98` | `u32` | rail upper bound (0x6B26 words in) |
| `+0x1ACCC` | `u8` | "arrived" latch cleared when the mode-3 branch fires |

The object is at least `0x1ACD0` bytes; the two counters are the only fields
this body reads outside `+0x000`, `+0x0B0` and `+0x394`.

Globals: `0x80429D60` (audio manager), `0x80429E00` (`u32` render/camera
object; `+0x3C` and `+0x44` are the two floats compared),
`0x8082FFE4` (`f32` base height), `0x8082FFEC` (`s32` mode),
`0x80830000`/`0x80830040` (BSS message blocks), `0x80809610`/`0x80809614`/
`0x80809618` (three `f32` curve parameters, all read as `0.0` from the image),
`0x80809634 = 8.0f`, `0x8080963C = 216.0f`, `0x80809640 = 300 (u32)`,
`0x8080964C = 800 (u32)`, `0x80809650 = 100 (u32)`, `0x80809658 = 0.02f`.

### Exact behaviour

Prologue `0x8071E130..0x8071E1BC`: frame `0x70`; save `r0`, `f31`, `f30`;
`_savegpr_26`; `r30 = this`; `r29 = 0x80809620`; `r28 = 0x80809538`;
`r31 = 0x8082FFE0`; `local_800 = [0x8080964C]` (800); `f30 = [0x80809628]`
(0.0).

1. `0x8071E17C..0x8071E1BC` - per-frame cue:
   ```rust
   let cam_y = fn_800b30e0();                 // f32 camera offset
   let cam_x = fn_800b30c0();
   let mut cue = [cam_x, cam_y];              // r1+0x20 / r1+0x24
   let mut out = [0u8; 8];
   call2(0x8006A3F0, &mut out, &cue);         // main.dol two-float copy
   call5(0x80199390, *(u32*)0x80429D60, 0x554, [this+0x000], &out, 0);
   ```
2. `0x8071E1C0..0x8071E1C8` - `if [0x8082FFEC] != 3 { goto step 9 }`.
3. `0x8071E1CC..0x8071E1EC` - clamp the upper bound:
   ```rust
   let mut cur = [this+0x1AC98];
   let lo = [this+0x1A488];
   if cur >= 800 && lo <= 800 { cur = lo; [this+0x1AC98] = lo; }
   ```
4. `0x8071E1F0..0x8071E234` - the sub-step:
   ```rust
   let mut lift = 0.0f32;                                   // f31
   if cur < 800 {
       lift = 216.0 * call3(0x806601B0, &cur, &800, 0x80809610);   // f32 in f1
   }
   let mut target = [0x8082FFE4] + 0.0;                     // r1+0x10
   let t = call3(0x806601B0, &cur, &800, 0x80809614);
   let step = (100.0f32 * t) as i32;                        // fctiwz
   let next = cur as i32 + step;
   if next > 0 && (lo as i32) < next {
       target += lift * call3(0x806601B0, &lo, &next, &cur);
   }
   ```
   `0x806601B0` is a three-pointer ratio helper: with
   `fn_806601B0(p, a, b)` it returns `0.0` when `*a == *b` or when `*p` is
   outside the closed interval, the clamped ratio `(*p-*a)/(*b-*a)` when inside,
   and `1.0` when `*p` is past `*b`; the exact arms are in
   `work/ghidra/rel-d_basesNP/export/806601b0_FUN_806601b0.c` (260 bytes,
   untranslated).
5. `0x8071E2D8..0x8071E2F0` - publish:
   `[this+0x0B0] = fn_8065c210(0.02, this+0x0B0, &target)` (the documented
   clamp-and-lerp; `0.02` is the `t` argument in `f1`).
6. `0x8071E2F4..0x8071E314` - floor test:
   ```rust
   let cam = *(u32*)0x80429E00;
   let floor = [cam+0x3C] - [cam+0x44];
   let y = [this+0x0B0] as f32;                             // frsp of the lerp
   let hit = 8.0 + y < floor;
   ```
7. `0x8071E31C..0x8071E348` - `if hit && (lo as i32) >= next { interface(this+0x394, 0x80830000); }`.
8. `0x8071E348` - jump to the epilogue.
9. `0x8071E34C..0x8071E398` - non-mode-3 arm:
   ```rust
   let lo = [this+0x1A488];
   let mut target = [0x8082FFE4] + 0.0;
   if lo <= 800 {
       target += 216.0 * call3(0x806601B0, &lo, &800, 0x80809618);
   }
   [this+0x0B0] = fn_8065c210(0.02, this+0x0B0, &target);
   let cam = *(u32*)0x80429E00;
   let floor = [cam+0x3C] - [cam+0x44];
   if 8.0 + f32([this+0x0B0]) < floor && 800 + 300 <= lo {
       *(u8*)(this+0x1ACCC) = 0;
       interface(this+0x394, 0x80830040);
   }
   ```
10. `0x8071E41C..0x8071E444` - epilogue: `psq_l f31`, `lfd f31`, `psq_l f30`,
    `lfd f30`, `_restgpr_26`, `blr`.

Register notes worth keeping: the branch at `0x8071E1C8` is `bne` on
`[0x8082FFEC] - 3`, so only the exact value 3 enters step 3; step 9's
comparisons are `lo <= 800` (`bgt` skip) and `800 > 0` (always true); step 3's
are `cur >= 800` and `lo <= 800`.

**Verdict: ready (high).** Both BSS globals are inputs; every literal is read
from the image; the only untranslated callee (`0x806601B0`) is named by address
and its own 260-byte body is small enough to transcribe separately if the
translation lane prefers to inline it.

---

## 3. `0x80788CB0` - the broom's rider/exit state machine

**Evidence.** `work/ghidra/rel-d_basesNP/export/80788cb0_FUN_80788cb0.c`,
788 bytes, 197 words, synthetic `0x80788CB0`. Body `0x80788CB0..0x80788FC4`
(ends `blr`). Ghidra's `unreachable block (ram,0x80788fa4)` is the `psq_l f31`
restore.

**What it is.** The four-state body of the `broom` ride object
(`docs/COVERAGE_WAVE115_BASES.md:42-47`, cluster `0x807883E0..0x8078A410`) that
runs after the ride placement: it lerps the broom's roll/pitch (`+0x3C4`) from
its rate (`+0x3C8`), drives the three `+0xDC/+0xE0/+0xE4` model scales, decides
whether the ride has reached the dismount point, spawns the `cobToride` door
with `0x80785600`, switches the broom part through `0x80789B80`, releases the
part slot through `0x80789E60` and posts the `0x17` world-map effect through
`0x80103390`. Returns 1 when state 3 (finished) is reached, 0 otherwise.

### Signature

```rust
pub unsafe fn fn_80788cb0(this: usize) -> u32
```

`r3` is the broom object; the body needs no saved GPRs. Returns the finished
flag in `r3` (`0x80788F94` sets 1 for state 3; `0x80788F9C` sets 0).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x0AC`/`+0x0B0`/`+0x0B4` | `f32` | position |
| `+0x0DC`/`+0x0E0`/`+0x0E4` | `f32` | model scale triple |
| `+0x0EC`, `+0x10C`, `+0x114`, `+0x118` | `f32` | four attitude caches zeroed when `0x800F6390` is set |
| `+0x0BC` | `f32` | "mounted" height threshold |
| `+0x100` | `f32[3]` | effect transform (rotation) handed to `0x80103390` |
| `+0x334` | `s32` | variant (2 selects the "already open" door path) |
| `+0x338` | `s32` | state 0..3 |
| `+0x33C` | `s32` | "waiting for dismount" flag |
| `+0x35C` | `s32` | part/sub-state; 9 is the dismount part |
| `+0x3C4` | `f32` | roll/pitch accumulator |
| `+0x3C8` | `f32` | roll/pitch rate |
| `+0x3CC` | `s32` | sub-mode selecting which state owns the lerp |
| `+0x40C` | `f32[4]` | last stamped position (fed to `0x801C1650`) |
| `+0x41C` | `u8` | "rider present" byte |
| `+0x420` | `u32*` | pointer to a parameter block; `[+4]` is the scale factor |

Globals: `0x8042A1AC` (`u32` scene/owner handle), `0x8042A24D` (`u8` scene id
tested against 7), `0x8042A26C` (`u32` effect-table base), `0x80832D10` (this
REL's BSS scratch passed as the fifth effect argument). Constants:
`0x807CFA08 = 0.0f`, `0x807CFA50 = 1.0f`, `0x807CFA58 = 0.1f`,
`0x807CFA5C = 5.0f`, `0x807CFA64 = 0.9f`, `0x80814C10 = 100.0f`,
`0x80814C24 = 10.0f`.

### Exact behaviour

Prologue `0x80788CB0..0x80788CE0`: frame `0x30`; save `r31`, `r30`, `r29`,
`f31`; `r31 = 0x807CFA08`; `r30 = this`; `r29 = 0`.

1. `0x80788CE0..0x80788D00` - attitude reset:
   ```rust
   if call1(0x800F6390, this) != 0 {
       [this+0x10C] = 0.0; [this+0x114] = 0.0;
       [this+0x0EC] = 0.0; [this+0x118] = 0.0;
       r29 = 1;                                 // "reset happened"
   }
   ```
2. `0x80788D04..0x80788D28` - dispatch on `[this+0x338]`; values 0, 1, 2, 3 are
   handled and anything else falls to the epilogue with 0.
3. `0x80788D2C..0x80788DC0` - state 0:
   ```rust
   if [this+0x3CC] == 0 {
       let do_lerp = if [this+0x33C] != 0 {
           call2(0x801C1650, this+0x40C, this+0x0AC);   // squared distance
           sqrt_f32() < 100.0                           // 0x802BE470
       } else { true };
       if do_lerp {
           let v = 0.9 * [this+0x3C4] + 0.1 * [this+0x3C8];
           [this+0x3C4] = v;
           let s = [*(u32*)(this+0x420) + 4] * v;
           [this+0x0DC] = s; [this+0x0E0] = s; [this+0x0E4] = s;
       }
   }
   if [this+0x0B0] < [this+0x0BC] { [this+0x338] = 1; }
   ```
4. `0x80788DC4..0x80788E40` - state 1 opens with the identical lerp block, but
   gated on `[this+0x3CC] == 1` (`0x80788DC8`), then:
   ```rust
   if [this+0x33C] != 0 && [this+0x35C] == 9 {
       call2(0x801C1650, this+0x40C, this+0x0AC);
       if sqrt_f32() < 10.0 { 0x8075F9E0(f1 = 1.0, f2 = 0.0); }
   }
   ```
   `0x8075F9E0` allocates factory object `0x272` and forwards the two floats to
   `0x8075FA40` (84 bytes, untranslated; `0x8075F9E0` is the "spawn the
   `0x272`-class helper" wrapper). `sqrt_f32()` above is `0x802BE470` applied to
   the `f1` that `0x801C1650` returned (a squared distance), i.e. the 3-D
   distance between `+0x40C` and `+0xAC`; the comparison against 100.0 (state 0)
   and 10.0 (state 1) is on that distance.
5. `0x80788E88..0x80788F48` - the dismount arm, only when `r29 != 0`:
   ```rust
   if call1(0x801015E0, *(u32*)0x8042A1AC) == 42 { call1(0x80785600, 1); }
   if [this+0x41C] == 0 {
       call1(0x80785600, if [this+0x334] == 2 { 0 } else { 1 });   // 0x80788EE4
   } else {
       if [this+0x334] == 2 { call1(0x80785600, 0); }
       0x80789B80(f1 = 5.0, r3 = this, r4 = 4);
   }
   [this+0x338] = 2;
   if ([this+0x33C] == 0 || [this+0x35C] != 9) && *(u8*)0x8042A24D != 7 {
       call5(0x80103390, *(u32*)0x8042A26C, 0x17, this+0x0AC, this+0x100, 0x80832D10);
   }
   ```
   The `(2 - v) | (v - 2)` / `>> 31` sequence at `0x80788EE4` is the
   "`v == 2 ? 0 : 1`" idiom and is written as such above. `0x80785600` is the
   documented `cobToride` door spawner; `0x80789B80` is the documented broom part
   switch `(duration, actor, slot)`.
6. `0x80788F4C..0x80788F90` - state 2:
   ```rust
   if ([this+0x41C] != 0 && call2(0x80789E60, this, 4) != 0) || [this+0x41C] == 0 {
       0x80789B80(f1 = 5.0, r3 = this, r4 = 0);
       [this+0x338] = 3;
   }
   ```
7. `0x80788F94..0x80788F98` - state 3: `return 1`.
8. `0x80788F9C` - every other state: `return 0`.
9. `0x80788FA0..0x80788FC0` - epilogue: `psq_l f31`, `lfd f31`, restore
   `r31/r30/r29`, `blr`.

**Verdict: ready (high).** `0x800F6390`, `0x801C1650`, `0x802BE470` and
`0x8075F9E0` are named by address or already in the ledger; the four messages
are static BSS; `0x80103390`'s five arguments are all live in registers at the
call site (`0x80788F28..0x80788F44`).

---

## 4. `0x80763290` - World-6 course-node ramp mean

**Evidence.** `work/ghidra/rel-d_basesNP/export/80763290_FUN_80763290.c`,
780 bytes, 195 words, synthetic `0x80763290`. Body `0x80763290..0x8076359C`
(ends `blr`). Ghidra's three `unreachable block` warnings (`0x8076358C`,
`0x80763584`, `0x8076357C`) are the `psq_l f31`/`f30`/`f29` restores; the
`unaff_GQR0` lanes are the `qr0` saves.

**What it is.** A leaf that returns the scaled mean of an `n`-step ramp for the
world-map course node selected by the low byte of `[this+4]`. The four records
at `0x80815494` (stride `0x18`) carry a scale and three halfwords; the caller
`0x80762EA0` writes the result into the node's animation element at `+0x1EC`
through `0x80168FE0` / `0x801680E0`. The record table is the World-6 set (the
course-name pointers it indexes at `0x80815470` read `"W601"`, `"W603"`,
`"W605"`).

### Signature

```rust
pub unsafe fn fn_80763290(this: usize) -> f32
```

`r3` is the course node; the return is `f1` (`f32`, Ghidra types it `double`
because every step is a `fadds` widened to `f64`). No saved GPRs.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | kind word; low byte selects the record |

### Record table `0x80815494`, stride `0x18`, four records

| idx | scale `+0x10` | `a` `+0x14` | `b` `+0x16` | `n` `+0x18` |
|--:|---:|---:|---:|---:|
| 0 | `1.0f` | 60 | 98 | 30 |
| 1 | `1.0f` | 30 | 75 | 15 |
| 2 | `1.0f` | 90 | 160 | 15 |
| 3 | `1.0f` | 120 | 155 | 15 |

The `n` halfword is read at `record + 0x18`, i.e. on the first halfword of the
following record in the image; the table above lists the values the machine
actually loads (`0x808154AC`, `0x808154C4`, `0x808154DC`, `0x808154F4` =
`30, 15, 15, 15`). The words at `record + 0x00..0x0C` are not read by this
body.

Constants: `0x807CEE64 = 1.0f`, `0x807CEE68 = 4503601774854144.0`
(`0x4330000080000000`, the unsigned-int double bias used by the
`xoris 0x8000` conversions).

### Exact behaviour

Prologue `0x80763290..0x807632D4`: frame `0x50`; save `f31/f30/f29`
(`stfd` + `psq_st` each); `r3 = 0x43300000` stored at `r1+0x08` and `r1+0x10`;
`r0 = ([this+4] & 0xFF) * 0x18`; `r7 = 0x80815494 + r0`; `r3 = 0`.

1. `0x807632D8..0x80763318` - seed:
   ```rust
   let n = *(r7 + 0x18) as i16;        // lha
   let a = *(r7 + 0x14) as i16;
   let b = *(r7 + 0x16) as i16;
   let scale = *(r7 + 0x10) as f32;
   let mut acc = (((b - a) as f32) - (n as f32)) * scale;
   ```
2. `0x8076331C` - `if n <= 0 { return acc; }` (`cmpwi cr1,r8,0` / `ble`).
3. `0x80763320..0x8076334C` - vectorisation guard, exactly as emitted:
   ```rust
   let mut ok = false;
   if n >= 0 {                                   // blt skips
       if n <= 0x7FFF_FFFE { ok = true; }        // cmpw against 0x7FFFFFFE
   }
   if !ok || n <= 8 { goto step 5; }             // 0x80763510
   ```
4. `0x80763350..0x8076350C` - unrolled loop: `cnt = (n - 1) >> 3` iterations of
   eight terms each,
   ```rust
   for k in 0..n { acc += (1.0 - (k as f32) / (n as f32)) * scale; }
   ```
   with `k` materialised one-by-one through the `xoris 0x8000` double trick
   (`0x8076338C..0x807634E4`). The eight terms are summed in the source order
   `k`, `k+1`, ... `k+7` and added to `acc` as they are produced, so the
   floating-point association matches the listing and not the closed form.
5. `0x80763510..0x80763578` - scalar remainder: the same term for
   `k in [8*ceil((n-1)/8) .. n)`.
6. `0x8076357C..0x80763598` - epilogue; the closed form of the accumulated sum
   is `scale * ((b - a) - (n - 1)/2)`, which is useful as a test oracle but must
   not replace the loop in the translation if bit-exactness is wanted.

**Verdict: ready (high).** Leaf, no callees, no globals, no branch out of the
body, table read out of the image.

---

## 5. `0x80794C00` - World-7 course-node ramp mean (near-twin of row 4)

**Evidence.** `work/ghidra/rel-d_basesNP/export/80794c00_FUN_80794c00.c`,
780 bytes, 195 words, synthetic `0x80794C00`. Body `0x80794C00..0x80794F0C`
(ends `blr`). Ghidra's three `unreachable block` warnings (`0x80794EFC`,
`0x80794EF4`, `0x80794EEC`) are the three `psq_l` restores.

**What it is.** Byte-for-byte the same algorithm as row 4 with a different
table, stride and constant pool: the World-7 course node set. Its own record
table is `0x8081B4EC` (stride `0x1C`, three records) and the course-name
pointers it indexes at `0x8081B4D4` read `"W7T0"` and `"W706"`. The caller
`0x807946D0` uses it together with `0x80794BF0` (12 bytes) to drive the node's
`+0x264` animation element.

### Signature

```rust
pub unsafe fn fn_80794c00(this: usize) -> f32
```

`r3` is the course node; the return is `f1` (`f32`).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | kind word; low byte selects the record |

### Record table `0x8081B4EC`, stride `0x1C`, three records

| idx | scale `+0x10` | `a` `+0x18` | `b` `+0x1A` | `n` `+0x1C` |
|--:|---:|---:|---:|---:|
| 0 | `1.7f` | 90 | 210 | 30 |
| 1 | `1.7f` | 190 | 310 | 30 |
| 2 | `1.7f` | 290 | 410 | 30 |

(`+0x14` of each record is `1.0f` and is not read.) Constants:
`0x807D005C = 1.0f`, `0x807D0060 = 4503601774854144.0`.

### Exact behaviour

Identical to row 4 (`0x80794C00..0x80794F08`) with these substitutions:

* `mulli r0,r0,0x1c` at `0x80794C34` and base `0x8081B4EC` (`0x80794C24`,
  `0x80794C38`);
* `lha` offsets `+0x1C` (n), `+0x18` (a), `+0x1A` (b), scale at `+0x10`;
* the double bias is `0x807D0060` and the `1.0f` is `0x807D005C`;
* the loop guards are at `0x80794C8C` (n <= 0), `0x80794C98` (n <= 8),
  `0x80794CBC` (the `0x7FFFFFFE` bound), the unrolled body runs
  `0x80794CFC..0x80794E7C` and the scalar remainder `0x80794E80..0x80794EE8`.

**Verdict: ready (high).** Same reasoning as row 4. The pair should be
translated as one shared helper with the table/stride/offsets passed in, or as
two literal bodies; both are bit-exact.

---

## 6. `0x8075F270` - `cobToride` gate state entry

**Evidence.** `work/ghidra/rel-d_basesNP/export/8075f270_FUN_8075f270.c`,
768 bytes, 192 words, synthetic `0x8075F270`. Body `0x8075F270..0x8075F570`
(ends `blr`). No `psq` words at all.

**What it is.** The state-entry/reset body of the castle-gate actor that
`docs/COVERAGE_WAVE106_BASES.md:49-54` calls the `cobCastle` door tail (its
predecessor, the transform writer, is `0x8075F1C0`; its tail functions
`0x8075F570` and `0x8075F980` are already translated). It clears the local
state, derives the gate state (`+0x298`) from the gate subsystem's mode and
sub-queries, posts that state through `0x801016B0`, and finally publishes two
`0x284` and one `0x2A3` scene messages before calling the timer-clear tail.

### Signature

```rust
pub unsafe fn fn_8075f270(this: usize)
```

`r3` is the gate actor (`r30 = r3` at `0x8075F29C`). No saved GPRs; the body
saves `r28..r31` by hand.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x0AC`/`+0x0B0`/`+0x0B4` | `f32` | position |
| `+0x1AC` | model | NW4R model; vtable slot `+0x18` re-syncs it |
| `+0x1B8` | element | NW4R element whose `+0x30` is the animation frame |
| `+0x1E0` | `f32` | closed-position height |
| `+0x298` | `s32` | gate state, `-1` = none |
| `+0x2A0` | `u8` | "raised to the closed height" flag |
| `+0x2B0` | `u8` | timer latch owned by `0x8075F570` |

Globals: `0x8042A1AC` and `0x8042A1A0` (the two owner handles passed to
`0x801016B0`), `0x80377DC8`/`0x80377DCC`/`0x80377DD0` (`main.dol` BSS, a
three-float world vector broadcast in the `0x284` message). Constants:
`0x807CED08 = 0.0f`, `0x807CED0C = 1.0f`, `0x807CED10 = 100.0f`.

### Exact behaviour

Prologue `0x8075F270..0x8075F2B8`: frame `0x30`; save `r0`, `r28..r31`;
`r30 = this`; `r29 = *(u32*)0x8042A1AC`; `r28 = *(u32*)0x8042A1A0`;
`r31 = 0x807CECE8`; then

```rust
*(u8*)(this+0x2B0) = 0;
[this+0x298] = -1;
*(u8*)(this+0x2A0) = 0;
```

1. `0x8075F2BC..0x8075F2C4` - `if call1(0x800FD8A0, this) == 0 { ... }`:
   ```rust
   if call1(0x800FD820, this) == 1 { [this+0x298] = 10; }
   ```
   and jump to step 3 (the publish at `0x8075F444..0x8075F458` falls into
   `0x8075F45C`).
2. `0x8075F2C8..0x8075F338` - otherwise `mode = call1(0x800FD820, this)`:
   ```rust
   if (mode as u32).wrapping_sub(2) <= 2 {          // mode in {2,3,4}
       model_sync();                                 // step 3
       0x80168FE0(f1 = 0.0, r3 = this+0x1B8);
       if call1(0x800FD920, this) == 0 { [this+0x298] = 9; }
       else if call1(0x800FC0A0, /*r3 = f920's result*/) == 0 { [this+0x298] = 0x1B; }
       else if call1(0x800FBFB0, /*r3 = f_c0a0's result*/) == 0 { [this+0x298] = 9; }
       else { [this+0x298] = 0x1C; }
       call1(0x8075F980, this);
   } else if mode == 6 {
       [this+0x298] = if call1(0x800FD880, this) == 0 { 0xB } else { 0xA };
       call1(0x8075F980, this);
   } else if mode == 5 || mode == 7 {
       model_sync();
       0x80168FE0(f1 = 0.0, r3 = this+0x1B8);
       [this+0x298] = 9;
       call1(0x8075F980, this);
   } else if (mode == 0 || mode == 8) && call1(0x800FD880, this) == 0 {
       *(u8*)(this+0x2A0) = 1;
       model_sync();
       0x80168FE0(f1 = [this+0x1E0] - 100.0, r3 = this+0x1B8);
       call1(0x8075F980, this);
   }
   ```
   `model_sync()` is `(*(u32*)(this+0x1AC)+0x18)(this+0x1AC, this+0x1B8)`
   (`lwz r12,0x1AC; addi r3,this+0x1AC; addi r4,this+0x1B8; lwz r12,0x18(r12);
   mtctr; bctrl`, at `0x8075F338` and `0x8075F3BC` and `0x8075F40C`).
   Ghidra prints `func_0x800fc0a0()` and `func_0x800fbfb0()` with no arguments;
   the register stream at `0x8075F36C`/`0x8075F378` shows `r3` still holds the
   previous call's return value, so both are called with that handle. This is
   the one argument-recovery in this document that changes a transcription.
3. `0x8075F45C..0x8075F478` - publish the state:
   ```rust
   if [this+0x298] >= 0 {
       call5(0x801016B0, *(u32*)0x8042A1AC, [this+0x298], this, *(u32*)0x8042A1A0, 200);
   }
   ```
4. `0x8075F47C..0x8075F548` - two message arms, both inside
   `call0(0x800FC370) == 0 && call1(0x800FBFB0, /*r3 = 0x800FC370's result*/) != 0`:
   ```rust
   if (([this+0x298] as u32).wrapping_sub(27) <= 1) {
       call5(0x800F2A00, 0x284, this, 2, &mut local, 0);
       if [this+0x298] == 0x1C {
           let pos = [[this+0x0AC], [this+0x0B0], [this+0x0B4] - 100.0];
           call5(0x800F2A00, 0x2A3, this, 0, &pos, 0);
       }
   }
   if call1(0x800FD8A0, this) == 0 {
       let v = [*(f32*)0x80377DC8, *(f32*)0x80377DCC, *(f32*)0x80377DD0];
       call5(0x800F2A00, 0x284, this, 1, &v, 0);
   }
   ```
   The first arm's `&mut local` is the 16-byte scratch at `r1+0x14` whose
   contents are never read; it is a required write destination.
5. `0x8075F548..0x8075F54C` - `call1(0x8075F570, this)`.
6. `0x8075F550..0x8075F56C` - epilogue.

**Verdict: ready (high).** The five gate queries are named by address; the two
already-translated tails do the actual work; the two `main.dol` BSS vectors are
inputs.

---

## 7. `0x8066DDC0` - `daCastleBossKey_c` per-player key placement

**Evidence.** `work/ghidra/rel-d_basesNP/export/8066ddc0_FUN_8066ddc0.c`,
764 bytes, 191 words, synthetic `0x8066DDC0`. Body `0x8066DDC0..0x8066E0BC`
(ends `blr`). Ghidra's seven `unreachable block` warnings
(`0x8066E09C..0x8066E068`) are the seven `psq_l f25..f31` restores.

**What it is.** The per-frame placement of the castle-boss key over the players:
for each of the `*0x80429CA0` player slots it reads the slot's player object
from `+0x42C`, asks the player's model interface for its character id
(`vtable + 0x6C`, first byte), and writes the key's per-character height into
`this + 0x3F0 + id*4`, spaced evenly by `96 / (1 + count)`. When the slot's key
is "active" (player flag `+0x10D4 & 1`) it hands the height to the player
through `0x80052080` and advances the slot state (`+0x40C`) from 1 to 2 when the
smoothing reaches the target; in state 2 it forces the player into the catch
state with `0x80051F30` / `0x80052170`. Returns 1 unconditionally.

### Signature

```rust
pub unsafe fn fn_8066ddc0(this: usize) -> u32
```

`r3` is the key object (recovered through `_savegpr_22` at `0x802DCD60`;
`r23 = r3` at `0x8066DE2C`). The body sets `r3 = 1` at `0x8066E06C` before the
epilogue, so the return is 1 (`Ghidra types it void`; the store is visible in
the instruction stream and the caller `0x8066D7CC` ignores it).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x3EC` | `f32` | key base height |
| `+0x3F0 + id*4` | `f32` | per-character key height (id = player character byte) |
| `+0x40C + slot*4` | `s32` | per-slot state: 0 idle, 1 flying, 2 caught |
| `+0x410` | `s32` | key mode; 2 enables the "caught" test in the far arm |
| `+0x42C + slot*4` | `u32` | player object for the slot |
| `+0x430 + slot*4` | `u32` | neighbour rider/player object for the distance test |

Globals: `0x80429CA0` (`u32` active player count, `<= 4`).
Constants: `0x807C8F54 = 24.0f`, `0x807C8F58 = 96.0f`, `0x807C8F60 = 2.5f`,
`0x807C8F64 = 0.1f`, `0x807C8F94 = 0.5f`, `0x807C8F98 = 0.0f`,
`0x807C8FA0 = 1.0f`, `0x807C8FC8 = 4503601774854144.0`.

Player-object fields read here: `+0x60` (model object, vtable `+0x6C` returns a
`char*` whose first byte is the character id, sign-extended by
`0x8066DE9C`/`0x8066DE90..0x8066DE9C`), `+0xAC` (x), `+0xE8` (the magnitude
smoothed into the key), `+0x10D4` (flag word, bit 0 = "key reachable").

### Exact behaviour

Prologue `0x8066DDC0..0x8066DE74`: frame `0xD0`; save `r0`, `f31..f25`;
`_savegpr_22`; `r23 = this`; `r28 = this`; `r27 = this+0x40C`; `r29 = r1+0x08`;
`r25 = 0`; `r26 = *(u32*)0x80429CA0`; then

```rust
let count = *(u32*)0x80429CA0;
let step  = 96.0f32 / (1.0f32 + count as f32);        // f29
let base  = [this+0x3EC] - 96.0 * 0.5;                // 48 below
let near  = 24.0f32;                                  // f27
let half  = 0.5f32;                                   // f25
let target = 2.5f32;                                  // f28
let zero  = 0.0f32;                                   // f26
let rate  = 0.1f32;                                   // [0x807C8F64]
let mut slot = 0usize;                                // r25
let mut player_ptr = this;                            // r28
let mut state_ptr  = this + 0x40C;                    // r27
```

1. `0x8066DE78..0x8066DE8C` - per-slot model lookup:
   ```rust
   let player = *(u32*)(player_ptr + 0x42C) as usize;
   let model  = *(u32*)(player + 0x60) as usize;
   let name   = call1(*(u32*)(model + 0x6C), player) as *const u8;   // bctrl
   let id     = *(name) as i8;                                       // extsb
   ```
2. `0x8066DE90..0x8066DEAC` - dispatch on `*state_ptr`: 1 -> step 3, 2 -> step 7,
   anything else -> step 8.
3. `0x8066DEB0..0x8066DEE4` - state 1 position write:
   ```rust
   let slot_f = (slot + 1) as f32;
   let pos = (this + 0x3F0 + (id as usize) * 4) as *mut f32;   // stfsu f0,0x3F0(r22)
   *pos = step * slot_f + base;
   ```
   The folded `48.0` is `f1 = 96.0 * 0.5` at `0x8066DEC0`, subtracted at
   `0x8066DED4`; the sum is stored once through `stfsu f0,0x3F0(r22)`, which also
   leaves `r22` pointing at the per-character slot for the calls below.
4. `0x8066DEE8..0x8066DEF0` - `if (*(u32*)(player+0x10D4) & 1) == 0 { goto step 8 }`.
5. `0x8066DEF4..0x8066DF54` - last slot (`slot == count-1`):
   ```rust
   let mut value = [player+0xE8];
   if value <= 0.0 { value = -value; }                  // 0x8066DF00..0x8066DF10
   let smoothed = fn_8065c210(rate, &value, 0x807C8F60);
   call3(0x80052080, player, pos, &smoothed);
   if smoothed == 2.5 { *state_ptr += 1; }
   ```
6. `0x8066DF58..0x8066E01C` - non-last slot:
   ```rust
   let nb = *(u32*)(player_ptr + 0x430) as usize;
   if [nb+0xAC] - [player+0xAC] <= 24.0 {
       let target = 2.5 * 0.5;                          // 1.25
       let mut value = abs([player+0xE8]);
       let s = fn_8065c210(rate, &value, &target);
       call3(0x80052080, player, pos, &s);
   } else {
       let mut value = abs([player+0xE8]);
       let s = fn_8065c210(rate, &value, 0x807C8F60);
       call3(0x80052080, player, pos, &s);
       if [this+0x410] == 2 && s == 2.5 { *state_ptr += 1; }
   }
   ```
7. `0x8066E020..0x8066E050` - state 2 catch:
   ```rust
   if call1(0x80052030, player) != 0 {
       call2(0x80051F30, player, 0);
       call2(0x80052170, player, 0x91);
       *state_ptr += 1;
   }
   ```
8. `0x8066E054..0x8066E064` - advance: `player_ptr += 4; state_ptr += 4;
   slot += 1; if slot < count { goto step 1; }`.
9. `0x8066E068..0x8066E0BC` - epilogue: seven `psq_l` restores, `r3 = 1`,
   `_restgpr_22`, `blr`.

Note the `fcmpo`/`fneg` pairs at `0x8066DF04..0x8066DF10`,
`0x8066DF78..0x8066DF84` and `0x8066DFE4..0x8066DFF0` implement `fabs` as
"if `x <= 0.0 { -x }", which differs from `fabs` only for `-0.0`; keep the
branch form.

**Verdict: ready (high).** All four call targets are already translated in
`src/game/player.rs`; the per-character table is inside the object; the vtable
slot is read from two loads.

---

## 8. `0x807727D0` - forest-manager three-slot course sync

**Evidence.** `work/ghidra/rel-d_basesNP/export/807727d0_FUN_807727d0.c`,
764 bytes, 191 words, synthetic `0x807727D0`. Body `0x807727D0..0x80772ACC`
(ends `blr`). No `psq` words.

**What it is.** A member of the `0x5C7F4`-byte forest manager
(`docs/COVERAGE_WAVE75_BASES.md:56-66`, constructor `0x80771540`) that walks
the three course-object slots of the current area and re-seeds each cell of the
global course manager with the area's mode word, choosing the respawn
identifier by RNG when the slot was empty, and publishing one of five
`0x800F2A00` messages (ids `0x28F`..`0x293`) selected by the area index. It ends
by emitting message `0x271` when the manager reports a pending course and the
mode word is zero.

### Signature

```rust
pub unsafe fn fn_807727d0(this: usize)
```

`r3` is the forest manager (recovered through `_savegpr_21` at `0x802DCD5C`;
`r24 = r3` at `0x807727E8`). Returns nothing.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x3388` | `s32` | area index; selects the message id and the course-manager row |
| `+0x338C` | `s32` | area mode word, written into each cell's `+0xA0` |
| `+0x0AC` | `f32[3]` | origin handed to every message as `r6` |

Globals: `0x80429F7C` (`u32` course-manager pointer), `0x8042A040` (`u32`
sprite-image owner), `0x8042A1A0` (`u32` owner; `+0x22C` is the "pending
course" value). Constants: the five message ids `0x28F`, `0x290`, `0x291`,
`0x292`, `0x293` and the final `0x271`.

### Cell accessors used

`0x800BBC10(manager, x, y, dst)` copies the 13-byte cell record
(`+0xA0/+0xA4/+0xA8/+0xAC`) into `dst`;
`0x800FC140(manager, x, y)` returns the cell's entry count;
`0x800FC190(manager, x, y, n)` truncates it;
`0x800BBBF0(manager, x, y, value)` writes `+0xA8`;
`0x800BBBD0(manager, x, y, a, b)` writes `+0xA0`/`+0xA4`.
All five are documented in `src/game/stage.rs` with the same
`x*0x40 + y*0x10` addressing; here `x = [this+0x3388]` and `y = slot`.

### Exact behaviour

Prologue `0x807727D0..0x8077286C`: frame `0x60`; save `r0`, `r21..r31`;
`_savegpr_21`; `r24 = this`; `r28 = 0` (slot); `r29 = *(u32*)0x80429F7C`;
`r31 = 0` (message id, "none"); `r30 = 0` (the "publish" flag);
`r21 = 0` (the identifier published to `0x800F2A00`);

```rust
call2(0x800E0380, *(u32*)0x8042A040, 0xFFFF_FFFF);   // reset the image record
```

1. `0x807727E4..0x80772854` - message selection:
   ```rust
   match [this+0x3388] {
       0 => { r31 = 0x28F; r30 = 1; }
       3 => { r31 = 0x290; r30 = 1; }
       4 => { r31 = 0x291; r30 = 1; }
       2 => { r31 = 0x292; r30 = 1; }
       6 => { r31 = 0x293; r30 = 1; }
       _ => {}
   }
   ```
2. `0x8077286C..0x807728A4` - slot loop head (slots 0, 1, 2):
   ```rust
   let has = call2(0x80100720, this, slot) != 0;      // r22
   if !(r30 != 0 && !has) {
       call4(0x800BBC10, *(u32*)0x80429F7C, [this+0x3388], slot, &mut cell);
       let id = cell[1];                              // local_44, cell+4
       let count = call2(0x800FC140, [this+0x3388], slot);
       let mut live = count;                          // r25
       if count > 0 && call0(0x800F7400) != 0 {
           live = count - 1;
           call3(0x800FC190, [this+0x3388], slot, live);
       }
       /* r30 != 0 arms */
   }
   ```
3. `0x807728E8..0x80772964` - the "empty slot" arm, `r30 != 0 && live == 0`:
   ```rust
   if id < 0 {
       let mut pick = call1(0x800B2EE0, r22);         // rng in r22's range
       if r31 == 0x292 && slot == 1 {                 // 0x80772914
           call4(0x800BBC10, *(u32*)0x80429F7C, [this+0x3388], 0, &mut cell0);
           pick = (cell0[1] + r22/2) % r22;           // 0x80772930..0x8077294C
       }
       call4(0x800BBBF0, *(u32*)0x80429F7C, [this+0x3388], slot, 0);
       publish = pick;                                // r21
   } else {
       publish = id;
       if count != 0 {
           let mut pick = call1(0x800B2EE0, r22);
           if r31 == 0x292 {
               let pending = *(u32*)(*(u32*)0x8042A1A0 + 0x22C);   // r22 = pending
               loop {
                   pick = call2(0x80772AD0, this, slot);
                   if call2(0x8078D0E0, pick, pending) == 0 { break; }
               }
           }
           call4(0x800BBBF0, *(u32*)0x80429F7C, [this+0x3388], slot, 0);
           publish = pick;
       }
   }
   call5(0x800BBBD0, *(u32*)0x80429F7C, [this+0x3388], slot, [this+0x338C], publish);
   ```
   The `%` at `0x80772944` is the `divw r0,r3,r22` the reused decoder did not
   tabulate; the surrounding `(x + sign(v)) >> 1` / `mullw` / `subf` at
   `0x80772930..0x8077294C` is the signed-modulo idiom, with `v = r22` (the
   value `0x80100720` returned).
4. `0x807729E0..0x807729F8` - `if r31 == 0x292 { call4(0x800BBBF0, manager,
   [this+0x3388], slot, 0); }`.
5. `0x807729FC..0x80772A54` - publish gate:
   ```rust
   let mut fire = 0u32;                               // r3
   if live > 0 && count == 4 && cell[12] == 0 { fire = 1; }
   if fire == 0 && live != 0 { goto step 6; }          // 0x80772A28
   if fire != 0 { publish = id; }
   call5(0x800F2A00, r31, this, ((publish as u32) << 4) | slot, this+0x0AC, 0);
   ```
   `cell[12]` is the byte at `cell + 0xC` (the `0x800BBC10` copy's `+0xAC`
   byte), read at `0x80772A10`.
6. `0x80772A58..0x80772A60` - `slot += 1; if slot < 3 { goto step 2; }`.
7. `0x80772A64..0x80772AB4` - tail:
   ```rust
   if call0(0x800FC7F0) != 0 && [this+0x338C] == 0 {
       let id = call1(0x800F9840, *(u32*)(*(u32*)0x80429F7C + 0x54));
       call5(0x800F2A00, 0x271, this, if id == 5 { 0 } else { 1 }, 0, 0);
   }
   ```
8. `0x80772AB4..0x80772AC8` - epilogue: `_restgpr_21`, `blr`.

**Verdict: ready (high).** Every callee is either already translated
(`0x800BBC10`, `0x800BBBD0`, `0x800BBBF0`, `0x800E0380`, `0x80100720`,
`0x800B2EE0`, `0x80772AD0`) or named by address (`0x800FC140`, `0x800FC190`,
`0x800F7400`, `0x800FC7F0`, `0x800F9840`, `0x800F2A00`); the five message ids
are immediate; the only global state is the course manager pointer.

---

## Verification

* `work/ghidra/rel-d_basesNP/export/functions.tsv`: all eight byte counts and
  `decompiled=true` re-read for this pass; the eight `.c` files exist at
  `work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`.
* Raw decode: `work/rels/elf/d_basesNP.elf`, program headers re-read, formula
  `(address - 0x80600000) + 0x1000` applied per instruction, not per document.
* Extent: `psq_l`/`stfd` prologue, one `blr` per body at `address + size - 4`,
  no earlier `blr`, no `b` out of the body, every reachable word decodable
  (1,559 words, one `divw` outside the decoder's table and hand-decoded).
* Duplicate check: SHA-1 per body against every same-size `d_basesNP` row - no
  duplicate of any of the eight.
* Ledger: `ledger/functions.csv` has no row with any of the eight `address`
  fields; `src/game/bases.rs` has no `fn_<addr>` token for any of the eight;
  no `#{1,6} ` heading in `docs/*.md` mentions any of the eight.
* Constants: every `lfs`/`lfd`/`lwz` literal in the eight bodies was resolved
  through the ELF segments and printed with its `f32`/`f64`/`u32` value in the
  tables above; `0x8082xxxx` values at or above `0x80822900` were treated as
  BSS and named by address.

## Handoff

* Rows 4 and 5 are the same algorithm with two tables; a translation lane can
  share one body but must keep the two `n` reads at `record + 0x18` /
  `record + 0x1C` and the two `lha` sign extensions exactly as listed.
* Row 6 carries one argument recovery that must not be dropped: `0x800FC0A0`
  and `0x800FBFB0` are entered with the previous call's return value in `r3`.
* Row 1 and row 7 are the two bodies that retire wave 94's and wave 126's
  paired-single rejections; `docs/COVERAGE_WAVE94_BASES.md:115-121` and
  `docs/COVERAGE_WAVE126_BASES.md:112-118` should be read as superseded for
  `0x806DA520` and `0x8066DDC0` once this wave's translations land.
* `docs/SPEC_WAVE147_BASES.md:88-90`'s list is consumed; this wave publishes
  the next eight in "The eight" above.
