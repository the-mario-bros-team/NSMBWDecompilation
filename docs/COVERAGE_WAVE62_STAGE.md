# Coverage - Wave 62 (`game::stage`, course-manager text, effect slots and marker records)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column points at the Ghidra C export the body was written from. All three runs
sit in the `0x800Bxxxx` course-manager band that wave 39 opened; the exact
subsystem names for the text and marker records are inferred, while every
offset and call below is literal in the export.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 30 | 3940 | 14 | 16 |

No body in this pass is `low` confidence and none is `partial`.

## What this pass covers

Three untranslated runs inside `0x800B3540..0x800B89B0`:

- **Message text and status word** (`0x800B3540..0x800B3B60`) - the colour-row
  selector that draws through `0x8010CFA0`/`0x8010CEF0`, its fixed-colour
  wrapper, the actor start/restart scan over the four `0x23C`-stride scene
  records, three guarded forwards to `0x801571B0`/`0x80157310`/`0x80157360`,
  the bit-0..3 status-word recomputation and its two-flag predicate, and the
  `0x10`-byte transform builder.
- **Effect-slot publication** (`0x800B3E50..0x800B4650`) - five copies of the
  same "register N descriptor addresses, walk `0x80168400`/`0x8023B8C0`,
  filter with the `0x802F8FF0` type test, submit through `0x8023D120`" pass,
  differing only in descriptor addresses and the command kind (`1`..`5`); the
  two text-object forwards at `0x800B44D0`; the cancel/arm pair for event
  `0x2B9`; the mode-`3`/`4` classifier; and the 100-byte name builder.
- **Marker/actor records** (`0x800B80E0..0x800B89B0`) - the constructor that
  copies one `0x20`-byte template row into eight three-word groups, the
  release loop over the eight-slot flag word, the per-slot activate and render
  submission, the record constructor/destructor with the live-count word at
  `r13-0x5718`, and the three variant-table helpers.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B3540` | `fn_800b3540` | 100 | high |
| `0x800B35B0` | `fn_800b35b0` | 20 | high |
| `0x800B3600` | `fn_800b3600` | 268 | medium |
| `0x800B3720` | `fn_800b3720` | 40 | high |
| `0x800B3780` | `fn_800b3780` | 40 | high |
| `0x800B37B0` | `fn_800b37b0` | 32 | high |
| `0x800B3A30` | `fn_800b3a30` | 240 | medium |
| `0x800B3B20` | `fn_800b3b20` | 36 | high |
| `0x800B3B60` | `fn_800b3b60` | 124 | medium |
| `0x800B3E50` | `fn_800b3e50` | 252 | medium |
| `0x800B3F50` | `fn_800b3f50` | 252 | medium |
| `0x800B4050` | `fn_800b4050` | 284 | medium |
| `0x800B4170` | `fn_800b4170` | 316 | medium |
| `0x800B42B0` | `fn_800b42b0` | 284 | medium |
| `0x800B44D0` | `fn_800b44d0` | 100 | high |
| `0x800B4540` | `fn_800b4540` | 96 | high |
| `0x800B45A0` | `fn_800b45a0` | 80 | high |
| `0x800B4610` | `fn_800b4610` | 56 | high |
| `0x800B4650` | `fn_800b4650` | 180 | medium |
| `0x800B80E0` | `fn_800b80e0` | 44 | medium |
| `0x800B81C0` | `fn_800b81c0` | 132 | high |
| `0x800B8110` | `fn_800b8110` | 156 | medium |
| `0x800B8250` | `fn_800b8250` | 124 | medium |
| `0x800B82D0` | `fn_800b82d0` | 104 | medium |
| `0x800B8440` | `fn_800b8440` | 148 | high |
| `0x800B84E0` | `fn_800b84e0` | 108 | medium |
| `0x800B86F0` | `fn_800b86f0` | 16 | high |
| `0x800B88D0` | `fn_800b88d0` | 92 | high |
| `0x800B8930` | `fn_800b8930` | 124 | medium |
| `0x800B89B0` | `fn_800b89b0` | 100 | medium |

## Deliberately skipped

- `0x800B37E0` (228 bytes) and `0x800B38D0` (164 bytes) - both exports are
  dominated by the `ldexpf`/`unaff_GQR0` paired-single pattern and
  `CONCAT44(0x43300000, …)` conversions, so the float arguments and the
  rounding mode cannot be recovered from the export alone.
- `0x800B39C0` (92 bytes) - the body reads `extraout_var` from `0x800B5C50`,
  which returns `void` in its own export; the real return register is lost, so
  a faithful call sequence cannot be written.
- `0x800B3000` (192 bytes) - same paired-single/`ldexpf` problem as the two
  skipped functions above.
- `0x800B3A20` (12 bytes), `0x800B3B50` (12 bytes) and the other sub-16-byte
  accessors in the bucket (`0x800B04A0`, `0x800B04B0`, `0x800B04C0`,
  `0x800B19B0`, `0x800B2EB0`, `0x800B2EC0`, `0x800B2ED0`, `0x800B2EE0`,
  `0x800B2F00`, `0x800B5C50`, `0x800B6450`, `0x800B81B0`, `0x800BAE50`) - below
  this module's 16-byte floor; they are still callable by address and
  `0x800B2EE0` is used that way by [`fn_800b82d0`].
- `0x800B50F0..0x800B56E0`, `0x800B5C60..0x800B6EF0`, `0x800B8700..0x800B8CF0`
  and the rest of the bucket - outside the three contiguous runs chosen here
  and still needing their own field maps.
- The already translated wave-39, wave-45 and wave-50 entries in this bucket
  were not duplicated.

## Verification

- Every address and byte size matches `functions.tsv` (30 rows, 3940 bytes).
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/stage.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors for
  `src/game/stage.rs` (only pre-existing warnings in other modules).
- `node tools/check-lane-csv.mjs wave62` reports `OK`.
