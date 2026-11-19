# Coverage - Wave 179, lane B (`game::bases`, three 496-byte bodies)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 3 | 1,488 | 2 | 1 |

All three lane-B addresses are exactly 496 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane B added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave179/laneB_block.rs`.

## Translated

- `0x8060_C400` - `corseClear` course-clear layout loader. `0x800C8CB0`
  loads `corseClear/corseClear.arc` with mode `1`; the 100-byte stack path is
  `corseClear/` + region + `.brlyt`, loaded through virtual slot `+0x14` of the
  layout object at `+0x70`. The region byte from `0x800B4600` is re-read after
  the layout call and selects one of five `corseClear_<region>_inText.brlan`
  pointer slots (`0x807D_3634/36AC/3724/379C/3814`; pointees verified in the
  REL). `A00_inText` is bound through `0x800C90F0` with the three-word table
  `0x807C_6948 = [0, 1, 2]`, the animation id goes to `+0x36C`, and
  `N_marioClear_00` is bound into `+0x370`. The body is word-for-word the
  translated `fn_80640180` shape with `+0x70/+0x78` interface offsets.
- `0x8061_B260` - `gameOver` layout loader. Identical instruction sequence to
  `0x8060_C400` (same offsets `+0x70/+0x78/+0x36C/+0x370`, same
  `0x800046B4` + three `0x802E1A68` path build, same `0x800B4600` switch), with
  `gameOver/gameOver.arc`, `gameOver_`/`.brlyt`, five `gameOver_<region>_inText`
  slots, the `A00_Text` animation, and the `0x807C_6C4C = [0, 1, 2]` table.
  Both bodies return `1`/`0`; the translation returns `bool`.
- `0x806F_0F20` - camera-relative UI projection. Pane `5` supplies the base
  position, offset by `0x807CC0A8[+0x348]` (table `[-8.0, 8.0, 12.0, 20.0]`)
  and `12.0` on Y, then written to `+0x710/+0x714` minus the camera origin
  `+0xAC/+0xB0`. Pane `1` is sampled three times: its `(0.0, 20.0, 0.0)` offset
  is rotated by the pane transform (`0x801C1160`) and averaged 50/50 with the
  pane-5 position into `+0x708/+0x70C`; its plain translation (+`12.0` on Y)
  becomes `+0x700/+0x704`; and it is the reference X for panes `2..=5`, whose
  absolute X spreads fill the four floats at `+0x718`. All constants were read
  from the REL: `0x807CC088 = 0.0`, `0x807CC0B0 = 12.0`,
  `0x807CC0B4 = 20.0`, `0x807CC0B8 = 0.5`.

## Deferred

None.

## Notes and limitations

- `0x800B4600` is the 4-byte thunk to `0x800B4610` in `main.dol`; here it is
  called only via `call0_ret` and its low byte is matched literally against
  `3/2/5/4`, with every other value taking the `EngUS` arm. The REL strings
  make the mapping explicit; the function's documented "game mode" name in
  `stage.rs` does not match a language id, so the translation comments call it
  a *region* byte and do not assert a name.
- The `0x800C8FB0` arguments in both loaders are addresses of pointer slots in
  `.data` (`&PTR_s_...` in Ghidra), not the string bytes themselves. The
  translation passes the slot addresses exactly as the code loads them
  (`addi r4,r31,offset`); the pointee names were confirmed from the REL but are
  not encoded in Rust.
- In `0x806F_0F20` the 52-byte scratch block passed to `0x8016F130` /
  `0x801C1160` matches the frame layout (saved `r29` starts at frame `+0x94`),
  so `[0u32; 13]` is the exact size.
- The Ghidra export renders the per-pane spread as an `fcmpu` plus a
  conditional `fneg`; with `BO = 4` that is a branch-if-false, i.e. the negate
  runs for negative deltas, which is `fabs`. The translation uses `.abs()`.
- Field meanings in `0x806F_0F20` (which destination is "camera-relative X/Y")
  are inferred from the transform calls and marked `// inferred:`; offsets and
  arithmetic are literal. That is why its CSV confidence is `medium`.
