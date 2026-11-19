# Coverage - Wave 180, lane A (`game::bases`, the GQR/`psq` tail)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 2 | 632 | 2 | 0 |

Both lane-A addresses are 308 and 324 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane A added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave180/laneA_block.rs`.

## Translated

- `0x806E_DE70` - sub-object curve tracker and sound cue. Walks
  `*(this+0x1024)`'s `+0xEC` vtable slot `+0x28`, then queries the returned
  object for tag `0x80358E50` on slot `+0x10`. When that binding is absent it
  samples `0x800C1530(obj+0x80)` as an unsigned halfword, feeds
  `sample * 0.00390625` through `0x80237BD0`, adds `curve * -0.03125` to the
  `+0x1384` scalar, and clamps it to `[0.5, 8.0]`. It stores the scalar back
  and emits event `0x232` via `0x80198870` with `(i16)(256.0 * scalar)`. Pool
  reads: `0x807CBF44 = 0.00390625`, `0x807CBF74 = 0.03125`,
  `0x807CBF7C = 0.5`, `0x807CBF70 = 8.0`, `0x807CBFA8 = 256.0`.
- `0x8064_DB70` - four-direction terrain probe. Lazily seeds the global table
  at `0x8082_81C8` with `(0,-12) (0,12) (-12,0) (12,0)` from
  `0x807C86A8 = 0.0`, `0x807C86C8 = -12.0`, `0x807C86CC = 12.0`, sets
  `0x8082_81C0 = 1`, then scans the four offsets around `this+0xAC` through
  `0x80070BF0(x, y, layer[+0x38F])`. On `((kind >> 16) & 0xFF)` equal to
  `4`/`5` it copies two raw pool words (`0x807C86B8 + (kind&2)*2`,
  `0x807C86C0 + (kind&1)*4`) into `out[0..1]`, writes the probe point and
  calls `fn_8064dcc0(this, out+8, out+0x10, &[x, y])`, returning the direction
  index; otherwise it returns `-1`.

## Deferred

None.

## Notes and limitations

- GQR3 = `0x00050005`; the `psq_l f1,...,1,3` in `0x806E_DE70` therefore loads
  one **unsigned 16-bit** sample, modelled as `sample16 as f32` (no scaling).
- GQR0 = `0`; every `psq_st`/`psq_l ...,0,0` in both bodies is an `f32` frame
  save/restore, and the `unaff_GQR0`/`ldexpf` blocks in the exports are the
  compiler's scalar-save guard noise - not modelled.
- `0x8064_DB70` calls `0x802DCD74` and `0x802DCDC0`: these are the
  `_savegpr`/`_restgpr` compiler helpers, not semantic calls. Ghidra's
  `uVar10 = func_0x802dcd74()` (split into the two argument words) is that
  helper's preserved `r3`/`r4`; the translation names them `this` and `out`.
- Clamp polarity in `0x806E_DE70` was re-read from the raw words: `fcmpo` +
  `bge` (`0x4080_000C`) against `0x807CBF7C = 0.5`, then `fcmpo` + `ble`
  (`0x4081_0008`) against `0x807CBF70 = 8.0`, i.e. `clamp(v, 0.5, 8.0)`.
- The numeric role of `0x80237BD0` is `// unknown:`; it is a literal
  `f32 -> f32` call (its `f1` result is immediately multiplied by `-0.03125`).
- Every opcode-59 form in the two bodies was decoded with the scratch
  decoder: all are two-operand `fadds`/`fsubs` (fB at bits 11-15) or `fmuls`
  (fC at bits 6-10) with single-precision `lfs` operands, so the reads are
  plain `f32`.
