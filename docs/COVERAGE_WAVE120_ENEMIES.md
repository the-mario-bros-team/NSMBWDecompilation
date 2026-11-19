# Coverage - Wave 120 (`game::enemies`, the `0x80AB0000` bucket)

## How to read this file

This page records the Wave 120 `game::enemies` lane: 30 functions of the
`0x80AB0000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 101 of that file. Every row of the machine
readable mirror (`ledger/agents/wave120-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 6,108 | 12 | 18 |

The bucket holds 188 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 129 were already in
`ledger/functions.csv`, leaving 59. This pass claims 30, all of them
contiguous, and leaves 29 untouched.

## What this pass covers

- **The `0x710`-byte `daEnManholeChoropu_c` actor
  (`0x80ABD750..0x80ABF650`, 30 bodies, one unbroken run).** The class state
  table is already translated as [`register_manhole_choropu_states`]
  (`0x80ABFA90`), which is what lets this pass name the roots:
  - the constructor `0x80ABD750` installs class vtable `0x80B580F8`, the
    `+0x528` animation holder, the `+0x548` body holder, the `+0x588`
    clip-blend sub-object (vtable `0x80B330E0`) and the `+0x608` effect driver;
  - `0x80ABD950` loads `g3d/choropoo.brres`, binds the `choropoo` model to
    `+0x548` and the `walk` clip to `+0x588`;
  - `0x80ABDB00`/`0x80ABDB70` are the per-frame holder refresh and the matrix
    rebuild over `+0xAC..+0xB4`, `+0x100..+0x104` and the `+0xDC` scale;
  - `0x80ABDC50`/`0x80ABDCC0` read the `manhole` and `arm_spanner_left`/
    `arm_spanner_right` node frames into the cached points `+0x6F0`/`+0x5C0`;
  - `0x80ABDF80` scans the four player slots for the nearest one in front of
    the actor (side byte `+0x348`) and returns its number, which
    `0x80ABEEB0` stores at `+0x5F4`;
  - `0x80ABE0B0`/`0x80ABE3C0` spawn effects `0x177` and `0x174 + side`, and
    link the new object's `+0x444` owner word;
  - `0x80ABE130`/`0x80ABE2D0`/`0x80ABE320`/`0x80ABE4A0` place the child
    object selected through `+0x700`/`+0x5F0` from a rotation matrix composed
    with a constant offset triple;
  - `0x80ABE630` writes the tracked player's world point into the child's
    `+0x438` target when the player stands on the side selected by the
    `main.dol` byte table at `0x8042C1A0`;
  - the five `throw_1..throw_5` clip entry/step pairs
    (`0x80ABE8B0..0x80ABF1A0`) drive the throw phase word `+0x360`, the
    `+0x5F8` timer, the two velocity triples at `+0xE8..+0xFC`, the turn
    toward the `0x8042C1A4` per-side angle, the `0x21F`/`0x220`/`0x221` sound
    events and the `Wm_en_landsmoke_s` landing effect; `0x80ABF650` is the
    small per-frame re-seat hook.

### State entry names

`register_manhole_choropu_states` passes each fState record plus its
`record + 0x10` word to the engine. The switch targets used by this run are
exactly those `+0x10` words, so the docs can name them:

| target | record | `StateID_` name |
|---|---|---|
| `0x80B756D8` | `0x80B756C8` | `Hide` |
| `0x80B75718` | `0x80B75708` | `AppearLittle` |
| `0x80B75758` | `0x80B75748` | `Search` |
| `0x80B75798` | `0x80B75788` | `AppearFull` |
| `0x80B757D8` | `0x80B757C8` | `Attack` |
| `0x80B75818` | `0x80B75808` | `Disappear` |

### Why eighteen rows are `medium`

- **Dropped arguments.** The export recovers the actor from the
  `0x802DCD74` section prologue in `0x80ABDF80` and `0x80ABE4A0`; the Rust
  takes it as an argument, and the `0x802DCDC0` epilogue is not modelled as a
  call, following the precedent of [`fn_80a77820`].
- **Unnamed engine callees.** `0x800964A0`/`0x80095C80`/`0x800A0FB0` (the
  collision dispatch), `0x800A81E0`, `0x8016ED10` and the two `main.dol`
  per-side tables have no established names; their roles are inferred from
  their arguments and their position in the state machine.
- **Decompiler gaps.** `0x80ABE130` and `0x80ABE0B0` extract the `+0x5C0`
  translation into a buffer the export never reads; both calls are kept and
  the dead store is documented instead of silently removed. `0x80ABE4A0`
  loses the arguments of `0x801C0BD0` in one branch, recovered from the three
  visible stack floats.
- **Paired-single noise.** `0x80ABDF80` opens and closes with `GQR0`/`ldexpf`
  fragments and drops the receiver of the player vtable `+0x6C` call; the
  visible comparisons are exact, the receiver is forwarded and marked.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80ABD750` | `fn_80abd750` | 140 | medium |
| `0x80ABD950` | `fn_80abd950` | 228 | high |
| `0x80ABDB00` | `fn_80abdb00` | 100 | medium |
| `0x80ABDB70` | `fn_80abdb70` | 220 | high |
| `0x80ABDC50` | `fn_80abdc50` | 112 | medium |
| `0x80ABDCC0` | `fn_80abdcc0` | 120 | medium |
| `0x80ABDDC0` | `fn_80abddc0` | 100 | high |
| `0x80ABDEF0` | `fn_80abdef0` | 132 | medium |
| `0x80ABDF80` | `fn_80abdf80` | 304 | medium |
| `0x80ABE0B0` | `fn_80abe0b0` | 116 | medium |
| `0x80ABE130` | `fn_80abe130` | 412 | medium |
| `0x80ABE2D0` | `fn_80abe2d0` | 80 | high |
| `0x80ABE320` | `fn_80abe320` | 160 | high |
| `0x80ABE3C0` | `fn_80abe3c0` | 132 | medium |
| `0x80ABE450` | `fn_80abe450` | 68 | high |
| `0x80ABE4A0` | `fn_80abe4a0` | 388 | medium |
| `0x80ABE630` | `fn_80abe630` | 340 | medium |
| `0x80ABE790` | `fn_80abe790` | 160 | medium |
| `0x80ABE840` | `fn_80abe840` | 112 | high |
| `0x80ABE8B0` | `fn_80abe8b0` | 200 | high |
| `0x80ABE990` | `fn_80abe990` | 364 | medium |
| `0x80ABEB00` | `fn_80abeb00` | 124 | high |
| `0x80ABEB90` | `fn_80abeb90` | 216 | high |
| `0x80ABEC70` | `fn_80abec70` | 232 | high |
| `0x80ABED70` | `fn_80abed70` | 308 | medium |
| `0x80ABEEB0` | `fn_80abeeb0` | 208 | medium |
| `0x80ABEF90` | `fn_80abef90` | 328 | medium |
| `0x80ABF0E0` | `fn_80abf0e0` | 176 | high |
| `0x80ABF1A0` | `fn_80abf1a0` | 444 | medium |
| `0x80ABF650` | `fn_80abf650` | 84 | medium |

## Deliberate skips

- **The 29 remaining addresses of the bucket were not attempted.** They lie
  outside the single contiguous run this pass targeted and are, in ascending
  order: `0x80AB0570` (664), `0x80AB0870` (204), `0x80AB0C00` (440),
  `0x80AB0EE0` (640), `0x80AB1160` (384), `0x80AB12E0` (244),
  `0x80AB13E0` (304), `0x80AB1510` (624), `0x80AB1780` (172),
  `0x80AB1830` (92), `0x80AB1890` (104), `0x80AB1900` (256),
  `0x80AB1A00` (8), `0x80AB21D0` (176), `0x80AB22D0` (276),
  `0x80AB2460` (444), `0x80AB2620` (112), `0x80AB26A0` (408),
  `0x80AB2840` (356), `0x80AB29C0` (376), `0x80AB5710` (1,064),
  `0x80AB9AF0` (284), `0x80ABA080` (308), `0x80ABA5B0` (64),
  `0x80ABA5F0` (64), `0x80ABA950` (156), `0x80ABAB50` (236),
  `0x80ABB1B0` (48), `0x80ABB1E0` (48). Nothing was written for them, so no
  ledger row claims them.
- **`0x80ABFA90` (`register_manhole_choropu_states`) is not duplicated.** It
  was already translated and is referenced by name from this pass.
- **The `+0x168`, `+0x118`, `+0x410` and `+0x434` fields stay `unknown`.**
  Their stores are literal, but the export offers no reader that would fix
  their meaning; they are documented as unknown in the bodies rather than
  given invented names.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  6,108 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave120` reports the lane CSV clean.
- `cargo check --message-format=short` emits no error naming
  `src/game/enemies.rs`.
- The module header was updated: cluster item 101, the cluster count (one
  hundred to one hundred and one), the function totals (2,697 to 2,727 public
  translations, 2,626 to 2,656 `fn_<addr>` bodies), the "start here" list and
  the records list.
