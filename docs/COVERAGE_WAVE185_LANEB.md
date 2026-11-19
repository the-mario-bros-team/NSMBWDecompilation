# Coverage - Wave 185 lane B (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 10 | 976 | 9 | 1 |

All ten claimed bodies from the `0x8073`/`0x8074`/`0x8075` part of
`d_basesNP` were translated; none were deferred. The lane is two child-slot
helpers, two interface dispatch wrappers, two byte-identical deleting
destructors, three angle/sine writers and one small record submitter. Only
existing helpers were used.

## What this pass covers

- **Child slot mark (`0x80734D00`).** Kind byte `actor+0x38C` restricted to
  `1`/`2`; the `+0x6C` virtual of the vtable at `actor+0x60` returns a slot
  pointer, and the sign-extended byte (`lbz` + `extsb r0,r0`) indexes
  `this+0x534`. Uses [`actor_method0`] for the `+0x60` vtable. Same shape as
  the `+0x38C`/`+0x6C` walk inside `fn_80681c50` (that neighbour reads the
  vtable at offset zero, while the raw words here are explicitly `0x60`).
- **Position copy + notify (`0x8073B1B0`).** Stores `(x, y + 8.0, z, 266.0)`
  into `+0x4A8..+0x4B4`, then dispatches slot `+0xC` of the embedded object at
  `+0x4A4` (`lwzu r12,0x4A4(r3)`). Constants re-read from the REL:
  `0x807C_DCAC = 8.0`, `0x807C_DCB4 = 266.0`.
- **Interface dispatch (`0x8073B240`).** Unconditional slot `+0x14` on the two
  embedded objects `+0x430` and `+0x45C`, exactly the `vmethod0(obj, 0x14)`
  shape already used by `fn_806b1830`.
- **Deleting destructors (`0x8073B440`, `0x8073C120`).** Byte-identical to
  `fn_806511d0`: `0x8015F800(this, 0)` for any non-null object, then
  `0x802B9280(this)` when `free > 0` (`cmpi cr0,r31,0` + `bge` over the call).
  Both return `this`.
- **Angle/sine writers (`0x8073F000`, `0x80751C90`, `0x80754620`).** All three
  use the 256-step sine helper `0x80237BD0` with the `1/256 = 0.00390625`
  scale:
  - `0x8073F000` advances `+0x398` by `100`, multiplies the sine by `12.5` and
    writes `scene[+0x7C]` (scene pointer at `0x8042_9E00`); returns `1`.
  - `0x80751C90` runs `0x8006CD40`, advances `+0x568` by `0x300`, and writes
    `camera[+0x8FEC4] + 2.0 * sin` into `+0xB0` (camera pointer at
    `0x8042_9DD0`; `addis +0x9`, `-0x13C`).
  - `0x80754620` masks `this[+0x1A4B8]` to 7 bits, shifts into the angle
    window (`rlwinm r0,r0,9,16,22` = `(x & 0x7F) << 9`), applies the sine and
    scales by `3.0`.
- **Record submitter (`0x807492B0`).** Fills a 20-byte frame block with
  `0x80749310(block, this, param)` (the callee selects a 12-byte entry via
  `this[+0x400]`), then calls
  `0x80064610(0x118, (this[+4] >> 12) & 0x30, block, 0, 0)`.
- **Camera height (`0x8074DB30`).** Signed-int-to-double magic
  (`xoris 0x8000` + `0x4330` + `0x807C_E670 = 4503601774854144.0`), times
  `16.0`, then `fn_8074db10(this)` steps the `+0x3D0` counter and the function
  returns `4500.0 - 16.0 * step`. This is exactly the `call_f32_1(0x8074_DB30,
  actor)` value already consumed by the translated `fn_8074dba0`.

## Verification

- Every body was read from the raw words (`node ppc159.js`) as well as from the
  Ghidra export. Two corrections were needed against the export:
  - `0x8074DB30`: Ghidra prints `FUN_8074db10(dVar1)` with a float argument, but
    `r3` is never clobbered, so the callee receives `this` and reads `+0x3D0` -
    the existing `fn_8074db10(actor)`.
  - `0x8073B1B0`: Ghidra prints the `+0xC` dispatch with no arguments, while the
    three `stfs` at `r1+8..r1+0x10` are the EABI 12-byte by-value position copy.
- Constants re-read from `work/rels/elf/d_basesNP.elf`:
  `0x807C_DCAC = 8.0`, `0x807C_DCB4 = 266.0`, `0x807C_DEE0 = 0.00390625`,
  `0x807C_DEE4 = 12.5`, `0x807C_E66C = 16.0`, `0x807C_E668 = 4500.0`,
  `0x807C_E670 = 4503601774854144.0`, `0x807C_E7C0 = 0.00390625`,
  `0x807C_E7A4 = 2.0`, `0x807C_E8C0 = 0.00390625`, `0x807C_E8C4 = 3.0`.
- The block was type-checked in isolation against stubs of the existing
  helpers (`rustc --emit=metadata`, exit 0).
- Only existing helpers were used (`platform::read_u8/u16/u32/f32`,
  `platform::write_u8/u16/f32`, `platform::call0`, `f32_of_f32`, `vmethod0`,
  `actor_method0`, `abi_call1/2/3/5`, and the translated `fn_8074db10`). No new
  helper or struct was introduced.
- `src/game/bases.rs` was not edited by this lane.

## Deferred

- None.

## Open caveat

- **`0x8073B1B0` (`medium`).** The `+0xC` slot of the `+0x4A4` sub-object takes
  its 12-byte position `(x, y+8, z)` by value in the EABI outgoing parameter
  area (`r1+8..r1+0x10`). Ghidra renders the dispatch argument-less and no
  existing helper passes an aggregate by value, so the body models the receiver
  only (`vmethod0(this + 0x4A4, 0x0C)`), matching the other `lwzu` wrappers. If
  the callee is later shown to read the parameter area, this call is the single
  place that needs an aggregate-aware helper.
