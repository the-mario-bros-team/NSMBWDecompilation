# Coverage - Wave 55 (`game::bases`, `0x8062` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address is a static analysis address,
not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores, animation ids and descriptor addresses are
literal, and `medium` when a callee role, a dropped argument or a table's
meaning had to be inferred.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4172 | 24 | 6 |

## What this pass covers

- The member-function body of the mini-game screen whose constructor is the
  wave-14 [`fn_806203d0`] (`+0x70` NW4R layout with its vtable word at `+0x78`,
  interface group at `+0x208`, root pane at `+0x244`, ten pane pointers at
  `+0x248..+0x270`, per-row panes at `+0x2B8..+0x2E4`, digit group at `+0x2E8`,
  picture group at `+0x30C`, four choice panes at `+0x32C..+0x338`, selected
  choice `+0x350`, mode/count word `+0x354`, seven per-player counts
  `+0x358..+0x370` and flag bytes `+0x378..+0x382`). The whole free run
  `0x806204D0..0x80621DC0` was translated; the two boundaries are the already
  recorded destructor [`fn_80622ce0`] and constructor [`fn_806203d0`].
- Resource loading and data access: the two release wrappers
  [`fn_806204d0`] / [`fn_80620510`], the `miniGameWire` archive, layout,
  animation, message and pane loader [`fn_806207a0`], the static record query
  [`fn_806209d0`] (`0x807C6E28 + (row - 1) * 0x1C + slot * 4`) and the
  seven-slot decimal counter writer [`fn_80620b60`].
- Pane visibility: the four-choice selector [`fn_80620c30`], the full reset
  [`fn_80620d70`], the turn-over-face switch [`fn_80621870`], the three-mode
  pane groups of [`fn_806211e0`] and the per-player rebuild [`fn_80621630`].
- State transitions: every handler that waits for the layout to stop and then
  dispatches a descriptor registered by the wave-11 table [`fn_80622010`] —
  `TitleDisp` (`0x808251C0`), `StartWait` (`0x80825240`), `NowDisp`
  (`0x808252C0`), `ResultDispAnimeEndWait` (`0x80825300`), `ResultNowDisp`
  (`0x80825340`), `BadResultNowDisp` (`0x80825380`), `TurnOverFaceOpenAnimeEndWait`
  (`0x80825400`), `TurnOverFaceDisp` (`0x80825440`), `FaceChengeWait`
  (`0x80825480`), `TurnOverFaceExitAnimeEndWait` (`0x808254C0`),
  `TurnOverExitAnimeEndWait` (`0x80825500`) and `ExitAnimeEndWait`
  (`0x80825540`) — plus the animation starts `0x11`..`0x1A` and the colour reset
  [`fn_80621dc0`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806204d0` | `fn_806204d0` | 64 | medium |
| `0x80620510` | `fn_80620510` | 96 | medium |
| `0x806207a0` | `fn_806207a0` | 280 | high |
| `0x806209d0` | `fn_806209d0` | 36 | medium |
| `0x80620b60` | `fn_80620b60` | 192 | medium |
| `0x80620c30` | `fn_80620c30` | 200 | high |
| `0x80620d70` | `fn_80620d70` | 332 | high |
| `0x80620ec0` | `fn_80620ec0` | 88 | high |
| `0x80620f30` | `fn_80620f30` | 64 | high |
| `0x80620fb0` | `fn_80620fb0` | 64 | high |
| `0x80620ff0` | `fn_80620ff0` | 96 | high |
| `0x806211e0` | `fn_806211e0` | 828 | high |
| `0x80621520` | `fn_80621520` | 172 | high |
| `0x80621630` | `fn_80621630` | 160 | medium |
| `0x806216d0` | `fn_806216d0` | 84 | high |
| `0x80621730` | `fn_80621730` | 56 | high |
| `0x80621770` | `fn_80621770` | 104 | high |
| `0x80621870` | `fn_80621870` | 200 | high |
| `0x80621940` | `fn_80621940` | 88 | high |
| `0x806219b0` | `fn_806219b0` | 60 | high |
| `0x806219f0` | `fn_806219f0` | 88 | high |
| `0x80621a60` | `fn_80621a60` | 64 | high |
| `0x80621aa0` | `fn_80621aa0` | 140 | high |
| `0x80621b80` | `fn_80621b80` | 52 | high |
| `0x80621bc0` | `fn_80621bc0` | 64 | high |
| `0x80621c00` | `fn_80621c00` | 88 | high |
| `0x80621c80` | `fn_80621c80` | 132 | high |
| `0x80621d20` | `fn_80621d20` | 64 | high |
| `0x80621d60` | `fn_80621d60` | 96 | high |
| `0x80621dc0` | `fn_80621dc0` | 120 | medium |

## Deliberate skips

- `0x80620a00` (340 bytes) is the colour-table refresh called by
  [`fn_80621630`]. Its export opens with a GQR/`ldexpf` pair and a stacked
  double constant, so Ghidra models the loop counter and the vector stores
  through unaffiliated condition-register state; translating it would invent
  the quantisation. [`fn_80621630`] keeps it as an address-only call.
- `0x80620570` is byte-identical to [`fn_80620510`] (same three calls, same
  guard); only one copy was translated so the lane does not pad its count with a
  duplicate body.
- `0x80621fb0` and `0x80621fe0` are 48-byte adjustor thunks whose whole body is
  `0x802DCBFC(a2, a2, a1)` with no recoverable object role; like the wave-52 and
  wave-54 thunks they add no evidence.
- The rest of the `0x8062` bucket (150 untranslated at lane end, mostly the
  `dModelPlayManager_c` and `dModeSelect_c` bodies above `0x80622E00`) is
  outside this window.

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
- `node tools/check-lane-csv.mjs wave55` reports `OK` for `wave55-bases.csv`.
