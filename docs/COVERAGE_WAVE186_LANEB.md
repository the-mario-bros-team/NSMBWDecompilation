# Coverage - Wave 186 lane B (`game::bases`, sub-300 sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 1,020 | 9 | 1 |

Lane B covers ten `d_basesNP` bodies in the `0x8078D290..0x8079F4B0` band:
constructors, one destructor helper, three state predicates, two table-driven
preset selectors and one recovered tail call. Nothing is deferred.

## Bodies

| Address | Size | Body |
|---|---:|---|
| `0x8078D290` | 124 | ctor; `0x8081A418` vtable, model/motion/animation blocks at `+0x18C/+0x1AC/+0x1B8` |
| `0x8078D7C0` | 76 | release helper; `0x80102DB0(node, f1=+0x1E0)` + `0x801680E0(+0x1B8)` when `node+0x0C` |
| `0x8078D8F0` | 76 | ctor; `0x8081A838` vtable, model at `+0x188`, clears `+0x1D0` |
| `0x8078F060` | 124 | one-shot latch from `0x8042A258`, sets timer `0x3C`, clears the global |
| `0x8078F2D0` | 116 | timer `0x1E` or class slot `+0x68` |
| `0x807970A0` | 128 | snap rotation triple to target, arm `+0x18C`/`+0x228`, zero `+0x24C` |
| `0x80799960` | 108 | `0x8031D39C` preset selector + model slot `+0x5C` dispatch |
| `0x8079B1D0` | 112 | goal-check gate (`+0x27C == 5`, save `+0x370 == 1`, `0x800FB2D0(0)`) |
| `0x8079BE80` | 120 | clears latches, clears sound flags `4`/`8`, calls `0x8079BF00` |
| `0x8079F4B0` | 68 | save mode `3` + `0x80102690(world,0,0)` tail call |

## Corrections and cross-checks

- `0x8079F4B0`: Ghidra emits `halt_baddata`; the raw words end in
  `b 0x80102690` at `0x8079F4F0` (a frameless tail call). The `rlwinm
  r0,r0,2,22,29` pair is `(x & 0xFF) * 4` over the byte table `0x807D0240`
  (`4,4,5,5,7,7,10,10,10,10`). The export declares `void`, so the tail result is
  discarded.
- `0x807970A0`: the four `divw rX,rX,r9` sites divide by `r9`, which the
  prologue sets to `li r9,0` (and stores to `+0x24C`). PowerPC `divw` by zero
  yields zero, so the three steps are zero; `checked_div(...).unwrap_or(0)`
  reproduces that. The four `sth` stores into the local frame are dead.
- `0x80799960`: the `Ghidra` jumptable warning is a plain `bcctr` through the
  model vtable slot `+0x5C`. Record 0 of the table resolves to the same
  `0x8031D634` selector / `0x8031D638` / `0x8031D63C` floats already used by
  `fn_807999d0`; `forced == 2` overrides the floats with
  `0x8031D650`/`0x8031D654`.
- `0x8078D7C0` reuses `fn_807a10b0`'s `call_f64_u32(0x80102DB0, f32, node)`
  shape; the C `(double)` first argument is an `lfs` value.
- `0x8079BE80` calls [`fn_8079bf00`] directly; the binary's `bl 0x8079BF00`
  targets the body already translated in this module.
- `0x8078D290`/`0x8078D8F0` follow `fn_8075a6c0`: the `0x800F5FD0` base
  constructor loses its `this`, so the actor is forwarded explicitly.

## Deferred

None.
