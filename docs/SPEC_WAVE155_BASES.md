# Wave 155 - `game::bases` - the next eight (584-576 byte band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified by this lane. The only file written is this one.
No merge or check tool was run. The throwaway tools live under
`%TEMP%\opencode\w155` (`pick.js`, `d.js`, `d2.js`, `cx.js`, `md.js`, `mdx.js`,
`who.js`, `callers.js`, `info.js`, `verify.js`); none of them is in the
repository. `d2.js` is `d.js` with one fix: a *conditional* `bclr`/`bctr`
(`BO != 20 || BI != 0`) no longer truncates the walk, it only ends one path
(`0x807BAE90` needs this, see row 3).

Waves 128-154 walked this bucket from the top in size order; the last row
claimed by wave 154 is 588 bytes, and that document's verification section names
the next two candidates explicitly (`docs/SPEC_WAVE154_BASES.md:180-183`, "the
ninth is `0x80695E00` (584) and the tenth is `0x8078F880` (584)"). This wave
starts with those two and ends at 576:

```
0x80695E00 584 | 0x8078F880 584 | 0x807BAC40 584 | 0x807BBE20 584
0x807649B0 580 | 0x807BE750 580 | 0x8075C6F0 576 | 0x807C22C0 576
```

## The eight

| # | address | TSV size | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x80695E00` | 584 | 146 | `80695e00_FUN_80695e00.c` | ready (high) |
| 2 | `0x8078F880` | 584 | 146 | `8078f880_FUN_8078f880.c` | ready (medium) |
| 3 | `0x807BAC40` | 584 | 146 | `807bac40_FUN_807bac40.c` | ready (medium) |
| 4 | `0x807BBE20` | 584 | 146 | `807bbe20_FUN_807bbe20.c` | ready (high) |
| 5 | `0x807649B0` | 580 | 145 | `807649b0_FUN_807649b0.c` | ready (medium) |
| 6 | `0x807BE750` | 580 | 145 | `807be750_FUN_807be750.c` | ready (medium) |
| 7 | `0x8075C6F0` | 576 | 144 | `8075c6f0_FUN_8075c6f0.c` | ready (medium) |
| 8 | `0x807C22C0` | 576 | 144 | `807c22c0_FUN_807c22c0.c` | ready (medium) |

**Ready: 8. Conditional: 0.** 4,648 bytes / 1,162 words in total.

Every `size` value above is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim and `decompiled` is
`true` for all eight. All eight extents end exactly on their own `blr`
(`address + size - 4`), so no row needs the `docs/REL_ELF_MAPPING.md:37-42`
extent correction:

| # | `blr` | `address + size - 4` | span is exact | next row | gap |
|--:|---|---|:--:|---:|---:|
| 1 | `0x80696044` | `0x80696044` | yes | `0x80696050` | 8 |
| 2 | `0x8078FAC4` | `0x8078FAC4` | yes | `0x8078FAD0` | 8 |
| 3 | `0x807BAE84` | `0x807BAE84` | yes | `0x807BAE90` | 8 |
| 4 | `0x807BC064` | `0x807BC064` | yes | `0x807BC090` | 40 |
| 5 | `0x80764BF0` | `0x80764BF0` | yes | `0x80764C00` | 12 |
| 6 | `0x807BE990` | `0x807BE990` | yes | `0x807BEA40` | 172 |
| 7 | `0x8075C92C` | `0x8075C92C` | yes | `0x8075C930` | 0 |
| 8 | `0x807C24FC` | `0x807C24FC` | yes | `0x807C25B0` | 176 |

Gap contents, read word by word:

* rows 1, 2, 3 and 5: only zero words (2, 2, 2 and 3 words);
* row 4: `0x807BC068..0x807BC08C` are zero words; the next row `0x807BC090`
  (196 bytes, the translated `fn_807bc090`) is the *third* handler of the
  following state (`dWorldSelect_c::StateID_OpenAnimeEndWait`, whose handlers are
  `0x807BC1F0`/`0x807BC160`/`0x807BC090` per
  `docs/SPEC_WAVE141_BASES.md:241`), see §4;
* row 6: `0x807BE994..0x807BE99C` are zero words, then two untranslated
  one-function bodies `0x807BE9A0` (calls `0x80101940(*(u32*)(this+0x4A8), 0)`,
  returns 1) and `0x807BE9D0` (a sound/`0x801015E0` variant of row 6). Neither is
  part of this function;
* row 8: `0x807C2500` is a one-word `blr` stub (it is also an entry of the same
  state table), `0x807C2510` is a second `blr` stub, the rest is zero words.

The decoder reports **no unknown word** in the 1,162 words: zero unknown
opcodes, zero unhandled X-form, zero unhandled paired-single form, zero branch
leaving its own body, and every `bl` lands on a named callee outside the body.
`psq` appears in three rows only, always as a prologue/epilogue `f31`/`f30`
save at `W = 0`, `qr0` - i.e. plain `f32` stack traffic with no data-side
quantisation:

| row | `psq` words |
|--:|---|
| 1 | 2 `psq_st` + 2 `psq_l` (`f31` at 72, `f30` at 56), all `W = 0`, `qr0` |
| 5 | 1 `psq_st` + 1 `psq_l` (`f31` at 104), `qr0` |
| 6 | 2 `psq_st` + 2 `psq_l` (`f31` at 120, `f30` at 104), `qr0` |
| 2, 3, 4, 7, 8 | none |

No row reads or writes a `qr3` value, so the wave-138 `GQR3 = 0x00050005` pin is
not needed by anything in this set.

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

Two rows deserve a sentence each, because a reader may expect a condition:

* Row 3's scan loop is guarded by `strlen(name) == 14`. That looks like a
  phantom condition until the two name formats are compared:
  `W%X/pointW%X.csv` is **14** characters for a one-digit world index and
  `W%X/pointW%Xa.csv` is **15**. The guard therefore means "the no-letter file
  was the one that existed, so there is no `a`/`b`/... sequence - stop", and the
  `a`,`b`,`c`... walk is entered only after the `a` file was found. Both
  behaviours are spelled out in §3; nothing is inferred.
* Row 1's `0x80695E00` was rejected by wave 88 for a "GQR0/`ldexpf`" body
  (`docs/COVERAGE_WAVE88_BASES.md:114` and `:123`). The prologue at
  `0x80695E10`/`0x80695E18` is `stfd f31` + `psq_st f31,...,0,qr0` (and the
  same for `f30`), i.e. the `qr0` register-pair save that
  `docs/ANALYSIS_WAVE127_BASES.md:50-59` already classifies as reconstructible.
  The rejection is now false and the row is *high*, not conditional.

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
is this REL's BSS and reads as zero at file level; `0x8082B320`, `0x808340F8`,
`0x80834138`, `0x80834460..0x80834D40` are all in it.

`main.dol`-resident data and code are read from `work/dol/main.elf` through
**its own** program headers:

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

so `0x803504E0` (row 6) and `0x803741F0`/`0x80374880`/`0x803748A8` (row 3) are
**file** data, while `0x80377DC8` (row 6), `0x80354E60`/`0x80354E70` (row 8),
`0x8042A038`, `0x8042A040`, `0x8042A234`, `0x8042A24D`, `0x8042A24F`,
`0x8042A488`, `0x8042A4A8`, `0x8042AF08` are `main.dol` **BSS** and read as
zero at file level. `0x80377C48` (rows 2, 5, 7) is `main.dol` BSS too.

**Calling-convention notes used by several rows.**

* `0x802DCD58`, `0x802DCD5C`, `0x802DCD70`, `0x802DCD74` are MWCC
  `__savegprlr_N` helpers (`_21`, `_22`, `_26`, `_27`) and
  `0x802DCDA4`/`0x802DCDA8`/`0x802DCDBC`/`0x802DCDC0` the matching
  `__restgprlr_N`; they preserve `r3..r10` and return nothing. Where the export
  prints `iVar = func_0x802dcd7x()` and then uses that "return" as `this`, the
  translation must take `this` from `r3`.
* `0x801680E0(value, element)`, `0x80168FE0(value, element)`,
  `0x80167B20(value, element, index)` and `0x80167B40(value, element, index)`
  all carry their value in **`f1`** (rows 2, 5, 7). The `resource::registry`
  translations of the last three drop that float and keep only
  `(element, index)`; the shipped bodies prove it is live:
  `0x80168FE0` stores `f1` at `element + 0x30` and `0x80167B40` /
  `0x80167B20` tail into `0x801680E0` / `0x80168FE0` after
  `element + *(u32*)(element + 0x28) + index * 0x38`.
* `0x8016A3A0(this, src, group, kind, arg4, out)` stores the new handle at
  `this + 4`; `kind != 0` selects the three-argument factory `0x8025CA20`,
  `kind == 0` the four-argument `0x8025A380` (rows 2, 5, 7 - all three pass a
  non-zero kind).
* `0x80169D90` is the six-argument sibling of `0x8016A3A0` used by row 5
  (`(holder, &model, heap, 0x20, 1, 0)`).

**Decoder note.** This lane used `%TEMP%\opencode\w155\d2.js`, a fresh copy of
the wave-154 decoder with the conditional-`bclr` fix described at the top.
`neg`/`addme`/`addze`/`subfme`/`subfze`/`mfcr` print the destination first
(row 4 contains `neg r0,r3` at `0x807BBFC0`); `psq_l`/`psq_st` are opcodes 56/57
and 60/61 with the quantiser in bits 12-14 exactly as
`docs/REL_ELF_MAPPING.md:46-52` states. Sizes are always copied from the TSV,
never inferred from a `blr` position.

## Candidate selection (recomputed)

`%TEMP%\opencode\w155\pick.js` re-reads four sources from scratch:

* `work/ghidra/rel-d_basesNP/export/functions.tsv` - 5,615 data rows, all
  `decompiled = true`, and all 5,615 export files named in the `file` column
  exist on disk;
* `ledger/functions.csv` - 17,170 distinct `address` fields (case-insensitive);
* `src/game/bases.rs` - every `fn fn_<addr>` definition token (3,771 at the
  first selection pass, 3,782 at the verification pass; the difference is the
  eleven bodies the concurrent wave-155 coverage pass added,
  `docs/COVERAGE_WAVE155_BASES.md:15-17`, none of them in this set);
* every `#{2,4} ` heading of **all** 27 `docs/SPEC_WAVE*_BASES.md` files that
  existed before this one (278 distinct `0x` addresses). The task names
  `SPEC_WAVE12*_BASES.md`, of which only waves 128 and 129 exist; the superset is
  used so that no address already claimed by a wave-130..154 heading can be
  picked twice.

That leaves **1,815** rows. The eight largest are the table above. Ties are
broken by **ascending address**; this is the only place the ordering is not
forced by size, and it is stated so wave 156 can reproduce it:

* the 584-byte group is
  `{0x80695E00, 0x8078F880, 0x807BAC40, 0x807BBE20}` - exactly four rows, so all
  four are claimed here;
* the 580-byte group is `{0x807649B0, 0x807BE750}`;
* the 576-byte group is `{0x8075C6F0, 0x807C22C0}`;
* the 572-byte group is `{0x806DB340, 0x807041B0}` - so **the ninth is
  `0x806DB340` (572)** and the tenth is `0x807041B0` (572), neither in the
  ledger, the Rust file or any spec heading;
* the 568-byte group is
  `{0x80745DC0, 0x8077D340, 0x80788090, 0x8079B7C0}`. Note `0x8079B7C0` is the
  angle metric that `docs/SPEC_WAVE154_BASES.md:522-531` decodes *in passing* as
  row 2's callee but never claims as a row; it is still unclaimed and still
  available.

No `0x806DB340`/`0x807041B0` input is lost by deferring them; they are simply
the next two rows of the same band.

None of the eight is referenced by a `fn fn_<addr>` body or by any spec
heading. The only prose appearances are the previous lanes' skip lists:

* `0x80695E00` - `docs/COVERAGE_WAVE88_BASES.md:114` and `:123` (rejected for
  the `qr0` prologue, now disproved) and `docs/SPEC_WAVE154_BASES.md:180-181`
  (named as the ninth candidate);
* `0x8078F880` - `docs/COVERAGE_WAVE115_BASES.md:104` and
  `docs/SPEC_WAVE154_BASES.md:180-181`;
* `0x807BAC40` - `docs/COVERAGE_WAVE143_BASES.md:122`,
  `docs/COVERAGE_WAVE152_BASES.md:133`, `docs/SPEC_WAVE154_BASES.md:180,185`;
* `0x807BBE20` - `docs/COVERAGE_WAVE103_BASES.md:129`,
  `docs/COVERAGE_WAVE143_BASES.md:122`,
  `docs/COVERAGE_WAVE152_BASES.md:133`,
  `docs/SPEC_WAVE141_BASES.md:240` (the `dWorldSelect_c` state table, see §4);
* `0x807BE750` - `docs/COVERAGE_WAVE143_BASES.md:122`,
  `docs/COVERAGE_WAVE152_BASES.md:133`;
* `0x8075C6F0` - `docs/COVERAGE_WAVE78_BASES.md:133,140,152` ("the `cobBoard`
  loader, 576 bytes") and `docs/COVERAGE_WAVE144_BASES.md:140`;
* `0x807649B0` and `0x807C22C0` - no prior mention anywhere.

The implementation pass of the *previous* wave's eight ran concurrently with this
lane and reports the same starting point:
`docs/COVERAGE_WAVE155_BASES.md:15-17` adds 11 bodies to `game::bases`
("3,771 before this pass + 11"), and its "Deliberate skips" section
(`:124-128`) says of this wave's first two rows, "`0x80695E00` and `0x8078F880`,
584 bytes each ... Both remain untranslated and fully specified". So the eight
rows here are still open at the time of writing; the verification pass
re-counted 3,782 `fn fn_<addr>` tokens and found none of the eight among them.

---

## 1. `0x80695E00` - `daIntermittent_c` Splush climb body

**Evidence.** `work/ghidra/rel-d_basesNP/export/80695e00_FUN_80695e00.c`,
584 bytes, 146 words, synthetic `0x80695E00`. Body `0x80695E00..0x80696044`
(ends `blr`, exactly one). Frame: `stwu r1,-80(r1)`, manual
`stfd f31`/`psq_st f31`, `stfd f30`/`psq_st f30`, `stw r31..r28`. No jump table,
no indirect call. The 8-byte gap after the extent is two zero words and the next
row is `0x80696050` (132 bytes, the translated `fn_80696050`).

**What it is.** The `Splush` state body of `daIntermittent_c` - the sand-pillar
actor whose other bodies wave 88/154 already translated (`fn_806957b0`,
`fn_80695830`, `fn_80695990`, `fn_80695a20`, `fn_80695b20`, `fn_80695c20`,
`fn_80696050`, `fn_806960f0`). The class name comes from the state-name strings
that follow the handler table at `0x807F0974`:

```
0x807F0974 "daIntermittent_c::StateID_Appear"
0x807F09AA "StateID_Wait"
0x807F09CA "StateID_SplushBegin"
0x807F09F2 "StateID_Splush"
0x807F0A16 "StateID_SplushEnd"
```

and this body is the third handler of the record at `0x807F0938`
(`0x807F0938 = {0, -1, 0x80695E00}`, part of the `{offset, index, target}`
12-byte table that starts at `0x807F08C0`, whose `0x802DCBFC`-style descriptor
form is documented in `docs/SPEC_WAVE154_BASES.md:1441-1461`). Its siblings in
that table are `0x80695950`, `0x80695960`, `0x80695830`, `0x80695990`,
`0x80695A20`, `0x80695B20`, `0x80695DF0`, `0x80695DC0`, `0x806960E0`,
`0x806960F0`, `0x80696050`, and `0x80695E00` is called from none of them by
`bl` - the engine's state runner invokes it through the table (no `bl` caller
and no data word references the table, so the runner is outside this REL).

It is the per-frame half of the climb: it re-commits the `+0x434` clip, re-emits
the three `sandpillar`/`spillarsign` effects, fires SE `0x308`, integrates the
climb speed through `fn_80695760`, adds the speed to the height `+0xE0`, applies
the two limit rules, and runs the two-state phase word `+0x360`
(`1` = counting down `+0x534`, `2` = armed).

**Signature.**

```rust
/// Advances the `daIntermittent_c` Splush climb and steps its phase word.
///
/// # Original
/// - Ghidra: `FUN_80695e00` at `0x80695E00` (584 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/80695e00_FUN_80695e00.c`
///
/// # Arguments
/// * `actor` - the `daIntermittent_c` sand pillar; clip holder `+0x434`,
///   effect holders `+0x12F8`/`+0x1420`/`+0x1548`, effect anchor `+0x1290`,
///   height `+0xE0`, speed `+0x538`, acceleration `+0x53C`, limit `+0x540`,
///   phase `+0x360`, countdown `+0x534`, interface `+0x394`.
///
/// # Returns
/// Nothing (the last `bctrl`'s `r3` is whatever the interface left there).
///
/// # Safety
/// `+0x434`, `+0x12F8`, `+0x1420`, `+0x1548` and `+0x394` must expose valid
/// vtables, and the sound-manager cell `0x80429D60` must hold a live manager.
pub unsafe fn fn_80695e00(actor: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x0AC`..`+0x0B4` | vec3 | actor position (x/y used, z replaced by `5500.0`) |
| `+0x0E0` | `f32` | climb height, `+= +0x538` every frame |
| `+0x360` | `u32` | phase word: `1` = countdown, `2` = armed |
| `+0x394` | object | state interface; vtable slot `+0x18` gets a state record |
| `+0x434` | object | clip/model holder (vtable `+0x14` = "commit") |
| `+0x534` | `s32` | countdown, `120` at entry (set by `0x80695DC0`) |
| `+0x538` | `f32` | climb speed, `0.017` at entry |
| `+0x53C` | `f32` | climb acceleration, `0.00015` at entry |
| `+0x540` | `f32` | climb speed limit (`0.0` at entry) |
| `+0x1290` | object | effect anchor block handed to the two `sandpillar` emits |
| `+0x12F8` | object | `Wm_ob_sandpillar01` effect holder |
| `+0x1420` | object | `Wm_ob_sandpillar02` effect holder |
| `+0x1548` | object | `Wm_ob_spillarsign02` effect holder |

Globals: `0x80429D60` (sound manager pointer cell), `0x8082B320` (this REL's
BSS, the 0x40-byte state record handed to `+0x394`'s slot `+0x18`).

### Exact behaviour

```
// 1. commit the clip holder
vmethod0(actor + 0x434, 0x14);

// 2. re-emit the three effects, all with "name, 0, position, 0, 0"
emit(actor + 0x12F8, 0x807F0A70 /* "Wm_ob_sandpillar01" */, actor + 0x1290);
emit(actor + 0x1420, 0x807F0A5C /* "Wm_ob_sandpillar02" */, actor + 0x1290);

let mut sign_pos = [
    *(f32*)(actor + 0xAC),
    *(f32*)(actor + 0xB0),
    *(f32*)0x807C9F10,          // 5500.0
];
emit(actor + 0x1548, 0x807F07C8 /* "Wm_ob_spillarsign02" */, &sign_pos);

// 3. one positional event, id 0x308, context word = the actor handle
let mut sample = [0u8; 40];
0x8006A4B0(sample.as_mut_ptr(), actor + 0xAC);
0x80199390(*(u32*)0x80429D60, 0x308, *(u32*)actor, sample.as_ptr(), 0);

// 4. climb
let old_height = *(f32*)(actor + 0xE0);
let old_speed  = *(f32*)(actor + 0x538);

fn_80695760(actor);             // if +0x538 < +0x540 { +0x538 += +0x53C }

let speed = *(f32*)(actor + 0x538);
*(f32*)(actor + 0xE0) = old_height + speed;

let mut flag = 0u32;            // 0x80695F2C `li r4,0`
if speed == 0.0 {
    if old_speed > 0.0 {
        *(f32*)(actor + 0x540) = -*(f32*)0x807C9E9C;   // -0.017
        flag = 1;
    } else if old_speed < 0.0 {
        *(f32*)(actor + 0x540) = *(f32*)0x807C9E9C;    // +0.017
    }
}

let height = *(f32*)(actor + 0xE0);
let ceiling = *(f32*)0x807C9E98;                       // 0.95
if old_height < ceiling && height >= ceiling {
    *(f32*)(actor + 0x540) = 0.0;
} else if old_height > ceiling && height <= ceiling {
    *(f32*)(actor + 0x540) = 0.0;
}

// 5. phase word
let phase = *(u32*)(actor + 0x360);
if phase == 1 {
    let left = *(i32*)(actor + 0x534) - 1;
    *(i32*)(actor + 0x534) = left;
    if left == 0 {
        *(u32*)(actor + 0x360) = 2;
    }
} else if phase == 2 && flag != 0 {
    vmethod1(actor + 0x394, 0x18, 0x8082B320);
}
```

`emit(holder, name, pos)` is the element vtable slot `+0xA0` used by every
sand-pillar body:

```
let vt = *(u32*)(holder);
(let f = *(u32*)(vt + 0xA0))(holder, name, 0, pos, 0, 0);
```

Two details the listing settles:

* `flag` is a **local**, not an argument. `0x80695F2C` is `li r4,0`; the only
  other write is the `li r4,1` in the `old_speed > 0` arm at `0x80695F54`. The
  Ghidra print `bVar2 = false` is therefore correct, but a translator who reads
  `r4` as an incoming register (as one might after the `__savegprlr` prologue)
  would add a phantom second argument.
* The two `+0x540 = 0.0` stores are *separate* arms, not a clamp: the first
  fires on the upward crossing (`old < 0.95 && new >= 0.95`), the second on the
  downward one. Both comparisons are `fcmpo` + `cror`, i.e. NaN makes both arms
  false; plain `>=`/`<=` on `f32` in Rust reproduces this because a NaN compare
  yields `false` for both.

### Constants

| address | value | use |
|---|---|---|
| `0x807C9E90` | `0.0f` | the three zero tests |
| `0x807C9E98` | `0.95f` | the crossing threshold |
| `0x807C9E9C` | `0.017f` | the limit magnitude written on the "speed hit zero" edge |
| `0x807C9F10` | `5500.0f` | z of the `spillarsign02` emit |
| `0x807F0A70` | string | `Wm_ob_sandpillar01` |
| `0x807F0A5C` | string | `Wm_ob_sandpillar02` |
| `0x807F07C8` | string | `Wm_ob_spillarsign02` |
| `0x8042_9D60` | cell | sound manager |
| `0x8083_2B20` (`0x8082B320`) | BSS record | `+0x394` slot `+0x18` argument |
| - | `0x308` (776) | sound/event id |
| - | `-0.017` | the negative limit (negated `0x807C9E9C`) |

The acceleration `+0x53C` is `0x807C9EA4 = 0.00015f` and the limit `0x807C540`
seed `0.0` come from the state entry body `0x80695DC0`, not from this body; they
are listed here because a reader of §1 alone would otherwise miss them:

| address | value | set by |
|---|---|---|
| `0x807C9EA4` | `0.00015f` | `0x80695DC0` writes it to `+0x53C` |
| `0x807C9E90` | `0.0f` | `0x80695DC0` writes it to `+0x540` |
| - | `120` | `0x80695DC0` writes it to `+0x534` |
| - | `1` | `0x80695DC0` writes it to `+0x360` |

**Reachability note.** With the shipped seeds (`+0x538 = 0.017`,
`+0x53C = 0.00015`, `+0x540 = 0.0`) `fn_80695760` never changes `+0x538`
because `0.017 >= 0.0`, so the `speed == 0.0` arms - and with them the
`flag = 1` path into the phase-2 dispatch - never fire in this state. They are
kept in the binary (and must be kept in the translation) because `+0x538`,
`+0x53C` and `+0x540` are shared with `fn_80695b20`/`fn_80695c20`, which write
`0.017`/`0.0008`/`0.017` and `0.25 * 0.017` respectively.

### Callees

`0x8006A4B0` (sample build, 40-byte result), `0x80199390`
(`(manager, id, context, sample, 0)`, the same 5-argument spawn
`fn_80695c20` uses for `0x308`), `0x80695760` (`fn_80695760`, the 44-byte
climb integrator: `if +0x538 < +0x540 { +0x538 += +0x53C }`), and the five
vtable slots `+0x14` of `+0x434`, `+0xA0` of `+0x12F8`/`+0x1420`/`+0x1548`
and `+0x18` of `+0x394`.

### Notes for the implementer

* `this` comes from `r3`, and `r3` is the *only* incoming register this body
  uses: the prologue reaches `li r4,0` at `0x80695F2C` without reading `r4`, so
  the signature has exactly one parameter (the "`bVar2 = false`" in the export is
  a local, not an argument).
* Steps 1-4 run **unconditionally**; step 5 is the only gated part.
* `old_height`/`old_speed` are read **before** `fn_80695760` and are reused
  after it (`f31`/`f30`); do not re-read `+0x538` for `old_speed`.
* The phase-1 arm stores the decremented value even when it does not reach 0,
  and stores `2` only on the exact `0` result (`stw` then `bne`).
* `0x8082B320` is BSS and reads as zero in the image; take it as an opaque
  0x40-byte state record whose *address* is the argument, exactly as
  `fn_80695a20`, `fn_80695c20` and `fn_806960f0` pass `0x8082B2A0`,
  `0x8082B2E0` and `0x8082B260`.

**Verdict: ready (high).** All 146 words decoded, the three floats and the three
strings read, the `qr0`-only prologue explained (the wave-88 rejection is
resolved), and the one soft spot - the `flag` local that the export prints as an
uninitialised `bool` - is called out with its two instructions.

---

## 2. `0x8078F880` - `CS_W5_Ibara` model/animation loader

**Evidence.** `work/ghidra/rel-d_basesNP/export/8078f880_FUN_8078f880.c`,
584 bytes, 146 words, synthetic `0x8078F880`. Body `0x8078F880..0x8078FAC4`
(ends `blr`, exactly one; the 8-byte gap is two zero words and the next row is
the 176-byte `0x8078FAD0`). No `psq`, no jump table. Frame:
`stwu r1,-48(r1)` with manual `stw r31..r28`. Its only caller is
`0x8078F6DC`, inside the **untracked hole** `0x8078F5AC..0x8078F880` (the
initialiser at `0x8078F670` that clears `+0x21C..+0x25C` to `-1`, calls
`0x8078FC80`, `0x8078FF30`, this body, `0x807901E0`, then destroys
`+0x1A4` with `0x8016A340`).

**What it is.** The model + animation loader of the course-5 briar actor
`CS_W5_Ibara` (`g3d/model.brres` member `CS_W5_Ibara` in archive `CS_W5`). It
creates the per-actor NW4R allocator, loads the model, binds one of five
`CS_W5_Ibara_XY` transition clips selected by `+0x214`, binds the matching
texture-SRT animation, and seeds the two animation rates from `+0x1D8` unless
the byte at `+0x218` asks for `0.0`. The five clip names in the table at
`0x8081A938` are `AB`, `AC`, `BC`, `CA`, `CB`.

**Signature.**

```rust
/// Loads the `CS_W5_Ibara` model and binds its variant clip and texture-SRT
/// animation.
///
/// # Original
/// - Ghidra: `FUN_8078f880` at `0x8078F880` (584 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/8078f880_FUN_8078f880.c`
///
/// # Arguments
/// * `actor` - the briar actor; allocator `+0x188`, model `+0x1A4`, clip
///   animation `+0x1B0`, visible byte `+0x1E4`, texture-SRT animation `+0x1E8`,
///   variant `+0x214`, rate selector `+0x218`, rate `+0x1D8`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `+0x1A4`/`+0x1B0`/`+0x1E8` must expose a vtable with slot `+0x18`, the
/// resource cell `0x8042A038` must hold a live archive table, and `+0x214` must
/// be in `0..5`.
pub unsafe fn fn_8078f880(actor: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x188` | allocator | per-actor NW4R allocator (5 words used) |
| `+0x1A4` | model holder | NW4R model object (created from `&model`) |
| `+0x1B0` | anim holder | `AnmObjChr` bound to `+0x1A4` |
| `+0x1D8` | `f32` | rate used when `+0x218 == 0` |
| `+0x1E4` | `u8` | set to `1` (visible / initialised flag) |
| `+0x1E8` | anim holder | `AnmObjTexSrt` bound to `+0x1A4` |
| `+0x214` | `u32` | variant index `0..4` into `0x8081A938` |
| `+0x218` | `u8` | `0` = use `+0x1D8` as the rate, non-zero = `0.0` |

Globals: `0x8042A038` (archive-table cell; `+4` is the `dRes` archive table),
`0x80377C48` (`main.dol` BSS word handed to the allocator as its parent).

### Exact behaviour

```
0x800690C0(actor + 0x188, -1, *(u32*)0x80377C48, 0, 0x20);

let res       = 0x800DF180(*(u32*)0x8042A038 + 4, 0x8081A960 /* "CS_W5" */,
                           0x8081A950 /* "g3d/model.brres" */);
let variant   = *(u32*)(actor + 0x214);
let clip_name = *(u32*)(0x8081A938 + variant * 4);

let model = 0x80239E30(&res, 0x8081A968 /* "CS_W5_Ibara" */);
0x8016A3A0(actor + 0x1A4, &model, actor + 0x188, 0x224, 1, 0);

let chr = 0x8023A0B0(&res, clip_name);
0x801650D0(actor + 0x1B0, &model, &chr, actor + 0x188, 0);
vmethod1(actor + 0x1A4, 0x18, actor + 0x1B0);      // model <- chr
*(u8*)(actor + 0x1E4) = 1;
0x801680E0(0.0, actor + 0x1B0);
0x80168FE0(0.0, actor + 0x1B0);

let tex = 0x8023A270(&res, clip_name);
0x80167420(actor + 0x1E8, &model, &tex, actor + 0x188, 0, 1);
vmethod1(actor + 0x1A4, 0x18, actor + 0x1E8);      // model <- tex-srt
0x80167B60(actor + 0x1E8, 1, 0);
0x80167B40(0.0, actor + 0x1E8, 0);
0x80167B20(0.0, actor + 0x1E8, 0);

// kept call whose result is discarded (0x8078FA14 -> 0x8078FA18)
let _ = 0x8023A270(&res, clip_name);

if *(u8*)(actor + 0x218) == 0 {
    0x801680E0(0.0, actor + 0x1B0);
    0x80168FE0(*(f32*)(actor + 0x1D8), actor + 0x1B0);
    0x80167B40(0.0, actor + 0x1E8, 0);
    0x80167B20(*(f32*)(actor + 0x1D8), actor + 0x1E8, 0);
} else {
    0x801680E0(0.0, actor + 0x1B0);
    0x80168FE0(0.0, actor + 0x1B0);
    0x80167B40(0.0, actor + 0x1E8, 0);
    0x80167B20(0.0, actor + 0x1E8, 0);
}

0x800F2A10(actor + 0x1A4);
0x800690E0(actor + 0x188);
```

`vmethod1(actor + 0x1A4, 0x18, anim)` is
`(*(*(u32*)(actor + 0x1A4) + 0x18))(actor + 0x1A4, anim)`; both binds use it.

### Constants

| address | value | use |
|---|---|---|
| `0x8081A950` | `"g3d/model.brres"` | archive member path |
| `0x8081A960` | `"CS_W5"` | archive name |
| `0x8081A968` | `"CS_W5_Ibara"` | model name |
| `0x8081A938` | `u32[5]` | pointer table `{0x8081A8E8, 0x8081A8F8, 0x8081A908, 0x8081A918, 0x8081A928}` |
| `0x8081A8E8` | `"CS_W5_Ibara_AB"` | variant 0 clip |
| `0x8081A8F8` | `"CS_W5_Ibara_AC"` | variant 1 |
| `0x8081A908` | `"CS_W5_Ibara_BC"` | variant 2 |
| `0x8081A918` | `"CS_W5_Ibara_CA"` | variant 3 |
| `0x8081A928` | `"CS_W5_Ibara_CB"` | variant 4 |
| `0x807CFD8C` | `0.0f` | the value passed to all six `f1` calls |
| `0x8042_A038` | cell | archive table (`+4` is the `dRes` table) |
| `0x8037_7C48` | `u32` (BSS) | allocator parent word |
| - | `0x224` (548) | `0x8016A3A0` kind word (non-zero -> 3-argument factory) |
| - | `5` | length of the variant table (not bounded by the body) |

The table is five words long: `0x8081A930` already belongs to the string
`CS_W5_Ibara_CB`, so a variant above 4 reads a string tail and must not be
modelled as a bounded array. The body performs no range check.

### Callees

`0x800690C0`/`0x800690E0` (the per-actor NW4R allocator create/destroy pair -
thin tail calls to `0x8016A7F0`/`0x8016A8E0`, with the second argument reduced
to `(arg != -1)`), `0x800DF180` (`resource::bres::fn_800df180`), `0x80239E30`
(`resource::bres::fn_80239e30`), `0x8016A3A0`
(`resource::registry::fn_8016a3a0`), `0x8023A0B0`
(`resource::bres::brres_get_anm_chr`), `0x8023A270`
(`resource::bres::brres_get_anm_tex_srt`), `0x801650D0`
(`fn_801650d0`), `0x80167420` (`fn_80167420`), `0x801680E0`/`0x80168FE0`/
`0x80167B60`/`0x80167B40`/`0x80167B20` (animation element helpers, see the
Method note), `0x800F2A10` (tail call `0x800B3F50(holder, 0)`, the kind-1
effect-submit pass of `game::stage`), and the `+0x18` slot of `+0x1A4`.

### Notes for the implementer

* `0x80239E30`, `0x8023A0B0` and `0x8023A270` take the **address of the word
  holding the resource pointer** as their first argument; here that word is the
  local `-0x1C(r1)` (copy of `0x800DF180`'s result), while `+0x1A4` is passed
  directly to `0x8016A3A0`/`0x801650D0`.
* The discarded `0x8023A270` call at `0x8078FA14` is real (the compiler kept it
  because the callee is not `const`); keep it and drop the result.
* The `f1 = 0.0` loads (`lfs f1,-628(r31)` with `r31 = 0x807CFD8C`) appear
  before `0x801680E0`, `0x80167B40` and `0x80167B20`; the value is passed
  through to the animation object's `+0x28` slot. Do not drop it because the
  `resource::registry` signature omits the float.
* `+0x224` (548) is the *kind* word of `0x8016A3A0`; it is non-zero, so the
  three-argument factory branch `0x8025CA20` runs. The siblings pass `0x20`
  (row 5) and `0x40` (row 7) for the same reason; the value is not a buffer
  length.

**Verdict: ready (medium).** All 146 words decoded, all five strings and the
only float read, and every callee named; medium because the three
`0x80069xxx`/`0x800F2Axx` helpers are thin unledgered tail calls and because
`0x8016A3A0`'s `kind` word is documented by its shape rather than by a symbol.

---

## 3. `0x807BAC40` - world `RouteInfo` CSV reader

**Evidence.** `work/ghidra/rel-d_basesNP/export/807bac40_FUN_807bac40.c`,
584 bytes, 146 words, synthetic `0x807BAC40`. Body `0x807BAC40..0x807BAE84`
(ends `blr`, exactly one; the 8-byte gap is two zero words and the next row is
the 120-byte `0x807BAE90`, decoded in §3.1). Frame: `stwu r1,-112(r1)`,
`__savegprlr_21`, epilogue `__restgprlr_21`. One `bl` caller: `0x807BA8F0`
(inside the 184-byte `0x807BA860`).

**What it is.** The loader that reads the world's course-point list. It formats
`W%X/pointW%X.csv` from the 1-based world number, falls back to
`W%X/pointW%Xa.csv`, and then walks `a`, `b`, `c`, ... until a letter file is
missing. Each file is a CSV whose records are separated by `\n` and whose first
field is a four-character token; the body classifies the token, counts the
`W?<other>` ones, and sets two bit tables per world.

**Signature.**

```rust
/// Reads the `RouteInfo` point files of one world and folds their tokens into
/// the per-world flag tables.
///
/// # Original
/// - Ghidra: `FUN_807bac40` at `0x807BAC40` (584 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807bac40_FUN_807bac40.c`
///
/// # Arguments
/// * `owner` - forward to `0x807BAE90` as its `r3`; that callee ignores it, so
///   it is a context pointer kept for signature compatibility.
/// * `world` - 0-based world index; the file name uses `world + 1`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `0x8042A038` must hold a live archive table and the two `main.dol` BSS
/// tables (`0x80374880`, `0x803748A8`) plus `0x803741F0` must be writable.
pub unsafe fn fn_807bac40(owner: usize, world: u32)
```

### Field map

There is no actor; the body only uses the two arguments and the globals.

| global | type | meaning |
|---|---|---|
| `0x8042A038` | cell | archive table; `+4` is passed as `this` to `dRes` |
| `0x803741F0` | `u32[?]` (BSS) | per-world flag words, `0xA8`-byte rows, bit `mask` at word `index` |
| `0x80374880` | `u32[?]` (BSS) | per-world "class-3 token" count, one word per world |
| `0x803748A8` | `u32[?]` (BSS) | per-world "files read" counter, one word per world |
| `0x8042A24F` | `u8` (BSS) | gate byte for both writers (`r13 - 0x5431`) |

### Exact behaviour

```
let file = *(u32*)0x8042A038;
let mut name = [0u8; 32];
let mut size = 0u32;                        // r1+8, written by dRes

format(name.as_mut_ptr(), 0x20, 0x80820B90 /* "W%X/pointW%X.csv" */,
       world + 1, world + 1);
let mut data = 0x800DF440(file + 4, 0x80820BA4 /* "RouteInfo" */,
                          name.as_ptr(), &mut size);
if data == 0 {
    format(name.as_mut_ptr(), 0x20, 0x80820BB0 /* "W%X/pointW%Xa.csv" */,
           world + 1, world + 1);
    data = 0x800DF1E0(file + 4, "RouteInfo", name.as_ptr(), &mut size);
}
if data == 0 { return; }

let mut letter = 0u8;        // r23
let mut hits   = 0i32;       // r25
loop {
    *(u32*)(0x803748A8 + world * 4) += 1;

    let mut at = 0usize;     // r24
    let mut end = 0u32;      // r22
    loop {                                          // one file
        // 1. skip to the next ',' (or to the end)
        loop {
            if at >= size { end = 1; break; }
            let c = *(u8*)(data + at);
            at += 1;
            if c == b',' { break; }
        }
        if end != 0 { break; }

        // 2. four-character token, NUL-terminated
        let mut token = [0u8; 5];
        // 0x802E19F8 = bounded copy that stops at a NUL; the CSV field
        // never contains one, so this is the first four bytes of the field.
        copy_until_nul_or_4(token.as_mut_ptr(), data + at, 4);
        token[4] = 0;

        // 3. classification
        if 0x800F9970(token.as_ptr()) == 3 { hits += 1; }
        if token[0] == b'W' && token[2] == b'W' {
            0x801026D0(world, 0x23, 0x40);
        }
        let code = 0x807BAE90(owner, token.as_ptr());
        if code != 0x2A {
            0x801026D0(world, code, 2);
        }

        // 4. skip to and over '\n'
        loop {
            if at >= size { end = 1; break; }
            let c = *(u8*)(data + at);
            at += 1;
            if c == b'\n' { break; }
        }
        if end != 0 { break; }
    }

    letter += 1;
    if strlen(name.as_ptr()) == 14 { break; }          // 0x802DC69C
    format(name.as_mut_ptr(), 0x20, 0x80820BB0 /* "W%X/pointW%Xa.csv" */,
           world + 1, world + 1);
    name[10] = name[10].wrapping_add(letter as i8 as u8);   // 'a' + letter
    data = 0x800DF1E0(file + 4, "RouteInfo", name.as_ptr(), &mut size, 0);
    if data == 0 { break; }
}

0x80102730(world, hits);
```

`0x800DF440` is `dres_get_res_silently_size` and `0x800DF1E0` is
`dres_get_res`; both are `(archive_table, arc_name, path, &size_out)` and return
the member pointer. `0x802E19F8` is the bounded copy that stops at a `NUL` and
zero-fills the rest (`strncpy` shape). The `format` is the varargs
`0x802E16E8(buf, 0x20, fmt, a, b)` - it is always called with a `crxor 6,6,6`
before the `bl`, which is the MWCC varargs marker.

The `strlen(name) == 14` guard is the interesting part and it is **not** a size
check: `W1/pointW1.csv` is 14 characters while `W1/pointW1a.csv` is 15, so the
guard fires exactly when the no-letter file was the one that existed and the
`a`/`b`/... walk must not start.

### 3.1 The address-only helpers this body needs

`0x807BAE90` (120 bytes, no ledger row, the next TSV row) is a seven-branch
token classifier:

```
if (token[0] != 'W') return 0x2A;                  // 42
if ((u8)(token[2] - '0') <= 9) return (s8)token[3] - 49;
switch ((s8)token[2]) {
    b'T' => 21, b'C' => 23, b'A' => 37, b'G' => 20, _ => 0x2A,
}
```

`0x800F9970` (`main.dol`, 128 bytes) is the token-kind classifier:
`b'F' -> 0`, `b'K' -> 1`, `"WS" -> 2`, `token[0] == b'W'` with
`token[2]` not in `{b'X', b'K', b'W'}` `-> 3`, otherwise `4`.

`0x801026D0(world, index, mask)` (`main.dol`) is
`if *(u8*)(r13 - 0x5431) == 0 { *(u32*)(0x803741F0 + world * 0xA8 + index * 4) |= mask }`
and `0x80102730(world, value)` is
`if *(u8*)(r13 - 0x5431) == 0 { *(u32*)(0x80374880 + world * 4) = value }`, with
`r13 = 0x8042F680` (so the gate byte is `0x8042A24F`). `0x80102750` is the
read-side counterpart (index `<= 41`, returns `(word & mask) != 0`).

### Constants

| address | value | use |
|---|---|---|
| `0x80820B90` | `"W%X/pointW%X.csv"` | first name tried |
| `0x80820BB0` | `"W%X/pointW%Xa.csv"` | letter-series name |
| `0x80820BA4` | `"RouteInfo"` | archive name |
| `0x8037_41F0` | BSS | per-world flag words, `0xA8`-byte row |
| `0x8037_4880` | BSS | per-world class-3 count |
| `0x8037_48A8` | BSS | per-world file counter |
| `0x8042_A038` | cell | archive table |
| `0x8042_A24F` | `u8` (BSS) | writer gate (`r13 - 0x5431`) |
| - | `0x23` (35), `0x40` | "saw `W?W`" flag bit |
| - | `2` | per-token flag bit |
| - | `0x2A` (42) | "not a `W` token" code |
| - | `14` | no-letter name length (guard) |
| - | `0x20` (32) | name buffer size |

### Callees

`0x802DCD58`/`0x802DCDA4` (`__savegprlr_21`/`__restgprlr_21`), `0x802E16E8`
(varargs formatter), `0x800DF440`/`0x800DF1E0` (`dres_get_res_silently_size`/
`dres_get_res`), `0x802E19F8` (bounded NUL-terminating copy), `0x800F9970`
(token kind, §3.1),
`0x801026D0`/`0x80102730` (gated writers, §3.1), `0x802DC69C` (`strlen`), and
`0x807BAE90` (token code, §3.1).

### Notes for the implementer

* Argument 1 is *unused by this body* but is forwarded to `0x807BAE90`, which
  also ignores it. Keep the parameter so the call shape stays literal.
* `world` is 0-based for both tables and the counters; the *file name* uses
  `world + 1`.
* The `name[10]` increment is on the **formatted** buffer, after the `a`
  variant has been written, so the first letter file after `a` is `b`; the
  no-letter file is never followed by a letter file (the guard breaks first).
* `size` comes from the `dRes` call (the 4th argument) and is the *member*
  size, not `0x20`; the two `0x20`s in the body are the formatter buffer size.
* The token window is 4 bytes but the copy is unconditional, so a record shorter
  than 4 bytes reads past `data + at` for the remaining bytes before the NUL
  store; the shipped files never do this.

**Verdict: ready (medium).** All 146 words decoded and every table, string and
gate read; medium only because four callees (`0x800F9970`, `0x801026D0`,
`0x80102730`, `0x807BAE90`) are address-only - all four are decoded in §3.1, and
`0x807BAE90` is the next TSV row, which is why the conditional-`bclr` decoder
fix was needed.

---

## 4. `0x807BBE20` - `dWorldSelect_c::StateID_OnStageWait` execute handler

**Evidence.** `work/ghidra/rel-d_basesNP/export/807bbe20_FUN_807bbe20.c`,
584 bytes, 146 words, synthetic `0x807BBE20`. Body `0x807BBE20..0x807BC064`
(ends `blr`, exactly one). Frame: `stwu r1,-48(r1)`, `__savegprlr_27`,
epilogue `__restgprlr_27`. No `psq`, no jump table. The 40-byte gap after the
extent is zero words; the next row `0x807BC090` is the *third* handler of the
same state.

**What it is.** The middle handler (the per-frame `execute`) of the first
`dWorldSelect_c` state. `docs/SPEC_WAVE141_BASES.md:236-252` decoded the state
table that names it:

```
0x80820EB8 ..: { 0, -1, 0x807BC070 }   <- create
               { 0, -1, 0x807BBE20 }   <- execute   (this row)
               { 0, -1, 0x807BBD80 }   <- delete
               state record 0x808340F8 = "dWorldSelect_c::StateID_OnStageWait"
```

It refreshes the ten-pane world-select screen (`+0x248` pane array, `+0x294`
count - the same object `fn_807bb9f0` and `fn_807bbc30` document as the
"world-select screen object"), reads the ten course-unlock bytes of the current
save record, re-centres the pane row on the selected index through the
`0x807BB870` spacing, sets every pane's state word `+0x284` to
`0`(selected)/`2`(locked)/`3`(unlocked), copies the selected pane's `+0x294`
float to every `+0x298`, forces `+0x2A0 = 1.0`, and finally dispatches the next
state record `0x80834138` (`StateID_OpenAnimeEndWait`).

**Signature.**

```rust
/// Runs one frame of `dWorldSelect_c::StateID_OnStageWait`.
///
/// # Original
/// - Ghidra: `FUN_807bbe20` at `0x807BBE20` (584 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807bbe20_FUN_807bbe20.c`
///
/// # Arguments
/// * `screen` - the world-select screen; panes `+0x248`, pane count `+0x294`,
///   selection `+0x27C`, unlock bytes `+0x2A8`, mode byte `+0x2B7`,
///   finish byte `+0x2B0`, sub-objects `+0x268`/`+0x26C`/`+0x270`/`+0x274`,
///   dDraw owner `+0x74`, state interface `+0x20C`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// Every pane pointer must be a live pane actor (it must expose `+0x2C4`,
/// `+0x284`, `+0x294`, `+0x298`, `+0x2A0`), `0x8042A040` must hold the save
/// image and `0x8042A4A8` a live sound state object.
pub unsafe fn fn_807bbe20(screen: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x074` | object | dDraw owner passed to `0x800C9500` |
| `+0x20C` | object | state interface; vtable slot `+0x18` receives `0x80834138` |
| `+0x248 + i*4` | ptr | pane actor `i` (`i < +0x294`, and `+0x294 + 1` in the write loop) |
| `+0x268` | pane | "selected pane" object (`+0x26C` word, `+0x275` byte, `+0x284`, `+0x2A0`, `+0x2A4`) |
| `+0x26C` | object | object whose `+0x230` receives `selection + 1` |
| `+0x270` | object | object whose `+0xBB` bit 0 is set |
| `+0x274` | object | object whose `+0xB8` byte is cleared in the `+0x2B7` arm |
| `+0x27C` | `u32` | selection; `>= 8` is clamped to `0` for the spacing product only |
| `+0x294` | `s32` | pane count used by the two loops |
| `+0x298` | `u32` | cleared once per frame |
| `+0x2A8..+0x2B1` | `u8[10]` | per-course unlock bytes (from `0x800CE350`) |
| `+0x2B0` | `u8` | value copied to the selected pane's `+0x275` |
| `+0x2B7` | `u8` | mode flag: selects state `3`, `+0x26C = 4` and `+0x2A4 = 500.0` |

### Exact behaviour

```
// 1. unlock bytes of the current save image
let save = 0x800E0380(*(u32*)0x8042A040, -1);
for i in 0..10 {
    *(u8*)(screen + 0x2A8 + i) = (0x800CE350(save, i, 1) != 0) as u8;
}

// 2. mark the selected pane object visible
let o270 = *(u32*)(screen + 0x270);
*(u8*)(o270 + 0xBB) = (*(u8*)(o270 + 0xBB) & 0xFE) | 1;
0x8019C9E0(*(u32*)0x8042A4A8);              // == 0x8019CA00(obj, 3)

// 3. pane row spacing
let spacing = 0x807BB870(screen);           // (s16)pane0[+0x2C4] - (s16)pane1[+0x2C4]
let mut sel = *(u32*)(screen + 0x27C);
if (sel as i32) > 7 { sel = 0; }
0x807BB9F0(spacing * sel as f32, screen);   // writes trunc(v) to every pane's +0x2C6
0x807BBC30(screen, 1);                      // every pane's +0x275 = 1, then 0x80766F20(pane)
*(u32*)(screen + 0x298) = 0;

// 4. selected pane object
let pane0 = *(u32*)(screen + 0x268);
if *(u8*)(screen + 0x2B7) == 0 {
    *(u8*)(pane0 + 0x275) = *(u8*)(screen + 0x2B0);
    0x80766F20(pane0);
} else {
    *(u8*)(*(u32*)(screen + 0x274) + 0xB8) = 0;
    *(u32*)(pane0 + 0x26C) = 4;
    *(u8*)(pane0 + 0x275) = 1;
    0x80766F20(pane0);
    *(f32*)(pane0 + 0x2A4) = *(f32*)0x807D0A0C;      // 500.0
}

// 5. per-pane state
let raw_sel = *(u32*)(screen + 0x27C);            // unclamped, used for i == raw_sel
let ref_pos = *(f32*)(*(u32*)(screen + 0x248 + sel * 4) + 0x294);
let count   = *(i32*)(screen + 0x294) + 1;
for i in 0..count {
    let pane = *(u32*)(screen + 0x248 + i * 4);
    let state = if i == raw_sel {
        if *(u8*)(screen + 0x2B7) != 0 { 3 } else { 0 }
    } else {
        2 + (*(u8*)(screen + 0x2A8 + i) != 0) as u32
    };
    *(u32*)(pane + 0x284) = state;
    *(f32*)(pane + 0x298) = ref_pos;
    *(f32*)(pane + 0x2A0) = *(f32*)0x807D0A10;       // 1.0
}

// 6. tail
if *(u8*)(screen + 0x2B7) != 0 {
    *(u32*)(pane0 + 0x284) = 0;
}
*(f32*)(pane0 + 0x2A0) = 1.0;
*(i32*)(*(u32*)(screen + 0x26C) + 0x230) = *(i32*)(screen + 0x27C) + 1;
0x800C9500(screen + 0x74);
vmethod1(screen + 0x20C, 0x18, 0x80834138);
```

Three details matter:

* The `2 + (unlock != 0)` expression is `neg`/`or`/`rlwinm` at
  `0x807BBFC0..0x807BBFCC`: `bit31((-b) | b)` is `1` exactly when `b != 0`, so a
  locked course gets `2` and an unlocked one `3`. The Ghidra print
  `2 - ((int)-(uint)byte >> 0x1f)` is the same thing.
* `ref_pos` is a **float** (`lfs f1,660(r5)`), taken from the *selected* pane's
  `+0x294` before the loop; `+0x298` of every pane then receives it via `stfs`.
* The write loop runs `+0x294 + 1` times, i.e. one more pane than the
  read loop of `fn_807bbc30`; the array is addressed without a bound check in
  both bodies.

### Constants

| address | value | use |
|---|---|---|
| `0x807D09F8` | `4503601774854144.0` (`2^52 + 2^31`) | `(u32)sel -> f64` bias |
| `0x807D0A0C` | `500.0f` | `pane0 + 0x2A4` in the `+0x2B7` arm |
| `0x807D0A10` | `1.0f` | `pane + 0x2A0`, and `pane0 + 0x2A0` |
| `0x8083_4138` | BSS record | next state (`StateID_OpenAnimeEndWait`) |
| `0x8042_A040` | cell | save-image cell (`0x800E0380` first argument) |
| `0x8042_A4A8` | cell | sound state object |
| - | `10` | unlock-byte count |
| - | `8` / `7` | selection wrap (`sel >= 8` reads as 0) |
| - | `2`, `3` | pane state words (locked / unlocked) |
| - | `4` | `pane0 + 0x26C` in the `+0x2B7` arm |
| - | `0xFE`, `1` | `+0xBB` bit 0 set |
| - | `0x18` | state interface vtable slot |

### Callees

`0x802DCD74`/`0x802DCDC0` (`__savegprlr_27`/`__restgprlr_27`), `0x800E0380`
(`engine::display::save_image_record_a`), `0x800CE350`
(`game::stage::fn_800ce350`, `mask & *(u8*)(base + slot + 0x32)`),
`0x8019C9E0` (`main.dol` 8-byte tail into `0x8019CA00`; here
`0x8019CA00(sound_state, 3)` = "set group state 3"), `0x807BB870` (spacing,
decoded below), `0x807BB9F0` (`fn_807bb9f0`), `0x807BBC30` (`fn_807bbc30`),
`0x80766F20` (`fn_80766f20`, pane pose update; takes the pane in `r3` - note
that `fn_807bbc30`'s translation calls it with no arguments), `0x800C9500`
(`game::stage::fn_800c9500`), and the `+0x18` slot of `+0x20C`.

`0x807BB870` (84 bytes, no ledger row) is:
`(f32)(s16)*(u32*)(screen+0x248)[0].+0x2C4 - (f32)(s16)*(u32*)(screen+0x248)[1].+0x2C4`,
both converted through the `xoris 0x8000`/`0x4330` idiom.

### Notes for the implementer

* `sel` is clamped to `0` only for the *spacing* product (`0x807BBEC4`), not for
  the `i == sel` comparisons in step 5, which use the raw `+0x27C` value.
* `state == 0` for the selected pane only when `+0x2B7 == 0`; the step-6
  override then forces `pane0 + 0x284 = 0` again whenever `+0x2B7 != 0`.
* `+0x298` (screen) is a `u32` store of `0`, while `+0x298` on a *pane* is an
  `f32` store; do not merge the two.
* The Ghidra export types the whole function as `void FUN_807bbe20(void)` and
  prints `func_0x802dcd74()` as the receiver - take `this` from `r3`.

**Verdict: ready (high).** All 146 words decoded, both tables and all four
floats read, the state identity is already documented in
`docs/SPEC_WAVE141_BASES.md`, and the one unledgered callee (`0x807BB870`) is
fourteen instructions quoted above.

---

## 5. `0x807649B0` - `hanachan` four-segment + head model loader

**Evidence.** `work/ghidra/rel-d_basesNP/export/807649b0_FUN_807649b0.c`,
580 bytes, 145 words, synthetic `0x807649B0`. Body `0x807649B0..0x80764BF0`
(ends `blr`, exactly one; the 12-byte gap is three zero words and the next row
is the 120-byte `0x80764C00`). One `bl` caller: `0x80764804`, inside the
untracked hole `0x807647EC..0x80764910` (the initialiser that precedes the
132-byte `0x80764910`). Frame: `stwu r1,-112(r1)`, `__savegprlr_22`,
`stfd f31` + `psq_st f31,...,0,qr0`.

**What it is.** The model/anim loader of `hanachan` (the caterpillar boss): it
builds the per-actor allocator, loads `g3d/hanachan_body.brres` and creates the
*four* body segments (`+0x1A8 + i*0x40` model holders, `+0x2A8 + i*0x38`
animation holders, `+0x2DC + i*0x38` a per-part byte), then loads
`g3d/hanachan_head.brres` for the head at `+0x388`/`+0x38C`/`+0x3CC`, and
finishes with `0x80765980`, which builds the five 12-byte `AnmObjChrBlend`
wrappers at `+0x404 + i*0xC`. The four body parts get animation rates
`4.0, 8.0, 12.0, 16.0`; the head gets `0.0`.

**Signature.**

```rust
/// Loads the `hanachan` four body segments and the head, and binds their
/// `walk_normal` animations.
///
/// # Original
/// - Ghidra: `FUN_807649b0` at `0x807649B0` (580 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807649b0_FUN_807649b0.c`
///
/// # Arguments
/// * `actor` - the caterpillar; allocator `+0x188`, body model data `+0x1A4`,
///   parts `+0x1A8 + i*0x40`, part animations `+0x2A8 + i*0x38`, part bytes
///   `+0x2DC + i*0x38`, head model data `+0x388`, head holder `+0x38C`,
///   head animation `+0x3CC`, head byte `+0x400`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `+0x1A4`, `+0x388` and every holder must be live; `0x8042A038` must hold a
/// live archive table.
pub unsafe fn fn_807649b0(actor: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x188` | allocator | per-actor NW4R allocator |
| `+0x1A4` | `u32` | `hanachan_body` model data pointer (held directly) |
| `+0x1A8 + i*0x40` | model | body-part model holder, `i = 0..3` |
| `+0x2A8 + i*0x38` | anim | body-part `AnmObjChr` holder, `i = 0..3` |
| `+0x2DC + i*0x38` | `u8` | body-part byte, cleared |
| `+0x388` | `u32` | `hanachan_head` model data pointer |
| `+0x38C` | model | head model holder |
| `+0x3CC` | anim | head `AnmObjChr` holder |
| `+0x400` | `u8` | head byte, cleared |
| `+0x404 + i*0xC` | model | five blend-animation wrappers (built by `0x80765980`) |

### Exact behaviour

```
let file = *(u32*)0x8042A038;
0x800690C0(actor + 0x188, -1, *(u32*)0x80377C48, 0, 0x20);
*(u32*)(actor + 0x1A4) =
    0x800DF180(file + 4, 0x808159E8 /* "hanachan_body" */,
               0x808159D0 /* "g3d/hanachan_body.brres" */);

for i in 0..4 {
    let model = 0x80239E30(actor + 0x1A4, 0x808159E8 /* "hanachan_body" */);
    let mut m = model;
    0x80169D90(actor + 0x1A8 + i * 0x40, &m, actor + 0x188, 0x20, 1, 0);

    let chr = 0x8023A0B0(actor + 0x1A4, *(u32*)0x808159C8);  // "walk_normal"
    0x801650D0(actor + 0x2A8 + i * 0x38, &m, &chr, actor + 0x188, 0);

    *(u8*)(actor + 0x2DC + i * 0x38) = 0;
    0x801680E0(1.0, actor + 0x2A8 + i * 0x38);
    0x80168FE0((i + 1) as f32 * 4.0, actor + 0x2A8 + i * 0x38);
    vmethod1(actor + 0x1A8 + i * 0x40, 0x18, actor + 0x2A8 + i * 0x38);
    0x800F2A30(actor + 0x1A8 + i * 0x40);
}

*(u32*)(actor + 0x388) =
    0x800DF180(file + 4, 0x80815A10 /* "hanachan_head" */,
               0x808159F8 /* "g3d/hanachan_head.brres" */);
let head = 0x80239E30(actor + 0x388, 0x80815A10 /* "hanachan_head" */);
let mut h = head;
0x80169D90(actor + 0x38C, &h, actor + 0x188, 0x20, 1, 0);

let hchr = 0x8023A0B0(actor + 0x388, *(u32*)0x808159CC);   // "walk_normal"
0x801650D0(actor + 0x3CC, &h, &hchr, actor + 0x188, 0);

*(u8*)(actor + 0x400) = 0;
0x801680E0(1.0, actor + 0x3CC);
0x80168FE0(0.0, actor + 0x3CC);
vmethod1(actor + 0x38C, 0x18, actor + 0x3CC);
0x800F2A30(actor + 0x38C);

0x800690E0(actor + 0x188);
0x80765980(actor);
```

The `1.0` in the two `0x801680E0` calls is `*(f32*)0x807CEF08`; the `4.0`
multiplier is `*(f32*)0x80815994` (the word right after the `walk_normal`
pointer pair); `(i + 1)` is built with the `0x43300000`/`xoris 0x8000` idiom
against the `2^52 + 2^31` double at `0x807CEF10`, then multiplied in `f32`.

### 5.1 The tail body `0x80765980` (272 bytes, address-only)

`0x80765980(actor)` builds the shadow/blend set:

```
0x8016A960(actor + 0x13C, -1, *(u32*)0x80377C48, 0, 0x20, 0);
let res = 0x800DF180(*(u32*)0x8042A038 + 4, "character_SV", "g3d/model.brres");
let model = 0x80239E30(&res, "character_SV");
for i in 0..5 {
    0x8016A3A0(actor + 0x404 + i * 0xC, &model, actor + 0x13C, 0x20, 1, 0);
    let wrap = 0x802B9210(12);              // allocate 12 bytes
    if wrap != 0 {
        0x8016A010(wrap);                   // construct
        *(u32*)wrap = 0x8032BC14;           // vtable
    }
    *(u32*)(actor + 0x440 + i * 4) = wrap;
    (*(*(u32*)wrap + 0x20))(wrap, actor + 0x13C, actor + 0x404 + i * 0xC);
}
0x8016A9B0(actor + 0x13C);
```

so `+0x404 + i*0xC` are five model wrappers and `+0x440 + i*4` five 12-byte
`vtable = 0x8032BC14` objects, each bound to its wrapper through slot `+0x20`.

### Constants

| address | value | use |
|---|---|---|
| `0x808159D0` | `"g3d/hanachan_body.brres"` | body archive path |
| `0x808159E8` | `"hanachan_body"` | body archive + model name |
| `0x808159F8` | `"g3d/hanachan_head.brres"` | head archive path |
| `0x80815A10` | `"hanachan_head"` | head archive + model name |
| `0x808159BC` | `"walk_normal"` | animation name |
| `0x808159C8` | `u32` -> `0x808159BC` | body animation name pointer |
| `0x808159CC` | `u32` -> `0x808159BC` | head animation name pointer |
| `0x80815994` | `4.0f` | per-part rate step |
| `0x807CEF08` | `1.0f` | `0x801680E0` value |
| `0x807CEF0C` | `0.0f` | head rate |
| `0x807CEF10` | `4503601774854144.0` (`2^52 + 2^31`) | `(i+1) -> f64` bias |
| `0x8032BC14` | vtable | the 12-byte blend wrapper (`main.dol`) |
| `0x8042_A038` | cell | archive table |
| `0x8037_7C48` | `u32` (BSS) | allocator parent word |
| - | `4` | body-part count |
| - | `0x40`, `0x38`, `0x20` | part model / part animation / factory strides |
| - | `0x20` (32) | `0x80169D90` kind word |

### Callees

`0x802DCD5C`/`0x802DCDA8` (`__savegprlr_22`/`__restgprlr_22`), `0x800690C0`/
`0x800690E0` (allocator pair, Method note), `0x800DF180`, `0x80239E30`,
`0x8023A0B0`, `0x80169D90` (six-argument model factory),
`0x801650D0`/`0x80167420`, `0x801680E0`/`0x80168FE0`,
`0x800F2A30` (tail `0x800B4170(holder, 0)`, the kind-4 pass), `0x80765980`
(§5.1), and the `+0x18` slot of each holder.

### Notes for the implementer

* The body-part loop reads `0x80239E30` **again** on every iteration from the
  same `+0x1A4`; keep the four calls.
* Part `i`'s animation rate is `(i + 1) * 4.0`, i.e. `4, 8, 12, 16` - it is not
  a table read.
* `+0x1A4` and `+0x388` hold the *model data pointers* directly (stored by this
  body), whereas the holders at `+0x1A8`/`+0x38C` are passed to the factories.
* `0x800F2A30` is called once per part **and** once for the head, after the
  bind; `0x800690E0` runs only once, after the head.
* The head uses the same `walk_normal` animation pointer as the parts
  (`0x808159CC` and `0x808159C8` both point at `0x808159BC`).

**Verdict: ready (medium).** All 145 words decoded, every string/float read, the
loop structure and strides literal; medium because `0x80169D90`, `0x800F2A30`
and the whole of `0x80765980` are address-only (the last one is quoted above,
the other two are five-line tail calls).

---

## 6. `0x807BE750` - world-map ribbon screen create body

**Evidence.** `work/ghidra/rel-d_basesNP/export/807be750_FUN_807be750.c`,
580 bytes, 145 words, synthetic `0x807BE750`. Body `0x807BE750..0x807BE990`
(ends `blr`, exactly one; the 172-byte gap holds zero words and then the two
neighbour bodies `0x807BE9A0`/`0x807BE9D0`). Frame: `stwu r1,-128(r1)`,
`__savegprlr_26`, `stfd f31`+`psq_st`, `stfd f30`+`psq_st`. Returns `1`.

**What it is.** The create/start body of the world-map screen object whose
layout builder is the translated `fn_807bef60` (wave 143/152: "layout owner A
`+0x184`, animation A `+0x20C`, owner B `+0x294`, animation B `+0x31C`, rects
`+0x3A4`/`+0x48C`, registry id `+0x4A8`, colour block `+0x468`"). It

1. decodes the parameter word `+0x04` into `+0x4A8` (`& 0xF`) and `+0x4AC`
   (`>> 4 & 0xF`),
2. runs the per-frame state body `0x807BEAF0` once (10-way dispatch on `+0x604`)
   and the layout builder `0x807BEF60`,
3. registers itself in the global slot table through `0x80102660`,
4. seeds two transform pairs from the `main.dol` `u16` table at `0x803504E0`
   indexed by the current world `*(u32*)0x8042AF08`,
5. builds two orthographic cameras (ids `5` and `6`) from the per-*course*
   table at `0x80821540` indexed by the current course id `*(u8*)0x8042A24D`,
   with ortho half-extents `150` and `85`, near `1.0`, far `10000.0`,
   position `(0,0,0)`, and
6. returns `1`.

**Signature.**

```rust
/// Creates the world-map ribbon screen: state, layout, two ortho cameras.
///
/// # Original
/// - Ghidra: `FUN_807be750` at `0x807BE750` (580 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807be750_FUN_807be750.c`
///
/// # Arguments
/// * `screen` - the screen object; parameters `+0x04`, state `+0x604`,
///   animations `+0x20C`/`+0x31C`, colour block `+0x468..+0x488`,
///   registry id `+0x4A8`, rects `+0x6F8`/`+0x71C`/`+0x720`/`+0x730`,
///   effect handle `+0x740`.
///
/// # Returns
/// `1` - create success.
///
/// # Safety
/// `*(u32*)0x8042AF08` must be a world index in `0..=3`,
/// `*(u8*)0x8042A24D` a course id in `0..=3`, `0x8042A24F` must be readable
/// (it gates the `0x80102660` store), and the two slot tables
/// (`0x80821540`, `0x808215F0`) must be readable.
pub unsafe fn fn_807be750(screen: usize) -> u32
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | parameters: `&0xF` -> `+0x4A8`, `>>4 &0xF` -> `+0x4AC` |
| `+0x20C` | animation | animation A (dirty word set to `1`) |
| `+0x214`/`+0x218` | `f32` | transform A x/z from the `main.dol` `u16` pair |
| `+0x224`/`+0x228` | `f32` | `1.0` / `20000.0` (transform A) |
| `+0x240` | `u16` | bit 0 set (transform A dirty) |
| `+0x31C` | animation | animation B (dirty word set to `1`) |
| `+0x324`/`+0x328` | `f32` | transform B x/z from the same `u16` pair |
| `+0x334`/`+0x338` | `f32` | `1.0` / `20000.0` (transform B) |
| `+0x350` | `u16` | bit 0 set (transform B dirty) |
| `+0x4A8` | `u32` | registry id (`params & 0xF`) |
| `+0x4AC` | `u32` | second parameter nibble |
| `+0x604` | `u32` | state word read by `0x807BEAF0` |
| `+0x6F8`/`+0x6FC` | `f32` | `0.0` |
| `+0x71C` | `u32` | `0` |
| `+0x720` | `u32` | `0` |
| `+0x724`/`+0x728` | `f32` | `0.0` |
| `+0x72C` | `u32` | `2` |
| `+0x730`..`+0x738` | vec3 | origin triple `*(f32*)0x80377DC8..D0` (BSS, all `0.0`) |
| `+0x740` | `u32` | `-1` (effect handle) |

### Exact behaviour

```
// 1. parameters
*(u32*)(screen + 0x4A8) = *(u32*)(screen + 0x04) & 0xF;
*(u32*)(screen + 0x4AC) = (*(u32*)(screen + 0x04) >> 4) & 0xF;

// 2. state + layout, and global registration
0x807BEAF0(screen);
0x807BEF60(screen);
0x80102660(*(u32*)0x8042A234, *(u32*)(screen + 0x4A8), screen);

// 3. transforms
*(u32*)(screen + 0x20C) = 1;
let world = *(u32*)0x8042AF08;
let p = 0x803504E0 + world * 12;                 // {u16 x, u16 z, u16 0}
let x = *(u16*)(p + 0);
let z = *(u16*)(p + 2);

*(u16*)(screen + 0x240) |= 1;
*(f32*)(screen + 0x214) = x as f32;
*(f32*)(screen + 0x218) = z as f32;
*(f32*)(screen + 0x224) = 1.0;
*(f32*)(screen + 0x228) = 20000.0;

*(u32*)(screen + 0x31C) = 1;
*(u16*)(screen + 0x350) |= 1;
*(f32*)(screen + 0x324) = x as f32;
*(f32*)(screen + 0x328) = z as f32;
*(f32*)(screen + 0x334) = 1.0;
*(f32*)(screen + 0x338) = 20000.0;

// 4. ribbon/trail seed
*(u32*)(screen + 0x71C) = 0;
*(f32*)(screen + 0x730) = *(f32*)0x80377DC8;      // 0.0
*(f32*)(screen + 0x734) = *(f32*)0x80377DCC;
*(f32*)(screen + 0x738) = *(f32*)0x80377DD0;
*(u32*)(screen + 0x720) = 0;
*(f32*)(screen + 0x724) = 0.0;
*(f32*)(screen + 0x728) = 0.0;
*(u32*)(screen + 0x72C) = 2;
*(f32*)(screen + 0x6F8) = 0.0;
*(f32*)(screen + 0x6FC) = 0.0;

// 5. two ortho cameras, ids 5 and 6
let course = *(u8*)0x8042A24D as u32;
let a = 0x80821540 + course * 0x40;
let b = 0x808215F0 + course * 0x40;
for i in 0..2 {
    let holder = [0x80164B20(i + 5)];             // camera by id -> r1+8
    let half  = 0.5 * *(f32*)(a + 0xA8 + i * 0x20);
    let ratio = *(f32*)(a + 0xAC + i * 0x20);
    0x80253C70(half, -half, -(half * ratio), half * ratio,
               1.0, 10000.0, holder.as_ptr());
    0x80253950(holder.as_ptr(), b + i * 0x20);    // position = (0,0,0)

    // 44-byte stack record: tag 2, three floats at +0x10..+0x18,
    // one float at +0x28; the words in between are left uninitialised.
    let mut rec = [0u32; 11];
    rec[0]  = 2;
    rec[4]  = *(u32*)(a + 0xBC + i * 0x20);
    rec[5]  = *(u32*)(a + 0xC0 + i * 0x20);
    rec[6]  = *(u32*)(a + 0xC4 + i * 0x20);
    rec[10] = *(u32*)0x807D0A6C;                  // 0.0f
    0x802539C0(holder.as_ptr(), rec.as_ptr());
}

*(u32*)(screen + 0x740) = 0xFFFF_FFFF;
1
```

`0x80253C70(a, b, c, d, e, f, holder)` writes (through `*(u32*)holder`):
`+0xA8 = 1`, `+0xBC = a`, `+0xC0 = b`, `+0xC4 = c`, `+0xC8 = d`, `+0xB4 = e`,
`+0xB8 = f`, and sets bit 6 of `+0x70`. `0x80253950(holder, src)` copies
`src[0..3]` into `+0x74..+0x7C` and clears bit 3 of `+0x70`.
`0x802539C0(holder, rec)` is the mode-tagged setter: `rec[0] == 0` writes
`rec[1..6]` to `+0x80..+0x94` and sets `+0x70` bit 0; `rec[0] == 1` writes
`rec[7..9]` to `+0x98..+0xA0` and `rec[10]` to `+0xA4`, setting bit 1;
`rec[0] == 2` writes `rec[4..6]` to `+0x8C..+0x94` and `rec[10]` to `+0xA4`,
setting bit 2. All three arms first compare against the current values and
return early when nothing changed (`+0x98..+0xA0`, `+0x8C..+0x94`).

The `main.dol` `u16` table (12-byte rows, `{x, z, 0, 0, 0, 0}`):

| world | row address | x | z |
|--:|---:|---:|---:|
| 0 | `0x803504E0` | `0x0260` (608) | `0x01C8` (456) |
| 1 | `0x803504EC` | `0x0340` (832) | `0x01C8` |
| 2 | `0x803504F8` | `0x0280` (640) | `0x01C8` |
| 3 | `0x80350504` | `0x0000` | `0x0000` |

The two camera tables are indexed by the **course id** and are identical for
course ids 0-3:

| field | element 0 | element 1 |
|---|---:|---:|
| `+0xA8` (used as `0.5 * v`) | `300.0` | `170.0` |
| `+0xAC` (ratio) | `1.0` | `1.0` |
| `+0xBC`, `+0xC0`, `+0xC4` (mode-2 record) | `0.0`, `0.0`, `0.0` | `0.0`, `0.0`, `0.0` |
| second table `+0x00`, `+0x04`, `+0x08` (position) | `0.0`, `0.0`, `0.0` | `0.0`, `0.0`, `0.0` |

so camera 5 gets `l/r/b/t = -150/+150/-150/+150` and camera 6
`-85/+85/-85/+85`, both with near `1.0` and far `10000.0`.

### Constants

| address | value | use |
|---|---|---|
| `0x807D0A6C` | `0.0f` | the zero stores and the mode-2 tail word |
| `0x807D0AC0` | `0.5f` | half-extent scale |
| `0x807D0AC8` | `1.0f` | near plane / transform scale |
| `0x807D0ADC` | `20000.0f` | `+0x228`/`+0x338` (float bit pattern) |
| `0x807D0AE0` | `10000.0f` | far plane |
| `0x807D0AE8` | `4503599627370496.0` (`2^52`) | `u16 -> f64` bias |
| `0x803504E0` | `u16[?]` (file) | per-*world* position table, 12-byte rows |
| `0x80377DC8`..`D0` | `f32` (BSS) | origin triple, `0.0` at file level |
| `0x80821540` | `f32[]` | camera-A table, `0x40`-byte rows per *course* |
| `0x808215F0` | `f32[]` | camera-B (position) table, `0x40`-byte rows |
| `0x8042_A234` | cell | passed as `0x80102660`'s first argument, which that callee ignores |
| `0x8042_AF08` | `u32` (BSS) | current world index (`0..3`) |
| `0x8042_A24D` | `u8` (BSS) | current course id (`0..3`), indexes the camera tables |
| `0x8042_A24F` | `u8` (BSS) | gate byte of `0x80102660` (`r13 - 0x5431`) |
| `0x8032_BC14` | vtable | (only used by row 5's tail) |
| - | `5`, `6` | camera ids handed to `0x80164B20` |
| - | `0x10`, `0x18` | `& 0xF` / `>> 4 & 0xF` parameter nibbles |
| - | `2` | `+0x72C`, and the mode-2 record tag |

### Callees

`0x802DCD70`/`0x802DCDBC` (`__savegprlr_26`/`__restgprlr_26`), `0x807BEAF0`
(the 10-way state dispatch, below), `0x807BEF60` (`fn_807bef60`, layout),
`0x80102660` (global slot store: `*(u32*)(0x8042A238 + id*4) = screen`, gated by
`*(u8*)(0x8042A24F) == 0`), `0x80164B20`
(`resource::registry::fn_80164b20`, the camera/context lookup through
`0x80258BD0`), `0x80253C70` (projection box + near/far, above), `0x80253950`
(position copy), `0x802539C0` (mode-tagged parameter setter).

`0x807BEAF0` (280 bytes, no ledger row) is `idx = *(u32*)(screen + 0x604)`; if
`idx <= 9` it jumps through the ten-entry table `0x8082187C` (targets
`0x807BEB34/B3C/B44/B54/B4C/B5C/B64/B6C/B74/B7C` = the state bodies
`0x807BF6B0`, `0x807BF760`, `0x807BF860`, `0x807BFB20`, `0x807BF930`,
`0x807BFBE0`, `0x807BFC20`, `0x807BFD00`, `0x807BFE20`, `0x807BF6B0`), then
runs the common tail: `0x807C1220(screen)`; if `*(u8*)(screen + 0x6C0) == 0`
`0x807BEC10(screen, 0x80821874)` and `+0x6C0 = 1`; then if `+0x6C0 != 0`,
`0x807C1380(screen) != 0 ? 0x807BEC10(screen, 0x80821558) :
0x807BEC10(screen, 0x80821878)`.

### Notes for the implementer

* `+0x228`/`+0x338` are `stfs` stores of `20000.0f`
  (`*(f32*)0x807D0ADC`); the export prints them as `undefined4` because it typed
  the destination as a word. Do not confuse them with the `10000.0f` far plane.
* `+0x214`/`+0x218` and `+0x324`/`+0x328` are the same `u16` pair converted
  twice; the two transforms are not independent.
* The dirty-bit ORs (`+0x240 |= 1`, `+0x350 |= 1`) are halfword
  read-modify-writes (`lhz`/`ori`/`sth`), not byte stores.
* `+0x20C` and `+0x31C` are animation objects; `= 1` writes their first word.
* The mode-2 record's three floats and tail word are all `0.0` in the shipped
  table, so the early-out in `0x802539C0` fires on the second frame; the first
  call still writes `+0x8C..+0x94` because bit 2 of `+0x70` is clear.
* `0x807BEAF0` is called with `screen` in `r3` and is a *state* body: it
  dispatches `*(u32*)(screen + 0x604)` and reads `+0x6C0`, which this create
  body never writes, so the first pass runs whatever state and `+0x6C0` value
  the object was allocated with (state `0` and the `+0x6C0 == 0` transition arm
  for a zeroed object).

**Verdict: ready (medium).** All 145 words decoded, both `main.dol` tables read,
all six floats read, and the three `0x80253xxx` camera setters decoded
instruction by instruction; medium because `0x807BEAF0`, its ten state targets,
`0x807C1220`/`0x807C1380` and `0x80258BD0` stay address-only (the first two are
quoted above).

---

## 7. `0x8075C6F0` - `cobBoard` board loader

**Evidence.** `work/ghidra/rel-d_basesNP/export/8075c6f0_FUN_8075c6f0.c`,
576 bytes, 144 words, synthetic `0x8075C6F0`. Body `0x8075C6F0..0x8075C92C`
(ends `blr`, exactly one; the gap to the next row is **0** bytes - the 176-byte
`0x8075C930`, the translated `fn_8075c930`, starts immediately). One `bl`
caller: `0x8075C4A4`, inside the untracked hole `0x8075C488..0x8075C550`.
Frame: `stwu r1,-64(r1)` with manual `stw r31..r28`.

**What it is.** The loader of the `cobBoard` object (a course "board"/platform
prop): it builds the per-actor allocator, loads `g3d/model.brres` member
`cobBoard`, forces the model's `ScnMdl` interface through a dynamic cast,
applies the model option `(0x30001, 0)`, binds the `cobBoard` texture-SRT
animation, and finally attaches the eight named nodes `group1..group8` to the
board's path/child list, selecting `group<params&0xF+1>` as the active one.

`docs/COVERAGE_WAVE78_BASES.md:133` already calls this row "the `cobBoard`
loader, 576 bytes".

**Signature.**

```rust
/// Loads the `cobBoard` model and attaches its eight `group1..group8` nodes.
///
/// # Original
/// - Ghidra: `FUN_8075c6f0` at `0x8075C6F0` (576 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/8075c6f0_FUN_8075c6f0.c`
///
/// # Arguments
/// * `board` - the board object; allocator `+0x18C`, model data `+0x1A8`,
///   model holder `+0x1AC`, `ScnMdl` pointer `+0x1B0`, animation `+0x1B8`,
///   parameter word `+0x04` (low nibble selects the active group).
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `+0x1B0` may be null; `0x8042A038` must hold a live archive table and
/// `0x8000D660` must accept the board's path list.
pub unsafe fn fn_8075c6f0(board: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x004` | `u32` | parameters; `& 0xF` selects the active group |
| `+0x18C` | allocator | per-actor NW4R allocator |
| `+0x1A8` | `u32` | model data pointer (stored directly) |
| `+0x1AC` | model holder | model object created from `&model` |
| `+0x1B0` | `*mut ScnMdl` | the `+4` word of `+0x1AC`, used for the cast |
| `+0x1B8` | anim | `AnmObjTexSrt` holder |

### Exact behaviour

```
0x800690C0(board + 0x18C, -1, *(u32*)0x80377C48, 0, 0x20);

*(u32*)(board + 0x1A8) =
    0x800DF180(*(u32*)0x8042A038 + 4, 0x80814650 /* "cobBoard" */,
               0x80814640 /* "g3d/model.brres" */);

let model = 0x80239E30(board + 0x1A8, 0x80814650 /* "cobBoard" */);
0x8016A3A0(board + 0x1AC, &model, board + 0x18C, 0x40, 1, 0);

// dynamic cast to nw4r::g3d::ScnMdl (type record 0x802F8FF0)
let mut scn = 0usize;
let p = *(u32*)(board + 0x1B0) as usize;
if p != 0 {
    let mut type_info = 0x802F8FF0u32;
    if (*(*(u32*)p + 8))(p, &mut type_info as *mut u32) != 0 {
        scn = p;
    }
}
(*(*(u32*)scn + 0x20))(scn, 0x30001, 0);

let tex = 0x8023A270(board + 0x1A8, *(u32*)0x80814638);   // "cobBoard"
0x80167420(board + 0x1B8, &model, &tex, board + 0x18C, 0, 1);
0x80167B60(board + 0x1B8, 0, 0);
0x80167B40(0.0, board + 0x1B8, 0);
0x80167B20(0.0, board + 0x1B8, 0);
vmethod1(board + 0x1AC, 0x18, board + 0x1B8);

let active = *(u32*)(board + 0x04) & 0xF;
let mut name = [0u8; 10];
for i in 0..8 {
    format(name.as_mut_ptr(), 10, 0x80814660 /* "group%d" */, i + 1);
    let node = 0x8023B200(&model, name.as_ptr());
    if node != 0 {
        let value = *(u32*)(node + 0xC);
        if i == active {
            0x8000D660(board + 0x1AC, value, 1);
        } else {
            0x8000D660(board + 0x1AC, value, 0);
        }
    }
}

0x800F2A20(board + 0x1AC);
0x800690E0(board + 0x18C);
```

The `0.0` in the two animation calls is `*(f32*)0x807CEC30`.

### Constants

| address | value | use |
|---|---|---|
| `0x80814640` | `"g3d/model.brres"` | archive path |
| `0x80814650` | `"cobBoard"` | archive name and model name |
| `0x80814660` | `"group%d"` | node-name format |
| `0x80814638` | `u32` -> `0x8081462C` | animation name pointer |
| `0x8081462C` | `"cobBoard"` | animation name |
| `0x807CEC30` | `0.0f` | animation frame/rate |
| `0x802F8FF0` | type record | `nw4r::g3d::ScnMdl` RTTI (`{7, "ScnMdl", 0, 0, 0xF, "ScnMdlSimple"}`) |
| `0x8042_A038` | cell | archive table |
| `0x8037_7C48` | `u32` (BSS) | allocator parent word |
| - | `0x40` (64) | `0x8016A3A0` kind word |
| - | `8` | node count |
| - | `0x30001` | `ScnMdl` vtable `+0x20` option |
| - | `10` | node-name buffer size |

### Callees

`0x800690C0`/`0x800690E0` (allocator pair), `0x800DF180`, `0x80239E30`,
`0x8016A3A0` (`resource::registry::fn_8016a3a0`), the `ScnMdl` vtable slots `+8`
(dynamic cast against `0x802F8FF0`) and `+0x20` (option set), `0x8023A270`
(`brres_get_anm_tex_srt`), `0x80167420`, `0x80167B60`/`0x80167B40`/`0x80167B20`,
`0x8023B200` (`resource::bres::mdl_get_node`, `(model_ptr, name)`),
`0x8000D660` (`game::path::fn_8000d660`, `(owner, value, selector)`; the
selector is the *sign* of the third argument), `0x800F2A20` (tail
`0x800B42B0(holder, 0)`, the kind-2 pass), and the `+0x18` slot of `+0x1AC`.

### Notes for the implementer

* The dynamic cast is the MWCC pattern `p->vtable[+8](p, &type_info)`: a
  non-zero result means "this object is a `ScnMdl`". When `+0x1B0` is null or
  the cast fails, the body still calls slot `+0x20` **through the null
  pointer** (Ghidra prints `piVar5 = (int*)0x0` then dereferences it); a
  faithful translation keeps the unconditional call, but the safe reading is
  that the shipped model is always a `ScnMdl`.
* `0x8023A270`'s name is a *loaded word*, not an address literal: `r4` is
  `*(u32*)0x80814638`, which points at `0x8081462C` (`"cobBoard"`).
* `0x8000D660` is called with only `r3`/`r4`/`r5`; the translation in
  `game::path` documents `param_3`'s sign as the selector, which matches
  `li r5,1` / `li r5,0`.
* The loop runs `i = 0..7` and compares `i` with `params & 0xF`, so parameter
  values `8..15` never select a group (nothing is marked active).
* `0x8023B200` takes the **address** of the word holding the model data
  (`r1+24`), exactly like `0x80239E30`.

**Verdict: ready (medium).** All 144 words decoded, every string/float/type
record read and the eight-iteration node walk literal; medium because
`0x80253xxx`-style main.dol helpers are not involved here but `0x8000D660`'s
selector consumer stays opaque and the null-cast path is a documented quirk.

---

## 8. `0x807C22C0` - eight-request state dispatcher + four-player resync

**Evidence.** `work/ghidra/rel-d_basesNP/export/807c22c0_FUN_807c22c0.c`,
576 bytes, 144 words, synthetic `0x807C22C0`. Body `0x807C22C0..0x807C24FC`
(ends `blr`, exactly one). Frame: `stwu r1,-32(r1)`, `__savegprlr_26`,
epilogue `__restgprlr_26`. No `bl` caller and no data reference except the
12-byte record `{0, -1, 0x807C22C0}` at `0x80821B60`.

**What it is.** A request-flag dispatcher: it tests eleven request bytes
`+0x53D`..`+0x54D` in a fixed priority order and, for the first one set, either
just switches the state interface `+0x70` to one of eight BSS state records or
performs a short side-effect block first. When no request is pending it
re-syncs the four player records: for every `i` with `+0x549 + i` set it clears
the byte, asks `0x800B4710(i)` whether player `i` is "state 3", and if so moves
`+0x560 + i*4` into the shared table `0x80354E70` at the slot named by
`0x80354E60[i]` and forwards it to `0x80759770(*(u32*)(this + 0x518 + slot*4))`.

The state records it dispatches live at `0x80834460 + n`, all `0x40`-byte BSS
records of the state machine whose constructor is `0x807C19D0` (which sets
`+0x70 = 0x80821A08`, `+0x74 = 0x807D1018`, `+0x78 = 0x80821A38`,
`+0x7C = 0x80821A50`, `+0x80 = this`, `+0x84 = 0`, builds the state machine at
`+0x88` with `0x8015FC90(+0x88, &+0x74, &+0x78, 0x80834480)`, stores `this` in
`*(u32*)0x80834470`, clears `+0x518..+0x524` and `+0x53C`).

**Signature.**

```rust
/// Dispatches the pending world-map request and re-syncs the four player
/// records when none is set.
///
/// # Original
/// - Ghidra: `FUN_807c22c0` at `0x807C22C0` (576 bytes)
/// - Synthetic base: `0x80600000`; this is not a runtime address.
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807c22c0_FUN_807c22c0.c`
///
/// # Arguments
/// * `this` - the actor; state interface `+0x70`, drawn record `+0xAC`,
///   request bytes `+0x53D..+0x54D`, value `+0x550`, player objects
///   `+0x518 + i*4`, player values `+0x560 + i*4`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The eight BSS state records must be constructed (`0x807C19D0`), and
/// `0x8042A488`/`0x8042A4A8` must hold sound state objects.
pub unsafe fn fn_807c22c0(this: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x070` | object | state interface; vtable `+0x18` receives a state record |
| `+0x0AC` | object | "drawn record" (`+0x268` word, `+0x271`/`+0x273` bytes) |
| `+0x4FF` | `u8` | set to `1` by `0x807C3AF0` |
| `+0x502` | `u8` | cleared by `0x807C3A70` |
| `+0x503` | `u8` | set to `1` by `0x807C3A70` |
| `+0x505` | `u8` | cleared by `0x807C3A70` |
| `+0x518 + slot*4` | ptr | player object by *slot* (`0..3`, 4 words cleared by the constructor) |
| `+0x53C` | `u8` | cleared by the constructor |
| `+0x53D`..`+0x548` | `u8` | request bytes (see the priority order) |
| `+0x549 + i` | `u8` | per-player resync request, `i = 0..3` |
| `+0x54D` | `u8` | last request byte |
| `+0x550` | `u32` | value copied to `record + 0x268` |
| `+0x560 + i*4` | `u32` | per-player value forwarded to the shared table |

### Exact behaviour

```
let base = 0x80834460;

// 1. priority-ordered request dispatch (first match wins)
if *(u8*)(this + 0x541) != 0 { vmethod1(this + 0x70, 0x18, base + 0x60);  return; }
if *(u8*)(this + 0x542) != 0 {
    *(u8*)(this + 0x53D) = 0;
    0x8019CA00(*(u32*)0x8042A4A8, 1);
    vmethod1(this + 0x70, 0x18, base + 0x260);
    return;
}
if *(u8*)(this + 0x544) != 0 { vmethod1(this + 0x70, 0x18, base + 0x720); return; }
if *(u8*)(this + 0x53E) != 0 {
    let record = *(u32*)(this + 0xAC);
    *(u8*)(record + 0x271) = 1;
    *(u32*)(record + 0x268) = *(u32*)(this + 0x550);
    *(u8*)(record + 0x273) = 0;
    0x80195380(*(u32*)0x8042A488, 0x7C, 1);
    0x8019CA00(*(u32*)0x8042A4A8, 2);
    0x807C3AF0(this);                     // +0x4FF = 1
    0x807C3A70(this);                     // +0x503 = 1, +0x502 = +0x505 = 0
    vmethod1(this + 0x70, 0x18, base + 0x120);
    return;
}
if *(u8*)(this + 0x53F) != 0 { vmethod1(this + 0x70, 0x18, base + 0x560); return; }
if *(u8*)(this + 0x547) != 0 { vmethod1(this + 0x70, 0x18, base + 0x7E0); return; }
if *(u8*)(this + 0x548) != 0 { vmethod1(this + 0x70, 0x18, base + 0x1A0); return; }
if *(u8*)(this + 0x54D) != 0 { vmethod1(this + 0x70, 0x18, base + 0x8A0); return; }

// 2. four-player resync
for i in 0..4 {
    if *(u8*)(this + 0x549 + i) != 0 {
        *(u8*)(this + 0x549 + i) = 0;
        if 0x800B4710(i) != 0 {
            let slot  = *(u32*)(0x80354E60 + i * 4);
            let value = *(u32*)(this + 0x560 + i * 4);
            *(u32*)(0x80354E70 + slot * 4) = value;
            0x80759770(*(u32*)(this + 0x518 + slot * 4), value);
        }
    }
}
```

`vmethod1(this + 0x70, 0x18, rec)` is
`(*(*(u32*)(this + 0x70) + 0x18))(this + 0x70, rec)`.

Three details:

* The eight records, ascending, are `0x808344C0`, `0x80834580`, `0x80834600`,
  `0x808346C0`, `0x808349C0`, `0x80834B80`, `0x80834C40`, `0x80834D00`; the
  dispatch order of the branches above is *not* the address order.
* `+0x53E`'s block is the only one that writes: `record + 0x271 = 1`,
  `record + 0x268 = +0x550`, `record + 0x273 = 0`, then two sound calls and the
  two small setters.
* The tail call's second argument is the *same word* that was written into the
  shared table (`r4` still holds it at `0x807C24D0`), so
  `0x80759770(obj, value)` gets the moved value, not a copy of the old one.

### Constants

| address | value | use |
|---|---|---|
| `0x8083_4460` | base | the eight state records are `+0x60`, `+0x120`, `+0x1A0`, `+0x260`, `+0x560`, `+0x720`, `+0x7E0`, `+0x8A0` |
| `0x8042_A488` | cell | sound state object for `0x80195380` |
| `0x8042_A4A8` | cell | sound state object for `0x8019CA00` |
| `0x80354E60` | `u32[4]` (BSS) | player-index -> slot table |
| `0x80354E70` | `u32[]` (BSS) | per-slot value table |
| `0x807D1018` | vtable | installed at `+0x74` by the constructor |
| `0x80821A08` | vtable | installed at `+0x70` by the constructor |
| `0x8083_4470` | `u32` (BSS) | "current actor" slot written by the constructor |
| `0x8083_4480` | BSS | state machine record handed to `0x8015FC90` |
| - | `0x7C` (124) | `0x80195380` sound id |
| - | `1`, `2` | `0x8019CA00` group states |
| - | `0x18` | state interface vtable slot |
| - | `4` | player count |

The handler table that names this body is a run of 12-byte
`{0, -1, target}` records starting at `0x80821AE8`:

| record | address | target |
|---:|---:|---:|
| 0 | `0x80821AE8` | `0x807C20D0` |
| 1 | `0x80821AF4` | `0x807C2070` |
| 2 | `0x80821B00` | `0x807C2060` |
| 3 | `0x80821B0C` | `0x807C2170` |
| 4 | `0x80821B18` | `0x807C20F0` |
| 5 | `0x80821B24` | `0x807C20E0` |
| 6 | `0x80821B30` | `0x807C2290` |
| 7 | `0x80821B3C` | `0x807C21B0` |
| 8 | `0x80821B48` | `0x807C2180` |
| 9 | `0x80821B54` | `0x807C2500` (`blr` stub) |
| 10 | `0x80821B60` | `0x807C22C0` (this row) |
| 11 | `0x80821B6C` | `0x807C22B0` |
| 12 | `0x80821B78` | `0x807C2590` |
| 13 | `0x80821B84` | `0x807C2520` |
| 14 | `0x80821B90` | `0x807C2510` (`blr` stub) |

### Callees

`0x802DCD70`/`0x802DCDBC` (`__savegprlr_26`/`__restgprlr_26`), `0x8019CA00`
(`audio::ax::fn_8019ca00`, "write group state"), `0x80195380`
(`audio::ax::fn_80195380`, `(manager, id, arg)`), `0x807C3AF0` (12 bytes:
`*(u8*)(this+0x4FF) = 1`), `0x807C3A70` (24 bytes: `+0x503 = 1`,
`+0x505 = 0`, `+0x502 = 0`), `0x800B4710` (`game::stage::fn_800b4710`,
"is course word 3"), `0x80759770` (`fn_80759770`, `+0x238 = value` then
`model->vtable[+0x30](value)`), and the `+0x18` slot of `+0x70`.

### Notes for the implementer

* `+0x53E`'s block calls `0x8019CA00` **after** `0x80195380`, and the two
  setters **before** the state switch; keep that order.
* `0x807C3A70` writes three bytes in the order `+0x503`, `+0x505`, `+0x502`;
  the stores do not alias.
* `0x800B4710(i)` is `countLeadingZeros(*(u32*)(manager + i*4 + 0x384) - 3) >> 5`,
  i.e. `1` exactly when the word is `3`.
* `slot` from `0x80354E60` is used as a *word index* for both tables but as a
  word index into `this` for `+0x518`; there is no bound check.
* All eleven request bytes are plain `u8` flags that some other body sets; this
  body only clears `+0x53D` (branch 2) and `+0x549 + i` (resync). Do not clear
  the others.

**Verdict: ready (medium).** All 144 words decoded, every offset, sound id and
state record read, and both unledgered neighbours (`0x807C3AF0`, `0x807C3A70`)
quoted in full; medium because the eleven request flags' setters live outside
this REL body set and the two `0x80354E6x` tables are `main.dol` BSS.

---

## Cross-checks performed

* Address mapping re-derived from both ELF program-header tables (quoted above);
  every literal was read through it. `0x8082B320`, `0x808340F8`, `0x80834138`,
  `0x80834460..0x80834D40` read through the REL BSS zero fill and are marked as
  BSS where they occur.
* `ledger/functions.csv` contains no row for any of the eight (checked against
  the full 17,170-address set, case-insensitive).
* No `#{2,4} ` heading of any of the 27 `docs/SPEC_WAVE*_BASES.md` files that
  existed before this one contains any of the eight (278 heading addresses).
  The prose appearances are the previous lanes' skip lists listed in the
  selection section; `docs/SPEC_WAVE141_BASES.md:240` is the only one that
  identifies a row (`0x807BBE20` as `dWorldSelect_c::StateID_OnStageWait`).
* `src/game/bases.rs` has no `fn fn_<addr>` definition for any of the eight
  (3,782 `fn fn_` tokens at the verification pass). No doc comment in that file
  mentions any of the eight either.
* Extent/`blr` recomputation for all eight is in the table near the top; all
  eight extents are exact, so the `docs/REL_ELF_MAPPING.md:37-42` correction is
  recorded as *not* needed rather than skipped silently.
* Every constant table value was read from the image (`d2.js`/`cx.js`) or from
  `main.dol` through its own program headers (`md.js`/`mdx.js`), including the
  five `CS_W5_Ibara_XY` strings, the four `hanachan` strings, the four
  `0x803504E0` per-world `u16` rows, the `0x80821540`/`0x808215F0` camera rows
  and the fifteen `{0,-1,target}` records.
* The three `psq`-bearing rows (1, 5, 6) were re-read at the bit level: opcodes
  56/60 with quantiser 0 and `W = 0`, i.e. no data-side quantisation, so no
  `GQR` pin is needed and the wave-88 rejection of row 1 does not survive.
* `src/game/bases.rs` and `ledger/functions.csv` are being edited by other lanes
  concurrently (the `fn fn_<addr>` count moved from 3,771 to 3,782 during this
  session because of the wave-155 coverage pass, none of them in this set), so
  this document quotes addresses and strings instead of line numbers.
* This lane wrote only this file.
