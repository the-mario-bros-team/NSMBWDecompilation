# Wave 211 lane D coverage (`game::player`)

Slice: the ten `game::player` bodies at `0x80064A50`, `0x80064AF0`,
`0x80064B00`, `0x80064BD0`, `0x80064BE0`, `0x80064BF0`, `0x80064C00`,
`0x80064D10`, `0x80064DA0` and `0x80064F50`. All ten are translated; nothing was
deferred.

Bodies live in `work/wave211/laneD_block.rs` and are appended to
`src/game/player.rs` by the merge step; ledger rows are in
`ledger/agents/wave211-laneD.csv`.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x80064A50` | 156 | `fn_80064a50` | Wraps two angles through `0x80101AA0(object, angle)` (`r3` plus `f1`, the second call fed by `fmr f1,f30`), differences them, and gates the difference against `*(f32 *)(*(u32 *)0x80429DD0 + 0x8FEA0) * 0.5` (`0x8042BB3C`). The test is deliberately asymmetric: `fcmpo` against `0.0` + `bc 4,0` sends `diff >= 0` to a `CR0[GT]` extraction (`mfcr` + `rlwinm r3,r3,2,31,31`), while the negative half negates the tolerance and runs `CR0[LT]` (`rlwinm r0,r0,1,31,31`) through `cntlzw` + `>> 5` to get `!LT`. |
| `0x80064AF0` | 12 | `fn_80064af0` | `lwz r12,-0x5458(r13)` / `mtspr CTR,r12` / `bcctr`: a tail jump through the SDA backend word `0x8042A228`. Every call site in the image jumps there with three argument registers (`out`, `in`, `mode`), so the wrapper is modelled as a three-argument forward. |
| `0x80064B00` | 4 | `fn_80064b00` | One `b 0x80101BC0`. That thunk stores `table[0x802F4A18 + index*4]` into `0x8042A228`; the table is `{0x80101BE0, 0x80101BE0, 0x80101BF0}` - modes `0`/`1` install the bare `blr`, mode `2` installs the point push-out routine. |
| `0x80064BD0` | 8 | `fn_80064bd0` | `li r4,0` + `b 0x800B4170`: forwards `(handle, 0)` to the handle-broadcast walker that starts at SDA list head `0x80427AF8`. |
| `0x80064BE0` | 8 | `fn_80064be0` | `li r4,0` + `b 0x800B3F50`: same shape, list head `0x80427AC8`. |
| `0x80064BF0` | 8 | `fn_80064bf0` | `li r4,0` + `b 0x800B42B0`: same shape, list head `0x80427B28`. |
| `0x80064C00` | 8 | `fn_80064c00` | `li r4,0` + `b 0x800B4050`: same shape, list head `0x80427AD8`. |
| `0x80064D10` | 144 | `fn_80064d10` | Calls `0x80162510(object)`, then loads the byte pointer at `+0x34C` and the halfword pointer at `+0x350` and bails if either is null. `flag == 0` clears bit `0` of the byte; otherwise it sets bit `0x08` and stores `300` (`li r0,0x12C`) into the halfword. |
| `0x80064DA0` | 432 | `fn_80064da0` | Hit-region probe. Stage record `0x80429E98 + (*(u8 *)(*(u32 *)0x8042A1C8 + 0x120E)) * 0x3B0` (nulled when its `+0x2C` word is `0`), shape from `0x8008E410(record, owner)`, four `u16` fields read through the `0x43300000` double conversion, then four band rejections against `+0x330`/`+0x334` with the `2.0` thickness scale `0x8042BB40`. |
| `0x80064F50` | 300 | `fn_80064f50` | The composite test: `+0x36C == 2` gate, `0x80070CA0(+0x1EC)` flag gate skipped by `flags & 8`, box `{+0x320, +0x324, 0.5*+0x318, 0.5*+0x31C}` into `fn_80064da0`, second box `{+0x338..+0x344}` into `0x800B56E0` when `flags & 4` is clear, and `fn_80064d10(object, *(u8 *)(object + 0x390))` when a hit was recorded and `flags & 2` is clear. |

## Sizes and extents

- `0x80064A50`: the lane brief said 16 bytes; the function table and the code
  both say 156 (prologue at `0x80064A50`, `blr` at `0x80064AE8`, padding at
  `0x80064AEC`). The `# Original` block keeps the function-table value.
- `0x80064F50`: the function table says 300 but the `blr` is at `0x80065078`, so
  the real extent is 296 bytes and `0x80065080` is the next body. The `#
  Original` block keeps the function-table value, as required for the lane check.
- The other eight entries match their prologue+`blr` extents exactly (12, 4,
  8, 8, 8, 8, 144 and 432 bytes; the four eight-byte entries are `li`+`b` tail
  jumps, `0x80064B00` is a single `b`, and `0x80064AF0` is a `lwz`/`mtspr`/
  `bcctr` triple).

## Notes on the raw words

- `0x80064A50`: `fcmpo` compares `f3` with `f0` (`0xFC030040`), so the branches
  read `diff >= 0` (`bc 4,0` = branch when `CR0[LT]` is clear) and `CR0[GT]` /
  `CR0[LT]` extractions use `rlwinm` shifts `2` and `1`. `cntlzw` of the extracted
  bit followed by `rlwinm r3,r0,27,5,31` (`>> 5`) inverts it, which is the same
  `clz(diff) >> 5` equality idiom this module already uses in `fn_8005b610`.
  Ghidra's `ldexpf`/`unaff_GQR0` noise is the `psq_st`/`stfd` prologue pair for
  `f30`/`f31`, not real code.
- `0x80064A50` asymmetry is literal: the non-negative half returns
  `diff > tolerance`, the negative half returns `diff >= -tolerance`. Ghidra
  prints the same two comparands in the opposite order; the instruction order
  (`fcmpo cr0,f3,f0`) and the `bc` directions were re-checked against the raw
  words, not taken from the C.
- `0x80064AF0`: Ghidra's `unaff_r13 + -0x5458` is the SDA word `0x8042A228`
  (`r13 = 0x8042F680`). The `bcctr` is a jump, so the three argument registers
  are forwarded untouched; `fn_80064af0` takes them as `out`, `in`, `mode`
  because the eleven call sites in the export all pass `(pointer, pointer|0, 1)`.
- `0x80064B00`: the four-byte body contains no code at all beyond the branch;
  the target thunk was disassembled to recover the table and its three entries.
- `0x80064BD0`/`BE0`/`BF0`/`C00`: identical two-instruction bodies; Ghidra
  models the walkers as `void`, but a tail jump forwards `r3`, so each wrapper
  returns the callee's `u32` instead of dropping it.
- `0x80064D10`: the export re-tests the `+0x34C` pointer inside both arms even
  though the outer guard already proved it non-null; the dead tests are omitted
  and noted in the doc block. The `+0x350` halfword is guarded only in the
  non-zero arm, which matches the `bc 12,2` at `0x80064D7C`.
- `0x80064DA0`: the four `u16` shape fields are at `+0x00`, `+0x02`, `+0x04`,
  `+0x06` (`lhz r4,0x0` / `lhz r0,0x2` / `lhz r0,0x4` / `lhz r0,0x6`), not
  four-word records: `grep`ing the `0x43300000` stores shows each halfword
  widened on its own. The `+0x0C` sub-object bit test is the first branch taken
  after the shape null check, so the `(record + 0x0C)` read only happens when a
  shape was resolved.
- `0x80064DA0` `bc` audit: `bc 12,0` at `0x80064EBC` returns `1` when
  `x + 2*B[2] < -H[0]`, `bc 4,1` at `0x80064EDC` falls through to `1` when
  `x > shape[2] + H[0]`, `bc 12,1` at `0x80064EFC` returns `1` when
  `y - 2*B[3] > H[1]`, and `bc 4,0` at `0x80064F20` returns `0` when
  `y >= -(shape[3] + H[1])`.
- `0x80064F50`: the two box blocks are written into the stack at `r1+0x18`
  (`{+0x320, +0x324, 0.5*+0x318, 0.5*+0x31C}`) and `r1+0x08`
  (`{+0x338, +0x33C, +0x340, +0x344}`); the call order is
  `fn_80064da0(object, object+0xAC, first, owner)` and then
  `0x800B56E0(object+0xAC, first, second, owner)`. Both scale factors are the
  same `0x8042BB3C` half constant, loaded once at `0x80064FB4`.

## Reused neighbours

- Existing module constants: `PLAYER_ARRAY_8042A1C8` (`r13 - 0x54B8`),
  `W18_STAGE_TABLE_80429E98` (`r13 - 0x57E8`), `W97_EFFECT_POS_80429DD0`
  (`r13 - 0x58B0`), `W92_HIT_TEST_800B56E0`, `FN_80162510_B`.
- Helpers: `platform::{read_u8, read_u16, read_u32, read_f32, write_u8,
  write_u16, call1, call2, call3, call4}`, plus `extern "C" fn(u32, f32) -> f32`
  for the float-returning `0x80101AA0` and `extern "C" fn(u32) -> u32` for
  `0x80070CA0`, following the module's existing `core::mem::transmute` style.
- No constant or helper already in `src/game/player.rs` was redefined; every new
  constant carries the `_D` lane suffix.

## Verification

Scratch copy at `%TEMP%\opencode\w211laneD` (`src/`, `Cargo.toml`,
`Cargo.lock`) with this block appended to `src/game/player.rs`:
`cargo check --lib` exits `0` with no diagnostics in the appended lines.

## Deferred

None.
