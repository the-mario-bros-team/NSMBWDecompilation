# Coverage - Wave 191 (`game::bases` 128-192 band, part 5, complete)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 26 | 4,412 | 24 | 2 |

The last bodies of the 128-192 band; that band is now complete.

## Highlights

- **World/scene starters (4).** `0x807ADAD0`, `0x807AED70`
  (`Env_world`/`scene.brres` + `MainSelect`), `0x807B0780`, `0x807B5C60`
  (`Env_movie`).
- **Layout light/fog binding (3).** `0x807AECB0`, `0x807B06C0`, `0x807BA510`
  (`.blight`/`.blmap`/`.bfog` through slot `+0x08`).
- **Display/GX bodies (4).** `0x807AE400` (20-call display-list reset),
  `0x807B8B90`/`0x807BA750` (GX viewport 6 floats + scissor 4 words),
  `0x807BEA40` (60-byte frame buffer render).
- **Save-end states (4).** `0x807C2C00`, `0x807C3020` (window on stage / error)
  and `0x807C2CC0`, `0x807C30E0` (anime wait), with descriptors pinned via the
  existing `dCourseSelectManager_c` table.
- **The rest** are prompt gates, teardown and ease helpers.

## Corrections and recoveries

- `0x807BE3D0`'s real signature is `(t: f32, q: f32) -> f32`; `q` is forwarded
  in `f2` to `fn_807be230` (neither branch writes `f2`, which is why Ghidra
  shows one parameter). The existing `fn_807bfe20` call site still uses
  `call_f64_u32(..., value as f64, 0)`, which is subtly wrong; fixing it needs
  an `(f32, f32) -> f32` shim, so it is recorded here.
- `0x807AFF90`'s branch polarity is reversed versus the Ghidra `if/else`
  (`bc 12,2` on the `+0xD0` test).
- `0x80833AA0`/`0x80833B60` are BSS prompt descriptors passed as literal
  addresses.
- `0x807B8B90`/`0x807BA750`: `fmr f2,f1` copies the `lfs`-loaded `0.0`, not a
  return value; the `0x43300000`/`fsubs 2^52` pairs are the u16->f32 idiom.
- `0x807AC2x0` are `vmethod0(this, 8)` stubs forwarding `r4`, so the three
  light/fog tails become `vmethod1(obj, 0x08, resource)`.
- `0x807BA860` zeros the 20 words at `0x80374880..0x803748CC` in two `stwu`
  blocks before running `fn_807bac40(this, i)` for `i = 0..9`.

## Verification

- `cargo check` succeeds with 4,844 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave191`: 26 rows, 26 unique addresses,
  24 high + 2 medium.
- All 26 evidence exports exist; 4,412 bytes total.
