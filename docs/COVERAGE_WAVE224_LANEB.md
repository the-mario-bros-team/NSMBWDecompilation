# Wave 224, lane B - `src/game/stage.rs`, addresses `0x800A74C0..0x800AA770`

Ten entries of `work/ghidra/main-gb/export/functions.tsv` in the `0x800A0000`
band of the `game::stage` ledger module. All ten are translated; none deferred.
Bytes translated: **748**.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x800A74C0` | 120 | 120 (`blr` at `0x800A7534`) | submit effect `0xDD` at a three-float position |
| `0x800A7550` | 8 | 8 (tail `b` at `0x800A7554`, no `blr`) | `extsb` the fifth argument and jump to `0x800A7560` |
| `0x800A7860` | 48 | 44 (`blr` at `0x800A788C`) + 4 bytes padding | player lookup then `0x801474B0` |
| `0x800A8720` | 12 | 12 (`blr` at `0x800A8728`) | return the shared type-info record `0x80377A04` |
| `0x800A8750` | 12 | 12 (`blr` at `0x800A8758`) | same type-info record |
| `0x800A87E0` | 12 | 12 (`blr` at `0x800A87E8`) | same type-info record |
| `0x800A8810` | 12 | 12 (`blr` at `0x800A8818`) | same type-info record |
| `0x800A9440` | 272 | 272 (`blr` at `0x800A954C`) | end a stage actor: timer reset, unregister, `+0xD4` report |
| `0x800AA090` | 188 | 188 (`blr` at `0x800AA148`) | scaled position against a camera-relative limit |
| `0x800AA770` | 64 | 64 (`blr` at `0x800AA7AC`) | refresh `+0x604` and reset `+0xAE0` to `5500.0f` |

## Size checks

Every body was decoded from `work/dol/main.elf` and not taken from the export.
Nine sizes match `functions.tsv` exactly, `blr` ending at `address + size`.
Two need the note the instructions ask for:

* `0x800A7860`: the `functions.tsv` size is 48 but the closing `bclr` is at
  `0x800A788C`, i.e. a real extent of 44 bytes; `0x800A7890` is the next
  function's `stwu` and the four bytes between are `0x00000000` padding. The
  `# Original` doc line keeps the `functions.tsv` value 48.
* `0x800A7550`: an 8-byte thunk with no `bclr` at all — `extsb r7,r7` followed
  by `b 0x800A7560`. The extent is the jump, and `0x800A7558`/`0x800A755C` are
  padding. The doc line keeps 8.

The six 8/12-byte entries were decoded word by word:

* `0x800A7550`: `7CE70774` = `extsb r7,r7`, `4800000C` = `b 0x800A7560`.
* `0x800A8720`/`0x800A8750`/`0x800A87E0`/`0x800A8810`: `3C608037`
  (`lis r3,0x8037`), `38637A04` (`addi r3,r3,0x7A04`), `4E800020` (`bclr`).

## Corrections to the exports

* `0x800A74C0`: Ghidra writes `local_14 = local_18`, hiding the second `lfs`
  at `0x800A74D8` (`r2 - 0x6CC4`, also `0.0f`). Both loads are kept. The third
  component is `8000.0f` in *both* branches: the `else` path stores
  `point[2]` at `0x800A7500` and the common path overwrites it at
  `0x800A7504`.
* `0x800A7550`: the export models the tail jump as `FUN_800a7560()` and drops
  the `extsb`; the signed narrowing is reproduced because `0x800A7560` tests
  that argument for `-1`.
* `0x800A9440`: the export prints `*pbVar2 < 4`, but the sequence is
  `lbz`/`extsb`/`cmpi 0,3` (`0x800A94D0`..`0x800A94D8`), so the test is signed
  (`<= 3`).
* `0x800AA090`: the export nests the conversion in `CONCAT44(0x43300000,…)`.
  The image word at `0x8042C430` is `0x43300000_80000000`, i.e. `2^52 + 2^31`,
  which makes this the ordinary *signed* integer -> float idiom applied to the
  `extsb`'d table byte, not to the `+0xAC` float.

## New constants

All are lane-suffixed `W224B_*`: `W224B_F_ZERO_6C6C`, `W224B_F_ZERO_6CC0`,
`W224B_F_ZERO_6CC4`, `W224B_F_8000` (`0x45FA0000` = `8000.0f`),
`W224B_F_5500` (`0x45ABE000`), `W224B_D_2P52_2P31` (`0x43300000_80000000`),
`W224B_DAT_80358404`, `W224B_FN_800A7560`, `W224B_PLAYER_AT`,
`W224B_FN_801474B0`, `W224B_FN_801662F0`, `W224B_FN_80168230`.

Reused helpers: `platform::call1`/`call2`/`call3`/`call5`/`call6`/`call0`,
`call0_ret`, `read_f32`/`write_f32`/`read_f64`/`read_u8`/`read_u32`/`write_u32`,
`fn_800b30c0`, `w223b_call_f32_ptr2`, `w108_call_2f64_3u`, `W84_TYPE_INFO`,
`W69_SIDE_TABLE`, `W69_F_SAMPLE_Y`, `W69_G_NOTICE_OWNER`, `W92_F_SAMPLE_X`,
`STAGE_EFFECT_SPAWN`, `COLLISION_AREA_UNREGISTER`.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
lane block appended to the copy's `src/game/stage.rs`, and `cargo check --lib`
run there: exit 0 (warnings only, all pre-existing).
