# Coverage - Wave 150 (`game::enemies`)

## How to read this file

This page records the Wave 150 `game::enemies` lane: 20 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 132 and mirrored one
row per body in `ledger/agents/wave150-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` headers as
described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment), and every
body was followed from its entry to its terminal instruction. Byte counts are the
`size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 20 | 5716 | 13 | 7 |

Status split: 20 `translated` (13 high, 7 medium), no `partial` and no `stub`.

Covered address range: `0x80A04D10..0x80B2A360` (3,928 bytes of
`docs/SPEC_WAVE149_ENEMIES.md` rows, 624 bytes of `0x80B20000` planner-bucket
rows and 1,164 bytes of the four rows below that page's size cut).

## What this pass covers

- **All twelve `docs/SPEC_WAVE149_ENEMIES.md` rows**, including the one marked
  *conditional*: `0x80A04D10` (the `daEnBuku_c` step that latches `SrcSwim` on a
  120-unit contact), `0x80B28F90` (the pivoted three-axis matrix build),
  `0x80A334D0` (the `ankou` per-frame step), `0x80B05740` (the world-map rail
  actor's rebind with its live-count/cap recompute), `0x80B25280`/`0x80B253D0`
  (the x/lateral and y/pitch distance-band effect steps), `0x80B2A360` (the
  four-player x-band occupancy flag), `0x80AD5CD0` (the contact-mask-gated
  two-state emitter), `0x80A62A30` (the `gesso` frame-`18.0` speed latch),
  `0x80AF16A0` (the four-player proximity effect `60`), `0x80A2FA30` (the
  `0x6950`-byte array constructor) and `0x80AC8850` (the side-based hit-record
  builder).
- **All four remaining implementation-ready bodies of the `0x80B20000` planner
  bucket**: `0x80B20010` (proximity effect `0xBD`), `0x80B22750` (the halfword
  angle ground clamp with its `qr3` widen), `0x80B23890` (the per-index clip
  rebind) and `0x80B28100` (the seventeen-slot child placement). The bucket's
  other two addresses are listed under "Deliberate skips".
- **Four rows below the specification's size cut**: `0x80A638F0` and
  `0x80A63A30` (the `gesso`-block left/right edge walks with their two forward
  probes), `0x80AEE850` (the `0x718`-byte animator actor constructor) and
  `0x80A390F0` (the eight-band state initialiser with the ten-argument
  `0x800DB180` install).

### `qr3` sites

One data `qr3` site is ported, the unsigned-halfword widen pinned by
`docs/REL_ELF_MAPPING.md` (`GQR3 = 0x00050005`, `W = 1`, `I = 3`):

| address | sites | port |
|---|---|---|
| `0x80B22750` | `0x80B22784` | the `+0x550` halfword is re-read as one `u16` and rendered `(f32)*(u16*)`, exactly what `psq_l f1,0(r1+0xA),qr3,W=1` loads |

Every other `psq_*` in the 20 rows is a `qr0` `f30`/`f31` stack spill
(`stfd`/`lfd` beside it) and carries no quantisation.

### Specification corrections made while porting

- **`0x80AC8850` speed operand.** `docs/SPEC_WAVE149_ENEMIES.md` step 4 writes
  `(table[side] + 0.4 * *(f32 *)(this+0xE8)) * scale`. The raw stream at
  `0x80AC8870` is `lfs f0,0xE8(r28)` and `r28` is the second argument
  (`mr r28,r4`), while `r27` is `this`; the record stores all go to `r27`. The
  port therefore reads `other + 0xE8` and says so in the body and in the row's
  confidence reason (still `medium` because the role of the field, not its
  address, is the inferred part).
- **`0x80B23890` record shape.** The export drops the clip-name pointer, but the
  raw stream writes a three-word record before `0x801651F0`:
  `stw r4,0x10(r1)` (name from `0x80B328B0 + index*8`), `stw r0,0x14(r1)`
  (the pair's second word) and `stw r3,8(r1)` (the `0x8023A0B0` clip id). The
  port passes all three words. The export's `0x80B328B4 + index*8` and the raw
  `lwz r0,4(r5)` read different addresses, but all four table entries hold `0`,
  so both readings are behaviourally identical; the body notes this.
- **`0x80B28100` signature.** The export prints `void FUN_80b28100(void)`
  because `_savegpr_24` is modelled as a 64-bit return. The prologue contains
  `mr r26,r4`, and `add r0,r26,r0` folds that incoming word into every `+0x102`
  heading, so the real signature is `(this, base_angle)`.

### Shared helpers reused

No new file-local ABI adapter was needed: the bodies reuse `read_f32`,
`write_f32`, `call2_ret_f32`, `call0_ret_u32`, `call2_f64`, `call3_f64`,
`call4_f64`, `call5`, `call3_f64_f64`, `call4_f64_f64_f64`, the wave-130
`w130_state_call`/`w130_rotate` adapters and the wave-101 ten-argument
`w101_call10_f64x5` installer used by `0x80A390F0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A04D10` | `fn_80a04d10` | 360 | high |
| `0x80A2FA30` | `fn_80a2fa30` | 308 | high |
| `0x80A334D0` | `fn_80a334d0` | 352 | high |
| `0x80A390F0` | `fn_80a390f0` | 300 | medium |
| `0x80A62A30` | `fn_80a62a30` | 312 | high |
| `0x80A638F0` | `fn_80a638f0` | 284 | high |
| `0x80A63A30` | `fn_80a63a30` | 284 | high |
| `0x80AC8850` | `fn_80ac8850` | 304 | medium |
| `0x80AD5CD0` | `fn_80ad5cd0` | 316 | high |
| `0x80AEE850` | `fn_80aee850` | 296 | medium |
| `0x80AF16A0` | `fn_80af16a0` | 312 | high |
| `0x80B05740` | `fn_80b05740` | 328 | high |
| `0x80B20010` | `fn_80b20010` | 152 | medium |
| `0x80B22750` | `fn_80b22750` | 128 | medium |
| `0x80B23890` | `fn_80b23890` | 156 | medium |
| `0x80B25280` | `fn_80b25280` | 328 | high |
| `0x80B253D0` | `fn_80b253d0` | 328 | high |
| `0x80B28100` | `fn_80b28100` | 188 | high |
| `0x80B28F90` | `fn_80b28f90` | 356 | high |
| `0x80B2A360` | `fn_80b2a360` | 324 | medium |

### Why seven rows are `medium`

- **`0x80B2A360`** - the mode test (`clz(mode-1)`), the band arithmetic, the
  four-slot loop and the single flag store are exact; the roles of
  `0x80429DD0 + 0x909CB` (scene occupancy byte) and
  `0x8042A1C8 + 0x120C..0x120E` (mode triple) live in `main.dol` and are
  `// inferred:`.
- **`0x80AC8850`** - the record, the template words and the multiply order are
  literal; the specification's `this+0xE8` reading is corrected to `other+0xE8`
  from the raw stream, and the scale arrives in `f1` where the export shows
  `extraout_f1`.
- **`0x80B20010`** - the record, the two offsets and the effect id are literal;
  the index returned by `0x80096240` selects among entries of `0x80B32760` whose
  count the export does not bound.
- **`0x80B22750`** - the `qr3` widen, the `0x300` step, the `1/256` scale and
  the surface offset are literal; the angle's role beyond "phase of a vertical
  bob" is inferred from the single call site.
- **`0x80B23890`** - every call and the `1.0` rate are literal; the third record
  word has the two address readings documented above.
- **`0x80AEE850`** - every store and call is literal, but the two five-word
  `0x8015FC90` records are animator/controller bindings only by inference, and
  the class name is not printed anywhere in this REL.
- **`0x80A390F0`** - the stores, the five installer doubles and the state word
  are literal; the eight-band constant roles are inferred from their positions.

## Deliberate skips

- **`0x80B27DC0` (272 bytes) and `0x80B27ED0` (432 bytes).** These two are the
  last untranslated bodies of the `0x80B20000` bucket. Both centre on the
  `2^52` integer-to-double fold at `0x80B27E3C..0x80B27E50`, and that block does
  not decode into a consistent expression: the raw words are
  `fsubs f3,f0,f3` followed by `fdivs f1,f2,f1`, i.e. `(16384 - 2^52)` divided by
  `2^52 + value`, while the Ghidra export prints `(0.4 * value) / 16384`. The two
  readings differ by seventeen orders of magnitude, so any body written here
  would be a guess rather than a translation. `0x80B27DC0` is a `f64 -> f64`
  helper and `0x80B27ED0` is its two-branch caller, so the pair needs one focused
  pass that resolves the register tracking; the other fourteen bucket addresses
  in the `0x80B20000` range are now translated.
- **`0x80A390F0`'s sibling emitters were not claimed.** Only the state
  initialiser was taken from that block; the surrounding effect records belong to
  the wave-101 subsystem and are outside this lane's selection.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 20 sizes sum to 5,716
  bytes (3,928 for the twelve specification rows, 624 for the four bucket rows
  and 1,164 for the four below-cut rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; all 20 addresses were absent from
  `ledger/functions.csv` and from `src/**/*.rs` before this pass.
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- The single data `qr3` site of `0x80B22750` is ported as `(f32)(u16)`; every
  other `psq` in these rows is a `qr0` spill.
- The module header was updated: cluster item 132, the cluster count (131 to
  132), the function totals (3431 to 3451 public translations, 3360 to 3380
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave150` reports 20 rows for this lane, 20
  unique addresses and `OK`.
