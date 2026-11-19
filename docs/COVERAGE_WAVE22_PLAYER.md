# Wave 22 - player action handlers, damage gates and model-effect records

## How to read this file

This document covers the Wave-22 slice of the gameplay player actor. Every row
is a `main.dol` function whose **entry address** is in
`0x80040000..0x80080000`; the Rust body lives in `src/game/player.rs`. No
staging module was created and no other lane's file was edited.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** - one plain sentence. Words such as "attack pose", "damage gate" and
  "model-effect record" are inferred from the fields and callees, not from a
  recovered symbol.
* **Status** - `translated` means the full control flow and every call are
  present. `partial` means one machine-level operand (here the hidden `r12`
  dispatch of `0x802DCBFC`) cannot be expressed through `crate::platform`.
* **Confidence** - strength of the evidence, not code quality:
  `high` = instruction-level match, `medium` = some field/effect roles guessed.
* **Evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

All bodies use `r2 = 0x80433080` and `r13 = 0x8042F680`. The
`0x80370Dxx`/`0x803711xx` vectors and the `0x8035xxxx` interface objects are
BSS and are read at runtime; the `0x8042B8xx` floats were re-read from
`work/dol/main.elf`. Where the Ghidra C disagrees with the instruction stream,
the Rust follows the bytes and the disagreement is listed below.

## Functions translated by this wave

| Address | Name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80048C70` | `FUN_80048C70` | Per-frame action effects; asks slot `+0xE8` for a scale, feeds the landing-smoke dispatcher, then starts a state-selected sound. | translated | high | `80048c70_FUN_80048c70.c` |
| `0x80048EB0` | `FUN_80048EB0` | Clears landing flags `0x7F`/`0x7A` and starts sound `0x43C` when slot `+0xE0` reports a hit. | translated | high | `80048eb0_FUN_80048eb0.c` |
| `0x80048F30` | `FUN_80048F30` | Selects attack pose `1`, starts effect `0x11`, and clears the vertical speed. | translated | high | `80048f30_FUN_80048f30.c` |
| `0x80048FA0` | `FUN_80048FA0` | Selects attack pose `2`, copies the model word and sets the pose's `-2.0`/`-6.0` vertical rate. | translated | high | `80048fa0_FUN_80048fa0.c` |
| `0x80049050` | `FUN_80049050` | Selects attack pose `4`, starts effect `0x12`, and sets the hit/pose flags. | translated | high | `80049050_FUN_80049050.c` |
| `0x80049110` | `FUN_80049110` | Selects attack pose `5` and starts effect `0x17`. | translated | high | `80049110_FUN_80049110.c` |
| `0x80049180` | `FUN_80049180` | Selects attack pose `6` and starts effect `0x13`. | translated | high | `80049180_FUN_80049180.c` |
| `0x80049540` | `FUN_80049540` | Chooses the air/ground action branch from the movement-state query and the `+0x6D8` latch. | translated | high | `80049540_FUN_80049540.c` |
| `0x800496D0` | `FUN_800496D0` | Chooses between the ground slot and the `0x8035425C` animation when `+0x390` is clear. | translated | high | `800496d0_FUN_800496d0.c` |
| `0x80049760` | `FUN_80049760` | Chooses between the ground slot and the `0x8035439C` animation. | translated | high | `80049760_FUN_80049760.c` |
| `0x800497E0` | `FUN_800497E0` | Commits the terrain animation, clears the motion rates and dispatches the pose-0/pose-1 handler. | translated | high | `800497e0_FUN_800497e0.c` |
| `0x80049880` | `FUN_80049880` | Writes `-4.0` to the vertical rate and clears eleven action flags. | translated | high | `80049880_FUN_80049880.c` |
| `0x80049940` | `FUN_80049940` | Slip/pose teardown that stops the pose timers and restores the stored position. | partial | medium | `80049940_FUN_80049940.c` |
| `0x80049AB0` | `FUN_80049AB0` | Clears the action flags, stops the movement-state object and starts effect `0x07`. | translated | high | `80049ab0_FUN_80049ab0.c` |
| `0x80049C70` | `FUN_80049C70` | Accepts a hit onto the current pose using the attacker height and a ground query. | translated | high | `80049c70_FUN_80049c70.c` |
| `0x80049D70` | `FUN_80049D70` | Copies the attached actor's position into the player with the `+0x6DC`/`+0x6E4` offsets. | translated | high | `80049d70_FUN_80049d70.c` |
| `0x8004C450` | `FUN_8004C450` | Dispatches to the ground/air speed-table selector from bit 0 of `+0x10D4`. | translated | high | `8004c450_FUN_8004c450.c` |
| `0x80051590` | `FUN_80051590` | Nine-state motion machine: fall, target wait, ground clamp, gravity, and the `0x8F` timer. | translated | high | `80051590_FUN_80051590.c` |
| `0x80053120` | `FUN_80053120` | Damage gate plus terrain/hand-state input handling and the five hand timers. | translated | high | `80053120_FUN_80053120.c` |
| `0x800533E0` | `FUN_800533E0` | Commits the damage interface unless the `0x1D4` riding/held exception applies. | translated | high | `800533e0_FUN_800533e0.c` |
| `0x80053540` | `FUN_80053540` | Resets the five model-effect records at `+0x148..+0x1384`. | translated | medium | `80053540_FUN_80053540.c` |
| `0x80053650` | `FUN_80053650` | Copies the player transform into two effect records and derives their mode words from `+0x178`. | translated | high | `80053650_FUN_80053650.c` |
| `0x800536B0` | `FUN_800536B0` | Enables the ten-frame effect pose and its `0x400` durations. | translated | high | `800536b0_FUN_800536b0.c` |
| `0x80053700` | `FUN_80053700` | Clears the seven-frame effect pose and both durations. | translated | high | `80053700_FUN_80053700.c` |

**Count: 24 new Rust bodies** (23 `translated`, 1 `partial`).

The table is in address order; the Rust section groups the bodies by data flow
so one path can be read at a time. The `0x80048C70..0x80049D70` group continues
the attack/effect chain, while the last four functions
(`0x80053540..0x80053700`) own the five model-effect records at `+0x148`,
`+0x1164`, `+0x1208`, `+0x12AC` and `+0x1350`.

## Where the Ghidra C and the PowerPC bytes disagree

All branch and argument checks were re-read from `work/dol/main.elf`. One
Ghidra C statement differs from the instruction stream:

| Address | Ghidra C | Instruction stream |
|---|---|---|
| `0x80053650` | `+0x12DC = 7` | `lwz +0x178`, `addi 7`, then store: `+0x12DC = [+0x178] + 7` |

Implementation note: the Ghidra C `param_2[0x18]` for `0x80049C70` correctly
means `[hit + 0x60]`; the Rust uses the actor-vtable helper because it performs
exactly that load before passing `hit` in `r3`.

The remaining critical branches were confirmed to match the Ghidra C after
decoding the `bc` bits:

* `0x80049540`: `bc 4,2` (`bne`) sends a non-zero `0x8005E500` result to the
  `0x80048E10` path.
* `0x80051590`: `bc 12,1` (`bgt`) enters state 2 when the target limit is below
  the player Y; `bc 4,0` (`bge`) returns when the new Y is at or above ground.
* `0x80053120`: the `cmpi`/`bne` pair resets the 180-frame timer when a target
  is present and increments it when absent; `bc 4,1` (`ble`) selects `1` when
  scene X is at or below the player X, and the same `ble` sends timers 3 and 4
  to mask `0x100`.
* `0x80049C70`: `bc 4,0` (`bge`) accepts when the adjusted height plus
  `+0x1030` is at or below the queried ground height.

## Known gaps

* `// unknown:` the gameplay names of effect ids `0x07`, `0x11`, `0x12`,
  `0x13`, `0x17`, `0x56`, `0x57`, `0x59` and `0x8F`; only the vector address,
  third float and id are observed.
* `// unknown:` the meaning of the five model-effect records at
  `+0x148..+0x1384`; their release call `0x8008C380`, their `+0x30` mode field
  and their `+0x34` coordinate mask are exact, the gameplay label is not.
* `// unknown:` the roles of `0x80429F50` (a four-entry frame/actor array) and
  of the scene object at `0x80429E00`; the fields read here are exact.
* `// inferred:` the attack-pose labels `1`, `2`, `4`, `5`, `6`, the action
  states `5`/`0x0F`, and the sub-object value `0x14`; only the stores and
  compares are proven.
* `// partial:` `0x80049940` loads
  `r12 = 0x80307C54 + 12 * player[0x14A4]` before `0x802DCBFC`. `platform`
  cannot set `r12`; the visible `r3`/`r4` arguments are preserved.
* `0x8004C450` is a 20-byte tail dispatcher; its two callees were already
  translated in earlier waves, so only the dispatch itself is new.
