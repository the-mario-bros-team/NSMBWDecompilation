# Coverage - Wave 49 (bases)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`). Every address below is a static analysis
address, not a runtime address. Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores and call targets are literal, `medium` when a
callee role or a field role is inferred from argument shape.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 4796 | 12 | 17 |

## What this pass covers

- The `togezo` (spiny) course actor, `0x807840F0..0x80784740`: the constructor
  [`fn_807840f0`] (vtable `0x80818A20`, model setup `+0x188`, resource slot
  `+0x1A4`, one `0x38` animation slot at `+0x1E8`), the
  `g3d/togezo.brres` loader [`fn_80784360`] with its `togezo` pane, the
  transform writer [`fn_80784410`], the camera-drop reset [`fn_807844d0`], the
  spawn body [`fn_80784650`] (saved position `+0xB8..+0xC0`, factory
  `0x800F69F0`, state descriptor `0x80377DE0`) and the landing handler
  [`fn_80784740`] which emits the `0x0E`/`0x56` events and calls the vtable
  `+0x68` slot.
- The `cobToride` fortress-door course object, `0x807848C0..0x807856E0`: the
  constructor [`fn_807848c0`] (vtable `0x80818B58`, four `0x38` slots at
  `+0x1B4`, the two-step `+0x294` interface vtable), the per-frame driver
  [`fn_80784a30`], the `g3d/model.brres` loader [`fn_80784c70`] that builds
  four door slots from the `cobTorideOpen` pointer table at `0x80818B10` and
  binds `LeftDoor`/`RightDoor` onto the three `+0x294` nodes, the transform
  writer [`fn_80784e50`], the door state machine [`fn_80784f00`], the timer
  clear [`fn_80785120`], the one-shot `Koopa` open animation
  [`fn_80785580`], the `0x280` door factory [`fn_80785600`], its
  position/sound/animation setup [`fn_80785640`] and the stage gate
  [`fn_807856e0`].
- The two neighbour course objects, `0x80785830..0x80785F50`: constructor,
  `g3d/model.brres` loader and transform writer for `cobTower`
  ([`fn_80785830`], [`fn_80785a50`], [`fn_80785b00`]) and for
  `cobTreasureShip` ([`fn_80785c90`], [`fn_80785ea0`], [`fn_80785f50`]),
  plus the treasure ship's active-frame body [`fn_80785e50`].
- The moving-platform event family, `0x80786000..0x80786380`: the dispatcher
  [`fn_80786000`] for events `0x21`/`0x22`/`4`, the phase setter
  [`fn_80786170`], the target resolver [`fn_80786290`], the saved-position
  restore [`fn_80786330`], the packed-id extractor [`fn_80786360`] and the
  snap body [`fn_80786380`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807840F0` | `fn_807840f0` | 116 | high |
| `0x80784360` | `fn_80784360` | 172 | high |
| `0x80784410` | `fn_80784410` | 188 | medium |
| `0x807844D0` | `fn_807844d0` | 84 | medium |
| `0x80784650` | `fn_80784650` | 236 | medium |
| `0x80784740` | `fn_80784740` | 156 | medium |
| `0x807848C0` | `fn_807848c0` | 172 | high |
| `0x80784A30` | `fn_80784a30` | 296 | medium |
| `0x80784C70` | `fn_80784c70` | 480 | medium |
| `0x80784E50` | `fn_80784e50` | 176 | medium |
| `0x80784F00` | `fn_80784f00` | 536 | medium |
| `0x80785120` | `fn_80785120` | 12 | high |
| `0x80785580` | `fn_80785580` | 128 | medium |
| `0x80785600` | `fn_80785600` | 60 | high |
| `0x80785640` | `fn_80785640` | 152 | medium |
| `0x807856E0` | `fn_807856e0` | 100 | high |
| `0x80785830` | `fn_80785830` | 84 | high |
| `0x80785A50` | `fn_80785a50` | 172 | high |
| `0x80785B00` | `fn_80785b00` | 176 | medium |
| `0x80785C90` | `fn_80785c90` | 76 | high |
| `0x80785E50` | `fn_80785e50` | 60 | medium |
| `0x80785EA0` | `fn_80785ea0` | 172 | high |
| `0x80785F50` | `fn_80785f50` | 176 | medium |
| `0x80786000` | `fn_80786000` | 356 | medium |
| `0x80786170` | `fn_80786170` | 12 | medium |
| `0x80786290` | `fn_80786290` | 160 | medium |
| `0x80786330` | `fn_80786330` | 44 | high |
| `0x80786360` | `fn_80786360` | 28 | high |
| `0x80786380` | `fn_80786380` | 216 | medium |

## Deliberate skips

- `0x80786180` (264 bytes) is called by [`fn_80786000`] and is left as an
  address-only call. Its only callee, `0x8015F140`, is a float-spline helper
  whose result Ghidra widens to `double` and whose argument list mixes four
  float values with a stack pointer; without an instruction listing the
  single- versus double-precision ABI cannot be settled, so no body is
  invented.
- `0x8078B5B0` and the other `0x38`-byte destructors in the
  `0x8078B5B0..0x8078C0D0` alternating pair pattern abort with
  `halt_baddata()`, so they cannot be reconstructed.
- The rest of the `0x80780000` bucket is outside this lane: 195 entries were
  untranslated before this pass and 166 remain after it. The chosen window is
  contiguous except for the skipped spline body above.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave49` reports `OK` for
  `wave49-bases.csv`.
