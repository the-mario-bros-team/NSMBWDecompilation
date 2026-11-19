# Coverage - Wave 90 (`game::worldmap`, `0x80140000` bucket)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
WorldMap player actor; `+0x1090` is the player variant, `+0x2A64` the controller
object and `+0x153D`/`+0x153E` the special-move state pair. `0x80056CF0` is the
actor status-bit test documented next to `STATUS_WORDS`. `r2` is `0x80433080`
and `r13` is `0x8042F680`; every `r2`-relative float below was read from
`work/dol/main.elf` at the absolute address named in the constant comment.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 30 | 5476 | 12 | 18 |

## What this pass covers

- **`0x80140590`..`0x80141640` - the special-move contact ladder.**
  `0x80140590` is the status-bit `0x72` hook, `0x80140670` resets the `+0x114`
  triple and the `+0x1550` timer through the controller's `+0x5C` slot,
  `0x80140AF0` is the two-mode contact dispatcher over `+0x1EC`/`+0x3FC` and the
  `0x8012DBE0` partner, and `0x80140CD0`/`0x80140DB0` are the two exit posters
  (`0x80375E6C` and `0x803765B0`). `0x80140E10` raises `9` in the shared
  `r13 - 0x5988` byte and routes the per-character contact through the
  `0x80354E60`/`0x80354E80` tables; `0x80141140` is the height-delta plus
  class-to-sound ladder over `0x802F4FB8` and `0x802F4FC4`.
- **`0x80141460`..`0x801416F0` - the mode/countdown state machine.**
  `0x80141460` stores `+0x153D` and runs the `0x802DCBFC` transition with
  `0x801414C0`, which owns the `+0x153E` countdown and the `+0x124` start flag.
  `0x80141580` dispatches the `+0xE4` slot, `0x80141640` picks the
  `0x2D`/`0x2E`/`0x35` event, and `0x801416F0` is the full two-actor move
  ladder for `+0x2D` ids `0x13`, `1`, `7`, `0xD` and `8`, ending in
  `0x802DCDC0`.
- **`0x80141BF0`..`0x80141FD0` - layout submission and colour records.**
  `0x80141BF0` submits the `0x802F510C` height through `0x80053940`;
  `0x80141E30` copies a `0x80325260` record (indexed through `0x802F50A8`)
  into `+0x164..+0x170`; `0x80141E90`/`0x80141EE0`/`0x80141F40` write the four
  colour words at `+0x1224..+0x1230` with style byte `+0x1235`;
  `0x80141F70`/`0x80141FD0` build and apply the same record through
  `0x8012E360`.
- **`0x80142F20`..`0x801436F0` - animation-table layout and effect attach.**
  `0x80142F20` clamps the animation-table entry's `+0x1C`/`+0x28` into the
  actor and shared bounds with the `0x802F4FE8` 16.16 offsets, `0x801432F0`
  copies the partner face byte into `+0x285` and then runs it, and
  `0x80143410` applies the `+0xC000` height bias. `0x801434C0`/`0x801434E0`
  are the `0x800534B0` wrapper and the `+0x10D4..+0x10E0` clear,
  `0x80143530` selects the `-4.0`/`-2.5` vertical speed, and
  `0x801435E0`/`0x801436F0` attach and retire the `+0x27F8` effect through
  `0x8008FCD0`/`0x8008FBF0` and the `+0xB0` receiver slot.
- **`0x801461D0`/`0x801465D0` - slot action and moving colour record.**
  `0x801461D0` runs the `+0x120`/`+0x450` pair only when `+0x1090` equals
  `+0x14E0` and posts sound `4`; `0x801465D0` reads the seven-float
  `0x802F57A0` table, applies the `+0x160` bit `0`/`2` scale ladder
  (`0.25`, `0.5`, `-0.5`) and calls `0x8015F480`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80140590` | `fn_80140590` | 72 | medium |
| `0x80140670` | `fn_80140670` | 108 | medium |
| `0x80140AF0` | `fn_80140af0` | 312 | medium |
| `0x80140CD0` | `fn_80140cd0` | 116 | high |
| `0x80140DB0` | `fn_80140db0` | 96 | medium |
| `0x80140E10` | `fn_80140e10` | 208 | medium |
| `0x80140EE0` | `fn_80140ee0` | 88 | medium |
| `0x80141140` | `fn_80141140` | 476 | high |
| `0x80141460` | `fn_80141460` | 92 | high |
| `0x801414C0` | `fn_801414c0` | 188 | high |
| `0x80141580` | `fn_80141580` | 24 | medium |
| `0x80141640` | `fn_80141640` | 168 | high |
| `0x801416F0` | `fn_801416f0` | 872 | medium |
| `0x80141BF0` | `fn_80141bf0` | 212 | medium |
| `0x80141E30` | `fn_80141e30` | 96 | medium |
| `0x80141E90` | `fn_80141e90` | 80 | high |
| `0x80141EE0` | `fn_80141ee0` | 96 | medium |
| `0x80141F40` | `fn_80141f40` | 44 | high |
| `0x80141F70` | `fn_80141f70` | 88 | high |
| `0x80141FD0` | `fn_80141fd0` | 96 | high |
| `0x80142F20` | `fn_80142f20` | 436 | medium |
| `0x801432F0` | `fn_801432f0` | 280 | medium |
| `0x80143410` | `fn_80143410` | 172 | medium |
| `0x801434C0` | `fn_801434c0` | 20 | high |
| `0x801434E0` | `fn_801434e0` | 80 | high |
| `0x80143530` | `fn_80143530` | 72 | high |
| `0x801435E0` | `fn_801435e0` | 272 | medium |
| `0x801436F0` | `fn_801436f0` | 156 | medium |
| `0x801461D0` | `fn_801461d0` | 204 | medium |
| `0x801465D0` | `fn_801465d0` | 252 | medium |

## Deliberate skips

- **GQR-reconstruction records `0x801406F0`, `0x80140FF0`, `0x80141A60` and
  `0x80142030`.** Each export opens with the `unaff_GQR0`/`ldexpf` prologue and
  removed unreachable blocks, so the float loads cannot be recovered without
  the live `GQR0` base. `0x80141460` and `0x801414C0` call `0x80142030` by raw
  address; `0x801416F0` calls `0x80142660` (same policy) only through
  [`fn_801414c0`].
- **`0x80142660` and `0x801430E0`.** Both are GQR records; they are invoked as
  raw addresses by [`fn_801414c0`] and [`fn_801432f0`] and are not claimed as
  bodies.
- **The rest of the `0x80140000` bucket.** `0x80144110`..`0x80145650`, the
  twelve-byte jump thunks `0x8014A340`..`0x8014A400`, the
  `0x8014B000`/`0x8014B020`/`0x8014B040` records and
  `0x8014CD70`..`0x8014FF00` remain untranslated. They are valid follow-up
  targets for a later `0x80140000` pass.

## Verification

- Every address and byte size matches `functions.tsv`; the 30 sizes sum to
  5,476 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 30 were untranslated
  entries of the `0x80140000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no warning or error naming
  `src/game/worldmap.rs` (the build still stops on other lanes' errors).
- `node tools/check-lane-csv.mjs wave90` reports the lane clean.
