# Coverage - wave 30 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the thirtieth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table under *Translated functions* lists the 26
   bodies appended to `src/game/enemies.rs`, with their original addresses,
   Rust names, plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   continuation after the wave-29 `jugem` throw step, covering the
   `jump_step` spring platform and the next node family.
3. **What is still missing?** - *Deferred neighbours* names the damaged
   exports inside both runs and the next cluster after them.

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

| Metric | Before wave 30 | Wave 30 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 709 | 26 | 735 |
| Of those, `translated` | 690 | 26 | 716 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| Selected bytes | - | 4,204 | - |

The 26 new bodies cover 4,204 bytes (0.82 % of the 515,668 bytes that were
still untranslated before this pass; 4,204 / 515,668 = 0.82 %) and 26 of the
2,846 remaining function entries (0.91 %). Confidence is **18 high** and
**8 medium**. There are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 735 bodies, 716 `translated`, 15 `partial`,
2,820 function entries and 511,464 bytes remain untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 26 bodies were written before
   any ledger row was added. The wave-29 `jugem` block ends at `0x80A952D0`
   and was not touched.
2. **Continued at the wave-29 edge.** The first export after the `jugem` throw
   step is `0x80A959A0`. It opens a contiguous run of 28 exports up to
   `0x80A977C0`; 21 of them are complete bodies and are translated here.
3. **Identified the actor from its assets.** `0x80A968D0` names
   `g3d/jump_step.brres` (`0x80B51254`) and `jump_step` (`0x80B51268`), and
   `0x80A95D90` builds a `0x670`-byte object with two model/animation triples.
   The four player slots (`+0x5B0 + index * 4`, `+0x61C + index * 2`,
   `+0x624 + index * 2`), the falling-player snap in `0x80A96500` and the
   `+0x62E` air flag make the "spring platform that launches a rider" reading
   the only consistent one. It is still marked `// inferred:` in the prose
   because the export prints no class name.
4. **Read every numeric constant from the REL image.** Used here:
   `0x80B2ECD4 = 0.0`, `0x80B2ED34 = 8.0`, `0x80B2ED38 = 1.0`,
   `0x80B2ED3C = -0.125`, `0x80B2ED40 = 16.0`, `0x80B2ED44 = 2.0`,
   `0x80B2ED48 = 10.0`, `0x80B2EDE8 = -0.1875`, `0x80B2EDEC = -0.5`,
   `0x80B2EDF0 = (2.5, -2.5)`, `0x80B2EDF8 = 0.75`, `0x80B2EDFC = -1.5`,
   `0x80B2EE00 = 0.5`, `0x80B2EE04 = -0.8125`, `0x80B2EE08 = 9.0`,
   `0x80B51244 = 250.0`. The three `0x8006D090` blocks are kept as raw word
   blocks: `0x80B2ECD8` (three zeros), `0x80B2ECE8`
   (`0x80000001, 0xFFFFE000, 0x00002000, 0x00010000`) and `0x80B2ED00`
   (`0x00000001, 0x00003000, 0x0000D000, 0x00008000`).
5. **Kept damaged exports out.** `0x80A96230` and `0x80A969C0` are dominated by
   `GQR0` paired-single artefacts; `0x80A965D0` and `0x80A96DC0` take their
   receiver from the `0x802DCD6C`/`0x802DCD70` prologue fragments;
   `0x80A97180` uses the `2^52` unsigned-to-double idiom. None was forced into
   a guess; they are called by address where a translated body needs them.
6. **Closed the next node family.** `0x80A97EA0..0x80A98130` repeats the
   `0x80A959A0` callback/constant/walker/predicate shapes with complete bodies.
   They are translated so the gap before `0x80A98250` is closed, and their
   duplication is recorded in the ledger evidence.
7. **Left the four `0x802DCBFC` thunks out.** `0x80A95D00`, `0x80A95D30`,
   `0x80A981F0` and `0x80A98220` (48 B each) perform a hidden-`r12`
   this-adjustment that `platform::call3` cannot model; earlier waves record
   that shape as `partial`. Keeping them out leaves this pass fully translated.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A959A0` | `fn_80a959a0` | Node arm callback, returns the node | translated | high | `80a959a0_FUN_80a959a0.c` |
| `0x80A95A00` | `fn_80a95a00` | Second node arm callback | translated | high | `80a95a00_FUN_80a95a00.c` |
| `0x80A95A60` | `fn_80a95a60` | Constant producer `0x80357BF4` | translated | high | `80a95a60_FUN_80a95a60.c` |
| `0x80A95A70` | `fn_80a95a70` | Constant producer `0x80357BB4` | translated | high | `80a95a70_FUN_80a95a70.c` |
| `0x80A95B60` | `fn_80a95b60` | Five-step `+0x30` chain walker with recursion | translated | high | `80a95b60_FUN_80a95b60.c` |
| `0x80A95C40` | `fn_80a95c40` | Name-suffix predicate over `0x802E1C40`/`0x802E1AB4` | translated | medium | `80a95c40_FUN_80a95c40.c` |
| `0x80A95D60` | `fn_80a95d60` | Allocates `0x670` and runs the constructor | translated | medium | `80a95d60_FUN_80a95d60.c` |
| `0x80A95D90` | `fn_80a95d90` | `0x670`-byte `jump_step` constructor | translated | high | `80a95d90_FUN_80a95d90.c` |
| `0x80A95E70` | `fn_80a95e70` | Spawn initialiser from the `+0x4` flag word | translated | medium | `80a95e70_FUN_80a95e70.c` |
| `0x80A96050` | `fn_80a96050` | Per-frame step: timers, controller, contact probe | translated | medium | `80a96050_FUN_80a96050.c` |
| `0x80A96500` | `fn_80a96500` | Snaps a falling player onto the platform | translated | medium | `80a96500_FUN_80a96500.c` |
| `0x80A966F0` | `fn_80a966f0` | Tests the two ride state descriptors | translated | high | `80a966f0_FUN_80a966f0.c` |
| `0x80A967B0` | `fn_80a967b0` | Arms the `0x11` flash countdown | translated | high | `80a967b0_FUN_80a967b0.c` |
| `0x80A968D0` | `fn_80a968d0` | Binds `g3d/jump_step.brres` + `jump_step` | translated | high | `80a968d0_FUN_80a968d0.c` |
| `0x80A96990` | `fn_80a96990` | Resets one player slot | translated | high | `80a96990_FUN_80a96990.c` |
| `0x80A97110` | `fn_80a97110` | Yaw drift step with arrival latch | translated | high | `80a97110_FUN_80a97110.c` |
| `0x80A97370` | `fn_80a97370` | Detaches the player and clears the contact record | translated | high | `80a97370_FUN_80a97370.c` |
| `0x80A973D0` | `fn_80a973d0` | Ride step: facing copy, state test, squash | translated | medium | `80a973d0_FUN_80a973d0.c` |
| `0x80A97530` | `fn_80a97530` | Launch step: seeded speed, four slots cleared | translated | medium | `80a97530_FUN_80a97530.c` |
| `0x80A97650` | `fn_80a97650` | Air step with side flip and ground probe | translated | high | `80a97650_FUN_80a97650.c` |
| `0x80A977C0` | `fn_80a977c0` | Land step with lazy `(9.0, 2.0)` vector | translated | high | `80a977c0_FUN_80a977c0.c` |
| `0x80A97EA0` | `fn_80a97ea0` | Node arm callback of the next family | translated | high | `80a97ea0_FUN_80a97ea0.c` |
| `0x80A97F00` | `fn_80a97f00` | Second node arm callback of that family | translated | high | `80a97f00_FUN_80a97f00.c` |
| `0x80A97F60` | `fn_80a97f60` | Constant producer `0x80358104` | translated | high | `80a97f60_FUN_80a97f60.c` |
| `0x80A98050` | `fn_80a98050` | Chain walker, same body as `0x80A95B60` | translated | high | `80a98050_FUN_80a98050.c` |
| `0x80A98130` | `fn_80a98130` | Name predicate, same body as `0x80A95C40` | translated | medium | `80a98130_FUN_80a98130.c` |

## What the handlers do

### The `jump_step` platform (21 bodies)

`0x80A95D90` builds the `0x670`-byte object. It installs the class vtable
`0x80B51274` at `+0x60`, an owner pair at `+0x528` (vtable `0x80B51518`, back
pointer `0x52C`), a model holder at `+0x530`, a zeroed resource handle at
`+0x54C`, an animation holder at `+0x550`, the `0x8016A740` sub-object at
`+0x638` and the animation controller at `+0x654` (vtable `0x80317860`).
`0x80A95D60` is its allocator thunk.

`0x80A95E70` is the spawn initialiser. Bits 28..31 of the spawn flags at `+0x4`
become the mode `+0x5F0`, bits 8..11 the variant `+0x5F4`, and a non-zero
bits 4..7 nibble nudges X by `8.0`. It binds the model by calling `0x80A968D0`,
seeds both scale triplets to `8.0`, initialises the hit record with
`0x8006D090`, registers `+0x148` through `0x8008C3E0`/`0x8008C330`, clears the
four player slots and enters [`0x80B741B8`].

The frame cycle is four steps:

- `0x80A973D0` is the ride step. It copies the player's facing (via
  `0x80129910`) while the low two flags of `+0x12C` are clear and the player is
  not in state `4`; otherwise it enters `0x80B74238` with `-0.1875`. A contact
  record from `0x8006EF40` re-enters that state with the speed
  `0.0 / -0.5 / 0.0`.
- `0x80A97530` is the launch step. The side byte `+0x348` indexes the
  `2.5 / -2.5` table and `0.75` of the player's vertical speed is added, the
  target speed becomes `-1.5`, the air flag `+0x62E` is derived from the
  player's speed, all four slots are reset and the spawn Z is restored from
  `+0x600`.
- `0x80A97650` is the air step. Flags `0x3C000000` at `+0x274` select
  `-0.8125`; a clear `+0x62E` halves the speed and selects `2.0`; otherwise the
  platform zeroes its speed and returns to `0x80B741B8`. A set bit of
  `0x15 << side` flips `+0x348`, the vertical speed and the facing speed, then
  `0x80097170` probes the ground.
- `0x80A977C0` is the land step. Bit 0 of `0x80096E60` latches `+0x61A`,
  clears `+0x610` and stores `+0x440` into `+0x5D8`; the clear branch consumes
  that latch and restores `+0xE8`. It lazily builds the `(9.0, 2.0)` vector at
  `0x80B742AC` and calls the untranslated `0x80A969C0` with id `0x12`.

`0x80A96050` is the shared tick: it ages five counters, runs controller slot
`+0x10`, and for variant `1` saves the transform and runs controller slot
`+0x0C` with the `250.0` timer. Mode `0` calls `0x80064F50(actor, 0)`; every
other mode runs the `0x80064DA0` contact probe and retires the actor on a hit.

Around the cycle sit the helpers: `0x80A96500` snaps a falling player,
`0x80A966F0` tests the two ride states, `0x80A967B0` arms the `0x11` flash,
`0x80A96990` resets one slot, `0x80A97110` steps the yaw toward zero, and
`0x80A97370` detaches the player and clears `+0x274`. `0x80A959A0`/`0x80A95A00`
are the node callbacks, `0x80A95A60`/`0x80A95A70` are constant producers and
`0x80A95B60`/`0x80A95C40` are the chain walker and name predicate.

### The next node family (5 bodies)

`0x80A97EA0`/`0x80A97F00` repeat the two node callbacks, `0x80A97F60` returns
`0x80358104`, and `0x80A98050`/`0x80A98130` repeat the chain walker and name
predicate byte for byte. They belong to the sibling object that starts at
`0x80A98250`; no class name is printed for it.

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A95D00` | 48 B | Hidden-`r12` this-adjust thunk through `0x802DCBFC`. |
| `0x80A95D30` | 48 B | Same `0x802DCBFC` thunk. |
| `0x80A96230` | 572 B | Body is dominated by `GQR0`/`ldexpf` paired-single artefacts. |
| `0x80A965D0` | 280 B | Receiver comes from the `0x802DCD6C` prologue fragment. |
| `0x80A969C0` | 844 B | `GQR0` artefacts; called by address from `0x80A977C0`. |
| `0x80A96DC0` | 848 B | `0x802DCD70` prologue fragment and a 32-word unrolled table copy. |
| `0x80A97180` | 476 B | `2^52` unsigned-to-double idiom; called by address from two steps. |
| `0x80A981F0` | 48 B | Same `0x802DCBFC` thunk. |
| `0x80A98220` | 48 B | Same `0x802DCBFC` thunk. |

### Next cluster

| Address | Size | Note |
|---|---:|---|
| `0x80A98250` | 188 B | First export after this run; the sibling object's constructor path. |
| `0x80A98250..0x80A98EA0` | 1.7 KB | Eight more exports of the same sibling object (through `0x80A98EA0`). |

## Unknowns preserved in code

1. **Class and state ownership.** The export names neither the platform class
   nor the `StateID` that owns each step. The steps keep `fn_<address>` names
   and the four descriptors stay `STATE_80B741B8`, `STATE_80B741F8`,
   `STATE_80B74238` and the in-call `0x80B741B8`.
2. **Spawn flag nibbles.** `+0x5F0` (bits 28..31) and `+0x5F4` (bits 8..11)
   are named "mode" and "variant" from their later use only.
3. **`0x8006D090` parameter blocks.** `0x80B2ECD8`, `0x80B2ECE8` and
   `0x80B2ED00` are passed by address; the two bitfield blocks are not
   decoded because the callee is outside this export.
4. **`0x80064DA0`.** Its four arguments follow the call site; the return value
   is treated as "contact happened" from the retire branch.
5. **`0x80049A40`.** No `main.dol` export exists; the landing snap is modelled
   only from the `< 10.0` height guard.
6. **`0x8009EB10`.** No `main.dol` export exists; the receiver is passed in `r3`
   by the same convention as every other constructor.
7. **`0x80129910`.** Treated as the player's facing offset because its result is
   added to the player's `+0x102`; the callee is not exported.
8. **`0x80A969C0` / `0x80A97180`.** Both are damaged exports and are kept by
   address with `// unknown:` markers.
9. **`0x80B742AC..0x80B742B4`.** A BSS latch plus a two-word vector, written
   lazily by `0x80A977C0`; the owner of the vector is outside this export.
10. **`+0x364 = 8`.** The halfword has no visible reader in this pass.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`cargo check` succeeds. The remaining warnings are in other lanes' files
(`src/runtime/bt.rs` and `src/runtime/input.rs`) and were not touched here.
`node tools/doc-lint.mjs src` reports 0 violations across 68 files.

This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave30-enemies.csv`.
