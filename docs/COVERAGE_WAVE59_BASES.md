# Coverage - Wave 59 (`game::bases`, `0x8064` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address is a static analysis address,
not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when guards, stores, call arguments and constants are literal, and
`medium` when a callee identity, a dropped receiver argument or a field role
had to be inferred (`// inferred:` / `// unknown:` in the body).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 4020 | 18 | 11 |

## What this pass covers

- The `dStaffCreditScore_c` helper tail, `0x80640530..0x80641300`. The screen's
  in-place constructor is [`fn_80640480`] and its eight-state table is
  [`fn_806415d0`]; these bodies operate on the class layout at `+0x70`
  (`staffCredit` `.arc`/`.brlyt`), the four score columns at
  `+0x288/+0x28C/+0x290/+0x294`, the four latches at `+0x298..+0x2A4`, the
  per-column markers at `+0x281..+0x284`, the running maximum `+0x2B0` and the
  winning index `+0x2B4`.
- The two release helpers [`fn_80640530`] / [`fn_80640570`] and the two
  sub-object destructors [`fn_806405b0`] / [`fn_80640610`] (both pairs are
  byte-identical twins at different addresses).
- The layout loader [`fn_80640890`], which mounts
  `staffCredit/staffCredit.arc`, loads `staffCredit_00.brlyt`, binds the
  `staffCredit_00_inPlayerScore` pane table at `0x807E1B18`, the `A00_1P` set at
  `0x807E1B58` with the float table `0x807C8218`, the six `T_coin_00` panes at
  `+0x248` and the six `N_mario_00` panes at `+0x260`, and publishes the root
  pane at `+0x244`.
- The column machinery: the re-latch loop [`fn_80640b40`], the full reset
  [`fn_80640bb0`], the two pane-animation ramps [`fn_80640c80`] (animations
  `0..3` then `8`) and [`fn_80640d80`] (animations `4..7`), and the four-way
  winner scan [`fn_80640ea0`] that raises `+0x2B0`, records `+0x2B4`, marks
  every tied column and plays sound `9`/`10`/`12`/`11` from the `0x80354E60`
  character table.
- The message/animation gates [`fn_80640d10`], [`fn_80640de0`],
  [`fn_80641040`], [`fn_806410d0`], [`fn_80641140`], [`fn_806411d0`],
  [`fn_80641280`] and [`fn_80641300`], which dispatch the
  `0x80827740`/`0x80827780`/`0x808277C0`/`0x80827800`/`0x80827880` interface
  blocks and publish the achieved score into the player and the result label.
- The water-bubble actor bodies [`fn_8064b5e0`] (animation bind),
  [`fn_8064b910`] / [`fn_8064baa0`] (three-band split of the wrapped
  `+0x394` timer), [`fn_8064bc60`] (mark every `0x19A` peer) and
  [`fn_8064bd80`] (the `+0x3D8`/`+0x3D4` countdown pair).
- The `MG_kinopio` bodies [`fn_8064c620`] (transform and `face_1` bone),
  [`fn_8064c700`] (the `KB_trs_open`/`KB_trs_jump` pair selected from the
  two-word table at `0x807C85F0`), [`fn_8064c8d0`] and [`fn_8064cc00`]
  (release virtuals) and [`fn_8064cc30`] (transform plus the
  `0x807C860C`/`0x807C8610` colour triple).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80640530` | `fn_80640530` | 64 | high |
| `0x80640570` | `fn_80640570` | 64 | high |
| `0x806405B0` | `fn_806405b0` | 96 | high |
| `0x80640610` | `fn_80640610` | 100 | high |
| `0x80640890` | `fn_80640890` | 244 | medium |
| `0x80640B40` | `fn_80640b40` | 112 | medium |
| `0x80640BB0` | `fn_80640bb0` | 128 | high |
| `0x80640C80` | `fn_80640c80` | 132 | high |
| `0x80640D10` | `fn_80640d10` | 88 | high |
| `0x80640D80` | `fn_80640d80` | 96 | high |
| `0x80640DE0` | `fn_80640de0` | 176 | high |
| `0x80640EA0` | `fn_80640ea0` | 416 | medium |
| `0x80641040` | `fn_80641040` | 128 | high |
| `0x806410D0` | `fn_806410d0` | 104 | medium |
| `0x80641140` | `fn_80641140` | 140 | high |
| `0x806411D0` | `fn_806411d0` | 88 | medium |
| `0x80641280` | `fn_80641280` | 116 | high |
| `0x80641300` | `fn_80641300` | 180 | medium |
| `0x8064B5E0` | `fn_8064b5e0` | 116 | high |
| `0x8064B910` | `fn_8064b910` | 196 | medium |
| `0x8064BAA0` | `fn_8064baa0` | 36 | high |
| `0x8064BC60` | `fn_8064bc60` | 72 | high |
| `0x8064BCD0` | `fn_8064bcd0` | 124 | medium |
| `0x8064BD80` | `fn_8064bd80` | 160 | high |
| `0x8064C620` | `fn_8064c620` | 212 | medium |
| `0x8064C700` | `fn_8064c700` | 216 | high |
| `0x8064C8D0` | `fn_8064c8d0` | 84 | high |
| `0x8064CC00` | `fn_8064cc00` | 48 | medium |
| `0x8064CC30` | `fn_8064cc30` | 284 | medium |

## Address range

`0x80640530..0x80641300`: the contiguous `dStaffCreditScore_c` helper run
(one `0x80640AC0` gap) — eighteen bodies.

`0x8064B5E0..0x8064CC30`: the water-bubble animation/span/timer helpers and
the `MG_kinopio` transform and release bodies — eleven bodies, with the
in-bucket `0x8064B660..0x8064C5FF` read/update code left outside this window.

The pass therefore covers twenty-nine of the 172 addresses that were
untranslated in the `0x8064` bucket at lane start (196 total; 24 were already
in `ledger/functions.csv`; 143 remain for later waves).

## Deliberate skips

- `0x80640AC0` (116 bytes) is a `halt_baddata()` stub in the export ("bad
  instruction data"). Its call site is preserved literally as
  `abi_call2!(0x80640AC0, owner, column)` inside [`fn_80640b40`], but no body
  was written because there is no recoverable instruction stream.
- `0x80641570` (48) and `0x806415A0` (48) are the four-word `0x802DCBFC`
  forwarders already represented by `fn_806512f0`/`fn_80651320`; they add no
  new evidence and were not duplicated.
- These addresses were already in `ledger/functions.csv` and were not
  duplicated: `0x80640180`, `0x80640240`, `0x80640480`, `0x806415D0`,
  `0x80641C00`, `0x80641D20`, `0x806441E0`, `0x806448C0`, `0x806449E0`,
  `0x80644B30`, `0x80644EF0`, `0x80645820`, `0x80645B60`, `0x80645C80`,
  `0x806465F0`, `0x80646930`, `0x8064A6D0`, `0x8064B1C0`, `0x8064B370`,
  `0x8064C3A0`, `0x8064C460`, `0x8064D790`, `0x8064E5F0`, `0x8064E870`.
- `0x8064B660` (84) calls the unresolved `0x802DCBFC` stub with only one
  argument while the export shows the receiver and model release first; the
  argument count is not recoverable, so it was left for a later wave.
- The rest of the `0x8064` bucket (143 untranslated at lane end, including the
  `0x80641E10..0x80647Fxx` credit/decor tail, the `0x8064B6xx..0x8064C5FF`
  water-bubble update code and the `0x8064CD60..0x8064F8xx` `MG_kinopio`
  state bodies) is outside this window.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave59` reports `OK` for
  `ledger/agents/wave59-bases.csv` (29 rows, 18 high / 11 medium).
- `cargo check --message-format=short` reports no error for
  `src/game/bases.rs`.
