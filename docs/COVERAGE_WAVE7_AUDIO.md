# Coverage - Wave 7 audio (AX/JAS/JAI) `0x801A18A0..0x801A8060`

## How to read this file

This is the status page for the wave-7 pass over the audio lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Selected functions*
   gives one row per function with address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
2. **Which JAS note-buffer families do the functions belong to?** - *Clusters*
   explains the JAS mode word, the base voice wrappers, the auxiliary 4-voice
   set, the 2-/3-buffer retry and size queries, and the 4-voice sequence set.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists
   the nearby addresses that were not selected, followed by *Uncertainties*
   and the ledger pointer.

Column meanings:

- **Status** - `translated` means the entry/exit, every call and every
  observable memory/MMIO write is accounted for. There are no `partial` or
  `stub` rows in this pass.
- **Confidence** - strength of the evidence, not code quality:
  `high` = the arithmetic or store sequence is explicit in the Ghidra C;
  `medium` = the flow is explicit but a callee identity, a float limit or a
  return convention is inferred; `low` = mostly guesswork.
- **Reason codes** (deferred inventory):
  - **S** - smaller than 24 bytes: SDA getter/setter or tiny accessor.
  - **M** - large mixer/note body reserved for a later pass.
  - **O** - opaque or unusable decompiler output (GQR/`ldexpf` artifacts).
  - **R** - audio-adjacent but not selected (size query or allocator).

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output), `// inferred:`
(role guessed from call sites), `// unknown:` (unresolved), `// partial:`
(deliberately incomplete).

**Promotion status.** All wave-7 functions live in `src/audio/ax.rs`. The
machine-readable ledger is `ledger/agents/wave7-audio.csv`. Wave 3 translated
the AI/DSP bring-up and the JAS frame pump (18 rows); wave 4 translated the JAS
object pools, the output ring, the parameter block and the BRSAR hook (23 rows);
wave 5 translated the voice parameter block, the 2-/3-/4-buffer sample sets and
the JAI timer queue (24 rows). Those rows are not repeated here.

## Range inventory

Source: `work/ghidra/main-gb/export/functions.tsv`, filtered by **entry**
address (`0x801A18A0 <= addr <= 0x801A8060`).

| Metric | Count |
|---|---:|
| Exported function entries in the searched band | 91 |
| Already translated before wave 7 (waves 3-5 + other lanes) | 39 |
| Translated by this wave | 25 |
| Deferred / not selected in this wave | 27 |

The band starts at the JAS mode word and ends at the free step of the 4-voice
sequence buffer set. Entries above `0x801A8060` (the sequence player, the
bank/voice classes and the JAI timer band) are handled by other passes; the
wave-5 rows already cover the JAI timer queue at `0x801AB9E0..0x801ABB40`.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x801A18A0` | `fn_801a18a0` | Write the JAS mode word. | L | translated | high | `801a18a0_FUN_801a18a0.c` |
| `0x801A18B0` | `fn_801a18b0` | Read the JAS mode word. | L | translated | high | `801a18b0_FUN_801a18b0.c` |
| `0x801A3420` | `fn_801a3420` | Stop one voice and return 1. | L | translated | high | `801a3420_FUN_801a3420.c` |
| `0x801A3460` | `fn_801a3460` | Byte size of the base voice buffer set. | L | translated | high | `801a3460_FUN_801a3460.c` |
| `0x801A3690` | `fn_801a3690` | Fast voice restart: free, then start. | L | translated | high | `801a3690_FUN_801a3690.c` |
| `0x801A3760` | `fn_801a3760` | Slow voice restart: clear, then configure. | L | translated | high | `801a3760_FUN_801a3760.c` |
| `0x801A44B0` | `fn_801a44b0` | Byte size of the auxiliary 4-voice set. | M | translated | high | `801a44b0_FUN_801a44b0.c` |
| `0x801A4540` | `fn_801a4540` | Start the auxiliary 4-voice note path. | M | translated | medium | `801a4540_FUN_801a4540.c` |
| `0x801A4700` | `fn_801a4700` | Auxiliary 4-voice restart. | M | translated | high | `801a4700_FUN_801a4700.c` |
| `0x801A47D0` | `fn_801a47d0` | Auxiliary 4-voice retry. | M | translated | high | `801a47d0_FUN_801a47d0.c` |
| `0x801A4890` | `fn_801a4890` | Auxiliary 4-voice abort. | M | translated | high | `801a4890_FUN_801a4890.c` |
| `0x801A4EB0` | `fn_801a4eb0` | Allocate the auxiliary 4-voice buffers. | M | translated | high | `801a4eb0_FUN_801a4eb0.c` |
| `0x801A5030` | `fn_801a5030` | Clear the auxiliary 4-voice buffers. | M | translated | high | `801a5030_FUN_801a5030.c` |
| `0x801A5160` | `fn_801a5160` | Free the auxiliary 4-voice buffers. | M | translated | high | `801a5160_FUN_801a5160.c` |
| `0x801A55C0` | `fn_801a55c0` | Byte size of the 3-buffer set. | N | translated | high | `801a55c0_FUN_801a55c0.c` |
| `0x801A5A50` | `fn_801a5a50` | Re-run the 3-buffer configuration. | N | translated | high | `801a5a50_FUN_801a5a50.c` |
| `0x801A6960` | `fn_801a6960` | Byte size of the 2-buffer set. | N | translated | high | `801a6960_FUN_801a6960.c` |
| `0x801A69C0` | `fn_801a69c0` | Start the 2-buffer voice path. | N | translated | medium | `801a69c0_FUN_801a69c0.c` |
| `0x801A76F0` | `fn_801a76f0` | Start the 4-voice sequence buffer set. | O | translated | medium | `801a76f0_FUN_801a76f0.c` |
| `0x801A7890` | `fn_801a7890` | Sequence-buffer restart. | O | translated | high | `801a7890_FUN_801a7890.c` |
| `0x801A7960` | `fn_801a7960` | Sequence-buffer retry. | O | translated | high | `801a7960_FUN_801a7960.c` |
| `0x801A7A20` | `fn_801a7a20` | Sequence-buffer abort. | O | translated | high | `801a7a20_FUN_801a7a20.c` |
| `0x801A7E10` | `fn_801a7e10` | Allocate the sequence buffers. | O | translated | high | `801a7e10_FUN_801a7e10.c` |
| `0x801A7F60` | `fn_801a7f60` | Clear the sequence buffers. | O | translated | high | `801a7f60_FUN_801a7f60.c` |
| `0x801A8060` | `fn_801a8060` | Free the sequence buffers. | O | translated | high | `801a8060_FUN_801a8060.c` |

Totals for this wave: **25 translated, 0 partial, 0 stubs**;
**22 high, 3 medium, 0 low**.

## Clusters

### Cluster L - JAS mode and base voice wrappers (`0x801A18A0..0x801A3760`)

The mode word at `r13 - 0x5118` gates the 4-voice note families: both
[`fn_801a4540`] and [`fn_801a76f0`] return 0 without touching the voice unless
the word is 2. The two wrappers [`fn_801a18a0`]/[`fn_801a18b0`] are the only
writers/readers translated here.

`fn_801a3420` is the thin stop wrapper: it forwards the incoming `r3` to
[`fn_801a3820`] and returns 1. `fn_801a3460` and `fn_801a44b0` are the two
size formulas used before allocation; they sum fixed unit lengths from
`0x8033145C..` and `0x803315FC..` and multiply once per voice count and once
per 4-byte word. The `r2 - 0x5148` / `r2 - 0x5108` scales are applied to the
voice pitch fields `+0x114` / `+0x144`.

[`fn_801a3690`] is the "fast" restart: it takes the interrupt lock, sets stop
bit 0, frees the sample buffers under a nested lock, and calls
[`fn_801a34f0`]. Success stores `(flags & ~1) | 2`; failure frees again under
a fresh lock and restores the outer state. [`fn_801a3760`] is the "slow"
restart: it clears the buffers with [`fn_801a3f30`] and calls the mixer builder
`0x801A4180` instead of reallocating.

### Cluster M - auxiliary 4-voice set (`0x801A44B0..0x801A5160`)

The auxiliary set is a 4-voice note body with one main buffer, an optional
second buffer, three tap buffers, two history buffers and one extra buffer:

| field | pointer base | length word |
|---|---|---|
| main | `+0x00 + voice*4` | `+0x20` |
| second | `+0x30 + voice*4` | `+0x48` |
| tap 0..2 | `+0x4C + voice*12 + tap*4` | `+0x94 + tap*4` |
| history 0..1 | `+0xAC + voice*8 + history*4` | `+0xDC + history*4` |
| extra | `+0xE4 + voice*4` | `+0x114 + voice*4` |

[`fn_801a4eb0`] allocates the table, [`fn_801a5030`] clears it and
[`fn_801a5160`] frees it. The start path [`fn_801a4540`] fills the length words
from `0x803315FC/0x80331738..0x80331758`, then runs allocate, clear and
`0x801A5280`. The three wrappers [`fn_801a4700`] (restart),
[`fn_801a47d0`] (retry without reallocating) and [`fn_801a4890`] (abort) all
set bit 0 at `+0x13C` and free through [`fn_801a5160`].

### Cluster N - 3-/2-buffer size and retry (`0x801A55C0..0x801A69C0`)

`fn_801a55c0` and `fn_801a6960` are the size queries for the wave-5 3-buffer
set (`+0x38`, scale `r2 - 0x50C8`) and the 2-buffer set (`+0xB8`, scale
`r2 - 0x5098`). `fn_801a5a50` is the 3-buffer retry: it calls
[`fn_801a5e90`] directly, and on failure frees `+0x00/+0x04/+0x08`.

[`fn_801a69c0`] is the 2-buffer start path driven by the wave-5 table helpers
[`fn_801a70a0`] (allocate), [`fn_801a71f0`] (clear) and [`fn_801a72f0`]
(free). It fills `+0x14/+0x30/+0x5C/+0x60/+0x94/+0x98` from
`0x8033177C/0x803317E0..0x803317EC` and calls the mixer builder `0x801A73E0`.
Unlike the auxiliary and sequence wrappers, its success path only clears bit 0
at `+0xB0`; it does not set bit 1.

### Cluster O - 4-voice sequence set (`0x801A76F0..0x801A8060`)

The sequence set is the 4-voice body used by the sequence player:

| field | pointer base | length word |
|---|---|---|
| main | `+0x00 + voice*4` | `+0x18` |
| second | `+0x20 + voice*4` | `+0x38` |
| tap 0..1 | `+0x3C + voice*8 + tap*4` | `+0x6C + tap*4` |
| history 0..1 | `+0x7C + voice*8 + history*4` | `+0xAC + history*4` |

[`fn_801a7e10`] allocates it, [`fn_801a7f60`] clears it and
[`fn_801a8060`] frees it. [`fn_801a76f0`] is the start gate; it fills
`+0x18/+0x38/+0x6C/+0x70/+0xAC/+0xB0` from
`0x8033180C/0x80331870..0x8033187C` and calls `0x801A8150`.
[`fn_801a7890`], [`fn_801a7960`] and [`fn_801a7a20`] are the restart, retry and
abort wrappers around the same flag word at `+0xCC`.

## Deferred inventory

| Address | Symbol in export | Reason | Note |
|---|---|---|---|
| `0x801A18C0..0x801A18E0` | `FUN_801a18c0`/`d0`/`e0` | S | 8-byte getters for the three `r13 - 0x512C..0x5130` shorts. |
| `0x801A1910..0x801A1930` | `FUN_801a1910`/`20`/`30` | S | Matching 8-byte setters. |
| `0x801A2180` | `FUN_801a2180` | S | 12-byte getter for the parameter block `0x8037CDA0`. |
| `0x801A25B0` | `FUN_801a25b0` | S | 8-byte getter for `r13 - 0x5068`. |
| `0x801A2E20` | `FUN_801a2e20` | S | 8-byte getter for `r13 - 0x5078`. |
| `0x801A3350`/`0x801A3360` | `FUN_801a3350`/`60` | S | 8-byte setter/getter for `r13 - 0x5060`/`-0x506C`. |
| `0x801A25C0` | `FUN_801a25c0` | M | 1360-byte output-parameter block apply. |
| `0x801A2B10` | `FUN_801a2b10` | M | 784-byte per-frame voice update. |
| `0x801A2E70` | `FUN_801a2e70` | M | 700-byte voice-table initialiser. |
| `0x801A3290` | `FUN_801a3290` | O | Ghidra emits GQR/`ldexpf` artifacts instead of the float sequence. |
| `0x801A3880` | `FUN_801a3880` | M | 1316-byte voice/parameter apply. |
| `0x801A3DB0` | `FUN_801a3db0` | M | 372-byte base sample-buffer allocator. |
| `0x801A4180` | `FUN_801a4180` | M | 804-byte mixer-state builder. |
| `0x801A48F0` | `FUN_801a48f0` | M | 1460-byte voice start/mix path. |
| `0x801A5280` | `FUN_801a5280` | M | 820-byte auxiliary mixer builder called by [`fn_801a4540`]. |
| `0x801A5C10` | `FUN_801a5c10` | M | 632-byte 4-tap inner mixer loop. |
| `0x801A6090` | `FUN_801a6090` | R | 40-byte 4-buffer size query (`+0x40`, scale `r2 - 0x50B0`). |
| `0x801A6580` | `FUN_801a6580` | M | 536-byte 4-voice 4-tap mixer loop. |
| `0x801A6D30` | `FUN_801a6d30` | M | 876-byte 4-tap inner mixer loop. |
| `0x801A73E0` | `FUN_801a73e0` | M | 684-byte 2-buffer mixer driver. |
| `0x801A7690` | `FUN_801a7690` | R | 96-byte sequence size query (`+0xD4`, scale `r2 - 0x5058`). |
| `0x801A7A80` | `FUN_801a7a80` | M | 900-byte sequence mixer body. |

The remaining 39 already-translated entries in the band belong to waves 3-5
and to the wave-4 JAS pool/ring work; they are recorded in their own ledgers.

## Uncertainties

1. **Register-save thunks.** `0x802DCD64`, `0x802DCD68`, `0x802DCD6C` and
   `0x802DCD70` are CodeWarrior save-register prologues. Their apparent return
   value is the incoming `r3`, so every translated allocate/clear/free function
   takes the table pointer explicitly. The matching
   `0x802DCDB0..0x802DCDBC` restore thunks pass the result word in `r3`.
2. **Return-through-epilogue.** Ghidra prints `void` for `0x801A4EB0`,
   `0x801A5030`, `0x801A5160`, `0x801A5280`, `0x801A7E10`, `0x801A7F60` and
   `0x801A8150`, but the epilogues restore a value in `r3`; the Rust callees
   return `u32` where the value is tested by the caller.
3. **Float fields.** The start gates compare the fields at `+0x114`, `+0x144`,
   `+0xB8` and `+0xD4` as single-precision floats, matching the `lfs` loads in
   the Ghidra C. The shared 3-buffer `+0x38`/`+0x3C` words are also printed as
   floats; wave 5 treated the same words as integer values converted to float,
   so the fixed-point representation of those two fields remains unresolved.
4. **Unnamed limits and units.** The constants at `r2 - 0x5104`, `-0x5108`,
   `-0x5094`, `-0x5098`, `-0x5054` and `-0x5058` are not named in the image.
   Their roles are inferred from the surrounding comparisons. The
   `0x80331xxx` unit lengths are likewise unnamed.
5. **Mixed failure paths.** In [`fn_801a4540`], [`fn_801a69c0`] and
   [`fn_801a76f0`] the pitch-reject, allocation-failure and configuration-
   failure branches run the identical flag-set/free/restore sequence; the Rust
   versions share one cleanup block after the success test. The observable
   order of stores and calls is unchanged.
6. **Partial allocations.** [`fn_801a4eb0`] and [`fn_801a7e10`] stop on the
   first zero allocation and leave earlier buffers attached to the table. The
   caller's free step is therefore the only owner of that cleanup, exactly as
   in the original.
7. **Integer wrap.** The size formulas use explicit wrapping arithmetic to
   match the 32-bit PowerPC multiplies; the Rust results are signed `i32`.
8. **Success flags.** The auxiliary and sequence wrappers set
   `(flags & ~1) | 2`, while [`fn_801a69c0`] only clears bit 0. The difference
   is observed at all call sites and is preserved.

## Ledger

Machine-readable rows: `ledger/agents/wave7-audio.csv` (module `audio::ax`,
25 rows, all `translated`). Earlier rows remain in
`ledger/agents/wave3-audio.csv`, `ledger/agents/wave4-audio.csv` and
`ledger/agents/wave5-audio.csv`.
