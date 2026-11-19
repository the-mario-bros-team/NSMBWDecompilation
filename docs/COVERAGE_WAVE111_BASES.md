# Coverage - Wave 111 (`game::bases`, `0x8063` bucket)

## How to read this file

This page documents the Wave 111 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`; byte counts were re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

"Confidence" is `high` when the guards, stores, constants and call arguments are
literal, and `medium` when a field role, a callee identity or a dropped receiver
had to be inferred. No body in this pass is `low`. Comments in the bodies keep
`// observed:`, `// inferred:` and `// unknown:` markers.

The `0x8063` bucket holds 201 functions; 87 were already in
`ledger/functions.csv`, leaving 114. This pass takes 25 of them in two clusters.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 25 | 7,032 | 12 | 13 |

## What this pass covers

- **The character-change contents screen member tail
  (`0x80632650..0x80634800`, 23 bodies).** The screen is constructed by
  [`fn_80631fe0`] and loads `characterChange_70.brlyt`; the loader
  [`fn_80632650`] binds ten `N_pos1P` panes at `+0x5DC`, the title pane at
  `+0x604`, the `yesBase` group at `+0x614` and the button group at `+0x644`.
  Around it sit the readiness gate [`fn_80632950`], the three-record pane
  placement [`fn_80632af0`], the seat bookkeeping
  [`fn_80633140`]/[`fn_806331f0`]/[`fn_80633330`]/[`fn_80633350`]/
  [`fn_80633570`], the confirm test [`fn_806336a0`], the 932-byte start-up
  [`fn_80633730`], the enter/cursor/exit machine
  [`fn_80633b20`]..[`fn_806344e0`] and the close body [`fn_80634800`].
- **The result-screen tail (`0x80636CB0..0x80637050`, 2 bodies).** The
  `pointResultDateFile_29.brlyt` loader [`fn_80636cb0`] binds the pane handle
  at `+0x6E4`, the nine-entry `N_PRDateFile` array at `+0x6E8`, the four score
  panes at `+0x70C`..`+0x73C` and the rank/date panes; [`fn_80637050`] clamps
  each saved score/star pair to `999` and formats the four 0x3C-byte panels.

Start with [`fn_80632650`] for the layout binding and [`fn_80633c50`] for the
smallest state-transition body. [`fn_80633730`] is the field map for the whole
screen: `+0x404` layout, `+0x59C` interface, `+0x5DC` pane array, `+0x64C`
player records, `+0x690` status words, `+0x6A0` selected count, `+0x6E0` seat
pairs and `0x80354E60` the shared seat array.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80632650` | `fn_80632650` | 364 | high |
| `0x80632950` | `fn_80632950` | 144 | medium |
| `0x80632AF0` | `fn_80632af0` | 712 | medium |
| `0x80633140` | `fn_80633140` | 176 | high |
| `0x806331F0` | `fn_806331f0` | 308 | medium |
| `0x80633330` | `fn_80633330` | 28 | high |
| `0x80633350` | `fn_80633350` | 120 | high |
| `0x80633570` | `fn_80633570` | 292 | medium |
| `0x806336A0` | `fn_806336a0` | 116 | medium |
| `0x80633730` | `fn_80633730` | 932 | medium |
| `0x80633B20` | `fn_80633b20` | 296 | medium |
| `0x80633C50` | `fn_80633c50` | 112 | high |
| `0x80633D00` | `fn_80633d00` | 316 | medium |
| `0x80633E90` | `fn_80633e90` | 420 | medium |
| `0x80634090` | `fn_80634090` | 248 | medium |
| `0x806341E0` | `fn_806341e0` | 296 | high |
| `0x80634310` | `fn_80634310` | 168 | high |
| `0x806343D0` | `fn_806343d0` | 88 | high |
| `0x80634430` | `fn_80634430` | 124 | high |
| `0x806344E0` | `fn_806344e0` | 416 | medium |
| `0x806346B0` | `fn_806346b0` | 88 | high |
| `0x80634720` | `fn_80634720` | 224 | high |
| `0x80634800` | `fn_80634800` | 508 | medium |
| `0x80636CB0` | `fn_80636cb0` | 332 | high |
| `0x80637050` | `fn_80637050` | 204 | medium |

**Totals:** 25 translated, 0 partial, 0 stub (12 high, 13 medium, 7,032 bytes).

## Deliberate skips

Read and rejected, with the reason:

- **Paired-single bodies.** `0x806329E0` (144 bytes), `0x80632DC0` (884) and
  `0x806333D0` (416) open with an `unaff_GQR0` bracket, `ldexpf` conversions and
  incoming `ps29`/`ps30`/`ps31` registers. The incoming GQR state cannot be
  reconstructed from the export, so the halfword scaling inside them is lost.
  Same reason as `0x80711590` in wave 110 and `0x806E3EB0` in wave 109. The
  caller [`fn_80633d00`] keeps `0x806333D0` as an address call with a
  `// unknown:` note.
- **`0x802DCBFC` indirect-jump stubs.** `0x80634C90` (48 bytes) and `0x80634CC0`
  (48) are tail-call trampolines that re-emit their two incoming words through
  `0x802DCBFC`; no target or argument list is recoverable.
- **Other untranslated functions in the bucket** were not opened in this pass
  (budget, not doubt). The largest remaining runs are
  `0x80636F30..0x80639000`, `0x8063A1D0..0x8063AEE0` and
  `0x8063E870..0x8063F4E0`.

## Notes on inference

- The class name of the `0x80632650` cluster is `// inferred:`. The constructor
  [`fn_80631fe0`] is adjacent to the `dMultiCourseSelectContents_c` state table
  at `0x80631950`, and earlier waves labelled the constructor from that
  adjacency, but the loader string (`characterChange_70.brlyt`) identifies this
  screen as the character-change contents screen. The page therefore says
  "the screen built by [`fn_80631fe0`]" where the class name is not proven.
- `0x80354E60..0x80354E6C` is the shared four-seat index array. The four
  per-player status words are at `+0x690`, `+0x694`, `+0x698` and `+0x69C`;
  status `3` means "chosen". `0x80429F7C` holds the parallel per-player mode
  words at `+0x384`, `+0x388`, `+0x38C` and `+0x390`.
- `0x80428732` is the global game mode (`3` selects the four-player layout,
  `10` the special result path). `0x8042A028` is the player-manager table used
  by the existing [`player_actor`] helper; `0x8042A468` is the pad manager and
  `0x8042A488` the sound object.
- The layout calls stay addresses: `0x800C8CB0` (archive open), `0x800C8FB0`
  (animation bind), `0x800C90F0` (pane/material bind), `0x800C8D60` (pane
  array), `0x800C8F20` (two-pane group), `0x800C8EB0`/`0x800C8E40`/
  `0x800C8DD0` (pane groups), `0x800C9A60` (number text) and `0x800C92F0`
  (clip start). No Nintendo symbol is invented.
- `0x80633330` is the only body whose first argument is genuinely unused; the
  C signature keeps it because the caller passes the screen, but the lookup
  depends only on the row and column.
- [`fn_80637050`] passes a local `int[9]` and a local `int` to `0x800B3BE0`;
  the translation keeps those stack objects instead of inventing a wider
  buffer, matching the C.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (25 rows, 7,032 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 87 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,768 bodies.
- `node tools/check-lane-csv.mjs wave111` reports 25 rows for
  `wave111-bases.csv` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
