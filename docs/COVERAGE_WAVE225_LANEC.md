# Wave 225, lane C - `game::stage`

The two remaining open entries of the `0x800AB6xx` actor band, appended from
`work/wave225/laneC_block.rs`. Both `functions.tsv` sizes were re-checked
against the prologue and the `blr` in `work/dol/main.elf`; both match exactly
(no prologue artefacts, no bytes added). The two entries are vtable methods of
one class, so they share a shape: query the `+0x394` interface, run one `+0x60`
callback, then test the callback's scalar against the `+0x644` animation block.

| address | name | tsv size | prologue -> `blr` | status | confidence |
|---|---|---|---|---|---|
| `0x800AB650` | [`fn_800ab650`] | 148 | `0x800AB650` -> `0x800AB6E0` | translated | high |
| `0x800AB700` | [`fn_800ab700`] | 164 | `0x800AB700` -> `0x800AB7A0` | translated | high |

Totals: 2 bodies, 312 bytes, no deferrals.

## What the lane covers

* **`0x800AB650`** - interface-gated range test. `+0x394` is the actor's
  interface sub-object; its vtable slot `+0x28` yields the state object, whose
  own `+0x14` slot is asked about the descriptor `0x80358304`. Only a non-zero
  answer continues: the actor's `+0x60` vtable slot `+0x510` runs with `r3 =
  this`, and the `f1` it leaves becomes the value handed to `0x801690E0`
  together with `this + 0x644`. The range test's non-zero answer is returned as
  `1`; every other path returns `0`.
* **`0x800AB700`** - the same shape with descriptor `0x80358344` and vtable
  slot `+0x514` instead of `+0x510`. The callback result is clamped first:
  `lfs r0,0x66c(r31)` / `fcmpu` / `bc 4,1` / `fmr f1,f0` replaces the value
  only when `+0x66C` is strictly smaller, so what reaches `0x801690E0` is
  `min(callback, +0x66C)`.

Both bodies keep the file's existing `+0x394` vocabulary (`interface`, `state`,
slots `+0x28`/`+0x14`) used by [`fn_80098ad0`], and `0x801690E0` is the range
predicate already reconstructed as
[`crate::resource::registry::fn_801690e0`].

## Export corrections recorded in the doc blocks

1. Both bodies - `(int)(-uVar2 | uVar2) < 0` is the compiler's "not zero" test
   (`neg r0,r3` / `or r0,r0,r3` / `rlwinm. r0,r0,0,1,31`), i.e. `uVar2 != 0`.
   It is not a positivity test and no sign handling is modelled.
2. `0x800AB650` - the `f1` argument of `0x801690E0` is absent from the export
   because the body never writes `f1`: only `r3 = this + 0x644` is set before
   the `bl`. The value is the `f1` result of the `+0x510` callback, which is
   why the block resolves that slot through `call_vtable_slot1_f32_C` instead
   of inventing a field read.
3. `0x800AB700` - the export prints `FUN_801690e0(dVar4, param_1 + 0x644)`,
   i.e. scalar first. The ABI order is the pointer in `r3` and the scalar in
   `f1`, so the block calls `(element, value)`. The clamp is a conditional move
   that keeps the callback's value on an unordered compare.

## New names

* `FN_801690E0_C` - `0x8016_90E0`, the range predicate.
* `W225_DESC_80358304_C` / `W225_DESC_80358344_C` - the two opaque descriptor
  words. Both live in the runtime-populated `0x8035xxxx` band, which has no
  file bytes in `main.elf`; the `lis`/`addi` pairs
  (`0x3C808036`/`0x38848304`, `0x3C808036`/`0x38848344`) are the evidence for
  the values.
* `call_f32_u32_C` - `f(f1 = f32, r3 = u32) -> u32`, needed because
  `platform::call2` is integer-only and would drop the scalar.
* `call_vtable_slot1_f32_C` - resolves a vtable slot and returns its `f1`.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to
`$env:TEMP\opencode\w225laneC`, this block was appended to the copy's
`src/game/stage.rs`, and `cargo check --lib` finished with exit code 0 (26
pre-existing warnings, none from this lane).
