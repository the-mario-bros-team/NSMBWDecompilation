# Coverage - Wave 76 (WorldMap course-entry controller)

## How to read this file

Every row below is one `pub unsafe fn` body in `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. The evidence column points at the
Ghidra C export the body was transcribed from; byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`. Field offsets are literal offsets
from the object pointer. The `r2` float values quoted in the bodies were read
from `work/dol/main.elf` at `0x80433080 - k`; the `r13` slots come from
`0x8042F680 - k`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 28 | 5724 | 13 | 15 |

## What this pass covers

- `0x80110050` - object-name comparison after the last `:` (`strrchr` +
  `strcmp` against vtable slot `+0x20`), plus the two byte-identical
  `0x802DCBFC(b, b, a)` wrappers `0x80110110` and `0x80110140`.
- `0x80110170`, `0x801102D0` - the `0x8A0`-byte course-entry controller. The
  constructor installs the `0x80323368` vtable at `+0x60` and the
  `0x80303408` / `0x803295D8` / `0x80329548` / `0x80329578` vtables on the five
  embedded records; the setup routine splits `+0x04` into the three 4-bit
  indices `+0x810`, `+0x814`, `+0x7EC`, selects one triple of the nine-word
  `0x80375278` table into `+0x7D4..+0x7DC`, writes the three collision boxes at
  `+0x75C..+0x788` and the float rectangle `0x802F4B70` (`0, 0, 64, 64`).
- `0x801111B0` / `0x80111580` / `0x801117A0` - the contact pair (byte-identical
  twins) and the position store. They share the `+0x394` name component, the
  `0x80375248` "arrived" record, the `+0x870` position cache and the `+0x86C`
  valid byte.
- `0x80111C70`..`0x80112120` - the placement and display group: the `+0x7C`
  layout build (`0x80111C70`), the two animation records at `+0x544`/`+0x584`
  (`0x80111D80`), the `+0x81D` blink timer (`0x80111E50`/`0x80111F90`), the
  exit effect/sound `0x286` (`0x80111FD0`), the display-child detach
  (`0x80112080`) and the exit gate over `0x801126A0` (`0x80112120`).
- `0x80112440`..`0x80113600` - the range and clamp ladder. `0x80112440` and
  `0x80112480` probe the `r13-0x5940` scene collider (the second one runs eight
  probes and packs them into one byte); `0x80112B30` clamps the height into the
  `scene+0x3C`/`+0x44` band using the `camera+0x8FEAC` value; `0x80113260` runs
  the `0x80160D10` test against the `+0x7B0` vector; `0x801132C0` and
  `0x80113470` are the retry pair; `0x80113320` and `0x801133B0` are the
  status-gated exit and the contact chain; `0x80113500` rotates the shared
  `r13-0x5770 + 0x110` lane counter and picks the `±1.7f` turn; `0x80113600`
  places the object on its `+0x7F0` lane with the `0x802F4BA8` table.
- `0x80113CC0`, `0x80113E40`, `0x80113FA0` - the per-frame update triple. They
  run the `+0x500`/`+0x502` timers, the `0x80112810`/`0x8015F480` eased value,
  both clamps and the two exit records, then `0x80113FA0` notifies the owner and
  re-runs the clamps.

Cross-evidence for the class: the wave-12 body [`wm_balloon_burst_effect`]
(`0x80113490`) reads the same `+0x7EC` class word and `+0x810` player index and
is called from [`fn_80112120`] and [`fn_80113320`], which confirms that both
groups operate on one object.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80110050` | `fn_80110050` | 136 | high |
| `0x80110110` | `fn_80110110` | 48 | high |
| `0x80110140` | `fn_80110140` | 48 | high |
| `0x80110170` | `fn_80110170` | 352 | medium |
| `0x801102D0` | `fn_801102d0` | 776 | medium |
| `0x801111B0` | `fn_801111b0` | 212 | medium |
| `0x80111580` | `fn_80111580` | 212 | medium |
| `0x801117A0` | `fn_801117a0` | 176 | medium |
| `0x80111C70` | `fn_80111c70` | 268 | medium |
| `0x80111D80` | `fn_80111d80` | 204 | medium |
| `0x80111E50` | `fn_80111e50` | 168 | high |
| `0x80111F90` | `fn_80111f90` | 48 | high |
| `0x80111FD0` | `fn_80111fd0` | 172 | medium |
| `0x80112080` | `fn_80112080` | 160 | medium |
| `0x80112120` | `fn_80112120` | 140 | high |
| `0x80112440` | `fn_80112440` | 60 | high |
| `0x80112480` | `fn_80112480` | 532 | high |
| `0x80112B30` | `fn_80112b30` | 220 | medium |
| `0x80113260` | `fn_80113260` | 88 | high |
| `0x801132C0` | `fn_801132c0` | 84 | high |
| `0x80113320` | `fn_80113320` | 140 | high |
| `0x801133B0` | `fn_801133b0` | 192 | medium |
| `0x80113470` | `fn_80113470` | 20 | high |
| `0x80113500` | `fn_80113500` | 256 | medium |
| `0x80113600` | `fn_80113600` | 400 | medium |
| `0x80113CC0` | `fn_80113cc0` | 200 | medium |
| `0x80113E40` | `fn_80113e40` | 316 | medium |
| `0x80113FA0` | `fn_80113fa0` | 96 | high |

## Deliberate skips

- `0x80110B60`, `0x80110FA0`, `0x80111F00`, `0x801129C0`, `0x80112C10`,
  `0x80112DB0`, `0x80112E80`, `0x80113950`, `0x80113AE0` and `0x80113D90` were
  skipped because their exports carry `unaff_GQR0` blocks (`ldexpf`
  reconstruction) and, in most cases, a removed unreachable block, so the
  paired-single scale factors are not recoverable from the C. Waves 36, 60, 64
  and 73 skipped the same record class; `0x80112C10`, `0x80112DB0` and
  `0x80112E80` are still called by this pass's bodies through their addresses.
- `0x801121B0` and `0x801126A0` were skipped because Ghidra misparses their
  prologues as `FUN_802DCD5C`/`FUN_802DCD64` and models the saved-register
  restore as a `undefined8` return, so the incoming `this` pointer arrives as
  the high half of that "result". Both are 652- and 356-byte bodies; with the
  receiver unresolved the field writes cannot be attributed honestly.
- `0x80110B10` and `0x80110CA0` were reviewed but not taken. `0x80110B10` is
  clean yet is a single `f64` band test that does not belong to any of the three
  clusters; `0x80110CA0` is a dense nested boolean over the `+0x10D8` status
  bits plus the `r13-0x5940` collider, and it is called by the contact pair
  without needing its own body.
- `0x80112F50`, `0x80112810` and `0x801126A0` remain outside this pass even
  though this pass calls them; they keep their raw addresses in the bodies.
- The remaining untranslated entries of the `0x80110000` bucket are the
  `0x80111850`/`0x801121B0`/`0x801126A0` record helpers, the
  `0x80113790`/`0x80114050` group and the `0x80114BA0..0x80115A90` follower
  group.

## Verification

- Every address and byte size matches `functions.tsv`; the 28 sizes sum to
  5,724 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv` (all 28 were untranslated
  entries of the `0x80110000` bucket at the start of the pass).
- `cargo check --message-format=short` reports no error and no diagnostic that
  names `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave76` reports `OK` (28 rows in this lane,
  unique addresses).
