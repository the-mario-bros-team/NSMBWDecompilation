# Coverage - Wave 6, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-6
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). The first-pass 23
  functions are documented in `docs/COVERAGE_REL_d_en_bossNP.md`, wave 3 adds
  20 (`docs/COVERAGE_WAVE3_BOSSES.md`), wave 4 adds 24
  (`docs/COVERAGE_WAVE4_BOSSES.md`), and wave 5 adds 25
  (`docs/COVERAGE_WAVE5_BOSSES.md`). This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value here is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x8016xxxx`, `0x8006xxxx`, `0x8009xxxx`,
  `0x8042xxxx`) are absolute and unaffected.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every row.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` means one
  documented piece is missing.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores and calls are exact and the role is already established by a
  string or a sibling function. `medium` means the stores and branches are
  exact but an object role, a vtable prototype or an untranslated helper is
  inferred.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table; no Nintendo symbol
  is invented.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 13 |
| confidence `medium` | 12 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 117 |
| cumulative share of 770 functions | 15.19% |

The pass follows the boss-family call graph rather than an address sample:
the six per-boss actor constructors, the `damage`/`damage_s` entries their
state tables install, the Koopa shout/death phases, the two castle aim/fire
scripts and the dispatcher that drives both aim solvers, three heading
helpers, and the `Wm_ob_witchcraft` spawner called by the wave-5 landing
handler `0x80C08DC0`.

## Cluster 1 - Per-boss actor constructors

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C18640` | 344 | `fn_80c18640` | Iggy-family actor constructor | translated | medium |
| `0x80C34D90` | 384 | `fn_80c34d90` | Larry-family actor constructor | translated | medium |
| `0x80C3D730` | 428 | `fn_80c3d730` | Ludwig-family actor constructor | translated | medium |
| `0x80C42D70` | 392 | `fn_80c42d70` | Morton-family actor constructor | translated | medium |
| `0x80C46970` | 408 | `fn_80c46970` | Roy-family actor constructor | translated | medium |
| `0x80C4A940` | 400 | `fn_80c4a940` | Wendy-family actor constructor | translated | medium |

All six follow the same shape: link the model sub-object, write motion
constants, derive the facing byte, write the clip-pointer table
(`jump_st`/`throw_st`/`damage`), set up the ground probe, and store the
family's `Wm_*_shelltail` effect name. The shared two-word tables at
`0x8042BB08` and `0x8042BB0C` are copied verbatim. Wendy's constructor
derives the initial heading directly from `0x8042C1A4` and calls class slot
`+0x484` instead of `0x800B30C0`.

## Cluster 2 - Damage and damage_s entries

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C19F50` | 236 | `fn_80c19f50` | Iggy-family `damage` bind | translated | high |
| `0x80C1EA60` | 136 | `fn_80c1ea60` | `damage_s` bind to secondary animation | translated | high |
| `0x80C35C90` | 348 | `fn_80c35c90` | Larry-family `damage` bound to primary and secondary | translated | high |
| `0x80C40220` | 280 | `fn_80c40220` | Ludwig-family `damage` with recoil fields | translated | high |
| `0x80C47E20` | 272 | `fn_80c47e20` | Roy-family `damage` with recoil fields | translated | high |

These entries look up the clip on the model at `+0x604`, bind it to the
animation at `+0x644` (and sometimes `+0x6AC`/`+0x67C`), reset the hit flag
at `+0x432`, re-derive the facing byte and heading, and write motion/speed
fields.

## Cluster 3 - Koopa shout/death and Mr. electric-shock

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C24730` | 608 | `fn_80c24730` | Koopa family shout attack | translated | high |
| `0x80C24C00` | 652 | `fn_80c24c00` | Koopa four-state death sequence | translated | medium |
| `0x80C27980` | 328 | `fn_80c27980` | Koopa reset into damage state | translated | high |
| `0x80C27AD0` | 468 | `fn_80c27ad0` | Koopa three-state knockdown/death | translated | medium |
| `0x80C31520` | 236 | `fn_80c31520` | Mr. electric-shock entry | translated | high |

The shout handler spawns `Wm_ko_shout` through effect slot `+0xE80`, emits
sound through `0x800D8960`/`0x800D8A80`, and returns to the castle phase
manager on clip finish. Death handlers use `Wm_ko_downsmoke` and update
timers at `+0x710`. The reset function walks the `+0xF94` sub-object array
(`stride 0xA4`) and sets the global byte at `0x8042A070+0xC`.

## Cluster 4 - Castle aim/fire scripts and phase dispatcher

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C21FE0` | 760 | `fn_80c21fe0` | Castle phase dispatcher | translated | medium |
| `0x80C27230` | 456 | `fn_80c27230` | Castle aim-and-fire script | translated | medium |
| `0x80C27860` | 272 | `fn_80c27860` | Castle multi-step aim script | translated | medium |

The dispatcher copies entry positions, updates auxiliary objects, calls
[`fn_80c22ca0`] (wave 4), selects a target with `0x80C23CD0`, clamps
approach distance, runs class slot `+0x390`, calls both aim solvers
[`fn_80c23680`]/[`fn_80c239c0`] (wave 5), and increments per-slot counters
at `+0xF98`. The aim scripts use the heading helpers and call through
`0x80C228A0`/`0x806BAEB0`.

## Cluster 5 - Heading and state helpers

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C07870` | 288 | `fn_80c07870` | Spawn `Wm_ob_witchcraft` effect | translated | medium |
| `0x80C120D0` | 328 | `fn_80c120d0` | Rebuild transform with velocity offset | translated | high |
| `0x80C12FE0` | 188 | `fn_80c12fe0` | Four-shot countdown state handler | translated | high |
| `0x80C13560` | 180 | `fn_80c13560` | Clamped ratio of value within bounds | translated | high |
| `0x80C13730` | 160 | `fn_80c13730` | Grounded jump/land state handler | translated | high |
| `0x80C27400` | 80 | `fn_80c27400` | Heading window clamp | translated | high |

`fn_80c07870` is called by the wave-5 landing handler `0x80C08DC0`. The
transform helper `fn_80c120d0` is a sibling of wave-4's `fn_80c01120`.
`fn_80c13560` and `fn_80c27400` are used by aim code for ratio/clamp logic.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C07870` | `FUN_80c07870` | `fn_80c07870` | `Wm_ob_witchcraft` spawn | translated | medium | `80c07870_FUN_80c07870.c` |
| `0x80C120D0` | `FUN_80c120d0` | `fn_80c120d0` | Transform rebuild | translated | high | `80c120d0_FUN_80c120d0.c` |
| `0x80C12FE0` | `FUN_80c12fe0` | `fn_80c12fe0` | Four-shot countdown | translated | high | `80c12fe0_FUN_80c12fe0.c` |
| `0x80C13560` | `FUN_80c13560` | `fn_80c13560` | Clamped ratio | translated | high | `80c13560_FUN_80c13560.c` |
| `0x80C13730` | `FUN_80c13730` | `fn_80c13730` | Jump/land handler | translated | high | `80c13730_FUN_80c13730.c` |
| `0x80C18640` | `FUN_80c18640` | `fn_80c18640` | Iggy constructor | translated | medium | `80c18640_FUN_80c18640.c` |
| `0x80C19F50` | `FUN_80c19f50` | `fn_80c19f50` | Iggy `damage` | translated | high | `80c19f50_FUN_80c19f50.c` |
| `0x80C1EA60` | `FUN_80c1ea60` | `fn_80c1ea60` | `damage_s` bind | translated | high | `80c1ea60_FUN_80c1ea60.c` |
| `0x80C21FE0` | `FUN_80c21fe0` | `fn_80c21fe0` | Castle dispatcher | translated | medium | `80c21fe0_FUN_80c21fe0.c` |
| `0x80C24730` | `FUN_80c24730` | `fn_80c24730` | Koopa shout | translated | high | `80c24730_FUN_80c24730.c` |
| `0x80C24C00` | `FUN_80c24c00` | `fn_80c24c00` | Koopa death | translated | medium | `80c24c00_FUN_80c24c00.c` |
| `0x80C27230` | `FUN_80c27230` | `fn_80c27230` | Castle aim/fire | translated | medium | `80c27230_FUN_80c27230.c` |
| `0x80C27400` | `FUN_80c27400` | `fn_80c27400` | Heading clamp | translated | high | `80c27400_FUN_80c27400.c` |
| `0x80C27860` | `FUN_80c27860` | `fn_80c27860` | Castle multi-step aim | translated | medium | `80c27860_FUN_80c27860.c` |
| `0x80C27980` | `FUN_80c27980` | `fn_80c27980` | Koopa damage reset | translated | high | `80c27980_FUN_80c27980.c` |
| `0x80C27AD0` | `FUN_80c27ad0` | `fn_80c27ad0` | Koopa knockdown | translated | medium | `80c27ad0_FUN_80c27ad0.c` |
| `0x80C31520` | `FUN_80c31520` | `fn_80c31520` | Mr. electric shock | translated | high | `80c31520_FUN_80c31520.c` |
| `0x80C34D90` | `FUN_80c34d90` | `fn_80c34d90` | Larry constructor | translated | medium | `80c34d90_FUN_80c34d90.c` |
| `0x80C35C90` | `FUN_80c35c90` | `fn_80c35c90` | Larry `damage` | translated | high | `80c35c90_FUN_80c35c90.c` |
| `0x80C3D730` | `FUN_80c3d730` | `fn_80c3d730` | Ludwig constructor | translated | medium | `80c3d730_FUN_80c3d730.c` |
| `0x80C40220` | `FUN_80c40220` | `fn_80c40220` | Ludwig `damage` | translated | high | `80c40220_FUN_80c40220.c` |
| `0x80C42D70` | `FUN_80c42d70` | `fn_80c42d70` | Morton constructor | translated | medium | `80c42d70_FUN_80c42d70.c` |
| `0x80C46970` | `FUN_80c46970` | `fn_80c46970` | Roy constructor | translated | medium | `80c46970_FUN_80c46970.c` |
| `0x80C47E20` | `FUN_80c47e20` | `fn_80c47e20` | Roy `damage` | translated | high | `80c47e20_FUN_80c47e20.c` |
| `0x80C4A940` | `FUN_80c4a940` | `fn_80c4a940` | Wendy constructor | translated | medium | `80c4a940_FUN_80c4a940.c` |

## Unknowns

- **Untranslated callees.** `0x80C22CA0` (called by `0x80C21FE0`), `0x80C23CD0`
  (target selection in dispatcher), and `0x80C228A0` (used by both castle aim
  scripts) are still untranslated; call sites preserve their raw addresses and
  arguments as printed by the export.
- **Missing arguments in export.** Several calls are printed without arguments
  in Ghidra (`0x8006CED0` in `0x80C27230`, class slot `+0x288` queries in
  `0x80C07870`, `0x8006CED0` usage). The translated forms match the call sites
  and wave-5 conventions.
- **Compiler noise.** `0x80C24C00` and `0x80C27AD0` begin/end with paired-single
  `unaff_GQR0`/`ldexpf` blocks printed by the decompiler; they are not game
  logic and are not translated.
- **Opaque constants.** Tables and constants at addresses like `0x8042BB08/0C`,
  `0x8042C1A0/A4`, `0x80C613xx`, `0x80C614xx`, `0x80C617F8`, `0x80C61Cxx`,
  `0x80C61Dxx`, and the `Wm_*` string addresses are read from their original
  locations; numeric values are not claimed.

## Verification

- `cargo check` passes (exit 0) with no error or warning in
  `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 70 files, 0 violations.
- All 25 evidence files referenced in the ledger were used; every function has
  a `# Original` block with synthetic base `0x80C00000`, address/size and the
  export path.
- The 25 rows in `ledger/agents/wave6-bosses.csv` match the Rust functions
  one-to-one: 25 `translated`; 13 `high`, 12 `medium`.
- No files outside `src/game/bosses.rs`, `docs/COVERAGE_WAVE6_BOSSES.md` and
  `ledger/agents/wave6-bosses.csv` were edited.
