# Coverage - Wave 125 (`game::enemies`, the `0x80AA0000` bucket)

## How to read this file

This page records the Wave 125 `game::enemies` lane: 27 functions of the
`0x80AA0000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 106 of that file. Every row of the
machine-readable mirror (`ledger/agents/wave125-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

The class covered here is `daEnKild_c`: the three state strings at
`0x80B5441C..0x80B54454` read literally `daEnKild_c::StateID_Wait`,
`daEnKild_c::StateID_Slide` and `daEnKild_c::StateID_Fall`, and the model path
at `0x80B544CC` is `g3d/killer_king.brres`. The lane name therefore does not
invent a symbol.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 6,456 | 25 | 2 |

The bucket holds 188 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 134 were already in
`ledger/functions.csv`, leaving 54. This pass claims 27 of them, all in the
five runs below, and leaves 27 untouched.

The two `medium` rows are `0x80AAD010` (its two shot spawners are truncated in
the export and its `2^52` pairs are modelled as the compiler's
integer-to-double bias) and `0x80AAF1C0` (the two cable-angle roles are
inferred from the `0x6AAA`/`0x1555` complements).

## What this pass covers

- **The matrix and child plumbing (`0x80AACC00..0x80AAD010`, 5 bodies).**
  `0x80AACC00` rebuilds the `+0x58C` holder matrix from the position and Euler
  triple, commits the `+0xDC` offset and caches node 2's translation at
  `+0xDE0..+0xDE8`. `0x80AACD20` refreshes the `+0xDCC`/`+0xDD8` snapshot
  (`32.0` offset, `0.0625` factor). `0x80AACD50` is the scene band test:
  modes `+0xE18` of 1/2 and `+0xE10 == 1` use the x band widened by
  `0.5 * +0x318`; every other mode adds the y band, the `16.0` margin and the
  `64.0` floor check. `0x80AACE90` finds the first free word of the twenty-slot
  child table at `+0xB44`, `0x80AACFA0` clears every slot whose handle no
  longer resolves through `0x80162D00`, and `0x80AAD010` spawns a killer shot:
  it picks the slot's height from the `0x80B2F8E8` row, packs the side/slot/
  flags into the spawn word, links the returned record into `+0xB44`, plays
  sound `0x1C1`/`0x1BD`/`0x1C4` and raises
  `Wm_en_mgkillershot_l`/`Wm_en_mgkillershot_r`.
- **The cable machinery (`0x80AAD680..0x80AAD840`, 4 bodies).**
  `0x80AAD680` is the zero-threshold predicate whose sense is selected by
  `+0xE3E`; `0x80AAD6D0` ticks the `+0xE34` shot latch and the `+0x58C`
  holder; `0x80AAD770` arms `+0x500 = 0x20` when the spawner succeeds;
  `0x80AAD7C0` snapshots the position and the `+0xDC` triple into
  `+0xDC8..+0xDDC` and sets `+0xE2C = 2`. `0x80AAD840` is the four-phase
  slide: phases 0 and 2 wait for `+0xE2C`, gate on bits 1 and 0 of `+0xE3F`
  and call the untranslated `0x80AAD490` effect step, phases 1 and 3 walk
  `+0xE04` between `+0xE00` and `+0xDFC` by `+0xE08`, and the tail applies
  the height through `0x8007FD10`, refreshes the snapshot and clears `+0xE30`.
- **The constructor and descriptor tail (`0x80AAE240..0x80AAEB30`, 4
  bodies).** A record teardown (the same `0x8015F800`/`0x802B9280` pair used
  throughout this file), the `0x1818`-byte constructor (class vtable
  `0x80B54518`, owner interface `0x80B54798`, three `8008C200` body objects
  and the `0xF0`-stride, `0xF`-slot container at `+0x90C`), the `+0x1815`
  frame hook, and the full matrix rebuild `0x80AAEB30` that derives
  `+0x8CC`/`+0x8D0` (node 0 scaled by `18.0`), `+0x8DC`/`+0x8E0` (node 0
  scaled by `-50.0`), `+0x17DC..+0x17E4` (node 1), `+0x17E8..+0x17F0`
  (node 2) and `+0x17F4..+0x17FC` (node 0 scaled by `-118.0`).
- **The model binder and the fifteen cable segments
  (`0x80AAF050..0x80AAF6F0`, 9 bodies).** `0x80AAF050` binds
  `g3d/killer_king.brres`, the `killer_king` node (animation `0x128`) and the
  `killer_king` clip. `0x80AAF1C0` solves the two aim angles with the
  `0x4000` bias, the `+-0xE39` clamp and the `0x6AAA`/`0x1555` complements.
  `0x80AAF320` clears the fifteen `+0x994 + n*0xF0` segment counters;
  `0x80AAF370`/`0x80AAF3F0` form the per-segment roll ramp at `+0x1812` with
  its `0x80` saturation and `+0x1814` direction flip; `0x80AAF470` is the
  left/left-plus-width band test; `0x80AAF500` plays sound `0x1C2` and switches
  to the `0x80B74F90` record; `0x80AAF5A0` places the actor on one of four
  spawn planes from the `0x80B2F9D8`/`0x80B2F9E0`/`0x80B2F9E8` attitude rows
  and the `0x80B2F9F0`/`0x80B2FA00` speed rows (all read from the ELF);
  `0x80AAF6F0` broadcasts the scale/identity/visibility to all fifteen
  segments and raises `Wm_en_kingkiller` with sound `0x1C3`.
- **The second descriptor (`0x80AAFB90..0x80AAFE00`, 3 bodies).** Its
  teardown, its `0x638`-byte constructor (base word `0x80303658`, class vtable
  `0x80B548C4`, two owner interfaces at `+0x5E0`/`+0x60C` with their own
  animation holders) and the release that ticks the `+0x394`/`+0x60C`/`+0x5E0`
  holders, frees the body with `0x8002B180` and clears the collision flag.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AACC00` | `fn_80aacc00` | 284 | high |
| `0x80AACD20` | `fn_80aacd20` | 44 | high |
| `0x80AACD50` | `fn_80aacd50` | 308 | high |
| `0x80AACE90` | `fn_80aace90` | 272 | high |
| `0x80AACFA0` | `fn_80aacfa0` | 108 | high |
| `0x80AAD010` | `fn_80aad010` | 1,140 | medium |
| `0x80AAD680` | `fn_80aad680` | 76 | high |
| `0x80AAD6D0` | `fn_80aad6d0` | 96 | high |
| `0x80AAD770` | `fn_80aad770` | 68 | high |
| `0x80AAD7C0` | `fn_80aad7c0` | 100 | high |
| `0x80AAD840` | `fn_80aad840` | 364 | high |
| `0x80AAE240` | `fn_80aae240` | 88 | high |
| `0x80AAE3C0` | `fn_80aae3c0` | 280 | high |
| `0x80AAEA50` | `fn_80aaea50` | 60 | high |
| `0x80AAEB30` | `fn_80aaeb30` | 936 | high |
| `0x80AAF050` | `fn_80aaf050` | 284 | high |
| `0x80AAF1C0` | `fn_80aaf1c0` | 236 | medium |
| `0x80AAF320` | `fn_80aaf320` | 68 | high |
| `0x80AAF370` | `fn_80aaf370` | 124 | high |
| `0x80AAF3F0` | `fn_80aaf3f0` | 128 | high |
| `0x80AAF470` | `fn_80aaf470` | 96 | high |
| `0x80AAF500` | `fn_80aaf500` | 160 | high |
| `0x80AAF5A0` | `fn_80aaf5a0` | 320 | high |
| `0x80AAF6F0` | `fn_80aaf6f0` | 364 | high |
| `0x80AAFB90` | `fn_80aafb90` | 88 | high |
| `0x80AAFD10` | `fn_80aafd10` | 240 | high |
| `0x80AAFE00` | `fn_80aafe00` | 124 | high |

## Deliberate skips

- **Truncated exports (`halt_baddata`).** `0x80AA0080` (28),
  `0x80AA0300` (52), `0x80AA39D0` (48), `0x80AAA3E0` (20) and
  `0x80AAA480` (20) end in bad instruction data, so their bodies cannot be
  reconstructed honestly. The latter two are still called by address from
  [`fn_80aad010`] and are documented there as untyped spawners.
  `0x80AA1030` (72) only forwards to the truncated `0x80AA0080`, so it is left
  with it.
- **GQR-damaged exports.** `0x80AA0530` (388), `0x80AA0C20` (872, which also
  loses `in_f31`/`in_ps31_1`), `0x80AA35E0` (408), `0x80AA37E0` (240),
  `0x80AA3A90` (216), `0x80AA3F50` (200), `0x80AA4070` (444),
  `0x80AA8470` (592), `0x80AA93F0` (496), `0x80AAC360` (680),
  `0x80AAD9B0` (180), `0x80AADC50` (628), `0x80AAE520` (1,156) and
  `0x80AAF2B0` (108) pass their halfwords and floats through
  `unaff_GQR0`/`unaff_GQR3` with `ldexpf` scale factors, so the source values
  of the stores cannot be typed. This is the same limitation recorded in
  waves 123 and 124.
- **`0x802DCBFC` trampolines.** `0x80AAE360` (48), `0x80AAE390` (48),
  `0x80AAFCB0` (48) and `0x80AAFCE0` (48) are the argument-swapping
  deleting-destructor forwarders already recorded many times in this file; the
  lane budget was better spent on the twenty-seven real bodies above.
- **Not claimed inside the budget.** `0x80AAC610` (596) and `0x80AAC870`
  (900) are clean matrix rebuilds but were left for a later pass once the
  25-30 body target was met; `0x80AAD490` (484) is the effect/sound step of
  the slide machine and is called by address from [`fn_80aad840`]. Nothing was
  written for them, so no ledger row claims them.
- **No address already in `ledger/functions.csv` was touched.**

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  6,456 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 27 was in
  `ledger/functions.csv` before this pass.
- `cargo check --message-format=short` is free of errors.
- `node tools/check-lane-csv.mjs wave125` reports `OK`.
- The module header was updated: cluster item 106, the cluster count (one
  hundred and five to one hundred and six), the function totals (2,839 to
  2,866 public translations, 2,768 to 2,795 `fn_<addr>` bodies), the "start
  here" list and the records list.
