# Coverage - Wave 108 (`audio::ax`)

## How to read this file

This is the status page for the wave-108 pass over the `audio::ax` lane of the
`main.dol` reconstruction. The `0x801A0000` bucket holds 271 inventory entries;
after wave 105 exactly seven were still unclaimed. Six of them are bytes that
the `runtime::os` range row already owns, so this pass adds the one leaf that
was left: `0x801A9CE0`.

The sections answer, in order:

1. **What did this pass translate?** - *Functions* lists the body with its
   address, Rust name, byte size and confidence.
2. **Why was it previously skipped?** - *What this pass covers* explains what
   the wave-105 pass read into the raw word and what the re-decode found.
3. **What was left out, and why?** - *Deliberate skips* records the six
   addresses that get no body.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:`, `// inferred:` and `// unknown:`.

**Ledger.** The machine-readable rows are `ledger/agents/wave108-ax.csv`
(module `audio::ax`, 1 row, `translated`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 1 | 8 | 1 | 0 |

Status split: 1 `translated`, 0 `partial`, 0 `stub`. Address range:
`0x801A9CE0..0x801A9CE0`.

## What this pass covers

- **The FPU non-IEEE-mode leaf, `0x801A9CE0`.** Wave 105 called this an
  eight-byte leaf "whose only non-branch word writes a non-volatile FPR" and
  gave it no body. The word is `0xFFA0004C`, which decodes as opcode 63
  (the float X-form space), XO `38` and BT `29` - that is `mtfsb1 29`, not an
  FPR write. `mtfsb1` sets one FPSCR bit; BT 29 is the `NI` (non-IEEE mode)
  bit, which makes the FPU flush denormals and stop raising the denormal
  exception. The second word is `0x4E800020` (`blr`), so the FPSCR write is
  the whole effect and the body is a one-instruction `asm!` block.
- **Cross-check on the decode.** `main.dol` contains exactly two `mtfsb1`
  instructions (this one at `0x801A9CE0` and a `mtfsb1 9` at `0x80344D28`) and
  no `mtfsb0`, and the same byte-level reading already backs the neighbouring
  bodies [`fn_801a9b80`], [`fn_801a9ca0`] and [`fn_801a9c20`]/[`fn_801a9c40`],
  whose Ghidra exports are likewise empty. The body is therefore ledgered at
  `high` confidence: the instruction and its operand are read from the file
  bytes at the cited address, not inferred from a call site.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801A9CE0` | `fn_801a9ce0` | 8 | high |

Totals: **1 body**; **1 high, 0 medium**; **8 bytes**.

## Deliberate skips

Reason code **D** means the bytes are already claimed by another module's row,
so a second body would translate the same bytes twice.

| Address | Reason | Note |
|---|---|---|
| `0x801A9BC0..0x801A9C10` | D | Six 8-byte `mtspr` leaves (`0x7C78EBA6`/`0x7C7CEBA6`/`0x7C79EBA6`/`0x7C7AEBA6`/`0x7C7DEBA6`/`0x7C7EEBA6`, i.e. SPRs 952/956/953/954/957/958 from `r3`, each followed by `blr`). They are already owned by the `runtime::os` range row `801a9bc0-801a9c10` (`clear_performance_monitor`). Individual rows would duplicate the same 48 bytes. |

## Verification

- The address and byte size match `work/ghidra/main-gb/export/functions.tsv`
  (`801a9ce0` / `FUN_801a9ce0` / 8 bytes).
- The evidence file exists at the cited
  `work/ghidra/main-gb/export/801a9ce0_FUN_801a9ce0.c` path.
- The CSV row has exactly one `pub unsafe fn fn_801a9ce0` body in
  `src/audio/ax.rs`, and its doc block carries the same confidence.
- `0x801a9ce0` is not present in `ledger/functions.csv`, and the six skipped
  addresses are covered there by the `801a9bc0-801a9c10` range row.
- `cargo check --message-format=short` rebuilds the crate (3.55 s) and reports
  no error and no warning in `src/audio/ax.rs`.
- `node tools/check-lane-csv.mjs wave108` reports `wave108-ax.csv: 1 rows` and
  `OK` for this lane.

[`fn_801a9b80`]: ../src/audio/ax.rs
[`fn_801a9ca0`]: ../src/audio/ax.rs
[`fn_801a9c20`]: ../src/audio/ax.rs
[`fn_801a9c40`]: ../src/audio/ax.rs
