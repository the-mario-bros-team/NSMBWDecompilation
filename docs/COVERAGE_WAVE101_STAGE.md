# Coverage - Wave 101 (`game::stage`, the `0x80070000` ground and tile-map tail)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave101-stage.csv` is the Ghidra C export each body was
written from. Field roles the export only hints at stay marked `observed`,
`inferred` or `unknown` in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 17 | 8248 | 0 | 17 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`. The pass covers `0x80070530..0x8007CA90`.

## What this pass covers

- **The tile-command pipeline** (`0x800709C0`, `0x80077860`, `0x80078300`).
  `0x800709C0` is the mask-guarded wrapper around the `0x80070BA0` probe and the
  `0x80076630` scan (note the `param_4`/`param_3` swap at the call). `0x80077860`
  upgrades command kind `2` to `4` in the special stage mode, resolves the cell
  through `0x80077520`, patches the two command bits `0x8A`/`0x8B` when
  `0x80083490` returns `0xFFFF`, and otherwise stores the id and mirrors it with
  `0x80083380`. `0x80078300` ticks the 256-entry `0x10`-byte effect timer table,
  re-applies expired tiles through `fn_80077860`, and submits effect `0x239`
  with `0x80198C30` from the two `r2-0x7360`/`r2-0x7350` word-to-double
  conversions.
- **The BG tile-map builders** (`0x80077B40`, `0x80077EB0`). `0x80077B40`
  derives the three map spans from `+0x8FE64..+0x8FE70` with the
  `(v & 0xF) == 0` rounding, adds the two extra columns in game mode `0`, and
  builds the three maps plus the `0x28`-byte overlay node. `0x80077EB0` fetches
  the unit model with `0x80081850`, allocates the `0x44`-byte map, samples every
  cell with `0x800775A0`, and remaps the `Pa1_nohara*` tile ids `0x1A0..0x1C2`
  into the two `+0x8F480`/`+0x8F930` effect lists.
- **The ground and slope walkers** (`0x80070530`, `0x80070EF0`, `0x80071060`,
  `0x800717A0`, `0x80071DC0`, `0x80072440`). These share the two direction
  probes `0x80070BA0`/`0x80070BF0` and the `0x80070A40` slope probe.
  `0x80071060` converts the two grid words and accumulates `1 << direction` /
  `0x10 << direction`; `0x800717A0` uses the `r2-0x73B4` tolerance and the
  `2`/`3`/`0x12` kind ladder before the `0x800CFEA0` area probe; `0x80071DC0`
  builds the six-point block from `0x802EF610` masks and publishes `+0xD0`/
  `+0xD4`; `0x80072440` walks the slope with the `>>3`/`>>4` cell compares and
  the `0x8015F480` step.
- **The scroll-layer list hits** (`0x800722C0`, `0x80072790`, `0x800766F0`,
  `0x800768D0`). All four walk `r13-0x58A0` through the `+0xDD`/`+0xDE` mask,
  `+0xD4` and `+0xCC` filters and the four-word `0x80080900` segment test.
  `0x800722C0` adds the direction-dependent `+0xA0` window, `0x800768D0` builds
  the four-quad sweep and calls the entry's `+0x40` method, and `0x800766F0`
  returns the matched entry's first word.
- **The player-near test and selector** (`0x80078670`, `0x8007CA90`).
  `0x80078670` checks the four `0x8005F900` slots against the `0x8005FC70` mask
  and the three `r2` near-plane offsets. `0x8007CA90` collects the four slots,
  drops kind `1` (and kind `3` when `+0x90079` is `1`), sorts them through the
  `0x802EF8B0` comparator and writes the `0`/`1`/`2`/`3` arms of the result
  table.

Address range: `0x80070530..0x8007CA90` (17 bodies, 8248 bytes).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80070530` | `fn_80070530` | 120 | medium |
| `0x800709C0` | `fn_800709c0` | 128 | medium |
| `0x80070EF0` | `fn_80070ef0` | 360 | medium |
| `0x80071060` | `fn_80071060` | 428 | medium |
| `0x800717A0` | `fn_800717a0` | 696 | medium |
| `0x80071DC0` | `fn_80071dc0` | 960 | medium |
| `0x800722C0` | `fn_800722c0` | 384 | medium |
| `0x80072440` | `fn_80072440` | 836 | medium |
| `0x80072790` | `fn_80072790` | 296 | medium |
| `0x800766F0` | `fn_800766f0` | 244 | medium |
| `0x800768D0` | `fn_800768d0` | 736 | medium |
| `0x80077860` | `fn_80077860` | 288 | medium |
| `0x80077B40` | `fn_80077b40` | 568 | medium |
| `0x80077EB0` | `fn_80077eb0` | 924 | medium |
| `0x80078300` | `fn_80078300` | 272 | medium |
| `0x80078670` | `fn_80078670` | 272 | medium |
| `0x8007CA90` | `fn_8007ca90` | 736 | medium |

## Deliberately skipped

The bucket still holds 25 untranslated functions after this pass (42 at the
start). Skipped on purpose:

- **Paired-single dequantisation bodies** (`0x80071210`, `0x800728C0`,
  `0x80073290`, `0x80073670`, `0x80073BC0`, `0x80073FC0`, `0x80074470`,
  `0x80074AF0`, `0x80075820`, `0x80075CA0`, `0x80075FD0`, `0x80079E80`,
  `0x8007AC40`, `0x8007CD70`, `0x8007E4E0`, `0x8007FFA0`). Each reads its float
  arguments through the `CONCAT13`/`CONCAT22` GQR forms of `in_f31`/`in_f30`/
  `in_f29`, i.e. a value the export cannot present as a normal argument. The
  wave-100 stage lane already refused the same shape; deriving the dequantised
  argument would be an invention.
- **The `extraout_f1` bodies** (`0x80070130`, `0x80071A60`, `0x8007FA40`). They
  consume `extraout_f1` (or the second register half of a save helper) as a real
  argument of `0x8006FEA0`/`0x800CFEA0`/`0x8007FBF0`, so the first argument of
  those calls cannot be grounded.
- **Bodies whose export loses the incoming stack/register pair**
  (`0x80077700`, `0x80077730`, `0x800777B0`, `0x80077810`). `0x80077700`
  returns the `r4` half of `0x80077650`; `0x80077730` returns `extraout_r4`;
  `0x800777B0` reads `local_14` and `in_r6` before they are written; and
  `0x80077810` tests `local_6 & local_8` before either local is written.
- **`0x8007A6C0`.** The 1200-byte body is mechanically convertible (its eighteen
  `ldexpf` prologue/epilogue calls are dead), but it spans the whole `+0x8FE44..
  +0x90120` camera block and was not attempted in this pass; it is the next
  natural follow-up for this bucket.
- **`0x800776F0`.** A 4-byte thunk of `0x80077650`, which already has a ledger
  row; per the wave-84/98/99/100 precedent thunks of translated bodies are not
  counted twice.

None of these is counted as translated in the CSV.

## Verification

- Every address and byte size matches `functions.tsv` (17 rows, 8248 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the 17
  addresses were chosen from the 42 open entries of the `0x80070000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the errors it reports are in other lanes' files:
  `ui::hud` and `game::worldmap`).
- `node tools/check-lane-csv.mjs wave101` prints `OK`; the wave-101 stage rows
  are clean.
