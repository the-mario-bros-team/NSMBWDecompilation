# Coverage - Wave 181, lane A (`game::bases`, sub-300-byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 8 | 836 | 8 | 0 |

All eight lane-A addresses are 88-128 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane A added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave181/laneA_block.rs`.

## Translated

- `0x8060_0B10` (96 bytes) - `+0x18`-teardown delete wrapper for the
  registration family that already contains [`fn_80600990`]..[`fn_80600ad0`].
  Runs `0x8015F9F0(this + 0x18, 0)`, then `0x802B9280(this)` when `free > 0`,
  and returns `this`.
- `0x8060_0B70` (100 bytes) - second copy of the same destructor; the extra
  bytes are the duplicated `cmpi cr0,3,r3,0` at `0x80600B90`/`0x80600B94`.
- `0x8060_1210` (88 bytes) - arrow-state nudge for the character-change select
  screen: starts `0x800C9380(+0x70, 2)` when `+0x250 == 2` and
  `0x800C9380(+0x70, 3)` when `+0x254 == 2`.
- `0x8060_1270` (128 bytes) - exit dispatch. A clear `+0x26B` dispatches the
  confirmed descriptor `0x8082_29D8` through slot `+0x18` of the `+0x208`
  interface vtable, gated on `+0x258 != 0`; a set `+0x26B` stops the layout with
  `0x800C9500(+0x70)` and always dispatches `0x8082_2A18`.
- `0x8060_1300` (128 bytes) - arrow out animation. `+0x258 == 2` runs
  `0x800C9490(+0x70, 2)` + `0x800C92F0(+0x70, 4, 0)`; `+0x258 == 1` runs
  `0x800C9490(+0x70, 3)` + `0x800C92F0(+0x70, 5, 0)`; both paths write
  `+0x26A = 1`.
- `0x8060_1F50` (100 bytes) - third copy of the `+0x18`-teardown destructor,
  next to [`fn_80601eb0`]; same 100-byte body as `0x8060_0B70`.
- `0x8060_5540` (96 bytes) - fourth copy, next to [`fn_80605500`]; same 96-byte
  body as `0x8060_0B10`.
- `0x8060_55A0` (100 bytes) - fifth copy, closing the `0x80605500` block; same
  100-byte body as `0x8060_0B70`.

## Deferred

None.

## Notes and limitations

- The five destructors (`0x8060_0B10`, `0x8060_0B70`, `0x8060_1F50`,
  `0x8060_5540`, `0x8060_55A0`) are one template instantiated five times. The
  differing `bl` words are `0x4BB5EEB1`/`0x4BB5EE4D`/`0x4BB5DA6D`/
  `0x4BB5A481`/`0x4BB5A41D` (`0x8015F9F0`) and `0x4BCB8731`/`0x4BCB86CD`/
  `0x4BCB72ED`/`0x4BCB3D01`/`0x4BCB3C9D` (`0x802B9280`); the role of
  `0x8015F9F0` is recorded as `// unknown:` in the block.
- The redundant `addic. r3,r3,24` guard around `0x8015F9F0` can only fail when
  the whole object is the null page, so it is modelled as an unconditional
  call inside the outer `this != 0` test.
- `0x8060_1270` end with `bcctr` followed by `b 0x806012DC` (`0x48000028`), i.e.
  a normal call plus a jump to the shared epilogue, not a tail call; the
  descriptor arguments `0x8082_29D8` and `0x8082_2A18` come from
  `lis 0x8082` + `addi 10712` / `addi 10776`. Those addresses sit past the
  file-backed end of the ELF (`0x8082_28B8`), in the image's zero-fill tail, so
  they are passed through as address literals and never read - the same
  treatment as the existing `0x8082_2958`/`0x8082_2998` arguments.
- `0x800C9380`, `0x800C9490`, `0x800C9500` and `0x800C92F0` stay address-only
  (`// unknown:`); the indices `2..5` and the `write_u8(+0x26A, 1)` are literal.
- `+0x250`/`+0x254`/`+0x258` and the `+0x26A`/`+0x26B` bytes match the offsets
  already used by [`fn_80601500`] and `0x80603860..0x80604170`, which
  identifies these three bodies as character-change select handlers.
