# Wave 225, lane B - `src/game/stage.rs`, addresses `0x800AADE0` and `0x800AB590`

Two entries of `work/ghidra/main-gb/export/functions.tsv`, the last open
`game::stage` bodies of the `0x800A0000` actor band. Both are translated;
none deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x800AADE0` | 148 | 148 (`bclr` at `0x800AAE70`) | submit one effect through the actor's `+0xAF0` `+0x20` and `+0x24` handles at `(x, y, 5500.0f)` with a three-word factor block |
| `0x800AB590` | 180 | 180 (`bclr` at `0x800AB640`) | probe the map cell at `+0xAC`, `+0xB0 - 8.0f` and `+0xB0 - 24.0f`; return `1` on attribute `1`/`0x8000`, else `0` |

Bytes translated: 328.

Neither `functions.tsv` size is a prologue artefact: both bodies end in a real
`bclr 20,0` after a matching `addi r1,r1,<frame>` (`0x30` and `0x40`).

## What the two bodies are

* `0x800AADE0` is a twin-handle effect submit. The actor's `+0xAF0` object
  holds two effect-resource handles at `+0x20` and `+0x24`; each non-null
  handle gets the same five-argument `0x8016C890` request: `(handle, 0,
  &position, 0, &factor)`. `position` is `{+0xAC, +0xB0, 5500.0f}` and
  `factor` is the incoming `f1` single stored three times. Ghidra types the
  argument as `double`, but the three `stfs f1,...` stores prove it is one
  `f32`.
* `0x800AB590` is a two-sample map query. It keeps the actor's `+0xAC` as
  `x`, starts `z` at `+0xB0 - 8.0f` and steps `+16.0f`, so the samples are
  `+0xB0 - 8.0f` and `+0xB0 - 24.0f`. Each sample calls `0x80070BA0` with the
  actor's `+0x38F` byte as the third argument (the same argument shape as
  [`fn_800951d0`] uses for the `0x80075270` ground query). A probe answering
  exactly `1` or exactly `0x8000` ends the loop with `1`; two misses give `0`.
  The `cmpi cr0,31,r31,2` / `bc 12,0` pair at `0x800AB608`/`0x800AB60C` is the
  literal loop bound.

## Constants

Reused from earlier lanes (no duplicates introduced):

* `W29_EFFECT_PATH` = `0x8016C890`
* `W61_CALL_80070BA0` = `0x80070BA0`
* `W224B_F_5500` = `r2 - 0x6C44` = `0x8042C43C`, image word `0x45ABE000`
  (`5500.0f`)
* `W224_F_RESULT_LIMIT_C` = `r2 - 0x6C58` = `0x8042C428`, image word
  `0x41800000` (`16.0f`); here it is the `z` step, not a result limit
* helpers `read_u32`, `read_f32`, `read_u8`, `platform::call5`,
  `call_f32x2_u32x2`, `SDA2_BASE`

New:

* `W225_F_PROBE_GAP_B` = `r2 - 0x6C34` = `0x8042C44C`, image word `0x41000000`
  (`8.0f`), read by `lfs f0,0xffff93CC(r2)` at `0x800AB598`.

Note: the wave-224 lane A comment on `W224_F_ZERO_6C44_A224` claims the slot
`0x8042C43C` holds `0.0f`; the image word is `0x45ABE000` (`5500.0f`), which
`W224B_F_5500` records correctly. This lane reuses `W224B_F_5500` and does not
touch the other constant.

## Verification

* Sizes checked against the prologue and `blr` in `work/dol/main.elf`
  (`stwu`/`bclr` pairs listed above); both equal `functions.tsv`.
* The `unaff_GQR0`/`ldexpf` noise and the three "removing unreachable block"
  warnings in the `0x800AB590` export are `psq_l`/`psq_st` spill-slots; the
  raw words are ordinary `lfs`/`stfs`/`fsubs`.
* Block: `work/wave225/laneB_block.rs`. Appended to a scratch copy of `src/`
  (`%TEMP%\opencode\wave225-laneb-crate`); `cargo check --lib` exits `0`
  (26 pre-existing warnings unchanged).
