# Coverage - Wave 179 lane E (`game::bases`, three 508-512 byte bodies)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane E block) | `game::bases` | 3 | 1,532 | 3 | 0 |

Three bodies from the REL `d_basesNP`: the `Wm_ro_pipeappear` appear state
machine, the pipe rise/spawn body, and the six-step arrival sub-machine that is
the peer of the already translated [`fn_8077d7b0`]. Source of the bodies:
`work/wave179/laneE_block.rs`; per-function ledger:
`ledger/agents/wave179-lanee.csv`.

## What this pass covers

- **`Wm_ro_pipeappear` appear state machine (1 body).** `0x80789790` drives the
  `+0x35C` word through `0 -> 1 -> 2 -> 3 -> 0xE -> 0xF -> 0xB`. It reuses the
  translated broom helpers [`fn_80789b80`] (rate `5.0`, part `2`) and
  [`fn_80789e60`], plus `0x80788CB0` and [`fn_80789a60`] for the ride gate. The
  spawn arm transforms the anchor at `+0x340` with the `(dst, scene, node)`
  helper `0x80100200` (the same one [`fn_8077cfe0`] and [`fn_8079f5f0`] use),
  copies the point to two stack slots, and passes
  `model[+0x14], 0.5, 1.0` to `0x80788B50` with the `fctiwz` of `model[+0x10]`
  as the count — the exact call shape of [`fn_807885d0`]. Both publication
  paths go through `0x80789990` (modes `0` and `1`).
- **Pipe rise body (1 body).** `0x807195A0` is a three-state machine at `+0x69C`
  over the per-`+0x694` tables at `0x808083D4`: the int budgets at
  `0x80808408` (`45`, `16`, …) gate the spawn, the float table at `0x808083EC`
  (`-6.0`, `-4.0`, …) sets the height limit
  `0x8082FF14 + 16.0 * table[idx]`, and `0x806601B0` ramps `90 -> 0` into
  `+0xA08`. The effect object is `+0x6A0` (slot `0xA0`, name
  `"Wm_ro_pipeappear"`), the part object is `+0x584` (slot `0x14`), and the
  final message is posted through the `+0x60` vtable slot `0xD4`.
- **Arrival sub-machine (1 body).** `0x8079F040` is byte-for-byte the same shape
  as [`fn_8077d7b0`] (states `0..6`, the `0x800F6AB0` / `0x80101090` /
  `0x801010B0` gates, the `+0x2C0` countdown) but keyed on `+0x2F4` instead of
  `+0x304`, and its state-5 tail is the `0x80100200` -> `0x80100DA0` ->
  `0x80429C30` transition publication already documented in [`fn_8079f5f0`]
  plus the `5`/`60` scene-exit pair.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80789790` | `fn_80789790` | 508 | high |
| `0x807195A0` | `fn_807195a0` | 512 | high |
| `0x8079F040` | `fn_8079f040` | 512 | high |

## Deferred

None.

## Corrections and notes

- **`0x80789790` state `1` tests a result its callee does not return.** The
  translated [`fn_80789e60`] is `void`, but the raw words at `0x8078981C` are
  `cmpi cr0,3,r3,0` + `beq`, so the caller really does branch on `r3`. The body
  therefore goes through `abi_call2!` (as the neighbouring
  `0x807886E0` translation already does) rather than the typed `fn_`.
- **`0x80789790` state `3` passes `r3` to a nullary export.**
  `0x807898BC` is a bare `bl 0x80788CB0` with `r3` still holding `this`, so the
  translated call keeps the receiver through `abi_call1!`. `0x80789A60` is
  genuinely nullary (`fn_80789a60()`), and its `FUN_80789a60(param_1)` rendering
  in the state-3 arm is Ghidra noise: the raw words only add `mr r3,r29`.
- **`0x80789790` state `0xE` writes a lowered point, not a copy.** The three
  stores are `+0x374 = +0xAC`, `+0x378 = +0xB0 - 150.0`, `+0x37C = +0xB4`, i.e.
  the cached target is 150 units below the object, and the two angle halfwords
  `+0x102`/`+0x108` are cleared in the same arm.
- **`0x80789790` state `0xF` reads an uninitialised stack slot.** `0x80789940`
  passes `r1+0x20`, which only state `2` ever writes, so on the `0xE -> 0xF`
  path the pointer is stale. Mode `1` of `0x80789990` provably never reads
  `param_3` (see `80789990_FUN_80789990.c`, lines 28-38), so the translation
  passes a zeroed buffer and says so, instead of inventing a value.
- **`0x80789790`'s `(longlong)iVar3` local is not live.** `0x8078988C` is
  `fctiwz f0,f0` + `stfd r0,0x30(r1)` and `0x80789894` reloads only the low word
  `0x34(r1)` as `r5`; Ghidra's `local_20` overlaps that scratch double. The
  translation uses `(f32 as i32) as u32`, the same truncation `fctiwz` performs.
- **`0x807195A0`'s prologue/epilogue GQR blocks are compiler noise.**
  `0x807195BC`-`0x807195D8` and `0x80719780`-`0x8071979C` are the
  `unaff_GQR0 & 0x3f00` / `ldexpf` guards around a plain `psq_st`/`psq_l` pair
  of `f31` on GQR0; nothing is emitted for them.
- **`0x807195A0`'s `0x806601B0` returns single precision.** Its own raw words
  load the results with `lfs r1` and divide with `fdivs`, so the export's
  `double` return is a prototype artifact; `call_u32x3_f32` is the right
  wrapper. The two integer pointers are the stack copy of `90`
  (`0x8080843C`) and the zero word at `0x808083BC`.
- **`0x8079F040`'s `0x8079B260` is genuinely nullary.** Its own body
  (`8079b260_FUN_8079b260.c`) never reads `r3` — it opens with
  `_savegpr_26` and reads `0x80429F7C` — so `call0_ret` is used. The existing
  [`fn_8077d7b0`] passes the player-manager global instead; both work, but the
  address is the literal one here.
- **`0x8079F040` state `1` re-arms from a global, not from `+0x294`.**
  `0x8079F0E4`-`0x8079F0F4` is `lis 0x8032 / addi -11364 / lha 0x21A`, i.e. the
  signed halfword `0x8031D5B6` of the `0x8031D39C` table. [`fn_8077d7b0`]'s
  `slot + 0x21A` reads the same offset through its per-slot pointer, so the two
  functions are not using the same source word.
- **`0x8079F040` state `5` keeps `r3` clear across the vector block.** The
  `0x8010_1090` result gates only the publication block; the state always moves
  to `6` once `0x800F6AB0` reports success.

## Verification

- The lane-E block type-checks standalone (`rustc --edition 2021
  --emit=metadata --crate-type lib`) against stubbed copies of the real helper
  ABIs (`platform::read_*`/`write_*`/`call1`/`call2`/`call3`/`call4`/`call5`/
  `call6`, `abi_call1!`..`abi_call4!`, `vmethod0`, `vmethod5`, `call0_f32`,
  `call0_ret`, `call_u32x3_f32`, `call_f32_u32x2`, `call_f32x3_u32x4`,
  `fn_80789b80`, `fn_80789a60`); the harness lives outside the repo in the
  session temp directory (`%TEMP%/opencode/w179e`).
- All three evidence exports exist under `work/ghidra/rel-d_basesNP/export/`;
  every constant, branch and offset was re-read from
  `work/rels/elf/d_basesNP.elf` with a raw-word dumper
  (`0x807CFA50`=1.0, `0x807CFA5C`=5.0, `0x807CFA68`=0.5, `0x807CFA74`=150.0,
  `0x807CCDFC`=16.0, `0x807CCE00`=0.0, `0x807CCE20`=5500.0,
  `0x808083D4`=6.0, `0x808083EC`=-6.0, `0x808083F0`=-4.0, `0x80808408`=45,
  `0x8080840C`=16, `0x8080842C`=96.0, `0x8080843C`=90; the bytes at
  `0x808083C0` are `"Wm_ro_pipeappear"`).
- `0x8082FF14`, `0x8082FFA8` and `0x8031D5B6` are outside the REL image (they
  map past the last section), i.e. `.bss`/engine data; they are read at run
  time, not from the file, and are documented as such.
- Only helpers that already exist in `src/game/bases.rs` are used. Typed:
  `fn_80789b80`, `fn_80789a60`, `vmethod0`, `vmethod5`, `call0_f32` (for
  `0x800B30E0`), `call0_ret` (`0x80101090`, `0x801010B0`, `0x8079B260`),
  `call_u32x3_f32` (`0x806601B0`), `call_f32_u32x2` (`0x80160D10`),
  `call_f32x3_u32x4` (`0x80788B50`). Raw: `abi_call1!` (`0x800955F0`,
  `0x800D8A80`, `0x800B0DB0`, `0x800B0A20`, `0x80785600`, `0x80788CB0`,
  `0x8079EF00`, `0x8079EF10`), `abi_call2!` (`0x8006CED0`, `0x80789E60`),
  `abi_call3!` (`0x80100200`, `0x80100DA0`), `abi_call4!` (`0x80789990`),
  `platform::call1` (`0x800F6AB0`).
- No `psq_l`/`psq_st`, GQR or `_savegpr`/`_restgpr` artifact is emitted.
- `src/game/bases.rs` was not modified by this lane.
