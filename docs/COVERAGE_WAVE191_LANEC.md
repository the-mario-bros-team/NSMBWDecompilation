# Wave 191, lane C coverage (`game::bases`)

Lane band `0x807BA860..0x807BEA40`, all inside REL `d_basesNP`. Five of five
requested bodies translated; none deferred.

## Bodies

| Address | Bytes | Function | Summary |
| --- | --- | --- | --- |
| `0x807BA860` | 184 | `fn_807ba860` | Guarded one-shot init: if `[0x8042A24F] == 0`, clears the twenty words `0x80374880..0x803748CC` and calls `fn_807bac40(this, i)` for `i = 0..9`. |
| `0x807BC220` | 156 | `fn_807bc220` | Adds `0.2` to the selected sub-object's `+0x2A0` timer; on `sum >= 1.6` calls the `+0x20C` interface slot `+0x18` with `0x808341B8` and stores `1.6`. |
| `0x807BE3D0` | 184 | `fn_807be3d0` | Symmetric ease through `fn_807be230`: `t >= 0.5` mirrors to `2*(1-t)` and blends `0.5 + 0.5*(1-g)`, otherwise `2*t` and `0.5*g`; result `blend*(1-s) + t*s`. |
| `0x807BE6B0` | 160 | `fn_807be6b0` | Teardown: clears `[0x8042A284]`, runs `0x802DCA98(+0x6C8, 0x80601CD0, 12, 4)`, invalidates `+0x158`/`+0x13C` with `-1`, calls `0x800F2770(this, 0)` and, for `flag > 0`, `0x80162920(this)`; returns `this`. |
| `0x807BEA40` | 176 | `fn_807bea40` | Renders the `+0x4A8` record and constant record `1` into `+0x4F8` through `0x80164B20` / `0x80253E20` / `0x802540B0` / `0x800FE030`; returns `1`. |

## Exact signature of `0x807BE3D0`

`0x807BE3D0` is already referenced by the translated `fn_807bfe20`, so its
prototype matters:

```rust
pub unsafe fn fn_807be3d0(t: f32, q: f32) -> f32
```

The second float argument is real even though Ghidra prints one parameter:
neither branch writes `f2` before `bl 0x807BE230`, because `fn_807be230`
starts with `fmr f30,f2` and uses it as the curvature `q`. The shipped call
site at `0x807BFECC` loads that argument with `lfs r2,0xB8(r31)` where
`r31 = 0x80821540`, i.e. `[0x808215F8] = 0.0`.

Consequence for the existing translation: `fn_807bfe20` calls
`call_f64_u32(0x807B_E3D0, value as f64, 0)`, which puts the value in `f1` as a
double and an integer in `r3`; it never sets `f2`. That is representative for
this one site only because `[0x808215F8]` is `0.0`, but the wrapped call should
be `call_f32_u32(0x807B_E3D0, value, 0)` if the `f2` hand-off is to be modelled.
`src/game/bases.rs` was left untouched per the lane rules; the note is for the
coordinator.

## Evidence and checks

- Every body was re-read from the raw words with
  `node ppc159.js work/rels/elf/d_basesNP.elf <addr>`; the truncated Ghidra
  exports were used only as an index.
- Constants read from the ELF: `0x807D0A14 = 1.6`, `0x807D0A18 = 0.2`,
  `0x807D0AC0 = 0.5`, `0x807D0AC4 = 2.0`, `0x807D0AC8 = 1.0`,
  `0x807D0ADC = 20000.0`, `0x80821548 = 16.0` and `0x808215F8 = 0.0`.
- `0x8042A24F`, `0x8042A284`, `0x80374880..0x803748CC`, `0x8043A0F4`,
  `0x808341B8` and `0x80821540` are main.dol globals/vtables, kept as literal
  addresses; `[0x8043A0F4]` is read with `platform::read_f32` because the word
  is outside this REL.
- Branch directions were taken from the `bc` encodings, not from the exports:
  `0x807BC26C` is the collected `GT|EQ` (`cror cr0[2] = cr0[1] | cr0[2]`)
  followed by `bne`, so the clamp runs on `sum >= 1.6`; `0x807BE404` is `bge`
  (`x >= 0.5`); `0x807BE728` is `ble` (`flag <= 0` skips the free call).
- `0x807BA860` collapses two `stwu`-based ten-word blocks (`0x80374880` and
  `0x803748A8`) into the single contiguous range `0x80374880..0x803748CC`;
  they overlap by nothing, so the twenty explicit stores are exact.
- `0x807BE6B0` folds the second `cmpwi r30,0` at `0x807BE6F8`, which re-tests
  the value the entry guard already proved non-zero.
- `0x807BEA40` models the frame as one `[u32; 17]` scratch array: index `0` is
  `r1+0x8`, index `1` is `r1+0xC`, and index `2` is the 60-byte render buffer at
  `r1+0x10`; the four `lfs` operands are the arguments of `0x800FE030` and the
  two trailing pointers are `this + 0x4F8` and the buffer.
- `fn_807ba860` reuses the already translated `fn_807bac40(owner, world)` and
  `fn_807be3d0` reuses `fn_807be230(param_1, param_2)`; no new helpers were
  added.

## Deferred

None.

## Files

- `work/wave191/laneC_block.rs`
- `ledger/agents/wave191-lanec.csv`
- `docs/COVERAGE_WAVE191_LANEC.md`
