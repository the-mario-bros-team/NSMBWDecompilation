# Coverage - Wave 103 (`game::player`, the `0x80050000` bucket)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 17 | 8720 | 1 | 14 | 2 |

## How to read this file

This document covers the Wave-103 slice of the `0x80050000` bucket of
`work/ghidra/main-gb/export/functions.tsv`. The bucket held 23 untranslated
functions when the pass started; 17 were reconstructed and 6 are listed under
"Deliberate skips" below, so the bucket is not empty after this wave. Every body
lives in `src/game/player.rs`; the ledger rows are in
`ledger/agents/wave103-player.csv`.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** - one plain sentence. Roles marked "inferred" come from shared field
  offsets and call shapes, not from a recovered symbol.
* **Status** - 15 rows are `translated`; the two `0x802DCBFC` forwarders are
  `partial` because the hidden `r12` this-adjust pointer cannot be modelled.
* **Confidence** - strength of the evidence, not code quality: `high` =
  instruction-level match, `medium` = a receiver, a field role or a callee
  argument had to be inferred, `low` = only the literal instruction sequence is
  certain.

The bodies use the two SDA bases this module already fixes: `r2 = 0x80433080`
and `r13 = 0x8042F680`. Every float in this pass was re-read from
`work/dol/main.elf`:

| address | value | use |
|---|---|---|
| `0x8042B878` | `0.0f` | zero constant (the wave-87 `W87_ZERO_8042B878`) |
| `0x8042B874` | `1.0f` | one constant (the wave-87 `W87_ONE_8042B874`) |
| `0x8042B8A8` | `-1.0f` | `+0x310` backstep |
| `0x8042B8B0` | `4096.0f` | fixed-point scale |
| `0x8042B8B8` | `0x4330000080000000` | signed-int-to-double bias |
| `0x8042B8C0` | `0.00390625` (`1/256`) | 16-bit angle scale |
| `0x8042B8C4` | `0.5f` | speed clamp and half height |
| `0x8042B8CC` | `2.5f` | absolute speed cap |
| `0x8042B8D8` | `-2.0f` | idle `+0xEC` |
| `0x8042B900` | `5.0f` | half-width clamp |
| `0x8042B958` | `20.0f` | default depth limit |
| `0x8042B9E0` | `0.01f` | step arm threshold |
| `0x8042B9E4` | `128.0f` | high clearance |
| `0x8042B9FC` | `64.0f` | special-mode depth limit |
| `0x8042BA00` | `-1420.0f` | hit-acceptance floor |
| `0x8042BA04` | `-0.5f` | lower speed clamp |
| `0x8042BA08` | `1.8f` | long step multiplier |
| `0x8042BA0C` | `-2.5f` | negative speed cap |
| `0x8042B840` | `1.0, -1.0, 3.628, 3.5` | per-facing side scale |
| `0x802EEB8C` | `0.6, 0.55, 0.5, 0.45, 0.4` | animation-rate table |

The `CONCAT44(0x43300000, value ^ 0x80000000) - 2^52` sequence that Ghidra
prints around several integer loads is the compiler's signed-int-to-double
idiom; it is reduced to `value as i32 as f32` in the bodies, matching the
`src/engine/display.rs` note for the same pattern.

## What this pass covers

- **The per-frame motion integrator, `0x80054F80..0x800573F0` (7 bodies).**
  [`fn_80054f80`] selects the vertical velocity `+0xEC` and the airborne cue
  (`0x272`, `0x273` or `0x281`, with `+0x1090 == 3` choosing the longer
  variant), then builds the `+0x14C8` step from `+0x310`. [`fn_80055330`] is the
  ground/water classifier: it advances the water level into `+0x113C`, calls
  `0x80075270` on `(x, y)`, refines codes `0`/`2` with [`fn_800552d0`], and ORs
  `0x8000`, `0x48000`, `0x14000`, `0x40000`, `0x4000` / `0x10000` into
  `+0x10D4`. [`fn_80055900`] latches a target `+0xAC` through the
  `sin/cos` ratio of the `0x80070920` slope. [`fn_80055ff0`] is the depth/edge
  band enforcer with the `20.0`/`64.0` limits and the `0x80082240` edge query.
  [`fn_80056620`] is the multi-hit acceptance predicate that turns `+0x3B0`
  request bits into `+0x3B4` accepted bits. [`fn_80056e40`] integrates the
  horizontal speed through `0x8015F480`, the `+/-5.0` clamp and the two-point
  `0x80075FD0` probe. [`fn_800571c0`] / [`fn_800573f0`] are the rotated-offset
  builders that end in `0x8006CD00`.
- **The effect-slot inserters, `0x80058B20..0x80058EE0` (2 bodies).** Both walk
  the owner's `+0x54`-counted `+0x74` array of eight-byte `{object, weight}`
  entries. [`fn_80058b20`] takes the first free entry and attaches through
  `0x8027B160`; [`fn_80058ee0`] first looks for an object whose `+0x9C` equals
  the key and then attaches through `0x8027B1C0`. Both stamp the `1.0` weight and
  post `0x80196190` / `0x80197400` only when the attach stored a live object.
- **The two `0x802DCBFC` forwarders, `0x80059260` / `0x80059290` (2 bodies).**
  Byte-identical 48-byte argument reshuffles: `0x802DCBFC(second, second,
  first)`. Marked `partial` because the pointer-to-member thunk reads a hidden
  `r12` this-adjust word that `platform::call3` cannot set - the same limitation
  wave 97 recorded for `0x800408F0..0x8004DB00`.
- **The spawn/HUD bookkeeping leaves, `0x8005BFD0..0x8005F5C0` (6 bodies).**
  [`fn_8005bfd0`] sets or clears bit `0x10` of the caller's `+0x10` word from a
  four-slot scan of events `0x65`/`0x6C`. [`fn_8005c830`] is the repeated-digit
  page check. [`fn_8005d280`] publishes the four ordered spawn slots, appending
  the free ones at pseudo-random positions from `0x800B2EE0` and copying the
  result to `0x80354E30`. [`fn_8005e040`] pushes one sample into the two
  ten-word history windows at `+0x09..+0x12` and `+0x31..+0x3A`; its twenty
  shift pairs are what finally pins the two window lengths wave 98 left open.
  [`fn_8005f5c0`] is the per-frame slot/target/input update: render-target
  refresh, the four `0x80354EB0` countdowns, the `0x8B`/`0x8C` cue pass, the
  page byte sync and the two tail toggles.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80054F80` | `fn_80054f80` | 820 | medium |
| `0x80055330` | `fn_80055330` | 512 | high |
| `0x80055900` | `fn_80055900` | 252 | medium |
| `0x80055FF0` | `fn_80055ff0` | 880 | medium |
| `0x80056620` | `fn_80056620` | 720 | medium |
| `0x80056E40` | `fn_80056e40` | 888 | medium |
| `0x800571C0` | `fn_800571c0` | 552 | medium |
| `0x800573F0` | `fn_800573f0` | 600 | medium |
| `0x80058B20` | `fn_80058b20` | 236 | medium |
| `0x80058EE0` | `fn_80058ee0` | 332 | medium |
| `0x80059260` | `fn_80059260` | 48 | low |
| `0x80059290` | `fn_80059290` | 48 | low |
| `0x8005BFD0` | `fn_8005bfd0` | 192 | medium |
| `0x8005C830` | `fn_8005c830` | 256 | medium |
| `0x8005D280` | `fn_8005d280` | 1064 | medium |
| `0x8005E040` | `fn_8005e040` | 624 | medium |
| `0x8005F5C0` | `fn_8005f5c0` | 696 | medium |

Byte total: 8720. Address range covered: `0x80054F80..0x8005F5C0`
(non-contiguous; the gaps are the deliberate skips below).

## Deliberate skips and why

Six of the 23 bucket functions stay untranslated. None is claimed to be
unreconstructable; each needs evidence this pass could not obtain honestly.

* **`0x8005EF50` (1408 B)** - the four-player spawn-position routine. Its
  prologue reads the incoming `f31` and `ps31` registers
  (`local_8 = (float)in_f31` / `fStack_4 = in_ps31_1`), so the true argument
  list includes a float pair the export never binds to a parameter. Writing the
  body would invent that ABI.
* **`0x8005B8A0` (1100 B)** - same defect: `in_f30`/`in_f31`/`in_ps30_1`/
  `in_ps31_1` are read before any store, and the body carries a 22-word stack
  array whose element type the export never resolves.
* **`0x800583A0` (1200 B)** - the `+0x1FC..+0x204` pose/aim routine (it reads
  the ground object at `0x80429D50`, calls `0x80069270`/`0x800693E0`, then
  `0x801611C0` and `0x8015F200`). Its
  `local_c8 = (float)CONCAT22((short)iVar7, ...)` mixes a float and an integer
  view of the same stack slot across the two `0x801611C0` calls, so the
  `+0x1FC..+0x204` angle triple cannot be written without guessing which view is
  live.
* **`0x800544F0` (2540 B)** - the largest remaining body. It is a long
  state-flagged chain that re-enters `0x800573F0`/`0x800552D0`; reconstructing
  it needs a second pass over its 40-odd branches, which the budget did not
  allow. Listed as **not attempted**, not as unreconstructable.
* **`0x800569C0` (496 B)** - the wall probe. Its two receivers come from
  `vtable[+0xF0]` and `vtable[+0xF4]`; the first is printed with **no argument
  at all** (`iVar3 = (**(code **)(...+0xf0))();`), so the object it is called on
  is not in the export. The four candidate points are then built from
  `(double)(CONCAT44(0x43300000, word) ^ 0x80000000) - 2^52`, where the XOR is
  printed on the 64-bit concat rather than on the word, leaving both the sign
  and the scale of the dequantised position ambiguous. Guessing either would
  produce a wrong probe.
* **`0x800592C0` (7620 B)** - the bucket's largest function. Its 1900-line
  export is a dispatch table over the same `+0x120C` stage bytes used by
  [`fn_80056620`]; it is a whole subsystem of its own and was out of scope for a
  single lane budget.

## Verification

- Every address and byte size matches `functions.tsv` (checked row by row
  against `work/ghidra/main-gb/export/functions.tsv`; the sizes are the ones
  printed in each `# Original` block).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/game/player.rs`.
- No address was already in `ledger/functions.csv`; all 23 candidates were
  listed as untranslated before any body was written.
- `cargo check --message-format=short` is free of errors and warnings for
  `src/game/player.rs` (the remaining workspace errors belong to other Wave-103
  lanes).
- `node tools/check-lane-csv.mjs wave103` reports `OK` with 17 rows in this lane.
