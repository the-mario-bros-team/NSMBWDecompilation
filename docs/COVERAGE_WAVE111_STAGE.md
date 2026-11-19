# Coverage - Wave 111 (`game::stage`, closing the `0x800B0000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave111-stage.csv` is the Ghidra C export each body was
written from.

The bucket opened with 235 entries and wave 111 starts from its last six open
rows. Two are landed here, leaving four that cannot be reconstructed honestly;
229 of the 235 already had `game::stage` rows, and this pass brings that to 231.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 2 | 2940 | 0 | 2 |

Both rows are `translated` and `medium`; neither is `partial` or `low`. The pass
covers `0x800BD970..0x800BEF00`.

## What this pass covers

- **The per-strip emitter** (`0x800BD970`, 1680 bytes). This is the body the
  `0x800BE000` strip loop calls once per row. It reads the `+0x9628` row count
  and the `+0x9630` flags, builds the first cell row from the two `f64`
  arguments, optionally writes a seventh top row at `+0x6020` (flag bit 1, mode
  other than 2), then emits one row per remaining column. Four x words advance
  by `r2-0x6908`; the final row adds `r2-0x6900` (or the pitch when the count is
  one), and flag bit 0 adds a closing row carrying `0xC000` in the `+0x9028`
  halfword. Every row is twelve floats at `+0x6020 + layer*0x300 + index*0x30`,
  a node at `+0x1C + layer*0x600 + index*0x60`, and one enable word at
  `+0x9228 + layer*0x40 + index*4`. The two per-mode tables are `0x802F0DE8`
  (two floats per mode) and `0x80316018` (four animation names per mode).
- **The 3x3 cell block** (`0x800BEF00`, 1260 bytes). The body writes nine
  six-word cells at `+0x6C + k*0x84`, the layout [`fn_800bec00`] fills. Row 0
  keeps the `r2-0x68D4` height and row 1 subtracts `pitch*(columns-1)`, row 2
  subtracts it twice; the x sizes run through the `r2-0x68D8` pitch in the same
  way. Cells 0, 2, 3, 4 and 6 also receive a [`fn_800be7b0`] translation; the
  two `0x802F0E4C` bytes of the variant swap its first two components. The
  `+0x228C` byte then gates a [`fn_800bec00`] rebuild of the full grid.

Both bodies follow the five-argument
`(f64 x, f64 y, this, index, mode)` convention that wave 104 already used for
the sibling [`fn_800bd7a0`], and both were cross-checked against the cell layout
in [`fn_800bec00`] and [`fn_800bf3f0`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800BD970` | `fn_800bd970` | 1680 | medium |
| `0x800BEF00` | `fn_800bef00` | 1260 | medium |

Address range: `0x800BD970..0x800BEF00` (2 bodies, 2940 bytes).

## Deliberately skipped

Four of the six open rows are still not landed. None of them is a missing
translation that a later pass can recover from this export alone.

- **Branch thunks** (`0x800B4600`, `0x800B54B0`, 4 bytes each). Their exports
  are the *target* bodies, `thunk_FUN_800b4610` and `thunk_FUN_801dd1d0`; the
  targets already own ledger rows (`game::stage` `0x800B4610` and
  `runtime::dvd` `0x801DD1D0`). Writing a second body would double-count the
  same code. Same precedent as waves 84 and 98-110.
- **The GQR-quantised stagger pass** (`0x800B96A0`, 1008 bytes). Both trig
  arguments pass through the `GQR3` paired-single load. The non-quantised arm of
  that conversion reads `auStack_1b4`, which nothing in the body writes, and the
  second value collapses to the constant `0.0004891455173492432`; the two values
  then feed `0x80237BD0` and `0x80237C40`. The angle is therefore not
  recoverable from the export. The live node stores (`+0x68..+0x7C`, `+0x88`)
  are already described by [`fn_800b9a90`].
- **The stack-local template** (`0x800BC140`, 344 bytes). It reads
  `local_44`/`local_34`/`local_24` as a source position before any store writes
  them. The only later stack writer is the `0x801C0C60` matrix build, whose
  output is `auStack_80`, not those three slots. Writing the translation would
  require inventing the source point.
Waves 104 and 110 skipped the same two templates for the same reasons; wave 111
only found new evidence for the two grid passes.

Two landed + four skipped = the six open entries of the `0x800B0000` bucket.

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 2940 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the two were
  chosen from the six open entries of the `0x800B0000` bucket.
- `cargo check --message-format=short` exits 0 and reports no error and no
  warning for `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave111` prints `OK` for this CSV.
- The module header of `src/game/stage.rs` gained a wave-111 paragraph naming
  both bodies and the four skips, and the wave-110 paragraph no longer claims
  the two grid passes are open.
