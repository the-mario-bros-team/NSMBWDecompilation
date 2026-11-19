# Coverage - Wave 24, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-24
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions) and
  `docs/COVERAGE_WAVE14_BOSSES.md` through
  `docs/COVERAGE_WAVE23_BOSSES.md`. This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (for example `0x80070BA0`) are absolute DOL
  addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel_d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` and `stub` are
  not used in this pass.
- **Confidence** rates the translation, not the guessed role. `high` means the
  stores, call arguments and branch tree are exact; `medium` means the body is
  exact but a helper role, ABI detail or dead paired-single staging was
  reconstructed from the surrounding code.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 4,644 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 16 |
| confidence `medium` | 9 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 441 |
| cumulative bytes in `game::bosses` | 121,028 |
| translated share of module functions | 57.27% |
| translated share of module text bytes | 33.96% |

This pass takes the largest remaining clean body in the module, the 1,056-byte
actor constructor `0x80C21BC0`, together with the aim pair that wave 23
deferred (`0x80C43B20`, `0x80C44110`), the two 184-byte effect sweeps, the
contact/travel segment update `0x80C54050`, the last five name comparators, one
magic-attack state entry and thirteen release wrappers. It clears every
`0x80C55F30` release wrapper and eight of the fourteen remaining
`0x8015F800` ones.

## Synthetic-base warning in one place

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it.

## Cluster 1 - actor constructor (1,056 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C21BC0` | 1,056 | `fn_80c21bc0` | actor constructor | translated | medium |

What is proven:

- The actor comes from `0x802DCD64`; the body writes 19 scalar fields before
  the first loop and calls class slot `+0x390` on the object at `+0x60`.
- The spawn loop is driven by `0x80C6147C` (value `5`) and the five-entry table
  at `0x80C61480`, three words per entry. Entry word 0 is the animation handle
  passed to `0x800E7880`; word 1 is a scale multiplied by `1.75` at `0x80C613D8`
  and stored at `+0x24/+0x28` of each `0xA4`-byte sub-object.
- The `+0xF98` index table is filled with `0..count-1`. The export unrolls it
  by eight; the translated loop preserves the same ascending writes.
- The hit box at `+0x728..+0x740` is derived from the 16.16 constants at
  `0x80C6111C..0x80C61130` (values `-1.5`, `+1.5`, `0`, `0`, `+2.0`, `+2.0`)
  divided and re-multiplied by `4096.0` (`0x80C57CF0`) and scaled by `+0xDC` or
  `+0xE0`. `0x8006D090` publishes the two three-word blocks.
- Four `0x14` counters are seeded at `+0x10C0`, `+0x1200`, `+0x1340` and
  `+0x1480`, then `0x802DCDB0(1)` unlocks.

## Cluster 2 - aim pair (488 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C43B20` | 372 | `fn_80c43b20` | vertical aim clamp | translated | medium |
| `0x80C44110` | 116 | `fn_80c44110` | elevation selector + clamp | translated | medium |

What is proven:

- `0x80C44110` calls `0x800AB9F0`, starts at `30.0` (`0x80C58B38`), switches to
  `45.0` (`0x80C58B3C`) when vtable slot `+0x14` of the object stored at
  `+0x5F8` returns nonzero, then calls `0x80C43B20` with `this`.
- `0x80C43B20` reads the signed direction byte `0x8042C1A0[+0x348]` through the
  `0x43300000` idiom against the bias at `0x80C58B20`, builds the two heights
  `base + spread * (target - 10)` and `base + spread * (target + 10)`
  (`+0xAC` is the base height, `0x80C58B2C` is `10.0`), samples `0x800B30C0`
  onto `+0x840`/`+0x844`, queries `0x80070BA0` at both candidates with the
  height `8.0` (`0x80C58B08`) plus `+0xE94`, and scales the vertical speed
  `+0xE8` by `(1.0 + target) / target` when the two ground codes differ.
- Both bodies carry paired-single/GQR preamble noise in the export; the
  translated code keeps the single-precision arithmetic and omits only the
  register save/restore staging.

## Cluster 3 - contact/travel segment update (864 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C54050` | 864 | `fn_80c54050` | travel segment / contact vector | translated | medium |

What is proven:

- The function copies `from`/`to` into `+0x128..+0x13C`, then takes the branch
  tree already seen in the translated sibling `fn_80c52730` (which uses X
  equality, modes `0`/`1` and `+0x24`); this body uses Z equality, modes `5`/`4`
  and the limit `+0x2C`.
- `0x800FA1D0` selects whether `dy`/`dz` come from the actor limits
  `+0x28`/`+0x2C`/`+0x44` or stay zero.
- The contact vector is `(dx, dy, dz)` where `dx` is
  `from.x - actor+0x24` only when both points are equal, otherwise `0`; the
  length goes through `0x801C14C0` into `+0xF4`; the vector is accumulated into
  `+0x10C..+0x114`.

## Cluster 4 - effect sweeps (368 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C2EBE0` | 184 | `fn_80c2ebe0` | seven-slot effect sweep | translated | high |
| `0x80C30940` | 184 | `fn_80c30940` | seven-slot effect sweep | translated | high |

What is proven:

- Both bodies call `0x802DCD74` for the manager, walk backwards from
  `manager + 0xD90` (or `+0xD6C`) in four-byte strides, and call `0x80683870`
  once per handle.
- The loop counter starts at `6` and the do-while condition is `-1 < counter`,
  so the body runs seven times (`0xD90..0xD78` and `0xD6C..0xD54`).
- The progress argument starts at `0` and grows by `5.0`
  (`0x80C57F54` / `0x80C5808C`); the third argument toggles `0`/`1` through an
  exclusive-or.
- `0x802DCDC0` unlocks at the end.

## Cluster 5 - the last five name comparators (680 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C37B90` | 136 | `fn_80c37b90` | `:`-suffix name compare | translated | medium |
| `0x80C3D480` | 136 | `fn_80c3d480` | `:`-suffix name compare | translated | medium |
| `0x80C42B60` | 136 | `fn_80c42b60` | `:`-suffix name compare | translated | medium |
| `0x80C46720` | 136 | `fn_80c46720` | `:`-suffix name compare | translated | medium |
| `0x80C4F280` | 136 | `fn_80c4f280` | `:`-suffix name compare | translated | medium |

These complete the thirteen-member comparator family documented once in
`fn_80c18470`. The candidate string is cut at the first `0x3A`, the object side
always advances past the character at the returned index, and `0x802E1AB4`
compares the remainders.

## Cluster 6 - release wrappers (1,144 bytes)

| address | size | Rust | base teardown | status | confidence |
|---|---:|---|---|---|---|
| `0x80C0DF20` | 88 | `fn_80c0df20` | `0x8015F800` | translated | high |
| `0x80C0DF80` | 92 | `fn_80c0df80` | `0x8015F800` | translated | high |
| `0x80C21180` | 92 | `fn_80c21180` | `0x8015F800` | translated | high |
| `0x80C21730` | 64 | `fn_80c21730` | none | translated | high |
| `0x80C29270` | 92 | `fn_80c29270` | `0x8015F800` | translated | high |
| `0x80C2C010` | 92 | `fn_80c2c010` | `0x8015F800` | translated | high |
| `0x80C2E150` | 92 | `fn_80c2e150` | `0x8015F800` | translated | high |
| `0x80C2FCF0` | 88 | `fn_80c2fcf0` | `0x8015F800` | translated | high |
| `0x80C2FD50` | 92 | `fn_80c2fd50` | `0x8015F800` | translated | high |
| `0x80C51D20` | 88 | `fn_80c51d20` | `0x80C55F30` | translated | high |
| `0x80C51EC0` | 88 | `fn_80c51ec0` | `0x80C55F30` | translated | high |
| `0x80C55330` | 88 | `fn_80c55330` | `0x80C55F30` | translated | high |
| `0x80C554D0` | 88 | `fn_80c554d0` | `0x80C55F30` | translated | high |

All thirteen bodies have the same three-part shape: test `this`, run the base
teardown (if the family has one), and call `0x802B9280(this)` only when
`free_it > 0`. They return `this` unchanged.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C0DF20` | `FUN_80c0df20` | `fn_80c0df20` | release wrapper | translated | high | `80c0df20_FUN_80c0df20.c` |
| `0x80C0DF80` | `FUN_80c0df80` | `fn_80c0df80` | release wrapper | translated | high | `80c0df80_FUN_80c0df80.c` |
| `0x80C21180` | `FUN_80c21180` | `fn_80c21180` | release wrapper | translated | high | `80c21180_FUN_80c21180.c` |
| `0x80C21730` | `FUN_80c21730` | `fn_80c21730` | release without base | translated | high | `80c21730_FUN_80c21730.c` |
| `0x80C21BC0` | `FUN_80c21bc0` | `fn_80c21bc0` | actor constructor | translated | medium | `80c21bc0_FUN_80c21bc0.c` |
| `0x80C29270` | `FUN_80c29270` | `fn_80c29270` | release wrapper | translated | high | `80c29270_FUN_80c29270.c` |
| `0x80C2C010` | `FUN_80c2c010` | `fn_80c2c010` | release wrapper | translated | high | `80c2c010_FUN_80c2c010.c` |
| `0x80C2E150` | `FUN_80c2e150` | `fn_80c2e150` | release wrapper | translated | high | `80c2e150_FUN_80c2e150.c` |
| `0x80C2EBE0` | `FUN_80c2ebe0` | `fn_80c2ebe0` | effect sweep | translated | high | `80c2ebe0_FUN_80c2ebe0.c` |
| `0x80C2FCF0` | `FUN_80c2fcf0` | `fn_80c2fcf0` | release wrapper | translated | high | `80c2fcf0_FUN_80c2fcf0.c` |
| `0x80C2FD50` | `FUN_80c2fd50` | `fn_80c2fd50` | release wrapper | translated | high | `80c2fd50_FUN_80c2fd50.c` |
| `0x80C30940` | `FUN_80c30940` | `fn_80c30940` | effect sweep | translated | high | `80c30940_FUN_80c30940.c` |
| `0x80C37B90` | `FUN_80c37b90` | `fn_80c37b90` | name compare | translated | medium | `80c37b90_FUN_80c37b90.c` |
| `0x80C39000` | `FUN_80c39000` | `fn_80c39000` | magic-attack state entry | translated | high | `80c39000_FUN_80c39000.c` |
| `0x80C3D480` | `FUN_80c3d480` | `fn_80c3d480` | name compare | translated | medium | `80c3d480_FUN_80c3d480.c` |
| `0x80C42B60` | `FUN_80c42b60` | `fn_80c42b60` | name compare | translated | medium | `80c42b60_FUN_80c42b60.c` |
| `0x80C43B20` | `FUN_80c43b20` | `fn_80c43b20` | vertical aim clamp | translated | medium | `80c43b20_FUN_80c43b20.c` |
| `0x80C44110` | `FUN_80c44110` | `fn_80c44110` | elevation selector | translated | medium | `80c44110_FUN_80c44110.c` |
| `0x80C46720` | `FUN_80c46720` | `fn_80c46720` | name compare | translated | medium | `80c46720_FUN_80c46720.c` |
| `0x80C4F280` | `FUN_80c4f280` | `fn_80c4f280` | name compare | translated | medium | `80c4f280_FUN_80c4f280.c` |
| `0x80C51D20` | `FUN_80c51d20` | `fn_80c51d20` | release wrapper | translated | high | `80c51d20_FUN_80c51d20.c` |
| `0x80C51EC0` | `FUN_80c51ec0` | `fn_80c51ec0` | release wrapper | translated | high | `80c51ec0_FUN_80c51ec0.c` |
| `0x80C54050` | `FUN_80c54050` | `fn_80c54050` | contact/travel update | translated | medium | `80c54050_FUN_80c54050.c` |
| `0x80C55330` | `FUN_80c55330` | `fn_80c55330` | release wrapper | translated | high | `80c55330_FUN_80c55330.c` |
| `0x80C554D0` | `FUN_80c554d0` | `fn_80c554d0` | release wrapper | translated | high | `80c554d0_FUN_80c554d0.c` |

## Unknowns

- **Constructor staging.** The first ~86 lines of `0x80C21BC0` build two dead
  stack slots from `f30`/`f31` and `ps30`/`ps31`. They are paired-single
  register saves, not fields, and are omitted. The meanings of the sub-object
  fields `+0x1C..+0x28`, `+0x9E`, `+0xA0` are inferred from the sibling
  `fn_80c27980`.
- **Aim ABI.** `0x80C43B20` is printed with a `double` first argument and
  `0x80C44110` converts float constants to `double`. The instruction semantics
  are single-precision; the translated code keeps `f32` and documents the
  promotion. The role of the direction table `0x8042C1A0` is inferred from
  `fn_80c3e970`/`fn_80c27980`.
- **Aim predicate.** Vtable slot `+0x14` of the object at `+0x5F8` is known only
  to return an integer that selects `45.0` over `30.0`.
- **Contact gate.** In `0x80C54050`, the role of `0x800FA1D0` and of the class
  slot `+0x24` check are inferred from the already-translated sibling
  `fn_80c52730`.
- **Effect sweep callee.** `0x80683870` receives a handle, a progress value and
  a parity bit; its internal role is not resolved in this workspace.
- **Release callees.** `0x8015F800`, `0x802B9280` and `0x80C55F30` are treated
  as base teardown, allocator release and full element destructor from the
  already-translated [`fn_80c046d0`]/[`fn_80c55f30`] family.

## Remaining candidates

| address | size | why deferred |
|---|---:|---|
| `0x80C51520` | 444 | route population; the `0x802DCD74` return pair and the dropped arguments are still unresolved |
| `0x80C389B0` | 72 | export truncated by bad instruction data after four stores |
| `0x80C38A00` | 72 | export truncated by bad instruction data after four stores |
| `0x80C3DFE0` | 72 | export truncated by bad instruction data after four stores |
| `0x80C3E030` | 72 | export truncated by bad instruction data after four stores |
| `0x80C228A0` | 40 | export truncated by bad instruction data after the first load |
| `0x80C31F90` | 88 | release wrapper, same shape as this pass; kept for a later sweep |
| `0x80C31FF0` | 92 | release wrapper, same shape as this pass; kept for a later sweep |
| `0x80C348F0` | 88 | release wrapper, same shape as this pass; kept for a later sweep |
| `0x80C34950` | 92 | release wrapper, same shape as this pass; kept for a later sweep |
| `0x80C3D190` | 92 | release wrapper, same shape as this pass; kept for a later sweep |
| `0x80C428F0` | 92 | release wrapper, same shape as this pass; kept for a later sweep |
| 45 bodies | 48 | `0x802DCBFC` destructor adapters |

The six remaining release wrappers and the forty-five destructor adapters are
mechanical repeats of shapes already translated; they are left only to keep
this pass at 25 bodies.

## Verification

- `cargo check` exits 0 and reports no diagnostic for `src/game/bosses.rs`
  (the remaining crate warnings belong to other lanes).
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; the five comparators and twelve
  release wrappers match their already-translated twins after relocation, and
  every `# Original` block names the file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave24-bosses.csv` match the Rust functions one
  to one: 25 `translated`; 16 `high`, 9 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-24 ledger
  were edited.
