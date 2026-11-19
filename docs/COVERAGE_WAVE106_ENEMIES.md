# Coverage - Wave 106 (`game::enemies`, the `0x80A50000` bucket)

## How to read this file

This page records the Wave 106 `game::enemies` lane: 30 functions of the
`0x80A50000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 87 of that file. Every row of the
machine-readable mirror (`ledger/agents/wave106-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

The two blocks are:

- `0x80A55550..0x80A558A0` - a seventh `0x548`-byte descriptor shell whose
  class is unnamed in this run.
- `0x80A570A0..0x80A58960` - the first half of `daEnFireSnake_c`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 4,964 | 13 | 12 |

Five rows are `low` confidence: the four 48-byte argument-swapping forwarders
(`0x80A55840`, `0x80A55870`, `0x80A571C0`, `0x80A571F0`), whose hidden `r12`
this-adjust pointer cannot be modelled from a C export, and the 16-byte
indirect dispatch at `0x80A578B0`, whose arguments Ghidra does not print.

## What this pass covers

- **Identity of the two blocks.** The `0x548`-byte shell at `0x80A55550` opens
  with the same seven-entry skeleton as the six earlier shells of this bucket
  (deleting destructor, constant producer, `+0x30` chain walk, `:`-suffix name
  predicate, two 48-byte forwarders, constructor), but its constructor installs
  vtable `0x80B45E88` and opens no model archive, so the class cannot be named
  from this run. The second block is `daEnFireSnake_c`: the name is literal in
  `register_fire_snake_states` (`0x80A5A350`, wave 4), which registers the
  fourteen `daEnFireSnake_c::StateID_*` records and seeds the class defaults at
  `0x80B716E0..0x80B716F4` that `0x80A578C0` and `0x80A581E0` read.
  `0x80A57B50` opens `g3d/fire_snake.brres` with the nodes `fire_snake_head`,
  `fire_snake_body` and `skl_root` and the clips `R_jump_st` and `0x80B468C4`,
  which fixes the class a second time.
- **`0x80A55550..0x80A558A0`, the seventh descriptor shell.** The destructor
  (`0x8015F800` clear, `0x802B9280` free), the constant producer returning
  `0x80358104` (a record shared with the `wanwan` block of header item 77), the
  five-step `+0x30` owner walk with its recursion, the colon-trim name
  comparison over `0x802E1C40`/`0x802E1AB4`, the two `0x802DCBFC` forwarders and
  the `0x548`-byte constructor with vtable `0x80B45E88` and the model holder at
  `+0x524`.
- **`0x80A570A0..0x80A578B0`, the fire-snake object layer.** The class deleting
  destructor and two forwarders; the `0x14E0`-byte constructor with the
  `8 x 0x29` node array at `+0x638`, the `4 x 0x260` element array at `+0x7B8`,
  the three `0x80329958` interface holders and the `0x80B46BC0` interface at
  `+0x14D0` with its `+0x14D4` back-pointer; the element constructor,
  destructor, `fire_snake_body` binder, per-index placement (the
  `0x80B2CFB8` vertical step), matrix rebuild (`0x80064AF0` +
  `0x801C0BD0`) and the `Wm_en_firesnkspark02` effect; and the 16-byte
  `+0x04`-owner dispatch.
- **`0x80A578C0..0x80A58960`, the fire-snake spawn and frame layer.** The spawn
  initialiser (facing byte at `+0x348`, six `0x80B716E0..` defaults, the
  `0x8008C3E0`/`0x8006D090` sub-objects, the 41-sample position ring at
  `+0x638..+0x77C`, the four element placements, the three
  `0xFFFFFFFF` timer words and the `+0x04`-gated state switch to
  `0x80B71948`/`0x80B71748`); the `fire_snake`/`fire_snake_head`/`R_jump_st`
  binder; the `+0x79C` dispatch; the `+0x7C` matrix rebuild with the three
  `+0x100` angles; the `skl_root` probe and `Wm_en_firesnkspark01` effect; the
  head-bob clamp writing `+0x7A0..+0x7AC`; the holder resets and releases; the
  41-sample ring shift; the ring-to-element copy; and the `+0x1154` countdown
  seed.

### Deliberate skips

- **`0x80A53850` (368 bytes).** A `dossun`-family function that reads the
  `0x8042A100` ground record and rewrites the fall speed from an
  `extraout_f1` pseudo-register. Ghidra leaves the floating-point result
  unbound, so the clamp cannot be reconstructed honestly; it is left for a
  later pass.
- **`0x80A55CB0` (80 bytes) and `0x80A55DE0` (56 bytes).** These belong to the
  larger firebar actor that follows (`+0x5EC`, `+0x6DC`), not to the `0x548`
  shell and not to `daEnFireSnake_c`, so they would be orphan rows in this
  lane.
- **`0x80A56470..0x80A56DB0`, the firebar actor.** Eight functions whose
  exports are dominated by `unaff_GQR0`/`unaff_GQR3` register artifacts and
  `ldexpf` reconstruction noise. Their real arithmetic (the `0x2480` angle
  accumulator, the `0x2484` node count and the `Wm_en_firebar*` emitters) is
  visible, but the quantised conversions cannot be reproduced exactly, so no
  body was written. Only the two empty 4-byte hooks (`0x80A56B00`,
  `0x80A56DB0`) would have been safe, and they are not worth a row on their
  own.
- **`0x80A584E0` (232 bytes).** The `0x80064740`/`0x801611C0` angle writer at
  `+0x780..+0x784`; its export is again `GQR`-quantised, so it is deferred.
- **`0x80A585D0` and `0x80A58610`.** The remaining two helpers of the
  fire-snake tail (`0x800B2EE0(3)` predicate and the `+0x164..+0x170` quad
  writer). They are clean but lie outside the contiguous run this pass stops
  at; the next `0x80A50000` pass should start there.
- **The rest of the bucket.** 52 of the bucket's 181 functions remain
  untranslated. The obvious next candidates are the fire-snake tail
  (`0x80A584E0..0x80A58960`), the firebar block
  (`0x80A55CB0..0x80A56DB0`), the `0x80A58A30..0x80A59F00` run and the
  `0x80A5AEF0..0x80A5F7E0` tail.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A55550` | `fn_80a55550` | 92 | high |
| `0x80A555B0` | `fn_80a555b0` | 12 | high |
| `0x80A556A0` | `fn_80a556a0` | 224 | medium |
| `0x80A55780` | `fn_80a55780` | 136 | medium |
| `0x80A55840` | `fn_80a55840` | 48 | low |
| `0x80A55870` | `fn_80a55870` | 48 | low |
| `0x80A558A0` | `fn_80a558a0` | 84 | medium |
| `0x80A570A0` | `fn_80a570a0` | 88 | high |
| `0x80A571C0` | `fn_80a571c0` | 48 | low |
| `0x80A571F0` | `fn_80a571f0` | 48 | low |
| `0x80A57220` | `fn_80a57220` | 480 | high |
| `0x80A57400` | `fn_80a57400` | 180 | high |
| `0x80A574C0` | `fn_80a574c0` | 180 | high |
| `0x80A575C0` | `fn_80a575c0` | 212 | high |
| `0x80A576A0` | `fn_80a576a0` | 216 | medium |
| `0x80A57780` | `fn_80a57780` | 136 | high |
| `0x80A57810` | `fn_80a57810` | 156 | medium |
| `0x80A578B0` | `fn_80a578b0` | 16 | low |
| `0x80A578C0` | `fn_80a578c0` | 648 | medium |
| `0x80A57B50` | `fn_80a57b50` | 472 | high |
| `0x80A57D30` | `fn_80a57d30` | 20 | medium |
| `0x80A57F60` | `fn_80a57f60` | 52 | medium |
| `0x80A57FA0` | `fn_80a57fa0` | 260 | medium |
| `0x80A580B0` | `fn_80a580b0` | 292 | medium |
| `0x80A581E0` | `fn_80a581e0` | 176 | medium |
| `0x80A58290` | `fn_80a58290` | 112 | medium |
| `0x80A58300` | `fn_80a58300` | 88 | high |
| `0x80A58360` | `fn_80a58360` | 176 | high |
| `0x80A58410` | `fn_80a58410` | 208 | high |
| `0x80A58960` | `fn_80a58960` | 56 | high |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  4,964 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was
  already in `ledger/functions.csv` (99 of the bucket's 181 rows were already
  translated, 82 were not).
- `node tools/check-lane-csv.mjs wave106` prints
  `wave106-enemies.csv: 30 rows` and `OK` with no problem naming this CSV.
- `cargo check --message-format=short` emits no error or warning naming
  `src/game/enemies.rs`. The remaining errors at the time of this pass belong
  to other lanes' files (`src/runtime/input.rs`, `src/runtime/bt.rs`,
  `src/engine/main_loop.rs`, `src/runtime/cpp.rs`).
- The module header was updated: a new cluster item 87, the cluster count
  (eighty-six to eighty-seven), the function totals (2,302 to 2,332 public
  translations, 2,231 to 2,261 `fn_<addr>` bodies) and the record list.
