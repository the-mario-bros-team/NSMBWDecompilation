# Coverage - wave 29 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twenty-ninth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   continuation after the wave-28 `jugem`/`cloud_wait` binder, through the
   fourteen frame steps of the rideable carrier and into the separate
   `jugem` platform actor.
3. **What is still missing?** - *Deferred neighbours* names the damaged
   exports skipped inside both clusters and the next cluster after them.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = the body is represented but a hidden ABI operand
  prevents a complete model; `stub` = address only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role or an ABI detail is
  inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_enemiesNP/export/`; that C file
  is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`; the entry
point is `0x80A000F0`. The REL has no embedded load address, and the retail
loader may place it anywhere. These are **not** runtime addresses.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 29 | Wave 29 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 679 | 30 | 709 |
| Of those, `translated` | 660 | 30 | 690 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| Selected bytes | - | 5,788 | - |

The 30 new bodies cover 5,788 bytes (1.11 % of the 521,456 bytes that were
still untranslated before this pass; 5,788 / 521,456 = 1.11 %) and 30 of the
2,876 remaining function entries (1.04 %). Confidence is **23 high** and
**7 medium**. There are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 709 bodies, 690 `translated`, 15 `partial`,
2,846 function entries and 515,668 bytes remain untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added. The wave-28 `fn_80a920d0` binder and the
   `daEnJimenFPakkun_c` state table at `0x80A90740` were already translated
   and were not touched.
2. **Continued at the wave-28 edge.** Wave 28 ends at the
   `jugem`/`cloud_wait` binder `0x80A920D0`; the next export is
   `0x80A921F0`. That address starts a contiguous run of fourteen complete
   bodies up to `0x80A92ED0`; the next export, `0x80A92F40` (1,172 bytes), is
   deferred for the wave-28 prologue-fragment reason.
3. **Identified the two clusters from their data.**
   - The `0x80A921F0` block never prints a class name. It resolves a player
     index through `0x8005F900`/`0x8005FA60`, copies that player's transform
     in `0x80A92E10` and releases it in `0x80A92ED0`. That is only consistent
     with a rideable carrier, which is therefore `// inferred:`.
   - The `0x80A93990` block binds `g3d/jugem.brres` (`0x80B50DA8`) with the
     model `jugem` (`0x80B50DB8`) and the five clips stored at
     `0x80B50DC8..0x80B50DF0` (`wait`, `throw_st`, `throw_wait`, `throw`,
     `dead`); the pointer table at `0x80B50DF8` indexes them.
4. **Read every numeric constant from the REL image.** Used here:
   `0x80B2EA78 = 0.0`, `0x80B2EA7C = 1.25`, `0x80B2EAA4 = (-0.0, 0.0, 0.0)`,
   `0x80B2EAB0 = (2.0, -2.0)`, `0x80B2EAE4 = 24.0`, `0x80B2EAE8 = 16.0`,
   `0x80B2EB00 = 0.1`, `0x80B2EB04 = 0.5`, `0x80B2EB08 = 0.0703125`,
   `0x80B2EB0C = 2048.0`, `0x80B2EB10 = 0.0078125`,
   `0x80B2EB14 = -2048.0`, `0x80B2EB18 = 0.0234375`,
   `0x80B2EB1C = 0.015625`, `0x80B2EB20 = 0.0625`, `0x80B2EB24 = 0.075`,
   `0x80B2EB28 = -1.5`, `0x80B2EB70..0x80B2EB7C = (0.0, 8.0, 8.0, 8.0)`,
   `0x80B2EBAC = 0.0`, `0x80B2EC34 = 1.0`, `0x80B2EC38 = (0, 1, 0, 1, 0)`,
   `0x80B2EC6C = 16.0`, `0x80B2EC70 = 14.0`, `0x80B2EC74 = 1.24`.
5. **Kept damaged exports out.** `0x80A92F40` reads its receiver from
   `0x802DCD74` and uses `GQR0`-derived paired-single arithmetic that the
   export cannot express; `0x80A933E0` is `halt_baddata`; `0x80A94100`
   returns a 64-bit prologue fragment (`0x802DCD6C`) whose two halves are the
   actor and a second word; `0x80A94D10` is made of paired-single artefacts.
   None was forced into a guess; see *Deferred neighbours*.
6. **Reused the established shapes.** `fn_80a949b0` repeats the wave-28
   `fn_80a920d0` binder call for call, `fn_80a92320` is the same
   `0x80064740` contact wrapper already used by `fn_80ae4f10`, and
   `fn_80a95150`/`fn_80a94bf0` use the `+0x394` owner-state recipe already
   present in the file.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A921F0` | `fn_80a921f0` | Player/reference side test selects the `+0xF4` facing word | translated | high | `80a921f0_FUN_80a921f0.c` |
| `0x80A922C0` | `fn_80a922c0` | `+0x708` countdown, `+0x722` flags, `+0x124` mirror | translated | high | `80a922c0_FUN_80a922c0.c` |
| `0x80A92320` | `fn_80a92320` | Contact query wrapper around `0x80064740` | translated | medium | `80a92320_FUN_80a92320.c` |
| `0x80A923B0` | `fn_80a923b0` | Yaw step from two proximity words | translated | high | `80a923b0_FUN_80a923b0.c` |
| `0x80A92450` | `fn_80a92450` | `+0x6F8 = 16 * (level + 10) - entry[+2] - entry[+6]` | translated | medium | `80a92450_FUN_80a92450.c` |
| `0x80A925C0` | `fn_80a925c0` | Rise phase, then contact follow | translated | high | `80a925c0_FUN_80a925c0.c` |
| `0x80A92710` | `fn_80a92710` | Dive arming and sound `0x1B8` | translated | high | `80a92710_FUN_80a92710.c` |
| `0x80A92800` | `fn_80a92800` | Dive loop with `+0x6FC` clamp and `1.25` cap | translated | high | `80a92800_FUN_80a92800.c` |
| `0x80A929B0` | `fn_80a929b0` | Contact class cache into `+0x1EC`, aim reset | translated | high | `80a929b0_FUN_80a929b0.c` |
| `0x80A92A50` | `fn_80a92a50` | Follow step with the two attack branches | translated | high | `80a92a50_FUN_80a92a50.c` |
| `0x80A92BC0` | `fn_80a92bc0` | Follow step without the attack branch | translated | high | `80a92bc0_FUN_80a92bc0.c` |
| `0x80A92CA0` | `fn_80a92ca0` | Descent damping, `0.075` drift, pitch levelling | translated | medium | `80a92ca0_FUN_80a92ca0.c` |
| `0x80A92E10` | `fn_80a92e10` | Copies the player transform onto the carrier | translated | high | `80a92e10_FUN_80a92e10.c` |
| `0x80A92ED0` | `fn_80a92ed0` | Detaches the player, sets the `0xFF` sentinel | translated | high | `80a92ed0_FUN_80a92ed0.c` |
| `0x80A93990` | `fn_80a93990` | Node arm helper, returns its argument | translated | high | `80a93990_FUN_80a93990.c` |
| `0x80A93B10` | `fn_80a93b10` | `0x830`-byte `jugem` constructor, vtable `0x80B50E10` | translated | high | `80a93b10_FUN_80a93b10.c` |
| `0x80A93E20` | `fn_80a93e20` | Release thunk, returns `1` | translated | medium | `80a93e20_FUN_80a93e20.c` |
| `0x80A93FB0` | `fn_80a93fb0` | Effect spawn/stop, then state `0x80B74134` | translated | high | `80a93fb0_FUN_80a93fb0.c` |
| `0x80A940A0` | `fn_80a940a0` | `dead` clip, unlink `+0x148`, forward release | translated | high | `80a940a0_FUN_80a940a0.c` |
| `0x80A947F0` | `fn_80a947f0` | Snap to the owner transform | translated | high | `80a947f0_FUN_80a947f0.c` |
| `0x80A949B0` | `fn_80a949b0` | `g3d/jugem.brres` + `jugem` + `wait` binder | translated | high | `80a949b0_FUN_80a949b0.c` |
| `0x80A94AD0` | `fn_80a94ad0` | Five-clip selector, writes `+0x818` | translated | high | `80a94ad0_FUN_80a94ad0.c` |
| `0x80A94B70` | `fn_80a94b70` | Retires the `+0x7F4` player handle | translated | high | `80a94b70_FUN_80a94b70.c` |
| `0x80A94BF0` | `fn_80a94bf0` | Owner-state test and `+0x820` retry timer | translated | medium | `80a94bf0_FUN_80a94bf0.c` |
| `0x80A94EB0` | `fn_80a94eb0` | Three-state throw machine over `+0x818` | translated | medium | `80a94eb0_FUN_80a94eb0.c` |
| `0x80A950C0` | `fn_80a950c0` | Four tilt words from the speed triplet | translated | high | `80a950c0_FUN_80a950c0.c` |
| `0x80A95120` | `fn_80a95120` | Pitch step `-0x44C` at `+0x100` | translated | high | `80a95120_FUN_80a95120.c` |
| `0x80A95150` | `fn_80a95150` | Main step: follow, tilt, fade, release | translated | medium | `80a95150_FUN_80a95150.c` |
| `0x80A95280` | `fn_80a95280` | Enters `throw_st`, clears `+0x824` | translated | high | `80a95280_FUN_80a95280.c` |
| `0x80A952D0` | `fn_80a952d0` | `throw_st` step: follow, machine, animation | translated | high | `80a952d0_FUN_80a952d0.c` |

## What the handlers do

### The carrier block (14 bodies)

The carrier is a rideable object (`// inferred:`), that a player can stand
on. `0x80A92710`
arms a dive: it re-derives the facing from the reference mid-line
(`0x80429E00 + 0x38 + 0x40 * 0.5`), sets the vertical speed to the negation
of that facing, seeds `+0x11C = 0.0703125`, `+0x700 = (2048 + z) / 128`,
drops `z` to `-2048` and plays sound `0x1B8` at its own matrix. `0x80A92800`
runs the dive: `+0xE8 == +0xF4` counts three frames, `+0x11C` decays to
`0.0234375`, the height moves by `+0x700` and is clamped to `+0x6FC`, and once
the horizontal speed also passes `1.25` the state `0x80B73FB8` is entered.

`0x80A925C0` is the opposite movement: `+0x718 == 0` is a rise phase that
fills `+0xDC` to `1.25`, after which the actor integrates its position, aims at
the contact and snaps to `+0x6F8` when the contact height crosses its own.

`0x80A92320` is the shared contact query: it forwards `0x80064740` and
returns the contact actor, with a two-word proximity block as an optional
output. `0x80A923B0` turns those two words into the `+0x102` yaw. `0x80A922C0`
is the shared countdown used by `0x80A92CA0`.

`0x80A92E10` and `0x80A92ED0` are the attach/detach pair: they resolve the
player index `+0x726` through `0x8005F900` or `0x8005FA60` (selected by
`+0x70C`), copy or release the player and write the `0xFF` sentinel.

### The `jugem` platform actor (16 bodies)

`0x80A93B10` allocates `0x830` bytes and builds two complete
model/animation/clip triples: `+0x528/+0x548/+0x588` and
`+0x72C/+0x74C/+0x78C`. It installs the base vtable `0x80B50938` (the
wave-28 cloud actor's vtable) first and the class vtable `0x80B50E10` last.

`0x80A949B0` binds the assets to the second triple, `0x80A94AD0` selects one of
the five clips and `0x80A950C0`/`0x80A95120` are the small tilt and pitch
steps. `0x80A947F0` keeps the actor glued to its owner's transform and
`0x80A95150` is the main per-frame step.

The throw sequence is `0x80A95280` (`throw_st`) -> `0x80A952D0` (the step) ->
`0x80A94EB0` (the state machine) -> `0x80A94BF0` (the wait gate) ->
`0x80A93FB0`/`0x80A940A0` (the release paths). `0x80A94EB0` spawns the carried
player object with `0x80A443A0` or `0x80B14D80`, links it through `+0x7F4`,
and releases it with `0x80A94D10`; `0x80A94B70` retires a stale handle.

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A92F40` | 1,172 B | Receiver comes from the `0x802DCD74` prologue fragment and the body is dominated by `GQR0` paired-single artefacts (`ldexpf`). |
| `0x80A933E0` | 36 B | Export is `halt_baddata`; no instruction flow. |
| `0x80A94100` | 408 B | Returns a 64-bit prologue fragment (`0x802DCD6C`) whose halves are the actor and a second word; the frame is not recoverable. |
| `0x80A94D10` | 404 B | The impulse vector is a stack `GQR3`/`ldexpf` block; kept by address in `fn_80a94eb0`. |

### Next cluster

| Address | Size | Note |
|---|---:|---|
| `0x80A959A0` | 88 B | First export after this run; the next clip/state helpers. |
| `0x80A959A0..0x80A95E70` | 1.9 KB | The remaining selector/state helpers of the same actor. |

## Unknowns preserved in code

1. **Carrier class name.** The export never prints a class name for the
   `0x80A921F0` block; "rideable carrier" is `// inferred:` from the attach and
   detach pair alone.
2. **`fn_80a92320` return value.** The callee is declared `void`, but three
   call sites dereference its result. The Rust returns `usize` and marks the
   recovery `// inferred:`.
3. **`fn_80a92ca0` hidden argument.** `0x80A922C0` is printed with no
   argument; `actor` is forwarded and marked `// inferred:`.
4. **`0x8006CF10`.** No `main.dol` export exists at the address, so the retry
   gate in `fn_80a94bf0` is modelled only from the `< 4` comparison.
5. **`0x8004A7B0`.** No `main.dol` export exists; the three-word store that
   follows the call is the only evidence for "copies the player transform".
6. **`0x80A443A0` / `0x80B14D80` / `0x80A94D10`.** All three are damaged
   exports. They are called by address with `// unknown:` markers; the
   `+0x9F8/+0x9FC` and `+0x9D4/+0x9D8` stores around them come from the
   export, not from their bodies.
7. **State descriptors.** `0x80B73FB8`, `0x80B73FF8`, `0x80B73F38`,
   `0x80B740B8`, `0x80B740F8` and `0x80B74134` live in BSS and are written by
   a class table outside this export, so their `StateID` names are `// unknown:`.
8. **Clip loop flags.** `0x80B2EC38` is `(0, 1, 0, 1, 0)` for clip indices
   `0..4`; the meaning of the flag beyond "loop" is `// unknown:`.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`cargo check` succeeds. The remaining warnings are in other lanes' files
(`src/runtime/bt.rs` and `src/runtime/input.rs`) and were not touched here.
`node tools/doc-lint.mjs src` reports 0 violations across 68 files.

This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave29-enemies.csv`.
