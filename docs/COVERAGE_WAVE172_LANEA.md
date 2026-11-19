# Coverage - Wave 172 lane A (`game::bases`, 332-336 byte band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (pending merge of `work/wave172/laneA_block.rs`) | `game::bases` | 5 | 1,668 | 5 | 0 |

All five bodies are in the 332-336 byte sub-band of lane A. Nothing was
deferred.

## What this pass covers

- **Land-smoke / floor probe (1 body).** `0x8068CB40` steps the two `+0x100`
  angles, then branches on `10.0 + this[+0x4A8] < this[+0xB0]`: the probe side
  calls `0x80075270` and may fire the `+0x394` slot `+0x18` with `0x8082ACF0`;
  the other side emits `Wm_en_landsmoke_s`, queues sound `0x2C8`, latches
  `+0xEC = 1.75`/`+0xB0` and rewrites `+0x4A8` through `0x8068C5A0`.
- **Sub-object re-seat (1 body).** `0x806DD1A0` dispatches the `+0x394` slot
  `+0x10`, the three `+0x5BC`/`+0x61C`/`+0x67C` slot `+0xC` movers, re-runs
  `0x806DDD40`/`0x806DD510`/`0x806DD550`, seeds the `+0x590` triple and
  `+0x598` scale and places the seat vector at `+0x474`.
- **Child node rebuild (1 body).** `0x806F42F0` walks `+0xBEBC` slots at a
  `4`-byte stride, tests the `+0xBC34`/`+0xBCD4`/`+0xBD74`/`+0xBE14` quad and
  rebuilds the `+0x7C` transform plus each `+0x1F4 + i*0x40` node.
- **Seven-state timer (1 body).** `0x8075C070` drives `+0x1A4`/`+0x1AC`,
  plays the `3064`-strided world record through `0x800EF700` and reports `1`
  in state `7`.
- **Seat twin (1 body).** `0x8069C620` mirrors `fn_8069c170` and adds the
  trailing `+0x500 = 100` halfword store.

## Verification notes

- `0x8068CB40`: the Ghidra `if (fVar1 < b0)` was re-checked against the raw
  words - `fcmpo cr0,f2,f0`, `cror cr0.EQ,LT,EQ`, `bne 0x8068CC3C` - so the
  probe runs when `b0 > 10.0 + h` and the smoke branch runs otherwise. This
  matches the already-translated sibling `fn_8068cd70`, which tests
  `b0 <= top` for the same burst.
- `0x806DD1A0`: the `fmuls` at `0x806DD278` uses `FRC` (not `FRB`), i.e.
  `+0x710 * 0.0625` from `0x807CB9CC`, confirming the Ghidra constant; the
  `0.5` from `0x807CB9D8` scales the `32.0 + radius` lift in the seat x.
- `0x806F42F0`: the `0x802DCD70`/`0x802DCDBC` register helpers are omitted;
  the live receiver is the incoming `r3` and is modelled as `this`. The body
  returns `1` from the `li r3,1` before the restore.
- `0x8069C620`: the child mode byte is sign-extended (`extsb`) before
  `0x8069ACD0`, unlike the earlier twin body which passed it raw.
- Callees without Rust bodies are reached through `abi_callN!`
  (`0x806DDD40`, `0x806DD550`, `0x8075BE90`, `0x800EF700`, `0x800DF890`,
  `0x800C0750`); `fn_806dd510` and `fn_8069acd0` are called directly.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8068CB40` | `fn_8068cb40` | 332 | high |
| `0x806DD1A0` | `fn_806dd1a0` | 332 | high |
| `0x806F42F0` | `fn_806f42f0` | 332 | high |
| `0x8075C070` | `fn_8075c070` | 336 | high |
| `0x8069C620` | `fn_8069c620` | 336 | high |

## Deferred

None.
