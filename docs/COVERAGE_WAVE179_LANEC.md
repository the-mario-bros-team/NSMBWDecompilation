# Coverage - Wave 179 lane C (`game::bases`, 500-byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 3 | 1,500 | 3 | 0 |

All three bodies translated, none deferred.

## Lane C (3 bodies, 1,500 bytes)

- `0x8070F7C0` - (500) intensity-ramp driver for the world-map witchcraft /
  kameck boss class: gates on `0x80095240`, runs the four-mode machine on the
  REL BSS block at `0x8082F760` (mode `0x8082F76C`, intensity `0x8082F770`,
  phase `0x8082F774`, counter `0x8082F778`) through the shared ratio helper
  `0x806601B0`, accumulates the phase while the `+0x694` idle counter is zero,
  re-snaps it with `0x8070F9C0` at step `16.0 * 0x808063A8`, and publishes
  `intensity * 0x808063B0` into `+0xE8`. Same class and same `+0x694`/`+0x1610`
  fields as the already-translated `fn_8070fab0` and `fn_8070fe40`.
- `0x807A2CA0` - (500) firepillar two-emitter burst phase: sibling of
  `fn_807a2ac0` with the effect/sound order swapped and the apex from
  `fn_807a24e0` instead of the `+0x360` phase bytes. Drives the `+0x3F0` model
  virtual, re-triggers `Wm_ob_firespillar01`/`02`, queues sound `0x30A`, adds
  the `+0x4A0` step to the `+0xE0` height, swaps the `+0x4A8` step sign at `0.0`
  (decrementing the `+0x4BC` counter) or at the per-size limit from the
  eight-float `0x807D0338` table, and dispatches `0x808330A0` at `+0x394` when
  the counter expires.
- `0x807B7540` - (500) world-map screen-state selector: one-shot latch on
  `0x80429010` (`!= 6` -> `0x800B0DB0(old)`, then `6`), then picks sub-state
  `1..5` from `0x8042A204`, `0x8042A21C`/`0x8042A21F`/`0x8042A220` and the
  player bookmark/row bytes `+0x120D`, `+0x120E`, `+0x120F`, `+0x1211`, using
  the `[0x80429E98] + index * 0x3B0` stage table with the same
  `0x8008E410`/`0x8008E3D0` row lookups as `fn_807b23b0`/`fn_807b84c0`.

## Deferred

None.

## Notes

- `0x8070F9C0` (232 bytes) is not part of this lane and is called by address.
  Its export mis-models the first argument: the callee subtracts an `f1` that
  Ghidra dropped from the prototype, while the caller leaves the accumulated
  phase in `f1` (`fadds f1,f3,f4` immediately before the `bl`). The Rust body
  passes it explicitly through `call_f32_u32x2_f32`. The `^0x80000000` /
  `0x43300000_80000000` pair at `0x807CCA90` is the signed-int-to-double
  conversion, so `0x808063A8` (48) becomes `48.0` and the snap step is
  `16.0 * 48.0`.
- `0x800B0DB0` remains unclaimed (`// unknown:`); the existing tree documents it
  as the scene-screen state setter/releaser, and this lane only forwards
  integers to it.
- BSS-only strings/descriptors `0x808330A0`, `0x8081C630`, `0x8081C61C` are
  used as raw address literals, matching the surrounding translations.
