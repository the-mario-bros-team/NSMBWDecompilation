# Wave 18 — player motion, animation-result and shared-accessor bodies

## How to read this file

This document covers one Wave-18 slice of the gameplay player actor. Every row
is a main.dol function whose **entry address** is in
`0x80040000..0x80080000`; the Rust body lives in `src/game/player.rs` and no
staging module was created.

* **Address** — virtual entry point in the DOL image (not a file offset).
* **Name** — the Ghidra name. There is no surviving symbol table, so
  `FUN_<address>` is the honest identity until a role is proven. The Rust body
  keeps the same `fn_<address>` name.
* **Role** — one plain sentence. Words such as “landing”, “carry” and
  “character change” are inferred from the fields and effects used, not from a
  recovered symbol.
* **Status** — `translated` means the full control flow and every call are
  present. All rows here are `translated`.
* **Confidence** — strength of the evidence, not code quality:
  `high` = instruction-level match, `medium` = some field/effect roles guessed.
* **Evidence** — the Ghidra C file under
  `work/ghidra/main-gb/export/`.

The wave covers the gap between the Wave-14 attack/character-change cluster
(`0x800480D0..0x8004E980`) and the Wave-5 movement helpers, plus the shared
leaf accessors those paths call. All bodies use `r2 = 0x80433080` and
`r13 = 0x8042F680`; constants were re-read from `work/dol/main.elf` and the
Ghidra C is noted where it disagrees.

## Functions translated by this wave

| Address | Name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8004E040` | `FUN_8004E040` | Returns bit 1 of the SDA state word at `0x80429F80`, the second-slot/co-op predicate. | translated | high | `8004e040_FUN_8004e040.c` |
| `0x8004F300` | `FUN_8004F300` | Runs the animation-result query for the current mode and forwards a recovered face id to `0x8004F480`. | translated | high | `8004f300_FUN_8004f300.c` |
| `0x8004F480` | `FUN_8004F480` | Stores a face/bonus id at `+0x3F8`, resolves it through the stage record, classifies it into mode `1`/`2`/`3`, and commits the change. | translated | high | `8004f480_FUN_8004f480.c` |
| `0x8004F740` | `FUN_8004F740` | Stores a face-change mode at `+0x418` and resets the pose, effect vector and motion rates for the new face. | translated | high | `8004f740_FUN_8004f740.c` |
| `0x800504B0` | `FUN_800504B0` | Starts the face effect from `0x80370DCC`/`0x80370DD0` and clears the motion block. | translated | high | `800504b0_FUN_800504b0.c` |
| `0x80050A00` | `FUN_80050A00` | Resolves the stored face id and starts `0x8035482C`/`0x803548AC`/`0x803548EC` based on the entry type byte. | translated | high | `80050a00_FUN_80050a00.c` |
| `0x80050E60` | `FUN_80050E60` | Hands an airborne player to the carry/throw manager: writes the position block, reserves the slot, probes the ground and updates the counters. | translated | high | `80050e60_FUN_80050e60.c` |
| `0x80051080` | `FUN_80051080` | Clears the horizontal/vertical motion rates and starts the `+0x3A0` pose when event flag `0x65` is clear. | translated | high | `80051080_FUN_80051080.c` |
| `0x800510F0` | `FUN_800510F0` | ORs `0x0F` into the SDA state byte and clears the carry bit on the current and partner actors of every flagged slot. | translated | high | `800510f0_FUN_800510f0.c` |
| `0x800511A0` | `FUN_800511A0` | Clears the low nibble of the SDA state byte and sets the carry bit on both actors; inverse of `0x800510F0`. | translated | high | `800511a0_FUN_800511a0.c` |
| `0x80051240` | `FUN_80051240` | Clears event flag `0x65`, optionally restarts pose `0x1F`/flag `0x7E`, and resets the whole motion block. | translated | high | `80051240_FUN_80051240.c` |
| `0x80051420` | `FUN_80051420` | Computes the active-slot Z: `+0x404 + 16 * slot_count`, minus `32 * +0x434` when more than one slot is active. | translated | high | `80051420_FUN_80051420.c` |
| `0x80051490` | `FUN_80051490` | Clears flag `0x7E`, probes the ground below the active slot and starts the airborne landing pose. | translated | high | `80051490_FUN_80051490.c` |
| `0x80051930` | `FUN_80051930` | Dispatches the landing reaction: sound path or actor-spawn path with event `8`/`9`, then tests actor slot `+0x284`. | translated | high | `80051930_FUN_80051930.c` |
| `0x80051BD0` | `FUN_80051BD0` | Integrates gravity (`-0.28`, clamp `-4.0`) into `+0xEC`/`+0xB0` and snaps to the stored ground position when falling with flag bit 0. | translated | high | `80051bd0_FUN_80051bd0.c` |
| `0x80051E70` | `FUN_80051E70` | Chooses between the damage interface (`0x80354BEC`) and the `0x71` reset based on damage modes `4` and `6`. | translated | high | `80051e70_FUN_80051e70.c` |
| `0x80051F30` | `FUN_80051F30` | Stores the terrain byte at `+0x348` and restarts the `0x8035425C` animation when it is not already playing. | translated | high | `80051f30_FUN_80051f30.c` |
| `0x80052080` | `FUN_80052080` | Starts the `0x72` dash/charge state and clamps `+0x45C` down to the model limit at `model + 8`. | translated | high | `80052080_FUN_80052080.c` |
| `0x80052170` | `FUN_80052170` | Sets state `2`, restarts the `0x8035461C` animation and feeds the indexed `0x80370DCC` effect vector to the interface. | translated | high | `80052170_FUN_80052170.c` |
| `0x80052390` | `FUN_80052390` | Commits the damage interface (`0x80354BEC`, selector `6`) when the character-change interface is ready or damage mode `5` is active. | translated | high | `80052390_FUN_80052390.c` |
| `0x80052420` | `FUN_80052420` | Commits the damage interface with selector `8` when no damage mode is active. | translated | high | `80052420_FUN_80052420.c` |
| `0x80052500` | `FUN_80052500` | Swept landing/step query: probes `+0xAC + amount * sign` and rejects the step when the rise or height test fails. | translated | high | `80052500_FUN_80052500.c` |

**Count: 22 new Rust bodies.**

## Where the Ghidra C and the PowerPC bytes disagree

All items were checked against `work/dol/main.elf`; the Rust follows the
instruction stream.

| Address | Ghidra C | Instruction stream |
|---|---|---|
| `0x80051490` | compares `manager+0x1C` against `2` but the printed float constants are unordered | `manager+0x1C < 2` selects `4.928`, otherwise `5.128` |
| `0x80051BD0` | teleports on any negative velocity | teleports only when `+0xEC < 0.0` and bit 0 of `+0x10D4` is set |
| `0x80052080` | writes the scale unconditionally | keeps the larger of `abs(scale)` and `model+8` bounds, clamping `+0x45C` down |
| `0x80052500` | mixes the two probe buffers | the rise is `max(4.0, 1.2 * amount)` and the step is rejected when `abs(outY - playerY) >= rise` |
| `0x8004F300` | loses the float pair in the mode `2`/`3` branch | `(2.0, -2.0)` for flag `0x3A`, `(8.0, 8.0)` for `0x17`, otherwise `(0, 0)` |

The swept query is also called by `0x800526C0` with `amount = 10.0`,
`flag = 0`; that wrapper already existed and now calls the real body.

## Known gaps

* `// unknown:` the gameplay names of the effect ids `0x55`, `0x58` and the
  `0x80370DCC`/`0x803711C8`/`0x803711EC` vectors; only the interface slot
  `+0x5C` and the three-float order are observed.
* `// unknown:` the meaning of `manager+0x1C` (used as both a slot count and a
  branch selector) and of `player+0x434`; both are read exactly but not named.
* `// unknown:` the roles of the event flags `0x65`, `0x6F`, `0x71`, `0x72`,
  `0x7E`; only the set/clear sites are proven.
* `// inferred:` “face/bonus id” for `+0x3F8` and “face-change mode” for
  `+0x414`/`+0x418`. The field movement is exact; the gameplay label is not
  proven.
* `0x80052500` and `0x800526C0` replaced the earlier `fn_80052500_f32`
  placeholder; that stub no longer exists.
