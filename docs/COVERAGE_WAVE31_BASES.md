# Coverage — wave 31 (`d_basesNP`: box-iron machine, warp cannon and spawner)

## How to read this file

This page records the thirty-first translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** — *Translated functions* lists the 30 new bodies in
   `src/game/bases.rs`: the unnamed actor spawner, the `box_iron_machine`
   actor, the `warp_cannon` actor and their lifecycle/effect helpers.
2. **Why these functions?** — *Selection* explains how the pass continued the
   address-ordered sweep above the wave-30 window and which decompiler outputs
   were skipped.
3. **What is still missing?** — *Deferred candidates* records nearby bodies
   that are thunks, use the `0x802DCDxx` save prologue, or remain recursive
   helpers called by address.

Column meanings:

- **Status** — `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** — strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, constant or called
  body is inferred; `low` = mostly guesswork.
- **Evidence** — file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8004xxxx`..`0x8043xxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 31 |
|---|---:|
| New translated functions | 30 |
| Actor constructors / in-place initialisers (`0x8074E120`, `0x8074E2F0`, `0x8074EDB0`) | 3 |
| BRRES model loaders (`0x8074E8D0`, `0x8074F1A0`) | 2 |
| Lifecycle, capture and effect bodies (`0x8074E280..0x8074FF90`) | 25 |
| Selected bytes | 3,704 |
| Translated functions in `src/game/bases.rs` (cumulative) | 789 (787 translated, 2 partial) |
| Cumulative selected bytes | 240,764 (21.69 % of indexed bytes) |

The 30 new bodies select 3,704 of the 1,109,860 indexed bytes (0.33 %) and 30
of the 5,409 eligible entries (0.55 %). 27 rows are `high` confidence; 3 are
`medium` (`0x8074F1A0`, `0x8074F580`, `0x8074FA80`) because an unprototyped
float ABI, a dropped argument on `0x8013FAC0`, and a pointer/value split across
two `0x80429xxx` globals are inferred.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2–30 were removed before selection. Wave 30 stopped at
`0x8073D340`; this pass began at `0x8074E120` and took the complete
`0x8074E120..0x8074FF90` cluster:

1. **Take the spawner as a unit.** `0x8074E120` allocates `0x3A0` bytes and
   installs vtable `0x80811E44`. Its mode word at `+0x39C` selects between two
   recursive actor counters (`0x8074E380` / `0x8074E3F0`) and two spawn
   templates (`0x59` and `0x156`) through `0x80064610`. `0x8074E2F0` seeds the
   retry timer, `0x8074E280` ticks it, and `0x8074E460` performs the spawn.
2. **Take the named `box_iron_machine` actor.** `0x8074E8D0` opens
   `g3d/box_iron_machine.brres` and binds the `box_iron_machine` pane at
   priority `0x20`. `0x8074E7D0` copies the actor transform into the model,
   `0x8074E980` spawns the hit effect, `0x8074EA10` owns its timer, and
   `0x8074EC30` is the adjacent release/delete helper.
3. **Take the named `warp_cannon` actor.** `0x8074EDB0` allocates `0x528`
   bytes and installs vtable `0x808120C0`; `0x8074F1A0` loads
   `g3d/warp_cannon.brres`, binds `warp_cannon` at priority `0x23`, attaches
   the unnamed clip at `0x80812370`, and builds the `language`/`number` panes.
   `0x8074F360` copies the transform, `0x8074F0D0` and `0x8074F410` test whether
   a player can be captured, `0x8074F550` sets the player mask and
   `0x8074F580` counts active masked players.
4. **Take the complete effect tail.** `0x8074FA80` triggers the all-players
   event. The three sound wrappers `0x8074FB70`, `0x8074FD00` and `0x8074FED0`
   play ids `0x29D`, `0x29E` and `0x29F`; `0x8074FC20`, `0x8074FDF0` and
   `0x8074FF90` start the three render phases; `0x8074FC90`, `0x8074FD60`,
   `0x8074FE60` and `0x8074FF30` advance them.
5. **Rejected or deferred nearby bodies.** `0x8074E0C0`, `0x8074E0F0`,
   `0x8074ED50` and `0x8074ED80` are 48-byte duplicate thunks into the
   unresolved indirect-jump stub `0x802DCBFC`. `0x8074E7A0` is a 40-byte
   model-disable helper. `0x8074E380` and `0x8074E3F0` are the recursive
   counters kept as address-only callees. `0x8074EFF0`, `0x8074F3C0` and
   `0x8074F600` use the `0x802DCDxx` save prologue, `extraout_` registers or a
   stack-guard artifact and need a separate ABI pass.

## Clusters

### Spawner (`0x8074E120..0x8074E520`)

- `0x8074E120` builds the `0x3A0`-byte object, vtable `0x80811E44`, timer
  `+0x394`. `0x8074E2F0` replaces the timer with
  `(flags & 0xFF) * 0x1E`, or `0xB4` when that product is zero.
- `0x8074E280` XORs the stored mask `+0x398` with the result of
  `0x8074E320` and evaluates the exact bit expression
  `((int)v >> 1) - (v & stored) < 0`. On a true result the timer falls; below
  one it calls `0x8074E460` and reseeds with `0x8074E2F0`.
- `0x8074E320` selects the recursive counter by `+0x39C`; `0x8074E460`
  selects the spawn template. `0x8074E4B0` spawns type `0x59` with
  `0x10000000 | 0x1000` and the position offset at `0x807CE69C`;
  `0x8074E520` spawns type `0x156` with flags from the table at `0x807CE6A0`.

### `box_iron_machine` (`0x8074E7D0..0x8074EC30`)

- `0x8074E8D0` is the loader: accessor `+0x3D4`, resource `+0x3F0`, model
  `+0x3F4`, BRRES `0x80811F24`, pane `0x80811F40`, priority `0x20`.
- `0x8074E7D0` adds `0x807CE6B8` to x/y, uses `+0x518` for the height, builds
  the matrix at `+0x7C` and binds the model. `0x8074E980` spawns effect type
  `4` through `0x800B2EE0`/`0x80B29A10`. `0x8074EA10` reloads the `+0x51C`
  timer from the float at `0x807CE6B0` and retriggers the effect.
- `0x8074EC30` calls `0x8015F800(actor, 0)` and, for a positive count,
  `0x802B9280(actor)`, then returns the actor unchanged.

### `warp_cannon` (`0x8074EDB0..0x8074F580`)

- `0x8074EDB0` allocates `0x528` bytes, calls `0x80094E80` and installs vtable
  `0x808120C0`. `0x8074F1A0` loads `g3d/warp_cannon.brres` (`0x8081234C`),
  pane `warp_cannon` (`0x80812364`), clip `0x80812370`, `language`
  (`0x80812378`) and `number` (`0x80812384`), then applies the two animation
  times passed in `f1`/`f2`.
- `0x8074F360` copies the actor transform. `0x8074F0D0` checks the player
  state byte `+0x38C` and player data `+0xEA4`, asks `0x8074F410`, probes
  `0x8013F9E0` at the cannon position plus `+0x678..+0x680`, then reads the
  character id through virtual slot `+0x6C` and sets the mask bit.
- `0x8074F410` calls interface virtuals `+0x28` and `+0x14`; the latter's
  nonzero result gates the player-state, character-range, mask and horizontal
  window checks (`0x807CE704`, `0x807CE708`, `0x807CE70C`).
  `0x8074F550` ORs `1 << slot` into `+0x6A4`; `0x8074F580` counts the active
  masked players through `0x8005F900` and `0x8013FAC0`.

### Effect tail (`0x8074FA80..0x8074FF90`)

- `0x8074FA80` compares the threshold at `0x80429CA0` with the player count,
  copies the manager word at `0x80429E00+0x38` to `+0x58`, stores effect id
  `0x0E`/time at `0x80429DD0+0x90114`/`+0x90110`, and triggers virtual event
  `0x80831AF8`.
- `0x8074FB70`, `0x8074FD00` and `0x8074FED0` copy the actor position through
  `0x8006A4B0` and play ids `0x29D`, `0x29E` and `0x29F` through
  `0x80198C30` with the audio manager at `0x80429D60`.
- `0x8074FC20`, `0x8074FDF0` and `0x8074FF90` initialise the render state at
  `+0x664` through the nine-argument `0x800DF860` and set the `+0x6A5`
  countdown to `0x20`. `0x8074FC90` reloads an angle and fires event
  `0x80831BB8`; `0x8074FD60` steps by `0x4000`, clamps to `0xE000` and fires
  `0x80831BF8`; `0x8074FE60` subtracts `0x6000` and fires `0x80831C38`;
  `0x8074FF30` steps by `0x2000` with limit `0x100` and fires `0x80831C78`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave31-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8074E120` | `fn_8074e120` | allocate the unnamed `0x3A0`-byte actor spawner | translated | high | `8074e120_FUN_8074e120.c` |
| `0x8074E2F0` | `fn_8074e2f0` | seed the spawner retry timer from the flags | translated | high | `8074e2f0_FUN_8074e2f0.c` |
| `0x8074E280` | `fn_8074e280` | tick the timer and request a respawn | translated | high | `8074e280_FUN_8074e280.c` |
| `0x8074E320` | `fn_8074e320` | count watched actors by spawner mode | translated | high | `8074e320_FUN_8074e320.c` |
| `0x8074E460` | `fn_8074e460` | dispatch the spawn request | translated | high | `8074e460_FUN_8074e460.c` |
| `0x8074E4B0` | `fn_8074e4b0` | spawn actor type `0x59` | translated | high | `8074e4b0_FUN_8074e4b0.c` |
| `0x8074E520` | `fn_8074e520` | spawn actor type `0x156` | translated | high | `8074e520_FUN_8074e520.c` |
| `0x8074E7D0` | `fn_8074e7d0` | copy the box-iron-machine transform | translated | high | `8074e7d0_FUN_8074e7d0.c` |
| `0x8074E8D0` | `fn_8074e8d0` | load `box_iron_machine` | translated | high | `8074e8d0_FUN_8074e8d0.c` |
| `0x8074E980` | `fn_8074e980` | spawn the box-iron-machine hit effect | translated | high | `8074e980_FUN_8074e980.c` |
| `0x8074EA10` | `fn_8074ea10` | tick the hit-effect timer | translated | high | `8074ea10_FUN_8074ea10.c` |
| `0x8074EC30` | `fn_8074ec30` | release and optionally delete an actor | translated | high | `8074ec30_FUN_8074ec30.c` |
| `0x8074EDB0` | `fn_8074edb0` | construct the `0x528`-byte warp cannon | translated | high | `8074edb0_FUN_8074edb0.c` |
| `0x8074F0D0` | `fn_8074f0d0` | capture a player standing on the cannon | translated | high | `8074f0d0_FUN_8074f0d0.c` |
| `0x8074F1A0` | `fn_8074f1a0` | load `warp_cannon`, clip and panes | translated | medium | `8074f1a0_FUN_8074f1a0.c` |
| `0x8074F360` | `fn_8074f360` | copy the warp-cannon transform | translated | high | `8074f360_FUN_8074f360.c` |
| `0x8074F410` | `fn_8074f410` | test cannon capture eligibility | translated | high | `8074f410_FUN_8074f410.c` |
| `0x8074F550` | `fn_8074f550` | set one player mask bit | translated | high | `8074f550_FUN_8074f550.c` |
| `0x8074F580` | `fn_8074f580` | count active masked players | translated | medium | `8074f580_FUN_8074f580.c` |
| `0x8074FA80` | `fn_8074fa80` | trigger the all-players event | translated | medium | `8074fa80_FUN_8074fa80.c` |
| `0x8074FB70` | `fn_8074fb70` | play sound `0x29D` | translated | high | `8074fb70_FUN_8074fb70.c` |
| `0x8074FC20` | `fn_8074fc20` | start effect phase one | translated | high | `8074fc20_FUN_8074fc20.c` |
| `0x8074FC90` | `fn_8074fc90` | advance effect phase one | translated | high | `8074fc90_FUN_8074fc90.c` |
| `0x8074FD00` | `fn_8074fd00` | play sound `0x29E` | translated | high | `8074fd00_FUN_8074fd00.c` |
| `0x8074FD60` | `fn_8074fd60` | advance the clamped effect angle | translated | high | `8074fd60_FUN_8074fd60.c` |
| `0x8074FDF0` | `fn_8074fdf0` | start effect phase two | translated | high | `8074fdf0_FUN_8074fdf0.c` |
| `0x8074FE60` | `fn_8074fe60` | advance effect phase two | translated | high | `8074fe60_FUN_8074fe60.c` |
| `0x8074FED0` | `fn_8074fed0` | play sound `0x29F` | translated | high | `8074fed0_FUN_8074fed0.c` |
| `0x8074FF30` | `fn_8074ff30` | advance effect phase three | translated | high | `8074ff30_FUN_8074ff30.c` |
| `0x8074FF90` | `fn_8074ff90` | start effect phase three | translated | high | `8074ff90_FUN_8074ff90.c` |

**Totals:** 30 translated, 0 partial, 0 stub (27 high, 3 medium).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x8074E0C0`, `0x8074E0F0`, `0x8074ED50`, `0x8074ED80` | 48 each | identical thunks into the unresolved indirect-jump stub `0x802DCBFC`; no Rust-visible effect. |
| `0x8074E7A0` | 40 | model-disable helper; below this pass's value threshold and not part of the loader pair. |
| `0x8074E380`, `0x8074E3F0` | 100 each | recursive counters for actor types `0x59` / `0x156`; translated callers keep them as address-only callees. |
| `0x8074EFF0` | 220 | uses the `0x802DCDxx` save prologue and a stack-guard artifact (`&stack0x00000000 != (undefined1 *)0x40`); needs an ABI pass. |
| `0x8074F3C0` | 72 | random float selector through `0x800B45F0`; not actor lifecycle. |
| `0x8074F600` | 396 | `0x802DCD74` save prologue, `func_0x8013FAA0` call and global effect-table setup; needs the ABI pass used for the other `0x802DCDxx` bodies. |

## Unknowns preserved

1. **`0x800DF180`.** The BRRES loader identity is not in this export; the two
   loaders keep an `// unknown:` note.
2. **`0x80B29A10`.** The box-iron-machine effect starter is outside this REL;
   it is called by address with the copied position, effect handle and type
   bits.
3. **`0x8013FAC0`.** The player-active predicate is printed without arguments;
   this pass passes the player returned by `0x8005F900` (`// inferred:` from
   the explicit form in `0x8074F600`).
4. **Spawner identity.** The `0x8074E120` class has no string. Its role is
   inferred from the two actor ids it spawns and from the mask/timer fields.
5. **`0x8074F1A0` ABI.** Ghidra prints the first two arguments as `undefined8`;
   they are used directly as the `f1`/`f2` times of `0x80166EF0`, so the Rust
   models them as `f32`.
6. **Pointer/value split at `0x80429E00` / `0x80429DD0`.** The C adds
   `+0x38`, `+0x58`, `+0x90110` and `+0x90114` to the loaded globals, so each
   global is a pointer; the reconstruction reads the pointer first.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` has 789 functions after this pass (787 `translated`, 2
pre-existing `partial`); this pass adds 30, all with `# Original` blocks and
the synthetic-base warning.
`node tools/doc-lint.mjs src` reports `ok: true`, 0 violations over 68 files.
`cargo check` exits 0; `src/game/bases.rs` produces no warnings. The crate's
remaining warnings are in other lanes (`src/runtime/bt.rs` cast suggestions).
