# Coverage - Wave 123 (`game::stage`, closing the `0x80090000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave123-stage.csv` is the Ghidra C export the body was
written from.

The `0x80090000` bucket holds 191 entries and 190 already had
`ledger/functions.csv` rows, so this pass starts from the single open entry:
the 4-byte branch thunk `0x80096700`. Wave 113 had skipped it as a double-count
because its Ghidra export prints the *target* body. The ELF word settles that:
the address holds `b 0x800967F0`, so the thunk is a real 4-byte wrapper and the
target (`fn_800967f0`, 236 bytes) is not re-translated. This closes the bucket
at 191/191.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 1 | 4 | 1 | 0 | 0 |

The single row is `translated`; none is `partial` or `stub`. The pass covers
`0x80096700` (4 bytes).

## What this pass covers

- **The notice/effect spawner thunk** (`0x80096700`, 4 bytes), Ghidra name
  `thunk_FUN_800967f0`. The word at `0x80096700` in `work/dol/main.elf` is
  `0x480000F0`, i.e. `b 0x800967F0` (opcode 18, `LI = +0xF0`). The body is
  [`fn_80096700`], a one-call tail-branch wrapper that forwards its `r3`/`r4`
  to the already-translated [`fn_800967f0`] (`high`), the 236-byte spawner that
  guards on the `0x8009F5F0` match index, builds the position record through
  `0x8006CED0` with the `+0xB0` y override, and selects `0x800E1F80` or
  `0x800E2220` from the `+0x510` selector. A tail branch returns the target's
  result straight to the thunk's caller, so the wrapper has the same observable
  behaviour without duplicating the target body.

This follows the precedent of waves 118, 119, 120 and 121, which landed their
buckets' last 4-byte thunks the same way: the instruction word is read from the
ELF rather than inferred from the thunk export.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80096700` | `fn_80096700` | 4 | high |

Address range: `0x80096700` (1 body, 4 bytes).

## Deliberately skipped

Nothing. The single open entry of the `0x80090000` bucket is landed; the bucket
is now 191/191.

## Verification

- Every address and byte size matches `functions.tsv` (1 row: `0x80096700` = 4;
  4 bytes total).
- The branch word `0x480000F0` was read from `work/dol/main.elf`; it resolves to
  `0x800967F0`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_80096700` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave123` reports this CSV clean.
- The module header of `src/game/stage.rs` gained a wave-123 paragraph naming
  the body (4 bytes), a wave-123 line in the original-range list, and the
  extension of the "functions smaller than 16 bytes" exception to this thunk.
