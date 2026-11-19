# Coverage - Wave 107 (`game::enemies`, the `0x80AA0000` bucket)

## How to read this file

This page records the Wave 107 `game::enemies` lane: 27 functions of the
`0x80AA0000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 88 of that file. Every row of the
machine-readable mirror (`ledger/agents/wave107-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

The three blocks are:

- `0x80AA95E0..0x80AA9E40` - the `daEnKiller_c` move-state handlers plus their
  shared tail, the lane predicate, the empty hook and one class-vtable reset.
- `0x80AAA330..0x80AAAF20` - the spawn-slot test with its two projectile
  spawners, the two state-arm callbacks and the six per-record constant
  producers of `register_killer_states`.
- `0x80AAB010..0x80AAB1E0` and `0x80AABEF0..0x80AAC270` - the descriptor
  skeleton (owner walk, name predicate, two forwarders) and the
  `killer_houdai` / `killer_mag_houdai` platform binders.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 4,088 | 14 | 11 |

Two rows are `low` confidence: the 48-byte `0x802DCBFC` forwarders
(`0x80AAB1B0`, `0x80AAB1E0`), whose hidden `r12` this-adjust pointer cannot be
modelled from a C export.

## What this pass covers

- **`0x80AA95E0..0x80AA9E40`, the four move-state middle handlers.** The
  move-state vtable `0x80B53FFC` - installed by `register_killer_states`
  (`0x80AAA600`, wave 4) on the `StateID_MagMove`, `Move`, `SearchMove` and
  `SearchMagM` records - names three callbacks per record. The record tables at
  `0x80B53D18`, `0x80B53D3C`, `0x80B53D60` and `0x80B53D84` name those
  addresses literally; the middle callback of each record is translated here:
  `0x80AA9790` (carry/roll step), `0x80AA9900` (spin, `Wm_ob_cmnspark` and
  sound `0x1BF`), `0x80AA9AE0` (countdown plus `0x80AA93F0` lane run) and
  `0x80AA9C70` (fly to the saved target pair `+0x9A0`/`+0x9A4`). `0x80AA95E0`
  is the shared probe tail (it calls the wave-90 side probe `0x80AA8FE0` and
  the smoke/search emitter `0x80AA8DF0`), `0x80AA9630` the player-lane
  predicate that section 71 already called by address as `W90_FN_80AA9630`,
  `0x80AA9720` the shared empty hook, and `0x80AA9E40` the class-vtable slot
  `+0x184` reset (base bookkeeping `0x800A16A0`, animator stop, speed triple
  and one animation frame).
- **`0x80AAA330..0x80AAAF20`, the spawn and descriptor furniture.**
  `0x80AAA330` tests a slot id against the `u16` live count at
  `0x80429F10 + 0x148`; `0x80AAA360` and `0x80AAA400` create object kinds
  `0x7F` and `0x80` through `0x80064610` behind that test with a one-byte
  output flag. The two state-arm callbacks sit at `+0x08` of the move and death
  state vtables (`0x80B53FFC`, `0x80B53FC8`). The six 12-byte producers are
  mapped to their records by the store that consumes each result:
  `0x80AAAED0` -> `DieFall` (`0x80357BF4`), `0x80AAAEE0` -> `DieFumi`
  (`0x80357BB4`), `0x80AAAEF0` -> `DieOther` (`0x80357D74`), `0x80AAAF00` ->
  `EatOut` (`0x80358104`), `0x80AAAF10` -> `DieGoal` (`0x80357D34`) and
  `0x80AAAF20` -> `DieBigFall` (`0x80357C34`).
- **`0x80AAB010..0x80AAB1E0`, the descriptor skeleton.** The five-level
  `+0x30` owner walk with its recursive tail, the `:`-suffix name predicate at
  `+0x1C` of both state vtables (the same `0x802E1C40`/`0x802E1AB4` pair that
  every other name predicate of this file uses), and the two `0x802DCBFC`
  forwarders at `+0x2C` (plus `+0x30` in the death table).
- **`0x80AABEF0..0x80AAC270`, the `killer_houdai` platform.**
  `0x80AABEF0` resets the `+0x528` model-holder context, switches on `+0xE18`
  and finishes the holder. `0x80AABF70` builds the multi-part model
  (`g3d/killer_houdai.brres` under the stem `killer_houdai`, nodes
  `killer_houdai_a`/`killer_houdai_b` and one `killer_houdai_c` per variant at
  `+0x54C`/`+0x60C + i * 0x40`, clip `shot` with a `1.0` blend), `0x80AAC110`
  the single-part variant, and `0x80AAC270` the `killer_mag_houdai` model,
  which - unlike the other two - has no `0x80064BD0` finish call on `+0x58C`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AA95E0` | `fn_80aa95e0` | 72 | high |
| `0x80AA9630` | `fn_80aa9630` | 240 | medium |
| `0x80AA9720` | `fn_80aa9720` | 4 | high |
| `0x80AA9790` | `fn_80aa9790` | 264 | high |
| `0x80AA9900` | `fn_80aa9900` | 412 | medium |
| `0x80AA9AE0` | `fn_80aa9ae0` | 336 | medium |
| `0x80AA9C70` | `fn_80aa9c70` | 464 | medium |
| `0x80AA9E40` | `fn_80aa9e40` | 172 | medium |
| `0x80AAA330` | `fn_80aaa330` | 44 | high |
| `0x80AAA360` | `fn_80aaa360` | 128 | medium |
| `0x80AAA400` | `fn_80aaa400` | 128 | medium |
| `0x80AAAE10` | `fn_80aaae10` | 88 | high |
| `0x80AAAE70` | `fn_80aaae70` | 92 | high |
| `0x80AAAED0` | `fn_80aaaed0` | 12 | high |
| `0x80AAAEE0` | `fn_80aaaee0` | 12 | high |
| `0x80AAAEF0` | `fn_80aaaef0` | 12 | high |
| `0x80AAAF00` | `fn_80aaaf00` | 12 | high |
| `0x80AAAF10` | `fn_80aaaf10` | 12 | high |
| `0x80AAAF20` | `fn_80aaaf20` | 12 | high |
| `0x80AAB010` | `fn_80aab010` | 224 | medium |
| `0x80AAB0F0` | `fn_80aab0f0` | 136 | high |
| `0x80AAB1B0` | `fn_80aab1b0` | 48 | low |
| `0x80AAB1E0` | `fn_80aab1e0` | 48 | low |
| `0x80AABEF0` | `fn_80aabef0` | 128 | high |
| `0x80AABF70` | `fn_80aabf70` | 408 | medium |
| `0x80AAC110` | `fn_80aac110` | 348 | medium |
| `0x80AAC270` | `fn_80aac270` | 232 | medium |

### Deliberate skips

- **`0x80AA93F0` (496 bytes).** The search-state movement step. Its export is
  dominated by `unaff_GQR0`/`unaff_GQR3` quantisation noise and four removed
  unreachable blocks around `0x80AA9500..0x80AA95BC`; the arithmetic cannot be
  reproduced exactly, so the body is skipped and the two callers
  (`0x80AA9AE0`) call the address. The constant is `W107_FN_80AA93F0`.
- **`0x80AAA3E0` and `0x80AAA480` (20 bytes each).** Both exports are
  `halt_baddata`: Ghidra truncated the control flow at the first instruction,
  so there is no evidence to translate. They sit between the two spawners and
  the state tables and are almost certainly copies of `0x80AAA360`/
  `0x80AAA400` with other object kinds, but "almost certainly" is not a body.
- **`0x80AA9730`, `0x80AA9780`, `0x80AA98A0`, `0x80AA98F0`, `0x80AA9AA0`,
  `0x80AA9AD0`, `0x80AA9C30`, `0x80AA9C60`.** The record tables at
  `0x80B53D18..0x80B53DA8` name these as the flanking callbacks of the four
  move records, but Ghidra created no function row and no export file for
  them, so they cannot be translated with evidence. Their identity as
  record callbacks is recorded in the coverage of the middle handlers.
- **`0x80AA9EF0` and `0x80AA9F00`.** Class-vtable slots `+0x18C` and `+0x188`,
  immediately after `0x80AA9E40`; they have no `functions.tsv` row either.
- **`0x80AA8470` (592 bytes).** The shared `daEnKiller_c` per-frame body, which
  every move handler calls. Its export is GQR-damaged and has removed blocks,
  so it is called by address (`W107_FN_80AA8470`). It remains the largest
  obvious target for a later `0x80AA0000` pass.
- **`0x80AA35E0..0x80AA4070`, the angle and trail helpers.** Every function
  of the `0x80AA35E0..0x80AA4070` run quantises its yaw through
  `0x80237BD0`/`0x80237C40` with `GQR3` scale factors that Ghidra leaves
  unbound, and `0x80AA39D0` additionally ends in `halt_baddata`; no body was
  written. This is the same reason wave 106 skipped the firebar and fire-snake
  angle writers.
- **`0x80AAC360`, `0x80AAC610`, `0x80AAC870`.** The three matrix writers of the
  `0x80AA` tail. Their `0x801C0BD0`/`0x801C0540` calls are printed with
  truncated or `GQR`-quantised argument lists at the first site of each body,
  so the matrices cannot be rebuilt honestly.
- **The rest of the bucket.** 54 of the bucket's 188 functions remain
  untranslated after this pass: `0x80AA0080..0x80AA8470` (the `killer_mag` /
  `killer` object layer that wave 90 opened), the two `halt_baddata` stubs
  above, `0x80AAC360..0x80AACFA0` (the matrix writers and the next block) and
  `0x80AAD010..0x80AAFExx`, which opens with the 1,140-byte function at
  `0x80AAD010` - the natural next target, since it is the constructor-shaped
  body of the `killer_houdai` state machine whose binders this pass translated.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  4,088 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 27 was
  already in `ledger/functions.csv` (107 of the bucket's 188 rows were already
  translated, 81 were not).
- `node tools/check-lane-csv.mjs wave107` prints
  `wave107-enemies.csv: 27 rows` and `OK` with no problem naming this CSV.
- `cargo check --message-format=short` emits no error or warning naming
  `src/game/enemies.rs`. The lane's own isolated type-check (the module plus
  `src/platform.rs` in a scratch crate) also reports no warning inside the new
  section.
- The module header was updated: a new cluster item 88, the cluster count
  (eighty-seven to eighty-eight), the function totals (2,332 to 2,359 public
  translations, 2,261 to 2,288 `fn_<addr>` bodies) and the record list. The
  stale "**Not translated** in this pass" note on `W90_FN_80AA9630` was
  corrected.
