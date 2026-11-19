# Coverage - Wave 5 audio (AX/JAS/JAI) `0x801A31A0..0x801ABB40`

## How to read this file

This is the status page for the wave-5 pass over the audio lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Selected functions*
   gives one row per function with address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
2. **Which parts of the JAS/JAI driver do the functions belong to?** -
   *Clusters* explains the voice parameter/envelope setup, the sample-buffer
   tables, the three note buffer layouts, and the JAI timer list.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists
   the nearby addresses that were not selected, followed by *Uncertainties*
   and the ledger pointer.

Column meanings:

- **Status** - `translated` means the entry/exit, every call and every
  observable memory/MMIO write is accounted for. There are no `partial` or
  `stub` rows in this pass.
- **Confidence** - strength of the evidence, not code quality:
  `high` = the arithmetic or store sequence is explicit in the Ghidra C;
  `medium` = the flow is explicit but a callee identity, a float limit, a
  field role or a return convention is inferred; `low` = mostly guesswork.
- **Reason codes** (deferred inventory):
  - **M** - large mixer/channel/voice code reserved for a later pass.
  - **O** - opaque helper (trampoline, register save/restore thunk, DVD
    reset loop).
  - **R** - audio-adjacent but not selected (needs a class layout, a 64-bit
    argument, or a caller that is not reconstructed yet).
  - **S** - fewer than 16 bytes: veneer, thunk or shared-`blr` helper.
  - **X** - not audio (OS exception or date code that shares the band).

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output), `// inferred:`
(role guessed from call sites), `// unknown:` (unresolved), `// partial:`
(deliberately incomplete).

**Promotion status.** All wave-5 functions live in `src/audio/ax.rs`. The
machine-readable ledger is `ledger/agents/wave5-audio.csv`. Wave 3 translated
the AI/DSP bring-up and the JAS frame pump (18 rows); wave 4 translated the JAS
object pools, the output ring, the parameter block and the BRSAR hook (23 rows).
Those rows are not repeated here.

## Range inventory

Source: `work/ghidra/main-gb/export/functions.tsv`, filtered by **entry**
address (`0x801A3000 <= addr < 0x801AC000`).

| Metric | Count |
|---|---:|
| Exported function entries in the searched band | 148 |
| Already translated before wave 5 (wave 4 + other lanes) | 18 |
| Translated by this wave | 24 |
| Deferred / not selected in this wave | 105 |

The searched band also contains OS exception handlers and date helpers
(`0x801A9B20..0x801AAD60`), the DVD interface reset loop at `0x801ABF60` and
the OS exception-table accessors at `0x801AB020..0x801AB040`. Those entries are
not part of the JAS/JAI audio driver and were not selected.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x801A31A0` | `fn_801a31a0` | Store a voice's 4-word parameter block and reset two waveform accumulator groups. | F | translated | high | `801a31a0_FUN_801a31a0.c` |
| `0x801A3370` | `fn_801a3370` | Hand out the next 0x38-byte voice slot from a round-robin ring. | F | translated | high | `801a3370_FUN_801a3370.c` |
| `0x801A33B0` | `fn_801a33b0` | Initialise the voice envelope fields and tail-call the start path. | F | translated | high | `801a33b0_FUN_801a33b0.c` |
| `0x801A34F0` | `fn_801a34f0` | Start one voice: pitch limit, sample count, mix constants, buffer allocation. | F | translated | medium | `801a34f0_FUN_801a34f0.c` |
| `0x801A3F30` | `fn_801a3f30` | Clear every sample buffer referenced by a voice buffer table. | G | translated | high | `801a3f30_FUN_801a3f30.c` |
| `0x801A4060` | `fn_801a4060` | Free every sample buffer referenced by a voice buffer table. | G | translated | high | `801a4060_FUN_801a4060.c` |
| `0x801A55F0` | `fn_801a55f0` | Allocate the three buffers of the 3-buffer sample set and start it. | H | translated | medium | `801a55f0_FUN_801a55f0.c` |
| `0x801A58C0` | `fn_801a58c0` | Restart the 3-buffer sample set: free, then call the allocator path. | H | translated | high | `801a58c0_FUN_801a58c0.c` |
| `0x801A5B60` | `fn_801a5b60` | Free the three buffers of the 3-buffer sample set. | H | translated | high | `801a5b60_FUN_801a5b60.c` |
| `0x801A5E90` | `fn_801a5e90` | Validate and configure the 3-buffer sample set. | H | translated | medium | `801a5e90_FUN_801a5e90.c` |
| `0x801A60C0` | `fn_801a60c0` | Allocate the four buffers of the 4-buffer sample set and start it. | I | translated | medium | `801a60c0_FUN_801a60c0.c` |
| `0x801A62F0` | `fn_801a62f0` | Restart the 4-buffer sample set. | I | translated | high | `801a62f0_FUN_801a62f0.c` |
| `0x801A6420` | `fn_801a6420` | Retry the 4-buffer configuration without reallocating. | I | translated | high | `801a6420_FUN_801a6420.c` |
| `0x801A64F0` | `fn_801a64f0` | Free the four buffers of the 4-buffer sample set. | I | translated | high | `801a64f0_FUN_801a64f0.c` |
| `0x801A67A0` | `fn_801a67a0` | Validate and configure the 4-buffer sample set. | I | translated | medium | `801a67a0_FUN_801a67a0.c` |
| `0x801A6B40` | `fn_801a6b40` | Start the 2-buffer voice after clearing its buffer table. | J | translated | high | `801a6b40_FUN_801a6b40.c` |
| `0x801A6C10` | `fn_801a6c10` | Alternate 2-buffer start path. | J | translated | high | `801a6c10_FUN_801a6c10.c` |
| `0x801A6CD0` | `fn_801a6cd0` | Abort the 2-buffer voice and free its table. | J | translated | high | `801a6cd0_FUN_801a6cd0.c` |
| `0x801A70A0` | `fn_801a70a0` | Allocate the 3-voice buffer table used by the 2-buffer path. | J | translated | high | `801a70a0_FUN_801a70a0.c` |
| `0x801A71F0` | `fn_801a71f0` | Clear the 3-voice buffer table without freeing it. | J | translated | high | `801a71f0_FUN_801a71f0.c` |
| `0x801A72F0` | `fn_801a72f0` | Free the 3-voice buffer table and zero every pointer. | J | translated | high | `801a72f0_FUN_801a72f0.c` |
| `0x801AB9E0` | `fn_801ab9e0` | Expire every JAI timer whose predicate reports it finished. | K | translated | high | `801ab9e0_FUN_801ab9e0.c` |
| `0x801ABA90` | `fn_801aba90` | Cancel JAI timers by owner id. | K | translated | high | `801aba90_FUN_801aba90.c` |
| `0x801ABB40` | `fn_801abb40` | Insert and coalesce a block in the JAI free list. | K | translated | high | `801abb40_FUN_801abb40.c` |

Totals for this wave: **24 translated, 0 partial, 0 stubs**;
**19 high, 5 medium, 0 low**.

`0x801AB640` is the shared OS alarm unlinker
(`runtime::os::cancel_alarm`). It is recorded once, under `runtime::os`; this
lane calls it instead of duplicating the body.

## Clusters

### Cluster F - voice parameter and envelope control (`0x801A31A0..0x801A34F0`)

`fn_801a3370` owns the round-robin voice ring at `r13 - 0x504C`: each slot is
0x38 bytes, the count is `r13 - 0x5050`, the cursor is `r13 - 0x5054`, and the
enable flag is `r13 - 0x5058`. The slot is returned before the cursor advances,
so the first call returns slot 0.

`fn_801a31a0` copies four caller words into the unaligned voice fields
`+0x96..+0xA2`, then uses the 16-bit tag at source `+2`:

- tag `10` clears `+0xA6..+0xC2`, writes `0x08000000` to `+0xC6` and zero to
  `+0xCA`;
- tag `0x19` does the same but writes `0x01000000`;
- other tags only reach the final flag rewrite `(... & 0xFFFF87FF) | 0x8400`.

`fn_801a33b0` fills the envelope fields from the voice's own `+0x148..+0x15C`
block, writes state 5 at `+0x110`, and copies the two global words at
`r2 - 0x5150`/`r2 - 0x514C` into `+0x130/+0x134/+0x144`. It then tail-calls
`0x801A34F0`.

`fn_801a34f0` is the start gate. It takes the interrupt lock, sets `+0x10C = 1`,
checks the global lower limit at `r2 - 0x5144`, converts
`r2 - 0x5148 * +0x114` into the sample count at `+0x40`, and copies the raw
constant words at `0x8033145C`/`0x80331580..0x8033159C` into the voice. It calls
the buffer allocator `0x801A3DB0`, clears the buffer tables through
`fn_801a3f30`, and calls the mixer-state builder `0x801A4180`. Any failure sets
bit 0 and frees through `0x801A4060`.

### Cluster G - voice buffer table clear/free (`0x801A3F30`, `0x801A4060`)

Both functions walk the same table layout:

| field | stride | length source |
|---|---|---|
| main buffer `+0x00` | 4 bytes per voice | `+0x1C` |
| optional second buffer `+0x2C` | 4 bytes per voice | `+0x40` |
| three tap buffers `+0x44` | 0x0C bytes per voice | `+0x80 + tap*4` |
| two history buffers `+0x98` | 8 bytes per voice | `+0xC0 + history*4` |
| extra buffer `+0xC8` | 4 bytes per voice | `+0xEC + voice*4` |

[`fn_801a3f30`] clears each non-null range with the memset-shaped helper
`0x800046B4`; [`fn_801a4060`] calls the JAS free hook and zeroes the pointer.
The two functions are the clear/free halves of the allocation performed by
`0x801A3DB0`.

### Cluster H - 3-buffer sample set (`0x801A55F0..0x801A5E90`)

The 3-buffer set uses three sample buffers at `+0x00/+0x04/+0x08`, a word count
at `+0x14`, a flag word at `+0x34`, and the pitch/limit words at
`+0x38..+0x54`.

- [`fn_801a55f0`] checks `r2 - 0x50C4 < +0x38`, computes
  `r2 - 0x50C8 * +0x38` into `+0x14`, allocates three `len*4` buffers, and calls
  [`fn_801a5e90`]. On success it stores `(flags & ~1) | 2`.
- [`fn_801a58c0`] frees the three buffers under nested interrupt locks and calls
  [`fn_801a55f0`] again.
- [`fn_801a5b60`] is the plain free path.
- [`fn_801a5e90`] validates `+0x3C <= +0x38`, the float window
  `r2 - 0x50C4 .. r2 - 0x50C0` for `+0x40/+0x44/+0x50/+0x54`, and the three
  buffer pointers, then clears the buffers and writes the six step words.

### Cluster I - 4-buffer sample set (`0x801A60C0..0x801A67A0`)

The 4-buffer set adds a fourth buffer at `+0x0C`, moves the flag word to
`+0x3C`, the count to `+0x18`, and the pitch/limit words to
`+0x40..+0x5C`. [`fn_801a60c0`] only runs when the JAS mode getter
`0x801A18B0` returns 2; it uses the window `r2 - 0x50AC .. r2 - 0x50A8`.
[`fn_801a67a0`] clears four buffers and fills eight step words. The restart,
retry and free wrappers are [`fn_801a62f0`], [`fn_801a6420`] and
[`fn_801a64f0`].

### Cluster J - 2-buffer path and 3-voice table (`0x801A6B40..0x801A72F0`)

`0x801A69C0` (deferred) drives this path. The state word is at `+0xB0`.
[`fn_801a6b40`] and [`fn_801a6c10`] are the two start wrappers;
[`fn_801a6cd0`] is the abort. The table helpers allocate, clear and free three
voices with six pointers each: main (`+0x00`), optional second (`+0x1C`), two
tap buffers (`+0x34/+0x38`) and two history buffers (`+0x6C/+0x70`). The
lengths come from `+0x14`, `+0x30`, `+0x5C..+0x60` and `+0x94..+0x98`.

### Cluster K - JAI timer queue and block free list (`0x801AB9E0..0x801ABB40`)

The timer list head/tail are `r13 - 0x5000`/`r13 - 0x4FFC`. A node uses:

| offset | meaning |
|---|---|
| `+0x00` | owner id; 0 means unlinked |
| `+0x04` | signed state word; -1 is cancellable |
| `+0x08/+0x0C` | 64-bit deadline, high word first |
| `+0x10` | previous link |
| `+0x14` | next link |
| `+0x28` | subsystem id used by [`fn_801aba90`] |

[`crate::runtime::os::cancel_alarm`] unlinks a node, updates the head/tail, and
re-arms `0x801A9B80` with `clamp(expiry - now, 0, 0x7FFFFFFF)`.
[`fn_801ab9e0`] walks the list and removes nodes for which `0x801CFA30`
returns 0. [`fn_801aba90`] removes nodes with `+0x04 == -1` and
`+0x28 == owner`. [`fn_801abb40`] is the address-ordered block free list:
`+0x00` previous, `+0x04` next, `+0x08` size; it coalesces the following block
first and then the preceding block.

## Deferred inventory

| Address | Symbol in export | Reason | Note |
|---|---|---|---|
| `0x801A2E70` | `FUN_801a2e70` | M | 700-byte voice-table initialiser; Ghidra drops the argument to `0x801A0240`. |
| `0x801A3880` | `FUN_801a3880` | M | 1316-byte voice/parameter apply. |
| `0x801A4180` | `FUN_801a4180` | M | 804-byte mixer state builder called by [`fn_801a34f0`]. |
| `0x801A48F0` | `FUN_801a48f0` | M | 1460-byte voice start/mix path. |
| `0x801A5C10` | `FUN_801a5c10` | M | 632-byte 4-tap inner mixer loop. |
| `0x801A6D30` | `FUN_801a6d30` | M | 876-byte 4-tap inner mixer loop. |
| `0x801A73E0` | `FUN_801a73e0` | M | 684-byte 2-buffer mixer-state builder. |
| `0x801AB2F0` | `FUN_801ab2f0` | M | 592-byte sorted timer insertion with 64-bit time arithmetic. |
| `0x801AB540` | `FUN_801ab540` | R | Timer add wrapper; the 64-bit delay argument is split across registers. |
| `0x801AB5B0` | `FUN_801ab5b0` | R | Timer add wrapper with two 64-bit arguments. |
| `0x801ABF60` | `FUN_801abf60` | O | DVD interface reset/wait loop; not audio. |
| `0x801A9B20..0x801AAD60` | multiple | X | OS exception/date code that shares the band. |
| `0x801AB020..0x801AB040` | `FUN_801ab020`/`f040` | X | OS exception-table accessors. |

The remaining unselected entries in the band are small veneers (reason S),
large mixer/voice routines (reason M), or OS/text helpers that are not audio
(reason X).

## Uncertainties

1. **Unnamed `r2` limits.** The float constants at `r2 - 0x5144/-0x5148`,
   `-0x50C4/-0x50C8/-0x50C0`, `-0x50BC/-0x50B8`, `-0x50AC/-0x50B0/-0x50A8`
   and `-0x50A4/-0x50A0` are not named in the image. Their roles are inferred
   from the comparisons and the fields they produce.
2. **Allocator and free hooks.** `r13 - 0x61C0` and `r13 - 0x61BC` are function
   pointers to the JAS heap allocator/free pair. The concrete functions are
   outside this band and are called opaquely.
3. **Float-to-int helper.** The original calls `0x802DCC4C` with a double and
   receives an integer. The Rust helper uses a truncating `as i32`; NaN and
   out-of-range saturation are not bit-exact with the PowerPC conversion.
4. **Register-save thunks.** `0x802DCD68`, `0x802DCD6C`, `0x802DCD70` and
   `0x802DCD74` are CodeWarrior save-register prologues, not getters. Their
   apparent return value is the incoming `r3`, so the affected Rust functions
   take the voice/table pointer explicitly. The matching restore thunks are
   omitted.
5. **Return-through-epilogue.** Ghidra prints `void` for `0x801A3F30`,
   `0x801A60C0`, `0x801A62F0` and `0x801A6420`, but their register-restore
   epilogues pass a result in `r3`. Caller `0x8026B900` tests the result, so
   the Rust signatures return `u32`.
6. **Timer fields.** The `+0x10`/`+0x14` links and the `+0x08/+0x0C` deadline
   order are inferred from the link surgery and the 64-bit subtract. The tick
   unit of `0x801B5FB0` is not named.
7. **Timer clamp.** [`crate::runtime::os::cancel_alarm`] reproduces the original
   `0x801A9B80(0 | low32 | 0x7FFFFFFF)` selection. `0x801A9B80` has an empty
   body in the export, so only the argument selection is verified.
8. **Free-hook arguments.** The free calls in [`fn_801a4060`],
   [`fn_801a72f0`] and the 3/4-buffer cleanup helpers have no visible argument
   in the Ghidra C. Each call passes the pointer of the surrounding null test;
   the pairing with the allocator makes this the only consistent reading.
9. **`0x800046B4`.** The helper is memset-shaped. The wave-5 call sites show
   three arguments (pointer, 0, byte length), which is what the Rust uses.
10. **Ring count.** [`fn_801a3370`] divides by `r13 - 0x5050`; a zero count
    traps in the original and panics in Rust. The initialiser is expected to
    install a non-zero count before the ring is enabled.

## Ledger

Machine-readable rows: `ledger/agents/wave5-audio.csv` (module `audio::ax`,
25 rows, all `translated`). Earlier rows remain in
`ledger/agents/wave3-audio.csv` and `ledger/agents/wave4-audio.csv`.
