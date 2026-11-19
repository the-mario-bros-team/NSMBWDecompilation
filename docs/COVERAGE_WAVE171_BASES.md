# Coverage - Wave 171 (`game::bases` 324-332 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 8 | 2,616 | 8 | 0 |

Fourth batch from the 300-512 byte band; all eight bodies are in the
324-332 byte sub-band.

## What this pass covers

- **Pose copy (1 body).** `0x8060A5D0` walks the four `+0x2D8` lane flags,
  maps each through the `0x807C68C0` table (`10, 11, 13, 12`) and copies the
  pose fields onto the matching `+0x24C` record.
- **Selector reconcile (1 body).** `0x80629530` applies the `&4`/`&8` world
  flag rules to `+0xA9C`, then re-requests the `0x900` object.
- **Gate twin (2 bodies).** `0x80697F10` and `0x80699E30` decrement `+0xD7C`
  and `+0xDB0` respectively, toggling the `0xD4` callback between the
  `0x803578F4`/`0x80357A74` descriptors.
- **Transform rebuild (1 body).** `0x806E7FF0` tests the `0x8082E448` state
  descriptor, rebuilds `+0x7C` with the `8.0` height offset and runs the
  `+0x6D0 + 0x10` finish method.
- **Layout loader (1 body).** `0x8062BA50` loads the multi-course-select
  layout and picks the `0x3D`/`0x3C` row from `0x80429F80 & 0x20`.
- **Throw start (1 body).** `0x80797810` nudges the spawn point one unit
  along the `+0x23C` direction and latches `+0x545`.
- **Throw end (1 body).** `0x8064EEB0` normalises `+0x5FC` for modes `2`/`3`,
  refreshes the `+0x584`/`+0x5BC` movers and latches `+0x3F8`/`+0x3F9`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8060A5D0` | `fn_8060a5d0` | 324 | high |
| `0x80629530` | `fn_80629530` | 324 | high |
| `0x80697F10` | `fn_80697f10` | 324 | high |
| `0x806E7FF0` | `fn_806e7ff0` | 324 | high |
| `0x8062BA50` | `fn_8062ba50` | 328 | high |
| `0x80797810` | `fn_80797810` | 328 | high |
| `0x8064EEB0` | `fn_8064eeb0` | 332 | high |
| `0x80699E30` | `fn_80699e30` | 332 | high |

## Corrections and notes

- `0x8060A5D0` opens with `bl 0x802DCD6C`, a `_savegpr` register-helper whose
  `r3` survives unchanged; the Ghidra `void` prototype hides the real first
  argument. The Rust body therefore takes `this` and does not model the save
  and restore helpers as calls.
- `0x8060A5D0` also carries several dead stack stores (`+0x18`..`+0x40`) copied
  from a struct assignment; only `+0x2C`/`+0x30`/`+0x34` and `+0x44`/`+0x48`
  reach the destination records.
- `0x80697F10` and `0x80699E30` are identical apart from the counter field and
  the `0x14` reload; the shared shape is now proven by two independent bodies.
- `0x80797810`'s `0x8031D39C + 0xBC` offset is a `+0xBC` field of the
  `0x8031D39C` record, not a standalone float constant.

## Verification

- `cargo check` succeeds.
- `node tools/check-lane-csv.mjs wave171` validates the eight-row lane CSV.
- All eight evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`.
