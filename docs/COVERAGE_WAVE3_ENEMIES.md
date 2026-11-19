# Coverage - wave 3 second pass (`d_enemiesNP`)

## How to read this file

This page records the second translation pass over the enemy REL,
`d_enemiesNP`. It answers the same three questions as the
[first-pass coverage page](COVERAGE_REL_d_enemiesNP.md):

1. **What was added?** - the table below lists the 24 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - the *Selection* section explains why eleven of
   the 36 deferred state tables, five of their constructors and one small
   helper cluster were chosen.
3. **What is still missing?** - *Remaining work* lists the tables that remain
   deferred and the unknowns that were preserved rather than guessed.

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

| Metric | Wave 2 | Wave 3 | Cumulative |
|---|---:|---:|---:|
| Translated functions in `game/enemies.rs` | 22 | 24 | 46 |
| `StateID`-bearing state tables translated | 7 | 11 | 18 |
| State registrations covered | 59 | 24 | 83 |
| Actor constructors translated | 5 | 5 | 10 |
| Handlers / infrastructure translated | 10 | 8 | 18 |
| Deferred state tables | 36 | -11 | 25 |

The module still contains 3,555 function entries and 43 `StateID` tables in
total; this pass intentionally does not claim complete actor coverage.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A05500` | `register_buku_states` | `daEnBuku_c`: `Swim`, `SrcSwim`, `AttackSwim`, `AttackWait`, `DieFall` | translated | high | `80a05500_FUN_80a05500.c` |
| `0x80A06DA0` | `register_baramaki_gesso_states` | `daEnBaramakiGesso_c`: one truncated `..._At` state | translated | high | `80a06da0_FUN_80a06da0.c` |
| `0x80A1BDA0` | `register_black_pakkun_states` | `daEnBlackPakkun_c`: `IceWa...`, `Move` | translated | high | `80a1bda0_FUN_80a1bda0.c` |
| `0x80A2CC40` | `register_bounce_ball_states` | `daEnBounceBall_c`: support object, container, null-state push, `Move` | translated | high | `80a2cc40_FUN_80a2cc40.c` |
| `0x80A399E0` | `register_cliff_chikuwa_states` | `daEnCliffChikuwa_c`: `Wait`, `Down`, `Zoom` | translated | high | `80a399e0_FUN_80a399e0.c` |
| `0x80A43C00` | `register_coin_floor_states` | `daEnCoinFloor_c`: `FloorSe...`, `AngleMo...` | translated | high | `80a43c00_FUN_80a43c00.c` |
| `0x80A444F0` | `register_coin_jugem_states` | `daEnCoinJugem_c`: `JugemHo...`, `Jugemth...` | translated | high | `80a444f0_FUN_80a444f0.c` |
| `0x80A5B900` | `register_fixation_icicle_states` | `daEnFixationIcicle_c`: one truncated `Wa...` state | translated | high | `80a5b900_FUN_80a5b900.c` |
| `0x80A6D180` | `register_gura_rock_states` | `daEnGuraRock_c`: `Wait`, `Fall`, `Still` | translated | high | `80a6d180_FUN_80a6d180.c` |
| `0x80A6E240` | `register_gura_yuka_states` | `daEnGuraYuka_c`: `Wait` | translated | high | `80a6e240_FUN_80a6e240.c` |
| `0x80A90740` | `register_jimen_fpakkun_states` | `daEnJimenFPakkun_c`: `Wai...`, two truncated `Atta...` states | translated | high | `80a90740_FUN_80a90740.c` |
| `0x80A05CC0` | `alloc_buku_actor` | Allocates/constructs a `0x1168`-byte `daEnBuku_c` actor | translated | high | `80a05cc0_FUN_80a05cc0.c` |
| `0x80A07280` | `alloc_baramaki_gesso_actor` | Allocates/constructs a `0x998`-byte `daEnBaramakiGesso_c` actor | translated | high | `80a07280_FUN_80a07280.c` |
| `0x80A1C0E0` | `alloc_black_pakkun_actor` | Allocates/constructs a `0x828`-byte `daEnBlackPakkun_c` actor | translated | high | `80a1c0e0_FUN_80a1c0e0.c` |
| `0x80A2D2D0` | `alloc_bounce_ball_actor` | Allocates/constructs a `0x528`-byte `daEnBounceBall_c` actor | translated | high | `80a2d2d0_FUN_80a2d2d0.c` |
| `0x80A39DE0` | `alloc_cliff_chikuwa_actor` | Allocates/constructs a `0x9A60`-byte `daEnCliffChikuwa_c` actor with six containers | translated | high | `80a39de0_FUN_80a39de0.c` |
| `0x80A05AC0` | `fn_80a05ac0` | Walks a five-link object chain via virtual predicate `+0x0C`, then recurses | translated | medium | `80a05ac0_FUN_80a05ac0.c` |
| `0x80A05BA0` | `fn_80a05ba0` | Colon-suffix name comparison through virtual slot `+0x20` | translated | medium | `80a05ba0_FUN_80a05ba0.c` |
| `0x80A05F80` | `fn_80a05f80` | Fills 128 `0x14`-byte records from one sample and seeds `+0xA00` | translated | high | `80a05f80_FUN_80a05f80.c` |
| `0x80A06030` | `fn_80a06030` | Appends one record at the rotating index in `+0xA05` | translated | high | `80a06030_FUN_80a06030.c` |
| `0x80A06080` | `fn_80a06080` | Ages four selection bytes toward staggered limits | translated | high | `80a06080_FUN_80a06080.c` |
| `0x80A060E0` | `fn_80a060e0` | Moves four selection bytes toward staggered targets | translated | high | `80a060e0_FUN_80a060e0.c` |
| `0x80A06130` | `fn_80a06130` | Copies the record selected by a masked index difference | translated | high | `80a06130_FUN_80a06130.c` |
| `0x80A06190` | `fn_80a06190` | Counts up to four live chain slots, returning `-1` when full | translated | medium | `80a06190_FUN_80a06190.c` |

## Selection

### State tables (11 functions, 24 registrations)

The first pass deferred 36 tables that share the same high-evidence shape: a
`0x8015F7C0` name object, a vtable overwrite, a nine-word parameter copy and a
`0x80A00160` push. Wave 3 takes eleven of them, spanning small one-state actors
and multi-state actors:

- one state: BaramakiGesso, BounceBall, FixationIcicle, GuraYuka;
- two states: BlackPakkun, CoinFloor, CoinJugem;
- three states: CliffChikuwa, GuraRock, JimenFPakkun;
- five states: Buku.

All use the existing [`register_state`] helper. The helper's copy order was
re-verified against each new table: destination `+0x0C..+0x2C` receives the
source block in the order `6,7,8,3,4,5,0,1,2`, which is exactly the sequence
implemented once in the helper. No table-specific copy variant was needed.

`daEnBounceBall_c` is the exception: before its single `Move` registration it
builds a six-word support object, a six-slot container with `0x28`-byte
elements, and a second push with a null state object. The null push and its
callback address `0x80A2CF00` are preserved verbatim and marked unknown because
that address has no entry in `functions.tsv`.

### Constructors (5 functions)

Each constructor follows the section-3 shape: `0x801628C0(size)`, base
constructor `0x80094E80`, class vtable at `+0x060`, then class-specific
sub-objects. Class labels are inferred from the immediately adjacent
name-bearing table. Two constructors are larger than usual:

- `alloc_buku_actor` builds a `0x14`-by-`0x80` record container at `+0x74C`
  and stores a self-pointer at `+0x748`;
- `alloc_cliff_chikuwa_actor` allocates `0x9A60` bytes and builds six
  containers of 500 slots each.

`alloc_baramaki_gesso_actor` and `alloc_cliff_chikuwa_actor` have no return
value in the Ghidra output, so the Rust functions return `()`; the allocation
result is used only inside the constructor. This is faithful to the export,
even though a profile factory would normally want the pointer back.

### Record-chain helpers (8 functions)

The `0x80A05...` block immediately after the Buku table contains a coherent
record manager: a chain walk, a name comparator, a 128-record fill, an append,
two ageing passes, a selected-record copy and a live-slot counter. These are
translated as `fn_<address>` because the export has no method symbols. Their
cluster placement is inferred; **no helper is assigned to a `StateID`**.

## Remaining work

### Deferred state tables (25)

`daEnCoin_c` (`0x80A41180`, 16 states), `daEnFireSnake_c` (`0x80A5A350`, 14
states), `daEnPuku_c` (`0x80AE9990`, 15 states), `daEnGabon_c` (`0x80A5EF70`),
`daEnHanachan_c` (`0x80A729E0`), `daEnHimanBros_c` (`0x80A75800`),
`daEnIcicle_c` (`0x80A7EF40`), `daEnJumpPuku_c` (`0x80A99DC0`),
`daEnKeronpa_c` (`0x80AA5790`), `daEnLiftRotHalf_c` (`0x80ABB210`),
`daEnLiftZenToge_c` (`0x80ABBF00`), `daEnManholeChoropu_c` (`0x80ABFA90`),
`daEnOnewayGate_c` (`0x80AD9E60`), `daEnPataKuribo_c` (`0x80ADFA90`),
`daEnPata_c` (`0x80AE3080`), `daEnRailPolyChild_c` (`0x80AECD40`),
`daEnRedcoin_c` (`0x80AF1C00`), `daEnRotBlock_c` (`0x80AF5FD0`),
`daEnRotarionGhost_c` (`0x80AF7520`), `daEnSakasaTogezo_c` (`0x80AF98A0`),
`daEnSanboEl_c` (`0x80AFE3C0`), `daEnTogeTekkyu_c` (`0x80B14220`),
`daEnTogezo_c` (`0x80B150A0`), `daEnBlockHelp_c` (`0x80B213D0`),
`daEnWrturn_c` (`0x80B287E0`).

The remaining constructors can be found by the shared `0x801628C0(size)`
prologue; the address list is in `functions.tsv`.

### Unknowns preserved in code

1. **Truncated state names.** Seven new names end at the export's 16-character
   limit (`IceWa...`, `FloorSe...`, `AngleMo...`, `JugemHo...`, `Jugemth...`,
   `Wa...`, `Atta...`). Their suffixes are not recoverable from the symbol
   list and are not guessed.
2. **Null-state registration.** `register_bounce_ball_states` pushes a null
   state with callback `0x80A2CF00`; that address is not a function entry in
   the export, so its role is unresolved.
3. **Helper ownership.** The eight `0x80A05...` helpers are not mapped to any
   `StateID` or class. They are documented by argument shape and field offsets
   only.
4. **Callee ABI in `fn_80a06190`.** `0x80162D00` is shown with no arguments;
   the Rust keeps the parameterless call and marks the input unknown.
5. **Class labels for constructors.** Vtable stores prove an actor
   constructor; the class name still comes from the adjacent state table. The
   profile registry in `d_profileNP` is the final authority.
6. **Prologue/epilogue pseudo-calls.** Ghidra still shows calls into
   `0x802DCD74`/`0x802DCDC0` around the larger functions. They are register
   save/restore fragments, not real calls, and are omitted as in wave 2.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`doc-lint` reports 0 violations across 68 files. `cargo check` for
`game/enemies.rs` was green when this pass landed, and the module still
compiles in isolation against a stub `platform` layer. A whole-crate
`cargo check` at report time is red from concurrent edits in other lanes
(`runtime/dvd.rs`, `game/stage.rs`, `game/player.rs`, `ui/hud.rs`); none of
those diagnostics point at `game/enemies.rs`, and they are not modified by this
lane.
