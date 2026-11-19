# Coverage - Wave 114 (AX / JAS audio driver)

## How to read this file

This is the status page for the wave-114 pass over the `audio::ax` lane of the
`main.dol` reconstruction. The `0x80190000` bucket holds 241 inventory entries.
After wave 104 exactly one of them had no body: `0x8019F490`, the AI DMA
calibration loop. This pass lands it, so the bucket now has a byte-level body
for all 241 entries.

The sections answer, in order:

1. **What did this pass translate?** - *Functions* lists the one body with its
   address, Rust name, byte size and confidence.
2. **Why was it open, and how is it reconstructed?** - *What this pass covers*
   explains the degenerate Ghidra export and the instruction-level decode that
   replaces it.
3. **Was anything left out?** - *Deliberate skips* is empty for this pass.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. The raw instruction words
come from the ELF image `work/dol/main.elf`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:`, `// inferred:` and `// unknown:`.

**Ledger.** The machine-readable row is `ledger/agents/wave114-ax.csv`
(module `audio::ax`, 1 row, `translated`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 1 | 460 | 0 | 1 |

Status split: 1 `translated`, 0 `partial`, 0 `stub`. Address range:
`0x8019F490`.

## What this pass covers

- **The last open entry of the `0x80190000` bucket, `0x8019F490` (460 bytes).**
  Wave 104 skipped it because the export keeps only the prologue spill call and
  an endless loop:

  ```c
  void FUN_8019f490(void)
  {
    FUN_802dcd74();
    do {
    } while( true );
  }
  ```

  Ghidra also prints eleven `Removing unreachable block` warnings for
  `0x8019F508`, `0x8019F540`, `0x8019F550`, `0x8019F5A8`, `0x8019F5B8`,
  `0x8019F5D8`, `0x8019F600`, `0x8019F610`, `0x8019F61C`, `0x8019F624` and
  `0x8019F644`. That is the evidence defect: the decompiler lost the whole
  body, so no C-level translation was possible from the export alone.

  The 460 bytes are intact in `work/dol/main.elf`, so this pass decodes
  `0x8019F490..0x8019F65C` instruction by instruction and translates them.
  `0x802DCD74` / `0x802DCDC0` are the CodeWarrior `__savegprlr_27` /
  `__restgprlr_27` spill pair (five `stw`/`lwz` of `r27..r31` into the caller's
  frame at `r11-0x14..-4`); they are not calls with observable side effects
  beyond the saved registers. `0x801B5F80` is the 64-bit time-base reader
  (`mftb` TBU/TBL/TBU until stable) already used by `fn_801a1940`.

- **What the decoded body does.** One iteration starts the audio interface
  (AI control bit 5 set, bit 1 cleared, bit 0 set at `0xCD006C00`), spins until
  the low 31 bits of `0xCD006C08` change, and latches the time base as `t0`; it
  then sets control bits 1 and 0, spins on the counter again and latches `t1`.
  It clears bits 1 and 0 (DMA off) and classifies
  `period = t1 - t0` against the five 64-bit scales that `fn_8019f1f0` writes
  around `r13 - 0x519C`. Each scale is a zeroed scratch word (high half)
  followed by the scaled value (low half):

  | pair | words | `fn_8019f1f0` scale | window result |
  |---|---|---|---|
  | `A` | `scratch4:scale4` | `bus/500000 * 3000 / 8000` | threshold |
  | `B` | `scratch0:scale0` | `bus/500000 * 0x7B24 / 8000` | threshold |
  | `C` | `scratch1:scale1` | `bus/500000 * 0xA428 / 8000` | threshold |
  | `D` | `scratch2:scale2` | `bus/500000 * 42000 / 8000` | selected wait |
  | `E` | `scratch3:scale3` | `bus/500000 * 63000 / 8000` | selected wait |

  `period < B - A` selects `D`; `A + B <= period < C - A` selects `E`; the two
  other windows (`B - A <= period < A + B` and `period >= C - A`) repeat the
  measurement. The selected value is added to `t1` and the routine spins on
  `0x801B5F80` until the time base reaches that target. With a 243 MHz bus
  clock the thresholds are 7.13/8.63/9.76 microseconds and the waits are
  10.5/15.75 microseconds; the image does not name the AI/DSP event they guard,
  so that role stays `// unknown:`.

- **Wiring change inside the same file.** `fn_8019f1f0` used to call this
  address as an opaque `platform::call0(FN_8019F490)`. It now calls
  `fn_8019f490()` directly, and the stale "opaque callee" sentences in that
  body's doc block and in the module header were corrected. The ABI matches:
  no arguments, no return value.

- **Bucket status.** With this body the `0x80190000` bucket has an
  individual-body entry for all 241 inventory functions, and the `0x801A0000`
  bucket already closed at 271 in wave 109. The module header now carries a
  twenty-second cluster for this pass.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8019F490` | `fn_8019f490` | 460 | medium |

Totals: **1 body**; **0 high, 1 medium**; **460 bytes**.

Confidence is `medium`, not `high`: every mask, store, spin condition, branch
target and byte count is literal in the image, but the export had to be
replaced by the decode, and the AI control bit roles plus the meaning of the
five windows are inferred from the init function that writes the scales.

## Deliberate skips

None. The single remaining entry of the bucket was reconstructible from the
instruction words.

## Verification

- The address and byte size match `work/ghidra/main-gb/export/functions.tsv`
  (`8019f490`, `FUN_8019f490`, `460`).
- The evidence file exists at the cited
  `work/ghidra/main-gb/export/8019f490_FUN_8019f490.c` path.
- The CSV row has exactly one `pub unsafe fn fn_8019f490()` body in
  `src/audio/ax.rs`, with the same `medium` confidence.
- The doc block's `# Original` line and `functions.tsv` agree on `460 bytes`.
- `8019f490` is not present in `ledger/functions.csv` (atomic or inside a
  range), so the row does not re-claim an existing address.
- `cargo check --message-format=short` reports no errors for
  `src/audio/ax.rs`.
- `node tools/check-lane-csv.mjs wave114` reports
  `wave114-ax.csv: 1 rows` and `OK`.
