# Coverage - wave 23 (`d_basesNP` next actor families)

## How to read this file

This page records the twenty-third translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 29 bodies appended to
   `src/game/bases.rs`: the unnamed `0x806B53F0..0x806B5CE0` palette/state
   family, three small vtable/name helpers, the `wendy` boss actor
   (`0x806B6DE0..0x806B7AD0`), and a `0x6D8`-byte Ludwig magic-attack effect
   actor (`0x806B8E70..0x806B9930`).
2. **Why these functions?** - *Selection* explains how the pass continued the
   address-ordered sweep above the wave-22 window and which
   paired-single failures were skipped.
3. **What is still missing?** - *Deferred candidates* records the remaining
   `ldexpf`/`unaff_GQR*` bodies and the one helper whose receiver Ghidra drops.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role or table meaning is
  inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8006xxxx`, `0x8016xxxx`, `0x8037xxxx`, `0x8042xxxx`) are imported absolute
values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 23 |
|---|---:|
| New translated functions | 29 |
| Unnamed palette/state family (`0x806B53F0..0x806B5CE0`) | 5 |
| Vtable/name helpers (`0x806B6AA0..0x806B6CC0`) | 3 |
| `wendy` boss actor (`0x806B6DE0..0x806B7AD0`) | 7 |
| Ludwig magic-attack effect actor (`0x806B8E70..0x806B9930`) | 14 |
| Selected bytes | 5,408 |
| Translated functions in `src/game/bases.rs` (cumulative) | 552 |
| Cumulative selected bytes | 192,884 (17.38 % of indexed bytes) |

The 29 new bodies select 5,408 of the 1,109,860 indexed bytes (0.49 %) and 29
of the 5,409 eligible entries (0.54 %). All 29 rows are `high` confidence: every
call, store, constant and branch is explicit. No row is `partial` or `stub`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2-22 were removed before selection. Wave 22 stopped at
`0x806B4870` and listed `0x806B53F0+` as the next actor family; this pass starts
at `0x806B53F0` and moves upward to `0x806B9930`.

1. **Continue the address sweep.** Every selected file has a complete
   control-flow graph; no selected file contains `halt_baddata`, `unaff_`, or
   an `ldexpf`/paired-single reconstruction.
2. **Follow the first constructor.** `0x806B6DE0` allocates `0x1730` bytes,
   installs vtable `0x807F5E80` and builds six inline arrays. The adjacent
   bodies `0x806B6F10..0x806B7AD0` are that object's loader, initialiser,
   predicate and per-frame dispatch.
3. **Follow the second constructor.** `0x806B8E70` allocates `0x6D8` bytes and
   installs vtable `0x807F61F0`. The adjacent bodies
   `0x806B9040..0x806B9930` are the `boss_ef_attack` / `ludwig_attack` model
   loader, transform, collision, effect and state bodies.
4. **Take the complete helper cluster.** `0x806B6AA0` (release), `0x806B6BE0`
   (five-link walk) and `0x806B6CC0` (name-suffix compare) are self-contained
   and were translated with the actor they serve.
5. **Rejected decompiler failures.** `0x806B5E00`, `0x806B7340`, `0x806B7690`,
   `0x806B7890`, `0x806B7B70`, `0x806B7CD0`, `0x806B8380`, `0x806B9800` and
   `0x806B9A30` contain `unaff_GQR0`/`unaff_GQR3` plus `ldexpf`;
   `0x806B9720` is `halt_baddata`. All were skipped and are listed below.

## Clusters

### Unnamed palette/state family (`0x806B53F0..0x806B5CE0`)

- `0x806B53F0` clears nine words at `+0x5BC..+0x5DC`, converts three virtual
  `+0x288` floats into fixed-point words and calls `0x8006D090`.
- `0x806B58E0` refreshes the `+0x544` sub-interface while the flag at `+0x6D0`
  is set.
- `0x806B59A0` copies the nine-word palette `0x807CAAE0..0x807CAAFC` into
  `+0x414..+0x432` and takes the variant byte from an optional source object.
- `0x806B5A80` rebuilds the two angles from the `main.dol` tables
  `0x8042C1A0`/`0x8042C1A4`, indexed by the variant byte at `+0x348`.
- `0x806B5CE0` initialises the palette, scale, virtual state `+0x2A4`, sound
  `0x536` and the transform at `+0x5E0`.

No class string survives for this object, so the attribution is `// unknown:`.

### `wendy` boss actor (`0x806B6DE0..0x806B7AD0`)

- `0x806B6DE0` builds the `0x1730`-byte object: five `0x40`-byte model
  holders at `+0x3F0`, 120 position records at `+0x540`, five `0xA4`-byte
  effect objects at `+0xCD4`, four `0x114`-byte objects at `+0x104C`, four
  periods at `+0x14AC` and two spawners at `+0x14DC`.
- `0x806B7170` loads `wendy` (`0x807F5DF8`) from
  `g3d/wendy.brres` (`0x807F5DE8`) and binds `wendy_ring` (`0x807F5E00`) once
  per ring.
- `0x806B6F10` is the state initialiser: ring count table `0x807CABAC`, 15
  position/speed blocks, then interface states `0x8082C5C0`/`0x8082C580`.
- `0x806B74B0` gates the per-ring `+0x14` update on the interface state.
- `0x806B79E0`/`0x806B7A60` spawn `Wm_we_magicattack_st` (`0x807F5E20`).
- `0x806B7AD0` dispatches to `0x80162510` or the rejected `0x806B7690`.

The class name is `// inferred:` from the `wendy`/`Wm_we_*` strings; no
`s_*_c__` class string exists in this range.

### Ludwig magic-attack effect actor (`0x806B8E70..0x806B9930`)

- `0x806B8E70` builds the `0x6D8`-byte object with vtable `0x807F61F0`,
  the accessor `+0x3D0`, the model holder `+0x3F0`, the animation binding
  `+0x434` and two transform blocks at `+0x488`/`+0x5B0`.
- `0x806B9040` loads `boss_ef_attack` (`0x807F6148`, BRRES path `0x807F612C`)
  and binds the `ludwig_attack` model and animation (`0x807F6158`).
- `0x806B9170` retires the actor unless a higher player exists; `0x806B98C0`
  retires it on the `+0x460` flag.
- `0x806B92C0`/`0x806B93B0` rebuild the matrix and snapshot the animation
  transform into `+0x464`.
- `0x806B9420` is the collision callback; `0x806B94E0` maps the three
  collision probes to `Wm_lu_mgcwallhit` variants.
- `0x806B9580`/`0x806B9620` spawn the burst and wall-hit effects;
  `0x806B96C0`/`0x806B96F0` spawn `Wm_lu_magicattack*`.
- `0x806B9740` picks an active player slot at random; `0x806B9930` initialises
  the attack countdown, direction, angle correction and sound `0x51D`.

The class name is `// unknown:`; the strings name the effects, not the class.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave23-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806B53F0` | `fn_806b53f0` | reset five transform blocks and rebuild the layout | translated | high | `806b53f0_FUN_806b53f0.c` |
| `0x806B58E0` | `fn_806b58e0` | refresh the `+0x544` sub-interface while `+0x6D0` is set | translated | high | `806b58e0_FUN_806b58e0.c` |
| `0x806B59A0` | `fn_806b59a0` | copy a nine-word palette and the source variant byte | translated | high | `806b59a0_FUN_806b59a0.c` |
| `0x806B5A80` | `fn_806b5a80` | set the two starting angles from the player variant tables | translated | high | `806b5a80_FUN_806b5a80.c` |
| `0x806B5CE0` | `fn_806b5ce0` | initialise palette, scale, state and sound `0x536` | translated | high | `806b5ce0_FUN_806b5ce0.c` |
| `0x806B6AA0` | `fn_806b6aa0` | release an object, optionally free it | translated | high | `806b6aa0_FUN_806b6aa0.c` |
| `0x806B6BE0` | `fn_806b6be0` | walk up to five `+0x30` links, then recurse | translated | high | `806b6be0_FUN_806b6be0.c` |
| `0x806B6CC0` | `fn_806b6cc0` | compare two names after the first `:` | translated | high | `806b6cc0_FUN_806b6cc0.c` |
| `0x806B6DE0` | `fn_806b6de0` | allocate `0x1730` bytes and build six arrays | translated | high | `806b6de0_FUN_806b6de0.c` |
| `0x806B6F10` | `fn_806b6f10` | run the `wendy` state initialiser | translated | high | `806b6f10_FUN_806b6f10.c` |
| `0x806B7170` | `fn_806b7170` | load the `wendy` BRRES and bind one ring model per ring | translated | high | `806b7170_FUN_806b7170.c` |
| `0x806B74B0` | `fn_806b74b0` | gate the per-ring update on the interface state | translated | high | `806b74b0_FUN_806b74b0.c` |
| `0x806B79E0` | `fn_806b79e0` | spawn `Wm_we_magicattack_st` at the actor | translated | high | `806b79e0_FUN_806b79e0.c` |
| `0x806B7A60` | `fn_806b7a60` | spawn the magic attack and select the default state | translated | high | `806b7a60_FUN_806b7a60.c` |
| `0x806B7AD0` | `fn_806b7ad0` | refresh the model, retire or run the ring burst | translated | high | `806b7ad0_FUN_806b7ad0.c` |
| `0x806B8E70` | `fn_806b8e70` | allocate `0x6D8` bytes and build the effect actor | translated | high | `806b8e70_FUN_806b8e70.c` |
| `0x806B9040` | `fn_806b9040` | load `boss_ef_attack` and bind `ludwig_attack` | translated | high | `806b9040_FUN_806b9040.c` |
| `0x806B9170` | `fn_806b9170` | retire unless a higher player exists | translated | high | `806b9170_FUN_806b9170.c` |
| `0x806B92C0` | `fn_806b92c0` | rebuild the matrix and apply the animation transform | translated | high | `806b92c0_FUN_806b92c0.c` |
| `0x806B93B0` | `fn_806b93b0` | snapshot the animation transform into `+0x464` | translated | high | `806b93b0_FUN_806b93b0.c` |
| `0x806B9420` | `fn_806b9420` | collision callback: state, hit bit, wall effect | translated | high | `806b9420_FUN_806b9420.c` |
| `0x806B94E0` | `fn_806b94e0` | map the three collision probes to wall-hit variants | translated | high | `806b94e0_FUN_806b94e0.c` |
| `0x806B9580` | `fn_806b9580` | play burst effects, sound `0x51F`, retire | translated | high | `806b9580_FUN_806b9580.c` |
| `0x806B9620` | `fn_806b9620` | spawn `Wm_lu_mgcwallhit` with one of three angles | translated | high | `806b9620_FUN_806b9620.c` |
| `0x806B96C0` | `fn_806b96c0` | spawn `Wm_lu_magicattack` on `+0x5B0` | translated | high | `806b96c0_FUN_806b96c0.c` |
| `0x806B96F0` | `fn_806b96f0` | spawn `Wm_lu_magicattack_st` on `+0x488` | translated | high | `806b96f0_FUN_806b96f0.c` |
| `0x806B9740` | `fn_806b9740` | pick an active player slot at random | translated | high | `806b9740_FUN_806b9740.c` |
| `0x806B98C0` | `fn_806b98c0` | retire on `+0x460` after refreshing the magic attack | translated | high | `806b98c0_FUN_806b98c0.c` |
| `0x806B9930` | `fn_806b9930` | initialise attack countdown, direction and sound `0x51D` | translated | high | `806b9930_FUN_806b9930.c` |

**Totals:** 29 translated, 0 partial, 0 stub (29 high).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x806B5E00` | 892 | **Rejected**: `unaff_GQR0` + `ldexpf`. |
| `0x806B7340` | 368 | **Rejected**: `unaff_GQR0` + `ldexpf`. Called by address from `fn_806b6f10`. |
| `0x806B7690` | 312 | **Rejected**: `unaff_GQR0` + `ldexpf`. Called by address from `fn_806b7ad0`. |
| `0x806B7890` | 336 | **Rejected**: `unaff_GQR0` + `ldexpf`. |
| `0x806B7B70` | 344 | **Rejected**: `unaff_GQR0` + `ldexpf`. |
| `0x806B7CD0` | 1,704 | **Rejected**: `unaff_GQR0`/`unaff_GQR3` + `ldexpf`. |
| `0x806B8380` | 1,500 | **Rejected**: `unaff_GQR0` + `ldexpf`. |
| `0x806B8CF0` | 88 | Duplicate of the release helper [`fn_806b6aa0`]; no independent behaviour. |
| `0x806B8E10`, `0x806B8E40` | 48 each | Vector-deleting-destructor thunks around `0x802DCBFC`. |
| `0x806B9720` | 32 | **Rejected**: `halt_baddata`. |
| `0x806B9800` | 132 | **Rejected**: `unaff_GQR0` + `ldexpf`. Called by address from `fn_806b9930`. |
| `0x806B9A30` | 476 | **Rejected**: `unaff_GQR3` + `ldexpf`. |
| `0x806B9CB0` | 72 | Calls the `halt_baddata` body `0x806B9720`; deferred with it. |
| `0x806B9CC0+` | - | Next actor family, untouched by this pass. |

## Unknowns preserved

1. **Class attribution.** Neither the `0x1730`-byte `wendy` actor, the
   `0x6D8`-byte effect actor, nor the `0x806B53F0` family carries a
   `s_<Class>_c__` string. All three attributions stay `// inferred:` or
   `// unknown:`; the effect names (`Wm_we_*`, `Wm_lu_*`) are not class names.
2. **Signed-integer conversion.** `0x806B5A80`/`0x806B5CE0` print
   `(double)CONCAT44(0x43300000, x ^ 0x80000000) - DAT_807CAB00`. The constant
   at `0x807CAB00` is `0x43300000_80000000` (2^52 + 2^31), so the expression is
   the compiler's signed-integer-to-double conversion; the Rust keeps that
   identity in a comment.
3. **Lost `this` arguments.** Ghidra drops the first argument of
   `0x806B7800`, `0x806B79E0`, `0x806B96F0`, the `+0x98` virtual in
   `fn_806b7ad0`, the `+0x288` virtual in `fn_806b53f0` and the `+0x6C`
   virtual in `fn_806b59a0`. The Rust passes the actor (or receiver) and marks
   each site `// observed:`.
4. **Rejected callees called by address.** `0x806B7340`, `0x806B7690`,
   `0x806B9800` and `0x806B9720` are not represented as Rust bodies. Their call
   sites keep the exact arguments and are documented as GQR-rejected or
   truncated.
5. **Float tables.** `0x807CAA64..0x807CAC50` are read as raw `f32`/`u32`;
   their physical meaning (speed, scale, effect size, angle) is `// unknown:`.
6. **Ludwig effect actor size.** The constructor allocates `0x6D8` bytes, but
   the bodies also touch `+0x6D4` and `+0x6CC`/`+0x6CD`; all offsets fit the
   allocation.
7. **`wendy` array geometry.** The 15 position blocks at `+0x540` and the 120
   speed floats at `+0xAE0` overlap the constructor's `0x78`-entry, `0xC`-stride
   array at `+0x540` exactly; the loop was translated with its literal 15 x 8
   shape.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` contains 552 translated functions after this pass; this pass
adds 29, all with `# Original` blocks and the synthetic-base warning.
`node tools/doc-lint.mjs src` reports `ok: true`, 0 violations over 68 files.
The first full `cargo check` of this pass finished green. The final run reports
zero diagnostics for `src/game/bases.rs`; the crate build itself is currently
blocked by unrelated concurrent edits in another lane's file.
