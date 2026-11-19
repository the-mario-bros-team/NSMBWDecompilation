# Coverage - wave 22 (`d_basesNP` actor behaviour and teardown)

## How to read this file

This page records the twenty-second translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 30 bodies appended to
   `src/game/bases.rs`: the `kazan_rock` actor's execute/transform/effect/
   collision/teardown cluster, the `0x510`-byte fire actor and its
   `Wm_en_keronpafire` spawn, the `0x1360`-byte array owner with its six inline
   arrays and teardown helpers, and one small `0x398`-byte constructor.
2. **Why these functions?** - *Selection* explains how the pass continued the
   address-ordered sweep above the wave-21 window and why the rejected
   decompiler failures were skipped.
3. **What is still missing?** - *Deferred candidates* and *Unknowns preserved*
   record the remaining backlog, the class attributions that stay inferred, and
   the arguments the decompiler loses.

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
(`0x8006xxxx`, `0x8016xxxx`, `0x8042xxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 22 |
|---|---:|
| New translated functions | 30 |
| `kazan_rock` actor behaviour | 14 (execute, transform, effects, collision, destructor) |
| `0x510`-byte fire actor | 7 (constructor, spawn, probe, hit, speed, update) |
| Array owner / teardown helpers | 8 |
| Small constructor | 1 |
| Selected bytes | 4,632 |
| Translated functions in `src/game/bases.rs` (cumulative) | 523 |
| Cumulative selected bytes | 187,476 (16.89 % of indexed bytes) |

The 30 new bodies select 4,632 of the 1,109,860 indexed bytes (0.42 %) and 30
of the 5,409 eligible entries (0.55 %). All 30 rows are `high` confidence: every
call, store, constant and branch is explicit. Four class attributions remain
`// inferred:` or `// unknown:` and are recorded in *Unknowns preserved*; they do
not weaken the bodies. No row is `partial` or `stub`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2-21 were removed before selection. Wave 21 stopped at
`0x806B1730`, so this pass started at `0x806B1830`, the next constructor-adjacent
body, and moved upward to `0x806B4870`.

1. **Continue the address sweep.** The selected range is
   `0x806B1830..0x806B4870`. Every body has a complete control-flow graph; no
   selected file contains `halt_baddata`, `unaff_`, or an `ldexpf`/paired-single
   reconstruction.
2. **Follow the actor cluster.** `0x806B1500` builds the `0x480`-byte object
   with vtable `0x807F50E8` and `0x806B1730` loads its `kazan_rock` BRRES
   (both wave 21). The adjacent bodies `0x806B1830..0x806B2370` are that
   object's runtime, so they were taken as one group rather than as isolated
   functions.
3. **Follow the second actor.** `0x806B2730` allocates `0x510` bytes and
   installs vtable `0x807F52B8`; `0x806B2AB0` spawns the model named
   `Wm_en_keronpafire`. The constructor, spawn, probe, hit and movement bodies
   form the second group.
4. **Take the self-contained teardown family.** `0x806B33C0` builds six inline
   arrays with `0x802DC9A0`; the matching loops and destructors
   `0x806B3530..0x806B3960` are complete and were translated with it.
5. **Rejected decompiler failures.** `0x806B1AE0` ends in `halt_baddata`;
   `0x806B2BB0`, `0x806B2D10`, `0x806B3770`, `0x806B4BA0` and `0x806B5180`
   contain `unaff_GQR0`/`unaff_GQR3` plus `ldexpf`; `0x806B3BB0` has an
   unrecovered jump table. All were skipped and are listed below.

## Clusters

### `kazan_rock` actor (`0x806B1830..0x806B2370`)

- `0x806B1830` is the frame entry: it refreshes the `+0x394` interface and the
  `+0x430` clip block, rebuilds the transform through `0x806B1970`, spawns the
  landing dust once, plays sound `0x2CC` on the first grounded frame, and calls
  `0x806B1AE0`.
- `0x806B1970` copies the position (`+0xAC`) and the three `s16` angles
  (`+0x100`), builds the matrix at `+0x7C` with `0x801C0BD0`, applies the three
  rotations and hands the result to the model holder at `+0x3F0`.
- `0x806B1B10`/`0x806B1BE0`/`0x806B1C90`/`0x806B1D40` spawn the landing sound
  and the two box-smoke/box-grain effects; the variant field `+0x464` selects
  the effect id and scale rows in the `0x807CA930..0x807CA958` tables.
- `0x806B1DA0` is the collision callback; `0x806B2010`/`0x806B2230` are the two
  execute variants (the second differs only in two float constants and the
  fallback gate); `0x806B2370` is the destructor.

### Fire actor (`0x806B2730..0x806B3240`)

- `0x806B2730` builds the `0x510`-byte object and the interface at `+0x3DC`.
- `0x806B2AB0` queries the `+0x394` interface, retires the actor on a confirmed
  probe, and otherwise calls the `+0x3DC` virtual `+0xA0` to create
  `Wm_en_keronpafire` (`0x807F52A0`).
- `0x806B2890` probes two collision helpers with a four-word filter built from
  `0x807CA9E8`/`0x807CA9EC`.
- `0x806B2930` plays `Wm_mr_fireball_hit` (`0x807F528C`) and sound `0x268`.
- `0x806B29C0` decays the speed; `0x806B2DF0` integrates it and grows the
  scale; `0x806B3240` is the state-object destructor.

### Array owner and teardown (`0x806B33C0..0x806B3960`, `0x806B4870`)

- `0x806B33C0` allocates `0x1360` bytes and creates six inline arrays:
  `+0x3F0` (3 × `0x60`), `+0x558` (`0x100` × `0xC`), `+0x1160` (9 × 8),
  `+0x11C8` (9 × `0xC`), `+0x1234` (3 × 8) and `+0x1254` (3 × `0x40`).
- `0x806B38B0`/`0x806B3960` reset, and `0x806B3900` destroys through the
  virtual `+0xC`, the three `0x60`-byte elements at `+0x3F0`.
- `0x806B3530` frees one array element; `0x806B3570`/`0x806B35F0`/
  `0x806B3690` destroy the arrays and the base object.
- `0x806B4870` is an independent `0x398`-byte constructor with vtable
  `0x807F5744`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave22-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806B1830` | `fn_806b1830` | run one frame: refresh, transform, dust, landing sound, tail call | translated | high | `806b1830_FUN_806b1830.c` |
| `0x806B1970` | `fn_806b1970` | rebuild the model matrix from position and three angles | translated | high | `806b1970_FUN_806b1970.c` |
| `0x806B1A60` | `fn_806b1a60` | snapshot the model position into the effect record | translated | high | `806b1a60_FUN_806b1a60.c` |
| `0x806B1AB0` | `fn_806b1ab0` | ground test against the scene height | translated | high | `806b1ab0_FUN_806b1ab0.c` |
| `0x806B1B10` | `fn_806b1b10` | start the actor, queue its sound and spawn effects | translated | high | `806b1b10_FUN_806b1b10.c` |
| `0x806B1BE0` | `fn_806b1be0` | spawn landing dust 4/5 with position and size | translated | high | `806b1be0_FUN_806b1be0.c` |
| `0x806B1C90` | `fn_806b1c90` | spawn `Wm_ob_cmnboxsmoke`/`Wm_ob_cmnboxgrain` | translated | high | `806b1c90_FUN_806b1c90.c` |
| `0x806B1D40` | `fn_806b1d40` | one-shot effect chosen by the variant field | translated | high | `806b1d40_FUN_806b1d40.c` |
| `0x806B1DA0` | `fn_806b1da0` | collision callback: flag, virtuals, hit sounds | translated | high | `806b1da0_FUN_806b1da0.c` |
| `0x806B1EB0` | `fn_806b1eb0` | mark the hit flag on the owning object | translated | high | `806b1eb0_FUN_806b1eb0.c` |
| `0x806B2010` | `fn_806b2010` | execute variant A: rotate, probe, land or retire | translated | high | `806b2010_FUN_806b2010.c` |
| `0x806B2230` | `fn_806b2230` | execute variant B with its own thresholds | translated | high | `806b2230_FUN_806b2230.c` |
| `0x806B2370` | `fn_806b2370` | destroy the `0x480`-byte actor, optionally free | translated | high | `806b2370_FUN_806b2370.c` |
| `0x806B25B0` | `fn_806b25b0` | destroy a `daKazanRock_c` state object | translated | high | `806b25b0_FUN_806b25b0.c` |
| `0x806B2730` | `fn_806b2730` | allocate `0x510` bytes and build the fire actor | translated | high | `806b2730_FUN_806b2730.c` |
| `0x806B2890` | `fn_806b2890` | probe two collision helpers with a four-word filter | translated | high | `806b2890_FUN_806b2890.c` |
| `0x806B2930` | `fn_806b2930` | play the fireball hit and retire the actor | translated | high | `806b2930_FUN_806b2930.c` |
| `0x806B29C0` | `fn_806b29c0` | decay the speed at `+0x10C` with a floor | translated | high | `806b29c0_FUN_806b29c0.c` |
| `0x806B2AB0` | `fn_806b2ab0` | spawn `Wm_en_keronpafire` and report a probe hit | translated | high | `806b2ab0_FUN_806b2ab0.c` |
| `0x806B2DF0` | `fn_806b2df0` | integrate velocity, grow scale, report collision | translated | high | `806b2df0_FUN_806b2df0.c` |
| `0x806B3240` | `fn_806b3240` | destroy a state object of the fire family | translated | high | `806b3240_FUN_806b3240.c` |
| `0x806B33C0` | `fn_806b33c0` | allocate `0x1360` bytes and build six arrays | translated | high | `806b33c0_FUN_806b33c0.c` |
| `0x806B3530` | `fn_806b3530` | free one `0xC`-stride array element | translated | high | `806b3530_FUN_806b3530.c` |
| `0x806B3570` | `fn_806b3570` | destroy the `+0x30` array and the base | translated | high | `806b3570_FUN_806b3570.c` |
| `0x806B35F0` | `fn_806b35f0` | destroy the `+0xC38`/`+0x30` arrays and the base | translated | high | `806b35f0_FUN_806b35f0.c` |
| `0x806B3690` | `fn_806b3690` | destroy four arrays and the base | translated | high | `806b3690_FUN_806b3690.c` |
| `0x806B38B0` | `fn_806b38b0` | reset the three `0x60`-byte elements | translated | high | `806b38b0_FUN_806b38b0.c` |
| `0x806B3900` | `fn_806b3900` | destroy the three elements through virtual `+0xC` | translated | high | `806b3900_FUN_806b3900.c` |
| `0x806B3960` | `fn_806b3960` | release the three `0x60`-byte elements | translated | high | `806b3960_FUN_806b3960.c` |
| `0x806B4870` | `fn_806b4870` | allocate `0x398` bytes and install vtable `0x807F5744` | translated | high | `806b4870_FUN_806b4870.c` |

**Totals:** 30 translated, 0 partial, 0 stub (30 high).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x806B1AE0` | 40 | **Rejected**: the body ends in `halt_baddata`; only the call site is kept (by address) in `fn_806b1830`. |
| `0x806B2BB0` | 340 | **Rejected**: `unaff_GQR3` + `ldexpf` paired-single reconstruction. |
| `0x806B2D10` | 200 | **Rejected**: `unaff_GQR3` + `ldexpf`. |
| `0x806B3770` | 308 | **Rejected**: `unaff_GQR0`/`unaff_GQR3` + `ldexpf`. |
| `0x806B4BA0` | 456 | **Rejected**: `unaff_GQR0` + `ldexpf`. |
| `0x806B5180` | 620 | **Rejected**: `unaff_GQR0` + `ldexpf`. |
| `0x806B3BB0` | 40 | **Rejected**: jump table not recovered ("too many branches"). |
| `0x806B39B0` | 268 | Needs the argument lost across the `0x802DCD70` save thunk; body is otherwise complete. |
| `0x806B4A10` | 268 | Attack helper; depends on the constant getters `0x806B4EC0`/`0x806B4F40` and six-argument virtual `+0x3FC`. |
| `0x806B4B20` | 124 | Attack variant; same dependency, plus `countLeadingZeros` rounding. |
| `0x806B26D0`, `0x806B2700`, `0x806B4810`, `0x806B4840` | 48 each | Vector-deleting-destructor thunks around `0x802DCBFC`; no independent behaviour. |
| `0x806B3AC0` | 92 | Loader wrapper; Ghidra shows the first two arguments as `undefined8`, so the signature is not yet trustworthy. |
| `0x806B53F0+` | - | Next actor family (constructor/loader/execute), untouched by this pass. |

## Unknowns preserved

1. **Class attribution.** The `0x480`-byte actor (vtable `0x807F50E8`) is
   labelled `daKazanRock_c` only by `// inferred:` from the adjacent state table
   `0x806B23F0`. The fire actor (`0x807F52B8`), the array owner
   (`0x807F54B0`) and the `0x398`-byte constructor (`0x807F5744`) carry no
   class string; all four attributions stay `// unknown:`.
2. **Truncated callee.** `0x806B1AE0` is called by address because its own
   decompilation stops at `halt_baddata`; its real tail is not represented.
3. **Lost arguments.** Ghidra drops the second argument of `0x8006A4B0` in
   `fn_806b2930` (it is still in `r4`, so the caller's position is passed), the
   `this` of `0x806B29C0` in `fn_806b2df0` (still in `r3`), and the argument of
   `0x802B9280` in `fn_806b3530` (still in `r3`).
4. **24-byte effect block.** `fn_806b1be0` passes one pointer to
   `0x80092830`, but the six contiguous stack floats show the callee reads
   position and size together. The Rust reserves all six floats and documents
   the reconstruction.
5. **Float tables.** `0x807CA930..0x807CAA10` are read as raw `f32`; their
   physical meaning (dampening, scale step, effect size) is `// unknown:`.
6. **Array descriptors.** `0x802DC9A0` creates and `0x802DCA98` destroys inline
   arrays; the constructor/destructor pointers (`0x806691D0`, `0x80601D10`,
   `0x80169CD0`/`0x80169D20`, `0x800DB130`) are kept as raw addresses.
7. **Compiler helpers.** `0x802DCD70`/`0x802DCDBC` in `fn_806b33c0` are the
   register-save/restore fragments already documented in waves 15-21; they are
   omitted with a comment and have no Rust-visible effect.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` contains 523 translated functions after this pass; this pass
adds 30, all with `# Original` blocks and the synthetic-base warning. `cargo
check` exits 0 and `doc-lint` reports `ok: true`, 0 violations over 68 files.
