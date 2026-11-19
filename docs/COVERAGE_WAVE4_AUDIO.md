# Coverage - Wave 4 audio (AX/JAS/BRSAR) `0x8019B660`, `0x801A0050..0x801A3820`

## How to read this file

This is the status page for the wave-4 pass over the audio lane of the
`main.dol` reconstruction. It answers three questions:

1. **What did this pass translate, and where is it?** - *Selected functions*
   gives one row per function with address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
2. **Which parts of the audio driver do the functions belong to?** - *Clusters*
   explains the JAS object pools, the ring/parameter path, the DSP task
   descriptor and the game sound/BRSAR hook.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists
   the nearby addresses that were not selected, followed by *Uncertainties*
   and the ledger pointer.

Column meanings:

- **Status** - `translated` means the entry/exit, every call and every
  observable memory/MMIO write is accounted for. There are no `partial` or
  `stub` rows in this pass.
- **Confidence** - strength of the evidence, not code quality:
  `high` = the arithmetic or store sequence is explicit in the Ghidra C;
  `medium` = the flow is explicit but a callee identity, a field role or a
  data-structure meaning is inferred; `low` = mostly guesswork.
- **Reason codes** (deferred inventory):
  - **A** - archive/text code, not audio.
  - **M** - large JAS mixer/channel/voice code reserved for a later pass.
  - **O** - opaque helper (trampoline, DSP upload, interrupt primitive).
  - **S** - fewer than 16 bytes: veneer, thunk or shared-`blr` helper.
  - **R** - audio-adjacent but not selected (needs a callee layout that is not
    reconstructed yet, or the Ghidra output loses an argument/return).

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output), `// inferred:`
(role guessed from call sites), `// unknown:` (unresolved), `// partial:`
(deliberately incomplete).

**Promotion status.** All wave-4 functions live in `src/audio/ax.rs`. The
machine-readable ledger is `ledger/agents/wave4-audio.csv`. Wave 3 already
translated the AI/DSP bring-up and the JAS frame pump
(`docs/COVERAGE_WAVE3_AUDIO.md`, 18 rows); those rows are not repeated here.

## Range inventory

Source: `work/ghidra/main-gb/export/functions.tsv`, filtered by **entry**
address (`0x80180000 <= addr < 0x801C0000`).

| Metric | Count |
|---|---:|
| Exported function entries in the searched band | 872 |
| Already translated before wave 4 (wave 3 + four DMA accessors) | 22 |
| Translated by this wave | 23 |
| Deferred / not selected in this wave | 827 |

The 872-entry figure covers a wide band that also contains text/font code,
JKRArchive/ARC code and unrelated engine helpers. The wave-4 selection is
deliberately limited to entries whose Ghidra C shows audio state: the JAS
free lists, the 16-bit output ring, the parameter block at `0x8037CDA0`, the
DSP task descriptor at `0x8037CD00`, and the game sound/BRSAR transition.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x8019B660` | `fn_8019b660` | Game sound state transition; loads `mj2d_sound.brsar` and edits BRSAR groups. | E | translated | medium | `8019b660_FUN_8019b660.c` |
| `0x801A0050` | `fn_801a0050` | One-shot JAS initialisation chain. | A | translated | high | `801a0050_FUN_801a0050.c` |
| `0x801A00E0` | `fn_801a00e0` | Free every object on the completed list. | A | translated | medium | `801a00e0_FUN_801a00e0.c` |
| `0x801A0190` | `fn_801a0190` | Clear both free-list arrays. | A | translated | high | `801a0190_FUN_801a0190.c` |
| `0x801A0240` | `fn_801a0240` | Push an object onto pool 0. | A | translated | high | `801a0240_FUN_801a0240.c` |
| `0x801A0260` | `fn_801a0260` | Push an object onto the completed list. | A | translated | high | `801a0260_FUN_801a0260.c` |
| `0x801A0270` | `fn_801a0270` | Unlink an object from its pool. | A | translated | high | `801a0270_FUN_801a0270.c` |
| `0x801A0300` | `fn_801a0300` | Release an object back to pool 0. | A | translated | high | `801a0300_FUN_801a0300.c` |
| `0x801A0510` | `fn_801a0510` | Link an object into a named pool. | A | translated | high | `801a0510_FUN_801a0510.c` |
| `0x801A05B0` | `fn_801a05b0` | Clear the three pool stores and callback slots. | A | translated | high | `801a05b0_FUN_801a05b0.c` |
| `0x801A1CE0` | `fn_801a1ce0` | Build and start the DSP task descriptor. | B | translated | medium | `801a1ce0_FUN_801a1ce0.c` |
| `0x801A1FD0` | `fn_801a1fd0` | Swap the per-frame callback; return the old one. | C | translated | high | `801a1fd0_FUN_801a1fd0.c` |
| `0x801A2020` | `fn_801a2020` | Readable ring-slot count. | C | translated | high | `801a2020_FUN_801a2020.c` |
| `0x801A2050` | `fn_801a2050` | Copy ring samples and invalidate the data cache. | C | translated | high | `801a2050_FUN_801a2050.c` |
| `0x801A2120` | `fn_801a2120` | Advance the ring consumer. | C | translated | high | `801a2120_FUN_801a2120.c` |
| `0x801A2190` | `fn_801a2190` | Split a 0x60-scale accumulator into value and step. | D | translated | high | `801a2190_FUN_801a2190.c` |
| `0x801A2200` | `fn_801a2200` | Split a 0x12-scale accumulator into value and step. | D | translated | high | `801a2200_FUN_801a2200.c` |
| `0x801A2270` | `fn_801a2270` | Publish the 20 parameter pairs and flush 0x78 bytes. | D | translated | high | `801a2270_FUN_801a2270.c` |
| `0x801A23F0` | `fn_801a23f0` | Clear the 20 accumulators. | D | translated | high | `801a23f0_FUN_801a23f0.c` |
| `0x801A2450` | `fn_801a2450` | Accumulate a voice's output parameters. | D | translated | high | `801a2450_FUN_801a2450.c` |
| `0x801A2E30` | `fn_801a2e30` | Reset one voice/parameter block. | D | translated | high | `801a2e30_FUN_801a2e30.c` |
| `0x801A3130` | `fn_801a3130` | Set one 16-bit voice parameter. | D | translated | high | `801a3130_FUN_801a3130.c` |
| `0x801A3820` | `fn_801a3820` | Stop one AX voice. | D | translated | high | `801a3820_FUN_801a3820.c` |

Totals for this wave: **23 translated, 0 partial, 0 stubs**;
**20 high, 3 medium, 0 low**.

## Clusters

### Cluster A - JAS object pools (`0x801A0050..0x801A05B0`)

JAS allocates voice objects from three pools. The pool arrays are in `.bss`:

- `0x80378C10` - 32-word head array;
- `0x80378C90` - 32-word tail array;
- `0x80378D20`, `0x80379F20`, `0x8037B120` - three 0x480-byte backing stores.

Each object uses `+0x00` as `next`, `+0x04` as `previous`, `+0x08` as the
"completed" list link, `+0x0C` as the pool index, and `+0x10` as an optional
destructor. [`fn_801a0050`] is the public one-shot entry point: it calls the
version helper, [`fn_801a0190`], the voice-table initialiser `0x801A2E70`,
[`fn_801a23f0`], [`fn_801a05b0`], [`fn_801a1850`], and finally
[`fn_801a1dd0`] with mode 0.

`fn_801a00e0` drains the completed list at `r13 - 0x5180`, running the
destructor and returning each object to pool 0. `fn_801a0300` releases one
object; `fn_801a0510` inserts one into an explicit pool. Both bracket the list
surgery with the interrupt primitives `0x801B1140`/`0x801B1180`.

`fn_801a05b0` clears the pools and also clears the three JAS callback slots
`r13 - 0x5140/-0x5144/-0x5148` and their arguments `-0x514C/-0x5150/-0x5154`.
Those are the same slots exposed by the wave-3 setters at
`0x801A0CA0..0x801A0D80`.

### Cluster B - DSP task descriptor (`0x801A1CE0`)

`0x8037CD00` is the DSP task object used by the wave-3 frame pump. This pass
translates the setup that fills it:

| offset | value |
|---|---|
| `+0x04` | 0 |
| `+0x0C` | `0x8032F400` (DSP code image) |
| `+0x10` | `*(u16 *)(r13 - 0x61C4)` |
| `+0x14` | 0 |
| `+0x18` | `0x8037CD60` (0x40-byte work buffer) |
| `+0x1C` | `0x40` |
| `+0x20` | `0xCD2` |
| `+0x24` | `*(u16 *)(r13 - 0x61C8)` |
| `+0x26` | `*(u16 *)(r13 - 0x61C6)` |
| `+0x28` | `0x801A1C50` (code label) |
| `+0x2C` | [`fn_801a1c60`] (frame-boundary callback) |
| `+0x30` | `0x801A1CC0` (code label) |
| `+0x34` | `0x801A1CD0` (data label) |

The function also initialises the task lock at `r13 - 0x50F0`, resets the DSP
queue when `0x801D5800` says a task is already queued, queues the descriptor
through `0x801D5810`, and spins on the latch at `r13 - 0x50E0`.

### Cluster C - JAS output ring (`0x801A1FD0..0x801A2120`)

The ring is four 0x168-byte buffers starting at `0x8037BFE0`. The producer
cursor is `r13 - 0x5100`, the consumer cursor `r13 - 0x5104`, and the length
`r13 - 0x50FC` (0xB4 slots, installed by [`fn_801a1dd0`]). The producer keeps
a 0x24-slot lead so the mixer can fill ahead of the reader.

- [`fn_801a2020`] returns how many slots can be read.
- [`fn_801a2050`] copies those 16-bit samples into a caller buffer, wraps at
  the end, and invalidates the cache line range for the source buffer with
  `0x801AC440`.
- [`fn_801a2120`] advances the consumer cursor by a clamped count.
- [`fn_801a1fd0`] replaces the per-frame callback at `r13 - 0x50DC` that
  [`fn_801a1940`] invokes.

### Cluster D - parameter accumulators and voice control (`0x801A2190..0x801A3820`)

Twenty `i32` accumulators occupy `r13 - 0x50C8..-0x507C`. They are:

- cleared by [`fn_801a23f0`];
- fed from a voice block by [`fn_801a2450`] (twelve 0x60-scale parameters
  from `+0x52..+0x68`, eight 0x12-scale parameters from `+0xFA..+0x108`);
- split into a whole value and a step by [`fn_801a2190`] (divisor 0x60) and
  [`fn_801a2200`] (divisor 0x12), with the unit count clamped to +/-20;
- published by [`fn_801a2270`] into twenty 4-byte-value/2-byte-step pairs at
  `0x8037CDA0..0x8037CE17`, followed by a 0x78-byte data-cache flush.

[`fn_801a2e30`] resets a voice/parameter block (the same body used by the
deferred voice-table initialiser `0x801A2E70`). [`fn_801a3130`] writes the
16-bit parameter at `+0x38` and sets dirty bit 2. [`fn_801a3820`] stops one AX
voice by setting bit 0 at `+0x10C` and calling the stop helper `0x801A4060`.

### Cluster E - game sound and BRSAR (`0x8019B660`)

`fn_8019b660` is the game-level sound state machine, one layer above JAS. On
state 0 it loads the BRSAR named by the Ghidra string at `0x8032E12C`
(`Sound/wii/mj2d_sound.brsar`) through the sound object's vtable slot `+0xA4`,
then loads the bank group at object `+0x5BC`. On a later state it walks the
loaded BRSAR's group table (`+0x3C` count), skipping groups 0, 0x10..0x12 and
0x15, sets the remaining groups through `0x8027A7D0`, applies the state-6/5
fades, and dispatches the new state through `0x8019E080`.

## Deferred inventory

| Address | Symbol in export | Reason | Note |
|---|---|---|---|
| `0x8019F010` | `FUN_8019f010` | R | Float interpolation helper; exact single/double conversion sequence not recovered in Rust. |
| `0x8019F050` | `FUN_8019f050` | R | Duplicate of `0x8019F010`. |
| `0x8019F090` | `FUN_8019f090` | R | `0x1D4C00 / n` timing helper; trivial but not selected. |
| `0x8019F490` | `FUN_8019f490` | O | Opaque DSP microcode upload; Ghidra renders an infinite loop. |
| `0x8019F660..0x8019F9FF` | `FUN_8019f660`/`f700`/`f9b0` | A | JKRArchive/ARC reader, magic `0x55AA382D`; not audio. |
| `0x801A00C0` | `FUN_801a00c0` | S | 20-byte pool-head getter. |
| `0x801A0380` | `FUN_801a0380` | R | Allocator fallback; `0x802DCD74` returns an indeterminate value in the export, so the pool/timestamp split is unresolved. |
| `0x801A0E90` | `FUN_801a0e90` | M | 2492-byte JAS mixer. |
| `0x801A25C0` | `FUN_801a25c0` | M | 1360-byte output-parameter block apply; only `0x801A2450` on the accumulate side is translated. |
| `0x801A2B10` | `FUN_801a2b10` | M | 784-byte per-frame voice update; uses the indeterminate return of `0x802DCD6C`. |
| `0x801A2E70` | `FUN_801a2e70` | R | 700-byte voice-table initialiser; Ghidra drops the argument to `0x801A0240`. |
| `0x801A34F0` | `FUN_801a34f0` | R | Voice start path using floats and data constants at `0x8033145C`. |
| `0x801A3690` | `FUN_801a3690` | R | Fast start wrapper; depends on `0x801A34F0`. |
| `0x801A3760` | `FUN_801a3760` | R | Slow start wrapper; depends on `0x801A34F0`. |
| `0x801A3DB0..0x801A4180` | `FUN_801a3db0`/`FUN_801a3f30`/`FUN_801a4060`/`FUN_801a4180` | M | Synth note-on primitives. |
| `0x801A4540..0x801A4890` | `FUN_801a4540`/`FUN_801a4700`/`FUN_801a4890` | M | Second voice start/stop wrappers. |
| `0x801A71F0` | `FUN_801a71f0` | M | JAS buffer clear loop. |
| `0x801A7F60..0x801B1FFF` | multiple | M | Remaining JAI/JAS sequence, bank and channel code. |

The rest of the 872-entry band is text/font, archive, OS or rendering code.

## Uncertainties

1. **Pool object layout.** The unlink code reads the pool number from object
   `+0x0C`, but the deferred allocator fallback `0x801A0380` stores a 64-bit
   value at `+0x0C..+0x13`. The two interpretations cannot both be right; the
   translated functions follow the unlink/insert code, and `0x801A0380` stays
   deferred.
2. **Pool count.** The head/tail arrays span 32 words but every translated
   caller uses indices 0..2. No range check is emitted, matching the original.
3. **`0x800046B4`.** Ghidra prints fewer arguments than the body needs; the
   three-byte clears in [`fn_801a05b0`] follow the wave-3 reconstruction.
4. **Cache helpers.** `0x801AC440` invalidates and `0x801AC470` writes back
   and invalidates; the distinction is read from their bodies, not symbols.
5. **DSP task labels.** `0x801A1C50`, `0x801A1CC0` and `0x801A1CD0` are
   interior code/data labels, not exported function entries.
6. **BRSAR conventions.** The vtable slot `+0xA4`, the meaning of its four
   arguments, and the success convention of `0x80277A90` are inferred from
   the call shape and the surrounding state machine.
7. **Interrupt tokens.** `0x801B1140` returns the old MSR and `0x801B1180`
   restores it; Ghidra drops the argument at several call sites, so the token
   is reconstructed from the ABI.
8. **Voice flag word.** The constant `0x18A80024` written by
   [`fn_801a2e30`] is a packed flag pair; its bit meanings are not recovered.
9. **`0x8032E12C`.** The string is named from the Ghidra label
   `s__Sound_wii_mj2d_sound_brsar_8032e12c`; the bytes themselves were not
   re-read in this pass.

## Ledger

Machine-readable rows: `ledger/agents/wave4-audio.csv` (module `audio::ax`,
23 rows, all `translated`). Wave-3 rows remain in
`ledger/agents/wave3-audio.csv`.
