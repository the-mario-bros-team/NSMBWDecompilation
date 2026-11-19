# Coverage - wave 19 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the nineteenth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 26 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found:
   the clean neighbours that wave 17 deferred, two complete class blocks
   around surviving `StateID` tables, and the rest of the `0xEB0` helper
   block.
3. **What is still missing?** - *Deferred neighbours* names the untranslated
   entries around the selected ranges and why each was left out.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = the body is represented but a hidden ABI operand
  prevents a complete model; `stub` = address only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, ABI detail or
  receiver is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_enemiesNP/export/`; that C file
  is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`; the entry
point is `0x80A000F0`. The REL has no embedded load address, and the retail
loader may place it anywhere. These are **not** runtime addresses.
`main.dol` addresses (`0x8000xxxx`, `0x803xxxxx`, `0x8042xxxx`) are imported
absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 19 | Wave 19 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 392 | 26 | 418 |
| Of those, `translated` | 377 | 26 | 403 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| `daEnFireSnake_c` helpers | - | 4 | - |
| `daEnFixationIcicle_c` helpers | - | 3 | - |
| `daEnIcicle_c` constructor/handlers | - | 8 | - |
| `0xEB0` actor handlers | - | 6 | - |
| State callbacks / predicates | - | 5 | - |
| Selected bytes | - | 4,288 | - |

The 26 new bodies cover 4,288 bytes (0.75 % of the 572,464 bytes that the gap
report listed as untranslated before this pass) and 26 of the 3,163 remaining
function entries (0.82 %). Confidence is **21 high** and **5 medium**. There
are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 418 bodies, 403 `translated`, 15 `partial`, 3,137
function entries and 568,176 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 26 bodies were written before
   any ledger row was added. The only pre-existing ledger mention of one of
   these addresses is the callback word in the `0x80A5B900` row; that row
   describes the state table, not a body at `0x80A5BA10`.
2. **Finished the `0xEB0` block.** Wave 17 named `0x80A7A700`,
   `0x80A7A7A0`, `0x80A7A810`, `0x80A7B370`, `0x80A7BB00` and `0x80A7BBF0`
   as clean next targets. The release/reset loops, the position-sync writer,
   the `wait_A_LmRm` binder and the walk/footstep handler all use offsets
   inside the same `0xEB0`-byte actor that `0x80A79460` constructs, so the
   block link is structural rather than name-based.
3. **Took the `daEnFireSnake_c` block.** The state table at `0x80A5A350`
   survives with fourteen `daEnFireSnake_c::StateID_*` strings, and the
   constructor at `0x80A5B1E0` allocates `0x5B8` bytes. Between them sit four
   clean helpers: the name-suffix predicate `0x80A5B0C0`, the contact
   callback `0x80A5B420`, the transform writer `0x80A5B620` and the
   `obj_turara_kotei` model binder `0x80A5B700`. The variant word at
   `+0x5B0` is the last word of the `0x5B8`-byte object, which ties the binder
   to this constructor.
4. **Took the `daEnFixationIcicle_c` block.** The table at `0x80A5B900`
   preserves `daEnFixationIcicle_c::StateID_Wa...` and names
   `0x80A5BA10` as its callback. The constructor at `0x80A5BB90` allocates
   `0x680` bytes and installs the adjacent class vtable; `0x80A5BCE0` is the
   second callback used by the same constructor's list.
5. **Took the `daEnIcicle_c` block.** The table at `0x80A7EF40` names
   `0x80B731B8`/`0x80B731F8`/`0x80B73238`/`0x80B73278`/`0x80B732B8`/
   `0x80B732F8`/`0x80B73338`/`0x80B73378` as `ParentWait`, `ParentChild`,
   `ParentCrea`, `ParentDel`, `AnmWait`, `DownWait`, `Vibration` and
   `DownMove`. The constructor `0x80A7DCF0` allocates `0x628` bytes, and the
   handlers at `0x80A7E190..0x80A7EA40` reference those descriptors
   directly. The model binder uses `obj_turara_rakka`, i.e. the falling
   icicle, which matches the class name.
6. **Included the two duplicated registration callbacks.** `0x80A7CF70`
   and `0x80A7CFD0` are byte-identical state callbacks and are kept as two
   bodies because the original has two independent function entries.
7. **Included the chain predicate and its name companion.** `0x80A7D130`
   walks five `+0x30` child links and recurses; `0x80A7D210` is the
   name-suffix predicate used by the adjacent `daEnIcicle_c` state logic.
8. **Excluded every export with damaged control flow.** The GQR/`ldexpf`
   artefacts in `0x80A7AB90`, `0x80A7ACA0`, `0x80A7ADA0`, `0x80A7AEF0`,
   `0x80A7BD30`, `0x80A7BEC0`, `0x80A7C600`, `0x80A7D760`, `0x80A7E640`,
   `0x80A7EB60` and `0x80A7FB50` were not guessed. `0x80A7C7D0` was left out
   because its stack-slot comparisons cannot be modelled without inventing
   operands. `0x80A7ED20` calls the `halt_baddata()` export at `0x80A7E630`,
   so its tail cannot be verified.
9. **Read the data constants from the REL image.** Address minus base equals
   the file offset for this module, so the `f32`/`u16` values were checked
   directly. The Rust keeps the original data addresses instead of baking
   the values in.

### The selected blocks (`0x80A5B0C0..0x80A7FCE0`, 26 functions)

The FireSnake group is four helpers around an existing constructor and state
table. The FixationIcicle group is its callback plus constructor. The Icicle
group starts at `0x80A7DCF0` and covers the constructor, contact callback,
model binder, break effect and four state helpers. The `0xEB0` group is the
release/reset/sync/wait tail that wave 17 deferred.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A5B0C0` | `fn_80a5b0c0` | Compare the state-name suffix after the last `:` | translated | medium | `80a5b0c0_FUN_80a5b0c0.c` |
| `0x80A5B420` | `fn_80a5b420` | FireSnake contact callback via the other actor's `+0x3F0`/`+0x3F4` | translated | high | `80a5b420_FUN_80a5b420.c` |
| `0x80A5B620` | `fn_80a5b620` | Rebuild the FireSnake matrix from position `+0xAC` and angles `+0x100` | translated | high | `80a5b620_FUN_80a5b620.c` |
| `0x80A5B700` | `fn_80a5b700` | Load `obj_turara_kotei` and bind its body/damage clips | translated | high | `80a5b700_FUN_80a5b700.c` |
| `0x80A5BA10` | `fn_80a5ba10` | `daEnFixationIcicle_c` registration callback | translated | high | `80a5ba10_FUN_80a5ba10.c` |
| `0x80A5BB90` | `alloc_fixation_icicle_actor` | Allocate and construct the `0x680`-byte FixationIcicle actor | translated | high | `80a5bb90_FUN_80a5bb90.c` |
| `0x80A5BCE0` | `fn_80a5bce0` | FixationIcicle list-element destructor with optional release | translated | medium | `80a5bce0_FUN_80a5bce0.c` |
| `0x80A7A700` | `fn_80a7a700` | Release the `0xEB0` actor's embedded list records | translated | medium | `80a7a700_FUN_80a7a700.c` |
| `0x80A7A7A0` | `fn_80a7a7a0` | Reset the ten `0xA4`-byte list records | translated | high | `80a7a7a0_FUN_80a7a7a0.c` |
| `0x80A7A810` | `fn_80a7a810` | Reset the two tail list records | translated | high | `80a7a810_FUN_80a7a810.c` |
| `0x80A7B370` | `fn_80a7b370` | Copy target position and reset the four speed words | translated | high | `80a7b370_FUN_80a7b370.c` |
| `0x80A7BB00` | `fn_80a7bb00` | Bind the `wait_A_LmRm` clip and enter the walk holder state | translated | high | `80a7bb00_FUN_80a7bb00.c` |
| `0x80A7BBF0` | `fn_80a7bbf0` | Per-frame walk handler with footstep `0x1E1` and state switch | translated | high | `80a7bbf0_FUN_80a7bbf0.c` |
| `0x80A7CF70` | `fn_80a7cf70` | State registration callback | translated | high | `80a7cf70_FUN_80a7cf70.c` |
| `0x80A7CFD0` | `fn_80a7cfd0` | Duplicate state registration callback | translated | high | `80a7cfd0_FUN_80a7cfd0.c` |
| `0x80A7D130` | `fn_80a7d130` | Walk five `+0x30` child links while vtable `+0x0C` returns zero | translated | medium | `80a7d130_FUN_80a7d130.c` |
| `0x80A7D210` | `fn_80a7d210` | Compare the state-name suffix after the last `:` | translated | medium | `80a7d210_FUN_80a7d210.c` |
| `0x80A7DCF0` | `fn_80a7dcf0` | Allocate and construct the `0x628`-byte `daEnIcicle_c` actor | translated | high | `80a7dcf0_FUN_80a7dcf0.c` |
| `0x80A7E190` | `fn_80a7e190` | Icicle contact callback via the other actor's `+0x3F0`/`+0x3F4` | translated | high | `80a7e190_FUN_80a7e190.c` |
| `0x80A7E3B0` | `fn_80a7e3b0` | Load `obj_turara_rakka` and bind three animation sets | translated | high | `80a7e3b0_FUN_80a7e3b0.c` |
| `0x80A7E780` | `fn_80a7e780` | Spawn the two break effects, play sound `0x2DA` and retire | translated | high | `80a7e780_FUN_80a7e780.c` |
| `0x80A7E8E0` | `fn_80a7e8e0` | `ParentChild` frame helper with effect respawn | translated | high | `80a7e8e0_FUN_80a7e8e0.c` |
| `0x80A7E960` | `fn_80a7e960` | Mark the resolved effect object finished | translated | high | `80a7e960_FUN_80a7e960.c` |
| `0x80A7E9B0` | `fn_80a7e9b0` | Clear the released effect and enter `ParentCrea` | translated | high | `80a7e9b0_FUN_80a7e9b0.c` |
| `0x80A7EA40` | `fn_80a7ea40` | Arm the 240-frame wait and enter `ParentWait` | translated | high | `80a7ea40_FUN_80a7ea40.c` |
| `0x80A7FCE0` | `fn_80a7fce0` | Copy the holder translation to `+0x660` | translated | high | `80a7fce0_FUN_80a7fce0.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A5B180` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A5B1B0` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A5BB30` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A5BB60` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A7AB90` | 272 B | GQR/`ldexpf` artefacts in the angle conversion |
| `0x80A7ACA0` | 252 B | GQR/`ldexpf` artefacts; still called by address |
| `0x80A7ADA0` | 220 B | GQR register arguments in the angle conversion |
| `0x80A7AEF0` | 200 B | GQR/`ldexpf` artefacts |
| `0x80A7B7C0` | 184 B | `0x800964A0` is printed with zero operands; its real four-argument ABI cannot be recovered from this export |
| `0x80A7BD30` | 372 B | GQR/`ldexpf` artefacts in the walk binder |
| `0x80A7BEC0` | 1,464 B | GQR register arguments and unreachable blocks |
| `0x80A7C600` | 184 B | GQR/`ldexpf` artefacts |
| `0x80A7C7D0` | 628 B | Stack-slot comparisons cannot be modelled without inventing operands |
| `0x80A7D2D0` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A7D300` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A7D760` | 400 B | GQR/`ldexpf` artefacts in the phase handler |
| `0x80A7DC90` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A7DCC0` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A7E640` | 212 B | GQR/`ldexpf` artefacts; still called by address |
| `0x80A7E630` | 8 B | Export is `halt_baddata()` |
| `0x80A7EB60` | 280 B | GQR/`ldexpf` artefacts in the phase timer |
| `0x80A7ECA0` | 84 B | Calls `0x80A7E630`, whose export is `halt_baddata()` |
| `0x80A7ED20` | 176 B | Calls `0x80A7E630`, whose export is `halt_baddata()` |
| `0x80A7EE20` | 128 B | Calls `0x80A7E630`, whose export is `halt_baddata()` |
| `0x80A7F690` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A7F6C0` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A7FB50` | 392 B | GQR/`ldexpf` artefacts in the matrix/rotation setup |

## Unknowns preserved in code

1. **`0xEB0` actor class.** The wave-17 note still applies: the offsets fit
   the `0xEB0`-byte actor built by `0x80A79460`, but the export has no state
   table for that class, so the functions keep `fn_<address>` names.
2. **`0x802DCD74`/`0x802DCDC0`.** These are section prologue/epilogue
   fragments, not real functions. `fn_80a7a700` reads the actor pointer from
   `0x802DCD74` exactly as wave 16 did for `0x802DCD70`.
3. **`0x8027A200`.** Ghidra prints no operand in `fn_80a5bce0`; the Rust
   passes the live state as `// inferred:` and documents the assumption.
4. **`0x80B73068`.** `fn_80a7bbf0` enters this BSS descriptor when the walk
   animation ends. It is not one of the eight `daEnIcicle_c` descriptors, so
   the name stays `DESC_`.
5. **`0x80B2DF78` table shape.** The index `side * 5 + variant` implies ten
   slots. The raw REL stores relocation addends, so the Rust reads the table
   at runtime instead of baking the pointers in.
6. **`0x800964A0`.** The main-DOL body takes four arguments, but this REL's
   call sites print none. `fn_80a7b7c0` is therefore deferred rather than
   guessed.
7. **FireSnake model name.** The `obj_turara_kotei` model and `0x5B8`-byte
   constructor link the helpers to `daEnFireSnake_c`, but the model name alone
   is not treated as a class label.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `cargo check` reports no warning or error in
`src/game/enemies.rs`, and `node tools/doc-lint.mjs src` reports 0 violations
across 68 files. This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave19-enemies.csv`.
