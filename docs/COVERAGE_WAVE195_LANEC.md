# Wave 195 lane C coverage (`game::bases`)

Ten functions, `0x80685150`..`0x80765A90`, all translated into
`work/wave195/laneC_block.rs` for the coordinator to append to
`src/game/bases.rs`. Nothing under `src/` was edited by this lane; the block was
type-checked against a scratch crate that stubs only the helpers it calls.

## Translated

| Address | Bytes | Rust name | Summary |
| --- | --- | --- | --- |
| `0x8068_5150` | 260 | `fn_80685150` | Damped vertical integrator: blends `+0x69C`/`+0x6A0` partner heights (or `+0x694`), writes `+0xEC`, advances `+0x66C` and clamps to `target - 32.0 .. target + 32.0`, then tail-calls `0x8015F470(0.0, this[+0x67C], this + 0x678)`. |
| `0x8068_6240` | 272 | `fn_80686240` | Proposes `other` (`r4`) into the `state` (`r5`) record: signed-byte variant `0x8042C1A0[state[+8]]`, two `>= 0.0` product tests using `+0xAC` and `fn_806861e0(this, state)`, then `variant * (other[+0x668] - state[+0x10][+0x668]) == 1` latches `state[+0x10] = other`. |
| `0x8068_C5A0` | 264 | `fn_8068c5a0` | Floor probe: 33 samples down `16.0` from `height`, hit mask `0x88001` on `0x80070BA0(x, y, this[+0x38F])`; first hit-then-miss returns `16.0 * trunc(y * 0.0625)`, otherwise `-1.0e7`. |
| `0x8069_1860` | 284 | `fn_80691860` | Steps `+0x530`/`+0x534` by `0x200` (only when `+0x536` is set) and `+0x532` by `0x400`, then writes `sin(u16 / 256) * {700,1024,256}` as `i16` to `+0x100`/`+0x102`/`+0x104`. |
| `0x8069_3DD0` | 268 | `fn_80693dd0` | Box-plus-radius proximity: `|dx| < 2.0 + this[+0x4A4]`, `|dy| < 2.0 + this[+0x4A8]`, then `sqrt(dx^2+dy^2) < sqrt(rx^2+ry^2)`; called by `fn_80693d50` as `(this, candidate)`. |
| `0x8076_5A90` | 268 | `fn_80765a90` | Five-pane transform: per `0x38`-stride record at `+0xDF8` place matrix `+0x7C` at `(x, y + offset, z)`, bind rotation `(0, record[+0x12], 0)`, copy to `+0x404` pane, scale `(scale, 1.0, scale)`, clear. |
| `0x8069_9A30` | 292 | `fn_80699a30` | Nearest-object search over list `299`: skips `0x8069A6E0` hits, null `+0x738` handles and `0x806A3D60` rejects, then keeps the smallest `dx^2 + dy^2 + dz^2` against `FLT_MAX`. |
| `0x806B_4A10` | 268 | `fn_806b4a10` | Player-carry feed for mode `+0x38C == 1`: `0x806B4ED0(this,1)`, picks virtual `+0x29C`/`+0x298` for height, calls player `+0x3FC(height, player[+0xE8], player, 1, 2, 0)`, then virtual `+0x2B8` and `+0xD4(0x8082C4FC)`. |
| `0x806C_0F00` | 276 | `fn_806c0f00` | Scale-driven tilt: reads `list 0x79[+0x394]`, adds `8192 * scale` with a `1024` bias to `+0x524`, and writes `16384 * scale + (4096 * scale +/- 512) * sin(u16 / 256)` to `+0x104`; returns the scale. |
| `0x806C_FFD0` | 264 | `fn_806cffd0` | Rider-lift height step: sample `0x807CB490[(idx & 0xFF)]` from `0x806CFED0(this, flags)` into `+0xB0`/`+0xEC`; below `target - 64.0` plays sound `0x2AF`, bumps `+0x656` by the `0x807CB498` halfword table and runs `0x806D0510(this, 0x40, 0x180)`, else settles with `0x806D0510(this, 4, 0xC0)`. |

## Deferred

None. All ten bodies were recovered from the raw words.

## Notes on the two requested callees

* `fn_8068c5a0(height: f32, this: usize) -> f32` - the exact signature is
  `f1 = height`, `r3 = actor`, result in `f1`. It is invoked by
  `fn_8068ca80` and `fn_8068cd70` through `call_f32_u32(0x8068_C5A0, height,
  actor as u32)` and the callers re-interpret the returned `u32` with
  `f32::from_bits`, which matches this body returning `f32`.
* `fn_80693dd0(this: usize, other: usize) -> u32` - `fn_80693d50` calls it as
  `abi_call2!(0x8069_3DD0, this, next)` and only tests the result against `0`,
  so the two-word signature and `u32` return are fixed by that call site.

## Two places where the raw words beat the export

1. `0x80685150`: the Ghidra export ends in `halt_baddata()` at `0x80685250`
   because the block after the clamp is a literal `b 0x8015F470` tail call (the
   next function starts at `0x80685254`, the exact size boundary). The tail call
   is reproduced with `call_f32x2_u32`, the same shape `fn_8068cd70` already
   uses for that helper.
2. `0x80691860`, `0x8068c5a0`, `0x806c0f00` and `0x8068c5a0`'s companions: the
   Ghidra exports reconstruct every `psq_l ...,1,3` through `ldexpf`/signed
   `short` arithmetic because GQR3 is unknown to the decompiler. The raw
   halfword store immediately before each load, plus the brief's established
   GQR3 = unsigned-16 rule, makes each sample `u16 as f32`.
