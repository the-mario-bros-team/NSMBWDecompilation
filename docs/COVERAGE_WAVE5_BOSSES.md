# Coverage - Wave 5, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-5
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). The first-pass 23
  functions are documented in `docs/COVERAGE_REL_d_en_bossNP.md`, the wave-3
  set of 20 in `docs/COVERAGE_WAVE3_BOSSES.md`, and the wave-4 set of 24 in
  `docs/COVERAGE_WAVE4_BOSSES.md`. This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value here is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x8016xxxx`, `0x8006xxxx`, `0x8009xxxx`,
  `0x8042xxxx`) are absolute and unaffected.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every row.
  Sizes are from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` means one
  documented piece is missing; `stub` is not used here.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores and calls are exact and the role is already established by a
  string or a sibling function. `medium` means the stores and branches are
  exact but an object role, a vtable prototype or an untranslated helper is
  inferred.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table; no Nintendo symbol
  is invented. Effect names appear only where the export points at the exact
  string address (`Wm_jr_kickhit`, `Wm_mo_drop`, `Wm_mo_landsmokebig`,
  `damage`).
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 5,392 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 16 |
| confidence `medium` | 9 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 92 |
| cumulative bytes in `game::bosses` | 52,360 |
| cumulative share of 770 functions | 11.95% |
| cumulative share of 356,396 text bytes | 14.69% |

The pass was chosen to close call-graph holes rather than to sample addresses:
it translates the Iggy actor constructor that owns the 0x18-byte controller
array, the three effect spawners that fill the rider/sled slots
`+0xE80/+0xE84`, the Castle phase/contact handlers in the
`0x80C05xxx..0x80C10xxx` range, both aim solvers used by the castle spawn
manager with their heading primitives, the two target-selection helpers that
the wave-4 manager called by raw address, and the `Wm_*` impact effects that
the Morton-family handlers use.

## Synthetic-base warning in one place

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The constants read at
`0x80C572xx`, `0x80C576xx`, `0x80C58Bxx`, `0x80C5A2xx`, `0x80C5C6xx`,
`0x80C613xx`, `0x80C61Dxx`, `0x80C64Axx` and `0x80C689xx` are analysis-image
data words, not hardware addresses.

## Cluster 1 - Iggy actor scaffolding and rider/sled effects

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C00240` | 252 | `fn_80c00240` | Iggy actor constructor (allocate `0x10A8`) | translated | high |
| `0x80C00380` | 64 | `fn_80c00380` | matching release wrapper | translated | medium |
| `0x80C00500` | 76 | `fn_80c00500` | 0x18-byte controller element constructor | translated | high |
| `0x80C00550` | 88 | `fn_80c00550` | controller element destructor | translated | high |
| `0x80C005B0` | 116 | `fn_80c005b0` | second element destructor variant | translated | high |
| `0x80C00900` | 116 | `fn_80c00900` | spawn effect `0x10D` into `+0xE84` | translated | high |
| `0x80C00980` | 92 | `fn_80c00980` | spawn effect `0x10C` into `+0xE80` | translated | high |

What is proven:

- `0x80C00240` allocates exactly `0x10A8` bytes through `0x801628C0`, writes
  the vtable words `0x80C59570` (actor), `0x80C5F838` (`+0xE74`) and
  `0x80C59B9C` (`+0xEB0`), stores the state-id factory address `0x80350068`
  at `+0xEBC`, and installs `0x80C00500`/`0x80C00550` as the constructor and
  destructor of a six-element, 0x18-byte controller array at `+0xECC`.
- `0x80C00900`/`0x80C00980` are the only two producers of `+0xE84`/`+0xE80`;
  the wave-4 sled family consumes exactly those two slots
  (`fn_80c00d50` reads `+0xE80`, `fn_80c012e0` reads `+0xE84`).
- The effect factory call is the same five-argument form in both spawners,
  differing only in the id (`0x10C`/`0x10D`) and whether the position is copied
  into the new effect (`0x62C/+0x630/+0x634`).

## Cluster 2 - phase and contact handlers

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C05D80` | 300 | `fn_80c05d80` | landing: reverse vertical velocity, clip `0x523`, state 2 | translated | medium |
| `0x80C08390` | 184 | `fn_80c08390` | convert speed to deceleration, arm `+0x500` | translated | medium |
| `0x80C08DC0` | 280 | `fn_80c08dc0` | wait for sub-object value, scale `+0x89D` byte | translated | medium |
| `0x80C0C650` | 136 | `fn_80c0c650` | cancel three effects, state 1 | translated | high |
| `0x80C0F660` | 464 | `fn_80c0f660` | bounce/facing/boundary handler | translated | medium |
| `0x80C0F830` | 276 | `fn_80c0f830` | bind `damage`, reset motion, state 1 | translated | high |
| `0x80C10570` | 88 | `fn_80c10570` | copy `f64` slot result to `+0x114`, state 1 | translated | high |

What is proven:

- The per-frame update ritual is identical across the handlers: model slot
  `+0x604/+0x1C`, physics `0x8006CDE0`, `0x800955F0`, ground probe
  `0x80072BC0(+0x1EC)`.
- `0x80C0F830` and `0x80C4D1C0` (cluster 5) both resolve the clip name at
  their own string address and bind it through the same
  `0x8023A0B0 -> 0x801651F0 -> 0x80169F80 -> 0x801680E0` sequence.
- `0x80C0F660` flips the facing byte `+0x348` with XOR 1 and negates the two
  horizontal speeds `+0xE8`/`+0xF4`, then calls [`fn_80c43d10`] with the hit
  flag.

## Cluster 3 - aim solvers and heading primitives

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C12790` | 120 | `fn_80c12790` | move a 16-bit heading toward a target by one step | translated | high |
| `0x80C23680` | 692 | `fn_80c23680` | vertical aim angle, axis correction, heading clamp | translated | medium |
| `0x80C23940` | 128 | `fn_80c23940` | blend two headings by a fraction | translated | high |
| `0x80C239C0` | 412 | `fn_80c239c0` | side-aware aim step for `+0x754` | translated | medium |

What is proven:

- `0x80C12790` has exactly three outcomes: return the target when it is within
  the step, clamp to `current +- step` otherwise, and treat a negative step as
  its absolute value. Both solvers call it with the constants
  `0x80C613FC` (`0x80C23680`) and `0x80C61D98` (`0x80C239C0`).
- `0x80C23940` uses three thresholds read from `0x80C57CE8`, `0x80C57D00` and
  the integer-to-double bias `0x80C57CF8`; outside the band it returns the
  endpoint heading unchanged.
- `0x80C23680` corrects the angle with the model axes `0x15`/`0x16` obtained
  through `0x80168180`, normalizes them with `0x80170980`, and compares their
  dot product against `0x80C57D00`/`0x80C57D4C`.
- The actor layout is consistent with the wave-4 manager: model at `+0x604`,
  side byte at `+0x348`, heading at `+0x744`/`+0x754`, target pointer at
  `+0x780`, cached target at `+0x746..+0x750`.

## Cluster 4 - castle target selection

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C23B60` | 368 | `fn_80c23b60` | pick the closest active player on the boss's side | translated | medium |
| `0x80C243F0` | 148 | `fn_80c243f0` | side-offset distance predicate | translated | high |

`fn_80c23fd0` (wave 4) called both addresses by raw address. They are now
translated: `0x80C23B60` iterates the four player ports through the mask at
`0x80429CA8` and passes the selected actor (or 0) to `0x802DCDC0`;
`0x80C243F0` returns 1 when phase `+0x76C` is 0 and
`side_offset + 0x800B30C0() <= x`, choosing the offset from the facing byte.

## Cluster 5 - impact effects and damage entry

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C33A20` | 112 | `fn_80c33a20` | `Wm_jr_kickhit` effect | translated | high |
| `0x80C43D10` | 380 | `fn_80c43d10` | movement/landing effect and sound dispatch | translated | medium |
| `0x80C43E90` | 96 | `fn_80c43e90` | `Wm_mo_drop` through the actor's effect object | translated | high |
| `0x80C43EF0` | 84 | `fn_80c43ef0` | `Wm_mo_landsmokebig` at the actor position | translated | high |
| `0x80C4D1C0` | 320 | `fn_80c4d1c0` | bind `damage` to model and damage animation | translated | high |

What is proven:

- The three effect names are string pointers in the export:
  `0x80C64A28` (`Wm_jr_kickhit`), `0x80C68908` (`Wm_mo_drop`) and
  `0x80C68914` (`Wm_mo_landsmokebig`).
- `0x80C4D1C0` is the two-object variant of `0x80C0F830`: it binds the same
  `damage` clip to `+0x644` and creates/binds the damage animation at `+0x6AC`
  through `0x8023A200`/`0x80166BD0`/`0x80166F10`.
- `0x80C43D10` reads a six-field table at `+0xAF0` and dispatches fields
  `+0x28/+0x2C`, `+0x30/+0x34` and `+0x38` through the `+0xA0` slots of the
  objects at `+0xC1C` and `+0xD44`.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C00240` | `FUN_80c00240` | `fn_80c00240` | Iggy actor constructor | translated | high | `80c00240_FUN_80c00240.c` |
| `0x80C00380` | `FUN_80c00380` | `fn_80c00380` | release wrapper | translated | medium | `80c00380_FUN_80c00380.c` |
| `0x80C00500` | `FUN_80c00500` | `fn_80c00500` | controller element constructor | translated | high | `80c00500_FUN_80c00500.c` |
| `0x80C00550` | `FUN_80c00550` | `fn_80c00550` | controller element destructor | translated | high | `80c00550_FUN_80c00550.c` |
| `0x80C005B0` | `FUN_80c005b0` | `fn_80c005b0` | element destructor variant | translated | high | `80c005b0_FUN_80c005b0.c` |
| `0x80C00900` | `FUN_80c00900` | `fn_80c00900` | effect `0x10D` spawn | translated | high | `80c00900_FUN_80c00900.c` |
| `0x80C00980` | `FUN_80c00980` | `fn_80c00980` | effect `0x10C` spawn | translated | high | `80c00980_FUN_80c00980.c` |
| `0x80C05D80` | `FUN_80c05d80` | `fn_80c05d80` | landing handler | translated | medium | `80c05d80_FUN_80c05d80.c` |
| `0x80C08390` | `FUN_80c08390` | `fn_80c08390` | deceleration entry | translated | medium | `80c08390_FUN_80c08390.c` |
| `0x80C08DC0` | `FUN_80c08dc0` | `fn_80c08dc0` | two-state sub-object handler | translated | medium | `80c08dc0_FUN_80c08dc0.c` |
| `0x80C0C650` | `FUN_80c0c650` | `fn_80c0c650` | cancel three effects | translated | high | `80c0c650_FUN_80c0c650.c` |
| `0x80C0F660` | `FUN_80c0f660` | `fn_80c0f660` | bounce/facing handler | translated | medium | `80c0f660_FUN_80c0f660.c` |
| `0x80C0F830` | `FUN_80c0f830` | `fn_80c0f830` | `damage` bind | translated | high | `80c0f830_FUN_80c0f830.c` |
| `0x80C10570` | `FUN_80c10570` | `fn_80c10570` | state entry | translated | high | `80c10570_FUN_80c10570.c` |
| `0x80C12790` | `FUN_80c12790` | `fn_80c12790` | heading step clamp | translated | high | `80c12790_FUN_80c12790.c` |
| `0x80C23680` | `FUN_80c23680` | `fn_80c23680` | vertical aim solver | translated | medium | `80c23680_FUN_80c23680.c` |
| `0x80C23940` | `FUN_80c23940` | `fn_80c23940` | heading blend | translated | high | `80c23940_FUN_80c23940.c` |
| `0x80C239C0` | `FUN_80c239c0` | `fn_80c239c0` | side-aware aim step | translated | medium | `80c239c0_FUN_80c239c0.c` |
| `0x80C23B60` | `FUN_80c23b60` | `fn_80c23b60` | player selection | translated | medium | `80c23b60_FUN_80c23b60.c` |
| `0x80C243F0` | `FUN_80c243f0` | `fn_80c243f0` | side-distance predicate | translated | high | `80c243f0_FUN_80c243f0.c` |
| `0x80C33A20` | `FUN_80c33a20` | `fn_80c33a20` | `Wm_jr_kickhit` | translated | high | `80c33a20_FUN_80c33a20.c` |
| `0x80C43D10` | `FUN_80c43d10` | `fn_80c43d10` | effect/sound dispatch | translated | medium | `80c43d10_FUN_80c43d10.c` |
| `0x80C43E90` | `FUN_80c43e90` | `fn_80c43e90` | `Wm_mo_drop` | translated | high | `80c43e90_FUN_80c43e90.c` |
| `0x80C43EF0` | `FUN_80c43ef0` | `fn_80c43ef0` | `Wm_mo_landsmokebig` | translated | high | `80c43ef0_FUN_80c43ef0.c` |
| `0x80C4D1C0` | `FUN_80c4d1c0` | `fn_80c4d1c0` | dual `damage` bind | translated | high | `80c4d1c0_FUN_80c4d1c0.c` |

## Unknowns

- **No-argument calls.** `0x80C00380` and the first `0x8006CED0` call in
  `0x80C23680`/`0x80C239C0` are printed without their object argument. They are
  translated literally (`call0`/`call1`) and marked `medium`; the surrounding
  flow does not depend on the missing register.
- **Vtable slot prototypes.** `0x80C0F660` (`+0x534/+0x544/+0x564/+0x570/+0x598`),
  `0x80C05D80` (`+0x54C/+0x550`), `0x80C10570` (`+0x4B4` returning `f64`), and
  the `+0xA0` slots in `0x80C43D10`/`0x80C43E90` are named by offset only. The
  argument shapes printed by the export are preserved, including the `f64`
  first argument of the `+0x550` call in `0x80C05D80`.
- **Untranslated callees.** `0x80C07870` (called by `0x80C08DC0`) and
  `0x80C0EBF0`/`0x80C00B30`/`0x80C0AB10` (wave-4 neighbours) remain outside
  this lane; their call sites keep the raw address.
- **Effect/sound table.** The six fields of the `+0xAF0` table used by
  `0x80C43D10` are addressed by offset; the export does not name the effect or
  sound resources, and the fallback z word `0x80C58B30` is read as data.
- **Aim axes.** `0x80168180(model, 0x15/0x16, out)` and `0x80170980` are
  translated as opaque model queries; the axis meaning is inferred from the
  dot-product use. `0x802E8608` is modelled as `f64 -> f64` because the export
  promotes its result to `double` before `0x80161120`.
- **Boss attribution.** `0x80C05D80`, `0x80C08390`, `0x80C08DC0`,
  `0x80C0C650`, `0x80C0F660`, `0x80C23680`, `0x80C239C0` and `0x80C43D10`
  sit between known registrars, but no export string proves the owner. The
  table records mechanics, not boss names.
- **Opaque constants.** `0x80C572D8`, `0x80C57310`, `0x80C57390`,
  `0x80C573B0`, `0x80C573BC/D4/D8`, `0x80C57474/78`, `0x80C5768C/90/D8/DC`,
  `0x80C57CE8/CF8/D00/D4C`, `0x80C58B00`, `0x80C58F74/78/FF0`,
  `0x80C5A274/78`, `0x80C5ACF0`, `0x80C5C644`, `0x80C5F838`, `0x80C59570`,
  `0x80C59B90/9C`, `0x80C613F8/FC/1400/1404/1408`, `0x80C61B7C/80`,
  `0x80C61D88/8C/98`, `0x80C64A28`, `0x80C662C6`, `0x80C68908/14`,
  `0x80C6AB78`, `0x80C70604` and `0x8042C1A0/A4` are read from their original
  addresses; their numeric values are not claimed.

## Verification

- `cargo check` passes (exit 0) with no error or warning in
  `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave5-bosses.csv` match the Rust functions one
  to one: 25 `translated`; 16 `high`, 9 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-5 ledger
  were edited.
