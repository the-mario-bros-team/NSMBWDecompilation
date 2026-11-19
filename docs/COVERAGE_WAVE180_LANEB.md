# Coverage - Wave 180 lane B (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 2 | 640 | 2 | 0 |

Two bodies from the 300-332 byte slice of the band, the last GQR/`psq`-heavy
pair assigned to this lane. Source of the bodies: `work/wave180/laneB_block.rs`.

## What this pass covers

- **`test_lift` model loader (1 body).** `0x806E3FC0` (316 bytes) is the
  four-slot asset body of the `test_lift` lift actor whose constructor/teardown
  and rail-angle writer (`0x806E3970`, `0x806E3CE0`, `0x806E3D80`, and the
  still-untranslated `0x806E3EB0`) already describe the same field layout.
  Each of four iterations writes `(3.0, 0.5, 1.0)` from
  `0x807CBC50`/`0x807CBC54`/`0x807CBC58` into the `0xC`-byte triple at
  `+0x5B4 + 0xC*i`, opens the `0x1C`-byte accessor at `+0x434 + 0x1C*i` with
  `0x8016A7F0`, loads `g3d/test_lift.brres` (`0x807FCDD8`) under the name
  `test_lift` (`0x807FCDEC`) from archive manager `0x8042A038` via
  `0x800DF180`, stores the resource word at `+0x4A4 + 4*i`, resolves the pane
  with `0x80239E30` and binds it into the `0x40`-byte block at
  `+0x4B4 + 0x40*i` through `0x80169D90` (priority `0x20`), `0x80064BF0`, and
  closes the accessor with `0x8016A8E0`.
- **Split-screen result placement writer (1 body).** `0x8063B900` (324 bytes)
  is the `+0x244` twin of the already translated `fn_8063ba50` (`+0x248`).
  Four slots: index from the `0x3C`-stride array at `+0x7E0`, record from the
  `4`-stride array at `+0x6EC`; for each record with `+0xBB` bit 0 set it
  writes `+0x2C = pane[0x90] + screen[index*4 + 0x940]`,
  `+0x30 = pane[0xA0]`, `+0x34 = 0.0`, taking the pane from `+0x250` by the
  running ordinal of `fn_8063b8d0`. The tail copies
  `*(screen + 0x274) + 0xB9` to `*(*(screen + 0x244) + 0x6E8) + 0xB8`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806E3FC0` | `fn_806e3fc0` | 316 | high |
| `0x8063B900` | `fn_8063b900` | 324 | high |

## Deferred

None.

## Corrections and notes

- **Qr0 scalar spills only.** Both bodies open with
  `stfd`/`psq_st f29..f31` (or `f31`) and close with the matching
  `psq_l`/`lfd`. Those `psq_st`/`psq_l` words carry `qr0` and `W=0`, so they are
  plain `f32` callee-save spills (`0xF3E10068` = `psq_st f31,0x68(r1),0,0`).
  No `qr3` sample, no `psq_l` type-6 pair and no dequantise step occurs in
  either body; the `unaff_GQR0` bit tests and every `ldexpf` call in the two
  exports are the established GQR0 compiler noise and are not modelled. This
  resolves the wave-109 deferral reason for `0x806E3FC0` (see
  `docs/COVERAGE_WAVE109_BASES.md`): the incoming GQR state is not needed.
- **`0x802DCD5C` is not a getter.** `0x806E3FC0` begins with
  `bl 0x802DCD5C`, which sits in the `_savegpr_N` range and preserves `r3`; the
  body's `mr r26,r3` / `addi r25,r3,0x434` therefore read the incoming actor
  pointer. The body is modelled as `fn_806e3fc0(this: usize)` rather than the
  older `call0_ret(0x802D_CD5C)` convention. (Older waves modelled the same
  fragment as a getter, e.g. `fn_80607740`; here the parameter form matches the
  brief's "`r3` survives" rule. The exact `_savegpr_N` rank is not claimed: the
  body touches `r21..r31`, so the stub is at least `_savegpr_21`.)
- **`0x8063B900` has two counters.** `r28` is the four-iteration slot counter
  (`+0x3C` stride) and increments unconditionally; `r29` is the *ordinal* of
  active records and increments only inside the `+0xBB` bit-0 branch. That is
  the argument pair of `fn_8063b8d0(screen, ordinal)`, exactly as in
  `fn_8063ba50`.
- **Dead stack staging.** The compiler stages twelve words from
  `pane + 0x84..0xB4` into `0x18..0x44(r1)` (plus `0x8(r1)`/`0xc(r1)`/
  `0x10(r1)`), but the only slots ever read back are `0x24(r1)` and `0x34(r1)`,
  which hold the `pane + 0x90` and `pane + 0xA0` words. There is no call after
  `0x8063B960` and none of the other staged slots is loaded, so they are
  unobservable and omitted; Ghidra marks the following block unreachable for
  the same reason. The `pane + 0xA0` copy is emitted as
  `write_f32(record + 0x30, read_f32(pane + 0xA0))` (bit-for-bit).
- **`rlwinm` indices.** `rlwinm r26,r0,2,0,29` and `rlwinm r0,r3,2,0,29` are
  `index * 4` with the low two bits cleared after the shift, i.e. a plain
  `index as usize * 4` for any in-range record index; both indices are read
  with `read_i32` to match the twin and `fn_8063ce40`.
- **Opcode 59 (`fadds`).** The only A-form word in these bodies is
  `0xEC21002A` = `fadds f1,f1,f0`, decoded with the two-operand rule (FRB at
  bits 16-20 = `f0`). There is no `fmuls`/four-operand word in this lane.
  `0x4BBF8D75` / `0x4BBF8CC1` / `0x4BCA1459` / `0x4BCA138D` are the
  `_savegpr`/`_restgpr` helper calls and are not emitted.

## Verification

- Both bodies were type-checked standalone against the real helper shapes
  (`platform::read_u8/u32/i32/f32`, `platform::write_u8/u32/f32`,
  `abi_call1!`..`abi_call6!`, `fn_8063b8d0`) with `rustc --edition 2021
  --crate-type lib`: no errors. `src/game/bases.rs` was not modified; the
  coordinator's `merge.cjs` appends the block.
- The evidence exports exist at
  `work/ghidra/rel-d_basesNP/export/806e3fc0_FUN_806e3fc0.c` and
  `work/ghidra/rel-d_basesNP/export/8063b900_FUN_8063b900.c`. The bodies are
  exactly `0x806E3FC0..0x806E40FC` (316 bytes) and `0x8063B900..0x8063BA44`
  (324 bytes); every constant (`0x807CBC50` = 3.0, `0x807CBC54` = 0.5,
  `0x807CBC58` = 1.0, `0x807C8058` = 0.0) and both strings were read from
  `work/rels/elf/d_basesNP.elf`, and every branch (`blt` back edge over
  `cmpwi ...,4` in both loops, `beq` on `+0xBB & 1`) was verified against the
  raw words.
