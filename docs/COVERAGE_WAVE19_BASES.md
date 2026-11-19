# Coverage - wave 19 (`d_basesNP` actor constructors and model loaders)

## How to read this file

This page records the nineteenth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 30 bodies appended to
   `src/game/bases.rs`: fifteen heap-allocating actor constructors and the
   fifteen model loaders that publish their BRRES resources.
2. **Why these functions?** - *Selection* explains how the pass found the
   remaining constructor/loader families after the state tables were exhausted.
3. **What is still missing?** - *Deferred candidates* and *Unknowns preserved*
   record the rejected `dWorldSelect_c` table, the remaining loader backlog,
   and every class attribution that stays inferred.

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
(`0x8030xxxx`, `0x8037xxxx`, `0x8042xxxx`, `0x807Cxxxx`) are imported absolute
values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 19 |
|---|---:|
| New translated functions | 30 |
| Heap-allocating actor constructors | 15 |
| Actor model loaders | 15 |
| Selected bytes | 6,096 |
| Translated functions in `src/game/bases.rs` (cumulative) | 435 |
| Cumulative selected bytes | 170,928 (15.40 % of indexed bytes) |

The 30 new bodies select 6,096 of the 1,109,860 indexed bytes (0.55 %) and 30
of the 5,409 eligible entries (0.55 %). All 30 rows are `high` confidence: the
control flow, allocation sizes, vtables, resource strings and pane/animation
bindings are explicit. No row is `partial` or `stub`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All 405 addresses already
translated by waves 2-17 were removed before selection; none of the 30 new
addresses appears in an earlier ledger row.

1. **State tables are exhausted.** A scan of all 5,021 untranslated files found
   exactly one file containing `StateID`: `0x807BD110` (`dWorldSelect_c`, 13
   states, 2,908 bytes). It still carries the `unaff_GQR0`/`ldexpf`
   paired-single reconstruction failure documented in waves 15-17, so this pass
   adds **zero** state tables.
2. **Constructor sweep.** The untranslated slice contained 27 bodies that call
   the shared allocator `0x801628C0` and the base actor constructor
   `0x80064110`. Fifteen were selected: ten paired with a loader family
   (including three 76-byte vtable-only bodies) and five with self-contained
   callback tables or multiple subobjects. The remaining eleven 76-byte and one
   84-byte vtable-only bodies were left for a later bulk pass.
3. **Matching loaders.** For each selected constructor the adjacent loader
   family was checked: the constructor's `+0x394..` resource block must match
   the loader's offsets. Fifteen loaders were selected this way. The BRRES
   strings (`g3d/*.brres` and their pane names) then supply the class name that
   the constructor lacks.
4. **Rejected the tiny factory.** `0x806C3010` (76 bytes) still allocates
   `0x84` bytes with no adjacent table or lifecycle; it remains deferred.

## Clusters

### Heap-allocating constructors (`0x8064B1C0..0x807A6360`)

Each constructor calls `0x801628C0(size)` (`// unknown:` shared heap
allocator), returns zero when it fails, runs the base actor constructor
`0x80064110` (Ghidra prints it without `this`; the Rust passes the object,
following the wave-3/4 convention), writes the class vtable at `+0x60`, and
initialises the resource accessor at `+0x394`. Additional subobjects use
`0x80069020`, `0x80169CD0`, `0x80168D10`, `0x8016A2F0`, `0x8016A5E0`,
`0x800D2390`, `0x8008C200` and `0x8007F7A0`; callback tables are built by
`0x802DC9A0`.

Two constructors replace their vtable at the end of the body
(`0x80687850`: `0x80306D60` then `0x807EE1E8`; `0x807A6360`: `0x80307A70` then
`0x8081D0D4`). Both writes are preserved in order.

### Model loaders (`0x8064B370..0x80745B30`)

Each loader follows the same NW4R sequence:

1. `0x800690C0` starts a resource accessor at `actor + 0x394` (or the class's
   own offset), with the main.dol archive handle at `0x80377C48`.
2. `0x800DF180(main.dol manager + 4, pane/name, brres)` loads the container.
3. `0x80239E30`/`0x8023A0B0`/`0x8023A200`/`0x8023A270` resolve the model and
   animation objects.
4. `0x80169D90`/`0x8016A3A0` bind the model; `0x80064BF0`/`0x80064BC0`
   finalise it; `0x801650D0`/`0x80166830`/`0x80167420` create clips.
5. `0x800690E0` releases the accessor.

The BRRES / pane evidence is:

| Address | BRRES / pane evidence |
|---|---|
| `0x8064B370` | `waterbubble` / `g3d/waterbubble.brres` |
| `0x8064C460` | `MG_kinopio` / `KDH_model`, `KDB_model`, `KB_trs_open` |
| `0x8064D790` | `lift_zen` / `lift_togeU` |
| `0x8064E870` | `BGM_anim_dummy` / `BGM_anim_RtoL`, `BGM_anim_LtoR` |
| `0x80668670` | `lift_zen_kaiten` / caller-supplied pane |
| `0x80674C00` | `middle_flag` / model, tag and two pane clips |
| `0x80676E10` | `houdai` / `houdai_mouth`, `houdai_body`, `houdai_root` |
| `0x80677870` | `obj_door_obake` / `obj_obake_door` |
| `0x80677DD0` | `block_slide` / `count` repeated panes |
| `0x80690430` | `Mask_flashlight_obake` / `flashlight_obake` |
| `0x806C0E10` | `hashigo` / pane table `0x807CAF5C` |
| `0x806CBCE0` | `lift_huriko` / `lift_huriko_tomegu` |
| `0x80705710` | `noboribou_obake` |
| `0x8071EDB0` | `obj_yashi` / `obj_yashi_wait` |
| `0x80745B30` | `untei_rope` / `untei_rope_tomegu` (two panes) |

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave19-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8064B1C0` | `fn_8064b1c0` | allocate `0x440` bytes and construct the `waterbubble` actor | translated | high | `8064b1c0_FUN_8064b1c0.c` |
| `0x8064B370` | `fn_8064b370` | load `waterbubble` and its animation | translated | high | `8064b370_FUN_8064b370.c` |
| `0x8064C3A0` | `fn_8064c3a0` | allocate `0x4A0` bytes and construct the `MG_kinopio` actor | translated | high | `8064c3a0_FUN_8064c3a0.c` |
| `0x8064C460` | `fn_8064c460` | load `MG_kinopio` with two models and a clip | translated | high | `8064c460_FUN_8064c460.c` |
| `0x8064D790` | `fn_8064d790` | load `lift_zen` and pane `lift_togeU` | translated | high | `8064d790_FUN_8064d790.c` |
| `0x8064E5F0` | `fn_8064e5f0` | allocate `0x398` bytes and construct the `BGM_anim_dummy` actor | translated | high | `8064e5f0_FUN_8064e5f0.c` |
| `0x8064E870` | `fn_8064e870` | load `BGM_anim_dummy` and select its RtoL/LtoR clip | translated | high | `8064e870_FUN_8064e870.c` |
| `0x80668670` | `fn_80668670` | load `lift_zen_kaiten` with a caller-supplied pane | translated | high | `80668670_FUN_80668670.c` |
| `0x80674C00` | `fn_80674c00` | load `middle_flag` model, tag and panes | translated | high | `80674c00_FUN_80674c00.c` |
| `0x80676E10` | `fn_80676e10` | load `houdai` mouth, body and root panes | translated | high | `80676e10_FUN_80676e10.c` |
| `0x80677730` | `fn_80677730` | allocate `0x398` bytes and construct the `obj_door_obake` actor | translated | high | `80677730_FUN_80677730.c` |
| `0x80677870` | `fn_80677870` | load `obj_door_obake` and its door pane | translated | high | `80677870_FUN_80677870.c` |
| `0x80677DD0` | `fn_80677dd0` | load `block_slide` and bind `count` panes | translated | high | `80677dd0_FUN_80677dd0.c` |
| `0x80687850` | `fn_80687850` | allocate `0x560` bytes and construct an unnamed actor | translated | high | `80687850_FUN_80687850.c` |
| `0x806901B0` | `fn_806901b0` | allocate `0x398` bytes and construct `Mask_flashlight_obake` | translated | high | `806901b0_FUN_806901b0.c` |
| `0x80690430` | `fn_80690430` | load `Mask_flashlight_obake` and its flashlight pane | translated | high | `80690430_FUN_80690430.c` |
| `0x806C0B70` | `fn_806c0b70` | allocate `0x528` bytes and construct the `hashigo` actor | translated | high | `806c0b70_FUN_806c0b70.c` |
| `0x806C0E10` | `fn_806c0e10` | load `hashigo` and the pane selected by index | translated | high | `806c0e10_FUN_806c0e10.c` |
| `0x806CBC70` | `fn_806cbc70` | allocate `0x3F8` bytes and construct the `lift_huriko` actor | translated | high | `806cbc70_FUN_806cbc70.c` |
| `0x806CBCE0` | `fn_806cbce0` | load `lift_huriko` and its `tomegu` pane | translated | high | `806cbce0_FUN_806cbce0.c` |
| `0x806F1CB0` | `fn_806f1cb0` | allocate `0x4A0` bytes and construct an unnamed actor | translated | high | `806f1cb0_FUN_806f1cb0.c` |
| `0x80705540` | `fn_80705540` | allocate `0x408` bytes and construct `noboribou_obake` | translated | high | `80705540_FUN_80705540.c` |
| `0x80705710` | `fn_80705710` | load `noboribou_obake` and its pane | translated | high | `80705710_FUN_80705710.c` |
| `0x80711E90` | `fn_80711e90` | allocate `0x650` bytes and construct an unnamed actor | translated | high | `80711e90_FUN_80711e90.c` |
| `0x8071EC20` | `fn_8071ec20` | allocate `0x560` bytes and construct the `obj_yashi` actor | translated | high | `8071ec20_FUN_8071ec20.c` |
| `0x8071EDB0` | `fn_8071edb0` | load `obj_yashi` and its wait animation | translated | high | `8071edb0_FUN_8071edb0.c` |
| `0x8072A8E0` | `fn_8072a8e0` | allocate `0x5A8` bytes and construct an unnamed actor | translated | high | `8072a8e0_FUN_8072a8e0.c` |
| `0x80745A80` | `fn_80745a80` | allocate `0x640` bytes and construct the `untei_rope` actor | translated | high | `80745a80_FUN_80745a80.c` |
| `0x80745B30` | `fn_80745b30` | load `untei_rope` and its two `tomegu` panes | translated | high | `80745b30_FUN_80745b30.c` |
| `0x807A6360` | `fn_807a6360` | allocate `0x5E0` bytes and construct an unnamed actor | translated | high | `807a6360_FUN_807a6360.c` |

**Totals:** 30 translated, 0 partial, 0 stub. All 30 rows are high confidence:
every allocation size, vtable, resource string and binding offset is explicit.

## Deferred candidates

| Address range | Why deferred |
|---|---|
| `0x807BD110` | `dWorldSelect_c` state table: 13 states, 2,908 bytes; **rejected** for the same `unaff_GQR0`/`ldexpf` paired-single reconstruction failure documented in waves 15-17. |
| `0x806C3010` | 76-byte allocator wrapper for a `0x84`-byte object with vtable `0x807F8068`; no adjacent table or lifecycle. |
| ~165 model loaders | The remaining `0x800DF180` callers are structurally identical to this pass; schedule them as address-ordered batches once the class tables that own them are known. |
| 11 vtable-only constructors | The remaining 76-byte bodies that only call `0x801628C0`/`0x80064110` and set `+0x60` (for example `0x80652760`, `0x806537E0`, `0x806677E0`) plus the 84-byte `0x8074E120` are best handled with the class registration that names them. |
| State handlers of the 46 tables closed in wave 16 | Still the largest behavioural gap in the module; each needs its own handler pass. |

## Unknowns preserved

1. **Constructor class attribution.** Five constructors have no class string; the
   other ten are named from the adjacent loader. Their class vtables and
   resource blocks are explicit, but the attribution stays `// inferred:`.
2. **Base-constructor arguments.** Ghidra prints `0x80064110` without a `this`
   argument. The Rust passes `object`, following the documented wave-3/4
   convention; the base prototype itself remains outside this slice.
3. **Allocator and table builder.** `0x801628C0` is recorded as the shared heap
   allocator and `0x802DC9A0` as a callback-table builder. Their prototypes are
   `// unknown:`; the Rust keeps the exact stride/count arguments.
4. **Animation playback helpers.** `0x80166F10`, `0x801680E0` and `0x80168FE0`
   are printed with `(double)` casts but receive single-precision loads. The
   Rust uses the existing `call_f32_u32`/`call_f32_u32x2` wrappers.
5. **Pane-name pointer globals.** `0x807CD064` is read as a pointer to the
   `obj_yashi_wait` string, not as the string itself, matching the C's
   `PTR_s_obj_yashi_wait_807cd064` form.
6. **Two-vtable constructors.** `0x80687850` and `0x807A6360` write a base
   vtable and then the final class vtable; the intermediate value is preserved
   as `// observed:` rather than discarded.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 435 translated functions; this pass adds 30,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files. `cargo check` passes (exit 0); the
remaining warnings are outside this lane.
