# Coverage - Wave 3 audio (AX/JAS) `0x80190000..0x801A8000`

## How to read this file

This is the status page for the audio lane of the `main.dol` reconstruction.
It answers three questions:

1. **What is in the band?** - *Range inventory* counts every Ghidra entry in
   `0x80190000..0x801A8000` and separates the audio driver from the archive
   reader that shares the low end of the band.
2. **What did this pass translate, and where is it?** - *Selected functions*
   gives one row per function with address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists
   the important omitted addresses with a reason code, followed by
   *Uncertainties* and the ledger pointer.

Column meanings:

- **Status** - `translated` means the entry/exit, every call and every
  observable memory/MMIO write is accounted for. There are no `partial` or
  `stub` rows in this pass.
- **Confidence** - strength of the evidence, not code quality:
  `high` = the arithmetic or register sequence is explicit in the Ghidra C;
  `medium` = the flow is explicit but a callee identity, register-bit role or
  data-structure meaning is inferred; `low` = mostly guesswork.
- **Reason codes** (deferred inventory):
  - **A** - archive/resource code (JKRArchive/ARC), not audio.
  - **M** - large JAS mixer/channel/voice code reserved for a later pass.
  - **O** - opaque helper (trampoline, DSP upload, noreturn path).
  - **S** - fewer than 16 bytes: veneer, thunk or shared-`blr` helper.
  - **R** - audio-adjacent but not selected (needs a class layout or a caller
    that is not reconstructed yet).

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output), `// inferred:`
(role guessed from call sites), `// unknown:` (unresolved), `// partial:`
(deliberately incomplete).

**Promotion status.** All wave-3 functions live in `src/audio/ax.rs`. The
machine-readable ledger is `ledger/agents/wave3-audio.csv`. The four older
audio-DMA accessors (`0x8019F180..0x8019F1D0`) were already in the same file and
are not counted as wave-3 rows.

## Range inventory

Source: `work/ghidra/main-gb/export/functions.tsv`, filtered by **entry**
address (`0x80190000 <= addr < 0x801A8000`).

| Metric | Count |
|---|---:|
| Exported function entries in the band | 369 |
| Already translated before wave 3 (audio-DMA accessors) | 4 |
| Translated by this wave | 18 |
| Deferred | 347 |

The low end of the band (`0x80190300..0x8019F5FF`) appears to be text/font
layout code (glyph tables at `0x8032D2D0`, character maps at `0x80378078`). The
range `0x8019F660..0x8019FF80` behaves as the JKRArchive/ARC reader: magic
`0x55AA382D`, name-table walks and archive-entry lookups. Neither group is
audio. The translated audio driver starts at `0x8019F0B0` and runs through
`0x801A1FC8`; later JAS channel and bank code continues past `0x801A2000`.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x8019F0B0` | `fn_8019f0b0` | Swap the AI/DSP interrupt callback; return the old one. | A | translated | high | `8019f0b0_FUN_8019f0b0.c` |
| `0x8019F100` | `fn_8019f100` | Program the audio-DMA source address and byte length. | A | translated | high | `8019f100_FUN_8019f100.c` |
| `0x8019F1E0` | `fn_8019f1e0` | Read the "AI initialised" word at `r13 - 0x51C8`. | A | translated | high | `8019f1e0_FUN_8019f1e0.c` |
| `0x8019F1F0` | `fn_8019f1f0` | One-shot AI/DSP bring-up; derive five timing scales; install handler 5. | A | translated | medium | `8019f1f0_FUN_8019f1f0.c` |
| `0x8019F370` | `fn_8019f370` | DSP interrupt handler: ack, callback, context restore. | A | translated | medium | `8019f370_FUN_8019f370.c` |
| `0x801A0CA0` | `fn_801a0ca0` | Set JAS callback slot A and its argument. | B | translated | medium | `801a0ca0_FUN_801a0ca0.c` |
| `0x801A0D10` | `fn_801a0d10` | Set JAS callback slot B and its argument. | B | translated | medium | `801a0d10_FUN_801a0d10.c` |
| `0x801A0D80` | `fn_801a0d80` | Set JAS callback slot C and its argument. | B | translated | medium | `801a0d80_FUN_801a0d80.c` |
| `0x801A0DF0` | `fn_801a0df0` | Read callback slot A and its argument. | B | translated | high | `801a0df0_FUN_801a0df0.c` |
| `0x801A0E10` | `fn_801a0e10` | Read callback slot B and its argument. | B | translated | high | `801a0e10_FUN_801a0e10.c` |
| `0x801A0E30` | `fn_801a0e30` | Read callback slot C and its argument. | B | translated | high | `801a0e30_FUN_801a0e30.c` |
| `0x801A0E60` | `fn_801a0e60` | Alternate between two 0x80-byte DSP scratch blocks. | B | translated | high | `801a0e60_FUN_801a0e60.c` |
| `0x801A1850` | `fn_801a1850` | Reset JAS output-driver defaults and levels. | C | translated | high | `801a1850_FUN_801a1850.c` |
| `0x801A18F0` | `fn_801a18f0` | Clamp a level to 0x8000 and store it. | C | translated | high | `801a18f0_FUN_801a18f0.c` |
| `0x801A1940` | `fn_801a1940` | Per-frame mix, DSP mailbox handshake, DMA kick. | C | translated | medium | `801a1940_FUN_801a1940.c` |
| `0x801A1BA0` | `fn_801a1ba0` | DMA-completion callback; advance the three-buffer ring. | C | translated | medium | `801a1ba0_FUN_801a1ba0.c` |
| `0x801A1C60` | `fn_801a1c60` | Frame-boundary state machine. | C | translated | medium | `801a1c60_FUN_801a1c60.c` |
| `0x801A1DD0` | `fn_801a1dd0` | Initialise the JAS output driver and start the first block. | C | translated | medium | `801a1dd0_FUN_801a1dd0.c` |

Totals for this wave: **18 translated, 0 partial, 0 stubs**;
**9 high, 9 medium, 0 low**.

## Clusters

### Cluster A - AI/DSP bring-up (`0x8019F0B0..0x8019F420`)

The AI block lives at `0xCD006C00`: control, volume and interrupt timing. The
DSP control/status halfword is `0xCC00500A`, and the audio DMA uses
`0xCC005030/32/36/3A`.

Flow:

1. `fn_8019f1f0` is one-shot. It derives five timing scales from the bus clock
   at `0x800000F8`, clears AI volume/timing, and (when control bit 6 is clear)
   masks control, calls the opaque DSP upload path `0x8019F490`, sets bit 6 and
   restores the masked value.
2. It installs `fn_8019f370` on interrupt 5, unmasks `0x04000000`, and sets the
   init word read by `fn_8019f1e0`.
3. `fn_8019f0b0` is the callback swap used by the JAS init. It returns the old
   callback so a caller can chain.
4. `fn_8019f100` is the writer for the three DMA registers. The high register
   keeps the old top three bits, the low register keeps the old address bits
   29..31, and the control register keeps the start bit.
5. `fn_8019f370` acknowledges the DSP, builds a fresh 0x2D4-byte OSContext,
   invokes the callback once (guarded by `r13 - 0x51C4`), and restores the
   interrupted context.

### Cluster B - JAS callback slots and scratch blocks (`0x801A0CA0..0x801A0E60`)

Three parallel callback slots are stored at `r13 - 0x5140/-0x5144/-0x5148`
with arguments at `-0x514C/-0x5150/-0x5154`. Clearing a slot also writes three
bytes of state (`-0x5134/-0x5138/-0x513C`) through `0x800046B4`, which is
`memset`-shaped in the Ghidra output. The getters copy the callback/argument
pair to caller words.

`fn_801a0e60` owns two adjacent 0x80-byte blocks at `0x8037BEA0`. It returns
the current block and flips the index at `r13 - 0x5110`; the word at
`r13 - 0x5114` is updated for the next caller.

### Cluster C - JAS output pump (`0x801A1850..0x801A1FC8`)

`fn_801a1dd0` is the complete bring-up: zero and flush three working areas
(0x480, 0x300, 0x5A0 bytes), configure the DSP task, install `fn_801a1ba0` as
the AI callback, prime the ring cursor at slot 0x12 of 0xB4, call the mixer
`0x801A0E90` for the initial configuration, programme the first 0x180-byte
block, and set the DMA start bit.

`fn_801a1940` runs once per frame:

- timestamps with `OSGetTime` (`0x801B5F80`);
- reads the DMA bytes-left register through `fn_8019f1a0`;
- sends `0xBABE0080` and then the scratch pointer through the DSP mailbox
  (`0x801D5720`), polling the busy bit with `0x801D56E0`;
- advances the 0xB4-slot ring over four 0x168-byte buffers;
- calls the mixer for the next 0x180-byte block;
- in two-buffer mode programmes the DMA immediately;
- copies 0x40 bytes of timing state into the next trace slot.

`fn_801a1ba0` is the DMA-completion callback. State 1 mixes the next block;
other states set state 2 and start the DSP task at `0x8037CD00`. In
three-buffer mode it also programmes the second cursor and advances it modulo
3 while avoiding the mixer cursor.

`fn_801a1c60` is the frame boundary: state 2 mixes and runs the optional
frame-end callback; any other state moves to 1.

## Deferred inventory

| Address | Symbol in export | Reason | Note |
|---|---|---|---|
| `0x8019F420` | `FUN_8019f420` | O | 12-byte stack-switch trampoline; called opaquely from the handler. |
| `0x8019F010..0x8019F090` | `FUN_8019f010`/`f050`/`f090` | R | Float interpolation and `1,920,000/n` helpers used by the 0x8019A6D0/0x8019C370 audio paths; callers not reconstructed. |
| `0x8019F490` | `FUN_8019f490` | O | Opaque DSP init path; Ghidra renders an infinite loop after `0x802DCD74`. |
| `0x8019F660` | `FUN_8019f660` | A | ARC archive init, magic `0x55AA382D`. |
| `0x8019F9B0` | `FUN_8019f9b0` | A | Archive entry lookup by index. |
| `0x801A00E0` | `FUN_801a00e0` | M | Releases completed JAS objects; called by the frame pump. |
| `0x801A08F0` | `FUN_801a08f0` | M | Large per-frame JAS helper (940 bytes). |
| `0x801A0E90` | `FUN_801a0e90` | M | Mixer; builds one 0x180-byte DSP command block (2492 bytes). |
| `0x801A1CE0` | `FUN_801a1ce0` | M | DSP-task setup and callback table for `0x8037CD00`. |
| `0x801A2B10` | `FUN_801a2b10` | M | Channel/voice update called by the frame pump. |
| `0x801A2E70` | `FUN_801a2e70` | M | 0x60-channel bank/voice table initialiser. |
| `0x801A1940` neighbours `0x801A1FD0..0x801A7FFF` | multiple | M | JAS channel, voice, bank and sequence code; next audio pass. |

The rest of the deferred entries are text/font code, archive code, small
veneers (reason S) or audio-adjacent routines that depend on the class layouts
above (reason R).

## Uncertainties

1. **AI control bits.** `0xCD006C00` is written with masks `0xFFFFFFCA`,
   `0xFFFFFF8A` and `OR 0x20/0x40`. The bit meanings (DSP reset, DMA enable,
   interrupt enable) are inferred from the AI register map, not from strings.
2. **Timing scales.** The five words derived from the bus clock are used as
   floats by `0x8019D480`/`0x8019EDB0`; their physical unit is unknown.
3. **DSP mailbox.** `0xBABE0080` is an opaque command word. Only the handshake
   order (command, poll, pointer, poll) is observed.
4. **Ring geometry.** The ring is 0xB4 slots over four 0x168-byte buffers, with
   a 0x12-slot look-ahead. The slot contents are produced by the untranslated
   mixer `0x801A0E90`, so the unit (stereo frames or bytes) is inferred.
5. **`0x800046B4`.** Ghidra prints one argument but the body matches `memset`
   (`0x80004600`). The three-byte clear is reproduced as
   `memset(state, 1, 3)` and marked `// unknown:` in the Rust.
6. **Context helpers.** `0x801ACF80`/`0x801ACD40` are called with a 0x2D4-byte
   buffer, the OSContext size; their exact OS names are not recovered.
7. **Callback ABIs.** Slot callbacks are invoked with no explicit argument in
   the recovered C. The non-zero branch of `fn_8019f370` routes through
   `0x8019F420`, whose only parameter is a code pointer.

## Ledger

Machine-readable rows: `ledger/agents/wave3-audio.csv` (module `audio::ax`,
18 rows, all `translated`).
