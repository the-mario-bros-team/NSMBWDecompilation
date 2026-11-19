# Wave 196 lane A coverage (`game::bases`)

Slice: final 256-300 byte band (`0x80768470` .. `0x8079E550`). Ten functions,
ten translated, none deferred.

| address | size | body |
| --- | --- | --- |
| `0x8076_8470` | 276 | `fn_80768470` - scans `0x275` actors, keeps the nearest id in `+0x204` |
| `0x8076_EE60` | 288 | `fn_8076ee60` - rebuilds the rider matrix from the `+0x4B4` handle |
| `0x8077_4BD0` | 296 | `fn_80774bd0` - route key scan, defers to `fn_80774df0` |
| `0x8078_2C40` | 284 | `fn_80782c40` - route node destructor |
| `0x8078_6180` | 264 | `fn_80786180` - three-state rail hand-over machine |
| `0x8079_0280` | 268 | `fn_80790280` - collision clip refresh (`+0x260` ids) |
| `0x8079_7190` | 264 | `fn_80797190` - `W1X0` node re-initialisation |
| `0x8079_9B00` | 288 | `fn_80799b00` - route-ring neighbour id resolution (`this, node, kind -> i32`) |
| `0x8079_E1E0` | 296 | `fn_8079e1e0` - rider gate test and node state driver |
| `0x8079_E550` | 292 | `fn_8079e550` - ride-state entry and effect placement |

## Signature recoveries

Three of the exports lose arguments because Ghidra modelled the `_savegpr_N`
helpers as value-returning calls:

* `0x80774BD0` - `bl 0x802DCD70` is `_savegpr_26`; `r3`/`r4` survive unchanged,
  so the real signature is `fn_80774bd0(this: usize, needle: u32) -> u32`.
  Ghidra printed a 64-bit return and a `void` function with no arguments.
* `0x80799B00` - same helper: `fn_80799b00(this: usize, node: u32, kind: u32)
  -> i32`. The existing call sites already use
  `abi_call3!(0x8079_9B00, this, node, kind) as i32`, which now has a matching
  Rust body.
* `0x80790280` - `bl 0x802DCD74` is `_savegpr_27`; `r3` is the only surviving
  argument, so `fn_80790280(this: usize)`. The leading `psq_st f31` block is
  GQR0 store noise and is not emitted.

`0x80797190` is now `fn_80797190(this: usize)`, matching the existing
`abi_call1!(0x8079_7190, this)` sites (two of them).

## Deliberate call-by-address sites

`0x80772300` and `0x80798CC0` have translated bodies that drop their second
argument (and, for `0x80772300`, the returned record), so those calls go
through `abi_call3!` / `abi_call2!` to preserve `r4`/`r5` and the result.
`0x80772280`, `0x80772370`, `0x80775150` and `0x8079A4B0` are called by name;
their existing bodies already ignore the dead `r4` that Ghidra dropped.

## Deferred

None.

## Files

* `work/wave196/laneA_block.rs`
* `ledger/agents/wave196-lanea.csv`
* `docs/COVERAGE_WAVE196_LANEA.md`
