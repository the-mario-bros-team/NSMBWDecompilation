# Coverage - Wave 175 lane C (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (pending merge of `work/wave175/laneC_block.rs`) | `game::bases` | 5 | 2,008 | 3 | 2 |

All five lane-C bodies were translated. Nothing was deferred.

## What this pass covers

- **Four-channel height rebuild (1 body).** `0x80682F40` clears the four
  channel floats `+0xABC..+0xAC8`, the pending bytes `+0xACC..+0xACF` and the
  `+0xAA8` flag, marks the player slots of the live actors in the `+0x93C` list
  (class byte `+0x38C` in `1..2`, slot from vtable `+0x60`/`+0x6C`), then for
  each channel whose `+0xAD0 + i` was active last frame scales the unsigned
  `+0xAB2` sample by `1/256`, runs it through `0x80237BD0`, multiplies by
  `400.0`, negates for a zero facing byte `+0x348` and halves into the channel
  word. It closes by copying pending bytes onto the active bytes `+0xAD0..`.
- **Grid-column picker (1 body).** `0x80746F90` fills a rectangle through
  `0x8008E890(stage_view(), +0x3D0 & 0xFF, rect)`, converts `rect.y` into a
  column against `camera[0x3C] - 16.0`, scans `camera[0x44]/32` grid words,
  picks one occupied column through `0x800B2EE0(count)`, clears that cell and
  writes `rect.y - 32*index - 16.0` through `out`. Returns whether any column
  was occupied. It reuses the existing `stage_view()` helper.
- **Spin-direction reapply (1 body).** `0x807586D0` is the direction half of the
  `0x80758E70`/`0x80759180` family: it clears `+0x265` and re-posts the idle
  message, rejects a set `+0x267`, maps the selected player record's `+0x8C`
  hit byte, the `+0x266`/`+0x26A` flags and the `+0x14` bit pair (`8`/`4`) onto
  the direction word `+0x244`, and posts `0x80832xxx` through the `+0x12C` pane
  vtable.
- **Screen-base bob update (1 body).** `0x806488E0` publishes
  `+0xB0 + +0x1A4B8` to `+0x1A49C`, runs the `+0x1A4F0` countdown, selects the
  `1.8 * speed` accumulator step while mode `3` is active, the speed is
  positive and the height is below `camera[0x3C] - camera[0x44] - 8.0`, calls
  `0x80648470(height - +0x1A4BC, this)` and, still in mode `3` with speed, cues
  the `[0x80429D64]` driver slot `+0x2C` with id `0x310` and the sign-extended
  `camera[0x4C]` height.
- **Pendulum angle step (1 body).** `0x80707620` nudges the `+0x3DA` pivot by
  the signed `0x8042C1A0` table byte (index `+0x3E1`) times the `+0x4` nibble
  scale while held, then advances `+0x104` by the signed `+0x3D0` step; when
  the remaining distance is below the step magnitude (`0x802DE514`/`0x802DE524`)
  it snaps to `+0x3DC`, inserts the position through `0x8008DDB0` and cues `3`
  through `0x800D8C60` for bit `8` of `+0x4`.

## Observations for later waves

- **Pointer globals.** `0x806488E0` and `0x80707620` dereference the pointer
  words at `0x80429DD0` and `0x8042A078` before applying their offsets
  (`[[0x80429DD0] + 0x9002C]`, `[[0x8042A078] + 0/4]`), which is what the raw
  `lwz` pairs do. Older bodies in this file (`fn_80648260`, `fn_80648440`,
  `fn_806741e0`) read those addresses directly; that difference is a known
  simplification in earlier waves, not a different reading of this lane's
  functions.
- **`0x800B2EE0`** is passed the occupied-column count in `0x80746F90`, matching
  the `abi_call1!(0x800B_2EE0, count)` shape already used elsewhere.

## Deferred

None.
