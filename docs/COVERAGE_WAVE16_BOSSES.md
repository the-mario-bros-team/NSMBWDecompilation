# Coverage - Wave 16, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-16
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions),
  `docs/COVERAGE_WAVE14_BOSSES.md` (25) and `docs/COVERAGE_WAVE15_BOSSES.md`
  (25). This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x8005xxxx` through `0x801Cxxxx`,
  `0x802Dxxxx`, `0x8035xxxx`, `0x8042xxxx`, `0x8066xxxx`, `0x806Bxxxx`) are
  absolute DOL addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every table
  row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` is not used in
  this pass.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores, call arguments and branch tree are exact. `medium` means the
  body is exact but a dropped argument, an object role or a helper name is
  reconstructed by convention.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 5,104 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 17 |
| confidence `medium` | 8 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 266 |
| cumulative bytes in `game::bosses` | 91,088 |

The pass fills the call-graph holes left open by waves 14 and 15. The three
companion spawners of the first flying boss (`0x80C2E8D0`, `0x80C2E980`,
`0x80C2EAC0`) and two of the three for the second (`0x80C30760`, `0x80C30820`)
are now present, together with the two destructor wrappers that release the
controller arrays they allocate. It also adds the water/hit probes, the
`face_1` joint heading helper, the route-node selector and the `jumpA` phase
entry. The large dispatchers and the paired-single-heavy
`0x80C30560`/`0x80C0A910`/`0x80C0AB10` bodies remain out of scope.

## Synthetic-base warning in one place

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The spawn-table rows at
`0x80C6F430..0x80C6F45C` are synthetic-BSS words, not hardware addresses.

## Cluster 1 - destructor wrappers and companion placement (1,988 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C11D60` | 140 | `fn_80c11d60` | release the `0x128`/`0x114` controller arrays | translated | high |
| `0x80C55F30` | 132 | `fn_80c55f30` | release two eight-slot `0x0C`-stride arrays | translated | high |
| `0x80C2E8D0` | 168 | `fn_80c2e8d0` | spawn the `0x128` companion and place it | translated | medium |
| `0x80C2E980` | 312 | `fn_80c2e980` | fill and emit the `0x19D` row table | translated | high |
| `0x80C2EAC0` | 288 | `fn_80c2eac0` | seven-object `0x12C` arc | translated | high |
| `0x80C30760` | 188 | `fn_80c30760` | mirrored `0x186` pair | translated | high |
| `0x80C30820` | 288 | `fn_80c30820` | seven-object `0x12D` arc | translated | high |
| `0x80C32680` | 324 | `fn_80c32680` | `0x137`/`0x12A` pair | translated | medium |
| `0x80C4B230` | 148 | `fn_80c4b230` | spawn the `0x107` helper | translated | medium |

What is proven:

- `0x80C11D60` releases the `0x128`-stride array at `+0xFBC` with the
  translated element destructor `0x80C005B0`, then the `0x114`-stride array at
  `+0xEA8` with `0x80C04CC0`. Both descriptors are byte-for-byte the same as
  the wave-15 constructor pair, which is the strongest cross-check in the pass.
- `0x80C55F30` is the mirror image of the wave-15 interval constructor: it
  releases `+0x21C` then `+0x1BC`, both `0x0C`-stride with eight slots, using
  the translated array callback `0x80C00200`.
- `0x80C2E8D0`, `0x80C30760` and `0x80C32680` store the spawned object's owner
  word at `+0x738` and then overwrite the child's `+0xAC`/`+0xB0` from the
  parent. These are the placements that wave 15 called by raw address.
- `0x80C2E980` is a one-time table builder. The flag is `0x80C6F428`; the four
  3-float rows start at `0x80C6F430` and are `(x-184, z-112, 500)`,
  `(x+184, z-112, 500)`, `(x-208, z-112, 500)` and `(x+208, z-112, 500)`.
  The row count is selected by `0x8005FEF0` from the four-word table at
  `0x80C57F90` (`2, 2, 3, 4`).
- `0x80C2EAC0` and `0x80C30820` are the same seven-iteration arc with type
  `0x12C`/`0x12D` and target fields `+0xD78`/`+0xD54`. The export's GQR and
  `ldexpf` preamble is compiler bookkeeping; the arithmetic
  `32 + (x-224) + 64*i`, `z-120`, `0` is exact.

## Cluster 2 - phase/clip/attack and motion (1,340 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C2CC60` | 152 | `fn_80c2cc60` | per-frame clip-finished notification | translated | high |
| `0x80C4B9F0` | 348 | `fn_80c4b9f0` | `jumpA` phase entry and launch speeds | translated | high |
| `0x80C43FD0` | 188 | `fn_80c43fd0` | hit/guard clip dispatch | translated | high |
| `0x80C477B0` | 172 | `fn_80c477b0` | damage/knockback state dispatch | translated | high |
| `0x80C56720` | 324 | `fn_80c56720` | attack-vector builder and integration | translated | high |
| `0x80C56870` | 156 | `fn_80c56870` | velocity + drift integration | translated | high |

What is proven:

- `0x80C2CC60` steps the two controllers `+0x548` (slot `+0x1C`) and `+0x5C0`
  (slot `+0x14`), and only when both `0x80169020(+0x588)` and
  `0x80166F20(+0x5C0, 0)` are non-zero writes `+0x634 = 1` and sends
  `0x80C6F1B0` through class slot `+0xD4`.
- `0x80C4B9F0` binds the `jumpA` clip (`0x80C6ABB0`) through `0x8023A200` and
  `0x80166BD0`, applies the `0.0`/`1.0` blend values, chooses the speed pair at
  `+0x14` or `+0x24` of the `+0x754` table from two controller flags, and
  writes `+0xEC/+0xE8` using the signed side table at `0x8042C1A0` and class
  slot `+0x620`.
- `0x80C43FD0` and `0x80C477B0` share the controller query
  `vtable(+0x394)+0x28` then `+0x14`; the first selects clips
  `0x507`/`0x4B3` or `0x505`, the second copies slot `+0x4B4` into `+0x114`,
  sets `+0x848 = 1` and sends `0x80358204`.
- `0x80C56720` selects `(-x,-y,-z)` or `(-x,-y,-300-z)` from bit `2` of
  `+0x70`, stores the vector angle at `+0x178`, and adds the selected vector
  to `+0xC/+0x10/+0x14`.
- `0x80C56870` sets flag `4` from `+0xF8` and adds velocity plus drift to
  `+0x24/+0x28/+0x2C`.

## Cluster 3 - collision, damage and impact (920 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C3E270` | 228 | `fn_80c3e270` | jittered water-column probe | translated | medium |
| `0x80C4B0E0` | 156 | `fn_80c4b0e0` | lowest-`+0x534` `0x107` selector | translated | medium |
| `0x80C4B5B0` | 148 | `fn_80c4b5b0` | splash at the clamped height | translated | medium |
| `0x80C543B0` | 244 | `fn_80c543b0` | segment hit and impulse record | translated | high |
| `0x80C3E1A0` | 144 | `fn_80c3e1a0` | `Wm_lu_jumpsmoke`/`Wm_lu_sweat` spawner | translated | high |

What is proven:

- `0x80C3E270` probes `(x, y)` with `0x80064790`, draws two `0x800B30C0`
  samples, adds them to `+0x840`/`+0x844`, and accepts only when
  `2.0 + (margin + sample)` is below x and x is below `upper - 2.0`; the `2.0`
  comes from `0x80C58844`. On success it writes the first byte of the class
  name (slot `+0x6C`) through the optional out pointer.
- `0x80C4B0E0` walks at most `0x32` objects of type `0x107` with
  `0x80162D50`, keeps the minimum `+0x534`, and returns null unless at least
  three were seen.
- `0x80C4B5B0` builds a point with `0x800E7880`, clamps its y to the actor
  height, writes `5500.0` (`0x80C58FA8`) as z and submits it with
  `0x8016C890`.
- `0x80C543B0` tests flag bit `8`, calls the collider at `+4` slot `+0x24`
  with the segment `+0x128`/`+0x134`, stores the result at `+0x140`, and on a
  hit records `|magnitude|` at `+0x118`, the direction at `+0x11C..+0x124`,
  the relative offset at `+0xB8..+0xC0`, and clears `+0x285/+0x286`.
- `0x80C3E1A0` fills a 20-byte buffer with `0x80168230` and sends the two
  strings at `0x80C67628`/`0x80C67638` through effect slot `+0xA0`.

## Cluster 4 - aim, heading and path (856 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C36F60` | 208 | `fn_80c36f60` | add a heading offset to `face_1` | translated | medium |
| `0x80C39380` | 140 | `fn_80c39380` | move progress toward a target by `0.05` | translated | high |
| `0x80C050D0` | 212 | `fn_80c050d0` | fall-step predictor | translated | high |
| `0x80C05200` | 224 | `fn_80c05200` | nearest adjacent route node | translated | medium |
| `0x80C05CB0` | 72 | `fn_80c05cb0` | clamped two-point interpolation | translated | high |

What is proven:

- `0x80C36F60` compares the resource name suffix with `face_1`
  (`0x80C6552C`), reads the joint rotation with `0x80241710`, adds
  `angle * 90 / 0x4000` to its third `f32` component, and writes it back with
  `0x80241AA0`. The signed division is the exact truncating form printed in
  the export.
- `0x80C39380` clamps the current `0x801680C0` value to within the `0.05`
  step at `0x80C585E8` of the requested target and writes it with
  `0x801680E0`.
- `0x80C050D0` loops while the velocity is positive or the ground probe
  `0x807131C0` is at or below the predicted height, applies the `+0x114`
  acceleration, clamps at `+0xF8`, and returns the step count.
- `0x80C05200` tries `current-1` and `current+1` (indices below `5`), compares
  the node x with the reference position from `0x80C051B0`, seeds the search
  with `FLT_MAX` (`0x80C57308`), and publishes the best index through
  `0x802DCDBC`.
- `0x80C05CB0` returns `low` below `0.0`, `high` above `1.0`, and
  `low + t*(high-low)` in between, with the interpolation rounded to `f32`.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C050D0` | `FUN_80c050d0` | `fn_80c050d0` | fall-step predictor | translated | high | `80c050d0_FUN_80c050d0.c` |
| `0x80C05200` | `FUN_80c05200` | `fn_80c05200` | route-node selector | translated | medium | `80c05200_FUN_80c05200.c` |
| `0x80C05CB0` | `FUN_80c05cb0` | `fn_80c05cb0` | clamped interpolation | translated | high | `80c05cb0_FUN_80c05cb0.c` |
| `0x80C11D60` | `FUN_80c11d60` | `fn_80c11d60` | controller-array destructor | translated | high | `80c11d60_FUN_80c11d60.c` |
| `0x80C2CC60` | `FUN_80c2cc60` | `fn_80c2cc60` | clip-finished notification | translated | high | `80c2cc60_FUN_80c2cc60.c` |
| `0x80C2E8D0` | `FUN_80c2e8d0` | `fn_80c2e8d0` | `0x128` companion spawn | translated | medium | `80c2e8d0_FUN_80c2e8d0.c` |
| `0x80C2E980` | `FUN_80c2e980` | `fn_80c2e980` | `0x19D` row table | translated | high | `80c2e980_FUN_80c2e980.c` |
| `0x80C2EAC0` | `FUN_80c2eac0` | `fn_80c2eac0` | `0x12C` seven-arc | translated | high | `80c2eac0_FUN_80c2eac0.c` |
| `0x80C30760` | `FUN_80c30760` | `fn_80c30760` | mirrored `0x186` pair | translated | high | `80c30760_FUN_80c30760.c` |
| `0x80C30820` | `FUN_80c30820` | `fn_80c30820` | `0x12D` seven-arc | translated | high | `80c30820_FUN_80c30820.c` |
| `0x80C32680` | `FUN_80c32680` | `fn_80c32680` | `0x137`/`0x12A` pair | translated | medium | `80c32680_FUN_80c32680.c` |
| `0x80C36F60` | `FUN_80c36f60` | `fn_80c36f60` | `face_1` heading offset | translated | medium | `80c36f60_FUN_80c36f60.c` |
| `0x80C39380` | `FUN_80c39380` | `fn_80c39380` | progress approach | translated | high | `80c39380_FUN_80c39380.c` |
| `0x80C3E1A0` | `FUN_80c3e1a0` | `fn_80c3e1a0` | `Wm_lu_*` effect spawner | translated | high | `80c3e1a0_FUN_80c3e1a0.c` |
| `0x80C3E270` | `FUN_80c3e270` | `fn_80c3e270` | water-column probe | translated | medium | `80c3e270_FUN_80c3e270.c` |
| `0x80C43FD0` | `FUN_80c43fd0` | `fn_80c43fd0` | hit/guard clip dispatch | translated | high | `80c43fd0_FUN_80c43fd0.c` |
| `0x80C477B0` | `FUN_80c477b0` | `fn_80c477b0` | damage state dispatch | translated | high | `80c477b0_FUN_80c477b0.c` |
| `0x80C4B0E0` | `FUN_80c4b0e0` | `fn_80c4b0e0` | lowest-`+0x534` selector | translated | medium | `80c4b0e0_FUN_80c4b0e0.c` |
| `0x80C4B230` | `FUN_80c4b230` | `fn_80c4b230` | `0x107` helper spawn | translated | medium | `80c4b230_FUN_80c4b230.c` |
| `0x80C4B5B0` | `FUN_80c4b5b0` | `fn_80c4b5b0` | splash submission | translated | medium | `80c4b5b0_FUN_80c4b5b0.c` |
| `0x80C4B9F0` | `FUN_80c4b9f0` | `fn_80c4b9f0` | `jumpA` phase entry | translated | high | `80c4b9f0_FUN_80c4b9f0.c` |
| `0x80C543B0` | `FUN_80c543b0` | `fn_80c543b0` | segment hit record | translated | high | `80c543b0_FUN_80c543b0.c` |
| `0x80C55F30` | `FUN_80c55f30` | `fn_80c55f30` | interval-array destructor | translated | high | `80c55f30_FUN_80c55f30.c` |
| `0x80C56720` | `FUN_80c56720` | `fn_80c56720` | attack-vector builder | translated | high | `80c56720_FUN_80c56720.c` |
| `0x80C56870` | `FUN_80c56870` | `fn_80c56870` | velocity + drift integration | translated | high | `80c56870_FUN_80c56870.c` |

## Data words read from the analysis image

These values are quoted so the constants in the Rust bodies are auditable.
They are analysis-image words, not runtime addresses.

| address | value | used by |
|---|---|---|
| `0x80C57F74` | `0.0` | `fn_80c2e980`, `fn_80c2eac0` |
| `0x80C57F78` | `120.0` | `fn_80c2eac0` |
| `0x80C57F7C` | `112.0` | `fn_80c2e980` |
| `0x80C57F88` | `3064.0` | `fn_80c2e8d0` |
| `0x80C57F8C` | `0.7` | `fn_80c2e8d0` |
| `0x80C57F90..9C` | signed words `2, 2, 3, 4` | `fn_80c2e980` |
| `0x80C57FA0` | `184.0` | `fn_80c2e980` |
| `0x80C57FA4` | `500.0` | `fn_80c2e980` |
| `0x80C57FA8` | `208.0` | `fn_80c2e980` |
| `0x80C57FAC` | `64.0` | `fn_80c2eac0` |
| `0x80C57FB0` | `0.5` | `fn_80c2eac0` |
| `0x80C57FB4` | `224.0` | `fn_80c2eac0` |
| `0x80C58090` | `120.0` | `fn_80c30820` |
| `0x80C580A4` | `0.5` | `fn_80c30820` |
| `0x80C580B0` | `224.0` | `fn_80c30760`, `fn_80c30820` |
| `0x80C580B4` | `-3500.0` | `fn_80c30760` |
| `0x80C580B8` | `0.0` | `fn_80c30820` |
| `0x80C580BC` | `64.0` | `fn_80c30820` |
| `0x80C581E4` | `3064.0` | `fn_80c32680` |
| `0x80C581E8` | `40.0` | `fn_80c32680` |
| `0x80C581EC` | `0.7` | `fn_80c32680` |
| `0x80C581F0` | `136.0` | `fn_80c32680` |
| `0x80C581F4` | `0.8` | `fn_80c32680` |
| `0x80C585E8` | `0.05` | `fn_80c39380` |
| `0x80C58844` | `2.0` | `fn_80c3e270` |
| `0x80C58F74` | `1.0` | `fn_80c4b9f0` |
| `0x80C58F78` | `0.0` | `fn_80c4b9f0` |
| `0x80C58FA8` | `5500.0` | `fn_80c4b5b0` |
| `0x80C58FB0` | double bias `2^52 + 2^31` | `fn_80c4b9f0` |
| `0x80C5932C` | `0.0` | `fn_80c56720` |
| `0x80C59350` | `-300.0` | `fn_80c56720` |
| `0x80C572D8` | `0.0` | `fn_80c050d0`, `fn_80c05cb0` |
| `0x80C57300` | signed `-1` | `fn_80c05200` |
| `0x80C57304` | signed `1` | `fn_80c05200` |
| `0x80C57308` | `FLT_MAX` (`3.4028235e38`) | `fn_80c05200` |
| `0x80C57310` | `1.0` | `fn_80c05cb0` |
| `0x80C6AB2C` | signed `1` | `fn_80c4b5b0` |
| `0x80C6F428` | BSS flag byte | `fn_80c2e980` |

## Unknowns

- **Dropped arguments.** Class slots `+0x314`, `+0x3F0`, `+0x3F4`, `+0x4B4`
  and `+0x620` are printed without arguments in several exports. The
  translations pass the receiver object by convention and mark the affected
  rows `medium`.
- **Untranslated callees.** `0x8005FEF0` (player/count source),
  `0x80064790` (water probe), `0x800B30C0` (random sample), `0x80162D50`
  (object list walk), `0x80663290` (imported thunk), `0x806B7690`,
  `0x807131C0` (ground height), `0x80C051B0` (reference position),
  `0x80C56240` (flag setter), `0x802DCD70`/`0x802DCDBC` (route accessor pair)
  and the spawn helpers `0x80064610`/`0x80064680`/`0x8006CED0` are still
  called by raw address.
- **Object kinds.** Spawn types `0x128`, `0x12A`, `0x137`, `0x186`, `0x19D`,
  `0x12C`, `0x12D`, `0x107` and `0x105` are opaque. Only their numeric ids,
  placement points and destination fields are proven.
- **BSS names.** `0x80C706C4` (used by `fn_80c43fd0`) and the DOL words
  `0x803585C4`/`0x80358204` (used by `fn_80c477b0`) are runtime-initialized
  class descriptors, not strings in the REL image.
- **Dead stack vector.** The export of `fn_80c56720` writes a second local
  vector in each branch but never reads it. Those dead stores are omitted and
  noted in the function doc; every observable store is preserved.
- **Paired-single noise.** `fn_80c2eac0`, `fn_80c30820`, `fn_80c3e270`,
  `fn_80c4b9f0` and `fn_80c543b0` carry the compiler's GQR save/restore and
  `ldexpf` blocks. They are bookkeeping, not gameplay; the arithmetic after
  them is translated.

## Verification

- `cargo check` passes (exit 0); no warning points at `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave16-bosses.csv` match the Rust functions one
  to one: 25 `translated`; 17 `high`, 8 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-16 ledger
  were edited.
