# Wave 177 lane B coverage (`game::bases`)

Four bodies translated (1,776 bytes) from REL `d_basesNP`; nothing deferred.
All four were read from the raw instruction stream and from the Ghidra export.
No edits were made to `src/game/bases.rs`; the bodies are in
`work/wave177/laneB_block.rs`.

| Address | Bytes | Body |
| --- | --- | --- |
| `0x807A28B0` | 440 | `fn_807a28b0` - firepillar burst counter, step bounce and `0x80833020` state dispatch |
| `0x80628450` | 444 | `fn_80628450` - copies ten per-slot model transforms into their `+0x244` targets |
| `0x807A3D00` | 444 | `fn_807a3d00` - loads `I_yoshi_egg` BRRES, `wait` clip and burst animation |
| `0x8062C0B0` | 448 | `fn_8062c0b0` - arranges the four multi-course-select player panes |

## Notes taken while translating

- `0x802DCD74` / `0x802DCDC0` are the `_savegpr_27` / `_restgpr_27` compiler
  helpers, not semantic calls. Ghidra prints `FUN_80628450(void)`,
  `FUN_8062c0b0(void)` and `FUN_807a3d00(void)`, but `r3` is the incoming
  argument; all three bodies take `this`/`actor`. The helpers are not emitted.
- `FUN_807A28B0`: Ghidra's comparison structure is inverted for the first
  `fcmpo` (`bc 4,1` is `ble`, `bc 4,0` is `bge`, not `bgt`/`blt`). The raw words
  give: post-call step `== 0` and pre-call step `> 0` -> target `-0.0075`;
  pre-call step `< 0` -> counter decrement, sound `0x309`, target `+0.0075`;
  pre-call step `== 0` -> nothing. The `0.18` window test then follows
  Ghidra's `(0.18 <= old || new < 0.18) { if (0.18 < old && new <= 0.18) }`
  otherwise-branch shape, which matches the raw `cror` folds.
- `FUN_807A3D00`: the Ghidra prologue (`in_f31`/`in_ps31_1`, `ldexpf`) is
  compiler noise - the body has no `psq_l`, only the `f31` spill. The seven
  adjacent stack words at `r1+0x08..0x24` are passed to the NW4R adapters
  through overlapping pointers, so the Rust uses one seven-word scratch array.
- `FUN_8062C0B0`: `(present - 1) * 16` is masked with `0xFFFFFFF0`, so a zero
  present count wraps to the `0x807C7A98` row; the Rust keeps the wrapping
  arithmetic. The two `FUN_8062ba30` calls in the all-present loop are the same
  pure table read (`0x807C7A18`), so one call is reused.
- Constant values read from the ELF: `0x807D0300 = 0.0075`,
  `0x807D0324 = 0.18`, `0x807D0388 = 0.0`, `0x807D0418 = 0.0`,
  `0x807D0420 = 1.0`, `0x807D0430 = 0.0`, `0x807C79D8 = 0.0`,
  `0x807C7A00 = [0,1,3,2]`, `0x807C7AA8` rows
  `[4,35,35,35] [5,6,35,35] [7,8,9,35] [0,0,2,3]`.

## Deferred

None.
