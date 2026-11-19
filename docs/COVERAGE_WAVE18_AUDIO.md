# Coverage - Wave 18 audio (BRSAR cue and bank-group hooks) `0x8019B880..0x8019D1C0`

## How to read this file

This is the status page for the wave-18 pass over the game sound-manager
methods in the audio lane of the `main.dol` reconstruction. It answers three
questions:

1. **What did this pass translate, and where is it?** - *Selected functions*
   gives one row per function with address, Rust name, plain-English role,
   cluster, status, confidence and evidence file.
2. **How do the hooks fit the JAS driver?** - *Clusters* explains the cue-latch
   path, the cue mask, the playback wrappers, the bank-group/flag sets, and the
   BGM stream switch.
3. **What was deliberately left out, and why?** - *Deferred inventory* lists
   the nearby addresses that were not selected, followed by *Uncertainties*
   and the ledger pointer.

Column meanings:

- **Status** - `translated` means the entry/exit, every call and every
  observable memory write is accounted for. There are no `partial` or `stub`
  rows in this pass.
- **Confidence** - strength of the evidence, not code quality:
  `high` = the guard, arithmetic, call order and stores are explicit in the
  Ghidra C; `medium` = the flow is explicit but a callee role, a field name or
  an SDA word is inferred; `low` = mostly guesswork.
- **Reason codes** (deferred inventory):
  - **M** - large sound-manager body reserved for a later pass.
  - **R** - audio-adjacent but not selected (needs a caller or class layout).
  - **O** - opaque helper already called by address from this pass.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:` (visible in the Ghidra output), `// inferred:`
(role guessed from call sites), `// unknown:` (unresolved), `// partial:`
(deliberately incomplete).

**Promotion status.** All wave-18 functions live in `src/audio/ax.rs`. The
machine-readable ledger is `ledger/agents/wave18-audio.csv` (module
`audio::ax`, 23 rows, all `translated`). Earlier audio rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv`,
`wave7-audio.csv` and `wave11-audio.csv`.

## Range inventory

The wave-18 band is one contiguous stretch of the game sound manager, below the
JAS voice buffers and above the sound-system primitives it calls:

| Span | Purpose | Selected |
|---|---|---:|
| `0x8019B880..0x8019BBA0` | cue latch and archive prime | 2 |
| `0x8019BC50..0x8019BE70` | cue mask and derived cue words | 4 |
| `0x8019C150..0x8019C4E0` | cue playback wrappers | 4 |
| `0x8019C670..0x8019D040` | bank-group enable/mute and flag sets | 10 |
| `0x8019D0F0..0x8019D140` | BGM stream switch | 2 |
| `0x8019D1C0` | group ids and volume publish | 1 |

Totals for this wave: **23 translated, 0 partial, 0 stubs**;
**21 high, 2 medium, 0 low**.

## Selected functions

| Address | Rust | Role (plain English) | Cluster | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x8019B880` | `fn_8019b880` | Latch one cue id and update the derived cue words. | A | translated | medium | `8019b880_FUN_8019b880.c` |
| `0x8019BBA0` | `fn_8019bba0` | Prime the archive when the sound object is idle. | A | translated | medium | `8019bba0_FUN_8019bba0.c` |
| `0x8019BC50` | `fn_8019bc50` | Apply a cue and refresh the running state. | B | translated | high | `8019bc50_FUN_8019bc50.c` |
| `0x8019BD20` | `fn_8019bd20` | Clear a cue and refresh the running state. | B | translated | high | `8019bd20_FUN_8019bd20.c` |
| `0x8019BDA0` | `fn_8019bda0` | Set or clear one bit of the `+0x1C` cue mask. | A | translated | high | `8019bda0_FUN_8019bda0.c` |
| `0x8019BE70` | `fn_8019be70` | Push the derived cue words to the sequence handle. | A | translated | high | `8019be70_FUN_8019be70.c` |
| `0x8019C150` | `fn_8019c150` | Pause/resume/stop dispatcher. | C | translated | high | `8019c150_FUN_8019c150.c` |
| `0x8019C1E0` | `fn_8019c1e0` | Play cue `0x750` or `0x751`. | C | translated | high | `8019c1e0_FUN_8019c1e0.c` |
| `0x8019C330` | `fn_8019c330` | Play cue `0x74E`. | C | translated | high | `8019c330_FUN_8019c330.c` |
| `0x8019C4E0` | `fn_8019c4e0` | Play the `0x23`-pair cue. | C | translated | high | `8019c4e0_FUN_8019c4e0.c` |
| `0x8019C670` | `fn_8019c670` | Enable the five music bank groups. | D | translated | high | `8019c670_FUN_8019c670.c` |
| `0x8019C730` | `fn_8019c730` | Disable the five music bank groups. | D | translated | high | `8019c730_FUN_8019c730.c` |
| `0x8019C7F0` | `fn_8019c7f0` | Set flag 2 and mute every non-global group. | D | translated | high | `8019c7f0_FUN_8019c7f0.c` |
| `0x8019C880` | `fn_8019c880` | Clear flag 2 and unmute every group. | D | translated | high | `8019c880_FUN_8019c880.c` |
| `0x8019C9C0` | `fn_8019c9c0` | "Any flag other than bit 0 is set". | D | translated | high | `8019c9c0_FUN_8019c9c0.c` |
| `0x8019CD70` | `fn_8019cd70` | Clear flag 3 and refresh or hand back to the engine. | D | translated | high | `8019cd70_FUN_8019cd70.c` |
| `0x8019CE70` | `fn_8019ce70` | Set flag 4 and mute every group. | D | translated | high | `8019ce70_FUN_8019ce70.c` |
| `0x8019CEF0` | `fn_8019cef0` | Clear flag 4 and unmute every group. | D | translated | high | `8019cef0_FUN_8019cef0.c` |
| `0x8019CFB0` | `fn_8019cfb0` | Set flag 5 and mute every non-global group. | D | translated | high | `8019cfb0_FUN_8019cfb0.c` |
| `0x8019D040` | `fn_8019d040` | Clear flag 5 and unmute every group. | D | translated | high | `8019d040_FUN_8019d040.c` |
| `0x8019D0F0` | `fn_8019d0f0` | Raise "music playing" and enable the BGM stream. | E | translated | high | `8019d0f0_FUN_8019d0f0.c` |
| `0x8019D140` | `fn_8019d140` | Clear "music playing" and pause the BGM stream. | E | translated | high | `8019d140_FUN_8019d140.c` |
| `0x8019D1C0` | `fn_8019d1c0` | Publish the two group ids and the volume. | E | translated | high | `8019d1c0_FUN_8019d1c0.c` |

## Clusters

### Cluster A - cue latch, archive prime and mask (`0x8019B880..0x8019BE70`)

The manager object passed as `sound` keeps its state at `+0x14`; every hook in
this band is gated on the value `0x11`. `fn_8019b880` is the latch: it stops
the current cue (`0x801965B0`), writes group parameter `0x32` on groups 3..8
and 13..14, then stores the cue argument at `+0x44` and the per-cue mode bytes
at `+0x3B`/`+0x3C`/`+0x3D`. Cue ids `0xD`, `0x19`, `0x1A`, `0x1B`, `0xE`,
`0xF`, `0x10`, `0x11` and `0x13` have explicit overrides; every other id below
`0x1C` keeps the cleared defaults.

`fn_8019bda0` is the only writer of the 16-bit mask at `+0x1C`. Setting or
clearing a bit also refreshes `+0x38` (bit 4), `+0x1F` (highest changed bit),
`+0x1E`/`+0x20` (derived bytes), and discards the entire mask when bit 0 is
clear. `fn_8019be70` compares the pending and applied bits, calls
`0x8019E4C0` when they differ, and refreshes the sequence through `0x8019E6F0`
whenever the mask is non-zero.

`fn_8019bba0` sits beside the latch: when the archive reports idle it marks
`+0x3A`, stops the manager through virtual slot `+0x94`
(`[manager] -> vtable`, confirmed in the disassembly), and records the archive
handle from `0x8027A410` at `r13 - 0x61E8`.

### Cluster B - cue apply and clear (`0x8019BC50..0x8019BD20`)

Both wrappers share one guard: state `0x11`, cue below `0x8002`, and a
sequence state at handle `+0x10` that is not `0x16`, `0x10` or `0x13` - unless
the cue is `1`. `fn_8019bc50` additionally handles cue `0x10` (marks `+0x38`
and re-applies the envelope through `0x8019E8E0`) and the cue-`1` re-apply
sentinel (`+0x1E = 0xFF` when `+0x1F == 1`). `fn_8019bd20` is the clear
counterpart. Both finish through `fn_8019be70`.

### Cluster C - playback wrappers (`0x8019C150..0x8019C4E0`)

`fn_8019c150` dispatches on an action word: `1` sets parameter `0x32` and
clears cue `1`, `2` plays `0x783`, `3` plays `0x78A`. `fn_8019c1e0` always sets
parameter `0x32` and then plays `0x750` or `0x751` from its variant word.
`fn_8019c330` plays `0x74E`. `fn_8019c4e0` calls `0x80196550` with the group
block at sound-system `+0x910` and the id pair `(0x23, 0x23)`. All four return
without touching the sound system unless the state is `0x11`.

### Cluster D - bank groups and flag sets (`0x8019C670..0x8019D040`)

The flag byte at `+0x30` is the gate and the result of this cluster:

| bit | mask | meaning (inferred) |
|---|---|---|
| 0 | `0x01` | a cue is playing; gates restart and the mask |
| 1 | `0x02` | BGM stream is running |
| 2 | `0x04` | non-global groups are muted |
| 3 | `0x08` | engine pause is handed back |
| 4 | `0x10` | group mute set 4 |
| 5 | `0x20` | group mute set 5 |

`fn_8019c670`/`fn_8019c730` enable or disable groups `3, 5, 8, 4, 4` with the
pair `(1, 10)` or `(0, 10)`. The duplicated group `4` is present in both
exports and is preserved. `fn_8019c7f0` and `fn_8019cfb0` publish the ids and
volume first (`fn_8019d1c0`), mute every group except `0` and `0x12`, and set
bit 2 or bit 5. `fn_8019c880` and `fn_8019d040` reverse those paths, re-apply
the group volumes/scalars (`0x8019D2D0`/`0x8019D480`), and restart the active
cue when bit 0 is still set. `fn_8019c9c0` exposes the "any non-playing bit"
test used by the caller. `fn_8019ce70`/`fn_8019cef0` and `fn_8019cd70` manage
the mute sets and the engine pause switch; the `0x942` byte on the sound
system is the paired stream flag.

### Cluster E - BGM stream and publish (`0x8019D0F0..0x8019D1C0`)

`fn_8019d0f0` raises "music playing" and enables the stream through
`0x80195FB0(1, 0)` while the flag byte is below 4; `fn_8019d140` clears the
stream through `0x80195FB0(0, 0)` and always clears bit 1. `fn_8019d1c0` reads
the two id sub-objects at sound-system `+0x910`/`+0x90C` (each contributes
`+0x9C`, or `0xFFFFFFFF` when null), substitutes `0x76` for both when the force
byte `+0x3E` is set, and publishes `(id1, id2, volume)` through `0x80195820`
after the stream state `(10, 1)`.

## Deferred inventory

| Address | Symbol in export | Reason | Note |
|---|---|---|---|
| `0x8019BED0` | `FUN_8019bed0` | M | 640-byte manager state body; needs the full field map. |
| `0x8019C370` | `FUN_8019c370` | R | 348-byte cue helper; no selected caller yet. |
| `0x8019CE00` | `FUN_8019ce00` | R | 112-byte flag helper adjacent to the flag-3/4 paths. |
| `0x8019CA00` | `FUN_8019ca00` | O | Cue restart, called opaquely by the clear paths. |
| `0x8019D2D0` | `FUN_8019d2d0` | O | Per-group volume re-apply, called opaquely. |
| `0x8019D480` | `FUN_8019d480` | O | Per-group scalar re-apply, called opaquely. |

`0x8019B660` (manager state transition and BRSAR load hook) was translated by
wave 4 and is not duplicated here. The sound-system primitives `0x801954A0`,
`0x80195820`, `0x80195E80`, `0x80195FB0`, `0x80195F10`, `0x80196550`,
`0x801965B0`, `0x80196BD0` and `0x8019E080` remain cross-lane callees and are
called by address.

## Uncertainties

1. **Manager layout.** Every field name (`+0x14` state, `+0x30` flags, `+0x1C`
   mask, `+0x34` cue id, and the `+0x3A..+0x44` bytes) is inferred from the
   writers and readers; the class layout is not in the image.
2. **Archive handle.** `r13 - 0x61E8` is written only by [`fn_8019bba0`] from
   the return of `0x8027A410(manager + 0x0C)`. Its name and later consumers
   are not recovered.
3. **Vtable slot.** The disassembly shows `lwz r12,0(r3)` then
   `lwz r12,0x94(r12)`; the slot role ("stop the sound object") is inferred.
4. **Group ranges and parameter.** The two ranges 3..8 and 13..14 in
   [`fn_8019b880`] and the parameter value `0x32` are observed; their musical
   meaning is unknown.
5. **Duplicate group 4.** [`fn_8019c670`] and [`fn_8019c730`] really do write
   group `4` twice; the Rust keeps both writes.
6. **Dropped argument.** Ghidra prints `FUN_8019d1c0()` with no argument at
   the start of [`fn_8019c7f0`] and [`fn_8019cfb0`]. The disassembly shows
   `r3` still holds the manager object, so the Rust passes `sound` explicitly.
7. **Dead stack stores.** The force-`0x76` branch of [`fn_8019d1c0`] also
   stores two `0x76` words and a float constant to unused stack slots. Ghidra
   omits them; they have no observable effect and are not reproduced.
8. **Bit trick.** [`fn_8019c9c0`] uses `(-v & ~v) >> 31`; for the byte-wide
   input `v` this is equivalent to "any bit above bit 0 is set", which is what
   the Rust expresses.

## Ledger

Machine-readable rows: `ledger/agents/wave18-audio.csv` (module `audio::ax`,
23 rows, all `translated`). Earlier rows remain in
`ledger/agents/wave3-audio.csv`, `wave4-audio.csv`, `wave5-audio.csv`,
`wave7-audio.csv` and `wave11-audio.csv`.
