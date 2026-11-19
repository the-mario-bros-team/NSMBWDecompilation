# Wave 150 - `game::bases` - the next eight (0.69-0.66 KB band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified. The only file written is this one. No merge or
check tool was run. The throwaway decoder and the ELF/literal readers live under
`%TEMP%\opencode\w150` (`ppc.js`, `dump2.js`, `an2.js`, `data2.js`, `callers.js`,
`enc.js`, `cdump.js`), never in the repository.

Waves 128-149 walked this bucket from the top in size order; the last row claimed
by `docs/SPEC_WAVE149_BASES.md` is 708 bytes. This wave continues one rung lower,
**708 bytes down to 676 bytes**, which is the eight-row set
`docs/SPEC_WAVE149_BASES.md:100-102` predicted verbatim. Every row is decoded
instruction by instruction, gets a field map, a numbered behaviour list, a
constant table, its callees and an exact signature.

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x80775D00` | 708 | 177 | `80775d00_FUN_80775d00.c` | ready (high) |
| 2 | `0x80626DB0` | 704 | 176 | `80626db0_FUN_80626db0.c` | ready (medium) |
| 3 | `0x8070F370` | 692 | 173 | `8070f370_FUN_8070f370.c` | ready (high) |
| 4 | `0x806D1EF0` | 684 | 171 | `806d1ef0_FUN_806d1ef0.c` | ready (high) |
| 5 | `0x807BC310` | 684 | 171 | `807bc310_FUN_807bc310.c` | ready (medium) |
| 6 | `0x8078FC80` | 680 | 170 | `8078fc80_FUN_8078fc80.c` | ready (high) |
| 7 | `0x807A6BE0` | 680 | 170 | `807a6be0_FUN_807a6be0.c` | ready (high) |
| 8 | `0x80680FD0` | 676 | 169 | `80680fd0_FUN_80680fd0.c` | ready (high) |

**Ready: 8. Conditional: 0.**

Every `bytes` value is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim and `decompiled` is
`true` for all eight. Seven of the eight sizes are the true instruction span:
the decoder finds exactly one `blr`, at `address + size - 4`
(`0x80775FC0`, `0x8070F620`, `0x806D2198`, `0x807BC5B8`, `0x8078FF24`,
`0x807A6E84`, `0x80681270`), and no earlier one. **Row 2 is the exception**: its
TSV extent stops at `0x8062706C` (`addi r1,r1,64`) and the terminating
`blr` is the very next word at `0x80627070`, so the real body is **708 bytes /
177 words** (`0x80626DB0..0x80627074`); the next TSV row starts at
`0x80627080`. This is exactly the "size is an extent" caveat in
`docs/REL_ELF_MAPPING.md:37-42` and is called out again in section 2.

There is no padding tail, no `halt_baddata` cut and no unconditional branch out of
any extent. The decoder has **no** unknown word in the 1,377 words of the eight
bodies (0 unknown opcodes, 0 unhandled X-form, 0 unhandled paired-single form):
every word is a `ppc.js` table hit. Every `b`/`bc` target is inside its own body
or is a tail call to a named callee.

### Why all eight are ready

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image or out of `main.dol`, every callee is named by address (or by its
  ledger name), and the body can be written as code with no assumed calling
  convention beyond the standard PowerPC EABI. Data in `main.dol` BSS
  (`0x8035xxxx`, `0x8037xxxx`, `0x8042xxxx`, `0x8043xxxx`), in this REL's BSS
  (`0x80822900..0x80834D84`), or in another REL is an *input*, named by address.
* **conditional** - exactly one named input would still be missing.

### The three old rejections retired

None of the three rows that earlier lanes rejected has a genuine blocker:

* **`0x806D1EF0`** was left by `docs/COVERAGE_WAVE28_BASES.md:189` because it
  "calls the truncated `0x806D1ED0` and reads the `0x802DCD70` register-save
  thunk's return; needs an ABI review". The raw bytes at `0x806D1ED0` are seven
  words and end in `b 0x800B3980` (opcode 18, `LK = 0`) - a **tail call**, not a
  truncated body. `r3` is `(s8)this[+0x430]`, `f1 = [0x807CB4F0] = 5.0f`,
  `f2 = [0x807CB4F4] = 1.0f`; there is no `0x802DCD70` return involved at all
  (`0x806D1EF0`'s own `_savegpr_26` at `0x802DCD70` is what the export saw).
  Section 4 transcribes the tail call.
* **`0x807A6BE0`** was rejected by `docs/COVERAGE_WAVE73_BASES.md:133` and
  `docs/COVERAGE_WAVE99_BASES.md:119` as "the paired-single matrix/fade body".
  The raw body contains **no data-side `psq`**: the only paired-single words are
  the `psq_st f31/f30/f29/f28` + `psq_l f31..f28` prologue/epilogue spills, all
  `W = 0`, `qr0`, scalar-float type 0. There is no `mfspr gqr`, no `ldexpf`
  lane extraction and no `CONCAT44` conversion anywhere in the 680 bytes. The
  `unaff_GQR0`/`ldexpf` text in the export is Ghidra noise from the spill pair.
  Section 7 transcribes every float operation.
* **`0x8070F370`** was flagged by `docs/COVERAGE_WAVE29_BASES.md:211` for "a
  dropped argument at the `0x8070F630` call". `0x8070F630` is a `(this)`-only
  helper (its own export is `void FUN_8070f630(int param_1)`) and the call site
  `0x8070F3D4` sets only `r3`; nothing is dropped. Section 3 transcribes the
  call.

### This is the list wave 149 predicted, exactly

Re-deriving the candidate set from the current tree gives **1,880** rows: the
5,615 `functions.tsv` rows, filtered against the 17,005 distinct addresses in
`ledger/functions.csv` and the 1,572 distinct `0x`/bare addresses that already
appear in a `#{1,6} ` heading of `docs/*.md`, and against `decompiled=true`.
`docs/SPEC_WAVE149_BASES.md:76` computed the same 1,880 and published the first
eight of this wave at `:100-102`:

```
0x80775D00 708 | 0x80626DB0 704 | 0x8070F370 692 | 0x806D1EF0 684
0x807BC310 684 | 0x8078FC80 680 | 0x807A6BE0 680 | 0x80680FD0 676
```

The 684-byte tie is ordered `0x806D1EF0` before `0x807BC310` and the 680-byte
tie `0x8078FC80` before `0x807A6BE0`, both by address ascending, exactly as wave
149 said. The 676-byte tie with `0x806C1040` is broken the same way, so
`0x80680FD0` is row 8. Each of the eight was re-tested against
`ledger/functions.csv` (exact `address` field), against every `#{1,6} ` heading
in `docs/*.md` (including the bare and lowercase spellings), and against
`src/game/bases.rs` (any `fn_<addr>` token absent). All eight are absent from all
three, and each has a `decompiled=true` TSV row with a matching `.c` export on
disk. SHA-1 of the eight bodies against every same-size `d_basesNP` row: **no
duplicate**.

Applying the same filter after this wave, the next eight are:
`0x806C1040` (676), `0x806C7C20` (668), `0x8070B550` (668), `0x8065DA00` (660),
`0x807B87F0` (660), `0x80682AA0` (656), `0x8068F220` (656), `0x80638120` (648).

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
data; `0x80822900..0x80834D84` is this REL's BSS and reads as zero at file
level, which matters for rows 3 and 5 (they pass BSS scratch pointers).

`main.dol`-resident data (`0x8031C9B0`, `0x8030A040`, `0x80320840`,
`0x803214B0`) is read from `work/dol/main.elf` through its own program headers:
segment `p_offset 0x2FA900 / p_vaddr 0x802FE3A0` covers
`0x802FE3A0..0x80351640`, which includes every `0x8031xxxx`/`0x8032xxxx`
constant used here. Code in `0x800xxxxx` (used only to *name* callees) lives in
segment `p_offset 0x2B00 / p_vaddr 0x80006780`; the two segments do not overlap
and the reader applies each rule directly rather than a `-base` shortcut.

**Decoder.** Reused the wave-146/wave-143 throwaway decoder
(`%TEMP%\opencode\w150\ppc.js`) and re-validated it against the facts
`docs/REL_ELF_MAPPING.md` pins: branch `LK` is bit 0 and `AA` bit 1;
conditional branches use the 14-bit `BD`; the `SPR` field is
`((word >> 11) & 0x1F) << 5 | ((word >> 16) & 0x1F)`; `psq_l` = 56 and
`psq_st` = 60 with the quantiser in bits 12-14 and `W` in bit 15; single-float
`fmuls`/`fdivs` use `FRC`. No decoder extension was needed for this pass; the
eight bodies contain only `addi/addis/ori/oris/xori/xoris/mulli/mulhw/srawi/
rlwinm/neg/subf/add/or/and/cmp/cmpl/extsb/extsh/lbz/lhz/lha/lwz/stb/sth/stw/
lfs/lfd/stfs/stfd/stwu/stwux/lwzu/lwzx/psq_l/psq_st/fmuls/fadds/fsubs/fdivs/
fmr/fneg/fabs/frsp/fcmpo/fcmpu/fctiwz/mfcr/mtcrf/mtspr/mfspr/b/bl/bc/bclr/
bcctr/bctrl/blr`, all of which are in the table.

**CodeWarrior helpers.** Only two rows use them:

| row | prologue | helper | registers saved | `this` kept in |
|--:|---|---|---|---|
| 4 | `0x806D1EFC` | `_savegpr_26` = `0x802DCD70` | `r26..r31` | `r28` |
| 6 | `0x8078FC90` | `_savegpr_26` = `0x802DCD6C`/`0x802DCD68` pair | `r25..r31` | `r29` |

`0x806D1EF0`'s epilogue is `_restgpr_26` at `0x802DCDBC`; `0x8078FC80`'s is
`0x802DCDB8`. The helpers write through `r11` and never touch `r3`, so the
incoming `this` survives. Rows 1, 2, 3, 5, 7 and 8 have no saved-GPR helper and
take `r3` directly (`r29`/`r30`/`r28`/`r31` as noted per section).

**Class attribution.** Five of the eight can be named from data in the produced
image that no earlier document used; three are named from their own dispatch
tables:

* `0x80817320` is a 12-byte-stride state table whose entries begin
  `{0x807718B0, 0, 0}`, `{0x80771870, 0, 0}`, `{0x807717A0, 0, 0}`; the
  class-data block immediately after it is `0x80817370 "F7C0"`,
  `0x80817378 "W7C0"`, `0x808173B0 "g3d/model.brres"`, `0x808173C0 "note"`,
  `0x808174A0 "g3d/peach.brres"`, `0x808174B0 "peach"`,
  `0x80817580 "cobPeachCastleFlag"`, `0x80817594 "cobPeachCastleOpen"`,
  `0x808175A8 "cobPeachCastleClose"`, `0x808175D8 "cobPeachCastle"`,
  `0x808175E8 "FlagA0"`. Row 1's state body is dispatched from `0x80772080`
  inside `0x807718B0`, i.e. it is a body of that world-map class.
* `0x807DB230` holds `{2, 0, "Wm_2d_movieopen", "Wm_2d_moviecoinkira"}` and the
  20-byte-stride handler block `0x807DB264..` whose entries include
  `0x80626AA0`, `0x80626DA0`, `0x80626CD0`, `0x80626D60`; the state-name
  strings `0x807DB405 "bsPdModelPlayDate_c::StateID_ActionWait"` and
  `0x807DB430 "dModelPlayDate_c::StateID_AnimeEndWait"` are the sibling table's
  names. Row 2 is a `dModelPlayDate_c`-family 2D-layout body.
* `0x80806638` is a plain vtable whose first word is `0x8070F370`; its
  neighbours (`0x807100B0`, `0x8070FAB0`, `0x8070F7C0`, `0x80710010`,
  `0x80710090`, `0x807100A0`) are the same block, and
  `docs/COVERAGE_WAVE29_BASES.md:211` already names it `boss_kameck_block`.
* `0x807FAB4C` is a `{handler, 0, -1}` table whose third entry is `0x806D2330`
  and whose neighbours `0x807FAB61 "daLiftRemoconSeesaw_c::StateID_Move"`,
  `0x807FABCC "mark"`, `0x807FABD4 "tire"`, `0x807FABDC "g3d/remo_lift_line.brres"`,
  `0x807FABF8 "remo_lift_line"`, `0x807FAC08 "change_mark"` name the class. Row 4
  is the pointer-angle stepper of `daLiftRemoconSeesaw_c`, called from its
  state-3 body `0x806D2390`.
* `0x80820F38` is another `{handler, 0, -1}` table; its first record is
  `{0x807BC310, 0, -1}` and the following records include `0x807BC2E0`,
  `0x807BC740`, `0x807BC660`, `0x807BC5D0`, `0x807BC9D0`, `0x807BC7E0`. Row 5
  is the first body of that table. `docs/SPEC_WAVE141_BASES.md:243` records that
  `0x808341B8` (`dWorldSelect_c::StateID_Select`) dispatches to
  `0x807BC5C0`, `0x807BC310`, `0x807BC2E0`, so row 5 is a `dWorldSelect_c`
  select-state body.
* `0x8081A8E8..0x8081A9CC` is a resource/name block: `"CS_W5_Ibara_AB"`,
  `"CS_W5_Ibara_AC"`, `"CS_W5_Ibara_BC"`, `"CS_W5_Ibara_CA"`,
  `"CS_W5_Ibara_CB"`, `0x8081A950 "g3d/model.brres"`, `0x8081A960 "CS_W5"`,
  `0x8081A968 "CS_W5_Ibara"`, `0x8081A974 "W502"`, `0x8081A97C "W5G0"`,
  `0x8081A984 "W503"`, `0x8081A98C "W505"`. Row 6 is the cut-scene `CS_W5`
  controller/pad-mode mapper.
* `0x807D0580` holds `0x8081D324 "F031"` at +0x48 and the neighbouring
  `0x8081D338 "dCsSeqMng_c::m_allocator"`, `0x8081D358 "cameraA"`; row 7 is a
  layer/matrix writer of the `dCsSeqMng_c` cut-scene sequence manager.
* `0x807ECDCC` is `{0x80680FD0, 0, -1, 0x80680FB0}` followed by
  `0x807ECDDC "daFloorDokanEight_c::StateID_Move"`, `0x807ECE44 "circle_ground"`,
  `0x807ECE54 "circle_ground_chika"`, `0x807ECE68 "circle_ground_koopa"`,
  `0x807ECF18 "circle_ground_TEST"`. Row 8 is exactly
  `daFloorDokanEight_c::StateID_Move`.

**Unexported callees.** `0x801C1650`, `0x802BE470`, `0x802C6AE0`, `0x800DB7D0`,
`0x800F9370`, `0x800F9840`, `0x800B3BE0`, `0x8006A4B0`, `0x800D8B00`,
`0x80198C30`, `0x80105240`, `0x80162D50` and `0x8070F630` have a named export in
this tree (ledger `game::*`/`resource::*`/`runtime::*` rows) even where Ghidra's
prototype is lossy; they are written by ledger name below. `0x800F2A20`-style
address-only callees do not appear in these eight bodies.

**Call notation.** Throughout this document `callN(addr, a, b, ...)` lists the
integer argument registers `r3, r4, ...` from left to right and treats the callee
as returning `f32` in `f1` when the value is used as a float. Where a callee takes
a float in `f1` in addition to (or instead of) `r3`/`r4`, the call is written
`addr(f1 = ..., r3 = ...)` explicitly. This matters for `0x8007FA40` (12
arguments: 8 GPR plus `f1..f4`), `0x80057820` (`r3 = player`, `f1 = value`),
`0x800DB7D0` (`r3 = node`, `r4 = index`, `f1..f5`), `0x802BF420`
(`r3 = node`, `f1..f4`) and `0x8015F4F0` (`r3 = s16*`, `r4 = target s16`,
`r5 = step`).

---

## 1. `0x80775D00` - castle pop-in state of the world-map manager

**Evidence.** `work/ghidra/rel-d_basesNP/export/80775d00_FUN_80775d00.c`,
708 bytes, 177 words, synthetic `0x80775D00`. Body `0x80775D00..0x80775FC4`
(ends `blr`). No GPR save helper; `r29 = this`, `r30 = result`, `r31 = 0x807CF388`.

**What it is.** A five-arm state body of the world-map manager that "pops" one
object onto the map: it seeds a 3-vector and a 3-vector delta, spawns actor
`0x29E` (670) after a 380-frame delay, submits world-map effect `98` at the
spawned actor, ramps the delta to 1.0 over 20 frames, holds it for 20 frames and
returns 1 once the animated vector has settled within 0.1 of the delta.

The state word is `+0x6C508`, i.e. `addis r4,r3,7` then `-0x3AF8`. The same
`this` is the object the route player `0x80773210` and the record installer
`0x80775190` run on (`docs/SPEC_WAVE147_BASES.md:743-779`): the caller
`0x807718B0` uses `+0x338C`/`+0x1A0` on it. The state body is reached from
`0x80772080` inside that vtable entry.

### Signature

```rust
pub unsafe fn fn_80775d00(this: usize) -> u32
```

`r3` is the world-map manager; the return is `0` while the pop is running and
`1` on the frame the settle test passes. The return is also `0` for every state
other than `5`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x6C508` | `u32` | state: `0`, `1`, `2`, `3`, `5` |
| `+0x6C44C` | `s32` | countdown for the current state |
| `+0x6C4F0` | `f32` | pop vector x |
| `+0x6C4F4` | `f32` | pop vector y |
| `+0x6C4F8` | `f32` | pop vector z |
| `+0x6C4FC` | `f32` | delta x |
| `+0x6C500` | `f32` | delta y |
| `+0x6C504` | `f32` | delta z |

Globals: `0x8042A27C` (`u32` world-map effect-manager pointer, the same one
`docs/SPEC_WAVE147_BASES.md:776` uses), `0x8031C9B0` (main.dol timing table).

### Exact behaviour

Prologue `0x80775D00..0x80775D2C`: frame `0x70`; `vrsave` to `+0x74`;
`r29 = r3`; `r31 = 0x807CF388`; `r30 = 0`; `r4 = this + 0x70000`.

1. `0x80775D2C..0x80775D58` - dispatch on `state = [r4 - 0x3AF8]`:
   `state == 0` -> step 2; `1` -> step 3; `2` -> step 4; `3` -> step 5;
   `5` -> step 6; anything else -> step 7.
2. **State 0** (`0x80775D5C..0x80775DA8`):
   ```rust
   let k = read_f32(0x807CF45C);          // 0.01f
   write_f32(this, 0x6C4F0, k);           // pos x
   write_f32(this, 0x6C4F4, k);           // pos y
   write_f32(this, 0x6C4F8, k);           // pos z
   write_f32(this, 0x6C4FC, k);           // delta x
   write_f32(this, 0x6C500, k);           // delta y
   write_f32(this, 0x6C504, k);           // delta z
   write_s32(this, 0x6C44C, read_s16(0x8031C9B0 + 0x54));  // 380
   write_u32(this, 0x6C508, 1);
   ```
   The six `stfs` to `r1+0x2C..0x40` in this arm are dead stores (the slots are
   never read on any path); a faithful Rust body may drop them, and this
   document records them only so the byte-match check is explainable.
3. **State 1** (`0x80775DAC..0x80775E1C`):
   ```rust
   if read_s32(this, 0x6C44C) > 0 {
       write_s32(this, 0x6C44C, read_s32(this, 0x6C44C) - 1);
   } else {
       let a = call2(0x80162D50, 670, 0);          // spawn actor 0x29E
       let p = [read_f32(a, 0xAC), read_f32(a, 0xB0), read_f32(a, 0xB4)];
       call4(0x80105240, read_u32(0x8042A27C), 98, &p, 1);   // wm_effect_record_submit
       write_s32(this, 0x6C44C, read_s16(0x8031C9B0 + 0x56)); // 20
       write_u32(this, 0x6C508, 2);
   }
   ```
   Actor `670` is the id of `"cobKoopaCastle"` in the world-map object table
   `0x80817010` (`{char* name, u32 id}` pairs; the pair `0x808171D4` /
   `0x808171D8` is `"cobKoopaCastle"` / `0x0000029E`).
4. **State 2** (`0x80775E20..0x80775E98`):
   ```rust
   if read_s32(this, 0x6C44C) > 0 {
       write_s32(this, 0x6C44C, read_s32(this, 0x6C44C) - 1);
       let step = read_f32(0x8031C9B0 + 0x5C) / (read_s16(0x8031C9B0 + 0x56) as f32);
       let v = read_f32(this, 0x6C4FC) + step;
       write_f32(this, 0x6C4FC, v);
       write_f32(this, 0x6C500, v);
       write_f32(this, 0x6C504, v);
   } else {
       write_s32(this, 0x6C44C, read_s16(0x8031C9B0 + 0x58)); // 20
       write_u32(this, 0x6C508, 3);
   }
   ```
   With the shipped table that is `1.0f / 20.0f = 0.05f` per frame, so the delta
   ramps `0.01 -> 1.01` while all three components stay equal.
5. **State 3** (`0x80775E98..0x80775EFC`):
   ```rust
   if read_s32(this, 0x6C44C) > 0 {
       write_s32(this, 0x6C44C, read_s32(this, 0x6C44C) - 1);
       let d = (read_f32(0x8031C9B0 + 0x5C) - read_f32(0x8031C9B0 + 0x60))
             / (read_s16(0x8031C9B0 + 0x58) as f32);   // (1.0 - 1.0) / 20 = 0.0
       let v = read_f32(this, 0x6C4FC) - d;
       write_f32(this, 0x6C4FC, v);
       write_f32(this, 0x6C500, v);
       write_f32(this, 0x6C504, v);
   } else {
       write_u32(this, 0x6C508, 5);
   }
   ```
6. **State 5** (`0x80775F08..0x80775F2C`):
   ```rust
   let len = { call2(0x801C1650, this + 0x6C4F0, this + 0x6C4FC); call0_f32(0x802BE470) };
   if len < read_f32(0x807CF460) { r30 = 1; }   // 0.1f
   ```
   `0x801C1650` is the translated runtime helper that subtracts two 3-vectors
   into the global scratch (see `docs/COVERAGE_WAVE104_GX_BAND.md:46`, where it is
   "turned into a length"); `0x802BE470` returns its magnitude.
7. **Common tail** (`0x80775F2C..0x80775FBC`), executed on every arm:
   ```rust
   let kp = read_f32(0x807CF464);   // 0.9f
   let kd = read_f32(0x807CF460);   // 0.1f
   let px = read_f32(this, 0x6C4F0) * kp + read_f32(this, 0x6C4FC) * kd;
   let py = read_f32(this, 0x6C4F4) * kp + read_f32(this, 0x6C500) * kd;
   let pz = read_f32(this, 0x6C4F8) * kp + read_f32(this, 0x6C504) * kd;
   write_f32(this, 0x6C4F0, px);
   write_f32(this, 0x6C4F4, py);
   write_f32(this, 0x6C4F8, pz);
   return r30;
   ```
   The eight `stfs` to `r1+0x08..0x28` at `0x80775F68..0x80775FB4` are dead
   stores (the same "keep the intermediate for the debugger" artifact as in
   step 2).
8. `0x80775F8C..0x80775FC4`: restore `r31/r30/r29`, `vrsave`, `lr`; return.

### Constants

| address | value | use |
|---|---|---|
| `0x807CF45C` | `0.01f` | initial pos/delta (state 0) |
| `0x807CF460` | `0.1f` | delta step; settle threshold |
| `0x807CF464` | `0.9f` | position damping |
| `0x807CF468` | `f64 4503599627370496.0` (0x4330000080000000) | u32->f64 magic |
| `0x8031C9B0 + 0x54` | `s16 380` | state-1 delay |
| `0x8031C9B0 + 0x56` | `s16 20` | state-2 length / divisor |
| `0x8031C9B0 + 0x58` | `s16 20` | state-3 length / divisor |
| `0x8031C9B0 + 0x5C` | `f32 1.0` | state-2 step numerator |
| `0x8031C9B0 + 0x60` | `f32 1.0` | state-3 step numerator |

### Callees

| target | name | shape |
|---|---|---|
| `0x80162D50` | `resource::registry::fn_80162d50` | `(actor_id, 0) -> *mut Actor` |
| `0x80105240` | `game::worldmap::wm_effect_record_submit` | `(mgr, id, f32*, flag)` |
| `0x801C1650` | `runtime::dvd::fn_801c1650` | `(dst, src) -> scratch` |
| `0x802BE470` | `resource::archive::fn_802be470` | `() -> f32` magnitude |

---

## 2. `0x80626DB0` - 2D layout counter/icon ticker of the model-play-date screen

**Evidence.** `work/ghidra/rel-d_basesNP/export/80626db0_FUN_80626db0.c`,
704 bytes in the TSV, **708 bytes of real body** (`0x80626DB0..0x80627074`),
177 words. The terminating `blr` is at `0x80627070`, one word past the TSV
extent; the next TSV row (`0x80627080`) is eight bytes later. No GPR helper;
`r30 = this`, `r31 = the message manager`.

**What it is.** The per-frame text/icon update of the "model play date" screen's
message window. It is gated by a byte `+0x27A` that another body sets, prints the
running counter as decimal text into the window at `+0x254` while the counter is
`<= 19`, and switches to one of five icon/animation styles at `+0x258` once the
counter passes 19. It also stages three "fade-in" panels at `+0x244`, `+0x260`,
`+0x264`, `+0x270` depending on `+0x294`, and finally submits the "coin" message
from the id table `0x807C6FB8`.

### Signature

```rust
pub unsafe fn fn_80626db0(this: usize)
```

`r3` is the screen object. The two call sites (`0x80626CFC`, `0x80627384`) pass
only `r3`; `r4..r12` are dead.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x244` | `*mut Layout` | left panel |
| `+0x248` | `*mut Layout` | counter layout |
| `+0x24C` | `u32` | sprite/anm argument for `0x800B3B60` |
| `+0x250` | `*mut Layout` | digit layout |
| `+0x254` | `*mut Layout` | counter-text pane |
| `+0x258` | `*mut Layout` | icon pane |
| `+0x260` | `*mut Layout` | icon pane 2 |
| `+0x264` | `*mut Layout` | fade panel A |
| `+0x270` | `*mut Layout` | fade panel B |
| `+0x27A` | `u8` | "tick this frame" trigger (cleared on entry) |
| `+0x284` | `u32` | submit argument / decrescent |
| `+0x288` | `u32` | variant (7 selects style 27) |
| `+0x28C` | `u32` | counter value |
| `+0x294` | `u32` | panel phase 0..3 |
| `+0x298` | `u32` | coin message index |

Every `*mut Layout` is a byte buffer: the engine's toggles are `lbz +0xBB`,
`rlwinm +0xBB,0,24,30`, optional `ori 1`, `stb +0xBB` (bit 0 of `+0xBB` is the
"visible" bit), plus `stb 0xFF,+0xB8` and `stfs` to `+0x44`/`+0x48`.

### Exact behaviour

Prologue `0x80626DB0..0x80626DD0`: frame `0x40`; save `r31`, `r30`;
`r30 = this`; `r31 = call0_u32(0x800CDC60)` (returns the global message manager
`[0x80359CB4]`).

1. `0x80626DCC..0x80626DDC`: `if *(u8*)(this + 0x27A) == 0 { return; }`;
   `*(u8*)(this + 0x27A) = 0`.
2. `0x80626DDC..0x80626DFC`:
   ```rust
   let n = read_u32(this, 0x288) + 1;
   call3(0x800B3BE0, &n, read_u32(this, 0x250), 1);   // bind digit layout to value
   ```
   The value bound to the digit layout is `+0x288 + 1`; the *text* below uses
   `+0x28C + 1`. They are two different counters.
3. `0x80626E00..0x80626E10`: `if read_u32(this, 0x28C) <= 19` -> step 4, else
   step 5.
4. **Counter arm** (`0x80626E0C..0x80626E9C`):
   ```rust
   if let Some(p) = read_u32(this, 0x254) { *(u8*)(p + 0xBB) = (*(u8*)(p + 0xBB) & 0xFE) | 1; }
   if let Some(p) = read_u32(this, 0x258) { *(u8*)(p + 0xBB) &= 0xFE; }
   let mut v = read_u32(this, 0x28C) + 1;
   let mut digits = 1i32;
   while v >= 10 { v /= 10; digits += 1; }        // 0x80626E50..0x80626E68
   let mut buf = [0u8; 16];
   call5(0x800CDDB0, read_u32(this, 0x28C) + 1, &mut buf, 16, digits, 0); // itoa
   call6(0x800C9A60, read_u32(this, 0x254), r31, 302, 22, 1, &buf);       // set text, style 22
   ```
   `0x800CDDB0` is the translated integer-to-string helper; the `crxor` word at
   `0x80626E94` is the varargs FP-flag clear the PowerPC EABI requires, not a
   data dependency.
5. **Icon arm** (`0x80626EA0..0x80626F28`):
   ```rust
   if let Some(p) = read_u32(this, 0x254) { *(u8*)(p + 0xBB) &= 0xFE; }
   if let Some(p) = read_u32(this, 0x258) { *(u8*)(p + 0xBB) |= 1; }
   let c = read_u32(this, 0x28C);
   let mut style: u32 = /* incoming r6, see the note */;
   if c.wrapping_sub(21) <= 1 { style = 23; }
   if c.wrapping_sub(23) <= 1 {
       style = 24;
       if read_u32(this, 0x288) == 7 { style = 27; }
   }
   if c == 20 { style = 25; }
   if c == 37 { style = 26; }
   call6(0x800C9A60, read_u32(this, 0x258), r31, 302, style, 0, 0);
   ```
   > **Note (the one residual of this row).** For `c` in `25..=36` no arm
   > assigns `style`; the original forwards the incoming `r6` for those
   > counters. Both call sites (`0x80626CFC`, `0x80627384`) reach this body with
   > `r6` unset, so the shipped game can only be relying on the counter never
   > resting in `25..=36`. The counter is bounded by the `37` sentinel and the
   > `20..24` thresholds; a translation should treat `25..=36` as unreachable and
   > use `style = 0` there (documented deviation, not a missing input).
6. `0x80626F28..0x80626F3C`:
   ```rust
   call4(0x800B3B60, read_u32(this, 0x284), 0x807DB230, read_u32(this, 0x24C), 1);
   ```
   `0x807DB230` is the `"Wm_2d_movieopen"` data block (`{2, 0, "Wm_2d_movieopen",
   "Wm_2d_moviecoinkira"}` followed by the handler table).
7. `0x80626F40..0x8062702C` - panel phasing on `read_u32(this, 0x294)`:
   ```rust
   match phase {
       0 => { clear_bit0(this, 0x244); return; }
       1 => { set_bit0(this, 0x244); set_bit0(this, 0x260); return; }
       3 => { set_bit0(this, 0x244); set_bit0(this, 0x260);
              clear_bit0(this, 0x264); clear_bit0(this, 0x270); }
       _ => { set_bit0(this, 0x244); set_bit0(this, 0x260);
              set_bit0(this, 0x264); set_bit0(this, 0x270);
              *(u8*)(read_u32(this,0x264) + 0xB8) = 0xFF;
              *(f32*)(read_u32(this,0x270) + 0x44) = 1.0f;
              *(f32*)(read_u32(this,0x270) + 0x48) = 1.0f;
              *(u8*)(read_u32(this,0x270) + 0xB8) = 0xFF; }
   }
   ```
   The two `stfs f0,16(r1)` / `stfs f0,20(r1)` at `0x80626FEC`/`0x80627000` are
   dead stores; the live stores are `0xFF` at `+0xB8` of both panels and `1.0f`
   (`0x807C6FC8`) at `+0x44`/`+0x48` of the `+0x270` panel.
8. `0x80627030..0x8062705C`:
   ```rust
   let id = read_u32(0x807C6FB8 + read_u32(this, 0x298) * 4);   // {4,1,3,2}
   call6(0x800C9A60, read_u32(this, 0x248), r31, 302, id, 0, 0);
   ```
9. `0x8062705C..0x80627074`: restore `r31/r30`, `vrsave`, `lr`;
   `addi r1,r1,0x40`; `blr` (`0x80627070`).

### Constants

| address | value | use |
|---|---|---|
| `0x807C6FB8` | `u32[4] {4, 1, 3, 2}` | coin message ids |
| `0x807C6FC8` | `1.0f` | panel alpha |
| `0x807DB230` | `{2, 0, ptr, ptr}` | `"Wm_2d_movieopen"` block |
| `302` (`0x12E`) | message bank id | all `0x800C9A60` calls |
| `22`, `23`, `24`, `25`, `26`, `27` | style ids | counter/icon arms |

### Callees

| target | name | shape |
|---|---|---|
| `0x800CDC60` | `game::stage::fn_800cdc60` | `() -> u32` (`[0x80359CB4]`) |
| `0x800B3BE0` | `game::stage::fn_800b3be0` | `(u32*, layout*, u32)` |
| `0x800B3B60` | `game::stage::fn_800b3b60` | `(u32, data*, layout*, u32)` |
| `0x800CDDB0` | `game::stage::fn_800cddb0` | `(u32, buf, len, digits, 0)` itoa |
| `0x800C9A60` | `game::stage::fn_800c9a60` | `(layout*, mgr, bank, style, flag, text)` |

---

## 3. `0x8070F370` - `boss_kameck_block` placement/create

**Evidence.** `work/ghidra/rel-d_basesNP/export/8070f370_FUN_8070f370.c`,
692 bytes, 173 words, synthetic `0x8070F370`. Body `0x8070F370..0x8070F624`
(ends `blr`). No GPR helper; `r30 = this`, `r31 = 0x807CCA78`.

**What it is.** Vtable slot 0 of `boss_kameck_block` (`0x80806638[0]`). The
object is one of 29 blocks of the Kameck boss arena; the index `+0x694`
(`*(u8*)(this + 4)`) selects one 20-byte record of the placement table
`0x80805EA0` (`[0] = 29`, entries at `+4`, stride 20). The function copies the
record into `+0x7B0`, resets the yaw angles and the scale, derives two
extents/effect sizes from the record, installs the block's animator (or a null
one when the index is the "hidden" 29th), and ends with a virtual call that
binds a BSS model workspace.

### Signature

```rust
pub unsafe fn fn_8070f370(this: usize) -> u32
```

Returns `1` on every path (`li r3,1` at `0x8070F604`).

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x04` | `u32` | packed flags; `& 0xFF` is the block index |
| `+0x100`,`+0x102`,`+0x104` | `s16` | yaw/pitch/roll, zeroed |
| `+0x0AC`,`+0x0B0`,`+0x0B4` | `f32` | position |
| `+0x0D0`,`+0x0D4`,`+0x0D8` | `f32` | render extents |
| `+0x0DC`,`+0x0E0`,`+0x0E4` | `f32` | scale, all `1.0f` |
| `+0x0E8`,`+0x0EC`,`+0x0F0` | `f32` | all `0.0f` |
| `+0x318`,`+0x31C` | `f32` | effect extents |
| `+0x38F` | `u8` | actor variant passed to the animator |
| `+0x5B0` | animator | installed by `0x8007FA40` |
| `+0x5FC` | `u32` | `0` |
| `+0x600` | `u32` | `0` |
| `+0x604` | `u32` | `0x807103C0` |
| `+0x680` | `u32` | flags, `|= 0x20` |
| `+0x694` | `u32` | block index (copied from `+4 & 0xFF`) |
| `+0x7B0` | `*const u8` | placement record (`0x80805EA4 + idx*0x14`) |
| `+0x1610` | `f32` | record `+4` (`0.0f`) |

### Exact behaviour

Prologue `0x8070F370..0x8070F3AC`: frame `0x30`; save `r31..r28`;
`r30 = this`; `r31 = 0x807CCA78`; `r5 = 0x80805EA0`.

1. `0x8070F3A4..0x8070F3C4`:
   ```rust
   let idx = read_u32(this, 0x04) & 0xFF;
   write_u32(this, 0x694, idx);
   let rec = 0x80805EA0 + idx * 20;
   write_u32(this, 0x7B0, rec + 4);
   if idx < read_u32(0x80805EA0) { call1(0x8070F630, this); }  // 29-entry guard
   ```
   `0x8070F630` is the translated `(this)` helper that snaps the block to its
   record; it is *not* called for `idx == 29` (the hidden block).
2. `0x8070F3D8..0x8070F3FC`:
   ```rust
   let one = read_f32(0x807CCA78);       // 1.0f
   write_s16(this, 0x100, 0); write_s16(this, 0x102, 0); write_s16(this, 0x104, 0);
   write_f32(this, 0x0DC, one); write_f32(this, 0x0E0, one); write_f32(this, 0x0E4, one);
   ```
3. `0x8070F400..0x8070F42C`:
   ```rust
   let (a, b) = if idx < read_u32(0x80805EA0) {
       (read_u32(rec + 0x0C), read_u32(rec + 0x10))
   } else { (0, 0) };
   ```
4. `0x8070F42C..0x8070F49C`:
   ```rust
   let zero = read_f32(0x807CCA78 + 0x04);      // 0.0f
   let z    = read_f32(0x807CCA78 + 0x08);      // -256.0f
   write_f32(this, 0x0AC, zero);
   write_f32(this, 0x0B0, zero);
   write_f32(this, 0x0B4, z);
   let count = read_u32(0x80805EA0) as f32;     // 29.0
   write_f32(this, 0x0D0, (a as f32) * 8.0);    // 0.5 * 16
   write_f32(this, 0x0D4, count * 8.0);         // 232.0
   write_f32(this, 0x0D8, zero);
   write_f32(this, 0x0E8, zero);
   write_f32(this, 0x0EC, zero);
   write_f32(this, 0x0F0, zero);
   ```
5. `0x8070F4A0..0x8070F52C` - animator install:
   ```rust
   if idx < read_u32(0x80805EA0) {
       call12(0x8007FA40, this + 0x5B0, this,
              0x80710120, 0x807101F0, 0x80710290, 3,
              read_u8(this, 0x38F), 0,
              f1 = 0.0f, f2 = 0.0f,
              f3 = (a as f32) * 16.0, f4 = (read_u32(0x80805EA0) as f32) * 16.0);
       write_u32(this, 0x5FC, 0);
       write_u32(this, 0x680, read_u32(this, 0x680) | 0x20);
       write_u32(this, 0x600, 0);
       write_u32(this, 0x604, 0x807103C0);
       call1(0x8007F900, this + 0x5B0);
   } else {
       call12(0x8007FA40, this + 0x5B0, this, 0, 0, 0, 3,
              read_u8(this, 0x38F), 0,
              f1 = 0.0f, f2 = 0.0f, f3 = 0.0f, f4 = 0.0f);
   }
   ```
   The `f3`/`f4` operands are the record `a` and the table count, scaled by
   `16.0` (`0x8070F4C0..0x8070F4EC`); the "dropped argument" that
   `docs/COVERAGE_WAVE29_BASES.md:211` suspected is the `f1..f4` bank, which the
   call site loads explicitly.
6. `0x8070F560..0x8070F5B4`:
   ```rust
   write_f32(this, 0x320, read_f32(this, 0x0D0));
   write_f32(this, 0x324, read_f32(this, 0x0D4));
   write_f32(this, 0x318, (a as f32) * 16.0);
   write_f32(this, 0x31C, -((b as f32) * 16.0));
   write_f32(this, 0x1610, read_f32(0x807CCA78 + 0x04));   // 0.0f
   ```
7. `0x8070F5B8..0x8070F600`:
   ```rust
   let arg = if idx < read_u32(0x80805EA0) { 0x8082F788u32 } else { 0x8082F848u32 };
   call3(vcall(this, 0x60, 0xD4), this, arg);      // BSS model workspace bind
   return 1;
   ```
   Both `0x8082F788` and `0x8082F848` are inside the REL's BSS
   (`0x80822900..0x80834D84`), i.e. zero-filled static workspaces; they must be
   modelled as static `*mut u8`, not as ROM data.
8. `0x8070F600..0x8070F624`: restore `r31..r28`, `vrsave`, `lr`; return `1`.

### Constants

| address | value | use |
|---|---|---|
| `0x807CCA78` | `1.0f` | scale |
| `0x807CCA7C` | `0.0f` | position / z-target |
| `0x807CCA80` | `-256.0f` | position z |
| `0x807CCA84` | `0.5f` | extent half |
| `0x807CCA88` | `16.0f` | extent scale |
| `0x807CCA90` | `f64 4503599627370496.0` | u32->f64 magic |
| `0x80805EA0` | `u32 29` | record count |
| `0x80805EA4 + idx*0x14` | 5 words | `{w0, w1, w2, w3, w4}` placement record |

### Callees

| target | name | shape |
|---|---|---|
| `0x8070F630` | `game::bases::fn_8070f630` | `(this)` |
| `0x8007FA40` | `game::stage::fn_8007fa40` | 12-arg animator install |
| `0x8007F900` | `game::stage::fn_8007f900` | `(animator)` |
| `vcall(this, +0x60, +0xD4)` | class virtual | `(this, *mut u8)` |

---

## 4. `0x806D1EF0` - `daLiftRemoconSeesaw_c` pointer-angle stepper

**Evidence.** `work/ghidra/rel-d_basesNP/export/806d1ef0_FUN_806d1ef0.c`,
684 bytes, 171 words, synthetic `0x806D1EF0`. Body `0x806D1EF0..0x806D219C`
(ends `blr`). Prologue `_savegpr_26` at `0x802DCD70`, epilogue
`_restgpr_26` at `0x802DCDBC`; `r28 = this`, `r29 = player mask`.

**What it is.** The see-saw platform of the "remote-control lift" object tracks
the Wii-remote pointer: it computes a target angle from the rider byte `+0x430`,
clamps it to `±0x1A2C` (`±6700`), steps the current angle `+0x54A` toward it in
`0x600` (1536) units with `0x8015F4F0` (`step_short_clamped_wrap`), plays the
`683` cue and a tick every eight `0x600` crossings, rumbles all four players and
mirrors the angle into the model yaw `+0x104`.

### Signature

```rust
pub unsafe fn fn_806d1ef0(this: usize, players: u32)
```

`r3` is the see-saw; `r4` is a 4-bit player mask (`r31 & 0xFF` at the call site
`0x806D24D8`). Returns nothing.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x104` | `s16` | model yaw (`-angle`) |
| `+0x430` | `s8` | rider slot, `-1` = none |
| `+0x530` | `u32` | "moving" latch |
| `+0x534` | `u32` | tick count, capped at 8 |
| `+0x53C` | `u32` | cleared on tick-overflow |
| `+0x54A` | `s16` | current pointer angle |
| `+0x54C` | `s16` | pointer centre angle |
| `+0x54E` | `s16` | current step target |
| `+0x550` | `s16` | last step target |
| `+0x552` | `u16` | 3-frame debounce |
| `+0x556` | `u8` | "was outside" latch |
| `+0x558` | `u8` | re-arm timer |

### Exact behaviour

Prologue `0x806D1EF0..0x806D1F04`: frame `0x30`; `_savegpr_26`; `r28 = r3`,
`r29 = r4`.

1. `0x806D1F04..0x806D1F28` - debounce:
   ```rust
   let t = read_u16(this, 0x552);
   if t != 0 { write_u16(this, 0x552, t - 1); }
   if read_u16(this, 0x552) != 0 { goto step 9; }
   ```
2. `0x806D1F2C..0x806D1F4C` - rider gate:
   ```rust
   let mut want = false;
   if (read_u8(this, 0x430) as i8) < 0 { goto step 6 with target = 0; }
   if read_u8(this, 0x556) != 0 { want = true; goto step 6; }
   ```
3. `0x806D1F50..0x806D1FC4` - band test:
   ```rust
   let centre = read_s16(this, 0x54C);
   let hi = centre.wrapping_add(910) as s16;
   let lo = centre.wrapping_sub(910) as s16;
   let target = call0_s16(0x806D1ED0, this);   // 0x800B3980(f1=5.0, f2=1.0, s8[+0x430])
   if (centre / 1536) != 0 {
       write_u8(this, 0x556, 1);
       want = true;                            // r30 stays 1 through the rest
   }
   if hi <= target {
       write_u8(this, 0x556, 1); want = true;
   } else if !(lo < target) {
       write_u8(this, 0x556, 1); want = true;
   }
   // equivalently: want |= !(lo < target && target < hi)
   ```
   `0x806D1ED0` is the seven-word tail call
   `return 0x800B3980(f1 = [0x807CB4F0], f2 = [0x807CB4F4], r3 = (s8)this[+0x430])`.
4. `0x806D1FC0..0x806D1FCC`: for the `+0x556 != 0` fast path `want = true`.
5. `0x806D1FC4..0x806D1FCC`: `if !want { goto step 9; }`
6. `0x806D1FCC..0x806D1FF4`:
   ```rust
   let target = call0_s16(0x806D1E70, this);          // clamped ±0x1A2C
   let target_index = target / 0x600;
   ```
7. `0x806D1FF4..0x806D2058` - step selection (only while `+0x530 == 0`):
   ```rust
   let mut moved = false;
   if read_u32(this, 0x530) == 0 {
       let q = read_s16(this, 0x54A) / 0x600;
       if target_index != q {
           // 0x806D2024..0x806D2044: next = q + sign(target_index - q)
           let next = if target_index > q { q + 1 } else { q - 1 };
           write_u32(this, 0x530, 1);
           write_s16(this, 0x54E, (next as s16).wrapping_mul(0x600));
           moved = true;
           goto step 9;                       // 0x806D2058 skips the tick block
       }
       goto step 8;
   }
   moved = true;                              // 0x806D20A4
   goto step 9;
   ```
8. `0x806D205C..0x806D20A4` - tick counter, only on the `target_index == q`
   (no-move) path:
   ```rust
   if target_index != read_s16(this, 0x550) {
       let n = read_u32(this, 0x534) + 1;
       write_u32(this, 0x534, n);
       if n >= 8 {
           write_u32(this, 0x534, 8);
           call3(0x800B3720, read_u8(this, 0x430) as i8, 0, 0);
           write_u32(this, 0x53C, 0);
       }
   }
   ```
9. `0x806D20A8..0x806D20C0`:
   ```rust
   let t = read_u8(this, 0x558);
   if t != 0 { write_u8(this, 0x558, t - 1); }
   write_s16(this, 0x550, target_index);
   if !moved { goto step 12; }
   ```
10. `0x806D20C8..0x806D2114` - step:
    ```rust
    if call3(0x8015F4F0, this + 0x54A, read_s16(this, 0x54E), 560) != 0 {
        write_u16(this, 0x552, 3);
        write_u32(this, 0x530, 0);
        let mut p = [0f32; 3];
        call2(0x8006A4B0, &mut p, this + 0xAC);
        call4(0x80198C30, read_u32(0x80429D60), 683, &p, 0);
        if (read_u8(this, 0x430) as i8) >= 0 && read_u8(this, 0x558) == 0 {
            write_u8(this, 0x558, 4);
            for i in 0..4 {
                if players & (1 << i) != 0 {
                    call6(0x800D8B00, read_u32(0x8042A008), i as i8, 9, 2, 0, 0);
                }
            }
        }
    }
    ```
11. `0x806D2178..0x806D2180`:
    ```rust
    write_s16(this, 0x104, (read_s16(this, 0x54A) as i16).wrapping_neg());
    ```
12. `0x806D2184..0x806D219C`: `_restgpr_26`, `vrsave`, `lr`; return.

The `0x806D1E70` helper (already translated) is
`0x800B3980(f1 = 5.0f, f2 = 1.0f, r3 = (s8)this[+0x430])`, clamped to
`[-0x1A2B, 0x1A2C]` (`docs/FUNCTION_INDEX.md:2850` and the export).

### Constants

| address | value | use |
|---|---|---|
| `0x807CB4F0` | `5.0f` | `0x800B3980` f1 |
| `0x807CB4F4` | `1.0f` | `0x800B3980` f2 |
| `0x807CB4EC` | `s16 0x1A2C` (6700) | clamp bound |
| `1536` (`0x600`) | step quantum | angle quantisation |
| `910` (`0x38E`) | angle band | target window |
| `560` | `0x8015F4F0` step | per-frame slew |
| `8` | tick cap | `0x800B3720` trigger |
| `3` | debounce frames | `+0x552` |
| `683` (`0x2AB`) | sound id | `0x80198C30` |
| `9`, `2` | rumble args | `0x800D8B00` |

### Callees

| target | name | shape |
|---|---|---|
| `0x806D1ED0` | code in this REL | tail call to `0x800B3980` |
| `0x800B3980` | unexported | `(f1, f2, i8) -> s16` |
| `0x806D1E70` | `game::bases::fn_806d1e70` | `(this) -> s16` |
| `0x8015F4F0` | `game::task::step_short_clamped_wrap` | `(s16*, target, step) -> u32` |
| `0x800B3720` | `game::stage::fn_800b3720` | `(i8, 0, 0)` |
| `0x8006A4B0` | `runtime` helper | `(f32*, f32*)` vector copy |
| `0x80198C30` | `audio::ax::fn_80198c30` | `(mgr, id, f32*, 0)` |
| `0x800D8B00` | `engine::display::fn_800d8b00` | `(mgr, slot, 9, 2, 0, 0)` rumble |

---

## 5. `0x807BC310` - `dWorldSelect_c` select-state body

**Evidence.** `work/ghidra/rel-d_basesNP/export/807bc310_FUN_807bc310.c`,
684 bytes, 171 words, synthetic `0x807BC310`. Body `0x807BC310..0x807BC5BC`
(ends `blr`). No GPR helper; `r28 = this`, `r30 = [0x80429F50]`,
`r31 = 0x808340E8` (a BSS scratch base, not ROM).

**What it is.** The world-select "Select" state body (first record of the
`{handler,0,-1}` table `0x80820F38`; `docs/SPEC_WAVE141_BASES.md:243` reaches it
from `0x808341B8`). It decides between the confirm cue `122` and the reject cue
`1631`, then either advances the selected slot (`+0x27C`) into `+0x298`/`+0x280`
or toggles the "hold" byte `+0x2B5` through `0x807BB8D0`.

### Signature

```rust
pub unsafe fn fn_807bc310(this: usize)
```

`r3` is the world-select screen.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x60` | `*mut VTable` | class vtable (`+0xD4` is the state-name setter) |
| `+0x20C` | state object | vtable `+0x18` = set state |
| `+0x27C` | `u32` | selected slot (`& 0xFF` used) |
| `+0x280` | `u32` | pending slot / `8` |
| `+0x284` | `u32` | hold bit word |
| `+0x298` | `u32` | confirmed slot |
| `+0x2B0` | `u8` | "can hold" |
| `+0x2B5` | `u8` | hold flag (computed) |
| `+0x2B6` | `u8` | "state changed" (cleared on entry) |

### Exact behaviour

Prologue `0x807BC310..0x807BC350`: frame `0x20`; save `r31..r28`;
`r28 = this`; `r30 = [0x80429F50]`; `r31 = 0x808340E8`;
`r29 = read_u32(this, 0x27C) & 0xFF`; `*(u8*)(this + 0x2B6) = 0`.

1. `0x807BC354..0x807BC36C`:
   ```rust
   let obj = read_u32(0x8042A468);
   if call3(vcall(obj, 0, 0x20), obj, 0x900) != 0 { goto step 2; }
   if call1(0x800B53A0, 0) != 0 { goto step 3; }
   let i = read_u32(0x8042A464);
   let p = read_u32(read_u32(r30 + i * 4) + 4);     // pad record
   if (read_u32(p, 0x1C) & 0x2000) == 0 { goto step 4; }
   ```
2. `0x807BC370..0x807BC3B0` - slot compare:
   ```rust
   let cur = call0_u32(0x807714F0) & 0xFF;
   if (r29 & 0xFF) != cur {
       call3(vcall(this + 0x20C, 0x18), this + 0x20C, 0x808340E8 + 0x210);
       call3(0x80195380, read_u32(0x8042A488), 1631, 1);
       return;
   }
   call3(0x80195380, read_u32(0x8042A488), 122, 1);
   call3(vcall(this + 0x20C, 0x18), this + 0x20C, 0x808340E8 + 0x1D0);
   return;
   ```
3. `0x807BC414..0x807BC440`: the "reject" arm (same two calls as step 2's
   first branch, with `0x808340E8 + 0x1D0`).
4. `0x807BC444..0x807BC47C` - slot 8 confirm:
   ```rust
   if r29 == 8 {
       if (flags & 1) == 0 { return; }
       write_u32(this, 0x280, read_u32(this, 0x298));
       *(u8*)(this + 0x2B6) = 1;
       call3(vcall(this + 0x20C, 0x18), this + 0x20C, 0x808340E8 + 0x110);
       return;
   }
   ```
5. `0x807BC480..0x807BC4C0` - bit 1:
   ```rust
   if (flags & 2) != 0 && *(u8*)(this + 0x2B0) != 0 {
       write_u32(this, 0x298, r29);
       write_u32(this, 0x280, 8);
       *(u8*)(this + 0x2B6) = 1;
       call3(vcall(this + 0x20C, 0x18), this + 0x20C, 0x808340E8 + 0x110);
       return;
   }
   ```
6. `0x807BC4C4..0x807BC52C` - bit 3 (`0x8` in the pad record):
   ```rust
   if (pad_flags & 0x8) != 0 {
       if call3(0x807BB8D0, this, r29, 0) != 0 {
           let x = read_u32(this, 0x284);
           let a = x ^ 1;
           *(u8*)(this + 0x2B5) = ((((a >> 1) - (a & x)) as i32) >> 31) as u8 & 1;
           call3(vcall(this + 0x20C, 0x18), this + 0x20C, 0x808340E8 + 0x110);
           return;
       }
       r29 = read_u32(this, 0x27C);
   }
   ```
   The shift/and/sub sequence is the compiler's sign-bit extraction; it is `0`
   for `x ∈ {0,1}` and `1` for every other `x` (verified over the 32-bit range
   in the throwaway decoder), so the pair of `xori/srawi/and/subf/rlwinm` words
   is `((a >> 1) - (a & x)) < 0`.
7. `0x807BC530..0x807BC59C` - bit 2 (`0x4` in the pad record):
   ```rust
   if (pad_flags & 0x4) != 0 {
       if call3(0x807BB8D0, this, r29, 1) != 0 {
           let x = read_u32(this, 0x284);
           let a = x ^ 1;
           *(u8*)(this + 0x2B5) = ((((a >> 1) - (a & x)) as i32) >> 31) as u8 & 1;
           call3(vcall(this + 0x20C, 0x18), this + 0x20C, 0x808340E8 + 0x110);
       }
   }
   ```
8. `0x807BC59C..0x807BC5BC`: restore `r31..r28`, `vrsave`, `lr`; return.

### Constants

| address | value | use |
|---|---|---|
| `0x80429F50` | `*mut PadManager` | pad array |
| `0x8042A464` | `u32` | pad index |
| `0x8042A468` | `*mut Object` | "expansion present" object |
| `0x8042A488` | `*mut Audio` | sound manager |
| `0x808340E8 + 0x110` | BSS state name | `+0x2B6` path |
| `0x808340E8 + 0x1D0` | BSS state name | confirm path |
| `0x808340E8 + 0x210` | BSS state name | reject path |
| `122` (`0x7A`) | cue | confirm |
| `1631` (`0x65F`) | cue | reject |
| `0x900` | vcall arg | expansion query |

### Callees

| target | name | shape |
|---|---|---|
| `0x807714F0` | `game::bases::fn_807714f0` | `() -> u32` current slot |
| `0x800B53A0` | `game::stage::fn_800b53a0` | `(0)` pad-status query (export prints `void`; caller tests `r3`) |
| `0x807BB8D0` | `game::bases` | `(this, slot, mode) -> u32` |
| `0x80195380` | `audio::ax::fn_80195380` | `(mgr, id, 1)` cue |
| `0x800B3720` | see row 4 | not used here |

---

## 6. `0x8078FC80` - `CS_W5_Ibara` controller/pad-mode mapper

**Evidence.** `work/ghidra/rel-d_basesNP/export/8078fc80_FUN_8078fc80.c`,
680 bytes, 170 words, synthetic `0x8078FC80`. Body `0x8078FC80..0x8078FF28`
(ends `blr`). Prologue `_savegpr_26` pair at `0x802DCD68`/`0x802DCD6C`,
epilogue `0x802DCDB8`; `r29 = the pad manager`, `r30 = 0x8081A8A8`,
`r28 = current slot`.

**What it is.** The cut-scene `CS_W5` (world-5 "Ibara") input-mode mapper. It
reads the pad manager, builds the current controller's name string with
`0x800F9370`, classifies the controller from `0x800F9840`, and writes the
"pointer mode" word into two per-slot arrays (`+0x320`, `+0x348`) through
`0x800BBC50`/`0x800BBC60`, depending on the name (`"W502"`, `"W5G0"`, `"W503"`,
`"W505"`) and the pad sub-type `[pad + 0x60]`.

### Signature

```rust
pub unsafe fn fn_8078fc80(this: usize)
```

`r3` is unused beyond the prologue (the body works entirely on the pad manager);
`this` is not stored.

### Field map (pad manager `[0x80429F7C]`)

| offset | type | meaning |
|---:|---|---|
| `+0x54` | `u32` | player index (`& 0xFF`) |
| `+0x60` | `u32` | pad sub-type |
| `+0x320 + slot*4` | `u32` | mode A (read `0x800BBC70`, write `0x800BBC50`) |
| `+0x348 + slot*4` | `u32` | mode B (write `0x800BBC60`) |

### Exact behaviour

Prologue `0x8078FC80..0x8078FCA4`: frame `0x30`; `_savegpr_26`;
`r29 = [0x80429F7C]`; `r30 = 0x8081A8A8`; `r28 = call0_u32(0x807714F0) & 0xFF`;
`r27 = read_u32(r29, 0x54) & 0xFF`; `r31 = call1_u32(0x800F9840, r27)`;
`r26 = read_u32(r29, 0x60)`; `r25 = call2_u32(0x800BBC70, r29, r28)`;
`call3(0x800F9370, r28, r27, &mut name)`.

1. `0x8078FCE4..0x8078FD44` - controller class gate:
   ```rust
   if (r31 - 4) <= 1 || r31 == 7 {
       call3(0x800BBC60, r29, r28, r25);
       call3(0x800BBC50, r29, r28, r25);
       return;
   }
   ```
2. `0x8078FD44..0x8078FDDC` - `"W502"` / `"W5G0"`:
   ```rust
   if (strcmp(&name, 0x8081A974 /* "W502" */) == 0
       || strcmp(&name, 0x8081A97C /* "W5G0" */) == 0) && r26 == 1 {
       call3(0x800BBC60, r29, r28, r25);
       match r25 {
           1 | 2 => call3(0x800BBC50, r29, r28, 3),
           3     => call3(0x800BBC50, r29, r28, 1),
           _     => {}
       }
       return;
   }
   ```
3. `0x8078FDDC..0x8078FE74` - `"W503"` / `"W505"`:
   ```rust
   if (strcmp(&name, 0x8081A984) == 0 || strcmp(&name, 0x8081A98C) == 0) && r26 == 1 {
       call3(0x800BBC60, r29, r28, r25);
       match r25 {
           1 | 2 => call3(0x800BBC50, r29, r28, 3),
           3     => call3(0x800BBC50, r29, r28, 2),
           _     => {}
       }
       return;
   }
   ```
4. `0x8078FE74..0x8078FF0C` - sub-type fallback:
   ```rust
   if r26.wrapping_sub(1) > 4 {
       if r26 != 0 { return; }
       call3(0x800BBC60, r29, r28, r25);
       call3(0x800BBC50, r29, r28, r25);
       return;
   }
   call3(0x800BBC60, r29, r28, r25);
   match r25 {
       1 => call3(0x800BBC50, r29, r28, 2),
       2 => call3(0x800BBC50, r29, r28, 3),
       3 => call3(0x800BBC50, r29, r28, 1),
       _ => {}
   }
   ```
5. `0x8078FF10..0x8078FF24`: `_restgpr_26`, `vrsave`, `lr`; return.

The string compares use `0x802E1AB4`, whose export is the engine's `strcmp`
(byte loop with a word-at-a-time fast path), so the `r3`/`r4` order is
`(buffer, literal)`.

### Constants

| address | value | use |
|---|---|---|
| `0x80429F7C` | `*mut PadManager` | all pad access |
| `0x8081A974` | `"W502"` | name 1 |
| `0x8081A97C` | `"W5G0"` | name 2 |
| `0x8081A984` | `"W503"` | name 3 |
| `0x8081A98C` | `"W505"` | name 4 |
| `0x800F9370` | name builder | writes `W` + slot + digits |
| `0x800F9840` | class of pad | `0..11` |

### Callees

| target | name | shape |
|---|---|---|
| `0x807714F0` | `game::bases::fn_807714f0` | `() -> u32` |
| `0x800F9840` | unexported | `(player) -> u32` class |
| `0x800F9370` | unexported | `(mode, player, buf)` name |
| `0x802E1AB4` | `engine::main_loop::fn_802e1ab4` | `strcmp` |
| `0x800BBC70` | `game::stage::fn_800bbc70` | `(mgr, slot) -> u32` |
| `0x800BBC60` | `game::stage::fn_800bbc60` | `(mgr, slot, v)` |
| `0x800BBC50` | `game::stage::fn_800bbc50` | `(mgr, slot, v)` |

---

## 7. `0x807A6BE0` - cut-scene layer matrix/fade writer (`dCsSeqMng_c` cluster)

**Evidence.** `work/ghidra/rel-d_basesNP/export/807a6be0_FUN_807a6be0.c`,
680 bytes, 170 words, synthetic `0x807A6BE0`. Body `0x807A6BE0..0x807A6E88`
(ends `blr`). Frame `0x80`; saves `f31..f28` with `stfd` + `psq_st` and restores
with `psq_l` + `lfd` at `0x807A6E50..0x807A6E6C`. `r29 = this`,
`r30 = [0x80429E00]`, `r31 = 0x807D0580`.

**What it is.** The per-frame layout writer for the cut-scene sequence manager's
two 3D "screens" (`+0x180` and `+0x268`) and their base pose `+0x354..+0x374`.
It copies the 9-float pose to `+0x15C..+0x17C`, derives a camera-relative
quad (using `[0x80429E00] + 0x40/+0x44`), builds the NW4R matrix at `+0x70`,
pushes the layer transforms, sets the fade rectangles at `+0x244..+0x264` and
`+0x32C..+0x34C`, and marks the object dirty (`+0xA4 |= 1`).

The resource block `0x807D0580 + 0x48` is `0x8081D324 "F031"` and the
neighbouring strings are `0x8081D338 "dCsSeqMng_c::m_allocator"`,
`0x8081D358 "cameraA"`, `0x8081D430 "otasukeInfo_18_inWindow.brlan"` - the
cut-scene manager's own data.

### Signature

```rust
pub unsafe fn fn_807a6be0(this: usize)
```

Returns nothing; `f1` is consumed only inside.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x70` | matrix | NW4R model matrix |
| `+0x78`,`+0x7C` | `f32` | matrix translation y/z |
| `+0x98`,`+0x9C`,`+0xA0` | `f32` | layer translation |
| `+0xA4` | `u16` | dirty flags (`|= 1`) |
| `+0xB4` | `f32` | camera-derived y |
| `+0x15C..+0x17C` | `f32[9]` | copied pose |
| `+0x180` | node | fade node A |
| `+0x1E0` | node | matrix node A |
| `+0x244..+0x254` | `f32[5]` | fade rect A (all `0.0`) |
| `+0x258` | `f32` | `-100.0` |
| `+0x25C..+0x264` | `f32[3]` | pose tail |
| `+0x268` | node | fade node B |
| `+0x2C8` | node | matrix node B |
| `+0x32C..+0x334` | `f32[3]` | cached layer vector |
| `+0x338` | `f32` | same as `+0x32C` |
| `+0x33C`,`+0x340` | `f32` | cached z - 100 |
| `+0x344..+0x34C` | `f32[3]` | pose tail |
| `+0x354..+0x374` | `f32[9]` | source pose |

### Exact behaviour

Prologue `0x807A6BE0..0x807A6C28`: frame `0x80`; `vrsave`; `stfd/psq_st`
`f31,f30,f29,f28`; `r29 = this`; `r30 = [0x80429E00]`;
`r31 = 0x807D0580`.

1. `0x807A6C2C..0x807A6C70` - pose copy:
   ```rust
   for i in 0..9 { write_f32(this, 0x15C + i*4, read_f32(this, 0x354 + i*4)); }
   ```
2. `0x807A6C74..0x807A6C80`: `call2(vcall(this + 0xF8, 0x14), this + 0xF8)`
   (the `lwzu r12,248(r3)` hidden-`this` update form).
3. `0x807A6C84..0x807A6CC0`:
   ```rust
   let s = read_f32(0x807D0580 + 0x20);          // 0.5f
   let cx = read_f32(r30, 0x40);
   let cy = read_f32(r30, 0x44);
   let f31 =  s * cy;
   let f29 =  s * (-cx);
   let f30 = -f31;
   let f28 = -f29;
   call5(0x802C6AE0, this + 0x70, f1 = f31, f2 = f30, f3 = f29, f4 = f28);
   ```
4. `0x807A6CC4..0x807A6CF0`: `call0(0x800E2580)`; then
   `call4(0x800E26F0, &scratch, f1 = f31, f2 = f30, f3 = f29, f4 = f28)` and
   `call0(0x800E27C0)` - the dSys matrix-stack push/transform/pop triple.
5. `0x807A6CF4..0x807A6D6C`:
   ```rust
   let zero = read_f32(0x807D0580);              // 0.0f
   write_f32(this, 0xB4, f1_from_800E27C0);      // camera y
   write_f32(this, 0x78, read_f32(r1, 0x10));
   write_f32(this, 0x7C, read_f32(r1, 0x14));
   write_f32(this, 0x98, read_f32(r1, 0x18));
   write_f32(this, 0x9C, read_f32(r1, 0x1C));
   write_f32(this, 0xA0, read_f32(r1, 0x20));
   write_u16(this, 0xA4, read_u16(this, 0xA4) | 1);
   let z = read_f32(this, 0x36C);
   let y = read_f32(this, 0x370);
   let x = read_f32(this, 0x374);
   for i in 0..5 { write_f32(this, 0x244 + i*4, zero); }
   write_f32(this, 0x258, zero - read_f32(0x807D0580 + 0x24));  // -100.0
   write_f32(this, 0x25C, z);
   write_f32(this, 0x260, y);
   write_f32(this, 0x264, x);
   ```
6. `0x807A6D6C..0x807A6D7C`: `call2(vcall(this + 0x1E0, 0x14), this + 0x1E0)`.
7. `0x807A6D7C..0x807A6D98`:
   ```rust
   call5(0x802BF420, this + 0x180,
         f1 = read_f32(0x8042A0F0), f2 = 0.0f, f3 = 0.0f,
         f4 = read_f32(0x8042A0F0));
   ```
8. `0x807A6D98..0x807A6DD0` - the one-time global layer cache:
   ```rust
   if *(u8*)(0x808333A0) == 0 {
       write_f32(0x808333A4, read_f32(0x807D0580 + 0x28));   // 512.0
       write_f32(0x808333A8, read_f32(0x807D0580 + 0x2C));   // -640.0
       write_f32(0x808333AC, read_f32(0x807D0580 + 0x00));   // 0.0
       *(u8*)(0x808333A0) = 1;
   }
   ```
9. `0x807A6DD0..0x807A6E30`:
   ```rust
   let a = read_f32(0x808333A4);
   let b = read_f32(0x808333A8);
   let c = read_f32(0x808333AC);
   for i in 0..3 { write_f32(this, 0x32C + i*4, [a,b,c][i]); }
   write_f32(this, 0x338, a);
   write_f32(this, 0x33C, b);
   write_f32(this, 0x340, c - read_f32(0x807D0580 + 0x24));   // -100.0
   write_f32(this, 0x344, read_f32(this, 0x36C));
   write_f32(this, 0x348, read_f32(this, 0x370));
   write_f32(this, 0x34C, read_f32(this, 0x374));
   call2(vcall(this + 0x2C8, 0x14), this + 0x2C8);
   ```
10. `0x807A6E30..0x807A6E48`:
    ```rust
    call5(0x802BF420, this + 0x268,
          f1 = read_f32(0x8042A0F0), f2 = 0.0f, f3 = 0.0f,
          f4 = read_f32(0x8042A0F0));
    ```
11. `0x807A6E4C..0x807A6E88`: restore `f31..f28`, `r31/r30/r29`, `vrsave`,
    `lr`; return.

### Constants

| address | value | use |
|---|---|---|
| `0x807D0580 + 0x00` | `0.0f` | fade rect / layer z |
| `0x807D0580 + 0x20` | `0.5f` | camera half-extent |
| `0x807D0580 + 0x24` | `100.0f` | fade offset |
| `0x807D0580 + 0x28` | `512.0f` | layer cache x |
| `0x807D0580 + 0x2C` | `-640.0f` | layer cache y |
| `0x807D0580 + 0x48` | `0x8081D324 "F031"` | resource name |
| `0x808333A0` | `u8` flag | layer cache init |
| `0x808333A4..0xAC` | `f32[3]` | layer cache |
| `0x8042A0F0` | `f32` | fade value |
| `0x800E2580/26F0/27C0` | dSys | matrix stack |

### Callees

| target | name | shape |
|---|---|---|
| `0x802C6AE0` | unexported | `(node, f1..f4)` matrix setup |
| `0x800E2580` | `engine::dsys::fn_800e2580` | `()` stack push |
| `0x800E26F0` | `engine::dsys::fn_800e26f0` | `(out, f1..f4)` transform |
| `0x800E27C0` | `engine::dsys::fn_800e27c0` | `() -> f32` stack pop |
| `0x802BF420` | `resource::archive::fn_802bf420` | `(node, f1..f4)` |
| `vcall(this, +0xF8, +0x14)` | node update | `(node)` |
| `vcall(this, +0x1E0, +0x14)` | node update | `(node)` |
| `vcall(this, +0x2C8, +0x14)` | node update | `(node)` |

---

## 8. `0x80680FD0` - `daFloorDokanEight_c::StateID_Move`

**Evidence.** `work/ghidra/rel-d_basesNP/export/80680fd0_FUN_80680fd0.c`,
676 bytes, 169 words, synthetic `0x80680FD0`. Body `0x80680FD0..0x80681274`
(ends `blr`). Frame `0x40`; saves `f31` with `stfd` + `psq_st` and restores with
`psq_l` + `lfd`; `_savegpr_27` at `0x802DCD74`, epilogue `0x802DCDC0`;
`r30 = this`, `r31 = 0x807C9620`.

**What it is.** The "move" state of the eight-pipe floor object. It rotates 24
`s16` tilt angles (`+0x36F4..+0x3724`, three arrays of eight) and the object
yaw `+0x104` by the signed step `+0x3732`, re-places the eight pipe controllers
at `+0x3D4` (stride `0x74`) with radius `400.0` and height `48.0`, flushes 48
`0xE4`-byte geometry blocks (`+0xC34..+0x36F4`), and finally pushes the
per-player weight `+0x3734[i]` into the player parameters when `+0x372C` is set.

### Signature

```rust
pub unsafe fn fn_80680fd0(this: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x104` | `s16` | object yaw |
| `+0x348` | `u8` | tilt direction (`0` = +step, else `-step`) |
| `+0x3D4 + k*0x74` (k<8) | controller | pipe controller nodes |
| `+0xC34 + n*0xE4` (n<48) | block | geometry blocks |
| `+0x36F4..+0x3724` | `s16[24]` | pipe tilt angles |
| `+0x372C` | `u32` | "apply player weights" flag |
| `+0x3732` | `s16` | tilt step |
| `+0x3734..+0x3740` | `f32[4]` | per-player weight |

### Exact behaviour

Prologue `0x80680FD0..0x80681004`: frame `0x40`; `stfd/psq_st f31`;
`_savegpr_27`; `r30 = this`; `r31 = 0x807C9620`; `call1(0x80680CD0, this)`;
`call1(0x80680E80, this)`.

1. `0x80681004..0x806810B4` - tilt step, direction `1` (`+0x348 != 0`):
   ```rust
   let d = read_s16(this, 0x3732);
   write_s16(this, 0x104, read_s16(this, 0x104).wrapping_sub(d));
   for block in 0..3 {
       for j in 0..8 {
           let o = 0x36F4 + block*0x10 + j*2;
           write_s16(this, o, read_s16(this, o).wrapping_sub(d));
       }
   }
   ```
   The loop is a `ctr = 3` `bdnz` with `r3 += 0x10` per pass; the inner body is
   fully unrolled over eight halves. The three passes therefore cover
   `+0x36F4..+0x3702`, `+0x3704..+0x3712` and `+0x3714..+0x3722`, i.e. the
   `s16[24]` span `+0x36F4..+0x3724`. The displacement is `-d` in this arm.
2. `0x806810B8..0x8068115C` - direction `0` (`+0x348 == 0`): the same loops with
   `add` instead of `sub` on `+0x104` and on all 24 halves.
3. `0x8068115C..0x806811DC` - pipe controllers:
   ```rust
   let base = (read_f32(r31 + 0x40) as i32) + read_s16(this, 0x104); // 400.0 -> 400
   for k in 0..8 {
       let node = this + 0x3D4 + k*0x74;
       call6(0x800DB7D0, node, base + k*8192,
             f1 = 400.0, f2 = 48.0, f3 = -400.0, f4 = 48.0, f5 = -400.0);
       call1(vcall(node, 0, 0x0C), node);
       call1(vcall(node, 0, 0x10), node);
   }
   ```
   The `f32 -> i32` conversion is `fctiwz` on `[0x807C9644] = 16768.0`, so the
   per-pipe angular offset is `+ k*0x2000` in 16-bit angle units.
4. `0x806811DC..0x8068120C` - geometry flush:
   ```rust
   for n in 0..24 { call1(0x8007FDA0, this + 0xC34  + n*0xE4); }
   for n in 0..24 { call1(0x8007FDA0, this + 0x2194 + n*0xE4); }
   ```
   The two runs are contiguous (`0xC34 + 24*0xE4 = 0x2194`,
   `0x2194 + 24*0xE4 = 0x36F4`), i.e. 48 blocks up to the tilt array.
5. `0x8068120C..0x80681254` - per-player weights:
   ```rust
   if read_u32(this, 0x372C) != 0 {
       for i in 0..4 {
           let w = read_f32(this, 0x3734 + i*4);
           if w != 1.0f {
               let p = call1(0x8005F900, i);   // -> *mut Player (tail call)
               if !p.is_null() { call2(0x80057820, p, f1 = w); }
           }
       }
   }
   ```
   `0x80057820` is the player clamp helper documented in
   `docs/COVERAGE_WAVE4_PLAYER.md:58` (`player->+0x14C4 = clamp(f1, lo, hi)`
   unless `player->+0x10D4 & 1`). `0x8005F900` is `tail 0x80162D00(&DAT_80354E10
   [i])`, i.e. the player-object lookup by slot, so `r3` from it is the player
   pointer the clamp needs.
6. `0x80681254..0x80681274`: restore `f31`, `r27..r31`, `vrsave`, `lr`; return.

### Constants

| address | value | use |
|---|---|---|
| `0x807C9640` | `7456.0f` | unused in this body (sibling) |
| `0x807C9644` | `16768.0f` | `fctiwz` -> 16768 pipe index base |
| `0x807C9660` | `400.0f` | pipe radius |
| `0x807C9664` | `48.0f` | pipe height |
| `0x2000` | `8192` | per-pipe angle stride |
| `0x372C` | `u32` | apply-weights flag |
| `1.0f` | cast | per-player neutral weight |

### Callees

| target | name | shape |
|---|---|---|
| `0x80680CD0` | in this REL | `(this)` (unexported helper) |
| `0x80680E80` | `game::bases::fn_80680e80` | `(this)` |
| `0x800DB7D0` | unexported | `(node, index, f1..f5)` |
| `0x8007FDA0` | `game::stage::fn_8007fda0` | `(block)` flush |
| `0x8005F900` | `game::player::fn_8005f900` | `(slot) -> *mut Player` |
| `0x80057820` | `game::player::fn_80057820` | `(player, f1)` clamp/store |

---

## Verification

* **Selection.** 5,615 `functions.tsv` rows; 1,880 after the ledger (17,005
  distinct addresses) and heading (1,572 distinct addresses) filters; the eight
  above are the eight largest, ties broken by address ascending. Each was
  re-tested against `ledger/functions.csv` (exact `address`), against every
  `#{1,6} ` heading in `docs/*.md` (bare and `0x` spellings), and against
  `src/game/bases.rs` (no `fn_<addr>` token for any of the eight).
* **Exports.** All eight have a `decompiled=true` TSV row and a matching
  `work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` on disk.
* **Duplicate check.** SHA-1 of each body against every same-size `d_basesNP`
  row: no duplicate for any of the eight.
* **Body spans.** Seven bodies end in `blr` at `address + size - 4` with no
  earlier `blr`. Row 2's TSV extent omits the final `blr` (real span 708 bytes);
  documented in section 2 and in the table note.
* **Decode.** 0 unknown words out of 1,377; every literal resolved against
  `work/rels/elf/d_basesNP.elf` or `work/dol/main.elf`.
* **Rejections retired.** Wave 28's `0x806D1ED0` "truncated" claim, wave 73/99's
  `0x807A6BE0` "paired-single" claim and wave 29's `0x8070F370` "dropped
  argument" claim are all shown to be export artifacts, not body gaps.
* **Handoff.** `docs/SPEC_WAVE149_BASES.md:100-102`'s list is consumed exactly;
  this wave publishes `0x806C1040`, `0x806C7C20`, `0x8070B550`, `0x8065DA00`,
  `0x807B87F0`, `0x80682AA0`, `0x8068F220`, `0x80638120` as the next eight.
* **Scope.** `work/ghidra/rel-d_basesNP/export/functions.tsv` and the ELF images
  were parsed read-only; `ledger/functions.csv`, `src/game/bases.rs` and all
  `docs/*.md` other than this file were read, never written. The only artefact
  is `docs/SPEC_WAVE150_BASES.md`; the throwaway decoder lives under
  `%TEMP%\opencode\w150` and is not in the repository.
