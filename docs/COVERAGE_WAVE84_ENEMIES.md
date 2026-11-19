# Coverage - Wave 84 (enemies: the `0x80AC0000` bucket's later runs)

## How to read this file

This page records the Wave 84 `game::enemies` lane: a pass over the
`0x80AC0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below is one real `pub unsafe fn fn_<addr>`
body in that file, and the `evidence` column of the machine-readable mirror
(`ledger/agents/wave84-enemies.csv`) names the Ghidra C export each body was
reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32` / `read_u16` / `read_u8`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 6,644 | 9 | 19 |

Two rows are `low` confidence: the 48-byte argument-swapping forwarders
`0x80ACF7A0` and `0x80ACF7D0`, whose hidden-`r12` this-adjust pointer is not
modelled (the same limitation already recorded for the `0x802DCBFC` copies in
waves 75, 80, 81 and 83).

The bucket holds 177 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 58 were already in
`ledger/functions.csv` before this pass, 30 are claimed here and 89 remain.

## What this pass covers

Three runs, each taken as a contiguous block.

**Cluster 1, `0x80AC2220..0x80AC3B24`, the `mecha_koopa` state steps.** Wave 55
opened this actor with its binder (`0x80AC16D0`) and its matrix rebuild
(`0x80AC1920`); this pass adds the twelve bodies that drive it:

- the four-player contact report `0x80AC2220`, which walks the four player
  slots through `0x8005F900` and releases the `+0x588`/`+0x548`/`+0x5C0`
  holders when the `0x8016A5D0` generation id moves;
- the player-copy initialiser `0x80AC2370` (slot byte at `+0x38D`, three
  floats cleared at `+0x5EC..+0x5F4`);
- the pair-contact record builder `0x80AC2A50`, which fills the same
  `+0x414..+0x432` record as the already-translated `fn_80ac1ed0` but derives
  the variant byte from the `0x80064A30` side test over the two actors;
- the region-triggered rebind `0x80AC2DC0` and the eight per-frame handlers
  `0x80AC2E70`, `0x80AC3020`, `0x80AC31A0`, `0x80AC3650`, `0x80AC3770`,
  `0x80AC3870`, `0x80AC39C0` and `0x80AC3A60`.

The handlers share one shape: release the `+0x5C0`/`+0x548` holders through
their `+0x14`/`+0x1C` slots, run the frame helpers `0x8006CDxx`/`0x800955F0`/
`0x80096F60`/`0x80097050`, and leave through the class vtable slot `+0xD4` with
one of the four region objects `0x80B759A0`, `0x80B759E0`, `0x80B75A20` or
`0x80B75B20`. The sound events they raise are `0x20A` (attack entry), `0x20B`,
`0x20D` (attack exit) and `0x20E` (clip-position edge).

**Cluster 2, `0x80AC4B50..0x80AC5D00`, the `g3d/met.brres` actor.** The run
opens with the binder `0x80AC4B50` (`g3d/met.brres`, model holder `+0x7F8`,
animation holder `+0x7FC`, clip holder `+0x83C`) and then covers its whole state
set:

- `0x80AC4C40` - the knock-away seed (drops the actor, picks the side from the
  attacker's X, clears the `+0xE8..+0xFC` speed block, enters `0x80353A64`);
- `0x80AC4EE0` - the contact predicate, gated by the attacker's vtable slot
  `+0x3F4` and by the `0x80353A64` region test;
- `0x80AC5220` - the side ground probe, whose `countLeadingZeros` idiom is the
  branch-free `status == 3` test;
- `0x80AC53E0` - the fifteen-word effect block built over the three `12`-byte
  records at `0x80B75DA4`;
- `0x80AC5650` and `0x80AC5960` - the two entry steps (the first gates the clip
  rebind on four region tests, the second rebinds unconditionally);
- `0x80AC57E0`, `0x80AC5A30`, `0x80AC5BC0` and `0x80AC5D00` - the four
  per-frame handlers around the `+0x7FC` holder and the `+0x874` latch.

**Cluster 3, `0x80ACF450..0x80ACF7D0`, a sixth descriptor skeleton.** Two
normalisers (`0x80ACF450`, `0x80ACF4B0`), the constant producer `0x80ACF510`
returning `0x803541CC`, the `+0x30` chain walker `0x80ACF600`, the `:`-suffix
name predicate `0x80ACF6E0` and the two 48-byte forwarders `0x80ACF7A0` /
`0x80ACF7D0`. This is the same five-entry shell already translated in waves 46,
53, 54, 55, 69 and 77; the walker and the predicate are byte-equivalent to
`fn_80ac0460` and `fn_80ac0540`.

### Identity evidence

- **`g3d/met.brres` is read, not guessed.** `0x80AC4B50` loads the archive path
  the export prints as `s_g3d_met_brres_80b59270`, exactly like the unnamed
  `0x798`-byte `met` actor of wave 54.
- **The `mecha_koopa` field roles are inherited.** The lane reuses the offsets,
  the `0x80B3027C` zero word, the `0x80B30318`/`0x80B30340` side tables and the
  `FN_80AC1800` call shape that wave 55 established for `fn_80ac1d00`,
  `fn_80ac1d50`, `fn_80ac1e10` and `fn_80ac1ed0`.
- **The record at `+0x414` is one layout.** `0x80AC2A50` writes the same eight
  words as `fn_80ac1ed0` and `fn_80ac1bf0`, from the same `0x8042C1B0` and
  `0x8042C2D0..D8` tables, so its field notes point at the translated siblings
  instead of repeating a guess.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AC2220` | `fn_80ac2220` | 200 | medium |
| `0x80AC2370` | `fn_80ac2370` | 120 | medium |
| `0x80AC2A50` | `fn_80ac2a50` | 440 | medium |
| `0x80AC2DC0` | `fn_80ac2dc0` | 152 | medium |
| `0x80AC2E70` | `fn_80ac2e70` | 372 | medium |
| `0x80AC3020` | `fn_80ac3020` | 312 | medium |
| `0x80AC31A0` | `fn_80ac31a0` | 140 | high |
| `0x80AC3650` | `fn_80ac3650` | 280 | medium |
| `0x80AC3770` | `fn_80ac3770` | 212 | high |
| `0x80AC3870` | `fn_80ac3870` | 324 | medium |
| `0x80AC39C0` | `fn_80ac39c0` | 132 | high |
| `0x80AC3A60` | `fn_80ac3a60` | 196 | medium |
| `0x80AC4B50` | `fn_80ac4b50` | 228 | high |
| `0x80AC4C40` | `fn_80ac4c40` | 332 | medium |
| `0x80AC4EE0` | `fn_80ac4ee0` | 328 | medium |
| `0x80AC5220` | `fn_80ac5220` | 100 | medium |
| `0x80AC53E0` | `fn_80ac53e0` | 360 | medium |
| `0x80AC5650` | `fn_80ac5650` | 380 | medium |
| `0x80AC57E0` | `fn_80ac57e0` | 380 | medium |
| `0x80AC5960` | `fn_80ac5960` | 192 | high |
| `0x80AC5A30` | `fn_80ac5a30` | 348 | medium |
| `0x80AC5BC0` | `fn_80ac5bc0` | 268 | medium |
| `0x80AC5D00` | `fn_80ac5d00` | 200 | high |
| `0x80ACF450` | `fn_80acf450` | 88 | high |
| `0x80ACF4B0` | `fn_80acf4b0` | 92 | high |
| `0x80ACF510` | `fn_80acf510` | 12 | high |
| `0x80ACF600` | `fn_80acf600` | 224 | medium |
| `0x80ACF6E0` | `fn_80acf6e0` | 136 | medium |
| `0x80ACF7A0` | `fn_80acf7a0` | 48 | low |
| `0x80ACF7D0` | `fn_80acf7d0` | 48 | low |

## Deliberate skips

- **`0x80AC1800` (276 bytes) stays untranslated.** Its export still contains
  two `unaff_GQR0` paired-single conversions (`ldexpf` of the `GQR0` fields)
  that Ghidra could not resolve, so the two float arguments cannot be
  reconstructed from the C alone. Waves 55 and 83 recorded the same limit.
  Three bodies of this pass call it: `0x80AC2DC0` (variant 0) and
  `0x80AC39C0` (variant 3), both through the existing `FN_80AC1800` constant
  and `call5_f64x2`, with the float ABI wave 55 established. `fn_80ac1cd0`,
  `fn_80ac1d00` and `fn_80ac1e10` already use the same convention.
- **`0x80AC0750` (240 bytes) and `0x80AC0840` (396 bytes) stay untranslated.**
  Both exports carry unresolved `unaff_GQR0`/`unaff_GQR3` paired-single
  artifacts; wave 55 documented the same reason and left `FN_80AC0750` /
  `FN_80AC0840` as raw call targets. Nothing in this pass depends on their
  bodies.
- **The rest of the bucket.** The remaining 89 addresses are the large
  untranslated blocks at `0x80AC2A50`'s neighbours (`0x80AC7790..0x80AC8A90`,
  `0x80AC8F70..0x80ACA1D0`, `0x80ACA9A0..0x80ACB200`,
  `0x80ACB280..0x80ACC000`, `0x80ACC150..0x80ACCF90` at up to 652 bytes,
  `0x80ACDB00..0x80ACED20` and `0x80ACF950..0x80ACFE60` at 720 bytes). They
  lie outside this lane's 30-body budget and are left for a later pass.
- **`0x80AC2A50` is `medium`, not `high`.** The `0x80065B80`/`0x8009F710`
  branch is literal, but the meaning of the slot word parked at `+0x428` and of
  the `0x80026C00` target index is inferred from the branch that guards them.
  The export also builds a quadword local it never reads; the body records that
  as a `// dead:` comment rather than modelling it.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  6,644 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 58 existing rows are untouched).
- `node tools/check-lane-csv.mjs wave84` reports 30 rows for this CSV and `OK`
  for the wave.
- `cargo check --message-format=short` exits 0 and produces no diagnostic
  naming `src/game/enemies.rs`.
