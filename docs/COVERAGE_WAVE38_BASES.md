# Coverage - Wave 38 (bases)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5276 | 27 | 3 |

## What this pass covers

- The `dFileSelect_c` state-handler block `0x80615800..0x80617700` that
  follows (and complements) the already-translated `dFileSelect_c` table
  ([`fn_80618d00`]) and constructor ([`fn_8061aec0`]). All 30 bodies share
  the screen layout: NW4R layout at `+0x74`, state interface at `+0x20C`
  (transitioned through its vtable `+0x18` slot), three slot objects at
  `+0x24C..+0x254`, and mode/index words at `+0x25C..+0x2DC`.
- The `fileSelectBase` layout loader ([`fn_80615800`]) plus the shared
  helpers: slot query ([`fn_80615b50`]), pane writer ([`fn_80615bd0`]),
  flag-byte status ([`fn_80615d20`]), index clamp/walk
  ([`fn_80615e80`]) and slot-state reset ([`fn_80615f00`]).
- The three input handlers that read the pad record from the frame array at
  `0x80429F50` (player index at `0x8042A464`) and probe the view object at
  `0x8042A468` (`+0x20` slot with `0x900`): [`fn_80616590`],
  [`fn_80616ea0`] and [`fn_80617440`], plus the remaining pane/animation
  (`fn_80616110`, ..., `fn_806176c0`) and state-transition handlers.
- Transition targets are named only by descriptor address (for example
  `0x808240F0`); the state names belong to the already-translated table
  and are not re-asserted here.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80615800` | `fn_80615800` | 312 | high |
| `0x80615B50` | `fn_80615b50` | 124 | high |
| `0x80615BD0` | `fn_80615bd0` | 228 | medium |
| `0x80615D20` | `fn_80615d20` | 108 | high |
| `0x80615E80` | `fn_80615e80` | 112 | high |
| `0x80615F00` | `fn_80615f00` | 336 | high |
| `0x80616110` | `fn_80616110` | 148 | high |
| `0x806161B0` | `fn_806161b0` | 132 | high |
| `0x80616270` | `fn_80616270` | 132 | high |
| `0x80616300` | `fn_80616300` | 112 | high |
| `0x80616380` | `fn_80616380` | 296 | high |
| `0x806164B0` | `fn_806164b0` | 128 | high |
| `0x80616590` | `fn_80616590` | 872 | medium |
| `0x80616930` | `fn_80616930` | 124 | high |
| `0x806169B0` | `fn_806169b0` | 128 | high |
| `0x80616B80` | `fn_80616b80` | 72 | high |
| `0x80616BD0` | `fn_80616bd0` | 88 | high |
| `0x80616C40` | `fn_80616c40` | 88 | high |
| `0x80616CA0` | `fn_80616ca0` | 88 | high |
| `0x80616DC0` | `fn_80616dc0` | 116 | high |
| `0x80616EA0` | `fn_80616ea0` | 468 | medium |
| `0x80617110` | `fn_80617110` | 88 | high |
| `0x806171C0` | `fn_806171c0` | 88 | high |
| `0x80617230` | `fn_80617230` | 64 | high |
| `0x80617270` | `fn_80617270` | 88 | high |
| `0x80617360` | `fn_80617360` | 116 | high |
| `0x80617440` | `fn_80617440` | 380 | medium |
| `0x80617650` | `fn_80617650` | 88 | high |
| `0x806176C0` | `fn_806176c0` | 64 | high |
| `0x80617700` | `fn_80617700` | 88 | high |

## Deliberate skips

- `0x80615A80` (204 bytes): the export is built from paired-single/GQR
  artefacts (`unaff_GQR0`, `ldexpf` calls); the literal float comparison
  cannot be justified from this decompilation.
- `0x80615CC0`, `0x80615CF0`, `0x80615D90`: the exports carry
  "Control flow encountered bad instruction data" warnings and end in
  `halt_baddata()`; the bodies are incomplete.
- `0x80615E00`, `0x80615E40`, `0x80617980`, `0x806181B0`, `0x806178D0`,
  `0x806187B0`, `0x80618A60`: these call the three truncated helpers above,
  so their full behaviour cannot be reconstructed honestly.
- `0x80618CA0`, `0x80618CD0` (48 bytes each): 48-byte jump stubs to
  `0x802DCBFC`, the deferred indirect-jump stub noted in the module header.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/bases.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/game/bases.rs` (remaining errors
  are in other lanes' files: `src/runtime/os.rs`, `src/engine/display.rs`,
  `src/resource/registry.rs`).
