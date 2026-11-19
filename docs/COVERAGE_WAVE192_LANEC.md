# Wave 192, lane C coverage (`game::bases`)

Lane band `0x8063A470..0x8064EC70`, all inside REL `d_basesNP`. Ten of ten
requested bodies translated; none deferred.

## Bodies

| Address | Bytes | Function | Summary |
| --- | --- | --- | --- |
| `0x8063A470` | 240 | `fn_8063a470` | Free-result loader: probes `pointResultDateFileFree.arc`, swaps to `pointResultDateFileFree_00.brlyt` through the `+0x78` slot `+0x14`, stores the `+0x244` handle and binds the five result panes. Returns `1`/`0`. |
| `0x8063C090` | 208 | `fn_8063c090` | Rebuilds the ten-row score table bucket by bucket (`10000..1`), calling `fn_8063bd90`/`fn_8063bf30` for the first entry of each hundred-point bucket. |
| `0x8063EF90` | 204 | `fn_8063ef90` | Finalises the `+0xAC` pane, applies `+0x274 + 0xE`, cues `0x78`, then reloads `+0x274` from `+0x26C + {2,5}` and applies that frame. |
| `0x80648470` | 224 | `fn_80648470` | Publishes a bob height to `+0x4CC`/`+0x504`, optionally to the shared scene timer, or to `0x800EBBA0` with the `(x, value, z)` point. |
| `0x80649090` | 216 | `fn_80649090` | Sine-driven height step; decrements `+0x1A4EE` and fires `0x80827E50` in mode `2`, then stores the new mode in `+0x1A4E0`. |
| `0x80649280` | 208 | `fn_80649280` | Steps `fn_806485c0(.., 1)` and `fn_80648710`, then dispatches `0x80827ED0` in mode `2` or `0x80827F50` outside it. |
| `0x80649460` | 196 | `fn_80649460` | Ramps `+0x1A4B8` by half `+0x1A4B4` (double `fmul`), clamps at `8.0 + +0x1A4B0`, and dispatches `0x80827F50` on the clamp. |
| `0x8064A9E0` | 224 | `fn_8064a9e0` | Two-phase `+/-0x400` halfword rotation state machine over `+0x75C`/`+0x758`, with `+0x76C`/`+0x770`/`+0x774` phases. |
| `0x8064EB50` | 220 | `fn_8064eb50` | Rebuilds the `+0x7C` matrix from `(+0xAC,+0xB0,+0xB4)` and the `+0x100..+0x104` angles, then rebinds `+0x544` with `+0x7C`/`+0xDC`. |
| `0x8064EC70` | 256 | `fn_8064ec70` | Mirrors the `+0x584`/`+0x5BC` clip position into the `[0x808283E8]` camera record, selected by mode `2`/`3` and the `+0x3F4`/`+0x3F5` bytes. |

## Exact signature of `0x8064EC70`

`0x8064EC70` is called three times per branch by the translated `fn_8064eeb0`,
and twice each by `fn_8064f390`/`fn_8064f6a0` as
`abi_call1!(0x8064_EC70, this)`. Its exact prototype is:

```rust
pub unsafe fn fn_8064ec70(this: usize)
```

It takes only `r3` (the `BGM_anim_dummy` owner), returns nothing, and touches
only the `+0x600` mode, the two clip movers and the `[0x808283E8]` camera
record. There is no hidden float argument: the entry block tests `+0x600` and
branches before any `f` register is read.

## Exact signature of `0x80648470`

Several translated lift bodies already call this address. The true prototype is

```rust
pub unsafe fn fn_80648470(value: f32, this: usize)
```

Ghidra prints `double param_1` only because the callee is unprototyped (the
same old-style promotion it applies to every `lfs` argument in this REL). The
callee opens with `fsubs f4,f1,f0` at `0x806484A8`, which reads `f1` as a
single, and the existing call sites use `call_f32_u32(0x8064_8470, height,
this)`. The body never converts to double.

Note for the coordinator: a handful of pre-existing lift bodies call this
address through `abi_call1!(0x8064_8470, this)` without setting `f1` (for
example at `src/game/bases.rs:165662`), which cannot be right for this
prototype. They still compile because they go through the raw `platform::call1`
trampoline; only the argument modelling is incomplete. Per lane rules
`src/game/bases.rs` was left untouched.

## Evidence and checks

- Every body was re-read from the raw words with
  `node ppc159.js work/rels/elf/d_basesNP.elf <addr>`; the truncated Ghidra
  exports were used only as an index.
- Reused already-translated neighbours instead of new trampolines:
  `fn_8063bd90`, `fn_8063bf30`, `fn_80648550`, `fn_806485c0`, `fn_80648710`.
- Constants read from the ELF: `0x807C847C = 8.0`, `0x807C850C = 0.0`,
  `0x807C84D8 = 16.0`, `0x807C8510 = 0.00390625`, `0x807C8474 = 4.0`,
  `0x807C7F80 = 8` (u32), and `0x807C8540 = 0.5` read as a **double**
  (`lfd` at `0x80649478`), not the `1.75` float that shares those bits.
- `0x8063C090`'s incoming `r3` is the score-table base: the body starts with
  `_savegpr_27` (`0x802DCD74`), which preserves `r3` and has no Rust-visible
  effect, so the Ghidra `void` prototype is wrong. The caller `fn_8063d260`
  and the raw `bl` sites pass `this`.
- `0x8064EC70` mode `3` dereferences `[0x808283E8]` without a null test; the
  safety note says so. Mode `2` tests the pointer explicitly.
- The two arms of `0x8063EF90`'s `[0x80429F80] & 0x10` test that both apply
  `+0x274 + 0xE` are byte-identical, so the first apply is folded to one path.
- `0x80648470` uses `call_f32x2_u32x3` for `0x800EBBA0` (two `f32`, three
  `u32`), matching the raw `lfs f1`/`fadds f2` and `r3`/`r4`/`r5` setup.
- `0x8064EB50` reuses the shape of the already translated `fn_806b1970` and
  differs only in the model holder address (`+0x544` here, `+0x3F0` there).
- All bodies parse cleanly against stub declarations of the reused helpers
  (checked with `rustc --edition 2021 --crate-type lib`).

## Deferred

None.

## Files

- `work/wave192/laneC_block.rs`
- `ledger/agents/wave192-lanec.csv`
- `docs/COVERAGE_WAVE192_LANEC.md`
