# Coverage - Wave 53 (`game::bases`, `dDateFile_c` tail and `dFileSelect_c` confirmation/delete states)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address is a static analysis address,
not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores, animation ids and descriptor addresses are
literal, and `medium` when a role or a dropped `this` argument had to be
inferred from the surrounding calls.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3660 | 25 | 5 |

## What this pass covers

- The `dDateFile_c` animation tail, `0x80610020..0x80610320`: the layout
  animation starts (`2`, `5`, `6`, `3`, `4`), the idle gate that selects
  `ActionWait` (`0x80823998`) or `InterruptSaveMarkExit` (`0x80823A58`) from the
  `+0xE4B`/`0xE4E` flags, the `+0xDD8`-driven `OnAnimeEndWait` (`0x808239D8`)
  branch, the shared `+0xE4D` update flag, and the button/digit renderer
  [`fn_80610320`] that scans controller buttons `8..0`, formats the index into
  the `+0xDFC` text object and the masked face byte into `+0xE04`, then drives
  the five visibility panes at `+0xE18..+0xE28` from pad byte `+2`.
- The `dFileSelect_c` confirmation/delete continuation,
  `0x80617860..0x80618AC0`: the copy, confirmation and deletion state handlers
  that follow the wave-38 file-select block. Transitions are quoted from the
  descriptors registered by [`fn_80618d00`], so each handler names the real
  state it enters (`CopyNoSelectButtonOff`, `CopyButtonMoveEndWait`,
  `ConfirmationButtonAnimeEndWait`, `ConfirmationButtonSelect`,
  `ConfirmationButtonExit`, `ConfirmationWindowExit`,
  `NowCopyOrDeleteDispWait`, `DeleteFileCenterToBack`,
  `DeleteFileCenterMove`, and the two `MultiButtnOnStageAnimeEndWait` /
  `ToCopyInfoWindowOnStage` branches).
- The menu-choice dispatcher [`fn_80617d70`]: pad bits `0x8`/`0x4` move the
  `+0x260` choice, an unchanged choice queries allocator `0x8042A468` for
  `0x900` bytes and then starts or refuses the game-start transition, and a
  changed choice writes `+0x264`/`+0x260` and returns to
  `ConfirmationButtonAnimeEndWait` (`0x808246F0`).
- The sound cues `0x78`, `0x79` and `0x7A` and the three per-player sub-object
  helpers: [`fn_806187b0`] hides and [`fn_80618a60`] shows every player pane
  except the current `+0x2C8` index.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80610020` | `fn_80610020` | 64 | high |
| `0x80610060` | `fn_80610060` | 144 | high |
| `0x80610100` | `fn_80610100` | 76 | high |
| `0x80610150` | `fn_80610150` | 88 | high |
| `0x806101c0` | `fn_806101c0` | 60 | high |
| `0x80610230` | `fn_80610230` | 96 | high |
| `0x80610290` | `fn_80610290` | 128 | high |
| `0x80610320` | `fn_80610320` | 488 | high |
| `0x80617860` | `fn_80617860` | 88 | high |
| `0x806178d0` | `fn_806178d0` | 64 | medium |
| `0x80617910` | `fn_80617910` | 88 | high |
| `0x80617980` | `fn_80617980` | 52 | medium |
| `0x806179c0` | `fn_806179c0` | 84 | high |
| `0x80617a30` | `fn_80617a30` | 88 | high |
| `0x80617a90` | `fn_80617a90` | 84 | high |
| `0x80617b00` | `fn_80617b00` | 100 | medium |
| `0x80617b70` | `fn_80617b70` | 88 | high |
| `0x80617bf0` | `fn_80617bf0` | 212 | high |
| `0x80617cd0` | `fn_80617cd0` | 88 | high |
| `0x80617d70` | `fn_80617d70` | 392 | medium |
| `0x80617f10` | `fn_80617f10` | 96 | high |
| `0x80617f70` | `fn_80617f70` | 140 | high |
| `0x80618010` | `fn_80618010` | 100 | high |
| `0x80618080` | `fn_80618080` | 84 | high |
| `0x80618100` | `fn_80618100` | 156 | medium |
| `0x806181f0` | `fn_806181f0` | 84 | high |
| `0x80618280` | `fn_80618280` | 156 | high |
| `0x806187b0` | `fn_806187b0` | 92 | high |
| `0x80618810` | `fn_80618810` | 88 | high |
| `0x80618a60` | `fn_80618a60` | 92 | high |

## Deliberate skips

- `0x80610510` (460 bytes) is the `dDateFile_c` per-frame controller updater.
  Its first animation call passes a float through the paired-single register
  pair (`func_0x801637d0((double)*(float *)(iVar4 + 0xE40), *puVar1)`), and
  Ghidra's prototype cannot distinguish the single/double form. The body could
  only be written with an invented floating-point ABI, so it was left out.
- `0x80610850` and `0x80610880` are 48-byte adjustor thunks whose whole body is
  `0x802DCBFC(a2, a2, a1)` with no recoverable object role; like the wave-52
  thunks they add no evidence.
- `0x806149a0` and the following `0x80614A30..0x80614D40` handlers belong to a
  different screen and were outside this lane's window. `0x806148f0` in that
  run decompiles with "bad instruction data" and a `halt_baddata()` tail.
- The remaining `0x8061` bucket (182 untranslated at lane start, 152 after this
  pass) is outside this window.
- The `0x80615CC0`, `0x80615CF0`, `0x80615D20` and `0x80615D90` helpers are
  still untranslated; they are called by address here and their bodies carry
  `halt_baddata()` tails from the state-table jump, so they were not wrapped.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave53` reports `OK` for `wave53-bases.csv`.
