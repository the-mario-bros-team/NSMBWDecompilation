# Coverage - Wave 176 lane E (`game::bases`, 428/432-byte classes)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane E block) | `game::bases` | 4 | 1,720 | 3 | 1 |

Four bodies: one 428-byte ctor/aim pair and one 432-byte player-reset/contact
pair. Source of the bodies: `work/wave176/laneE_block.rs`; per-function ledger:
`ledger/agents/wave176-lanee.csv`.

## What this pass covers

- **Sand-pillar contact registration (1 body).** `0x80694F30` is the
  registration half of the translated sand-pillar cluster
  (`0x806950F0`/`0x80695170`/`0x80695260`/`0x806955D0`/`0x80695580`). For an
  incoming record whose other actor is in state `1`/`2` it runs the
  `0x8008C550`/`0x8008C530` window pair, notifies with `0x8004A210` and appends
  to the ten-entry ring. State `3` additionally gates on
  `actor_method0(other, 0x244) == 0`, spawns `Wm_en_sphitsmoke` for hit kind `9`
  and then runs `actor_method0(other, 0x23C)` before appending.
- **Player constructor (1 body).** `0x807BE4C0` installs vtable `0x808218C8`,
  the three `0x80350078`-tagged ten-word records at `+0x184`, `+0x294`,
  `+0x404`, the 12x4 array at `+0x6C8` (`0x802DC9A0` with the
  `0x80601E60`/`0x80601CD0` callbacks), the `+0x4F8` collision table
  `0x80821938`, and finally writes itself to `0x8042A284`, the global player
  pointer the already-translated `0x80768680`/`0x80768590` read as
  `0x8042A284 + 0x4F8`.
- **Player placement + course reset (1 body).** `0x8079E030` re-places every
  node of the `+0x184` chain through the translated [`fn_8077d230`], copies the
  position triple to `+0xAC`/`+0x264` and the listener triple to `+0x1FC`,
  runs `0x80799960(this, 4, -1)` plus the `0x80104FD0` interface publish, then
  splits on the mode word `+0x27C`: mode `6` spawns effect `60` at `10.0`/
  `0.38`/`1.9`, mode `7` may demote to `0x80799960(this, 2, -1)` and spawns
  effect `190` at `5.0`/`1.9`/`1.9` with a `+/-1.0`/`0.0` offset table chosen by
  `0x800FC040`.
- **Scene-node aim refresh (1 body).** `0x807A02C0` reads the scene-node world
  position with `0x800F8050`, spawns effect `60` at scale `1.5`, then aims at
  either the node or the cached point `+0x784` depending on whether the node
  moved, writes the direction through `0x800F6410`, re-derives the facing
  halfword `+0x6B2` with `0x801611C0(dx, dz)`, clears `+0x77C` and copies
  `+0x100` into `+0x6B0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80694F30` | `fn_80694f30` | 432 | high |
| `0x807BE4C0` | `fn_807be4c0` | 428 | high |
| `0x8079E030` | `fn_8079e030` | 432 | high |
| `0x807A02C0` | `fn_807a02c0` | 428 | medium |

## Deferred

None.

## Corrections and notes

- **`0x80694F30`'s window predicates are not symmetric.** The two `fcmpo`+
  `cror` folds differ: `0x80694F88` uses `cror 2,1,2` (`GT|EQ`, so the body
  needs `first <= second`) while `0x80694FAC`/`0x8069501C` use `cror 2,0,2`
  (`LT|EQ`, so the body needs `second <= first`). The Ghidra export prints
  `dVar4 <= dVar5` in one arm and `dVar5 <= dVar4` in the other; both readings
  are correct as printed.
- **The `0x8008C550`/`0x8008C530` probes receive the collision records.** At
  `0x80694F78`/`0x80694F94` `r3` still holds the incoming `param_1`/`param_2`
  record pointers; the actor pointers are only recovered afterwards through
  `+0x4`. The export's missing first argument for the first call of each pair
  is that `r3`, not a lost `this`.
- **`0x8079E030` is not a two-return-value call.** The export's
  `uVar7 = func_0x802dcd74()` split into a `puVar5`/`iVar4` pair is
  `_savegpr_26`, which returns `r3` unchanged; `iVar4` is simply `r4`
  (`param_2`), i.e. the object the body writes. `0x802dcdc0` is
  `_restgpr_26`. Neither is emitted.
- **`0x8079E030`'s table is `0x8031D39C`, not `0x8032D39C`.** `lis r8,0x8032`
  followed by `addi r8,r8,-11364` (`-0x2C64`) lands on `0x8031D39C`, the same
  table the existing bodies build (`0x8032D39C` was the `ori`-style misreading
  of that immediate; wave 236 corrected every body that used it). The
  arm-6 constants are `+0x18 = 1.9`, `+0x170 = 10.0`, `+0x174 = 60` (s16),
  `+0x178 = 0.2`, `+0x17C = 1.0`; the arm-7 constants are `+0x184 = 5.0`,
  `+0x188 = 190` (s16), `+0x18C = 1.0`, `+0x190 = 1.0`. All read from
  `work/dol/main.elf`.
- **`0x807BE4C0`'s `+0x4C8..+0x4D0` triple is this REL's own bss.** The sources
  `0x80834440`/`0x80834444`/`0x80834448` fall inside the `0x80822900`+
  `.bss` segment (`memsz 0x12484`), so they are copied, not invented; no other
  export in the REL references them.
- **`0x807A02C0` drops one dead store.** The C builds the same `(dx, dy, dz)`
  triple twice (`r1+0x2C` and `r1+0x20`); only the first is passed to
  `0x800F6410`, and `0x801611C0` consumes only `f1`/`f2` (`dx`, `dz`). The
  second copy is emitted as the same live operands, the same treatment the
  existing `fn_80762ce0` gives the identical pattern.
- **`0x800F69F0` keeps the 8-argument form.** At both `0x80377DE0`-terminated
  call sites of this lane (`0x80694F30` is a different callee) `r8` still holds
  the effect table (`0x8031D39C`, `0x8081BE40`). Every existing body of this
  callee in `src/game/bases.rs` (`fn_80769f00`, `fn_80784650`,
  `fn_807337d0`) passes 5 trailing `u32`s and drops that register, and
  `0x80784650` never writes `r8` at all, so `r8` is table-base noise for this
  ABI rather than a ninth argument. The lane follows the corpus; the
  buffer+table form (9 arguments) is used only where the buffer is a real
  argument, as in `0x8079E030`'s mode `7`.
- No `psq_l`/`psq_st`, GQR or `_savegpr`/`_restgpr` artifact is emitted; the
  exports' `unaff_GQR0`/`ldexpf` blocks in `0x80694F30` are compiler noise.

## Verification

- The lane-E block type-checks standalone against stubbed helper signatures
  (temporary `rustc --emit=metadata` harness with the real helper ABIs and the
  real `abi_callN!` macro bodies); the harness lives outside the repo.
- All four evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset
  was re-read from `work/rels/elf/d_basesNP.elf` (or `work/dol/main.elf` for the
  `0x8031D39C` table) with a raw-word dumper.
- Only helpers that already exist in `src/game/bases.rs` are called
  ([`fn_80695170`], [`fn_80695260`], [`fn_8077d230`], `actor_method0`,
  `call0_ret`, `call_f32_1`, `call_f32x2`, `call_f32x3_u32x5`,
  `call_f32x3_u32x6`); `0x8004A210`, `0x8008C550`, `0x8008C530`, `0x800F6410`,
  `0x800F8050`, `0x800F69F0`, `0x800F5FD0`, `0x800FC040`, `0x80104FD0`,
  `0x801611C0`, `0x8016C890`, `0x802BFF30`, `0x802BF3D0`, `0x802D0CC0`,
  `0x802DC9A0` and `0x80799960` have no Rust body and go through the
  `abi_callN!` macros or the typed `call_*` helpers.
- `src/game/bases.rs` was not modified by this lane.
