# Coverage - Wave 158 (`game::bases`)

## How to read this file

This page documents the Wave 158 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL (REL module 2). Every address is a **static
analysis address** under the canonical synthetic base **`0x80600000`**; the Wii
REL loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the line `Synthetic base: 0x80600000; this is not a runtime
address.`

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a table meaning had
to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 13 | 5,432 | 4 | 9 |

All 13 rows are `translated`; none is `partial`. The module header now reports
3,828 bodies (3,815 before this pass + 13). Eight rows are the eight "ready" rows
of `docs/SPEC_WAVE157_BASES.md` (4,416 bytes); the other five are one
`dEasyPairing_c` screen cluster from the `0x8061` planner bucket (1,016 bytes),
which takes that bucket from 67 to 62 untranslated addresses.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE157_BASES.md`**, the
  560-to-544-byte band:

  | cluster | body |
  |---|---|
  | `cobGhost` model-load cue | `0x80763C20`: state `+0x3D4`, latch `+0x3D0`, clip `+0x1B4`/`+0x224`, effect `0x2B0`, cue `0x801016B0` |
  | world-map rail vehicle | `0x807A0090`: rail name arrays, `+0x688` node offset, `+0x184` element, aim point `+0x784..+0x78C` |
  | world-map route actor | `0x8077C440`: `+0x210` player-count index, `+0x274` target kind, direction states `2..5` |
  | marker angle tape | `0x80741A70`: 16-slot tape `+0x7E0`, 64-entry ring `+0x800`, `0x8015F4F0` step `0x200` |
  | course mount body | `0x807390D0`: `0x807CDC00` mirror table, mount branch on `+0x12C`, three descriptors |
  | world-map action handler | `0x80776440`: actions `31`/`32`, 60-frame approach, node position, idle countdown |
  | archive-name collector | `0x807B2550`: four group records, four `Pa3_` compares, `0x8031A834` registry, two `dres_mng` requests |
  | rider clamp/release | `0x806D8B90`: clamp modes `0..4`, `+0x58C` latch, rider pass, `+0x508` release, vtable `+0xE0` |

- **Five `0x8061`-bucket bodies of the `dEasyPairing_c` screen** (1,016 bytes):

  | cluster | body |
  |---|---|
  | teardown | `0x80612D30`: `+0x18` sub-object destructor, optional allocator release, returns the receiver |
  | layout load | `0x80612EC0`: `easyPairing_easyPairing.arc`, `easyPairing_11.brlyt`, `inWindow.brlan`, `A00_Window`, `T_title_00` |
  | occupancy scan | `0x806131D0`: four player slots through `player_actor`, state `2 -> 3` / `0 -> 1` |
  | state apply | `0x80613380`: system `+0x1D8` bit `0x40`, twelve seeded words, pane/text pass, sound `0x83` |
  | gated exit | `0x80613A30`: layout animation `-1`, clears the system bit, pane byte, exit state object |

The five bucket bodies share one layout: `+0x70` NW4R layout, `+0x78` layout
interface, `+0x244` pane pointer, `+0x248` four-entry pane array, `+0x258`
four-entry text array, `+0x27C + i*4` per-player state words, `+0x28C..+0x2A8`
animation times and `+0x208` the interface that receives the state objects.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80612D30` | `fn_80612d30` | 100 | high |
| `0x80612EC0` | `fn_80612ec0` | 220 | medium |
| `0x806131D0` | `fn_806131d0` | 184 | medium |
| `0x80613380` | `fn_80613380` | 384 | medium |
| `0x80613A30` | `fn_80613a30` | 128 | medium |
| `0x806D8B90` | `fn_806d8b90` | 544 | high |
| `0x807390D0` | `fn_807390d0` | 548 | medium |
| `0x80741A70` | `fn_80741a70` | 552 | high |
| `0x80763C20` | `fn_80763c20` | 560 | high |
| `0x80776440` | `fn_80776440` | 548 | medium |
| `0x8077C440` | `fn_8077c440` | 556 | medium |
| `0x807A0090` | `fn_807a0090` | 560 | medium |
| `0x807B2550` | `fn_807b2550` | 548 | medium |

**Totals:** 13 bodies, 5,432 bytes. The address window runs from `0x80612D30` to
`0x807B2550`; the bucket subset is `0x80612D30..0x80613A30`.

## Where the binary differed from the prose

1. **`0x80776440`'s distance test is the other way round.** The specification
   says the 60-frame countdown is armed when `|d| > 1.1920929e-7f` and blames a
   `cror 2,0,2`. The raw instruction at `0x807765EC` is `4c401382` = `cror 2,1,2`
   (`GT|EQ`, the standard `>=` materialisation), so the `beq` at `0x807765F8`
   jumps to the epilogue when `|d| >= eps`; the arming block at
   `0x807765FC..0x80776608` therefore runs only when the remaining distance is
   **below** the epsilon (or unordered). Ghidra's own C shows the correct
   polarity. The translation keeps the raw sequence as
   `if !(distance.abs() >= 1.192_092_9e-7)`, so a NaN distance arms the timer
   exactly as the shipped branch does.
2. **`0x807A0090`'s `0x800FC140` argument is a byte value, not an address.** The
   words at `0x807A00E8..0x807A00F4` are `lis r3,0x8043` / `li r4,0` /
   `lbz r3,-0x5DB3(r3)` / `bl`; the callee receives `*(u8*)0x8042A24D`. The
   export prints `DAT_8042a24d`, which reads the same, but the translation passes
   the loaded byte so the argument is unambiguous.
3. **`0x80612EC0`'s pane binding reads its vtable from `+0x78`, not from the
   `+0x70` receiver.** The C is
   `(**(code **)(*(int *)(param_1 + 0x78) + 0x14))(param_1 + 0x70, ...)`, which
   [`vmethod1`] cannot express; the body reads the vtable word and calls the slot
   with `r3 = this + 0x70`, `r4` the pane name and `r5 = 0`.

Confirmations rather than corrections: `0x80763C20`'s Ghidra "receiver-less"
calls keep `r3 = this` (`mr r27,r3` at `0x80763C4C`); `0x80741A70`'s only `psq`
words are the `f29`/`f30`/`f31` save bracket; `0x807390D0`'s `cntlzw` chain at
`0x807391C4..0x807391D0` materialises `x < 0.0f` through `cror 2,1,2`, matching
the specification; `0x8077C440` really calls `0x80780A40` twice and keeps the
first result for `n1 - 1`; and `0x806D8B90`'s mode-`4` arm counts its two hits in
a single byte register, exactly as written.

## Notes on inference

- `0x807A0090` is `medium` because the semantic role of the `0x8081BE40` block
  passed to slot `+0xC0` is unknown; every store and constant is literal.
- `0x8077C440` is `medium`: `0x80780A40` (player count), `0x80780BD0` (route
  mapping) and `0x80101630` (context record) are named by address only.
- `0x80776440` is `medium`: the action ids `31`/`32` are inferred from their
  blocks, and `0x80772BD0` (node lookup) / `0x80160AE0` (steer-toward, returns
  the remaining distance) are address-only.
- `0x807B2550` is `medium`: the `0x8031A834` registry and the group table are
  runtime data; only the access shape is proven.
- `0x80612EC0` and `0x80613A30` are `medium` for the roles of the
  `A00_Window`/`T_title_00` panes and of the two `0x80823D30`/`0x80823D70` state
  objects; `0x80109340` and `0x80195380` stay address-named.
- `0x806131D0` and `0x80613380` are `medium`: the per-player state transitions
  are literal, but the `0x80371870` argument and the `+0xBB`/`+0xB8` pane bytes
  are only known by their use.
- The remaining four bodies are `high`: every guard, store, constant and call
  argument is visible in the export and confirmed against the raw words.

## Deliberate skips

- `0x80613290` (216, same `dEasyPairing_c` window) was not written: its export
  declares `unaff_GQR0`, Ghidra reports an unreachable block at `0x8061334C`, and
  the `0x807C6AB8` index table plus the `0x28`-byte slot stride would have to be
  assumed. It stays deferred rather than guessed.
- `0x8061F2F0` (516), `0x8061B260` (496), `0x80610510` (460) and `0x8061DC70`
  (460) were not started: they are the four largest rows left in the planner
  bucket and each needs its own full decode.
- Nothing was written that the raw words did not support.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (13 rows, 5,432 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 13 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave158` reports `OK` for
  `wave158-bases.csv` (13 rows, 13 unique addresses).
- `cargo check --message-format=short` produces no error and no diagnostic naming
  `src/game/bases.rs`.
- The eight spec bodies were read word by word from
  `work/rels/elf/d_basesNP.elf` with
  `file_offset = (address - 0x80600000) + 0x1000`; the only correction needed is
  item 1 above.
