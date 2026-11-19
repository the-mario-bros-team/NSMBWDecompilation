# Coverage - wave 185 lane E (`game::bases`, `d_basesNP`)

## How to read this file

This is the lane-E slice of the wave-185 sub-300 sweep: ten small bodies from
the PowerPC REL `d_basesNP`, spanning `0x80775080` to `0x80778770`. Every new
body lives in `work/wave185/laneE_block.rs` and is appended to
`src/game/bases.rs` by the coordinator; lane E did not edit `src/game/bases.rs`,
`lib.rs`, any `mod.rs`, `Cargo.toml`, `src/platform.rs`, another lane's files,
`ledger/functions.csv` or any generated document.

- **address** - REL virtual address; the synthetic analysis base is
  `0x80600000`, so these are not runtime addresses.
- **name** - the Rust function name that will exist in `src/game/bases.rs`.
- **role** - one plain sentence.
- **confidence** - `high` when every load/store/branch/constant was read from
  the raw words; `medium` when a callee's role is inferred because it is named
  by address only.

All ten entries were translated; nothing was deferred.

## Summary

| Metric | Count |
|---|---:|
| Bodies translated | 10 |
| Bytes of original code | 1028 |
| `translated` | 10 |
| Deferred | 0 |

## Bodies

| Address | Size | Name | Role | Confidence |
|---|---:|---|---|---|
| `0x80775080` | 128 | `fn_80775080` | Returns the first of the six `-1`-terminated words at `+0x30..+0x44` of the current zone record (falling back to `+0x44`). | medium |
| `0x807754D0` | 108 | `fn_807754d0` | Scans the 192 registry slots of type `0x27E` and ANDs `fn_80761310`'s parity, returning whether all are clear. | high |
| `0x80775800` | 92 | `fn_80775800` | `1` when `0x80817050` (6) equals the `0x8042A24D` byte and `0x801002C0(value, "F7C0")` equals `expected`. | medium |
| `0x80775860` | 100 | `fn_80775860` | Decodes three floats from a record through `0x801002F0`/`0x80100270` and copies them to the caller buffer. | high |
| `0x80776750` | 116 | `fn_80776750` | Course-actor constructor: base ctor, vtable `0x808174B8`, `+0x184`/`+0x1A4` sub-objects and a two-slot `0x38` array at `+0x1E4`. | high |
| `0x80777990` | 96 | `fn_80777990` | Registered destructor: `0x8015F9F0(state+0x18,0)` then `0x802B9280` when `free > 0`. | high |
| `0x807779F0` | 100 | `fn_807779f0` | Byte-twin of `0x80777990` with one redundant `beq`; same destructor. | high |
| `0x80778640` | 84 | `fn_80778640` | Resets the `+0x1EC` float object to `1.0` and stores `0x807CF570[type & 0xFF]` at `+0x4F0`. | high |
| `0x807786E0` | 100 | `fn_807786e0` | As above plus writes the `0.0` at `0x807CF584` into `+0x4F4`. | high |
| `0x80778770` | 104 | `fn_80778770` | Resets at `0.0`, clears `+0x4F4`, and stores `-0x807CF510[type & 0xFF]` at `+0x4F8`. | high |

## Deferred

None.

## Notes and corrections

- `0x807754D0`: Ghidra prints `FUN_80761310()` with no argument, but the
  shipped `bl` still holds the `0x800F9BE0` probe result in `r3`, so the actor
  is forwarded to the existing Rust body `fn_80761310`. The accumulator is
  `neg`/`or`/`rlwinm`, i.e. a nonzero test of `result & alive`; with the
  callee's `0/1` result this is the all-clear conjunction.
- `0x80775080`: the raw word at `0x807750BC` is `addi r3,r3,8`, so the last
  four guards read `+0x38..+0x44` of the original record. Ghidra renders this
  as `+0x38..+0x44` directly, which checks out against the words. The zone
  table (`+0x338C`, stride `0x3F08`, base `+0x5C7F4`) matches
  `fn_80775030`/`fn_80775100`/`fn_80775150`.
- `0x80775800`: the two globals are real REL data - `0x80817050 = 6`,
  `0x80817054 = 0x80817040`, and `0x80817040` holds the NUL-padded label
  `"F7C0"`. The comparison callee `0x801002C0` has no Rust body, so it is
  called by address.
- `0x80776750`: `r4` still holds the vtable descriptor at the `0x80069020`
  call and is forwarded as the second argument, matching the already-translated
  sibling `fn_807760b0`. The `0x802DC9A0` array arguments are `0x806F0060`,
  `0x8064B270`, `0x38`, `2` exactly as in `fn_8075a6c0`.
- `0x80778640`/`0x807786E0`/`0x80778770`: the three bodies share the
  `0x807CF4F8` table base. The index is `rlwinm 2,22,29` on the `+4` type word,
  which is exactly `(type & 0xFF) * 4`. The stores land at `+0x78`
  (`0x807CF570`, `lwz`/`stw`) and `+0x18` (`0x807CF510`, `lfs`) of that base.
  The rate constants are `0x807CF580 = 1.0` and `0x807CF584 = 0.0`. In
  `0x80778770` the `lfs` targets `f0`, not GPR `r0`, so it does not clobber the
  index computed by `rlwinm`.
- `0x80777990`/`0x807779F0`: the second body has a duplicated outer `beq`
  (`0x80777A14`) that changes no behaviour; both keep identical Rust bodies so
  the original call targets still resolve, the same policy used for
  `fn_806357e0`/`fn_80635840`.

## Verification

The block was type-checked with `rustc --edition 2021 --crate-type lib` against
local stubs for `platform` and the `abi_callN!` macros (including the existing
`fn_80761310` body); it compiles with no errors. The stub file and `rlib` were
deleted afterwards.
