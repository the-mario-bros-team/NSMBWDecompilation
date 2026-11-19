# Wave 158 - `game::bases` - the next eight (540-516 byte band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified by this lane. The only file written is this one.
No merge or check tool was run. The throwaway tools live under
`%TEMP%\opencode\w158` (`pick.js`, `dump.js`, `callers.js`, `gaps.js`, `vals.js`,
`tabs.js`, `bytes.js`, `tsvnear.js`, `mdump.js`, plus copies of the wave-156
decoders `d.js`/`md.js`); none of them is in the repository. `d.js` decodes
`work/rels/elf/d_basesNP.elf` through its own program headers, `md.js` is the
same decoder retargeted at `work/dol/main.elf`.

Waves 128-157 walked this bucket from the top in size order; wave 157 ended at
544 bytes and named the next band explicitly
(`docs/SPEC_WAVE157_BASES.md:180-184`): "the ninth is `0x80731D10` (540), then
`0x806D6950` (536), then `0x806E0110` (532), then `0x806D62C0` (528), then
`0x807B7B40` (524), then the 520-byte group `{0x806F2100, 0x806F2710}` and
`0x8061F2F0`/`0x806D5FB0`/`0x806D6680`/`0x80702FD0`/`0x807826B0`/`0x807B8CB0`
(516)". This wave claims exactly the first eight of that list - the last row is
the **first** member (ascending address) of the six-way 516-byte tie:

```
0x80731D10 540 | 0x806D6950 536 | 0x806E0110 532 | 0x806D62C0 528
0x807B7B40 524 | 0x806F2100 520 | 0x806F2710 520 | 0x8061F2F0 516
```

## The eight

| # | address | TSV size | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x80731D10` | 540 | 135 | `80731d10_FUN_80731d10.c` | ready (high) |
| 2 | `0x806D6950` | 536 | 134 | `806d6950_FUN_806d6950.c` | ready (medium) |
| 3 | `0x806E0110` | 532 | 133 | `806e0110_FUN_806e0110.c` | ready (medium) |
| 4 | `0x806D62C0` | 528 | 132 | `806d62c0_FUN_806d62c0.c` | ready (high) |
| 5 | `0x807B7B40` | 524 | 131 | `807b7b40_FUN_807b7b40.c` | **conditional** |
| 6 | `0x806F2100` | 520 | 130 | `806f2100_FUN_806f2100.c` | ready (high) |
| 7 | `0x806F2710` | 520 | 130 | `806f2710_FUN_806f2710.c` | ready (high) |
| 8 | `0x8061F2F0` | 516 | 129 | `8061f2f0_FUN_8061f2f0.c` | ready (medium) |

**Ready: 7. Conditional: 1.** 4,216 bytes / 1,054 words in total.

Every `size` above is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim, `decompiled` is
`true` for all eight and all eight export files exist on disk. Seven extents end
exactly on their own `blr` (`address + size - 4`); row 5 is the
`docs/REL_ELF_MAPPING.md:37-42` extent case and is described in its section:

| # | reachable `blr` | `address + size - 4` | span is exact | next row | gap |
|--:|---|---|:--:|---:|---:|
| 1 | `0x80731F28` | `0x80731F28` | yes | `0x80731F30` | 4 |
| 2 | `0x806D6B64` | `0x806D6B64` | yes | `0x806D6B70` | 8 |
| 3 | `0x806E0320` | `0x806E0320` | yes | `0x806E0720` | 1,020 |
| 4 | `0x806D64CC` | `0x806D64CC` | yes | `0x806D64D0` | 0 |
| 5 | `0x807B7D50` | `0x807B7D48` | **no** (+8) | `0x807B7D60` | 20 |
| 6 | `0x806F2304` | `0x806F2304` | yes | `0x806F2420` | 280 |
| 7 | `0x806F2914` | `0x806F2914` | yes | `0x806F2920` | 8 |
| 8 | `0x8061F4F0` | `0x8061F4F0` | yes | `0x8061F500` | 12 |

Gap contents, read word by word (`%TEMP%\opencode\w158\gaps.js`):

* row 1: one zero word;
* row 2: two zero words;
* row 3: three zero words, then **252 words of code at `0x806E0330`** that has
  neither a TSV row nor an export (`9421fff0 7c0802a6 2c030000 ...`, a
  `stwu r1,-16` prologue). It is a separate body, not part of row 3, and is left
  for a later wave;
* row 4: no gap at all; the next TSV row starts on the very next word;
* row 5: the state-0 worker's own epilogue (`38210030 addi r1,r1,48` at
  `0x807B7D4C`, `4e800020 blr` at `0x807B7D50`), then three zero words. The
  extent stops eight bytes short of that `blr` (see the section);
* row 6: two zero words, then **68 words of code at `0x806F2310`** with no TSV
  row and no export (`9421fff0 7c0802a6 2c030000 ...`). Separate body, out of
  scope;
* row 7: two zero words;
* row 8: three zero words.

The decoder reports **no unknown word** in the 1,054 words: zero unknown
opcodes, zero unhandled X-form, zero unhandled paired-single form, and every
`bl` lands on a named callee outside the body. The only branches that leave
their own body are row 5's three dispatcher tail branches, which go to sibling
TSV rows and are documented as such. `psq` appears in **three rows**, always
`qr0`, always `W = 0`:

| row | `psq` words |
|--:|---|
| 4 | `psq_st f31,72(r1),0,qr0` (prologue) and `psq_l f31,72(r1),0,qr0` (epilogue), i.e. the standard `stfd`/`psq_st` + `psq_l`/`lfd` single-double bracket |
| 6, 7 | `psq_st f31/f30/f29/f28` at 136/120/104/88 and the matching `psq_l` words, four pairs each, all `qr0`, all `W = 0` |
| 1, 2, 3, 5, 8 | none |

The wave-51 rejection of `0x806D62C0` ("unrecovered `GQR0` register and `ldexpf`
bookkeeping", `docs/COVERAGE_WAVE51_BASES.md:84-88`) is therefore an artefact of
Ghidra's model of that bracket: there is exactly one `psq` pair in the body and
it is a plain `f31` spill. `GQR3 = 0x00050005` (`docs/REL_ELF_MAPPING.md:52-56`)
is not needed by anything in this set; no row reads `qr3`.

## Method

**Address mapping.** Re-derived from the program headers of
`work/rels/elf/d_basesNP.elf` (`node d.js ph`), not copied:

```
PT_LOAD  p_offset   p_vaddr     filesz    memsz
  0      0x0010F0   0x806000F0  0x1C6004  0x1C6004
  1      0x001C70F4 0x807C60F4  0x0004FC  0x0004FC
  2      0x1C75F0   0x807C65F0  0x00000C  0x00000C
  3      0x1C7600   0x807C6600  0x00A5FC  0x00A5FC
  4      0x1D1C00   0x807D0C00  0x051CB8  0x051CB8
  5      0x000000   0x80822900  0x000000  0x012484 (zero fill)
```

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) +
0x1000` for every code address below `0x807C60F4`, exactly as
`docs/REL_ELF_MAPPING.md:27-35` states. Every word quoted below was read with
that formula; the Ghidra export was used only as a cross-check.

`main.dol`-resident data and code are read from `work/dol/main.elf` through its
**own** program headers (`node mdump.js ph`):

```
p_offset  p_vaddr     filesz    memsz
0x000200  0x80004000  0x0026C0  0x0026C0
0x002900  0x800066C0  0x000060  0x000060
0x002A00  0x80006720  0x000060  0x000060
0x002B00  0x80006780  0x2E7260  0x2E7260
0x2E9E00  0x802ED9E0  0x0002E0  0x0002E0
0x2EA100  0x802EDCC0  0x000020  0x000020
0x2EA200  0x802EDCE0  0x0106C0  0x0106C0
0x2FA900  0x802FE3A0  0x0532A0  0x0532A0
0x000000  0x80351680  0x000000  0x0D6000  (BSS)
0x34DC00  0x80427680  0x002540  0x002540
0x000000  0x80429BC0  0x000000  0x0014C0  (BSS)
0x350200  0x8042B080  0x004B60  0x004B60
0x000000  0x8042FBE0  0x000000  0x00005C  (BSS)
```

so the per-type record table `0x8030A040`, the stage table base cell
`0x80429E98` (a BSS pointer) and the scene word `0x8042900C` are all reachable,
while `0x80354E50..0x80354EB0`, `0x80358D54` and `0x8042A0xx..0x8042Axxx` are
`main.dol` **BSS** and read as zero at file level.

**Candidate selection (recomputed).** `%TEMP%\opencode\w158\pick.js` re-reads
four sources from scratch:

* `work/ghidra/rel-d_basesNP/export/functions.tsv` - 5,615 data rows, all
  `decompiled = true`, and every export file named in the `file` column exists
  on disk;
* `ledger/functions.csv` - 17,238 distinct `address` fields (case-insensitive).
  The wave-158 translation lane has already landed its 13 rows, so the ledger
  count is 17,238 rather than wave 157's 17,219;
* `src/game/bases.rs` - every `fn fn_<addr>` definition token (3,815);
* every `## N. \`0xADDR\`` heading of **all** 30 `docs/SPEC_WAVE*_BASES.md`
  files (292 distinct addresses). The task names `SPEC_WAVE12*_BASES.md`, of
  which waves 128 and 129 are the only matches; the superset is used so that no
  address already claimed by a wave-130..157 heading can be picked twice.

That leaves **1,771** rows with an export. The eight largest are the table
above. Ties are broken by **ascending address**, the same rule wave 157 used:

* the 540, 536, 532, 528 and 524-byte rows are all singletons;
* the 520-byte row is a two-way tie `{0x806F2100, 0x806F2710}` - both are
  claimed here, which is exactly what `docs/COVERAGE_WAVE84_BASES.md:120-122`
  asked for ("a self-contained cluster ... a single pass should take the whole
  run") and what `docs/COVERAGE_WAVE154_BASES.md:141-143` predicted as the two
  largest open rows of the `0x806F` bucket;
* the 516-byte row is a six-way tie
  `{0x8061F2F0, 0x806D5FB0, 0x806D6680, 0x80702FD0, 0x807826B0, 0x807B8CB0}`;
  ascending address makes `0x8061F2F0` the eighth row here, and the remaining
  five are the natural wave-159 band.

None of the eight is referenced by a `fn fn_<addr>` body, by a ledger row or by
any spec heading. The only prose appearances are earlier lanes' skip lists and
neighbour notes:

* `0x80731D10` - `docs/SPEC_WAVE157_BASES.md:180` (the ninth) only;
* `0x806D6950` - `docs/SPEC_WAVE157_BASES.md:181` only;
* `0x806E0110` - `docs/COVERAGE_WAVE58_BASES.md:130` and
  `docs/COVERAGE_WAVE80_BASES.md:126` (both "rest of the bucket, out of
  window") and `docs/SPEC_WAVE157_BASES.md:181`;
* `0x806D62C0` - `docs/COVERAGE_WAVE51_BASES.md:84-88` (rejected for the
  `GQR0`/`ldexpf` reading, now disproved) and `docs/SPEC_WAVE157_BASES.md:182`;
* `0x807B7B40` - `docs/COVERAGE_WAVE143_BASES.md:123` (listed as "not
  started"), `docs/COVERAGE_WAVE152_BASES.md:134`,
  `docs/SPEC_WAVE146_BASES.md:1094/1106/1174` (the teardown calls it) and
  `docs/SPEC_WAVE157_BASES.md:182`;
* `0x806F2100` - `docs/COVERAGE_WAVE63_BASES.md:41,113-117` (rejected: "the
  `+0xD` flag byte ... would force guesses"), `docs/COVERAGE_WAVE84_BASES.md:120`
  and `docs/COVERAGE_WAVE154_BASES.md:141-143`;
* `0x806F2710` - `docs/COVERAGE_WAVE63_BASES.md:124`,
  `docs/COVERAGE_WAVE84_BASES.md:120`, `docs/COVERAGE_WAVE154_BASES.md:141`,
  `docs/FUNCTION_INDEX.md:3754` (a comment-only mention) and
  `docs/SPEC_WAVE157_BASES.md:183`;
* `0x8061F2F0` - `docs/COVERAGE_WAVE158_BASES.md:138` (the translation lane's
  own "not started: needs its own full decode") and
  `docs/SPEC_WAVE157_BASES.md:183`.

### Why seven are ready and one is not

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image or out of `main.dol`, every callee is named by address (or by its
  ledger name), and the body can be written as code with no assumed calling
  convention beyond the standard PowerPC EABI. Data in `main.dol`
  (`0x800xxxxx`..`0x804xxxxx`), in this REL's data segments, or in this REL's
  BSS is an *input*, named by address.
* **conditional** - exactly one named input would still be missing.

Three rows deserve a paragraph up front, because an earlier lane rejected them
for reasons that no longer hold:

* **Row 4** was rejected by wave 51 for "an unrecovered `GQR0` register and
  `ldexpf` bookkeeping around their paired-single moves". The raw words show
  one `psq_st f31,72(r1),0,qr0` in the prologue, one `psq_l f31,72(r1),0,qr0`
  in the epilogue and nothing else; the export's `unaff_GQR0`, `in_f31`,
  `dVar11`/`dVar12` and `bVar2 = (byte)unaff_GQR0 & 7` are the decompiler's model
  of that bracket. The two `int -> float` conversions use the ordinary
  `xoris 0x8000` + `0x4330` bias, not a quantiser.
* **Row 6** was rejected by wave 63 for "the `+0xD` flag byte of the level
  record and the paired-single scaffolding". The `+0xD` byte is simply the
  selector between the two z constants (`-1800.0` / `1500.0`), and the
  `0x8008E770` "position/player query" is the translated
  [`section4b_entry_at_point`] (`src/resource/course.rs`, ledger
  `resource::course,8008e770`) which returns the section-4B area id of a point.
  With both pinned the body is a plain filter/append loop.
* **Row 5** is the conditional one: two of its three execution paths are
  **separate TSV rows** (`0x807B7D60`, 456 bytes; `0x807B7F30`, 396 bytes) and
  the state-0 worker at `0x807B7B70` has **no TSV row and therefore no Ghidra
  export at all**. The decode below is complete and self-consistent, but the
  "one function at `0x807B7B40`" premise is false, so a translation lane must
  first decide the body boundary and then implement three entry points (see the
  section's "Condition").

---

## 1. `0x80731D10` - `daRotDokan_c` scale/edge sweep (twin of `fn_8072e860`)

**Evidence.** `work/ghidra/rel-d_basesNP/export/80731d10_FUN_80731d10.c`,
540 bytes, 135 words, synthetic `0x80731D10`. Body `0x80731D10..0x80731F28`
(one `blr`, last word). Frame: `stwu r1,-32(r1)` at `0x80731D10`, LR at
`36(r1)`, `r31`/`r30`/`r29` at `28`/`24`/`20(r1)`, `r30 = 0x807CD6C8`
(`lis r30,0x807D` / `addi r30,r30,-10552`) as the constant block, `mr r31,r3`
at `0x80731D20`. No `psq`, no jump table, no indirect call. Epilogue
`0x80731F10..0x80731F28`.

**What it is.** The scale/edge sweep of the `0xA08`-byte rotating pipe
`daRotDokan_c` (constructor `0x80730AF0`, vtable `0x8080D0F8`, interface vtable
`0x8080D1D8` at `+0x3D0`; `docs/COVERAGE_WAVE7_BASES.md:108-111` and
`docs/COVERAGE_WAVE128_BASES.md:44-58`). It is the **exact structural twin** of
the pedestal's [`fn_8072e860`] (`0x8072E860`, 572 bytes): the same two-flag
tween with [`fn_80731cb0`] as the stepper, the same four-slot value ring, the
same edge-mask pair writes. The differences are all field offsets and two
literals, listed under "Differences from the pedestal twin" below.

The reset body [`fn_80731c60`] proves the field roles in the same window: it
stores `1.0f` (`0x807CD710`) into `+0x9A8`/`+0x9AC` and the four ring words
`+0x9B8..+0x9C4`, `1` into `+0x9E4` and zero into `+0x9E8`, `+0x9F0`, `+0x9F4`,
`+0x9F8`, `+0xA07`. The stepper [`fn_80731cb0`] reads the direction byte
`+0x9F8` and clamps to `1.0f`. The camera-rect visibility test
[`fn_80731f30`] and the two `0x200`-step arm sweeps
[`fn_807320f0`]/[`fn_80732230`] are the siblings.

**Exact behaviour.**

1. `state = *(s32*)(this+0x9E4)`; if `state == 0`, jump to step 6 (the body does
   nothing at all in state 0 - not even the store block).
2. `state == 1`:
   ```
   done = 0
   if *(u32*)(this+0x9F4) == 0 {
       if fn_80731cb0(*(f32*)0x807CD780 /*1.5*/, *(f32*)0x807CD784 /*0.08*/,
                      this, this+0x9AC) != 0 { *(u32*)(this+0x9F4) = 1; done = 1; }
   } else { done = 1; }
   if *(u32*)(this+0x9F0) == 0 {
       if fn_80731cb0(*(f32*)0x807CD788 /*0.75*/, *(f32*)0x807CD78C /*0.04*/,
                      this, this+0x9A8) != 0 { *(u32*)(this+0x9F0) = 1; done += 1; }
   } else { done += 1; }
   if done == 2 {
       *(u32*)(this+0x9F0) = 0;
       *(u32*)(this+0x9F4) = 0;
       let dir = *(u8*)(this+0x9F8) ^ 1;
       *(u8*)(this+0x9F8) = dir;
       if dir == 0 { *(u32*)(this+0x9E4) = 2; }   // else stay in state 1
   }
   ```
   The store order is `+0x9F0` then `+0x9F4` (`0x80731DD0`, `0x80731DDC`) and the
   byte is written **between** the second clear and the state store
   (`0x80731DE0`, `0x80731DE4`).
3. `state == 2`:
   ```
   if *(u8*)(this+0x9F8) == 0 {
       *(f32*)(this+0x9AC) = *(f32*)0x807CD790;   // 1.15
       *(f32*)(this+0x9A8) = *(f32*)0x807CD794;   // 1.075
       *(u8*)(this+0x9F8) = 1;
   } else {
       let n = *(s32*)(this+0x9E8) + 1;
       *(f32*)(this+0x9AC) = *(f32*)0x807CD710;   // 1.0
       *(f32*)(this+0x9A8) = *(f32*)0x807CD710;   // 1.0
       *(u8*)(this+0x9F8) ^= 1;
       *(s32*)(this+0x9E8) = n;
       if n >= 3 {
           *(u32*)(this+0x9E4) = 0;
           *(u8*)(this+0xA05) = 0;
           for a in [0x94C,0x950,0x954,0x958,0x95C,0x968,0x96C,0x970,0x974] {
               *(f32*)(this+a) = *(f32*)0x807CD710;   // 1.0
           }
           return;
       }
   }
   ```
   Note the guard is `n >= 3`, i.e. the counter must reach **2**: the pedestal
   twin uses `4 < counter` (counter 5), so this actor's state-2 pulse loop is
   shorter than the pedestal's. Copy the comparison, not the sibling's number.
4. Common tail (state 1 and the non-resetting state-2 arms) - the four-slot ring:
   ```
   let i = *(u8*)(this+0xA07) as usize;
   *(f32*)(this+0x94C) = *(f32*)(this+0x9A8);          // "first"
   *(f32*)(this+0x9B8 + i*4) = *(f32*)(this+0x9AC);    // ring[i] = "second"
   let prev = (i.wrapping_sub(2)) & 3;
   let second = *(f32*)(this+0x9B8 + prev*4);          // ring[i-2]
   *(f32*)(this+0x950) = second;
   *(u8*)(this+0xA07) = (i as u8).wrapping_add(1) & 3;
   ```
   The index arithmetic is the compiler's `rlwinm r4,r4,2,28,29` on `i-2`, so
   `prev` is the modulo-4 predecessor twice removed, not `i-1`.
5. Edge pairs selected by the mask byte `+0xA05` (the byte
   [`fn_807318f0`] writes from its eight-entry mask table):
   ```
   let e = *(u8*)(this+0xA05);
   if e & 8 != 0 { *(f32*)(this+0x958) = *(f32*)(this+0x94C);
                   *(f32*)(this+0x95C) = *(f32*)(this+0x950); }
   if e & 2 != 0 { *(f32*)(this+0x968) = *(f32*)(this+0x94C);
                   *(f32*)(this+0x96C) = *(f32*)(this+0x950); }
   if e & 1 != 0 { *(f32*)(this+0x970) = *(f32*)(this+0x94C);
                   *(f32*)(this+0x974) = *(f32*)(this+0x950); }
   ```
   Bit `4` is **not** tested: `+0x960`/`+0x964` are never written by this body
   (the pedestal twin writes four pairs at `+0x8B4/+0x8BC/+0x8C4/+0x8CC`; this
   actor's shifted equivalents are `+0x958/+0x968/+0x970` - the `+0x960` pair is
   left to a sibling body).
6. Return.

**Differences from the pedestal twin** (state them in the Rust comment, they are
real and easy to get wrong):

| item | this row | `fn_8072e860` |
|---|---|---|
| state word | `+0x9E4` | `+0x944` |
| reset counter | `+0x9E8`, reset at `>= 3` | `+0x948`, reset at `> 4` |
| end flags | `+0x9F0`, `+0x9F4` | `+0x94C`, `+0x950` |
| direction byte | `+0x9F8` | `+0x955` |
| edge mask | `+0xA05` | `+0x954` |
| ring index | `+0xA07` | `+0x956` |
| tween values | `+0x9A8`, `+0x9AC` | `+0x910`, `+0x914` |
| ring | `+0x9B8` (4 words) | `+0x918` (4 words) |
| output block | `+0x94C`, `+0x950`, `+0x958`..`+0x974` | `+0x8AC`, `+0x8B0`, `+0x8B4`..`+0x8D0` |
| edge pairs written | 3 (bits 8, 2, 1) | 4 (bits 8, 4, 2, 1) |

**Signature.**

```rust
/// Runs the rotating pipe's scale/edge sweep.
///
/// # Original
/// - Ghidra: `FUN_80731d10` at `0x80731D10` (540 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80731d10_FUN_80731d10.c`
///
/// # Arguments
/// * `this` - live `daRotDokan_c` (`0xA08` bytes, vtable `0x8080D0F8`);
///   state `+0x9E4`, reset counter `+0x9E8`, end flags `+0x9F0`/`+0x9F4`,
///   direction `+0x9F8`, tween values `+0x9A8`/`+0x9AC`, four-word ring
///   `+0x9B8`, edge mask `+0xA05`, ring index `+0xA07`, output block
///   `+0x94C..+0x974`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Steps the two tween values, toggles the direction byte, cycles the ring
/// index and writes up to three edge pairs of the output block. State 2's final
/// arm resets the state word, the mask and nine output words to `1.0f`.
///
/// # Safety
/// `this` must be a live `daRotDokan_c`; the reset body `0x80731C60` must have
/// run at least once so the ring and the flags are initialised.
///
/// # Confidence
/// high - every offset, state value, mask bit and constant is literal, and the
/// twin `fn_8072e860` fixes the semantics of each block.
pub unsafe fn fn_80731d10(this: usize)
```

**Constants.** `1.0f` = `0x807CD710`; `1.5f` = `0x807CD780`; `0.08f` =
`0x807CD784`; `0.75f` = `0x807CD788`; `0.04f` = `0x807CD78C`; `1.15f` =
`0x807CD790`; `1.075f` = `0x807CD794`; constant block base `0x807CD6C8`
(offsets `+0x48` = 1.0, `+0xB8` = 1.5, `+0xBC` = 0.08, `+0xC0` = 0.75,
`+0xC4` = 0.04, `+0xC8` = 1.15, `+0xCC` = 1.075); states 0/1/2; reset threshold
3; ring length 4; edge bits `0x8`, `0x2`, `0x1`; output words `+0x94C`,
`+0x950`, `+0x954`, `+0x958`, `+0x95C`, `+0x968`, `+0x96C`, `+0x970`, `+0x974`
(the reset list skips `+0x960`/`+0x964`).

---

## 2. `0x806D6950` - number-panel lift rider weight/dedupe walk

**Evidence.** `work/ghidra/rel-d_basesNP/export/806d6950_FUN_806d6950.c`,
536 bytes, 134 words, synthetic `0x806D6950`. Body `0x806D6950..0x806D6B64`
(one `blr`, last word). Frame: `stwu r1,-48(r1)`, LR at `52(r1)`,
`addi r11,r1,48`, `bl 0x802DCD6C` (`__savegprlr_27`) at `0x806D6960`;
`r27 = this`, `r28 = node`, `r29 = weight`, `r30 = ids`, `r31 = count`. Epilogue
`0x806D6B4C..0x806D6B64` (`bl 0x802DCDB8`). No `psq`, no jump table, one
self-recursive pair of `bl`s and one indirect `bctrl`.

**What it is.** The recursive rider walk of the `0xF10`-byte **number-panel lift**
actor (constructor `0x806D5290`, base constructor `0x80066FC0`, vtable
`0x807FB0E8`; `docs/COVERAGE_WAVE51_BASES.md:21-42`). It accumulates the
"weight" that the panel displays, deduplicating by actor id through a word
array, and registers each rider in the 128-slot ridden ring at `+0x7B4`.
Its only external caller is:

* [`fn_806d6b70`] (`0x806D6B70`): the per-actor load query. It walks the nine
  `0x5C`-byte motion records at `+0xBB4..+0xEF0`, and for each record's
  `+0x10` chain calls this body with `weight = &stack[0]`,
  `ids = &stack[16]` (a 512-byte, 128-word array) and `count = weight[0]`
  (a quirk: the incoming count is the current weight, which is 0 on entry);
  it then adds [`fn_806d5eb0`]'s live-slot count.

The only other reference is the body's own recursive `bl` pair
(`0x806D6A64` inside the rider-chain walk and `0x806D6B14` in the player
branch), so the export's single self-reference is recursion, not a second
entry point.

The ledger already carries the helpers this body calls:
`fn_806d68e0` (id search), `fn_806d5da0` (refresh a live slot),
`fn_806d5e30` (claim the first free slot), `fn_806d5eb0` (live count).

**Exact behaviour.**

```
if node == 0: return count
kind = *(u16*)(node+8)          // 0x19B or 0x1A1 carry a rider list
listed = 0
if kind == 0x19B:
    id = *(u32*)node
    if fn_806d68e0(id, ids, count) == 0:
        *(u32*)(ids + count*4) = id
        listed = node + 0x460
        count += 1
        rider = fn_8011f3f0(node)        // rider id of the attached object
        if fn_806d5da0(this, rider) == 0 { fn_806d5e30(this, rider) }
else if kind == 0x1A1:
    id = *(u32*)node
    if fn_806d68e0(id, ids, count) == 0:
        *(u32*)(ids + count*4) = id
        listed = node + 0x4CC
        count += 1
        if fn_806d5da0(this, id) == 0 { fn_806d5e30(this, id) }

if listed != 0:
    // walk the rider chain: node = *(u32*)(link+4), next = *(u32*)(link+0xC)
    let mut link = *(u32*)(listed + 0x0C)
    while link != 0 {
        count = fn_806d6950(this, *(u32*)(link+4), weight, ids, count)
        link = *(u32*)(link + 0x6C)
    }
    return count

// not a rider node (or already counted): the player/kind branch
k = *(u8*)(node+0x38C)
if k == 1:
    vtable = *(u32*)(node+0x60)
    ch = (*(u8*)(vcall(vtable+0x6C)(node)) as i8) as i32     // player character
    if ch < 4:
        id = *(u32*)node
        if fn_806d68e0(id, ids, count) != 0: return count     // already counted
        *(u32*)(ids + count*4) = id
        count += 1
        *weight = min(*weight + fn_80057cc0(node), 0x80)      // node+0x1088
        let next = fn_80162d00(*(u32*)(node+0x2A78))          // registry lookup
        count = fn_806d6950(this, next, weight, ids, count)
        return count
else if k == 3:
    id = *(u32*)node
    if fn_806d5da0(this, id) == 0 { fn_806d5e30(this, id) }
return count
```

Details that a translator must keep:

* the two `0x19B`/`0x1A1` arms set `listed` **only on the dedupe miss**; when the
  id is already in `ids` the code falls straight into the `+0x38C` branch, so an
  already-counted rider node is *also* considered by the player test;
* the `0x19B` arm registers the **rider id** from `0x8011F3F0(node)`, the `0x1A1`
  arm and the `kind == 3` arm register the **actor id** `*(u32*)node`; the
  dedupe/`ids` array always uses the actor id;
* the weight add is signed-then-clamped: `if (*weight > 0x80) *weight = 0x80`
  (`cmpli cr0,r0,128` / `ble`), so 128 is the ceiling;
* the recursion into `0x80162D00`'s result happens **after** the weight add, and
  a null result simply ends that path (the recursive entry returns `count`).

**Signature.**

```rust
/// Sums the rider weight of one rider tree and registers every rider.
///
/// # Original
/// - Ghidra: `FUN_806d6950` at `0x806D6950` (536 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/806d6950_FUN_806d6950.c`
///
/// # Arguments
/// * `this` - live number-panel lift (`0xF10` bytes, vtable `0x807FB0E8`);
///   ridden ring `+0x7B4` (128 slots, 8 bytes each).
/// * `node` - current rider actor, or `0`.
/// * `weight` - pointer to the `0x80`-clamped weight accumulator.
/// * `ids` - pointer to a word array that receives the distinct actor ids.
/// * `count` - number of ids already stored.
///
/// # Returns
/// The updated id count.
///
/// # Side effects
/// Writes up to 128 words of `ids`, the weight word, and the ring slots
/// (rearm to 30 frames / claim) through `0x806D5DA0`/`0x806D5E30`.
///
/// # Safety
/// `ids` must have room for `count + 1` words (all callers pass a 512-byte
/// stack buffer); every visited node must be a live actor.
///
/// # Confidence
/// medium - every offset, tag, bound and call argument is literal; the rider /
/// slot / registry roles are named from the translated siblings rather than
/// from symbols.
pub unsafe fn fn_806d6950(
    this: usize,
    node: usize,
    weight: usize,
    ids: usize,
    count: u32,
) -> u32
```

**Constants.** rider tags `0x19B` (411) and `0x1A1` (417); weight ceiling `0x80`
(128); kind byte `+0x38C` values 1 (player) / 3; character bound 4; chain
offsets `+0x460` (`0x19B`), `+0x4CC` (`0x1A1`), `+0x4` (node), `+0xC` (head),
`+0x6C` (next); vtable slots `+0x60`/`+0x6C`; player weight `+0x1088`; registry
key `+0x2A78`. Callees: `0x806D68E0` `fn_806d68e0`, `0x806D5DA0` `fn_806d5da0`,
`0x806D5E30` `fn_806d5e30`, `0x8011F3F0` `fn_8011f3f0` (ledger
`game::worldmap`), `0x80057CC0` `fn_80057cc0` (ledger `game::player`, returns
`player+0x1088`), `0x80162D00` `fn_80162d00` (ledger `resource::registry`).

---

## 3. `0x806E0110` - `lift_spin_kinoko` landing/rest-height state body

**Evidence.** `work/ghidra/rel-d_basesNP/export/806e0110_FUN_806e0110.c`,
532 bytes, 133 words, synthetic `0x806E0110`. Body `0x806E0110..0x806E0320`
(one `blr`, last word). Frame: `stwu r1,-32(r1)`, LR at `36(r1)`,
`r31`/`r30`/`r29` at `28`/`24`/`20(r1)`, `mr r31,r3` at `0x806E0120`. No `psq`,
no jump table, one indirect call (interface `+0x394`, slot `+0x18`) and one
virtual query (`+0x60`, slot `+0xE4`). Epilogue `0x806E0308..0x806E0320`.

**What it is.** The landing handler of the `0x700`-byte **`lift_spin_kinoko`**
lift (constructor `0x806DEC80`, vtable `0x807FC428`, accessor `+0x3D4`;
`docs/COVERAGE_WAVE122_BASES.md:36-50`). The field set is proven by the
translated siblings in the same window: [`fn_806dff70`] uses `+0x664`, `+0x6A8`,
`+0x624`, `+0x648`, `+0x6AA`, `+0x5F4`, `+0x602` and the interface `+0x394`
with message cells `0x8082DFA8`/`0x8082DF28`; [`fn_806dfcc0`] uses the `+0xE4`
virtual and `0x8082DF28`; the spawn initialiser `0x806DED20`
(`docs/SPEC_WAVE129_BASES.md:1131-1198`) writes the same `+0x658`/`+0x660`/
`+0x6A9` block and switches between `0x8082DFA8` and `0x8082DEE8`. This body
adds the third cell, `0x8082DEE8`, as its "landed" state and the `+0x64C` ramp
target.

**Exact behaviour.**

1. Hold-off timer: `if *(s32*)(this+0x660) != 0 { *(s32*)(this+0x660) -= 1 }`.
2. Blend ramp, only while the timer reads 0:
   ```
   let a = *(f32*)(this+0xEC);
   let b = *(f32*)(this+0x64C);
   if a != b {
       if *(u32*)(this+0x658) == 0 {
           let v = a + *(f32*)0x807CBAAC;       // +0.08
           *(f32*)(this+0xEC) = v;
           if v >= b { *(f32*)(this+0xEC) = b; }
       } else {
           let v = a - *(f32*)0x807CBAAC;       // -0.08
           *(f32*)(this+0xEC) = v;
           if v <= b { *(f32*)(this+0xEC) = b; }
       }
   }
   ```
   The comparison is `fcmpo` + `cror eq,gt,eq` (`>=`) in the first arm and
   `cror eq,lt,eq` (`<=`) in the second; the store happens **before** the clamp,
   so the clamp is a second store of the same slot.
3. `0x8006CD40(this)`.
4. `state = vmethod0(this+0x60, 0xE4)(this)` - the same query `fn_806dff70` uses.
5. Three-way dispatch:
   ```
   let at_rest = if *(u32*)(this+0x658) == 0 {
       *(f32*)(this+0x624) >= *(f32*)(this+0xB0)
   } else {
       *(f32*)(this+0x624) <= *(f32*)(this+0xB0)
   };
   if at_rest {
       *(f32*)(this+0xB0) = *(f32*)(this+0x624);
       *(f32*)(this+0xEC) = 0.0f;             // 0x807CBA54
       *(u8*)(this+0x6A9) = 0;
       vmethod1(this+0x394, 0x18, 0x8082_DEE8);
       return;
   }
   if state == 2 {
       if *(u8*)(this+0x6A9) == 0 {
           fn_806dfc40(this, -(*(u16*)(this+0x6AA) as i16) as i32);
           *(u16*)(this+0x5F4) = *(u16*)(this+0x602);
       }
       vmethod1(this+0x394, 0x18, 0x8082_DF28);
       return;
   }
   // every other state
   if *(u8*)(this+0x6A9) != 0 { *(u8*)(this+0x6A9) -= 1; }
   if *(u8*)(this+0x6A9) == 0 {
       if *(u32*)(this+0x654) == 0 {
           let id = *(u32*)this;                        // actor word
           let snd = *(u32*)0x80429D60;                 // sound manager
           let mut p = [0u32; 2];
           0x8006A4B0(p.as_mut_ptr(), this+0xAC);
           0x80199390(snd, 0x2C2, id, p.as_ptr(), 0);
       }
       fn_806dfc40(this, -819);                         // 0xFFFFFCCD
   }
   ```
   `0x8006A4B0` is the 3-D to screen-space convertor used by the sibling
   `fn_806d5c60`, `0x80199390` is the translated sound play (ledger
   `audio::ax,80199390`), and the third argument is the actor's first word
   (`lwz r30,0(r31)`), not a constant.

**Signature.**

```rust
/// Lands the spinning lift at its rest height and switches its interface.
///
/// # Original
/// - Ghidra: `FUN_806e0110` at `0x806E0110` (532 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/806e0110_FUN_806e0110.c`
///
/// # Arguments
/// * `this` - live `lift_spin_kinoko` (`0x700` bytes, vtable `0x807FC428`);
///   blend `+0xEC`, target `+0x64C`, height `+0xB0`, rest height `+0x624`,
///   direction `+0x658`, hold timer `+0x660`, angle `+0x6AA`, steer helper
///   state `+0x5F4`/`+0x602`, auxiliary byte `+0x6A9`, mode `+0x654`,
///   interface `+0x394`, vtable `+0x60`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x660`, `+0xEC`, `+0xB0`, `+0x6A9`, `+0x5F4`; calls `0x8006CD40`,
/// the `+0xE4` virtual, `fn_806dfc40`, the `+0x394` `+0x18` virtual and
/// (once, when `+0x654 == 0`) sound `0x2C2` through `0x8006A4B0`/`0x80199390`.
///
/// # Safety
/// `this` must be live, its vtable and interface valid, and the sound manager
/// at `0x80429D60` mapped.
///
/// # Confidence
/// medium - every offset, branch, message cell and constant is literal; the
/// semantic roles come from the sibling bodies (`fn_806dff70`,
/// `fn_806dfcc0`, `0x806DED20`), not from symbols.
pub unsafe fn fn_806e0110(this: usize)
```

**Constants.** `0.08f` = `0x807CBAAC`; `0.0f` = `0x807CBA54`; steer argument
`-819` (`0xFFFFFCCD`); sound id `0x2C2` (706); message cells `0x8082DEE8` and
`0x8082DF28` (immediates `0xDEE8` and `0xDF28` on `lis r4,0x8083`, i.e.
`0x80830000 - 0x2118` and `0x80830000 - 0x20D8`); interface slot `+0x18`;
virtual query slot `+0xE4`; sound manager `0x80429D60`.

> Note for the translator: the `0x8082DF28` cell is materialised as
> `lis r4,0x8083` + `addi r4,r4,0xDF28` (`0x806E0290`; the immediate is the
> signed `-8408`), i.e. `0x80830000 - 0x20D8 = 0x8082DF28`. The
> `0x8082DEE8` cell is `addi r4,r4,0xDEE8` (`0x806E0234`, signed `-8472`) =
> `0x80830000 - 0x2118 = 0x8082DEE8`. Do not type either cell as a `f32`
> global; both are message/descriptor blocks passed by address.

---

## 4. `0x806D62C0` - number-panel lift rider band histogram + digit arming

**Evidence.** `work/ghidra/rel-d_basesNP/export/806d62c0_FUN_806d62c0.c`,
528 bytes, 132 words, synthetic `0x806D62C0`. Body `0x806D62C0..0x806D64CC`
(one `blr`, last word). Frame: `stwu r1,-80(r1)`, LR at `84(r1)`,
`addi r11,r1,64`, `stfd f31,64(r1)` + `psq_st f31,72(r1),0,qr0` at
`0x806D62D0/0x806D62D4`, `bl 0x802DCD60` (`__savegprlr_26`) at `0x806D62D8`.
`r27 = this`, `r31 = this+0xBB4`, `r30 = this+0xEF0`, `r24 = r1+8` (the eight
byte counters), `r26 = 4`, `f31 = 48.0f`, `r23 = 0x807D0000`. Epilogue
`0x806D64B0..0x806D64CC` (`psq_l f31,72(r1),0,qr0`, `lfd f31,64(r1)`,
`bl 0x802DCDAC`).

**What it is.** The per-frame **rider band histogram** of the same number-panel
lift actor (`0xF10` bytes, `docs/COVERAGE_WAVE51_BASES.md:21-42`). Its only
in-REL caller is the per-frame dispatcher [`fn_806d6d50`], which runs
`0x806D6610()` then `0x806D62C0(this)` then the `+0x394` `+0x10` virtual and
`0x806D64D0(this)` (export `806d6d50_FUN_806d6d50.c:7-10`), i.e. this body is
the second step of every panel frame. For each of the nine `0x5C`-byte
motion records at `+0xBB4`, it walks the record's `+0x10` rider chain, keeps the
riders whose `+0x38C` kind is 1 or 2 and whose class vtable `+0x6C` returns a
character index below 4, sorts `rider_x - self_x` into eight 48-unit bands, and
then writes the eight band counts into the first eight of the sixteen `0x58`-byte
panels at `+0x4C4` (byte `+0x3A`). Each band entry also arms one rolling digit
record through [`fn_806d5c60`] (sound `0x2AA`), rate-limited per character by the
`+0xF06..+0xF09` cooldown bytes.

**Exact behaviour.**

```
for p in this+0xF06 .. this+0xF0A {           // four per-character cooldowns
    if *(u8*)p != 0 { *(u8*)p -= 1 }
}
let mut bins = [0u8; 8];
0x80160AD0(&mut bins, 0, 8);                  // memset(bins, 0, 8)

let step   = *(f32*)0x807CB6F8;               // 48.0f
let origin = *(f32*)0x807CB6F4;               // -192.0f

for record in (this+0xBB4 .. this+0xEF0).step(0x5C) {
    let mut link = *(u32*)(record+0x10);
    while link != 0 {
        let rider = *(u32*)(link+4);
        let mut picked = 0;
        let k = *(u8*)(rider+0x38C);
        if k == 1 || k == 2 {
            let vt = *(u32*)(rider+0x60);
            let p = vcall(vt+0x6C)(rider);        // pointer to a character byte
            if (*(u8*)p as i8) < 4 { picked = rider; }
        }
        if picked != 0 {
            let d = *(f32*)(picked+0xAC) - *(f32*)(this+0xAC);
            let mut band = 0usize;
            let mut limit = origin;
            loop {
                limit += step;
                if d < limit { bins[band] += 1; break; }
                band += 1;
                if band < 7 { continue; }
                bins[7] += 1;                     // overflow band
                break;
            }
            if *(u8*)(picked+0x38C) == 1 {
                let vt = *(u32*)(picked+0x60);
                let ch = (*(u8*)vcall(vt+0x6C)(picked) as i8) as usize;
                if ch < 4
                   && *(u8*)(this + 0xF06 + ch) == 0
                   && fn_80056cf0(picked, 28) != 0
                {
                    fn_806d5c60(this, band as u8);
                    *(u8*)(this + 0xF06 + ch) = 4;
                }
            }
        }
        link = *(u32*)(link+0x0C);
    }
}

let mut panel = this + 0x4C4;
for i in 0..8 {
    *(u8*)(panel + 0x3A) = bins[i];
    panel += 0x58;
}
```

The band ladder, stated once so it cannot be mis-transcribed: the loop
pre-increments `limit` before the first compare, so with `origin = -192.0` and
`step = 48.0` the thresholds are `-144, -96, -48, 0, 48, 96, 144` for bands
`0..6`, and band 7 collects `d >= 144`:

| band | interval of `rider_x - self_x` |
|--:|---|
| 0 | `< -144` |
| 1 | `[-144, -96)` |
| 2 | `[-96, -48)` |
| 3 | `[-48, 0)` |
| 4 | `[0, 48)` |
| 5 | `[48, 96)` |
| 6 | `[96, 144)` |
| 7 | `>= 144` |

The word at `0x806D6494` is `ori r0,r0,0x0`, i.e. the compiler's own no-op (it
re-fetches `r0` that the following `lbz` overwrites); not a store.

**Signature.**

```rust
/// Fills the number-panel lift's eight rider band counters.
///
/// # Original
/// - Ghidra: `FUN_806d62c0` at `0x806D62C0` (528 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/806d62c0_FUN_806d62c0.c`
///
/// # Arguments
/// * `this` - live number-panel lift; nine motion records `+0xBB4` stride
///   `0x5C`, rider chain head `+0x10`, panel records `+0x4C4` stride `0x58`,
///   per-character cooldowns `+0xF06..+0xF0A`, position `+0xAC`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Decrements the four cooldown bytes, writes `+0x3A` of the first eight panel
/// records, arms rolling digit records through `fn_806d5c60` (sound `0x2AA`)
/// and can raise event sound `0x2AA` repeatedly, rate-limited to once per four
/// frames per character.
///
/// # Safety
/// `this` must be live, the nine records and sixteen panels constructed, and
/// every rider a live actor with a valid `+0x60` vtable.
///
/// # Confidence
/// high - the band ladder, the four cooldown bytes, the eight panel writes and
/// the sound path are all literal; the only named-by-use item is the `+0x38C`
/// kind test and the `+0x6C` character accessor, both already documented by
/// `fn_8061ec00`/`fn_806d6b70`.
pub unsafe fn fn_806d62c0(this: usize)
```

**Constants.** `48.0f` = `0x807CB6F8`; `-192.0f` = `0x807CB6F4`; band count 8;
band count to write 8 of 16; panel stride `0x58`, panel patch offset `+0x3A`;
record stride `0x5C`, first record `+0xBB4`, end `+0xEF0`; rider chain `+0x10`
(head) / `+0x4` (actor) / `+0xC` (next); kind byte `+0x38C` (1 or 2), character
bound 4, vtable `+0x60`/`+0x6C`; cooldowns `+0xF06..+0xF09`, reload value 4;
event flag 28; sound path `0x8006A4B0` + `0x80199390`, sound `0x2AA` (inside
`fn_806d5c60`).

---

## 5. `0x807B7B40` - course-leave save bookkeeping (CONDITIONAL)

**Evidence.** `work/ghidra/rel-d_basesNP/export/807b7b40_FUN_807b7b40.c`,
524 bytes, 131 words, synthetic `0x807B7B40`. The *reachable* stream is
`0x807B7B40..0x807B7D50` = 40 bytes of dispatcher + 2 padding words + 484 bytes
of state-0 worker (the worker's own `blr` is at `0x807B7D50`; the TSV extent
`0x807B7B40+524 = 0x807B7D4C` cuts the last two words, `addi r1,r1,48` at
`0x807B7D4C` and that `blr`, so the reachable span is 532 bytes, not 524).
`0x807B7B70` has **no TSV row**; `0x807B7D60`
(456 bytes) and `0x807B7F30` (396 bytes) are the next two TSV rows and are
reached only by `b` (tail branches), not by `bl`.

**What it is.** The save-record bookkeeping that the stage/world-map scene
teardown [`fn_807b77e0`] runs between "stop the subsystems" and "publish the
four player panes" (`docs/SPEC_WAVE146_BASES.md:1094-1110`: the teardown calls
`0x807B8EC0(this,0)` then `0x807B7B40(this)`). It reads the current stage/player
snapshot at `0x80358D54` and the loaded save image, and writes the current
course's bits in the `10 x 42` save grid at `image + world*0xA8 + course*4 +
0x6C` (`0x800CE180(image,row,col,bits)` ORs three bits into that word;
`0x800CE260`/`0x800CE2C0` are the byte-grid accessors at `+0x7C4`, both already
specified at `docs/SPEC_WAVE152_BASES.md:1125-1241`).

**Dispatcher (40 bytes, `0x807B7B40..0x807B7B68`).** `r4 = 0x80430000`,
`r0 = *(u32*)0x8042900C` (the word reads **2** in the shipped image, so the
default arm is the live one):

```
match *(u32*)0x8042900C {
    0 => 0x807B7B70(this),      // state-0 worker, no TSV row
    1 => 0x807B7F30(this),      // TSV row 5453
    _ => 0x807B7D60(this),      // TSV row 5452
}
```

The three branches are plain `b` tail branches (`48000014`, `480003d0`,
`480001fc`), so `r3` (the caller's argument) reaches the callee unchanged.

**Arm A - state-0 worker, `0x807B7B70` (no export).**

```
r29 = this
mgr = *(u32*)0x8042A1C8
world  = *(u8*)(mgr+0x120C)
course = *(u8*)(mgr+0x120D)
if world >= 10 || course >= 42 { return }
if *(u32*)0x8042A220 != 0 {
    *(u32*)0x80354E80 = *(u32*)(0x80358D54+0x8C)
    *(u32*)0x80354E84 = *(u32*)(0x80358D54+0x90)
    *(u32*)0x80354E88 = *(u32*)(0x80358D54+0x94)
    *(u32*)0x80354E8C = *(u32*)(0x80358D54+0x98)
}
image = 0x800E0380(*(u32*)0x8042A040, -1)          // save-image record A
if *(u32*)0x80315B98 == 0 {
    if (course < 25 || course > 31) && course != 40 {
        0x8008EDB0(*(u32*)0x80429F7C + 8)          // stage slot reset
    }
}
if 0x8005FFB0() == 1 {                             // exactly one player
    0x800CE260(image, world, course, *(u8*)(*(u32*)0x80429F7C + 0x380), 0)
}
for i in 0..3 {
    if *(u32*)(0x803741B0 + 4*i) != 4 {
        0x800CE180(image, world, course, [1,2,4][i])
    }
}
*(u8*)0x8042A178 = 0
*(u8*)0x8042A180 = 0
// fall through to the shared tail at 0x807B7CCC
```

**Shared tail (`0x807B7CCC..0x807B7D50`), used by arm A only.**

```
block = *(u32*)(0x80315B90+8)
let mut a = (*(u32*)0x8042A1FC == 1) ? 1 : 0
if block != 0 {
    if world == 2 && course == 3 { /* keep a */ } else { a = 0 }
}
let b = (block == 1) ? 1 : 0
0x800FB9E0(a, b, world, course)
if *(u32*)(0x80315B90+8) != 0 { 0x807B7D60(this) }   // tail into arm C
```

**Arm B - `0x807B7F30` (TSV row 5453, 396 bytes).**

```
world  = *(u8*)(*(u32*)0x8042A1C8 + 0x120C)
course = *(u8*)(*(u32*)0x8042A1C8 + 0x120D)
if *(u32*)(0x80315B90+8) != 0 {
    if *(u32*)0x8042A204 == 1 && world < 10 && course < 42 {
        0x800FB9E0(0, 1, world, course)
    }
    0x807B7D60(this)
} else {
    *(u32*)(*(u32*)0x80429F7C + 0x60) = 1
    if 0x8005FFB0() == 1 && *(u32*)(0x80315B90+8) == 0
       && world < 10 && course < 42
    {
        let image = 0x800E0380(*(u32*)0x8042A040, -1)
        let v = 0x800CE2C0(image, world, course,
                           *(u8*)(*(u32*)0x80429F7C + 0x380)) & 0xFF
        let next = if v < 8 { (v + 1) & 0xFF } else { v }
        0x800CE260(image, world, course,
                   *(u8*)(*(u32*)0x80429F7C + 0x380), next)
    }
    if *(u16*)0x80428730 == 9 {
        0x8008EDB0(*(u32*)0x80429F7C + 8)
        0x800FBD30(0)
        *(u8*)0x8042A178 = 0
        *(u8*)0x8042A180 = 0
    } else {
        0x800FFE70()
        0x800FFF90()
    }
}
```

**Arm C - `0x807B7D60` (TSV row 5452, 456 bytes).** Copies the stage/player
snapshot `0x80358D54` into the live save image
(`*(u32*)0x80429F7C`) and into the global player arrays:

```
img = *(u32*)0x80429F7C
src = 0x80358D54
*(u32*)(img+0x0C) = *(u32*)(src+0x04)
*(u32*)(img+0x10) = *(u32*)(src+0x08)
*(u32*)(img+0x14) = *(u32*)(src+0x0C)
*(u32*)(img+0x18) = *(u32*)(src+0x10)
*(u32*)(img+0x1C) = *(u32*)(src+0x14)
*(u8*)(img+0x20)  = *(u8*)(src+0x18)
*(u8*)(img+0x21)  = *(u8*)(src+0x19)
*(u8*)(img+0x22)  = *(u8*)(src+0x1A)
*(u8*)(img+0x23)  = *(u8*)(src+0x1B)
*(u8*)(img+0x24)  = *(u8*)(src+0x1C)
*(u32*)(img+0x28) = *(u32*)(src+0x20)
*(u32*)(img+0x2C) = *(u32*)(src+0x24)
*(u32*)(img+0x30) = *(u32*)(src+0x28)
*(u32*)(img+0x34) = *(u32*)(src+0x2C)
*(u32*)(img+0x38) = *(u32*)(src+0x30)
*(u8*)(img+0x380) = *(u8*)(src+0x34)
*(u32*)0x80429CC0 = *(u32*)(src+0x44)
*(u8*)0x8042A178 = *(u8*)0x8042A179     // 0x807B7EE0 / 0x807B7F08
*(u8*)0x8042A180 = *(u8*)0x8042A181     // 0x807B7EE8 / 0x807B7F0C
*(u8*)(*(u32*)0x80429F7C + 0x6C) = 0
```

The slot block is one `mtspr ctr,r0` (`r0 = 2`) + `bdnz` loop
(`0x807B7E40..0x807B7ED0`) whose two passes are interleaved by the compiler.
Written out, it is exactly these stores (`p` = 0 then 1; sub-slot `k = 2*p`
for the first index and `2*p + 1` for the second):

```
// indexed copies: the word at src+0x48+4k is a slot number 0..3
for k in 0..4 {
    let idx = *(u32*)(src + 0x48 + 4*k);
    *(u32*)(0x80354E60 + idx*4) = k;
    *(u32*)(0x80354E50 + idx*4) = *(u8*)(src + 0x68 + k) as u32;
}

// straight copies, 4 words each, destination offset = 8*p + (0 or 4)
for p in 0..2 {
    *(u32*)(0x80354E70 + 8*p + 0) = *(u32*)(src + 0x58 + 8*p);
    *(u32*)(0x80354E70 + 8*p + 4) = *(u32*)(src + 0x5C + 8*p);
    *(u32*)(0x80354E90 + 8*p + 0) = *(u32*)(src + 0x7C + 8*p);
    *(u32*)(0x80354E90 + 8*p + 4) = *(u32*)(src + 0x80 + 8*p);
    *(u32*)(0x80354E80 + 8*p + 0) = *(u32*)(src + 0x8C + 8*p);
    *(u32*)(0x80354E80 + 8*p + 4) = *(u32*)(src + 0x90 + 8*p);
    *(u32*)(0x80354EA0 + 8*p + 0) = *(u32*)(src + 0x6C + 8*p);
    *(u32*)(0x80354EA0 + 8*p + 4) = *(u32*)(src + 0x70 + 8*p);
}
```

so `0x80354E70` receives `src+0x58/0x5C/0x60/0x64`, `0x80354E90` receives
`src+0x7C/0x80/0x84/0x88`, `0x80354E80` receives `src+0x8C/0x90/0x94/0x98` and
`0x80354EA0` receives `src+0x6C/0x70/0x74/0x78`. Arm A's four-word copy of
`src+0x8C..0x98` into `0x80354E80..0x8C` is a prefix of this block, which is a
useful sanity check on the source offsets.

The loop's `r3` source pointer advances by 8 per pass (`addi r3,r3,8` at
`0x807B7EA0`, after the `+0x48/+0x4C` reads and before the `+0x58/+0x5C` ones),
which is why the `+0x48` family uses a 4-byte stride across the four sub-slots
while the copy family uses 8.

**Condition.** This row is the one **conditional** entry of the set, for one
named reason: the state-0 worker at `0x807B7B70` **has no TSV row and no Ghidra
export**, and the TSV extent (`524`) stops eight bytes short of that worker's
`blr` while the other two arms are separate rows. A translation lane must
therefore decide, before writing code, whether the row is

1. the 40-byte dispatcher only (then `0x807B7B70` becomes a new, unexported
   body to add to the module and to the ledger as its own function), or
2. the dispatcher plus the unexported worker (48 + 484 = 532 bytes, which is
   what the reachable stream says and what this spec documents).

Either choice is defensible; what must **not** happen is writing 524 bytes as if
`0x807B7B40+524` were the end of the function. The other two arms are already
separate TSV rows and should be left to their own specs
(`0x807B7D60`/`0x807B7F30` are still untranslated).

**Signature** (option 2, the recommended one).

```rust
/// Saves the course-leave state into the current save image.
///
/// # Original
/// - Ghidra: `FUN_807b7b40` at `0x807B7B40` (524-byte TSV extent; the
///   reachable stream is `0x807B7B40..0x807B7D50`, 532 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807b7b40_FUN_807b7b40.c`
/// - `0x807B7B70` has no export; that block was decoded from the raw words
///   only (see the condition above).
///
/// # Arguments
/// * `this` - scene/context object; it is forwarded unchanged to
///   `0x807B7D60`/`0x807B7F30` and otherwise unused.
///
/// # Returns
/// Nothing. (The export's `void` is accurate; only the arm-C row has a
/// meaningful `r3` on return, and its callers ignore it.)
///
/// # Side effects
/// Writes the save image (`0x80429F7C`) words `+0x0C..+0x38`, `+0x60`, `+0x6C`,
/// `+0x380` and the `10 x 42` grids at `+0x6C`/`+0x7C4`; writes the global
/// player arrays `0x80354E50..0x80354EB0`; clears `0x8042A178`/`0x8042A180`
/// and `0x80429CC0`; calls `0x800E0380`, `0x8005FFB0`, `0x800CE180`,
/// `0x800CE260`, `0x800CE2C0`, `0x8008EDB0`, `0x800FB9E0`, `0x800FBD30`,
/// `0x800FFE70`, `0x800FFF90`.
///
/// # Safety
/// The stage manager `0x8042A1C8`, the save image pointer `0x80429F7C` and the
/// sound/scene cells must be initialised; the caller must be the teardown
/// `fn_807b77e0` (or the same scene path).
///
/// # Confidence
/// medium - all three paths are decoded instruction by instruction, but the
/// state-0 worker has no Ghidra cross-check and the two save-grid accessors'
/// "row = world / col = course" reading is inferred from the `10 x 42` grid
/// shape and the `world < 10 && course < 42` guards.
pub unsafe fn fn_807b7b40(this: usize)
```

**Constants.** state word `0x8042900C` (= 2 in the image); manager
`0x8042A1C8` with `+0x120C` (world, bound 10) and `+0x120D` (course, bound 42);
stage object pointer `0x80429E98`; save image pointer cell `0x80429F7C`; save
record pair `0x8042A040` (arg to `0x800E0380`) and slot `-1`; snapshot block
`0x80358D54`; player arrays `0x80354E50`, `0x80354E60`, `0x80354E70`,
`0x80354E80`, `0x80354E90`, `0x80354EA0`; scene words `0x80315B90` (`+8`),
`0x80315B98`, `0x8042A204`, `0x8042A220`, `0x8042A1FC`, `0x8042A178`,
`0x8042A179`, `0x8042A180`, `0x8042A181`, `0x80428730` (u16), `0x80429CC0`;
bit table `0x807D0810` = `01 02 04`; BSS gate table `0x803741B0` (three words,
reject when equal to 4); course special cases `course < 25 || course > 31` and
`course == 40` (skip the stage slot reset), `world == 2 && course == 3` (keep the
`0x8042A1FC == 1` bit), `course == 37`/`world == 7` (inside `0x800FFF90`).

---

## 6. `0x806F2100` - respawn-spawner level-record collector

**Evidence.** `work/ghidra/rel-d_basesNP/export/806f2100_FUN_806f2100.c`,
520 bytes, 130 words, synthetic `0x806F2100`. Body `0x806F2100..0x806F2304`
(one `blr`, last word). Frame: `stwu r1,-144(r1)`, LR at `148(r1)`,
`addi r11,r1,80`, four `stfd`/`psq_st` pairs (`f31@128/136`, `f30@112/120`,
`f29@96/104`, `f28@80/88`), `bl 0x802DCD64` (`__savegprlr_23`) at
`0x806F2130`. `r24 = this`, `r30 = 0x8030A040` (per-type record table),
`r29 = stage`, `r28 = stage->records+0x24`, `r27 = count`, `f28 = 2^52+2^31`,
`f29 = -1800.0`, `f30 = 1500.0`, `f31 = 80.0`. Epilogue
`0x806F22D0..0x806F2304`.

**What it is.** The collector that fills the respawn spawner's 16-byte records
at `+0x39C` (class documented at `docs/COVERAGE_WAVE63_BASES.md:39-44` and
translated as [`fn_806f1d70`]/[`fn_806f1fd0`]). It walks the loaded stage's
sprite list of type `0xE0` (224), converts each sprite's `u16` pair to world
coordinates through the per-type 40-byte record at
`0x8030A040 + 224*40 = 0x8030C340`, keeps only the records whose point falls in
the same section-4B area as the actor's `+0x349` byte, appends them linearly at
`+0x39C + count*16`, and raises each by `80.0 * (count & 3)` in z. The spawn
loop [`fn_806f1fd0`] then spawns actor `0x20F` from every record.

**Exact behaviour.**

```
stage_index = *(u8*)(*(u32*)0x8042A1C8 + 0x120E)
stage = *(u32*)0x80429E98 + stage_index*0x3B0
if *(u32*)(stage+0x2C) == 0 { stage = 0 }          // the compiler's null substitution
records = *(u32*)(stage+0x24)                      // 16-byte sprite records
count   = *(u32*)(stage+0x94)                      // record count
group   = *(u32*)(this+4) & 0xF                    // rlwinm r26,r0,0,28,31
*(u32*)(this+0x398) = 0
let mut i = 0
while i < count {
    let rec = records + i*16
    let tag = *(u16*)(rec+0)
    if tag != 0xE0 { i += 1; continue }
    if (*(u32*)(rec+8) & 0xF) != group { i += 1; continue }
    let tbl = 0x8030A040 + tag*40                  // = 0x8030C340 for 0xE0
    let mut p = [0f32; 3];
    p[0] = (*(u16*)(rec+2) as i32 + *(i32*)(tbl+4)) as f32;
    p[1] = (*(u16*)(rec+4) as i32 - *(i32*)(tbl+8)) as f32;
    p[2] = if *(u8*)(rec+0xD) != 0 { f29 /*-1800.0*/ } else { f30 /*1500.0*/ };
    let k = *(u32*)(this+0x398);
    p[2] += f31 * (k & 3) as f32;                  // +0, +80, +160, +240
    if section4b_entry_at_point(stage, &p) != *(u8*)(this+0x349) {
        i += 1; continue
    }
    let slot = this + 0x39C + k*16;
    *(f32*)(slot+0) = p[0];
    *(f32*)(slot+4) = p[1];
    *(f32*)(slot+8) = p[2];
    *(u32*)(slot+0xC) = *(u32*)(rec+8);
    *(u32*)(this+0x398) = k + 1;
    i += 1
}
```

The two `int -> float` conversions are the signed `xoris 0x8000` + `0x4330`
idiom with the `2^52 + 2^31` bias (`lfd f28,8(r6)` with `r6 = 0x807CC0D0`; the
double at `0x807CC0D8` is `0x4330000080000000`), followed by `fsubs`. The
`+0xC` byte selector is read as `lbz r0,13(r28)`; a non-zero value picks
`-1800.0`, zero picks `1500.0`.

`section4b_entry_at_point` (`0x8008E770`, ledger `resource::course`) is called
with `r3 = stage`, `r4 = &p` and returns the area byte (`entry+0xC`, `0xFF` when
nothing matches), which is compared against the actor's `+0x349`.

The `if *(u32*)(stage+0x2C) == 0 { stage = 0 }` arm is the same substitution
`fn_806514d0` documents: the code then reads `36(r29)` unconditionally, i.e. it
would dereference `0x24`. In the shipped game `+0x2C` is the "stage loaded"
pointer and is non-zero whenever this body runs; reproduce the substitution
verbatim (do not "fix" it) and note the latent null dereference.

**Signature.**

```rust
/// Collects the respawn spawner's type-0xE0 level records.
///
/// # Original
/// - Ghidra: `FUN_806f2100` at `0x806F2100` (520 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/806f2100_FUN_806f2100.c`
///
/// # Arguments
/// * `this` - live respawn spawner; sprite group `+0x4 & 0xF`,
///   area byte `+0x349`, record count `+0x398`, records `+0x39C`
///   (16 bytes each: `[x, y, z, flags]`).
///
/// # Returns
/// Nothing; the count is published through `+0x398`.
///
/// # Side effects
/// Appends up to `stage->+0x94` records, each with `+8 & 0xF` matching the
/// actor's flag nibble, raising each by `80.0 * (count & 3)` in z.
///
/// # Safety
/// The stage table `0x80429E98` and the stage manager `0x8042A1C8` must be
/// initialised; the record block `+0x39C` must have room for
/// `stage->+0x94` entries (callers allocate it as part of the spawner).
///
/// # Confidence
/// high - every guard, table, constant and store is literal, and the consumer
/// `fn_806f1fd0` fixes the record layout and the `0x20F` actor id.
pub unsafe fn fn_806f2100(this: usize)
```

**Constants.** type tag `0xE0` (224); per-type table `0x8030A040`, stride 40,
entry `0x8030C340` = `{0x020F0000, 8, -8, 0, 8, 48, 32, 0, 0, 0x00080000}` (the
first word carries actor id `0x20F`; `+0x14` = 48 and `+0x18` = 32 are the
collision half-extents consumed by `fn_806f1fd0`; `+0x24` = 0x00080000 is the
halfword it copies); `+4` = x bias (8), `+8` = y bias (-8, subtracted); z
constants `-1800.0` (`0x807CC0E0`) and `1500.0` (`0x807CC0E4`), z step `80.0`
(`0x807CC0E8`), bias double at `0x807CC0D8`; stage table `0x80429E98` stride
`0x3B0`, stage `+0x2C` loaded pointer, `+0x24` records, `+0x94` count, `+0x9C`
section count (inside `0x8008E770`); stage index byte `+0x120E` of
`0x8042A1C8`; **group nibble = `*(u32*)(this+4) & 0xF`**, matched against
`*(u32*)(rec+8) & 0xF` (both `rlwinm ...,0,28,31`); record stride 16, ring
offset `+0x39C`, count `+0x398`.

---

## 7. `0x806F2710` - rider-timer record collector (twin of row 6)

**Evidence.** `work/ghidra/rel-d_basesNP/export/806f2710_FUN_806f2710.c`,
520 bytes, 130 words, synthetic `0x806F2710`. Body `0x806F2710..0x806F2914`.
Frame identical to row 6 (`stwu r1,-144`, four `stfd`/`psq_st` pairs,
`bl 0x802DCD64`), `r24 = this`, `r30 = 0x8030A040`, `r6 = 0x807CC0F0` instead of
`0x807CC0D0`.

**Byte-level relationship to row 6.** A word-for-word diff of the two bodies
(`%TEMP%\opencode\w158`, `d_806f2100.txt` vs `d_806f2710.txt`) shows exactly 14
differing words, all attributable to three things:

1. the five counter/record offsets `+0x398 -> +0x3D8` and
   `+0x39C -> +0x3DC`, `+0x3A0 -> +0x3E0`, `+0x3A4 -> +0x3E4`,
   `+0x3A8 -> +0x3E8` (ten words);
2. the literal-pool pointer `0x807CC0D0 -> 0x807CC0F0` (one word) - the block
   at `0x807CC0F0` repeats the used tail (`2^52+2^31`, `-1800.0`, `1500.0`,
   `80.0`) at `+8..+0x18`, so the *values* are identical;
3. the three `bl` displacements (three words).

So this is the same algorithm on the same stage sprite list, storing into the
**rider-timer object**'s own record block. It is the collector that
[`fn_806f2460`] starts: `0x806F2460` calls it, tests `+0x3D8`, and then either
runs `0x806F25B0` plus interface message `0x8082E9B0`, or kills the actor with
`0x80064D10(actor, 1)`. The class is the "rider-timer object"
(`0x806F2460..0x806F2A90`, `docs/COVERAGE_WAVE63_BASES.md:45-50`), whose
destructor `0x806F2A90` tears down the `0x40`-element `0x10`-stride array at
`+0x3DC` - i.e. the block this body fills is a **64-entry** array, not a
four-slot ring, which matches the linear index arithmetic.

**Exact behaviour.** Identical to row 6 with `+0x398 -> +0x3D8` and
`+0x39C -> +0x3DC`; the z ladder, the tag test, the flag test, the
`0x8008E770` area test and the `count & 3` z offset are unchanged.

**Signature.**

```rust
/// Collects the rider-timer object's type-0xE0 level records.
///
/// # Original
/// - Ghidra: `FUN_806f2710` at `0x806F2710` (520 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/806f2710_FUN_806f2710.c`
///
/// # Arguments
/// * `this` - live rider-timer object; sprite group `+0x4 & 0xF`,
///   area byte `+0x349`, record count `+0x3D8`, records `+0x3DC`.
///
/// # Returns
/// Nothing; the count is published through `+0x3D8`.
///
/// # Side effects
/// Same append/filter/z-ladder as `fn_806f2100`, into `+0x3DC`.
///
/// # Safety
/// Same as `fn_806f2100`; the `+0x3DC` block must have room for
/// `stage->+0x94` entries (the destructor documents 64 `0x10`-byte elements).
///
/// # Confidence
/// high - the body is row 6 modulo five offsets, and the diff is reproducible.
pub unsafe fn fn_806f2710(this: usize)
```

**Constants.** As row 6, except: record count `+0x3D8`, records `+0x3DC`
(16 bytes each, `+0x3DC` x, `+0x3E0` y, `+0x3E4` z, `+0x3E8` flags); literal
pool base `0x807CC0F0` (`+8` = `2^52+2^31` = `0x807CC0F8`, `+0x10` = `-1800.0`
= `0x807CC100`, `+0x14` = `1500.0` = `0x807CC104`, `+0x18` = `80.0` =
`0x807CC108`). Consumer `0x806F25B0`; interface message `0x8082E9B0`; kill
`0x80064D10(actor, 1)`.

---

## 8. `0x8061F2F0` - cannon player-select pane show/hide

**Evidence.** `work/ghidra/rel-d_basesNP/export/8061f2f0_FUN_8061f2f0.c`,
516 bytes, 129 words, synthetic `0x8061F2F0`. Body `0x8061F2F0..0x8061F4F0`
(one `blr`, last word). Frame: `stwu r1,-16(r1)`, LR at `20(r1)`,
`r31`/`r30` at `12`/`8(r1)`, `mr r30,r3` at `0x8061F304`. No `psq`, no jump
table, no indirect call.

**What it is.** The pane visibility/state setter of the cannon player-select
screen (the class of `0x8061EBD0`, `0x8061EC00` and `0x8061EE50`: "four cannon
player slots", panes at `+0x244`, `+0x248`, `+0x280`; see
`docs/COVERAGE_WAVE145_BASES.md:44-48,87` and the `fn_8061ee50` doc block in
`src/game/bases.rs:165600-165660`). It stops the layout animations, shows the
main pane, then shows or hides the seven child panes depending on the
"players present" word `+0x2C0` and on [`fn_8061ee50`], and finally starts the
layout. The pane flag it writes is bit 0 of the byte at `+0xBB` of each NW4R
pane - the same idiom as the "pane show" body `0x8061B6D0`
(`docs/COVERAGE_WAVE145_BASES.md:56`, `[+0x36C]+0xBB |= 1`).

**Exact behaviour.**

```
0x800C9500(this+0x70)                                  // stop all animations
pane(this+0x244) |= 1                                   // +0xBB bit 0
if *(u32*)(this+0x2C0) == 0 {
    pane(this+0x270) |= 1
    pane(this+0x274) &= !1
    pane(this+0x27C) &= !1
    pane(this+0x280) &= !1
    pane(this+0x284) &= !1
    pane(this+0x288) &= !1
    pane(this+0x28C) &= !1
} else {
    pane(this+0x270) &= !1
    if fn_8061ee50(this) != 0 {                        // any slot occupied
        pane(this+0x280) |= 1
        pane(this+0x284) |= 1
        pane(this+0x288) |= 1
        pane(this+0x28C) |= 1
        for id in 3..7 { 0x800C93D0(this+0x70, id, 0) } // play animations 3..6
        pane(this+0x274) |= 1
        pane(this+0x27C) |= 1
    } else {
        pane(this+0x274) &= !1
        pane(this+0x27C) |= 1
        pane(this+0x280) &= !1
        pane(this+0x284) &= !1
        pane(this+0x288) &= !1
        pane(this+0x28C) &= !1
    }
}
0x800C92F0(this+0x70, 0, 0)                            // start the layout
*(u8*)(this+0x2E1) = 1
```

`pane(x)` is `*(u8*)(*(u32*)(x) + 0xBB)`; every write is the compiler's
`rlwinm r0,r0,0,24,30` (clear bit 0) followed by an optional `ori r0,r0,1`
(set bit 0). The loop is `r31 = 0..3`, `r4 = r31+3`, `r5 = 0`
(`0x8061F41C..0x8061F434`), i.e. animation ids 3, 4, 5, 6, played with
argument 0.

**Signature.**

```rust
/// Shows or hides the cannon select screen's panes for the current slot state.
///
/// # Original
/// - Ghidra: `FUN_8061f2f0` at `0x8061F2F0` (516 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/8061f2f0_FUN_8061f2f0.c`
///
/// # Arguments
/// * `this` - cannon player-select screen; layout `+0x70`, main pane `+0x244`,
///   slot panes `+0x270`/`+0x274`/`+0x27C`/`+0x280`/`+0x284`/`+0x288`/`+0x28C`,
///   slot gate `+0x2C0`, flag byte `+0x2E1`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stops all layout animations, sets/clears bit 0 of the `+0xBB` byte of eight
/// panes, plays animations 3-6 when a cannon slot is occupied, starts the
/// layout and sets `+0x2E1 = 1`.
///
/// # Safety
/// `this` and every pane pointer must be live NW4R pane objects; the layout at
/// `+0x70` must be initialised.
///
/// # Confidence
/// medium - every guard, pane offset, animation id and flag write is literal;
/// the roles of `+0x2C0` ("any slot armed") and `+0x2E1` ("screen started") are
/// named from use, and `0x800C9500`/`0x800C92F0`/`0x800C93D0` are cited by
/// address. Those three take the same generic `+0x70` channel object that the
/// ledger files under `game::stage` (rail start/stop), and this family uses it
/// as the NW4R layout animation controller
/// (`docs/COVERAGE_WAVE145_BASES.md:56`).
pub unsafe fn fn_8061f2f0(this: usize)
```

**Constants.** layout object `+0x70`; panes `+0x244`, `+0x270`, `+0x274`,
`+0x27C`, `+0x280`, `+0x284`, `+0x288`, `+0x28C`; pane flag byte `+0xBB`, bit 0;
screen gate `+0x2C0`; flag byte `+0x2E1`; animations 3-6 (arg 0); layout calls
`0x800C9500(obj)`, `0x800C93D0(obj, id, 0)`, `0x800C92F0(obj, 0, 0)`;
occupancy query `0x8061EE50` (`fn_8061ee50`, slots `+0x2C4..+0x2D0`).

---

## Cross-checks a translation lane should repeat

1. Re-read every `size` from
   `work/ghidra/rel-d_basesNP/export/functions.tsv`; the eight must be
   540/536/532/528/524/520/520/516 and all `decompiled = true`.
2. Re-check that none of the eight addresses appears in
   `ledger/functions.csv`, in a `fn fn_<addr>` token of `src/game/bases.rs`, or
   in a `## N. \`0xADDR\`` heading of any `docs/SPEC_WAVE*_BASES.md`. (The
   wave-158 translation lane has already landed 13 rows; if it lands more before
   this spec is used, re-run `pick.js` rather than trusting this page.)
3. Row 5: decide the body boundary first. `0x807B7B70` has no export, and the
   TSV extent ends 8 bytes before its `blr` (`0x807B7D48` is the last extent
   word, `0x807B7D50` the `blr`). `0x807B7D60`/`0x807B7F30` are separate rows
   and remain untranslated.
4. Rows 6 and 7 are the same algorithm; the diff is five offsets plus the
   literal-pool base. Implement one and call the other with a field base if the
   module style allows it, otherwise duplicate with a comment naming the diff.
5. Rows 1 and the pedestal's `fn_8072e860` are twins with **three** behavioural
   differences (reset threshold 3 vs 5, three edge pairs vs four, and the
   direction/flag offsets). Do not copy the pedestal body unchanged.
6. Row 4's `psq` bracket is `qr0`/`W=0` only; do not emit a quantised load.

## References

* `docs/SPEC_WAVE157_BASES.md:180-184` - the next-band prediction this wave
  follows.
* `docs/COVERAGE_WAVE7_BASES.md:100-115` - `daRotDokan_c` identity, vtable
  `0x8080D0F8`, `0xA08` bytes.
* `docs/COVERAGE_WAVE128_BASES.md:44-58` - the `0xA08` state-machine cluster,
  `fn_80731c60`/`fn_80731cb0`/`fn_80731f30` roles.
* `docs/COVERAGE_WAVE51_BASES.md:21-42,84-88` - number-panel lift identity and
  the `0x806D62C0` rejection this wave disproves.
* `docs/COVERAGE_WAVE122_BASES.md:36-50` - `lift_spin_kinoko` identity and
  `fn_806dff70`/`fn_806dfcc0` field vocabulary.
* `docs/SPEC_WAVE129_BASES.md:1131-1198` - the `0x806DED20` spawn initialiser
  that writes the same `+0x658`/`+0x660`/`+0x6A9` block and the two message
  cells.
* `docs/COVERAGE_WAVE63_BASES.md:39-50,105-130` - respawn spawner and
  rider-timer object windows, plus the `0x806F2100` rejection.
* `docs/COVERAGE_WAVE84_BASES.md:120-122` - the `0x806F2100..0x806F2710`
  cluster request.
* `docs/COVERAGE_WAVE154_BASES.md:141-143` - the same pair named as the largest
  remaining `0x806F` rows.
* `docs/SPEC_WAVE146_BASES.md:1075-1180` - the scene teardown that calls
  `0x807B7B40`.
* `docs/SPEC_WAVE152_BASES.md:1125-1241` - the `10 x 42` save grids and the
  `0x800CE260`/`0x800CE2C0` accessors.
* `docs/COVERAGE_WAVE145_BASES.md:44-56,87` - the `0x8061` pane family and
  `0x8061EE50`.
* `docs/REL_ELF_MAPPING.md:7-42` - the `+0x1000` mapping and the extent rule.
* `src/resource/course.rs` - `section4b_entry_at_point` (`0x8008E770`).
