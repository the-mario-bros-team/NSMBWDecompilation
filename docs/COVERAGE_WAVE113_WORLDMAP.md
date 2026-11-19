# Coverage - Wave 113 (`game::worldmap`, `0x80130000` bucket close-out)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); the bodies reuse the module's already-verified `R2_*`/`W98_*`/
`W104_*` constants and the [`w102_ease`] wrapper instead of declaring duplicates.

The bucket had exactly four untranslated entries at the start of this pass. All
four are written here, which closes the `0x80130000` bucket (226 of 226
entries in `ledger/functions.csv`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 4 | 2,268 | 0 | 4 |

Every row is `translated`; none is a `stub` or `partial`.

## What this pass covers

Wave 36 skipped `0x8013B0B0` as a "GQR-reconstruction record" and the other
three were simply the tail of the bucket. Re-reading the instruction stream in
`work/dol/main.elf` shows that every `psq_l fD,0(..),0,1` in these records is a
16-bit quantised load whose net effect is a halfword converted to `float` and
scaled by `1/256` — the same conversion the module already models as
`read_i16(..) as u16 as f64` in [`fn_8011dab0`] and its neighbours. No state was
missing, so all four records are reconstructed:

- **`0x80130F40` - the ride steering/easing pass (1,160 bytes, the largest
  record in the bucket).** Resolves the ride through `0x802DCD70`, derives the
  `busy`/`riding` booleans from the `+0xEA4` sub-controller, and builds a
  steering step of `0x3E00`, `0x2000` or the negated `+0x2070` counter. The step
  is converted with the `0x80237BD0` cosine into `+0x206C`; two rate/limit
  ladders over the `0x802F5748` table then drive the `0x8015F140` easing of
  `+0xEC` and the second-stage tilt speed stored at `+0x2068`, followed by
  `+0x110`, `+0x11C`, `+0x114`, `+0x118` and the `+0x2064` gate.
- **`0x80137320` - the per-frame turn/look-at update.** Guards on action `0x36`,
  on `+0x10D4` bit `0x180000` and on `+0x1508`; samples the `+0x14FC` angle
  twice and mirrors it for facing `1`; rebuilds the `+0x2A6C`/`+0x2A70` tilt
  pair from the `+0x102` yaw; runs the landing attempt [`fn_801363c0`] and
  re-posts the `+0x14A4`-indexed animation entry when it declines.
- **`0x80138510` - the mount/pose setup.** Clears action `3`, posts event
  `0x27`, plays animation `0x157`, emits the `+0x38D` sound through the
  `r13-0x5678` audio object, loads the three 30-frame timers, and selects the
  `+0x410` or `+0x3FC` pose slot from `+0x10D4` bit `0x4000`.
- **`0x8013B0B0` - the ground-relative X/Y offset builder.** Copies the target's
  transform through `0x80152480`, stores the target X/Y at `+0xAC`/`+0xB0`, and
  writes `dx / cos(+0x102 yaw)` to `+0x2774` and `dy` to `+0x2778`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80130F40` | `fn_80130f40` | 1160 | medium |
| `0x80137320` | `fn_80137320` | 492 | medium |
| `0x80138510` | `fn_80138510` | 416 | medium |
| `0x8013B0B0` | `fn_8013b0b0` | 200 | medium |

The four sizes sum to 2,268 bytes.

## Export-versus-stream differences

All four exports carry "removing unreachable block" warnings and two of them
drop or invert an operand. Each spot is marked `// observed:` in the body:

- **`0x8013B0B0` halfword load.** The export renders the value as an
  `unaff_GQR3` quantised load and prints two float copies that are never read.
  The stream stores `lha +0x102` with `sth r0,8(r1)`, loads it with
  `psq_l f1,0(r3),0,1` and multiplies by `r2-0x5820` (`1/256`).
- **`0x80137320` `+0x104` store.** The export prints `-sVar8` (the mirrored
  angle). The stream's store is `sth r0,0x104(r31)` where `r0` is the `+0x1DEC`
  word; the `neg r3,r0` before it writes a dead register.
- **`0x80137320` mirrored angle.** `neg r3,r0` / `extsb r30,r0` narrows the
  resampled angle to `i8`, which is what actually reaches `+0x100`. The same
  8-bit narrowing is applied by the caller at `0x80130F40` before `0x8015F340`.
- **`0x80130F40` steering step.** At `0x8013100C` the stream negates and
  narrows the `+0x2070` word (`neg r29,r0` / `extsb r29,r0`), not the
  `0x3E00`/`0x2000` step the export's `sVar12 = -sVar12` implies.
- **`0x80130F40` second-stage clamp.** The `|+0x10C| >= 1.5` clamp at
  `0x80131248` is reached only from the `variant == 0` paths; the
  `variant != 0` paths branch straight to `0x80131270`.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`;
  the four sizes sum to 2,268 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path
  (`80130f40_FUN_80130f40.c`, `80137320_FUN_80137320.c`,
  `80138510_FUN_80138510.c`, `8013b0b0_FUN_8013b0b0.c`).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all four were the remaining
  untranslated entries of the `0x80130000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave113` reports `wave113-worldmap.csv: 4 rows`
  and `OK`.

## Deliberate skips

None. The `0x80130000` bucket is closed by this pass.

[`w102_ease`]: crate::game::worldmap::w102_ease
[`fn_8011dab0`]: crate::game::worldmap::fn_8011dab0
[`fn_801363c0`]: crate::game::worldmap::fn_801363c0
