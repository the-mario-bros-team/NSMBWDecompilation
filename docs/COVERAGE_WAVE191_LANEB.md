# Wave 191, lane B coverage (`game::bases`)

Lane band `0x807B0780..0x807BA750`, in the scene-setup / screen cluster of REL
`d_basesNP`. Five of five requested bodies translated; none deferred.

## Bodies

| Address | Bytes | Function | Summary |
| --- | --- | --- | --- |
| `0x807B0780` | 188 | `fn_807b0780` | Loads `MainSelect` from `scene/scene.brres` (`Env_world`), starts the world animation with `0.0`, and runs `0x807B06C0` when the mode is `1`. |
| `0x807B5C60` | 188 | `fn_807b5c60` | Loads `MovieScene` from `scene/scene.brres` (`Env_movie`), starts the animation with `0.0`, and runs `0x807B5D20`. |
| `0x807B8B90` | 192 | `fn_807b8b90` | Rebuilds the GX viewport (six floats) and scissor (four words) from the screen halfwords at `[0x8042A480]+4/+6`. |
| `0x807BA510` | 188 | `fn_807ba510` | Binds `light/Layout3D.blight`, `light/Layout3D.blmap` (to `world+0x14`) and `fog/Layout3D.bfog` through vtable slot `+0x08`. |
| `0x807BA750` | 192 | `fn_807ba750` | Duplicate of `fn_807b8b90` with the constant slots `0x807D0950`/`0x807D0958`. |

## Evidence and checks

- Every body was re-read from the raw words with
  `ppc159.js work/rels/elf/d_basesNP.elf <addr>`; the Ghidra exports were used
  only as an index. `bl 0x802DCD6C/74` and `bl 0x802DCDB8/C0` are the
  `_savegpr`/`_restgpr` compiler helpers and are not emitted.
- `0x807B0780` is instruction-for-instruction the twin of the already-translated
  `fn_807abe20` (`0x807ABE20`): same `Env_world` → `scene/scene.brres` →
  `MainSelect` chain, same `0x8023A2E0`/`0x802426D0` pair, same
  `0x802C8840(0.0, world, &entry, -1, 3)`. Only the rate slot
  (`0x807D07C0` vs `0x807D0738`, both `0.0f32`) and the tail callee differ.
- `0x807B5C60` is the movie counterpart; it additionally calls `0x80164B80` and
  ends with an unconditional `0x807B5D20(this, index)`. `0x807B06C0` and
  `0x807B5D20` have no Rust body yet, so both are reached through
  `abi_call2!` with the exact `r3`/`r4` values.
- `0x807BA510` is the twin of `fn_807abd60`; its three tails load the vtable
  word at `+0x00` and branch to slot `+0x08`, forwarding `r4`. That is exactly
  `vmethod1(obj, 0x08, resource)`, so the slot is called directly instead of
  through `fn_807ac270`/`fn_807ac260`/`fn_807ac280`, whose Rust signatures drop
  the forwarded argument.
- `0x807B8B90`/`0x807BA750`: the frame is `r1+0x10` (word from `0x80164B30`)
  with the two float outputs at `r1+0x0C`/`r1+0x08`; `0x80253E60` is called with
  `(&frame, 0, 0, 0, 0, &out_a, &out_b)` and its result is discarded — the
  `fmr f2,f1` at `0x807B8BE8` copies the `lfs`-loaded `0.0` from `0x807D0808`,
  not a return value. The `0x43300000`/`0x43300000` pairs and the `fsubs`
  against `0x807D0820`/`0x807D0958` (`2^52`) are the compiler's u16→f32 idiom,
  written as `as f32`.
- `0x801C9C10` takes six floats and `0x801C9C60` four words. The pairs match the
  already-translated `stage_camera_apply` (`0x8008A280`) in `src/game/stage.rs`,
  which calls the same two addresses with the camera viewport floats and scissor
  words; `bases.rs` only has `call_f32x6_u32`, whose trailing `u32` is the
  ignored `r3`.
- Constants read from the ELF: `0x807D07C0 = 0.0`, `0x807D07E8 = 0.0`,
  `0x807D0808 = 0.0`, `0x807D0950 = 0.0`,
  `0x807D0820 = 0x43300000 00000000`, `0x807D0958 = 0x43300000 00000000`.
- Strings read from the ELF: `0x8081E6EC "Env_world"`,
  `0x8081E6F8 "scene/scene.brres"`, `0x8081E70C "MainSelect"`,
  `0x8081FBE0 "Env_movie"`, `0x8081FBCC "scene/scene.brres"`,
  `0x8081FBEC "MovieScene"`, `0x80820B18 "Env_world"`,
  `0x80820B24 "light/Layout3D.blight"`, `0x80820B3C "light/Layout3D.blmap"`,
  `0x80820B54 "fog/Layout3D.bfog"`.

## Deferred

None.

## Files

- `work/wave191/laneB_block.rs`
- `ledger/agents/wave191-laneb.csv`
- `docs/COVERAGE_WAVE191_LANEB.md`
