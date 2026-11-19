# Coverage - Wave 4, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 24 functions added to `src/game/bosses.rs` by the wave-4
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). The first-pass 23
  functions are documented in `docs/COVERAGE_REL_d_en_bossNP.md`; the wave-3
  second pass of 20 functions is in `docs/COVERAGE_WAVE3_BOSSES.md`. This file
  does not repeat either set.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value here is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x80xxxxxx`, `0x807Cxxxx`, `0x8042xxxx`)
  are absolute and unaffected.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every row.
  Sizes are from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` means one
  documented piece is missing; `stub` is not used here.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores and calls are exact and the role is already established by a
  sibling or a string. `medium` means the stores and branches are exact but an
  object role, virtual-call prototype or untranslated helper is inferred.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table; no Nintendo symbol
  is invented. Boss and clip names appear only where an export string or a
  registered `StateID` proves them.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 24 |
| text bytes added | 7,472 |
| status `translated` | 24 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 11 |
| confidence `medium` | 13 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 67 |
| cumulative bytes in `game::bosses` | 46,968 |
| cumulative share of 770 functions | 8.70% |
| cumulative share of 356,396 text bytes | 13.18% |

The pass was chosen for control-flow completeness: the 17-function Castle Iggy
sled action family is now closed end to end, and the remaining entries add the
two `0x80C10xxx` action handlers, two shared-shape phase runners, the Larry
actor constructor, the castle spawn/attack phase manager and the `0x80C0CF60`
phase handler.

## Synthetic-base warning in one place

| item | value |
|---|---:|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in the Rust file is an analysis
address. A live dump will not match it. The constants read at `0x80C571xx`,
`0x80C572xx`, `0x80C574xx`, `0x80C585xx` and neighbours are analysis-image data
words, not hardware addresses.

## Cluster 1 - Castle Iggy sled action family

The clip names `sled_wait`, `sled_prov`, `sled_quickstA/B`,
`sled_damage_down`, `sled_damage_down_wait`, `sled_damage_up` and the effects
`Wm_ig_dizzy` / `Wm_ob_witchcraft` identify this family. The rider and sled
sub-object pointers at `+0xE80` and `+0xE84` are exact; their roles are
inferred from the fields they expose.

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C00D50` | 108 | `fn_80c00d50` | copy rider x, advance offset, rebuild y | translated | high |
| `0x80C00EA0` | 88 | `fn_80c00ea0` | shared hurt reaction, clip `0x566` | translated | high |
| `0x80C00FF0` | 124 | `fn_80c00ff0` | spawn `Wm_ig_dizzy` with the model transform | translated | medium |
| `0x80C01120` | 308 | `fn_80c01120` | rebuild the model matrix from position and Euler angles | translated | high |
| `0x80C012E0` | 216 | `fn_80c012e0` | spawn `Wm_ob_witchcraft`, sample rider values | translated | medium |
| `0x80C013E0` | 96 | `fn_80c013e0` | wait action plus rider-hurt re-entry | translated | high |
| `0x80C01440` | 168 | `fn_80c01440` | bind `sled_wait`, stop all velocity, clip `0x49B` | translated | high |
| `0x80C01500` | 324 | `fn_80c01500` | chase/attack with a three-way action choice | translated | medium |
| `0x80C01650` | 144 | `fn_80c01650` | bind `sled_prov`, clip `0x49C` | translated | high |
| `0x80C016E0` | 108 | `fn_80c016e0` | advance the two 16-bit damage timers | translated | high |
| `0x80C01750` | 192 | `fn_80c01750` | attack wait; end action on clip completion | translated | high |
| `0x80C01810` | 172 | `fn_80c01810` | select `sled_quickstA/B` from the rider side | translated | high |
| `0x80C018D0` | 220 | `fn_80c018d0` | quick-step end and hit re-check | translated | medium |
| `0x80C02770` | 196 | `fn_80c02770` | enter `sled_damage_down`, arm 300-frame timer | translated | medium |
| `0x80C02850` | 492 | `fn_80c02850` | damage-down / wait / up three-state sequence | translated | high |
| `0x80C02B20` | 52 | `fn_80c02b20` | stop the active effect and reset `+0x148` | translated | high |
| `0x80C02DC0` | 244 | `fn_80c02dc0` | witchcraft phase then rider byte-scale phase | translated | medium |

What is proven:

- The action strings `0x80C6C918`, `0x80C6C958`, `0x80C6C998`, `0x80C6C9D8`
  and `0x80C6CA54` are dispatched through the same class slot `+0xD4` and in
  the same branch positions as the export.
- The animation bind ritual is identical across the family: clip lookup
  `0x8023A0B0`, bind `0x801651F0`, blend `0x80169F80`, rate `0x801680E0`.
- The state counter is `+0x360`; the heading is `+0x102`; the timer is `+0x500`.
- Every clip name and id is taken directly from the C export; none is invented.

## Cluster 2 - action handlers in the `0x80C10xxx` range

`// inferred:` both addresses sit between the Castle Morton registrar
`0x80C10BD0` and the Castle Roy registrar `0x80C146B0`, but the export has no
boss-name string for either function, so the rows record mechanics only.

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C10610` | 872 | `fn_80c10610` | six-state action runner with six shrinking phase objects | translated | medium |
| `0x80C14390` | 288 | `fn_80c14390` | recoil/landing: invert velocity, slot `+0x550` | translated | medium |

`0x80C10610` scales the bytes at `+0x664` of the objects at
`+0xEA0..+0xEB4`, then dispatches on `+0x360`: entry poll (1), fall crossing
`0x80C57690` (2), heading spin `0x1990` (3), wait (4), landing and clip `0x50A`
(5), loop (6). `0x80C14390` is the paired landing response; its condition uses
the recorded ground height at `+0x10EC` and the three constants
`0x80C5D3A4/A8/AC`.

## Cluster 3 - shared-shape phase runners

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C198B0` | 408 | `fn_80c198b0` | bounce, facing flip, heading increment, two-state action | translated | medium |
| `0x80C44770` | 348 | `fn_80c44770` | sibling variant with constants `0x80C58B00` and id `0x512` | translated | medium |

Both consume the same layout: `+0x1EC` field probe, class slots `+0x544`,
`+0x564`, `+0x570`, `+0xD4`, the facing byte `+0x348`, the horizontal speeds
`+0xE8`/`+0xF4`, the heading `+0x102` and the per-frame adder `+0xAC0`.

## Cluster 4 - Larry constructor (inferred) and castle spawn phase

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C37DF0` | 620 | `fn_80c37df0` | Larry actor constructor and clip-name table | translated | medium |
| `0x80C23FD0` | 1,048 | `fn_80c23fd0` | castle spawn/attack phase manager keyed on `+0x76C` | translated | medium |

`0x80C37DF0` allocates through `0x802DCD74`, snapshots the spawn position,
fills the state block at `+0xE8C..+0xEB4`, installs the clip table at
`+0x750..+0x760` (`jump_st`, `throw_st`, `damage`), writes the scale pair at
`+0x840/+0x844`, and ends with `0x802DCDC0(1)`.

`0x80C23FD0` has two phases. Phase 0 tracks the player's side through
`0x80C23B60`, compares x, and selects a reaction from the distance bands at
`0x80C61D04..+0x10`. Phase 1 walks the four-slot timing table at
`0x80C61410/18` and drives sub-state `+0x774` through values 0, 3, 4 and 5.

## Cluster 5 - phase handler at `0x80C0CF60`

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C0CF60` | 636 | `fn_80c0cf60` | one-frame phase handler with a three-object vertical rig | translated | medium |

The handler updates the model and physics, scales the `+0x580` bytes of the
three rig objects, integrates `+0x112C/+0x1128` up to `+0x1124` and
`0x80C574A4`, unlinks the completed rig entries, and advances `+0x360`.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C00D50` | `FUN_80c00d50` | `fn_80c00d50` | rider transform sync | translated | high | `80c00d50_FUN_80c00d50.c` |
| `0x80C00EA0` | `FUN_80c00ea0` | `fn_80c00ea0` | hurt reaction | translated | high | `80c00ea0_FUN_80c00ea0.c` |
| `0x80C00FF0` | `FUN_80c00ff0` | `fn_80c00ff0` | dizzy effect | translated | medium | `80c00ff0_FUN_80c00ff0.c` |
| `0x80C01120` | `FUN_80c01120` | `fn_80c01120` | model matrix rebuild | translated | high | `80c01120_FUN_80c01120.c` |
| `0x80C012E0` | `FUN_80c012e0` | `fn_80c012e0` | witchcraft effect | translated | medium | `80c012e0_FUN_80c012e0.c` |
| `0x80C013E0` | `FUN_80c013e0` | `fn_80c013e0` | wait action | translated | high | `80c013e0_FUN_80c013e0.c` |
| `0x80C01440` | `FUN_80c01440` | `fn_80c01440` | `sled_wait` bind | translated | high | `80c01440_FUN_80c01440.c` |
| `0x80C01500` | `FUN_80c01500` | `fn_80c01500` | chase/attack choice | translated | medium | `80c01500_FUN_80c01500.c` |
| `0x80C01650` | `FUN_80c01650` | `fn_80c01650` | `sled_prov` bind | translated | high | `80c01650_FUN_80c01650.c` |
| `0x80C016E0` | `FUN_80c016e0` | `fn_80c016e0` | damage timer tick | translated | high | `80c016e0_FUN_80c016e0.c` |
| `0x80C01750` | `FUN_80c01750` | `fn_80c01750` | attack wait | translated | high | `80c01750_FUN_80c01750.c` |
| `0x80C01810` | `FUN_80c01810` | `fn_80c01810` | quick-step clip select | translated | high | `80c01810_FUN_80c01810.c` |
| `0x80C018D0` | `FUN_80c018d0` | `fn_80c018d0` | quick-step end | translated | medium | `80c018d0_FUN_80c018d0.c` |
| `0x80C02770` | `FUN_80c02770` | `fn_80c02770` | damage-down entry | translated | medium | `80c02770_FUN_80c02770.c` |
| `0x80C02850` | `FUN_80c02850` | `fn_80c02850` | damage sequence | translated | high | `80c02850_FUN_80c02850.c` |
| `0x80C02B20` | `FUN_80c02b20` | `fn_80c02b20` | effect stop | translated | high | `80c02b20_FUN_80c02b20.c` |
| `0x80C02DC0` | `FUN_80c02dc0` | `fn_80c02dc0` | witchcraft phase | translated | medium | `80c02dc0_FUN_80c02dc0.c` |
| `0x80C0CF60` | `FUN_80c0cf60` | `fn_80c0cf60` | phase handler | translated | medium | `80c0cf60_FUN_80c0cf60.c` |
| `0x80C10610` | `FUN_80c10610` | `fn_80c10610` | six-state action runner | translated | medium | `80c10610_FUN_80c10610.c` |
| `0x80C14390` | `FUN_80c14390` | `fn_80c14390` | recoil/landing | translated | medium | `80c14390_FUN_80c14390.c` |
| `0x80C198B0` | `FUN_80c198b0` | `fn_80c198b0` | bounce/phase runner | translated | medium | `80c198b0_FUN_80c198b0.c` |
| `0x80C23FD0` | `FUN_80c23fd0` | `fn_80c23fd0` | castle spawn phase | translated | medium | `80c23fd0_FUN_80c23fd0.c` |
| `0x80C37DF0` | `FUN_80c37df0` | `fn_80c37df0` | Larry constructor | translated | medium | `80c37df0_FUN_80c37df0.c` |
| `0x80C44770` | `FUN_80c44770` | `fn_80c44770` | bounce-phase runner | translated | medium | `80c44770_FUN_80c44770.c` |

## Unknowns

- **Untranslated callees.** `0x80C00B30` (shared sled contact/attack test),
  `0x80C0AB10` (rig integration), `0x80C0EBF0`, `0x80C23B60`, `0x80C243F0`,
  `0x80C43E90` and `0x80C43EF0` are still outside this lane. Every call keeps
  its raw address, and no behavior is assumed beyond the export's use of the
  return value.
- **Paired-single preambles.** `0x80C23FD0` and `0x80C10610` begin with the
  GQR save/restore and `ldexpf` sequence that Ghidra prints for paired-single
  code. It is compiler bookkeeping, not game state, and is not translated; the
  gameplay body after it is complete.
- **Ground height at `0x80C14390`.** The handler compares `+0xB0` against
  `+0x10EC + 0x80C5D3AC`. `+0x10EC` is read as a `f32`; the export does not
  name the object that writes it.
- **Boss attribution.** The export has no boss-name string for `0x80C0CF60`,
  `0x80C10610` or `0x80C14390`. They sit between the Castle Morton registrar
  `0x80C10BD0` and the Castle Roy registrar `0x80C146B0`, but that range alone
  does not prove ownership, so the table records mechanics, not boss names.
- **Action strings.** The five sled action addresses and the other action
  addresses are data pointers; their text is not printed by the C export, so
  only the address is recorded.
- **Opaque constants.** `0x8042BB08`, `0x8042BB0C`, `0x80C6632C..38`,
  `0x80C61D24/28`, `0x80C61410/18` and the `0x80C571xx..0x80C585xx` banks are
  read from their original addresses; their numeric values are not claimed.
- **`0x80C37DF0` actor size.** The constructor writes through `+0xAF0` and
  `+0xEBC`; the allocation site `0x802DCD74` is the only evidence for the
  object size, which is not printed.

## Verification

- `cargo check` passes for the workspace (exit 0) with no error or warning in
  `src/game/bosses.rs`; `bosses.rs` was also compiled in an isolated `no_std`
  crate with `src/platform.rs` and finished with exit 0.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- The 24 evidence files were read in full; every `# Original` block names the
  file and size from `functions.tsv`.
- The 24 rows in `ledger/agents/wave4-bosses.csv` match the Rust functions one
  to one: 24 `translated`; 11 `high`, 13 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-4 ledger
  were edited.
