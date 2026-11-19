# Coverage - Wave 116 (`game::bases`)

## How to read this file

This page documents the Wave 116 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

"Confidence" is `high` when the guards, stores, constants and call arguments are
literal, and `medium` when a field role, a callee identity or a lost receiver
had to be inferred. No body in this pass is `low`. Comments in the bodies keep
`// observed:`, `// inferred:` and `// unknown:` markers.

The `0x8060` bucket holds 240 functions. Before this pass 130 were already
translated (waves 3, 12-16, 55, 59, 62, 63 and later), leaving 110. This pass
takes 29 of them from three complete screen tails between `0x80603860` and
`0x80608090`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 4,732 | 20 | 9 |

## What this pass covers

- **Character-change select tail (`0x80603860..0x80604170`).** Ten bodies that
  complete the screen whose state table, preview picker and commit handler
  were translated earlier: the preview placement [`fn_80603860`] (with the
  folded `2^52` X scale), the sound-and-slot combination [`fn_80603a10`], the
  slot scan/cancel [`fn_80603c60`], the pane reset [`fn_80603eb0`], the selected
  row restore [`fn_80603fa0`], and the five animation gates
  [`fn_80603990`] / [`fn_80603f30`] / [`fn_80604090`] / [`fn_80604130`] /
  [`fn_80604170`]. Shared fields: layout `+0x90`, interface `+0x228`, records
  `+0x80`, preview `+0x2D4`, selected slot `+0x2D8`, owner `+0x2DC`, counter
  `+0x288` and the `0x80354E60` chosen-slot array.
- **`dCharacterChangeSelectContents_c` tail (`0x806057B0..0x80606300`).** Nine
  bodies: the `characterChangeSelectContents_05` layout loader
  [`fn_806057b0`] (binding `N_cSelectCon_00`, `PbuttonShadow_00` and
  `T_zanki_01`), the three counter rows [`fn_80605a20`] / [`fn_80605aa0`] /
  [`fn_80605b20`], the row refresh [`fn_80606030`], the inner selection marker
  [`fn_806060a0`], the reset [`fn_80606140`] and the two state gates
  [`fn_80606260`] / [`fn_80606300`]. Shared fields: layout `+0x70`, interface
  `+0x20C`, base pointer `+0x208`, text objects `+0x2A8..+0x2B0`, panes
  `+0x248`/`+0x254`/`+0x274` and flags `+0x29D..+0x2A0`.
- **`dDateFile_c` coin-date bodies (`0x80607740..0x80608090`).** Ten bodies
  continuing the wave-63 helpers: the 9x12 unlock table build and saved row
  order copy [`fn_80607740`], the open sequence [`fn_80607a50`], the column
  availability flags [`fn_80607b10`], the idle gate [`fn_80607bc0`], the
  button routing [`fn_80607c50`], the row-two pane start [`fn_80607d00`], the
  previous-pane reopen [`fn_80607db0`], the column-change state selection
  [`fn_80607e50`], the open sound [`fn_80608040`] and the open gate
  [`fn_80608090`]. Shared fields: inner layouts `+0x74`/`+0x78`, primary
  layout `+0xE0`, interface `+0x420`, table `+0x47C`, cursor `+0xB44` and
  unlocked bytes `+0xB51..+0xB59`.

Start with [`fn_80603860`] for the select screen, [`fn_806057b0`] for the
contents layout and [`fn_80607740`] for the coin-date table.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80603860` | `fn_80603860` | 292 | medium |
| `0x80603990` | `fn_80603990` | 88 | high |
| `0x80603A10` | `fn_80603a10` | 400 | high |
| `0x80603C60` | `fn_80603c60` | 232 | medium |
| `0x80603EB0` | `fn_80603eb0` | 120 | high |
| `0x80603F30` | `fn_80603f30` | 88 | high |
| `0x80603FA0` | `fn_80603fa0` | 232 | high |
| `0x80604090` | `fn_80604090` | 132 | high |
| `0x80604130` | `fn_80604130` | 64 | high |
| `0x80604170` | `fn_80604170` | 88 | high |
| `0x806057B0` | `fn_806057b0` | 220 | medium |
| `0x80605A20` | `fn_80605a20` | 116 | medium |
| `0x80605AA0` | `fn_80605aa0` | 116 | medium |
| `0x80605B20` | `fn_80605b20` | 116 | medium |
| `0x80606030` | `fn_80606030` | 100 | medium |
| `0x806060A0` | `fn_806060a0` | 160 | high |
| `0x80606140` | `fn_80606140` | 84 | high |
| `0x80606260` | `fn_80606260` | 124 | high |
| `0x80606300` | `fn_80606300` | 88 | high |
| `0x80607740` | `fn_80607740` | 504 | medium |
| `0x80607A50` | `fn_80607a50` | 164 | high |
| `0x80607B10` | `fn_80607b10` | 176 | high |
| `0x80607BC0` | `fn_80607bc0` | 104 | high |
| `0x80607C50` | `fn_80607c50` | 148 | medium |
| `0x80607D00` | `fn_80607d00` | 172 | high |
| `0x80607DB0` | `fn_80607db0` | 160 | high |
| `0x80607E50` | `fn_80607e50` | 280 | high |
| `0x80608040` | `fn_80608040` | 76 | high |
| `0x80608090` | `fn_80608090` | 88 | high |

**Totals:** 29 translated, 0 partial, 0 stub (20 high, 9 medium, 4,732 bytes).

## Deliberate skips

Read and rejected, with the reason:

- **Delete wrappers.** `0x80600990`, `0x806009D0`, `0x80600A10`, `0x80600A50`,
  `0x80600A90`, `0x80600AD0`, `0x80601D10`, `0x80601EB0` and `0x80605500`
  (64 bytes each) are the same three-line `0x802B9280` teardown with every call
  argument dropped; `0x80600B10`, `0x80600B70`, `0x80601F50`, `0x80605540`
  and `0x806055A0` (96/100 bytes) add one `0x8015F9F0(+0x18, 0)` call. The
  missing argument makes the ownership semantics unprovable, and nine copies
  would add no information, so they stay out.
- **`0x802DCBFC` thunks.** `0x80601780`, `0x806017B0`, `0x80604360`,
  `0x80604390`, `0x806064E0`, `0x80606510`, `0x80608350` and `0x80608380`
  only forward two unnamed arguments to the unresolved indirect-jump stub
  `0x802DCBFC`. The module header already records that stub as unresolved, so
  a body here would be a signature guess.
- **The `0x80605BA0` row builder.** Translated nowhere and 1,164 bytes of
  paired-single-free visibility arithmetic over an explicit window bound; it is
  documented as an `// unknown:` callee by [`fn_80606030`] rather than
  reconstructed from call sites alone.
- **The `0x80609440` family and the `0x8060A5D0`/`0x8060A950` block.** They are
  larger manager bodies outside the three screen tails selected for this pass;
  they remain for a later sweep.

## Notes on inference

- No class name is present in these exports. The strings
  `characterChangeSelectContents_05`, `N_cSelectCon_00`, `PbuttonShadow_00`
  and `T_zanki_01` identify the contents screen; the earlier wave-62/63 prose
  and the shared `+0x2D4`/`+0x2D8`/`+0x2E0` layout identify the select screen
  and the coin-date screen. All three names are marked `// inferred:` in the
  module prose, never as fact.
- `0x80607740` is declared `void FUN_80607740(void)` in the export. Its `r3`
  is recovered through the `0x802DCD5C` prologue-save fragment exactly as
  [`fn_80607120`] does with `0x802DCD74`; the body uses that value as the
  screen pointer and calls `0x802DCDA8` at the end.
- The `0x80605A20`/`0x80605AA0`/`0x80605B20` trio writes pane slots
  `index + 2`, `index + 4` and `index + 6`; the binding call in
  [`fn_806057b0`] covers eight `PbuttonShadow_00` handles, so the supplied
  `index` is range-limited by the caller. The bodies keep the literal
  arithmetic and do not invent a bound.
- The sound ids (`0x365`/`0x366`, `0x3AC`/`0x3AD`, `0x3F2`/`0x3F3`, `0x7A`,
  `0x7D`), the state descriptors (`0x80822A58`..`0x808231B0`), the archive
  strings and the frame-array globals (`0x80429F50`, `0x8042A464`) keep their
  raw addresses; only the three screen families are named in prose.
- `0x80607C50` reads the same `0x80429F50`/`0x8042A464` frame record used by
  [`fn_80603260`]; its bit `3`/bit `2` routing is literal, while the two
  destinations are named from the wave-63 cursor helpers.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 4,732 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 130
  translated addresses were re-read first and none is duplicated here. The file
  now holds 2,911 bodies.
- `node tools/check-lane-csv.mjs wave116` reports `OK` and names no problem in
  `wave116-bases.csv`.
- `cargo check --message-format=short` reports no error naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
