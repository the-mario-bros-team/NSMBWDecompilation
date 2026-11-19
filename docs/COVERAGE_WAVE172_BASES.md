# Coverage - Wave 172 (`game::bases` 332-348 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 24 | 8,144 | 23 | 1 |

Fifth batch from the 300-512 byte band and the first five-lane wave since the
paid model came back. All 24 bodies are in the 332-348 byte sub-band.

## Lane A (5 bodies, 1,668 bytes)

- `0x8068CB40` - angle step plus the `10.0 + h < +0xB0` probe; takes the
  `0x8082ACF0` event or spawns `Wm_en_landsmoke_s` with sound `0x2C8`.
- `0x806DD1A0` - seat/layout reset, `+0x5BC`/`+0x61C`/`+0x67C` teardown, seat
  vector at `+0x474`, returns `1`.
- `0x806F42F0` - `+0xBEBC` loop rebuilding `+0x7C` and the `+0x1F4` node ring.
- `0x8075C070` - seven-state `+0x1A4`/`+0x1AC` timer; `0x800EF700` on the
  `0xBF8`-strided world record; returns `1` in state 7.
- `0x8069C620` - twin of `fn_8069c170` with `+0x500 = 100` and a signed child
  mode byte.

## Lane B (5 bodies, 1,688 bytes)

- `0x806843B0` - lane height test over the `0x8042C1A0` table with the
  `0x80072DB0` transform probe and the `+0x348` mirror.
- `0x80719250` - model rebuild plus the `f32(+0xA08) * u8(0x80808438)` byte
  patch into the `0x8000D5B0` word.
- `0x8075B2B0` - `+0x744` slider driver for messages `0x48`/`0x4A`.
- `0x80620A00` - seven-slot pane refresh through `0x806209D0`; the `0x28`
  sentinel continues rather than breaks.
- `0x806F25B0` - record collector with the `(word >> 12) & 0xFFF` metric at
  `+0x7DC`.

## Lane C (5 bodies, 1,704 bytes)

- `0x80706510` / `0x80706690` - lift height-followers; sounds `0x2F7`/`0x2F6`
  and the `0x8082F3A0`/`0x8082F360` interface events. The clamp compares the
  truncated 16-bit value, not Ghidra's 32-bit difference.
- `0x80733C30` - burst-table spawn from `0x807CD7C8`, effect `0x85`, sound
  `0x2A2`.
- `0x8075C550` - `cobKoopaShip` frame body; **medium** because
  `0x802DCBFC` keeps its hidden `r12` descriptor.
- `0x80746910` - rail vertical scan, `count = bounds[3] * 0.03125`, step `32.0`.

## Lane D (5 bodies, 1,736 bytes)

- `0x8077E410` - pose/motion re-apply over the `0xC`-stride record at
  `*(+0x294)`; model virtuals `+0x5C` and `+0x84`.
- `0x8064F390` / `0x8064F6A0` - the two `+0x5BC`/`+0x584` animation gates with
  the `0x808282B8`/`0x80828278` callbacks.
- `0x80675090` - player-count frame rebuild with the `+0x14D4` count.
- `0x80772BD0` - unused-name picker over the `0x16518`-stride table.

## Lane E (4 bodies, 1,348 bytes)

- `0x806D16B0` - variant selects the `+0x3D0` box height (`148` when the
  `+8` halfword is `0x1FD`, else `116`) and seeds the five-double
  `0x800DB180` call.
- `0x80734E30` - four-slot audio fade with the GQR3 `u16` sample at `+0x51C`
  scaled by `0.00390625`, then the `+0x534..+0x537` shifts.
- `0x80775BB0` - walks up to 30 clip ids at `+0x6C470`, starts/stops
  `0x800DFF30`.
- `0x80788B50` - effect placement; `0x80789B80` is `(f1 rate, r3 object,
  r4 slot)`, so `call_f32_u32x2(..., 5.0, this, 3)`.

## Corrections applied outside the lane blocks

- **Wave 171 fix.** `0x801680E0` takes the quotient in `f1`; `fn_8064eeb0` was
  calling it with `abi_call1`, dropping the value. Both mode arms now use
  `call_f32_u32` with the computed quotient.
- **Earlier waves fix.** Seven `call_f64_u32x3(0x8077_E410, 0.0, ...)` sites
  were changed to `call_f32_u32x3`; the first argument is `f32` at every call
  site (the `f64` form only worked because the value was `0.0`).
- **Earlier waves fix.** `fn_80621630` called `0x80620A00` with
  `platform::call0`; the original passes `this` in `r3`, now `call1`.

## Verification

- `cargo check` succeeds with 4,170 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave172`: 24 rows, 24 unique addresses,
  23 high + 1 medium.
- All 24 evidence exports exist and every size matches `functions.tsv`.
