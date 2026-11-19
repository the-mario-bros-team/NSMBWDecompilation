# Coverage - Wave 104 (`game::enemies`, the `0x80AE0000` bucket)

## How to read this file

This page records the Wave 104 `game::enemies` lane: 30 functions of the
`0x80AE0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs` (section 85 of that file). Every row below is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave104-enemies.csv`) names the Ghidra
C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

Before this pass the bucket held 160 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`, 74 of them already in
`ledger/functions.csv`; 30 are claimed here, leaving 56.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 6,184 | 13 | 17 |

No row is `low` confidence. The seventeen `medium` rows fall into three
groups: the two matrix/transform helpers whose holder roles come from the
identical sequence in [`fn_80b00460`]; the `daEnPata_c` binder and player tests
whose table indices and player offsets are inferred from call shape; and the
`daEnPuku_c` steps whose tilt tables and effect-block words are literal but
whose field roles are not named by the export.

## What this pass covers

- **Identity of the two blocks.** `0x80AE3080` (already translated) is
  `register_pata_fly_states`: it builds the four records whose `StateID`
  strings are `daEnPata_c::StateID_UDfly`, `_LRfly`, `_PataJ` and
  `_PataJTurn`, and it hands each one the same callback, `0x80AE33C0`. The
  constructor at `0x80AE3540` follows immediately and its binder loads
  `g3d/poltergeist.brres`, which is the Pata model. The second block is
  `daEnPuku_c`: `register_puku_states` (`0x80AE9990`, already translated)
  names the fifteen records at `0x80B771C0..0x80B7754C` and passes exactly the
  helper addresses claimed here (`0x80AEA530`, `0x80AEA590`,
  `0x80AEA5F0..0x80AEA620`).
- **`0x80AE33C0..0x80AE38D0`, the Pata fly-state plumbing.** The shared
  registration callback (the module-wide `0x8015F800`/`0x802B9280` pair), two
  48-byte `0x802DCBFC` forwarders, the `0x6C8`-byte constructor with class
  vtable `0x80B60468` (model holder `+0x528`, two matrices `+0x548`/`+0x588`,
  animation holder `+0x5C8`, three `0x38`-byte sub-objects at
  `+0x600`/`+0x62C`/`+0x658`), the two-slot matrix release, and the placement
  that commits the same matrix twice, the second copy `32.0` units lower.
- **`0x80AE3C60..0x80AE4600`, the poltergeist binder and the contact tests.**
  The binder opens `g3d/poltergeist.brres`, binds `pol_block_hatena` to
  `+0x548` (id `0x128`), `pol_light` to `+0x588` (id `0x32C`),
  `pol_light_st` to `+0x658` and the light clips to `+0x5C8`/`+0x600`, all
  selected by the `+0x6B0` variant through the four pointer arrays at
  `0x80B603E0`, `0x80B603E8`, `0x80B60418` and `0x80B60420`. The two tests are
  the 128-unit contact box (`0x80AE3ED0`, with an optional rectangle from
  `0x8008E890`) and the fixed 48-unit player box (`0x80AE4080`). The two
  steps that follow (`0x80AE4530`, `0x80AE4600`) switch between
  `0x80B77010` and `0x80B77050`.
- **`0x80AEA530..0x80AEA8E0`, the Puku descriptor skeleton.** The registration
  callback, the four constant producers that supply the extra death-record
  words (`0x80358004` for `Ice`, `0x80358104` for `EatOut`, `0x80357D74` for
  `DieOther`, `0x80357BF4` for `DieFall` - the order the registration table
  uses), the five-link `+0x30` chain walker with its recursion, the
  `:`-suffix name predicate and two 48-byte forwarders.
- **`0x80AE6560..0x80AE8F40`, the Puku steps.** The 64-frame stage-effect
  timer, the damage step (sound `0x17B`, side from the contact's x speed,
  `+0x414` record through [`fn_80ae6880`]), the fifteen-word effect block
  (mode-selected `9.0`/`15.6`/`10.4` offset and `1.2`/`1.8` scale), the
  pursuit pair (`0x80AE7CA0` aims and scales, `0x80AE7E00` reports the
  distance), the tilt ramp (`0x80AE7F10`, two mode tables plus a clamped
  table, gated by the global frame counter `0x8042A200`), and the state
  handlers `0x80AE8A10` (`WaterMove`), `0x80AE8B80` (clamped tilt),
  `0x80AE8CF0` (`LrTurnSwim`/`LrSrcSwim`) and `0x80AE8F40`
  (`EscapeSwim`/`HellSrcRetur`).

### Deliberate skips

- **`0x80AEA590` (92 bytes).** The second registration callback, used by the
  four death records. It is byte-for-byte the body of `0x80AEA530`, so it is
  documented in the module comment but no duplicate body is claimed.
- **`0x80AE69E0` (444 bytes).** A contact callback that recovers both its
  actor and its parent from the `0x802DCD68` save fragment's 64-bit result
  (`(int)uVar7` and `(int)(uVar7 >> 32)` are used as two different objects)
  and then dispatches three unknown virtual slots (`+0x6C`, `+1000`). The
  register recovery cannot be tied to a Rust parameter list without guessing,
  so it is left for a pass with scaled-register evidence.
- **`0x80AE63D0` (388 bytes).** Its final three stores read `local_2c`,
  `local_1c` and `local_c`, which lie outside the 12-byte buffer filled by
  `0x80168180`, so the three words written to `+0x60C..+0x614` have no
  recoverable source.
- **`0x80AE4A50` (660 bytes).** Three `unaff_GQR3` merge blocks convert the
  signed halfwords `+0x6BA`/`+0x6BC` to `double`; the folded values feed both
  `0x80237BD0` and `0x80237C40`, and the branch that chooses between them
  depends on a local written by the quantiser. Only the instruction sequence
  is certain, so it is not claimed.
- **The remaining 56 functions of the bucket.** The obvious next candidates
  are the rest of the `daEnPata_c` per-frame block
  (`0x80AE0780`, `0x80AE2250`) and the `daEnPuku_c` handlers that were not
  part of this pass (`0x80AE7CA0`'s neighbours `0x80AE8140`, `0x80AE8440`,
  `0x80AE8610`, `0x80AE8780`, `0x80AE8E40`, and the `0x80AECxxx` tail).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AE33C0` | `fn_80ae33c0` | 88 | high |
| `0x80AE34E0` | `fn_80ae34e0` | 48 | high |
| `0x80AE3510` | `fn_80ae3510` | 48 | high |
| `0x80AE3540` | `fn_80ae3540` | 264 | high |
| `0x80AE3860` | `fn_80ae3860` | 84 | high |
| `0x80AE38D0` | `fn_80ae38d0` | 404 | medium |
| `0x80AEA530` | `fn_80aea530` | 88 | high |
| `0x80AEA5F0` | `fn_80aea5f0` | 12 | high |
| `0x80AEA600` | `fn_80aea600` | 12 | high |
| `0x80AEA610` | `fn_80aea610` | 12 | high |
| `0x80AEA620` | `fn_80aea620` | 12 | high |
| `0x80AEA710` | `fn_80aea710` | 224 | medium |
| `0x80AEA7F0` | `fn_80aea7f0` | 136 | medium |
| `0x80AEA8B0` | `fn_80aea8b0` | 48 | high |
| `0x80AEA8E0` | `fn_80aea8e0` | 48 | high |
| `0x80AE3C60` | `fn_80ae3c60` | 624 | medium |
| `0x80AE3ED0` | `fn_80ae3ed0` | 432 | medium |
| `0x80AE4080` | `fn_80ae4080` | 320 | medium |
| `0x80AE4530` | `fn_80ae4530` | 172 | medium |
| `0x80AE4600` | `fn_80ae4600` | 124 | high |
| `0x80AE6560` | `fn_80ae6560` | 88 | medium |
| `0x80AE6D70` | `fn_80ae6d70` | 332 | medium |
| `0x80AE6F10` | `fn_80ae6f10` | 436 | medium |
| `0x80AE7CA0` | `fn_80ae7ca0` | 352 | medium |
| `0x80AE7E00` | `fn_80ae7e00` | 132 | medium |
| `0x80AE7F10` | `fn_80ae7f10` | 524 | medium |
| `0x80AE8A10` | `fn_80ae8a10` | 280 | medium |
| `0x80AE8B80` | `fn_80ae8b80` | 284 | medium |
| `0x80AE8CF0` | `fn_80ae8cf0` | 276 | medium |
| `0x80AE8F40` | `fn_80ae8f40` | 280 | medium |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  6,184 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave104` reports
  `wave104-enemies.csv: 30 rows` and `OK` with no problem naming this CSV.
- `cargo check --message-format=short` emits no error or warning naming
  `src/game/enemies.rs`. The remaining errors at the time of this pass belong
  to other lanes' files (`src/runtime/bt.rs`, `src/audio/ax.rs`).
- The module header was updated: a new cluster item 85, the cluster count
  (eighty-four to eighty-five), the function totals (2,244 to 2,274 public
  translations, 2,173 to 2,203 `fn_<addr>` bodies) and the record list.
