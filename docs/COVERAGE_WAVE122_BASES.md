# Coverage - Wave 122 (`game::bases`)

## How to read this file

This page documents the Wave 122 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

The `0x806D` bucket holds 219 functions. Before this pass 117 were already
translated, leaving 102; this pass takes 30 of them from four windows that share
field offsets and call shapes: the `lift_spin_kinoko` actor at
`0x806DEC80..0x806DFF70`, the neighbouring `bolt`/gear object at
`0x806DDA70..0x806DEB00`, the rider-counter actor at `0x806D3680..0x806D3900`,
and the panel/table remainder `0x806D1670..0x806D9FC0`.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or a helper's
semantics had to be inferred. Comments in the bodies keep
`// observed:`/`// inferred:`/`// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,464 | 22 | 8 |

All 30 rows are `translated`; none is `partial`.

## What this pass covers

- **`lift_spin_kinoko` actor, 10 bodies.** The `0x700`-byte constructor
  [`fn_806dec80`] installs vtable `0x807FC428` and the accessor `+0x3D4`, the
  resource word `+0x3F0`, the model holders `+0x3F4`/`+0x434`/`+0x6AC` and the
  four `0x60`-byte records at `+0x474`. [`fn_806df4b0`] opens
  `s_g3d_bolt_brres_807fc3b4`, [`fn_806df570`] opens
  `s_g3d_lift_spin_kinoko_brres_807fc3cc` and binds the `A`/`B` panes into
  `+0x3F4`/`+0x434`. The three transform writers [`fn_806df1c0`],
  [`fn_806df2c0`] and [`fn_806df3b0`] rebuild `+0x6AC`, `+0x3F4` and `+0x434`
  through `0x80064AF0`/`0x801C0BD0`/`0x8016A170`. The tail bodies are the
  presence sweep [`fn_806df770`], the facing stepper [`fn_806dfc40`] and the two
  state machines [`fn_806dfcc0`] / [`fn_806dff70`].
- **`bolt`/gear object, 8 bodies.** [`fn_806dda70`] answers the mode query for
  one child node; [`fn_806ddba0`] and [`fn_806ddaf0`] rebuild the two plane
  heights from the phase at `+0x710`; [`fn_806ddd40`] compares the four-slot
  presence map at `+0x6F4..+0x700` and rebases `+0x5A8`; [`fn_806ddf50`] and
  [`fn_806de1c0`] are the two phase steppers (sound `0x2C3`/`0x2C4`);
  [`fn_806dd510`] places the stepping planes; and [`fn_806de400`] /
  [`fn_806deb00`] are the teardown pair.
- **Rider-counter actor, 5 bodies.** [`fn_806d3680`] counts the `+0x3E0` child
  list by `+0x38C` kind, [`fn_806d37d0`] converts the reported slot into a
  steering angle clamped to ±3000, [`fn_806d3840`] wraps the `+0x104` heading
  through `0x8015F2E0`, [`fn_806d3890`] tests a height against the
  `0x80429E00` window, and [`fn_806d3900`] latches the slot and posts sound
  `0x2A7`. These are the helpers that the already-translated
  [`fn_806d3b60`] / [`fn_806d3a60`] / [`fn_806d39e0`] call.
- **Panel/table remainder, 7 bodies.** The two `0x58`-record initialisers
  [`fn_806d5970`] / [`fn_806d5a90`] (differing only in `0x80088DD0` vs
  `0x80088E80`), the two-plane rider placement [`fn_806d9fc0`], the
  `Wm_ob_cmnsparkloop` effect spawn [`fn_806d3510`], and the release helpers
  [`fn_806d1670`] / [`fn_806db2c0`] / [`fn_806deb00`].

Shared fields: model holders `+0x3F4`/`+0x434`/`+0x6AC`, resource word `+0x3F0`,
accessor `+0x3D4`, matrix `+0x7C`, position `+0xAC..+0xB4`, angles
`+0x100..+0x104` and `+0x638..+0x63C`, interface `+0x394`, class vtable `+0x60`,
child chains (`+0x0C` next, `+0x04` actor), and the panel record strides `0x58`
and `0x60`.

Start with [`fn_806dec80`] (the constructor) for the layout, then
[`fn_806ddba0`] (the two-list rider pass) and [`fn_806df1c0`] (the transform
write).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806D1670` | `fn_806d1670` | 64 | high |
| `0x806D3510` | `fn_806d3510` | 44 | medium |
| `0x806D3680` | `fn_806d3680` | 328 | medium |
| `0x806D37D0` | `fn_806d37d0` | 100 | high |
| `0x806D3840` | `fn_806d3840` | 80 | high |
| `0x806D3890` | `fn_806d3890` | 60 | medium |
| `0x806D3900` | `fn_806d3900` | 212 | high |
| `0x806D5970` | `fn_806d5970` | 276 | medium |
| `0x806D5A90` | `fn_806d5a90` | 276 | medium |
| `0x806D9FC0` | `fn_806d9fc0` | 144 | high |
| `0x806DB2C0` | `fn_806db2c0` | 64 | high |
| `0x806DD510` | `fn_806dd510` | 64 | high |
| `0x806DDA70` | `fn_806dda70` | 124 | high |
| `0x806DDAF0` | `fn_806ddaf0` | 172 | medium |
| `0x806DDBA0` | `fn_806ddba0` | 416 | medium |
| `0x806DDD40` | `fn_806ddd40` | 472 | medium |
| `0x806DDF50` | `fn_806ddf50` | 204 | high |
| `0x806DE1C0` | `fn_806de1c0` | 204 | high |
| `0x806DE400` | `fn_806de400` | 204 | high |
| `0x806DEB00` | `fn_806deb00` | 88 | high |
| `0x806DEC80` | `fn_806dec80` | 160 | high |
| `0x806DF1C0` | `fn_806df1c0` | 244 | high |
| `0x806DF2C0` | `fn_806df2c0` | 240 | high |
| `0x806DF3B0` | `fn_806df3b0` | 244 | high |
| `0x806DF4B0` | `fn_806df4b0` | 180 | high |
| `0x806DF570` | `fn_806df570` | 188 | high |
| `0x806DF770` | `fn_806df770` | 176 | high |
| `0x806DFC40` | `fn_806dfc40` | 96 | high |
| `0x806DFCC0` | `fn_806dfcc0` | 120 | high |
| `0x806DFF70` | `fn_806dff70` | 220 | high |

**Totals:** 30 bodies, 5,464 bytes (22 high, 8 medium).

## Deliberate skips

Read and rejected, with the reason:

- **`0x802DCBFC` thunks.** `0x806D1410`, `0x806D1440`, `0x806D4450`,
  `0x806D4480`, `0x806D5230`, `0x806D5260`, `0x806D7510`, `0x806D7540`,
  `0x806D8260`, `0x806D8290`, `0x806D9200`, `0x806D9230`, `0x806DB050`,
  `0x806DB080`, `0x806DBE70`, `0x806DBEA0`, `0x806DCDE0`, `0x806DCE10`,
  `0x806DEC20` and `0x806DEC50` forward two unnamed arguments to the
  unresolved indirect-jump stub. Earlier waves already recorded that family as
  `partial`; repeating a body whose receiver is a hidden `r12` adjustment adds
  no evidence.
- **`halt_baddata` bodies.** `0x806D1ED0`, `0x806D21A0`, `0x806D38D0` and
  `0x806D3AC0` decode to a truncated jump (`Bad instruction`); `0x806D3AC0`
  still shows its guard, but the guarded path ends in the same undecodable
  branch, so only the guard could be written. They stay out, as the equivalent
  bodies did in waves 121 and earlier.
- **Paired-single / GQR conversion failures.** `0x806D6530`, `0x806D65A0`,
  `0x806DDA10`, `0x806DE0B0`, `0x806DE320` and `0x806DF630` convert a halfword
  to `f32` through Ghidra's `unaff_GQR3` branch ladder (`ldexpf` plus
  `CONCAT22`). The stores and call sites are visible, but which GQR slot feeds
  the conversion cannot be decided without the instruction stream, so the
  numeric result would be a guess. `0x806DDA10` is the height helper called by
  [`fn_806ddf50`] / [`fn_806de1c0`]; those two call it by address.
- **Long per-frame tails.** `0x806D3C20` (784 bytes), `0x806DD730` (736 bytes)
  and `0x806DED20` (688 bytes) mix the same GQR ladder with a six-way entry
  switch and an unsigned-to-float conversion whose constants Ghidra inlines
  into the expression. They need an instruction-level pass.
- **The rest of the bucket.** The 72 other untranslated addresses in `0x806D`
  were not attempted in this pass; they were read only far enough to rule them
  out of the four windows above.

## Notes on inference

- [`fn_806d3510`] is `medium`: the six call arguments and virtual slot `+0xA0`
  are literal, but Ghidra reports `Could not recover jumptable` for the callee,
  so the meaning of the `+0x650` argument stays `// unknown:`.
- [`fn_806d3680`], [`fn_806d5970`] and [`fn_806d5a90`] load their constants
  through the paired-single prologue. The C shows the resulting `f32` values
  explicitly, so they are written as `platform::read_f32`; the guard only ever
  checks that the GQR scale is zero.
- [`fn_806d3890`] takes its argument as `f32` although the C declares `double`:
  the call site loads `f1` with `lfs`, and both bounds are `f32` globals.
- [`fn_806ddaf0`] and [`fn_806ddba0`] show `(double)` temporaries around
  single-precision products because `0x800DB2D0` has no prototype. The ratio is
  rebuilt in `f32`, and the one division that the C keeps in `double`
  (`total / +0x714`) is kept in `f64` in the Rust body.
- [`fn_806ddba0`] and [`fn_806ddd40`] fetch the actor through the
  `0x802DCD74` / `0x802DCD70` prologs, exactly as the already-translated
  [`fn_806d5f20`] groups do; the list selectors are read back from
  `0x807CB9F0`/`0x807CB9F4` rather than assumed to be `0`/`1`.
- [`fn_806ddd40`] calls virtual slot `+0x6C` of the child's `+0x60` vtable and
  reads the returned byte; the callee is unprototyped, so the byte is fetched
  through the returned pointer, and the index is bounds-checked against the
  four-slot map the C indexes blindly.
- [`fn_806dec80`] names the object `lift_spin_kinoko` only because
  [`fn_806df570`] opens that archive; the C-level class identity stays
  `// inferred:`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,464 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 117 translated
  addresses were re-read first and none is duplicated here. The file now holds
  3,082 bodies.
- `node tools/check-lane-csv.mjs wave122` reports `OK` for
  `wave122-bases.csv` (30 rows, 30 unique addresses).
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
