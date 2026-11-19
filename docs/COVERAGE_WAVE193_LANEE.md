# Wave 193, lane E coverage (`game::bases`)

Lane band `0x80706280..0x8071DBF0`, all inside REL `d_basesNP`. Ten of ten
requested bodies translated; none deferred.

## Bodies

| Address | Bytes | Function | Summary |
| --- | --- | --- | --- |
| `0x80706280` | 224 | `fn_80706280` | Proximity snap: when the second object's `+0x38C` state is 1 or 2, its x is within `0.5 * +0x404`, and its height is at least `+0xB0 + 0.2 * +0x408`, run `fn_80706360` and, on success, `fn_80706410`. |
| `0x807074B0` | 236 | `fn_807074b0` | Pendulum angle step: when neither input mask matches, advance the signed `+0x104` by the sign-extended `+0x3D0`; snap to `+0x3DC` when the `0x802D_E524`/`0x802D_E514` remaining-distance test passes and cue sound `3` for `+4` bit 8. The masked arm posts pane message `0x8082_F420`. |
| `0x80707800` | 224 | `fn_80707800` | Recomputes `+0x3D0` from the `+0x3D8` amplitude (`<<12`, negated on the masked arm and when `+0x3E1` is set), clamps it with the `0x802D_E524` magnitude, and advances `+0x104` by `+0x3D0 >> 12`. |
| `0x80708070` | 240 | `fn_80708070` | Decrements the `+0x3E6` cooldown, sets `+0x3E8` from `0x8070_7F40`, and when unblocked, off cooldown, gated by `+0x3E4` and with `+0x3E8` clear, posts a `(x-8.0, y+8.0, 2, 0, 2,0,1)` effect record through `0x8008_82E0`. |
| `0x8070F9C0` | 232 | `fn_8070f9c0` | Snaps the `f1` phase to the step grid anchored at `*r3`: zero step returns the anchor, otherwise `f1 + step*trunc((anchor-f1)/step)` is walked with the two sign-dependent `while` windows. |
| `0x80711F60` | 204 | `fn_80711f60` | Opens `+0x394`, loads `obj_kinoko` / `g3d/obj_kinoko.brres` into `+0x3B0`, binds the `variant`-th 24-byte clip row into `+0x3B4`, and closes the pane. |
| `0x80713250` | 244 | `fn_80713250` | Copies the `[[+0x6D0]+0xC]` halfword to `+0x6D4`/`+0x6D6`, derives the `+0x6D8` swing scalar from one `sin(angle/256)` sample, then writes `+0xB0 = state[0x8082_F8C4] + (swing-160.0) + (32.0-\|swing\|)*sin(angle/256)`. |
| `0x8071BA20` | 236 | `fn_8071ba20` | Seeds `+0xB4=420.0`, calls `fn_8071b8e0((+4 & 0xF)*8)` and `fn_8071b990`, writes `+0xD0=(+4&0xF+2)*8`, `+0xD4=-8.0`, `+0xD8=0.0`, and applies two `0x8004_5F40` vectors. Returns `1`. |
| `0x8071BD20` | 216 | `fn_8071bd20` | Resizes the `+0x4C4` area with `(f32, (+4&0xF)+2, ((+4>>4)&0xF)+2, 0, 4)` through `0x800B_FC60`, stores the `(+4>>24)&3` variant at `+0x3DD`, applies a `0x8004_5F40` vector and calls `fn_8071bc50`. |
| `0x8071DBF0` | 204 | `fn_8071dbf0` | Zeroes `+0xB0`, then spawns five `0x1C`-flag effects at `state[0x38] + state[0x40]*rand()` through `0x8007_8410`, and clears `+0x1A488`/`+0x1A59C`. |

## Exact signature of `0x8070F9C0`

The export declares a `double` first parameter because the REL has no
prototype, but the raw words carry a single-precision `f1` input that Ghidra
folds into that `double`. The body starts with `lfs`/`fsubs`/`fadds` and
returns through `bclr` with the result in `f1`, so the prototype is:

```rust
pub unsafe fn fn_8070f9c0(f1: f32, anchor: usize, step: usize) -> f32
```

`r3` is the grid anchor pointer (`*param_2`) and `r4` the step pointer
(`*param_3`). The already translated caller `fn_8070f7c0` reaches it through
`call_f32_u32x2_f32(0x8070_F9C0, phase, 0x8080_5DF0, &mut step)`, which matches
this shape.

## Ghidra corrections made against the raw words

- `0x80707800`: the masked arm is `neg` then `<<12` (`rlwinm 12,0,19`), and the
  `+0x3E1` flip is a second `neg`; the implementation keeps both in that order.
- `0x8070F9C0`: the `bc 4,1` at `0x8070_FA24` takes the **negative-step** arm
  when the final `fcmpo` is not `GT`, so the two `while` pairs are the reverse of
  the C text's arm assignment. The raw branch order is implemented.
- `0x807074B0`: the mask test is `(g[0] & +0x358) | (g[1] & +0x35C)` after the
  `and`/`or` words at `0x8070_74E4..0x8070_74EC`; the C's two `&&` tests are
  equivalent.
- `0x8071BD20`: `0x802DCD74` is `_savegpr_27`, not a call returning `this`; the
  incoming `r3` is the actor. The trailing `0x802DCDC0(1)` is `_restgpr_27` and
  is not emitted.
- `0x8071DBF0`: `0x800B2EF0` is the argument-less random-float helper; the
  `lwz r3,[0x8042_9E00]` feeds the `+0x38`/`+0x40` loads and the loop does not
  reload `r3` before its `bl`.

## Deferred

None.
