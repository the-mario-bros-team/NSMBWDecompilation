# Coverage - wave 20 (`d_basesNP` stage-actor constructors and model loaders)

## How to read this file

This page records the twentieth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 28 bodies appended to
   `src/game/bases.rs`: eleven heap-allocating actor constructors and the
   seventeen BRRES model loaders that build their model/clip sub-objects.
2. **Why these functions?** - *Selection* explains how the pass found a
   coherent, contiguous slice of the unused stage-actor range after waves
   17-19 exhausted the select screens and the first loader families.
3. **What is still missing?** - *Deferred candidates* and *Unknowns preserved*
   record the rejected `dWorldSelect_c` table, the remaining loader backlog,
   and every class attribution that stays inferred or unknown.

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
(`0x8030xxxx`, `0x8032xxxx`, `0x8037xxxx`, `0x8042xxxx`, `0x807Cxxxx`) are
imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 20 |
|---|---:|
| New translated functions | 28 |
| Heap-allocating actor constructors | 11 |
| Actor model loaders | 17 |
| Selected bytes | 7,084 |
| Translated functions in `src/game/bases.rs` (cumulative) | 462 |
| Cumulative selected bytes | 178,012 (16.04 % of indexed bytes) |

The 28 new bodies select 7,084 of the 1,109,860 indexed bytes (0.64 %) and 28
of the 5,409 eligible entries (0.52 %). All 28 rows are `high` confidence: the
allocation sizes, vtables, callback-table geometries, resource strings and
pane/clip bindings are explicit. No row is `partial` or `stub`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All 435 addresses already
translated by waves 2-19 were removed before selection; none of the 28 new
addresses appears in an earlier ledger row or in `ledger/functions.csv`.

1. **One contiguous slice.** After wave 19 closed `0x8064B1C0..0x807A6360`
   with constructor/loader pairs, the untranslated scan still contained 318
   bodies that call the shared allocator `0x801628C0`, the in-place base
   constructor `0x80066FC0`, or the BRRES loader `0x800DF180`. The pass took a
   single address-ordered window, `0x8067AE50..0x80696D50`, so the next agent
   can continue from either end without re-searching.
2. **Constructors.** Eleven bodies were selected because their allocation
   size, class vtable and sub-object layout are complete in the C. All eleven
   keep the `fn_` name; their class identity is `// unknown:`. The two that sit
   directly below a loader family in this pass (`0x8068B3E0` below
   `gabon_rock`, `0x806936B0` below `lift_icebox`) let us infer their role but
   not their class name.
3. **Matching loaders.** For every constructor the adjacent resource block was
   checked: `0x800690C0` starts an accessor, `0x800DF180` loads the BRRES, and
   `0x80169D90`/`0x8016A3A0` bind the model. Seventeen loaders in the same
   window were selected this way, including two indexed families
   (`circle_ground`, `gabon_rock`) and the three `lift_icebox`/`sandpillar`
   clip loaders.
4. **Rejected decompiler failures.** No selected file contains `halt_baddata`,
   `unaff_`, or an unresolved `ldexpf`/paired-single reconstruction. The one
   `StateID` table left in the module (`0x807BD110`) remains rejected for the
   wave-15..19 paired-single failure.

## Clusters

### Heap-allocating constructors (`0x8067AE50..0x80694640`)

Each constructor allocates with `0x801628C0(size)`, returns zero when the
allocation fails, runs the in-place base constructor `0x80066FC0` (Ghidra drops
its `this` argument; the Rust passes the object, following the wave-3/4
convention), writes the class vtable at `+0x60`, and initialises its sub-objects.
The observed helper families are:

- resource accessors: `0x80069020`, created in pairs in the larger bodies;
- model holders: `0x80169CD0`, `0x8016A2F0`, `0x8016A5E0`;
- transform helpers: `0x802D7AA0` (with the shared vtable `0x80329958`) and
  `0x80168D10` (with `0x807E3E98`);
- callback tables: `0x802DC9A0` with explicit stride/count pairs (`0x74x8`,
  `0x88x8`, `0xE4x24`, `0xE4x3`, `0xE4x9`, `0x40x2`, `0x154x10`);
- model initialisers: `0x8007F7A0`, `0x800DB130`, `0x800DB450`,
  `0x800DB6F0`, `0x8008C200`.

Two constructors replace a pointer after building a sub-object: `0x8068A410`
and `0x80692920` write eight `0x124`-byte clip blocks and then fill a
self-pointer/index table. `0x80691450` is the only body that replaces the
`+0x60` vtable itself (`0x80306F78` then `0x807EFCFC`); both writes are
preserved in order.

### Model loaders (`0x80680560..0x80696D50`)

Each loader follows the NW4R sequence seen in waves 17-19:

1. `0x800690C0` starts a resource accessor at `actor + offset`, with the
   main.dol archive handle at `0x80377C48`.
2. `0x800DF180(main.dol manager + 4, pane/name, brres)` loads the container.
3. `0x80239E30`/`0x8023A0B0`/`0x8023A190`/`0x8023A200`/`0x8023A270` resolve
   the model and animation objects.
4. `0x80169D90`/`0x8016A3A0` bind the model; `0x80064BD0`/`0x80064BF0`/
   `0x80064C10` finalise it; `0x801650D0`/`0x80165C30`/`0x80167420` create
   clips and `0x801651F0`/`0x80165FD0`/`0x80166340`/`0x80167800`/`0x80169F80`
   apply them.
5. `0x800690E0` releases the accessor.

The BRRES / pane evidence is:

| Address | BRRES / pane evidence |
|---|---|
| `0x80680560` | `circle_ground_holeD8` / `g3d/circle_ground_holeD8.brres` |
| `0x80680610` | `obj_dokan` / `g3d/obj_dokan.brres`, panes `obj_dokan_A` and `obj_dokan_B` |
| `0x80681B90` | indexed `circle_ground` tables at `0x807ECE80`/`0x807ECEF8`/`0x807ECFA8` |
| `0x80682900` | indexed `circle_ground_holeD` tables at `0x807ED188`/`0x807ED1D8`/`0x807ED1E0` |
| `0x806829C0` | `obj_dokan` second body, panes A/B at `+0xA20`/`+0xA60` |
| `0x80683900` | `boss_koopaJr_down_asiba` / `g3d/boss_koopaJr_down_asiba.brres` |
| `0x80684B90` | `boss_koopaJr_move_asiba` / `g3d/boss_koopaJr_move_asiba.brres` |
| `0x80688290` | `pakkun_water` / `g3d/pakkun_water.brres`, pane `pakkun_water_ball` |
| `0x80689AB0` | `lift_freehole` / `g3d/lift_freehole.brres` |
| `0x8068A860` | `block_ice` / `g3d/block_ice.brres` |
| `0x8068B5F0` | indexed `gabon_rock` tables at `0x807EF028`/`0x807EF058`/`0x807EF070`, clip `throw_rock` |
| `0x8068E820` | `obj_gear` / `g3d/obj_gear.brres`, pane table `0x807EF490` |
| `0x80690850` | `light_search` / `g3d/light_search.brres`, fog clip and fog animation |
| `0x80692C90` | `lift_icebox` / `g3d/lift_icebox.brres` |
| `0x806938B0` | `lift_icebox` plus `lift_icebox_crack` |
| `0x80694A70` | `sandpillar` / `g3d/sandpillar.brres`, animation `sandpillar_end` |
| `0x80696D50` | `I_ship_key` / `g3d/I_ship_key.brres` |

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave20-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8067AE50` | `fn_8067ae50` | allocate `0x540` bytes and construct an unnamed actor | translated | high | `8067ae50_FUN_8067ae50.c` |
| `0x8067F4D0` | `fn_8067f4d0` | allocate `0x3758` bytes and build four callback tables | translated | high | `8067f4d0_FUN_8067f4d0.c` |
| `0x80680560` | `fn_80680560` | load `circle_ground_holeD8` and its model | translated | high | `80680560_FUN_80680560.c` |
| `0x80680610` | `fn_80680610` | load `obj_dokan` and fill the A/B model pair array | translated | high | `80680610_FUN_80680610.c` |
| `0x80681B90` | `fn_80681b90` | load the indexed `circle_ground` variant | translated | high | `80681b90_FUN_80681b90.c` |
| `0x80682120` | `fn_80682120` | allocate `0xAD8` bytes and build two accessors and three holders | translated | high | `80682120_FUN_80682120.c` |
| `0x80682900` | `fn_80682900` | load the indexed `circle_ground_holeD` variant | translated | high | `80682900_FUN_80682900.c` |
| `0x806829C0` | `fn_806829c0` | load `obj_dokan` and its A/B panes | translated | high | `806829c0_FUN_806829c0.c` |
| `0x80683900` | `fn_80683900` | load `boss_koopaJr_down_asiba` | translated | high | `80683900_FUN_80683900.c` |
| `0x80684B90` | `fn_80684b90` | load `boss_koopaJr_move_asiba` | translated | high | `80684b90_FUN_80684b90.c` |
| `0x80688290` | `fn_80688290` | load `pakkun_water` and its ball pane | translated | high | `80688290_FUN_80688290.c` |
| `0x80689A40` | `fn_80689a40` | allocate `0x470` bytes and construct an unnamed actor | translated | high | `80689a40_FUN_80689a40.c` |
| `0x80689AB0` | `fn_80689ab0` | load `lift_freehole` | translated | high | `80689ab0_FUN_80689ab0.c` |
| `0x8068A410` | `fn_8068a410` | allocate `0xDA8` bytes and build eight helper blocks | translated | high | `8068a410_FUN_8068a410.c` |
| `0x8068A860` | `fn_8068a860` | load `block_ice`, its clip and default frame | translated | high | `8068a860_FUN_8068a860.c` |
| `0x8068B3E0` | `fn_8068b3e0` | allocate `0x720` bytes and build two transform helpers | translated | high | `8068b3e0_FUN_8068b3e0.c` |
| `0x8068B5F0` | `fn_8068b5f0` | load the indexed `gabon_rock` variant and `throw_rock` clip | translated | high | `8068b5f0_FUN_8068b5f0.c` |
| `0x8068DE20` | `fn_8068de20` | allocate `0xC80` bytes and build a nine-entry callback table | translated | high | `8068de20_FUN_8068de20.c` |
| `0x8068E820` | `fn_8068e820` | load the indexed `obj_gear` model | translated | high | `8068e820_FUN_8068e820.c` |
| `0x80690850` | `fn_80690850` | load `light_search`, its fog clip and fog animation | translated | high | `80690850_FUN_80690850.c` |
| `0x80691450` | `fn_80691450` | allocate `0x508` bytes and install the final vtable | translated | high | `80691450_FUN_80691450.c` |
| `0x80692920` | `fn_80692920` | allocate `0xDA0` bytes and build eight helper blocks | translated | high | `80692920_FUN_80692920.c` |
| `0x80692C90` | `fn_80692c90` | load `lift_icebox` and its clip | translated | high | `80692c90_FUN_80692c90.c` |
| `0x806936B0` | `fn_806936b0` | allocate `0x5A0` bytes and build a model table and helper pair | translated | high | `806936b0_FUN_806936b0.c` |
| `0x806938B0` | `fn_806938b0` | load `lift_icebox` and the `crack` model | translated | high | `806938b0_FUN_806938b0.c` |
| `0x80694640` | `fn_80694640` | allocate `0x1678` bytes and build a ten-entry table and three blocks | translated | high | `80694640_FUN_80694640.c` |
| `0x80694A70` | `fn_80694a70` | load `sandpillar`, its clip and end animation | translated | high | `80694a70_FUN_80694a70.c` |
| `0x80696D50` | `fn_80696d50` | load `I_ship_key` | translated | high | `80696d50_FUN_80696d50.c` |

**Totals:** 28 translated, 0 partial, 0 stub. All 28 rows are high confidence:
every allocation size, vtable, callback-table geometry, resource string and
binding offset is explicit.

## Deferred candidates

| Address range | Why deferred |
|---|---|
| `0x807BD110` | `dWorldSelect_c` state table: 13 states, 2,908 bytes; **rejected** for the same `unaff_GQR0`/`ldexpf` paired-single reconstruction failure documented in waves 15-19. |
| `0x80696D50+` | The remaining stage-actor constructors and loaders continue immediately above this pass; they are structurally identical and best scheduled as the next address-ordered batch. |
| ~148 model loaders | The remaining `0x800DF180` callers are the same NW4R sequence; schedule them once their owning class tables are known. |
| Unnamed constructors in this pass | Their vtables and layouts are translated, but no string names the class; a registration/state-table pass is needed to attribute them. |
| State handlers of the 46 tables closed in wave 16 | Still the largest behavioural gap in the module; each needs its own handler pass. |

## Unknowns preserved

1. **Constructor class attribution.** None of the eleven constructors carries a
   class string. Their vtables (`0x807EC158`, `0x807ECCD8`, `0x807ED220`,
   `0x807EEAB0`, `0x807EEC78`, `0x807EF0B8`, `0x807EF4B8`, `0x807EFCFC`,
   `0x807F01D8`, `0x807F0588`, `0x807F07E0`) are explicit, but every
   attribution stays `// unknown:`.
2. **Base-constructor arguments.** Ghidra prints `0x80066FC0` without a `this`
   argument. The Rust passes `object`, following the documented wave-3/4
   convention; the base prototype itself remains outside this slice.
3. **Allocator and table builder.** `0x801628C0` is recorded as the shared heap
   allocator and `0x802DC9A0` as a callback-table builder. Their prototypes are
   `// unknown:`; the Rust keeps the exact stride/count arguments.
4. **Clip/animation playback helpers.** `0x80165C30`, `0x80167420` and
   `0x80169F80` are printed with `(double)` casts but receive single-precision
   loads. The Rust uses `call_f32_u32x2` for `0x80169F80` and keeps the other
   calls as raw ABI calls.
5. **Indexed pane tables.** `0x80681B90`, `0x80682900`, `0x8068B5F0` and
   `0x8068E820` index pointer tables with an actor field. The tables and field
   offsets are explicit; the meaning of each variant index is `// inferred:`.
6. **Misaligned string symbol.** `0x806829C0` prints
   `s_lg3d_obj_dokan_brres_807ed1e7 + 1`; the Rust uses the resolved address
   `0x807ED1E8` and notes the C form.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 462 translated functions; this pass adds 28,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files. `cargo check` passes (exit 0); the
remaining warnings are outside this lane.
