# Coverage - Wave 94 (`game::enemies`, the `0x80B20000` bucket)

## How to read this file

This page records the Wave 94 `game::enemies` lane: 28 functions of the
`0x80B20000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs` (section 75 of that file). Every row below is one real
`pub unsafe fn fn_<addr>` body in that file, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave94-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. The float
and string constants cited below were read out of
`work/rels/elf/d_enemiesNP.elf` at the cited synthetic addresses, never guessed
from the C text.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 3,692 | 11 | 11 | 6 |

The bucket holds 139 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 33 were already in
`ledger/functions.csv` before this pass (the wave-79 `0x80B24xxx..0x80B28Axx`
run, the `80b201e0..80b20860` handlers, `register_block_help_states` at
`0x80B213D0`, `register_wrturn_states` at `0x80B287E0` and
`alloc_wrturn_actor` at `0x80B28B20`). 28 are claimed here and 78 remain.

## What this pass covers

Three clusters, all inside `0x80B20980..0x80B23610`.

**Run 1 - the `block_help` actor, `0x80B20980..0x80B21660`.**

- `0x80B20980`/`0x80B209B0` are the two 48-byte `0x802DCBFC`
  argument-swapping forwarders of the class.
- `0x80B209E0` allocates the `0x790`-byte actor: `0x801628C0(0x790)`, the base
  constructor `0x80094E80`, the interim vtable `0x80301EA8`, the `+0x534`
  helper reset `0x8007F7A0`, then the class vtable `0x80B6BFC0` and five
  embedded sub-objects (`+0x698` model block, `+0x6B8` holder, `+0x6F8` with
  vtable `0x803295D8` rewritten to `0x80329518`, `+0x758` via `0x8016A740`,
  `+0x774` with vtable `0x80317860`).
- `0x80B20DE0` runs the stance update only when `0x80064540()` answers with a
  live player object; `0x80B20E60` is the one-call `+0x534` reset that always
  returns `1`.
- `0x80B20E90` is the stance step: the `+0xAC` position is pre-transformed with
  the `+0x100`/`+0x102`/`+0x104` angle triple by `0x80064AF0`, built into the
  `+0x7C` matrix by `0x801C0BD0` plus the three axis setters, attached to the
  `+0x6B8` holder and followed by the `+0x774` `+0x10` slot.
- `0x80B20F80` binds `g3d/block_help.brres` (`0x80B6BF9C`) and the `block_help`
  clip (`0x80B6BFB4`), with element size `0x128` for the model holder and a
  `0x80165C30` damage-clip pair.
- `0x80B210D0` tests the actor's `+0x394` object against the `0x80B79538`
  descriptor and then maps the player's carried-object state onto the
  `0x80352A34`/`0x80352A74` states, writing `+0x754`, `+0x68A` and `+0x689`.
- `0x80B214E0` is the deleting destructor; `0x80B21600`/`0x80B21630` are two
  more forwarders; `0x80B21660` is the `0x600`-byte sibling constructor with
  class vtable `0x80B6C368` and the `0x80B330E0` animation holder.

**Run 2 - the `waterbubble` actor, `0x80B21AC0..0x80B22680`.**

- `0x80B21AC0` tilts the actor through `0x8012F810`/`0x8012F8E0` with
  `abs(value) / divisor` and the sign of the original value; the divisor is
  `4.0` when `+0x5F4 > 9` and `10.0` otherwise, and a zero `+0xEC` falls back to
  `+0xE8`.
- `0x80B21BD0` binds `g3d/waterbubble.brres` (`0x80B6C344`) and the
  `waterbubble` clip (`0x80B6C35C`) into the `+0x524`/`+0x544`/`+0x584`
  holder triple.
- `0x80B21CF0` is the same matrix rebuild as `0x80B20E90` without the trailing
  sub-object call; `0x80B21DF0` saves the `+0xE8`..`+0xF0` velocity triple
  around the `0x80096E60` test and latches `+0x5F8`.
- `0x80B21EB0`/`0x80B22050` are the full and deleting destructors;
  `0x80B22170`/`0x80B221A0` the two forwarders of the next skeleton.
- `0x80B221D0` allocates the `0x558`-byte `paipo` sibling with class vtable
  `0x80B6C6A8`; `0x80B222E0` binds `g3d/paipo.brres` (`0x80B6C690`) and the
  `paipo` clip (`0x80B6C6A0`).
- `0x80B22540` builds the fifteen-word effect record
  `{0, x, y-10.0, z, 1.0, 1.0, 1.0, 0.0 x 8}` from the actor position, hands it
  to `0x800B8700` and starts the emitter with `0x802DCA98(record, 0x80A0B5A0,
  0x3C, 1)`.
- `0x80B22680` clamps `+0xB0` to the scene floor
  (`0x80429DD0` + `0x8FEC4`) and enters `0x80B795F8`.

**Run 3 - four `Wm_en_pakkun` helpers, `0x80B23200..0x80B23610`.**

- `0x80B23200` promotes the joint-2 transform (`0x80168180(actor + 0x544, 2,
  buffer)`) into the `+0x5F4` world point that the `Wm_en_pakkun_foo` effect
  spawner reads.
- `0x80B232B0` forwards the record word at `+4` to `0x80096CA0` and to the
  actor's `+0x258` vtable slot.
- `0x80B235D0`/`0x80B23610` resolve the `+0x5F0` handle through `0x80162D00`
  and set `+0x550`/`+0x554` on the answer.

### Identity evidence

- **No class name is claimed beyond the three model strings.** The names
  `block_help`, `waterbubble` and `paipo` are the literal BRRES/clip strings
  read from `0x80B6BF9C`/`0x80B6BFB4`, `0x80B6C344`/`0x80B6C35C` and
  `0x80B6C690`/`0x80B6C6A0` in the ELF image. The `Wm_en_pakkun_foo` string at
  `0x80B6CA3C` belongs to the run-3 actor, but no state table for it is
  translated here, so its class keeps no `StateID` name.
- **The two class vtables are cited by address only.** `0x80B6BFC0`,
  `0x80B6C368` and `0x80B6C6A8` are data words in the REL image; the export
  prints no symbol for them.
- **Every float is dumped, not assumed.** `0x80B32820` = `10.0f`,
  `0x80B32830` = `4.0f`, `0x80B3285C` = `1.0f`, `0x80B32860` = `0.0f`,
  `0x80B3286C` = `10.0f`, `0x80B328EC` = `1.0f`, `0x80B32814` = `0.0f`.
- **The state descriptors `0x80B79538`/`0x80B795F8` live in BSS**, so they are
  entered by address; their `StateID` names are not recorded in this export.
  `0x80352A34`/`0x80352A74` are `main.dol` descriptors and are already used by
  `fn_80b26f90`/`fn_80b26fa0` in the wave-79 block.

### Deliberate skips

- **`0x80B23480` (180 bytes) and `0x80B22750` (128 bytes).** Both decompile
  into the GQR3 `ldexpf` artifact: the 16-bit angle at `+0x5BC`/`+0x550` is
  converted with a quantisation scale that lives in a *runtime* GQR register,
  and Ghidra marks the result unreachable. The scale is not present in the
  export or in the image, so the conversion cannot be reconstructed honestly;
  only the increments (`0x300`, `0x200`) and the `0x80237BD0` call are certain.
- **`0x80B23890` (156 bytes).** The export prints
  `0x8023A0B0(actor + 0x540)` with one argument while the value loaded
  immediately before is never used, so the clip name is silently dropped. A
  translation would have to invent the register, so the body is left for a pass
  with disassembly-level evidence.
- **The rest of the bucket stays untranslated: 78 functions.** The largest
  untouched runs after this pass are `0x80B20010..0x80B20860` (splits around
  the claimed `0x80B201E0..0x80B20860` helpers),
  `0x80B21660..0x80B21AC0`, `0x80B23200..0x80B23F00` (the `Wm_en_pakkun`
  state machine, of which only the four helpers above are claimed),
  `0x80B25280..0x80B266E0` and `0x80B27980..0x80B2A360`.
- **Nothing was claimed "by shape alone".** The six `low` rows are the 48-byte
  forwarders; their argument reshuffle is literal, but neither the target's
  role nor the hidden `r12` this-adjust pointer is established, and every body
  carries the same `// partial:` note.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B20980` | `fn_80b20980` | 48 | low |
| `0x80B209B0` | `fn_80b209b0` | 48 | low |
| `0x80B209E0` | `fn_80b209e0` | 212 | high |
| `0x80B20DE0` | `fn_80b20de0` | 72 | medium |
| `0x80B20E60` | `fn_80b20e60` | 40 | medium |
| `0x80B20E90` | `fn_80b20e90` | 240 | medium |
| `0x80B20F80` | `fn_80b20f80` | 292 | medium |
| `0x80B210D0` | `fn_80b210d0` | 308 | medium |
| `0x80B214E0` | `fn_80b214e0` | 88 | high |
| `0x80B21600` | `fn_80b21600` | 48 | low |
| `0x80B21630` | `fn_80b21630` | 48 | low |
| `0x80B21660` | `fn_80b21660` | 132 | high |
| `0x80B21AC0` | `fn_80b21ac0` | 204 | medium |
| `0x80B21BD0` | `fn_80b21bd0` | 276 | high |
| `0x80B21CF0` | `fn_80b21cf0` | 220 | medium |
| `0x80B21DF0` | `fn_80b21df0` | 192 | medium |
| `0x80B21EB0` | `fn_80b21eb0` | 132 | high |
| `0x80B22050` | `fn_80b22050` | 88 | high |
| `0x80B22170` | `fn_80b22170` | 48 | low |
| `0x80B221A0` | `fn_80b221a0` | 48 | low |
| `0x80B221D0` | `fn_80b221d0` | 100 | high |
| `0x80B222E0` | `fn_80b222e0` | 172 | high |
| `0x80B22540` | `fn_80b22540` | 264 | medium |
| `0x80B22680` | `fn_80b22680` | 112 | medium |
| `0x80B23200` | `fn_80b23200` | 68 | medium |
| `0x80B232B0` | `fn_80b232b0` | 88 | high |
| `0x80B235D0` | `fn_80b235d0` | 52 | high |
| `0x80B23610` | `fn_80b23610` | 52 | high |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  3,692 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave94` prints
  `wave94-enemies.csv: 28 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The only remaining workspace errors are two
  `write_i32` lookups in `src/game/stage.rs`, which belongs to another Wave 94
  lane.
