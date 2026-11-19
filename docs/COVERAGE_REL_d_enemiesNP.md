# Coverage - `d_enemiesNP` (REL module 3)

## How to read this file

This is the status page for the enemy-actor REL, `d_enemiesNP`. It answers
three questions:

1. **What was translated?** - the table below lists every function promoted to
   `src/game/enemies.rs`, with its original address, Rust name, plain-English
   role, status, confidence and Ghidra evidence file.
2. **Why these functions?** - the *Clusters* section explains the selection:
   state tables with surviving `StateID` strings, their actor constructors,
   and a small run of `daEnKuribo_c` handlers.
3. **What is still missing?** - *Deferred candidates* names the 36 state tables
   and the handler/helper clusters that the next pass should take.

Column meanings:

- **Status** - `translated` = control flow and every call are accounted for;
  `partial` = entry/exit and major effects only; `stub` = address only.
- **Confidence** - strength of the *evidence*, not code quality:
  `high` = bytes/flow are explicit and cross-checked; `medium` = flow is
  explicit but a role or state ownership is inferred; `low` = mostly guesswork.
- **Evidence** - file under
  `work/ghidra/rel-d_enemiesNP/export/`; that C file is the source of truth.

**Synthetic base warning.** All addresses below are analysis addresses from
`tools/rel2elf.mjs`, using the canonical base `0x80A00000`. The REL has no
embedded load address; the retail loader may place this module anywhere.
`0x80A000F0` is the entry point *in the synthetic image only*. Every Rust
function doc repeats this warning.

**Image layout used here.** Text starts at `0x80A000F0` and runs to
`0x80B2A518`; initialised data follows through `0x80B6E438`; BSS starts at
`0x80B6E500` and ends at `0x80B799E0`. The global state-list head is the first
BSS word, `0x80B6E500`.

**Evidence markers in the Rust:** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Range inventory

Source: `work/ghidra/rel-d_enemiesNP/export/functions.tsv` (3,555 rows) and the
3,555 matching `.c` files.

| Metric | Count |
|---|---:|
| Function entries in the module | 3,555 |
| `StateID`-bearing state tables | 43 |
| Translated by this pass | 22 |
| Actor state tables translated | 7 |
| Actor constructors translated | 5 |
| Handlers / infrastructure translated | 10 |
| Deferred state tables | 36 |

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A000F0` | `entry` | REL entry: passes the data-section descriptor to `0x80160A20`, calls the empty local prolog helper | translated | high | `80a000f0_entry.c` |
| `0x80A00160` | `fn_80a00160` | Pushes a three-word state registration onto the global list | translated | high | `80a00160_FUN_80a00160.c` |
| `0x80A00180` | `fn_80a00180` | Drains the list, calling each callback with `(state, -1)` | translated | high | `80a00180_FUN_80a00180.c` |
| `0x80A0EB10` | `register_beans_kuribo_states` | `daEnBeansKuribo_c`: `Walk`, `Turn`, `Aroun`, `DieFa` | translated | high | `80a0eb10_FUN_80a0eb10.c` |
| `0x80A2A350` | `register_bomhei_states` | `daEnBomhei_c`: 14 states from `Walk` through `EatOut` | translated | high | `80a2a350_FUN_80a2a350.c` |
| `0x80A49C60` | `register_crasher_states` | `daEnCrasher_c`: 9 states including `Damege`, `DieFumi`, `DieFall` | translated | high | `80a49c60_FUN_80a49c60.c` |
| `0x80A9D0F0` | `register_kanibo_states` | `daEnKanibo_c`: `Wait`, `RockUp`, `Jump`, `BackMove`, two deaths | translated | high | `80a9d0f0_FUN_80a9d0f0.c` |
| `0x80AA14E0` | `register_kanon_states` | `daEnKanon_c`: 9 states including `Damage` and three deaths | translated | high | `80aa14e0_FUN_80aa14e0.c` |
| `0x80AAA600` | `register_killer_states` | `daEnKiller_c`: 10 states including six death/eaten states | translated | high | `80aaa600_FUN_80aaa600.c` |
| `0x80AB4A50` | `register_kuribo_states` | `daEnKuribo_c`: 7 states including two `Born_Split` entries | translated | high | `80ab4a50_FUN_80ab4a50.c` |
| `0x80A4A730` | `alloc_crasher_actor` | Allocates/constructs a `0x718`-byte `daEnCrasher_c` actor | translated | high | `80a4a730_FUN_80a4a730.c` |
| `0x80A9D990` | `alloc_kanibo_actor` | Allocates/constructs a `0x5B0`-byte `daEnKanibo_c` actor | translated | high | `80a9d990_FUN_80a9d990.c` |
| `0x80AA1FB0` | `alloc_kanon_actor` | Allocates/constructs a `0x5E0`-byte `daEnKanon_c` actor | translated | high | `80aa1fb0_FUN_80aa1fb0.c` |
| `0x80AAB210` | `alloc_killer_actor` | Allocates/constructs a `0xE48`-byte `daEnKiller_c` actor with three containers | translated | high | `80aab210_FUN_80aab210.c` |
| `0x80AB5140` | `alloc_kuribo_actor` | Allocates/constructs a `0x578`-byte `daEnKuribo_c` actor | translated | high | `80ab5140_FUN_80ab5140.c` |
| `0x80AB4FC0` | `fn_80ab4fc0` | Common `daEnKuribo_c` state callback (`0x8015F800`, optional `0x802B9280`) | translated | high | `80ab4fc0_FUN_80ab4fc0.c` |
| `0x80AB5E50` | `fn_80ab5e50` | Copies a nine-word hit/attack block to `+0x414..+0x432` and marks it active | translated | high | `80ab5e50_FUN_80ab5e50.c` |
| `0x80AB5F20` | `fn_80ab5f20` | Kuribo per-frame handler: animation query, speed damping, timer, state switch | translated | medium | `80ab5f20_FUN_80ab5f20.c` |
| `0x80AB60A0` | `fn_80ab60a0` | Kuribo walking/turning handler with ground test and state switch | translated | medium | `80ab60a0_FUN_80ab60a0.c` |
| `0x80AB6280` | `fn_80ab6280` | Kuribo running/turning handler with airborne effect branch | translated | medium | `80ab6280_FUN_80ab6280.c` |
| `0x80AB6430` | `fn_80ab6430` | Kuribo death/removal: particle burst, sound `0x2CF`, retire call | translated | medium | `80ab6430_FUN_80ab6430.c` |
| `0x80A77540` | `fn_80a77540` | Contact callback: player-character id bookkeeping and damage call `0x80056370` | translated | medium | `80a77540_FUN_80a77540.c` |

## Clusters

### State-table machinery (3 functions)

`entry` calls `0x80160A20` with the end-of-text data pointer, then the empty
`0x80A001E0`. `fn_80a00160`/`fn_80a00180` are the intrusive-list push and
drain. Every state table uses `0x8015F7C0` to build a name object
(vtable, `char*`, integer id), overwrites the vtable, copies nine words from a
read-only parameter block, and calls `0x80A00160` with a per-actor callback.

The nine-word copy order is not address order; it is
`+0x24, +0x28, +0x2C, +0x18, +0x1C, +0x20, +0x0C, +0x10, +0x14`. This is
implemented once in `register_state` and reused by all seven tables.

### State tables (7 tables, 59 registrations)

| Class | States | Damage/death coverage | Extra-callback states |
|---|---:|---|---|
| `daEnBeansKuribo_c` | 4 | `DieFa` | `DieFa` |
| `daEnBomhei_c` | 14 | `Explode`, `AfterIce`, `InIceLump`, `EatOut` | `Carry`, `EatOut` |
| `daEnCrasher_c` | 9 | `Damege`, `DieFumi`, `DieFall` | both deaths |
| `daEnKanibo_c` | 6 | `DieFumi`, `DieFall` | both deaths |
| `daEnKanon_c` | 9 | `Damage`, `FumiStop`, `SpinDie`, `DieOther`, `DieFall` | two deaths |
| `daEnKiller_c` | 10 | six death/eaten states | six death/eaten states |
| `daEnKuribo_c` | 7 | `Down`, `Dokan_Down`, `EatOut_Rea` | none |

The tables are strong evidence because the engine's own strings survive:
`daEn<Class>_c::StateID_<Name>`. Two `daEnKuribo_c` names are truncated by the
export to 16 characters (`Born_Split...`); the differing suffixes are not
recoverable from the symbol list.

### Constructors (5 functions)

Each constructor follows the same shape: `0x801628C0(size)`, base actor
constructor `0x80094E80`, class vtable at `+0x060`, then class-specific
sub-objects. The class identity is inferred from the immediately adjacent,
name-bearing state table, and is reinforced by the class vtable sitting just
after that table's state vtables (for example `0x80B556A8` follows the
`daEnKuribo_c` state vtable `0x80B55640`). Confidence is high for the
construction flow and the cluster label; the profile registry in
`d_profileNP` remains the final authority.

### Kuribo handlers (5 functions + callback)

The exact `StateID` that owns each handler is **not** recoverable: state
records point at handlers indirectly through their vtable, and the export has
no symbol for those methods. The functions keep `fn_<address>` names. They all
sit in the `0x80AB...` block immediately after the `daEnKuribo_c` table and
share helpers `0x80AB5B40`, `0x80AB5BD0`, `0x80AB5C70`, `0x80AB5D30` and the
state-switch vtable slot `+0xD4`.

### Contact damage (1 function)

`fn_80a77540` is one of only two functions in the module that call
`0x80056370(victim, attacker, 2)`. It also copies a player-character id into
the owner's `+0x8A0..+0x8A9` block. The class that owns the callback is not
identified; it lies in the `0x80A7...` actor block, between the Puku and Pata
clusters.

## Deferred candidates

The following are deliberately **not** translated in this pass. They are good
next targets because each has the same high-evidence `StateID` shape as the
seven tables above.

### State tables (36)

`daEnBuku_c` (`0x80A05500`), `daEnBaramakiGesso_c` (`0x80A06DA0`),
`daEnBlackPakkun_c` (`0x80A1BDA0`), `daEnBounceBall_c` (`0x80A2CC40`),
`daEnCliffChikuwa_c` (`0x80A399E0`), `daEnCoin_c` (`0x80A41180`, 16 states),
`daEnCoinFloor_c` (`0x80A43C00`), `daEnCoinJugem_c` (`0x80A444F0`),
`daEnFireSnake_c` (`0x80A5A350`, 14 states),
`daEnFixationIcicle_c` (`0x80A5B900`), `daEnGabon_c` (`0x80A5EF70`),
`daEnGuraRock_c` (`0x80A6D180`), `daEnGuraYuka_c` (`0x80A6E240`),
`daEnHanachan_c` (`0x80A729E0`), `daEnHimanBros_c` (`0x80A75800`),
`daEnIcicle_c` (`0x80A7EF40`), `daEnJimenFPakkun_c` (`0x80A90740`),
`daEnJumpPuku_c` (`0x80A99DC0`), `daEnKeronpa_c` (`0x80AA5790`),
`daEnLiftRotHalf_c` (`0x80ABB210`), `daEnLiftZenToge_c` (`0x80ABBF00`),
`daEnManholeChoropu_c` (`0x80ABFA90`), `daEnOnewayGate_c` (`0x80AD9E60`),
`daEnPataKuribo_c` (`0x80ADFA90`), `daEnPata_c` (`0x80AE3080`),
`daEnPuku_c` (`0x80AE9990`, 15 states), `daEnRailPolyChild_c` (`0x80AECD40`),
`daEnRedcoin_c` (`0x80AF1C00`), `daEnRotBlock_c` (`0x80AF5FD0`),
`daEnRotarionGhost_c` (`0x80AF7520`), `daEnSakasaTogezo_c` (`0x80AF98A0`),
`daEnSanboEl_c` (`0x80AFE3C0`), `daEnTogeTekkyu_c` (`0x80B14220`),
`daEnTogezo_c` (`0x80B150A0`), `daEnBlockHelp_c` (`0x80B213D0`),
`daEnWrturn_c` (`0x80B287E0`).

### Constructors and handlers

- The remaining actor constructors can be found by the shared
  `0x801628C0(size)` prologue; the address list is in `functions.tsv`.
- Kuribo helpers `0x80AB5B40`, `0x80AB5BD0`, `0x80AB5C70`, `0x80AB5D30` and the
  per-state callbacks `0x80A0EE70`, `0x80A0EED0`, `0x80A2AE10`, `0x80A2AE70`,
  `0x80A4A370`, `0x80A4A3D0`, `0x80A9D5D0`, `0x80A9D630`,
  `0x80AA1BF0`, `0x80AA1C50`, `0x80AAAE10`, `0x80AAAE70`.
- The extra-callback factories
  `0x80A0EF30`, `0x80A2AEE0`, `0x80A2AED0`, `0x80A4A440`, `0x80A4A430`,
  `0x80A9D6A0`, `0x80A9D690`, `0x80AA1CC0`, `0x80AA1CB0`,
  `0x80AAAF20`..`0x80AAAED0`.
- The early framework block `0x80A00280`..`0x80A00D00` (base enemy constructor,
  `0x80A00710` contact-damage sibling, mode helpers).
- Collision/damage responders throughout the `0x80A7...` range; only
  `0x80A77540` is translated here.

## Uncertainties

1. **Class labels for constructors.** The vtable stores prove an actor
   constructor, but the class name comes from the adjacent state table. The
   profile registry in `d_profileNP` is the authority and is another lane.
2. **Handler ownership.** `fn_80ab5f20`, `fn_80ab60a0`, `fn_80ab6280` and
   `fn_80ab6430` are in the Kuribo cluster but cannot be matched to a
   `StateID` from this export. They are marked `medium`.
3. **Opaque callees.** Calls such as `0x8006CD90`, `0x800955F0`,
   `0x8006A4B0` and `0x80198C30` are kept by address. Their names and exact
   ABI are outside this lane; argument counts follow the Ghidra call site.
4. **Data-section constants.** Speed/timer constants are read from their
   synthetic addresses (`0x80B2FCE4`, `0x80B2FD50`, ...) instead of being
   baked in, because the REL data values are relocated and not part of the
   text export. This preserves the original indirection.
5. **Prologue/epilogue pseudo-calls.** Ghidra splits the register save blocks
   at `0x802DCD68`..`0x802DCDC0` into callable fragments and shows them as
   calls at the start/end of every state table. They are not real calls and
   are omitted, with a note in each table's Rust section.
6. **Truncated names.** Two `daEnKuribo_c` `Born_Split` symbols lose their
   suffixes after 16 characters; the table treats them as distinct states, as
   the original does.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

Both pass for this lane. `cargo check` may report warnings from other lanes'
files; `doc-lint` reports 0 violations across 67 files.
