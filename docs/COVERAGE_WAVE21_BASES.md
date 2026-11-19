# Coverage - wave 21 (`d_basesNP` constructors, loaders and lifecycle)

## How to read this file

This page records the twenty-first translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 30 bodies appended to
   `src/game/bases.rs`: sixteen heap-allocating constructors, six in-place
   constructors, five BRRES model loaders, one destructor and two small
   lifecycle helpers.
2. **Why these functions?** - *Selection* explains how the pass continued the
   address-ordered sweep above the wave-20 window and how each constructor was
   paired with its loader or teardown code.
3. **What is still missing?** - *Deferred candidates* and *Unknowns preserved*
   record the remaining constructor/loader backlog, the class attributions that
   stay inferred, and the compiler thunks that were deliberately omitted.

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
(`0x8030xxxx`, `0x8037xxxx`, `0x8042xxxx`, `0x807Cxxxx`) are imported absolute
values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 21 |
|---|---:|
| New translated functions | 30 |
| Heap-allocating constructors | 16 |
| In-place constructors | 6 |
| BRRES model loaders | 5 |
| Destructor / lifecycle helpers | 3 (one destructor, two helpers) |
| Selected bytes | 4,832 |
| Translated functions in `src/game/bases.rs` (cumulative) | 492 |
| Cumulative selected bytes | 182,844 (16.47 % of indexed bytes) |

The 30 new bodies select 4,832 of the 1,109,860 indexed bytes (0.44 %) and 30
of the 5,409 eligible entries (0.55 %). Twenty-nine rows are `high`
confidence; the lifecycle helper `0x806A6980` is `medium` because its role is
inferred from the surrounding actor code even though its body is complete. No
row is `partial` or `stub`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All 462 addresses already
translated by waves 2-20 were removed before selection. The candidate scan then
kept bodies with a complete allocation/vtable/store sequence or the NW4R
`0x800DF180` loader sequence.

1. **Continue the address sweep.** Wave 20 stopped at `0x80696D50`, so this pass
   started at `0x806969B0`, the constructor immediately below that loader, and
   moved upward to `0x806B1730`. Addresses below the start that were already
   translated were left untouched; `0x806A8640` (wave 17) is the one candidate
   inside the window that was skipped because its body already exists.
2. **Constructors.** The selected factories allocate `0x398..0xFD0` bytes,
   call a base constructor, and install an explicit vtable. The two in-place
   constructor families at `0x806992E0`/`0x806A15F0` and
   `0x8069DBC0`/`0x8069DB90` show the usual subclass pattern: the shared base
   initialiser runs first and the derived vtable is written last.
3. **Matching loaders.** Every constructor that owns a model block was paired
   with the adjacent `0x800DF180` caller: `I_ship_key` (`0x80696D50`, already
   translated), `koopa_clown` (`0x8069DCE0`), `boss_ef_attack`
   (`0x806AE2F0`), `kanban_yajirushi` (`0x806AF450`),
   `lift_katamukiyuka` (`0x806AF930`) and `kazan_rock` (`0x806B1730`).
4. **Lifecycle.** `0x806A6230` is the destructor matching the `0x806A60E0`
   layout (it tears down the same `+0x488`/`+0x5B0` transform blocks).
   `0x806A6980` polls the model holder at frame boundaries, and
   `0x806A74A0` starts the actor by resetting its model, snapshotting its
   position and selecting the initial state.
5. **Rejected decompiler failures.** No selected file contains
   `halt_baddata`, `unaff_`, or an unresolved `ldexpf`/paired-single
   reconstruction. The `0x80699940` and `0x80699A30` bodies above the
   `I_ship_key` constructor were rejected for that reason.

## Clusters

### `I_ship_key` and the `0x8069` constructors (`0x806969B0..0x8069DCE0`)

- `0x806969B0` allocates `0x448` bytes and builds the accessor/model pair that
  `0x80696D50` fills with `I_ship_key`.
- `0x806976D0` is a minimal `0x3D0`-byte factory (base plus vtable).
- `0x806992B0`/`0x806992E0` and `0x8069DB90`/`0x8069DBC0` are two
  allocator/in-place constructor pairs; the second pair owns the
  `koopa_clown` model block.
- `0x8069DCE0` loads `koopa_clown`, binds pane `launch`, creates the launch
  clip and the damage clip, and stores the damage animation at `+0x5E8`.

### `0x806A` factories (`0x806A15C0..0x806A9740`)

- `0x806A15C0`/`0x806A15F0` reuse `0x806992E0` and then install
  `0x807F22E0`.
- `0x806A2290`/`0x806A2E00` build a `0xFD0`-byte object with a tall transform
  block at `+0xEAC` and an interface at `+0xFC0`.
- `0x806A60E0` is the largest constructor in the `0x806A` block: two transform blocks
  (`+0x488`, `+0x5B0`), a helper pair (`+0x434`/`+0x440`) and a `0xC`-stride
  callback table at `+0x6E0`.
- `0x806A6230` is the matching destructor; it tears the same blocks down in
  reverse order and optionally frees the object.
- `0x806A8560` forwards to the already-translated `0x806A8640`;
  `0x806A8590` builds two `0x80310ED8` transform blocks.
- `0x806A96D0`/`0x806A9740` build three model holders, three
  `0x807E3E98` transforms and one `0x807EDF98` interface block.

### Model loaders and their owners (`0x806AB680..0x806B1730`)

| Constructor | Loader | Resource |
|---|---|---|
| `0x806AB680` | - | `0x968`-byte object with sound/animation block |
| `0x806AE130` | `0x806AE2F0` | `boss_ef_attack` / `kameck_attack` |
| `0x806AF1B0` | `0x806AF450` | `kanban_yajirushi` |
| `0x806AF8C0` | `0x806AF930` | `lift_katamukiyuka` |
| `0x806B0270` | - | `0x398`-byte base-only object |
| `0x806B1500` | `0x806B1730` | `kazan_rock` |

Each loader follows the same NW4R sequence as waves 17-20: `0x800690C0`
starts the accessor, `0x800DF180` loads the BRRES, `0x80239E30`/`0x8023A0B0`/
`0x8023A190`/`0x8023A200` resolve the model/animation objects, `0x80169D90`
binds the model, `0x80064BE0`/`0x80064BF0` finalises it, `0x801650D0`/
`0x80165C30`/`0x80166830` create clips, and `0x800690E0` releases the
accessor. The float-taking calls `0x80169F80`, `0x801680E0` and `0x80168FE0`
use the existing single-precision ABI wrappers.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave21-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806969B0` | `fn_806969b0` | allocate `0x448` bytes and construct the `I_ship_key` actor | translated | high | `806969b0_FUN_806969b0.c` |
| `0x806976D0` | `fn_806976d0` | allocate `0x3D0` bytes and install vtable `0x807F0CF0` | translated | high | `806976d0_FUN_806976d0.c` |
| `0x806992B0` | `fn_806992b0` | allocate `0xDE8` bytes and forward to the in-place constructor | translated | high | `806992b0_FUN_806992b0.c` |
| `0x806992E0` | `fn_806992e0` | initialise the `0xDE8`-byte object and write `0x807F1450` | translated | high | `806992e0_FUN_806992e0.c` |
| `0x8069DB90` | `fn_8069db90` | allocate `0xEB0` bytes and forward to the in-place constructor | translated | high | `8069db90_FUN_8069db90.c` |
| `0x8069DBC0` | `fn_8069dbc0` | initialise the `koopa_clown` object and its transform | translated | high | `8069dbc0_FUN_8069dbc0.c` |
| `0x8069DCE0` | `fn_8069dce0` | load `koopa_clown` and bind its launch/damage clips | translated | high | `8069dce0_FUN_8069dce0.c` |
| `0x806A15C0` | `fn_806a15c0` | allocate `0xDE8` bytes and forward to the derived constructor | translated | high | `806a15c0_FUN_806a15c0.c` |
| `0x806A15F0` | `fn_806a15f0` | reuse `0x806992E0` and install vtable `0x807F22E0` | translated | high | `806a15f0_FUN_806a15f0.c` |
| `0x806A2290` | `fn_806a2290` | allocate `0xFD0` bytes and forward to the in-place constructor | translated | high | `806a2290_FUN_806a2290.c` |
| `0x806A2E00` | `fn_806a2e00` | initialise the `0xFD0`-byte object with its tall transform and interface | translated | high | `806a2e00_FUN_806a2e00.c` |
| `0x806A60E0` | `fn_806a60e0` | allocate `0x738` bytes and build two transform blocks and a callback table | translated | high | `806a60e0_FUN_806a60e0.c` |
| `0x806A6230` | `fn_806a6230` | destroy the `0x806A60E0` layout and optionally free it | translated | high | `806a6230_FUN_806a6230.c` |
| `0x806A6980` | `fn_806a6980` | poll the model holder and finalise the actor when the frame ends | translated | medium | `806a6980_FUN_806a6980.c` |
| `0x806A74A0` | `fn_806a74a0` | start the actor, snapshot its position and select the initial state | translated | high | `806a74a0_FUN_806a74a0.c` |
| `0x806A8560` | `fn_806a8560` | allocate `0x5F0` bytes and call `fn_806a8640` | translated | high | `806a8560_FUN_806a8560.c` |
| `0x806A8590` | `fn_806a8590` | allocate `0x790` bytes and build two `0x80310ED8` transforms | translated | high | `806a8590_FUN_806a8590.c` |
| `0x806A96D0` | `fn_806a96d0` | allocate `0x860` bytes and forward to `fn_806a9740` | translated | high | `806a96d0_FUN_806a96d0.c` |
| `0x806A9740` | `fn_806a9740` | build three model holders, three transforms and one interface | translated | high | `806a9740_FUN_806a9740.c` |
| `0x806AB680` | `fn_806ab680` | allocate `0x968` bytes and forward to `fn_806ab6e0` | translated | high | `806ab680_FUN_806ab680.c` |
| `0x806AB6E0` | `fn_806ab6e0` | build two callback tables, a sound emitter and an animation controller | translated | high | `806ab6e0_FUN_806ab6e0.c` |
| `0x806AE130` | `fn_806ae130` | allocate `0x6E0` bytes and build two transform blocks | translated | high | `806ae130_FUN_806ae130.c` |
| `0x806AE2F0` | `fn_806ae2f0` | load `boss_ef_attack` and bind `kameck_attack` | translated | high | `806ae2f0_FUN_806ae2f0.c` |
| `0x806AF1B0` | `fn_806af1b0` | allocate `0x470` bytes with an accessor, holder and transform | translated | high | `806af1b0_FUN_806af1b0.c` |
| `0x806AF450` | `fn_806af450` | load `kanban_yajirushi`, its clip and default frame | translated | high | `806af450_FUN_806af450.c` |
| `0x806AF8C0` | `fn_806af8c0` | allocate `0x498` bytes with a `0x800DB130` helper | translated | high | `806af8c0_FUN_806af8c0.c` |
| `0x806AF930` | `fn_806af930` | load `lift_katamukiyuka` and bind a variant pane | translated | high | `806af930_FUN_806af930.c` |
| `0x806B0270` | `fn_806b0270` | allocate `0x398` bytes and install vtable `0x807F4C84` | translated | high | `806b0270_FUN_806b0270.c` |
| `0x806B1500` | `fn_806b1500` | allocate `0x480` bytes with a helper pair | translated | high | `806b1500_FUN_806b1500.c` |
| `0x806B1730` | `fn_806b1730` | load `kazan_rock`, its variant model and clip | translated | high | `806b1730_FUN_806b1730.c` |

**Totals:** 30 translated, 0 partial, 0 stub (29 high, 1 medium:
`0x806A6980`).

## Deferred candidates

| Address range | Why deferred |
|---|---|
| `0x806B1830+` | The next constructor/loader family starts immediately above this pass; `0x806B1830`, `0x806B1970` and the `0x806B6DE0`/`0x806B8E70` loaders are structurally identical. |
| `0x80699940`, `0x80699A30` | **Rejected**: `unaff_GQR0`/`ldexpf` paired-single reconstruction, the same failure documented in waves 15-20. |
| `0x807BD110` | `dWorldSelect_c` state table, rejected for the same paired-single failure. |
| ~140 model loaders | The remaining `0x800DF180` callers are the same NW4R sequence; schedule them once their owning class tables are known. |
| Unnamed constructors in this pass | Their vtables and layouts are translated, but no string names the class; a registration/state-table pass is needed to attribute them. |
| State handlers of the tables closed in wave 16 | Still the largest behavioural gap in the module; each needs its own handler pass. |

## Unknowns preserved

1. **Constructor class attribution.** None of the sixteen heap-allocating
   constructors carries a class string. Their vtables (`0x807F0AB8`,
   `0x807F0CF0`, `0x807F1450`, `0x807F1B38`, `0x807F22E0`, `0x807F2868`,
   `0x807F2F58`, `0x807F3288`, `0x807F39C8`, `0x807F3FF0`, `0x807F4668`,
   `0x807F4900`, `0x807F4AD8`, `0x807F4C84`, `0x807F50E8`) are explicit, but
   every attribution stays `// unknown:`.
2. **Base-constructor arguments.** Ghidra prints `0x80066FC0`, `0x80064110`,
   `0x80094E80` and `0x800A2D20` without a `this` argument. The Rust passes the
   object explicitly, following the wave-3/4 convention; the base prototypes
   themselves remain outside this slice.
3. **Compiler thunks.** `0x802DCD60`, `0x802DCD68`, `0x802DCD74`,
   `0x802DCDAC`, `0x802DCDB4` and `0x802DCDC0` are register-save/restore
   helpers with no Rust-visible effect. They are omitted and named in comments
   at the call sites.
4. **Callback label.** `0x806AB6E0` passes `0x806AB8C0` to the table builder as
   a code label rather than a function entry; the Rust keeps the raw address.
5. **Variant tables.** `0x806AF930` and `0x806B1730` index pane tables with an
   actor field or argument. The tables and offsets are explicit; the meaning of
   each variant index is `// inferred:`.
6. **Lifecycle helpers.** `0x806A6980` reads a five-word record but the record
   layout is not named; `0x806A74A0` starts the actor through the virtuals at
   `+0x60`/`+0x5B0`, whose exact slots are explicit but whose class names
   remain `// unknown:`.
7. **Float ABI.** `0x80169F80`, `0x801680E0` and `0x80168FE0` are printed with
   `(double)` casts but receive single-precision loads. The Rust uses the
   existing `call_f32_u32`/`call_f32_u32x2` wrappers and `read_f32` for the
   constants.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` contains 492 translated functions after this pass; this
pass adds 30, all with `# Original` blocks and the synthetic-base warning.
`doc-lint` reports `ok: true`, 0 violations over 68 files. `cargo check` exits
0 at the time of this pass; `src/game/bases.rs` also compiles cleanly in an
isolated crate with the current `src/platform.rs`, and adds no warnings. The
remaining crate warnings are outside this lane.
