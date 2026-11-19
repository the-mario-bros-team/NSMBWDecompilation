# Coverage - Wave 110 (`game::worldmap`, `0x80100000` bucket remainder)

## How to read this file

Each row is one reconstructed function in `src/game/worldmap.rs`. The address is
the original `main.dol` address, the byte count is taken from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence file is the Ghidra
C export used for the body. `medium` confidence means the instruction sequence
is literal but one or more data roles are inferred.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 3 | 3,284 | 0 | 3 |

All three rows are `translated`; none is a `stub` or `partial`.

## What this pass covers

At the start of this pass the `0x80100000` bucket had seven untranslated
entries. Three are written here; the four deliberate skips are listed below.
The pass covers two clusters:

- **`0x80103410` - the slot-allocator thunk.** A 16-byte tail branch onto
  `0x80103440` (`wm_effect_slot_alloc`). Because a
  frame-free tail branch forwards all argument registers unchanged, the Rust
  wrapper uses the target's already-reconstructed `(base, tag, p3..p7)`
  signature; no new parameter is invented. Ghidra prints the call with no
  operands because it could not recover the dropped registers.
- **`0x8010B890` - the `dGoalManager_c` state table.** Copies three SDA words
  (`r2 - 0x5D88`/`-0x5D84`/`-0x5D80`) and the shared word `23`
  (`r2 - 0x6114`) into four goal-manager globals, registers
  `0x8010BF10`/`0x80374C78` with `0x802DC780(0, code, data)`, then builds eight
  `0x40`-byte records at `0x80374C90..0x80374E8F`. Each record is constructed
  by `0x8015F7C0(record, name)`, receives the `0x8032269C` vtable, copies nine
  handler words from `0x80322450 + n*0x24` into `+0x0C..+0x2C`, and registers a
  12-byte descriptor at `record - 0x10`. The first descriptor is at
  `table - 0x0C`, where there is no preceding record. The state names printed
  by the export identify `StartWait`, `KiMePose` (twice), `MessageOpen`,
  `ScoreCalc`, `MessageDone`, `MessageEnd` and `ClearWait`.
- **`0x8010E830` - the `dWarningManager_c` state table.** The same eight-record
  geometry at `0x80374EC8..0x803750C7`, with templates at
  `0x80322A18 + n*0x24`, vtable `0x80322C94` and handler `0x8010EE80`. It also
  contains a one-shot guard: when the byte at `r13 - 0x539C` is zero, two SDA
  words are copied to `0x80322880`/`0x80322884` and the guard is set to `1`.
  The export truncates every state label to `dWarningManager_c::StateID_Warni`,
  so only the class name is claimed.

The copy order inside a record is genuinely rotated: the source words are read
in the order `+0x18`, `+0x1C`, `+0x20`, `+0x0C`, `+0x10`, `+0x14`, `+0x00`,
`+0x04`, `+0x08` and stored at destination `+0x0C..+0x2C`. The Rust helper
`w110_fill_state_record` preserves that order and the constructor-before-store
sequence.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80103410` | `fn_80103410` | 16 | medium |
| `0x8010B890` | `fn_8010b890` | 1652 | medium |
| `0x8010E830` | `fn_8010e830` | 1616 | medium |

The three sizes sum to 3,284 bytes.

## Deliberate skips

**`0x80107050` (280 bytes), `0x80107170` (320 bytes), `0x80107460` (580
bytes).** The first two are C-variadic formatters: they rebuild a Metrowerks
`va_list` from the caller's home area (`stack0x00000008`, the GPR save block at
`local_488`/`local_498` and the `0x02000000`/`0x03000000` descriptor words) and
pass it to `0x802E15DC`, `0x8022DFB0` and `0x8022E1C0`. Stable Rust cannot
define a C-variadic function, and the home-area layout is not recoverable from
the C export. `0x80107460` calls both formatters, so it inherits the loss.

**`0x801097F0` (344 bytes).** The export's frame is internally inconsistent:
the layout object initialised by `0x802D0CC0` is `0x58` bytes (`+0x00..+0x54`),
but Ghidra splits it into `local_90`, `local_8c`, `local_c8`, `local_c4`,
`local_d0`, `local_d8` and a 48-byte `auStack_c0`. Those names therefore alias
one record and its scratch rectangle, and `local_8c`/`local_5c` are read before
any write. Reproducing the body would mean guessing the real object boundaries
and initial values, so it is left out rather than invented.

## Notes for the coordinator

- `0x802DCD74`/`0x802DCDC0` are the compiler GPR save/restore pair. They are
  called for fidelity with the export but have no game semantics.
- `0x80322450..0x8032256F` and `0x80322A18..0x80322B37` are data-segment
  handler templates. The individual word roles are not visible in any export;
  the bodies keep them as plain words with `unknown:` notes.
- `0x8010BF10` is not a function entry in `functions.tsv`; it is carried as a
  raw code address because the export prints it as `LAB_8010bf10`.
- All three bodies reuse the existing `SDA_BASE`/`SDA2_BASE` bases and the
  existing `R2_6114_23` constant. No existing constant was redefined.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- None of the three addresses was in `ledger/functions.csv` at the start of the
  pass; all three were untranslated entries of the `0x80100000` bucket.
- `cargo check --message-format=short` reports no error for
  `src/game/worldmap.rs`.
