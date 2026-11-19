# Coverage - Wave 119 (`game::stage`, closing the `0x800C0000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave119-stage.csv` is the Ghidra C export each body was
written from.

The bucket held 290 entries and 288 already had rows, so wave 119 starts from
the two entries wave 116 deliberately skipped: the 4-byte branch thunks
`0x800C3BA0` and `0x800C8A90`. Both are landed here as one-call tail-branch
wrappers, which closes the bucket at 290/290.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 2 | 8 | 2 | 0 | 0 |

Both rows are `translated`; none is `partial` or `stub`. The pass covers
`0x800C3BA0` and `0x800C8A90` (8 bytes total).

## What this pass covers

- **The angle-normaliser thunk** (`0x800C3BA0`, 4 bytes), Ghidra name
  `thunk_FUN_800c3bac`. The word at `0x800C3BA0` in `work/dol/main.elf` is
  `0x4800000C`, i.e. `b 0x800C3BAC` (opcode 18, `LI = +0xC`). The body is
  [`fn_800c3ba0`], a direct call to the already-translated [`fn_800c3bac`]
  (`game::stage`, `high`), which adds `0xC000` steps to an angle halfword until
  it reaches `0xC000` and returns the sign-extended low halfword. A tail branch
  returns the target's `r3` straight to the thunk's caller, so the wrapper has
  the same observable behaviour and the target is not re-translated.
- **The collection-lookup thunk** (`0x800C8A90`, 4 bytes), Ghidra name
  `thunk_FUN_80007320`. The word at `0x800C8A90` is `0x4BF3E890`, i.e.
  `b 0x80007320` (opcode 18, `LI = -0xC1770`). The body is [`fn_800c8a90`],
  which forwards its `r3`/`r4` to the already-translated
  [`crate::render::geometry::fn_80007320`] (`high`), the lookup that walks the
  `r13-0x48A8` small-data anchor over an object's `+0x0C` list and returns the
  collection object or `0`.

The instruction words above were read from `work/dol/main.elf` (PT_LOAD
mapping, big-endian), not inferred from prose. The Ghidra export for each entry
prints the *target* body (that is how Ghidra renders a thunk), which is why
previous waves treated these two as skips. The ELF words confirm the branches,
so the wrappers here are evidence-backed rather than assumed.

The wrappers are written out, instead of being left as skips again, because
each is the last open entry of the bucket and a 4-byte direct branch is fully
reconstructible; the precedent for a landed tail-branch thunk in this tree is
`fn_800b4600`/`fn_800b54b0` in this same file (wave 118) and `fn_8019df40` in
`src/audio/ax.rs`. Each row owns a distinct address and a distinct 4-byte range,
so nothing is double-counted: the target bodies remain the ones cited by their
existing rows.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C3BA0` | `fn_800c3ba0` | 4 | high |
| `0x800C8A90` | `fn_800c8a90` | 4 | high |

Address range: `0x800C3BA0` and `0x800C8A90` (2 bodies, 8 bytes).

## Deliberately skipped

Nothing. The two open entries of the `0x800C0000` bucket are both landed; the
bucket is now 290/290.

## Verification

- Every address and byte size matches `functions.tsv` (2 rows: `0x800C3BA0` = 4,
  `0x800C8A90` = 4; 8 bytes total).
- The branch words `0x4800000C` and `0x4BF3E890` were read from
  `work/dol/main.elf`; they resolve to `0x800C3BAC` and `0x80007320`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave119` reports this CSV clean.
- The module header of `src/game/stage.rs` gained a wave-119 paragraph naming
  both bodies (8 bytes), a wave-119 line in the original-range list, and the
  extension of the "functions smaller than 16 bytes" exception to these two
  thunks.
