# Coverage - Wave 185, lane C (`game::bases`, sub-300-byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 984 | 10 | 0 |

All ten lane-C addresses are 76-120 bytes in
`work/ghidra/rel-d_basesNP/export/functions.tsv`. Lane C added no helpers and
did not touch `src/game/bases.rs`; the block is in
`work/wave185/laneC_block.rs`, the ledger is
`ledger/agents/wave185-lanec.csv`, and the bodies are appended by the
coordinator.

## Translated

- `0x80757280` (120 bytes) - ride-slot liveness check. A signed byte at
  `+0x63D` selects a record in the `0x8042_A320 + slot * 0x23C` player table;
  `-1` clears `+0x60C`, and a non-zero `+0x60C` plus a record whose `+0x381`
  byte is zero or `+0x368` word is `14` ends in the tail branch
  `0x800B_3600(slot, 14)`.
- `0x80757E10` (100 bytes) - deleting destructor: `0x8015_F9F0(this + 0x18, 0)`
  and, when `free > 0`, `0x802B_9280(this)`; returns `this`. Byte-for-byte the
  same template as [`fn_806357e0`]/[`fn_80635840`].
- `0x8075AEF0` (104 bytes) - ride pose reset: [`fn_8075b040`], position copied
  from `+0xAC..+0xB4` to `+0x128..+0x130`, `100.0` at `+0x134`, class vtable
  `+0x60` slot `+0xC8`; returns `1`.
- `0x8075C010` (96 bytes) - returns `1` when `0x800F_C140(world, slot)` reports
  zero for slot `0` or `1`, else `0`.
- `0x8075C1C0` (104 bytes) - the setter twin: writes state `4` through
  `0x800F_C190(world, slot, 4)` for the first slot whose probe returns zero.
- `0x8075F9E0` (84 bytes) - if `0x8016_2D50(0x272, 0)` finds the profile
  `0x272` object, calls `0x8075_FA40` with that object and the two incoming
  floats.
- `0x80762BF0` (80 bytes) - [`fn_80762ce0`], `0x80762EA0(this)`, `1.0` stored
  at `+0xDC/+0xE0/+0xE4`, then [`fn_80762c40`] (zeroes `+0x254`).
- `0x80764350` (76 bytes) - in-place constructor: `0x800F_2710(this)`, vtable
  `0x8081_58C0` at `+0x60`, `0x8006_9020(this + 0x138)`,
  `0x8016_A2F0(this + 0x154)`; returns `this`.
- `0x80764C00` (120 bytes) - dispatches virtual `+0x1C` on the four sub-objects
  at `this + 0x1A8 + i * 0x40` and then on `this + 0x38C`. Neighbour
  [`fn_80764c80`] re-uses both address families.
- `0x8076B560` (100 bytes) - the second ride family's pose reset:
  [`fn_8076b710`], position copy, `120.0` at `+0x134`, the empty
  [`fn_8076ba60`], then `0x8076B9B0(this)`; returns `1`.

## Deferred

None.

## Notes and limitations

- `0x80757280` reads the player table through `lwz`, so the translation
  dereferences `0x8042_A320` before adding `slot * 0x23C`, matching the
  `platform::read_u32(0x8042_A320)` reads already used elsewhere in
  `src/game/bases.rs`. `slot` is sign-extended with `extsb` and `-1` is handled
  before any table access.
- `0x80757E10` repeats the `r3 != 0` test at `0x80757E30` and `0x80757E34`
  (both `bc 12,2` on the same `cr0`); the second is dead and folds into the one
  guard.
- `0x8075F9E0` was deferred by wave 106 because Ghidra typed it with two
  `undefined8` parameters. The raw words resolve that: `f1`/`f2` are saved to
  `f30`/`f31`, `0x80162D50(0x272, 0)` is called, and the still-live `r3`
  (that lookup result) is the first argument of the `bl 0x8075FA40` at
  `0x8075FA18`. The host wrapper `call_f32x2_u32` carries the owner last while
  the guest prototype keeps it in `r3`; this helper order is the only modelling
  compromise in the lane.
- `0x80764350`'s base constructor and `0x80069020` call show Ghidra arguments
  as the raw `bl` leaves them; `r3 = this` and `r3 = this + 0x138` are passed
  explicitly, matching the wave-184 constructor convention. The `r4` holding
  `0x8081_58C0` in the second call is the just-stored vtable value and is not a
  second argument (the same dropped-`r4` shape as [`fn_8075a6c0`]).
- `0x80762BF0` and `0x8076B560` call callees that have no Rust body
  (`0x80762EA0`, 312 bytes; `0x8076B9B0`, 176 bytes); both are emitted as
  `abi_call1!` with the observed `this`. `0x8076BA60` is the 4-byte `blr` that
  is already [`fn_8076ba60`]; the raw `bl` passes `this` in `r3`, which the
  empty body ignores.
- All constants were re-read from the ELF: `0x807C_EB98 = 100.0`,
  `0x8081_5494 = 1.0`, `0x807C_F0C8 = 120.0`. `0x8081_58C0` is a REL vtable in
  the relocation-filled data section (zero in the file) and is stored literally.
