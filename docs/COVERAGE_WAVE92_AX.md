# Coverage - Wave 92 (`audio::ax`, `0x801A0000` bucket tail)

## How to read this file

This is the status page for the wave-92 pass over the `audio::ax` lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Functions* gives one row
   per function with address, Rust name, byte size and confidence.
2. **What is in this band and how does it relate to the audio driver?** -
   *What this pass covers* groups the 28 bodies. Wave 90 had already finished
   the mixer span (`0x801A08F0..0x801A93F0`); this pass takes the remaining
   `0x801A0000` bucket entries, which are the JAI block-list tail and the
   kernel/cache/charset leaves above it.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists the
   47 remaining untranslated entries of the bucket with a reason.

Column meanings:

- **Confidence** - strength of the evidence, not code quality: `high` = the
  guards, arithmetic, call order and stores are explicit in the Ghidra C;
  `medium` = the flow is explicit but one constant, field role or dropped
  argument is inferred; `low` = mostly guesswork (none this pass).
- **Reason codes** (deferred inventory):
  - **D** - duplicate: the same routine is already carried by another module or
    is the target of an existing ledger row.
  - **F** - float/paired-single kernel too tangled to resolve honestly here.
  - **M** - large body reserved for a later pass.
  - **O** - kernel/OS-shaped code with no audio role.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output),
`// inferred:` (role guessed from call sites), `// unknown:` (unresolved).

**Ledger.** The machine-readable rows are `ledger/agents/wave92-ax.csv`
(module `audio::ax`, 28 rows: 27 `translated`, 1 `partial`).

**Ownership note.** The audio code in this bucket ends at the wave-90 mixers.
Every address below belongs to the kernel tail that follows the JAI block
pools; the bodies are reconstructed here because this lane owns the `0x801A0000`
bucket. Where a function is plainly an OS primitive, the doc block says so and
the skipped siblings are listed as **O** in *Deferred inventory*.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 28 | 1,584 | 22 | 6 |

Status split: 27 `translated`, 1 `partial`, 0 `stub`. No `low`.

## What this pass covers

- **JAI block-list tail, `0x801ABCF0`.** The wave-5 allocator
  ([`fn_801abb40`](../src/audio/ax.rs), [`fn_801abd80`](../src/audio/ax.rs),
  [`fn_801abdf0`](../src/audio/ax.rs)) builds the `r13 - 0x4FEC` block table.
  `0x801ABCF0` is the matching unlink: it removes a block from a bucket's
  in-use list and hands it back to the free list. This is the only body in the
  pass that is definitely audio/JAI code.
- **Arena and allocator words, `0x801ABD70`, `0x801ABE60..0x801ABF20`.** Twelve
  leaves read or write the four SDA words that hold the arena bounds and two
  allocator cursors, plus the alignment routine `0x801ABF20` that carves a
  request out of the low bound. Three addresses are alias entry points with
  identical bodies (`0x801ABE80`, `0x801ABEB0`, `0x801ABEE0`, `0x801ABF10`).
- **DSP halt path, `0x801AC350`.** Clears the audio-DMA start bit, writes DSP
  control `0x08AC` and command 0, waits for the mailbox busy bit, holds for
  `0x2C` timebase ticks and spins. This is the counterpart of
  [`fn_8019f180`](../src/audio/ax.rs); the tail is `partial` because Ghidra
  removed three unreachable blocks.
- **Device leaves, `0x801A9C90`, `0x801A9E70`, `0x801AB1B0`, `0x801AB1C0`,
  `0x801AB1D0`, `0x801AB260`, `0x801AC770`, `0x801AC810`, `0x801AC820`,
  `0x801ACDA0`.** A `sync` barrier, the RTC counter decoder used by timestamp
  records, the DI status byte, the fixed-format report, the machine-code copy,
  one configuration byte, the chunked range walker and the two `HID2`
  locked-cache accesses.
- **Self-contained converters, `0x801ADD20`, `0x801AFDB0`.** A UTF-16BE to
  lowercase-hex formatter and the font-map lookup with its two double-byte
  ranges. Both are pure table/arithmetic code with no unresolved callee.

The two bodies that close previously dangling call edges are `0x801ABCF0`
(calls [`fn_801abb40`](../src/audio/ax.rs)) and `0x801AC770` (calls the
three-register helper `0x801AC740`, which remains untranslated).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801A9C90` | `fn_801a9c90` | 12 | high |
| `0x801A9E70` | `fn_801a9e70` | 108 | high |
| `0x801AB1B0` | `fn_801ab1b0` | 16 | high |
| `0x801AB1C0` | `fn_801ab1c0` | 16 | medium |
| `0x801AB1D0` | `fn_801ab1d0` | 132 | high |
| `0x801AB260` | `fn_801ab260` | 32 | high |
| `0x801ABCF0` | `fn_801abcf0` | 120 | medium |
| `0x801ABD70` | `fn_801abd70` | 16 | high |
| `0x801ABE60` | `fn_801abe60` | 8 | high |
| `0x801ABE70` | `fn_801abe70` | 8 | high |
| `0x801ABE80` | `fn_801abe80` | 8 | high |
| `0x801ABE90` | `fn_801abe90` | 8 | high |
| `0x801ABEA0` | `fn_801abea0` | 8 | high |
| `0x801ABEB0` | `fn_801abeb0` | 8 | high |
| `0x801ABEC0` | `fn_801abec0` | 8 | high |
| `0x801ABED0` | `fn_801abed0` | 8 | high |
| `0x801ABEE0` | `fn_801abee0` | 8 | high |
| `0x801ABEF0` | `fn_801abef0` | 8 | high |
| `0x801ABF00` | `fn_801abf00` | 8 | high |
| `0x801ABF10` | `fn_801abf10` | 8 | high |
| `0x801ABF20` | `fn_801abf20` | 52 | high |
| `0x801AC350` | `fn_801ac350` | 204 | medium |
| `0x801AC770` | `fn_801ac770` | 160 | medium |
| `0x801AC810` | `fn_801ac810` | 12 | medium |
| `0x801AC820` | `fn_801ac820` | 20 | medium |
| `0x801ACDA0` | `fn_801acda0` | 12 | high |
| `0x801ADD20` | `fn_801add20` | 340 | high |
| `0x801AFDB0` | `fn_801afdb0` | 312 | high |

Totals: **28 translated/partial, 0 stubs**; **22 high, 6 medium, 0 low**;
**1,584 bytes**.

## Deferred inventory

The bucket holds 271 `main.dol` entries; 196 were in `ledger/functions.csv`
(155 of them `audio::ax`) when this pass started, leaving 75. This pass
reconstructs 28; the 47 below stay untranslated.

| Address(es) | Reason | Note |
|---|---|---|
| `0x801A9AF0`, `0x801A9B00` | D | Allocator hook get/set pair; wave 90 recorded them as already carried by `resource::archive` (the `0x80265330` pair). |
| `0x801A9BC0..0x801A9C10` | D | The six 8-byte performance-monitor leaves are already covered by the `runtime::os` range row `801a9bc0-801a9c10` (`clear_performance_monitor`). |
| `0x801A9B80`, `0x801A9CA0`, `0x801A9CE0` | O | Bare 8-byte `blr` leaves with no observable effect; padding, not translation. |
| `0x801A9C20`, `0x801A9C40` | O | FPSCR image capture/restore and the `mtfsf` install; owned by `runtime::init`/`runtime::os`, and the export reads every FPSCR bit as a pseudo-register. |
| `0x801A9EE0`, `0x801AA550`, `0x801AAD60` | O | Console-type decode, the kernel banner printer and the `OSInit` exception-vector installer (640 bytes, `sync`/ICFI loops). |
| `0x801AA140`, `0x801AA320` | M | The two cache flush/invalidate loops around the DVD read buffer (476/484 bytes); six nested branches over two cursors, better done with the DVD lane. |
| `0x801ABF60`, `0x801AC180`, `0x801AC710`, `0x801AC740` | O | EXI/DVD register bring-up, the 0x80-byte segment snapshot, and the two 36-byte cache leaves whose bodies Ghidra printed with most instructions dropped (`0x801AC710` is `return;` for 36 bytes). Too little of the real sequence survives to translate honestly. |
| `0x801ACAD0`, `0x801ACC00`, `0x801ACD30`, `0x801ACDB0`, `0x801ACE30`, `0x801ACF10`, `0x801ACF20`, `0x801ACF50` | O | FPU context save/restore (`psq_st`/`psq_l` pairs), `FPSCR` composition, interrupt return, and the `r2`-relocating call thunks. Ghidra models these with pseudo-registers and stack frames. |
| `0x801AD070`, `0x801AD3B0`, `0x801ADBA0`, `0x801ADE80`, `0x801AE050`, `0x801AE0E0`, `0x801AE770`, `0x801AE840`, `0x801AEFA0`, `0x801AF180` | M | The DVD/DI command and DMA block (608..1,876 bytes each). They belong to `runtime::dvd`; several end in unrecovered jump tables. |
| `0x801AF4B0`, `0x801AF5D0`, `0x801AF5D4`, `0x801AF7C0`, `0x801AFEF0` | O | The 4-byte thunk, the EXI transfer helper and the three boot-stage state machines; too large or unmodelled, and none has an audio role. |

## Uncertainties

- **`0x801ABCF0` (JAI unlink).** The export calls `0x801ABB40` with one visible
  argument. The block header is 0x20 bytes before the payload (the two links
  the function unlinks live at `payload - 0x20` / `- 0x1C`), so the dropped
  second argument is modelled as `data - 0x20`. Marked `// inferred:`.
- **`0x801AC350` (DSP halt).** Ghidra removed three unreachable blocks
  (`0x801AC380`, `0x801AC398`, `0x801AC40C`) and prints the tail as
  `do { } while (true)`. The Rust body reproduces the exported control flow and
  is `partial`, not `translated`.
- **`0x801AC770` (range walker).** The callee `0x801AC740` is declared with one
  parameter, but the call site passes three registers. The call is modelled
  with `platform::call3` and marked `medium`.
- **`0x801AC810`/`0x801AC820` (`HID2`).** Both go through the existing
  [`crate::runtime::os::get_hid2`](../src/runtime/os.rs), so no inline assembly
  is needed here. Bits 24..27 are named "locked-cache field" from the Dolphin
  `HID2` layout; that name is inferred, the shift and mask are literal.
- **`0x801A9E70` (RTC).** The nibble arithmetic is exact. Which calendar field
  each byte holds is inferred from the `+2000` biased two-digit year.
- **Machine-code leaves.** `0x801AB1D0` / `0x801AB260` read the 4-byte code at
  `0x80003180`/`0x80003194`; the "machine/game code" role is inferred from the
  4+1 byte shape, the addresses and the flag at `r13 - 0x5048`.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn` body in `src/audio/ax.rs`.
- No address in this CSV is present in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error in `src/audio/ax.rs`.
- `node tools/check-lane-csv.mjs wave92` reports `OK` for `wave92-ax.csv`.
