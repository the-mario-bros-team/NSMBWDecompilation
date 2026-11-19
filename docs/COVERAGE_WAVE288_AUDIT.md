# Audit - Wave 288 (four tables closed, sweep near completion)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80322838` (`pointGet`) | **18/18** | world-map point/counter HUD controller |
| `0x80322FB0` (`obj_coin`) | **158/158** | coin actor (`daEnEatCoin_c`) |
| `0x803272D8` (`fukidashiInfo`) | **18/18** | balloon info records (`fukidashiInfo.arc`) |

**Eighty-four vtables complete.**

## 2. Identity corrections

- Wave 270/271 labelled `0x80323368` `daEnEatCoin_c`, but that table carries
  `balloon.brres` and six `daEnHatenaBalloon_c` state names. The real
  `daEnEatCoin_c` is `obj_coin` at `0x80322FA8` (single `EatOut` state).
- `0x803272D0`'s base is 8 bytes before its first method; its descriptor
  `{0x80156EA0, 0x02C202C2, 0}` at `0x80327298` is the same creator word that
  wave 287 identified as the previous table's boundary - both readings agree.
- `pointGet` is the world-map HUD controller (34 `pointGet_02.brlyt` scenes,
  parked at `640.0f`/`352.0f`).

## 3. Page-offset gate caught a live mis-decode

Mid-lane, the new page checker flagged `lis r4,0x8036` + `addi -32636` as
`0x80358084`, not `0x80368084`. That is the tool doing its job before merge -
the first time a page error was stopped *during* translation rather than found by
a later audit.

Lane E independently corrected three more sign-extension folds
(`0x80358BD8`, `0x8034E080` = `AnmScn(NW4R)`, `0x8034FBD0`).

## 4. `_savegpr`/`_restgpr` sweep, part 13

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 94 | **47** |
| Bodies containing them | 49 | **24** |
| Bodies changed | - | 25 |
| Calls removed | - | **47** |
| Genuine calls found | - | **0** |

Second reversed-pair case: `fn_80c0add0` bound `mode = r3`/`actor = r4`, but the
image has `r3` = actor and `r4` = mode (`cmpwi cr0,r4,2`). Now `(actor, mode)`
with the downstream call matching. Four constructors returned their object
through the rest stub (`-> usize`); five epilogue "arguments" were return values;
13 bodies gained an `r3` parameter.

**The sweep is now at 47 calls in 24 bodies - one round from completion.**

## 5. Survey

| Metric | Wave 286 | Wave 288 |
|---|---:|---:|
| Fully covered runs | 90 | **102** |
| Distinct prologue targets (runs) | 588 | **538** |
| Max prologue in one run | 5 | **4** |
| Uncovered slots | 4,684 | **4,608** |

The entire top ten turned over; the new leaders are `ResFont`, two BTE
palette-device records, an audio element player, three small NW4R animation
tables, `VT_SEQ_PLAYER` and an EGG heap factory.

One **strict-test false negative** found: `0x80310db0`
(`COLLISION_DISPATCH_TABLE`, 16 slots) has five targets already implemented under
descriptive names (`collision_area_pair_*_resolve`). Adding `fn_<addr>` aliases
would remove 13 slots from the census with zero new translation. Queued.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `pointGet` + `obj_coin` close | 12 | 1,316 |
| B | `fukidashiInfo` close | 3 | 360 |
| C | savegpr sweep part 13 | 25 fixed | - |
| D | survey refresh | 0 | - |
| E | gap batch 18 | 12 | ~1,300 |
| F | audit wave 287 | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0 (31 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 52 pairs / 13,830 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
