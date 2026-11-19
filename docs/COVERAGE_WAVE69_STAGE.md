# Coverage - Wave 69 (`game::stage`, the `0x8009xxxx` actor band)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column points at the Ghidra C export the body was written from.

This pass opens the `0x8009xxxx` band that the rest of the tree had only been
calling by address. The object layout is read from the constructor at
`0x80094E80`: the vtable pointer lives at `+0x60`, the position triple at
`+0xAC..+0xB4`, the height-field segment at `+0x1EC`, the collision/contact
accumulators at `+0x440..+0x454`, the state bytes at `+0x390`/`+0x391`/
`+0x432` and the countdown timers at `+0x500..+0x50C`. Every offset below is
literal in the export; the subsystem name `Wm` is inferred from the
`Wm_en_*`/`Wm_mr_*`/`Wm_ob_*` effect strings registered in the same run.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 30 | 4156 | 19 | 11 |

No body in this pass is `low` confidence and none is `partial`.

## What this pass covers

Four connected clusters, all inside `0x800945D0..0x8009AB80`:

- **Effect-name registration tail** (`0x800945D0`) - the last of the six
  `Wm_*` descriptor wrappers. The other six (`0x80094370`, `0x80094390`,
  `0x800943B0`, `0x800943D0`, `0x800943F0`, `0x80094560`) are already in
  `ledger/functions.csv` as the semantic `stage_effect_*_remove` family and
  were deliberately not duplicated.
- **`Wm` actor base** (`0x80094E80..0x80095240`) - the constructor (two
  embedded interface sub-objects at `+0x394`/`+0x3D4`, two `0x8015FC90`
  builds, the marker record at `+0x484`, the 4-byte holder at `+0x51C`), the
  matching destructor, the position-cache/ground-query refresh, and the frame
  state machine with its five virtual slots and seven countdown timers.
- **Ground, side and contact helpers** (`0x80095480..0x80096460`) - the
  `+0x34A` mode clear, the `0x80064540` status wrapper, the three-word
  `Wm_en_hit` submission, the two identical `+0x220` event handlers, the
  `0x80064790` height probe, the scene-limit compare, the `+0x38D` kind-byte
  copy and the bare deleting destructor.
- **Effect/notice helpers, position cache and object chain**
  (`0x800967A0..0x8009AB80`) - the `+0x3FC` transition call with its event
  latch, the notice spawner pair behind `+0x510`, the two `0x80197F00` event
  submissions (`0x15F`/`0x161`), the ground/side flag fold, the
  `+0x440..+0x454` contact refresh, the two-direction `0x80072CB0` probe, the
  `+0x10D8` event filter, the position cache, the effect `0x266` submission,
  the two-state virtual dispatch, the chain/deleting destructor pair and the
  two path-tail comparators.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800945D0` | `fn_800945d0` | 36 | high |
| `0x80094E80` | `fn_80094e80` | 556 | medium |
| `0x80095130` | `fn_80095130` | 160 | high |
| `0x800951D0` | `fn_800951d0` | 104 | medium |
| `0x80095240` | `fn_80095240` | 572 | medium |
| `0x80095480` | `fn_80095480` | 32 | medium |
| `0x800954A0` | `fn_800954a0` | 44 | medium |
| `0x800954D0` | `fn_800954d0` | 80 | high |
| `0x80095C20` | `fn_80095c20` | 92 | high |
| `0x80095C80` | `fn_80095c80` | 92 | high |
| `0x80096240` | `fn_80096240` | 92 | high |
| `0x80096300` | `fn_80096300` | 40 | medium |
| `0x80096330` | `fn_80096330` | 116 | high |
| `0x80096460` | `fn_80096460` | 64 | high |
| `0x800967A0` | `fn_800967a0` | 76 | medium |
| `0x800967F0` | `fn_800967f0` | 236 | medium |
| `0x80096CA0` | `fn_80096ca0` | 84 | high |
| `0x80096D00` | `fn_80096d00` | 84 | high |
| `0x80096E60` | `fn_80096e60` | 248 | medium |
| `0x80096F60` | `fn_80096f60` | 236 | high |
| `0x80097050` | `fn_80097050` | 276 | medium |
| `0x800971F0` | `fn_800971f0` | 92 | high |
| `0x80097DF0` | `fn_80097df0` | 52 | high |
| `0x80097E30` | `fn_80097e30` | 24 | high |
| `0x80097E50` | `fn_80097e50` | 76 | high |
| `0x80097F40` | `fn_80097f40` | 32 | high |
| `0x800980B0` | `fn_800980b0` | 108 | medium |
| `0x8009A960` | `fn_8009a960` | 92 | high |
| `0x8009AAA0` | `fn_8009aaa0` | 224 | high |
| `0x8009AB80` | `fn_8009ab80` | 136 | high |

## Deliberately skipped

- The six already-translated effect-name wrappers at
  `0x80094370`/`0x80094390`/`0x800943B0`/`0x800943D0`/`0x800943F0`/
  `0x80094560`, and the earlier `0x80094DE0` (`stage_effect_entry_destroy`).
  All seven are in `ledger/functions.csv` with bodies already present in this
  file, so no row and no second body were created.
- `0x800955F0` (444 bytes) and `0x800957B0` (212 bytes) - both exports are
  dominated by `ldexpf`/`unaff_GQR0` paired-single blocks whose rounding and
  float argument values cannot be recovered from the export alone.
- `0x800963B0` (176 bytes) - the three computed floats are passed to
  `0x800709C0`, but the export prints only that call's three integer
  arguments; which of the three floats goes in `f1..f3` cannot be proven.
- `0x80094660` (648 bytes) - same paired-single/`ldexpf` problem as
  `0x800955F0`, plus six `0x43300000` conversions of vector cell values.
- `0x800964A0` (604 bytes), `0x80095240`-adjacent `0x80095xxx` gaps and the
  remaining `0x8009Bxxx`/`0x8009Cxxx` code - outside the four runs chosen here
  and still needing their own field maps.
- The sub-16-byte accessors in the band (`0x80096700`, `0x80096780`,
  `0x80096790`, `0x80098350..0x800983B0`, `0x80099F80..0x8009A000`,
  `0x8009AC70..0x8009AD20`, `0x8009EAF0`, `0x8009EB00`, `0x8009F7C0..0x8009F830`)
  - below this module's 16-byte floor. `0x80096780`/`0x80096790` are also
  pure indirect jumps into the vtable, so their exports carry no recoverable
  argument list.

## Verification

- Every address and byte size matches `functions.tsv` (30 rows, 4156 bytes).
- Every evidence file exists at the cited path under `work/ghidra/main-gb/`.
- Every row has exactly one body in `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave69` reports `OK`.
