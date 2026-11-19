# Coverage - Wave 104 (`game::stage`, the `0x800B0000` remainder)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave104-stage.csv` is the Ghidra C export each body was
written from. Field roles the export only hints at stay marked `observed`,
`inferred` or `unknown` in the body.

Three conventions matter for this bucket:

* `0x802DCD40..0x802DCD74` are the Metrowerks `__savegprlr_NN` prologue helpers.
  They preserve every register, so the `u64` Ghidra prints as their return value
  is simply the caller's own `r3:r4` pair. When a body reads
  `(int)((ulonglong)uVar >> 0x20)` the Rust names that value `this` (or the
  draw-list owner) and reads the low half as the second argument, the same
  reading as `fn_80070cd0` and the wave-100 lift helpers.
* The `ldexpf` blocks in every prologue are dead stack writes, and the
  `CONCAT44(0x43300000, x ^ 0x80000000) - 2^52` chains are the signed-int to
  double idiom. They are reproduced as `i32`/`i16` casts, not as `GQR` state.
* The lift objects use two parallel arrays: `0x84`-byte cells with a twelve-word
  matrix at `+0x6C`, and `0x40`/`0x600`-stride draw nodes at `+0x1C` whose
  matrices live at `+0x6020`. The `u16` flag array is at `+0x9028` (stride
  `0x20`) and the `u32` state array at `+0x9228` (stride `0x40`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 14 | 5348 | 1 | 13 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`. The pass covers `0x800B3000..0x800BF3F0`.

## What this pass covers

- **The lift-grid writers** (`0x800BE000`, `0x800BD7A0`, `0x800BE390`,
  `0x800BF3F0`). `0x800BE000` clears the two flag arrays (`0x400` bytes of
  `u32` at `+0x9228`, `0x200` bytes of `u16` at `+0x9028`), loads the variant
  model with `0x800DF180`, and emits the row strip through `0x800BD7A0` /
  `0x800BD970`. `0x800BD7A0` writes one row of four `(x, y, z)` triples with
  the `+0x6020` matrix, binds `0x80239F20`'s animation and registers the draw
  record. `0x800BE390` rebuilds the 16×16 grid with the five-step matrix chain
  (translate, uniform scale, translate, `0x8016EEF0` rotate, translate back).
  `0x800BF3F0` fills the up-to-four travel cells from the `0x80358DF0` word
  triple and the two scaled sizes.
- **The sprite-node helpers** (`0x800BE690`, `0x800BE950`, `0x800BEA00`,
  `0x800BEB50`). `0x800BE690` is the leaf: one model word through
  `0x80239E30`, the node build `0x80169D90`, and the optional clip pair
  (`0x8023A200`/`0x80166830`/`0x80166BD0`/`0x80166EF0`). The other three are
  its callers: the `columns × rows` fill at `+0x4A4`, the 3×3 variant block,
  and the `lift_toge` bit-mask walk over `+0x207C..+0x228C`.
- **The shared-matrix helper** (`0x800BC6B0`). One twelve-word matrix is copied
  three times into the `+0x00`/`+0x40`/`+0x80` sub-objects at yaw `+0xC8`,
  zero and `-yaw`, each multiplied by the source and installed before the
  sub-object's virtual `+0x14` method runs.
- **The screen-space converters** (`0x800B3000`, `0x800B33D0`, `0x800B39C0`).
  `0x800B3000` rotates a two-double vector by an `s16` angle through
  `0x80237BD0`/`0x80237C40`; `0x800B33D0` subtracts the two `fn_800b30c0`/
  `fn_800b30e0` offsets and divides by the `[owner+0x40]`/`[owner+0x44]` scale
  pair, with `0x801DD1D0` picking the y divisor; `0x800B39C0` is the
  `table[index]` sample wrapper around `fn_800b5c50`/`fn_800b38d0`.
- **The slope probe and the strip projection** (`0x800B8CF0`, `0x800BBE10`).
  `0x800B8CF0` runs the `0x80075270` grid test and the `0x800757B0` segment
  test at two x positions and compares the hit height with the `r2-0x6A14`
  tolerance. `0x800BBE10` builds the three six-float projection columns at
  `+0xD4` from the `s16` angle at `+0x11C`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B3000` | `fn_800b3000` | 192 | medium |
| `0x800B33D0` | `fn_800b33d0` | 248 | medium |
| `0x800B39C0` | `fn_800b39c0` | 92 | high |
| `0x800B8CF0` | `fn_800b8cf0` | 496 | medium |
| `0x800BBE10` | `fn_800bbe10` | 232 | medium |
| `0x800BC6B0` | `fn_800bc6b0` | 664 | medium |
| `0x800BD7A0` | `fn_800bd7a0` | 452 | medium |
| `0x800BE000` | `fn_800be000` | 832 | medium |
| `0x800BE390` | `fn_800be390` | 436 | medium |
| `0x800BE690` | `fn_800be690` | 284 | medium |
| `0x800BE950` | `fn_800be950` | 168 | medium |
| `0x800BEA00` | `fn_800bea00` | 336 | medium |
| `0x800BEB50` | `fn_800beb50` | 172 | medium |
| `0x800BF3F0` | `fn_800bf3f0` | 744 | medium |

Address range: `0x800B3000..0x800BF3F0` (14 bodies, 5348 bytes).

## Deliberately skipped

The bucket held 23 open entries. Fourteen are landed above; the other nine are
skipped on purpose.

- **Already-translated targets behind 4-byte branch thunks** (`0x800B4600`,
  `0x800B54B0`). Their exports are `thunk_FUN_800b4610` and
  `thunk_FUN_801dd1d0`; both targets already have ledger rows
  (`game::stage` `0x800B4610` and `runtime::dvd` `0x801DD1D0`), so a row here
  would double-count the body. Same precedent as waves 84 and 98-103.
- **Stack-local template with unwritten locals** (`0x800BC140`). It probes four
  `0x802F0D40` type records and then reads `local_44`/`local_34`/`local_24`
  before any store writes them. Those words are the caller's frame slots, not
  values the export defines, so writing the matrix block would be invention.
- **Paired-single node builders** (`0x800BC2E0`, `0x800BC5D0`). Both open with
  three `CONCAT13`/`CONCAT22` dequantisation blocks that write the *stack* words
  `local_8`/`local_18`/`local_28` from `in_f29..in_f31` and then never read
  them, while the real integer arguments arrive through the packed
  `__savegprlr` return. The bodies are readable, but the argument list cannot be
  stated without guessing which of the three float registers was a real
  parameter, so no row is claimed.
- **Bodies whose argument registers are dropped** (`0x800BD970`, `0x800B96A0`,
  `0x800B9A90`, `0x800BEF00`). `0x800BD970` is the `1680`-byte per-strip double
  loop whose `iVar3`/`iVar4` pair is `r3`/`r4`; it is called by
  [`fn_800be000`] through [`call2_f64_3u32`] instead of being reproduced, because
  its own body mixes the pairing with a `-extraout_f1` that the export never
  defines. `0x800B96A0` and `0x800B9A90` are the two `lift` stagger/follow
  passes: their eight and ten `in_f25..in_f31` words are all dead stack copies,
  and the surviving code reads a mix of `r2` float constants and matrices whose
  row/column roles the export keeps ambiguous. `0x800BEF00` is the 1260-byte
  lift frame with five flag-gated `0x84`-byte cells; its `in_f30`/`in_f31`
  words are dead as well, but three of its blocks read `local_64`/`local_68`
  after the export has already reused those slots for a different value, so the
  write order would be a guess.

None of the nine skipped entries is counted as translated; 14 landed + 9
skipped = the 23 open entries of the `0x800B0000` bucket.

## Verification

- Every address and byte size matches `functions.tsv` (14 rows, 5348 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the fourteen
  were chosen from the 23 open entries of the `0x800B0000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the remaining errors belong to other lane files).
- `node tools/check-lane-csv.mjs wave104` prints `OK`; the wave-104 stage rows
  are clean.
