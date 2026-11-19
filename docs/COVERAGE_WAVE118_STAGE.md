# Coverage - Wave 118 (`game::stage`, closing the `0x800B0000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave118-stage.csv` is the Ghidra C export each body was
written from.

The bucket held 235 entries and 233 already had rows, so wave 118 starts from
the two entries wave 114 deliberately skipped: the 4-byte branch thunks
`0x800B4600` and `0x800B54B0`. Both are landed here as one-call wrappers, which
closes the bucket at 235/235.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 2 | 8 | 2 | 0 | 0 |

Both rows are `translated`; none is `partial` or `stub`. The pass covers
`0x800B4600` and `0x800B54B0` (8 bytes total).

## What this pass covers

- **The mode-classifier thunk** (`0x800B4600`, 4 bytes), Ghidra name
  `thunk_FUN_800b4610`. The word at `0x800B4600` in `work/dol/main.elf` is
  `0x48000010`, i.e. `b 0x800B4610` (opcode 18, `LI = +0x10`). The body is
  [`fn_800b4600`], a direct call to the already-translated
  [`fn_800b4610`] (`game::stage`, `high`), which collapses the low byte of
  `0x801DD320` to `1` unless it is `3` or `4`. A tail branch returns the
  target's `r3` straight to the thunk's caller, so the wrapper has the same
  observable behaviour and the target is not re-translated.
- **The settings item-1 thunk** (`0x800B54B0`, 4 bytes), Ghidra name
  `thunk_FUN_801dd1d0`. The word at `0x800B54B0` is `0x48127D20`, i.e.
  `b 0x801DD1D0` (opcode 18, `LI = +0x127D20`). The body is
  [`fn_800b54b0`], which uses this file's existing `call0_ret(0x801D_D1D0)`
  helper and returns the low byte. The target (`runtime::dvd::fn_801dd1d0`,
  `high`) reads DVD settings item 1 through `0x801DC8B0(buffer, 1)` and yields
  `1` only when that byte is `1`.

The instruction words above were read from `work/dol/main.elf` (PT_LOAD
mapping, big-endian), not inferred from prose: this is the first pass that
landed a 4-byte thunk in this bucket, and the byte size in `functions.tsv`
alone cannot distinguish `b target` from any other single instruction. The
Ghidra export for each entry prints the *target* body (that is how Ghidra
renders a thunk), which is why previous waves treated these two as skips. The
ELF words confirm the branch, so the wrappers here are evidence-backed rather
than assumed.

The wrappers are written out, instead of being left as skips again, because
each is the last open entry of the bucket and a 4-byte direct branch is fully
reconstructible; the precedent for a landed tail-branch thunk in this tree is
`fn_8019df40` in `src/audio/ax.rs`. Each row owns a distinct address and a
distinct 4-byte range, so nothing is double-counted: the target bodies remain
the ones cited by their existing rows.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B4600` | `fn_800b4600` | 4 | high |
| `0x800B54B0` | `fn_800b54b0` | 4 | high |

Address range: `0x800B4600` and `0x800B54B0` (2 bodies, 8 bytes).

## Deliberately skipped

Nothing. The two open entries of the `0x800B0000` bucket are both landed; the
bucket is now 235/235.

## Verification

- Every address and byte size matches `functions.tsv` (2 rows: `0x800B4600` = 4,
  `0x800B54B0` = 4; 8 bytes total).
- The branch words `0x48000010` and `0x48127D20` were read from
  `work/dol/main.elf`; they resolve to `0x800B4610` and `0x801DD1D0`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave118` reports this CSV clean.
- The module header of `src/game/stage.rs` gained a wave-118 paragraph naming
  both bodies (8 bytes), a wave-118 line in the original-range list, the note
  that wave 118 closed the two wave-114 skips, and an exception to the
  "functions smaller than 16 bytes" rule for these two thunks.
