# Coverage - wave 31 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the thirty-first translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table under *Translated functions* lists the 28
   bodies appended to `src/game/enemies.rs`, with their original addresses,
   Rust names, plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   continuation after the wave-30 node family at `0x80A98130`, covering the
   `pukupuku` model actor and the clean half of the `killer_rot_houdai`
   platform.
3. **What is still missing?** - *Deferred neighbours* names the damaged
   exports left in both blocks and the next cluster after them.

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

| Metric | Before wave 31 | Wave 31 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 735 | 28 | 763 |
| Of those, `translated` | 716 | 28 | 744 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| Selected bytes | - | 4,388 | - |

The 28 new bodies cover 4,388 bytes (0.86 % of the 511,464 bytes that were
still untranslated before this pass; 4,388 / 511,464 = 0.86 %) and 28 of the
2,820 function entries that were untranslated before it (0.99 %). Confidence
is **24 high** and **4 medium**. There are no `stub` rows and no new `partial`
rows.

Cumulative after this pass: 763 bodies, 744 `translated`, 15 `partial`,
2,792 function entries and 507,076 bytes remain untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 28 bodies were written before
   any ledger row was added. The wave-30 block ends at `0x80A98130`/`0x80A98220`
   and was not touched.
2. **Continued at the wave-30 edge.** The first export after the wave-30
   thunks is `0x80A98250`, a `0x608`-byte constructor. It binds
   `g3d/pukupuku.brres` (`0x80B516F4`), the model name `pukupuku`
   (`0x80B51708`) and the clip `swim` (`0x80B51714`), and installs the class
   vtable `0x80B51720`. The block runs through the destructor `0x80A99D30`.
3. **Followed the state table that names this family.** The state descriptors
   used by the steps (`0x80B742C8`, `0x80B74308`, `0x80B74348`, `0x80B74388`)
   are the `daEnJumpPuku_c` records installed by
   [`register_jump_puku_states`] (`0x80A99DC0`, already translated). The seven
   helper exports `0x80A9A430..0x80A9A6E0` that belong to that class were still
   untranslated, so they close the gap between the actor and the next block.
4. **Selected the clean half of the `killer_rot_houdai` platform.** The
   `0xC90`-byte constructor `0x80A9A800` and its slot helpers
   `0x80A9AB50..0x80A9B870` have complete, ordinary bodies. The model binder
   `0x80A9B070`, the slot writer `0x80A9B1B0`, the release pair
   `0x80A9B280`/`0x80A9B310` and the state step `0x80A9B3C0` are kept out
   because they take their receiver from a `0x802DCDxx` prologue fragment or
   use the hidden-`r12` helper `0x802DCBFC`.
5. **Read every numeric constant from the REL image.** Used here:
   `0x80B2EE90` (`2.0, -2.0, 3.0, -3.0, 4.0, -4.0, 1.0, -1.0`),
   `0x80B2EEB0 = 1.0`, `0x80B2EEB4 = 0.0`, `0x80B2EEC4 = 3.0`,
   `0x80B2EEC8 = -4.0`, `0x80B2EECC = -0.1875`, `0x80B2EED4/8 = 0xFFFFFFFF`,
   `0x80B2EF40 = 12.0`, `0x80B2EF44 = -1.5`, `0x80B2EF4C = -4.0`,
   `0x80B2EF54 = -0.1875`, `0x80B2EF68 = 0.125`, `0x80B2EF6C = 0.25`,
   `0x80B2EF70 = 0.375`, `0x80B2EF84 = 4.875`, `0x80B2EF8C = 7.0`,
   `0x80B2EFA8 = (2.5, -2.5)`, `0x80B2EFB0 = (0.5, -0.5)` and the two
   `i16` targets `0x4000/0xC000` at `0x80B2EFDC`. The `2^52` idiom is kept as
   the signed table value it computes, with a `// observed:` note.
6. **Kept damaged exports out.** `0x80A98950` and `0x80A98C10` take their
   receiver from the `0x802DCD74` prologue fragment, and `0x80A99360` is
   dominated by `GQR0` paired-single artefacts. None was forced into a guess;
   the two state entries call the surviving pieces by address where needed.
7. **Marked the four inferred ABI details.** `0x80A99A10`, `0x80A9A6E0`,
   `0x80A9AFF0` and `0x80A9BE80` print a call without its receiver or inherit
   the string-helper roles from the earlier copies; they are `medium` and
   carry `// inferred:` notes at the call sites.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A98250` | `fn_80a98250` | `0x608`-byte pukupuku actor constructor | translated | high | `80a98250_FUN_80a98250.c` |
| `0x80A98420` | `fn_80a98420` | Dispose callback: virtual `+0x10`, hide actor | translated | high | `80a98420_FUN_80a98420.c` |
| `0x80A98780` | `fn_80a98780` | Hit record from side table and attacker class | translated | high | `80a98780_FUN_80a98780.c` |
| `0x80A98DC0` | `fn_80a98dc0` | Height above the stage ground | translated | high | `80a98dc0_FUN_80a98dc0.c` |
| `0x80A98DF0` | `fn_80a98df0` | Hit record from four `f64` coordinates | translated | high | `80a98df0_FUN_80a98df0.c` |
| `0x80A98EA0` | `fn_80a98ea0` | `Swim`/`JumpRear` entry with height gate | translated | high | `80a98ea0_FUN_80a98ea0.c` |
| `0x80A990F0` | `fn_80a990f0` | Frame wrapper: base updates then `Swim` entry | translated | high | `80a990f0_FUN_80a990f0.c` |
| `0x80A991F0` | `fn_80a991f0` | `Swim` step: gravity, turn, timer | translated | high | `80a991f0_FUN_80a991f0.c` |
| `0x80A99460` | `fn_80a99460` | Jump step: pitch reset and turn | translated | high | `80a99460_FUN_80a99460.c` |
| `0x80A99580` | `fn_80a99580` | `JumpWait` step: constant `7.0` turn, timer | translated | high | `80a99580_FUN_80a99580.c` |
| `0x80A996D0` | `fn_80a996d0` | `JumpRear` step: turn, ground probe, sound `0x1DB` | translated | high | `80a996d0_FUN_80a996d0.c` |
| `0x80A99A10` | `fn_80a99a10` | Move entry: class byte, per-side speeds, bounds | translated | medium | `80a99a10_FUN_80a99a10.c` |
| `0x80A99D30` | `fn_80a99d30` | Destructor: four holder releases and free | translated | high | `80a99d30_FUN_80a99d30.c` |
| `0x80A9A430` | `fn_80a9a430` | Node arm callback (normal states) | translated | high | `80a9a430_FUN_80a9a430.c` |
| `0x80A9A490` | `fn_80a9a490` | Node arm callback (death states) | translated | high | `80a9a490_FUN_80a9a490.c` |
| `0x80A9A4F0` | `fn_80a9a4f0` | Constant producer `0x80358104` | translated | high | `80a9a4f0_FUN_80a9a4f0.c` |
| `0x80A9A500` | `fn_80a9a500` | Constant producer `0x80358044` | translated | high | `80a9a500_FUN_80a9a500.c` |
| `0x80A9A510` | `fn_80a9a510` | Constant producer `0x80357D74` | translated | high | `80a9a510_FUN_80a9a510.c` |
| `0x80A9A600` | `fn_80a9a600` | Five-step `+0x30` chain walker with recursion | translated | high | `80a9a600_FUN_80a9a600.c` |
| `0x80A9A6E0` | `fn_80a9a6e0` | Name-suffix predicate over `0x802E1C40`/`0x802E1AB4` | translated | medium | `80a9a6e0_FUN_80a9a6e0.c` |
| `0x80A9A800` | `fn_80a9a800` | `0xC90`-byte `killer_rot_houdai` constructor | translated | high | `80a9a800_FUN_80a9a800.c` |
| `0x80A9AB50` | `fn_80a9ab50` | Slot teardown and hide/retire tail | translated | high | `80a9ab50_FUN_80a9ab50.c` |
| `0x80A9AF00` | `fn_80a9af00` | Release `+0x904` holder, return `1` | translated | high | `80a9af00_FUN_80a9af00.c` |
| `0x80A9AF50` | `fn_80a9af50` | Clear one eight-word slot record | translated | high | `80a9af50_FUN_80a9af50.c` |
| `0x80A9AF80` | `fn_80a9af80` | First free sub-slot query | translated | high | `80a9af80_FUN_80a9af80.c` |
| `0x80A9AFF0` | `fn_80a9aff0` | Four-step slot-handle expiry sweep | translated | medium | `80a9aff0_FUN_80a9aff0.c` |
| `0x80A9B870` | `fn_80a9b870` | Slot toggle: side bit, angle target, descriptor | translated | high | `80a9b870_FUN_80a9b870.c` |
| `0x80A9BE80` | `fn_80a9be80` | Frame update callback, returns `1` | translated | medium | `80a9be80_FUN_80a9be80.c` |

## What the handlers do

### The `pukupuku` actor (13 bodies)

`0x80A98250` allocates `0x608` bytes and builds the actor that binds
`g3d/pukupuku.brres` and the `pukupuku`/`swim` clips. The class vtable
`0x80B51720` goes to `+0x60`; the `+0x524`/`+0x544`/`+0x584` holders and the
`+0x5C0` pair (`0x803295D8`, then `0x80329548` after `0x8016A5E0`) follow.
`0x80A98420` is its dispose callback: the `+0x394` sub-object receives its
virtual `+0x10` slot and `0x80064F50(actor, 0)` hides the actor.

The two hit-record writers share an eleven-word layout at `+0x414..+0x432`.
`0x80A98780` takes the first word from the per-side table `0x8042C1B0`, the
next six from `0x80B2EEC4..0x80B2EED8`, and the class byte from the attacker's
vtable `+0x6C`. `0x80A98DF0` takes the four float words as `f64` arguments and
uses the constant block `0x80B2EF30..0x80B2EF38` (`0, -1, -1`) instead.
`0x80A98DC0` is the height query: `0x80429E00[+0x38]` minus
`0x80429DD0[+0x8FEA8]`.

The state cycle is four entries plus a wrapper:

- `0x80A98EA0` is the common entry. It refuses to act while the actor is at
  most `12.0` above the ground. With `mode 0` it writes the entry speed
  (`0x80B2EE90[side, +0x5FC]`, or `0x8042A100 + 0x8C0` when `+0x5F0` is set),
  sets `+0xEC = -1.5`, enters `0x80B742C8` (`Swim`) and plays sound `0x1DC`
  from the vector `(+0xAC, ground, +0xB4)`. A non-zero mode enters
  `0x80B74348` (`JumpRear`) directly.
- `0x80A991F0` is the `Swim` step. Gravity `+0xEC` rises by `0.125` to the
  `0.25` cap, or the actor is snapped to `ground - 12.0`. A `|+0xE8|` above
  `0.375` turns the facing through `0x801611C0`/`0x8015F2E0`; the `+0x5F8`
  timer then switches to `0x80B74308` (`JumpWait`).
- `0x80A99460` is the jump step. It first asks `0x80A98EA0(actor, 1)`; when
  that returns `0`, `+0x118` is reset to `4.875` if it equals `+0xEC`, the
  facing turns with the `0x10`/`0x1000` mask, and the `+0x544` holder updates.
- `0x80A99580` is `JumpWait`: the angle helper receives the constant `7.0` and
  `+0xE8 * side`, uses the `0xC`/`0x5000` mask, and switches to `0x80B74388`
  (`Jump`) when `+0x5F8` reaches zero.
- `0x80A996D0` is `JumpRear`: the same turn, then a ground probe that plays
  sound `0x1DB` once (latching `+0x601`); a negative `+0xEC` returns to
  `0x80A98EA0(actor, 0)`.
- `0x80A990F0` is the trivial wrapper: `0x8006CDE0`, `0x800955F0`, then the
  `Swim` entry.

`0x80A99A10` is the move entry. It resolves the child at `+0x368`, copies the
first byte of its vtable `+0x6C` name to `+0x38D`, and selects the ground
`(2.5, -2.5)` or airborne `(0.5, -0.5)` speed pair. It also copies the
`0x8042C1A4` bound to `+0x102`, sets `+0x460 = 5`, clears `+0x462`, arms the
`+0x5F4` timer with `0x3C`, and writes `+0xF8 = -4.0`/`+0x114 = -0.1875`.
`0x80A99D30` releases the four holders, runs the base destructor and frees the
storage when asked.

### The `daEnJumpPuku_c` helpers (7 bodies)

`0x80A9A430`/`0x80A9A490` are the eighth and ninth copies of the
`0x8015F800`/`0x802B9280` node-arm callback. `0x80A9A4F0`, `0x80A9A500` and
`0x80A9A510` return the three `main.dol` pointers `0x80358104`, `0x80358044`
and `0x80357D74` that the death states carry as extra callbacks.
`0x80A9A600` is the five-step `+0x30` chain walker and `0x80A9A6E0` the
name-suffix predicate; both repeat the bodies already used at `0x80A98050`
and `0x80A98130`.

### The `killer_rot_houdai` platform (8 bodies)

`0x80A9A800` allocates `0xC90` bytes, installs the vtable `0x80B51D10` and
builds four `0x802DC9A0` containers: `+0x544` (`0x40` bytes, 8 elements),
`+0x744` (`0x38`, 8), `+0xB64` (`0x0C`, 8) and `+0xBC4` (`0x0C`, 8), plus the
`0x8007F7A0` holder at `+0x904`. `0x80A9AB50` is its teardown: both sides of
every `+0xC81` slot are swept, the holders are released, `+0xC84` is cleared
and the actor is hidden or retired. `0x80A9AF00` releases `+0x904`.

The four slot helpers address the record at `+0xA04 + index * 0x20`:
`0x80A9AF50` zeroes its eight words, `0x80A9AF80` returns the first free
sub-slot (`0..3`, or `-1`), `0x80A9AFF0` clears handles that `0x80162D00`
cannot resolve, and `0x80A9B870` flips the side bit `1 << slot`, compares the
`+0xC38` angle with the `0x4000`/`0xC000` targets of `0x80B2EFDC`, and either
writes the `0x80B51D04` descriptor or advances the angle by
`0x8042C1A8[bit]`. `0x80A9BE80` is the small `0x80097D30` frame callback.

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A98950` | 280 B | Receiver comes from the `0x802DCD74` prologue fragment. |
| `0x80A98C10` | 424 B | Same `0x802DCD74` receiver; model/clip binder. |
| `0x80A99360` | 476 B | Body is dominated by `GQR0`/`ldexpf` paired-single artefacts. |
| `0x80A9A7A0` | 48 B | Hidden-`r12` this-adjust thunk through `0x802DCBFC`. |
| `0x80A9A7D0` | 48 B | Same `0x802DCBFC` thunk. |
| `0x80A9B070` | 312 B | Receiver comes from the `0x802DCD64` prologue fragment. |
| `0x80A9B1B0` | 208 B | Hidden-`r12` helpers `0x802DCBC0`/`0x802DCB90`/`0x802DCBFC`. |
| `0x80A9B280` | 48 B | Hidden-`r12` this-adjust through `0x802DCBFC`. |
| `0x80A9B310` | 164 B | `0x802DCD6C` receiver plus the same hidden-`r12` dispatch. |
| `0x80A9B3C0` | 1,196 B | `0x802DCD70` receiver, `GQR0` block and a 32-word table copy. |

### Next cluster

| Address | Size | Note |
|---|---:|---|
| `0x80AA0080` | 28 B | First export after this run; the `killer_rot_houdai` state-table/callback cluster starts here. |
| `0x80AA0080..0x80AA0xxx` | - | The next contiguous export family after the deferred `killer_rot_houdai` helpers. |

## Unknowns preserved in code

1. **Class names.** The export prints no class name for the `0x80A98250`
   actor; its role is inferred from `g3d/pukupuku.brres`, `pukupuku` and
   `swim`. The constructor keeps a `fn_<address>` name.
2. **`0x80A99A10` accessor.** Ghidra prints the child's vtable `+0x6C` call
   without an argument; the child at `+0x368` is passed in `r3` by the method
   convention (`// inferred:`).
3. **`0x80A9AFF0` handle query.** `0x80162D00` is printed without its
   argument; the `+0xA04` handle is used as the receiver (`// inferred:`).
4. **`0x80A9BE80` receiver.** The export declares the callback `void` with no
   parameter; the actor is passed in `r3` (`// inferred:`).
5. **`0x8042C1B0`.** The per-side word table copied into the hit record has no
   export in this REL; only its address and index are known.
6. **`+0x5F0` mode and `+0x5FC` row.** Both are named from their use in the
   `0x80B2EE90` index only; the writer of those fields is outside this pass.
7. **`0x8042A100 + 0x8C0`.** The object behind the pointer is a `main.dol`
   global; the word is read as printed.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`cargo check` succeeds. The remaining warnings are in other lanes' files
(`src/runtime/bt.rs` and `src/runtime/input.rs`) and were not touched here.
`node tools/doc-lint.mjs src` reports 0 violations across 68 files.

This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave31-enemies.csv`.
