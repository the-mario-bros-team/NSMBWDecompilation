# Coverage - Wave 101 (`audio::ax`, `0x801A0000` bucket remainder)

## How to read this file

This is the status page for the wave-101 pass over the `audio::ax` lane of the
`main.dol` reconstruction. The `0x801A0000` bucket held 271 inventory entries;
244 were already in `ledger/functions.csv`, leaving 27. This pass reconstructs
the 6 that can be reconstructed honestly and records the other 21 here with a
reason.

Three answers, in the same order as the sections below:

1. **What did this pass translate?** - *Functions* gives one row per body with
   address, Rust name, byte size and confidence.
2. **What does the band contain?** - *What this pass covers* groups the bodies
   into the two audio mixers and the four boot-stage leaves.
3. **What was left out, and why?** - *Deliberate skips* lists every remaining
   address of the bucket with a reason. The lane is short on purpose: wave 98
   had already classified this remainder, and padding it would mean inventing
   behaviour for register-only bodies.

Column meanings:

- **Confidence** - strength of the evidence, not of the Rust: `high` = the
  guards, arithmetic, call order and stores are explicit; `medium` = the flow
  is explicit but a coefficient role, callee role or dropped operand is
  inferred; `low` = only the instruction sequence is certain.
- **Status** - `translated` reproduces the whole exported body; `partial` means
  a named piece of the export has no faithful Rust form and is called out in
  the doc block.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:`, `// inferred:` and `// unknown:`.

**Ledger.** The machine-readable rows are `ledger/agents/wave101-ax.csv`
(module `audio::ax`, 6 rows: 3 `translated`, 3 `partial`).

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 6 | 4,136 | 0 | 5 | 1 |

Status split: 3 `translated`, 3 `partial`, 0 `stub`. Address range:
`0x801A3880..0x801AF5D4`.

The six rows are 4,136 bytes, larger than the wave-92 lane's 1,584 bytes,
because the two mixers alone are 2,776 bytes.

## What this pass covers

- **The two remaining float effect mixers, `0x801A3880`/`0x801A48F0`.** These
  are the audio work of the band. They are the 3- and 4-channel delay/filter
  kernels that sit below the wave-90 resamplers and share the `r2 - 0x51xx`
  coefficient block with the wave-90 tap builders. Each runs 96 frames; per
  frame and channel it reads one sample from a cursor plane (plus the optional
  extra plane), converts it with the `f64` bias at `r2 - 0x5130` (resp.
  `-0x50F0`), runs three delay lines, two one-pole filters and an envelope,
  folds the channel results with the `r2 - 0x5138` (resp. `-0x50F8`)
  cross-feed coefficient, and writes the gained values back into the cursor
  planes. The per-channel cursors advance inside the loop (output plane) or
  after it (the shared tap, sidechain, delay and filter indices).
- **`0x801AC180`, the `0x80`-byte segment reload.** It re-runs the mode setup
  `fn_801abf60(1)` when the machine-configuration word is zero, copies `0x80`
  bytes from `arena_low - 0x80` to `0x81000000`, copies the block at
  `0x80332770` over it, flushes the range through `0x801AC470` and spins. The
  terminal branch is not in the export, so the row is `partial`.
- **`0x801ACE30`, the context-resume patch.** It redirects a saved LR that
  points inside the interrupt wrapper `0x801B1140..0x801B114C`, clears bit 1 of
  the state halfword at `+0x1A2`, and returns the `+0xC`/`+0x10` pair. The
  `rfi` and its MSR composition have no Rust form and are omitted.
- **`0x801AE770`, the IOS boot probe.** It installs the two arena bounds,
  opens the IOS device through `0x801D6230`, issues `0x801D6380(1, 2, ...)`
  twice (second time with a fresh `0xE0`-byte buffer), calls `0x801D62E0` and
  spins if everything succeeded. The three callees are the translated
  `runtime::dvd` bodies.
- **`0x801AF5D4`, the boot hand-off.** It resets the DSP/video/DI paths, polls
  the DI sequence word, waits for the power callbacks or the
  `bus_clock / 4000 * 1000` tick deadline, unlocks EXI channel 0, drains the VI
  FIFO, moves the arena to `0x81400000`, saves the incoming `r3`/`r4`/`r5` at
  `0x8038E8C8..0x8038E8D0`, and switches stacks into `0x801AF7C0` through
  `0x801ACF20`. The timebase comparison and the interrupt-restore operand are
  inferred, so the row is `partial`/`low`.

### The r3/r4 convention

`0x802DCD68`/`0x802DCD6C` are CodeWarrior register-save thunks, not functions.
The 64-bit pair Ghidra prints as their return value is `(r3 << 32) | r4`, so the
high word is the first source argument and the low word the second. The mixers
therefore take `(planes, state)`, which matches the wave-90 family
(`fn_801a7a80(inputs, voice)`); the low word of the pair is the state block.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801A3880` | `fn_801a3880` | 1,316 | medium |
| `0x801A48F0` | `fn_801a48f0` | 1,460 | medium |
| `0x801AC180` | `fn_801ac180` | 460 | medium |
| `0x801ACE30` | `fn_801ace30` | 216 | medium |
| `0x801AE770` | `fn_801ae770` | 200 | medium |
| `0x801AF5D4` | `fn_801af5d4` | 484 | low |

Totals: **6 bodies**; **0 high, 5 medium, 1 low**; **4,136 bytes**.

`src/audio/ax.rs` also gained one host-side helper that carries no ledger row,
`wave101_bump` (advance one ring index and wrap it at its length word).

## Deliberate skips

The bucket's remaining 21 addresses. Reasons reuse the wave-92 codes with one
addition: **D** duplicate, **F** float/paired-single kernel, **M** large body
reserved for a later pass, **R** the export drops part of the body, **S** the
body only writes CPU state (an SPR, an FP register or the stack pointer).

| Address(es) | Reason | Note |
|---|---|---|
| `0x801A9BC0..0x801A9C10` | D | Six 8-byte `mtspr` leaves already covered by the `runtime::os` range row `801a9bc0-801a9c10` (`clear_performance_monitor`); adding individual rows would translate the same bytes twice. |
| `0x801A9CE0` | S | Eight bytes `fneg f29,f0; blr` (wave-98 raw-byte note): it writes a non-volatile FP register and returns nothing observable. |
| `0x801AC710`, `0x801AC740` | R | Two 36-byte SPR 922/923 compositions from three register arguments. Ghidra prints `0x801AC710` as `return;` and reduces `0x801AC740` to a bare `param_1 & 0x1fffffff` mask; the dropped operands are not recoverable. |
| `0x801ACAD0`, `0x801ACC00`, `0x801ACDB0`, `0x801AD3B0` | F | The `OSContext`/FPU save/restore family: the exports read `r13..r31`, `f0..f31`, `FPSCR`, the GQRs and MSR as unaffiliated pseudo-registers, so a Rust body would have to invent the register file. |
| `0x801ACF10`, `0x801ACF20`, `0x801ACF50` | S | Stack-pointer getters/switchers: the returned `r1` and the incoming `r7`/`r8` are not expressible as Rust arguments. |
| `0x801AE050` | R | `sync; isync;` followed by an unrecovered jump table ("too many branches"), so the dispatch target is unknown. |
| `0x801AE0E0`, `0x801AE840`, `0x801AF180`, `0x801AF7C0` | M | The 1,668/1,876/808/1,512-byte `OSExec` loaders, boot-info reader, proportional-font blitter and framebuffer clear; they interleave the disc/ELF loader and are a pass of their own. |

The six duplicated performance-monitor leaves are the only reason the bucket
report still shows them as unclaimed; the `runtime::os` row is the canonical
entry and this pass deliberately adds no second one.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (1,316 / 1,460 / 460 / 216 / 200 / 484).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/audio/ax.rs`, and its doc block carries the same confidence and status.
- No address in this CSV is present in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning in
  `src/audio/ax.rs` (the remaining errors name other lanes' files).
- `node tools/check-lane-csv.mjs wave101` reports `wave101-ax.csv: 6 rows` and
  no problem naming this file.
