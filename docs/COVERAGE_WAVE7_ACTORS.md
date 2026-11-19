# Coverage — wave 7 actor/gameplay helpers

## How to read this file

This document tracks **one translation wave**, not an address range: the
twenty-five `main.dol` functions added in wave 7 to the semantic gameplay
modules `src/game/actor.rs`, `src/game/interaction.rs`, `src/game/path.rs`,
`src/game/yoshi.rs` and `src/game/boot_scene.rs`.

* **Address** — virtual entry point in the DOL image (image base `0x80004000`),
  written in plain hex without a `0x` prefix, matching `ledger/functions.csv`.
* **Name** — the Rust function name. Ghidra names are always `FUN_<address>`;
  descriptive Rust names keep the address in their `# Original` block.
* **Role** — one plain sentence. Roles marked *(inferred)* come from call
  sites or field use, never from a recovered symbol.
* **Status** — `translated` = full control flow, every call accounted for;
  `partial` = one machine-level operand is unrecoverable; `stub` = address and
  role only. Every row below is `translated`.
* **Confidence** — strength of evidence: `high` = instruction-level match,
  `medium` = one field or callee role guessed, `low` = unresolved.
* **Evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Status and confidence are mirrored in `ledger/agents/wave7-actors.csv`; the
same bodies also have rows in the master `ledger/functions.csv`.

## Scope and counts

| item | count |
|---|---|
| new bodies in this wave | 25 |
| translated bytes (sum of `functions.tsv` sizes) | 2,768 |
| modules touched | 5 |
| `partial` / `stub` rows | 0 / 0 |
| unresolved jump tables in the selected bodies | 0 |

By module:

| module | functions | bytes |
|---|---:|---:|
| `src/game/interaction.rs` | 7 | 544 |
| `src/game/path.rs` | 3 | 324 |
| `src/game/actor.rs` | 3 | 600 |
| `src/game/yoshi.rs` | 7 | 948 |
| `src/game/boot_scene.rs` | 5 | 352 |

## Translated functions

| address | Rust name | role (plain English) | status | conf. | module | evidence |
|---|---|---|---|---|---|---|
| `8014e700` | `fn_8014e700` | reset the interaction substate, clear status bit `0xB2`, release the secondary target | translated | high | `game/interaction.rs` | `8014e700_FUN_8014e700.c` |
| `8014e760` | `fn_8014e760` | call actor virtual `+0x378(wm, 0)` and enter interaction mode 1 | translated | high | `game/interaction.rs` | `8014e760_FUN_8014e760.c` |
| `8014e9b0` | `wm_secondary_target` | resolve the secondary target handle at `+0x1524` through the actor registry | translated | high | `game/interaction.rs` | `8014e9b0_FUN_8014e9b0.c` |
| `8014e9d0` | `fn_8014e9d0` | readiness predicate: bit `0x91` clear, bit 28 of `+0x10D4` clear, retries not exhausted | translated | high | `game/interaction.rs` | `8014e9d0_FUN_8014e9d0.c` |
| `8014ed50` | `fn_8014ed50` | re-enable secondary-path status when bit `0xB2` is set | translated | high | `game/interaction.rs` | `8014ed50_FUN_8014ed50.c` |
| `80150020` | `wm_play_select_sound` | play `0x15D` for type id `0x13`, otherwise `0xC0` | translated | high | `game/interaction.rs` | `80150020_FUN_80150020.c` |
| `801507c0` | `wm_target_accept` | guarded virtual `+0x8C` accept callback, folded to one bit | translated | high | `game/interaction.rs` | `801507c0_FUN_801507c0.c` |
| `8000d0a0` | `fn_8000d0a0` | store a three-word vector at `+0x48 + a*0x3390 + b*0xC` | translated | medium | `game/path.rs` | `8000d0a0_FUN_8000d0a0.c` |
| `8000efc0` | `fn_8000efc0` | initialise a 0x20-byte route/node record | translated | high | `game/path.rs` | `8000efc0_FUN_8000efc0.c` |
| `8000f0f0` | `fn_8000f0f0` | write one word into every node of a collection | translated | high | `game/path.rs` | `8000f0f0_FUN_8000f0f0.c` |
| `8000f270` | `fn_8000f270` | constructor variant: set up `+8`, run actor base init, store `+0x24` | translated | high | `game/actor.rs` | `8000f270_FUN_8000f270.c` |
| `8000f570` | `fn_8000f570` | flag-guarded state dispatch with two virtual-call loops | translated | high | `game/actor.rs` | `8000f570_FUN_8000f570.c` |
| `8000fbf0` | `fn_8000fbf0` | free an object when the signed release flag is positive | translated | high | `game/actor.rs` | `8000fbf0_FUN_8000fbf0.c` |
| `80152480` | `fn_80152480` | copy the owner's 12-word interaction state block | translated | high | `game/yoshi.rs` | `80152480_FUN_80152480.c` |
| `80152520` | `fn_80152520` | forward a 0x10-byte query to owner virtual `+0x50` | translated | high | `game/yoshi.rs` | `80152520_FUN_80152520.c` |
| `801525a0` | `fn_801525a0` | change the secondary target animation, then notify the primary target | translated | medium | `game/yoshi.rs` | `801525a0_FUN_801525a0.c` |
| `80152750` | `fn_80152750` | forward a query to secondary-target virtual `+0x3EC` | translated | high | `game/yoshi.rs` | `80152750_FUN_80152750.c` |
| `80152880` | `fn_80152880` | spawn `Wm_mr_ystonguehit` once and play sound `0x159` | translated | high | `game/yoshi.rs` | `80152880_FUN_80152880.c` |
| `80154550` | `fn_80154550` | five-level `+0x30` chain walk, then recurse on the sixth child | translated | high | `game/yoshi.rs` | `80154550_FUN_80154550.c` |
| `80154630` | `fn_80154630` | compare the last `':'`-separated name component | translated | high | `game/yoshi.rs` | `80154630_FUN_80154630.c` |
| `8015bb60` | `fn_8015bb60` | boolean controller-state query for id 3 | translated | high | `game/boot_scene.rs` | `8015bb60_FUN_8015bb60.c` |
| `8015bc40` | `fn_8015bc40` | create resource `0x26C` and store the handle at `+0xF0` | translated | high | `game/boot_scene.rs` | `8015bc40_FUN_8015bc40.c` |
| `8015bdd0` | `fn_8015bdd0` | poll controller readiness and latch `+0x102` | translated | high | `game/boot_scene.rs` | `8015bdd0_FUN_8015bdd0.c` |
| `8015c010` | `fn_8015c010` | create resources `0x2C0`/`0x2CF` into `+0xF4`/`+0xF8` | translated | high | `game/boot_scene.rs` | `8015c010_FUN_8015c010.c` |
| `8015c630` | `fn_8015c630` | clear the reset latch and rebind the save/controller scale | translated | high | `game/boot_scene.rs` | `8015c630_FUN_8015c630.c` |

## What the bodies complete

### Interaction object (`0x8014E4E0..0x80150AB0`)

The existing ten-body pick-up/release flow gains the secondary-target slot
`+0x1524` and the accept callback it already referred to by address:

* `0x8014E9B0` resolves `+0x1524` with the same registry lookup as the primary
  handle `+0x1520`.
* `0x8014E700` / `0x8014ED50` clear and re-enable the secondary path's status
  bit `0xB2`.
* `0x80150020` and `0x801507C0` are the two callees the commit path previously
  left as named constants (`PLAY_SELECT_SOUND`, `TARGET_ACCEPT`); their bodies
  now exist and the constants remain as call-site documentation.
* `0x8014E9D0` gives the missing readiness predicate used before a selection.

### Path/route band (`0x8000D0A0..0x8000F0F0`)

Two bodies complete the route-record vocabulary: `0x8000EFC0` clears the
0x20-byte record, and `0x8000F0F0` is the single-value twin of the collection
writer `0x8000D260`. `0x8000D0A0` writes a vector into the fixed-stride record;
its arithmetic is exact, but the owning subsystem is not proven (see
Uncertainties).

### Actor base (`0x8000F270..0x8000FBF0`)

`0x8000F270` is the second constructor variant (the body is byte-identical to
`0x8000F370`, which remains untranslated), `0x8000F570` is the flag-guarded
per-state dispatch with the two virtual-call loops, and `0x8000FBF0` is the
"release if positive" helper shared by the actor build path.

### Yoshi (`0x80152480..0x80154630`)

Seven `daYoshi_c` helpers: the owner-interface forwards, the secondary-target
animation/notify pair, the `Wm_mr_ystonguehit` spawn, the five-level `+0x30`
chain walk, and the last-name-component comparison. The 3,832-byte
`daYoshi_c` descriptor initialiser at `0x80153400` is still untranslated.

### Boot scene (`0x8015BB60..0x8015C630`)

Five save/controller helpers between the banner loader and the state handlers:
the boolean controller query, two resource registrations, the readiness latch,
and the scale rebind through the `+0xF0` resource object.

## Evidence quality notes

* **Dropped call operands.** Ghidra prints several calls without arguments.
  Where the callee's own body proves the register, the Rust call supplies it:
  `0x8004A8C0(wm)` in `fn_8014e700`, `0x80162D00(*(wm+0x1524))` in
  `wm_secondary_target`, and `0x8014E9B0(this)` at the Yoshi call sites. Each
  site is marked in the function doc.
* **`r2` constants.** Only `fn_8015c630` needs SDA2; it reads the float at
  `0x8042DBF0` (`r2 - 0x5490`) through [`read_f32_abs`], following the existing
  `engine/display.rs` convention.
* **`r13` constants.** None of the new bodies reads an `r13` global directly;
  the boot helpers reach their globals only through callees.
* **Virtual-base shape.** Actor calls use the vtable pointer stored at `+0x60`;
  interface calls use the pointer stored at `+0x14D8` or `+0xDEC`. These are
  observed at the call sites and are not interchangeable.
* **Name comparison.** `fn_80154630` reproduces the original's `col + 1`
  pointer even when `strrchr` returns null; the safety note records that this
  matches the image rather than adding a guard the original does not have.

## Uncertainties

1. `0x8000D0A0` — the `0x3390` record stride and `0xC` vector stride are exact,
   but no caller in the export proves whether the record is per-player or
   per-node. It is filed under `game/path.rs` because it writes node-shaped
   vectors; the ledger confidence is `medium`.
2. `0x801525A0` — the `+0x38C` byte is treated as a "ready" state and the
   `+0x3E0` slot as a notification, inferred from the guard shape and the
   neighbouring interaction code.
3. `0x8000F570` — the two `kind` values `5` and `6` are observed; their
   semantic names are not recovered. The `slw` shift is modelled with a
   six-bit mask and a zero result for counts >= 32.
4. `0x8015C630` — the object reached through `+0xF0` and `+0x1F4` is described
   only by this call site; its class name is unknown.
5. `0x8015BB60` — `0x8008F620`'s export omits its parameter, yet every call
   site passes an id. The Rust call keeps the literal `3` from the source.

## Next targets

Ordered by value per effort inside the same bands:

| priority | target | size | why |
|---|---|---|---|
| 1 | `0x80153400` | 3,832 B | `daYoshi_c` state descriptors (19 states); mechanical once the descriptor layout is decoded |
| 2 | `0x8015DDD0` | 4,716 B | `dScBoot_c` state descriptors (24 states); same descriptor pattern |
| 3 | `0x80150700`, `0x80150760`, `0x80150830` | 88–108 B | the remaining guarded target callbacks next to `wm_target_accept` |
| 4 | `0x8014E910`, `0x8014ECE0`, `0x8014F3D0`, `0x8014FD60` | 12–96 B | small interaction neighbours; `0x8014ECE0` needs its jump table confirmed first |
| 5 | `0x80152B30`, `0x80152CF0`, `0x80151C30`, `0x80151D00` | 204–324 B | remaining Yoshi tongue/selection helpers |
| 6 | `0x8000F370`, `0x8000FE90`, `0x8000FED0`, `0x8000FFB0`, `0x8000FFF0`, `0x80010030` | 64–108 B | byte-identical release/constructor siblings; batch them |
| 7 | `0x8000D740`, `0x8000D7E0`, `0x8000D820` | 60–160 B | rail/follower helpers that need the SDA2 float values and the lost `0x8000DE70` operands resolved first |
