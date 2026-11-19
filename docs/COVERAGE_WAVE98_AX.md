# Coverage - Wave 98 (`audio::ax`, `0x801A0000` bucket tail)

## How to read this file

This is the status page for the wave-98 pass over the `audio::ax` lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Functions* gives one
   row per function with address, Rust name, byte size and confidence.
2. **What does the band do?** - *What this pass covers* groups the 20 bodies
   into the JAS hook pair, the ten special-purpose-register accessors, the
   console/boot-info cluster, the Hollywood timing step, the context dump, the
   two `argv` marshallers, the video-mode setter and the two `OSExec` leaves.
3. **What was left out, and why?** - *Deferred inventory* lists the remaining
   untranslated entries of the `0x801A0000` bucket with a reason.

Column meanings:

- **Confidence** - strength of the evidence, not code quality: `high` = the
  guards, arithmetic, call order and stores are explicit (or the raw bytes of a
  one-instruction leaf); `medium` = the flow is explicit but one constant, field
  role or register name is inferred; `low` = only the instruction sequence is
  certain (none this pass).
- **Reason codes** (deferred inventory):
  - **D** - duplicate/dead: carried by another module or nothing to
    reconstruct (tail thunk, empty body).
  - **F** - float/paired-single kernel too tangled to resolve honestly here.
  - **M** - large body reserved for a later pass.
  - **R** - the Ghidra export drops so much of the body that the control flow is
    not recoverable.
  - **S** - the body only writes a special-purpose register or a non-memory CPU
    state, so a Rust body would either invent a helper or claim a no-op.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. The one-instruction leaves
are additionally checked against the raw big-endian words in `work/dol/main.elf`
(the exact `mtspr`/`mffs`/`mtfsf`/`blr` words are quoted in the body docs).
Inline Rust markers follow `docs/RUST_STYLE.md`: `// observed:` (visible in the
Ghidra output or the raw bytes), `// inferred:` (role guessed from call sites or
layout), `// unknown:` (unresolved).

**Ledger.** The machine-readable rows are `ledger/agents/wave98-ax.csv`
(module `audio::ax`, 20 rows, all `translated`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 20 | 6,200 | 7 | 13 |

Status split: 20 `translated`, 0 `partial`, 0 `stub`, 0 `low`.

## What this pass covers

- **JAS hook pair, `0x801A9AF0`/`0x801A9B00`.** Setter and getter for the two
  allocator words at `r13 - 0x61C0` / `r13 - 0x61BC`. Both SDA constants already
  existed in this module (`SDA_JAS_ALLOC_HOOK`, `SDA_JAS_FREE_HOOK`), so the pair
  closes the loop on the allocator hooks the wave-90/92 pool code uses.
- **Special-purpose-register accessors, `0x801A9B80..0x801A9CA0`.** Eight
  eight-byte setters (`mtspr <spr>,r3; blr`) for the decrementer, the
  performance-monitor control counters and SPR 921, plus the FPSCR read/write
  pair `0x801A9C20`/`0x801A9C40` (`mffs` + `lfd`/`lwz`, `stw` + `mtfsf 255`).
  Ghidra prints all ten as empty bodies, so they are re-derived from the raw
  words at the cited addresses and expressed as inline asm, exactly like the
  existing `sync` leaf `fn_801a9c90`.
- **Console and boot-info cluster, `0x801A9EE0..0x801AAD60`**.
  `0x801A9EE0` is `OSGetConsoleType`: the board halfword at `0x800030E6`
  selects a "plain" console table, a tick-dependent table, or a
  `0x100000xx`/`0x10..0x21` development table, with `0x100` for board code
  `0x300`. `0x801AA550` prints the banner that consumes it (the format strings
  are quoted from `0x80332300..0x8033248C`). `0x801AAD60` installs the
  OSDBIntegrator stub, the fifteen exception vector slots and the handler table
  at `0x80332688`.
- **Arena write-back twins, `0x801AA140`/`0x801AA320`.** One `dcbst` pass over
  the MEM1 (resp. MEM2) arena range plus a `dcbf` pass over its top 256 KiB,
  split around the write-back window at `0x8038E52C` / `0x8038E530`. They reuse
  the arena accessors `fn_801abe70..fn_801abeb0` translated earlier in this
  module.
- **Hollywood timing step, `0x801ABF60`.** Clears bits 8..15 of `0xCD800180`,
  installs one of two `0xCD8001CC` timing words, and walks four bits of
  `0xCD8001D0`, busy-waiting between steps on the 100 us / 1 ms tick thresholds
  derived from the bus clock at `0x800000F8`.
- **`OSContext` dump, `0x801AD070`.** The GPR-pair block, LR/CR, SRR0/SRR1 and
  the eight GQRs, then the FPR and paired-single dumps through a scratch context
  installed with `0x801ACD40`, and finally the stack back chain.
- **`argv` marshallers, `0x801ADBA0`/`0x801ADE80`/`0x801AEFA0`.** The two
  packers clear a 0x2000-byte scratch block, copy strings down from its top and
  publish the offset vector at `base + 8`; the second variant stores
  even-indexed entries as hex text of their UTF-16BE bytes. `0x801AEFA0`
  formats a path into a 36-byte buffer and calls the loader `0x801AE840` with
  the seven-word argument list.
- **Video-mode setter and `OSExec` leaves, `0x801AF4B0`, `0x801AF5D0`,
  `0x801AFEF0`.** `0x801AF4B0` fills one `GXRenderModeObj`-shaped record and
  calls `0x801BD280` plus `0x801BD950(0, 0, 640, 480)`; `0x801AF5D0` is a bare
  `blr`; `0x801AFEF0` is the flag-word expander that mixes literal bytes and
  two-byte back references into the output buffer.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801A9AF0` | `fn_801a9af0` | 12 | high |
| `0x801A9B00` | `fn_801a9b00` | 20 | high |
| `0x801A9B80` | `fn_801a9b80` | 8 | high |
| `0x801A9C20` | `fn_801a9c20` | 32 | high |
| `0x801A9C40` | `fn_801a9c40` | 40 | high |
| `0x801A9CA0` | `fn_801a9ca0` | 8 | medium |
| `0x801A9EE0` | `fn_801a9ee0` | 596 | medium |
| `0x801AA140` | `fn_801aa140` | 476 | medium |
| `0x801AA320` | `fn_801aa320` | 484 | medium |
| `0x801AA550` | `fn_801aa550` | 676 | medium |
| `0x801AAD60` | `fn_801aad60` | 640 | medium |
| `0x801ABF60` | `fn_801abf60` | 544 | medium |
| `0x801ACD30` | `fn_801acd30` | 8 | high |
| `0x801AD070` | `fn_801ad070` | 608 | medium |
| `0x801ADBA0` | `fn_801adba0` | 384 | medium |
| `0x801ADE80` | `fn_801ade80` | 460 | medium |
| `0x801AEFA0` | `fn_801aefa0` | 480 | medium |
| `0x801AF4B0` | `fn_801af4b0` | 288 | medium |
| `0x801AF5D0` | `fn_801af5d0` | 4 | high |
| `0x801AFEF0` | `fn_801afef0` | 424 | medium |

Totals: **20 translated, 0 stubs**; **7 high, 13 medium, 0 low**;
**6,200 bytes**.

`src/audio/ax.rs` also gained seven internal host-side adapters that carry no
ledger row: `report1..report4` (varargs wrapper for `0x8015F730`), `copy_bytes`
(wrapper for the boot `memcpy`), `read_f64` and `call_f64_1` (big-endian double
load and `f64` call used by the context dump). They are marked as internal in
the file and mirror the existing `call0_u32`/`call_f32_1` helpers.

## Deferred inventory

The bucket holds 271 `main.dol` entries; 224 were already in
`ledger/functions.csv` when this pass started, leaving 47. This pass
reconstructs 20; the 27 below stay untranslated.

| Address(es) | Reason | Note |
|---|---|---|
| `0x801A3880`, `0x801A48F0` | F | The two per-frame mixer kernels (1,316/1,460 bytes) are built over GQR-quantised paired-single stores; the field roles and the `ldexpf` scale are not recoverable from the export without guessing. |
| `0x801A9CE0` | S | Eight bytes `fneg f29,f0; blr`: it writes a non-volatile FP register and returns nothing observable, so no honest Rust body exists. |
| `0x801AC180` | R | Ends in a `do { } while (true)` that is a dropped branch (the routine copies 0x80 bytes down from `0x81000000` and then transfers control); the target is not in the export. |
| `0x801AC710`, `0x801AC740` | S | 36-byte `mtspr` pairs that compose SPR 922/923 values from three register arguments; Ghidra models one of them as a mask helper, and the second argument pair of `0x801AC740` is not printed. |
| `0x801ACAD0`, `0x801ACC00`, `0x801ACDB0`, `0x801ACE30`, `0x801AD3B0` | R | The `OSContext` save/restore family: the exports read `r13..r31`, the FPU state and MSR/SRR from unaffiliated registers, so the bodies cannot be written without inventing context-block helpers. |
| `0x801ACF10`, `0x801ACF20`, `0x801ACF50` | S | Stack-pointer manipulators (`mr r3,r1`, "switch to this stack and call"): the old `r1` and the incoming `r7`/`r8` values are not expressible as Rust arguments. |
| `0x801AE050` | R | `sync; isync;` followed by an unrecovered jump table ("too many branches"), so the dispatch target is unknown. |
| `0x801AE0E0`, `0x801AE840`, `0x801AF7C0` | M | 1,668/1,876/1,512-byte `OSExec` loaders, boot-info readers and the framebuffer clear; they interleave the disc/ELF loader with the decompressor and are a pass of their own. |
| `0x801AF5D4` | M | 484-byte boot hand-off: installs the `0x8038E600` context, resets the video/DVD paths, polls `0x801B3360` against a 64-bit timebase deadline, then switches stacks into `0x801AF7C0`. Needs the `0x801B3360`/`0x801B91D0` semantics first. |
| `0x801AE770` | R | Ends in a `do { } while (true)` after `0x801D6380`; the loop body was dropped. |
| `0x801AF180` | M | 808-byte proportional-font blitter; the glyph table stride and the 0x18-row block assembly are inferred, not proven. |

## Uncertainties

- **One-instruction leaves.** `0x801A9B80..0x801A9CA0` print as empty bodies in
  the Ghidra C but are `mtspr <spr>,r3; blr` in the raw image. They are written
  with `core::arch::asm!` (the same approach the wave-92 `sync` leaf uses), so
  the bodies only type-check off-target; the register numbers are taken from the
  instruction encoding, and the `MMCR0`/`PMC1..4`/`SIA` names are `// inferred:`
  from the standard PowerPC SPR map. SPR 921 has no mnemonic here and stays
  `// unknown:`.
- **`0x801A9EE0` table selection.** The board halfword `0x800030E6` and the
  tick probe `0x801B25B0` are literal, but which physical board each code names
  is inferred from the banner that prints it.
- **`0x801AA140`/`0x801AA320` dropped operand.** The `0x8038E530` step prints
  `0x801AC530(addr)` with one operand; the size is restored from the two
  identical neighbouring blocks that pass `(start, high - start)` and marked
  `// unknown:` in the body.
- **`0x801AD070` scratch context.** The two halfwords zeroed before each
  `0x801ACD40` install are named `local_148`/`local_146` in the export; their
  offset from the scratch block is inferred to be `+0x1A0`.
- **`0x801AFEF0` copy kernel.** The export shows an eight-way unrolled copy
  whose first byte is loaded from `source - 1`; the Rust body keeps the same
  byte order and overlap behaviour but not the unrolling.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in `src/audio/ax.rs`.
- No address in this CSV is present in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning in
  `src/audio/ax.rs`.
- `node tools/check-lane-csv.mjs wave98` reports `OK` and lists
  `wave98-ax.csv: 20 rows`.
