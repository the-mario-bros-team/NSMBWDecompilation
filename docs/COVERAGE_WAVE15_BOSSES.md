# Coverage - Wave 15, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-15
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions),
  `docs/COVERAGE_WAVE14_BOSSES.md` (25) and the wave-3/4/5/6/10/12/13 boss
  documents (20 + 24 + 25 + 25 + 25 + 24 + 25). This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x8005xxxx` through `0x8019xxxx`,
  `0x802Dxxxx`) are absolute DOL addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every table
  row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` is not used in
  this pass.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores, call arguments and branch tree are exact. `medium` means the
  body is exact but a dropped argument, an untranslated helper prototype or an
  object role is reconstructed by convention.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. Data words are quoted only where the analysis image was
  read directly.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 5,644 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 20 |
| confidence `medium` | 5 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 241 |
| cumulative bytes in `game::bosses` | 85,984 |

The pass fills three gaps left by waves 13 and 14: the remaining actor and
element constructors of the castle/crew families, the state-entry and per-frame
half of the fly-boss and `0x15C8` bodies, and the aim/collision math helpers
that those handlers call. It deliberately keeps the large dispatchers out of
scope.

## Synthetic-base warning in one place

| item | value |
|---|---:|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The vtables `0x80C5D3B8`,
`0x80C60960`, `0x80C61DF0`, `0x80C62248`, `0x80C62900`, `0x80C634E8`,
`0x80C676C8`, `0x80C67D78`, `0x80C69A00` and `0x80C6A058` are data/BSS words in
the analysis image, not hardware addresses.

## Cluster 1 - constructors and element lifecycle (1,732 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C11C70` | 168 | `fn_80c11c70` | `0x80C69A00`-family constructor (`0x1118`) | translated | high |
| `0x80C215C0` | 368 | `fn_80c215c0` | large actor/phase constructor (`0x15C8`) | translated | high |
| `0x80C21770` | 88 | `fn_80c21770` | `0x128` element initializer | translated | medium |
| `0x80C217D0` | 104 | `fn_80c217d0` | `0x140` element constructor | translated | high |
| `0x80C21840` | 140 | `fn_80c21840` | `0x140` element destructor | translated | medium |
| `0x80C295C0` | 140 | `fn_80c295c0` | small actor constructor (`0x6B0`) | translated | high |
| `0x80C2C350` | 216 | `fn_80c2c350` | medium actor constructor (`0x768`) | translated | high |
| `0x80C3D5A0` | 128 | `fn_80c3d5a0` | second `0x1118` actor constructor | translated | high |
| `0x80C55DB0` | 380 | `fn_80c55db0` | effect/interval object constructor | translated | medium |

What is proven:

- `0x80C11C70` is the base allocator that installs `0x80C69A00` and then the
  derived vtable `0x80C5D3B8`; it registers the `0x114` and `0x128`
  controllers. The previously untranslated constructor referenced by
  `fn_80c46840` is now present.
- `0x80C215C0` is a flat constructor: eight embedded objects, four vtable
  words (`0x803295D8`, `0x80329518`, `0x80329548`, `0x80329578`) and three
  controller registrations at `+0x790`, `+0xE80`, `+0x10C0`.
- The `0x128` element pair `0x80C21770`/`0x80C217D0` and the `0x140` element
  destructor `0x80C21840` share the vtable `0x80310ED8` (and `0x80329958` in
  the destructor), exactly as printed.
- `0x80C55DB0` stores the raw word at `0x80C59330` (the float `9999.0`) at
  `+0x190`, registers two eight-slot `0xC`-stride arrays, and configures two
  intervals through `0x80C508F0` with `(0.1, 3.0, 1.0, 0.0)` and
  `(0.1, 10.0, 1.0, 0.0)`, values read from `0x80C59334/38/3C/2C/40`.

## Cluster 2 - state entries and per-frame handlers (1,796 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C2E5C0` | 352 | `fn_80c2e5c0` | fly-boss A state entry | translated | high |
| `0x80C30240` | 276 | `fn_80c30240` | fly-boss B state entry | translated | high |
| `0x80C316D0` | 284 | `fn_80c316d0` | per-frame `+0x360` handler | translated | high |
| `0x80C33A90` | 292 | `fn_80c33a90` | model-matrix rebuild | translated | high |
| `0x80C35990` | 236 | `fn_80c35990` | animation-progress clip selector | translated | high |
| `0x80C53C60` | 356 | `fn_80c53c60` | interval/position per-frame update | translated | medium |

What is proven:

- The two fly-boss entries share one shape: clear the three `+0xDC` speeds,
  record the side byte from `0x80096240(actor, actor + 0xAC)`, clear the nine
  wing stores `+0xC3C..+0xC5C`, initialize the collision sphere at `+0x148`,
  set model state `3`, derive `+0x788/+0x78C/+0xAC/+0x7A0`, then call three
  sub-spawners and `0x8006D090`. The differences are the reference offsets
  (`120`/`112` versus `120`/`112`), the companion constant (`+0xD74 = 1.0`
  versus `+0xD50 = 1.0`) and the spawner addresses.
- `0x80C316D0` is the two-state tail of the fly-boss: state 1 waits on
  `0x80169020(+0x644)`, releases `+0x7A8`, plays controller clip `0x5AF`,
  increments `+0x360` and writes the 20-frame timer at `+0x500`; state 2
  waits for the timer and sends `0x80C6F5F0` through class slot `+0xD4`.
- `0x80C33A90` copies a 12-word matrix from `0x800A42B0` into `+0x7C`, applies
  the rig heading `+0xD80 + 0x4000` and the selector `+0x102`, and pushes the
  matrix into model `+0x604`.
- `0x80C35990` uses the six progress values `8`, `28`, `47`, `21`, `40`, `60`
  to choose controller clip `0x522` or `0x508`.
- `0x80C53C60` reads the global object `0x8042A1A0`, integrates
  `+0x94/+0x98/+0x9C` into `+0x24/+0x28/+0x2C`, steps `+0x7C`, updates the
  `+0x74` flag word through `0x80C56240`, and mirrors the position to
  `+0x0C/+0x10/+0x14`.

## Cluster 3 - facing, collision and damage (1,016 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C38900` | 152 | `fn_80c38900` | side-dependent turn toward partner | translated | high |
| `0x80C390B0` | 228 | `fn_80c390b0` | two-name collision/action query | translated | high |
| `0x80C39410` | 132 | `fn_80c39410` | damage multiplier from tier | translated | high |
| `0x80C399B0` | 236 | `fn_80c399b0` | state entry with side-derived impulse | translated | high |
| `0x80C47340` | 168 | `fn_80c47340` | four-slot Morton damage relay | translated | high |
| `0x80C473F0` | 100 | `fn_80c473f0` | release of the four relay slots | translated | medium |

What is proven:

- `0x80C38900` uses the two signed halfwords at `0x80C662D0` (`0x4000` =
  `16384`) and `0x80C662D2` (`0x1000` = `4096`) and applies the delta with
  `0x8015F4F0(actor + 0xEB8, delta, 0x1000)`.
- `0x80C390B0` gates on `+0x5F4 == 0`, `+0x1EA != 0` and `(+0x794 & 2) == 0`,
  compares two controller names, and reduces the second comparison with the
  `countLeadingZeros((-v | v) >> 31) >> 5` idiom.
- `0x80C39410` returns `1.1` for a non-zero first tier and `1.5`/`1.25` for the
  second tier.
- `0x80C399B0` selects `2.0`/`2.5` from the `+0x5F8` tiers, converts the signed
  direction byte through the bias at `0x80C585E0`, and writes `+0xE8/+0xEC/
  +0xF0/+0xF4/+0x11C`, with the duplicate `+0xEC` clear preserved.
- `0x80C47340` is the wave-15 sibling of the wave-14 `0x80C43860` relay: four
  slots, the same live/side tests, `0x80137510(player, 1)`, a signed-halfword
  timer at `0x80C698EC` (`150`) written to `+0xE94`, and the lock pair
  `0x802DCD6C`/`0x802DCDB8`.
- `0x80C473F0` walks the same four slots and calls `0x80137650` for each
  flagged live player.

## Cluster 4 - aim/travel math (1,100 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C22580` | 260 | `fn_80c22580` | clamped ratio for signed halfwords | translated | high |
| `0x80C25CB0` | 260 | `fn_80c25cb0` | clamped ratio for signed words | translated | high |
| `0x80C2EDC0` | 292 | `fn_80c2edc0` | model-space projection into `+0xC48` group | translated | high |
| `0x80C33880` | 288 | `fn_80c33880` | model-space projection into `+0xC9C` group | translated | high |

What is proven:

- `0x80C22580` and `0x80C25CB0` are the same comparison tree with halfword and
  word loads. Equal bounds return `0.0`; outside the interval the result is
  `0.0` or `1.0`; inside, the ratio is `(sample - low) / (high - low)` using the
  `0x43300000` bias at `0x80C57CF8`.
- `0x80C2EDC0` and `0x80C33880` call `0x80168180(model + 0x604, 0x1E, buf)`,
  transform the caller's point with `0x8016F130(buf, point)`, and write the
  transformed point to `+0xC48/+0xC4C/+0xC50` (or `+0xC9C/+0xCA0/+0xCA4`).
  The disassembly shows the temporary is three 16-byte vectors; the source
  reads their first words at offsets `0`, `0x10`, `0x20`.
- The near offset is `8` and the far offset is `100` in both helpers; the
  vertical base is `5500` (`0x80C57FC4` / `0x80C58220`).

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C11C70` | `FUN_80c11c70` | `fn_80c11c70` | `0x80C69A00`-family constructor | translated | high | `80c11c70_FUN_80c11c70.c` |
| `0x80C215C0` | `FUN_80c215c0` | `fn_80c215c0` | large actor/phase constructor | translated | high | `80c215c0_FUN_80c215c0.c` |
| `0x80C21770` | `FUN_80c21770` | `fn_80c21770` | `0x128` element init | translated | medium | `80c21770_FUN_80c21770.c` |
| `0x80C217D0` | `FUN_80c217d0` | `fn_80c217d0` | `0x140` element ctor | translated | high | `80c217d0_FUN_80c217d0.c` |
| `0x80C21840` | `FUN_80c21840` | `fn_80c21840` | `0x140` element dtor | translated | medium | `80c21840_FUN_80c21840.c` |
| `0x80C22580` | `FUN_80c22580` | `fn_80c22580` | halfword ratio clamp | translated | high | `80c22580_FUN_80c22580.c` |
| `0x80C25CB0` | `FUN_80c25cb0` | `fn_80c25cb0` | word ratio clamp | translated | high | `80c25cb0_FUN_80c25cb0.c` |
| `0x80C295C0` | `FUN_80c295c0` | `fn_80c295c0` | small actor constructor | translated | high | `80c295c0_FUN_80c295c0.c` |
| `0x80C2C350` | `FUN_80c2c350` | `fn_80c2c350` | medium actor constructor | translated | high | `80c2c350_FUN_80c2c350.c` |
| `0x80C2E5C0` | `FUN_80c2e5c0` | `fn_80c2e5c0` | fly-boss A entry | translated | high | `80c2e5c0_FUN_80c2e5c0.c` |
| `0x80C2EDC0` | `FUN_80c2edc0` | `fn_80c2edc0` | projection group A | translated | high | `80c2edc0_FUN_80c2edc0.c` |
| `0x80C30240` | `FUN_80c30240` | `fn_80c30240` | fly-boss B entry | translated | high | `80c30240_FUN_80c30240.c` |
| `0x80C316D0` | `FUN_80c316d0` | `fn_80c316d0` | per-frame state handler | translated | high | `80c316d0_FUN_80c316d0.c` |
| `0x80C33880` | `FUN_80c33880` | `fn_80c33880` | projection group B | translated | high | `80c33880_FUN_80c33880.c` |
| `0x80C33A90` | `FUN_80c33a90` | `fn_80c33a90` | matrix rebuild | translated | high | `80c33a90_FUN_80c33a90.c` |
| `0x80C35990` | `FUN_80c35990` | `fn_80c35990` | clip selector | translated | high | `80c35990_FUN_80c35990.c` |
| `0x80C38900` | `FUN_80c38900` | `fn_80c38900` | turn toward partner | translated | high | `80c38900_FUN_80c38900.c` |
| `0x80C390B0` | `FUN_80c390b0` | `fn_80c390b0` | two-name query | translated | high | `80c390b0_FUN_80c390b0.c` |
| `0x80C39410` | `FUN_80c39410` | `fn_80c39410` | tier multiplier | translated | high | `80c39410_FUN_80c39410.c` |
| `0x80C399B0` | `FUN_80c399b0` | `fn_80c399b0` | side-derived impulse | translated | high | `80c399b0_FUN_80c399b0.c` |
| `0x80C3D5A0` | `FUN_80c3d5a0` | `fn_80c3d5a0` | second `0x1118` ctor | translated | high | `80c3d5a0_FUN_80c3d5a0.c` |
| `0x80C47340` | `FUN_80c47340` | `fn_80c47340` | four-slot damage relay | translated | high | `80c47340_FUN_80c47340.c` |
| `0x80C473F0` | `FUN_80c473f0` | `fn_80c473f0` | relay release | translated | medium | `80c473f0_FUN_80c473f0.c` |
| `0x80C53C60` | `FUN_80c53c60` | `fn_80c53c60` | interval/position update | translated | medium | `80c53c60_FUN_80c53c60.c` |
| `0x80C55DB0` | `FUN_80c55db0` | `fn_80c55db0` | effect/interval object ctor | translated | medium | `80c55db0_FUN_80c55db0.c` |

## Data words read from the analysis image

These values are quoted so the constants in the Rust bodies are auditable.
They are analysis-image words, not runtime addresses.

| address | value | used by |
|---|---|---|
| `0x80C57F34` | `1.25` | `fn_80c2e5c0` |
| `0x80C57F58` | `5.0` | `fn_80c2e5c0` |
| `0x80C57F78` | `120.0` | `fn_80c2e5c0` |
| `0x80C57F7C` | `112.0` | `fn_80c2e5c0` |
| `0x80C57F80` | `16.0` | `fn_80c2e5c0` |
| `0x80C57F84` | `1.0` | `fn_80c2e5c0` |
| `0x80C5806C` | `1.25` | `fn_80c30240` |
| `0x80C58090` | `120.0` | `fn_80c30240` |
| `0x80C58094` | `112.0` | `fn_80c30240` |
| `0x80C58098` | `1.0` | `fn_80c30240` |
| `0x80C58378` | `8.0` | `fn_80c35990` |
| `0x80C58380` | `28.0` | `fn_80c35990` |
| `0x80C58384` | `47.0` | `fn_80c35990` |
| `0x80C58388` | `21.0` | `fn_80c35990` |
| `0x80C5838C` | `40.0` | `fn_80c35990` |
| `0x80C58390` | `60.0` | `fn_80c35990` |
| `0x80C585EC` | `1.1` | `fn_80c39410` |
| `0x80C585F0` | `1.25` | `fn_80c39410` |
| `0x80C585F4` | `1.5` | `fn_80c39410` |
| `0x80C5859C` | `0.0` | `fn_80c399b0` |
| `0x80C662B0` | `2.0` | `fn_80c399b0` |
| `0x80C662B4` | `2.0` | `fn_80c399b0` |
| `0x80C662B8` | `2.5` | `fn_80c399b0` |
| `0x80C662C0` | `0.3` | `fn_80c399b0` |
| `0x80C662D0` | signed halfword `0x4000` (16384) | `fn_80c38900` |
| `0x80C662D2` | signed halfword `0x1000` (4096) | `fn_80c38900` |
| `0x80C698EC` | signed halfword `0x0096` (150) | `fn_80c47340` |
| `0x80C5932C` | `0.0` | `fn_80c55db0` |
| `0x80C59330` | `9999.0` bits | `fn_80c55db0` |
| `0x80C59334` | `0.1` | `fn_80c55db0` |
| `0x80C59338` | `3.0` | `fn_80c55db0` |
| `0x80C5933C` | `1.0` | `fn_80c55db0` |
| `0x80C59340` | `10.0` | `fn_80c55db0` |
| `0x80C57F50` | `8.0` | `fn_80c2edc0` |
| `0x80C57FC4` | `5500.0` | `fn_80c2edc0` |
| `0x80C57FC8` | `100.0` | `fn_80c2edc0` |
| `0x80C5821C` | `8.0` | `fn_80c33880` |
| `0x80C58220` | `5500.0` | `fn_80c33880` |
| `0x80C58224` | `100.0` | `fn_80c33880` |

## Unknowns

- **Dropped arguments.** `0x80C21770` calls `0x802D7AA0` with no printed
  argument; the translation passes `this`, following the sibling
  `0x80C04C80`. `0x80C215C0` drops the argument to `0x800983C0`; the callee
  takes the actor in `r3`, so the translation passes `this`.
  `0x80C21840` drops the argument to `0x8016D4B0`; the translation passes
  `this + 0x18`. `0x80C473F0` drops the player argument to `0x80137650`; the
  translation passes the player pointer recovered from `r3`.
- **Untranslated callees.** `0x80C00630` (element destructor),
  `0x80C508A0`/`0x80C508F0` (interval helpers), `0x80C2E8D0`, `0x80C2E980`,
  `0x80C2EAC0`, `0x80C30560`, `0x80C30760`, `0x80C30820` and `0x80C56240`
  are still called by raw address.
- **Return-only callback.** The word at `0x80C04C70` is `0x4E800020`, a bare
  `blr`. It is used as the first controller callback in `0x80C55DB0`.
- **Matrix helper semantics.** `0x80168180`, `0x8016F130` and `0x80170980`
  are called with the same argument order as the export. The translation
  observes three 16-byte vectors in the temporary buffer but does not name the
  matrix operations.
- **Global object.** The object pointed to by `0x8042A1A0` is read for `+0xB4`
  and `+0xC0` in `fn_80c53c60`; its role is not proven.
- **Vtable stores.** The `0x8032...` and `0x80C6...` words written by the
  constructors are BSS/data addresses in the analysis image. Their class roles
  are inferred from the constructor families, not from a symbol table.
- **`+0x7AC` gate.** `fn_80c33a90` and the two fly-boss entries treat `+0x7AC`
  as a "model ready" flag. The translation keeps it as a raw gate; its writer
  is outside this pass.

## Verification

- `cargo check` passes (exit 0); no warning points at `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave15-bosses.csv` match the Rust functions
  one to one: 25 `translated`; 20 `high`, 5 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-15 ledger
  were edited.




