# Coverage - Wave 180 lane E (`game::bases`, `0x8063BA50`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane E block) | `game::bases` | 1 | 324 | 1 | 0 |

One body from the REL `d_basesNP`: the per-player result placement writer that
wave 81 left as an address-only call ("paired-single register state not
recoverable"). Source of the body: `work/wave180/laneE_block.rs`; per-function
ledger: `ledger/agents/wave180-lanee.csv`.

## What this pass covers

- **Per-player result placement (1 body).** `0x8063BA50` walks the four
  0x1C-byte slot descriptors off `screen+0x248`; the index at descriptor
  `+0x2E0` selects a record from the 4-byte pointer array at `+0x24C`, and
  records whose `+0xBB` bit 0 is set are placed: the pane from the `+0x250`
  array is chosen with the running ordinal returned by [`fn_8063b8d0`], then
  `record+0x2C = pane[0x90] + screen[index*4 + 0x940]`,
  `record+0x30 = pane[0xA0]` (raw word) and `record+0x34 = 0.0`. The tail copies
  the byte `*(screen+0x274)+0xB9` into `table[0x248]+0xB8`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8063BA50` | `fn_8063ba50` | 324 | high |

## Deferred

None.

## Corrections and notes

- **The wave-81 deferral was caused by a scalar `f31` spill, not by paired
  math.** `0x8063BA64`/`0x8063BB74` are `psq_st`/`psq_l f31,0x68(r1),0,0` on
  GQR0 with `W=0`, i.e. a plain 4-byte `f32` save of `f31`; the
  `unaff_GQR0`/`ldexpf` guards in the export are the usual compiler-save noise
  and emit nothing. No `psq` arithmetic exists in the body.
- **`f31` is `0.0`.** `0x8063BA74` loads `lfs f31,-0x7FA8(r4)` with
  `r4 = 0x807D0000`, i.e. `0x807C8058`, which is `0x00000000` (read from the
  ELF). Its only live use is `stfs f31,0x34(record)`; the earlier
  `stfs f31,0x10(r1)` is scratch.
- **The stack staging block is dead.** `0x8063BB1C`..`0x8063BB48` stores
  `record`-side words (`+0x84`..`+0xB0`) to `r1+0x18`..`r1+0x44`; the only
  reloads (`0x8063BADC` from `r1+0x24`, `0x8063BB10` from `r1+0x34`) happen
  *before* those stores, and the epilogue only touches `r1+0x60`/`0x68`/`0x74`.
  Nothing is emitted for them (Ghidra flags the region as unreachable).
- **`+0x30` is a bit copy.** `0x8063BAF8` is `lwz r6,0xA0(pane)` and
  `0x8063BB18` is `stfs f0,0x30(record)` where `f0` was reloaded from that raw
  word, so the translation uses `read_u32`/`write_u32` rather than a float
  round-trip.
- **`r26` is reused after the offset load.** `0x8063BAC0` reads
  `0x940(r3)` with `r3 = screen + index*4` (computed at `0x8063BAB8`, before
  `r26` is overwritten by the pane pointer at `0x8063BAC4`), so the offset base
  is `screen`, not the `+0x248` table.
- **Loop-invariant hoist.** The raw loop re-loads `screen+0x248` every
  iteration (`0x8063BA84`); the translation reads it once before the loop.
  Nothing in the body writes `screen+0x248`.

## Verification

- The lane-E block type-checks standalone (`rustc --edition 2021
  --emit=metadata --crate-type lib`) against stubbed copies of
  `platform::read_u8/read_u32/read_i32/read_f32/write_u8/write_u32/write_f32`
  and `fn_8063b8d0`; the harness lives outside the repo in the session temp
  directory (`%TEMP%/opencode/w180e`).
- The evidence export exists at
  `work/ghidra/rel-d_basesNP/export/8063ba50_FUN_8063ba50.c` (324 bytes in
  `functions.tsv`, matching `0x8063BA50..0x8063BB90` plus `blr`).
- Every offset, branch and constant was re-read from the raw words of
  `work/rels/elf/d_basesNP.elf` (`0x807C8058` = `0x00000000` = `0.0`).
- Only helpers that already exist in `src/game/bases.rs` are used: the same
  `platform::read_*`/`write_*` set as the neighbouring wave-81 bodies and the
  translated [`fn_8063b8d0`] (`screen`, `column`).
- No `psq_l`/`psq_st`, GQR, `_savegpr`/`_restgpr` or `ldexpf` artifact is
  emitted.
- `src/game/bases.rs` was not modified by this lane.
