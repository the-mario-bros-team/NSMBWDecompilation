# Coverage - Wave 170 (`game::bases` 300-332 byte band, part 3)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 8 | 2,520 | 8 | 0 |

Third batch from the 300-512 byte band; all eight bodies are in the
312-320 byte sub-band.

## What this pass covers

- **Spawn reset (1 body).** `0x80719460` rebuilds the position from the
  `0x808083D4` table with the `16`/`-256`/`-1` constants and plays `0x533` for
  lane `0`.
- **Render push (1 body).** `0x80758010` scales `+0x220..+0x228` by `4.1`
  (and `0.95` for types `{2,3}`), then pushes position, scale and angles to the
  `+0x128` node.
- **Source re-seed (1 body).** `0x8069C170` re-reads the `+0xDAC` source,
  selects the mode through the `+0x738` child and sets the `+0x348` direction
  byte.
- **Request gate (1 body).** `0x807B00C0` is the third copy of the
  `0x900`/`0x800B53A0` gate family first seen in Wave 168, with the
  `0x80833B20` descriptor.
- **Layout loaders (2 bodies).** `0x8061E980` loads the cannon minigame layout;
  `0x8063EAF0` loads the player-select layout. Both run the
  `0x800C8CB0`..`0x800C8F20` node cache calls.
- **Ship controller (1 body).** `0x8076FC60` builds the `shipRoute`/`shipStop`
  controller from the `0x80816C34`..`0x80816C48` seeds.
- **State dispatch (1 body).** `0x80799200` maps the `0x80798F10` state into a
  bool (`{4}` → `1`, `{2,3}`, `{6}`, `0xE..=0x1C` → `0`), propagates it and
  refreshes the `+0x29C` renderer flag.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80719460` | `fn_80719460` | 316 | high |
| `0x80758010` | `fn_80758010` | 316 | high |
| `0x8069C170` | `fn_8069c170` | 320 | high |
| `0x807B00C0` | `fn_807b00c0` | 320 | high |
| `0x8061E980` | `fn_8061e980` | 312 | high |
| `0x8063EAF0` | `fn_8063eaf0` | 312 | high |
| `0x8076FC60` | `fn_8076fc60` | 312 | high |
| `0x80799200` | `fn_80799200` | 312 | high |

## Corrections and notes

- `0x8061E980` and `0x8063EAF0` dispatch through a vtable pointer stored in the
  object (`+0x78` and `+0xB4`) instead of at offset `0`, so the Rust body loads
  the slot address and calls it with `abi_call3`.
- `0x80758010`'s `(byte)(*(char*)(...(param_1 + 0x128) + 4) + 0x151) - 2U) < 2`
  wraps, so the Rust body uses `wrapping_sub(2) < 2` over the byte.
- `0x80799200`'s `0x80798D40`/`0x80798E20`/`0x80798F10` chain has no register
  results except the state integer; the final `0x80C561E0` flag is
  `present != 0`.
- `0x8076FC60`'s fourth `0x80105FD0` argument is an integer word at
  `0x80816C44`, not a float, so it is passed with `read_u32`.

## Verification

- `cargo check` succeeds.
- `node tools/check-lane-csv.mjs wave170` validates the eight-row lane CSV.
- All eight evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`.
