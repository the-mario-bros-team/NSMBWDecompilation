# Coverage - Wave 121 (`game::stage`, closing the `0x80080000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave121-stage.csv` is the Ghidra C export the body was
written from.

The `0x80080000` bucket holds 222 entries and 221 already had
`ledger/functions.csv` rows, so this pass starts from the single open entry:
the 4-byte branch thunk `0x80083B30`. It is landed here as a one-call
tail-branch wrapper, which closes the bucket at 222/222.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 1 | 4 | 1 | 0 | 0 |

The single row is `translated`; none is `partial` or `stub`. The pass covers
`0x80083B30` (4 bytes).

## What this pass covers

- **The saved-root-heap restore thunk** (`0x80083B30`, 4 bytes), Ghidra name
  `thunk_FUN_8016e980`. The word at `0x80083B30` in `work/dol/main.elf` is
  `0x480EAE50`, i.e. `b 0x8016E980` (opcode 18, `LI = 0xEAE50`). The body is
  [`fn_80083b30`], a direct call to the already-translated
  [`crate::resource::registry::fn_8016e980`] (`high`), which swaps the saved
  root heap back in through `0x802B9170` and clears the `r13-0x5238` save slot.
  A tail branch returns the target's state straight to the thunk's caller, so
  the wrapper has the same observable behaviour and the target is not
  re-translated.

Wave 117 deliberately skipped this entry because the Ghidra export prints the
*target* body and a second reconstruction looked like a double-count. The ELF
word confirms the 4-byte branch, so the wrapper here is evidence-backed rather
than assumed, and it follows the precedent of waves 118, 119 and 120 in this
same file, which landed their buckets' last 4-byte thunks the same way.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80083B30` | `fn_80083b30` | 4 | high |

Address range: `0x80083B30` (1 body, 4 bytes).

## Deliberately skipped

Nothing. The single open entry of the `0x80080000` bucket is landed; the bucket
is now 222/222.

## Verification

- Every address and byte size matches `functions.tsv` (1 row: `0x80083B30` = 4;
  4 bytes total).
- The branch word `0x480EAE50` was read from `work/dol/main.elf`; it resolves to
  `0x8016E980`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_80083b30` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave121` reports this CSV clean.
- The module header of `src/game/stage.rs` gained a wave-121 paragraph naming
  the body (4 bytes), a wave-121 line in the original-range list, and the
  extension of the "functions smaller than 16 bytes" exception to this thunk.
