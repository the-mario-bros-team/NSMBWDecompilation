# Wave 5 — player actor coverage (`0x8004A2C0..0x8004F700`)

## How to read this file

This is the lane report for the fifth player pass. It lists the **23** functions
added to `src/game/player.rs` from `main.dol`, all with complete recorded
control flow. Every row gives the original address, the Ghidra name, a
one-sentence role, the status, the confidence, and the evidence file.

* **Address** — DOL virtual address (image base `0x80004000`), not a file
  offset.
* **Name** — the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; descriptive roles are marked as
  inferred in the Rust doc comments.
* **Status** — `translated` means the recorded branches, calls and field
  writes are represented in Rust. It does not claim the field roles are proven.
* **Confidence** — strength of evidence, not code quality: `high` means the
  Ghidra C and the instruction stream were both checked; `medium` means some
  field/probe roles remain inferred.
* **Evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`. The
  Rust body uses the same `fn_<address>` name in `src/game/player.rs`.

Addresses are DOL virtual addresses. `r2 = 0x80433080` and
`r13 = 0x8042F680`; every `r2 - k` / `r13 - k` constant below was resolved
with those bases and then read from `work/dol/main.elf`. The `.text` mapping
used for the raw checks is file offset `0x2B00 + (address - 0x80006780)`.

## Summary

| Source | Range | Functions | Translated | Partial | Stub |
|---|---|---:|---:|---:|---:|
| `main.dol` | `0x8004A2C0..0x8004F700` | 23 | 23 | 0 | 0 |

Confidence: **21 high**, **2 medium**, **0 low**.

The selection covers the gameplay actor's movement target, the two remaining
animation-rate selectors, the knockback accumulator, damage/state predicates,
state-indexed sound and effect tables, and the position-move helpers. No
function from another lane was moved or edited.

## Functions

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8004A2C0` | `fn_8004a2c0` | Conditionally copies two coordinates into actor X/Y after an interface name query. | translated | high | `8004a2c0_FUN_8004a2c0.c` |
| `0x8004A400` | `fn_8004a400` | Cancels the active state, damps speed by `0.7`, sends interface effect `0x8A`, and notifies the owner. | translated | high | `8004a400_FUN_8004a400.c` |
| `0x8004A4D0` | `fn_8004a4d0` | Clears event flag `0x7F` and requests movement mode `4`. | translated | high | `8004a4d0_FUN_8004a4d0.c` |
| `0x8004A830` | `fn_8004a830` | Resets motion, stores `0.1` in `+0xEC`, sends effect `7`, and sets `+0x30C` bit `0x04`. | translated | high | `8004a830_FUN_8004a830.c` |
| `0x8004A8C0` | `fn_8004a8c0` | Clears the ride/effect state, two event flags and `+0x30C` bit `0x04`. | translated | high | `8004a8c0_FUN_8004a8c0.c` |
| `0x8004A920` | `fn_8004a920` | Copies a linked actor's position, three halfwords (inferred rotation) and terrain byte when the movement state is idle. | translated | medium | `8004a920_FUN_8004a920.c` |
| `0x8004AC10` | `fn_8004ac10` | Sends interface effect `0x93` once and clears event flag `3`. | translated | high | `8004ac10_FUN_8004ac10.c` |
| `0x8004AD90` | `fn_8004ad90` | Probes the ground X and chooses effect `0x91`/`0x90`, then feeds the class result to `0x8015F340`. | translated | medium | `8004ad90_FUN_8004ad90.c` |
| `0x8004B730` | `fn_8004b730` | Recomputes the target horizontal speed from the movement entry, model scale and direction table. | translated | high | `8004b730_FUN_8004b730.c` |
| `0x8004BC80` | `fn_8004bc80` | Selects the animation rate from the nine-float pose block and the speed thresholds. | translated | high | `8004bc80_FUN_8004bc80.c` |
| `0x8004C310` | `fn_8004c310` | Selects the animation rate from the two seven-entry halves at `0x802F5580`. | translated | high | `8004c310_FUN_8004c310.c` |
| `0x8004C780` | `fn_8004c780` | Accumulates the signed knockback/slide value and clamps it to the transfer-curve limit. | translated | high | `8004c780_FUN_8004c780.c` |
| `0x8004CA60` | `fn_8004ca60` | Selects a state-indexed sound id, with a special state-`5` path. | translated | high | `8004ca60_FUN_8004ca60.c` |
| `0x8004D5C0` | `fn_8004d5c0` | Selects a state-indexed effect id after the damage predicate and mode gate. | translated | high | `8004d5c0_FUN_8004d5c0.c` |
| `0x8004DB40` | `fn_8004db40` | Evaluates one of six damage/state modes. | translated | high | `8004db40_FUN_8004db40.c` |
| `0x8004DD00` | `fn_8004dd00` | Any of damage modes `1..3`. | translated | high | `8004dd00_FUN_8004dd00.c` |
| `0x8004DD70` | `fn_8004dd70` | Any of damage modes `4..6`. | translated | high | `8004dd70_FUN_8004dd70.c` |
| `0x8004DDE0` | `fn_8004dde0` | Any of the six damage modes. | translated | high | `8004dde0_FUN_8004dde0.c` |
| `0x8004F1C0` | `fn_8004f1c0` | Moves X toward `+0x3FC` and reports whether the remaining distance is within tolerance. | translated | high | `8004f1c0_FUN_8004f1c0.c` |
| `0x8004F240` | `fn_8004f240` | Moves Y toward `+0x400 + value`. | translated | high | `8004f240_FUN_8004f240.c` |
| `0x8004F260` | `fn_8004f260` | Reports that no damage mode and none of flags `4`, `6`, `8`, `0x53` is active. | translated | high | `8004f260_FUN_8004f260.c` |
| `0x8004F670` | `fn_8004f670` | Resets motion, calls the actor's `+0x428` slot with `-1800.0`, and starts the stop effect. | translated | high | `8004f670_FUN_8004f670.c` |
| `0x8004F700` | `fn_8004f700` | Clears event flags `0x2A` and `0x5E`. | translated | high | `8004f700_FUN_8004f700.c` |

Evidence root: `work/ghidra/main-gb/export/`.

## Constants and tables

All values below were read from `main.elf` with `r2 = 0x80433080`.

| address | value / contents | use |
|---|---|---|
| `0x8042B840` | `1.0, -1.0, 3.628, 3.5, 2.3, 0.5625, 1.125, 1.25, -3.0, -4.0, -2.0, -8.5` | direction-scale table used by the speed/animation selectors |
| `0x8042B8C4` | `0.5` | speed threshold and knockback delta factor |
| `0x8042B8A0` | `3.0` | animation-rate boost and knockback limit factor |
| `0x8042B910` | `0.004` | near-stop animation rate |
| `0x8042B90C` | `0.75` | rate when speed exceeds the model maximum |
| `0x8042B8F4` | `0.7` | speed factor in `0x8004A400` |
| `0x8042B8D0` | `0.1` | vertical-speed store in `0x8004A830` |
| `0x8042B954` | `-1800.0` | stop impulse passed to `+0x428` |
| `0x8042B924` / `0x8042B928` | `0.45` / `0.55` | short/long knockback limits |
| `0x802F5580` | seven rates `0, 0.08, 0.03, 0.06, 0.029, 0.021, 0.09`; second half `0, 0.11, 0.04, 0.078, 0.039, 0.027, 0.12` | `0x8004C310` |
| `0x803546C4` | `u32[13]`: `E5 E8 E9 E7 E5 E5 E5 E9 E5 E9 E5 E5 E5` | state-indexed sound ids |
| `0x8035472C` | `u32[13]`: `C7 CF CB C7 C5 D1 D7 D3 D5 CB D9 CD DB` | state-indexed effect ids |
| `0x80429C89` / `0x80429C8B` | ready bytes | lazy table guards for the two tables above |
| `0x8042A204` | `i32` global mode (BSS) | `2` suppresses `0x8004D5C0` |
| `0x8042A008` | owner-table global (BSS) | passed to `0x800D8BB0` |
| `0x80429D50` | probe context global (BSS) | first argument of `0x80069270` |
| `0x80370E20` / `0x80371444` / `0x8037148C` / `0x80371498` / `0x803714B0` | `f32` pairs (BSS, runtime) | effect position arguments |
| `0x8035455C` | name string pointer | `0x8004A2C0` query |
| `0x8035431C`, `0x8035429C` | name pointers | `0x8004A920` refusal paths |

## Where Ghidra C and the bytes disagree

These were checked against `work/dol/main.elf`; the Rust follows the bytes and
the difference is documented on the function.

| Address | Ghidra C | Instruction stream |
|---|---|---|
| `0x8004A2C0` | `+0xAC = param_1`, `+0xB0 = param_2` | `f2` (second float) is stored at `+0xAC`; `f1` (first) at `+0xB0` (`0x8004A318`/`0x8004A320`). |
| `0x8004A920` | adds the `+0x3AC` return value to `+0xB0` | the return in `f1` is overwritten by `0.0` before the add (`0x8004A9E8..0x8004A9F8`); only the call's side effects remain. |
| `0x8004AD90` | `local_28[0] <= player+0xAC` sets `+0x348 = 1` | `mfcr` + `rlwinm ...,2,31,31` extracts the `EQ` bit only (`0x8004ADF0..0x8004AE18`). |
| `0x8004C310` | table offset `variant * -0x1C` | assembly computes `variant * +0x1C` (`mulli r0,r0,28` at `0x8004C35C`/`0x8004C360`); both spellings select the second table half for a non-zero variant. |
| `0x8004D5C0` | `FUN_8004DD00()` with no printed argument | `r3` is still the player at the call (`0x8004D5D0`/`0x8004D5E0`). |
| `0x8004DB40` | `FUN_8004DE30()` without an argument | `r3` is the player at the call (`0x8004DB88`). |
| `0x8004F1C0` / `0x8004F240` | `undefined8` parameter grouping | the helper `0x8015F470` receives `(pointer, f1 target, f2 step)`; the first incoming float is the tolerance in `0x8004F1C0`. |

## Unknowns and inferred pieces

* `// unknown:` the interface name at `0x8035455C` and the effect position
  pairs are BSS/resource strings; only their use is known.
* `// inferred:` `+0x348` is called a "ground/match" byte because it is set
  from the probe's X equality and then copied as a terrain byte by
  `0x8004A920`.
* `// inferred:` the six damage modes are described by their gates; no
  recovered enum names survive.
* `// inferred:` the global at `0x8042A204` is called a "mode" because value
  `2` suppresses the state effect; its exact owner is unknown.
* All table indices and object fields are unchecked in the original; the Rust
  preserves that and documents the expected range at each table.

## Gates

* `cargo check` — exit `0`, no errors or warnings.
* `node tools/doc-lint.mjs src` — `ok: true`, 68 files, 0 violations.

## Next targets

1. The callers of the new damage predicates (`0x8004DB40` family) have not been
   read; their state machines are the natural continuation.
2. `0x8004F300` and `0x8004F480` (power-up/effect selection through the level
   actor table) remain untranslated and are the next largest coherent pair.
3. `0x8004E050..0x8004E980` (character-select/teardown state) is still outside
   this lane's gameplay cluster.
