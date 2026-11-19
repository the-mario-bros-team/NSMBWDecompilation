# Coverage - wave 5 final pass (`d_enemiesNP`)

## How to read this file

This page records the fifth and final translation pass over the enemy REL,
`d_enemiesNP`. It answers the same three questions as the earlier coverage
pages:

1. **What was added?** - the table below lists the 28 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains why the six remaining state
   tables, their six constructors, the fourteen `daEnGabon_c` handlers and two
   `LiftRotHalf` transform helpers were chosen.
3. **What is still missing?** - *Remaining work* lists the deferred actors and
   the unknowns that were preserved rather than guessed.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  accounted for; `partial` = entry/exit and major effects only; `stub` =
  address only.
- **Confidence** - strength of the *evidence*: `high` = bytes/flow are
  explicit and cross-checked; `medium` = flow is explicit but a role or
  ownership is inferred; `low` = mostly guesswork.
- **Evidence** - file under
  `work/ghidra/rel-d_enemiesNP/export/`; that C file is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`. The REL has no
embedded load address; the retail loader may place it anywhere. These are not
runtime addresses.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Range inventory

| Metric | Wave 2 | Wave 3 | Wave 4 | Wave 5 | Cumulative |
|---|---:|---:|---:|---:|---:|
| Translated functions in `game/enemies.rs` | 22 | 24 | 30 | 28 | 104 |
| `StateID`-bearing state tables translated | 7 | 11 | 19 | 6 | 43 |
| State registrations covered | 59 | 24 | 108 | 37 | 228 |
| Actor constructors translated | 5 | 5 | 8 | 6 | 24 |
| Handlers / infrastructure translated | 10 | 8 | 3 | 16 | 37 |
| Deferred state tables | 36 | 25 | 6 | 0 | 0 |

The module still contains 3,555 function entries. This pass raises the
`StateID` table coverage from 37 to 43, which is the complete inventory from
the export: seven first-pass tables, eleven from wave 3, nineteen from wave 4
and the final six here. No table remains deferred.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A5EF70` | `register_gabon_states` | `daEnGabon_c`: `Appear`, `Move`, `MoveEd`, `Attack`, `DieFumi`, `DieFall` | translated | high | `80a5ef70_FUN_80a5ef70.c` |
| `0x80A75800` | `register_himan_bros_states` | `daEnHimanBros_c`: 11 attack/jump/walk/death states | translated | high | `80a75800_FUN_80a75800.c` |
| `0x80A7EF40` | `register_icicle_states` | `daEnIcicle_c`: 8 parent/animation/down states | translated | high | `80a7ef40_FUN_80a7ef40.c` |
| `0x80ABB210` | `register_lift_rot_half_states` | `daEnLiftRotHalf_c`: `Create`, `Move` | translated | high | `80abb210_FUN_80abb210.c` |
| `0x80AECD40` | `register_rail_poly_child_states` | `daEnRailPolyChild_c`: `Wait`, `Move` | translated | high | `80aecd40_FUN_80aecd40.c` |
| `0x80AFE3C0` | `register_sanbo_el_states` | `daEnSanboEl_c`: `Move`, `Vacume`, three damage states, `CriticalFall`, `FreezeFall`, `Ice` | translated | high | `80afe3c0_FUN_80afe3c0.c` |
| `0x80A5F810` | `alloc_gabon_actor` | Allocates/constructs a `0x720`-byte `daEnGabon_c` actor | translated | high | `80a5f810_FUN_80a5f810.c` |
| `0x80A76450` | `alloc_himan_bros_actor` | Allocates/constructs a `0x8B0`-byte `daEnHimanBros_c` actor | translated | high | `80a76450_FUN_80a76450.c` |
| `0x80A7F6F0` | `alloc_icicle_actor` | Allocates/constructs a `0x798`-byte `daEnIcicle_c` actor | translated | high | `80a7f6f0_FUN_80a7f6f0.c` |
| `0x80ABC4F0` | `alloc_lift_rot_half_actor` | Allocates/constructs a `0x828`-byte `daEnLiftRotHalf_c` actor | translated | high | `80abc4f0_FUN_80abc4f0.c` |
| `0x80ACE540` | `alloc_rail_poly_child_actor` | Allocates/constructs a `0x600`-byte `daEnRailPolyChild_c` actor | translated | high | `80ace540_FUN_80ace540.c` |
| `0x80AFEFD0` | `alloc_sanbo_el_actor` | Allocates/constructs a `0x550`-byte `daEnSanboEl_c` actor | translated | high | `80afefd0_FUN_80afefd0.c` |
| `0x80A5FDF0` | `fn_80a5fdf0` | Gabon: seeds the probe/parameter block | translated | high | `80a5fdf0_FUN_80a5fdf0.c` |
| `0x80A5FE90` | `fn_80a5fe90` | Gabon: two-point ground probe | translated | high | `80a5fe90_FUN_80a5fe90.c` |
| `0x80A60000` | `fn_80a60000` | Gabon: binds `net_walk2` and seeds speed | translated | high | `80a60000_FUN_80a60000.c` |
| `0x80A601A0` | `fn_80a601a0` | Gabon `MoveEd`: same body as `fn_80a60000` | translated | high | `80a601a0_FUN_80a601a0.c` |
| `0x80A60250` | `fn_80a60250` | Gabon `Move`: probe-table init and state switch | translated | high | `80a60250_FUN_80a60250.c` |
| `0x80A60360` | `fn_80a60360` | Gabon `MoveEd`: timer countdown and switch | translated | high | `80a60360_FUN_80a60360.c` |
| `0x80A603D0` | `fn_80a603d0` | Gabon: neutral speed and phase-1 entry | translated | high | `80a603d0_FUN_80a603d0.c` |
| `0x80A60440` | `fn_80a60440` | Gabon: two-phase attack timer | translated | high | `80a60440_FUN_80a60440.c` |
| `0x80A60520` | `fn_80a60520` | Gabon death entry: long then short blend | translated | high | `80a60520_FUN_80a60520.c` |
| `0x80A60590` | `fn_80a60590` | Gabon `Attack`: sound event and hit switch | translated | high | `80a60590_FUN_80a60590.c` |
| `0x80A60670` | `fn_80a60670` | Gabon: speed reset and 25-frame timer | translated | high | `80a60670_FUN_80a60670.c` |
| `0x80A606E0` | `fn_80a606e0` | Gabon: delayed attack with sound event | translated | high | `80a606e0_FUN_80a606e0.c` |
| `0x80A60800` | `fn_80a60800` | Gabon: recovery blend and 60-frame timer | translated | high | `80a60800_FUN_80a60800.c` |
| `0x80A609A0` | `fn_80a609a0` | Gabon: full teardown and optional free | translated | high | `80a609a0_FUN_80a609a0.c` |
| `0x80ABC770` | `fn_80abc770` | LiftRotHalf: place model from flag-selected height | translated | medium | `80abc770_FUN_80abc770.c` |
| `0x80ABC940` | `fn_80abc940` | LiftRotHalf: read model position into actor | translated | high | `80abc940_FUN_80abc940.c` |

## Selection

### State tables (6 functions, 37 registrations)

The six tables left deferred by wave 4 are exactly the six now translated. All
of them use the existing [`register_state`] helper; the nine-word copy order
was re-checked against every table and no table-specific variant was needed.

- Gabon (6), HimanBros (11), SanboEl (8): the three large tables.
- Icicle (8): the parent/child ice family.
- LiftRotHalf (2) and RailPolyChild (2): the two small tables.

`register_sanbo_el_states` is the one table that seeds its descriptor block
before the first registration. It copies thirty-six read-only words from
`0x80B317C0..0x80B31884` into `0x80B77EE8..0x80B77F78`; the order is not
monotonic (the block reuses `0x80B317D8`, `0x80B31868`, `0x80B317E0` and
`0x80B317C0`), so the source list is preserved verbatim in
[`SANBO_EL_PARAM_WORDS`] and copied with a loop. The two `DamageJum` states
share a name prefix, matching the two distinct registrations in the export.

The Gabon and HimanBros death states each carry a tenth descriptor word
produced by a local factory (`0x80A5F520`/`0x80A5F510` and
`0x80A76160`/`0x80A76150`). The SanboEl `Ice` state does the same with
`0x80AFECE0`. Those words are passed through the `extra` argument of
[`register_state`].

### Constructors (6 functions)

Each constructor follows the section-3 shape: `0x801628C0(size)`, base
constructor `0x80094E80`, a class vtable at `+0x060`, then class-specific
sub-objects. Class labels come from the adjacent name-bearing table.

Two details are worth calling out:

- `alloc_sanbo_el_actor` initialises its `+0x544` holder with `0x8016A2F0`,
  not the `0x80169CD0` used by the other five classes. The different helper is
  preserved.
- `alloc_rail_poly_child_actor` installs a temporary vtable `0x8030533C`
  before the model block and the final vtable `0x80B5ABC0`; both stores are
  kept in order.

All six exports declare `int` returns, so the Rust signatures return `usize`
(the actor pointer, or zero when allocation fails). This differs from
`alloc_coin_actor`/`alloc_oneway_gate_actor` in wave 4, whose exports declare
`void`.

### Gabon handlers and helpers (16 functions)

The Gabon table's own callbacks (`0x80A5F450`, `0x80A5F4B0`) are only six
bytes apart from the handler cluster, so the handlers were selected as the run
`0x80A5FDF0..0x80A609A0`. They form a complete state machine:

- `fn_80a5fdf0` and `fn_80a5fe90` are the shared setup and probe primitives;
- `fn_80a60000`/`fn_80a601a0` bind the `net_walk2` clip and seed per-direction
  speed;
- `fn_80a60250` lazily initialises the two probe tables at
  `0x80B71EB0`/`0x80B71EB4` and switches on a zero probe;
- `fn_80a60360`, `fn_80a603d0`, `fn_80a60440`, `fn_80a60520`, `fn_80a60590`,
  `fn_80a60670`, `fn_80a606e0` and `fn_80a60800` implement the move, attack
  and death phases;
- `fn_80a609a0` is the complete destructor.

`fn_80a60000` and `fn_80a601a0` have the same 156-byte body. Both are kept as
separate functions because the export lists them at separate addresses and the
table binds them to different states.

The two `LiftRotHalf` helpers close the constructor's `+0x734` transform
object:

- `fn_80abc770` writes the model position, using a height offset selected from
  the `0x80B30020` table by bits of the actor flag word at `+0x04`;
- `fn_80abc940` reads the model position back into `+0xAC`/`+0xB0`.

`fn_80abc770` is the only new function marked `medium`: the bit masks
`& 0x1C` and `& 0x20` read naturally as byte offsets, but the Ghidra cast does
not prove the table element type. The address arithmetic is explicit; the
`f32` interpretation is inferred from the `(double)*(float *)` cast.

## Remaining work

### Deferred state tables (0)

None. All 43 `StateID` tables in the export now have a Rust registration
function.

### Unknowns preserved in code

1. **Byte-offset arithmetic.** Ghidra prints several tables as
   `&DAT_x + index * 4` or `(&DAT_x)[index * 2]`. Following the convention
   established in waves 3 and 4 and re-confirmed against the raw REL bytes,
   the multiplier is a **byte** offset: `0x80B2D230 + index * 4` and
   `0x80B71EB4 + index * 8`. No element scaling is applied a second time.
2. **Indirect calls with no printed argument.** `fn_80a60250`,
   `fn_80a60360`, `fn_80a60440`, `fn_80a60590` and `fn_80a606e0` call the
   `+0x1C` animation method. Ghidra sometimes prints an empty argument list;
   the receiver at `+0x544` is still live in `r3` and is passed explicitly,
   marked `// inferred:`.
3. **Sound-object method.** `fn_80a60590` and `fn_80a606e0` dispatch virtual
   slot `+0xA0` on the object at `+0x5F4` with the clip name
   `Wm_mr_sndbrakesmk` and a three-word position. The object type is not
   established, so the call stays an address-based virtual dispatch.
4. **Probe offsets.** `0x80B47D98`/`0x80B47D9C` and
   `0x80B47DB4`/`0x80B47DB8` are passed to [`fn_80a5fe90`] as data
   addresses, not decoded floats. Their values remain in the REL image.
5. **Class labels for constructors.** Vtable stores prove an actor
   constructor; the class name still comes from the adjacent state table. The
   profile registry in `d_profileNP` remains the final authority.
6. **Prologue/epilogue pseudo-calls.** Ghidra still shows calls into
   `0x802DCD70`/`0x802DCD74`/`0x802DCDBC`/`0x802DCDC0` around the larger
   functions. They are register save/restore fragments, not real calls, and are
   omitted as in waves 2 through 4.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`node tools/doc-lint.mjs src` reports 0 violations across 68 files. At the
time of this pass `cargo check` reports two pre-existing type errors in
`src/runtime/bt.rs` (lines 998 and 1080) and one warning in `src/runtime/bt.rs`
from a concurrent lane; `src/game/enemies.rs` compiles without errors or
warnings. This lane did not modify `runtime/bt.rs` or any file outside
`src/game/enemies.rs` and the two wave-5 records.
