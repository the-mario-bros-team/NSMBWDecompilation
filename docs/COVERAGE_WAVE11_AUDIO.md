# Coverage - Wave 11 audio (AX/JAS/JAI) `0x801A00C0..0x801ABDF0`

## How to read this file

This is the status page for the wave-11 pass over the audio lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Selected functions*
   gives one row per function with address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
2. **How do the new functions fit the existing JAS driver?** - *Clusters*
   explains the pool/voice-cost pass, the size queries, the two sequence
   sample-buffer sets and the JAI block pool.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists
   the nearby addresses that were not selected, followed by *Uncertainties*
   and the ledger pointer.

Column meanings:

- **Status** - `translated` means the entry/exit, every call and every
  observable memory write is accounted for. There are no `partial` or `stub`
  rows in this pass.
- **Confidence** - strength of the evidence, not code quality:
  `high` = the arithmetic or store sequence is explicit in the Ghidra C;
  `medium` = the flow is explicit but a callee role, an unnamed constant or a
  return convention is inferred; `low` = mostly guesswork.
- **Reason codes** (deferred inventory):
  - **M** - large mixer/voice body reserved for a later pass.
  - **O** - opaque or unusable decompiler output (GQR/`ldexpf` artifacts).
  - **R** - audio-adjacent but not selected (needs a caller or class layout).
  - **S** - tiny accessor, below the 24-byte selection floor.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output), `// inferred:`
(role guessed from call sites), `// unknown:` (unresolved), `// partial:`
(deliberately incomplete).

**Promotion status.** All wave-11 functions live in `src/audio/ax.rs`. The
machine-readable ledger is `ledger/agents/wave11-audio.csv` (module
`audio::ax`, 22 rows, all `translated`). Earlier audio rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv` and
`wave7-audio.csv`.

## Range inventory

Wave 11 is not one contiguous band: it fills gaps around the wave-7 note
families and the JAI timer queue. The selected entries are:

| Span | Purpose | Selected |
|---|---|---:|
| `0x801A00C0..0x801A0E50` | pool head and SDA cost readers | 2 |
| `0x801A0380` | JAS pool acquire/reclassify | 1 |
| `0x801A25C0..0x801A3DB0` | voice parameter/cost pass and base buffers | 3 |
| `0x801A6090..0x801A7690` | remaining size queries | 2 |
| `0x801A8400..0x801A8E10` | 3-voice sequence set | 6 |
| `0x801A8F20..0x801A9990` | 4-voice sequence set | 6 |
| `0x801ABD80..0x801ABDF0` | JAI block pool | 2 |

Totals for this wave: **22 translated, 0 partial, 0 stubs**;
**16 high, 6 medium, 0 low**.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x801A00C0` | `fn_801a00c0` | Read one JAS pool head by index. | P | translated | high | `801a00c0_FUN_801a00c0.c` |
| `0x801A0E50` | `fn_801a0e50` | Read the SDA cost base at `r13 - 0x511C`. | P | translated | high | `801a0e50_FUN_801a0e50.c` |
| `0x801A0380` | `fn_801a0380` | Take a free pool object and reclassify it. | P | translated | high | `801a0380_FUN_801a0380.c` |
| `0x801A25C0` | `fn_801a25c0` | Apply one voice parameter block. | P | translated | high | `801a25c0_FUN_801a25c0.c` |
| `0x801A2B10` | `fn_801a2b10` | Run the per-frame voice cost pass. | P | translated | medium | `801a2b10_FUN_801a2b10.c` |
| `0x801A3DB0` | `fn_801a3db0` | Allocate the base voice sample buffers. | P | translated | high | `801a3db0_FUN_801a3db0.c` |
| `0x801A6090` | `fn_801a6090` | Byte size of the 4-buffer set. | Q | translated | high | `801a6090_FUN_801a6090.c` |
| `0x801A7690` | `fn_801a7690` | Byte size of the 4-voice sequence set. | Q | translated | high | `801a7690_FUN_801a7690.c` |
| `0x801A8400` | `fn_801a8400` | Size constant of the 3-voice set. | Q | translated | high | `801a8400_FUN_801a8400.c` |
| `0x801A8F20` | `fn_801a8f20` | Size constant of the 4-voice set. | Q | translated | high | `801a8f20_FUN_801a8f20.c` |
| `0x801A8410` | `fn_801a8410` | Start the 3-voice sequence set. | R | translated | medium | `801a8410_FUN_801a8410.c` |
| `0x801A8670` | `fn_801a8670` | Restart the 3-voice sequence set. | R | translated | high | `801a8670_FUN_801a8670.c` |
| `0x801A8820` | `fn_801a8820` | Free the 3-voice sequence set. | R | translated | high | `801a8820_FUN_801a8820.c` |
| `0x801A8C10` | `fn_801a8c10` | Configure the 3-voice sequence set. | R | translated | medium | `801a8c10_FUN_801a8c10.c` |
| `0x801A8E10` | `fn_801a8e10` | Resample 96 samples (3-voice). | R | translated | high | `801a8e10_FUN_801a8e10.c` |
| `0x801A8F30` | `fn_801a8f30` | Start the 4-voice sequence set. | S | translated | medium | `801a8f30_FUN_801a8f30.c` |
| `0x801A91B0` | `fn_801a91b0` | Restart the 4-voice sequence set. | S | translated | high | `801a91b0_FUN_801a91b0.c` |
| `0x801A9360` | `fn_801a9360` | Free the 4-voice sequence set. | S | translated | high | `801a9360_FUN_801a9360.c` |
| `0x801A9780` | `fn_801a9780` | Configure the 4-voice sequence set. | S | translated | medium | `801a9780_FUN_801a9780.c` |
| `0x801A9990` | `fn_801a9990` | Resample 96 samples (4-voice). | S | translated | high | `801a9990_FUN_801a9990.c` |
| `0x801ABD80` | `fn_801abd80` | Initialise the JAI block pool. | T | translated | medium | `801abd80_FUN_801abd80.c` |
| `0x801ABDF0` | `fn_801abdf0` | Allocate one JAI block. | T | translated | high | `801abdf0_FUN_801abdf0.c` |

## Clusters

### Cluster P - pool acquire and voice cost (`0x801A00C0..0x801A3DB0`)

`fn_801a0380` is the pool acquire step. The two incoming registers are packed
as `pool << 32 | callback`: the high word is the requested size class, the low
word a completion callback. Pool 0 is popped directly; when the class is 2 or
more the lower classes are scanned from `class - 1` down to 1. A free object is
unlinked from the tail of its old class, inserted at the head of the requested
class, stamped with the packed word at `+0x0C` (installing the callback at
`+0x10`) and the owner word at `+0x14`, then reset by `fn_801a2e30`. The
installed callback, when non-null, runs with the object. Caller `0x80192E60`
passes `(0x1F, 0x80193E40, owner)`, which fixes the low-word role.

`fn_801a25c0` is a large flag-driven copy/apply. The `+0x1C` flag word is a
bitmap of source-to-destination field groups; a zero flag word reads four
values back from the destination, and a negative flag word copies the whole
0x140-byte block from source `+0x28`.

`fn_801a2b10` is the frame cost pass. It walks pool classes 31..1, accumulates
the per-note overhead and the priority costs from `0x8032E398`/`0x8032E418`,
and either applies a note (`fn_801a25c0`) or finishes it and moves it to the
pending list. It publishes the total at `r13 - 0x5064` and flushes both
per-voice tables through `0x801AC440`/`0x801AC470`.

`fn_801a3db0` is the base 3-voice buffer allocator, pairing with the wave-5
`fn_801a3f30`/`fn_801a4060` clear/free halves.

### Cluster Q - size queries (`0x801A6090..0x801A8F20`)

Four small size helpers complete the wave-7 set: the 4-buffer `+0x40` scale
(`r2 - 0x50B0`) shifted by four, the 4-voice sequence `+0xD4` formula scaled by
`r2 - 0x5058` and multiplied by `0x10`, and the two fixed constants `0x9600`
and `0xC800`.

### Cluster R - 3-voice sequence set (`0x801A8410..0x801A8E10`)

The 3-voice set uses three buffers at `+0x00/+0x04/+0x08`, a sample count at
`+0x1C` (`0xC80`), phase words at `+0x0C..+0x18`, the rate at `+0x80`, the
pitch window at `+0x80..+0x9C`, and the stop bit at `+0x7C`.

`fn_801a8410` allocates, clears and configures; `fn_801a8670` restarts without
reallocating; `fn_801a8820` frees; `fn_801a8c10` validates the pitch window and
writes the phase ratios; `fn_801a8e10` runs the 96-sample fixed-point
resampler. The shared failure tail takes a fresh interrupt lock, sets bit 0,
frees all three pointers and restores both lock tokens.

### Cluster S - 4-voice sequence set (`0x801A8F30..0x801A9990`)

The 4-voice set adds a fourth buffer at `+0x0C`, moves the count to `+0x20`,
the rate to `+0x94`, the pitch window to `+0x94..+0xB0`, and the stop bit to
`+0x90`. `fn_801a8f30` only runs when the JAS mode word is 2; unlike the
auxiliary and 3-voice wrappers, its success path only clears bit 0.
`fn_801a91b0` is the restart and stores `(flags & ~1) | 2` on success;
`fn_801a9360` frees; `fn_801a9780` configures; `fn_801a9990` resamples.

### Cluster T - JAI block pool (`0x801ABD80..0x801ABDF0`)

The block table lives at `r13 - 0x4FEC`, the record count at `r13 - 0x4FF0`,
the aligned pool limit at `r13 - 0x4FF8`, the aligned first free address at
`r13 - 0x4FF4`, and the initialised marker at `r13 - 0x6188`. Each record is
three words: `+0x00` size (negative when free), `+0x04` block pointer, `+0x08`
next/owned pointer. `fn_801abd80` marks every record free, publishes the
aligned pool limit and first free address, and returns that address (the
caller `0x802DD3C4` uses it); `fn_801abdf0` claims the first free record and
writes the three-word block header at the requested address.

## Deferred inventory

| Address | Symbol in export | Reason | Note |
|---|---|---|---|
| `0x801A08F0` | `FUN_801a08f0` | M | 940-byte per-frame helper. |
| `0x801A0E90` | `FUN_801a0e90` | M | 2492-byte JAS mixer for a 0x180-byte DMA block. |
| `0x801A3880` | `FUN_801a3880` | M | 1316-byte base-voice mixer loop. |
| `0x801A4180` | `FUN_801a4180` | O | 804-byte mixer builder with GQR/`ldexpf` artifacts. |
| `0x801A48F0` | `FUN_801a48f0` | O | 1460-byte voice start/mix path with GQR artifacts. |
| `0x801A5280` | `FUN_801a5280` | O | 820-byte auxiliary mixer builder with GQR artifacts. |
| `0x801A5C10` | `FUN_801a5c10` | M | 632-byte 4-tap inner mixer; needs the argument pair hidden by `0x802DCD58`. |
| `0x801A6580` | `FUN_801a6580` | M | 536-byte 4-voice 4-tap mixer. |
| `0x801A6D30` | `FUN_801a6d30` | M | 876-byte 4-tap inner mixer loop. |
| `0x801A73E0` | `FUN_801a73e0` | O | 684-byte 2-buffer mixer driver with GQR artifacts. |
| `0x801A7A80` | `FUN_801a7a80` | M | 900-byte sequence mixer body. |
| `0x801A8150` | `FUN_801a8150` | O | 688-byte sequence mixer builder with GQR artifacts. |
| `0x801A93F0` | `FUN_801a93f0` | O | 900-byte 4-voice mixer with GQR artifacts. |
| `0x801AB5B0` | `FUN_801ab5b0` | R | Second timer-add wrapper; the 64-bit deadline split and record layout need a class definition. |
| `0x801ABCF0` | `FUN_801abcf0` | R | JAI block unlink; Ghidra drops one argument to `0x801ABB40`. |

The JAI timer primitives `0x801AB2F0` (insert), `0x801AB540` (set) and
`0x801AB640` (cancel) were translated by an earlier pass and live under
`runtime::os`; they are not duplicated here.

## Uncertainties

1. **Cost-pass return.** Ghidra prints `void` for `0x801A2B10`, but the last
   computed word survives in `r3` through the restore epilogue. The Rust
   function returns it and marks the fact medium.
2. **Allocator/free hooks.** The sequence start/free paths call the JAS heap
   hooks through `r13 - 0x61C0`/`r13 - 0x61BC`; the concrete functions are
   outside this band and are called opaquely through `wave5_alloc`/`wave5_free`.
3. **Float-to-int conversion.** The original calls `0x802DCC4C` with a double.
   The Rust uses a truncating `as i32`; NaN and out-of-range saturation are not
   bit-exact with the PowerPC conversion.
4. **Packed class/callback.** `0x801A0380` receives `pool` and `callback` in
   the two halves of the `0x802DCD74` save-register pair. The high word is the
   class (read back from object `+0x0C` by `fn_801a0270`); the low word lands
   at `+0x10` and is invoked as a completion callback. Caller `0x80192E60`
   passes `(0x1F, 0x80193E40, owner)`, which fixes both roles.
5. **Shared resampler body.** `0x801A8E10` and `0x801A9990` are byte-identical;
   both public functions call one private helper. The 64-bit multiplies are
   reproduced with `i64` high halves, but the surrounding loop is unchanged.
6. **Block record layout.** The JAI block record fields (`size`, `pointer`,
   `next`) are inferred from their writers/readers; the allocator's `+0x08` is
   only observed to be zero after a claim.
7. **Unnamed `r2` limits.** The constants behind `0x801A8C10` and
   `0x801A9780` are not named in the image. Their roles (pitch window and
   ratio scale) are inferred from the comparisons and the fields they produce.
8. **`0x801A6090` shift.** The export applies the shift after the C cast;
   the Rust uses `wrapping_shl(4)` on the `i32` result to preserve the 32-bit
   PowerPC shift.

## Ledger

Machine-readable rows: `ledger/agents/wave11-audio.csv` (module `audio::ax`,
22 rows, all `translated`). Earlier rows remain in
`ledger/agents/wave3-audio.csv`, `ledger/agents/wave4-audio.csv`,
`ledger/agents/wave5-audio.csv` and `ledger/agents/wave7-audio.csv`.
