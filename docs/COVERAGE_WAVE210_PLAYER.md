# Coverage - Wave 210 (`game::player`, main.dol part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 60 | 9,872 | 47 | 13 |

First 60 main.dol bodies owning `src/game/player.rs`, addresses
`0x80060110`-`0x800630B0`, assigned by nearest translated neighbour.

## What landed

- **Roster and counter bodies (lane A).** `0x80060110` (character id to slot
  index), `0x80060170`, `0x80060200` (four-slot roster tally), the 520-byte
  `0x80060250` per-frame roster driver, `0x80060460` (counter add clamped to
  99), `0x80060590`, `0x80060600`, `0x80060690` (clamped to 999999999), and
  the sound resume/pause pair `0x80060700`/`0x80060720`.
- **Effect and state bodies (lane B).** `0x80060750`, `0x800607D0`,
  `0x80060830`, `0x80060860`, `0x800608E0`, the 344-byte `0x80060AB0`, the
  416-byte `0x80060C10`, the 312-byte `0x80060DB0`, `0x80060EF0`, `0x80060F20`.
- **Slot bookkeeping (lane C).** `0x80060FE0`, `0x80061080`, `0x800610E0`,
  `0x80061110`, `0x80061160`, `0x800613B0`, `0x80061540`, `0x80061590`,
  `0x800616A0`, `0x800618B0`.
- **Lookup and placement (lanes D/E).** `0x80061950`, `0x80061A80`,
  `0x80061D50`, `0x80061DB0`, `0x80061EF0`, `0x80061FD0`, `0x80062090`,
  `0x800620C0`, `0x800620F0`, `0x80062260`; plus `0x800622A0`, `0x80062300`,
  `0x80062310`, `0x80062320`, `0x80062380`, `0x800623E0`, `0x80062420`, the
  196-byte `0x80062450`, `0x80062520`, `0x800625A0`.
- **Dokan block (lane F).** `0x80062600`, `0x80062610`, `0x80062B60`,
  `0x80062BC0`, `0x80062D00`, `0x80062DE0`, `0x80062EA0`, `0x80062ED0`, the
  368-byte `0x80062F00` spawn/init and the 336-byte `0x800630B0`
  `g3d/obj_dokan.brres` build.

## Corrections and recoveries

- **Size artefacts fixed.** `0x80060AB0` = 344 and `0x80060DB0` = 312 (both
  listed as 20; the 20 is the `psq_st f31` prologue save),
  `0x80062450` = 196 (listed 20), `0x800616A0` = 384 (listed 16).
- **`0x80061540` is 80 bytes, not 436.** The body is four `b` tail calls to
  `0x800613C0/1420/1480/14E0`; Ghidra inlined those scans into its export and
  inflated the size. The Rust follows the disassembly.
- `0x80062090`/`0x800620C0` are consecutive 12-byte Metrowerks
  pointer-to-member records at `+0x18`/`+0x24`, dispatched through the
  `0x802DCBFC` thunk. Lane D confirmed that thunk is a PMF dispatcher
  (`add this`, optional vtable double-indirection, `bcctr`), matching the
  `{self_adjust, index, table}` model used for the 473 `game::bases` thunks.
- `0x800620F0`'s Ghidra `double` parameters are an artefact: all operations are
  single precision (`0.5`, `-0.5`, `1.5`, `0.0` at `0x8042BAA8..0x8042BAB4`).
- `0x80062450`'s `ldexpf`/GQR noise is `psq_l` widening of a signed 16-bit
  angle; `0x80237C40` = cos, `0x80237BD0` = sin, cross-checked against
  `engine::display`.
- `0x800608E0`'s `((n^1)>>1)-((n^1)&n) < 0` is the `n >= 2` idiom, and
  `0x80060750` gates on `+0x1070 >= 60` (not `> 60`).
- `0x800610E0` reproduces PowerPC `slw` modulo-64 shift semantics rather than
  a naive `1 << (x & 0xFF)`.
- `0x80062F00`/`0x800630B0`: `+0x38D` is `params & 3` here, unlike the
  neighbouring dokan spawn's `(params >> 12) & 3`, and the name tables
  (`0x80309C54`, `0x80309C6C`) differ from the `actors::dokan` pair - a
  distinct class despite the similar shape.
- Four previously unnamed globals resolved from the image: `0x80427904` and
  `0x80427900` (99), `0x80427908` (999999999), `0x8042BA98` (`0.5f`).
- `0x80060250` is `void f(void)` in Ghidra only because of the `0x802DCD74`
  save stub; caller `0x8006D170` passes the character index, so the body takes
  a `slot`.
- Two `0x802DCBFC` forwarders (`0x80062EA0`, `0x80062ED0`) keep the
  module-wide `// partial:` convention for the hidden `r12` descriptor.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave210`: 60 rows, 60 unique, 47 high + 13 medium.
- Ledger: 19,437 rows (95.98%); `game::player` 597/818 (72.98%).
- `scripts/check.ps1` exit 0.

## Next

221 `game::player` bodies remain, then `src/engine/dsys.rs` (238),
`src/resource/archive.rs` (178), `src/game/stage.rs` (128) and smaller files.
