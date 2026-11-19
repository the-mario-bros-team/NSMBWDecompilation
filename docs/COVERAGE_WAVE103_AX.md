# Coverage - Wave 103 (`audio::ax`)

## How to read this file

This is the status page for the wave-103 pass over the `audio::ax` lane of the
`main.dol` reconstruction. The `0x801A0000` bucket holds 271 inventory entries;
after wave 101, 21 were still unclaimed. This pass reconstructs 7 of them and
records the other 14 here with a reason.

Three answers, in the same order as the sections below:

1. **What did this pass translate?** - *Functions* gives one row per body with
   address, Rust name, byte size and confidence.
2. **What does the band contain?** - *What this pass covers* groups the bodies
   into the `OSContext` snapshot, the SPR 922/923 descriptor pair and the four
   boot-stage control leaves.
3. **What was left out, and why?** - *Deliberate skips* lists every remaining
   address of the bucket with a reason.

Column meanings:

- **Confidence** - strength of the evidence, not of the Rust: `high` = the
  guards, arithmetic, call order and stores are explicit; `medium` = the flow
  is explicit but a register role or the jump/argument convention is
  reconstructed; `low` = only the instruction sequence is certain.
- **Status** - `translated` reproduces the whole exported body; `partial` means
  a named piece of the export has no faithful Rust form and is called out in
  the doc block.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Because several of these
exports drop the body (`801ac710` prints `return;`) or model registers as
pseudo-variables, the doc blocks also quote the raw instruction words, read
from `work/dol/main.elf` at the mapped virtual address. Inline Rust markers
follow `docs/RUST_STYLE.md`: `// observed:`, `// inferred:` and `// unknown:`.

**Ledger.** The machine-readable rows are `ledger/agents/wave103-ax.csv`
(module `audio::ax`, 7 rows: 7 `translated`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 7 | 344 | 3 | 4 |

Status split: 7 `translated`, 0 `partial`, 0 `stub`. Address range:
`0x801AC710..0x801AE050`.

The byte total is small because the band is the last remainder of the bucket:
one 128-byte context snapshot, two 48-byte stack switches, a 40-byte tail jump
and three 8- or 36-byte leaves.

## What this pass covers

- **`0x801ACDB0`, the `OSContext` snapshot.** It stores `r13..r31` at
  `+0x34..+0x7C`, `GQR1..GQR7` at `+0x1A8..+0x1C0`, CR at `+0x80`, LR at
  `+0x84`, CTR at `+0x88`, XER at `+0x8C`, LR again into `SRR0` at `+0x198`
  and MSR into `SRR1` at `+0x19C`; the live stack pointer goes to `+0x04`,
  `r2` to `+0x08`, and the saved `r3` slot at `+0x0C` is set to `1` before the
  routine returns `0`. The layout matches the fields that
  `runtime::os::{init_context, clear_context}` already document, and the
  snapshot is the missing writer between them.
- **`0x801AC710` / `0x801AC740`, the SPR 922/923 descriptor pair.**
  `0x801AC740` is the helper that `fn_801ac770` calls for each 0x1000-byte
  chunk. Both build two words from `(r3, r4, r5)` and write them with
  `mtspr 922` / `mtspr 923`; they differ only in which of `r3`/`r4` receives
  the 29-bit address mask and in the command bit (`0x12` versus `0x02`). The
  Ghidra export keeps almost nothing of either body, so the arithmetic is
  transcribed from the raw words.
- **`0x801ACF10`, the stack-pointer reader.** `mr r3,r1; blr`.
- **`0x801ACF20` / `0x801ACF50`, the alternate-stack call pair.** Both park
  the old stack pointer in the new frame's back-chain slot, switch `r1`,
  store LR in the old frame, install the callback in LR and `blrl` into it;
  the epilogue reads the back-chain word and reloads LR. `0x801ACF20` takes
  `(r3, r4)`; `0x801ACF50` is the same body entered with the callback in `r7`
  and the stack in `r8`. The one in-file caller,
  `fn_801af5d4`, uses the first form to enter `0x801AF7C0` on the arena stack.
- **`0x801AE050`, the tail jump.** It calls `0x801AC5A0` (which sets
  `HID0[ICE]` and returns `HID0 | 0x800`), executes `sync; isync`, and
  `bctr`s through the saved `r3`. Ghidra calls this an unrecovered jump table;
  the raw words are `mtspr 9,r31; bctr`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801AC710` | `fn_801ac710` | 36 | medium |
| `0x801AC740` | `fn_801ac740` | 36 | medium |
| `0x801ACDB0` | `fn_801acdb0` | 128 | high |
| `0x801ACF10` | `fn_801acf10` | 8 | high |
| `0x801ACF20` | `fn_801acf20` | 48 | high |
| `0x801ACF50` | `fn_801acf50` | 48 | medium |
| `0x801AE050` | `fn_801ae050` | 40 | medium |

Totals: **7 bodies**; **3 high, 4 medium**; **344 bytes**.

`src/audio/ax.rs` gained one host-side constant that carries no ledger row,
`FN_801AC5A0` (the I-cache enable callee of `fn_801ae050`).

## Deliberate skips

The bucket's remaining 14 addresses. Reasons reuse the wave-92/101 codes:
**D** duplicate, **F** float/paired-single kernel, **G** paired-single/GQR,
**M** large body reserved for a later pass, **S** the body only writes CPU
state.

| Address(es) | Reason | Note |
|---|---|---|
| `0x801A9BC0..0x801A9C10` | D | Six 8-byte `mtspr` leaves already covered by the `runtime::os` range row `801a9bc0-801a9c10` (`clear_performance_monitor`); individual rows would translate the same bytes twice and the validator rejects merged addresses. |
| `0x801A9CE0` | S/F | Eight bytes whose one non-branch word is `0xFFA0004C` (unique in the image), followed by `blr`. Wave 98 and wave 101 read it as `fneg f29,f0`, but neither the export nor this pass can confirm the mnemonic, and the effect would be only a non-volatile FPR write with no observable result, so no body is claimed. |
| `0x801ACAD0` | G | `ldexpf`/`unaff_GQR0` export. Raw: guard on `context+0x1A2` bit 0, `lfd f0,0x190` + `mtfsf`, then 32 `psq_l` at `+0x1C8+8i` and 32 `lfd` at `+0x90+8i`. It is the FPU-context load half of the `0x801ACC00` save; the paired-single block needs the GQR-aware treatment that `docs/COVERAGE_GX_BAND.md` defers. |
| `0x801ACC00` | G | `ldexpf`/`unaff_GQR0` export. Raw: set the owner bit at `context+0x1A2`, 32 `stfd` at `+0x90+8i`, `mffs`/`stfd` FPSCR at `+0x190`, reload `f0`, then 32 `psq_st` at `+0x1C8+8i`. Same GQR dependency as above. |
| `0x801AD3B0` | G | `ldexpf`/`unaff_GQR0` export. Raw: `mfmsr`/`ori 0x2000`/`mtmsr`/`isync` (enable MSR[FP]), then the same double/FPSCR/paired-single snapshot as `0x801ACC00`. Deferred for the same GQR reason, not for its size. |
| `0x801AE0E0` | M | 1,668-byte first-stage `OSExec` loader; interleaves arena allocation, DVD/IOS reads and the entry jump, and reads a dozen `r13` state words. A pass of its own. |
| `0x801AE840` | M | 1,876-byte `OSExec` body: DVD command blocks, the ELF section copy and the entry hand-off. `0x801AEFA0` already calls into it, so it should be done together with that family. |
| `0x801AF180` | M | 808-byte proportional-font blitter: glyph decode through `0x801B0860`, three-byte pixel writes and per-glyph colour mixing. The export survives but every role is inferred from call order. |
| `0x801AF7C0` | M | 1,512-byte video/YUV bring-up: allocates a `0x96000`-byte buffer, fills it with a `0x1E0 x 0x14 x 0x40` pattern, programs the EFB/VI path through `0x801AF4B0` and `0x801BDCB0`, and reads a large block of `r2` float coefficients. It is entered through `fn_801acf20` and belongs with the video lane. |

The three FPU-context routines are the only skips that a later pass could take
without new information: their raw streams are fully regular (32 double slots
at `+0x90`, FPSCR at `+0x190`, 32 paired-single slots at `+0x1C8`), so a
GQR-aware convention would finish them.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (36 / 36 / 128 / 8 / 48 / 48 / 40; total 344).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/audio/ax.rs`, and its doc block carries the same confidence and status.
- No address in this CSV is present in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning in
  `src/audio/ax.rs` (the remaining errors name other lanes' files).
- `node tools/check-lane-csv.mjs wave103` reports `wave103-ax.csv: 7 rows` and
  no problem naming this file.
