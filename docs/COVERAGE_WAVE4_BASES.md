# Coverage - wave 4 (`d_basesNP` stage actors)

## How to read this file

This page records the third translation pass over `d_basesNP`, the base gameplay
REL (`game::bases`). It answers:

1. **What was added?** - the table below lists the 28 functions appended to
   `src/game/bases.rs`, with their synthetic addresses, Rust names,
   plain-English roles, status, confidence, and Ghidra evidence.
2. **Why these functions?** - the *Selection* section explains how four
   complete stage-actor clusters were chosen from the class/state-name family.
3. **What is still missing?** - *Deferred candidates* lists the neighbouring
   functions that were intentionally left for later, and *Unknowns preserved*
   records every unresolved address or role instead of guessing it.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the *evidence*: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, ownership, or
  hidden ABI operand is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8000xxxx`, `0x8037xxxx`, `0x8042xxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 3 | Wave 4 | Cumulative |
|---|---:|---:|---:|
| Translated functions in `game/bases.rs` | 55 | 28 | 83 |
| `daCannonPipe_c` cluster | 0 | 9 | 9 |
| `da2DPlayer_c` cluster | 0 | 8 | 8 |
| `daPowBlock_c` cluster | 0 | 5 | 5 |
| `daPropBlock_c` cluster | 0 | 6 | 6 |
| State registrations covered | 11 | 30 | 41 |
| Selected bytes | 10,284 | 8,976 | 19,260 |
| Coverage of indexed bytes | 0.93 % | 0.81 % | 1.73 % |
| Coverage of eligible entries (5,409) | 1.02 % | 0.52 % | 1.53 % |

The module still contains 5,615 function entries and 79 `StateID`-bearing
files. This pass deliberately takes four coherent actor clusters, not a range.

## Selection

The pass was evidence-first and cluster-first:

1. **Started from `functions.tsv` and the existing `bases.rs`.** The 55 wave-2
   and wave-3 addresses were removed from the candidate set before selection.
2. **Swept the remaining class/state-name files.** The export has 79 files that
   reference `StateID`; 78 of them contain full
   `s_<Class>_c__StateID_<Name>` strings. Four clusters were chosen because each
   has a complete, self-contained story: a multi-state registration table, a
   constructor that installs a vtable, a model load, a small set of update
   helpers, and a registered destructor.
3. **Required complete control flow.** Functions whose only body is a branch
   thunk (`0x802DCBFC` forwarders) were left out so that this pass adds no new
   `partial` rows. The paired-single reconstruction at `0x80651640` was rejected
   again for the reason recorded in wave 3.
4. **Verified the registration template.** Every selected table builds 0x30-byte
   descriptors at `descriptor-0x10` nodes, uses the same nine-word parameter
   order `6,7,8,3,4,5,0,1,2`, and calls `fn_80600160`. The existing
   [`register_state_desc`] helper therefore covers all four tables without a
   second template.
5. **Cross-checked constructors against their vtables.** `0x8066B010` installs
   `0x807E8E60`, `0x8075A6C0` installs `0x80814310`, `0x807262F0` installs
   `0x8080AF40`/`0x8080B1C0`, and `0x80728260` installs `0x8080B380`. These are
   the objects whose state tables are registered by the four selected
   registration functions.

The clusters are consecutive, so the selected set also preserves the original
address ordering inside each actor.

**Repair carried by this pass.** While translating `0x8075AA50`, the same
Ghidra argument loss was found in the wave-2 `0x80650D40` body: `0x801C0BD0`
takes three position floats plus a matrix pointer, but Ghidra printed only the
matrix and left the three position values as stack temporaries. Both call sites
now pass the position values; the wave-2 ledger row stays `translated/high`
because only the lost arguments changed.

## Clusters

### `daCannonPipe_c` (`0x8066A040..0x8066B010`)

The cannon pipe that launches the player. The class name and eight states come
from the strings in `fn_8066a820`:

- `StateID_Wait`, `StateID_PilderOn`, `StateID_CanstWai`,
  `StateID_CannonSe`, three `StateID_WaitShoo` variants, `StateID_Return`.
- Shared vtable `0x807E8DB0`, callback `0x8066AEC0` (the registered destructor).
- `fn_8066a040` calls `0x80669530(actor)` to select the sprite record, copies
  its angle into `+0x44C`, and derives the facing-down byte at `+0x768`.
- `fn_8066a0b0`/`fn_8066a2c0` store their incoming angle at `+0x44C`, rebuild
  the actor name at `+0x44E`, and dispatch the `WaitShoo`/`Wait` descriptors
  through the interface object at `+0x454`.
- `fn_8066b010` is the constructor; `0x8066AFE0`/`0x8066AFF0`/`0x8066B000`
  are three vtable getters that all return `0x80377A04`.

### `da2DPlayer_c` (`0x80759C60..0x8075ABE0`)

A 2-D player proxy used by a side-scrolling stage. Eleven states are registered
by `fn_80759c60`: `InitialSet`, `StandbyWai`, two `MuKiChange` entries,
`TargetPlac`, `ChangeFron`, `SpinJump`, `LandingAni`, `JumpUp`, `JumpDown`, and
`EndSetup`. Shared vtable `0x80814260`, callback `0x8075A5A0`.

- `fn_80759c60` also initialises the shared block at `0x80813BC0` from
  `0x807CEA10..0x807CEA4C`, guarded by `0x808325B9`.
- `fn_8075a6c0` constructs the object (`+0x184 = -1`, vtable `0x80814310`).
- `fn_8075a9a0` loads `g3d/model.brres` and binds the `cobAnchor` resource.
- `fn_8075aa50` applies three halfword source angles; `fn_8075ab00` places the
  actor at the `cobKoopaShip` spawn or at the fallback point.
- Ghidra prints `unaff_GQR0`/`ldexpf` at the entry and exit of `fn_80759c60`
  because it misreads the paired-single `psq_st` save/restore of `f30`/`f31`.
  The bytes at `0x8075A578`/`0x8075A580` are inside the save area, so those
  lines are compiler artifacts, not gameplay code.

### `daPowBlock_c` (`0x80725790..0x807262F0`)

The carried/throwable POW block. Seven states (`Wait`, `Carry`, `Drop`,
`Drop2`, `Throw`, `Shock`, `Shock2`), vtable `0x8080AEBC`, callback `0x80726140`.

- `fn_80725790` and `fn_807259E0` are byte-identical bodies: they play virtual
  slot `+0x1C` on the model at `+0x3F0` and update the collision transform.
- `fn_807262F0` constructs the actor and installs the model vtable
  `0x8080B1C0` at `+0x524`.

### `daPropBlock_c` (`0x807276D0..0x807282D0`)

The interactive prop block. Four states (`Wait`, `Carry`, `Throw`, `Down`),
vtable `0x8080B310`, callback `0x807280E0`.

- `fn_807276D0` detaches the block from its player and clears the carried flag.
- `fn_80727BD0` resets velocity, timers, model flags, and collision.
- `fn_80728260` allocates `0x430` bytes; `fn_807282D0` loads
  `g3d/obj_door.brres` and binds the `obj_doorA` resource.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave4-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8066A040` | `fn_8066a040` | copy the cannon-pipe sprite angle and derive the facing flag | translated | high | `8066a040_FUN_8066a040.c` |
| `0x8066A0B0` | `fn_8066a0b0` | rebuild the name, play sound `0x29A`, dispatch `WaitShoo` | translated | high | `8066a0b0_FUN_8066a0b0.c` |
| `0x8066A2C0` | `fn_8066a2c0` | rebuild the name and dispatch `Wait` | translated | high | `8066a2c0_FUN_8066a2c0.c` |
| `0x8066A820` | `fn_8066a820` | register the eight `daCannonPipe_c` states | translated | high | `8066a820_FUN_8066a820.c` |
| `0x8066AEC0` | `fn_8066aec0` | destroy a registered cannon-pipe state | translated | high | `8066aec0_FUN_8066aec0.c` |
| `0x8066AFE0` | `fn_8066afe0` | return the constant `0x80377A04` | translated | high | `8066afe0_FUN_8066afe0.c` |
| `0x8066AFF0` | `fn_8066aff0` | return the constant `0x80377A04` | translated | high | `8066aff0_FUN_8066aff0.c` |
| `0x8066B000` | `fn_8066b000` | return the constant `0x80377A04` | translated | high | `8066b000_FUN_8066b000.c` |
| `0x8066B010` | `fn_8066b010` | construct a `daCannonPipe_c` actor | translated | high | `8066b010_FUN_8066b010.c` |
| `0x80759C60` | `fn_80759c60` | register the eleven `da2DPlayer_c` states and shared block | translated | high | `80759c60_FUN_80759c60.c` |
| `0x8075A5A0` | `fn_8075a5a0` | destroy a registered 2-D player state | translated | high | `8075a5a0_FUN_8075a5a0.c` |
| `0x8075A6C0` | `fn_8075a6c0` | construct a `da2DPlayer_c` actor | translated | high | `8075a6c0_FUN_8075a6c0.c` |
| `0x8075A7F0` | `fn_8075a7f0` | save the previous position and start model setup | translated | high | `8075a7f0_FUN_8075a7f0.c` |
| `0x8075A9A0` | `fn_8075a9a0` | load `g3d/model.brres` and bind `cobAnchor` | translated | high | `8075a9a0_FUN_8075a9a0.c` |
| `0x8075AA50` | `fn_8075aa50` | apply the halfword source angles and colour | translated | medium | `8075aa50_FUN_8075aa50.c` |
| `0x8075AB00` | `fn_8075ab00` | place the actor at the `cobKoopaShip` spawn | translated | medium | `8075ab00_FUN_8075ab00.c` |
| `0x8075ABE0` | `fn_8075abe0` | clear the reset word at `+0x1F0` | translated | high | `8075abe0_FUN_8075abe0.c` |
| `0x80725790` | `fn_80725790` | update the POW block model and collision transform | translated | high | `80725790_FUN_80725790.c` |
| `0x807259E0` | `fn_807259e0` | second copy of the POW block update | translated | high | `807259e0_FUN_807259e0.c` |
| `0x80725BC0` | `fn_80725bc0` | register the seven `daPowBlock_c` states | translated | high | `80725bc0_FUN_80725bc0.c` |
| `0x80726140` | `fn_80726140` | destroy a registered POW block state | translated | high | `80726140_FUN_80726140.c` |
| `0x807262F0` | `fn_807262f0` | construct a `daPowBlock_c` actor | translated | high | `807262f0_FUN_807262f0.c` |
| `0x807276D0` | `fn_807276d0` | detach the prop block from its player | translated | high | `807276d0_FUN_807276d0.c` |
| `0x80727BD0` | `fn_80727bd0` | reset the prop block to its resting state | translated | high | `80727bd0_FUN_80727bd0.c` |
| `0x80727D60` | `fn_80727d60` | register the four `daPropBlock_c` states | translated | high | `80727d60_FUN_80727d60.c` |
| `0x807280E0` | `fn_807280e0` | destroy a registered prop block state | translated | high | `807280e0_FUN_807280e0.c` |
| `0x80728260` | `fn_80728260` | allocate and construct a `daPropBlock_c` actor | translated | high | `80728260_FUN_80728260.c` |
| `0x807282D0` | `fn_807282d0` | load `g3d/obj_door.brres` and bind `obj_doorA` | translated | high | `807282d0_FUN_807282d0.c` |

**Totals:** 28 translated, 0 partial, 0 stub. 26 high confidence, 2 medium
(`0x8075AA50`, `0x8075AB00`). The four registration functions add 30 state
descriptors (8 + 11 + 7 + 4).

## Deferred candidates

These are the nearest untranslated neighbours; each was deliberately left for a
later pass.

| Address range | Why deferred |
|---|---|
| `0x8066A7C0`, `0x8066A7F0` | byte-identical forwarders to the unresolved indirect stub `0x802DCBFC`; they carry a hidden `r12` and would be `partial` |
| `0x80669530`, `0x80669560` | shared cannon-pipe update helpers outside the selected cluster; no class/state string of their own |
| `0x80759790`, `0x80759860`, `0x80759A20` | 2-D player helpers before the registration table; `0x80759A20` contains an unrecovered jump table |
| `0x80759C00`, `0x80759C30` | the same `0x802DCBFC` forwarder shape |
| `0x80725270`, `0x807255D0`, `0x80725830` | large POW block behaviour functions next to `0x80725BC0`, with many main.dol float APIs; `0x80725270` has a 22-local stack frame |
| `0x80726260`, `0x80726290` | the same `0x802DCBFC` forwarder shape |
| `0x80727720`, `0x807278F0`, `0x80727A30` | prop block behaviour functions that call the still-untranslated `0x80727190..0x807272C0` family |
| `0x80728200`, `0x80728230` | the same `0x802DCBFC` forwarder shape |
| `0x80651990`, `0x80651A60` | still blocked on the `0x80828430` spawn-table layout (wave-3 deferral) |
| `0x80651640` | **rejected**: paired-single reconstruction (`ldexpf`, `unaff_GQR0`, unreachable blocks) |
| `0x806043C0` | `dCharacterChangeSelectBase_c` table, 22 states; still the largest mechanical registration |
| `0x806083B0`, `0x8060CBE0`, `0x8060F010`, `0x80618D00` | remaining menu state tables; `0x80618D00` has 44 `StateID` references |

The four selected clusters also show the next useful step: the class/state
family is regular enough that a table generator could emit the remaining
`register_state_desc` calls from the C, but only after each string block and
shared word has been checked against the export.

## Unknowns preserved

1. **`daCannonPipe_c` type getters.** `0x8066AFE0`, `0x8066AFF0`, and
   `0x8066B000` all return `0x80377A04`. The value behaves like a shared
   main.dol descriptor, but the REL does not name the type.
2. **Lost `this`, lost register arguments, and pair/float lowering.**
   `0x8066A0B0`, `0x8075A7F0`, and `0x80727BD0` call a helper without a visible
   argument (the `0x802DCD74`/`0x802DCDC0` family). The Rust passes the actor,
   which the callee body clearly consumes. The three constructors `0x8066B010`,
   `0x8075A6C0`, and `0x807262F0` also pass the new object to their base
   constructors (`0x80094E80`/`0x800F5FD0`), whose `this` Ghidra drops;
   `0x8066B010` and `0x807262F0` are confirmed by
   `80094e80_FUN_80094e80.c` in the main.dol export. `0x80728260` likewise
   passes the new object to `0x80066FC0`. Finally, `0x8066A0B0` and
   `0x8066A2C0` begin with `sth r4,0x44C(r3)`: Ghidra dropped the second
   parameter, so both Rust signatures take `angle: i16`.
3. **Paired-single save/restore in `0x80759C60`.** Ghidra renders the
   `psq_st`/`psq_l` pairs as `unaff_GQR0` and `ldexpf`. They are ABI save slots,
   not game logic; the Rust omits them and documents the artifact.
4. **Lost floating-point arguments and `double` casts.** `0x8075AA50` and the
   wave-2 `0x80650D40` show only the matrix pointer for `0x801C0BD0`; the three
   position floats survive as stack temporaries. Both bodies now pass them as
   the `(f32, f32, f32, matrix)` prototype seen at the other `0x801C0BD0` call
   sites. Likewise `0x80725790`/`0x807259E0` call `0x8007FBF0` with `f1`..`f4`
   loaded by `lfs`; Ghidra's `(double)` spellings are an artifact of the missing
   prototype, so the Rust helpers take `f32`. The short angle vectors in
   `0x8075AA50` and the binding arrays in `0x8075A9A0`/`0x807282D0` still have a
   second half/word that the C leaves unset; the Rust initialises it to zero and
   records that the callee contract for those bytes is not recovered.
5. **Scene/table arithmetic.** `0x8075AB00` uses the `cobKoopaShip` string and
   the scene strides `+0x338C`/`0xBF8`; the arithmetic is explicit but the
   record names are inferred from `src/game/stage.rs`.
6. **Engine APIs left unnamed.** `0x8007FBF0` (four-float transform helper),
   `0x8016EF50` (three-vector setter), `0x80199390` (sound queue),
   `0x800DF180` (BRRES loader), and `0x80239E30` (resource binding) are called
   by address with `// unknown:` notes.
7. **Duplicate POW update.** `0x80725790` and `0x807259E0` are byte-identical;
   the REL keeps both as separate vtable entries. The Rust keeps both bodies so
   the original call targets still resolve.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`doc-lint` reports `ok: true`, 0 violations over 68 files. `cargo check` exits 0
for the whole crate after the wave-4 additions. `src/game/bases.rs` was also
compiled in isolation against a copy of `platform.rs` with a minimal `#![no_std]`
crate (`cargo check`, exit 0), so its status is independent of concurrent edits
in other lanes.
