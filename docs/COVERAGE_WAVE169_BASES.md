# Coverage - Wave 169 (`game::bases` 300-332 byte band, part 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 8 | 2,428 | 8 | 0 |

Second batch from the 300-512 byte band. All eight bodies are in the
300-316 byte sub-band.

## What this pass covers

- **Camera/anchor movers (1 body).** `0x8064ED70` picks `+0x584` or `+0x5BC`
  from `+0x600` and runs the `0x801651F0`/`0x80169F80`/`0x80168FE0`/
  `0x801680E0` mover chain, then mirrors the `0x808283E8` byte flags.
- **Player-window test (1 body).** `0x80656080` masks `+0x3D4`, samples the
  window through `0x8008E890` and compares each masked player against
  `[xmin, ymax, width, height]`.
- **Air-state tick (1 body).** `0x8079E680` runs the `+0x2C0` countdown, the
  `0x8079EE70` pose push and the `0x800F69F0` spawn with the `0x8031D39C`
  table constants.
- **Transform rebuild (1 body).** `0x807337D0` rotates the stored position,
  applies the three `+0x100..+0x104` angles, composes the `(0, 8, 0)` offset
  and stores the translation column at `+0x60C..+0x614`.
- **Pose copy (1 body).** `0x807BB710` copies `+0x84`/`+0x98` into the
  `+0x208` slot, gates the `+0xB8` visibility byte on `+0x2B7` and adds the
  `+0x234` offset to the `+0x224` vertical position.
- **Shared-state lerp (1 body).** `0x806FE020` lerps
  `0x80429DD0 + 0x90110` with `(1.0, 0.5, 0.02)`, runs the four `0x806FD*`
  passes and emits `0x5E9` for each player with flag `0x49`.
- **Renderer seed (1 body).** `0x806C8880` unpacks the `+0x04` bitfield into
  five fields, writes the `16`/`48`/`-312`/`8`/`624` state words and spawns
  the `(0, 0, 32, -384)` rectangle through `0x8007FA40`.
- **Sample axis test (1 body).** `0x80799D70` queries the `0x8042A18C` world,
  samples through `0x80100200` and compares the x or z delta with the
  `0x8031D39C + 0xBC` limit.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8064ED70` | `fn_8064ed70` | 300 | high |
| `0x80656080` | `fn_80656080` | 300 | high |
| `0x8079E680` | `fn_8079e680` | 300 | high |
| `0x807337D0` | `fn_807337d0` | 304 | high |
| `0x807BB710` | `fn_807bb710` | 304 | high |
| `0x80799D70` | `fn_80799d70` | 304 | high |
| `0x806C8880` | `fn_806c8880` | 308 | high |
| `0x806FE020` | `fn_806fe020` | 308 | high |

## Corrections and notes

- `0x80656080`'s window layout is `[xmin, ymax, width, height]`: the four
  decoded comparisons are `xmin <= px`, `px <= xmin + width`, `py <= ymax`
  and `ymax - height <= py`. The `local_28/24/20` stack copies of the player
  position are dead and omitted.
- `0x806FE020`'s Ghidra prototype shows a fourth `0.02` float that is not a
  register input. The fourth value is the in/out `+0x90110` buffer, so the
  Rust body uses `call_f32x3_u32` with a `&mut f32`.
- `0x80799D70` is a two-part function: the `0x80799D70` prologue shuffles
  `r3/r4/r5` and falls through to the `0x80799D90` body. The decoded axis
  windows are `{2, 3}` for z and `{4, 5}` for x.
- `0x806C8880`'s five `rlwinm` extracts are `(word >> 12) & 0xF`,
  `(word >> 16) & 0xF`, `(word >> 20) & 0xF` and `(word >> 8) & 3`.

## Verification

- `cargo check` succeeds.
- `node tools/check-lane-csv.mjs wave169` validates the eight-row lane CSV.
- All eight evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`.
