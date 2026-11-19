# Coverage - Wave 183 lane E (`game::bases`, `0x806BF4B0`..`0x806CB210`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane E block) | `game::bases` | 10 | 932 | 10 | 0 |

Ten bodies from the REL `d_basesNP`. Source of the bodies:
`work/wave183/laneE_block.rs`; per-function ledger:
`ledger/agents/wave183-lanee.csv`. Nothing was deferred.

## What this pass covers

- **Seven deleting-destructor wrappers (`0x806BF4B0`, `0x806C6E40`,
  `0x806C6F00`, `0x806C8200`, `0x806C8F00`, `0x806CA820`, `0x806CAA70`).** All
  seven run a teardown call when `this != 0` and then release the storage with
  `0x802B9280(this)` when `free > 0`, returning `this`. Six of them are
  byte-identical (`0x8015F800(this, 0)` as the teardown); `0x806CAA70` is the
  outlier that calls the NW4R element destructor `0x80169D20(this, -1)`.
  `0x806C6F00` is four bytes longer only because the compiler emitted the
  `this != 0` test twice.
- **A `Position_*` bone copier (`0x806BFE60`).** The Junior-boss sibling of the
  already-translated `0x806BFDF0`: refresh the model matrix with `0x806BFD50`,
  resolve `"Position_Jr"` at `0x807F749C` through `0x80168400`/`0x8023B200`,
  then copy the bone transform into `+0x910` with `0x80168180`.
- **An interface state predicate (`0x806C1B00`).** Gate on `0x80064540`,
  dispatch `+0x394` virtual `+0x28`, ask the returned view virtual `+0x14` for
  descriptor `0x8082CB04`, and fold the answer to `(answer == 0) as u32`.
- **A handle-follow updater (`0x806CB210`).** Resolve `+0x40C` through the
  actor-handle lookup `0x80162D00` and, when it is live, call `0x806CB140` to
  copy that actor's position/angle into `this`; the resolved pointer is the
  return value.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806BF4B0` | `fn_806bf4b0` | 88 | high |
| `0x806BFE60` | `fn_806bfe60` | 104 | high |
| `0x806C1B00` | `fn_806c1b00` | 124 | high |
| `0x806C6E40` | `fn_806c6e40` | 88 | high |
| `0x806C6F00` | `fn_806c6f00` | 92 | high |
| `0x806C8200` | `fn_806c8200` | 88 | high |
| `0x806C8F00` | `fn_806c8f00` | 88 | high |
| `0x806CA820` | `fn_806ca820` | 88 | high |
| `0x806CAA70` | `fn_806caa70` | 88 | high |
| `0x806CB210` | `fn_806cb210` | 84 | high |

## Deferred

None.

## Corrections and notes

- **`0x80064540` loses its argument in the export.** `0x806C1B00` prints
  `func_0x80064540()` with no argument, but `mr r31,r3` at `0x806C1B10` leaves
  the receiver in `r3` at the branch (`0x806C1B14`), and the main.dol body
  `FUN_80064540(int param_1)` reads the actor fields `+0x38C`/`+0x36C`/`+0x120`.
  The translation forwards `this`; this is the same class of recovery as
  `0x806BFD50` in `0x806BFDF0`.
- **The `cntlzw` fold in `0x806C1B00` is `(value == 0)`, not `value != 0`.**
  The raw tail is `7C0300D0 neg r0,r3` / `7C001B78 or r0,r0,r3` /
  `54000FFE rlwinm r0,r0,1,31,31` (= `srwi r0,r0,31`) /
  `7C000034 cntlzw r0,r0` / `5403D97E rlwinm r3,r0,27,5,31` (= `srwi r3,r0,5`).
  `cntlzw` of the 0/1 sign bit is `32` for zero and `31` otherwise, so `>> 5`
  yields `1` exactly when the query returned `0` - the same idiom already used
  in `fn_80781c30` (`wrapping_sub(..).leading_zeros() >> 5`).
- **`0x806C6F00` has a duplicated null guard.** `0x806C6F20` and `0x806C6F24`
  are both `bc 12,2` on the same `cr0` from `cmpi cr0,r3,0` at `0x806C6F08`;
  the inner branch only skips the teardown call. It is a compiler artifact and
  the translation keeps a single `if`.
- **The free test is `free > 0`, not `free != 0`.** In the wrappers,
  `cmpi cr0,r31,0` (`0x806BF4DC` in the first) is followed by `bc 4,1`
  (`4081000C`), i.e. "branch if `cr0[GT]` is clear", which skips the release
  exactly when `free <= 0`. This matches Ghidra's `0 < param_2` and the
  existing `fn_806511d0`/`fn_8067c340` convention.
- **`0x806CAA70` targets `0x80169D20`, not `0x8015F800`.** The raw
  `3880FFFF li r4,-1` at `0x806CAA94` and `4BA9F289 bl` resolving to
  `0x80169D20` (`0x806CAA98 - 0x560D78`) are both explicit; `0x80169D20` is the
  NW4R element destructor already used elsewhere in this file.
- **`"Position_Jr"`, not `"Position_Peach"`.** The literal at `0x807F749C` was
  re-read from the ELF: `Position_Jr\0`. The sibling at `0x807F748C` is
  `Position_Peach` and belongs to `0x806BFDF0`.
- **`0x806CB140` has no Rust body yet.** It is the position-follow copier of
  this window (208 bytes, `0x806CB140`) and is therefore reached through
  `abi_call2!`; the doc only claims what the export shows (`+0x104` angle
  plus `+0x4000`, `+0x53C`/`+0x540` offsets applied to `+0xAC`/`+0xB0`).

## Verification

- The lane-E block type-checks standalone (`rustc --edition 2021
  --emit=metadata --crate-type lib`) against stubbed copies of
  `platform::read_u32/call1/call2/call3`, `vmethod0`/`vmethod1` and a
  `fn_806bfd50` stub; the harness lives outside the repo in the session temp
  directory (`%TEMP%/opencode/w183e`).
- Every export in `work/ghidra/rel_d_basesNP/export/` was read and each byte
  size matches `functions.tsv` (88/104/124/88/92/88/88/88/88/84).
- Each body was re-checked against the raw words with
  `node %TEMP%/opencode/ppc159.js work/rels/elf/d_basesNP.elf <addr>`,
  including the two branch encodings (`bc 12,2` = `beq`, `bc 4,1` = `ble`).
- Only helpers that already exist in `src/game/bases.rs` are used:
  `abi_call1!`/`abi_call2!`/`abi_call3!`, `platform::read_u32`, `vmethod0`,
  `vmethod1`, and `fn_806bfd50`.
- No `_savegpr`/`_restgpr` helper, GQR or `psq` artifact is emitted.
- `src/game/bases.rs` was not modified by this lane.
