# Wave 199 lane A coverage (`game::bases`)

Slice: the two bodies that close out the `0x80782`/`0x8079C` bucket -
`0x807826B0` (516 bytes) and `0x8079C330` (4232 bytes, the largest body in
`d_basesNP`). Two translated, none deferred.

| address | size | body |
| --- | --- | --- |
| `0x8078_26B0` | 516 | `fn_807826b0` - builds the `cobHatenaSwitch` accessor, model adapter (`+0x1A4`), animation record (`+0x1E8`, `0x224` bytes) and frame object (`+0x1B0`) |
| `0x8079_C330` | 4232 | `fn_8079c330` - `switch (state - 6)` dispatcher for states `6..=0xA4` of the hatena-switch actor (jump table `0x8081B884`, 0x9F entries) |

Verified with `cargo check` against a scratch copy of `src/game/bases.rs` with
the block appended (the repository file was not modified).

## Signature recoveries

* `0x8079C330` - Ghidra declares `void FUN_8079c330(void)` and switches on
  `(int)uVar14`, where `uVar14 = func_0x802dcd6c()`. `0x802DCD6C` is the
  `_savegpr_*` thunk: it preserves `r3`/`r4`, and Ghidra models the pair as a
  64-bit return. On big-endian PowerPC the MSW is `r3`, so the real signature is
  `(this: usize, state: i32)`: `mr r28,r3` is `this`, `addi r0,r4,-6` +
  `cmpli 0x9E` dispatch on `state - 6`, and `lwz r7,0xFFFFA1AC(r27)` hoists the
  player manager whose `+0x1AC`/`+0x1B0` fields are the `r6`/`r4` snapshots the
  arms use (`iVar12`/`uVar11` in the Ghidra output). The `r28` base is what the
  arms address as `iVar5`.
* `0x807826B0` - no helper ambiguity; the only reading is `this` at `r3`, kept
  in `r28` for the whole body.
* Virtual calls. `0x8079C330` state `0x70` and `0x97` dispatch through vtable
  pointers that are **not** at object offset 0 (`object + 0`, slot `0x5C`, and
  `handle + 4`, slot `0x38`), so those two sites load the slot address with
  `ptr_at` and call it through `abi_callN!`/`call_f32x3_u32x2`. Everything else
  is a plain `vmethod0`/`vmethod1`/`vmethod2` on `this` or on
  `*(this + 0x1D0)`. In `0x807826B0` the `+0x1A4` member is vcalled with
  `r3 = this + 0x1A4` (not the loaded pointer), which is exactly
  `vmethod1(this + 0x1A4, 0x18, ...)`.

## Decode notes

* Ghidra's FPR naming is unreliable in this export: `lfs rD,disp(rA)` prints the
  float register as `rD`, so every load/store was re-read from the raw words.
  This changes several constant roles, e.g. state `0x25` really computes
  `(candidate.y - 0x8042D0BC) - this.y`, and state `0x1A`/`0x23` differ only in
  the subtracted constant (`0x8042D0C0` vs `0x8042D0BC`).
* `lis` immediates: the jump table base is `0x80820000 - 0x477C =
  0x8081B884`, and the `0x807826B0` string block is `0x80820000 - 0x7A28 =
  0x808185D8`, i.e. `+0x68` = `0x80818638` (pointer slot to
  `cobHatenaSwitch`), `+0x68`'s pointee = `0x80818628` (`cobHatenaSwitch`),
  `+0x78` = `0x80818650` (`cobHatenaSwitch`), `+0x88` = `0x80818640`
  (`g3d/model.brres`).
* REL-resident constants read out of the ELF: `0x807C_F81C = 0.0`,
  `0x807D_0080 = {1,2,3,4,5,6,7,0}`, `0x807D_0160 = 0.0`,
  `0x807D_00C4 = 1.0`, `0x807D_018C = 50.0`, `0x807D_01A4 = 60.0`,
  `0x807D_01F8 = 1500.0`, `0x807D_01FC = 20.0`, `0x807D_0200 = 800.0`,
  `0x807D_0204 = -1.0`, `0x807D_0208 = 5.0`, `0x8081_B874 = "W1X0"`.
  The `0x8042D0B0`..`0x8042D0C0`, `0x8031D39C`-relative and `0x8042A1AC`/
  `0x8042A18C`/`0x8042A27C`/`0x80429F7C`/`0x8042A284` slots live in main.dol,
  so they stay runtime reads (`platform::read_f32`/`read_u16`).
* State `0x5B`/`0x64`/`0x91` load their timer with `lha` from
  `0x8031D5B2`/`0x8031D5B0`/`0x8031D452`, so they are sign-extended
  (`read_u16 as i16 as i32`); the `0x800F69F0` descriptors
  (`0x8031D470`, `0x8031D4FC`, `0x8031D5DC`, `0x8031D5F0`, `0x8031D5F8`,
  `0x8031D5E4`) are likewise signed halfwords.
* State `0xA4` is `neg`/`or`/`rlwinm ..,31,31`, i.e. `(carrier < 0) as u32`,
  passed as the third argument of `0x80C56270`.
* Three sites call `0x8079F500` (states `0x4B`, `0x4D`, `0x57`) and branch on
  its `r3`, but the existing Rust body `fn_8079f500` returns `()` because the
  value only appears through the `_restgpr` epilogue at `0x802DCDC0`. Those
  three sites therefore use `abi_call1!(0x8079_F500, this)` and keep the
  machine-level result; every other callee with a Rust body is called through
  its `fn_*` wrapper.
* State `0x4C` builds a five-byte character name (`"Fk0?"` / `"Ka0?"`) whose
  buffer is left untouched when both `0x800FC810` and `0x800FC830` return zero.
  Rust cannot express an undefined buffer, so it is materialised as an empty
  string; this is the only place where the translation had to choose a value the
  original leaves indeterminate.

## Deferred

None. There are no unproven address ranges: the dispatch bounds and all 0x9F
jump-table entries, every arm's branch targets, constants and argument
registers were read from the raw words. `0x8079C330` is rated `medium` rather
than `high` only because the roles of the engine callees it drives
(`0x800F9BE0` spawner, `0x800F69F0` particle spawner, `0x80C56B90`/
`0x80C565A0`/`0x80C561E0`/`0x80C56270` render-object calls, `0x80103410`
message submit) are inferred from call shape, and because of the two notes
above (raw `0x8079F500` call sites, undefined name buffer).
