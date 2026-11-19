# Coverage - Wave 12, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 24 functions added to `src/game/bosses.rs` by the wave-12
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions) and in the
  wave-3/4/5/6/10 boss documents (20 + 24 + 25 + 25 + 25). This file does not
  repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value here is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x8016xxxx`, `0x8019xxxx`, `0x8023xxxx`,
  `0x802Exxxx`) and the absolute DOL words `0x8082FFE0..0x8082FFEC`,
  `0x80429D40`, `0x80429D60`, `0x80429DD0` and `0x80429EE8` are unaffected.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every row.
  Sizes are from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` means one
  documented piece is missing; `stub` is not used here.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores and calls are exact. `medium` means the stores and branches are
  exact but a dropped receiver or a missing argument is reconstructed by
  convention.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. Strings appear only where the export points at the exact
  data address.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 24 |
| text bytes added | 3,996 |
| status `translated` | 24 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 15 |
| confidence `medium` | 9 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 166 |
| cumulative bytes in `game::bosses` | 70,256 |

The pass continues the Castle Iggy phase-object library that wave 10 opened and
then closes the `0xFD0` actor that uses the water flag, the `jumpA`/`jumpB`
clips and the demo-escape animation. It adds the rig height helper, the
name-compare/release utilities, the `Wm_ob_witchcraft` spawner, the `0xFD0`
constructor and destructor, the water-impact probe, the two per-frame jump
handlers, the two-sided impact spawner and the demo-escape entries.

## Synthetic-base warning in one place

| item | value |
|---|---:|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The constants read at
`0x80C576xx`, `0x80C578xx`, `0x80C57Axx`, `0x80C5E2xx`, `0x80C5E3xx`,
`0x80C5A938`, `0x80C5C614` and `0x80C6Axxx` are analysis-image data words, not
hardware addresses.

## Cluster 1 - phase-object utilities and effects (696 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C0B4E0` | 40 | `fn_80c0b4e0` | add the selected rig height to `+0xB0` | translated | high |
| `0x80C0E230` | 136 | `fn_80c0e230` | compare the `:`-suffix name | translated | medium |
| `0x80C0E350` | 48 | `fn_80c0e350` | allocate `0x1010` and initialize | translated | medium |
| `0x80C0E420` | 64 | `fn_80c0e420` | conditional allocator release | translated | medium |
| `0x80C0E460` | 88 | `fn_80c0e460` | base teardown and optional release | translated | high |
| `0x80C0E7B0` | 224 | `fn_80c0e7b0` | spawn `Wm_ob_witchcraft` at `+0xEC4` | translated | high |
| `0x80C0E890` | 96 | `fn_80c0e890` | controller step and `+0xE7C` countdown | translated | high |

What is proven:

- `0x80C0B4E0` scales the signed index byte at `+0x1134` by four into the rig
  table at `+0x1140`, then adds `+0x1138` and the rig `+0xB0`. This is the same
  rig table and signed-index idiom as the wave-10 launch handler `0x80C0B810`.
- `0x80C0E230` splits both strings at `0x3A` (`:`) with `0x802E1C40`, then
  compares the suffixes with `0x802E1AB4` and inverts the result.
- `0x80C0E350` allocates `0x1010` bytes and calls `0x80C0E380` only on success;
  `0x80C0E460` is the matching `0x800A8B10`/`0x80162920` teardown wrapper.
- `0x80C0E7B0` is the second `Wm_ob_witchcraft` spawner; it mirrors
  `0x80C07870`/`0x80C0B520` but reads `+0xEC4/+0xEC8` and the
  `0x80C5762C/84/88/80/8C/30` table.
- `0x80C0E890` calls vtable slot `+0x10` of `+0x394` every frame and calls
  `0x80C43900(actor)` when the signed counter reaches zero.

## Cluster 2 - the `0xFD0` actor (2,320 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C15C20` | 136 | `fn_80c15c20` | construct the `0xFD0` actor | translated | high |
| `0x80C15CF0` | 112 | `fn_80c15cf0` | destroy the `0xFD0` actor | translated | high |
| `0x80C15D60` | 228 | `fn_80c15d60` | move to spawn and attach effect `0x111` | translated | medium |
| `0x80C15E50` | 348 | `fn_80c15e50` | water/ground impact probe | translated | high |
| `0x80C16320` | 120 | `fn_80c16320` | align `value` around `target` by `step` | translated | high |
| `0x80C16520` | 148 | `fn_80c16520` | select a duration from two class slots | translated | high |
| `0x80C166E0` | 120 | `fn_80c166e0` | spawn `Wm_we_wlandsmoke` | translated | high |
| `0x80C169A0` | 208 | `fn_80c169a0` | model step and two hit queries | translated | medium |
| `0x80C16B10` | 376 | `fn_80c16b10` | per-frame `jumpA` handler | translated | high |
| `0x80C16D00` | 376 | `fn_80c16d00` | per-frame `jumpB` handler | translated | high |
| `0x80C17550` | 148 | `fn_80c17550` | `damage` state entry | translated | medium |

What is proven:

- The constructor installs `0x80C6AC34` at `+0x60`, then overwrites it with
  `0x80C5E360`; the intermediate write is preserved in source order. `+0xE74`
  receives `0x80C5E988`, `+0xE78` points back to the object, and `+0xE7C` is
  initialized with `0x802D7AA0` and then set to the controller vtable
  `0x80C5A938`.
- The destructor tears the `+0xE7C` sub-object down with `0x802D7B20`, calls
  `0x800A8B10` and optionally `0x80162920`; it is the exact sibling of the
  wave-10 release wrappers.
- `0x80C15D60` copies the position to `+0xF94..+0xF9C`, applies the
  `0x80C5E314/E318/E31C` deltas and writes the three absolute DOL words
  `0x8082FFE0..0x8082FFEC`, then spawns effect `0x111` and clears `+0xFB4`.
- `0x80C15E50` is the only grid probe in this pass: it calls `0x800757B0`
  with the actor x, the requested height and `0x80C5783C`, compares the
  remaining distance against `0x80C57840`, builds the sound argument
  `(side << 16) | 1` with `0x800929F0`, selects mode 7/6 for `0x80078410` and
  finishes with `0x8006A4B0`/`0x80198FC0`.
- `0x80C16B10` and `0x80C16D00` are the same body with different clip pairs:
  `jumpA`/`jumpA_water` at `0x80C5E28C/0x80C5E280` and
  `jumpB`/`jumpB_water` at `0x80C5E2A0/0x80C5E294`. Both use `0x800E7970`,
  `0x80168FE0`, the model update, `0x800955F0`, class slot `+0x52C`,
  `0x800AA150`, `0x800AA620(actor, 0x400)` and the ground probe `0x80072BC0`.
- `0x80C17550` copies `+0xE8..+0xF0` to `+0xFA4..+0xFAC`, sets `0x8082FFEC`
  to 3 and `+0x18->+0x3D4` to 1, then writes the state words `+0xFBC`,
  `+0xFC0` and `+0x360`.

## Cluster 3 - impact and animation entries (980 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C18E40` | 380 | `fn_80c18e40` | two-sided impact effects | translated | high |
| `0x80C191A0` | 140 | `fn_80c191a0` | collision result transfer | translated | medium |
| `0x80C194E0` | 128 | `fn_80c194e0` | select clip `0x496`/`0x497` | translated | high |
| `0x80C19560` | 52 | `fn_80c19560` | facing-indexed motion table | translated | high |
| `0x80C1A0D0` | 124 | `fn_80c1a0d0` | start the effect-slot clip | translated | medium |
| `0x80C1AD50` | 156 | `fn_80c1ad50` | demo-escape clip and velocity clear | translated | medium |

What is proven:

- `0x80C18E40` selects the impact name by comparing the x speed `+0xE8`
  against `0x80C57A1C`; the low case uses `+0x2C` and the high case `+0x28`.
  The landing block is gated by the second argument and the `+0xF4` switch,
  and the `+0x38` name is always spawned through `+0xD44`.
- `0x80C19560` indexes the four-word table at `0x80C57A60` with the facing
  byte `+0x348` and mirrors `0x80C57A68/0x80C57A6C/0x80C57A68` into
  `+0x7BC/+0x7C0/+0x7C4`.
- `0x80C1A0D0` and `0x80C1AD50` share the same clip-bind sequence:
  `0x8023A0B0` → `0x801651F0` → `0x80169F80` → `0x801680E0`. The only
  differences are the guard word (`+0x760 + 4` vs `+0x754` first word) and
  the height constant (`0x80C57A1C` vs `0x80C57A24`).

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C0B4E0` | `FUN_80c0b4e0` | `fn_80c0b4e0` | rig height add | translated | high | `80c0b4e0_FUN_80c0b4e0.c` |
| `0x80C0E230` | `FUN_80c0e230` | `fn_80c0e230` | suffix name compare | translated | medium | `80c0e230_FUN_80c0e230.c` |
| `0x80C0E350` | `FUN_80c0e350` | `fn_80c0e350` | 0x1010 constructor wrapper | translated | medium | `80c0e350_FUN_80c0e350.c` |
| `0x80C0E420` | `FUN_80c0e420` | `fn_80c0e420` | conditional release | translated | medium | `80c0e420_FUN_80c0e420.c` |
| `0x80C0E460` | `FUN_80c0e460` | `fn_80c0e460` | teardown wrapper | translated | high | `80c0e460_FUN_80c0e460.c` |
| `0x80C0E7B0` | `FUN_80c0e7b0` | `fn_80c0e7b0` | `Wm_ob_witchcraft` spawner | translated | high | `80c0e7b0_FUN_80c0e7b0.c` |
| `0x80C0E890` | `FUN_80c0e890` | `fn_80c0e890` | controller step/countdown | translated | high | `80c0e890_FUN_80c0e890.c` |
| `0x80C15C20` | `FUN_80c15c20` | `fn_80c15c20` | 0xFD0 constructor | translated | high | `80c15c20_FUN_80c15c20.c` |
| `0x80C15CF0` | `FUN_80c15cf0` | `fn_80c15cf0` | 0xFD0 destructor | translated | high | `80c15cf0_FUN_80c15cf0.c` |
| `0x80C15D60` | `FUN_80c15d60` | `fn_80c15d60` | spawn reposition | translated | medium | `80c15d60_FUN_80c15d60.c` |
| `0x80C15E50` | `FUN_80c15e50` | `fn_80c15e50` | water/ground impact | translated | high | `80c15e50_FUN_80c15e50.c` |
| `0x80C16320` | `FUN_80c16320` | `fn_80c16320` | integer align helper | translated | high | `80c16320_FUN_80c16320.c` |
| `0x80C16520` | `FUN_80c16520` | `fn_80c16520` | duration selection | translated | high | `80c16520_FUN_80c16520.c` |
| `0x80C166E0` | `FUN_80c166e0` | `fn_80c166e0` | `Wm_we_wlandsmoke` spawner | translated | high | `80c166e0_FUN_80c166e0.c` |
| `0x80C169A0` | `FUN_80c169a0` | `fn_80c169a0` | model step/hit query | translated | medium | `80c169a0_FUN_80c169a0.c` |
| `0x80C16B10` | `FUN_80c16b10` | `fn_80c16b10` | `jumpA` handler | translated | high | `80c16b10_FUN_80c16b10.c` |
| `0x80C16D00` | `FUN_80c16d00` | `fn_80c16d00` | `jumpB` handler | translated | high | `80c16d00_FUN_80c16d00.c` |
| `0x80C17550` | `FUN_80c17550` | `fn_80c17550` | `damage` state entry | translated | medium | `80c17550_FUN_80c17550.c` |
| `0x80C18E40` | `FUN_80c18e40` | `fn_80c18e40` | two-sided impacts | translated | high | `80c18e40_FUN_80c18e40.c` |
| `0x80C191A0` | `FUN_80c191a0` | `fn_80c191a0` | collision transfer | translated | medium | `80c191a0_FUN_80c191a0.c` |
| `0x80C194E0` | `FUN_80c194e0` | `fn_80c194e0` | clip selection | translated | high | `80c194e0_FUN_80c194e0.c` |
| `0x80C19560` | `FUN_80c19560` | `fn_80c19560` | motion table mirror | translated | high | `80c19560_FUN_80c19560.c` |
| `0x80C1A0D0` | `FUN_80c1a0d0` | `fn_80c1a0d0` | effect-slot clip start | translated | medium | `80c1a0d0_FUN_80c1a0d0.c` |
| `0x80C1AD50` | `FUN_80c1ad50` | `fn_80c1ad50` | demo-escape entry | translated | medium | `80c1ad50_FUN_80c1ad50.c` |

## Unknowns

- **Owner.** No export string in this band names the boss. The water flag
  `+0xFB4`, the `jumpA`/`jumpB` clips and the demo-escape strings suggest a
  water-capable boss, but the documents do not assign a name.
- **Dropped receivers.** `0x80C0E350` (`FUN_80c0e380`), `0x80C0E420`
  (`0x802B9280`), `0x80C15D60` (`FUN_80c4a940`), `0x80C169A0` (slot `+0x28`),
  `0x80C191A0` (`0x80162D00`) and `0x80C17550` (`FUN_80c4d1c0`) are printed
  without arguments. Each receiver is reconstructed by the sibling convention
  and the rows are `medium`.
- **`0x8023A0B0` arity.** In `0x80C1A0D0`/`0x80C1AD50` the export prints a
  single argument, while the clip binders in the jump handlers print the model
  and the name. The missing clip-name argument is preserved as missing; the
  rows are `medium`.
- **Virtual prototypes.** Slots `+0xA0`, `+0x74`, `+0x2C`, `+0x24`, `+0x28`,
  `+0x10`, `+0x14`, `+0x1C`, `+0x52C` and `+0xD4` are named by offset only;
  the argument shapes printed by the export are preserved.
- **Opaque constants.** `0x80C5762C/30/80/84/88/8C`, `0x80C57838/3C/40/44/50`,
  `0x80C5786C`, `0x80C57A18/1C/24/60/68/6C`, `0x80C5E258/280/28C/294/2A0/
  2D2/2D4/314/318/31C/320/328/360/988`, `0x80C5A938`, `0x80C5C614`,
  `0x80C6AC34`, `0x80C6DEF4`, `0x80830000`, `0x80830040` and the four
  absolute DOL words are read from their original addresses; their numeric
  values are not claimed (except the string addresses named in the text).
- **GQR noise.** `0x80C15E50`, `0x80C16B10` and `0x80C16D00` carry
  `ldexpf`/GQR preambles in the export. They are compiler bookkeeping and are
  documented, not translated.

## Verification

- `cargo check` passes (exit 0); `src/game/bosses.rs` produces no warning.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 24 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 24 rows in `ledger/agents/wave12-bosses.csv` match the Rust functions
  one to one: 24 `translated`; 15 `high`, 9 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-12 ledger
  were edited.
