# Coverage - Wave 74 (enemies: `0x80AE0000` bucket)

## How to read this file

This page records the Wave 74 `game::enemies` lane: the `0x80AE0000` bucket of
REL module 3, `d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row below
corresponds to one real `pub unsafe fn fn_<addr>` body in that file, and the
`evidence` column names the Ghidra C export the body was reconstructed from.
The machine-readable mirror of the table is
`ledger/agents/wave74-enemies.csv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Float and
integer constants quoted in the docs were read from the REL image
(`work/rels/elf/d_enemiesNP.elf`) at the cited synthetic address, not guessed
from the C text.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 3,860 | 21 | 8 |

The bucket holds 160 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 17 were already in
`ledger/functions.csv`, leaving 143. This pass claims 29 and leaves 114.

## What this pass covers

The claimed bodies form one contiguous run, `0x80AE5800..0x80AE90C0`, inside a
single 0xEC8-byte actor class. They were chosen because they share fields
(`+0x348` side, `+0xE54` pivot, `+0xE60` timer, `+0xE68`/`+0xE6C` state and
mode, `+0xE74`/`+0xE78` spin, `+0xE8E` trail cursor, `+0xE9A`/`+0xEA2` latches)
and because their exports are clean (no GQR/`unaff` pseudo-code and no
`halt_baddata`).

- **Lifecycle (`0x80AE5800..0x80AE5980`).** The deleting destructor, its two
  48-byte forwarders and the allocating constructor. The constructor is the
  anchor: it allocates `0xEC8` bytes, installs the class vtable `0x80B609E0`,
  and initialises the model holder `+0x524`, the matrix `+0x544`, the animation
  holder `+0x584` (vtable `0x80B330E0`) and the sub-object `+0x5CC`.
- **Model binders (`0x80AE7320`, `0x80AE7470`).** The two literal asset
  binders: `g3d/pukupuku.brres` under `pukupuku` with the `swim` clip, and
  `g3d/pukupuku_toge.brres` under `pukupuku_toge`, which additionally joins the
  `+0x5C0` holder to the model through vtable slot `+0x18`.
- **Speed, latch and motion helpers (`0x80AE65C0..0x80AE7BF0`).** The ceiling
  clamp, the argument-pair compare, the eleven-word attack-record seed, the
  squash-pivot reset, the water-region query, the four mode/side speed tables,
  the two one-shot clip latches, the 256-entry trail history, the empty virtual
  hook, the spin arming/step pair, the yaw blend, the side-aware overlap band,
  the normalise-and-aim helper, the contact proximity test, and the six-phase
  squash ramp with its decay.
- **State helpers (`0x80AE7E90..0x80AE90C0`).** The player damage-mask test and
  four state entries/steps: restore-the-saved-side, reset-the-speeds,
  restart-the-clip and the water-move frame.

### Identity evidence

- `0x80AE5980` stores `0x80B609E0` at `+0x60` and initialises exactly the
  holders that `0x80AE7320`/`0x80AE7470` bind into (`+0x524`, `+0x540`,
  `+0x544`, `+0x584`, `+0x5BC`, `+0x5C0`), so constructor and binders belong to
  one class.
- `0x80AE7470` contains the literal strings `g3d/pukupuku_toge.brres` and
  `pukupuku_toge`; `0x80AE7320` contains `g3d/pukupuku.brres` and `pukupuku`;
  both bind the clip `swim`.
- The state transitions in `0x80AE90C0`/`0x80AE85A0` target the records
  `0x80B77210`, `0x80B77290`, `0x80B77350`, `0x80B77390`, `0x80B773D0`,
  `0x80B77410` and `0x80B77450`. The already-ledgered `register_puku_states`
  (`0x80AE9990`) names those same records
  `daEnPuku_c::StateID_SrcSwim`, `HellSrcRetur`, `WaterMove`, `LrSwim`,
  `LrTurnSwim`, `LrSrcSwim` and `EscapeSwim`.
- The class name itself is not printed anywhere in this bucket, so the module
  header and the bodies describe it as "the 0xEC8-byte `pukupuku` actor"
  instead of asserting a C++ class name.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AE5800` | `fn_80ae5800` | 88 | high |
| `0x80AE5920` | `fn_80ae5920` | 48 | high |
| `0x80AE5950` | `fn_80ae5950` | 48 | high |
| `0x80AE5980` | `fn_80ae5980` | 188 | high |
| `0x80AE65C0` | `fn_80ae65c0` | 64 | medium |
| `0x80AE6600` | `fn_80ae6600` | 28 | high |
| `0x80AE6880` | `fn_80ae6880` | 172 | high |
| `0x80AE6F00` | `fn_80ae6f00` | 16 | high |
| `0x80AE7160` | `fn_80ae7160` | 60 | high |
| `0x80AE71A0` | `fn_80ae71a0` | 148 | high |
| `0x80AE7240` | `fn_80ae7240` | 100 | medium |
| `0x80AE72B0` | `fn_80ae72b0` | 100 | medium |
| `0x80AE7320` | `fn_80ae7320` | 336 | high |
| `0x80AE7470` | `fn_80ae7470` | 428 | high |
| `0x80AE7620` | `fn_80ae7620` | 144 | high |
| `0x80AE76B0` | `fn_80ae76b0` | 4 | high |
| `0x80AE76C0` | `fn_80ae76c0` | 96 | medium |
| `0x80AE7720` | `fn_80ae7720` | 228 | medium |
| `0x80AE7810` | `fn_80ae7810` | 140 | high |
| `0x80AE78A0` | `fn_80ae78a0` | 316 | medium |
| `0x80AE79E0` | `fn_80ae79e0` | 104 | high |
| `0x80AE7A50` | `fn_80ae7a50` | 172 | high |
| `0x80AE7B00` | `fn_80ae7b00` | 228 | high |
| `0x80AE7BF0` | `fn_80ae7bf0` | 172 | high |
| `0x80AE7E90` | `fn_80ae7e90` | 64 | medium |
| `0x80AE85A0` | `fn_80ae85a0` | 92 | high |
| `0x80AE8CA0` | `fn_80ae8ca0` | 56 | high |
| `0x80AE8EF0` | `fn_80ae8ef0` | 48 | high |
| `0x80AE90C0` | `fn_80ae90c0` | 172 | medium |

## Deliberate skips

- **`0x80AE63D0` (388 bytes).** The matrix/rotation commit builds its output
  triple from stack slots Ghidra prints as separate uninitialised locals
  (`local_2c`, `local_1c`, `local_c`) that do not line up with the 12-byte
  buffer passed to `0x80168180`. The aliasing cannot be proven, so the body was
  not written.
- **`0x80AE6560` (88 bytes).** The export is a `halt_baddata` stub after the
  `+0xE60` countdown. `0x80AE90C0` still calls it by address with a `// unknown:`
  note, as the original does.
- **`0x80AE69E0` (444) and `0x80AE6D70` (332).** The two partner-record
  writers. Their exports are dominated by `CONCAT12`/`CONCAT13` byte surgery on
  an untyped partner object and by virtual calls whose receivers Ghidra prints
  as bare `code **` casts, so the record fields cannot be named honestly.
- **`0x80AE6F10` (436 bytes).** The drop-record builder. Its stack layout
  (a struct starting at `&local_44`, with the fields Ghidra shows at *lower*
  offsets) is internally inconsistent in the export, so the field map would be
  a guess.
- **`0x80AE7CA0` (352) and `0x80AE7E00` (132).** Both carry `unaff_GQR0`
  quantiser fiction and `ldexpf` calls that are not in the original code.
  `0x80AE8F40` calls them by address only.
- **`0x80AE8E40` (172).** `unaff_GQR3` on the yaw-to-float conversion; the
  angle arithmetic is not recoverable from the export.
- **`0x80AE7F10` (524), `0x80AE8140` (716), `0x80AE8440` (352),
  `0x80AE8610` (324), `0x80AE8780` (620), `0x80AE8A10` (280),
  `0x80AE8B80` (284), `0x80AE8CF0` (324) and `0x80AE8F40` (280).** These are
  the remaining per-frame state steps of the same class. They are clean, but
  each needs its own float-table and state-record pass, and the lane budget
  went to the constructor/binder/helper spine. `0x80AE8780` (620) is the first
  candidate for the next pass.
- **The earlier and later parts of the bucket.** `0x80AE0560..0x80AE4A50`
  (the 20-function block at the head of the bucket plus the untranslated
  `0x80AE33C0..0x80AE4A50` remainder) and `0x80AEAB90..0x80AEFCF0` (about 40
  functions at the tail) are untouched. They belong to other classes; the
  largest clean tail candidates are `0x80AEDD40` (1,208 bytes) and
  `0x80AEF130` (1,180).

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  3,860 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 17 existing rows are
  `0x80AE3080`, `0x80AE41C0`..`0x80AE5110` and `0x80AE9990`/`0x80AEA910`/
  `0x80AECD40`).
- `node tools/check-lane-csv.mjs wave74` prints
  `wave74-enemies.csv: 29 rows`, `unique addresses: 113` and `OK`.
- `cargo check --message-format=short` finishes with no diagnostic naming
  `src/game/enemies.rs`; the remaining warnings are in `src/runtime/*`.
