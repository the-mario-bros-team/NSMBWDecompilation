# Coverage - Wave 120 (`game::stage`, closing the `0x80070000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave120-stage.csv` is the Ghidra C export the body was
written from.

The `0x80070000` bucket held 192 entries and 191 already had `game::stage` rows,
so this pass starts from the single open entry: the 4-byte branch thunk
`0x800776F0`. It is landed here as a one-call tail-branch wrapper, which closes
the bucket at 192/192.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 1 | 4 | 1 | 0 | 0 |

The single row is `translated`; none is `partial` or `stub`. The pass covers
`0x800776F0` (4 bytes).

## What this pass covers

- **The cell-store thunk** (`0x800776F0`, 4 bytes), Ghidra name
  `thunk_FUN_80077650`. The word at `0x800776F0` in `work/dol/main.elf` is
  `0x4BFFFF60`, i.e. `b 0x80077650` (opcode 18, `LI = -0xA0`). The body is
  [`fn_800776f0`], a direct call to the already-translated [`fn_80077650`]
  (`game::stage`, `medium`), which normalises a raw cell index through
  `0x80101AA0`, truncates it to 16 bits, resolves the cell through `0x80077520`
  and stores the cell's halfword through `0x800774A0`. A tail branch returns the
  target's `r3` straight to the thunk's caller, so the wrapper has the same
  observable behaviour and the target is not re-translated.

The instruction word above was read from `work/dol/main.elf` (PT_LOAD mapping,
big-endian), not inferred from prose. The Ghidra export for this entry prints the
*target* body (that is how Ghidra renders a thunk), which is why wave 115 treated
it as a double-count and skipped it. The ELF word confirms the branch, so the
wrapper here is evidence-backed rather than assumed, and it follows the
precedent of waves 118 and 119 in this same file.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800776F0` | `fn_800776f0` | 4 | high |

Address range: `0x800776F0` (1 body, 4 bytes).

## Deliberately skipped

Nothing. The single open entry of the `0x80070000` bucket is landed; the bucket
is now 192/192.

## Verification

- Every address and byte size matches `functions.tsv` (1 row: `0x800776F0` = 4;
  4 bytes total).
- The branch word `0x4BFFFF60` was read from `work/dol/main.elf`; it resolves to
  `0x80077650`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave120` reports this CSV clean.
- The module header of `src/game/stage.rs` gained a wave-120 paragraph naming
  the body (4 bytes), a wave-120 line in the original-range list, and the
  extension of the "functions smaller than 16 bytes" exception to this thunk.
