# Wave 182 lane C - `game::bases` (REL `d_basesNP`)

Ten small bodies, all translated; none deferred. Bodies live in
`work/wave182/laneC_block.rs` and are appended to `src/game/bases.rs` by the
coordinator. Evidence is `work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`,
re-read together with the raw words from
`work/rels/elf/d_basesNP.elf` (`%TEMP%\opencode\ppc159.js`).

## Translated

| address | bytes | confidence | body |
|---|---:|---|---|
| `0x80653CE0` | 124 | high | Folds the `cntlzw` of a `0.0 < x` compare into the 0/1 index of the float table at `0x807C8848` (`[2.0, 2.0]`), picks `+0x74`/`+0x70` from `other[+0x2D] == 0x0B`, and returns `1` when the extent is negative and `2.0 * -extent` is below the entry. |
| `0x80656210` | 72 | high | `flag != 0` -> `scene[+0x38] + scene[+0x40] + 32.0`, else `scene[+0x38] - 32.0`, with `scene = [0x80429E00]`; `+0x38`/`+0x40` are the left edge and width already used by `fn_806515b0`. |
| `0x806564A0` | 92 | medium | Builds the three-float payload `{fn_80656210(this, flag), fn_80656260(this), 500.0}` and submits it with a `0x800B2EE0(2)` selector to `0x80A46F60`. |
| `0x80656500` | 80 | high | Looks up profile `0x79` through `0x80162D50(id, 0)` and copies the record's first word into `this + 0x3D0`; returns whether the lookup succeeded. |
| `0x80656570` | 124 | high | Refreshes the `0x79` handle, then dispatches the `+0x394` interface's virtual `+0x18` with tag `0x80828920` when `+0x3E8 == 1` and `0x808288E0` otherwise. |
| `0x80656B70` | 88 | high | Callback-state destructor: `0x8015F800(state, 0)` and, for `free > 0`, `0x802B9280(state)`; returns `state`. Byte-identical to `fn_806511d0`. |
| `0x80658610` | 104 | high | `Wm_jr_bomb` launch: copies the `+0xAC` position through `0x8006A4B0`, queues sound `0x5C1` on `[0x80429D60]`, then stores `7.5` (`0x807C8954`) at `+0xEC`. |
| `0x8065E460` | 128 | high | Fixed-point clamp/interpolate: `t < 0.0` -> `*low`, `t > 1.0` -> `*high`, else `*low + (i32)((f32)t * (f32)(*high - *low))` truncated to `s16` and sign-extended (`fctiwz` + `extsh`). |
| `0x806626A0` | 92 | high | Same destructor as `0x80656B70`; the duplicated `cmpwi r3,0` pair is the inlined null test, not a second guard. |
| `0x806677E0` | 76 | high | Plain `0x398`-byte factory: `0x801628C0(0x398)`, base constructor `0x80064110(object)`, vtable `0x807E8304` at `+0x60`. Same shape as `fn_806537e0`. |

## Deferred

None. `0x806564A0` was deferred by waves 92 and 157 because `0x80A46F60` is
outside this REL's text range; per the lane rules the call is now emitted by
address (`abi_call2!`) with the argument shape fixed by the raw words
(`r3 = &stack[3 floats]`, `r4 = 0x800B2EE0(2)`), so the body is complete and the
only inferred part is the callee's meaning, recorded in `# Confidence`.

## Decoding notes

- The two data constants this lane needed were read out of the ELF image, not
  taken from prose: `0x807C8848` = `[2.0, 2.0]`, `0x807C8850` = `0.0`,
  `0x807C8854` = `2.0`, `0x807C88F8` = `32.0`, `0x807C8914` = `500.0`,
  `0x807C8954` = `7.5`, `0x807C8AE8` = `0.0`, `0x807C8AF4` = `1.0`,
  `0x807C8AF8` = `0x4330000080000000` (`2^52 + 2^31`).
- The `lis 0x807D` / negative `addi` forms in these bodies resolve to the
  `0x807Cxxxx` pool, as in `fn_80651830`; the block uses the pool addresses,
  matching the rest of `bases.rs`.
- `0x800B2EE0(n)` returns a selector below `n` (random index), matching its use
  in `fn_806b9740`.
- The `0x8065E460` multiply is a single-precision `fmuls`, so the double `t`
  argument is rounded to `f32` before the product; the `fsubs` against
  `2^52 + 2^31` is exact for the `s16` span.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (re-read, not copied).
- Every cited export file exists.
- Each of the ten bodies was decoded from the raw words; the `fcmpo`/`bc`
  pairs (BO=4 = "branch if false") were re-checked against Ghidra's `<`/`<=`.
- The block compiles against the helper signatures used in `bases.rs`
  (`platform::read_*`, `abi_call1/2/4!`, `fn_80656260`).
