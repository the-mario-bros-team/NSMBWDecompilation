# Coverage - Wave 48 (`game::stage`, stage flags and the player-stat record)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body.

This pass picks up the cluster wave 41 named as deferred in
[`docs/COVERAGE_WAVE41_STAGE.md`](COVERAGE_WAVE41_STAGE.md): the stage-flag
accessors and the player-stat record in the top of the `0x800C0000` bucket.
Two layouts are involved, both *inferred* from the stores below:

- **Stage-flag object** (passed as `base`): `+0x09` and `+0x32` are per-slot
  byte tables; `+0x3C`, `+0x74C`, `+0x774` and `+0x79C` are four byte tables
  indexed `slot * 4 + entry`; `base + area * 0xA8 + stage * 4 + 0x6C` is one
  `u32` of stage bits, with `0..0x2A` stages and three bits per stage.
- **Player-stat record** (0x6A0 bytes, at `r13-0x56D0`): `+0` vtable, `+4`
  state byte (default `0xE`), `+5` spare, `+8..+0x17` four per-player id
  words, `+0x18`/`+0x28`/`+0x38`/`+0x48` four `i32` arrays (one entry per
  player), `+0x58..+0x5B` four player-flag bytes and `+0x698` the active-stage
  `u16` mask.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 29 | 1688 | 25 | 4 |

## What this pass covers

- **Stage-flag bit accessors** (`0x800CE190..0x800CE510`, 16 bodies) - the
  `0x2A × 3` set-bit counter behind the `0x802DCD74`/`0x802DCDC0` pair, the
  single-bit test, the raw word read, the set/clear pair, the "any masked bit
  set" test, and the four `write`/`read` byte-table pairs at `+0x3C`, `+0x74C`,
  `+0x774` and `+0x79C`. The offset shape
  `base + area * 0xA8 + stage * 4 + 0x6C` is identical in all four word
  accessors.
- **Player-stat record lifecycle** (`0x800CE580`, `0x800CE680`, `0x800CE6C0`,
  `0x800CE800`, `0x800CE820`, `0x800CE860`) - the 0x6A0-byte reset with the
  four-byte tag from `r13-0x7A20` and state `0xE`, the `+0x698` mask bit set,
  the "reset if not already `0xE`" wrapper, the vtable install/publish through
  `r13-0x56D0`, the guarded release, and the twenty-store per-player clear.
- **Player counters and popups** (`0x800CE8C0`, `0x800CE940`, `0x800CE950`,
  `0x800CE9D0`, `0x800CEA20`, `0x800CEA70`, `0x800CEB80`) - the camera-biased
  `0xB7` score marker, the per-player id store, the `+0x18` score add clamped
  at `1_000_000`, the `+0x28` and `+0x38` counter increments (all normalising
  the player id through `0x80060170`), the point-tier ladder that spawns the
  `0x15..0x1A` fukidashi balloons, and the active-slot flag rebuild.

Deliberately skipped:

- `0x800CE530`, `0x800CE540` (12 bytes), `0x800CE550`, `0x800CE560`
  (8 bytes), `0x800CE570` (4 bytes), `0x800CE5F0` (8 bytes) - below this
  module's stated 16-byte floor; their fields are already exercised by the
  translated siblings.
- `0x800CE700` (vtable `0x803179A0` install), `0x800CE7A0`/`0x800CE7D0`
  (getters on the `0x802D79A0` object) - a separate class that sits between the
  two sub-clusters; it shares neither the stage-flag object nor the 0x6A0
  record, so it is left for a later pass rather than guessed at.
- `0x800CEB80` is included; `0x800CEC10` and above (the 0x4000-byte constructor
  built from three 0x980 subtables) are outside this run.
- The rest of the `0x800C0000` bucket: the dMaskMng manager
  (`0x800CBF60..0x800CD770`) and the layout library
  (`0x800C8CB0..0x800C9DF0`) remain separate systems.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800CE190` | `fn_800ce190` | 124 | medium |
| `0x800CE210` | `fn_800ce210` | 40 | high |
| `0x800CE330` | `fn_800ce330` | 28 | high |
| `0x800CE350` | `fn_800ce350` | 16 | high |
| `0x800CE3A0` | `fn_800ce3a0` | 24 | high |
| `0x800CE3C0` | `fn_800ce3c0` | 40 | high |
| `0x800CE3F0` | `fn_800ce3f0` | 32 | high |
| `0x800CE410` | `fn_800ce410` | 32 | high |
| `0x800CE430` | `fn_800ce430` | 20 | high |
| `0x800CE450` | `fn_800ce450` | 20 | high |
| `0x800CE470` | `fn_800ce470` | 20 | high |
| `0x800CE490` | `fn_800ce490` | 20 | high |
| `0x800CE4B0` | `fn_800ce4b0` | 20 | high |
| `0x800CE4D0` | `fn_800ce4d0` | 20 | high |
| `0x800CE4F0` | `fn_800ce4f0` | 20 | high |
| `0x800CE510` | `fn_800ce510` | 20 | high |
| `0x800CE580` | `fn_800ce580` | 104 | high |
| `0x800CE680` | `fn_800ce680` | 28 | high |
| `0x800CE6C0` | `fn_800ce6c0` | 64 | medium |
| `0x800CE800` | `fn_800ce800` | 20 | high |
| `0x800CE820` | `fn_800ce820` | 64 | medium |
| `0x800CE860` | `fn_800ce860` | 88 | high |
| `0x800CE8C0` | `fn_800ce8c0` | 120 | medium |
| `0x800CE940` | `fn_800ce940` | 16 | high |
| `0x800CE950` | `fn_800ce950` | 124 | high |
| `0x800CE9D0` | `fn_800ce9d0` | 76 | high |
| `0x800CEA20` | `fn_800cea20` | 76 | high |
| `0x800CEA70` | `fn_800cea70` | 268 | high |
| `0x800CEB80` | `fn_800ceb80` | 144 | high |

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/stage.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error in `src/game/stage.rs`
  (the only remaining errors are in another lane's `src/game/bases.rs`).
- `node tools/check-lane-csv.mjs wave48` reports `OK` for the stage CSV.

## Known limits

- The four `medium` bodies are inferred in one place each:
  `0x800CE190` splits the 64-bit `0x802DCD74` result into flag-object base and
  area index because those are the two arguments it forwards to
  `0x800CE210`; `0x800CE6C0` and `0x800CE820` have their dropped call argument
  restored from sibling call sites; `0x800CE8C0` names the camera and draw
  globals from their read/write sites, while every arithmetic step and call is
  literal.
- `0x800CE8C0` is declared with no parameters: its export models none and the
  body never reads `r3`, although every caller passes the player-stat record.
- The `+0x58` slot flags in `0x800CEB80` are built from the low byte of the
  `0x802F1160`/`64`/`68` words; the translation truncates the big-endian word
  load instead of reading the last byte.
