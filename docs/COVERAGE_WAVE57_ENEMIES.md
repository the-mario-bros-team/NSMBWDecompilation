# Coverage - Wave 57 (enemies: `Wm_en_burst_m`)

## How to read this file

This page records the Wave 57 enemies lane: one contiguous run inside the
`0x80AF0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to a real `pub unsafe fn`
body in that file; the `evidence` column of the lane CSV names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these are runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 5,856 | 11 | 19 |

## What this pass covers

The pass is one run, `0x80AF20F0..0x80AF45B0`, read from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Addresses already in
`ledger/functions.csv` (34 of the bucket's 208 functions, including
`0x80AF3540`, `0x80AF4BF0..0x80AF4F90` and `0x80AF57A0..0x80AF7BA0`) were
checked and skipped.

- **Identification.** `0x80AF20F0` allocates `0xB60` bytes and loads
  `g3d/MG_remo_cannon.brres`; `0x80AF3C80` spawns `Wm_en_burst_m`, and
  `0x80AF3F70` spawns the four colour variants of `Wm_mg_discharge03_*`. The
  block is therefore the `burst` world-map enemy. The class vtable at
  `+0x60` is `0x80B62CB8`.
- **Constructor cluster (`0x80AF20F0..0x80AF2810`).** The `0xB60`-byte
  constructor and its `0x128`-byte sub-object constructor, the cannon
  sub-object binder, the `guide_wait` rebinder, the matrix rebuild/commit
  helper and the full `MG_remo_cannon` model/clip binder.
- **Ring cluster (`0x80AF2FF0..0x80AF33A0`).** The four-slot controller-id
  history ring at `+0x778`: dead-slot predicate, insert, occupancy count,
  head validity test and retire path. `0x80AF3210` enumerates the plugged
  pads from the `0x80429CA8` bit byte.
- **Steer cluster (`0x80AF3640..0x80AF3B40`).** The cannon-node sampler,
  the thirteen-by-three short-angle stepper, the contact responder, the two
  wait/retire states, and the sweep setup that derives `32` frames from
  `24.0 / 0.75`.
- **Frame cluster (`0x80AF3C00..0x80AF45B0`).** The forward and reverse spin
  steps, the sweep entry, the three-state main frame step, the effect spawn,
  the timer completion step, the animation reset and the `mark` node sampler.
- **Float constants.** `0x80B31378 = 0.75`, `0x80B3139C = 1.0`,
  `0x80B313A0 = 0.0`, `0x80B313A4 = 2.0`, `0x80B313A8 = 3.0`,
  `0x80B313C4 = 4.0`, `0x80B313C8 = 5500.0`, `0x80B313CC = 6.0`,
  `0x80B313D0 = 14.0`, `0x80B313E8 = 24.0` were read byte-for-byte from the
  REL image (`work/rels/elf/d_enemiesNP.elf`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AF20F0` | `fn_80af20f0` | 348 | high |
| `0x80AF2290` | `fn_80af2290` | 88 | high |
| `0x80AF22F0` | `fn_80af22f0` | 268 | medium |
| `0x80AF2520` | `fn_80af2520` | 116 | high |
| `0x80AF25A0` | `fn_80af25a0` | 260 | medium |
| `0x80AF2810` | `fn_80af2810` | 412 | medium |
| `0x80AF2BF0` | `fn_80af2bf0` | 256 | medium |
| `0x80AF2F60` | `fn_80af2f60` | 144 | medium |
| `0x80AF2FF0` | `fn_80af2ff0` | 108 | medium |
| `0x80AF3060` | `fn_80af3060` | 96 | high |
| `0x80AF30C0` | `fn_80af30c0` | 100 | high |
| `0x80AF3130` | `fn_80af3130` | 220 | medium |
| `0x80AF3210` | `fn_80af3210` | 148 | high |
| `0x80AF32B0` | `fn_80af32b0` | 76 | high |
| `0x80AF3300` | `fn_80af3300` | 76 | high |
| `0x80AF3350` | `fn_80af3350` | 72 | medium |
| `0x80AF33A0` | `fn_80af33a0` | 88 | medium |
| `0x80AF3640` | `fn_80af3640` | 76 | high |
| `0x80AF3690` | `fn_80af3690` | 432 | medium |
| `0x80AF3950` | `fn_80af3950` | 248 | medium |
| `0x80AF3A90` | `fn_80af3a90` | 168 | medium |
| `0x80AF3B40` | `fn_80af3b40` | 168 | medium |
| `0x80AF3C00` | `fn_80af3c00` | 120 | medium |
| `0x80AF3C80` | `fn_80af3c80` | 208 | medium |
| `0x80AF3D60` | `fn_80af3d60` | 524 | medium |
| `0x80AF3F70` | `fn_80af3f70` | 452 | medium |
| `0x80AF4150` | `fn_80af4150` | 184 | high |
| `0x80AF4210` | `fn_80af4210` | 156 | medium |
| `0x80AF43C0` | `fn_80af43c0` | 120 | high |
| `0x80AF45B0` | `fn_80af45b0` | 124 | medium |

## Deliberate skips

- **`0x80AF2CF0` (624 bytes), `0x80AF3400` (308 bytes) and `0x80AF3570` (196
  bytes).** All three exports contain `unaff_GQR0`/`unaff_GQR3` conversion
  artifacts, duplicated `ldexpf` guards and merged float paths. The callers
  keep the calls (`0x80AF3400` in [`fn_80af33a0`], `0x80AF3570` in
  [`fn_80af3950`], [`fn_80af3c80`] and [`fn_80af3f70`]), but the bodies are
  not reconstructed.
- **`0x80AF3840` (16 bytes).** The export is `halt_baddata()` after a bad
  instruction warning, so there is no body. [`fn_80af3690`] keeps the call.
- **`0x80AF3850` (88 bytes) and `0x80AF38B0` (160 bytes).** The pair mixes a
  double-precision random range (`0x800B3980`) with the same
  float-to-double conversion artifacts. `0x80AF3850` is called from
  [`fn_80af3d60`] but not translated.
- **`0x80AF2400` (284 bytes).** Same GQR artifacts as the three bodies above;
  it is called from [`fn_80af2bf0`] by address only.
- **`0x80AF20F0..0x80AF45B0` remainder.** The run is contiguous apart from the
  skips above; no other function in this range was translated.
- **Rest of the `0x80AF0000` bucket.** 174 of the bucket's 208 functions were
  untranslated at the start of this pass; 30 are covered here.
- **Hidden argument drops.** `0x8005F900` and `0x8013FAC0` appear without
  arguments in `0x80AF2FF0` and `0x80AF3A90`, and `0x80AF3400`/`0x80AF3850`
  are shown without arguments at their call sites. The bodies pass the values
  that the surrounding instructions had just loaded, mark the inference in a
  `// inferred:` comment and are scored `medium`. [`fn_80af33a0`] is
  `partial` for this reason.
- **State descriptor names.** The three descriptors at `0x80B779F0`,
  `0x80B77A70` and `0x80B77AB0` live in BSS and no export writes them by
  name, so the docs cite the addresses only.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  5,856 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/enemies.rs` (the only errors in the tree at the time of writing
  belong to another lane, `src/game/bases.rs`).
- `node tools/check-lane-csv.mjs wave57` is clean for `wave57-enemies.csv`.
