# Coverage - Wave 101 (`game::worldmap`, `0x80140000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); the bodies reuse the module's already-verified `R2_*` and
`SDA_*` constants instead of restating float values. The three `0x80354E50`
tables are byte/word arrays indexed by the player byte at `+0x38D`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 28 | 4,444 | 17 | 11 |

Every row is `translated`; no body is `stub`, `partial` or `low` confidence.

## What this pass covers

- **`0x80144280`/`0x801442B0`/`0x801446E0` - the `0x2D08`-byte map object.**
  `0x80144280` allocates the object through `0x801628C0` and calls the
  constructor with the new pointer still in `r3`. The constructor stores
  `0x80325420` at `+0x60` and `0x80325878` at `+0x14FC`, then initialises
  eighteen embedded records: twelve with the shared `0x80329958` vtable and the
  six cleared fields at `+0x114..+0x124`, four with `0x80308A80` and no cleared
  fields (`+0x18FC`, `+0x2538`, `+0x264C`, `+0x2A94`), and two with
  `0x80310ED8` (`+0x2804`, `+0x292C`). It then seeds the `+0x2A60` controller
  through `0x8005FBE0`/`0x800D6CC0`, registers with `0x800646E0`, copies the
  `+0x04 & 0xF` variant into `+0x38D`, resolves `+0x1090`/`+0x14D4` through the
  `0x80354E60`/`0x80354E70` tables, handles the special variant (bit `3` of
  `0x80354E80`, writing `r13 - 0x5704`'s `+0x6C` byte), binds the player slot
  through `0x8005F8C0` and finishes with `0x802DCDBC`. `0x801446E0` reverses the
  same list, restores the vtable, releases the `+0x2A60` controller through
  `0x800D6E00` and optionally frees the storage with `0x80162920`.
- **`0x80144110` - partner snap.** Finds the attached partner with
  `0x8012DBE0`; when `arg3 != 0` and neither `+0x1090` variant is `3`, it copies
  the partner's `+0xAC..+0xB4` vector into a scratch buffer, probes the ground
  with `0x80075CA0`, converts the animation entry's `+0x1C` word through the
  signed `0x43300000` idiom (`signed_to_double` / `4096.0`) and subtracts `2.0`
  before writing the vector back. Otherwise the partner's `+0x60` `+0x118` slot
  runs, and every path falls through to `0x800564F0`.
- **`0x80144B20`/`0x80144D50`/`0x80145040` - per-player state copy.**
  `0x80144D50` is the `r13 - 0x6674` mode word. `0x80144B20` reads the `+0x60`
  `+0xE0` slot, then either copies `+0x1090`/flags into
  `0x80354E70`/`0x80354E80`, or (mode `3`) resolves the partner word `+0x1564`
  through `0x80139950` and `0x8005FC20` and posts sound `0xC5`; both paths write
  the `0x80354E50` byte and the two `r13 - 0x59D0`/`r13 - 0x59C8` halfwords.
  `0x80145040` temporarily copies `+0x38E` from the partner, runs
  `0x80046AB0(object)` and restores it.
- **`0x801451F0` - per-frame gate.** Drives the `+0x2A7C` idle counter from
  `|+0x10C| < 1.0`, then runs the `7`/`0x53`/`0x8B`/`0x8C`/`0x4B` status-bit
  ladder, the eight state gates
  (`0x80132440`, `0x80133B60`, `0x801341B0`, `0x80134BC0`, `0x801285D0`,
  `0x80147040`, `0x8012FA00`, `0x8013BA80`), the `+0x1464` `+0x28`/`+0x10`
  child probe, the `+0x60` `+0x294` post of `0x803762B0`, the `+0x60` `+0x404`
  slot with `4.0`, and the `0x80137550` timed state.
- **`0x8014A340..0x8014A400` - per-map block getters.** Thirteen four-
  instruction functions returning the `0x40`-stride data blocks at
  `0x8035425C..0x80354B2C` (eleven contiguous, then `0x8035459C`/`0x803545DC`).
  Only the returned address is observable; the block contents are not part of
  the body.
- **`0x8014AAF0` - ice map-object model loader.** The `+0x3B0` variant of wave
  94's `+0x3EC` loader: opens `"g3d_ice_brres"` through the `r13 - 0x5648`
  loader and `r13 - 0x62E0` name table, builds the `0x20`-byte model record at
  `+0x3B4`, resolves the layout into `+0x3F4`, fills `+0x3F8` and closes the
  `+0x394` name component with `0x8016A8E0`.
- **`0x8014B000..0x8014B040` - exit thunks.** Three identical forwards to
  `0x8014AFC0` (`wm_map_object_exit_if_flagged`) with the object still in `r3`.
- **`0x8014FC50..0x8014FF00` - `+0x14D8` child state handlers.**
  `0x8014FC50` moves the child to state `99` through its `+0x64` slot and stores
  `5` at `+0x151C`; `0x8014FE60` clears `+0x17C` bit `3`, calls the `+0x68`
  slot with `0.0`, runs `0x8014F160` and stores `0x1E` at `+0x153C`;
  `0x8014FF00` sets `+0x17C` bit `4`, calls the `+0x64` slot with state `0x61`
  and clears `+0x1534`/sets `+0x1538 = 0x1E`.
- **Shared notation.** `0x80056CF0` is the actor status-bit test,
  `0x80056CB0`/`0x80056C70` clear/set one bit, `0x802D7AA0` initialises an
  embedded record and `0x802D7B20` releases it. `0x8014F160` and
  `0x80147040` were translated earlier and are called by name.
- Already-translated neighbours are reused instead of re-declared:
  [`wm_anim_table_entry`] (`0x80142510`), [`wm_map_object_exit_if_flagged`]
  (`0x8014AFC0`) and [`fn_80147040`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80144110` | `fn_80144110` | 368 | medium |
| `0x80144280` | `fn_80144280` | 48 | high |
| `0x801442B0` | `fn_801442b0` | 1000 | medium |
| `0x801446E0` | `fn_801446e0` | 956 | high |
| `0x80144B20` | `fn_80144b20` | 552 | medium |
| `0x80144D50` | `fn_80144d50` | 8 | high |
| `0x80145040` | `fn_80145040` | 84 | medium |
| `0x801451F0` | `fn_801451f0` | 584 | medium |
| `0x8014A340` | `fn_8014a340` | 12 | high |
| `0x8014A350` | `fn_8014a350` | 12 | high |
| `0x8014A360` | `fn_8014a360` | 12 | high |
| `0x8014A370` | `fn_8014a370` | 12 | high |
| `0x8014A380` | `fn_8014a380` | 12 | high |
| `0x8014A390` | `fn_8014a390` | 12 | high |
| `0x8014A3A0` | `fn_8014a3a0` | 12 | high |
| `0x8014A3B0` | `fn_8014a3b0` | 12 | high |
| `0x8014A3C0` | `fn_8014a3c0` | 12 | high |
| `0x8014A3D0` | `fn_8014a3d0` | 12 | high |
| `0x8014A3E0` | `fn_8014a3e0` | 12 | high |
| `0x8014A3F0` | `fn_8014a3f0` | 12 | high |
| `0x8014A400` | `fn_8014a400` | 12 | high |
| `0x8014AAF0` | `fn_8014aaf0` | 252 | medium |
| `0x8014B000` | `fn_8014b000` | 32 | medium |
| `0x8014B020` | `fn_8014b020` | 32 | medium |
| `0x8014B040` | `fn_8014b040` | 32 | medium |
| `0x8014FC50` | `fn_8014fc50` | 108 | medium |
| `0x8014FE60` | `fn_8014fe60` | 116 | high |
| `0x8014FF00` | `fn_8014ff00` | 116 | medium |

## Deliberate skips

The bucket had 45 untranslated entries at the start of the pass. Twenty-eight
were written; the remaining seventeen are listed here.

- **Paired-single / GQR-reconstruction records: `0x801406F0` (828),
  `0x80140FF0` (336), `0x80141A60` (388), `0x80142030` (1,012),
  `0x80142660` (2,236), `0x801430E0` (516), `0x80145650` (816),
  `0x801469D0` (812), `0x801472F0` (244), `0x80147700` (276),
  `0x8014AC10` (412), `0x8014CDD0` (216), `0x8014F630` (528).** These exports
  quantise incoming paired-single registers (`unaff_GQR0`/`unaff_GQR3` with
  `ldexpf` and `CONCAT` over the 16-bit payloads) or feed `psq_l`-style stack
  slots, so the float operands cannot be recovered from the C alone. This is the
  same exclusion wave 100 applied to its GQR records.
- **`0x801459B0` (168).** A three-vector product that Ghidra lowers through
  overlapping stack slots: the three `0x80145980` (`wm_actor_scaled_vector`)
  outputs and the three `0x800541C0` scale vectors alias (`local_28`/`local_1c`,
  `auStack_34`/`local_30`, `auStack_4c`/`local_44`), and the stored components
  (`[0] = local_1c[0]*local_28[0]`, `[1] = local_30*local_3c`,
  `[2] = local_44*local_50`) cannot be paired with a source vector without the
  instruction listing.
- **`0x801468D0` (256).** A tail adapter that recovers `this` from
  `0x802DCD74`'s 64-bit return and forwards the leftover `f1` register
  (`extraout_f1`) to the `+0x60` `+0x400` slot. The `extraout_f1` operand has no
  source in the C, so the forwarding body would be invented.
- **`0x8014A690` (48) and `0x8014A6C0` (48).** Both exports are
  `FUN_802DCBFC(param_2, param_2, param_1)`: the first argument is duplicated
  and the result is ignored, which is the decompiler's view of a register-pair
  or `__register_global_object`-style call. The true argument order cannot be
  proven from the C.
- No address is duplicated: all 28 rows were untranslated entries of the
  `0x80140000` bucket at the start of the pass.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`;
  the 28 sizes sum to 4,444 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` was run on the crate: at the end of the pass the only errors
  came from `src/game/bases.rs` (another lane). A standalone harness that
  compiles `src/platform.rs`, `src/engine/globals.rs` and
  `src/game/worldmap.rs` with `cargo check` reports zero errors, so the module
  itself type-checks; the pre-existing warnings are at lines 17956, 20314,
  34102, 34188 and 41738.
- `node tools/check-lane-csv.mjs wave101` reports `OK` for
  `ledger/agents/wave101-worldmap.csv`.

[`wm_anim_table_entry`]: crate::game::worldmap::wm_anim_table_entry
[`wm_map_object_exit_if_flagged`]: crate::game::worldmap::wm_map_object_exit_if_flagged
[`fn_80147040`]: crate::game::worldmap::fn_80147040
