# Audit - Wave 287 (six tables closed, savegpr under 100)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80323898` | **54/54** | `daEnemyIce_c` |
| `0x80327250` | **19** (18 slots) | game-scene controller |
| `0x8030F0F8` / `0x8030F060` | 10 each | audio small-record A base/final |
| `0x8030F190` / `0x8030F090` | 10 each | audio small-record B base/final |

**Eighty-one vtables complete.**

## 2. Boundary and identity corrections

- `0x80327250`'s base is `0x80327248`; the 19-word run is **18 methods + the
  next class's creator word** (`0x80327298 = 0x80156EA0`, magic `0x02C202C2`).
  Identity is the **game-scene controller** (`gameScene/gameScene.arc`,
  `gameScene_37.brlyt`); the `fukidashiInfo` label belongs to the next table at
  `0x803272D0`. The installer is an untabled gap body (`0x801546C0`, 128 B,
  allocating 0x90BE4 bytes), now translated.
- `daEnemyIce_c` is fixed by its `StateID_Freeze/Revival/Melt/Break` literals and
  a 12-member teardown matching the 0x6B8-byte constructor.
- The audio quartet is two base/final pairs (`0x74`-byte A, `0x70`-byte B); all
  four share slots 1-5, and the finals add the `fn_80199e60` + `0x8019dc90` gate.
  Twenty uncovered words became 10 implementations + 10 forwards.

## 3. `0x80352670` ownership settled

The block is **`daBossDemo_c`**, not `daBullet_c`. `fn_8001D2C0` passes
`daBossDemo_c::StateID_{Ready,BattleStDemo,BattleIn,BattleEdDemo}`
(`0x803016D0/16EC/1710/1730`) to the record constructor at `0x8015F7C0`, giving
records `0x80352670/6B0/6F0/730` with vtable `0x80301754`. `daBullet_c`'s strings
are a different set at `0x80301A68..0x80301B57`. Two docs corrected.

## 4. `_savegpr`/`_restgpr` sweep, part 12

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 145 | **94** |
| Bodies containing them | 74 | **49** |
| Bodies changed | - | 25 |
| Calls removed | - | **51** |
| Genuine calls found | - | **0** |

Most consequential correction: `fn_80ae0880` had the incoming `r3:r4` pair
**backwards** - it used the high word (the player) as the actor. Now
`(actor, player, slot)` with the zero-test on `player`. Four bodies gained correct
`-> u32`/`-> usize` returns; 15 gained real first parameters, 9 call sites
re-checked.

## 5. Survey

90 fully covered runs, 588 distinct prologue targets, max 5 per run. This wave
closed the last of the ten 5-prologue runs; the next ranking will come from wave
288's refresh.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `daEnemyIce_c` close | 6 | 520 |
| B | game-scene controller close | 6 | 6,976 |
| C | audio quartet close | 10 (+10 forwards) | 5,072 |
| D | savegpr sweep part 12 | 25 fixed | - |
| E | gap batch 17 | 12 | ~1,270 |
| F | audit wave 286 | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0 (31 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 49 pairs / 13,832 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
