# Coverage - Wave 114 (`game::worldmap`, `0x80110000` bucket close-out)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
course-background object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is
`SDA2_BASE` (`0x80433080`); the bodies reuse the module's already-verified
`W89_*`, `W100_*`, `W106_*` and `W107_*` constants instead of declaring
duplicates. Only one new callee constant is introduced
(`W114_FUN_8016A130`), plus two host-side matrix wrappers.

The bucket had exactly three untranslated entries at the start of this pass. All
three are written here, which closes the `0x80110000` bucket (190 of 190
entries in `ledger/functions.csv`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 3 | 3,768 | 0 | 2 |

Every row is `translated`; none is a `stub` or `partial`. Two rows are `medium`
and one (`0x801194A0`) is `low`; the difference is explained under
"Confidence and the one reconstruction gap".

## What this pass covers

Waves 89/100 rebuilt the WorldMap course-background object, and Wave 106 added
its balloon/ice-object neighbours, but these three records were left out because
their exports are dominated by `GQR` conversion blocks and by
`FUN_802DCD54`/`FUN_802DCD60`/`FUN_802DCDA0`/`FUN_802DCDAC` calls. Neither is
game code:

- The `GQR` blocks are the decompiler's rendering of `psq_st`/`psq_l` saves and
  restores of the non-volatile paired-single registers `ps28..ps31`. The stack
  words they build are never read by the bodies.
- `0x802DCD54`/`0x802DCD60` are `__savegprlr_21`/`__savegprlr_22` and
  `0x802DCDA0`/`0x802DCDAC` are the matching `__restgprlr_*` helpers. They do
  not write `r3`, so the object each record works on is the incoming `r3` and is
  modelled as `this`. This is the same convention Wave 3 documented for
  [`wm_effect_slot_alloc`] and Wave 109 for the other `0x802DCD*` helpers.

The three records are one cluster and share one field map, which the module
already uses in `0x80116D20`/`0x80118DC0`/`0x80118450`:

| field | role |
|---|---|
| `+0x3B8` | pointer to the 66 x 34 grid of `0x10`-byte cells `[x, y, z, class]` |
| `+0x3C4 + class*0x24 + slot*4` | the nine tile records of each of the three classes |
| `+0x430` | one extra tile record, used at column `1` of class `+0x434` |
| `+0x434` | class whose second column uses `+0x430` |
| `+0x480`/`+0x484` | cell X/Y offsets added by the rail lookup |
| `+0x498`/`+0x49C` | rail rotation terms subtracted in the matrix chain |
| `+0x5E8 + class` | six-bit mask of the clip slots a class owns |
| `+0x5F0` | rail-matrix variant selector |
| `+0x5F4`/`+0x5F6` | per-clip selector halfwords forwarded to `0x80168230` |
| `+0x5F8`/`+0x5FA` | current grid column / first non-empty grid row |
| `+0x5FC..+0x628` | twelve-word rail matrix rebuilt by `0x801194A0` |

A cell whose class word is `9` is empty. `0x801194A0` tests that value through
the export's float comparison `pfVar4[3] != 1.26117e-44`: that constant is
exactly the `f32` bit pattern `9` (`9 * 1.401298e-45`), not a scale factor, and
`0x80119C50` compares the same word to `9` as an integer.

- **`0x80119240` - the tile visibility/refresh pass (608 bytes).** Gated on
  `0x800B3B50(0xFFFFFFFF) == 0`, on the low input nibble at `r13-0x5986` and on
  `+0x4E4 != 1`. It walks three groups x nine records x two clips; a record
  takes part only when its head word is non-zero and its `+0x11` and `+0x14`
  bytes are set. When the collider's `+0x30` word was non-zero the body clears
  it and takes the first-frame path: each clip's position is taken from
  `0x80168230`, the volcano effect is applied through the `+0xA0` slot
  ([`wm_volcano_effect_apply`]), the clip closest to the projected scene centre
  is remembered, and sound event `0x2CD` is posted at it through `0x8006A4B0`
  and `0x80198C30`. Later frames only refresh the per-clip bind byte through
  the `+0xB0` slot ([`fn_801191e0`]).
- **`0x801194A0` - the rail matrix rebuild (1,960 bytes).** Scans the grid for
  the first non-empty cell of column `+0x5F8`, stores its row in `+0x5FA`,
  adds `+0x480`/`+0x484` to the cell X/Y, reads the base position through
  `0x8011A100` and the size class through `0x8011A1A0`, then composes
  `+0x5FC` from eight to eleven translation/scale/rotation matrices
  (`0x801C0BD0`, `0x801C0C60`, `0x801C0540`). A zero size class selects the
  short fallback chain; otherwise `+0x5F0` selects the variant that drops the
  horizontal cell term.
- **`0x80119C50` - the on-screen tile binder (1,200 bytes).** Clears the
  "on screen" byte at `+0x14` of the 27 class records and of the `+0x430`
  record, then walks all 2,244 grid cells. For a non-empty cell it rebuilds the
  twelve-word matrix from `+0x5FC` with the translation moved by the cell's
  column/row distance, projects each clip enabled by the per-class mask into
  the rectangle `(scene+0x38, scene+0x3C - scene+0x44)` ..
  `(scene+0x38 + scene+0x40, scene+0x3C)` through `0x800B3100`, marks the next
  free record of the class, and binds the matrix through `0x8016A170`,
  `0x8016A130` and `0x8016A1A0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80119240` | `fn_80119240` | 608 | medium |
| `0x801194A0` | `fn_801194a0` | 1960 | low |
| `0x80119C50` | `fn_80119c50` | 1200 | medium |

The three sizes sum to 3,768 bytes.

## Confidence and the one reconstruction gap

`0x80119240` and `0x80119C50` are `medium` only because some field roles are
inferred (the `+0x11`/`+0x14`/`+0x10` tile-record bytes, the `+0x5F4`/`+0x5F6`
per-clip halfwords and the `+0x5F0` variant selector); every guard, constant,
offset and call is literal in the export.

`0x801194A0` is `low` for one concrete reason: four of its `0x801C0BD0` call
sites lose all three `f64` arguments in the export
(`FUN_801c0bd0(param_1 + 0x5fc)` and the two `auStack_*` sites, plus the
fallback-chain site). The decompiler prints the matrix pointer, which is the
fourth argument, and drops the register-passed translation triple. The dropped
operands are reconstructed as the all-zero triple that every identity build in
this module uses (for example [`fn_8011f1d0`]), and every other operand of the
chain is transcribed literally. The export alone cannot confirm those four
triples, so the record is marked `low` and the four sites carry an
`// observed:` note.

## Export-versus-stream differences

All three exports carry "removing unreachable block" warnings. Each spot that
matters is marked `// observed:` in the body:

- **`0x801194A0` class test.** The export prints the empty-cell marker as the
  float `1.26117e-44`; that is bit pattern `9`, which is also what
  `0x80119C50` compares against.
- **`0x801194A0` dead base-position stores.** The export adjusts
  `local_534`/`local_530`/`local_52c` in the non-zero size-class branch but
  only reads the unadjusted pair in the fallback chain, so those stores are
  dead; they are kept and annotated.
- **`0x80119240` prologue.** `iVar9 = FUN_802dcd60()` is
  `__savegprlr_22`; the object is `r3`, not a return value.
- **`0x80119C50` null guards.** The export dereferences the 27 `+0x3C4`
  pointers and `+0x430` unconditionally. The Rust body guards the clears and
  the `+0x430` comparison so a zeroed field cannot trap on the host; the
  constructor fills all of them, so the guard never changes shipped behaviour.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`;
  the three sizes sum to 3,768 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path
  (`80119240_FUN_80119240.c`, `801194a0_FUN_801194a0.c`,
  `80119c50_FUN_80119c50.c`).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all three were the remaining
  untranslated entries of the `0x80110000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave114` reports `wave114-worldmap.csv: 3 rows`
  and `OK`.

## Deliberate skips

None. The `0x80110000` bucket is closed by this pass.

[`wm_effect_slot_alloc`]: crate::game::worldmap::wm_effect_slot_alloc
[`wm_volcano_effect_apply`]: crate::game::worldmap::wm_volcano_effect_apply
[`fn_801191e0`]: crate::game::worldmap::fn_801191e0
[`fn_8011f1d0`]: crate::game::worldmap::fn_8011f1d0
