# Wave 183 lane C coverage (`game::bases`)

Ten small bodies in REL `d_basesNP`, all in the `0x806A8..0x806AF` range. Nine
are scalar-deleting state-object destructors of the form

```text
if (this != 0) { 0x8015F800(this, 0); if (free > 0) 0x802B9280(this); }
return this;
```

and one (`fn_806abbc0`) is a two-virtual-call reset helper. All ten are
translated; nothing deferred.

## Bodies

| Address | Size | Body | Evidence |
| --- | --- | --- | --- |
| `0x806A81B0` | 88 | deletes a `daJrFire_c` state object (vtable `0x807F31E8`) | `806a81b0_FUN_806a81b0.c` |
| `0x806A8210` | 92 | deletes a `daJrFire_c` state object (vtable `0x807F31B4`) | `806a8210_FUN_806a8210.c` |
| `0x806A93D0` | 88 | deletes a `daJrFloorFire_c` state object (vtable `0x807F3858`) | `806a93d0_FUN_806a93d0.c` |
| `0x806A9430` | 88 | deletes a `daJrFloorFire_c` state object (vtable `0x807F388C`) | `806a9430_FUN_806a9430.c` |
| `0x806AB340` | 92 | deletes a `daKameckForAirshipDemo_c` state object (vtable `0x807F3ED8`) | `806ab340_FUN_806ab340.c` |
| `0x806AB8E0` | 88 | deletes an object whose teardown is `0x8027A200` (twin of `fn_807B4EE0`) | `806ab8e0_FUN_806ab8e0.c` |
| `0x806ABBC0` | 68 | resets the object returned by interface slot `+0x28` via its slot `+0x14` with descriptor `0x8082BD04` | `806abbc0_FUN_806abbc0.c` |
| `0x806AEDF0` | 88 | deletes a `daKameckMagic_c` state object (vtable `0x807F4894`) | `806aedf0_FUN_806aedf0.c` |
| `0x806AEE50` | 92 | deletes a `daKameckMagic_c` state object (vtable `0x807F4860`) | `806aee50_FUN_806aee50.c` |
| `0x806AF740` | 88 | deletes the `daKanbanArrow_c::StateID_Wait` state object (vtable `0x807F4A44`) | `806af740_FUN_806af740.c` |

## Notes

- **92-byte variants** (`0x806A8210`, `0x806AB340`, `0x806AEE50`) and
  `0x806AB8E0` carry the compiler's duplicated `cmpi cr0,3,r3,0` + `bc 12,2`
  null re-test (`0x806A8234`, `0x806AB364`, `0x806AEE74`, `0x806AB904`).
  Both branches test the same `cr0.eq`, and the path is only reachable when
  `r3 != 0`, so the inner branch is dead and the teardown call is
  unconditional inside the outer guard. Emitted that way.
- **Class attribution** comes from the `..._c::StateID_*` name strings emitted
  immediately before each state vtable cluster (e.g. `0x807F3170`/`0x807F3190`
  for `daJrFire_c`, `0x807F3810`/`0x807F3830` for `daJrFloorFire_c`,
  `0x807F3EA0` for `daKameckForAirshipDemo_c`, `0x807F4818`/`0x807F4838` for
  `daKameckMagic_c`, `0x807F4A1C` for `daKanbanArrow_c`). Where a class has two
  adjacent state vtables the name-to-vtable pairing is *not* proven, so no
  specific state is claimed for those rows.
- **`fn_806ab8e0`** is byte-for-byte the body of the already-translated
  `fn_807b4ee0` (`0x806AB8FC..0x806AB91C`), which is why it reuses the same
  `call0_ret(0x8027_A200)` form; no literal vtable word in this REL points at
  it, so its class is left unnamed.
- **`fn_806abbc0`** reproduces the established
  interface-slot pattern (`fn_806751f0`): `lwzu r12,0x394(r3)` both loads the
  interface vtable and advances the `this` argument, so slot `+0x28` is called
  with `this + 0x394`. The second call goes through the *returned* object's
  vtable at offset 0, so it is emitted with `vmethod1`. `0x8082BD04` lies
  outside this REL (`file end ≈ 0x8081_33xx`); it is passed as an opaque
  pointer literal, as the neighbouring `0x806ABCD0` does for slot `+0xD4`.
- No new helpers, field offsets or constants were introduced; the only
  addresses used are `0x8015_F800`, `0x802B_9280`, `0x8027_A200`,
  `0x8082_BD04`, `0x14`, `0x28` and `0x394`, all read from the raw words.

## Deferred

None.

## Files

- `work/wave183/laneC_block.rs`
- `ledger/agents/wave183-lanec.csv`
- `docs/COVERAGE_WAVE183_LANEC.md`
