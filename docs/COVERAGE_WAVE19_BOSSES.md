# Coverage - Wave 19, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-19
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions),
  `docs/COVERAGE_WAVE14_BOSSES.md` (25), `docs/COVERAGE_WAVE15_BOSSES.md` (25),
  `docs/COVERAGE_WAVE16_BOSSES.md` (25) and `docs/COVERAGE_WAVE17_BOSSES.md`
  (25). This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x8005xxxx` through `0x8042xxxx`) are
  absolute DOL addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every table
  row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` is not used in
  this pass.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores, call arguments and branch tree are exact; `medium` means the
  body is exact but an argument dropped by the export, an object role or a
  helper role is reconstructed by convention or from a sibling.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 7,540 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 13 |
| confidence `medium` | 12 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 316 |
| cumulative bytes in `game::bosses` | 101,488 |

The pass takes the remaining mid-size activity: object teardown, the nearest
player selector, the landing/impact state machine, the scene-gated attack cue
selector, several clip entries, the effect-slot publisher and the two vector
helpers that the collision and heading code call.

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

## Cluster 1 - teardown and collision probes (1,436 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C00630` | 120 | `fn_80c00630` | state-object teardown | translated | medium |
| `0x80C1D350` | 224 | `fn_80c1d350` | proximity rejection scan | translated | high |
| `0x80C3E560` | 108 | `fn_80c3e560` | `damage_s` clip entry | translated | high |
| `0x80C475C0` | 108 | `fn_80c475c0` | Wendy `damage_s` clip entry | translated | high |
| `0x80C04EF0` | 248 | `fn_80c04ef0` | two-slot ground probe | translated | medium |
| `0x80C52730` | 628 | `fn_80c52730` | X-axis contact update | translated | medium |

What is proven:

- `0x80C00630` writes vtable `0x80329958` at `+0`, calls `0x8016D4B0`,
  conditionally `0x802D7B20(this, 0)` and, when `free_it > 0`,
  `0x802B9280(this)`.
- `0x80C1D350` rejects a point when a listed actor is within `96.0`
  (`0x80C601B0`) on X and lower than the point after `0x8006CED0`; exhaustion
  returns `1`.
- `0x80C3E560` and `0x80C475C0` have identical shape: `0x8023A190` resolves
  `damage_s`, `0x80165FD0` installs it at `+0x67C`, and animation slot `+0x18`
  is called on `+0x604`. The only difference is the string instance
  (`0x80C67644` versus `0x80C69980`).
- `0x80C04EF0` probes two `+0x10E8` slot positions through `0x800757B0` with
  the `1500.0` height from `0x80C572F8`, writes the result back to `+0x10EC`,
  and advances each `+0xEC0` owner through slot `+0xB0`.
- `0x80C52730` is the X-axis twin of the already-translated `0x80C54050`
  shape: it copies both points, asks class slot `+0x24` for acceptance, builds
  the `+0xD0..0xD8` contact vector, measures it with `0x801C14C0`, and adds it
  to `+0x10C..0x114`.

## Cluster 2 - phase, attack and aim handlers (4,332 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C00B30` | 544 | `fn_80c00b30` | nearest player and side gate | translated | high |
| `0x80C0C6F0` | 720 | `fn_80c0c6f0` | landing/impact state machine | translated | medium |
| `0x80C4F4B0` | 612 | `fn_80c4f4b0` | scene cue selector | translated | medium |
| `0x80C4FC70` | 360 | `fn_80c4fc70` | input-gated action selector | translated | medium |
| `0x80C53860` | 1,024 | `fn_80c53860` | scene phase transition | translated | medium |
| `0x80C484C0` | 556 | `fn_80c484c0` | side heading and lean | translated | medium |
| `0x80C56390` | 516 | `fn_80c56390` | projected height ratio | translated | medium |

What is proven:

- `0x80C00B30` compares players 1..3 with player 0 using squared 3-D distance,
  stores the winner at `+0x764`, then gates the result with the `-16.0`
  (`0x80C57220`), `0.0` (`0x80C571F8`) and `64.0` (`0x80C57224`) side tests.
  The two `+0x6DC` modes select opposite comparisons.
- `0x80C0C6F0` runs states `1..3`: descent and `0.98` acceleration down to
  `1.5`; hand-off of the remaining height to `+0x11A8`; and the impact drain
  with sound `0x58B` and class notification `0x80C6D328`.
- `0x80C4F4B0` clamps the studio byte `0x80315B9C` to `9`, marks one of
  `+0x1AC`/`+0x1A8`, and maps the event byte `0x80315B9D` through the cue
  switch (`0x14..0x28`). Unhandled events store `event + 1` at `+0x3B8`.
- `0x80C4FC70` requires a normal in-level state (`0x80315B98 == 0`), scene id
  `0x2EE`, both `0x80429F80` mode bits clear, and the `')'` exception absent
  before scanning four controller slots for `0x80371870`.
- `0x80C53860` copies the player position twice, stages `4`, discovers the
  phase through the `4`/`8`/`0x10`/`0x20` masks, and stages `5` or `6` on the
  two fallback paths.
- `0x80C484C0` reads the side with `0x80096240`, samples slots `+0x624` and
  `+0x620`, applies the `0x8042C1A0` sign and the `0x80C58D88` bias, then
  reduces both angles in `0.1` steps.
- `0x80C56390` selects the scene pair from `0x8042A1A0`, measures both pairs
  with `0x801C1650` plus `0x802BE470`, and returns either the manager height
  delta or `distance_b * (b.y - a.y) / distance_a`.

## Cluster 3 - clips, effects and small helpers (1,772 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C34280` | 116 | `fn_80c34280` | `clown_wait` clip | translated | high |
| `0x80C3F840` | 140 | `fn_80c3f840` | `jump_HB3` clip | translated | high |
| `0x80C3FF40` | 120 | `fn_80c3ff40` | armed jump-effect clip | translated | high |
| `0x80C4BBF0` | 136 | `fn_80c4bbf0` | `jumpB` clip | translated | high |
| `0x80C4F3A0` | 264 | `fn_80c4f3a0` | effect-slot publisher | translated | high |
| `0x80C508F0` | 108 | `fn_80c508f0` | flash block init | translated | high |
| `0x80C53F90` | 192 | `fn_80c53f90` | manager-delta follow | translated | high |
| `0x80C55220` | 192 | `fn_80c55220` | interpolated position | translated | medium |
| `0x80C30A00` | 176 | `fn_80c30a00` | type-`0x186` notify | translated | high |
| `0x80C30AB0` | 92 | `fn_80c30ab0` | type-`0x186` cleanup | translated | high |
| `0x80C4A750` | 136 | `fn_80c4a750` | suffix name compare | translated | medium |
| `0x80C43900` | 100 | `fn_80c43900` | per-player radius hook | translated | medium |

What is proven:

- All four clip entries use the same `0x8023Axxx -> 0x8016xxxx` shape. The
  strings are `clown_wait` (`0x80C649E8`), `jump_HB3` (`0x80C67670`) and
  `jumpB` (`0x80C6ABB8`); `0x80C3FF40` passes no name and installs the default
  clip with the `1` flag.
- `0x80C4F3A0` reads the three four-word tables `0x80C59108`, `0x80C59118`
  and `0x80C59128`, resolves a model handle through `0x80354E60` and
  `0x80354E90`, and publishes four `0x800B3B60` handles; key zero adds
  `+0x204` and `+0x208`.
- `0x80C508F0` is the guarded `+0x29` initialiser used by `0x80C53860`; it
  zeroes words `0..5` and `9`, then stores the four parameters at
  `+0x2C..+0x38`.
- `0x80C53F90` adds the manager `+0xAC - +0xB8` delta to the actor X and the
  two zero-biased components, then updates `0x80C71170..0x80C71178`.
- `0x80C55220` copies `source + 0x24..0x2C`, overwrites X with `base`, runs
  the untranslated `0x80C566F0` helper on the zero-blend path, and adds
  `end_x - start_x` last.
- `0x80C30A00` and `0x80C30AB0` are the two `0x186` list walks: the first
  notifies with `0x8082F150`, the second calls `0x80700260`.
- `0x80C4A750` repeats the `fn_80c04a10` suffix comparison with the vtable at
  `*object`; `0x80C43900` is the four-player `+0xE80` radius hook.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C00630` | `FUN_80c00630` | `fn_80c00630` | state-object teardown | translated | medium | `80c00630_FUN_80c00630.c` |
| `0x80C00B30` | `FUN_80c00b30` | `fn_80c00b30` | nearest player and side gate | translated | high | `80c00b30_FUN_80c00b30.c` |
| `0x80C04EF0` | `FUN_80c04ef0` | `fn_80c04ef0` | two-slot ground probe | translated | medium | `80c04ef0_FUN_80c04ef0.c` |
| `0x80C0C6F0` | `FUN_80c0c6f0` | `fn_80c0c6f0` | landing/impact state machine | translated | medium | `80c0c6f0_FUN_80c0c6f0.c` |
| `0x80C1D350` | `FUN_80c1d350` | `fn_80c1d350` | proximity rejection scan | translated | high | `80c1d350_FUN_80c1d350.c` |
| `0x80C30A00` | `FUN_80c30a00` | `fn_80c30a00` | type-`0x186` notify | translated | high | `80c30a00_FUN_80c30a00.c` |
| `0x80C30AB0` | `FUN_80c30ab0` | `fn_80c30ab0` | type-`0x186` cleanup | translated | high | `80c30ab0_FUN_80c30ab0.c` |
| `0x80C34280` | `FUN_80c34280` | `fn_80c34280` | `clown_wait` clip | translated | high | `80c34280_FUN_80c34280.c` |
| `0x80C3E560` | `FUN_80c3e560` | `fn_80c3e560` | `damage_s` clip entry | translated | high | `80c3e560_FUN_80c3e560.c` |
| `0x80C3F840` | `FUN_80c3f840` | `fn_80c3f840` | `jump_HB3` clip | translated | high | `80c3f840_FUN_80c3f840.c` |
| `0x80C3FF40` | `FUN_80c3ff40` | `fn_80c3ff40` | armed jump-effect clip | translated | high | `80c3ff40_FUN_80c3ff40.c` |
| `0x80C43900` | `FUN_80c43900` | `fn_80c43900` | per-player radius hook | translated | medium | `80c43900_FUN_80c43900.c` |
| `0x80C475C0` | `FUN_80c475c0` | `fn_80c475c0` | Wendy `damage_s` clip entry | translated | high | `80c475c0_FUN_80c475c0.c` |
| `0x80C484C0` | `FUN_80c484c0` | `fn_80c484c0` | side heading and lean | translated | medium | `80c484c0_FUN_80c484c0.c` |
| `0x80C4A750` | `FUN_80c4a750` | `fn_80c4a750` | suffix name compare | translated | medium | `80c4a750_FUN_80c4a750.c` |
| `0x80C4BBF0` | `FUN_80c4bbf0` | `fn_80c4bbf0` | `jumpB` clip | translated | high | `80c4bbf0_FUN_80c4bbf0.c` |
| `0x80C4F3A0` | `FUN_80c4f3a0` | `fn_80c4f3a0` | effect-slot publisher | translated | high | `80c4f3a0_FUN_80c4f3a0.c` |
| `0x80C4F4B0` | `FUN_80c4f4b0` | `fn_80c4f4b0` | scene cue selector | translated | medium | `80c4f4b0_FUN_80c4f4b0.c` |
| `0x80C4FC70` | `FUN_80c4fc70` | `fn_80c4fc70` | input-gated action selector | translated | medium | `80c4fc70_FUN_80c4fc70.c` |
| `0x80C508F0` | `FUN_80c508f0` | `fn_80c508f0` | flash block init | translated | high | `80c508f0_FUN_80c508f0.c` |
| `0x80C52730` | `FUN_80c52730` | `fn_80c52730` | X-axis contact update | translated | medium | `80c52730_FUN_80c52730.c` |
| `0x80C53860` | `FUN_80c53860` | `fn_80c53860` | scene phase transition | translated | medium | `80c53860_FUN_80c53860.c` |
| `0x80C53F90` | `FUN_80c53f90` | `fn_80c53f90` | manager-delta follow | translated | high | `80c53f90_FUN_80c53f90.c` |
| `0x80C55220` | `FUN_80c55220` | `fn_80c55220` | interpolated position | translated | medium | `80c55220_FUN_80c55220.c` |
| `0x80C56390` | `FUN_80c56390` | `fn_80c56390` | projected height ratio | translated | medium | `80c56390_FUN_80c56390.c` |

## Data words read from the analysis image

These values are quoted so the constants in the Rust bodies are auditable.
They are analysis-image words, not runtime addresses.

| address | value | used by |
|---|---|---|
| `0x80C571F8` | `0.0` | `fn_80c00b30` |
| `0x80C57220` / `0x80C57224` | `-16.0` / `64.0` | `fn_80c00b30` |
| `0x80C572F8` | `1500.0` | `fn_80c04ef0` |
| `0x80C57474` / `0x80C57478` | `0.0` / `256.0` | `fn_80c0c6f0`, `fn_80c53f90` |
| `0x80C574D8` / `0x80C574DC` | `11.0` / `1000.0` | `fn_80c0c6f0` |
| `0x80C57498` | `64.0` | `fn_80c0c6f0` |
| `0x80C5B8C0` / `0x80C5B8C4` | `0.98` / `1.5` | `fn_80c0c6f0` |
| `0x80C58D34` | `0.0` | `fn_80c484c0` |
| `0x80C58D84` | `0.1` | `fn_80c484c0` |
| `0x80C58D88` | double bias `2^52 + 2^31` | `fn_80c484c0` |
| `0x80C592B8` | `0.0` | `fn_80c52730`, `fn_80c53f90`, `fn_80c53860` |
| `0x80C592D8` / `0x80C592DC` | `130.0` / `-100.0` | `fn_80c53860` |
| `0x80C592E0..0x80C592EC` | `0.2`, `15.0`, `1.0`, `0.05` | `fn_80c53860` |
| `0x80C5932C` / `0x80C59330` | `0.0` / `9999.0` | `fn_80c56390` |
| `0x80C601B0` | `96.0` | `fn_80c1d350` |
| `0x80C649E8` | `"clown_wait"` | `fn_80c34280` |
| `0x80C67644` / `0x80C69980` | `"damage_s"` | `fn_80c3e560`, `fn_80c475c0` |
| `0x80C67670` | `"jump_HB3"` | `fn_80c3f840` |
| `0x80C6ABB8` | `"jumpB"` | `fn_80c4bbf0` |
| `0x80C59108` / `0x80C59118` / `0x80C59128` | `0,1,3,2` / `5,6,8,7` / `9,10,12,11` | `fn_80c4f3a0` |
| `0x80C6C81C` | staging word (`1` in the analysis image) | `fn_80c52730`, `fn_80c53860` |
| `0x80377DC8..0x80377DD0` | transform seed words | `fn_80c52730` |

## Unknowns

- **Dropped receivers.** `0x8016D4B0` (`fn_80c00630`), slot `+0x280`
  (`fn_80c0c6f0`), `0x80137650` (`fn_80c43900`) and slot `+0x4C`
  (`fn_80c53f90`) are printed without arguments. The translation keeps the
  printed shape and records the receiver convention in a comment.
- **Probe argument split.** `0x800757B0` receives two adjacent stack vectors in
  `fn_80c04ef0`; the translation keeps the two arrays in the same order and
  width, but which one is input and which one is output is not proven.
- **GQR staging.** The exports of `0x80C00B30`, `0x80C04EF0`, `0x80C484C0`
  and `0x80C52730` contain `unaff_GQR0`/`ldexpf` blocks. These are compiler
  spills of paired-single stores, not game logic, and are not modelled.
- **Untranslated helper.** `0x80C566F0` is still untranslated; `fn_80c55220`
  keeps the raw call address.
- **Scene tables.** The masks passed to `0x800F57A0`/`0x800F10B0` in
  `fn_80c53860` and the row shape of `0x80354E60`/`0x80354E90` in
  `fn_80c4f3a0` are used literally, but their gameplay meaning is inferred.
- **Opaque descriptors.** `0x8082F150`, `0x80C6D328`, `0x80357BF4` and
  `0x80371870` are numeric ids or interface tags only.
- **No runtime addresses.** Recovering the real REL base remains the job of
  the `OSLink` loader lane, not this module.

## Verification

- `cargo check` exits 0 and reports no diagnostics for `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave19-bosses.csv` match the Rust functions one
  to one: 25 `translated`; 13 `high`, 12 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-19 ledger
  were edited.
