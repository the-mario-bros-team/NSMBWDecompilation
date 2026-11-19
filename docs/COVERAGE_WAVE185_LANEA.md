# Coverage - Wave 185 lane A (`game::bases`)

Ten small bodies from REL `d_basesNP`, all translated. No deferrals.

| Address | Size | Rust | Shape |
| --- | --- | --- | --- |
| `0x80778A50` | 96 | `fn_80778a50` | list insert: vtable slot `+0x0C` predicate, store at `+0x0C`, return `this+4` |
| `0x80722560` | 88 | `fn_80722560` | registered destructor (`0x8015F800` + optional `0x802B9280`) |
| `0x807225C0` | 92 | `fn_807225c0` | same destructor plus a redundant duplicate null test |
| `0x807238F0` | 92 | `fn_807238f0` | same destructor (duplicate null test variant) |
| `0x807242C0` | 88 | `fn_807242c0` | POW-block collision rebuild, wide half-extents |
| `0x80724320` | 88 | `fn_80724320` | POW-block collision rebuild, narrow half-extents |
| `0x8072C830` | 88 | `fn_8072c830` | registered destructor |
| `0x80731330` | 80 | `fn_80731330` | resets five `0xE4`-byte slots at `+0x4D8`, returns `1` |
| `0x80733EC0` | 68 | `fn_80733ec0` | clears launch state, sets six floats to `1.0f` |
| `0x80733F10` | 88 | `fn_80733f10` | steps a float toward a bound, returns "clamped" flag |

## Notes

- `0x807242C0` and `0x80724320` both end at `halt_baddata()` in the Ghidra
  export. The raw words show a `b 0x8006D090` tail call with
  `r3 = this+0x1EC`, `r4 = this`, `r5 = this+0x47C`, `r6 = this+0x488`,
  `r7 = this+0x494`. Both are emitted with that `abi_call5!` tail; this
  supersedes the earlier "no recovered tail" note on `0x80724320`.
- The four destructor variants reuse the already-translated `fn_8075a5a0`
  shape and the existing `0x8015_F800`/`0x802B_9280` call helpers; no new
  helpers were introduced.
- `0x80733EC0` is the `abi_call1!(0x8073_3EC0, this)` tail of the translated
  `fn_80733c30`, which confirms the `this`-only signature.
- `0x80733F10`: Ghidra prints `double` parameters because the ABI keeps the two
  float arguments in `f1`/`f2`; the instructions are `fadds`/`fsubs`/`fcmpo`,
  so both are `f32`. `f1` is only compared (the limit) and `f2` is only added or
  subtracted (the step). The `DAT_807CD7EC` floor reads as `1.0f`.
- The destructor bodies intentionally keep the source order of the raw stores
  only where it matters; the byte/word/float widths follow the `stb`/`stw`/`stfs`
  opcodes, not Ghidra's `undefined4` typing (the ppc printer renders `stfs`'s
  FRS field as `r0`).
