# Coverage - wave 17 (`d_basesNP` remaining constructors, registrations and model loaders)

## How to read this file

This page records the seventeenth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 30 bodies appended to
   `src/game/bases.rs`: thirteen in-place constructors, eight descriptor-less
   class registrations, and nine actor model loaders.
2. **Why these functions?** - *Selection* explains how the pass found the last
   untranslated constructor and interface-wrapper families and the remaining
   named BRRES loaders.
3. **What is still missing?** - *Deferred candidates* and *Unknowns preserved*
   record the rejected `dWorldSelect_c` table, the one remaining StateID file,
   the tiny `0x806C3010` factory, and every class role that is only inferred.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a class attribution or a
  vtable role is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x80374188..0x803741AC`, `0x80377A04`, `0x80377C48`, `0x8042A038`,
`0x8042A1B4`, `0x8042A488`, `0x8042CF6C`, `0x8042CF84`) are imported absolute
values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 17 |
|---|---:|
| New translated functions | 30 |
| In-place constructors | 13 |
| Descriptor-less class registrations | 8 |
| Actor model loaders | 9 |
| Selected bytes | 8,420 |
| Translated functions in `src/game/bases.rs` (cumulative) | 405 |
| Cumulative selected bytes | 164,832 (14.85 % of indexed bytes) |

The 30 new bodies select 8,420 of the 1,109,860 indexed bytes (0.76 %) and 30
of the 5,409 eligible entries (0.55 %). Confidence is **22 high** and **8
medium**; there are no `partial` or `stub` rows. The total is 405 function
bodies in `src/game/bases.rs`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All 375 addresses already
translated by waves 2-16 were removed before selection; none of the 30 new
addresses appears in an earlier ledger row.

1. **Constructor sweep.** The untranslated slice contained thirteen bodies
   calling the shared NW4R interface constructor `0x8015FC90` or one of the
   base constructors `0x80064110`, `0x80066FC0`, `0x8006C6D0`, `0x80094E80` and
   `0x800E19B0`. All thirteen are complete, branch-free in-place constructors
   with explicit class vtables, interface groups and callback tables. They were
   selected as one family.
2. **Descriptor-less registrations.** Wave 16 deferred eight boot-time
   functions (`0x80764270`, `0x80765C10`, `0x80779820`, `0x80785BB0`,
   `0x8078A280`, `0x8078B460`, `0x8078D1B0`, `0x807A1170`) because they were
   not class-name tables. Their bodies are nevertheless complete: one registry
   node each, explicit parameter/default blocks and three guarded default
   blocks. They were selected and are marked `medium` only because the class
   has no string.
3. **Named model loaders.** The nine remaining self-contained loader methods
   each name a BRRES container and a pane/animation. They were selected in
   address order; the bodies are explicit and therefore `high`.
4. **Rejected the only remaining StateID table.** `0x807BD110`
   (`dWorldSelect_c`, 13 states, 2,908 bytes) is still the only untranslated
   file containing `StateID`. Its decompilation still carries
   `unaff_GQR0`/`ldexpf` and removed unreachable blocks, so it was not guessed.
   This pass therefore adds **zero** StateID tables.
5. **Left the tiny factory.** `0x806C3010` (76 bytes) allocates `0x84` bytes
   and calls a constructor, but has no adjacent table or lifecycle; it remains
   deferred.

## Clusters

### Remaining in-place constructors (`0x806856E0..0x807B1700`)

Thirteen constructors with no class string. Three share the `0x807EDE64`
family layout: `0x806856E0`, `0x806A8640` and `0x806BEB60` all build two
interface groups and the `0x807D1018` middle vtable, then diverge into their
own helper blocks. `0x806C8380`, `0x806E6C10` and `0x8070B0A0` allocate their
own storage and run `0x80066FC0`/`0x80094E80`. `0x80757AB0`, `0x807777B0`,
`0x80786540`, `0x807AA9A0`, `0x807AF9C0` and `0x807B1700` take the object in
place and run `0x8006C6D0` or `0x800F5FD0`/`0x800E19B0`.

The class vtables are `0x807EDE64`, `0x807F3508`, `0x807F711C`, `0x807F93C8`,
`0x807FD7D0`, `0x807FBA7C`, `0x80805710`, `0x80813C60`, `0x80817888`,
`0x80818ED0`, `0x8081DB40`, `0x8081E718` and `0x8081EEB8`. None of these
vtables occurs in another export file, so no neighbouring table supplies a
class name.

### Descriptor-less class registrations (`0x80764270..0x807A1170`)

Each function copies one class parameter block, links one 12-byte registry node
with [`fn_80600160`], then writes the shared `0x8042CF84` value into the class
context. Two of them (`0x80765C10`, `0x8078B460`) and the large `0x807A1170`
also initialise a guarded default block on first run. The guard bytes are
`0x808326F0`, `0x80832D48` and `0x80832EF8`. Callbacks are `0x80764300`,
`0x80765D40`, `0x807798B0`, `0x80785C40`, `0x8078A330`, `0x8078B540`,
`0x8078D240` and `0x807A1360`; the nodes end at `0x808326C0`, `0x808326E0`,
`0x80832B90`, `0x80832C50`, `0x80832D00`, `0x80832D38`, `0x80832DE0` and
`0x80832EE8`.

### Actor model loaders (`0x8064A6D0..0x8066C130`)

Nine model/animation loaders. Each builds a `0x800690C0` resource accessor,
loads a BRRES through `0x800DF180` from the main.dol manager at
`0x8042A038 + 4`, creates one or two animation bindings, and releases the
accessor with `0x800690E0`. The resource names are:

| Address | BRRES / pane evidence |
|---|---|
| `0x8064A6D0` | `block_arrow` / `g3d/block_arrow.brres` |
| `0x80652C70` | `light_block` / `g3d/light_block.brres` |
| `0x80654300` | `block_taru` / `g3d/block_taru.brres` |
| `0x806574D0` | `koopa_clown_bomb` / `em_g3d_koopa_clown_bomb.brres` |
| `0x8065B660` | `FrontBlack_small`, `FrontBlack`, `backWall` |
| `0x80665620` | `obj_rubble` with `_m` and `_l` panes |
| `0x80666BC0` | `boss_koopa_ashiba`, `...C`, `...B` |
| `0x8066B170` | `boss_shutter`, `boss_shutter_koopa` |
| `0x8066C130` | `key_boss_castle` / `g3d/key_boss_castle.brres` |

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave17-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806856E0` | `fn_806856e0` | construct the `0x807EDE64` actor with two interface groups | translated | high | `806856e0_FUN_806856e0.c` |
| `0x806A8640` | `fn_806a8640` | construct the `0x807F3508` actor sharing the first constructor's groups | translated | high | `806a8640_FUN_806a8640.c` |
| `0x806BEB60` | `fn_806beb60` | construct the `0x807F711C` actor with three helper blocks | translated | high | `806beb60_FUN_806beb60.c` |
| `0x806C8380` | `fn_806c8380` | allocate `0x568` bytes and construct the `0x807F93C8` actor | translated | high | `806c8380_FUN_806c8380.c` |
| `0x806DB0E0` | `fn_806db0e0` | construct the `0x807FBA7C` actor with two interface groups | translated | high | `806db0e0_FUN_806db0e0.c` |
| `0x806E6C10` | `fn_806e6c10` | allocate `0x560` bytes and construct the `0x807FD7D0` actor | translated | high | `806e6c10_FUN_806e6c10.c` |
| `0x8070B0A0` | `fn_8070b0a0` | allocate `0xB18` bytes and construct the `0x80805710` actor | translated | high | `8070b0a0_FUN_8070b0a0.c` |
| `0x80757AB0` | `fn_80757ab0` | construct the `0x80813C60` actor with an animation helper | translated | high | `80757ab0_FUN_80757ab0.c` |
| `0x807777B0` | `fn_807777b0` | construct the `0x80817888` actor with two tag blocks | translated | high | `807777b0_FUN_807777b0.c` |
| `0x80786540` | `fn_80786540` | construct the `0x80818ED0` actor with two tag blocks | translated | high | `80786540_FUN_80786540.c` |
| `0x807AA9A0` | `fn_807aa9a0` | construct the `0x8081DB40` object with a callback table | translated | high | `807aa9a0_FUN_807aa9a0.c` |
| `0x807AF9C0` | `fn_807af9c0` | construct the `0x8081E718` object with a callback table | translated | high | `807af9c0_FUN_807af9c0.c` |
| `0x807B1700` | `fn_807b1700` | construct the `0x8081EEB8` object and set ten main.dol mode words | translated | high | `807b1700_FUN_807b1700.c` |
| `0x80764270` | `fn_80764270` | register an unnamed class and copy its parameter block | translated | medium | `80764270_FUN_80764270.c` |
| `0x80765C10` | `fn_80765c10` | register an unnamed class with a guarded default block | translated | medium | `80765c10_FUN_80765c10.c` |
| `0x80779820` | `fn_80779820` | register an unnamed class | translated | medium | `80779820_FUN_80779820.c` |
| `0x80785BB0` | `fn_80785bb0` | register an unnamed class | translated | medium | `80785bb0_FUN_80785bb0.c` |
| `0x8078A280` | `fn_8078a280` | register an unnamed class and repeat one default three times | translated | medium | `8078a280_FUN_8078a280.c` |
| `0x8078B460` | `fn_8078b460` | register an unnamed class with a guarded default block | translated | medium | `8078b460_FUN_8078b460.c` |
| `0x8078D1B0` | `fn_8078d1b0` | register an unnamed class | translated | medium | `8078d1b0_FUN_8078d1b0.c` |
| `0x807A1170` | `fn_807a1170` | register an unnamed class with a 36-word guarded default block | translated | medium | `807a1170_FUN_807a1170.c` |
| `0x8064A6D0` | `fn_8064a6d0` | load `block_arrow` and its animation | translated | high | `8064a6d0_FUN_8064a6d0.c` |
| `0x80652C70` | `fn_80652c70` | load `light_block` and two animations | translated | high | `80652c70_FUN_80652c70.c` |
| `0x80654300` | `fn_80654300` | load `block_taru` and its animation | translated | high | `80654300_FUN_80654300.c` |
| `0x806574D0` | `fn_806574d0` | load `koopa_clown_bomb` and its launch animation | translated | high | `806574d0_FUN_806574d0.c` |
| `0x8065B660` | `fn_8065b660` | load one of three `FrontBlack` wall models | translated | high | `8065b660_FUN_8065b660.c` |
| `0x80665620` | `fn_80665620` | load `obj_rubble` with one of three panes | translated | high | `80665620_FUN_80665620.c` |
| `0x80666BC0` | `fn_80666bc0` | load one of three `boss_koopa_ashiba` panes | translated | high | `80666bc0_FUN_80666bc0.c` |
| `0x8066B170` | `fn_8066b170` | load `boss_shutter` or `boss_shutter_koopa` | translated | high | `8066b170_FUN_8066b170.c` |
| `0x8066C130` | `fn_8066c130` | load `key_boss_castle` and its pane | translated | high | `8066c130_FUN_8066c130.c` |

**Totals:** 30 translated, 0 partial, 0 stub. 22 high confidence, 8 medium.
The medium rows are the eight registrations whose class has no name string;
their flow is explicit and only the attribution is unknown.

## Deferred candidates

| Address range | Why deferred |
|---|---|
| `0x807BD110` | `dWorldSelect_c` state table: 13 states, 2,908 bytes; **rejected** for the same `unaff_GQR0`/`ldexpf` paired-single reconstruction failure documented in waves 15-16. |
| `0x806C3010` | 76-byte allocator wrapper for a `0x84`-byte object with vtable `0x807F8068`; no adjacent table or lifecycle. |
| `0x8064B370` | `waterbubble` model loader (228 bytes); valid but outside the 30-function budget, and structurally identical to `0x80654300`. |
| `0x8064B5E0` | 116-byte `waterbubble` state helper; needs the actor class attribution first. |
| `0x80764300`, `0x80765D40`, `0x807798B0`, `0x80785C40`, `0x8078A330`, `0x8078B540`, `0x8078D240`, `0x807A1360` | Callbacks registered by this pass; their bodies are still untranslated. |
| State handlers of the 46 tables closed in wave 16 | Still the largest behavioural gap in the module; each needs its own handler pass. |

## Unknowns preserved

1. **Constructor class attribution.** All thirteen constructors have no class
   string. Their class vtables, interface contexts and helper calls are
   explicit, but the class names stay `// unknown:`.
2. **Registration class identity.** The eight registrations have no name string
   and no state table. They are recorded as "unnamed class" rather than guessed
   from the callback address.
3. **Base-constructor arguments.** Ghidra prints `0x80064110`, `0x80066FC0`,
   `0x8006C6D0`, `0x80094E80` and `0x800E19B0` without a `this` argument. The
   Rust passes `actor`, following the documented wave-3/4 convention; the
   base prototypes themselves remain outside this slice.
4. **Float playback helpers.** `0x80166F10`, `0x80168FE0` and `0x801662F0` are
   printed with `(double)` casts but are called with single-precision loads.
   The Rust uses the existing `call_f32_u32`/`call_f32_u32x2` wrappers and
   marks the prototypes `// unknown:`.
5. **Unnamed panes.** `0x807E7D20` (`obj_rubble` fallback) and `0x807E80A4`
   (`boss_koopa_ashiba` fallback) are data addresses whose strings Ghidra does
   not print. They are kept as addresses with `// unknown:` notes.
6. **The `0x800E19B0` family.** The three `0x8081DB40`/`0x8081E718`/
   `0x8081EEB8` constructors share a base constructor and a `0x8015F600`
   callback table, but their class identities are not recoverable from the
   bodies.
7. **Compiler thunks.** `0x802DCD60`, `0x802DCD68`, `0x802DCD74`, `0x802DCDAC`,
   `0x802DCDB4` and `0x802DCDC0` save/restore registers or return the object
   pointer; they have no Rust-visible effect and are omitted with `// observed:`
   notes.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 405 translated functions; this pass adds 30,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files. `cargo check` passes (exit 0) for the
workspace and in an isolated copy of `bases.rs` + `platform.rs`; the remaining
warnings are in `src/runtime/bt.rs`, `src/runtime/input.rs` and
`src/game/bosses.rs`, outside this lane, and `src/game/bases.rs` produces none.
