# Coverage - Wave 111 (`game::worldmap`, `0x80100000` bucket close-out)

## How to read this file

Each row is one reconstructed function in `src/game/worldmap.rs`. The address is
the original `main.dol` address, the byte count is taken from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence file is the Ghidra
C export used for the body. `medium` confidence means the instruction sequence
is literal but one or more data roles are inferred. `low` means the variadic
argument area itself cannot be expressed, as explained below.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 3 | 944 | 0 | 1 |

One row is `translated`/`medium` and two are `partial`/`low` (the variadic
pair). None is a `stub`.

## What this pass covers

At the start of this pass the `0x80100000` bucket had exactly four untranslated
entries: `0x80107050`, `0x80107170`, `0x80107460` and `0x801097F0`. Three are
written here, which closes the bucket except for `0x80107460` (see the skip
below). The pass covers two clusters:

- **`0x80107050` / `0x80107170` - the two bounded text builders.** Both open by
  saving `r3..r10` plus `f1..f8` into a contiguous register-save block and
  building the three-word Metrowerks `va_list` descriptor
  `{word, overflow-area pointer, register-save pointer}`; the descriptor words
  are `0x02000000` (next GPR index 2, so `r5` is the first variadic word) and
  `0x03000000` (next GPR index 3, so `r6` is the first variadic word). Both then
  call `0x802E15DC(buffer, 0x400, format, descriptor)`.
  - `0x80107050` treats `r3` as the destination text record and `r4` as the
    format. After formatting it stores the record's first word at `+0x34`
    (Ghidra prints four big-endian byte stores, which is one 32-bit store on the
    target), calls `0x8022D9A0(record + 0x1C)` twice, copies `+0x03` to `+0x5E`,
    copies `+0x04`/`+0x08` to `+0x48`/`+0x4C`, and hands `record + 0x1C`,
    the buffer and `strlen(buffer)` to `0x8022EED0`.
  - `0x80107170` treats `r3` as a four-float output rectangle, `r4` as the text
    record and `r5` as the format. It measures the formatted line with
    `0x8022DFB0` (width) and `0x8022E1C0` (height), then writes
    `[min(w,C), min(h,C), max(w,C), max(h,C)]` where `C` is the SDA float at
    `r2 - 0x5E38`; the comparison/selection order is preserved exactly.
  Both bodies model the variadic tail as explicit `gpr`/`fpr` arrays, the same
  convention already used by `engine::main_loop` for `0x802E16E8`/`0x802E17DC`.
- **`0x801097F0` - the layout-record quad draw.** Reuses the exact `0x38`-byte
  scratch record already reconstructed for its sibling `0x80109780`
  (`0x802D0CC0` constructor, `+0x04` flag word, `+0x34` flag halfword,
  `+0x18`/`+0x1C` SDA words, then `0x802D0F40`). On top of that it gates on
  `inset < fade` (`r2 - 0x5E00` versus `this + 0x1E0`), truncates
  `scale * fade` through `0x802DCC4C` into a white RGBA word
  (`0xFFFFFF00 | alpha`), picks one of two inset pairs from the
  `r13 - 0x4778` region index, builds two rectangle coordinates from the
  `r2 - 0x5DD4`/`-0x5DD0` pair, and calls `0x802C17E0`, `0x802C2300`,
  `0x802D1850` and `0x802C2F70` in that order. The Wave 110 note that this
  function "reads `local_8C`/`local_5C` before any write" is resolved here: both
  are reads *after* `0x802D0CC0` has initialised the record, i.e. they are
  record fields `+0x04` and `+0x34`, not uninitialised locals. The only genuine
  partial-store is the colour word, and its carried low byte is overwritten by
  the immediately following store, so the reconstructed value is exact.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80107050` | `fn_80107050` | 280 | low |
| `0x80107170` | `fn_80107170` | 320 | low |
| `0x801097F0` | `fn_801097f0` | 344 | medium |

The three sizes sum to 944 bytes.

## Deliberate skip

**`0x80107460` (580 bytes).** The visible body is a GX banner pipeline that
calls `0x80107850`, `0x802BFF30`, `0x802BF3D0`, `0x802BFF70`, `0x802C0110`,
`0x802BF650`, `0x802BED10`, `0x801C5860`, `0x801C9940`, `0x802BEF00`, the two
text builders above and `0x80107020`. The blocker is the text record built on
the stack at `r1 - 0xE8`: its vtable word (`local_E8`) is read for the indirect
call through slot `+0x18` but is never written anywhere in the export, and the
paired-single `ldexpf` guards show that several `psq_l`/`psq_st` accesses were
not decompiled either. Reproducing the body would therefore mean inventing both
the vtable pointer and the missing SDA loads, so it is left out rather than
guessed. Everything else needed for a later pass is now available:
`0x80107050`/`0x80107170` are reconstructed, and the record helpers
(`0x80106EE0`, `0x80106F80`, `0x80106FC0`, `0x80106FA0`) were already translated
in this file.

## Notes for the coordinator

- `0x80107050`/`0x80107170` are C-variadic in the binary. Stable Rust cannot
  define such a function, so the rows are `partial`: the register-save block and
  the descriptor are literal, but the caller's overflow-argument area
  (`&stack0x00000008`) is passed as null and the `$cr1` guard around the float
  save area is not modelled (all eight doubles are stored unconditionally).
- No existing constant was redefined. The bodies reuse `W103_R2_5E00`,
  `W103_R2_5DF0`, `W103_R2_5DEC`, `W103_FUN_802D0CC0`, `W103_FUN_802D0F40`,
  `W100_FUN_802DCC4C`, `SDA_4778_REGION`, `read_f32`, `write_f32` and
  `w100_call_f64`; new names are all `W111_*`.
- `0x802DC69C` is `strlen`; `0x802E15DC` is the shared bounded `vsnprintf` from
  `engine::main_loop`.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- None of the three addresses was in `ledger/functions.csv` at the start of the
  pass; all three were untranslated entries of the `0x80100000` bucket.
- `cargo check --message-format=short` reports no error for
  `src/game/worldmap.rs`.
