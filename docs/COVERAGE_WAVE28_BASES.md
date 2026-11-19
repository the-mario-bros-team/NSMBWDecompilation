# Coverage â€” wave 28 (`d_basesNP`: `remo_lift`/`remo_lift_line` lifts and loaders)

## How to read this file

This page records the twenty-eighth translation pass over `d_basesNP`, the
base gameplay REL (`game::bases`). It answers:

1. **What was added?** â€” *Translated functions* lists the 30 new bodies in
   `src/game/bases.rs`: the `remo_lift` and `remo_lift_line` rope-lift actors,
   the four-block `lift_suisha` `ashiba` loader, and ten single-pane lift
   loaders.
2. **Why these functions?** â€” *Selection* explains how the pass continued the
   address-ordered sweep above the wave-27 window and which decompiler outputs
   were skipped.
3. **What is still missing?** â€” *Deferred candidates* records the paired-single
   bodies, the two truncated `halt_baddata` bodies and the one body that needs
   an ABI review.

Column meanings:

- **Status** â€” `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** â€” strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, table meaning or
  called body is inferred; `low` = mostly guesswork.
- **Evidence** â€” file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8004xxxx`, `0x8005xxxx`, `0x8006xxxx`, `0x8016xxxx`, `0x8019xxxx`,
`0x802Dxxxx`, `0x8037xxxx`, `0x8042xxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output (or in the instruction stream where the C drops an argument);
`// inferred:` = role guessed from callers/shape; `// unknown:` = unresolved;
`// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 28 |
|---|---:|
| New translated functions | 30 |
| `remo_lift` lifecycle (`0x806D12F0..0x806D2330`) | 12 |
| `remo_lift_line` lifecycle (`0x806D2870..0x806D3540`) | 5 |
| `lift_suisha` four-block loader (`0x806E2320..0x806E3480`) | 3 |
| Single-pane model loaders (`0x806D5680..0x806E9E90`) | 10 |
| Selected bytes | 5,828 |
| Translated functions in `src/game/bases.rs` (cumulative) | 699 (697 translated, 2 partial) |
| Cumulative selected bytes | 221,296 (19.94 % of indexed bytes) |

The 30 new bodies select 5,828 of the 1,109,860 indexed bytes (0.53 %) and 30 of
the 5,409 eligible entries (0.55 %). 25 rows are `high` confidence; 5 are
`medium` (`0x806D1470`, `0x806D1D30`, `0x806D1E70`, `0x806D3540`,
`0x806E5DF0`) because a class role, a dropped argument or an unprototyped call
is inferred. No row is `partial` or `stub`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2â€“27 were removed before selection. Wave 27 stopped at
`0x806D0820`; the next address, `0x806D09B0`, is still the rejected
paired-single body, so the sweep restarts at `0x806D12F0`.

1. **Take the `remo_lift` actor as a lifecycle.** `0x806D15A0` constructs the
   `0x560`-byte object (vtable `0x807FAA14`, interface `0x807FAAF8`);
   `0x806D1570` is the heap wrapper; `0x806D1B10` loads
   `g3d/remo_lift.brres`; `0x806D1A90`/`0x806D2F80` update the transform and
   model; `0x806D1D30` tracks the rider count and the animation state.
2. **Take the `remo_lift_line` actor.** `0x806D2870` constructs the
   `0x650`-byte object (vtable `0x807FAC38`, interface `0x807FAD20`) and
   `0x806D31B0` loads `g3d/remo_lift_line.brres`; `0x806D3540` repeats the
   rider-count logic with the line actor's `+0x754`/`+0x760` fields.
3. **Take the `lift_suisha` block loader.** `0x806E2320` loads the main
   `lift_suisha` pane, then `0x806E23E0` walks four `0xE0`-byte `ashiba`
   sub-blocks and calls `0x806E3480` for each one.
4. **Take the single-pane lift loaders.** These are self-contained BRRES
   loaders with explicit resource/pane strings: `lift_number_ship`,
   `lift_tenbin`, `lift_kinoko_yoko`, `lift_zen_hammer`, `lift_zen_kaihei`,
   `lift_zen_shiro`, `lift_zen_shiro_yogan`, `lift_zen_suisya`, `block_light`
   and `cube_kinoko`.
5. **Rejected or deferred decompiler failures.** `0x806D1ED0` and
   `0x806D21A0` decode to `halt_baddata`; `0x806D1EF0` and `0x806D2390` call
   them. `0x806D29C0` and `0x806D09B0` are paired-single reconstructions
   (`unaff_GQR0`, `ldexpf`). `0x806D3370` reads `extraout_f1` after
   `0x800EB930` and needs an ABI review before it can be called complete.

## Clusters

### `remo_lift` (`0x806D12F0..0x806D2330`)

- `0x806D1570` allocates `0x560` bytes; `0x806D15A0` installs vtable
  `0x807FAA14`, the interface at `+0x454`, the self pointer `+0x458`, the
  accessor `+0x460`, the model block `+0x480`, the clip block `+0x4C0` and the
  animation block `+0x508`.
- `0x806D1B10` opens `g3d/remo_lift.brres` (`0x807FA9D8`), resolves the
  `remo_lift` pane (`0x807FA9EC`), adds `change_mark` (`0x807FA9F8`), then
  binds the `remo_lift` animation at `+0x4FC`.
- `0x806D1D30` derives the rider count from the player object at `+0x14D4`,
  switches `+0x538` between the idle and release states, and spawns
  `Wm_en_burst_s` through `0x806D1CD0`.
- `0x806D21D0` clears the release timer `+0x52C` and posts sound `0x2A7` after
  `0x800B3750(riders, 0, 0x14)`.

### `remo_lift_line` (`0x806D2870..0x806D3540`)

- `0x806D2870` builds the `0x650`-byte object with vtable `0x807FAC38`, the
  interface `0x807FAD20`, the model/clip blocks at `+0x480`/`+0x4C0`, the
  animation `+0x508`, the spark-loop block `+0x528` and the motion block
  `+0x65C`.
- `0x806D31B0` opens `g3d/remo_lift_line.brres` (`0x807FABDC`), resolves the
  `remo_lift_line` pane (`0x807FABF8`), adds `change_mark` (`0x807FAC08`) and
  binds the animation at `+0x4FC`.
- `0x806D2F80` samples the model transform with `0x80168180` and caches the
  translation at `+0x650..+0x658`.

### `lift_suisha` (`0x806E2320..0x806E3480`)

- `0x806E2320` loads `g3d/lift_suisha.brres` and the `_4line` pane, then calls
  `0x806E23E0`.
- `0x806E23E0` is a `do`/`while` over `+0x430`, `+0x510`, `+0x5F0` and
  `+0x6D0` (`+0x7B0` limit, `0xE0` stride). Each block calls `0x806E3480`.
- `0x806E3480` treats its argument as the block base: accessor at `+0x00`,
  resource at `+0x1C`, model at `+0x20`, pane
  `s_lift_suisha_ashiba_807fcc04`.

### Single-pane loaders (`0x806D5680..0x806E9E90`)

All ten share the same shape: `0x800690C0` constructs the resource accessor,
`0x800DF180` opens the BRRES, `0x80239E30` resolves the pane, `0x8016A3A0`
or `0x80169D90` binds it, `0x80064BF0` finalises the part and `0x800690E0`
releases the accessor. `0x806D5680` additionally binds a `ridden` mark, a
number animation and a warning animation; `0x806E4DA0` and `0x806E5DF0` bind
clips.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave28-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806D12F0` | `fn_806d12f0` | release a `remo_lift` and optionally free it | translated | high | `806d12f0_FUN_806d12f0.c` |
| `0x806D1470` | `fn_806d1470` | allocate the unnamed `0x3D0` helper | translated | medium | `806d1470_FUN_806d1470.c` |
| `0x806D1570` | `fn_806d1570` | allocate the `0x560` `remo_lift` actor | translated | high | `806d1570_FUN_806d1570.c` |
| `0x806D15A0` | `fn_806d15a0` | construct the `0x560` `remo_lift` actor | translated | high | `806d15a0_FUN_806d15a0.c` |
| `0x806D18C0` | `fn_806d18c0` | reset `+0x3D0` and return 1 | translated | high | `806d18c0_FUN_806d18c0.c` |
| `0x806D1A90` | `fn_806d1a90` | rebuild the lift transform and model | translated | high | `806d1a90_FUN_806d1a90.c` |
| `0x806D1B10` | `fn_806d1b10` | load `remo_lift` and bind pane/mark/animation | translated | high | `806d1b10_FUN_806d1b10.c` |
| `0x806D1CD0` | `fn_806d1cd0` | spawn `Wm_en_burst_s` at the lift | translated | high | `806d1cd0_FUN_806d1cd0.c` |
| `0x806D1D30` | `fn_806d1d30` | track rider count and animation | translated | medium | `806d1d30_FUN_806d1d30.c` |
| `0x806D1E70` | `fn_806d1e70` | clamp the ground-probe angle | translated | medium | `806d1e70_FUN_806d1e70.c` |
| `0x806D21D0` | `fn_806d21d0` | clear the release timer and post sound `0x2A7` | translated | high | `806d21d0_FUN_806d21d0.c` |
| `0x806D2330` | `fn_806d2330` | reset the rider-selection state | translated | high | `806d2330_FUN_806d2330.c` |
| `0x806D2870` | `fn_806d2870` | construct the `0x650` `remo_lift_line` actor | translated | high | `806d2870_FUN_806d2870.c` |
| `0x806D2F80` | `fn_806d2f80` | rebuild the line transform and cache translation | translated | high | `806d2f80_FUN_806d2f80.c` |
| `0x806D31B0` | `fn_806d31b0` | load `remo_lift_line` and bind its parts | translated | high | `806d31b0_FUN_806d31b0.c` |
| `0x806D34B0` | `fn_806d34b0` | spawn `Wm_en_burst_s` for the line lift | translated | high | `806d34b0_FUN_806d34b0.c` |
| `0x806D3540` | `fn_806d3540` | track line-lift rider count and animation | translated | medium | `806d3540_FUN_806d3540.c` |
| `0x806D5680` | `fn_806d5680` | load `lift_number_ship` and its three parts | translated | high | `806d5680_FUN_806d5680.c` |
| `0x806D96A0` | `fn_806d96a0` | load `lift_tenbin` and four panes | translated | high | `806d96a0_FUN_806d96a0.c` |
| `0x806DD650` | `fn_806dd650` | load `lift_kinoko_yoko` and two stalks | translated | high | `806dd650_FUN_806dd650.c` |
| `0x806E2320` | `fn_806e2320` | load `lift_suisha` and start the block loop | translated | high | `806e2320_FUN_806e2320.c` |
| `0x806E23E0` | `fn_806e23e0` | build the four `0xE0`-byte `ashiba` blocks | translated | high | `806e23e0_FUN_806e23e0.c` |
| `0x806E3480` | `fn_806e3480` | load one `lift_suisha` `ashiba` block | translated | high | `806e3480_FUN_806e3480.c` |
| `0x806E4540` | `fn_806e4540` | load `lift_zen_hammer` | translated | high | `806e4540_FUN_806e4540.c` |
| `0x806E4DA0` | `fn_806e4da0` | load `lift_zen_kaihei` model and clip | translated | high | `806e4da0_FUN_806e4da0.c` |
| `0x806E5DF0` | `fn_806e5df0` | load `lift_zen_shiro` with optional animation | translated | medium | `806e5df0_FUN_806e5df0.c` |
| `0x806E6F00` | `fn_806e6f00` | load `lift_zen_shiro_yogan` | translated | high | `806e6f00_FUN_806e6f00.c` |
| `0x806E7900` | `fn_806e7900` | load `lift_zen_suisya` | translated | high | `806e7900_FUN_806e7900.c` |
| `0x806E8140` | `fn_806e8140` | load `block_light` | translated | high | `806e8140_FUN_806e8140.c` |
| `0x806E9E90` | `fn_806e9e90` | load `cube_kinoko` | translated | high | `806e9e90_FUN_806e9e90.c` |

**Totals:** 30 translated, 0 partial, 0 stub (25 high, 5 medium).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x806D1410`, `0x806D1440` | 48 each | `0x802DCBFC` thunks with no independent behaviour. |
| `0x806D1670` | 64 | Release helper with the same shape as [`fn_806d12f0`] but no object teardown; left for a batch of release helpers. |
| `0x806D1ED0`, `0x806D21A0` | 28, 36 | **Rejected**: `halt_baddata` in the export. |
| `0x806D1EF0` | 684 | Calls the truncated `0x806D1ED0` and reads the `0x802DCD70` register-save thunk's return; needs an ABI review. |
| `0x806D2390` | 364 | Calls the truncated `0x806D1ED0`/`0x806D21A0`. |
| `0x806D16B0`, `0x806D29C0` | 332, 724 | **Rejected**: paired-single reconstruction (`unaff_GQR0`, `ldexpf`). |
| `0x806D3370` | 312 | Reads `extraout_f1` after `0x800EB930`; the float return ABI is unresolved. |
| `0x806D09B0` | 1,508 | **Rejected**: paired-single body with unreachable blocks. |
| `0x806D3680`, `0x806D38D0`, `0x806D3AC0`, `0x806D3C20` | 328, 36, 120, 784 | Paired-single or truncated bodies in the same line-lift neighbourhood. |

## Unknowns preserved

1. **`0x8005F900`.** The C drops the slot argument in [`fn_806d1d30`] and
   [`fn_806d3540`]. The Rust passes the actor's `+0x430` rider byte, matching
   the convention already used by `0x806CF700` and `0x806517C0`.
2. **`0x800DF180`.** The identity of the BRRES loader is not in this export;
   every loader keeps an `// unknown:` note.
3. **`0x800690C0` / `0x800690E0`.** The accessor constructor and release pair
   is explicit in shape but has no recovered class name.
4. **`0x80168180`.** [`fn_806d2f80`] passes a `0x30`-byte stack block and reads
   the words at `+0x0C`, `+0x1C` and `+0x2C`; the C's `local_34`/`local_24`/
   `local_14` labels are reconstructed as a 12-word matrix.
5. **Overlapping stack records.** [`fn_806d1b10`], [`fn_806d31b0`] and
   [`fn_806d5680`] use adjacent stack words for `0x801650D0`/`0x80166830`;
   the Rust reconstructs the visible pairs and records the reconstruction in
   comments.
6. **Class identity.** `remo_lift` and `remo_lift_line` are the BRRES/pane
   names; no class string appears in this window. The `0x3D0` helper allocated
   by [`fn_806d1470`] remains unnamed.
7. **`0x800B3980`.** [`fn_806d1e70`] uses the two REL floats as single
   precision; the C prints `(double)` because the callee lacks a prototype.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` has 699 functions after this pass (697 `translated`,
2 pre-existing `partial`); this pass adds 30, all with `# Original` blocks and
the synthetic-base warning.
`node tools/doc-lint.mjs src` reports `ok: true`, 0 violations over 68 files.
`cargo check` exits 0.
