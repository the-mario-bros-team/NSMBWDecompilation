# Coverage - Wave 179, lane A (`game::bases`, three 492-byte bodies)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 3 | 1,476 | 3 | 0 |

All three lane-A addresses are exactly 492 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane A added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave179/laneA_block.rs`.

## Translated

- `0x8068_C210` - `gabon_rock` throw re-arm. Ghidra drops the second argument
  (`r4`); `lbzx r0,r3,r30` at `0x8068C340`/`0x8068C39C` indexes the character
  table `0x8042C1A0`, and both callers (`0x8068BE0C`, `0x8068BEDC`) pass
  `mr r4,r30`, where `r30` is the `0`/`1` flag built at `0x8068BDB8` from the
  comparison of the two objects' `+0xAC` and also stored at `+0x484` by
  `fn_8068c140`. If the `0x8082ADF0` state is unbound it plays sound `0x2C8`,
  then writes `+0xE8/+0xEC/+0xF4` and (in the `0x8082ACF0` arm) `+0x114`, and
  finishes with mode `3` at `+0x48C`. Constants read from the pool at
  `0x807C9A70`: `0.0`, `1.8`, `1.0`, `0.85`, `0.35`, `1.5`, `-0.1`, plus the
  signed int-to-double magic `0x807C9B38 = 4503601774854144.0`.
- `0x806F_64B0` - `MG_balloon` model/animation loader. Word-for-word the
  already-translated `fn_807a3d00` (`I_yoshi_egg`) shape: `0x800690C0` resource
  accessor at `+0x560`, `g3d/MG_balloon.brres` into `+0x57C`, model at
  `+0x580`, clip at `+0x5C0` selected by bit `4` of `+0x04` (`wait_2` when set,
  `wait` otherwise), the `MG_balloon` animation handle at `+0x52C`, the second
  animation at `+0x530`, then blend = float of `+0x524` and `0.0`
  (`0x807CC228 = 1.0`, `0x807CC218 = 0.0`, magic `0x807CC230`).
- `0x8070_03C0` - `boss_koopaJr` electric-start. Spawns four
  `Wm_jr_electricstart` effects (`0x80803A70`) from `y - 104.0` to `y + 104.0`
  (`0x807CC4D0 = 8.0`, `0x807CC4D4 = 112.0`, `0x807CC4D8 = 208.0` over a
  clamp-at-1.0 `3.0`), with x shifted by `character * 13.5` (`0x807CC4AC`) and
  the third halfword flag `0x7FFF` when `+0x348 == 1`; then resolves
  `boss_koopaJr_toge` (`0x80803A5C`) on `+0x540` and binds it into each of the
  four `+0x544 + i*0x40` objects through virtual `+0x18`.

## Deferred

None.

## Notes and limitations

- `0x806F_64B0` and `0x8070_03C0` both load a `0x43300000` word and follow it
  with an `fsubs` (single-precision) subtract of a magic constant loaded by
  `lfd`. The literal reading is nonsense in single precision, so - as in the
  already-accepted `fn_806843b0` and `fn_8068c450` - the conversions are modelled
  as `int as f32` (via the explicit `f64::from_bits(0x4330_... | raw)` idiom
  where the surrounding code is otherwise float, `as f32` where the value is
  immediately multiplied by single-precision constants). The three sources are
  `xoris`-based `(int)` values: the literal `3` and the loop index in
  `0x8070_03C0`, the character byte in both, and the signed word at `+0x524` in
  `0x806F_64B0`.
- Ghidra's local `f28..f31` spills in `0x8070_03C0` (`psq_st`/`psq_l` with
  GQR0 plus `stfd`/`lfd`) are the frame slots `r1+0x50..0x88`, not parameters;
  the export's whole `ldexpf`/`unaff_GQR0` prologue is that save noise
  (the same pattern the file documents for `fn_806f25b0`).
- Ghidra also drops the `r3` argument of `0x8070_03C0`, because the `0x802DCD6C`
  save helper is rendered as a call returning `r3`; the translation names it
  `this`.
- `0x8082ACF0`, `0x8082AD70` and `0x8082ADF0` are zero-initialised state
  descriptor records, not strings, so they are passed as raw addresses.
- Field roles (`+0xE8/+0xEC/+0xF4` as the velocity triple, `+0x48C` as the mode
  word, `+0x348` as the character/player byte) are `// inferred:` from
  `fn_8068c030`, `fn_8068c400` and `fn_8068c450`; every offset, store and branch
  in the three bodies is literal.
