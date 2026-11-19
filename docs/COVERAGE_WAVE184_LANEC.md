# Coverage - Wave 184, lane C (`game::bases`, sub-300-byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 1,004 | 10 | 0 |

All ten lane-C addresses are 68-124 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane C added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave184/laneC_block.rs`, the ledger is
`ledger/agents/wave184-lanec.csv`, and the bodies are appended by the
coordinator.

## Translated

- `0x806F_8720` (92 bytes) - registered-state deleting destructor:
  `0x8015_F800(state, 0)`, then `0x802B_9280(state)` when `free > 0`; returns
  `state`. Same template as [`fn_806f9b40`].
- `0x806F_B740` (120 bytes) - walks the 17 handle words at `this + 0x6E8`
  (stride 4), stopping at the first zero handle, and forwards `angle` to
  [`fn_806fb610`] for each live slot.
- `0x806F_C840` (88 bytes) - the same `0x8015_F800` registered-state deleting
  destructor, byte-for-byte equal in shape to [`fn_806f8720`] and
  [`fn_806ff180`].
- `0x806F_CA20` (88 bytes) - in-place constructor: `0x8006_4110(this)`, vtable
  `0x8080_3330` at `+0x60`, then
  `0x8082_F078 = 0x8016_2B00(0x166, this, 0, 0)`; returns `this`. Same idiom
  as [`fn_806f8ad0`].
- `0x806F_D7A0` (124 bytes) - when `+0x6A0` is set, samples
  `0x800B_30C0`/`0x800B_30E0`, builds a three-float block with `5500.0`
  (`0x807C_C40C`) and dispatches layout `+0x464` slot `+0xA0` with
  `Wm_mg_panelperfect` (`0x8080_32F8`).
- `0x806F_E210` (116 bytes) - panel player selection: guarded by `+0x210 > 1`
  and the clear `+0x1D5`/`+0x1D6` bytes, stores `+0x208`, calls
  [`fn_806fd3d0`], releases the resolved player's `+0x14D4` handle and calls
  [`fn_806fd510`].
- `0x806F_E290` (116 bytes) - scans player ids `start..=3` through
  `0x8005_F900`; for the first hit it returns the signed byte from class vtable
  slot `+0x6C`, else `-1`.
- `0x806F_E810` (68 bytes) - clears `+0x1D4`/`+0x21C`, calls the untranslated
  `0x806F_DBB0(this)` and sets the `+0x218` countdown to `0x2D` through
  [`fn_806fdca0`].
- `0x806F_EBB0` (96 bytes) - links `object` into the list word at `this + 0x0C`
  when the object's vtable slot `+0x0C` returns `0`; returns `this + 4`, else
  `0`.
- `0x806F_F390` (96 bytes) - two-phase alternating timer over `+0x394`/`+0x398`
  with reload `0x78` and phase word `+0x39C`.

## Deferred

None.

## Notes and limitations

- `0x806F_8720` and `0x806F_C840` are the same registered-state destructor
  template as the already-translated [`fn_806f9b40`] and [`fn_806ff180`]; the
  class each release belongs to remains `// unknown:`. `0x806F_8720` also
  contains a dead duplicate `r3` null test in the raw words that collapses to
  the same single guard.
- `0x806F_CA20`'s base-constructor call prints in Ghidra with no arguments, but
  the raw `bl 0x80064110` still has `this` in `r3`, so the translation passes it
  explicitly with `abi_call1!` (the same reading recorded for the wave-183
  `0x8068_F0D0` constructor).
- `0x806F_D7A0`'s two probes are typed with `call_f32_0` because the export
  renders their `f1` result through a `double` cast; both are the same probes
  already modelled that way in [`fn_80749d00`]. The layout name pointer
  `0x8080_32F8` and the `5500.0` constant are read from the ELF.
- `0x806F_E290` never reads `r3`; the signature keeps a leading `_unused`
  parameter so the caller-visible register convention is preserved.
- `0x806F_E810`'s first callee `0x806F_DBB0` (152 bytes) has no Rust body in
  `src/game/bases.rs`, so it is called through `abi_call1!` with the observed
  `this`; adding its body is left to a later lane. Its other callee
  [`fn_806fdca0`] is already translated.
