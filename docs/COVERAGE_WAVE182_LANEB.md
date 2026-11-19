# Coverage - Wave 182 lane B (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 10 | 868 | 8 | 2 |

Ten sub-300-byte bodies spread over four already-mapped families in
`d_basesNP`: the results-screen (`dPointResultMulti_c`) bucket, the unbounded
`0x1A500`-byte UI actor, the credit screen, and the `waterbubble` /
treasure-box effect actors. None of the ten needed a new helper; two calls
(`0x802DCBFC` and, indirectly, `0x8063A740`) stay address-only.

## What this pass covers

- **Results-screen transition (`0x8063D260`).** Runs the score-table
  preparation `0x8063C090` and raises
  `dPointResultMulti_c::StateID_PlayCountSave` (`0x808273E0`, registered by
  `fn_8063d7a0` into the twenty descriptor slots) on the `+0x208` state
  machine. This is the transition that feeds the already translated
  `fn_8063d2b0` (PlayCountSave handler).
- **Scene/pad idle probe (`0x8063A560`).** Same three-probe shape as the
  already translated `fn_80623260`: `0x8006C5C0` must succeed,
  `0x800B3B50(-1)` must return `0`, and `0x80600510` must return `0`; the
  `cntlzw`/`rlwinm 27,5,31` fold is the `== 0` answer.
- **Record upload sweep (`0x8063A850`).** Four outer iterations of three
  `0x8063A740(this, column)` calls. The outer counter is provably dead in the
  raw words, so the twelve calls are exactly `column` `0..3` four times;
  `0x8063A740` itself cascades the 28-byte record at `this + column*0x1C`.
- **Credit screen column bind (`0x80640AC0`).** Ghidra gave up on this body
  ("bad instruction data") and an earlier wave had to call it address-only from
  `fn_80640b40`. The raw words decode completely: `0x80354E60[column]` is
  permuted `0->0, 1->1, 2->3, 3->2`, the seat's handle is read from
  `+0x248 + seat*4`, and `0x800B3B60(this + column*4 + 0x288, 0x807E1C9C,
  handle, 1)` tail-binds the column's score word.
- **Phase halfword advance (`0x80648710`).** `lha`/`addi 0x200`/`sth` on
  `+0x1A4EA`, the `psq_l ...,1,3` GQR3 unsigned-16 sample, a `1/256` scale, the
  sine helper `0x80237BD0` and a final `4.0` scale.
- **`waterbubble` release wrapper (`0x8064B660`).** `+0x3B4` model slot `+0x1C`,
  the `0x802DCBFC` trampoline with `r12 = this + 0x42C`, then `0x8006CD40`.
- **Treasure-box wait-effect re-emit (`0x8064D150`).** `+0x3F0` slot `+0x1C`
  then `fn_8064cf50(this, 1)` (tail effect suppressed) - the `amount`-style
  sibling of `fn_8064d0a0`, which passes `0`.
- **Timer fraction (`0x806505B0`).** Two `0x8006A230` calls (first discarded),
  low halfword masked with `rlwinm ...,0,16,31`, the `0x43300000`/`2^52`
  widening, then `3600.0 / sample`. This is the span helper already called as
  `call_f32_0(0x8065_05B0)` from `fn_8064eeb0`.
- **Sound 0x15F/0x160 (`0x80653B60`, `0x80653BC0`).** Identical 88-byte bodies:
  a 28-byte transform from `0x8006CED0`, an 8-byte payload from `0x8006A4B0`,
  and `0x80198C30(singleton@0x80429D60, id, payload, 0)`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8063A560` | `fn_8063a560` | 84 | high |
| `0x8063A850` | `fn_8063a850` | 100 | high |
| `0x8063D260` | `fn_8063d260` | 72 | high |
| `0x80640AC0` | `fn_80640ac0` | 116 | high |
| `0x80648710` | `fn_80648710` | 84 | high |
| `0x8064B660` | `fn_8064b660` | 84 | medium |
| `0x8064D150` | `fn_8064d150` | 72 | high |
| `0x806505B0` | `fn_806505b0` | 80 | medium |
| `0x80653B60` | `fn_80653b60` | 88 | high |
| `0x80653BC0` | `fn_80653bc0` | 88 | high |

## Deferred

None. `0x80640AC0`, previously listed as opaque, is translated here.

## Corrections and notes

- **`0x80640AC0` is not bad instruction data.** The export truncates at
  `halt_baddata()` only because Ghidra's jump-table recovery failed. The words
  `0x80640AC0..0x80640B30` are a plain four-way `cmpi` chain with results
  `0/1/3/2` and a default that keeps the table value, followed by a `b` tail
  call. `0x80354E60` is the established main.dol player/seat table.
- **`0x8063A850`'s outer loop is dead.** `r31` is incremented and compared
  (`cmpi cr0,31,r31,4`) but never reaches `0x8063A740`; only the inner `r30`
  is moved into `r4`. The body therefore issues `(this, 0..2)` four times.
- **`0x8063D260` drops an argument in Ghidra.** `FUN_8063c090` is typed
  `void(void)`, but the callee immediately uses the incoming `r3`
  (`mr r27,r3`, then `+0x28C` and the 168-byte row stride), and `8063d260`
  reaches the `bl` with `r3 = this`. The call passes `this`.
- **`0x802DCBFC` in `0x8064B660`.** It is the unresolved indirect-jump stub
  documented in the module header: it reads `{offset, index, target}` from
  `r12`, which this wrapper sets to `this + 0x42C` (`addi r12,r31,1068`), the
  three-word descriptor block that `fn_8064b1c0` fills from
  `0x802FD328..0x802FD330` and closes with `0x3C`. Those main.dol words are not
  in this REL, so the dispatch is an address-only `abi_call1!(0x802D_CBFC,
  this)`; Rust cannot supply `r12`.
- **`0x806505B0` ladder.** The shipped sequence is `lis 0x4330` / `stw` /
  `stw` / `lfd` / `lfd 0x807C8720 (=2^52)` / `fsubs`. The block is written as
  the plain `u16 as f32` widening used everywhere else in this module for the
  same instruction pair; a strict single-precision reading of the two `fsubs`
  operands would subtract `2^31` from `2^31`. The division by a zero sample is
  left as-is, matching the `fdivs`.
- **`0x80648710` signed-to-unsigned reduction.** The `lha` sign only matters
  before the `sth`, so the body reads `read_u16(this + 0x1A4EA)` and adds
  `0x200` with `wrapping_add`, identical to the reduction documented for
  `fn_80734e30`.
- **Stack buffers in `0x80653B60`/`0x80653BC0`.** The frame is `0x30` bytes with
  `r31` saved at `r1+0x2C`, so the two objects span `r1+0x10..0x2C` (28 bytes,
  `[u32; 7]`) and `r1+8..0x10` (8 bytes, `[u32; 2]`). Ghidra's 28/8 split is
  consistent with that.

## Verification

- All ten bodies were compiled standalone against the real helper shapes
  (`platform::read_u16/read_u32/read_f32/write_u16`, `abi_call1!`/`abi_call2!`/
  `abi_call4!`, `call0_ret`, `vmethod0`/`vmethod1`, `f32_of_f32`,
  `fn_8064cf50`) with `rustc --edition 2021 --crate-type lib --emit=metadata`:
  no errors. `src/game/bases.rs` was not modified; the coordinator's merge step
  appends `work/wave182/laneB_block.rs`.
- Every branch, offset, vtable slot and descriptor was checked against the raw
  words at the listed addresses, including the two 88-byte sound twins (only
  `li r4,351` vs `li r4,352` differ) and the unsupported `0x80640AC0` body.
