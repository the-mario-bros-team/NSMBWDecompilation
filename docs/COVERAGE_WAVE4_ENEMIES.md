# Coverage - wave 4 third pass (`d_enemiesNP`)

## How to read this file

This page records the third translation pass over the enemy REL,
`d_enemiesNP`. It answers the same three questions as the earlier coverage
pages:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains why nineteen of the
   remaining state tables, eight of their constructors, and three damage
   responders were chosen.
3. **What is still missing?** - *Remaining work* lists the six state tables
   that remain deferred and the unknowns that were preserved rather than
   guessed.

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

| Metric | Wave 2 | Wave 3 | Wave 4 | Cumulative |
|---|---:|---:|---:|---:|
| Translated functions in `game/enemies.rs` | 22 | 24 | 30 | 76 |
| `StateID`-bearing state tables translated | 7 | 11 | 19 | 37 |
| State registrations covered | 59 | 24 | 108 | 191 |
| Actor constructors translated | 5 | 5 | 8 | 18 |
| Handlers / infrastructure translated | 10 | 8 | 3 | 21 |
| Deferred state tables | 36 | 25 | 6 | 6 |

The module still contains 3,555 function entries and 43 `StateID` tables in
total. This pass raises the table coverage from 18 to 37; the remaining six
are the largest or least-connected actors.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A41180` | `register_coin_states` | `daEnCoin_c`: 16 states from `Move` through `DieGoal` | translated | high | `80a41180_FUN_80a41180.c` |
| `0x80A5A350` | `register_fire_snake_states` | `daEnFireSnake_c`: 14 states from `Wait` through `DieFall` | translated | high | `80a5a350_FUN_80a5a350.c` |
| `0x80AE9990` | `register_puku_states` | `daEnPuku_c`: 15 swim/water/death states | translated | high | `80ae9990_FUN_80ae9990.c` |
| `0x80A729E0` | `register_hanachan_states` | `daEnHanachan_c`: `Walk`, `Angry`, `Dush`, `Down`, `Recover`, `DieFall` | translated | high | `80a729e0_FUN_80a729e0.c` |
| `0x80A99DC0` | `register_jump_puku_states` | `daEnJumpPuku_c`: 8 swim/jump/hit states | translated | high | `80a99dc0_FUN_80a99dc0.c` |
| `0x80AA5790` | `register_keronpa_states` | `daEnKeronpa_c`: 9 move/attack/death states | translated | high | `80aa5790_FUN_80aa5790.c` |
| `0x80ABBF00` | `register_lift_zen_toge_states` | `daEnLiftZenToge_c`: `Still`, `MoveN`, plus a null-state push | translated | high | `80abbf00_FUN_80abbf00.c` |
| `0x80ABFA90` | `register_manhole_choropu_states` | `daEnManholeChoropu_c`: 10 truncated state names | translated | high | `80abfa90_FUN_80abfa90.c` |
| `0x80AD9E60` | `register_oneway_gate_states` | `daEnOnewayGate_c`: `CloseW`, `Open`, `OpenWa`, `Close` | translated | high | `80ad9e60_FUN_80ad9e60.c` |
| `0x80ADFA90` | `register_pata_kuribo_states` | `daEnPataKuribo_c`: `Pata_W`, `Pata_T` | translated | high | `80adfa90_FUN_80adfa90.c` |
| `0x80AE3080` | `register_pata_states` | `daEnPata_c`: `UDfly`, `LRfly`, `PataJ`, `PataJTurn` | translated | high | `80ae3080_FUN_80ae3080.c` |
| `0x80AF1C00` | `register_redcoin_states` | `daEnRedcoin_c`: `Wait`, `Move`, `GetDemo`, `EatOut` | translated | high | `80af1c00_FUN_80af1c00.c` |
| `0x80AF5FD0` | `register_rot_block_states` | `daEnRotBlock_c`: `Search`, `Move` | translated | high | `80af5fd0_FUN_80af5fd0.c` |
| `0x80AF7520` | `register_rotarion_ghost_states` | `daEnRotarionGhost_c`: one truncated rotation state | translated | high | `80af7520_FUN_80af7520.c` |
| `0x80AF98A0` | `register_sakasa_togezo_states` | `daEnSakasaTogezo_c`: two `Saka_`, one `Slid_` | translated | high | `80af98a0_FUN_80af98a0.c` |
| `0x80B14220` | `register_toge_tekkyu_states` | `daEnTogeTekkyu_c`: `Wait`, `Move`, `Waki`, `InIceL` | translated | high | `80b14220_FUN_80b14220.c` |
| `0x80B150A0` | `register_togezo_states` | `daEnTogezo_c`: `Hold_Jugem` | translated | high | `80b150a0_FUN_80b150a0.c` |
| `0x80B213D0` | `register_block_help_states` | `daEnBlockHelp_c`: `Wait` | translated | high | `80b213d0_FUN_80b213d0.c` |
| `0x80B287E0` | `register_wrturn_states` | `daEnWrturn_c`: `Wait`, `Move` | translated | high | `80b287e0_FUN_80b287e0.c` |
| `0x80A42180` | `alloc_coin_actor` | Allocates/constructs a `0xAF8`-byte `daEnCoin_c` actor | translated | high | `80a42180_FUN_80a42180.c` |
| `0x80A5B1E0` | `alloc_fire_snake_actor` | Allocates/constructs a `0x5B8`-byte `daEnFireSnake_c` actor | translated | high | `80a5b1e0_FUN_80a5b1e0.c` |
| `0x80AEA910` | `alloc_puku_actor` | Allocates/constructs a `0x838`-byte `daEnPuku_c` actor | translated | high | `80aea910_FUN_80aea910.c` |
| `0x80A73260` | `alloc_hanachan_actor` | Allocates/constructs a `0x528`-byte `daEnHanachan_c` actor | translated | high | `80a73260_FUN_80a73260.c` |
| `0x80ADA320` | `alloc_oneway_gate_actor` | Allocates a `0x4BA0`-byte `daEnOnewayGate_c` actor via `0x80ADAB80` | translated | high | `80ada320_FUN_80ada320.c` |
| `0x80ADFDD0` | `alloc_pata_kuribo_actor` | Allocates/constructs a `0x838`-byte `daEnPataKuribo_c` actor | translated | high | `80adfdd0_FUN_80adfdd0.c` |
| `0x80B146E0` | `alloc_toge_tekkyu_actor` | Allocates/constructs a `0x9E0`-byte `daEnTogeTekkyu_c` actor | translated | high | `80b146e0_FUN_80b146e0.c` |
| `0x80B28B20` | `alloc_wrturn_actor` | Allocates/constructs a `0x748`-byte `daEnWrturn_c` actor | translated | high | `80b28b20_FUN_80b28b20.c` |
| `0x80A48390` | `fn_80a48390` | Crasher: binds the `damage` clip and blend constants | translated | high | `80a48390_FUN_80a48390.c` |
| `0x80A79B60` | `fn_80a79b60` | Ibaramushi: builds the model plus two damage holders | translated | high | `80a79b60_FUN_80a79b60.c` |
| `0x80A79CC0` | `fn_80a79cc0` | Ibaramushi: selects and binds an indexed damage clip | translated | high | `80a79cc0_FUN_80a79cc0.c` |

## Selection

### State tables (19 functions, 108 registrations)

These are the nineteen remaining tables with the strongest evidence and the
largest control-flow payoff. Every one uses the existing [`register_state`]
helper; its nine-word copy order was re-checked against each new table and no
table-specific variant was needed.

- multi-state actors: Coin (16), FireSnake (14), Puku (15), ManholeChoropu
  (10), Keronpa (9), JumpPuku (8), Hanachan (6), Pata (4), OnewayGate (4),
  Redcoin (4), TogeTekkyu (4), SakasaTogezo (3);
- small actors: LiftZenToge (2 plus a null push), PataKuribo (2), RotBlock (2),
  Wrturn (2), RotarionGhost (1), Togezo (1), BlockHelp (1).

`register_lift_zen_toge_states` is the exception: before its two named states
it pushes a null state with callback `0x80ABC310`, an address that is not a
function entry in `functions.tsv`. The push and the address are preserved
verbatim and marked unknown. The same table also copies ten six-word motion
templates from the read-only block `0x80B2FFB0..0x80B3000C`; every destination
word is written explicitly.

`register_fire_snake_states` and `register_keronpa_states` seed class-default
words before their registrations; `register_redcoin_states` writes four class
words after them. Those writes are kept because they are observable.

### Constructors (8 functions)

Each constructor follows the section-3 shape: `0x801628C0(size)`, a base
constructor, a class vtable at `+0x060`, then class-specific sub-objects.
Class labels come from the adjacent name-bearing table.

Two constructors are structurally distinct and are worth calling out:

- `alloc_puku_actor` uses base constructor `0x80066FC0`, not `0x80094E80`;
- `alloc_toge_tekkyu_actor` uses `0x80037EF0`, installs a temporary vtable
  `0x803063F8`, and invokes virtual slot `+0x98` on its `+0x89C` helper before
  installing the final vtable `0x80B694BC`.

`alloc_oneway_gate_actor` delegates the entire body to the local constructor
`0x80ADAB80`, which builds a two-element `0x2290`-byte container with
constructor `0x80AD5050` and destructor `0x80AD51C0`. `alloc_coin_actor` is
the largest at 384 bytes and builds two repeated model blocks.

The export declares `alloc_coin_actor` and `alloc_oneway_gate_actor` as
`void`; the Rust signatures follow the export, so the allocation result is not
returned even though a profile factory would normally want it.

### Damage responders (3 functions)

- `fn_80a48390` is the Crasher damage-animation binder. It uses the
  `crasher`/`g3d_crasher_brres` resources and the `damage` clip.
- `fn_80a79b60` builds the ibaramushi model and binds `wait_A_LmRm` plus two
  damage holders.
- `fn_80a79cc0` is the indexed damage-state selector for the same actor. It
  reads the clip-name table at `0x80B2DFA0` and stores the index at `+0xEA7`.

The two larger responders pass `double` blend constants. A small
`call3_f64`/`read_f64` adapter keeps those arguments in the original register
class instead of forcing them through the integer helpers. Their local stack
frames are modelled explicitly because the original passes overlapping pairs
of local words to `0x801650D0`, `0x80165C30`, `0x80166830` and `0x80167420`.

## Remaining work

### Deferred state tables (6)

| Class | Table address | Why still deferred |
|---|---|---|
| `daEnGabon_c` | `0x80A5EF70` | one large table; constructor and handler cluster not yet selected |
| `daEnHimanBros_c` | `0x80A75800` | multi-part actor; needs its container/part constructors |
| `daEnIcicle_c` | `0x80A7EF40` | table is coupled to the ice/particle helpers |
| `daEnLiftRotHalf_c` | `0x80ABB210` | two states; left for the lift cluster pass |
| `daEnRailPolyChild_c` | `0x80AECD40` | two states; rail-child ownership is unresolved |
| `daEnSanboEl_c` | `0x80AFE3C0` | large table with several damage names |

### Unknowns preserved in code

1. **Truncated state names.** ManholeChoropu keeps ten two-character prefixes,
   and RotarionGhost, SakasaTogezo, LiftZenToge and Keronpa each keep one or
   more truncated names. Suffixes are not guessed.
2. **Null-state callback.** `register_lift_zen_toge_states` pushes a null state
   with callback `0x80ABC310`; that address has no entry in `functions.tsv`.
3. **`alloc_coin_actor` return value.** The export declares `void`; the Rust
   signature is `()`. The same applies to `alloc_oneway_gate_actor`.
4. **Damage clip names.** `0x80B42818`, `0x80B42820`, `0x80B42824`,
   `0x80B4C864`, `0x80B4C870` and the `0x80B2DFA0` table are kept as data
   addresses. Their string contents are visible in the export but are not
   re-encoded in Rust.
5. **Blend constants.** `0x80B2C8A4` and `0x80B2C8C4` are read as `f64` from
   their synthetic addresses rather than baked in, preserving the original
   indirection.
6. **Class labels for constructors.** Vtable stores prove an actor
   constructor; the class name still comes from the adjacent state table. The
   profile registry in `d_profileNP` remains the final authority.
7. **Prologue/epilogue pseudo-calls.** Ghidra still shows calls into
   `0x802DCD6C`/`0x802DCD74`/`0x802DCDB8`/`0x802DCDC0` around the larger
   functions. They are register save/restore fragments, not real calls, and are
   omitted as in waves 2 and 3.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`doc-lint` reports 0 violations across 68 files. A clean rebuild
(`cargo clean -p nsmbw` followed by `cargo check`) is green with no warnings
from `game/enemies.rs`. During the pass a concurrent lane briefly left
`runtime/dvd.rs` with four unresolved `DVD_*` constants; that lane fixed them
before this report, and this lane did not modify that file.
