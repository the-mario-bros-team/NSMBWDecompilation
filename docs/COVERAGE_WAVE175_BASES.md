# Coverage - Wave 175 (`game::bases` 392-412 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 26 | 10,448 | 19 | 7 |

Fourth five-lane wave on the paid model; 26 bodies in the 392-412 byte
sub-band.

## Lane A (5 bodies, 1,964 bytes)

- `0x806787C0` - multi-pane row writer, step `16*t/span` gated on `0.25`,
  `+0x530` panes zig-zagging around `+0xAC`.
- `0x806C7890` - lift-probe scanner over `+0x440`; kinds 1/2 accumulate
  `node[+0x24] - 60.0`, kind 3 attaches via `fn_806c7a70`.
- `0x80743790` - `daTorchiLight_c` loader (`g3d/center_fire.brres`).
- `0x807B9F40` - world-select scene reset; the player table is doubly indirect
  (`*0x8042A028 + slot*4 + 4`). **medium** (global roles inferred).
- `0x8060FB50` - `Wm_2d_titlestar01/02` on the five map panes.

## Lane B (5 bodies, 1,984 bytes)

- `0x8069FF00` - generic shot update; the un-specialised twin of
  `fn_8069F640`'s tail.
- `0x806F7500` - prop re-initialiser; ~30 floats from the `0x807CC2E0` pool.
- `0x8074F600` - world-map burst (`Wm_mr_landsmoke` + three
  `Wm_ob_cmnshot*`), sound `0x2A0`.
- `0x807B7F30` - scene-ready selector on `0x80315B98`.
- `0x80637D20` - rebuilds the four `0x3C`-byte player records at `+0x7BC`.

## Lane C (5 bodies, 2,008 bytes)

- `0x80682F40` - four-channel height rebuild; `400.0`, `1/256`, `0.5`.
- `0x80746F90` - grid-column picker returning `rect.y - 32*idx - 16.0`.
- `0x807586D0` - spin-direction reapply from the selected player's `+0x8C` hit
  byte and the `+0x14` bits 8/4.
- `0x806488E0` - screen-base bob with `1.8*speed` mode-3 arm, cue `0x310`.
- `0x80707620` - pendulum angle step with the `0x8042C1A0` signed table.

## Lane D (5 bodies, 2,028 bytes)

- `0x80762010` - pose-slider updater; same tail as `fn_8075aa50`.
- `0x80779CA0` - per-variant `CS_W%d` cloud loader; structurally identical to
  `fn_80760070`.
- `0x8078E9A0` - seven-phase cloud interaction machine on `+0x1C8`.
- `0x8064DCC0` - duplicated ground-probe span snap (`0.0625`, `14.0`).
- `0x806DFDB0` - lift height clamp, twin of `fn_806dff70`.

## Lane E (6 bodies, 2,464 bytes)

- `0x80773EF0` - the element-row writer called by `fn_80773d70`
  (`call_f32_u32x4` convention).
- `0x8079AC60`, `0x8067E160`, `0x8071F390`, `0x80795AA0`, `0x807BE230` -
  the remainder of the 408-412 band.

## Corrections and cross-checks

- The wave 62/103/108/109 deferrals of `0x806787C0`, `0x806C7890`,
  `0x80743790` and friends were **GQR false alarms**: every `psq_*` in them is
  an `f27..f31` save slot on GQR0, not vector data.
- Four exports drop `this` through `_savegpr` thunks (`0x802DCD64/68/70/74`);
  `r3` survives, so all four take `this: usize`.
- `0x8064DCC0`'s magic `0x807C86E0 = 0x4330000080000000` (`2^52+2^31`) makes
  the reconstruction exactly `v as i32 as f32`; Ghidra's XOR-on-`2^52` model
  was wrong.
- `0x806488E0` and `0x80707620` dereference `0x80429DD0`/`0x8042A078` before
  offsetting, matching the raw `lwz` pairs.
- `0x8078E9A0` state 3's `bc 4,1` is `<= 0`, matching Ghidra's `< 1`;
  `0x806DFDB0`'s `fcmpo`+`cror`+`bc 4,2` window tests match Ghidra.

## Verification

- `cargo check` succeeds with 4,242 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave175`: 26 rows, 26 unique addresses,
  19 high + 7 medium.
- All 26 evidence exports exist and every size matches `functions.tsv`.
