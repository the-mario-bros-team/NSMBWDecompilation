# Coverage - Wave 124 (`game::bases`)

## How to read this file

This page documents the Wave 124 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the same warning line. Cross-module
pointers (`0x8035xxxx`, `0x8042xxxx`) keep their absolute `main.dol` values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

The `0x806A` bucket holds 158 functions. Before this pass 59 were translated,
leaving 99; this pass takes 30 of them, from `0x806A13C0` to `0x806AE9D0`.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role, a lost argument or a
helper's semantics had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4,800 | 11 | 19 |

All 30 rows are `translated`; none is `partial`.

## What this pass covers

- **Per-class helper sets (8 bodies).** The bucket repeats the helper set that
  every registered actor carries. This pass takes the three copies of the
  five-link `+0x30` child-chain walk
  [`fn_806a13c0`]/[`fn_806a2090`]/[`fn_806a8360`], the three copies of the
  `:`-suffix name compare [`fn_806a14a0`]/[`fn_806a2170`]/[`fn_806a8440`], one
  registered destructor [`fn_806a1f40`] and one class-name getter
  [`fn_806a1fa0`] (the getter returns `0x8082B52C`; that it belongs to the
  `0xDE8`-byte class around `0x806A15C0` is `// inferred:` from the layout).
- **Miss-hit effects (3 bodies).** [`fn_806a22c0`] is the two-threshold
  classifier; [`fn_806a2300`] plays the `Wm_jr_misshit` effect ladder (sounds
  `0x5B7`/`0x5B8`/`0x5B9`, effect names from `0x807CA384..0x807CA38C`); and
  [`fn_806a1850`] spawns the `Wm_en_landsmoke` burst at the actor position.
- **Collision handlers (2 bodies).** [`fn_806a6450`] (state name `0x8082BAC8`
  plus the `0x806C2140` spark) and [`fn_806a8b90`] (state name `0x8082BBC8`
  plus the [`fn_806a8dd0`] burst) share the `+0x04` record/event split and the
  `+0x38C == 1` / `+0x2D == 2` branch pair.
- **Four-slot bit container (4 bodies).** [`fn_806a6b10`] allocates the first
  free slot, [`fn_806a7cc0`] clears every occupancy bit, [`fn_806a7f30`] builds
  the `count * 4` element array through `0x802B9270`, and [`fn_806a70c0`] is
  the "no live element" test used by the container sweep.
- **Landing dispatch (1 body).** [`fn_806a72e0`] probes the `+0x1EC` status
  block three times and selects animation tags `0`, `1` or `2`, or leaves
  through the `0x8082BAC8` interface.
- **`Wm_jr_fireplace` actor (5 bodies).** [`fn_806a8980`] spawns or refreshes a
  child in the owner's 56-slot handle array, [`fn_806a8a40`] maps the X
  coordinate to the slot index, [`fn_806a8b90`] is its collision handler,
  [`fn_806a8dd0`] is the burst death with sound `0x51F`, and [`fn_806a8ee0`] is
  the per-frame pane/timer step.
- **`0x860` actor tail (3 bodies).** [`fn_806a9990`] is the state-tag setter,
  [`fn_806a99f0`] the `+0x6C0` getter, and [`fn_806a9ea0`] releases the
  `+0x544`/`+0x5BC` holders and conditionally `+0x638`.
- **`0x968`/`0x6E0` tails (4 bodies).** [`fn_806aa190`] and [`fn_806ac430`]
  push the animation frame from vtable slots `+0x2C0`/`+0x2B8` into `+0x102`;
  [`fn_806ac260`] rebuilds the two inline `0x18`-stride animation tables at
  `+0x650` (three entries) and `+0x6CC` (seven entries); [`fn_806addf0`] is the
  registered destructor of that class and [`fn_806ae9d0`] the per-frame step of
  the `0x6E0` effect actor (sound `0x58D`).

Start with [`fn_806a8ee0`] (the richest distinct body), [`fn_806a72e0`] (the
three-way landing dispatch) and [`fn_806ac260`] (the two table builders).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806A13C0` | `fn_806a13c0` | 224 | medium |
| `0x806A14A0` | `fn_806a14a0` | 136 | medium |
| `0x806A1850` | `fn_806a1850` | 124 | medium |
| `0x806A1F40` | `fn_806a1f40` | 92 | high |
| `0x806A1FA0` | `fn_806a1fa0` | 12 | high |
| `0x806A2090` | `fn_806a2090` | 224 | medium |
| `0x806A2170` | `fn_806a2170` | 136 | medium |
| `0x806A22C0` | `fn_806a22c0` | 56 | high |
| `0x806A2300` | `fn_806a2300` | 176 | medium |
| `0x806A6450` | `fn_806a6450` | 236 | medium |
| `0x806A6B10` | `fn_806a6b10` | 304 | medium |
| `0x806A70C0` | `fn_806a70c0` | 16 | high |
| `0x806A72E0` | `fn_806a72e0` | 372 | medium |
| `0x806A7CC0` | `fn_806a7cc0` | 184 | medium |
| `0x806A7F30` | `fn_806a7f30` | 120 | medium |
| `0x806A8360` | `fn_806a8360` | 224 | medium |
| `0x806A8440` | `fn_806a8440` | 136 | medium |
| `0x806A8980` | `fn_806a8980` | 184 | medium |
| `0x806A8A40` | `fn_806a8a40` | 92 | high |
| `0x806A8B90` | `fn_806a8b90` | 148 | medium |
| `0x806A8DD0` | `fn_806a8dd0` | 196 | high |
| `0x806A8EE0` | `fn_806a8ee0` | 316 | high |
| `0x806A9990` | `fn_806a9990` | 92 | medium |
| `0x806A99F0` | `fn_806a99f0` | 8 | high |
| `0x806A9EA0` | `fn_806a9ea0` | 156 | high |
| `0x806AA190` | `fn_806aa190` | 88 | medium |
| `0x806AC260` | `fn_806ac260` | 332 | high |
| `0x806AC430` | `fn_806ac430` | 88 | medium |
| `0x806ADDF0` | `fn_806addf0` | 92 | high |
| `0x806AE9D0` | `fn_806ae9d0` | 236 | medium |

**Totals:** 30 bodies, 4,800 bytes (11 high, 19 medium).

## Deliberate skips

Read and rejected, with the reason:

- **`0x802DCBFC` forwarders.** `0x806A1560`, `0x806A1590`, `0x806A2230`,
  `0x806A2260`, `0x806A6080`, `0x806A60B0`, `0x806A9550`, `0x806A9580`,
  `0x806A9670`, `0x806A96A0`, `0x806AF150`, `0x806AF180`, `0x806AF860` and
  `0x806AF890` are the two-argument thunk pair already recorded as `partial`
  by earlier waves; repeating them adds no evidence.
- **`halt_baddata` bodies.** `0x806A6A70` and `0x806A6A80` are truncated by bad
  instruction data, and `0x806A6AA0` calls the first of them, so its argument
  value cannot be proved.
- **Paired-single / GQR conversions.** `0x806A23B0`, `0x806A4CA0`,
  `0x806A53E0`, `0x806A75B0` and `0x806A8AA0` convert halfwords through
  Ghidra's `unaff_GQR0` ladder (`ldexpf` plus `CONCATxx`). The stores and call
  sites are visible, but which GQR slot feeds each conversion is not decidable
  from the export, so the numeric result would be a guess. This is the same
  family the wave-123 pass recorded at `0x80680720` and neighbours.
  `0x806A8AA0` is still called from [`fn_806a8980`] by address.
- **Long container sweep.** `0x806A6C50` (780 bytes) uses the `0x802DCD70`
  bracket and six duplicated cursor guards; the recovered labels overlap and
  the element order is not provable from the export.
- **Large per-frame body.** `0x806A0290` (948 bytes) was read only far enough
  to exclude it: it needs a separate pass for its state-1 tail.
- **Everything else.** The remaining untranslated `0x806A` addresses were not
  attempted here.

## Notes on inference

- [`fn_806a13c0`], [`fn_806a2090`], [`fn_806a8360`], [`fn_806a14a0`],
  [`fn_806a2170`] and [`fn_806a8440`] are `medium`: the unrolling, the recursion
  and every call argument are literal, but the export drops the receiver of the
  `+0xC` and `+0x20` virtual calls (it is `r3`) and the probe's meaning is
  unknown. The bodies are byte-for-byte the shape already written as
  [`fn_80672e20`], [`fn_806be930`], [`fn_80672f00`] and [`fn_806bea10`].
- [`fn_806a1850`] is `medium` because `0x800757B0` receives two overlapping
  stack vectors that start at different slots in the export; both are kept as
  printed rather than rationalised.
- [`fn_806a2300`] is `medium`: the sound ladder is literal, but that the three
  data words at `0x807CA384..0x807CA38C` are string pointers is inferred from
  their position in the effect call.
- [`fn_806a8980`] is `medium`: `0x806A8A40` is called with its arguments dropped
  by the export, so they are supplied as `(owner, source)` from the sibling call
  in `0x806A8AA0`; the two leading words of the signature stay unused (`_a`,
  `_b`) instead of being guessed.
- [`fn_806a72e0`] and [`fn_806ae9d0`] are `medium`: every branch and slot is
  literal, but the `0x8006xxxx` status probes and the 20-byte state block are
  unprototyped, so what each stage means stays `// unknown:`.
- [`fn_806a6b10`], [`fn_806a7cc0`] and [`fn_806a7f30`] are `medium`: the
  bitmap, the counts and the allocation size are literal; the `+0x18` virtual
  notification and the `0x8042AE10` default heap are inferred from the
  neighbouring containers and allocators.
- [`fn_806a9990`] is `medium` because Ghidra reports two unrecovered jump tables
  at `0x806A99C4` and `0x806A99DC`; the recovered branches are still literal.
- [`fn_806aa190`] and [`fn_806ac430`] are `medium` for the same dropped-receiver
  reason at vtable slots `+0x2C0` and `+0x2B8`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 4,800 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named
  `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 59 translated
  addresses were re-read first and none is duplicated here.
- `node tools/check-lane-csv.mjs wave124` reports `OK` for
  `wave124-bases.csv` (30 rows, 30 unique addresses).
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`.
