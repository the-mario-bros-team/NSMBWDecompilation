# Coverage — wave 20 engine display/frame (continue `0x800C0000..0x80100000`)

## How to read this file

This is the completion record for the wave-20 continuation of the engine
display/frame lane. It answers, for a reader with no context:

1. **What was added?** Twenty-five function bodies: the pause cursor/target
   class, the ten-slot water-effect pool, the save-image object built at engine
   start-up, one display-object helper and three frame-support bodies.
2. **Where is it?** Twenty-two bodies are in
   [`src/engine/display.rs`](../src/engine/display.rs) and three are in
   [`src/engine/frame.rs`](../src/engine/frame.rs). Nothing outside those two
   files was edited.
3. **How sure is it?** `confidence` measures the evidence, not the code style:
   `high` = every branch, store and call operand is explicit in the Ghidra C;
   `medium` = the flow is complete but a callee operand, a constant's meaning or
   a class name had to be recovered from the call site.
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
| Bytes represented | **2,524** |
| `engine::display` functions / bytes | 22 / 2,100 |
| `engine::frame` functions / bytes | 3 / 424 |
| Status `translated` | 25 |
| Status `partial` | 0 |
| Confidence `high` / `medium` | 20 / 5 |
| Invented symbol names | 0 |

Running totals for the two destination files after this pass:

| File | Functions | Bytes | Ranges covered |
|---|---:|---:|---|
| `src/engine/display.rs` | 69 | 12,612 | `800c9b00`, `800d0940..800d3070`, `800e0180..800e04b0`, `801568d0..8015b840` |
| `src/engine/frame.rs` | 7 | 808 | `800c9a10`, `800d08a0`, `800e2ad0`, `800e4e40` plus three getters |

### Selection

The lane brief names the engine band `0x800C0000..0x80100000` and the themes
"display/pause/save-banner, HUD scene helpers, frame dispatch, scene transition,
dSys helpers". Both destinations were checked before selecting, then every
candidate address was checked against `ledger/functions.csv`:

* The pause-menu controller (`800d0940..800d11f0`) was completed in wave 16.
  The cursor/target class that consumes its selection follows at
  `800d14e0..800d2200` and was explicitly listed as next target 10 in
  `docs/COVERAGE_WAVE16_ENGINE.md`.
* `800d29f0..800d3070` is the water-effect pool. Its element stride `0x13C`
  matches the fields written by the `Wm_mr_watersplash` spawn at `800d2ac0`
  (next target list, entry 3-8 neighbourhood), so the pool is displayed in
  `engine::display` next to the other `Wm_2d_*` effects.
* `800e0180..800e04b0` builds the 0x3FC0-byte save image whose base
  (`0x8035DCC0`) is already read by `src/game/scene.rs`. These are start-up
  object constructors, not the boot scene's DVD state machine.
* `800c9b00` writes the display object's `+0x3DA` byte and belongs beside the
  other `G_DISPLAY_OBJECT` users.
* `800d08a0`, `800c9a10` and `800e2ad0` are the frame/render support bodies left
  in the band: the thunk the frame itself calls at step 27, the static heap
  registration that owns the `dsys` heap object, and the two-array vertex
  builder used by the scene draw code.

**Already translated elsewhere, therefore excluded:** `800e5060`, `800e5110`,
`800e51a0`, `800e5200`, `800e52d0`, `800e5400` and `800e5480` already have
bodies in [`src/engine/dsys.rs`](../src/engine/dsys.rs) (function names
`fn_800e5060` ... `fn_800e5480`), even though those addresses had no
`ledger/functions.csv` row. They were **not** re-translated and no ledger row is
claimed for them here; the ledger gap is recorded in **Uncertainties**.

## Translated functions — pause cursor (`800d14e0..800d2200`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `800d14e0` | `cursor_selection_active` | Reads the cursor's selection-valid byte, or `0`. | translated | high | `800d14e0_FUN_800d14e0.c` |
| `800d1500` | `cursor_set_target` | Stores a target pointer and its follow flag. | translated | high | `800d1500_FUN_800d1500.c` |
| `800d1510` | `cursor_acquire_target` | Latches the first accepted list target and clamps its position. | translated | medium | `800d1510_FUN_800d1510.c` |
| `800d1640` | `cursor_tick_lock` | Ages the lock timer and the target countdown. | translated | high | `800d1640_FUN_800d1640.c` |
| `800d1f10` | `cursor_begin_track` | Starts the tracking animation and stores its phase byte. | translated | medium | `800d1f10_FUN_800d1f10.c` |
| `800d2200` | `cursor_unlink` | Removes the cursor from either list shape. | translated | high | `800d2200_FUN_800d2200.c` |

## Translated functions — water-effect pool (`800d29f0..800d3070`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `800d29f0` | `water_effect_construct` | Constructs one 0x13C-byte water effect. | translated | high | `800d29f0_FUN_800d29f0.c` |
| `800d2a50` | `water_effect_release` | Releases one water effect. | translated | medium | `800d2a50_FUN_800d2a50.c` |
| `800d2c20` | `water_effect_pool_construct` | Builds the ten-slot pool and publishes it. | translated | high | `800d2c20_FUN_800d2c20.c` |
| `800d2c80` | `water_effect_pool_release` | Destroys the ten-slot pool. | translated | high | `800d2c80_FUN_800d2c80.c` |
| `800d2d80` | `water_effect_pool_update` | Ticks all ten elements. | translated | high | `800d2d80_FUN_800d2d80.c` |
| `800d3030` | `water_node_construct` | Constructs one 0x80-byte grid node. | translated | medium | `800d3030_FUN_800d3030.c` |
| `800d3070` | `water_node_release` | Releases one grid node. | translated | high | `800d3070_FUN_800d3070.c` |

## Translated functions — save image (`800e0180..800e04b0`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `800e0180` | `save_image_create` | Allocates 0x3FC0 bytes and constructs the save image. | translated | high | `800e0180_FUN_800e0180.c` |
| `800e01d0` | `save_image_construct` | Builds the header and the two three-record arrays. | translated | high | `800e01d0_FUN_800e01d0.c` |
| `800e02d0` | `save_image_refresh` | Copies the on-disc image into the live object. | translated | high | `800e02d0_FUN_800e02d0.c` |
| `800e0380` | `save_image_record_a` | Selects one record of array A. | translated | high | `800e0380_FUN_800e0380.c` |
| `800e03b0` | `save_image_record_b` | Selects one record of array B. | translated | high | `800e03b0_FUN_800e03b0.c` |
| `800e0430` | `save_image_flag_get` | Reads (and maybe clears) the dirty byte. | translated | high | `800e0430_FUN_800e0430.c` |
| `800e0470` | `save_image_update_header_checksum` | Hashes the header into `+0x6BC`. | translated | high | `800e0470_FUN_800e0470.c` |
| `800e04b0` | `save_image_update_checksums` | Hashes the header and all six records. | translated | high | `800e04b0_FUN_800e04b0.c` |

## Translated functions — display helper (`800c9b00`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `800c9b00` | `display_object_mark_dirty` | Sets `display + 0x3DA` when a named resource resolves. | translated | medium | `800c9b00_FUN_800c9b00.c` |

## Translated functions — frame support (`800c9a10`, `800d08a0`, `800e2ad0`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `800c9a10` | `fn_800c9a10` | Constructs and registers the static heap object. | translated | high | `800c9a10_FUN_800c9a10.c` |
| `800d08a0` | `fn_800d08a0` | Renderer vtable slot `+0x0C` (the frame's step 27). | translated | high | `800d08a0_thunk_FUN_8016f410.c` |
| `800e2ad0` | `fn_800e2ad0` | Builds two 0x0C-stride vertex arrays. | translated | high | `800e2ad0_FUN_800e2ad0.c` |

Sizes: 28 + 12 + 300 + 104 + 108 + 96 = **648** (cursor);
96 + 100 + 92 + 104 + 76 + 56 + 104 = **628** (water pool); 76 + 192 + 176 +
40 + 40 + 32 + 56 + 132 = **744** (save image); 80 (display helper); 4 + 72 +
348 = **424** (frame support). Total 648 + 628 + 744 + 80 + 424 = **2,524**.

## What each cluster does

### Pause cursor

`cursor_acquire_target` is the entry point. It only runs while `cursor + 0x0C`
is empty, walks the list at `0x80429FE8`, asks `0x800D16B0(cursor, node, arg)`
whether the node is acceptable, latches both `+0x0C` and `+0x10`, and copies
the owner's position into `+0x14`/`+0x18`. Four SDA2 floats clamp that position:

| Constant | Absolute | Use |
|---|---|---|
| `r2 - 0x6698` | `0x8042C9E8` | vertical base offset and low clamp |
| `r2 - 0x6694` | `0x8042C9EC` | vertical scale |
| `r2 - 0x6690` | `0x8042C9F0` | horizontal low clamp |
| `r2 - 0x668C` | `0x8042C9F4` | horizontal high clamp |
| `r2 - 0x6688` | `0x8042C9F8` | vertical high clamp |

`cursor_tick_lock` decrements the `+0x2D` lock timer, drops `+0x10` when it
expires, then ages the `+0x28` countdown and reloads it with `2` when the
`+0x2E`/`+0x2F` bytes differ. `cursor_unlink` implements both list shapes seen
in the original: the flat head uses `+0x10`, the nested node uses `+0x08`.

### Water-effect pool

The pool is 4 bytes of vtable plus ten elements of `0x13C` bytes
(`4 + 10 * 0x13C`). `water_effect_construct` writes vtable `0x80317AFC`, builds
the base object at `+4` with `0x802D7AA0`, replaces its vtable with `0x80308A80`
and clears `+0x138`, which the spawn at `0x800D2AC0` tests as "free". The pool
constructor/destructor register the pair through `0x802DC9A0` / `0x802DCA98`
and publish the pool at `0x80429FF0`. `water_effect_pool_update` calls the
per-element update `0x800D2B90` ten times; that body is float heavy and is
called by address only.

`800d3030`/`800d3070` are the 0x80-byte node pair registered by the water actor
constructor at `800d2dd0` (not translated): two 0x40-byte sub-objects through
`0x80169CD0` / `0x80169D20`.

### Save image

The object is 0x3FC0 bytes:

```text
+0x000  byte      constructed flag (cleared by save_image_construct)
+0x020  0x6A0     header; data at +0x024 for 0x698, checksum word at +0x6BC
+0x6C0  3 x 0x980 record array A (payload + tail checksum at +0x97C)
+0x2340 3 x 0x980 record array B
```

`save_image_construct` clears the flag, initialises the header through
`0x800CE570`/`0x800CE580`, builds both arrays with `0x802DC9A0` (element
constructor `0x800CDEC0`, destructor `0x800E0290`), publishes the object at
`0x8042A040`, and finalises each record twice with `0x800CDED0`.
`save_image_refresh` copies the whole live image from `0x8035DCC0` (header,
then three records of each array) and retags every record with
`0x800CE020`/`0x800CE6C0` (`0x0E`).

`save_image_record_a`/`_b` are the two selectors; `slot == -1` reads the signed
byte at `+0x26` instead. `save_image_update_checksums` hashes the header and
then each record with the `0x8015F130` checksum thunk, storing the result in the
word that follows the payload.

### Frame support

`800d08a0` is the four-byte thunk the frame reaches at step 27: it fetches
`*(r13 - 0x5220)`, takes the object at `+0x10` and calls vtable slot `+0x0C`.
`800c9a10` constructs the static heap object at `0x80359470` through
`0x800E5400` (already translated as `engine::dsys::fn_800e5400`) and links its
destructor `0x800E5480` through `0x802DC780`. `800e2ad0` builds two vertex
arrays: `count` elements of `0x0C` bytes and `count * byte_6 + 1` elements of
the same stride, both filled with the constant vector at
`0x80377DC8..0x80377DD0`.

## Evidence quality notes

* The `802DCD74` / `802DCDC0` pairs printed by `800e02d0` (and by the
  not-translated `800d2ac0`) are CodeWarrior register save/restore helpers, not
  game logic. They are omitted, following `docs/COVERAGE_WAVE16_ENGINE.md` and
  the note already recorded in `src/audio/ax.rs`.
* `800d2a50` and `800d3030` print a call with no operand. The callee
  (`0x802B9280`, `0x80169CD0`) does take `r3`; the Rust passes the object and
  marks the operand reconstructed.
* The element stride `0x13C` is proven twice: the pool constructor's
  `0x802DC9A0` argument and the highest field (`+0x138`) written by the
  `Wm_mr_watersplash` spawn at `0x800D2AC0`.
* `800e0380` was already referenced by the HUD code in `display.rs` as
  `FN_800E0380` ("course/map id"). The const is kept for the call-by-address
  sites and now points at the translated body; the doc was corrected to say
  "course/map record".
* No `# Original` block reuses a Ghidra name that the export does not contain.
  All Rust names are descriptive and every one keeps its address.

## Uncertainties

1. **`800d1510` callee operands.** The export prints
   `0x800D16B0(cursor, node, arg)` and consumes `r3`, while its own decompiled
   body is a parameterless tail call to `0x802DCBFC`. The Rust keeps the
   call-site arguments; what those three words mean is unknown.
2. **`800d1f10` descriptor.** `0x802DCBFC(cursor, 0x802F1220)` returns a word
   whose low byte is stored at `+0x2A`. `0x802F1220`'s contents are not decoded.
3. **Five SDA2 clamp floats.** The five values at `0x8042C9E8..0x8042C9F8` are
   read at run time; no numeric value is invented.
4. **Water-effect class names.** `water_effect_*` and `water_node_*` are
   inferred from the `Wm_mr_watersplash` spawn that writes the same 0x13C-byte
   layout; the Ghidra export gives no class names.
5. **`800d3030` first operand.** The first `0x80169CD0` call prints no argument;
   the Rust passes `node` because `r3` still holds it at the call site.
6. **Save-image checksum split.** `800e0470` hashes `+0x24` for `0x698` bytes
   and stores at `+0x6BC`, while `800e04b0` hashes each 0x980 record from its
   base for `0x97C` bytes and stores at `+0x97C`. That is what the export says;
   the differing header/record splits are not reconciled with
   `src/game/scene.rs`'s `+0x69C` layout, which describes a different buffer.
7. **`800c9b00` resource.** What `0x802AC480(name, 0x80427C08)` looks up, and
   why a non-zero nested word means "dirty", is not established.
8. **`800e2ad0` second array.** The `count * byte_6 + 1` array is inferred to be
   a strip/fan index buffer; the export only proves the size expression.
9. **`800e2ad0` constant vector.** `0x80377DC8..0x80377DD0` are read from
   memory; their values are not decoded.
10. **`dsys` ledger gap.** `800e5060..800e5480` have bodies in
    `src/engine/dsys.rs` but no `ledger/functions.csv` rows. This lane did not
    claim them; the owning lane should add the rows.
11. **`800e0180` allocator lock.** The `0x8016E4F0` thunk takes no argument in
    the export but redirects `r3`; the Rust reads the previous value and passes
    it back on unlock, which is the only use that makes the pair symmetric.

## Next targets

Ordered by value per effort, with the evidence file to read first.

| Priority | Target | Size | Why | First evidence |
|---:|---|---:|---|---|
| 1 | `800d2ac0` | 208 B | `Wm_mr_watersplash` spawn; completes the water pool | `800d2ac0_FUN_800d2ac0.c` |
| 2 | `800d2b90` | 144 B | per-element water-effect update; float ABI (`f1`) | `800d2b90_FUN_800d2b90.c` |
| 3 | `800d2dd0` | 472 B | water actor constructor; owns the four 0x80-byte nodes | `800d2dd0_FUN_800d2dd0.c` |
| 4 | `800d3280`, `800d32f0` | 112 + 132 B | water actor per-frame update and model load | `800d3280_FUN_800d3280.c` |
| 5 | `800e0070` | 264 B | per-slot visibility setter; the `auStack_1c` local needs an instruction check | `800e0070_FUN_800e0070.c` |
| 6 | `800e0290` | 64 B | save-record destructor; same missing-operand pattern as `800d2a50` | `800e0290_FUN_800e0290.c` |
| 7 | `800e1f80`, `800e20a0` | 340 + 380 B | scene-transition position helpers; `extraout_r4` must be checked first | `800e1f80_FUN_800e1f80.c` |
| 8 | `800d0470` | 360 B | effect-record spawn; `extraout_r4` must be checked first | `800d0470_FUN_800d0470.c` |
| 9 | `800d0610` | 656 B | 4-view 3-D setup; paired-single/GQR artefacts | `800d0610_FUN_800d0610.c` |
| 10 | `800d19e0` | 976 B | cursor 3-D setup; paired-single/GQR artefacts | `800d19e0_FUN_800d19e0.c` |

Do not start with `800e2de0`/`800e2f10`/`800e3040`/`800e3160`: all four carry
GQR/`ldexpf` artefacts and need the instruction words before a faithful
translation is possible.

## Ledger

`ledger/agents/wave20-engine.csv` has one row per function above with the
seven-column header from `docs/RUST_STYLE.md`
(`module,address,name,status,confidence,evidence,rust_file`). The `module`
column is `engine::display` for the 22 `display.rs` rows and `engine::frame`
for the 3 `frame.rs` rows, matching each `rust_file` path. The same 25 rows were
appended to `ledger/functions.csv`.
