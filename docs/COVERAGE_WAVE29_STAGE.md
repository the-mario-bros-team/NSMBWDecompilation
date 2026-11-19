# Coverage: wave 29 stage band (`0x80080000..0x800C0000`)

## How to read this file

This document covers the wave-29 batch of the stage/course lane. All 28 new
bodies live in `src/game/stage.rs`; no other Rust file was changed. The batch
finishes the stage-block variant cluster that wave 15 left open
(`0x80090230..0x80092C00`): the four- and six-segment conveyor/water-wheel
movers, the water-wave effect and ground snap, the variant destructors and list
walkers, and the short/long/motion block spawners.

Every row describes one Ghidra entry point:

- **address** - virtual address of the entry in `main.dol` (image base
  `0x80004000`, so these are not file offsets);
- **name** - the Rust function in `src/game/stage.rs`;
- **role** - one plain sentence;
- **status** - `translated` means the whole recorded control flow is present,
  with calls outside the batch left as absolute-address calls;
- **confidence** - `high` when every load/store/branch was read directly from
  the Ghidra C; `medium` when a field meaning, an indirect call, a table value
  or a hardware/effect role is inferred from call sites;
- **bytes** - Ghidra's recorded function size;
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read from the Ghidra output,
`// inferred:` marks roles reconstructed from call sites, and `// unknown:`
marks unresolved details. Each function's `# Original` block repeats the Ghidra
name, address, size and evidence path.

The only files written by this pass are `src/game/stage.rs`, this document and
`ledger/agents/wave29-stage.csv`. `lib.rs`, every `mod.rs`, `Cargo.toml`,
`platform.rs`, generated documents and other lanes' files were not touched.

## Summary

| Metric | Count |
|---|---:|
| Functions translated in this batch | 28 |
| Bytes of Ghidra function size | 4,976 |
| `translated` | 28 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 19 |
| Confidence `medium` | 9 |
| Confidence `low` | 0 |
| Destination `src/game/stage.rs` | 28 |
| New staging modules | 0 |

The selection was made by walking
`work/ghidra/main-gb/export/functions.tsv` over `0x80080000..0x800C0000`,
subtracting every address already in `ledger/functions.csv`, and then checking
each candidate against `src/game/stage.rs`. Every address below was absent from
both. The batch deliberately stays inside the stage-block object family already
documented by wave 15; no BG-unit or course-parser body was displaced.

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---:|---:|---|
| `0x80090230` | [`stage_block_advance_segments`] | advance the animated block's segment table by one frame | translated | medium | 392 | `80090230_FUN_80090230.c` |
| `0x800913A0` | [`stage_block_four_segment_matrices`] | rebuild four conveyor matrices from their positions and angles | translated | medium | 276 | `800913a0_FUN_800913a0.c` |
| `0x800914C0` | [`stage_block_four_segment_reset`] | copy the origin into all four conveyor segments | translated | high | 84 | `800914c0_FUN_800914c0.c` |
| `0x80091520` | [`stage_block_four_segment_motion_init`] | build and select the four-segment motion table | translated | high | 244 | `80091520_FUN_80091520.c` |
| `0x80091620` | [`stage_block_four_segment_angles_clear`] | clear the four-segment angle table | translated | high | 56 | `80091620_FUN_80091620.c` |
| `0x80091660` | [`stage_block_four_segment_advance`] | integrate four conveyor segments and their yaw angles | translated | high | 292 | `80091660_FUN_80091660.c` |
| `0x80091A80` | [`stage_block_six_segment_matrices`] | rebuild six water-wheel matrices from their positions and angles | translated | medium | 276 | `80091a80_FUN_80091a80.c` |
| `0x80091BA0` | [`stage_block_six_segment_reset`] | reset six segments to `(x, y, 3250.0f)` | translated | high | 108 | `80091ba0_FUN_80091ba0.c` |
| `0x80091C10` | [`stage_block_six_segment_motion_init`] | build and copy the six-segment motion table | translated | high | 248 | `80091c10_FUN_80091c10.c` |
| `0x80091D10` | [`stage_block_six_segment_angles_clear`] | clear the six-segment angle table | translated | high | 80 | `80091d10_FUN_80091d10.c` |
| `0x80091D60` | [`stage_block_six_segment_advance`] | integrate six water-wheel segments and their yaw angles | translated | high | 292 | `80091d60_FUN_80091d60.c` |
| `0x80091FB0` | [`stage_block_waterwave_step`] | ground-snap the water wave, then flip it when it settles | translated | medium | 96 | `80091fb0_FUN_80091fb0.c` |
| `0x80092030` | [`stage_block_waterwave_effect`] | submit the mode-selected water-wave effect | translated | medium | 284 | `80092030_FUN_80092030.c` |
| `0x80092150` | [`stage_block_waterwave_ground_snap`] | probe downward and write back the ground height | translated | medium | 176 | `80092150_FUN_80092150.c` |
| `0x80092200` | [`stage_block_variant_destroy`] | destroy one block variant and unlink it | translated | high | 132 | `80092200_FUN_80092200.c` |
| `0x80092290` | [`stage_block_position_add`] | add a three-float offset to the block position | translated | high | 52 | `80092290_FUN_80092290.c` |
| `0x800922D0` | [`stage_block_vertical_advance`] | integrate and clamp the block's vertical velocity | translated | high | 36 | `800922d0_FUN_800922d0.c` |
| `0x80092310` | [`stage_block_list_release`] | release a block variant and clear the global block root | translated | high | 92 | `80092310_FUN_80092310.c` |
| `0x80092370` | [`stage_block_insert`] | insert a freshly allocated block variant into the list | translated | high | 84 | `80092370_FUN_80092370.c` |
| `0x80092480` | [`stage_block_list_activate`] | call the activation slot on every idle block | translated | medium | 88 | `80092480_FUN_80092480.c` |
| `0x800924E0` | [`stage_block_list_destroy_active`] | destroy every active block in the global list | translated | medium | 132 | `800924e0_FUN_800924e0.c` |
| `0x80092570` | [`stage_block_list_destroy_all`] | destroy every block in the global list | translated | high | 120 | `80092570_FUN_80092570.c` |
| `0x80092720` | [`stage_block_spawn_short`] | spawn the 0x128-byte short block variant | translated | high | 272 | `80092720_FUN_80092720.c` |
| `0x80092830` | [`stage_block_spawn_long`] | spawn the 0x2DC-byte long block variant | translated | high | 312 | `80092830_FUN_80092830.c` |
| `0x80092970` | [`stage_block_element_construct`] | construct one array element of the long variant | translated | high | 100 | `80092970_FUN_80092970.c` |
| `0x800929F0` | [`stage_block_spawn_motion`] | spawn the motion block variant with a three-word offset | translated | medium | 224 | `800929f0_FUN_800929f0.c` |
| `0x80092AD0` | [`stage_block_spawn_six`] | spawn the six-segment block variant | translated | high | 272 | `80092ad0_FUN_80092ad0.c` |
| `0x80092C00` | [`stage_block_bind_buffer`] | create a render buffer and apply two fill selectors | translated | high | 156 | `80092c00_FUN_80092c00.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### A. Four-segment conveyor (`0x800913A0..0x80091660`)

The object stores four positions at `+0xB0` (stride `0x0C`), four velocities at
`+0xE0`, twelve `u16` angles at `+0x110`, and four 12-word matrices at `+0x80`.
`0x800914C0` copies the origin into all four positions; `0x80091620` clears the
angles; `0x80091520` selects one of two eight-word rows in the global
`0x80356060` table and writes it to `+0xE0..+0x10C`; `0x80091660` then
integrates two segments per iteration (three iterations would not fit the four
segments, so the loop count is two).

`0x800913A0` is the matrix builder: `0x80064AF0` rotates a position by the
three `u16` angles, `0x801C0BD0` builds the translation matrix, and the three
rotation helpers run in the order y, x, z before the matrix is bound to the
model at `+0x80`.

### B. Six-segment water wheel (`0x80091A80..0x80091D60`)

The layout is the same idea at a different base: positions at `+0xC8`, six
velocities at `+0x110`, eighteen `u16` angles at `+0x158`, and six matrices at
`+0x80`. `0x80091BA0` resets every position to `(x, y, 3250.0f)`; the z value
was read from the shipped ELF at `r2-0x6F6C`. `0x80091C10` copies six motion
triples into `+0x110..+0x154` with a constant `2.0f` in every third slot.
`0x80091D60` integrates two segments per iteration over three iterations.

### C. Water-wave mode (`0x80091FB0..0x80092150`)

`0x80092150` starts at `y - 8.0f` and retries up to five times with a `+4.0f`
step through the ground probe `0x80075270`. Status `1`/`3` writes the returned
height back to `+0x08`; status `2` stops without writing. `0x80091FB0` calls
that snap and then virtual slot `+0xB0` of the interface at `+0x68`.
`0x80092030` selects one of the `0x803115C8` effect names by the block mode;
modes 6 and 7 take the direct `0x8016C890` path, every other mode takes the
`+0xA0` virtual path. In stage 7 area 2, mode 4 scales the offset by `0.6f`
and mode 5 is rewritten to mode 4.

### D. Variant lifecycle (`0x80092200..0x80092570`)

`0x80092200` is the destroying destructor: release the `+0x68` subobject,
reinstall the base vtable, unlink `+0x54`, and delete when the size is positive.
`0x80092310` clears the global block root at `r13-0x5798` after calling the
destroy-all walker. `0x80092370` is the shared insert helper: copy three
position words to `+4..+0xC`, store the selector at `+0x44` and the byte at
`+0x4C`, call virtual `+0x0C`, and return `1`.

The three list walkers use the intrusive list at `r13-0x5790`: node `+4` is the
next link and node `+8` is the object. `0x80092480` calls virtual `+0x14` only
for objects whose `+0x50` word is zero; `0x800924E0` calls virtual `+0x18` and
then deleting slot `+0x08` only for objects whose `+0x50` word is non-zero;
`0x80092570` does both calls for every object without the `+0x50` predicate.

### E. Block spawners (`0x80092720..0x80092C00`)

The four spawners share the same prologue: allocate through `0x802B9210`,
install `0x8031133C`, build the `+0x54` intrusive node, append it through
`0x80160FE0`, then install the variant vtable and construct its arrays through
`0x802DC9A0`. The dump below records the observed array geometry.

| spawner | size | vtable | arrays at `+offset` | count x element |
|---|---:|---|---|---|
| `0x80092720` | `0x128` | `0x803114A8` | `+0x80`, `+0xB0`, `+0xE0` | 4 x `0xC` |
| `0x80092830` | `0x2DC` | `0x80311320` | `+0x80`, `+0x180`, `+0x230`, `+0x2A0` | `0x40/0x2C/0x1C/0xC` x 4 |
| `0x800929F0` | `0x17C` | `0x803115E8` | `+0x68` subobject | - |
| `0x80092AD0` | `0x17C` | `0x80311514` | `+0x80`, `+0xC8`, `+0x110` | 6 x `0xC` |

`0x80092970` is the element constructor used by the long variant's `0x2C` array:
it installs `0x803295D8`, clears `+4/+8`, constructs the `+0x0C` buffer owner,
clears `+0x28`, and installs `0x80329518`.

`0x80092C00` creates the buffer through `0x80164FD0` (the same helper already
named `TILEMAP_CREATE_BUFFER` by the wave-23 code), applies the two fill
selectors through `0x8016A2A0`, and sets flag 6/7 through `0x8016A0F0` when the
corresponding selector is negative.

## Uncertainties carried forward

- `0x80090230`: the fields `+0x48` and `+0x50` are a countdown and a latch by
  their use; the `0.5f` scale at `r2-0x6FE0` is observed, but which model
  animation it drives is unknown. `0x80090230` and `0x80092150` are the only
  bodies here with Ghidra's `GQR`/`ldexpf` guards around float loads; those
  guards are treated as load-format artefacts because the recovered data flow
  is ordinary single precision. An instruction-level pass can still confirm
  that the shipped `psq_l` format uses the default scale of one.
- `0x800913A0` / `0x80091A80`: the three rotation helpers are called in the
  order y, x, z; their axis names are inferred from the GX helpers, not from a
  symbol table. The stack matrix is modelled as 12 words, the GX 3x4 layout
  already used by `stage_block_actor_setup`.
- `0x80091520` / `0x80091C10`: the global tables are written once at first use.
  All values were read from the shipped ELF, but their gameplay role (wave
  amplitude/phase columns) is unknown.
- `0x80091FB0` / `0x80092030` / `0x80092150`: the `+0xB0` and `+0xA0` slots and
  the `Wm_en_waterwave_in_*` table are observed through their call sites; the
  interface type and the probe status `2` meaning are inferred.
- `0x80092200` / `0x80092310` / `0x80092480` / `0x800924E0` / `0x80092570`:
  the list node layout is observed from `0x80160FE0`; the `+0x14` and `+0x18`
  virtual slots are named activation/destroy from the matching predicate, not
  from a recovered vtable symbol.
- `0x80092720` / `0x80092830` / `0x800929F0` / `0x80092AD0`: the array element
  types and constructor roles are unknown beyond their addresses and sizes.
  The spawners reproduce the original null-check only around allocation; the
  insert helper dereferences the result, exactly as the original does.
- `0x800929F0`: the three words copied from `param_5` are written outside the
  allocation check in the original; the Rust keeps that placement.
- `0x80092C00`: Ghidra prints `thunk_FUN_80164fd0`; the Rust calls the already
  documented `0x80164FD0` target under its existing constant name.

## Method notes

- Every function cites its Ghidra name, address, size and exact `.c` file in
  `# Original`, and carries `# Confidence` plus `# Safety`.
- Loads/stores go through `crate::platform` so big-endian semantics do not
  depend on the host.
- Calls outside the batch are absolute-address calls with the original
  argument registers; none was "simplified" away.
- In-batch calls are direct Rust calls, including
  [`stage_block_advance_segments`] → [`stage_block_vertical_advance`] /
  [`stage_block_position_add`], [`stage_block_waterwave_step`] →
  [`stage_block_waterwave_ground_snap`], the spawners →
  [`stage_block_insert`], and [`stage_block_list_release`] →
  [`stage_block_list_destroy_all`].
- `cargo check` passes; it reports no diagnostics for `src/game/stage.rs` (the
  crate keeps 11 pre-existing warnings in `runtime/bt.rs` and `runtime/input.rs`).
- `node tools/doc-lint.mjs src` reports 68 files and 0 violations.
