# Wave 153 - `game::bases` - the next eight (624-604 byte band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified by this lane. The only file written is this one.
No merge or check tool was run. The throwaway tools live under
`%TEMP%\opencode\w153` (`pick.js`, `dis.js`, `range.js`, `hex.js`, `dump.js`,
`rdef.js`, `facts.js`, `ptr.js`, `xref.js`) and reuse the wave-150 decoder
`%TEMP%\opencode\w150\ppc.js`; none of them is in the repository. That temp
directory is shared with other wave-153 lanes, so files there can be overwritten
after a run; every number below was recorded from this lane's own run.

Waves 128-152 walked this bucket from the top in size order; the last row
claimed by `docs/SPEC_WAVE152_BASES.md` is 628 bytes. That document's
verification section names the next candidate explicitly
(`docs/SPEC_WAVE152_BASES.md:1656`, "the ninth is `0x807310C0` (624)"), so this
wave starts one rung lower, at **624 bytes**, and ends at 604.

```
0x807310C0 624 | 0x80794980 624 | 0x80787BD0 620 | 0x807B2050 620
0x80637A40 616 | 0x806E7080 616 | 0x80741010 612 | 0x8063A960 604
```

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x807310C0` | 624 | 156 | `807310c0_FUN_807310c0.c` | ready (high) |
| 2 | `0x80794980` | 624 | 156 | `80794980_FUN_80794980.c` | ready (medium) |
| 3 | `0x80787BD0` | 620 | 155 | `80787bd0_FUN_80787bd0.c` | ready (medium) |
| 4 | `0x807B2050` | 620 | 155 | `807b2050_FUN_807b2050.c` | ready (medium) |
| 5 | `0x80637A40` | 616 | 154 | `80637a40_FUN_80637a40.c` | ready (high) |
| 6 | `0x806E7080` | 616 | 154 | `806e7080_FUN_806e7080.c` | ready (medium) |
| 7 | `0x80741010` | 612 | 153 | `80741010_FUN_80741010.c` | ready (high) |
| 8 | `0x8063A960` | 604 | 151 | `8063a960_FUN_8063a960.c` | ready (medium) |

**Ready: 8. Conditional: 0.** 4,936 bytes / 1,234 words in total.

Every `bytes` value is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim and `decompiled` is
`true` for all eight. Seven of the eight extents end on their `blr`:

| # | `blr` | `address + size - 4` | span is exact |
|--:|---|---|:--:|
| 1 | `0x8073132C` | `0x8073132C` | yes |
| 2 | `0x80794BEC` | `0x80794BEC` | yes |
| 3 | `0x80787E38` | `0x80787E38` | yes |
| 4 | `0x807B22BC` | `0x807B22B8` | **no - 4 short** |
| 5 | `0x80637CA4` | `0x80637CA4` | yes |
| 6 | `0x806E72E4` | `0x806E72E4` | yes |
| 7 | `0x80741270` | `0x80741270` | yes |
| 8 | `0x8063ABB8` | `0x8063ABB8` | yes |

Row 4 is the `docs/REL_ELF_MAPPING.md:31-35` case: the TSV `size` is an extent,
not the instruction span. `0x807B2050` closes with
`lwz r0,116(r1)` / `mtlr r0` / `addi r1,r1,112` at `0x807B22B0..0x807B22B8` and
its `blr` at `0x807B22BC`; the extent stops one word short. The true body is
624 bytes. The following TSV row starts at `0x807B22E0`, so the `blr` belongs
unambiguously to this function. The four bytes past the extent are that `blr`;
`0x807B22C0..0x807B22D8` then hold two bare `blr` stubs and zero padding, which
are not part of the body.

Gaps after the eight extents (`next row - (address + size)`): `0`,
`0`, `596`, `36`, `8`, `104`, `12`, `36` bytes. Only row 4's gap includes the
`blr` above. Row 3's 596-byte hole begins at `0x80787E40` with a 16-byte sibling
(`li r0,0` / two stores / `blr`) and continues with the unlisted `0x80787EA0`;
row 7's 12 bytes and row 5's 8 bytes are zero padding.

The decoder reports **no unknown word** in the 1,234 words: zero unknown
opcodes, zero unhandled X-form, zero unhandled paired-single form. No `b`/`bc`
target leaves its own body, and every `bl` goes to a named callee outside the
body.

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

Nothing here needs `qr0`/`qr3` guessing except row 1, whose `qr3` use is a
single-element `u16` load through the GQR3 word already pinned at
`0x00050005` by the wave-138 lane (`docs/REL_ELF_MAPPING.md:52-56`):

| row | `psq` words |
|--:|---|
| 1 | two `f31`/`f30` prologue spills + epilogue reloads through `qr0` (`W = 0`), and four `psq_l f1,0(rX),1,qr3` single-halfword loads |
| 2 | one `f31` spill pair through `qr0` |
| 3-8 | none |

Row 1's four `psq_l` words are `0xE023B000` (three times) and `0xE026B000`: type
field 5 (unsigned 16-bit), scale 0, `W = 1` - exactly `GQR3 = 0x00050005`
load half. No row uses a non-zero store quantiser.

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
is this REL's BSS and reads as zero at file level; `0x80832EB0` and
`0x80833EA8` (rows 8 and 4) are in it.

`main.dol`-resident data and code are read from `work/dol/main.elf` through
**its own** program headers:

```
PT_LOAD  p_offset  p_vaddr     filesz
  0      0x000200  0x80004000  0x0026C0
  1      0x0002900  0x800066C0  0x000060
  2      0x002A00  0x80006720  0x000060
  3      0x002B00  0x80006780  0x2E7260   <- all 0x800xxxxx/0x802xxxxx code
  4      0x2E9E00  0x802ED9E0  0x0002E0
  5      0x2EA100  0x802EDCC0  0x000020
  6      0x2EA200  0x802EDCE0  0x0106C0
  7      0x2FA900  0x802FE3A0  0x0532A0   <- 0x8031xxxx/0x8032xxxx data
  8      0x000000  0x80351680  0x000000   (memsz 0xD6000, BSS)
  9      0x34DC00  0x80427680  0x002540
 10      0x000000  0x80429BC0  0x000000   (memsz 0x14C0, BSS)
 11      0x350200  0x8042B080  0x004B60
 12      0x000000  0x8042FBE0  0x000000   (memsz 0x5C, BSS)
```

`r13 = 0x8042F680` (`SDA_BASE`), so the small-data cells used below are
`r13-0x5404 = 0x8042A27C` (effect manager), `r13-0x5454 = 0x8042A22C`
(resource pool), `r13-0x54E0 = 0x8042A1A0` (the "target" object) and
`r13-0x56D0 = 0x80429FB0` (player-record base). `0x80354E50..0x80354EA0`,
`0x803741D0`, `0x80373D7C` and `0x80429CC0` are `main.dol` BSS.

**Calling-convention note used by rows 4, 6, 8.** `0x802DCD4C`, `0x802DCD64`,
`0x802DCD68` and `0x802DCD74` are MWCC `__savegprlr_N` helpers; they preserve
`r3` and return no value. Ghidra prints them as `iVar = func_0x802dcd74()` and
then uses that "return" as `this`. The translation must take `this` from `r3`
instead. The matching `__restgprlr_N` tails are `0x802DCD98`, `0x802DCDB0`,
`0x802DCDB4` and `0x802DCDC0`.

**Decoder note.** The wave-150 decoder prints the `neg` family
(`neg`/`addme`/`addze`/`subfme`/`subfze`/`mfcr`) as `reg,rA` while the
architecture puts the destination in bits 6-10. The only occurrence in this set
is `0x7C0400D0` in row 4, which is `neg r0,r4` (not `neg r4,r0`) and is read as
such below.

## Candidate selection (recomputed)

`%TEMP%\opencode\w153\pick.js` re-reads three sources from scratch:

* `work/ghidra/rel-d_basesNP/export/functions.tsv` - 5,615 rows, keeps
  `decompiled = true` and an export file that exists on disk;
* `ledger/functions.csv` - 17,102 distinct `address` fields (case-insensitive);
* `src/game/bases.rs` - every `fn fn_<addr>` definition token (3,748 at
  selection time; other lanes have since added 10 more, none of them in this
  set);
* every `#{2,4} ` heading of **all** 25 `docs/SPEC_WAVE*_BASES.md` files
  (260 distinct `0x` addresses). The task names `SPEC_WAVE12*_BASES.md`, of
  which only waves 128 and 129 exist; the superset is used so that no address
  already claimed by a wave-130..152 heading can be picked twice.

That leaves **1,838** rows. The eight largest are the table above. Ties are
broken by **ascending address**; this is the only place the ordering is not
forced by size, and it is stated so wave 154 can reproduce it:

* the 620-byte pair is `{0x80787BD0, 0x807B2050}`;
* the 616-byte pair is `{0x80637A40, 0x806E7080}`;
* the 604-byte trio is `{0x8063A960, 0x8068D060, 0x80799340}` - this wave takes
  `0x8063A960`, so **the ninth is `0x8068D060` (604) and the tenth is
  `0x80799340` (604)**, both fully decoded in `%TEMP%\opencode\w153` and neither
  in the ledger, the Rust file or any spec heading.

No `0x8068D060`/`0x80799340` input is lost by deferring them; they are simply
the next two rows of the same band.

Four of the eight are already *referenced* (never defined) by doc comments in
`src/game/bases.rs`, and the citations below quote the comment text rather than
line numbers because other lanes are editing that file concurrently:

* `0x80794980` - the `//!` block that defers "the state machine `0x80794980`
  that consumes its result";
* `0x80787BD0` - the `///` block on the `(double)(**(code **)(*obj + slot))(obj)`
  helper, which cites `80787bd0_FUN_80787bd0.c`;
* `0x807B2050` - the `///` block of `fn_807b29a0`, which links `[fn_807b2050]`
  as testing "the same array";
* `0x80741010` - the `///` block of `fn_807419c0` ("(`0x80741010`,
  `0x80741510`) are not reconstructed this pass") and its `// unknown:` line.

A `fn fn_<addr>` body for any of the eight does not exist, so all pass the
filter.

---

## 1. `0x807310C0` - `daRotDokan_c` two end-anchor pose update

**Evidence.** `work/ghidra/rel-d_basesNP/export/807310c0_FUN_807310c0.c`,
624 bytes, 156 words, synthetic `0x807310C0`. Body `0x807310C0..0x8073132C`
(ends `blr`, exactly one). No branch leaves the body. The six
`Removing unreachable block` warnings are the four in-body `psq_l` loads
(`0x80731150`, `0x80731180`, `0x80731234`, `0x80731264`) and the two epilogue
`psq_l` reloads (`0x80731304`, `0x8073130C`); the export's
`unaff_GQR0`/`unaff_GQR3`/`ldexpf` text is decompiler noise around `W`-field
loads.

**What it is.** The pose routine for the two end anchors of the `0xA08`-byte
`daRotDokan_c` rotating pipe (the class documented in the `///` block above
`fn_80730af0`: vtable `0x8080D0F8` at `+0x60`, interface vtable
`0x8080D1D8` at `+0x3D0`, three motion holders at `+0x418`/`+0x458`/`+0x498`).
The caller at `0x80731098` is the tail of the same actor's create/rebuild body:
it stores three floats at `+0x99C..+0x9A4`, then calls this function and
returns 1.

**Signature.**

```rust
/// Poses the two pipe-end motion holders of a `daRotDokan_c` actor.
///
/// # Original
/// - Ghidra: `FUN_807310c0` at `0x807310C0` (624 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807310c0_FUN_807310c0.c`
///
/// # Arguments
/// * `this` - the `0xA08` actor; yaw `+0x9D4`, anchor origin
///   `+0x978/+0x97C/+0x980`, holders `+0x458` and `+0x498`, matrix scratch
///   `+0x7C`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// Both motion holders must have been constructed by `0x80169CD0`
/// (`fn_80730af0`); the NW4R layout binder `0x802574E0` must be live.
pub unsafe fn fn_807310c0(this: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x07C` | `f32[12]` | 3x4 matrix written by `0x801C0BD0`, then rotated in place |
| `+0x458` | holder (0x40) | first motion holder ("entry" anchor) |
| `+0x498` | holder (0x40) | second motion holder ("exit" anchor) |
| `+0x978` | `f32` | anchor origin x |
| `+0x97C` | `f32` | anchor origin y |
| `+0x980` | `f32` | anchor origin z |
| `+0x9D4` | `s16` | yaw in 1/65536 turn |

### Exact behaviour

```
let yaw   = *(s16*)(this + 0x9D4);
let q     = (u16)(0x4000 - yaw);          // 16-bit wrap, stored with sth
let theta = (f32)q / 256.0;               // 0x807CD740 = 1/256

let x0 = *(f32*)(this + 0x978);
let y0 = *(f32*)(this + 0x97C);
let z0 = *(f32*)(this + 0x980);

for (holder, radius) in [(this + 0x458, 24.0f), (this + 0x498, 16.0f)] {
    let cx = x0 + cosf(theta) * radius;   // 0x80237C40
    let cy = y0 + sinf(theta) * radius;   // 0x80237BD0
    let cz = z0;

    0x801C0BD0(cx, cy, cz, this + 0x7C); // translation matrix
    let zero  = 0u16;
    let neg   = (u16)(-(i32)yaw);
    0x8016EDD0(this + 0x7C, &zero);      // first rotation component
    0x8016ECB0(this + 0x7C, &zero);      // second rotation component
    0x8016EEF0(this + 0x7C, &neg);       // third rotation component = -yaw
    0x8016A170(holder, this + 0x7C);     // bind matrix slot 0
    let one = [1.0f, 1.0f, 1.0f];
    0x8016A150(holder, &one);            // handle +0xDC..+0xE4
    0x8016A1A0(holder, 0);               // finalise
    let vt = *(u32*)holder;
    (*(vt + 0x14))(holder);              // apply/commit
}
```

`0x80237C40` is the cosine-table evaluator and `0x80237BD0` the sine-table
evaluator (`src/resource/bres.rs`, `fn_80237c40`/`fn_80237bd0`); the angle is
already reduced inside them, so the `/256` scale must be kept.

The three halfword stores before the trig calls (`[r1+0x28]=0`, `[r1+0x2A]=0`,
`[r1+0x2C]=-yaw`) are the three rotation components, not part of the position.
The `0x4000-yaw` halfword is stored twice (`r1+0x0E`, `r1+0x0C`) because the
compiler reloads the live range for each trig call; both hold the same value.

### Constants

| address | value | use |
|---|---:|---|
| `0x807CD710` | `1.0f` | the three scale components (`1,1,1`) |
| `0x807CD720` | `8.0f` | subtracted from `24.0` to give the second radius |
| `0x807CD73C` | `24.0f` | first radius |
| `0x807CD740` | `0.00390625f` (`1/256`) | halfword angle -> trig-table domain |
| - | `0x4000` | quarter-turn bias added to the yaw |
| - | `0x00050005` | `GQR3` load half (u16, scale 0) |

### Callees

`0x80237C40` (cosine), `0x80237BD0` (sine), `0x801C0BD0`
(`fn_801c0bd0`, translation matrix: `f1,f2,f3` = x,y,z, `r3` = out),
`0x8016EDD0`/`0x8016ECB0`/`0x8016EEF0` (halfword rotation setters,
`fn_8016edd0`/`fn_8016ecb0`/`fn_8016eef0`), `0x8016A170`/`0x8016A150`/
`0x8016A1A0` (`fn_8016a170`/`fn_8016a150`/`fn_8016a1a0`), and vtable slot
`+0x14` of the holder.

### Notes for the implementer

* `0x801C0BD0` takes the three floats in `f1,f2,f3` and the output pointer in
  `r3` (see `src/runtime/dvd.rs`); Ghidra's call prints the pointer last.
* `0x8016EDD0`/`0x8016ECB0`/`0x8016EEF0` each take `(matrix, &u16)` and
  concatenate a rotation; they are *not* independent setters - the order above
  is the order in the instruction stream (`0x807311C0`, `0x807311D4`,
  `0x807311E8`).
* The second pass reuses `+0x7C`; do not hoist the matrix out of the loop.
* `0x8016A170` stores the matrix into the holder's `+4` handle through
  `0x802574E0`, so `+0x458`/`+0x498` must be the holder objects, not matrices.

**Verdict: ready (high).** Leaf with respect to its own state; every literal is
read from the image; only the documented `qr3` single-element loads need the
GQR3 word.

---

## 2. `0x80794980` - `cobTsuta` growth-scale state machine

**Evidence.** `work/ghidra/rel-d_basesNP/export/80794980_FUN_80794980.c`,
624 bytes, 156 words, synthetic `0x80794980`. Body `0x80794980..0x80794BEC`
(ends `blr`, exactly one). One `Removing unreachable block` at `0x80794BCC` is
the `psq_l f31` reload. The next TSV row is the 12-byte leaf `0x80794BF0`, which
this body calls twice (`0x80794B10` in state 3, `0x80794B58` in state 4).

**What it is.** The five-state growth/scale driver of the ivy actor whose model
table sits at `0x8081B4EC` (strings `cobTsuta_up`, `cobTsuta_wait`,
`g3d/model.brres`, `cobTsuta`, names `K202`/`K213` at `0x8081B544..0x8081B570`).
This is the same record table used by the World-7 ramp evaluator
`0x80794C00` (`docs/SPEC_WAVE148_BASES.md:759-796`), and the same
"was deferred" body named in the `//!` block above `fn_8079fbd0`.

**Signature.**

```rust
/// Advances the `cobTsuta` growth scale, one call per frame.
///
/// # Original
/// - Ghidra: `FUN_80794980` at `0x80794980` (624 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80794980_FUN_80794980.c`
///
/// # Arguments
/// * `this` - the plant actor; state `+0x344`, timer `+0x330`, the two
///   animation-element scales `+0x22C`/`+0x29C`, character byte `+0x004`.
///
/// # Returns
/// Nothing.
pub unsafe fn fn_80794980(this: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | low byte = record index into `0x8081B4EC` (character/kind) |
| `+0x060` | vtable* | class vtable; slot `+0x68` = "begin/finish" |
| `+0x0AC` | vec3 | position handed to the effect submitter |
| `+0x1EC` | element | animation element; slot `+0x1C` = update |
| `+0x22C` | `f32` | scale channel A (`0x801680E0` target) |
| `+0x264` | element | second animation element |
| `+0x29C` | `f32` | scale channel B (`0x801680E0` target) |
| `+0x2C4` | `f32` | input of `0x80794BF0` |
| `+0x330` | `s32` | countdown frames |
| `+0x340` | `u8` | "binding enabled" latch (cleared, then set to 1) |
| `+0x344` | `s32` | state 0..4 |
| `+0x350` | `u32` | handle passed to `0x80102DB0` by `0x80794BF0` |

### Record table `0x8081B4EC`, stride `0x1C`, three rows

| idx | `+0x10` scale | `+0x18` start | `+0x1A` end | `+0x1C` fade |
|--:|---:|---:|---:|---:|
| 0 | `1.7f` | 90 | 210 | 30 |
| 1 | `1.7f` | 190 | 310 | 30 |
| 2 | `1.7f` | 290 | 410 | 30 |

Read out of the image at `0x8081B4EC` (`1.8f`, `0`, `0`, `0`, `1.7f`, `1.0f`,
`90`, `210`), `0x8081B508` and `0x8081B524`. `+0x1C` is **one halfword past the
28-byte row**, i.e. it aliases the following row's `+0x00`; every one of the
three words there is `0x001E0000`, so all three rows read `30`. This agrees with
the `n` column of `docs/SPEC_WAVE148_BASES.md:781-783` (90/210/30, 190/310/30,
290/410/30 for the same table). The index is `(u8)(*(u32*)(this + 4))` with
**no** bound check; index 3 would use a record at `0x8081B540` whose `+0x18` is
the string bytes at `0x8081B558` (`"atut"`), which the game never does.

### Exact behaviour

Let `s = *(s32*)(this + 0x344)`, `kind = *(u32*)(this + 4) & 0xFF`,
`row = 0x8081B4EC + kind * 0x1C`.

```
match s {
  0 => { (*(vt+0x68))(this); }                      // begin
  1 => {
      if *(s32*)(this + 0x330) < 1 { *(s32*)(this + 0x344) = 2; }
      else { *(s32*)(this + 0x330) -= 1; }
  }
  2 => {
      let scale = *(f32*)(row + 0x10);
      0x801680E0(scale, this + 0x22C);
      0x801680E0(scale, this + 0x29C);
      *(s32*)(this + 0x330) = (s32)*(s16*)(row + 0x1A) - (s32)*(s16*)(row + 0x18);
      0x80105240(*(u32*)0x8042A27C, 0x3E, this + 0xAC, 1);
      *(s32*)(this + 0x344) = 3;
  }
  3 => {
      if *(s32*)(this + 0x330) < 1 { *(s32*)(this + 0x344) = 4; }
      else {
          let t0 = *(s32*)(this + 0x330) - 1;          // stored back first
          *(s32*)(this + 0x330) = t0;
          let fade = (s32)*(s16*)(row + 0x1C);
          if t0 <= fade {
              let t = (f32)(fade - t0) / (f32)fade;    // 0 -> 1
              let scale = *(f32*)(row + 0x10);
              0x801680E0((1.0f - t) * scale, this + 0x22C);
              let v = fn_80794bf0(this);               // f32
              0x801680E0((1.0f - t) * scale + t * v, this + 0x29C);
          }
      }
  }
  4 => {
      0x801680E0(0.0f, this + 0x22C);                  // 0x807D0058
      let v = fn_80794bf0(this);
      0x801680E0(v, this + 0x29C);
      (*(vt+0x68))(this);
  }
  _ => {}
}

// common tail, executed for states 1-4 only (state 0 jumps to the epilogue)
0x80168080(this + 0x22C);
0x80168FE0(this + 0x264);
0x801680E0(1.0f, this + 0x264);                         // 0x807D005C
(*(*(u32*)(this+0x1EC) + 0x1C))(this + 0x1EC);
*(u8*)(this + 0x340) = 0;
fn_80794480(this, this + 0x1EC);
*(u8*)(this + 0x340) = 1;
```

State 0 diverges: it calls `vt[0x68]` and **jumps to the epilogue**, skipping the
common tail (branch at `0x807949F0` -> `0x80794BC8`). All other states fall into
the tail. The `float -> int` conversions use the `0x43300000`/`xoris` idiom
(`0x807D0060 = 2^52 + 2^31`, `4503601774854144.0`) and `f32` rounding after
`fsubs`, so `t` is a *single-precision* quotient.

`0x80794BF0` is 12 bytes and fully decodable even though Ghidra prints
`halt_baddata` for it:

```
lfs f1, 0x2C4(r3)
lwz r3, 0x350(r3)
b   0x80102DB0            ; tail call, f32 result in f1
```

### Constants

| address | value | use |
|---|---:|---|
| `0x807D0058` | `0.0f` | channel-A end value |
| `0x807D005C` | `1.0f` | channel-B end value and the `+0x264` seed |
| `0x807D0060` | `4503601774854144.0` (`2^52 + 2^31`) | int -> double bias |
| table `+0x10` | `1.7f` | growth peak scale |
| - | `0x3E` (62) | effect/record id submitted to `0x80105240` |

### Callees

`0x801680E0` (`fn_801680e0`, element float setter, vtable `+0x28`),
`0x80168080` (`fn_80168080`, vtable `+0x20`), `0x80168FE0` (`fn_80168fe0`),
`0x80105240` (`wm_effect_record_submit`, `(base, tag, transform, action)`),
`0x80794BF0` (leaf above), `0x80794480` (`fn_80794480`), vtable slots `+0x68`
(on `+0x60`) and `+0x1C` (on `+0x1EC`).

### Notes for the implementer

* `0x80105240`'s first argument is the *contents* of `0x8042A27C`, i.e. the
  effect manager cell, not the cell address.
* The `t` quotient is `(fade - t0) / fade` computed in double and narrowed
  through `fsubs`; `t0 <= fade` includes equality.
* `0x801680E0(float, usize)` is declared `(value, target)` in
  `src/resource/registry.rs`; the instruction stream puts the value in `f1` and
  the target in `r3`, so Ghidra's `func_0x801680e0(value, ptr)` order is right.
* The `+0x340` latch brackets `0x80794480` exactly (0, call, 1); keep the order.

**Verdict: ready (medium).** Table and all four constants are read; the only
softness is the arithmetic association of the `t` interpolation, which the
listing fixes.

---

## 3. `0x80787BD0` - World-7 `Koopa` create tail

**Evidence.** `work/ghidra/rel-d_basesNP/export/80787bd0_FUN_80787bd0.c`,
620 bytes, 155 words, synthetic `0x80787BD0`. Body `0x80787BD0..0x80787E38`
(ends `blr`, exactly one; `0x80787E4C`/`0x80787E50` are separate `blr` stubs in
the 596-byte gap). No `psq` word. The export's only warning is the
`_DAT_...`/`DAT_...` overlap at `0x807CFA08`.

**What it is.** The tail of the spawn/create path of the World-7 Koopa actor
whose model node is named `"Koopa"` (`0x80818C78`, followed by course tags
`F7C0`/`W7C0`) and whose part strings are `Head`/`face_1`/`hair_1`
(`0x808192E1..0x808192F4`). `0x80787BD0` is the first entry of the part
descriptor block at `0x808192F8` and is reached through nine such tables
(pointer scan: `0x808192FC`, `0x808194DC`, `0x80819758`, `0x808198EC`,
`0x80819A6C`, `0x80819BEC`, `0x80819D64`, `0x80819ED8`, `0x8081A054`). The
block continues `0x800F27D0`, `0x800F2800`, `0x80788040`, `0x800F2810`,
`0x800F2840`, `0x80787EA0`, ... and `0x80788090` is the model loader that
builds `g3d/%s.brres` from `+0x43C`.

**Signature.**

```rust
/// Completes the spawn of a World-7 Koopa actor: publishes the pose, chooses
/// the per-player visibility, and allocates the 28-byte point record.
///
/// # Original
/// - Ghidra: `FUN_80787bd0` at `0x80787BD0` (620 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80787bd0_FUN_80787bd0.c`
///
/// # Arguments
/// * `this` - the actor; vtable `+0x60`, player index `+0x004`,
///   position `+0x0AC`, scale `+0x0DC`, profile pointer `+0x420`.
///
/// # Returns
/// `1` - the create callback always reports success.
pub unsafe fn fn_80787bd0(this: usize) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | low byte = player/character index 0..3 |
| `+0x060` | vtable* | class vtable; slots `0x74`, `0x78`, `0x88`, `0x90`, `0x94`, `0xA8`, `0xAC` |
| `+0x0AC`..`+0x0B4` | vec3 | position |
| `+0x0DC`..`+0x0E4` | vec3 | scale triple |
| `+0x128`..`+0x130` | vec3 | previous position |
| `+0x134` | `f32` | speed/offset from vtable `+0xAC` |
| `+0x3C4` | `f32` | cleared |
| `+0x3DC`..`+0x3E4` | vec3 | delta to the targeted object |
| `+0x3E8` | `s32` | state word, set to `6` |
| `+0x420` | ptr | parameter record; `+0x26` `s16` count, `+0x28` `s16` |
| `+0x428` | ptr | the newly allocated 28-byte record |
| `+0x434` | `f32` | cleared |

### Exact behaviour

```
let vt = *(u32*)(this + 0x60);
(*(vt+0x94))(this);                          // 0x80787BF0
fn_80788090(this);                           // model/binding pass (568 B)
(*(vt+0x78))(this);                          // 0x80787C10
fn_807882E0(this);                           // 28 B helper
*(u32*)(this+0x128) = *(u32*)(this+0x0AC);
*(u32*)(this+0x12C) = *(u32*)(this+0x0B0);
*(u32*)(this+0x130) = *(u32*)(this+0x0B4);
*(f32*)(this+0x134) = (f32) (*(vt+0xAC))(this);   // f32 return widened
(*(vt+0x90))(this);

let kind = *(u32*)(this + 4) & 0xFF;
let one  = *(f32*)(*(u32*)(this+0x420) + 4);
*(f32*)(this+0x0DC) = one;
*(f32*)(this+0x0E0) = one;
*(f32*)(this+0x0E4) = one;

if kind == 1 || kind == 3 {
    let mut v = [f32;3];
    0x80788AE0(&v, this);                    // position of node "Koopa"
    *(this+0x0AC) = v.x; *(this+0x0B0) = v.y; *(this+0x0B4) = v.z;
    v.y += (*(vt+0x88))(this);               // f32 offset
    *(f32*)(this+0x0B0) = v.y;
    let t = *(u32*)0x8042A1A0;               // target object (pointer cell)
    *(f32*)(this+0x3DC) = *(f32*)(t+0x0AC) - *(f32*)(this+0x0AC);
    *(f32*)(this+0x3E0) = *(f32*)(t+0x0B0) - v.y;
    *(f32*)(this+0x3E4) = *(f32*)(t+0x0B4) - *(f32*)(this+0x0B4);
} else if kind == 2 {
    let mut v = [f32;3];
    0x80788AE0(&v, this);
    *(this+0x0AC) = v.x; *(this+0x0B0) = v.y; *(this+0x0B4) = v.z;
    *(f32*)(this+0x0B0) = v.y - 100.0f;      // 0x807CFA0C
    let s = 0.01f;                            // 0x807CFA10
    *(f32*)(this+0x0DC) = s; *(f32*)(this+0x0E0) = s; *(f32*)(this+0x0E4) = s;
} else if kind == 0 {
    let s = 0.01f;
    *(f32*)(this+0x0DC) = s; *(f32*)(this+0x0E0) = s; *(f32*)(this+0x0E4) = s;
}

(*(vt+0xA8))(this);
fn_807883E0(this);                           // 20 B helper
(*(vt+0x74))(this);
*(f32*)(this+0x3C4) = 0.0f;                  // 0x807CFA08
*(f32*)(this+0x434) = 0.0f;
*(s32*)(this+0x3E8) = 6;

let rec = 0x802B9210(0x1C);                  // allocator; may return 0
if rec != 0 {
    *(u32*)(rec+0x00) = 0x803214B0;          // descriptor word
    *(u32*)(rec+0x04) = 0;
    *(u32*)(rec+0x08) = 0;
    *(u8*)(rec+0x0C)  = 0;
    *(u8*)(rec+0x0D)  = 0;
    *(u8*)(rec+0x0E)  = 0;
    *(u32*)(rec+0x18) = 0;
}
let src = *(u32*)(this+0x420);
*(u32*)(this+0x428) = rec;
*(u32*)(rec+0x18) = src + 0x26;
*(u32*)(rec+0x04) = (u32)((s32)*(s16*)(src+0x26) - 1);
*(u32*)(rec+0x08) = (u32)(s32)*(s16*)(src+0x28);
return 1;
```

Note the unconditional `rec[0x18] = src + 0x26` after the null check: the
original writes through `rec` even when the allocator returned 0 (a shipped
fault). Reproduce it literally, or guard with `// observed:` and the same
behaviour when `rec != 0`; the null path traps on hardware.

`0x803214B0` is a 40-byte `main.dol` descriptor starting
`{0, 0, 0x800EE430, 0x80102CA0, 6, 0x80429060, 6, 0, 4, 0x80429068}` followed
by the tag `"Wm_cs_mrland"`; the body only stores its address in `rec+0`.

### Constants

| address | value | use |
|---|---:|---|
| `0x807CFA08` | `0.0f` | `+0x3C4` and `+0x434` |
| `0x807CFA0C` | `100.0f` | the player-2 vertical drop |
| `0x807CFA10` | `0.01f` | the hidden scale for players 0 and 2 |
| `0x803214B0` | descriptor | 28-byte record tag word |
| `0x8042A1A0` | object cell | target whose position forms the delta |
| - | `6` | state word `+0x3E8` |
| - | `0x1C` | record size |

### Callees

`0x80788090` (568 B, model/part binder), `0x807882E0` (28 B),
`0x80788AE0` (108 B, "Koopa"-node position), `0x807883E0` (`fn_807883e0`),
`0x802B9210` (`fn_802b9210`, allocator), vtable slots `+0x74`, `+0x78`,
`+0x88`, `+0x90`, `+0x94`, `+0xA8`, `+0xAC`.

### Notes for the implementer

* `0x80788AE0(&vec, this)` fills the caller buffer with the world position of
  the node named `"Koopa"` (`0x80164F00(vec, "Koopa")` + `0x80168230`); the
  buffer is the first argument, `this` the second.
* Ghidra prints `(double)` around `vt[0xAC]` because the vtable slot returns a
  single-precision value; store the `f32` directly.
* `DAT_807CFA08` is `0.0f`, so `+0x3C4` and `+0x434` are plain zero stores.
* `+0x420` is the actor's parameter block; `+0x26`/`+0x28` are `s16` and are
  read *after* the two `+0x18` writes, so the record must be built in that
  order only for the null-allocator fault above.

**Verdict: ready (medium).** All seven virtual slots are fixed by their use, all
constants read, the descriptor word read from `main.dol`, and the only hazards
(the `rec == 0` write-through and the `f32`/`f64` print) are explicit.

---

## 4. `0x807B2050` - player-link table rebuild (`0x80354E50..0x80354EA0`)

**Evidence.** `work/ghidra/rel-d_basesNP/export/807b2050_FUN_807b2050.c`,
620 bytes as an extent, 155 words, synthetic `0x807B2050`. Body
`0x807B2050..0x807B22BC` (624 bytes; the TSV extent omits the final `blr`, see
the extent table above). No `psq`, no jump table, one `b` out of the loop at
`0x807B21A4` and one at `0x807B2258`, both forward inside the body.

**What it is.** The pass that (a) probes the four per-player archive names and
(b) rebuilds the four `main.dol` BSS arrays `0x80354E50` (active flags),
`0x80354E60` (player order), `0x80354E70` (id a), `0x80354E80` (id b),
`0x80354E90` (constant `5`) and `0x80354EA0` (scratch). Its translated sibling
`fn_807b29a0` (the `///` block that links `[fn_807b2050]`) documents the same
`+0x3C4` array as
"player joined" and names `0x801024A0` as the archive-name builder and
`0x80373D7C` as its template; `fn_807b2b10` (`:88477`) walks the same records.
`0x80354E60` is exactly the table that row 8 (`0x8063A960`) reads.

**Signature.**

```rust
/// Rebuilds the player-link arrays after the stage byte flips, then replays a
/// layout descriptor.
///
/// # Original
/// - Ghidra: `FUN_807b2050` at `0x807B2050` (620-byte extent; 624-byte body)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807b2050_FUN_807b2050.c`
///
/// # Arguments
/// None; `this` is the current object left in `r3` by `0x802DCD4C`.
///
/// # Returns
/// Nothing.
pub unsafe fn fn_807b2050()
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x3C4` | `u8[4]` | per-player "joined" flags |
| `+0x74` | object | interface object; vtable slot `+0x18` receives the descriptor |

Globals: `0x80315B90` (stage object; `+7` bit 0 = stage ready, `+8` word,
`+4` byte), `0x803741D0` (4 pointers, one per player), `0x80373D7C`
(archive-name template), `0x80377C48` (name-builder data word), `0x8042A22C`
(resource pool cell), `0x80429CC0` (cleared), `0x80833EA8` (layout descriptor in
this REL's BSS), and the six BSS arrays listed above.

### Exact behaviour

```
let this = r3;                              // preserved by 0x802DCD4C
if (*(u8*)0x80315B97 & 1) != 0 {
    for i in 0..4 {
        if *(u8*)(this + i + 0x3C4) != 0 {
            if *(u32*)(0x803741D0 + i*4) == 0 {
                0x80101FF0(*(u32*)0x80377C48, i);
            }
            let obj = *(u32*)(0x803741D0 + i*4);
            let mut buf = [u8; 0x38];
            0x801024A0(&buf, 0x80373D7C,
                       *(u32*)(0x80315B90 + 8), *(u8*)(0x80315B90 + 4), i);
            if 0x800DDEA0(obj, *(u32*)0x8042A22C, &buf) == 0 {
                goto tail;                    // 0x807B22A8
            }
        }
    }
}

for i in 0..4 {
    let p = *(u32*)(0x803741D0 + i*4);
    if p == 0 {
        *(u32*)(0x80354E50 + i*4) = 0;
    } else {
        0x800DDF00(p);                         // detach/prepare
        if i == 0 {
            0x800B2EB0(*(u32*)(p + 0xB0));
            *(u8*)(*(u32*)0x80429F7C + 0x380) = (*(u8*)(p + 0xCD) != 0) as u8;
        }
        let new_id = *(i32*)(p + 0xBC);
        if new_id < 0 {
            *(u32*)(0x80354E50 + i*4) = 0;
        } else {
            let old = *(u32*)(0x80354E60 + i*4);   // value displaced from slot i
            *(u32*)(0x80354E50 + i*4) = 1;
            *(u32*)(0x80354E60 + i*4) = new_id as u32;
            *(u32*)(0x80354E70 + new_id*4) = *(u32*)(p + 0xC0);
            *(u32*)(0x80354E80 + new_id*4) = *(u32*)(p + 0xC4);
            *(u32*)(0x80354E90 + new_id*4) = 5;
            // compiler-unrolled swap: first other slot holding new_id takes `old`
            for j in 0..4 {
                if j != i && *(u32*)(0x80354E60 + j*4) == new_id as u32 {
                    *(u32*)(0x80354E60 + j*4) = old;
                    break;
                }
            }
        }
    }
    *(u32*)(0x80354EA0 + 4 * *(u32*)(0x80354E60 + i*4)) = 0;
}

*(u32*)0x80429CC0 = 0;
let obj = this + 0x74;
(*(*(u32*)obj + 0x18))(obj, 0x80833EA8);

tail:
0x802DCD98();                                  // __restgprlr_17
```

The four `if (j != i && array[j] == new_id) { array[j] = old; break; }` arms are
the compiler's unrolled form of "if the id just installed in slot `i` was already
held by another slot, hand the displaced id to that slot" - a swap. The listing
checks the arms in the order `j = 0, 1, 2, 3`, skips `j == i`
(`0x807B21E0`/`0x807B21FC`/...), and leaves the arm sequence after the first
match (`b 0x807B2264`). `old` is `0x80354E60[i]` **before** the store. In
practice the ids are a permutation of 0..3, so exactly one arm can fire.

### Constants

| address | value | use |
|---|---:|---|
| `0x80354E90 + id*4` | `5` | per-slot constant |
| `0x80833EA8` | descriptor | second argument of vtable `+0x18` |
| `0x80315B97` | bit 0 | "stage ready" gate |
| - | `0x38` | name-buffer stack size (56 bytes) |

### Callees

`0x802DCD4C`/`0x802DCD98` (`__savegprlr_17`/`__restgprlr_17`), `0x80101FF0`
(`fn_80101ff0`-family, per-slot init), `0x801024A0` (name builder),
`0x800DDEA0` (resource probe), `0x800DDF00` (detach/prepare),
`0x800B2EB0` (`fn_800b2eb0`), vtable slot `+0x18` on `this+0x74`.

### Notes for the implementer

* `this` is not an argument: `0x802DCD4C` preserves `r3`. Take it from `r3`
  (see the calling-convention note in Method).
* `0x80315B90` is a `main.dol` object; `+7` bit 0 is the same "stage ready"
  bit that `fn_807b29a0` reads, `+8` is a word and `+4` a byte.
* The `!=` at `*(u8*)(p+0xCD)` is the `neg`/`or`/`rlwinm` idiom at
  `0x807B2180..0x807B218C` (see the decoder note); it is not a negated byte
  store.
* Do not fold the `0x80354EA0` write into the loop-exit: it uses the *possibly
  rewritten* `0x80354E60[i]`, so it must run after the four collision arms.

**Verdict: ready (medium).** Body fully decoded (624 bytes), all globals named,
no branch out; medium because the four collision arms are compiler-unrolled and
the TSV extent is 4 bytes short.

---

## 5. `0x80637A40` - four-record player-order permutation (no calls)

**Evidence.** `work/ghidra/rel-d_basesNP/export/80637a40_FUN_80637a40.c`,
616 bytes, 154 words, synthetic `0x80637A40`. Body `0x80637A40..0x80637CA4`
(ends `blr`, exactly one). **Zero** `bl` and zero `b` outside the body - this is
a leaf. Caller: `0x80637E20` inside `0x80637D20` (400 bytes).

**What it is.** The "reorder the four 0x3C-byte per-player records so that
controller order 0,1,3,2 is physical order" pass. It is the twin of row 8
(`0x8063A960`), but over the `+0x7BC` block and with the record id at `+0x24`
instead of `+0x14`. The permutation table `0x807C7D18 = {0, 1, 3, 2}` is shared
with `0x8063A960`'s `0x807C7F70 = {0, 1, 3, 2}`.

**Signature.**

```rust
/// Reorders the four 0x3C-byte player records at `+0x7BC` into controller
/// order `{0,1,3,2}` when no record is pending.
///
/// # Original
/// - Ghidra: `FUN_80637a40` at `0x80637A40` (616 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80637a40_FUN_80637a40.c`
///
/// # Arguments
/// * `this` - the page/state object; records `+0x7BC`, stride `0x3C`, four of them.
///
/// # Returns
/// Nothing.
pub unsafe fn fn_80637a40(this: usize)
```

### Field map - one record, stride `0x3C`, four records at `+0x7BC + i*0x3C`

| offset | type | meaning |
|---:|---|---|
| `+0x00` | `u8` | active/used flag |
| `+0x04` | `u32` | field 1 |
| `+0x08` | `u32` | field 2 |
| `+0x0C` | `u32` | field 3 |
| `+0x10` | `u32` | field 4 |
| `+0x14` | `u32` | field 5 |
| `+0x18` | `u32` | field 6 |
| `+0x1C` | `u32` | field 7 |
| `+0x20` | `u32` | field 8 |
| `+0x24` | `u32` | record id (0..3) - the sort key |
| `+0x28` | `u8` | flag A |
| `+0x29` | `u8` | "pending/active" flag B |
| `+0x2A` | `u8` | flag C |
| `+0x2C` | `f32` | value 1 |
| `+0x30` | `f32` | value 2 |
| `+0x34` | `f32` | value 3 |
| `+0x38` | `f32` | value 4 |

### Exact behaviour

```
fn pending(r: usize) -> bool {
    // slot 0 at +0x7BC; slot k at +0x7BC + 0x3C*k
    *(u8*)(r + 0x29) != 0 &&
      !(*(u32*)(r + 0x04) == 0 && *(u32*)(r + 0x0C) == 0 && *(u8*)(r + 0x00) == 0)
}

for i in 0..4 {
    if pending(this + 0x7BC + i*0x3C) { return; }
}

let mut tmp = MaybeUninit::<[u8; 0xF0]>::uninit();   // 4 * 0x3C, uninitialised
for k in 0..4 {
    let want = *(u32*)(0x807C7D18 + k*4);            // 0, 1, 3, 2
    for i in 0..4 {
        let r = this + 0x7BC + i*0x3C;
        if *(u32*)(r + 0x24) == want {
            copy 0x3C bytes from r to &mut tmp[k*0x3C];
            break;
        }
    }
}
for k in 0..4 {
    copy 0x3C bytes from &tmp[k*0x3C] to this + 0x7BC + k*0x3C;
}
```

The `pending` test is exactly the guard the export spells at
`0x80637A44..0x80637B14`: per slot, if `+0x29` is set then `+0x04`, `+0x0C` and
`+0x00` must all be zero, otherwise the function returns without touching
anything. `0x807C7D18` holds `0x00000000`, `0x00000001`, `0x00000003`,
`0x00000002` (read from the image).

### Constants

| address | value | use |
|---|---|---|
| `0x807C7D18` | `0, 1, 3, 2` | target order, indexed by the outer loop |
| - | `0x3C` (60) | record stride and copy size |
| - | `0xF0` (240) | temp buffer (`stwu r1,-256`) |

### Callees

None. This is a pure leaf: no `bl`, no `b` out, no indirect call, no global
write.

### Notes for the implementer

* If no record matches `want`, the original leaves that `tmp` slot
  uninitialised and copies stack garbage into the destination (a shipped
  fault). The safest reconstruction keeps the `break`-less path but zero-fills
  the temp and adds `// observed: uninitialised in the original`.
* `+0x2A` and the four floats are copied but never interpreted here.
* The copy loops are `bdnz` over `ctr = 4`; the machine copy order is
  ascending, so a plain byte copy is bit-exact.

**Verdict: ready (high).** Leaf, no callees, no globals, table read from the
image, one `blr` at `address + size - 4`.

---

## 6. `0x806E7080` - `lift_zen_shiro_yogan` create/orientation selector

**Evidence.** `work/ghidra/rel-d_basesNP/export/806e7080_FUN_806e7080.c`,
616 bytes, 154 words, synthetic `0x806E7080`. Body `0x806E7080..0x806E72E4`
(ends `blr`, exactly one). The 104-byte gap after the extent is two zero words
plus the prologue of the next, unnamed function at `0x806E72F0`.

**What it is.** The spawn/create tail of the elevator actor whose model is
`lift_zen_shiro_yogan.brres` (string at `0x807FD7B4`, tag
`lift_zen_shiro_yogan` at `0x807FD7BC`); the descriptor at `0x807FD7CC` lists
`0x806E7080` followed by `0x80064350`, `0x80064380`, `0x806E73D0`, ... and the
pointer-scan hit is `0x807FD7D8`. The body
has four spawn-quadrant arms selected by bits 22..23 of the object flags, and
selects a spawn offset `(x, z)` per quadrant.

**Signature.**

```rust
/// Completes the spawn of the `lift_zen_shiro_yogan` elevator.
///
/// # Original
/// - Ghidra: `FUN_806e7080` at `0x806E7080` (616 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/806e7080_FUN_806e7080.c`
///
/// # Arguments
/// * `this` - the actor; flags `+0x04`, platform state at `+0x3FC`,
///   path/element block at `+0x474`.
///
/// # Returns
/// `1` - create success.
pub unsafe fn fn_806e7080(this: usize) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | flags: bits 8..11, 12..15, 16..19, 22..23, 6..7 (see below) |
| `+0x0D0`/`+0x0D4`/`+0x0D8` | `f32` | 72.0 / -24.0 / 0.0 |
| `+0x318` | `f32` | x offset |
| `+0x31C` | `f32` | z offset |
| `+0x320` | `f32` | second x offset |
| `+0x324` | `f32` | second z offset |
| `+0x3FC` | object | platform state, initialised by `0x800D77D0` |
| `+0x474` | object | path/element block |
| `+0x4C0`/`+0x4C4`/`+0x4C8` | fnptr | `0x806E6E40` / `0x806E6E70` / `0x806E6EA0` |
| `+0x544` | `s32` | set to `1` |

### Exact behaviour

```
let w = *(u32*)(this + 4);
let a = (w >> 8)  & 0xF;
let b = (w >> 22) & 0x3;          // quadrant selector
let c = (w >> 12) & 0xF;
let d = (w >> 16) & 0xF;
let e = (w >> 6)  & 0x3;

0x800D77D0(this + 0x3FC, a, b, c, d, e);
fn_806E6FB0(this);

let s = (f32)b;
let (x1, z1, x2, z2) = match b {
    0 => (656.0 + 16.0*s, 560.0,  72.0 -  8.0*s, -24.0),
    1 => (656.0 + 16.0*s, 560.0,  72.0 +  8.0*s, -24.0),
    2 => (656.0,         560.0 + 16.0*s, 72.0, -24.0 - 8.0*s),
    _ => (656.0,         560.0 + 16.0*s, 72.0, -24.0 + 8.0*s),
};
*(f32*)(this+0x318) = x1;  *(f32*)(this+0x31C) = z1;
*(f32*)(this+0x320) = x2;  *(f32*)(this+0x324) = z2;
*(f32*)(this+0x0D0) = 72.0;
*(f32*)(this+0x0D4) = -24.0;
*(f32*)(this+0x0D8) = 0.0;

fn_806E6F00(this);

0x8007FA40(this + 0x474, this,
           0.0, 0.0, 144.0, -48.0,          // f1..f4
           0x806E6CF0, 0x806E6D50, 0x806E6DB0, 3, 0, 0);
*(u32*)(this+0x4C0) = 0x806E6E40;
*(u32*)(this+0x4C4) = 0x806E6E70;
*(u32*)(this+0x4C8) = 0x806E6EA0;
*(s32*)(this+0x544) = 1;
0x8007F900(this + 0x474);
fn_806E7010(this, w);
fn_806E6FE0(this);
return 1;
```

The four arms are the *only* difference between the quadrants: arms 0/1 offset
x and keep z, arms 2/3 keep x and offset z, and the sign flips between the two
members of each pair. `s` is the 2-bit quadrant value as a float, so offsets are
`8*s` / `16*s`.

### Constants

| address | value | use |
|---|---:|---|
| `0x807CBD58` | `0.0f` | `+0xD8` |
| `0x807CBD5C` | `72.0f` | `+0xD0` and the x base |
| `0x807CBD60` | `8.0f` | quadrant step (x/z) |
| `0x807CBD64` | `-24.0f` | `+0xD4` and the z base |
| `0x807CBD68` | `656.0f` | second x base |
| `0x807CBD6C` | `16.0f` | quadrant step (z/x) |
| `0x807CBD70` | `560.0f` | second z base |
| `0x807CBD74` | `144.0f` | `f3` of `0x8007FA40` |
| `0x807CBD78` | `-48.0f` | `f4` of `0x8007FA40` |
| `0x807CBD80` | `2^52` | `u32 -> double` bias for `s` |

### Callees

`0x800D77D0` (platform state init, 6 args), `0x806E6FB0` (`fn_806e6fb0`),
`0x806E6F00` (`fn_806e6f00`), `0x8007FA40` (element/path constructor with three
callbacks), `0x8007F900` (register element), `0x806E7010` (`fn_806e7010`),
`0x806E6FE0` (`fn_806e6fe0`).

### Notes for the implementer

* `0x8007FA40` takes four float arguments before its six integer arguments;
  the constants `0.0, 0.0, 144.0, -48.0` are literal in `f1..f4`.
* `0x800D77D0`'s five bitfields are *not* independent: the same flag word is
  sliced five times; keep the shifts and masks exactly.
* `fn_806E7010(this, w)` receives the whole flags word, not a field.
* Ghidra prints `(double)` around `0x8007FA40`'s floats because the callee
  saves them as doubles; the source values are single precision.

**Verdict: ready (medium).** Every field and constant is literal; medium only
because three of the element-builder callbacks (`0x806E6CF0`, `0x806E6D50`,
`0x806E6DB0`) stay address-only, which the ready rule allows.

---

## 7. `0x80741010` - four-player result tally and rotating counter

**Evidence.** `work/ghidra/rel-d_basesNP/export/80741010_FUN_80741010.c`,
612 bytes, 153 words, synthetic `0x80741010`. Body `0x80741010..0x80741270`
(ends `blr`, exactly one). **Zero** `bl` - a leaf. Caller: `0x807419DC` inside
`fn_807419c0` (108 bytes), whose `///` block already records that
"(`0x80741010`, `0x80741510`) are not reconstructed this pass".

**What it is.** A per-frame result-screen update: it turns four "request" rows
into up to sixteen popup slots, then advances the shared counter angle from the
four players' scores/rates, picking the leading player. The structure matches a
4-player score/result page (records with ids, scores at `+0x39C` and rates at
`+0x3A0`).

**Signature.**

```rust
/// Advances the four-player result tally and the shared counter wheel.
///
/// # Original
/// - Ghidra: `FUN_80741010` at `0x80741010` (612 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80741010_FUN_80741010.c`
///
/// # Arguments
/// * `this` - the result page; requests `+0x4E4`, slots `+0x514`, players
///   `+0x394`, wheel angle `+0x514`.
///
/// # Returns
/// Nothing.
pub unsafe fn fn_80741010(this: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x394 + k*0x10` | `u32` | player id, `-1` = empty |
| `+0x398 + k*0x10` | `u8` | "has entry" |
| `+0x399 + k*0x10` | `u8` | "suppress" |
| `+0x39C + k*0x10` | `f32` | score |
| `+0x3A0 + k*0x10` | `f32` | rate (delta per frame) |
| `+0x3D8` | `u8` | wheel-latch enable |
| `+0x3D9` | `u8` | wheel-latch suppress |
| `+0x3DC` | `s32` | set to `4` when the wheel is aligned |
| `+0x4E4 + i*0x0C` | request | `+0x0A` `s8` kind (0..3), `+0x0B` `s8` previous kind |
| `+0x514 + j*0x20` | slot | `+0x14` `f32`, `+0x18` `f32`, `+0x1C` `u8` busy, `+0x1D` `u8` kind |
| `+0x514` | `s16` | shared wheel angle (aliases slot 0 `+0x00`) |

### Exact behaviour

```
// phase 1 - claim slots for up to four requests
for i in 0..4 {
    let req = this + 0x4E4 + i*0x0C;
    let kind = *(s8*)(req + 0x0A);
    if (kind as u8) < 4 && kind != *(s8*)(req + 0x0B) {
        for j in 0..16 {
            let slot = this + 0x514 + j*0x20;
            if *(u8*)(slot + 0x1C) == 0 {
                *(u8*)(slot + 0x1C) = 1;
                *(u8*)(slot + 0x1D) = kind as u8;
                *(f32*)(slot + 0x14) = 0.0f;
                *(f32*)(slot + 0x18) = 0.0f;
                break;
            }
        }
    }
}

// phase 2 - for each of the four player records
for k in 0..4 {
    let rec = this + k*0x10;
    let id  = *(i32*)(rec + 0x394);
    if id == -1 || *(u8*)(rec + 0x398) == 0 || *(u8*)(rec + 0x399) != 0 { continue; }

    let score = *(f32*)(rec + 0x39C);
    let mut blocked = false;
    for j in 0..4 {
        if j == k { continue; }
        let other = this + j*0x10;
        if *(i32*)(other + 0x394) != -1 && *(i32*)(other + 0x394) != id
           && score < *(f32*)(other + 0x39C) {
            blocked = true;
        }
    }
    if blocked { continue; }

    if *(u8*)(this + 0x3D8) != 0 && *(u8*)(this + 0x3D9) == 0 {
        let angle = *(s16*)(this + 0x514) as f64;
        if angle.abs() <= 64.0 { *(s32*)(this + 0x3DC) = 4; }
    }

    let rate = *(f32*)(rec + 0x3A0);
    let delta: i32;
    if rate < 0.0f || 0.1f < rate {             // outside the dead zone
        if rate >= 0.0f || rate < -0.1f {
            let mut d = rate * (score / 100.0f) * 160.0f;
            if d > 480.0f { d = 480.0f; }
            delta = (i32)d;
        } else {
            delta = -16;                        // -0.1 <= rate < 0
        }
    } else {
        delta = 16;                             // 0 <= rate <= 0.1
    }
    *(s16*)(this + 0x514) =
        (*(s16*)(this + 0x514) as i32 + delta) as u16 as i16;
    // end of this record's body; the `b 0x80741264` is the loop latch
}
```

All four records are processed on every call. The latch is `0x80741264`
(`addi r8,r8,0x10`; `bdnz 0x807410C0` at `0x80741268`), reached from the blocked
test (`0x80741190`), the `+16` arm (`0x80741200`), the `-16` arm (`0x80741228`)
and the proportional fall-through at `0x80741260`. The function returns at
`0x80741270` only after `ctr` reaches 0. Ghidra's `return` at the end of the
record body is its `bdnz` reconstruction and must be written as "end of loop
body", not `return`.

The exact compiled condition tree for `rate` (from the `lfs`/`fcmpo`/`cror`
chain at `0x807411D8..0x80741200` and the export's `fVar4 = 0.0`,
`fVar6 = 0.1`, `fVar7 = -0.1`) is:

```
if (rate < 0.0f) || (0.1f < rate) {            // outside [-0.1, 0.1] on the upper side
    if (rate >= 0.0f) || (rate < -0.1f) { ...proportional... }
    else { delta = -16; }                       // -0.1 <= rate < 0
} else { delta = 16; }                          // 0 <= rate <= 0.1
```

Note the asymmetry: a *positive* rate in the dead zone takes `+16`, a negative
rate in the dead zone takes `-16`, and the proportional arm is reached only for
`rate > 0.1` or `rate < -0.1`.

`0x807CE0A0` is loaded with **`lfd`** (`0x807411A0`, `176(r7)`), and `fcmpo` is
untyped in the ISA - it always compares the FPRs as *doubles*. The 8-byte pool
entry at that address is `0x40500000 0x00000000`, i.e. the double **`64.0`**, so
the test is `|angle| <= 64.0` on the integer angle (a nine-step window at the
wheel's 16-unit granularity). The first four bytes of the same cell are also the
float `3.25f`, which is what a float-only reader (and Ghidra's `DAT_` comment)
sees; that value must **not** be used here. The `fsub`/`fabs` pair is the signed
`s16 -> f64` conversion idiom: `0x43300000 | (angle ^ 0x8000)` builds
`angle + 2^52 + 2^31`, and subtracting the 8-byte constant at `0x807CE0C0`
(`4503601774854144.0`) leaves `(double)angle`.

### Constants

| address | value | use |
|---|---:|---|
| `0x807CE08C` | `0.0f` | slot floats and the "no change" rate |
| `0x807CE0A0` | `64.0` (double) | wheel-aligned window (`lfd`; the first word is also `3.25f`) |
| `0x807CE0A8` | `0.1f` | upper dead-zone bound |
| `0x807CE0AC` | `-0.1f` | lower dead-zone bound |
| `0x807CE0B0` | `100.0f` | score normaliser |
| `0x807CE0B4` | `160.0f` | rate gain |
| `0x807CE0B8` | `480.0f` | gain clamp |
| `0x807CE0C0` | `4503601774854144.0` (`2^52 + 2^31`) | signed int -> double bias |

### Callees

None (leaf).

### Notes for the implementer

* `+0x514` is *both* the 16-slot table base and the `s16` wheel angle: slot 0's
  `+0x00` is the shared angle. Phase 1 only writes slot fields `+0x14..+0x1D`,
  so the overlap is safe, but a Rust struct must model it explicitly
  (`#[repr(C)] struct Slot { angle_lo: u16, ... }` or a separate `angle` field
  at the same offset).
* Phase 1 does **not** update the request's `+0x0B` byte; some other body does.
* The four "blocked" comparisons use the other records' scores, but the
  addresses are hard-coded per record (`+0x39C`, `+0x3AC`, `+0x3BC`, `+0x3CC`),
  which is the `j*0x10` form above.
* The `rate` arms must keep the exact order: `+16` for `0 <= rate <= 0.1`,
  `-16` for `-0.1 <= rate < 0`, proportional otherwise. Writing
  `if rate > 0 { ... }` collapses the dead zone into the proportional arm and
  changes two branches.
* All four records run every call; the `bdnz` at `0x80741268` is the only loop
  back-edge and `0x80741270` is the `blr`.

**Verdict: ready (high).** Leaf, no callees, no globals, every constant read
from the image, one `blr` at `address + size - 4`. The only inference is the
`bdnz` loop-shape reading above, which the instruction at `0x80741268` settles.

---

## 8. `0x8063A960` - four-record player-order permutation over `+0x2CC`

**Evidence.** `work/ghidra/rel-d_basesNP/export/8063a960_FUN_8063a960.c`,
604 bytes, 151 words, synthetic `0x8063A960`. Body `0x8063A960..0x8063ABB8`
(ends `blr`, exactly one; the `blr` at `0x8063ABD0` is a separate stub in the
36-byte gap). No `psq`, no `b` out of the body. Pointer-table entry:
`0x807E00D4` (inside the descriptor block that also lists `0x8063ABD0`,
`0x8063A950`, `0x8063ACC0`, ...). No direct `bl` caller.

**What it is.** The "build the four per-player records from the controller
order, then reorder them to `{0,1,3,2}`" initialiser that pairs with row 5. It
reads the order table `0x80354E60` that `0x807B2050` (row 4) writes, so the
three rows form one chain: 4 builds the order, 8 materialises the records, 5
reorders the *other* (`+0x7BC`) record block.

**Signature.**

```rust
/// Builds the four per-player records at `+0x2CC` and reorders them to
/// controller order `{0,1,3,2}`.
///
/// # Original
/// - Ghidra: `FUN_8063a960` at `0x8063A960` (604 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/8063a960_FUN_8063a960.c`
///
/// # Arguments
/// None; `this` is `r3` preserved by `0x802DCD64`.
///
/// # Returns
/// Nothing.
pub unsafe fn fn_8063a960()
```

### Field map - one record, stride `0x1C`, four records at `+0x2CC + id*0x1C`

| offset | type | meaning |
|---:|---|---|
| `+0x00` | `u8` | colour/character byte (from `player_base[id + 0x58]`) |
| `+0x04` | `u32` | from `player_base[i*4 + 0x18]` |
| `+0x08` | `u32` | from `player_base[i*4 + 0x28]` |
| `+0x0C` | `u32` | from `player_base[i*4 + 0x38]` |
| `+0x10` | `u32` | from `player_base[i*4 + 0x08]` |
| `+0x14` | `u32` | record id (0..3) - the sort key |
| `+0x18` | `u8` | flag A (cleared) |
| `+0x19` | `u8` | "occupied" (1 when the player exists) |

(The block spans `+0x2CC..+0x33B` for four records; the final copy is a
0x70-byte blit, so `+0x1A`/`+0x1B` are padding.)

### Exact behaviour

```
let this = r3;                              // preserved by 0x802DCD64
let base = *(u32*)0x80429FB0;                // player-record base

for i in 0..4 {
    let id = *(u32*)(0x80354E60 + i*4);      // order written by fn_807b2050
    let rec = this + 0x2CC + id*0x1C;
    *(u32*)(rec + 0x14) = id;
    if 0x800B4710(i) == 0 {
        *(u8*)(rec + 0x00) = 0;
        *(u32*)(rec + 0x04) = 0;
        *(u8*)(rec + 0x19) = 0;
        *(u32*)(rec + 0x08) = 0;
        *(u32*)(rec + 0x0C) = 0;
        *(u32*)(rec + 0x10) = 0;
    } else {
        *(u8*)(rec + 0x19) = 1;
        *(u32*)(rec + 0x04) = *(u32*)(base + i*4 + 0x18);
        *(u8*)(rec + 0x00)  = *(u8*)(base + id + 0x58);
        *(u32*)(rec + 0x08) = *(u32*)(base + i*4 + 0x28);
        *(u32*)(rec + 0x10) = *(u32*)(base + i*4 + 0x38);
        *(u32*)(rec + 0x0C) = *(u32*)(base + i*4 + 0x08);
    }
    *(u8*)(rec + 0x18) = 0;
    fn_8063a8c0(this, id);
}

// sort into a 0x70-byte temp by id order {0,1,3,2}
let mut tmp = MaybeUninit::<[u8; 0x70]>::uninit();
for k in 0..4 {
    let want = *(u32*)(0x807C7F70 + k*4);
    for i in 0..4 {
        let rec = this + 0x2CC + i*0x1C;
        if *(u32*)(rec + 0x14) == want {
            copy the fields at `+0x00`, `+0x04..+0x17`, `+0x18`, `+0x19`
            (offsets `0..0x19`; `+0x1A`/`+0x1B` are not copied)
            break;
        }
    }
}
// write back as two 0x38-byte blits (2 * 0x38 == 4 * 0x1C == 0x70)
copy 0x70 bytes from tmp to this + 0x2CC;

fn_8063a680(this);
fn_8063a850(this);
*(u8*)(this + 0x33E) = 0;
*(u8*)(this + 0x33D) = 1;
*(u8*)(this + 0x340) = 0;
let obj = this + 0x208;
(*(*(u32*)obj + 0x18))(obj, 0x80826EE0);
```

The final copy in the listing is `2 x 0x38` (`ctr = 2`, `addi r3,r3,0x38`), and
`2 * 0x38 = 0x70 = 4 * 0x1C`, so it is exactly the 0x70-byte blit above. The
eight field stores per 0x38 chunk are the compiler's unrolled byte copy, not a
different record size; do **not** model a 0x38 record.

### Constants

| address | value | use |
|---|---:|---|
| `0x807C7F70` | `0, 1, 3, 2` | target order |
| `0x80354E60` | `u32[4]` | source order (row 4's output) |
| `0x80429FB0` | `u32` | player-record base cell |
| `0x80826EE0` | object | second argument of vtable `+0x18` |
| - | `0x1C` (28) | record stride |
| - | `0x70` (112) | block size |

### Callees

`0x802DCD64`/`0x802DCDB0` (`__savegprlr_23`/`__restgprlr_23`), `0x800B4710`
(player-exists predicate), `0x8063A8C0`, `0x8063A680`, `0x8063A850`, vtable
slot `+0x18` on `this+0x208`.

### Notes for the implementer

* `this` comes from `r3` (helper), not from a return value.
* The sort copies only `0x1A` of each `0x1C`-byte record into the temp, then
  writes back `0x70` bytes; `+0x1A`/`+0x1B` therefore propagate whatever the
  stack held. Keep the `MaybeUninit` shape or zero-fill with an
  `// observed:` comment.
* `base + i*4 + 0x18` etc. are four consecutive words of a 4-byte-stride
  structure, i.e. the *i-th* player's fields, while `base + id + 0x58` is a
  byte array indexed by the *record id*. Keep both indexes distinct.
* `0x800B4710(i)` is called with the slot index `i`, not with `id`.

**Verdict: ready (medium).** All fields, constants and callees are literal;
medium because the 0x1A/0x1C temp copy is a shipped uninitialised-byte quirk and
`0x8063A8C0`/`0x8063A680`/`0x8063A850` stay address-only.

---

## Cross-checks performed

* Address mapping re-derived from both ELF program-header tables (quoted above);
  every literal was read through it. `0x8083xxxx` reads resolve through the REL
  BSS zero fill and are marked as BSS where they occur.
* `ledger/functions.csv` contains no row for any of the eight (checked against
  the full 17,102-address set, case-insensitive).
* No `#{2,4} ` heading of any `docs/SPEC_WAVE*_BASES.md` contains any of the
  eight; the only textual appearances are body prose in
  `docs/SPEC_WAVE151_BASES.md:149` (`0x807310C0`) and
  `docs/SPEC_WAVE152_BASES.md:1656` (naming it as the next candidate).
* `src/game/bases.rs` has no `fn fn_<addr>` definition for any of the eight;
  the four doc-comment appearances are listed in the selection section.
* Extent/`blr` recomputation for all eight is in the table near the top; row 4
  is the only short extent and is documented.
* Every constant table value was read from the image (`node ph.js`/`rdef.js`)
  or from `main.dol` through its own program headers. Prose citations are
  limited to the `GQR3 = 0x00050005` fact (`docs/REL_ELF_MAPPING.md:52-56`), the
  `fn_807b29a0`/`fn_807b2b10` cross-references in `src/game/bases.rs` and the
  table-`n` comparison with `docs/SPEC_WAVE148_BASES.md`.
* `src/game/bases.rs` is being edited by other lanes concurrently, so this
  document quotes its comments instead of citing line numbers.
* This lane wrote only this file.

