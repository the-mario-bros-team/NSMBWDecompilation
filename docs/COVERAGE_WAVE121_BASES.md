# Coverage - Wave 121 (`game::bases`)

## How to read this file

This page documents the Wave 121 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

The `0x8070` bucket holds 204 functions. Before this pass 101 were already
translated, leaving 103; this pass takes 30 of them from three windows that
share field offsets: the sled actor at `0x80708450`/`0x80708860`/`0x807089d0`,
its animation binds and state bodies at `0x8070C880..0x8070E1B0`, and two
neighbour groups at `0x807045B0..0x80704710` and `0x80707040..0x80707E20`.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or a helper's
semantics had to be inferred. Comments in the bodies keep
`// observed:`/`// inferred:`/`// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,056 | 20 | 10 |

All 30 rows are `translated`; none is `partial`.

## What this pass covers

- **Sled animation binds, 5 bodies.** `0x8070C880`, `0x8070C970`, `0x8070CA60`,
  `0x8070CB50` and `0x8070CC40` are one body shape: bind the `0x808056B0` model
  entry into the clip at `+0x584`, tick the holder at `+0x544`, bind the same
  entry into the secondary clip at `+0x5C0`, then apply the rate and scale
  constants. They differ only in the `0x801651F0` table index (`0`/`1`) and in
  which of `0x807CC900`/`0x807CC95C`/`0x807CC96C`/`0x807CC9A8` is passed to
  `0x801680E0`.
- **Sled state bodies, 6 bodies.** The cached-height selector
  [`fn_8070cd30`], the `magic_body` bind and velocity reset [`fn_8070cda0`],
  the three-holder steps [`fn_8070ceb0`] and [`fn_8070d000`], the rebind-plus-
  countdown body [`fn_8070d0d0`] and the sound body [`fn_8070e120`].
- **Sled state entry points, 4 bodies.** [`fn_8070cf90`] (rebind + velocity
  reset), [`fn_8070d6a0`] (rebind + sound `0x562`/`0x563`), [`fn_8070d9e0`]
  (rebind + state `+0x360 = 1`) and [`fn_8070dbd0`] (rebind + doubled tilt).
- **Sled landing machine and bite idle, 2 bodies.** [`fn_8070da40`] walks states
  `1`, `2`, `3` and reports `0x8082F5F0`/`0x8082F6B0`;
  [`fn_8070e1b0`] rescales the body radius, writes the two
  `0x80805664`/`0x80805668` globals and dispatches `Wm_ig_wanwanbig`.
- **Sled construction, transform and teardown, 3 bodies.** The `0x5C0`-byte
  constructor [`fn_80708450`] (`0x80804D48`, holders `+0x524`/`+0x544`/
  `+0x584`), the transform writer [`fn_80708860`] and the destructor
  [`fn_807089d0`].
- **Neighbour group `0x807045B0..0x80704710`, 4 bodies.** The record reset
  [`fn_807045b0`], the two interface reports [`fn_80704630`] /
  [`fn_80704710`] and the counter step [`fn_807046b0`].
- **Neighbour group `0x80707040..0x80707E20`, 6 bodies.** The sound gate
  [`fn_80707040`] with its one-shot latch [`fn_80707400`], the two destructors
  [`fn_807078e0`]/[`fn_80707bc0`], the `0x3F0`-byte allocator [`fn_80707d40`]
  and the enter-state body [`fn_80707e20`].

Shared fields: model archive `+0x540`, primary holder `+0x544`, primary clip
`+0x584`, cached model `+0x5BC`, secondary holder `+0x5F0` (cached `+0x5EC`),
state block `+0x6DC`/`+0x6E0`/`+0x6F0`/`+0x6F4`, cached limits
`+0x788`/`+0x78C`, velocity `+0xE8..+0xF4`, tilt `+0x114`, class vtable `+0x60`,
and the animation table `0x807CC910`.

Start with [`fn_80708450`] (the constructor) for the layout, then
[`fn_8070da40`] for the state machine and [`fn_80708860`] for the matrix
rebuild.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807045B0` | `fn_807045b0` | 56 | medium |
| `0x80704630` | `fn_80704630` | 96 | medium |
| `0x807046B0` | `fn_807046b0` | 64 | medium |
| `0x80704710` | `fn_80704710` | 96 | medium |
| `0x80707040` | `fn_80707040` | 92 | high |
| `0x80707400` | `fn_80707400` | 76 | high |
| `0x807078E0` | `fn_807078e0` | 88 | high |
| `0x80707BC0` | `fn_80707bc0` | 88 | high |
| `0x80707D40` | `fn_80707d40` | 76 | high |
| `0x80707E20` | `fn_80707e20` | 84 | high |
| `0x80708450` | `fn_80708450` | 132 | high |
| `0x80708860` | `fn_80708860` | 284 | high |
| `0x807089D0` | `fn_807089d0` | 132 | high |
| `0x8070C880` | `fn_8070c880` | 232 | high |
| `0x8070C970` | `fn_8070c970` | 232 | high |
| `0x8070CA60` | `fn_8070ca60` | 236 | high |
| `0x8070CB50` | `fn_8070cb50` | 228 | high |
| `0x8070CC40` | `fn_8070cc40` | 232 | high |
| `0x8070CD30` | `fn_8070cd30` | 108 | high |
| `0x8070CDA0` | `fn_8070cda0` | 256 | high |
| `0x8070CEB0` | `fn_8070ceb0` | 212 | high |
| `0x8070CF90` | `fn_8070cf90` | 96 | medium |
| `0x8070D000` | `fn_8070d000` | 208 | high |
| `0x8070D0D0` | `fn_8070d0d0` | 264 | medium |
| `0x8070D6A0` | `fn_8070d6a0` | 216 | medium |
| `0x8070D9E0` | `fn_8070d9e0` | 80 | medium |
| `0x8070DA40` | `fn_8070da40` | 392 | high |
| `0x8070DBD0` | `fn_8070dbd0` | 132 | medium |
| `0x8070E120` | `fn_8070e120` | 128 | high |
| `0x8070E1B0` | `fn_8070e1b0` | 440 | medium |

**Totals:** 30 bodies, 5,056 bytes (20 high, 10 medium).

## Deliberate skips

Read and rejected, with the reason:

- **`0x802DCBFC` thunks.** `0x80700F90`, `0x80700FC0`, `0x807017C0`,
  `0x807017F0`, `0x80704E00`, `0x80704E30`, `0x807054E0`, `0x80705510`,
  `0x80706CF0`, `0x80706D20`, `0x80707CE0`, `0x80707D10`, `0x807083F0` and
  `0x80708420` forward two unnamed arguments to the unresolved indirect-jump
  stub. Earlier waves already recorded that family as `partial`; repeating a
  body whose receiver is a hidden `r12` adjustment adds no evidence.
- **Paired-single / GQR reconstruction failures.** `0x80702AA0`, `0x80702BC0`,
  `0x80702CE0`, `0x80704050` and `0x807040C0` carry Ghidra's `unaff_GQR0` /
  `unaff_GQR3` and `ldexpf` reconstruction. The stores and call sites are
  visible, but the float register slots cannot be rebuilt without guessing
  which paired-single slot holds which value.
- **Long per-frame tails.** `0x8070D1F0` (1,200 bytes) and `0x8070DC70`
  (1,196 bytes) build their heading speed from
  `(float)(4503601774855168.0 - DAT_807cca30)`, the compiler's unsigned-to-float
  conversion whose constants Ghidra inlines into the expression. The six-way
  entry switch underneath is readable, but the speed halfwords would be a
  guess; both are left for a pass that can check the instruction stream.
- **Contact body.** `0x8070D790` (584 bytes) compares a `f64` conversion of
  `0x801C14C0`'s result against a `.sdata` float and snaps the actor onto the
  per-side target; the matrix walk is readable but the two `(double)` casts and
  the `DAT_8082F670` event need the same instruction check.
- **The rest of the bucket.** The 73 other untranslated addresses in `0x8070`
  were not translated in this pass.

## Notes on inference

- Five binds (`0x8070C880..0x8070CC40`) are the same body with two literal
  parameters, so each is `high`: the export prints the differing table index and
  scale constant explicitly, and the shared `0x808056B0` name is already
  documented by [`fn_8070c7a0`].
- `0x8070C970`, `0x8070CA60`, `0x8070CB50`, `0x8070CC40` and the four state
  entry points call their sibling with no printed argument. The sibling bodies
  all address the actor through `+0x540`/`+0x544`, so the actor is forwarded and
  the row is `medium`; the call itself is literal.
- [`fn_80704630`] and [`fn_80704710`] compare `0x80429CA0` against the result of
  `0x80703DC0`, which Ghidra prototypes `void`. The result is therefore taken
  from `r3` as an `i32`; the two addresses keep `// unknown:` roles.
- [`fn_80708860`] replaces only the z coordinate produced by `0x80064AF0` with
  `0x807CC790` or `0x807CC794` before `0x801C0BD0` builds the matrix; the two
  constants are unnamed but the choice is literal.
- [`fn_807089d0`] keeps the compiler's always-true `+0x584 != 0` test, as do the
  other destructors of this family.
- [`fn_8070e1b0`] tests the sign bit of `0x8015F470`'s result through the C's
  `(-value | value) < 0`, which is `value as i32 < 0`; the dispatch on that bit
  is literal.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,056 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 101 translated
  addresses were re-read first and none is duplicated here. The file now holds
  3,052 bodies.
- `node tools/check-lane-csv.mjs wave121` reports `OK` and names no problem in
  `wave121-bases.csv`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
