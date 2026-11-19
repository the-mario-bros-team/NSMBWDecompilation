# Coverage - Wave 62 (`game::bases`, `0x8060` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address here is a static analysis
address, not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the cited evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when guards, stores, call arguments and constants are literal, and
`medium` when a callee identity, a field role or a lost argument had to be
inferred (`// inferred:` / `// unknown:` in the body).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3928 | 14 | 16 |

## What this pass covers

- The `dCourseSelectMenu_c` member tail at `0x8060D1B0..0x8060D990`, the object
  whose state table `0x8060DE60`, destructor `0x8060E390` and in-place
  constructor `0x8060E4B0` were recorded earlier. This pass adds the four
  element destructor wrappers (`0x8060D1B0`, `0x8060D1F0`, `0x8060D230`,
  `0x8060D290`), the `corseSelectMenu_11.brlyt` loader [`fn_8060d4f0`] (archive
  `corseSelectMenu_corseSelectMenu.arc`, panes `A00_Window`, `P_SBBase_01`,
  `T_corseSelectS03`, `T_corseSelectS01`), the input-availability fold
  [`fn_8060d5e0`], the exit guard [`fn_8060d620`], the window rebuild
  [`fn_8060d740`] (panes `2..6`, visible bit at `+0xBB`, event `0x80823710`),
  the open/close animation gates [`fn_8060d810`] / [`fn_8060d910`], the pane
  slot writer [`fn_8060d890`] (cue `0x78`, frames `slot+0xE` and `slot+2`) and
  the confirm/cancel handler [`fn_8060d990`] (slot walk over pad bits `1/4`,
  `0x900` view allocation, events `0x80823750`, `0x80823810`, `0x808237D0`).
- The `dCourseTimeUp_c` member tail at `0x8060E5E0..0x8060EDB0`, following the
  state table `0x8060F010` and destructor `0x8060F370` from wave 12. Bodies:
  three element destructor wrappers, the `timeUpinCourse_01.brlyt` loader
  [`fn_8060e840`] (archive `timeUpinCorse.arc`, `A00_timeUp`, `T_timeUP_00`),
  the countdown gate [`fn_8060e900`], the `Wm_2d_timeup` transform/spawn
  writer [`fn_8060ea50`], the countdown start [`fn_8060eb50`], the banner
  show/hide pair [`fn_8060ec00`] / [`fn_8060edb0`], the event gate
  [`fn_8060ec60`], the scroll arm [`fn_8060ecd0`] and the exit animation
  [`fn_8060ed60`].
- The `dDateFile_c` member tail at `0x8060F6D0..0x8060FFB0`: the element
  destructor wrapper, the `dateFile_09.brlyt` loader [`fn_8060f8b0`] (archive
  `dateFile_dateFile.arc`, `N_dateFile_00`, `W_select_00`,
  `T_worldNumber_00`, `P_Star_00`, plus the two file-count format calls), the
  opening reset [`fn_8060fce0`] (animation constants, animation `5`, the
  `+0x1F8 + 0xC8` channel float), the seven-case transition dispatcher
  [`fn_8060fe20`] (events `0x80823958`..`0x80823AD8`, the save-stop case and
  the scene resume), the save-frame writer [`fn_8060ff70`] and the close gate
  [`fn_8060ffb0`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8060D1B0` | `fn_8060d1b0` | 64 | medium |
| `0x8060D1F0` | `fn_8060d1f0` | 64 | medium |
| `0x8060D230` | `fn_8060d230` | 96 | medium |
| `0x8060D290` | `fn_8060d290` | 100 | medium |
| `0x8060D4F0` | `fn_8060d4f0` | 240 | high |
| `0x8060D5E0` | `fn_8060d5e0` | 64 | medium |
| `0x8060D620` | `fn_8060d620` | 100 | medium |
| `0x8060D740` | `fn_8060d740` | 160 | high |
| `0x8060D810` | `fn_8060d810` | 88 | high |
| `0x8060D890` | `fn_8060d890` | 124 | medium |
| `0x8060D910` | `fn_8060d910` | 88 | high |
| `0x8060D990` | `fn_8060d990` | 380 | medium |
| `0x8060E5E0` | `fn_8060e5e0` | 64 | medium |
| `0x8060E620` | `fn_8060e620` | 96 | medium |
| `0x8060E680` | `fn_8060e680` | 100 | medium |
| `0x8060E840` | `fn_8060e840` | 192 | high |
| `0x8060E900` | `fn_8060e900` | 84 | medium |
| `0x8060EA50` | `fn_8060ea50` | 240 | medium |
| `0x8060EB50` | `fn_8060eb50` | 132 | medium |
| `0x8060EC00` | `fn_8060ec00` | 84 | high |
| `0x8060EC60` | `fn_8060ec60` | 88 | high |
| `0x8060ECD0` | `fn_8060ecd0` | 60 | high |
| `0x8060ED60` | `fn_8060ed60` | 68 | high |
| `0x8060EDB0` | `fn_8060edb0` | 120 | high |
| `0x8060F6D0` | `fn_8060f6d0` | 100 | medium |
| `0x8060F8B0` | `fn_8060f8b0` | 308 | high |
| `0x8060FCE0` | `fn_8060fce0` | 164 | medium |
| `0x8060FE20` | `fn_8060fe20` | 308 | high |
| `0x8060FF70` | `fn_8060ff70` | 64 | high |
| `0x8060FFB0` | `fn_8060ffb0` | 88 | high |

## Address range

`0x8060D1B0..0x8060FFB0`, three contiguous member-tail windows separated by the
already-translated state tables and constructors of the same screens
(`0x8060DE60`, `0x8060E390`, `0x8060E4B0`, `0x8060F010`, `0x8060F370`).

## Deliberate skips

- `0x8060D6F0` (64 bytes) is truncated by bad instruction data in the export
  (`halt_baddata`), so its body was not reconstructed. It is still called from
  [`fn_8060d990`] with the screen pointer restored in `r3`; the call is marked
  `// unknown:`.
- `0x8060FB50` (396 bytes) contains paired-single `GQR` loads/stores and the
  decompiler lost the transform inputs, so it was left for a later pass.
- `0x8060F630` (64) and `0x8060F670` (96) were left untranslated: they are the
  same one-line free wrappers as the recorded element destructors and add no
  new behavior; they remain in the bucket for completeness passes.
- `0x806001E0` and `0x806001F0` (4 bytes each) are empty `return` thunks with
  no behavior to reconstruct.
- The rest of the `0x8060` bucket outside this pass remains for later waves,
  including the large bodies `0x806043C0` (3860), `0x80605BA0` (1164) and
  `0x80609440` (1040).

At lane start the `0x8060` bucket held 240 functions, 64 of them already
translated. This pass adds 30, leaving 146 for later waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv` (checked against the whole
  ledger, not just `game::bases`).
- `node tools/check-lane-csv.mjs wave62` reports `OK` and
  `wave62-bases.csv: 30 rows`.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/bases.rs`.
