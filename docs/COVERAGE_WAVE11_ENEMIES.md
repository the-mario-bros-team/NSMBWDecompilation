# Coverage - wave 11 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the eleventh translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 28 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the two contiguous
   runs and the three callbacks were found (the base-enemy framework, the
   `daEnBaramakiGesso_c` fire-cannon helpers, and three per-state callbacks
   installed by already-translated state tables).
3. **What is still missing?** - *Deferred neighbours* names the untranslated
   helpers that sit between or beside the selected bodies.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` =
  address only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, ABI detail or
  slot meaning is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_enemiesNP/export/`; that C
  file is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`; the entry
point is `0x80A000F0`. The REL has no embedded load address, and the retail
loader may place it anywhere. These are **not** runtime addresses.
`main.dol` addresses (`0x8000xxxx`, `0x8032xxxx`, `0x8042xxxx`) are imported
absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 11 | Wave 11 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Translated functions in `src/game/enemies.rs` | 192 | 28 | 220 |
| Functions selected from the base-enemy block | 0 | 14 | - |
| Functions selected from the BaramakiGesso block | 0 | 11 | - |
| State callbacks added | 0 | 3 | - |
| Selected bytes | 0 | 3,692 | - |

The 28 new bodies cover 3,692 bytes (0.61 % of the 602,844 bytes that the gap
report listed as untranslated before this pass) and 28 of the 3,363 remaining
function entries (0.83 %). Confidence is **24 high** and **4 medium**; there
are no `partial` or `stub` rows.

## Selection

This pass was evidence-first and continuation-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 28 bodies were written before
   any ledger row was added.
2. **Took the base-enemy framework as one run.** `0x80A00280` allocates
   `0x940` bytes, installs the class vtable `0x80B32E60`, and initialises the
   holders at `+0x5C4`, `+0x5FC`, `+0x608`, `+0x628`, `+0x728` and `+0x824`.
   The binder at `0x80A00EB0` fills exactly the `+0x5C4` and `+0x5FC` holders
   with the `shell_fish` model, so the constructor and the binder corroborate
   each other. The small transforms, the contact-damage sibling and the four
   per-player loops are the rest of that framework.
3. **Took the `daEnBaramakiGesso_c` fire-cannon helpers.** The already
   translated constructor at `0x80A07280` installs two helper objects with
   vtable `0x80329958` at `+0x744`/`+0x86C`. The run at
   `0x80A07580..0x80A07BA0` binds `fire_cannon` into the first helper, binds
   `fire_effect1x3_right` into the second, and emits
   `Wm_en_fireburner3ind`/`Wm_en_fireburner` through both. The callback and
   extra word at `0x80A06F30`/`0x80A06F90` belong to the same class and are
   installed by its state table at `0x80A06DA0`.
4. **Added three callbacks with exact owners.** `0x80AF7630`,
   `0x80B14560` and `0x80B151B0` are the callback words stored by
   `register_rotarion_ghost_states`, `register_toge_tekkyu_states` and
   `register_togezo_states`. Their bodies are the same guard/call shape as
   the callbacks translated in earlier waves.

### Base-enemy framework (`0x80A00280..0x80A00EB0`, 14 functions)

The constructor is followed by its state callback (`0x80A00360`), a vtable
helper (`0x80A00690`), the update delegate (`0x80A006C0`), the contact-damage
responder (`0x80A00710`) and the matrix/snapshot updater (`0x80A00940`). The
four per-player loops (`0x80A00AF0`, `0x80A00B80`, `0x80A00C10`,
`0x80A00C90`) and the three four-byte clears (`0x80A00A90`, `0x80A00AB0`,
`0x80A00AD0`) operate on the same slot arrays. The run closes with the
`shell_fish` binder.

### BaramakiGesso fire-cannon helpers (`0x80A06F30..0x80A07BA0`, 11 functions)

`0x80A07580` builds the `fire_cannon` model, `0x80A07600` binds its body and
clip to `+0x544`/`+0x588`, and `0x80A076E0` binds the selected
`fire_effect1x3_right` pair to `+0x5B4`/`+0x5F8`. `0x80A077D0` is the
per-frame holder update, `0x80A07900` advances the animation, `0x80A079A0`
and `0x80A07A30` emit the two burner effects, and `0x80A07AB0`/`0x80A07BA0`
write the body and effect transforms.

### State callbacks (3 functions)

Each body is the canonical `fState` callback: guard the null state, call
`0x8015F800(state, 0)`, call `0x802B9280(state)` only when the mode is
positive, and return the state unchanged.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A00280` | `fn_80a00280` | Base-enemy constructor (size `0x940`) | translated | high | `80a00280_FUN_80a00280.c` |
| `0x80A00360` | `fn_80a00360` | Base-enemy state callback | translated | high | `80a00360_FUN_80a00360.c` |
| `0x80A00690` | `fn_80a00690` | Run the `+0x544` holder's `+0x14` slot, return `1` | translated | high | `80a00690_FUN_80a00690.c` |
| `0x80A006C0` | `fn_80a006c0` | Advance animation, update `+0x824`, release effect slots | translated | high | `80a006c0_FUN_80a006c0.c` |
| `0x80A00710` | `fn_80a00710` | Contact-damage responder | translated | high | `80a00710_FUN_80a00710.c` |
| `0x80A00940` | `fn_80a00940` | Rebuild the matrix and snapshot animation matrices 6 and 5 | translated | high | `80a00940_FUN_80a00940.c` |
| `0x80A00A90` | `fn_80a00a90` | Clear `+0x936..+0x939` | translated | high | `80a00a90_FUN_80a00a90.c` |
| `0x80A00AB0` | `fn_80a00ab0` | Clear `+0x93C..+0x93F` | translated | high | `80a00ab0_FUN_80a00ab0.c` |
| `0x80A00AD0` | `fn_80a00ad0` | Clear `+0x92E..+0x931` | translated | high | `80a00ad0_FUN_80a00ad0.c` |
| `0x80A00AF0` | `fn_80a00af0` | Start per-player effects for slots in state `1` | translated | medium | `80a00af0_FUN_80a00af0.c` |
| `0x80A00B80` | `fn_80a00b80` | Poll four player slots and advance/release their effects | translated | medium | `80a00b80_FUN_80a00b80.c` |
| `0x80A00C10` | `fn_80a00c10` | Return slots in state `3` to `0` and release effects | translated | medium | `80a00c10_FUN_80a00c10.c` |
| `0x80A00C90` | `fn_80a00c90` | Release every non-zero per-player effect | translated | medium | `80a00c90_FUN_80a00c90.c` |
| `0x80A00EB0` | `fn_80a00eb0` | Bind `shell_fish` model, `close` clip and second clip | translated | high | `80a00eb0_FUN_80a00eb0.c` |
| `0x80A06F30` | `fn_80a06f30` | `daEnBaramakiGesso_c` state callback | translated | high | `80a06f30_FUN_80a06f30.c` |
| `0x80A06F90` | `fn_80a06f90` | Return the extra state word `0x80B7238C` | translated | high | `80a06f90_FUN_80a06f90.c` |
| `0x80A07580` | `fn_80a07580` | Bind the `fire_cannon` model | translated | high | `80a07580_FUN_80a07580.c` |
| `0x80A07600` | `fn_80a07600` | Bind the body holder and clip at `+0x544`/`+0x588` | translated | high | `80a07600_FUN_80a07600.c` |
| `0x80A076E0` | `fn_80a076e0` | Bind the indexed `fire_effect1x3_right` holder | translated | high | `80a076e0_FUN_80a076e0.c` |
| `0x80A077D0` | `fn_80a077d0` | Per-frame update for the body and effect holders | translated | high | `80a077d0_FUN_80a077d0.c` |
| `0x80A07900` | `fn_80a07900` | Advance the animation object at `+0x62C` | translated | high | `80a07900_FUN_80a07900.c` |
| `0x80A079A0` | `fn_80a079a0` | Emit the indexed `Wm_en_fireburner3ind` effect | translated | high | `80a079a0_FUN_80a079a0.c` |
| `0x80A07A30` | `fn_80a07a30` | Emit the fixed `Wm_en_fireburner` effect | translated | high | `80a07a30_FUN_80a07a30.c` |
| `0x80A07AB0` | `fn_80a07ab0` | Place the body holder at the actor position | translated | high | `80a07ab0_FUN_80a07ab0.c` |
| `0x80A07BA0` | `fn_80a07ba0` | Place the effect holder with the facing offset | translated | high | `80a07ba0_FUN_80a07ba0.c` |
| `0x80AF7630` | `fn_80af7630` | `daEnRotarionGhost_c` state callback | translated | high | `80af7630_FUN_80af7630.c` |
| `0x80B14560` | `fn_80b14560` | `daEnTogeTekkyu_c` state callback | translated | high | `80b14560_FUN_80b14560.c` |
| `0x80B151B0` | `fn_80b151b0` | `daEnTogezo_c` state callback | translated | high | `80b151b0_FUN_80b151b0.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A00D00` | 424 B | Paired-single/GQR noise; the export contains removed unreachable blocks and `ldexpf` artefacts |
| `0x80A07080` | 224 B | Recursive chain finder; the recursion shape needs a separate review |
| `0x80A07160` | 136 B | String-prefix comparison helper; role outside the fire-cannon flow |
| `0x80A07220` | 48 B | Prologue thunk shown as a call to `0x802DCBFC` |
| `0x80A07250` | 48 B | Prologue thunk shown as a call to `0x802DCBFC` |
| `0x80A073B0` | 12 B | Returns `0x80377A04`; no caller in this export |
| `0x80A073C0` | 12 B | Returns `0x80377A04`; no caller in this export |
| `0x80A07C80` | 16 B | Export reports bad instruction data and `halt_baddata()` |

These remain address-based calls or untouched entries; none is presented as a
new symbol.

## Unknowns preserved in code

1. **Base-enemy class label.** The constructor and the `shell_fish` binder are
   a matched pair, but the export carries no class symbol for this base. The
   function keeps `fn_<address>` and is described by role only.
2. **Per-player slot meanings.** The bytes at `+0x92E`, `+0x936` and `+0x93C`
   are used as small state machines (values `0`, `1`, `2`, `3`). The exact
   gameplay meaning is not recoverable from this export; the four functions
   are marked `medium`.
3. **`+0x90C` flag.** Set to `1` by the contact responder when the other
   participant blocks the hit but reports ground contact. Its consumer is
   outside this run.
4. **Fire-effect tables.** `0x80B33ED8`, `0x80B33EE0` and `0x80B33F40` are
   pointer tables in the data section. Their entries are not part of the text
   export, so the code keeps the table address and the actor index at `+0x740`.
5. **Snapshot target.** The six floats written to `+0x8E0..+0x8F4` are the
   translation words of matrices 6 and 5; their consumers are not in this
   block.
6. **Facing table.** `0x80B6E928`/`0x80B6E92C` are the X/Y pair of an
   eight-byte entry selected by the facing byte at `+0x348`. The per-entry
   compass meaning is inferred from the arithmetic.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `cargo check` succeeds and
`node tools/doc-lint.mjs src` reports 0 violations across 68 files. This lane
modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave11-enemies.csv`.
