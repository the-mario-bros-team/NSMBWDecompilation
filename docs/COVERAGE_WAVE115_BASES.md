# Coverage - Wave 115 (`game::bases`)

## How to read this file

This page documents the Wave 115 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

"Confidence" is `high` when the guards, stores, constants and call arguments are
literal, and `medium` when a field role, a callee identity, a string or a lost
receiver had to be inferred. No body in this pass is `low`. Comments in the
bodies keep `// observed:`, `// inferred:` and `// unknown:` markers.

The `0x8078` bucket holds 203 functions. Before this pass 92 were already
translated (wave 66 and later waves), leaving 111. This pass takes 26 of them
from three clusters between `0x807866A0` and `0x8078A410`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 26 | 4,928 | 16 | 10 |

## What this pass covers

- **The second `cobfirepillar` object (`0x807866A0..0x807873A0`).** The
  `g3d_model.brres` / `cobfirepillar` loader [`fn_80786cf0`], the transform
  initialiser [`fn_80786980`] with its `0x1C`-byte and `0x40`-byte runtime
  records at `+0x2B4`/`+0x2B8`, the matrix writer [`fn_80786c00`], the
  per-player animation arm [`fn_80786e10`], the two effect spawns
  [`fn_80787330`]/[`fn_807873a0`], and the four release wrappers
  [`fn_807866a0`]/[`fn_807866e0`]/[`fn_80786720`]/[`fn_80786780`].
- **The `cobHatenaSwitch` tail (`0x80783400..0x80783CE0`).** The per-variant
  setup [`fn_80783400`], the `skl_root` bind and saved-offset restore
  [`fn_807836f0`], and the two halves of the `+0x450` state machine
  [`fn_807839d0`] (states `0`..`7`) / [`fn_80783ce0`] (states `8`..`0x10`).
- **The `broom` ride object (`0x807883E0..0x8078A410`).** Mode setters
  [`fn_807883e0`]/[`fn_807884d0`]/[`fn_80788530`]/[`fn_80788580`], the ride
  placement [`fn_807885d0`], the `countLeadingZeros` query [`fn_80789a60`],
  the nine-word placement table [`fn_80789b10`], the two animation drivers
  [`fn_80789b80`]/[`fn_80789c60`], the slot release [`fn_80789e60`], the lean
  integrator [`fn_8078a170`] and the destructor [`fn_8078a410`].

Start with [`fn_80786cf0`] for the firepillar layout, [`fn_807839d0`] for the
switch machine and [`fn_80789b80`] for the broom's part switching.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807866A0` | `fn_807866a0` | 64 | medium |
| `0x807866E0` | `fn_807866e0` | 64 | high |
| `0x80786720` | `fn_80786720` | 96 | high |
| `0x80786780` | `fn_80786780` | 100 | high |
| `0x80786980` | `fn_80786980` | 360 | high |
| `0x80786C00` | `fn_80786c00` | 164 | high |
| `0x80786CF0` | `fn_80786cf0` | 288 | medium |
| `0x80786E10` | `fn_80786e10` | 136 | high |
| `0x80787330` | `fn_80787330` | 104 | high |
| `0x807873A0` | `fn_807873a0` | 104 | high |
| `0x80783400` | `fn_80783400` | 284 | medium |
| `0x807836F0` | `fn_807836f0` | 244 | medium |
| `0x807839D0` | `fn_807839d0` | 756 | medium |
| `0x80783CE0` | `fn_80783ce0` | 748 | medium |
| `0x807883E0` | `fn_807883e0` | 20 | high |
| `0x807884D0` | `fn_807884d0` | 12 | high |
| `0x80788530` | `fn_80788530` | 80 | high |
| `0x80788580` | `fn_80788580` | 80 | high |
| `0x807885D0` | `fn_807885d0` | 140 | medium |
| `0x80789A60` | `fn_80789a60` | 24 | high |
| `0x80789B10` | `fn_80789b10` | 100 | high |
| `0x80789B80` | `fn_80789b80` | 224 | medium |
| `0x80789C60` | `fn_80789c60` | 292 | medium |
| `0x80789E60` | `fn_80789e60` | 100 | high |
| `0x8078A170` | `fn_8078a170` | 256 | medium |
| `0x8078A410` | `fn_8078a410` | 88 | high |

**Totals:** 26 translated, 0 partial, 0 stub (16 high, 10 medium, 4,928 bytes).

## Deliberate skips

Read and rejected, with the reason:

- **`halt_baddata` stubs.** `0x807812D0` (4), `0x80781C80` (16),
  `0x80781C90` (32), `0x807882E0` (28), `0x8078A270` (16), `0x8078A3A0` (48),
  `0x8078A740` (48), `0x8078B5B0`, `0x8078B7D0`, `0x8078B9F0`, `0x8078BC10`,
  `0x8078BE30` (48 each) and `0x8078C060` (48) have no recoverable body: the
  export truncates at the first bad instruction.
- **`character_SV` GQR bodies.** `0x80780010`, `0x807807E0`, `0x80780A90`,
  `0x80780BD0`, `0x80780D00`, `0x80780E20`, `0x80780FA0`, `0x807811D0`,
  `0x807813F0`, `0x807816B0`, `0x807819B0`, `0x80781B60`, `0x80781D10` and
  `0x80781DB0` open with `unaff_GQR0`/`ldexpf` brackets and the folded `2^52`
  conversion; the source values cannot be proved, so they stay out.
- **ABI-mangled broom placement.** `0x8078B120` (168) and `0x8078B1D0` (340)
  call `0x80789B80` and `0x80776C80` with receivers and float arguments that the
  export drops or misprints; reconstructing the missing register values would be
  a guess, so they remain untranslated.
- **The `0x8078DC70`..`0x8078FF30` tail.** The large bodies
  `0x8078DC70` (2,576), `0x8078E9A0`, `0x8078EB40`, `0x8078F880`,
  `0x8078FC80` and `0x8078FF30` belong to a different manager family and were
  not opened in this pass; several are already covered by earlier waves.

## Notes on inference

- No class name is present in these exports. The archive/pane strings
  (`cobfirepillar`, `g3d_model.brres`, `Wm_cs_magmasign`, `Wm_cs_firespillar`,
  `firepillar`, `ef_firepillar`) identify the first cluster and the
  `ride_wait_broom`/`broom` strings identify the third; both names are marked
  `// inferred:` in the module prose, never as fact.
- `0x80783400`..`0x80783CE0` lie inside the `cobHatenaSwitch` range that wave 66
  documented (`0x80782280..0x80783CD0`); the `+0x424`/`+0x450` state words and
  the `+0x430` model pointer are shared with [`fn_80783890`].
- `0x80783590` is a 32-byte export truncated after its first store. It is called
  by address from [`fn_80783400`] and documented as `unknown:`.
- The folded `CONCAT44(0x43300000, value ^ 0x80000000)` sequence in
  [`fn_8078a170`] is reproduced bit for bit with
  `f64::from_bits(0x4330000000000000 | ...) - read_f64(0x807CFAC8)`; no symbolic
  angle value is invented.
- [`fn_80789b80`]'s `0x80168FE0` call prints only the part pointer. The missing
  `f32` argument is recovered from the otherwise unused `DAT_807CFA08` load in
  the same export and marked `medium`.
- `0x8042A038`, `0x8042A18C`, `0x8042A27C` and `0x8042A284` keep their raw
  addresses; only the archive, effect and class strings are named in prose.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (26 rows, 4,928 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 92 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,882 bodies.
- `node tools/check-lane-csv.mjs wave115` reports `OK` and names no problem in
  `wave115-bases.csv`.
- `cargo check --message-format=short` reports no error naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
