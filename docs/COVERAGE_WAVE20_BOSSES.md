# Coverage - Wave 20, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-20
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions),
  `docs/COVERAGE_WAVE14_BOSSES.md` (25), `docs/COVERAGE_WAVE15_BOSSES.md` (25),
  `docs/COVERAGE_WAVE16_BOSSES.md` (25), `docs/COVERAGE_WAVE17_BOSSES.md` (25)
  and `docs/COVERAGE_WAVE19_BOSSES.md` (25). This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (for example `0x802DCD70`) are absolute DOL
  addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` and `stub` are
  not used in this pass.
- **Confidence** rates the translation, not the guessed role. `high` means the
  stores, call arguments and branch tree are exact; `medium` means the body is
  exact but a register-pair split, an object role or a helper role is
  reconstructed from convention or from a sibling.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 4,900 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 9 |
| confidence `medium` | 16 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 341 |
| cumulative bytes in `game::bosses` | 106,388 |

The pass finishes the `0x1118` effect rig, translates the two 64-slot
proximity scans and the nearest-player selector, and adds the missing
`0x780`/`0xE98` constructors, the two controller destructors, the `0x129`/`299`
ring spawner, the phase selector and the shared position helper that two
already-translated functions called by raw address.

## Synthetic-base warning in one place

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The globals at
`0x80C71158..0x80C71178` and the staging word `0x80C6C81C` are analysis-image
words, not hardware addresses.

## Cluster 1 - the `0x1118` effect rig (1,692 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C06980` | 224 | `fn_80c06980` | `+0x30` chain resolver | translated | high |
| `0x80C06A60` | 136 | `fn_80c06a60` | `:`-suffix name compare | translated | medium |
| `0x80C0A910` | 504 | `fn_80c0a910` | `0x188`/`0x189` chain spawner | translated | medium |
| `0x80C0AB10` | 320 | `fn_80c0ab10` | `3 x 8` tile probe and one-shot sound | translated | medium |
| `0x80C0ADD0` | 508 | `fn_80c0add0` | phase-2 chain update | translated | medium |

What is proven:

- `0x80C06980` performs exactly five `chain_probe` steps (`*(node + 0x30)`
  through vtable slot `+0x0C`) and then tail-calls itself with the next link.
  It is the last untranslated member of the wave-3 chain-resolver family.
- `0x80C06A60` cuts both strings at `0x3A`, always advances the object-name
  side past the delimiter, and compares the remainders with `0x802E1AB4`.
- `0x80C0A910` appends `+0x114C`, `+0x115C` and `+0x116C` to the list at
  `+0x1188`, writes a `0x188` effect to `+0x0C` of every listed node on a
  `256.0`-step line from `+0x11AC`, spawns the `0x189` tip at `+0x11A4`,
  unlinks the `+0x1150`/`+0x1154` pair and appends `+0x1198`.
- `0x80C0AB10` walks the three holders at `+0x1140..+0x1148`; each row probes
  eight columns from `x - 64.0` with a `16.0` step and `y + 16.0`, and the
  first hit sets `+0x11B8 = 1` and plays `0x58A`.
- `0x80C0ADD0` gates on the `0x802DCD6C` mode word `2`, matches the three
  `+0x394` tags `0x80C6D4A4`/`0x80C6D2E8`/`0x80C6D328`, drops `+0x11AC` by
  `+0x11B4`, swaps the two holder slots below `+0x11B0`, and rewrites the
  `+0xAC/+0xB0/+0xB4` position of every chained effect.

## Cluster 2 - proximity scans and target selection (876 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C1D430` | 276 | `fn_80c1d430` | X-band rejection scan | translated | medium |
| `0x80C1D550` | 404 | `fn_80c1d550` | 2-D box rejection scan | translated | medium |
| `0x80C23CD0` | 196 | `fn_80c23cd0` | greatest-`+X` player selector | translated | medium |

What is proven:

- Both scans test exactly 64 slots at `actor + 0xE80 + index * 4`, resolve the
  `+0x394` list through vtable slot `+0x28`, and test the class tag with slot
  `+0x14` (`0x8082F7C8` and `0x8082F788`). They return `0` on a match through
  `0x802DCDBC` and `1` after the last slot.
- `0x80C1D430` compares the slot actor's `+0xAC` against the point in a
  `16.0 * 0x807100C0(actor)` band.
- `0x80C1D550` adds the four zero offsets from `0x80C601BC..0x80C601C8` and
  checks both X and Y, using `0x807100C0` for the width and `0x807100F0` for
  the height.
- `0x80C23CD0` gates four player slots with the mask at `0x80429CA8`, keeps
  the greatest `player.x - actor.x` that is at least `0.0`, and publishes the
  winner through `0x802DCDC0`. The in-crate caller `fn_80c21fe0` supplies the
  actor.

## Cluster 3 - constructors, teardown and dispatch (796 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C2A390` | 204 | `fn_80c2a390` | `0x780` actor constructor | translated | medium |
| `0x80C2A460` | 60 | `fn_80c2a460` | `0x38` element constructor | translated | medium |
| `0x80C2E550` | 112 | `fn_80c2e550` | `0xC60` controller destructor | translated | high |
| `0x80C2EF60` | 116 | `fn_80c2ef60` | partner point forward | translated | medium |
| `0x80C300E0` | 100 | `fn_80c300e0` | `0x38C == 1` / `0x4F` gate | translated | medium |
| `0x80C301D0` | 112 | `fn_80c301d0` | `0xC3C` controller destructor | translated | high |
| `0x80C34CD0` | 92 | `fn_80c34cd0` | `0xE98` actor constructor | translated | high |

What is proven:

- `0x80C2A390` allocates `0x780`, installs vtable `0x80C62F70` at `+0x60`,
  constructs `+0x524`, clears `+0x544`/`+0x548`, and registers a `0x40`-stride
  array at `+0x54C` and a `0x38`-stride array at `+0x60C` through
  `0x802DC9A0`.
- `0x80C2A460` is the element constructor passed to the second array; it
  installs vtable `0x80C60960`.
- `0x80C2E550` and `0x80C301D0` are byte-for-byte siblings with different
  controller offsets (`+0xC60`, `+0xC3C`). Both release the controller with
  `0x802D7B20(ptr, 0)`, run `0x8009AF40(this, 0)` and free only for a positive
  `free_it`.
- `0x80C2EF60` forwards the three-float point to slot `+0x128` of the
  `0x80162D00` partner, and `0x80C300E0` calls `0x80095950` unless the owner
  byte `+0x38C` is `1` and `0x80056CF0(owner, 0x4F)` succeeds.
- `0x80C34CD0` allocates `0xE98`, installs vtable `0x80C65534` at `+0x60`,
  `0x80C65B68` at `+0xE74` and the self pointer at `+0xE78`.

## Cluster 4 - per-frame, phase and position helpers (1,536 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C12070` | 96 | `fn_80c12070` | timer tick and Morton release | translated | high |
| `0x80C123E0` | 60 | `fn_80c123e0` | `0x52C` sound and base release | translated | medium |
| `0x80C298F0` | 220 | `fn_80c298f0` | matrix rebuild from position/angles | translated | medium |
| `0x80C30560` | 512 | `fn_80c30560` | `0x129` guard and `299` ring spawn | translated | medium |
| `0x80C34220` | 96 | `fn_80c34220` | action-runner dispatch | translated | high |
| `0x80C34360` | 116 | `fn_80c34360` | render-scale copy and reset | translated | medium |
| `0x80C508A0` | 80 | `fn_80c508a0` | flash-block clear | translated | high |
| `0x80C536F0` | 192 | `fn_80c536f0` | interpolated position | translated | medium |
| `0x80C53EC0` | 120 | `fn_80c53ec0` | phase `4`/`5`/`6` selector | translated | high |
| `0x80C566F0` | 44 | `fn_80c566f0` | shared position rebuild | translated | high |

What is proven:

- `0x80C12070` ticks slot `+0x10` of the `+0x394` controller, decrements
  `+0xE94`, and calls `0x80C473F0` when it reaches zero.
- `0x80C123E0` plays sound `0x52C` on `+0x544` and calls `0x800A9EC0(actor)`.
- `0x80C298F0` copies `+0xAC..+0xB4` and the three `+0x100..+0x104` angles,
  runs `0x80064AF0`, builds the `+0x7C` matrix with `0x801C0BD0`, applies the
  three angles, and publishes to `+0x548`.
- `0x80C30560` spawns the `0x129` guard with `0x80064680` at `+0x7A8`, sets
  its `+0x738` owner and `+0xB0` height, then spawns `299` effects with
  `0x80064610`. The effect X is `+0x788 - 112.0 + step * (i + 1)` with
  `step = 224 / (1 + count)`; both heights are `+0x7A0 + 0.5 * slot(+0x314)`.
- `0x80C34220` updates model slot `+0x1C` and dispatches to `fn_80c32800`
  or `fn_80c331c0` on `0x80663290(actor) == 0`.
- `0x80C34360` copies the `f32` from class slot `+0x418` into `+0xDC/+0xE0/+0xE4`,
  applies it through slot `+0x424`, and zeroes `+0x770..+0x778`.
- `0x80C508A0` writes `0x80C59298` (`0.0`) to words `0..5` and `9` and clears
  byte `+0x29`; the three existing call sites now call `fn_80c508a0`.
- `0x80C536F0` and `0x80C566F0` are the pair that `fn_80c55220` used to call by
  address; the final Z offset is `c.z - b.z`.
- `0x80C53EC0` returns `4` for scene mode `2`, `5` for mode `3`, and `6`
  otherwise, skipping the guarded result when `0x80C6C81C` already matches.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C06980` | `FUN_80c06980` | `fn_80c06980` | `+0x30` chain resolver | translated | high | `80c06980_FUN_80c06980.c` |
| `0x80C06A60` | `FUN_80c06a60` | `fn_80c06a60` | `:`-suffix name compare | translated | medium | `80c06a60_FUN_80c06a60.c` |
| `0x80C0A910` | `FUN_80c0a910` | `fn_80c0a910` | `0x188`/`0x189` chain spawner | translated | medium | `80c0a910_FUN_80c0a910.c` |
| `0x80C0AB10` | `FUN_80c0ab10` | `fn_80c0ab10` | `3 x 8` tile probe | translated | medium | `80c0ab10_FUN_80c0ab10.c` |
| `0x80C0ADD0` | `FUN_80c0add0` | `fn_80c0add0` | phase-2 chain update | translated | medium | `80c0add0_FUN_80c0add0.c` |
| `0x80C12070` | `FUN_80c12070` | `fn_80c12070` | timer tick and release | translated | high | `80c12070_FUN_80c12070.c` |
| `0x80C123E0` | `FUN_80c123e0` | `fn_80c123e0` | `0x52C` sound and release | translated | medium | `80c123e0_FUN_80c123e0.c` |
| `0x80C1D430` | `FUN_80c1d430` | `fn_80c1d430` | X-band rejection scan | translated | medium | `80c1d430_FUN_80c1d430.c` |
| `0x80C1D550` | `FUN_80c1d550` | `fn_80c1d550` | 2-D box rejection scan | translated | medium | `80c1d550_FUN_80c1d550.c` |
| `0x80C23CD0` | `FUN_80c23cd0` | `fn_80c23cd0` | greatest-`+X` player selector | translated | medium | `80c23cd0_FUN_80c23cd0.c` |
| `0x80C298F0` | `FUN_80c298f0` | `fn_80c298f0` | matrix rebuild | translated | medium | `80c298f0_FUN_80c298f0.c` |
| `0x80C2A390` | `FUN_80c2a390` | `fn_80c2a390` | `0x780` constructor | translated | medium | `80c2a390_FUN_80c2a390.c` |
| `0x80C2A460` | `FUN_80c2a460` | `fn_80c2a460` | `0x38` element constructor | translated | medium | `80c2a460_FUN_80c2a460.c` |
| `0x80C2E550` | `FUN_80c2e550` | `fn_80c2e550` | `0xC60` destructor | translated | high | `80c2e550_FUN_80c2e550.c` |
| `0x80C2EF60` | `FUN_80c2ef60` | `fn_80c2ef60` | partner point forward | translated | medium | `80c2ef60_FUN_80c2ef60.c` |
| `0x80C300E0` | `FUN_80c300e0` | `fn_80c300e0` | `0x38C == 1` gate | translated | medium | `80c300e0_FUN_80c300e0.c` |
| `0x80C301D0` | `FUN_80c301d0` | `fn_80c301d0` | `0xC3C` destructor | translated | high | `80c301d0_FUN_80c301d0.c` |
| `0x80C30560` | `FUN_80c30560` | `fn_80c30560` | guard and ring spawn | translated | medium | `80c30560_FUN_80c30560.c` |
| `0x80C34220` | `FUN_80c34220` | `fn_80c34220` | action-runner dispatch | translated | high | `80c34220_FUN_80c34220.c` |
| `0x80C34360` | `FUN_80c34360` | `fn_80c34360` | render-scale copy | translated | medium | `80c34360_FUN_80c34360.c` |
| `0x80C34CD0` | `FUN_80c34cd0` | `fn_80c34cd0` | `0xE98` constructor | translated | high | `80c34cd0_FUN_80c34cd0.c` |
| `0x80C508A0` | `FUN_80c508a0` | `fn_80c508a0` | flash-block clear | translated | high | `80c508a0_FUN_80c508a0.c` |
| `0x80C536F0` | `FUN_80c536f0` | `fn_80c536f0` | interpolated position | translated | medium | `80c536f0_FUN_80c536f0.c` |
| `0x80C53EC0` | `FUN_80c53ec0` | `fn_80c53ec0` | phase selector | translated | high | `80c53ec0_FUN_80c53ec0.c` |
| `0x80C566F0` | `FUN_80c566f0` | `fn_80c566f0` | shared position rebuild | translated | high | `80c566f0_FUN_80c566f0.c` |

## Data words read from the analysis image

These values are quoted so the constants in the Rust bodies are auditable.
They are analysis-image words, not runtime addresses.

| address | value | used by |
|---|---|---|
| `0x80C57474` | `0.0` | `fn_80c0a910`, `fn_80c0add0` |
| `0x80C57478` | `256.0` | `fn_80c0a910`, `fn_80c0add0` |
| `0x80C57480` | `16.0` | `fn_80c0ab10` |
| `0x80C5748C` | `-100.0` | `fn_80c0a910` |
| `0x80C57490` | double `2^52 + 2^31` | `fn_80c0a910`, `fn_80c0add0` |
| `0x80C57498` | `64.0` | `fn_80c0ab10` |
| `0x80C574A0` | `-3500.0` | `fn_80c0add0` |
| `0x80C57BFC` | `16.0` | `fn_80c1d430`, `fn_80c1d550` |
| `0x80C57C00` | double `2^52 + 2^31` | `fn_80c1d430` |
| `0x80C57CE8` | `0.0` | `fn_80c23cd0` |
| `0x80C5807C` | `14.0` | `fn_80c30560` |
| `0x80C58080` | `0.0` | `fn_80c30560` |
| `0x80C58098` | `1.0` | `fn_80c30560` |
| `0x80C5809C` | `3064.0` | `fn_80c30560` |
| `0x80C580A0` | `16.0` | `fn_80c30560` |
| `0x80C580A4` | `0.5` | `fn_80c30560` |
| `0x80C580A8` | double `2^52 + 2^31` | `fn_80c30560` |
| `0x80C581D8` | `0.0` | `fn_80c34360` |
| `0x80C59298` | `0.0` | `fn_80c508a0` |
| `0x80C601BC..0x80C601C8` | `0.0` (four words) | `fn_80c1d550` |
| `0x80C6C81C` | staging word (`1` in this image) | `fn_80c53ec0` |

## Unknowns

- **Save-thunk register pairs.** `0x802DCD6C`, `0x802DCD70` and `0x802DCD74`
  are prologue save/restore helpers. The export models their return as a
  `ulonglong`; `fn_80c0a910` and `fn_80c0add0` keep the pair split printed by
  the C, which is a register-spill artifact, not proven ABI. The functions with
  in-crate callers (`fn_80c0ab10`, `fn_80c1d430`, `fn_80c1d550`,
  `fn_80c23cd0`) take the actor and point explicitly from those callers.
- **List helper.** The role of `0x802BE800(head, node)` is inferred from the
  three appends and the final unlink in `fn_80c0a910`; it is not proven to be
  an insert or a splice.
- **Tile probe.** `0x80C0AC50` truncates both coordinates to integers and calls
  `0x800775A0`; the tile-size/block semantics are inferred.
- **Paired-single staging.** `fn_80c0a910`, `fn_80c0ab10`, `fn_80c0add0` and
  `fn_80c30560` contain `unaff_GQR0`/`ldexpf` blocks in the export. These are
  compiler spills and are not modelled; the float math after them is.
- **Dropped receivers.** `0x80168D10` (`fn_80c2a460`) and `0x80094E80`
  (`fn_80c2a390`) are printed without arguments; the translations pass the
  object following the sibling constructors in `game::bases`.
- **Opaque ids.** `0x80056CF0`, `0x8009AF40`, `0x800A8D90`, `0x800A9EC0`,
  `0x80095950`, `0x80162D00`, `0x80197BF0`, `0x806F53C0` and `0x806F53E0` are
  numeric callees with inferred roles only.
- **Array destructor.** The element destructor `0x80C1C840` passed to the
  second `0x802DC9A0` array in `fn_80c2a390` is still untranslated and is kept
  by address.
- **Scene tag.** `0x8082F7C8` and `0x8082F788` are identity tags only; the
  classes they name are not recovered.
- **Uncalled translation.** `fn_80c1d550` has no caller in the currently
  translated set; its signature follows the sibling `fn_80c1d430`.
- **No runtime addresses.** Recovering the real REL base remains the job of the
  `OSLink` loader lane, not this module.

## Verification

- `cargo check` exits 0 and reports no diagnostics for `src/game/bosses.rs`
  (the remaining crate warnings belong to other lanes).
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave20-bosses.csv` match the Rust functions one
  to one: 25 `translated`; 9 `high`, 16 `medium`.
- Existing raw-address call sites for `0x80C0AB10`, `0x80C1D430`,
  `0x80C23CD0`, `0x80C508A0` and `0x80C566F0` now call the translated
  functions.
- No files outside `src/game/bosses.rs`, this document and the wave-20 ledger
  were edited.
