# Coverage - Wave 102 (`game::bases`, `0x8061` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the
`d_basesNP` REL. Every address on this page is a **static analysis address**
under the canonical synthetic base **`0x80600000`** (produced by
`tools/rel2elf.mjs`), never a runtime address: the Wii REL loader chooses the
real load address, so the numbers only identify positions inside the REL image.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (checked with a glob,
never assumed). "Confidence" is `high` when guards, stores, constants and call
arguments are literal, and `medium` when a field role, a global, a callee or the
owning class had to be inferred (those bodies carry `// observed:` /
`// inferred:` / `// unknown:` notes). No body in this pass is `low`.

The bucket holds 241 functions; 119 were already in `ledger/functions.csv`
when this pass began, leaving 122. This pass takes 30 of them, all inside the
`0x8061` menu/screen band.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3,272 | 24 | 6 |

## What this pass covers

- **The `dFileSelect_c` copy/delete confirmation tail
  (`0x806181B0..0x80618AC0`, 11 bodies).** This is the half of the file-select
  screen that runs *after* a copy or delete was confirmed and wave 53 stopped.
  It contributes the screen entry [`fn_806181b0`], the save-file-manager setup
  [`fn_80618330`] (branch word `+0x25C`; two file indices `+0x2B4`/`+0x2B8` for
  the copy branch, one `+0x2C8` for the delete branch), the manager completion
  poll [`fn_80618420`] (sound `0xA2`, game flags `+0xB8F`/`+0xB50`), the two
  end-message writers [`fn_80618560`]/[`fn_80618600`] and five clip-driven
  transitions plus the two-clip starter [`fn_80618910`]. The state descriptors are named from the table registered by
  [`fn_80618d00`]: `DeleteOtherDispOff` (`0x808249F0`),
  `DeleteConfirmationButtonOnStage` (`0x80824A70`),
  `ConfirmationWindowOsStage` (`0x808246B0`), `DeleteOtherDispOn`
  (`0x80824AF0`), `NowCopyOrDeleteDispExitAnimeEndWait` (`0x808248F0`),
  `CopyOrDeleteEndMessageWindowExitAnimeEndWait` (`0x80824970`) and
  `MultiButtnOnStageAnimeEndWait` (`0x808240B0`).
- **The `dLetterWindow_c` layout and button-clip tail
  (`0x8061C310..0x8061CEC0`, 15 bodies).** The class itself is proven by the
  constructor [`fn_8061c560`], whose interface context `0x80824C40` is the
  first `dLetterWindow_c` state descriptor, and by the six-entry table
  [`fn_8061d110`]. This pass adds the `letterWindow/letterWindow.arc` loader
  [`fn_8061c8a0`] (brlyt `letterWindow_00.brlyt`, the five-clip
  `inWindow/inBtn/loopBtn/hitBtn/outWindow` group at `0x807D7F88`, animation id
  at `+0x244`, `T_message_00` at `+0x248`, `P_line_00` at `+0x24C`), the text
  writer [`fn_8061cae0`], the clip starts/stops
  [`fn_8061cc40`]/[`fn_8061cce0`]/[`fn_8061cdb0`]/[`fn_8061ce80`] and the five
  idle transitions to `StartWait`, `ButtonOnStageAnimeEndWait`,
  `ButtonInputWait`, `HitAnimeEndWait` and `ExitAnimeEndWait`. The
  `infoWindow/infoWindow.arc` loader [`fn_8061c310`] sits in the same window
  but binds its layout directly at `+0x70`; its owning class is
  `// unknown:`. The three element destructor wrappers
  [`fn_8061c660`]/[`fn_8061c6a0`]/[`fn_8061c700`] are the family's array
  release shapes (`+0x18` sub-object plus the `0x802B9280` free).
- **The `dMessageWindow_c` tail (`0x8061DE40..0x8061DFE0`, 4 bodies).** The
  idle transitions to `ExitWait` (`0x80824E40`) and `StartWait`
  (`0x80824DC0`), the `StartAnimeEndWait` clip start
  [`fn_8061df10`], and the small container append helper
  [`fn_8061dfe0`]. The shared descriptor table is [`fn_8061e1b0`] and the
  registered destructor is [`fn_8061e4f0`].

Start with [`fn_8061c8a0`] for the layout binding, [`fn_8061cae0`] for the
text/player-scan pattern and [`fn_80618330`] for the save-manager flow.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806181B0` | `fn_806181b0` | 52 | medium |
| `0x80618330` | `fn_80618330` | 228 | medium |
| `0x80618420` | `fn_80618420` | 192 | medium |
| `0x80618560` | `fn_80618560` | 148 | high |
| `0x80618600` | `fn_80618600` | 148 | high |
| `0x80618740` | `fn_80618740` | 88 | high |
| `0x806188A0` | `fn_806188a0` | 88 | high |
| `0x80618910` | `fn_80618910` | 72 | high |
| `0x80618960` | `fn_80618960` | 84 | high |
| `0x806189F0` | `fn_806189f0` | 84 | high |
| `0x80618AC0` | `fn_80618ac0` | 88 | high |
| `0x8061C310` | `fn_8061c310` | 212 | medium |
| `0x8061C660` | `fn_8061c660` | 64 | high |
| `0x8061C6A0` | `fn_8061c6a0` | 96 | high |
| `0x8061C700` | `fn_8061c700` | 100 | high |
| `0x8061C8A0` | `fn_8061c8a0` | 212 | high |
| `0x8061CAE0` | `fn_8061cae0` | 228 | medium |
| `0x8061CBD0` | `fn_8061cbd0` | 88 | high |
| `0x8061CC40` | `fn_8061cc40` | 64 | high |
| `0x8061CC80` | `fn_8061cc80` | 88 | high |
| `0x8061CCE0` | `fn_8061cce0` | 60 | high |
| `0x8061CD30` | `fn_8061cd30` | 104 | high |
| `0x8061CDB0` | `fn_8061cdb0` | 96 | high |
| `0x8061CE10` | `fn_8061ce10` | 88 | high |
| `0x8061CE80` | `fn_8061ce80` | 64 | high |
| `0x8061CEC0` | `fn_8061cec0` | 88 | high |
| `0x8061DE40` | `fn_8061de40` | 88 | high |
| `0x8061DF10` | `fn_8061df10` | 76 | high |
| `0x8061DF60` | `fn_8061df60` | 88 | high |
| `0x8061DFE0` | `fn_8061dfe0` | 96 | medium |

**Totals:** 30 translated, 0 partial, 0 stub (24 high, 6 medium, 3,272 bytes).

## Deliberate skips

Nine untranslated neighbours in the same window were read and rejected:

- `0x80615CC0` (36 bytes), `0x80615CF0` (36 bytes) and `0x80615D90` (100
  bytes) are truncated by bad instruction data: each export ends in
  `halt_baddata()` after the first store, so the rest of the body is unknown.
  [`fn_806181b0`] therefore calls their wrappers `0x80615E00`/`0x80615E40` by
  address with a `// partial:` note instead of using them as Rust helpers.
- `0x80615A80` (204 bytes) opens with an `unaff_GQR0`/`ldexpf` paired-single
  save bracket; the incoming PSQ registers cannot be reconstructed.
- `0x8061D0B0` (48 bytes), `0x8061D0E0` (48 bytes), `0x80618CA0` (48 bytes) and
  `0x80618CD0` (48 bytes) are two-word stubs that tail-call the unresolved
  indirect-jump helper `0x802DCBFC`; the same trap is documented in the module
  header and in `docs/COVERAGE_r80260_802ed.md`.
- `0x8061DC70` (460 bytes) reads two uninitialised registers (`unaff_r28`) and
  a stack buffer whose producer the decompiler dropped; the text-draw argument
  list cannot be proven.

The remaining 83 untranslated functions of the bucket lie outside these three
windows (budget, not doubt).

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 3,272 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- The literal string and table addresses were checked against the REL bytes
  (`address = 0x80600000 + file_offset`): `0x807D7E60`
  (`infoWindow/infoWindow.arc`), `0x807D800C` (`letterWindow_00.brlyt`),
  `0x807D7FEC` (`letterWindow/letterWindow.arc`), `0x807D7F88`..`0x807D7FE8`
  (the clip and pane pointer table), `0x807C6C94` (the text row table) and
  `0x807C6C70`/`0x807C6C80` (the shared pane-animation tables).
- No address was already in `ledger/functions.csv`: the bucket's 119
  translated addresses were re-read first and none is duplicated here.
- `node tools/check-lane-csv.mjs wave102` reports `wave102-bases.csv: 30 rows`
  and no problem naming this lane.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; the crate's remaining errors belong to the parallel
  wave-102 lanes (`src/runtime/input.rs`).
