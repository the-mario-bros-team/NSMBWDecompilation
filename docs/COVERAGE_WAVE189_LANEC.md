# Coverage - Wave 189, lane C (`game::bases`, 128-192-byte band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 1528 | 9 | 1 |

All ten lane-C addresses are 136-192 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane C added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave189/laneC_block.rs`, the ledger is `ledger/agents/wave189-lanec.csv`,
and the bodies are appended by the coordinator through `work/wave189/merge.cjs`.

## Translated

- `0x807227E0` (136 bytes) - trimmed name compare: advances the argument past
  its first `:` through `0x802E1C40`, fetches the object name with vtable slot
  `+0x20`, trims that too, and returns whether `0x802E1AB4` reports equality.
- `0x80723B10` (136 bytes) - byte-for-byte clone of `0x807227E0`; only the `bl`
  displacements to the three helpers differ.
- `0x8072C330` (160 bytes) - rotating fire cannon aim step: the `+0x348` variant
  selects a signed `+192`/`-192` step from `0x807CD4F0`, added to the angle at
  `+0x4A0`; reaching the `+16384`/`-16384` limit at `0x807CD4F4` clamps the
  angle and posts `0x80830958` on the `+0x394` interface.
- `0x8072FBA0` (188 bytes) - initialises the watch actor's target record:
  handle `+0x4E8`, `sqrt(dx*dx+dy*dy)` distance `+0x4EC` (`0x802BE470`), yaw
  difference `+0x4F0`, aim difference `+0x4F2` (`0x801611C0` and `0x807073D0`),
  target yaw `+0x4F4` and selector byte `+0x4F8`.
- `0x807316F0` (144 bytes) - rider character latch for the pipe orbit: with the
  rider in state `1`/`2` and the `+0x9EC` row's `+0x5B0` word carrying bit
  `0x10`, the `0xA4` query passes and vtable slot `+0x6C` supplies the byte
  copied to `+0xA04`.
- `0x80733D90` (136 bytes) - camera-window test: dereferences the scene pointer
  at `0x80429E00` and checks the object's `+0xAC`/`+0xB0` against the scene
  spans at `+0x38`/`+0x40`/`+0x3C`/`+0x44` widened by `16.0`
  (`0x807CD7F0`); returns `1`/`0`.
- `0x80734170` (148 bytes) - fire cannon burst timer: decrements `+0x63C`, and
  on zero runs `0x80733B00` or [`fn_80733c30`] by `+0x640` when inside the
  window, always posting `0x80830EA0` on `+0x394`.
- `0x80734340` (192 bytes) - perch tilt oscillation: index
  `(2*+0x634 + u8(+0x348))*2 & 0x1FE` drives the `0x807CD890` step and, inside
  `-0x2000..=0x2000`, the `0x807CD7E0` snap plus `0x80830E60`; `+0x646`
  mirrors the tilt as `0x8000 - tilt` for variant 1 and `-tilt` otherwise.
- `0x8073B290` (152 bytes) - destructor: tears down the `+0x488`, `+0x45C`,
  `+0x430`, `+0x3F0` and `+0x3D0` sub-objects, runs the base destructor
  `0x800671B0(object, 0)` and frees through `0x80162920` when `free > 0`;
  returns `object`.
- `0x80743480` (136 bytes) - trimmed name compare clone of `0x807227E0`.

## Deferred

None. All ten addresses were translated.

## Notes

- `0x80733D90` reads the scene through the pointer stored at `0x80429E00`
  (`lwz r5, 0xFFFF9E00(r5)` then `lfs ...,0x38(r5)`), matching the raw words.
  Some older bodies in `src/game/bases.rs` add the offset directly to
  `0x80429E00`; that form is not used here.
- `0x8072C330` compares the two `0x802DE514` keys with the raw unsigned `cmp`;
  the helper's exact transform remains `// inferred:` from the compare/clamp
  shape already documented for [`fn_806eae50`].
