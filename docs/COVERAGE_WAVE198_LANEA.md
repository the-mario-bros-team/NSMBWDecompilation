# Wave 198 lane A coverage (`game::bases`)

Slice: the three largest remaining bodies (`0x80605BA0` .. `0x8065CB20`,
516-3788 bytes). Three translated, none deferred.

| address | size | body |
| --- | --- | --- |
| `0x8060_5BA0` | 1164 | `fn_80605ba0` - clears the row-pane visibility bits of both pane banks and re-shows the two slots the selected builder needs |
| `0x8061_F2F0` | 516 | `fn_8061f2f0` - re-arms the results screen animation and switches the winner panes by sheet/occupancy |
| `0x8065_CB20` | 3788 | `fn_8065cb20` - samples the stage tile grid around a point, marks nearby cells, picks tile variants and emits spawn triples |

## Signature recoveries

* `0x8065CB20` - `bl 0x802DCD40` is `_savegpr_15`; Ghidra modelled its return as
  the array base (`iVar11`). The real base is `r3` (`this`), and the live
  arguments are `r3` (grid), `r4` (signed output cap), `r5` (triple buffer),
  `f1`/`f2` (sample x/y) and `f3` (radius and grid scale). The Ghidra
  `param_1`/`param_4`/`param_5` split is an artifact of that helper.
* `0x80605BA0` - its `mode` ladder is `cmpi r4` against `2,0,1,5,3,4` in that
  order; `index` is signed (`cmpi r5,7` and the negative-index prologue guards).
* `0x8061F2F0` - no register-helper ambiguity; the `+0x2C0` sheet pointer gate
  and the `fn_8061ee50` occupancy test are the two branch keys.

## Decode notes

* `0x80605BA0` - the compiler emitted the two "hide" sweeps with an 8-byte
  (two-slot) stride: `9-index >> 1` and `5-index >> 1` are the iteration counts
  and the unrolled bodies step `+0x40` (16 slots). The literal two-slot stride
  is preserved. The sign-overflow prologue guards (Ghidra's `bVar2`..`bVar6`)
  were proved dead and elided.
* `0x8061F2F0` - pane flags are the same `& 0xFE` / `| 1` bit 0 toggles as the
  neighbouring row builders; the `0x800C93D0` loop runs frames `3..6`.
* `0x8065CB20` - the 200-line Ghidra dequantise prologue is the `psq_st`
  register save with GQR0 = 0, so it is dead; the real constants were read from
  the REL as `0x807C8AF4=1.0`, `0x807C8B00=0.0625`, `0x807C8B04=8.0`,
  `0x807C8B08=16.0`, `0x807C8B0C=5500.0`, `0x807E7034=64.0`,
  `0x807E7294=0.5`, and the `0x807C8AE8` slot is `0.0`. The `0x4330`
  int-to-float magics (`0x807C8AF8`, `0x807C8B10`) were folded to `as f32`,
  which is identical (round-to-nearest) for the small/u16 values involved.
  The tile/variant tables are read at runtime from `0x807E703C` (43 entries of
  12 bytes) and `0x807E7240` (7 entries), not baked in.
* `0x8065CB20` pass 3 iterates the grid transposed relative to pass 1/2 (ids
  with `i*0x50 + j*2`, flag bytes with `i*12 + j*0x1E0`); that transposition is
  literal in the raw words, not a transcription slip.

## Deferred

None. There are no unproven address ranges: every branch target, table stride,
constant, truncation and loop bound in all three bodies is backed by the raw
words; only the engine-call roles (tile lookup `0x800775A0`, painter
`0x80077860`, camera queries, `0x800925F0` marker submit) are inferred from
call shape, which is why `0x8065CB20` is rated `medium` rather than `high`.
