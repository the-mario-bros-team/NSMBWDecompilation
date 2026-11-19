# Coverage - Wave 178 lane C (`game::bases` 464-472 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 4 | 1,876 | 4 | 0 |

All four bodies translated, none deferred.

## Lane C (4 bodies, 1,876 bytes)

- `0x8078AF50` - (464) scripted-screen command handler, sibling of
  `fn_807886e0`: forwards to that routine, gates on the `+0x64` virtual and the
  `+0x65C` once-only latch, then runs the `run_first` phase over the `+0x444`
  interface and the always-on phase that either retires the command through the
  `+0x68` virtual or sets the finished byte `+0x139`.
- `0x80774880` - (468) castle loader: matches the record's name against the
  eleven-entry `{char *name; u16 profile}` table at `0x807CF3C0`, seeds a
  five-word position through `0x801006C0`, hashes the committed name with
  `0x800F31E0` inside the per-world arena
  `this + [this+0x338C] * 0x3F08 + 0x5C7F4`, spawns through `0x800F2A00` and
  writes the handle's `+0x184`. Direct predecessor of the already-translated
  fallback `fn_80774a60` (`0x807CF418`, eight entries).
- `0x8066E0C0` - (472) key-gate wall driver: per masked-in player it advances
  the segment state at `+0x41C + i*4` from `1` to `2` to `3`, adding then
  clearing player move-state `0x104` and publishing `1.5` at
  `this + selector*4 + 0x3F0` through `0x80052080`; returns the `and`-folded
  "all segments reached" flag.
- `0x806DFA60` - (472) lift seat recount: walks the two child lists
  `this + 0x484 + slot*0x60` for `slot` in `{2, 3}` (from `0x807CBA88`) plus the
  rider list `+0x4E4`, marks each kind-`1`/`2` actor's `+0x6C` player selector
  in a four-word bitmap, writes the four counters `+0x668..+0x674` and snaps
  `+0x602` into `+0x5F4` when one changed.

## Corrections and cross-checks

- Ghidra's `8066e0c0` export is dominated by `unaff_GQR0`/`ldexpf`
  "dequantise" blocks and four dead `local_*`/`fStack_*` pairs. They are the
  `psq_st f31/f30/f29/f28`/`psq_l` saves of the callee-saved FPRs (the frame
  stores at `0x38`/`0x48`/`0x58`/`0x68`), not arithmetic; the whole block was
  dropped. The real constants are `0x807C8FD0` = 168.0, `0x807C8FD4` = 112.0,
  `0x807C8FD8` = 1.5 and `0x807C8FDC` = 48.0, read from the ELF.
- `806dfa60`'s `lha`/`sth` pair at `0x602`/`0x5F4` is a halfword move (Ghidra
  prints `undefined2`); the Rust uses `read_u16`/`write_u16`.
- Both `806dfa60` and `8066e0c0` print as `void FUN_xxxx(void)` in Ghidra and
  lost the `r3` receiver: the `bl 0x802dcd70` bracket helper passes `r3`
  through untouched, so both bodies take `this: usize` explicitly (and
  `8066e0c0` returns the `u32` that `_restgpr` hands back in `r3`).
- `bc 4,2` after `fcmpo` + `cror` folds: in `8066e0c0` the two windows are
  `player[0xAC] >= [this+0x3E8] - 168.0` (state 1) and
  `[this + selector*4 + 0x3F0] <= player[0xAC]` (state 2), matching Ghidra's
  emitted `<`/`<=`, so no inversion was needed.
- `80774880`'s `rlwimi r5,r30,8,16,23` uses MSB-based MB/ME, i.e. the `0xFF00`
  window; the flag word is `(variant & 0xFF) << 8 | (resource & 0xFF)` as
  Ghidra renders it.

## Deferred

- None.

## Verification

- The block type-checks standalone (`rustc --emit=metadata`) against stubbed
  `platform` helpers, macros and the three translated callees it reuses
  (`fn_807886e0`, `fn_8066e2a0`, `fn_80774a60`).
- All four evidence exports exist and their sizes match `functions.tsv`
  (464 / 468 / 472 / 472).
