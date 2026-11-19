# Coverage - Wave 75 (enemies: `0x80AA0000` bucket)

## How to read this file

This page records the Wave 75 `game::enemies` lane: the `0x80AA0000` bucket of
REL module 3, `d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row below
corresponds to one real `pub unsafe fn fn_<addr>` body in that file, and the
evidence column names the Ghidra C export the body was reconstructed from. The
machine-readable mirror of the table is `ledger/agents/wave75-enemies.csv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Float and
integer constants quoted in the docs were read from the REL image
(`work/rels/elf/d_enemiesNP.elf`) at the cited synthetic address, not guessed
from the C text.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 4,756 | 19 | 11 |

The bucket holds 188 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 47 were already in
`ledger/functions.csv`, leaving 141. This pass claims 30 and leaves 111.

## What this pass covers

The claimed bodies form two runs inside a single class, `daEnKeronpa_c` (the
`keronpa` plant enemy, `0xA58` bytes, class vtable `0x80B533C0`). The runs were
chosen because they share fields (`+0xA4E` light yaw, `+0xA50` timer, `+0xA52`
pinch flag, `+0xA53`/`+0xA54` child indices, `+0xA55` explode countdown,
`+0xA56` trail cursor, `+0x614..+0x814` 64-entry trail, `+0x814..+0x914`
32-entry sample buffer, `+0x914..+0x924` four child ids) and because their
exports are clean (no GQR/`unaff` pseudo-code and no `halt_baddata`).

- **Constructor (`0x80AA49A0`).** The anchor: installs class vtable
  `0x80B533C0`, resets the render-list holder `+0x524`, initialises the
  transform `+0x544` and animation `+0x584` holders, builds the two
  `0x8016A5E0` sub-objects at `+0x5C8`/`+0x5F4`, allocates the `0x40 x 8` and
  `0x20 x 8` trail arrays at `+0x614`/`+0x814`, the list header at `+0x924`,
  and clears the counters and the four child slots.
- **Clip, spawn and child plumbing (`0x80AA3580..0x80AA3EC0`).** The clip-row
  binder, the trail-ring commit, the child spawn (`id 0xE1`), the four-slot
  release, the four-slot virtual call, the "spawn all four and enter `Move`"
  helper and the "refresh the active children" helper.
- **Surface test and `Move` frame (`0x80AA4020`, `0x80AA4300`, `0x80AA44A0`).**
  The `0x80070BA0` surface query with its three miss codes, the per-frame child
  coherence step (window test, sub-object update, five child-name compares,
  trail append, child seating) and the matrix rebuild/commit for the exploding
  light.
- **State callbacks (`0x80AA4DB0..0x80AA54E0`).** The `Move`, `AttackSt`,
  `AttackMd`, `AttackMdB`, `Attack` and `Explode` callbacks: the seat height
  arithmetic and `pinch` clip, the holder releases, the child arming bytes
  `+0x508`/`+0x509`, the child index walk, the `12.0` clip predicate with the
  aim placement, and the two-phase self-destruct with `Wm_en_explosion`.
- **Death-state tail (`0x80AA5ED0..0x80AA6270`).** Two arm callbacks, the three
  constant producers installed as the `DieFumi`/`DieFall`/`DieBigFal` extra
  words, the five-link `+0x30` chain walker, the `:`-suffix name predicate and
  two `0x802DCBFC` forwarders.

### Identity evidence

- `register_keronpa_states` (`0x80AA5790`, already ledgered) writes the strings
  `daEnKeronpa_c::StateID_Move`, `_AttackSt`, `_AttackMd`, `_AttackMdB`,
  `_Attack`, `_Explode`, `_DieFumi`, `_DieFall` and `_DieBigFal` into
  `0x80B749D0`, `0x80B74A10`, `0x80B74A50`, `0x80B74A90`, `0x80B74AD0`,
  `0x80B74B10`, `0x80B74B4C`, `0x80B74B8C` and `0x80B74BCC`. That names every
  state word this block switches to.
- The same function's record layout fixes the three callback words per state:
  it copies the nine-word blocks at `0x80B53658..0x80B53798` into the records,
  which maps `0x80AA4EF0` / `0x80AA4F00` / `0x80AA4DB0` onto `Move`,
  `0x80AA50D0` / `0x80AA50E0` / `0x80AA5060` onto `AttackSt`,
  `0x80AA51C0` / `0x80AA51D0` / `0x80AA5150` onto `AttackMd`,
  `0x80AA52B0` / `0x80AA52C0` / `0x80AA5270` onto `AttackMdB`,
  `0x80AA5380` / `0x80AA5390` / `0x80AA5360` onto `Attack` and
  `0x80AA54D0` / `0x80AA54E0` / `0x80AA54B0` onto `Explode` (the middle word of
  each triple is the per-frame step). The six `..4EF0`/`..50D0`/`..51C0`/
  `..52B0`/`..5380`/`..54D0` targets sit in unlisted gaps between exported
  functions, so Ghidra did not split them out; they are absent from
  `functions.tsv` and from this pass. It also installs `0x80AA5ED0` on the six
  live states and `0x80AA5F30` on the three death states, and stores the
  results of `0x80AA5FB0` / `0x80AA5FA0` / `0x80AA5F90` on `DieFumi` /
  `DieFall` / `DieBigFal`.
- `0x80AA3310` (already ledgered) allocates `0xA58` bytes and calls
  `0x80AA49A0`, so the constructor's size is not guessed.
- `0x80AA4F00` and `0x80AA54E0` name the effect string `Wm_en_keronpalight`
  (`0x80B53398`) and `Wm_en_explosion` (`0x80B533B0`); `0x80AA4DB0` looks up
  the `pinch` clip (`0x80B53380`).
- The REL prints no C++ class name in this bucket; the name `daEnKeronpa_c`
  comes from the state-name strings above, and every doc block cites that
  derivation instead of asserting it.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AA3580` | `fn_80aa3580` | 92 | high |
| `0x80AA38D0` | `fn_80aa38d0` | 144 | medium |
| `0x80AA3960` | `fn_80aa3960` | 108 | medium |
| `0x80AA3A10` | `fn_80aa3a10` | 120 | high |
| `0x80AA3B70` | `fn_80aa3b70` | 116 | medium |
| `0x80AA3E30` | `fn_80aa3e30` | 144 | high |
| `0x80AA3EC0` | `fn_80aa3ec0` | 140 | high |
| `0x80AA4020` | `fn_80aa4020` | 80 | high |
| `0x80AA4300` | `fn_80aa4300` | 416 | medium |
| `0x80AA44A0` | `fn_80aa44a0` | 312 | high |
| `0x80AA49A0` | `fn_80aa49a0` | 348 | high |
| `0x80AA4DB0` | `fn_80aa4db0` | 308 | medium |
| `0x80AA4F00` | `fn_80aa4f00` | 348 | medium |
| `0x80AA5060` | `fn_80aa5060` | 108 | medium |
| `0x80AA50E0` | `fn_80aa50e0` | 112 | high |
| `0x80AA5150` | `fn_80aa5150` | 108 | high |
| `0x80AA51D0` | `fn_80aa51d0` | 160 | high |
| `0x80AA5270` | `fn_80aa5270` | 64 | high |
| `0x80AA52C0` | `fn_80aa52c0` | 160 | high |
| `0x80AA5390` | `fn_80aa5390` | 276 | medium |
| `0x80AA54E0` | `fn_80aa54e0` | 420 | medium |
| `0x80AA5ED0` | `fn_80aa5ed0` | 88 | high |
| `0x80AA5F30` | `fn_80aa5f30` | 92 | high |
| `0x80AA5F90` | `fn_80aa5f90` | 12 | high |
| `0x80AA5FA0` | `fn_80aa5fa0` | 12 | high |
| `0x80AA5FB0` | `fn_80aa5fb0` | 12 | high |
| `0x80AA60A0` | `fn_80aa60a0` | 224 | high |
| `0x80AA6180` | `fn_80aa6180` | 136 | high |
| `0x80AA6240` | `fn_80aa6240` | 48 | medium |
| `0x80AA6270` | `fn_80aa6270` | 48 | medium |

## Deliberate skips

- **`0x80AA35E0` (408 bytes).** The `Move` angle/speed integrator. Its export
  is built from `unaff_GQR0`/`unaff_GQR3` quantiser fiction plus `ldexpf` calls
  that are not in the original code, so the angle arithmetic cannot be
  recovered. `0x80AA4F00` and `0x80AA54E0` still call the address, marked
  `// partial:`.
- **`0x80AA37E0` (240 bytes).** Same damage: the `+0xA4E` angle is expanded
  into the 32-entry sample buffer through `unaff_GQR3` conversions.
  `0x80AA5060` calls it by address.
- **`0x80AA3F50` (200 bytes) and `0x80AA4070` (444 bytes).** The aim-point
  builder and its sweep wrapper, both `unaff_GQR3`. `0x80AA5390` and
  `0x80AA4F00` call them by address.
- **`0x80AA39D0` (48 bytes).** The child-slot getter is truncated by
  `halt_baddata` after the bounds check. Three callers (`0x80AA5060`,
  `0x80AA5150`, `0x80AA5270`, plus `0x80AA5390`) call the address and document
  that the returned word is the slot's child object.
- **`0x80AA3A90` (216 bytes).** The child-seating step is `unaff_GQR0` on both
  of its float conversions. `0x80AA4300` calls it by address.
- **The rest of the bucket.** The head gaps left by earlier waves
  (`0x80AA0080`, `0x80AA0300`, `0x80AA0530`, `0x80AA0C20`, `0x80AA1030`) and the
  next class family from `0x80AA62A0` to `0x80AAFE00` (about 100 functions,
  including the large `0x80AA67A0` and `0x80AAE520` registrations) are
  untouched; 111 functions of the bucket remain. The largest clean candidates
  for the next pass are `0x80AA67A0` (1,520 bytes), `0x80AAD010` (1,140),
  `0x80AAE520` (1,156) and `0x80AAEB30` (936).

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  4,756 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 47 existing rows include the previously
  translated `0x80AA1BF0`..`0x80AA37B0` state spine and `0x80AA5790`
  `register_keronpa_states`).
- `node tools/check-lane-csv.mjs wave75` prints
  `wave75-enemies.csv: 30 rows`, `unique addresses: 113` and `OK`.
- `cargo check --message-format=short` finishes with no diagnostic naming
  `src/game/enemies.rs`; the remaining warnings are in `src/runtime/*`.
