# Coverage - Wave 71 (`game::stage`, the `0x800C0000` `dLineMng` seeders, `dMaskMng` manager and per-player record accessors)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column of `ledger/agents/wave71-stage.csv` points at the Ghidra C export the
body was written from.

This pass closes the remaining gaps inside `0x800C0000..0x800CFFFF`. All of
them are read from the stores themselves:

* **`dLineMng` state seeders** (`0x800C6E20..0x800C71A0`). Each one builds a
  two-float delta from `r2` small data and calls `0x800C3A10(angle, obj, delta)`,
  which adds the delta to the line object's `+0x50`/`+0x54` anchor, copies the
  sum to `+0x58`/`+0x5C`, and then offsets `+0x40`/`+0x44` by the heading. The
  deltas are `(8,-8)`, `(16,-16)`, `(0,-16)`, `(16,0)`, `(0,0)`, `(0,-32)`,
  `(32,-32)`, `(32,0)` and `(0,0)`; the heading is either a literal `16.0f` or
  `32.0f`, or the caller's live-in `f1`, which the export does not print.
* **`dMaskMng` construction** (`0x800CD520..0x800CD580`). The manager owns a
  `0x4000`-byte cell heap at `+0x04` (created with the
  `dMaskMng::m_allocator` name at `0x803178C8`) and two auxiliary objects at
  `+0x24`/`+0x28` (sizes `0x14` and `0x18`, ids `0x8A` and `0x87`).
* **`dMaskMng` records** (`0x800CD770..0x800CDBF0`). A `0x1C`-byte record whose
  first `0x14` bytes are a dDyl-style slot; `+0x14` is the cell allocator base,
  `+0x18` the `0x20`-byte cell. The constructors/destructors pair with the
  already-translated `0x8008F140`/`0x8008F170`.
* **Per-player record accessors** (`0x800CE060..0x800CE0F0`). Five byte arrays
  indexed by player: `+0x1E` (signed), `+0x22`, `+0x26` (bit 0 only), `+0x2A`
  and `+0x2E`. The only caller in the exports, `0x800E1840`, walks player
  `0..3` and copies `+0x22`/`+0x26`/`+0x2A`/`+0x2E` into the
  `DAT_80354E70..DAT_80354EA0` tables, which is what establishes the index
  range; the meaning of each array is unknown.
* **Lift-actor helpers** (`0x800C0540`, `0x800C12B0`, `0x800C1510`), the
  damping integrator, the two-word position copy and the sign-flipping speed
  store.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 29 | 1372 | 18 | 11 |

No body in this pass is `low` confidence and none is `partial`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C0540` | `fn_800c0540` | 44 | high |
| `0x800C12B0` | `fn_800c12b0` | 20 | high |
| `0x800C1510` | `fn_800c1510` | 24 | high |
| `0x800C6E20` | `fn_800c6e20` | 52 | medium |
| `0x800C6F00` | `fn_800c6f00` | 52 | medium |
| `0x800C6F60` | `fn_800c6f60` | 56 | high |
| `0x800C6FC0` | `fn_800c6fc0` | 52 | medium |
| `0x800C7020` | `fn_800c7020` | 52 | high |
| `0x800C7080` | `fn_800c7080` | 56 | high |
| `0x800C70E0` | `fn_800c70e0` | 52 | medium |
| `0x800C7140` | `fn_800c7140` | 52 | medium |
| `0x800C71A0` | `fn_800c71a0` | 52 | high |
| `0x800CD520` | `fn_800cd520` | 88 | medium |
| `0x800CD580` | `fn_800cd580` | 124 | medium |
| `0x800CD770` | `fn_800cd770` | 88 | medium |
| `0x800CDA70` | `fn_800cda70` | 60 | medium |
| `0x800CDAB0` | `fn_800cdab0` | 124 | medium |
| `0x800CDBF0` | `fn_800cdbf0` | 76 | medium |
| `0x800CDDF0` | `fn_800cddf0` | 116 | high |
| `0x800CE060` | `fn_800ce060` | 12 | high |
| `0x800CE070` | `fn_800ce070` | 12 | high |
| `0x800CE080` | `fn_800ce080` | 12 | high |
| `0x800CE090` | `fn_800ce090` | 12 | high |
| `0x800CE0A0` | `fn_800ce0a0` | 12 | high |
| `0x800CE0B0` | `fn_800ce0b0` | 12 | high |
| `0x800CE0C0` | `fn_800ce0c0` | 16 | high |
| `0x800CE0D0` | `fn_800ce0d0` | 16 | high |
| `0x800CE0E0` | `fn_800ce0e0` | 12 | high |
| `0x800CE0F0` | `fn_800ce0f0` | 16 | high |

## Deliberately skipped

- `0x800C6E70` (140 bytes) - the try-move/rollback sibling of the seeders. It
  calls `0x800C4F50` with only two floats printed, but that helper takes
  `(object, point, flag)` in every translated caller, and the export prints
  neither the object nor the point. Writing it would require inventing the two
  integer arguments.
- `0x800C3A10` (188 bytes) - the seed helper itself. Its two coordinate results
  are produced by `GQR3`-guarded `ldexpf`/paired-single blocks over the `+0x64`
  halfword, so the quantisation of the angle cannot be stated honestly; the
  seeders call it by address instead.
- `0x800C75B0` (4772 bytes) - the `dLineMng` state-table initialiser. The body
  is 24 near-identical blocks that copy nine `r2`-relative words into each
  `0x40`-byte state record; the blocks are mechanical but each reads a
  different `0x80316D28..0x803170A8` pair, and a table-driven loop would
  reorder the observable `0x8015F7C0`/`0x802DC780` call pairs. Left for a pass
  that can transcribe all 24 blocks.
- `0x800C9680`, `0x800C9A60`, `0x800C9B50` - the GX submission helpers around
  the `engine::frame`/`engine::display` functions. Their argument lists are
  register-only (`in_f30`/`in_f31`, `ps30`/`ps31`) or lost entirely
  (`FUN_800c9b50()`), so no honest body can be written.
- `0x800C1070` and `0x800C10B0` (64 bytes each) - byte-for-byte duplicates of
  the already-translated `ddyl_slot_delete` shape. They are left out to avoid
  adding two rows that only repeat a known body at new addresses.

## Verification

- Every address and byte size matches `functions.tsv` (29 rows, 1372 bytes).
- Every evidence file exists at the cited path under `work/ghidra/main-gb/`.
- Every row has exactly one body in `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave71` is run after the CSV is written; the
  wave-71 rows in `wave71-stage.csv` are clean.
