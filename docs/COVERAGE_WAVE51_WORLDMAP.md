# Coverage - Wave 51 (WorldMap ice map object and record helpers)

## How to read this file

Each row is one original `main.dol` function, reconstructed as a
`pub unsafe fn fn_<address>` body in `src/game/worldmap.rs`. The *address* is the
PowerPC virtual address from `work/ghidra/main-gb/export/functions.tsv`; the
*bytes* column is the exact size from that file; *confidence* matches the
`# Confidence` line of the body's doc comment. "high" means the guard, every
store and every call are literal in the Ghidra export; "medium" means a callee
argument, a field role or a callee identity is inferred and called out in the
body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 30 | 2404 | 25 | 5 |

## What this pass covers

- `0x80114940..0x801153E0` - the `0x6B8`-byte ice map object. The constructor
  at `0x80114AC0` installs the vtables at `+0x60` and `+0x434` and binds the
  sub-objects at `+0x3D0`, `+0x3F0`, `+0x440`, `+0x460` and `+0x58C`; the
  neighbouring records name the archive `g3d_ice_brres`, the model `ice_A1`
  and the part `ice_inside`. `0x80114ED0` rebuilds the layout from
  `+0xAC..+0xB4` and the angles at `+0x100`, `0x80114FC0` adds the object's
  offsets to the shared `0x80162590` position, and `0x80115360` applies the
  stored per-frame motion. The four 4-byte entries `0x801151F0`,
  `0x80115200`, `0x80115210` and `0x80115220` are empty virtual hooks; the
  wrappers `0x80115250`, `0x801152A0`, `0x80115300` and `0x801153E0` call them
  from the four update/destroy slots. `0x80114940` is the release path.
- `0x8011AD90..0x8011C630` - record/state helpers: the `+0x274` flag word
  (`0x8011AE80`), the `+0x2DC` collision handle (`0x8011AE20`), the combined
  action gate on `+0x274` bit `0x8000`, the `+0x2DC` probe and the `+0x1EC`
  state object (`0x8011AD90`), the `0x80375540`/`0x80375550` four-entry tables
  (`0x8011AF20`), the release duplicate `0x8011B1B0`, the constant-pointer
  getter `0x8011B210`, the name-suffix compare behind virtual slot `+0x20`
  (`0x8011B3E0`), the two more `0x802DCBFC` wrappers
  `0x8011B4A0`/`0x8011B4D0`, the four-player detach loop `0x8011BDA0` and the
  eight-float default-vector compare `0x8011C630`.
- `0x8011D6E0..0x8011F5D0` - the position helpers `0x8011D6E0`/`0x8011D8C0`
  and the two records that sit beside wave 12's `0x8011F430`: the
  `0x80162590` first-word getter `0x8011F3F0` and the query-and-submit pair
  `0x8011F5D0` (`0x8011EDA0` then `0x8011EB00` with tag `2`).

## Confidence notes

- `medium` bodies: `0x80114AC0` (`0x80066FC0` is printed without an argument
  even though the fresh object is in `r3`), `0x80115300` (the two ratio helpers
  `0x80115030`/`0x801150A0` are only recorded by address; both read
  `r2 - 0x5C40` and were not translated), `0x8011AD90` (collision and state
  field roles are inferred from the callee names and the sibling
  `0x8011AE20`), `0x8011AF20` (the two tables are literal; their meaning is
  unknown) and `0x8011BDA0` (the `+0x2A78` "identity" role is inferred from the
  equality test).
- Every other body is `high`: the export's control flow, stores and calls are
  reproduced literally, including Ghidra's signed
  `((int)s >> 31) + (0x2D15 < (uint)s)` sum in `0x8011AD90`.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80114940` | `fn_80114940` | 88 | high | release an ice-object record and optionally free it |
| `0x80114A60` | `fn_80114a60` | 48 | high | forward two words to `0x802DCBFC` |
| `0x80114A90` | `fn_80114a90` | 48 | high | second identical forwarder |
| `0x80114AC0` | `fn_80114ac0` | 224 | medium | construct the `0x6B8`-byte ice map object |
| `0x80114ED0` | `fn_80114ed0` | 228 | high | rebuild the ice object's layout from position and angles |
| `0x80114FC0` | `fn_80114fc0` | 104 | high | add the object's offsets to the shared position |
| `0x801151F0` | `fn_801151f0` | 4 | high | empty virtual hook |
| `0x80115200` | `fn_80115200` | 4 | high | empty virtual hook |
| `0x80115210` | `fn_80115210` | 4 | high | empty virtual hook |
| `0x80115220` | `fn_80115220` | 4 | high | empty virtual hook |
| `0x80115250` | `fn_80115250` | 80 | high | update wrapper: hook, virtual refresh, position add |
| `0x801152A0` | `fn_801152a0` | 52 | high | destroy wrapper: hook plus layout reset |
| `0x80115300` | `fn_80115300` | 68 | medium | update wrapper calling the two ratio helpers |
| `0x80115360` | `fn_80115360` | 116 | high | apply the stored per-frame motion |
| `0x801153E0` | `fn_801153e0` | 52 | high | fourth wrapper: hook plus layout reset |
| `0x8011AD90` | `fn_8011ad90` | 144 | medium | action gate from flags, collision probe and state |
| `0x8011AE20` | `fn_8011ae20` | 88 | high | test status bit `0x1000` of the collision object |
| `0x8011AE80` | `fn_8011ae80` | 12 | high | return the `0x1FE000` flag field |
| `0x8011AF20` | `fn_8011af20` | 76 | medium | test the two global four-entry tables |
| `0x8011B1B0` | `fn_8011b1b0` | 92 | high | second release path |
| `0x8011B210` | `fn_8011b210` | 12 | high | return the constant pointer `0x80353F0C` |
| `0x8011B3E0` | `fn_8011b3e0` | 136 | high | compare the name suffix after the last `:` |
| `0x8011B4A0` | `fn_8011b4a0` | 48 | high | third `0x802DCBFC` forwarder |
| `0x8011B4D0` | `fn_8011b4d0` | 48 | high | fourth `0x802DCBFC` forwarder |
| `0x8011BDA0` | `fn_8011bda0` | 148 | medium | detach a record from the four player objects |
| `0x8011C630` | `fn_8011c630` | 176 | high | install an eight-float vector unless it is the default |
| `0x8011D6E0` | `fn_8011d6e0` | 56 | high | forward a word and `0x802F4CD8` to `0x802DCBFC` |
| `0x8011D8C0` | `fn_8011d8c0` | 104 | high | add the record's offsets to the shared position |
| `0x8011F3F0` | `fn_8011f3f0` | 52 | high | return the first word of the `0x80162590` object |
| `0x8011F5D0` | `fn_8011f5d0` | 88 | high | query a record and submit it with tag `2` |

## Deliberate skips

- `0x80115030`, `0x801150A0` and `0x80115100` lie inside the ice-object range but
  read unverified `r2 - 0x5C38..0x5C50` float constants and, for the first two,
  Ghidra's `CONCAT44(0x43300000, ...)` signed-int conversion. They are called by
  address from `0x80115300`/`0x80114ED0` and are left for a later pass instead of
  being reconstructed from a guessed constant.
- `0x80114BA0` and `0x80114C40` consume `unaff_r2`/`unaff_r13` globals and the
  `g3d_ice_brres` archive names; the register bases are not recoverable from the
  export, so they were skipped.
- `0x8011B300` is a five-level recursive vtable chain whose indirect calls are
  printed by Ghidra with no receiver argument at all; the receiver cannot be
  proven from the export, so it was skipped rather than guessed.
- `0x8011C040` and `0x8011C6E0` call `0x8011C100` and `0x8011C750` with no
  argument even though those callees take an object pointer. `0x8011C630`, the
  clean half of the same family, was translated instead.
- No GQR-reconstruction record exists in the chosen ranges.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/game/worldmap.rs`.
