# Wave 156 - `game::bases` - the next eight (572-560 byte band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified by this lane. The only file written is this one.
No merge or check tool was run. The throwaway tools live under
`%TEMP%\opencode\w156` (`pick.js`, `verify.js`, `d.js`, `md.js`, `info.js`,
`who.js`, `cx.js`, `scanaddr.js`); none of them is in the repository. `d.js` is
the wave-154/155 decoder (ELF32-BE program-header mapping, ELF `psq`/GQR fields,
conditional `bclr` does not truncate a path) and `md.js` is the same decoder
retargeted at `work/dol/main.elf` so every `main.dol` callee below could be
disassembled at its real address instead of guessed from the export.

Waves 128-155 walked this bucket from the top in size order; wave 155 ended at
576 bytes and named the next two rows explicitly
(`docs/SPEC_WAVE155_BASES.md:284-288`): "the 572-byte group is
`{0x806DB340, 0x807041B0}` - so **the ninth is `0x806DB340` (572)** and the
tenth is `0x807041B0` (572)". This wave claims those two and the six rows after
them:

```
0x806DB340 572 | 0x807041B0 572 | 0x80745DC0 568 | 0x8077D340 568
0x80788090 568 | 0x8079B7C0 568 | 0x806DF820 564 | 0x80628F70 560
```

## The eight

| # | address | TSV size | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x806DB340` | 572 | 143 | `806db340_FUN_806db340.c` | ready (high) |
| 2 | `0x807041B0` | 572 | 143 | `807041b0_FUN_807041b0.c` | ready (medium) |
| 3 | `0x80745DC0` | 568 | 142 | `80745dc0_FUN_80745dc0.c` | ready (high) |
| 4 | `0x8077D340` | 568 | 142 | `8077d340_FUN_8077d340.c` | ready (medium) |
| 5 | `0x80788090` | 568 | 142 | `80788090_FUN_80788090.c` | ready (medium) |
| 6 | `0x8079B7C0` | 568 | 142 | `8079b7c0_FUN_8079b7c0.c` | ready (high) |
| 7 | `0x806DF820` | 564 | 141 | `806df820_FUN_806df820.c` | ready (high) |
| 8 | `0x80628F70` | 560 | 140 | `80628f70_FUN_80628f70.c` | ready (medium) |

**Ready: 8. Conditional: 0.** 4,540 bytes / 1,135 words in total.

Every `size` above is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim, `decompiled` is
`true` for all eight and all eight export files exist on disk. All eight
extents end exactly on their own `blr` (`address + size - 4`), so no row needs
the `docs/REL_ELF_MAPPING.md:37-42` extent correction:

| # | `blr` | `address + size - 4` | span is exact | next row | gap |
|--:|---|---|:--:|---:|---:|
| 1 | `0x806DB578` | `0x806DB578` | yes | `0x806DB6E0` | 356 |
| 2 | `0x807043E8` | `0x807043E8` | yes | `0x807045B0` | 452 |
| 3 | `0x80745FF4` | `0x80745FF4` | yes | `0x80746100` | 264 |
| 4 | `0x8077D574` | `0x8077D574` | yes | `0x8077D580` | 8 |
| 5 | `0x807882C4` | `0x807882C4` | yes | `0x807882E0` | 24 |
| 6 | `0x8079B9F4` | `0x8079B9F4` | yes | `0x8079BA00` | 8 |
| 7 | `0x806DFA50` | `0x806DFA50` | yes | `0x806DFA60` | 12 |
| 8 | `0x8062919C` | `0x8062919C` | yes | `0x806291A0` | 0 |

Gap contents, read word by word:

* row 1: one zero word, then `0x806DB580` (span `0x806DB580..0x806DB640`,
  196 bytes: a per-frame body of the same class that runs both the `+0x58C` and
  `+0x5C8` `+0x10` slots, advances the two `0x800DF890` oscillators at
  `+0x4C8`/`+0x4DC` into `+0x104`/`+0x4B0..+0x4B4` and hands a position to
  `0x800C0750`), three zero words, `0x806DB650` (16 bytes, `lwzu r12,0x18(r3)` /
  `lwz r12,0x10(r12)` / `bctr` - a one-slot forwarder), two zero words,
  `0x806DB660` (40 bytes, `0x800C07B0` on `+0x394`) and `0x806DB690` (80 bytes,
  the release body: unlinks `+0x4F0` through `0x800DAED0`, clears `+0x58A`).
  None of them is part of this body;
* row 2: one zero word, then `0x807043F0` (span `0x807043F0..0x80704530`,
  324 bytes: the class's per-frame dispatcher - two descriptor lookups against
  `0x8082F260`/`0x8082F2A0`, the `0x80064F50` gate, then fourteen `bl` calls
  into the `0x8070` update run) and `0x80704540` (112 bytes), three zero words,
  then the next TSV row;
* row 3: two zero words, then `0x80746000` (span `0x80746000..0x80746064`,
  104 bytes: the `0x80064F50` gate plus a **2**-iteration `0xE4`-stride
  `0x8007FDA0` loop over `+0x474..+0x63C`) and `0x80746070` (span
  `0x80746070..0x807460FC`, 144 bytes: `0x8016A0F0(+0x3B4,2,1)` and the
  `+0x3B4` `+0x14` slot, then a **2**-iteration `0x40`-stride
  `0x8016A0F0(...,2,1)` + `+0x14` loop over `+0x3F4..+0x474`), two zero words,
  then the next row;
* rows 4, 6 and 7: only zero words (2, 2 and 3 words);
* row 5: two zero words, a one-word `blr` stub at `0x807882D0`, then three zero
  words and the next row `0x807882E0` (28 bytes, translated `fn_807882e0`);
* row 8: no gap at all; the next row `0x806291A0` (76 bytes, the translated
  `fn_806291a0`) starts on the very next word.

The decoder reports **no unknown word** in the 1,135 words: zero unknown
opcodes, zero unhandled X-form, zero unhandled paired-single form, zero branch
leaving its own body, and every `bl` lands on a named callee outside the body.
`psq` appears in three rows only, always as the `qr0` prologue/epilogue save of
`f31` (row 1, one `psq_st` + one `psq_l` at `W = 0`), `f31` (row 3) or
`f31`/`f30` (row 6):

| row | `psq` words |
|--:|---|
| 1 | 1 `psq_st f31,88` + 1 `psq_l f31,88`, `qr0` |
| 3 | 1 `psq_st f31,72` + 1 `psq_l f31,72`, `qr0` |
| 6 | `psq_st f31,152`/`f30,136` + `psq_l f31,152`/`f30,136` in the frame, and **twelve data-side `qr0` words** at `0x8079B844..0x8079B8E0` |
| 2, 4, 5, 7, 8 | none |

Row 6 is the only row with data-side paired-single traffic, and it is all
`qr0`: `psq_l`/`psq_st` of two `f32` pairs at `W = 0` and `W = 1` over 12-byte
vectors, plus `ps_sub` (`f2 = f0 - f1`) on the same pairs. That is exactly the
type-0 (`f32`) case that `docs/ANALYSIS_WAVE127_BASES.md:50-59` classifies as
fully reconstructible: every lane stores an ordinary `stfs`-compatible float.
No row reads or writes a `qr3` value, so the wave-138
`GQR3 = 0x00050005` pin is not needed by anything in this set.

### Why all eight are ready

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image or out of `main.dol`, every callee is named by address (or by its
  ledger name), and the body can be written as code with no assumed calling
  convention beyond the standard PowerPC EABI. Data in `main.dol`
  (`0x800xxxxx`, `0x802xxxxx`, `0x803xxxxx`, `0x804xxxxx`), in this REL's data
  segments, or in this REL's BSS (`0x80822900..0x80834D84`) is an *input*, named
  by address.
* **conditional** - exactly one named input would still be missing.

Three rows deserve a sentence each, because a reader may expect a condition:

* Row 1 was rejected by waves 9 and 112 for a "paired-single reconstruction"
  (`docs/COVERAGE_WAVE9_BASES.md:64-65` and `:163`,
  `docs/COVERAGE_WAVE112_BASES.md`) and wave 155 repeated that rejection.
  The only `psq` in the body is the `qr0` pair that saves the second half of
  `f31` next to `stfd f31` - the standard MWCC float save, with no data-side
  quantisation. The `ldexpf` in the export is the decompiler's model of the
  *prologue* `psq_st`, not of the body. The rejection is now false.
* Row 4 was deferred by wave 100 (`docs/COVERAGE_WAVE100_BASES.md:155-159`)
  because "reconstructing all four branches honestly needs the
  `0x8079EE70`/`0x800F6410` prototypes confirmed first". Both are confirmed in
  this lane from `main.dol` (see the shared callee notes); nothing else was
  missing.
* Row 8 stores its state descriptors in this REL's BSS (`0x80825BA0`,
  `0x80825E60`, `0x80825EE0`, `0x80825F20`, all reached as
  `0x80825A78 + offset`). They read as zero at file level and are built by the
  class registrar at run time, exactly like row 1's `0x8082DB68`/`0x8082DC28`
  and row 2's `0x8082F1E0`/`0x8082F260`. The body only pushes them; their
  contents are not an input to the body.

## Method

**Address mapping.** Re-derived from the program headers of
`work/rels/elf/d_basesNP.elf`, not copied:

```
PT_LOAD  p_offset  p_vaddr     filesz    memsz
  0      0x0010F0  0x806000F0  0x1C6004  0x1C6004
  1      0x001C70F4 0x807C60F4 0x0004FC  0x0004FC
  2      0x1C75F0  0x807C65F0  0x00000C  0x00000C
  3      0x1C7600  0x807C6600  0x00A5FC  0x00A5FC
  4      0x1D1C00  0x807D0C00  0x051CB8  0x051CB8
  5      0x000000  0x80822900  0x000000  0x012484 (zero fill)
```

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) +
0x1000` for every code and data address below `0x807C60F4`, and segments 1-4
cover the literal pools in `0x807Cxxxx`/`0x807Dxxxx`. `0x80822900..0x80834D84`
is this REL's BSS and reads as zero at file level; `0x80825A78`,
`0x80825BA0`, `0x80825E60`, `0x80825EE0`, `0x80825F20`, `0x80832EB0`,
`0x8082DB68`, `0x8082DC28` and `0x8082F1E0`/`0x8082F260` are all in it.

`main.dol`-resident data and code are read from `work/dol/main.elf` through
**its own** program headers (`md.js ph`):

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

so `0x80315B90` (row 2) is **file** data while `0x80377DC8..0x80377DF0`,
`0x80377C48`, `0x8042A038`, `0x8042A18C`, `0x8042A1A0`, `0x8042A1C8`,
`0x80429D28`, `0x80429F50`, `0x80429F7C`, `0x8042A27C`, `0x8042A468` and
`0x8042A488` are `main.dol` **BSS** and read as zero at file level.

**Candidate selection (recomputed).** `%TEMP%\opencode\w156\pick.js` re-reads
four sources from scratch:

* `work/ghidra/rel-d_basesNP/export/functions.tsv` - 5,615 data rows, all
  `decompiled = true`, and all 5,615 export files named in the `file` column
  exist on disk;
* `ledger/functions.csv` - 17,198 distinct `address` fields (case-insensitive);
* `src/game/bases.rs` - every `fn fn_<addr>` definition token (3,782 at the
  first selection pass, 3,799 at the verification pass; the difference is
  bodies the concurrent wave-155/156 coverage lanes added, none of them in this
  set);
* every `#{2,4} ` heading of **all** `docs/SPEC_WAVE*_BASES.md` files that
  existed before this one (285 distinct `0x` addresses at the selection pass,
  294 at the verification pass as concurrent spec lanes landed). The task names
  `SPEC_WAVE12*_BASES.md`, of which waves 128 and 129 are the only matches; the
  superset is used so that no address already claimed by a wave-130..155
  heading can be picked twice.

That leaves **1,803** rows. The eight largest are the table above. Ties are
broken by **ascending address**; this is the only place the ordering is not
forced by size, and it is stated so wave 157 can reproduce it:

* the 572-byte group is `{0x806DB340, 0x807041B0}` - exactly the two rows
  wave 155 named as the ninth and tenth, so both are claimed here;
* the 568-byte group is
  `{0x80745DC0, 0x8077D340, 0x80788090, 0x8079B7C0}` - exactly four rows;
* the 564-byte row is `0x806DF820` alone;
* the 560-byte group is `{0x80628F70, 0x80763C20, 0x807A0090}` - so **the ninth
  is `0x80763C20` (560)** and the tenth is `0x807A0090` (560), neither in the
  ledger, the Rust file or any spec heading;
* the 556-byte row is `0x8077C440` (556), then `0x80741A70` (552), then the
  548-byte group `{0x807390D0, 0x80776440, 0x807B2550}`.

No `0x80763C20`/`0x807A0090` input is lost by deferring them; they are simply
the next two rows of the same band.

None of the eight is referenced by a `fn fn_<addr>` body or by any spec
heading (`verify.js` re-counts all four sources independently). The only prose
appearances are the previous lanes' skip lists and neighbour notes:

* `0x806DB340` - `docs/COVERAGE_WAVE9_BASES.md:64-65,163`,
  `docs/COVERAGE_WAVE112_BASES.md` (paired-single rejection, now disproved) and
  `docs/SPEC_WAVE155_BASES.md:284-286,296`;
* `0x807041B0` - `docs/COVERAGE_WAVE50_BASES.md:22` ("The create body
  `0x807041B0`") and `docs/SPEC_WAVE155_BASES.md:284-286`;
* `0x80745DC0` - `docs/COVERAGE_WAVE87_BASES.md:145` (bulk skip list);
* `0x8077D340` - `docs/COVERAGE_WAVE100_BASES.md:155-159`;
* `0x80788090` - `docs/COVERAGE_WAVE140_BASES.md:53`,
  `docs/SPEC_WAVE153_BASES.md:520-534` (named as `fn_80787bd0`'s model/binding
  pass) and `docs/SPEC_WAVE155_BASES.md:296`;
* `0x8079B7C0` - `docs/SPEC_WAVE154_BASES.md:522-531` and
  `docs/SPEC_WAVE155_BASES.md:296` (decoded *in passing* as row 5's callee,
  never claimed as a row);
* `0x80628F70` - `docs/COVERAGE_WAVE140_BASES.md:53` (skip list, "560");
* `0x806DF820` - no prior mention anywhere.

### Shared callee notes used by several rows

Everything below was read out of `work/dol/main.elf` with `md.js`, not inferred
from the Ghidra export. These are the prototypes the earlier waves were missing.

* `0x800690C0(dst, mode, name, arg4, size)` = `0x8016A7F0(dst, mode, name,
  arg4, size, mode != -1)` (a `dRes`-style holder create that returns 0/1;
  the sixth word is the wrapper's `addis`/`cmpli` test for `mode == -1`);
  `0x800690E0(dst)` = `0x8016A8E0` (release: `0x8016A5D0`, then virtual `+0xC`,
  then `0x8016E760`).
* `0x800DF860(out, a2..a9)` packs eight **halfwords** at `out` in the order
  `[a7, a6, a3, a4, a5, a8, a9, a2]` (`fn_800df860`); `0x800DF890(record)` is
  the integer oscillator of `fn_800df890`.
* `0x800C06C0(holder)` = create `0x800690C0(holder, -1, [0x80377C48], 0,
  0x20)`, call `0x800C0310(holder)`, release `0x800690E0(holder)`, then
  pre-compute a float from `+0x12C` (`0x800C06C0..0x800C074C`).
* `0x800C0750(iface, vec3)` stores the three words at `iface+0xE0..+0xE8`,
  stores `f1` at `iface+0x128`, and when `iface+0x124 != 0` calls
  `0x800C0540` then `0x800C0440`.
* `0x800DB180(f1..f5, rec, owner, id, flag, arg8)` stores the five floats at
  `rec+0x4C..+0x5C` (scaled by `arg8`'s vector when `arg8 != 0`), `id` at
  `rec+0x40`, `flag` at `rec+0x45`, `rec+0x34 = f4 - f5`, then tail-calls the
  record's `+0xC` slot (`0x800DB180..0x800DB204`).
* `0x800D9130(out, spawn_id, pos, vel, area, angle255, flag)` looks `spawn_id`
  up with `0x800D90C0` (an id-keyed record table: 8-byte entries, id at `+0`,
  count at `+0xA8`, base at `+0x38`), clears `out+0x1C`/`out+4`, and fills the
  placement record (`0x800D9130..0x800D9278`).
* `0x800F6AB0(actor)` returns 0 while `actor+0x180 > 0`, otherwise clamps the
  progress `+0x174` into `[0, +0x17C]` at `+0x178` per frame, mirrors it into
  the `+0xDC..+0xE4` scale triple and tail-calls `0x800F6390`
  (`0x800F6AB0..0x800F6B48`).
* `0x800FC040()` reads the current input record through `0x801002F0(r13-0x54F4)`
  and returns **0** when the record's `+3` byte is `'1'`, **1** when it is
  `'0'`, and **2** otherwise.
* `0x800F6410(actor, vec3)`: `0x80170980` on the vector, `fabs` of x and z
  against the SDA2 dead band, and the resulting placement
  (`0x800F6410..0x800F6550`).
* `0x800F69F0(x, y, z, owner, target, arg6, angle, vec)` is the world-map
  placement/effect spawn: `0x801611C0`, the three floats, then the owner's
  `+0x5C` and `+0x84` virtual slots with `angle` and `vec`
  (`0x800F69F0..0x800F6AB0`).
* `0x80100200(out, scene, node_id)` = `0x800F32F0(scene + 0x338C*0x3F08 +
  0x6C7F4, node_id)`, then copies the node's `+4` transform into `out`
  (three `f32`).
* `0x801C0BD0(x, y, z, out)` writes the twelve floats of
  `[1 0 0 x; 0 1 0 y; 0 0 1 z]` (`fn_801c0bd0`).
* `0x801C14C0(vec3)` is the vector magnitude: the body forms
  `mag = z*z + x*x + y*y`, refines `1/sqrt(mag)` and returns `mag * refine`
  (`fn_801c14c0`; its doc comment calls the result "squared length", but the
  reciprocal-square-root refinement only produces the *length*, and row 6's
  cosine would not normalise otherwise).
* `0x80128E60(actor, f32)` sets event flag `0x2C` through `0x80056C70` and
  stores the float at `actor+0x2528` (`0x80128E60..0x80128E9C`).
* `0x802DE514(x)` is `abs(x)`: `srawi r4,r3,31; xor; subf`
  (`0x802DE514..0x802DE520`).
* `0x802DC69C(s)` is `strlen`; `0x802E16E8` is the variadic `snprintf`.
* `0x8016A150(element, vec3)` copies three words to `[element+4]+0xDC..+0xE4`;
  `0x8016A170(element, value)` forwards `[element+4]` to `0x802574E0`;
  `0x8016A1A0(element, flag)` runs `0x8016A0F0(element,2,0)`, the `+0xC` slot
  with `(1,0,0)` and `0x8016A0F0(element,2,1)` when `flag == 0`.
* `0x80195380(manager, id, flag)` = `0x80195090(manager+0x8F8, id, flag)`.
* `0x800B2EE0(v)` = `0x80161460(r13-0x573C, v)`, which advances the scale
  record and returns `(new * v) >> 32`; the **return value is live** in rows 2
  and 8 even though `fn_800b2ee0` declares `()`.
* `0x8007FA40(x0,y0,x1,y1, layer, owner, cb0, cb1, cb2, mask, group, scale)` is
  `fn_8007fa40` (scroll-layer builder); `0x8007F900(layer)` links it into the
  draw list (`fn_8007f900`).

---

## 1. `0x806DB340` - `daLiftScalesStep_c` constructor

**Evidence.** `work/ghidra/rel-d_basesNP/export/806db340_FUN_806db340.c`,
572 bytes, 143 words, synthetic `0x806DB340`. Body `0x806DB340..0x806DB578`
(one `blr`, last word). Frame: `stwu r1,-96(r1)`, manual `stfd f31` +
`psq_st f31,88(0,qr0)`, `__savegprlr_27` (`0x802DCD74`) and
`__restgprlr_27` (`0x802DCDC0`). No jump table, no indirect call. The export's
`unaff_GQR0`/`ldexpf` block is the prologue save only (see "Why all eight are
ready").

**What it is.** The constructor of the `daLiftScalesStep_c` lift actor
(`docs/COVERAGE_WAVE9_BASES.md:163` names the class; the four-state registrar
`fn_806dbed0` builds `StateID_Hold`/`StateID_Release`/`StateID_NoDisp`/
`StateID_Disp` at `0x8082DB68`/`0x8082DBA8`/`0x8082DBE8`/`0x8082DC28` and the
shared vtable is `0x807FBD80`). This body pushes the **Disp** descriptor into
the `+0x5C8` interface and the **Hold** descriptor into `+0x58C`, configures the
two 16-halfword parameter blocks, and sets the two window widths from the low
nibble of the actor flags. It is the `create` pointer of the spawn record at
`0x807FBA84` (`{0x806DB340, ...}`; the same record's vtable
`0x807FBA88..` holds `0x80064350`, `0x80064380`, the release body
`0x806DB690` and the per-frame bodies `0x806DB580`/`0x806DB660`).

**Signature.**

```rust
/// Constructs the `daLiftScalesStep_c` lift actor.
///
/// # Original
/// - Ghidra: `FUN_806db340` at `0x806DB340` (572 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/806db340_FUN_806db340.c`
///
/// # Arguments
/// * `actor` - the lift actor; parameter nibble in `+0x004`, state id `+0x104`,
///   interfaces `+0x394`/`+0x58C`/`+0x5C8`, width/offset `+0x570..+0x578`,
///   emitters `+0x4C8`/`+0x4DC`, state record `+0x4F0`.
///
/// # Returns
/// `1` - the last instruction before the epilogue is `li r3,1`.
///
/// # Safety
/// `actor` must be a fresh, zeroed actor and the `+0x58C`/`+0x5C8` interfaces
/// must expose a callable `+0x18` slot.
pub unsafe fn fn_806db340(actor: usize) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | actor flags: bits 0-3 = window index, bit 4 = `+0x589` |
| `+0x0AC`..`+0x0B4` | vec3 | actor position (read only, for the `+0x394` hand-off) |
| `+0x0DC`..`+0x0E4` | vec3 | scale, `4096.0` after this body |
| `+0x104` | `s16` | state id, `0` |
| `+0x318`,`+0x31C` | `f32` | `16 * (index + 1)` |
| `+0x320`,`+0x324` | `f32` | `0.0` |
| `+0x394` | object | state interface (holder `+0x0`, ctor `0x800C06C0`) |
| `+0x48C`..`+0x494` | vec3 | `0.0` |
| `+0x4B0`,`+0x4B2`,`+0x4B4` | `u16` | `0` |
| `+0x4C0` | `u32` | window index (flags & 0xF) |
| `+0x4C4` | `u32` | `0` |
| `+0x4C8` | 8 halfwords | display parameter block A (`a2 = 0x80`) |
| `+0x4DC` | 8 halfwords | display parameter block B (`a2 = 0x40`) |
| `+0x4F0` | record | state record handed to `0x800DB180` |
| `+0x53A` | `u8` | copy of `*0x80429D28` |
| `+0x570` | `f32` | window offset, `2.0 - 0.5 * +0x574` |
| `+0x574` | `f32` | window width, `16 * index` |
| `+0x578` | `f32` | `8 * index + 4.0` |
| `+0x57C` | `f32` | `0.0` |
| `+0x580`,`+0x582` | `u16` | `0` |
| `+0x589` | `u8` | `(flags >> 4) & 1` |
| `+0x58A` | `u8` | `1` (armed) |
| `+0x58C` | interface | Hold state holder |
| `+0x5C8` | interface | Disp state holder |

### Exact behaviour

```
// __savegprlr_27 saved r27 = this
let flags = read_u32(actor + 4);
let index = flags & 0xF;

write_u16(actor + 0x104, 0);
write_f32(actor + 0x57C, 0.0);
write_u8 (actor + 0x58A, 1);
write_u16(actor + 0x580, 0);
write_u16(actor + 0x582, 0);

0x806DB9D0(actor);                       // address-only pre-init of +0x4F0

let flags = read_u32(actor + 4);         // re-read at 0x806DB38C
write_u8(actor + 0x589, ((flags >> 4) & 1) as u8);

let f = index as f32;                    // via the 2^52 bias at 0x807CB908
write_f32(actor + 0x574, 16.0 * f);      // 0x807CB8EC
write_u32(actor + 0x4C0, index);
write_f32(actor + 0x48C, 0.0);
write_f32(actor + 0x490, 0.0);
write_f32(actor + 0x494, 0.0);
write_u32(actor + 0x4C4, 0);
write_u16(actor + 0x4B0, 0);
write_u16(actor + 0x4B2, 0);
write_u16(actor + 0x4B4, 0);

0x800C06C0(actor + 0x394);               // state-interface holder ctor

let band = 8.0 * f;                      // f31
write_f32(actor + 0xDC, 4096.0);         // 0x807CB8F0
write_f32(actor + 0xE0, 4096.0);
write_f32(actor + 0xE4, 4096.0);
write_f32(actor + 0x578, band + 4.0);    // 0x807CB8F8
write_u8 (actor + 0x53A, read_u8(0x80429D28));

0x800DB180(
    /*f1*/ 2.0, /*f2*/ 0.0, /*f3*/ 8.0,  // 0x807CB8FC, 0x807CB8E8, 0x807CB8F4
    /*f4*/ band + 4.0, /*f5*/ -(band + 4.0),
    /*r3*/ actor + 0x4F0, /*r4*/ actor, /*r5*/ read_i16(actor + 0x104),
    /*r6*/ 1, /*r7*/ 0, /*r8*/ read_u8(0x80429D28));

let w = 16.0 * f + 16.0;                 // 2.0 * band + 16.0
write_f32(actor + 0x320, 0.0);
write_f32(actor + 0x324, 0.0);
write_f32(actor + 0x318, w);
write_f32(actor + 0x31C, w);

vmethod0(actor + 0x5C8, 0x18, 0x8082DC28);   // push Disp
vmethod0(actor + 0x58C, 0x18, 0x8082DB68);   // push Hold

0x800DF860(actor + 0x4C8, 0x80, 0x10, 2, 0x800, 0, 0, 8, 0x80);
0x800DF860(actor + 0x4DC, 0x40, 0x10, 2, 0x800, 0, 0, 8, 0x80);

let x = -(0.5 * read_f32(actor + 0x574)) + 2.0;   // 0x807CB900, 0x807CB8FC
write_f32(actor + 0x570, x);
let pos = [read_f32(actor + 0xAC) + x,
           read_f32(actor + 0xB0),
           read_f32(actor + 0xB4)];
0x800C0750(actor + 0x394, &pos);

return 1;
```

### Constants

| address | value | use |
|---|---|---|
| `0x807CB8E8` | `0.0f` | zero for `+0x57C`, `+0x48C..`, `+0x320/0x324`, `f2` |
| `0x807CB8EC` | `16.0f` | width per index, and the re-used `f1` of the `+0x318` pair |
| `0x807CB8F0` | `4096.0f` | initial scale |
| `0x807CB8F4` | `8.0f` | half-width per index (`f31`) |
| `0x807CB8F8` | `4.0f` | `+0x578` bias |
| `0x807CB8FC` | `2.0f` | `f1` of the state record and the `+0x570` bias |
| `0x807CB900` | `0.5f` | `+0x570` slope |
| `0x807CB908` | `2^52` (f64) | int -> float bias |
| `0x80429D28` | BSS byte | actor "mode" byte, copied to `+0x53A` and arg 8 |
| `0x8082DB68` | BSS object | Hold state descriptor (built by `fn_806dbed0`) |
| `0x8082DC28` | BSS object | Disp state descriptor (built by `fn_806dbed0`) |

### Callees

`0x802DCD74`/`0x802DCDC0` (`__savegprlr_27`/`__restgprlr_27`), `0x806DB9D0`
(24-byte export, no ledger row: `+0x4F0` pre-init; also called by
`0x806DB6E0`, `0x806DBB20`, `0x806DBB80`), `0x800C06C0` (state-interface holder
ctor, verified), `0x800DB180` (state-record writer, verified), `0x800DF860`
(`fn_800df860`, eight-halfword packer), `0x800C0750` (interface position setter,
verified).

### Verdict

**ready (high).** Every store, both int->float conversions, both descriptor
pushes, both parameter blocks and the final interface write are literal. The
wave-9/112/155 rejection was the `qr0` prologue save, which carries no
information about the body.

---

## 2. `0x807041B0` - `boat_light_wood` actor create

**Evidence.** `work/ghidra/rel-d_basesNP/export/807041b0_FUN_807041b0.c`,
572 bytes, 143 words, synthetic `0x807041B0`. Body `0x807041B0..0x807043E8`
(one `blr`). Frame: `stwu r1,-32(r1)`, `__savegprlr_29`. No `psq`, no jump
table, one indirect call per state push. The export's
`/* WARNING: Globals starting with '_' overlap smaller symbols */` is the
`0x8042A1C8` cell, not a reconstruction failure.

**What it is.** The create body of the `boat_light_wood` actor
(`docs/COVERAGE_WAVE50_BASES.md:22`; the loader `fn_80702750` opens
`g3d/boat_light_wood.brres` and binds `light_fog_st`). It is the `create`
pointer of the spawn record at `0x8080411C`; the same record carries the class
vtable (`0x80064350`, `0x80064380`, `0x807045B0`, ...), the state names
`light_fog_st` (`0x808040E8`) and `light_fog_wait` (`0x80804104`) and the effect
name `Wm_ob_boat` (`0x808040F8`). It resets the actor, runs the model loader,
optionally re-places the actor from the spawn flags through `0x800D9130`,
allocates the two 16-bit draws, clears the `+0xA14` palette and pushes one of
two state descriptors, then returns the "keep" code (1) or the "discard" code
(2) from the world/mode gates.

**Signature.**

```rust
/// Creates the `boat_light_wood` light actor.
///
/// # Original
/// - Ghidra: `FUN_807041b0` at `0x807041B0` (572 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807041b0_FUN_807041b0.c`
///
/// # Arguments
/// * `actor` - the actor; flags `+0x004`, position `+0x0AC`, scale `+0x0DC`,
///   velocity `+0x0E8`, tilt `+0x318..+0x324`, state interface `+0x394`,
///   model handle `+0x474`, model holder `+0x478`, animation block `+0x53C`,
///   animation vtable `+0x558`, palette `+0xA14`, transform record `+0xDBC`,
///   spawn words `+0xDF0..+0xE08`.
///
/// # Returns
/// `1` when the actor stays, `2` when the mode gates ask the caller to discard
/// it.
///
/// # Safety
/// `actor` must be a fresh actor, `+0x394`/`+0x558` must expose their virtual
/// slots, and the `main.dol` cells `0x80429F7C`, `0x8042A1C8`, `0x8042A038`
/// must be live.
pub unsafe fn fn_807041b0(actor: usize) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | spawn flags: bit 0 = "re-place", bits 12-13 = index selector, bit 16 = mirror flag, bits 20-23 = spawn id, bits 24-31 = angle |
| `+0x0AC`..`+0x0B4` | vec3 | position |
| `+0x0DC`..`+0x0E4` | vec3 | scale, `1.0` |
| `+0x0E8`..`+0x0F0` | vec3 | velocity, `0.0` |
| `+0x10C` | object | placement area/param |
| `+0x318`,`+0x31C` | `f32` | `1024.0` |
| `+0x320`,`+0x324` | `f32` | `-84.0` / `80.0` |
| `+0x394` | interface | state interface (vtable `+0x18` = push) |
| `+0x474` | object | model handle (set by `fn_80702750`) |
| `+0x478` | object | animation handle, word store via `0x8016A000` |
| `+0x53C` | object | animation selector passed to the `+0x558` `+0x8` slot |
| `+0x558` | object | animation vtable |
| `+0xA14` | palette | four colour words, fade `+0x0C`, flags `+0x10..+0x13` |
| `+0xDBC` | record | placement record written by `0x800D9130` |
| `+0xDB4` | object | source of the `+0x478` handle |
| `+0xDF0`..`+0xDF8` | vec3 | placement output |
| `+0xDFC` | `f32` | copy of the post-load `+0xB0` |
| `+0xE00`,`+0xE02` | `u16` | two `0x80161460` draws |
| `+0xE08` | `u8` | read by the per-frame body `0x80704540`, not here |

### Exact behaviour

```
let flags = read_u32(actor + 4);

write_f32(actor + 0xB4, 1000.0);      // 0x807CC5EC
write_f32(actor + 0xE8, 0.0);         // 0x807CC530
write_f32(actor + 0xEC, 0.0);
write_f32(actor + 0xF0, 0.0);
write_f32(actor + 0xDC, 1.0);         // 0x807CC550
write_f32(actor + 0xE0, 1.0);
write_f32(actor + 0xE4, 1.0);
write_f32(actor + 0x320, -84.0);      // 0x807CC5F0
write_f32(actor + 0x324, 80.0);       // 0x807CC5F4
write_f32(actor + 0x318, 1024.0);     // 0x807CC5F8
write_f32(actor + 0x31C, 1024.0);

0x80043CA0(actor);                    // +0x442..+0x449 = 0xFF
fn_80702750(actor);                   // boat_light_wood.brres + light_fog_st
vmethod0(actor + 0x558, 0x08, actor + 0x53C, 2);

if (flags & 1) != 0 {
    write_f32(actor + 0xDF0, read_f32(actor + 0xAC));
    write_f32(actor + 0xDF4, read_f32(actor + 0xB0));
    write_f32(actor + 0xDF8, read_f32(actor + 0xB4));
    0x800D9130(actor + 0xDBC,
               (flags >> 20) & 0xF,          // spawn id
               actor + 0xDF0,                // position
               actor + 0xE8,                 // velocity
               actor + 0x10C,                // area
               (flags >> 24) & 0xFF,         // angle (255 units)
               (flags >> 16) & 1);           // mirror flag
    write_f32(actor + 0xAC, read_f32(actor + 0xDF0));
    write_f32(actor + 0xB0, read_f32(actor + 0xDF4));
    write_f32(actor + 0xB4, read_f32(actor + 0xDF8));
}

fn_80701CD0(actor + 0xA14);           // palette clear
write_f32(actor + 0xDFC, read_f32(actor + 0xB0));
write_u16(actor + 0xE00, 0x80161460(0x10000) as u16);   // via 0x800B2EE0
write_u16(actor + 0xE02, 0x80161460(0x10000) as u16);

fn_80702AA0(actor);
fn_80702DF0(actor);
fn_80703750(actor, (flags >> 12) & 3);
fn_80703C50(actor);
0x8016A000(actor + 0x478, actor + 0xDB4);

if (flags & 1) == 0 {
    vmethod0(actor + 0x394, 0x18, 0x8082F1E0);
} else {
    vmethod0(actor + 0x394, 0x18, 0x8082F260);
}

fn_80702970(actor);
fn_80702F40(actor);
fn_80702FD0(actor);                   // 516-byte export, no ledger row
fn_807031E0(actor);
fn_80702CE0(actor);

let side = (flags >> 4) & 1;
if read_u32(0x80315B98) == 4
   && read_u8(read_u32(0x8042A1C8) + 0x120C) == 5
   && read_u8(read_u32(0x8042A1C8) + 0x120D) == 5 {
    if side == 0 { return 2; }
} else {
    let word = read_u32(read_u32(0x80429F7C) + 0xAF4);
    if side != (((word >> 31) ^ 1) as u32) { return 2; }
}
return 1;
```

Note the four `rlwinm` masks, re-derived: `rlwinm r4,r31,20,30,31` is
`(flags >> 12) & 3`; `rlwinm r4,r31,12,28,31` is `(flags >> 20) & 0xF`;
`rlwinm r8,r31,8,24,31` is `(flags >> 24) & 0xFF`; `rlwinm r9,r31,16,31,31`
is `(flags >> 16) & 1`. The export prints the same four (`>>0xc`,
`>>0x14`, `>>0x18`, `>>0x10`), so the masks agree.

### Constants

| address | value | use |
|---|---|---|
| `0x807CC530` | `0.0f` | velocity triple |
| `0x807CC550` | `1.0f` | scale triple |
| `0x807CC5EC` | `1000.0f` | initial `+0xB4` |
| `0x807CC5F0` | `-84.0f` | `+0x320` |
| `0x807CC5F4` | `80.0f` | `+0x324` |
| `0x807CC5F8` | `1024.0f` | `+0x318`/`+0x31C` |
| `0x80315B98` | file word, `0` | game-mode word compared with `4` |
| `0x8042A1C8` | BSS cell | world record; bytes `+0x120C`/`+0x120D` compared with `5` |
| `0x80429F7C` | BSS cell | player/system record; `+0xAF4` bit 31 |
| `0x8082F1E0` | BSS object | default state descriptor (built by the class registrar) |
| `0x8082F260` | BSS object | re-placed state descriptor |
| `0x10000` | immediate | argument of both `0x80161460` draws |

### Callees

`0x802DCD5C`/`0x802DCDB0` (`__savegprlr_29`/`__restgprlr_29`), `0x80043CA0`
(byte invalidation of `+0x442..+0x449`, verified), `0x80702750`
(`fn_80702750`), `0x800D9130` (placement builder, verified), `0x80701CD0`
(`fn_80701cd0`), `0x800B2EE0` -> `0x80161460` (scale-record draw, return live),
`0x80702AA0`, `0x80702DF0`, `0x80703750`, `0x80703C50`, `0x8016A000`
(`fn_8016a000`), `0x80702970`, `0x80702F40`, `0x80702FD0` (516-byte export, no
ledger row), `0x807031E0`, `0x80702CE0` (all `fn_*` translated by waves 29/50
unless marked).

### Verdict

**ready (medium).** The actor identification (the `boat_light_wood` light) and
the roles of `+0x318..+0x324` (tilt/scale band) and `+0xE00`/`+0xE02` (two
`0x80161460` draws) are inferred from the neighbouring `0x8070` bodies; every
store, mask, call and gate is literal. Nothing is missing that would change the
emitted code.

---

## 3. `0x80745DC0` - `untei_rope` lift-rope create

**Evidence.** `work/ghidra/rel-d_basesNP/export/80745dc0_FUN_80745dc0.c`,
568 bytes, 142 words, synthetic `0x80745DC0`. Body `0x80745DC0..0x80745FF4`
(one `blr`). Frame `stwu r1,-80(r1)`, `__savegprlr_29`, one `qr0` `f31` save.
The export's "Removing unreachable block (ram,0x80745fd0)" is the
`psq_l f31` in the epilogue.

**What it is.** The create body of the `untei_rope` lift-rope actor. The spawn
record at `0x80810B04` is `{0x80745DC0, 0x80064350, 0x80064380, 0x80746100,
0x80064390, 0x800643E0, 0x80746000, 0x800643F0, 0x80064490, 0x80746070, ...}`
and the same record's strings are `g3d/untei_rope.brres` (`0x80810AC4`),
`untei_rope` (`0x80810ADC`) and `untei_rope_tomegu` (`0x80810AE8`), with the
actor ids `0x00CF`/`0x00D0` at `0x80810ABC`. The body builds the two scroll
layers at `+0x474` and `+0x558`, hands the rope count to `0x80745D10`, and
positions the three `d3d` elements at `+0x3B4`, `+0x3F4` and `+0x434`.

**Signature.**

```rust
/// Creates the `untei_rope` rope/lattice lift.
///
/// # Original
/// - Ghidra: `FUN_80745dc0` at `0x80745DC0` (568 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80745dc0_FUN_80745dc0.c`
///
/// # Arguments
/// * `actor` - the rope actor; flags `+0x004`, position `+0x0AC`, scale
///   `+0x0DC`, tilt `+0x318..+0x324`, matrix scratch `+0x7C`, elements
///   `+0x3B4`/`+0x3F4`/`+0x434`, layers `+0x474`/`+0x558`.
///
/// # Returns
/// `1`.
///
/// # Safety
/// `actor` must be a fresh actor; both layers must be writable records and all
/// three elements must have a live `+4` handle.
pub unsafe fn fn_80745dc0(actor: usize) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | flags: bits 0-5 = rope segment count minus one |
| `+0x07C` | 12 floats | translation matrix scratch |
| `+0x0AC`..`+0x0B4` | vec3 | actor position |
| `+0x0DC`..`+0x0E4` | vec3 | scale, `1.0` |
| `+0x318` | `f32` | `1056.0` |
| `+0x31C` | `f32` | `16.0` |
| `+0x320`,`+0x324` | `f32` | `520.0` / `0.0` |
| `+0x394` | holder | resource holder used by the model loader |
| `+0x3B0` | handle | `g3d/untei_rope.brres` handle (set by `0x80745B30`) |
| `+0x3B4` | element | `untei_rope` element, mode `0x25` |
| `+0x3B8` | record | rope/curve record read by `0x80745C40` |
| `+0x3F4` | element | `untei_rope_tomegu` element (mode `0x20`) |
| `+0x434` | element | `untei_rope_tomegu` element (mode `0x20`) |
| `+0x474` | layer | scroll layer A (`0xE4` bytes) |
| `+0x558` | layer | scroll layer B (`0xE4` bytes) |

### Exact behaviour

```
let flags = read_u32(actor + 4);
let n = (flags & 0x3F) + 1;                 // f31: segment count

write_f32(actor + 0xB4, 400.0);             // 0x807CE37C
write_f32(actor + 0xDC, 1.0);               // 0x807CE380
write_f32(actor + 0xE0, 1.0);
write_f32(actor + 0xE4, 1.0);
write_f32(actor + 0x320, 520.0);            // 0x807CE384
write_f32(actor + 0x324, 0.0);              // 0x807CE388
write_f32(actor + 0x318, 1056.0);           // 0x807CE38C
write_f32(actor + 0x31C, 16.0);             // 0x807CE378

fn_80745B30(actor);                         // BRRES + four elements (address-only)

let s = [n as f32, 1.0, 1.0];               // r1+20..28; 1.0 = pool+8
fn_80745C40(actor, &s);                     // rope curve through +0x3B8 (address-only)

// layer A: fixed 16x16 quad
fn_8007FA40(-8.0, 8.0, 8.0, -8.0,          // 0x807CE390, 0x807CE394
            actor + 0x474, actor,
            0, 0, 0, 3, 0, 0);
fn_8007F900(actor + 0x474);

// layer B: 16x16 quad shifted by 16 per segment
let d = 16.0 * n;                            // 0x807CE378
fn_8007FA40(8.0 + d, 8.0, 24.0 + d, -8.0,   // 0x807CE394, 0x807CE398
            actor + 0x558, actor,
            0, 0, 0, 3, 0, 0);
fn_8007F900(actor + 0x558);

fn_80745D10(actor, n as i32);                // rope configuration (address-only)

// element 1: x + 8n + 8, scale (n, 1, 1)
0x801C0BD0(read_f32(actor+0xAC) + 8.0*n + 8.0,
           read_f32(actor+0xB0), read_f32(actor+0xB4), actor + 0x7C);
0x8016A170(actor + 0x3B4, actor + 0x7C);
let scale1 = [n as f32, 1.0, 1.0];
0x8016A150(actor + 0x3B4, &scale1);
0x8016A1A0(actor + 0x3B4, 0);

// element 2: the actor position, scale +0xDC
0x801C0BD0(read_f32(actor+0xAC), read_f32(actor+0xB0),
           read_f32(actor+0xB4), actor + 0x7C);
0x8016A170(actor + 0x3F4, actor + 0x7C);
0x8016A150(actor + 0x3F4, actor + 0xDC);
0x8016A1A0(actor + 0x3F4, 0);

// element 3: x + 16n + 16, scale +0xDC
0x801C0BD0(read_f32(actor+0xAC) + 16.0*n + 16.0,
           read_f32(actor+0xB0), read_f32(actor+0xB4), actor + 0x7C);
0x8016A170(actor + 0x434, actor + 0x7C);
0x8016A150(actor + 0x434, actor + 0xDC);
0x8016A1A0(actor + 0x434, 0);

return 1;
```

The two `0x8007FA40` rectangles are exactly what the `f1..f4` register contents
give: the first call has `f1 = -8.0` (moved to `f4` as well), `f2 = f3 = 8.0`;
the second has `f1 = 8 + 16n`, `f2 = 8.0`, `f3 = 24 + 16n`, `f4 = -8.0`. Both
pass `mask = 3`, `group = 0`, `scale = 0`.

### Constants

| address | value | use |
|---|---|---|
| `0x807CE378` | `16.0f` | per-segment length, layer-B offset |
| `0x807CE37C` | `400.0f` | `+0xB4` |
| `0x807CE380` | `1.0f` | scale |
| `0x807CE384` | `520.0f` | `+0x320` |
| `0x807CE388` | `0.0f` | `+0x324` |
| `0x807CE38C` | `1056.0f` | `+0x318` |
| `0x807CE390` | `-8.0f` | rectangle edge |
| `0x807CE394` | `8.0f` | rectangle edge / y base |
| `0x807CE398` | `24.0f` | layer-B far edge |
| `0x807CE39C` | `0.0f` | unused by this body |
| `0x807CE3A0` | `2^52` (f64) | int -> float bias |
| `3` | immediate | layer mask |
| `0` | immediate | layer group / scale / element mode |

### Callees

`0x802DCD5C`/`0x802DCDB0` (save/restore), `0x80745B30` (address-only: opens
`g3d/untei_rope.brres` through `0x800DF180` into `+0x3B0`, binds `untei_rope`
into `+0x3B4` with mode `0x25`, then `untei_rope_tomegu` into `+0x3F4` and
`+0x434` with mode `0x20`), `0x80745C40` (address-only: builds the rope curve
through `+0x3B8` from the three-word argument with `0x801C0C60`/`0x8023CFA0`),
`0x80745D10` (address-only: for `count` segments from `+0xAC` step `16.0`,
`0x80077860(0x80429DD0, (int)x & 0xFFFF, (int)-y & 0xFFFF, 0, 0x93)`),
`0x8007FA40` (`fn_8007fa40`), `0x8007F900` (`fn_8007f900`), `0x801C0BD0`
(`fn_801c0bd0`), `0x8016A170`/`0x8016A150`/`0x8016A1A0` (`fn_8016a170`,
`fn_8016a150`, `fn_8016a1a0`).

### Verdict

**ready (high).** Every constant, both rectangles, all three element transforms
and the layer linkage are literal. `0x80745B30`/`0x80745C40`/`0x80745D10` are
nearby address-only siblings of the same class; their argument shape is fixed by
this call site (`r3 = actor`; `r4` = the three-word vector for `0x80745C40` and
the segment count for `0x80745D10`).

---

## 4. `0x8077D340` - world-map walker frame driver

**Evidence.** `work/ghidra/rel-d_basesNP/export/8077d340_FUN_8077d340.c`,
568 bytes, 142 words, synthetic `0x8077D340`. Body `0x8077D340..0x8077D574`
(one `blr`). Frame `stwu r1,-80(r1)`, `__savegprlr_29`; no `psq`, no jump
table. Two `bl` calls to `0x800FC040` (the side selector) and two to
`0x800F69F0` (placement spawn) in distinct arms.

**What it is.** The per-frame driver of the world-map walker actor that
`0x8079E1E0` runs for every child of its `+0x184` list with `+0x124 != 0`
(`0x8079E1E0` calls it at `0x8079E218`). Its siblings in the `0x8077B450`
update cluster (`fn_8077d230`, `fn_8077d7b0`, `fn_8077e570`) are already
translated and describe the same object as a "player-model actor" whose
`+0x1CC` view holds the model at `+4`. This body is the `+0x2C0` countdown state
machine: while the counter is above 1 it only decrements; at 1 it plays the
"move" effects and decrements to 0; at 0 it either performs the map-node
transition (`0x8079EE70` + `0x800F6410` + `fn_8077e570`) or, when the actor is
below its `+0xBC` floor, plays animation 5.

**Signature.**

```rust
/// Advances the world-map walker: countdown, move effects and node transition.
///
/// # Original
/// - Ghidra: `FUN_8077d340` at `0x8077D340` (568 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/8077d340_FUN_8077d340.c`
///
/// # Arguments
/// * `actor` - the walker; position `+0x0AC`, floor `+0xBC`, countdown
///   `+0x2C0`, audio/visibility byte `+0x124`, view `+0x1CC`, model record
///   `+0x294`, state id `+0x1F0`, node handle `+0x1F4`, copied words
///   `+0x1E6`/`+0x200`, listener `+0x1D8`.
///
/// # Returns
/// `0` for the "nothing settled" arms and the `0x800F6AB0` result in the
/// transition arm (non-zero when the actor is settled).
///
/// # Safety
/// `actor` must be live; `+0x1CC` must point at a view whose `+4` is the model
/// and `+0x294` at a readable node record.
pub unsafe fn fn_8077d340(actor: usize) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x0AC`..`+0x0B4` | vec3 | position |
| `+0x0B0` | `f32` | y, compared with `+0xBC` |
| `+0x0BC` | `f32` | floor height |
| `+0x124` | `u8` | "emit the move effect" flag |
| `+0x1CC` | object | view; `+4` = model (`+0x151`/`+0x152` = character/costume) |
| `+0x1D8` | vec3 | listener/effect anchor |
| `+0x1E6` | `u16` | copy of `[0x8042A1A0]+0x102` |
| `+0x1F0` | `u32` | state id (`0xA9` = the "no transition" id) |
| `+0x1F4` | `u32` | effect/node handle handed to `0x8079EE70` |
| `+0x200` | `u32` | copy of `[0x8042A1A0]+0x240` |
| `+0x294` | record | node record: `+0x170..+0x190` positions, `+0x18` scale, `+0x174`/`+0x188` angles |
| `+0x2C0` | `s32` | countdown / state word |

### Exact behaviour

```
let mut result = 0;                       // r31
let counter = read_i32(actor + 0x2C0);

if counter > 1 {
    write_i32(actor + 0x2C0, counter - 1);
    return result;                        // 0
}

if counter == 1 {
    if read_u8(actor + 0x124) != 0 {
        let view  = read_u32(actor + 0x1CC);
        let model = read_u32(view + 4);
        wm_effect_record_play(0x8042A27C, 8,
                              read_u8(model + 0x151) as i32,
                              read_u8(model + 0x152) as u32);
    }
    let mode = read_i32(read_u32(0x8042A1A0) + 0x27C);
    if mode == 6 {
        let rec = read_u32(actor + 0x294);
        0x800F69F0(read_f32(rec + 0x170),
                   read_f32(rec + 0x178) * read_f32(rec + 0x18),
                   read_f32(rec + 0x17C) * read_f32(rec + 0x18),
                   actor, actor + 0x1D8, 0,
                   read_i16(rec + 0x174) as i32,
                   0x80377DE0);
    } else if mode == 7 {
        let side = 0x800FC040();
        let v: [f32; 3] = if side == 0 {
            [0.0, 0.0, -1.0]              // 0x807CF698, 0x807CF698, 0x807CF6C0
        } else {
            [1.0, 0.0, 0.0]               // 0x807CF69C, 0x807CF698, 0x807CF698
        };
        let rec = read_u32(actor + 0x294);
        0x800F69F0(read_f32(rec + 0x184),
                   read_f32(rec + 0x18C) * read_f32(rec + 0x18),
                   read_f32(rec + 0x190) * read_f32(rec + 0x18),
                   actor, actor + 0x1D8, 0,
                   read_i16(rec + 0x188) as i32,
                   &v);
    }
    write_i32(actor + 0x2C0, read_i32(actor + 0x2C0) - 1);
    return result;                        // 0
}

// counter <= 0
result = 0x800F6AB0(actor);
if result != 0 && read_u32(actor + 0x1F0) != 0xA9 {
    let view  = read_u32(actor + 0x1CC);
    let model = read_u32(view + 4);
    0x8079EE70(read_u32(actor + 0x1F4), actor + 0xAC,
               read_u8(model + 0x151) as u32,
               read_u8(model + 0x152) as u32);
    write_u16(actor + 0x1E6, read_i16(read_u32(0x8042A1A0) + 0x102) as u16);
    write_u32(actor + 0x200, read_u32(read_u32(0x8042A1A0) + 0x240));
    let side = 0x800FC040();
    let v: [f32; 3] = if side == 0 { [0.0, 0.0, -1.0] } else { [1.0, 0.0, 0.0] };
    0x800F6410(actor, &v);
    fn_8077e570(actor);                   // animation re-arm
} else if read_f32(actor + 0xB0) < read_f32(actor + 0xBC) {
    0x8077E410(0.0, actor, 5, 0xFFFF_FFFF);   // fall/respawn animation
}
return result;
```

The two vectors are stored as three `stfs` each into the stack block and passed
as `r7`/`r4`. The `-1.0` is `0x807CF6C0`; the branch order (side `0` first) is
the `beq` at `0x8077D414`.

### Constants

| address | value | use |
|---|---|---|
| `0x807CF698` | `0.0f` | vector components, `f1` of the fall animation |
| `0x807CF69C` | `1.0f` | vector x |
| `0x807CF6C0` | `-1.0f` | vector z |
| `0x80377DE0` | BSS vec3 | extra placement vector (zero at file level) |
| `0xA9` | immediate | "no transition" state id |
| `5` | immediate | fall animation slot |
| `0xFFFFFFFF` | immediate | "keep the record's animation id" |
| `8` | immediate | effect id of the move sound |
| `6`, `7` | immediates | `[0x8042A1A0]+0x27C` modes 6 and 7 |
| `0x8042A1A0` | BSS cell | world-map master object (`+0x102`, `+0x240`, `+0x27C`) |
| `0x8042A27C` | BSS cell | effect pool base for `wm_effect_record_play` |

### Callees

`0x802DCD5C`/`0x802DCDB0` (save/restore), `0x80104FD0`
(`wm_effect_record_play`), `0x800F69F0` (placement spawn, verified),
`0x800FC040` (side selector, verified), `0x8079EE70` (node effect: `0x80103390`
+ `0x80105060`, verified), `0x800F6410` (placement, verified), `0x8077E570`
(`fn_8077e570`), `0x8077E410` (animation selector; the body reads
`+0x294 + slot*0xC`, `+0x1F0`, `+0x1CC` and calls the model's `+0x5C`/`+0x84`
slots), `0x800F6AB0` (scale ramp, verified).

### Verdict

**ready (medium).** Wave 100 deferred this body until `0x8079EE70` and
`0x800F6410` had confirmed prototypes; both are confirmed above from
`main.dol`. The state name of `+0x2C0` (a delay counter shared with
`fn_8077d7b0`'s respawn sub-machine) is the only inferred role; the emitted
code does not depend on it.

---

## 5. `0x80788090` - World-7 Koopa model/part binder

**Evidence.** `work/ghidra/rel-d_basesNP/export/80788090_FUN_80788090.c`,
568 bytes, 142 words, synthetic `0x80788090`. Body `0x80788090..0x807882C4`
(one `blr`). Frame `stwu r1,-320(r1)`, `__savegprlr_24` (`0x802DCD68`), a
256-byte stack path buffer, no `psq`. The gap after the extent holds two zero
words and a one-word `blr` stub at `0x807882D0` (a no-op virtual slot), which is
**not** part of this body.

**What it is.** The model/binding pass of the World-7 Koopa actor, called by
`fn_80787bd0` at its second instruction (`abi_call1!(0x8078_8090, this)` in
`src/game/bases.rs:197885`) and described by
`docs/SPEC_WAVE153_BASES.md:520-534`. It builds `g3d/<name>.brres` from
`+0x43C`, opens it, binds the model holder `+0x1A4`, then walks the **six**
0x38-byte part slots at `+0x1E4` (names from the `+0x438` array, five entries
plus a repeat of entry 0 in the sixth), sets each part's byte at
`+0x218 + i*0x38` from the table at `0x807CFA38`, re-arms each part at rate
`1.0`, and finishes with `fn_80789b80`/`fn_80789c60`, the `+0x39C` animation
wrapper and the state hand-off. The nine records that share
`0x80787BD0` - `iggy`, `larry`, `lemmy`, `ludwig`, `morton`, `roy`, `wendy`
plus `wand_model`/`broom` and `jump_ed` at
`0x808192FC..0x8081A06C` - all use this binder.

**Signature.**

```rust
/// Loads the Koopa's BRRES and binds its six 0x38-byte animation parts.
///
/// # Original
/// - Ghidra: `FUN_80788090` at `0x80788090` (568 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80788090_FUN_80788090.c`
///
/// # Arguments
/// * `actor` - the World-7 Koopa actor; resource holder `+0x184`, BRRES
///   `+0x1A0`, model holder `+0x1A4`, six parts `+0x1E4`, part bytes
///   `+0x218`, sixth part `+0x2FC`, wrapper `+0x39C`, name `+0x43C`, name
///   array `+0x438`, class vtable `+0x60`, state interface `+0x394`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `actor` must be a fresh actor whose `+0x43C` points at a NUL-terminated
/// name and whose `+0x438` array holds six readable name pointers; the NW4R
/// resource manager cell `0x8042A038` must be live.
pub unsafe fn fn_80788090(actor: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x060` | vtable | class vtable; `+0x7C` slot called with `this` |
| `+0x184` | holder | 0x20-byte resource holder (`0x800690C0`/`0x800690E0`) |
| `+0x1A0` | handle | BRRES handle |
| `+0x1A4` | holder | model/animation holder (`0x80169D90`, `0x8016A000`) |
| `+0x1E4 + i*0x38` | part | six part records, `i = 0..5` |
| `+0x218 + i*0x38` | `u8` | part flag (`0,0,1,1,1,1`) |
| `+0x2FC` | part | sixth part (= `+0x1E4 + 5*0x38`) |
| `+0x330` | `u8` | sixth part flag, `1` |
| `+0x39C` | wrapper | 8-byte animation wrapper (`0x80165520` mode 8) |
| `+0x404` | word | stored at `+0x1A4 + 0x38` by `0x8016A000` |
| `+0x438` | `[u32; 6]` | part-name pointer array |
| `+0x43C` | `*char` | model name (e.g. `iggy`) |

### Exact behaviour

```
0x800690C0(actor + 0x184, 0xFFFF_FFFF, read_u32(0x80377C48), 0, 0x20);

let name = read_u32(actor + 0x43C) as *const u8;      // r25
let suffix_len = strlen(0x80819208);                  // "g3d/.brres"
let name_len   = strlen(name);
let mut path = [0u8; 0x100];                          // r1+0x20
snprintf(path.as_mut_ptr(), name_len + suffix_len + 1,
         0x80819214 /* "g3d/%s.brres" */, name);

let brres = 0x800DF180(read_u32(0x8042A038) + 4, name, path.as_ptr());
write_u32(actor + 0x1A0, brres);
let model = 0x80239E30(actor + 0x1A0, name);
let mut handle = [model];
0x80169D90(actor + 0x1A4, &handle, actor + 0x184, 0x20, 1, 0);

// five named parts
for i in 0..5 {
    let anim = 0x8023A0B0(actor + 0x1A0,
                          read_u32(read_u32(actor + 0x438) + i*4));
    if anim != 0 {
        let mut h = model;
        let mut a = [anim];
        0x801650D0(actor + 0x1E4 + i*0x38, &h, &a, actor + 0x184, 0);
        write_u8(actor + 0x218 + i*0x38, read_u8(0x807CFA38 + i*4));
        0x801680E0(1.0, actor + 0x1E4 + i*0x38);      // f1 = 0x807CFA50
    }
}

// sixth part repeats name 0
let anim5 = 0x8023A0B0(actor + 0x1A0, read_u32(read_u32(actor + 0x438)));
if anim5 != 0 {
    let mut h = model;
    let mut a = [anim5];
    0x801650D0(actor + 0x2FC, &h, &a, actor + 0x184, 0);
    write_u8(actor + 0x330, 1);
    0x801680E0(1.0, actor + 0x2FC);
}

let mut h = model;
0x80165520(actor + 0x39C, &h, 8, actor + 0x184, 0);

fn_80789b80(1.0, actor, 3);
fn_80789b80(1.0, actor, 0);
fn_80789c60(actor);
0x8016A000(actor + 0x1A4, actor + 0x404);
vmethod0(actor, 0x7C);                     // via [actor+0x60]+0x7C, r3 = this
vmethod0(actor + 0x1A4, 0x18, actor + 0x1A4, actor + 0x39C);   // push the wrapper
0x800F2A40(actor + 0x1A4);                 // -> 0x800B4050(handle, 0)
0x800690E0(actor + 0x184);
```

The flag table `0x807CFA38` holds five words `{0, 0, 1, 1, 1}`; the body loads
a **word** and stores its low byte with `stb`, so the five part bytes are
`0, 0, 1, 1, 1` and the sixth is `1`.

### Constants

| address | value | use |
|---|---|---|
| `0x80819208` | `"g3d/.brres"` | `strlen` source for the path size |
| `0x80819214` | `"g3d/%s.brres"` | `snprintf` format |
| `0x807CFA38` | `{0,0,1,1,1}` | part bytes (low byte of each word) |
| `0x807CFA50` | `1.0f` | `f1` of every `0x801680E0` |
| `0x80377C48` | BSS word | default resource name (zero) |
| `0x8042A038` | BSS cell | NW4R resource manager (`+4` = the BRRES root) |
| `0x20` | immediate | holder size, `0x80169D90` mode |
| `8` | immediate | `0x80165520` mode |
| `0x100` | stack | path buffer (`r1+0x20`, 256 bytes) |

### Callees

`0x802DCD68`/`0x802DCDB4` (save/restore), `0x800690C0`/`0x800690E0` (holder
create/release, verified), `0x802DC69C` (`strlen`), `0x802E16E8`
(`snprintf`), `0x800DF180` (`fn_800df180`), `0x80239E30` (`fn_80239e30`),
`0x80169D90` (`fn_80169d90`), `0x8023A0B0` (`brres_get_anm_chr`),
`0x801650D0` (`fn_801650d0`), `0x801680E0` (`fn_801680e0`, value in `f1`),
`0x80165520` (`fn_80165520`), `0x80789B80` (`fn_80789b80`), `0x80789C60`
(`fn_80789c60`), `0x8016A000` (`fn_8016a000`), `0x800F2A40`
(`0x800B4050(handle, 0)`).

### Verdict

**ready (medium).** The only inferred parts are the *roles* of the six part
slots and of `+0x218` (a per-part visibility/animation flag): the 0x38 stride,
the `+0x438` name array, the flag table and every call are literal, and
`fn_80789b80`/`fn_80789c60` (already translated) use the same `+0x1E4`/`+0x438`
layout, which fixes the stride independently.

---

## 6. `0x8079B7C0` - XZ cosine of the node angle

**Evidence.** `work/ghidra/rel-d_basesNP/export/8079b7c0_FUN_8079b7c0.c`,
568 bytes, 142 words, synthetic `0x8079B7C0`. Body `0x8079B7C0..0x8079B9F4`
(one `blr`). Frame `stwu r1,-160(r1)`, `__savegprlr_29`, `f31`/`f30` saves and
**twelve data-side `qr0` words** (`psq_l`/`psq_st`/`ps_sub` on 12-byte vectors).
Ghidra's prototype says `double FUN_8079b7c0(int)`; the body ends in `fdivs`
and returns `f1`, so the real result is a single-precision `f32` - the `double`
is the export's guess from the `f31` restore.

**What it is.** The straightness metric used by the world-map node selector
`fn_80799340` (two call sites, `0x807993F0` and `0x807994FC`). It returns the
cosine of the angle between the **XZ-plane** segment
`pos(0x80832EB0) -> pos(+0x22C)` and the segment
`pos(+0x22C) -> pos(+0x230)`: `1.0` means the two segments are collinear,
`0.0` means degenerate. `docs/SPEC_WAVE154_BASES.md:522-531` decoded it in
passing as "the cosine of the angle between the node `0x80832EB0` and the node
`+0x22C`"; this section pins every offset, stack slot and early-out.

**Signature.**

```rust
/// Returns the XZ cosine between the published node, this node and the next.
///
/// # Original
/// - Ghidra: `FUN_8079b7c0` at `0x8079B7C0` (568 bytes, prints `double`)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/8079b7c0_FUN_8079b7c0.c`
///
/// # Arguments
/// * `owner` - the map object: current node id `+0x22C`, next node id `+0x230`.
///
/// # Returns
/// `dot(c, f) / (|c| * |f|)` as `f32`, or `0.0` when either node id is the
/// published id `0x80832EB0`, when the current node is the next node, or when
/// either segment is the zero vector.
///
/// # Safety
/// `owner` must be readable at `+0x22C`/`+0x230` and the scene manager cell
/// `0x8042A18C` must be live (both node ids must resolve).
pub unsafe fn fn_8079b7c0(owner: usize) -> f32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x22C` | `u32` | current node id |
| `+0x230` | `u32` | next node id |
| `0x80832EB0` | BSS word | published node id (REL BSS, zero at file level) |
| `0x8042A18C` | BSS cell | scene manager |

Stack layout (all 12-byte vectors):

| slot | contents |
|---|---|
| `r1+0x2C` | `A = pos(0x80832EB0)` |
| `r1+0x38` | `B = pos(+0x22C)` |
| `r1+0x44` | `C = B - A` (12 bytes; the y lane is written by `ps_sub`) |
| `r1+0x5C` | `C` copied for the length call, with `C.y` forced to `0.0` |
| `r1+0x08` | `D = pos(+0x22C)` |
| `r1+0x14` | `E = pos(+0x230)` |
| `r1+0x20` | `F = E - D` |
| `r1+0x50` | `F` copied for the length call, `F.y = 0.0` |

### Exact behaviour

```
let node = read_u32(owner + 0x22C);
if node == read_u32(0x80832EB0) || node == read_u32(owner + 0x230) {
    return read_f32(0x807D0160);                 // 0.0
}

let mgr = read_u32(0x8042A18C);
let mut a = [0.0f32; 3];  0x80100200(&mut a, mgr, read_u32(0x80832EB0));
let mut b = [0.0f32; 3];  0x80100200(&mut b, mgr, node);

// ps_sub on the two 12-byte blocks, then psq_st of the pair and the z lane
let mut c = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];

let mut d = [0.0f32; 3];  0x80100200(&mut d, mgr, node);
let mut e = [0.0f32; 3];  0x80100200(&mut e, mgr, read_u32(owner + 0x230));
let mut f = [e[0] - d[0], e[1] - d[1], e[2] - d[2]];

// ground-plane projection: the y lanes are overwritten with 0.0
c[1] = 0.0;
f[1] = 0.0;

if (c[0] == 0.0 && c[1] == 0.0 && c[2] == 0.0)
   || (f[0] == 0.0 && f[1] == 0.0 && f[2] == 0.0) {
    return read_f32(0x807D0160);                 // 0.0
}

let lc  = 0x801C14C0(&c);                        // |C|
let dot = c[0]*f[0] + c[1]*f[1] + c[2]*f[2];
let lf  = 0x801C14C0(&f);                        // |F|
return (dot / lc) / lf;
```

The zero comparisons are against the `main.dol` BSS triple
`0x80377DC8`/`0x80377DCC`/`0x80377DD0`, all zero at file level; both "is zero"
tests are three separate `fcmpu`/`bne` pairs (`0x8079B900..0x8079B924` and
`0x8079B930..0x8079B970`).

### Constants

| address | value | use |
|---|---|---|
| `0x807D0160` | `0.0f` | degenerate return |
| `0x80377DC8`,`0x80377DCC`,`0x80377DD0` | BSS zeros | origin triple compared component-wise |
| `0x80832EB0` | BSS word | published node id |
| `0x8042A18C` | BSS cell | scene manager |
| `0x807D00C4` | `1.0f` | *caller's* cosine reference (not used in the body) |
| `0x807D01C4` | `0.05f` | *caller's* dead band (not used in the body) |

### Callees

`0x802DCD5C`/`0x802DCDB0` (save/restore), `0x80100200` (scene node position,
verified), `0x801C14C0` (`fn_801c14c0`, vector length).

### Verdict

**ready (high).** Every load, both subtractions, both zero projections, both
early-outs, the dot product and both divisions are literal. The row is the only
one in this set that uses data-side `qr0`, and all of it is the plain `f32`
pair form (`docs/ANALYSIS_WAVE127_BASES.md:50-59`).

---

## 7. `0x806DF820` - `lift_spin_kinoko` player-occupancy handler

**Evidence.** `work/ghidra/rel-d_basesNP/export/806df820_FUN_806df820.c`,
564 bytes, 141 words, synthetic `0x806DF820`. Body `0x806DF820..0x806DFA50`
(one `blr`). Frame `stwu r1,-64(r1)`, `__savegprlr_24` (`0x802DCD64`), no
`psq`, no jump table. Only data reference: `0x807FC50C`, the first record of the
12-byte `{handler, 0, -1}` table that follows the `lift_spin_kinoko` vtable
`0x807FC428` (`docs/COVERAGE_WAVE122_BASES.md:19-21`: "the `lift_spin_kinoko`
actor at `0x806DEC80..0x806DFF70`"). The sibling handler `0x806DFCB0` starts at
`0x807FC518`.

**What it is.** A per-frame handler of the `lift_spin_kinoko` lift actor (the
spring mushroom lift): it scans the actor's child list at `+0x484` for live
player actors, records for each of the four player slots whether a player is
standing on the lift (`1`) or has just left (`2`), plays the "step on/off"
cue, and folds the four per-player state words `+0x678..+0x684` forward. It is
the producer of the four `+0x698..+0x6A4` presence words that the translated
`fn_806df770` consumes ("Ends the player-switch for every presence word that is
set", `src/game/bases.rs:150795-150837`).

**Signature.**

```rust
/// Tracks which of the four players stand on the lift and steps their states.
///
/// # Original
/// - Ghidra: `FUN_806df820` at `0x806DF820` (564 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/806df820_FUN_806df820.c`
///
/// # Arguments
/// * `actor` - the lift; position `+0x0AC`, child list `+0x484`, per-player
///   state `+0x678`, counters `+0x688`, presence `+0x698`, mode `+0x65C`,
///   value `+0x6AA`, x offset `+0x6EC`.
///
/// # Returns
/// The mode: `0` when no player was seen, `1` when a player is present,
/// `2` when at least one player carries event flag `0x2B`.
///
/// # Safety
/// `actor` must be live, the `+0x484` list must be a valid `{+4 actor, +0xC
/// next}` chain, and every child must be a player actor with a readable vtable.
pub unsafe fn fn_806df820(actor: usize) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x0AC` | `f32` | actor x |
| `+0x484` | list | child list head; `+4` = actor, `+0xC` = next |
| `+0x65C` | `u32` | output mode (0/1/2) |
| `+0x678 + i*4` | `u32` | per-player state, written back at the end |
| `+0x688 + i*4` | `u32` | per-player "leaving" counter, clamped to 2 |
| `+0x698 + i*4` | `u32` | per-player presence word (0/1) |
| `+0x6AA` | `s16` | largest-magnitude player value `+0x2520` seen |
| `+0x6EC` | `f32` | x offset added before the per-player float store |
| child `+0x38C` | `u8` | must be `1` |
| child `+0x2520` | `s16` | per-player value compared by magnitude |

### Exact behaviour

```
let mut flags = [0u32; 4];               // r1+8..+0x14
let mut mode  = 0u32;                    // r29
let mut best  = 0i32;                    // r28 (runs over |value|)
let mut child = read_u32(actor + 0x484);

while child != 0 {
    let subject = read_u32(child + 4);
    if read_u8(subject + 0x38C) == 1
       && subject != 0
       && 0x8013AAA0(subject) == 0
       && 0x80056CF0(subject, 0x32) == 0
       && vmethod0(subject, 0xE0) == 0 {
        let tag = (*(vcall(subject, 0x6C) as *const i8) as i32) * 4;   // extsb, <<2
        flags[tag / 4] = 1;
        0x80128E60(read_f32(actor + 0xAC) + read_f32(actor + 0x6EC), subject);
        if 0x80056CF0(subject, 0x2B) != 0 {
            let value = read_i16(subject + 0x2520) as i32;
            if abs(best) <= abs(value) { best = value; }   // 0x802DE514
            mode = 2;
            flags[tag / 4] = 2;
        }
        if mode == 0 { mode = 1; }
    }
    child = read_u32(child + 0xC);
}

for i in 0..4 {
    let state = read_u32(actor + 0x678 + i*4);
    if state != 2 && flags[i] == 2 {
        let c = read_u32(actor + 0x688 + i*4) + 1;
        write_u32(actor + 0x688 + i*4, c);
        if c > 1 {
            write_u32(actor + 0x688 + i*4, 2);
            write_u32(actor + 0x698 + i*4, 0);
            0x800B3720(i as u32, 9, 0);
        }
    }
    if state == 0 && flags[i] >= 1 && flags[i] <= 2 {
        write_u32(actor + 0x698 + i*4, 1);
    }
    if (state == 1 || state == 2) && flags[i] == 0 {
        write_u32(actor + 0x688 + i*4, 0);
        write_u32(actor + 0x698 + i*4, 0);
        0x800B3750(i as u32, 9, 60);
    }
    if flags[i] == 0 {
        write_u32(actor + 0x698 + i*4, 0);
    }
    write_u32(actor + 0x678 + i*4, flags[i]);
}

fn_806df770(actor);
write_i16(actor + 0x6AA, best as i16);
write_u32(actor + 0x65C, mode);
return mode;
```

The `tag` index: `lwz r0,0(r3)` → `lbz`-style byte load, `extsb`, `rlwinm
r25,r0,2,0,29` = `value * 4`, then `stwx r24,r27,r25` with `r27 = r1+8`. So the
array is indexed by the *signed byte* returned by the `+0x6C` vcall, times 4 -
i.e. a 4-entry `u32` array, which is why the first player is index 0..3 and no
bounds check exists.

### Constants

| address | value | use |
|---|---|---|
| `0x8042A320` | BSS table | per-player record (`+0x23C` stride) used by `fn_806df770`, not here |
| `0x32` | immediate | event flag "on the lift" |
| `0x2B` | immediate | event flag "just left" |
| `0x9` | immediate | sound id of both cues |
| `0` / `60` | immediates | `0x800B3720`/`0x800B3750` extra argument |
| `2` | immediate | "leaving" clamp |
| `4` | immediate | player loop bound |

### Callees

`0x802DCD64`/`0x802DCDB0` (save/restore), `0x8013AAA0` (`fn_8013aaa0`),
`0x80056CF0` (`fn_80056cf0`), `0x80128E60` (event flag `0x2C` + float store,
verified), `0x802DE514` (`abs`, verified), `0x800B3720`/`0x800B3750`
(`fn_800b3720`/`fn_800b3750`), `0x806DF770` (`fn_806df770`).

### Verdict

**ready (high).** The child walk, the five child gates, the signed-byte array
index, the magnitude comparison, the four-slot state machine and the final two
stores are literal. The only inferred item is the handler's `StateID_*` name:
the record table at `0x807FC50C` makes it the **first** handler of the
`lift_spin_kinoko` block, while the `StateID_*` strings immediately before the
vtable (`0x807FC210..0x807FC36C`, `daLiftSlideKinoko_c::StateID_StretchMove` ...
`StretchReadyWait`) belong to the neighbouring data block. The body's "who is on
the lift" step is the per-frame half of the stretch/shrink cycle. The emitted
code does not depend on the name.

---

## 8. `0x80628F70` - nine-row board screen handler

**Evidence.** `work/ghidra/rel-d_basesNP/export/80628f70_FUN_80628f70.c`,
560 bytes, 140 words, synthetic `0x80628F70`. Body `0x80628F70..0x8062919C`
(one `blr`), **contiguous** with the next TSV row `0x806291A0` (the translated
`fn_806291a0`). Frame `stwu r1,-16(r1)`, `__savegprlr_30`. No `psq`, no jump
table. The six state pushes all go through `+0x74` slot `+0x18` with a
descriptor whose address is `0x80825A78 + offset`.

**What it is.** A screen/board handler of the `+0xA80` nine-row board family
(`docs/COVERAGE_WAVE130_BASES.md:48-56` and `src/game/bases.rs:163976`), whose
neighbours `fn_80628770` (row-major index), `fn_80628740` (cell finish),
`fn_80628940` (row wrap/occupancy), `fn_80629460`, `fn_80629770` and
`fn_806291a0` are already translated. This body reads the current input nibble,
moves the column cursor on "left"/"right" (`4`/`8`), otherwise re-evaluates the
selected row through `0x80628940`, and either plays the move cue, opens the
"cell" message, or pushes one of four state descriptors (`0x80825E60`,
`0x80825EE0`, `0x80825F20`, `0x80825BA0`).

**Signature.**

```rust
/// Steps the nine-row board cursor and pushes the next board state.
///
/// # Original
/// - Ghidra: `FUN_80628f70` at `0x80628F70` (560 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80628f70_FUN_80628f70.c`
///
/// # Arguments
/// * `board` - the board screen; state interface `+0x74`, cursor `+0xA88`,
///   selected row `+0xA90`, highlighted row `+0xA8C`, input word `+0xAB8`,
///   pending `+0xA78`, column count `+0xAB0`, cell records `+0x1D0`/`+0x1D4`
///   (stride `0x20`), owner `+0xB0`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `board` must be live, `+0x74` must expose a callable `+0x18` slot, and the
/// `main.dol` input/audio cells `0x80429F50`, `0x8042A468`, `0x8042A488` must
/// be live.
pub unsafe fn fn_80628f70(board: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x074` | interface | state interface; vtable `+0x18` = push descriptor |
| `+0x0B0` | object | owner record; `+0x288` = a count compared with a cell |
| `+0x0A78` | `u32` | pending direction (`1` = up, `2` = down) |
| `+0x0A88` | `s32` | column selector (`*5` in `0x80628770`) |
| `+0x0A8C` | `s32` | target column |
| `+0x0A90` | `s32` | previously selected row (input of `0x80628940`) |
| `+0x0AB0` | `s32` | column limit |
| `+0x0AB8` | `u32` | last input nibble |
| `+0x1D0 + sel*0x20` | `s32` | cell threshold |
| `+0x1D4 + sel*0x20` | `s32` | cell kind (`1`, `2`, other) |

### Exact behaviour

```
let input = read_u32(read_u32(0x80429F50) + 4);
let bits  = read_u32(input + 0x1C) & 0xF;      // r5 keeps this value
write_u32(board + 0xAB8, bits);

if bits == 4 && read_i32(board + 0xA88) < read_i32(board + 0xAB0) - 1 {
    write_i32(board + 0xA8C, read_i32(board + 0xA88) + 1);
    write_u32(board + 0xA78, 1);
    vmethod0(board + 0x74, 0x18, board + 0x74, 0x80825E60);
    return;
}

if bits == 8 && read_i32(board + 0xA88) > 0 {
    write_i32(board + 0xA8C, read_i32(board + 0xA88) - 1);
    write_u32(board + 0xA78, 2);
    vmethod0(board + 0x74, 0x18, board + 0x74, 0x80825E60);
    return;
}

// r5 is still `bits` here - the third argument is the input nibble
let row = 0x80628940(board, read_i32(board + 0xA90), bits as i32);

if row != read_i32(board + 0xA90) {
    write_i32(board + 0xA90, row);
    0x80195380(read_u32(0x8042A488), 0x78, 1);
    vmethod0(board + 0x74, 0x18, board + 0x74, 0x80825EE0);
    return;
}

let window = read_u32(0x8042A468);
if vmethod0(window, 0x20, 0x900) == 0 {
    if 0x800B53A0(0) != 0 {
        vmethod0(board + 0x74, 0x18, board + 0x74, 0x80825F20);
    }
    return;
}

let cell = board + 0x80628770(board) * 0x20;   // row-major index
let kind = read_i32(cell + 0x1D4);
if kind == 1 {
    0x80195380(read_u32(0x8042A488), 0x7B, 1);
} else if kind == 2 {
    if read_i32(read_u32(board + 0xB0) + 0x288) < read_i32(cell + 0x1D0) {
        0x80195380(read_u32(0x8042A488), 0x7B, 1);
    } else {
        vmethod0(board + 0x74, 0x18, board + 0x74, 0x80825BA0);
        0x80195380(read_u32(0x8042A488), 0x79, 1);
    }
} else {
    vmethod0(board + 0x74, 0x18, board + 0x74, 0x80825BA0);
    0x80195380(read_u32(0x8042A488), 0x79, 1);
}
```

**Register-level finding.** The export prints `FUN_80628940(param_1,
*(int *)(param_1 + 0xa90))` with **two** arguments, but `0x80628940` reads its
`kind` from `r5` (`0x80628940` starts `cmplwi cr0,r5,2`). At the call site
`0x80629030` nothing writes `r5`, and `r5` still holds the input nibble loaded
at `0x80628FA0` (`rlwinm r5,r0,0,28,31`); every path that reaches the call goes
through that load. The third argument is therefore `bits`, and `1`/`2` inputs
take the row-adjust arms of `0x80628940`. A translation that omits it would
change behaviour for nibbles 1 and 2.

### Constants

| address | value | use |
|---|---|---|
| `0x80825A78` | BSS base | descriptor base (`+0x3E8`, `+0x468`, `+0x4A8`, `+0x128`) |
| `0x80825E60` | BSS object | `+0x3E8`: "cursor moved" descriptor |
| `0x80825EE0` | BSS object | `+0x468`: "row changed" descriptor |
| `0x80825F20` | BSS object | `+0x4A8`: "accepted" descriptor |
| `0x80825BA0` | BSS object | `+0x128`: "cell opened" descriptor |
| `0x80429F50` | BSS cell | input manager (`+4` = current pad record, `+0x1C` = nibble) |
| `0x8042A468` | BSS cell | window/message object; `+0x20` slot queried with `0x900` |
| `0x8042A488` | BSS cell | sound manager |
| `0x78` / `0x79` / `0x7B` | immediates | sound ids 120 / 121 / 123 |
| `4` / `8` | immediates | input nibbles (left / right) |
| `1` / `2` | immediates | pending-direction words |

### Callees

`0x802DCD5C`/`0x802DCDB0` (save/restore), `0x80628940` (`fn_80628940`),
`0x80195380` (`fn_80195380`), `0x80628770` (`fn_80628770`), `0x800B53A0`
(`fn_800b53a0`; the export reads its `r3`, which `fn_800b53a0` drops, so call
the address directly when the flag matters).

### Verdict

**ready (medium).** The class's *nominal* name is not recoverable from this body
alone (the family is named only as "the `+0xA80` nine-row board" in
`docs/COVERAGE_WAVE130_BASES.md:48-56`); every offset, nibble, sound id,
descriptor and gate is literal, and the `r5` argument is proven from the raw
stream. The four BSS descriptors are pushed, not read.

---

## Verification

`%TEMP%\opencode\w156\verify.js` re-derives the four sources independently of
`pick.js` and prints, for each row: TSV size, `decompiled`, export existence,
ledger membership, `fn fn_<addr>` membership, spec-heading membership, reachable
word count, last reachable word, computed extent end, whether that word is
`blr`, number of `blr`s and number of unknown words. Result:

```
0x806db340 size=572 dec=true export=yes ledger=absent rust=absent heading=absent reach=143 last=0x806db578 end=0x806db578 enderBlr=true blr=1 unknown=0
0x807041b0 size=572 dec=true export=yes ledger=absent rust=absent heading=absent reach=143 last=0x807043e8 end=0x807043e8 enderBlr=true blr=1 unknown=0
0x80745dc0 size=568 dec=true export=yes ledger=absent rust=absent heading=absent reach=142 last=0x80745ff4 end=0x80745ff4 enderBlr=true blr=1 unknown=0
0x8077d340 size=568 dec=true export=yes ledger=absent rust=absent heading=absent reach=142 last=0x8077d574 end=0x8077d574 enderBlr=true blr=1 unknown=0
0x80788090 size=568 dec=true export=yes ledger=absent rust=absent heading=absent reach=142 last=0x807882c4 end=0x807882c4 enderBlr=true blr=1 unknown=0
0x8079b7c0 size=568 dec=true export=yes ledger=absent rust=absent heading=absent reach=142 last=0x8079b9f4 end=0x8079b9f4 enderBlr=true blr=1 unknown=0
0x806df820 size=564 dec=true export=yes ledger=absent rust=absent heading=absent reach=141 last=0x806dfa50 end=0x806dfa50 enderBlr=true blr=1 unknown=0
0x80628f70 size=560 dec=true export=yes ledger=absent rust=absent heading=absent reach=140 last=0x8062919c end=0x8062919c enderBlr=true blr=1 unknown=0
TOTAL bytes=4540 words=1135 fns=3799 ledgerAddrs=17198 headings=294 tsvRows=5615
```

* reachable words equal `size / 4` for all eight, so no word inside an extent is
  unreachable and no word outside it is part of the body;
* each body has exactly one `blr`, in the last word of its extent;
* `unknown=0` for all eight (the decoder's counters for unknown opcodes,
  unknown `op31`/`op19` forms and unhandled float forms);
* the ledger and the Rust file grew between the selection pass and this
  verification pass (3,782 -> 3,799 `fn fn_` tokens, 285 -> 294 spec headings)
  because of the concurrent wave-155/156 lanes; none of the eight is among the
  additions.

The run above was made **before** this document was written. Re-running
`verify.js` now reports `heading=PRESENT:SPEC_WAVE156_BASES.md` for all eight -
i.e. this file is the only spec heading that names them - and `ledger=absent` /
`rust=absent` unchanged.

**Ready/conditional split: 8 / 0.**

## Next band (for wave 157)

After this wave the largest unclaimed rows are:

```
0x80763C20 560 | 0x807A0090 560 | 0x8077C440 556 | 0x80741A70 552
0x807390D0 548 | 0x80776440 548 | 0x807B2550 548 | 0x806D8B90 544
```

The two 560-byte rows are the remainder of the group this wave started; the
556- and 552-byte rows are singletons and the 548-byte group is exactly three.
None of them is in `ledger/functions.csv`, `src/game/bases.rs` or any spec
heading at the time of writing.
