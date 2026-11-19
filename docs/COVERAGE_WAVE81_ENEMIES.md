# Coverage - Wave 81 (enemies: `0x80B10000` bucket opening)

## How to read this file

This page records the Wave 81 `game::enemies` lane: the first pass over the
`0x80B10000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to one real
`pub unsafe fn fn_<addr>` body in that file, and the evidence column of the
machine-readable mirror (`ledger/agents/wave81-enemies.csv`) names the Ghidra
C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32` / `read_u16` / `read_i16`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 5,184 | 19 | 8 |

Three rows are `low` confidence: the 48-byte argument-swapping forwarders
`0x80B19B40`, `0x80B19B70` and `0x80B1A2E0`, whose hidden-`r12` this-adjust is
not modelled (the same limitation already recorded for `0x802DCBFC` copies in
waves 75 and 80). The bucket holds 211 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 85 were already in
`ledger/functions.csv` before this pass (waves 47, 53 and 66), 30 are claimed
here and 96 remain.

## What this pass covers

One cluster, contiguous except for the four functions skipped below.

**`0x80B19A20..0x80B1B8F0`, thirty bodies.** The `pakkun_walk` plant actor.
The run opens with the shared shell helpers: two freeing helpers
(`0x8015F800` + release, and `0x80095130` + release), the two 48-byte
forwarders, the `0x530`-byte spawn marker (`0x80B19BA0`) with its
register/unregister callback and the four-slot side-count retire walk, the
camera-rectangle overlap test, and the `0x53` marker spawn with the `+0x6E8`
back-pointer. Then the `0x5F8`-byte actor itself: the constructor
(vtable `0x80B6AE28`, `+0x524` model block, `+0x544` owner, `+0x584` holder
rewritten to `0x80B330E0`, `+0x5E8` helper with vtable `0x80B6B0A8`), the
spawn initialiser, the `g3d/pakkun_walk.brres` binder, the model matrix
rebuild with its `check`-node height probe, the contact/damage responder, and
the four named state entries (`Walk`, `AttackSt`, `Attack`, `AttackEd`) with
their entry/frame steps and the shared predicate.

### Identity evidence

- **The model strings were read, not guessed.** The export names
  `s_g3d_pakkun_walk_brres_80b6adc4` (`g3d/pakkun_walk.brres`),
  `s_pakkun_walk_80b6addc` (`pakkun_walk`), `s_attack_st_80b6adf8`
  (`attack_st`), `s_attack_80b6ae04` (`attack`) and `s_attack_ed_80b6ae0c`
  (`attack_ed`). The `walk` and `check` strings at `0x80B6ADE8` and
  `0x80B6ADF0` are passed to the clip lookup and the node lookup; they are
  cited by address only because the export prints them as `DAT_` symbols.
- **The state words are BSS.** `0x80B79138`, `0x80B79178`, `0x80B791B8`,
  `0x80B791F8` and `0x80B79238` carry no bytes in the image, so the docs call
  them name words and never claim their text.
- **The class layout is shared with earlier waves.** The `+0x394` helper, the
  `+0x544`/`+0x584` holder pair, the `+0x348` facing byte, the `+0x1EC`
  controller and the `+0x60`-relative virtual `+0xD4` state switch are the
  same shapes already documented for the kanon, lift and ben blocks.
- **The virtual blend slot is the documented `+0x3FC`.** `0x80B1AA20` calls
  the contacted actor's `+0x3FC` with two doubles and four integers, exactly
  the shape that `call6_f64x2` was introduced for in wave 9.
- **The player rectangle helper is reused, not re-derived.** `0x80B19CB0`
  calls `0x8008E890` for four floats and treats them as
  x / y / width / height with the y band `y - height .. y`, the interpretation
  already written for `fn_80b191b0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B19A20` | `fn_80b19a20` | 88 | high |
| `0x80B19B40` | `fn_80b19b40` | 48 | low |
| `0x80B19B70` | `fn_80b19b70` | 48 | low |
| `0x80B19BA0` | `fn_80b19ba0` | 76 | high |
| `0x80B19C60` | `fn_80b19c60` | 72 | high |
| `0x80B19CB0` | `fn_80b19cb0` | 232 | medium |
| `0x80B19DA0` | `fn_80b19da0` | 140 | high |
| `0x80B19E30` | `fn_80b19e30` | 164 | medium |
| `0x80B19F50` | `fn_80b19f50` | 28 | high |
| `0x80B1A050` | `fn_80b1a050` | 88 | high |
| `0x80B1A1C0` | `fn_80b1a1c0` | 88 | high |
| `0x80B1A2E0` | `fn_80b1a2e0` | 48 | low |
| `0x80B1A340` | `fn_80b1a340` | 148 | high |
| `0x80B1A420` | `fn_80b1a420` | 348 | medium |
| `0x80B1A580` | `fn_80b1a580` | 228 | high |
| `0x80B1A760` | `fn_80b1a760` | 204 | high |
| `0x80B1A830` | `fn_80b1a830` | 48 | high |
| `0x80B1A860` | `fn_80b1a860` | 316 | medium |
| `0x80B1A9A0` | `fn_80b1a9a0` | 128 | medium |
| `0x80B1AA20` | `fn_80b1aa20` | 284 | medium |
| `0x80B1ACD0` | `fn_80b1acd0` | 96 | high |
| `0x80B1AE40` | `fn_80b1ae40` | 308 | medium |
| `0x80B1B110` | `fn_80b1b110` | 268 | medium |
| `0x80B1B230` | `fn_80b1b230` | 392 | high |
| `0x80B1B4F0` | `fn_80b1b4f0` | 192 | high |
| `0x80B1B5C0` | `fn_80b1b5c0` | 304 | high |
| `0x80B1B6F0` | `fn_80b1b6f0` | 132 | high |
| `0x80B1B790` | `fn_80b1b790` | 156 | high |
| `0x80B1B830` | `fn_80b1b830` | 168 | high |
| `0x80B1B8F0` | `fn_80b1b8f0` | 344 | high |

## Deliberate skips

- **`0x80B19EE0`** (104 bytes) calls `FUN_80b19e30` with no argument and then
  dereferences the "result" as a pointer, although the callee's export is
  `void`. The real return value cannot be established from the C, so the body
  was left rather than guessed.
- **`0x80B19FA0`** (176 bytes) reads `FUN_80b19cb0` as a 64-bit value and tests
  the high word, while both the callee and the caller print it as a 32-bit
  result. Its `bVar1` latch also depends on that truncated pair, so the
  control flow cannot be written faithfully.
- **`0x80B1B0E0`** (32 bytes) is a `halt_baddata()` body: Ghidra could not
  decode the instructions (bad data in the image). Nothing can be translated.
- **`0x80B1B3F0`** (248 bytes) calls `FUN_80b1b0e0` and tests its return.
  Since that callee cannot be reconstructed, this caller is left as well.
- **`0x80B1AD30`** (308 bytes) takes its actor from the `0x802DCD74` prologue
  fragment and passes two words of the resulting register pair to
  `0x800A0A30`; the second word is not identifiable from the export, so the
  body was skipped.
- **The rest of the bucket.** `0x80B10110..0x80B12470`,
  `0x80B15C60..0x80B17CB0`, `0x80B1CC50..0x80B1DFE0`,
  `0x80B1E200..0x80B1EF80` and `0x80B1F970..0x80B1FF30` are untouched; most
  are clean exports but lie outside this lane's 30-body budget. The
  `0x80B10110..0x80B12470` run in particular carries `GQR` paired-single noise
  in several bodies.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  5,184 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 85 existing rows are untouched).
- `node tools/check-lane-csv.mjs wave81` reports 30 unique addresses for this
  CSV and `OK` for the lane.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`.
