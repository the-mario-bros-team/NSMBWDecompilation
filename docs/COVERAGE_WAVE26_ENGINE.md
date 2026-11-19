# Coverage — wave 26 engine display/frame (continue `0x800C0000..0x80100000`)

## How to read this file

This is the completion record for the wave-26 continuation of the engine
display/frame lane. It answers, for a reader with no context:

1. **What was added?** Twenty-five function bodies: the two remaining pause
   cursor value helpers, the `Wm_mr_water*` actor that owns the wave-20 effect
   pool, three save-banner/save-image helpers, and eight scene-transition /
   vertex-array helpers.
2. **Where is it?** Seventeen bodies are in
   [`src/engine/display.rs`](../src/engine/display.rs) and eight are in
   [`src/engine/frame.rs`](../src/engine/frame.rs). Nothing outside those two
   files was edited except the two allowed records (`docs/COVERAGE_WAVE26_ENGINE.md`
   and `ledger/agents/wave26-engine.csv`) plus the shared `ledger/functions.csv`.
3. **How sure is it?** `confidence` measures the evidence, not the code style:
   `high` = every branch, store and call operand is explicit in the Ghidra C;
   `medium` = the flow is complete but a `this` pointer, a callee operand or a
   class role had to be recovered from the call site.
4. **What is still unknown?** See **Uncertainties** and **Next targets**.

Addresses are **plain lowercase hex** virtual image addresses (image base
`0x80004000`), not file offsets. Evidence is the per-function Ghidra C file
under `work/ghidra/main-gb/export/`. Every Rust function keeps an `# Original`
block with the same address, size and evidence path.

Status follows `docs/RUST_STYLE.md`: `translated` = the full control flow and
every call are represented; `partial` = control flow is represented but one
operand or callee body is missing; `stub` = address/role only. Markers inside
the Rust code: `// observed:`, `// inferred:`, `// unknown:`.

Terms: **DOL** is the Wii executable container; **r13** is the PowerPC
small-data base (`0x8042F680`); **r2** is the second small-data base
(`0x80433080`); **SDA/SDA2** are those small-data areas; **NW4R / lyt** is
NintendoWare's layout library. See `docs/GLOSSARY.md`.

## Scope and result

| Item | Count |
|---|---:|
| New function bodies in this pass | **25** |
| Bytes represented | **3,224** |
| `engine::display` functions / bytes | 17 / 2,804 |
| `engine::frame` functions / bytes | 8 / 420 |
| Status `translated` | 25 |
| Status `partial` | 0 |
| Confidence `high` / `medium` | 20 / 5 |
| Invented symbol names | 0 |

Running totals for the two destination files after this pass:

| File | Functions | Bytes | New ranges covered |
|---|---:|---:|---|
| `src/engine/display.rs` | 86 | 15,416 | `800d2270`, `800d2510`, `800d2ac0..800d4660`, `800e0070..800e0450` |
| `src/engine/frame.rs` | 15 | 1,228 | `800e22f0`, `800e23c0`, `800e2c70..800e2d90` |

### Selection

Both destinations were checked before selecting, then every candidate address
was checked against `ledger/functions.csv` and against the Rust files (none of
the 25 had a body anywhere under `src/`). The selection follows the wave-20
next-target list and the remaining clusters of the brief:

* Entries 1, 2, 3, 4 and 6 of the wave-20 next-target list
  (`800d2ac0`, `800d2b90`, `800d2dd0`, `800d3280`/`800d32f0`, `800e0290`) plus
  entry 5 (`800e0070`) are now translated.
* The water actor needs its release/update/model helpers to be readable as one
  object, so `800d2ff0`, `800d30e0`, `800d3ad0`, `800d3b20`, `800d3bf0`,
  `800d3cb0` and `800d4660` are included.
* `800d2270` and `800d2510` close the cursor-value pair immediately after the
  wave-20 cursor.
* `800e0450` completes the save-manager test used by the save-banner scenes.
* The frame destination gets the scene-transition position wrappers and the
  five readable helpers of the 0x0C-stride vertex array whose builder
  (`800e2ad0`) was translated in wave 20.

**Deliberately not selected:** `800e1f80` / `800e20a0` / `800e24b0` print
`extraout_r4` or save-register results, so their parameter split needs an
instruction check first. `800e2de0`, `800e2f10`, `800e3040` and `800e3160` are
the GQR/`ldexpf` group the wave-20 doc already flags as unsafe to translate
without instruction words. `800d3ce0` / `800d3ef0` and `800d3c80` contain
unrecovered jump tables.

## Translated functions — cursor value helpers (`800d2270`, `800d2510`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `800d2270` | `cursor_target_value` | Interpolates the value of the target under the cursor. | translated | medium | `800d2270_FUN_800d2270.c` |
| `800d2510` | `cursor_position_sum` | Sums the cursor's float table into `+0x14`. | translated | high | `800d2510_FUN_800d2510.c` |

`cursor_target_value` has three shapes: mode `1..3` uses the float table at
target `+0x2C` and the signed halfwords at `+0x30`; mode `0` calls the target's
vtable slot `+0x0C`; no target returns `0`. The backwards walk skips zero
entries from the cursor's `+0x2E` index and ends at the first halfword.

## Translated functions — `Wm_mr_water*` actor (`800d2ac0..800d4660`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `800d2ac0` | `water_effect_spawn` | Starts one water effect in a free pool element. | translated | high | `800d2ac0_FUN_800d2ac0.c` |
| `800d2b90` | `water_effect_update` | Advances one active water effect. | translated | medium | `800d2b90_FUN_800d2b90.c` |
| `800d2dd0` | `water_actor_construct` | Builds the actor base and every embedded sub-object. | translated | medium | `800d2dd0_FUN_800d2dd0.c` |
| `800d2ff0` | `fn_800d2ff0` | Frees an optional object and returns it. | translated | high | `800d2ff0_FUN_800d2ff0.c` |
| `800d30e0` | `water_actor_release` | Releases the actor and optionally its storage. | translated | high | `800d30e0_FUN_800d30e0.c` |
| `800d3280` | `water_actor_update` | Binds the four nodes and refreshes the model position. | translated | high | `800d3280_FUN_800d3280.c` |
| `800d32f0` | `water_actor_load_models` | Loads the three `g3d_model.brres` models. | translated | high | `800d32f0_FUN_800d32f0.c` |
| `800d3ad0` | `water_actor_update_model_position` | Resolves and applies the shared position handle. | translated | high | `800d3ad0_FUN_800d3ad0.c` |
| `800d3b20` | `water_actor_apply_animation` | Applies one animation to the current model node. | translated | medium | `800d3b20_FUN_800d3b20.c` |
| `800d3bf0` | `water_actor_set_state` | Changes the actor state from the 0x10-byte table. | translated | high | `800d3bf0_FUN_800d3bf0.c` |
| `800d3cb0` | `water_actor_model_node` | Selects `+0x6C0`, `+0x6EC` or `+0x718` by mode. | translated | high | `800d3cb0_FUN_800d3cb0.c` |
| `800d4660` | `water_actor_set_mode` | Saves the old mode and stores the new one. | translated | high | `800d4660_FUN_800d4660.c` |

The object layout is proven by the constructor and the destructor together:

```text
+0x000  vtable 0x80317D30
+0x20C  three model handles
+0x218  0x128-byte embedded sub-object (vtable 0x80329958)
+0x340  0x128-byte embedded sub-object (vtable 0x80329958)
+0x468  ten 0x80-byte nodes (construct 0x800D3030 / release 0x800D3070)
+0x668  animation handle 1        +0x694  animation handle 2
+0x6C0  animation node (mode 0)   +0x6EC  animation node (mode 2)
+0x718  animation node (mode 3)   +0x744  shared animation node
+0x770  current mode              +0x774  previous mode
+0x794  adapter table 0x80317E08  +0x79C  adapter table 0x80317DF0
```

`water_effect_spawn` uses kind `4` for the `"Wm_mr_watersplash"` name at
`0x80317AEC` and the other kinds for the four `"Wm_mr_waterwave_out"` pointers
at `0x80317ADC`. `water_effect_update` subtracts the SDA2 constant at
`0x8042CA28`, looks the effect up through `0x80075270`, then asks vtable
`+0xB0` whether the effect finished.

## Translated functions — save banner / save image (`800e0070..800e0450`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `800e0070` | `save_banner_set_slot_visible` | Sets or clears one slot's visibility bit. | translated | medium | `800e0070_FUN_800e0070.c` |
| `800e0290` | `save_record_release` | Releases one 0x980-byte save record. | translated | high | `800e0290_FUN_800e0290.c` |
| `800e0450` | `save_manager_busy` | Returns `1` when manager `+0x74` is non-zero. | translated | high | `800e0450_FUN_800e0450.c` |

`save_banner_set_slot_visible` resolves the slot manager through vtable slot
`+8` and descriptor `0x802F8FF0`, builds the 12-byte key with `0x8025C960`,
performs the same lookup twice (as the export does), and sets bit `0x100` in
the record's `+0x14` flags. `save_manager_busy` keeps the original
`(-value | value) >> 31` test instead of collapsing it to `value != 0`.

## Translated functions — scene transition and vertex arrays (`800e22f0..800e2d90`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `800e22f0` | `scene_transition_position_a` | Adds the object position and calls `0x800E2220`. | translated | high | `800e22f0_FUN_800e22f0.c` |
| `800e23c0` | `scene_transition_position_b` | The same for `0x800E2340`. | translated | high | `800e23c0_FUN_800e23c0.c` |
| `800e2c70` | `vertex_finalize_indices` | Fills the second array and copies the `+8` word. | translated | high | `800e2c70_FUN_800e2c70.c` |
| `800e2d00` | `vertex_set` | Stores one vertex triple. | translated | high | `800e2d00_FUN_800e2d00.c` |
| `800e2d30` | `vertex_count` | Returns `count * byte_6 + 1` or `count * byte_6`. | translated | high | `800e2d30_FUN_800e2d30.c` |
| `800e2d60` | `vertex_at` | Returns one vertex address. | translated | high | `800e2d60_FUN_800e2d60.c` |
| `800e2d70` | `curve_weight_neg_square` | Evaluates `(c - v)^2 * scale`. | translated | high | `800e2d70_FUN_800e2d70.c` |
| `800e2d90` | `curve_weight_product` | Evaluates `(c - v) * v + scale`. | translated | high | `800e2d90_FUN_800e2d90.c` |

The transition wrappers read the object position from `+0xAC` / `+0xB0`, add
the two incoming doubles, and store the third component from the SDA2 word at
`0x8042CC90`. The vertex helpers are the readable remainder of the class whose
builder is `engine::frame::fn_800e2ad0`: `0x800E2DE0` performs the heavy fill
and is called by address; the five helpers only index, count, copy or evaluate
the two curve weights.

Sizes: 288 + 252 = **540** (cursor); 208 + 144 + 472 + 64 + 292 + 112 + 132 +
80 + 208 + 140 + 44 + 16 = **1,912** (water actor); 264 + 64 + 24 = **352**
(save banner); 68 + 68 + 132 + 36 + 48 + 16 + 24 + 28 = **420** (frame).
Total 540 + 1,912 + 352 + 420 = **3,224**.

## Evidence quality notes

* The `802DCD74` / `802DCDC0` and `802DCD70` pairs are CodeWarrior register
  save/restore helpers, not game logic. They are omitted, following
  `docs/COVERAGE_WAVE16_ENGINE.md`.
* `800d2dd0` loses its `this` pointer to `FUN_802dcd74`. The Rust takes the
  actor as `actor` and the base-constructor byte as `base_mode`; the call sites
  (`800bcb80`, `800ca060`, `800fe1d0`) confirm that `FUN_800d5330` is a base
  constructor called on the same object.
* `800d3b20` prints the first `FUN_800d3cb0` call without an operand. The Rust
  passes `actor`, which is the only object in scope and the receiver of the
  following vtable call.
* `800d3ad0` and `800d3bf0` use the same `800d3b20` shape, so the three bodies
  read as one animation path.
* `800d32f0` passes `s__Pg3d_model_brres_80317b05 + 3`, i.e. the address
  `0x80317B08`; the string resolves to `"g3d_model.brres"` (the other three
  model loaders in the export pass a full `g3d_*.brres` name). The `+3` is
  preserved in the constant's doc.
* `800e2c70` calls `0x800E2DE0` by address. That callee is one of the GQR
  bodies the wave-20 doc warns about; no body was invented for it.
* Every `# Original` block cites the Ghidra name, the plain address, the byte
  size and the export file.

## Uncertainties

1. **`800d2dd0` hidden `this` / `base_mode`.** The save-register helper hides
   both the object pointer and the byte forwarded to `0x800D5330`. The Rust
   makes them explicit parameters. Which character/mode values callers pass is
   not established.
2. **`800d2b90` float ABI.** The export prints `(double)` arguments for
   `0x80075270`; the Rust models them as `f64`. If the target uses `f32`
   registers, the numeric values are the same but the ABI interpretation
   differs.
3. **`800d2270` cursor tables.** The float table at target `+0x2C` and the
   signed halfword table at `+0x30` are proven by indexing and arithmetic; what
   the values mean (distance, frame index, lane position) is unknown.
4. **`800d32f0` archive global.** `0x8042A038` is eight bytes before the save
   image pointer. Its `+4` word is passed as the archive to `0x800DF180`; the
   owner's name is not in the export.
5. **`800d3bf0` state table.** `0x802F1250` entries are 0x10 bytes with a name
   pointer, a word and two floats. The table length is not printed, and the
   export does not bound `state`.
6. **`800d3b20` receiver.** The first `0x800D3CB0` call prints no argument; the
   Rust passes `actor` because `r3` still holds it. The vtable receiver
   (`actor + mode * 0x80 + 0x4A8`) and the handle argument (`+0x6C0` family)
   are different objects, and both are preserved.
7. **`800e0070` 12-byte key.** `0x8025C960` fills `auStack_1c`; its contents are
   not used by the decompiled body, so the Rust keeps an opaque byte array.
   The duplicate lookup is literal.
8. **`800e0450` ignored argument.** Banner callers pass the save-image pointer
   as `r3`, but the decompiled body is parameterless and reads the manager from
   `0x80429FB8`. The Rust keeps the parameterless signature.
9. **Transition vector third component.** `0x8042CC90` is read as a word and
   stored unchanged; its numeric value is not decoded.
10. **`800d2ff0` class.** No caller appears in the export, so the function keeps
    its address name even though its guard/release shape matches the two
    destructors.
11. **Semantic `f64` helpers.** The host `extern "C"` helpers preserve argument
    order and arithmetic, not PowerPC register placement. They exist only for
    bodies whose evidence prints `double` parameters.

## Next targets

Ordered by value per effort, with the evidence file to read first.

| Priority | Target | Size | Why | First evidence |
|---:|---|---:|---|---|
| 1 | `800d0470` | 360 B | effect-record spawn; `extraout_r4` must be checked first | `800d0470_FUN_800d0470.c` |
| 2 | `800e1f80`, `800e20a0` | 280 + 380 B | scene-transition position helpers; `extraout_r4` must be checked first | `800e1f80_FUN_800e1f80.c` |
| 3 | `800d0610` | 656 B | 4-view 3-D setup; paired-single/GQR artefacts | `800d0610_FUN_800d0610.c` |
| 4 | `800d19e0` | 976 B | cursor 3-D setup; paired-single/GQR artefacts | `800d19e0_FUN_800d19e0.c` |
| 5 | `800d3380` | 860 B | water actor model/material setup; GQR artefacts | `800d3380_FUN_800d3380.c` |
| 6 | `800d36e0`, `800d38a0` | 328 + 388 B | water actor transform and per-frame model update | `800d36e0_FUN_800d36e0.c` |
| 7 | `800d3ce0`, `800d3ef0` | 520 + 356 B | large animation-name switches; jump tables need recovery | `800d3ce0_FUN_800d3ce0.c` |
| 8 | `800e24b0` | 152 B | transition wrapper around `800e1f80`; save-register result needs checking | `800e24b0_FUN_800e24b0.c` |
| 9 | `800e0540` | 1,756 B | large save/engine body; read before selecting | `800e0540_FUN_800e0540.c` |
| 10 | `800e2de0`, `800e2f10`, `800e3040`, `800e3160` | 304 + 296 + 276 + 276 B | vertex-fill group; GQR/`ldexpf` artefacts | `800e2de0_FUN_800e2de0.c` |

## Ledger

`ledger/agents/wave26-engine.csv` has one row per function above with the
seven-column header from `docs/RUST_STYLE.md`
(`module,address,name,status,confidence,evidence,rust_file`). The `module`
column is `engine::display` for the 17 `display.rs` rows and `engine::frame`
for the 8 `frame.rs` rows, matching each `rust_file` path. The same 25 rows were
appended to `ledger/functions.csv`. No row was added before its body existed.
