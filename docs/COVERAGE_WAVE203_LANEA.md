# Coverage - wave 203, lane A (`ui::hud`, `0x80020000..0x800208B0`)

Ten functions of `src/ui/hud.rs` translated from `sys/main.dol` in wave 203,
lane A. All ten now have a faithful Rust body appended to `src/ui/hud.rs`;
nothing in this lane is deferred.

## Scope

| address | size | name | confidence | role |
|---|---|---|---|---|
| `0x80020000` | 32 | `fn_80020000` | high | sum the three `+0x04` counts of the scene registry |
| `0x80020020` | 80 | `fn_80020020` | high | seed the `+0x600`/`+0x604` segment cursors |
| `0x80020070` | 144 | `fn_80020070` | high | re-probe/release the three tables when the manager word changes |
| `0x80020100` | 652 | `fn_80020100` | medium | per-frame segment advance, effect/sound and angle update |
| `0x800203C0` | 120 | `fn_800203c0` | high | scale the speed by the mode in bits `20..23` of `+0x04` |
| `0x80020440` | 144 | `fn_80020440` | high | segment lookup and in-segment offset |
| `0x800204D0` | 24 | `fn_800204d0` | high | tail-call `0x800D8C60(*(0x8042A008), 0, 1, 0, 0)` |
| `0x80020640` | 72 | `fn_80020640` | medium | speed curve `256 - (256 / total) * value` |
| `0x80020850` | 64 | `fn_80020850` | medium | release a record array and return its base |
| `0x800208B0` | 12 | `fn_800208b0` | high | returns `0x80377A04` |

Total: 1,344 bytes, 10/10 translated.

## What the cluster is

`0x80020000..0x800208B0` is the immediate continuation of the wave-96/107 wipe
and `daikonbou` run (`0x8001B3A0..0x8001FF90`). Two data anchors tie the ten
bodies to their neighbours:

- **`0x80352974`, three `0x3C`-byte twelve-slot tables.** `0x3C` is exactly the
  size of the table that `fn_8001f800`/`fn_8001f8a0` walk (`+0x00` pass byte,
  `+0x04` live count, `+0x08` published word, `+0x0C..+0x38` twelve slots), so
  the registry is three of those tables back to back. `fn_80020000` reaches the
  three counts through `+0x04`/`+0x40`/`+0x7C`; `fn_80020070` walks them with
  the same `0x3C` stride and calls the two existing passes.
- **`0x80301BE8` = `"Wm_ob_cmnspark"`.** The burst emitted at the segment
  boundary is the world-map spark the neighbouring `daikonbou` actors already
  use (`fn_8001fd40`/`fn_8001e790` share the same effect helpers
  `0x8006A4B0`/`0x80198C30`).

The per-object record uses `+0x5F0..+0x5FC` for four segment lengths, `+0x600`
and `+0x604` as cursors, `+0x5E4` as the registry index and `+0x6AC`/`+0x6B8` as
the two sample tables the unresolved stub `0x802DCBFC` indexes. Segment order in
the cursor table is `0x5F4, 0x5F8, 0x5F0, 0x5FC`, which is not the field order -
the bounds are built in that observed order, not sorted by offset.

## Resolved constants

The six `.sdata2` floats that drive the speed and angle maths were read from
`work/dol/main.elf` (`SDA2_BASE = 0x80433080`):

| address | offset | value | use |
|---|---|---|---|
| `0x8042B26C` | `SDA2 - 0x7E14` | `5500.0` | spark z in `fn_80020100` |
| `0x8042B270` | `SDA2 - 0x7E10` | `256.0` | curve span / selector `2` |
| `0x8042B274` | `SDA2 - 0x7E0C` | `512.0` | angle weight before the `0x800` clamp |
| `0x8042B278` | `SDA2 - 0x7E08` | `7.0` | speed mode `1` |
| `0x8042B27C` | `SDA2 - 0x7E04` | `0.0625` | shared `1/16` factor |
| `0x8042B280` | `SDA2 - 0x7E00` | `14.0` | speed mode `2` |
| `0x8042B284` | `SDA2 - 0x7DFC` | `10.0` | speed mode `3` |
| `0x8042B288` | `SDA2 - 0x7DF8` | `2^52` (double) | magic-double conversion |

The angle update is `min(512 * |speed|, 2048)` (truncated to `i16`, `extsh`),
added on selector `1` and subtracted on selector `3` at the signed halfword
`+0x102`.

## Notes on `0x80020100`

Three points are documented in the body rather than silently normalised:

1. **`r12` operand.** `0x802DCBFC` is entered with `r3 = this`, `r4 = offset`
   and the segment table (`this + 0x6AC` / `this + 0x6B8`) in `r12`.
   `platform::call3` cannot place an operand in `r12`; the table is passed as
   the third operand, the same convention the existing `0x802DCBFC` sites in
   this module use, and the difference is called out in the doc block.
2. **Unwritten stack slot.** `local_34` is only stored inside the four selector
   arms. The selector cannot leave `0..3` because `fn_80020440` scans four
   intervals, so the `0.0` initialisation is unreachable in practice.
3. **`divwu` by the segment total.** A zero total traps in the original exactly
   as the reconstructed `/` and `%` do; no guard was invented.

## Evidence

- Disassembly: `work/dol/main.elf` via `ppc159.js`, `LEN=80`/`140`/`200`,
  `STOP=0`, for all ten entries plus the interior of `0x80020100` (0x28C bytes).
- Decompilation: `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` (lowercase)
  for all ten addresses; `work/ghidra/main/` was not needed as a fallback.
- Strings/floats: `0x80301BE8` = `"Wm_ob_cmnspark"`; the eight `.sdata2` words
  above were read directly from the ELF image.

## Verification

`work/wave203/laneA_block.rs` was appended to a temporary copy of `src/`,
`Cargo.toml` and `Cargo.lock`, and `cargo check --lib` exited `0` with 0 errors
and no warning pointing at the appended lines (the 23 warnings are pre-existing
ones in `src/ui/menu.rs` and elsewhere).
