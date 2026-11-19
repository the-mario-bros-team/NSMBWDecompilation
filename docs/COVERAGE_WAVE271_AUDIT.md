# Audit - Wave 271 (twenty-five vtables)

## 1. Three tables closed

| Table | Slots | Class |
|---|---:|---|
| `0x80323370` | **158/158** | `daEnEatCoin_c` |
| `0x8032B524` | **29/29** | `nw4hbm::lyt::Window` |
| `0x8032B43C` | **27/27** | `nw4hbm::lyt::TextBox` |

**Twenty-five vtables complete.**

The two NW4R tables share 11 targets. `TextBox` has only five prologue-shaped
uncovered bodies of its own (not the twelve the prompt assumed), so lane F
translated all nine TextBox-only bodies - including a 1,812-byte `DrawSelf` -
and left the five shared to the sibling lane, which closed `Window` with a
3,376-byte eight-cell frame body.

## 2. `daBossDemo_c` base and derived

`0x80301504` is the **base** vtable (77 slots: 44 inherited, 33 overrides, 15
appended) and `0x803017d8` the derived one (92 slots). Override slots:
`0,3,6,7,9,12,16,30,33,48,54..76`.

Notable: **no image installs `0x80301504`** - no `lis`/`addi`/`ori` and no data
word in `main.dol` or the four RELs references it. Target containment is 0/77.
The wave-266 "materialised outside a function" tag came from the adjacent wipe
descriptor base `0x803014E0`, which `0x8001D2D4`/`0x8001D690` do materialise.

The class block carries `daBossDemo_c::StateID_Ready/BattleStDemo/BattleIn/
BattleEdDemo` and the `wipeCircle`/`wipeDokan`/`wipeKuppa`/`wipeMario` names;
`0x8001D2C0` builds four `0x40`-byte wipe records at `0x80352670/6B0/6F0/730`.

## 3. Derived-override subtleties recorded

- `daBossDemo_c` slot 55 replaces the parent's `0x8001CF70` (180 B, three gates
  to `0x803526B0`) with a body that returns unless `+0x36C == 2`.
- Slot 60's **empty body suppresses** the parent's real 92-byte reset at
  `0x8001D090` - the derived table deliberately does nothing where the base
  resets.
- `bclr 4,2` was initially read inverted by the lane and corrected against the
  existing `fn_8003d290` precedent: it is branch-if-false.

## 4. NW4R colour accessor coherence

In `0x8034F9FC` (`wnd1`), slots 9/10's index-16 `bc` arms resolve to vtable
`+0x34`/`+0x38`, which are slots 11/12 of the same table - confirming slots 7-12
are the six consistent NW4R `Pane` colour accessors:
`GetVtxColor`, `SetVtxColor`, `GetColorElement`, `SetColorElement`,
`GetVtxColorElement`, `SetVtxColorElement`.

## 5. Coverage

| Table | Coverage |
|---|---:|
| `0x803017d8` (`daBossDemo_c`) | 74/92 |
| `0x80301504` (base) | 60/77 |
| `0x8034f9fc` (`wnd1`) | 29/33 |

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | close `0x8032b524` (13) | 13 | ~3,600 |
| B | close `0x80323370` (8) | 8 | 512 |
| C | `0x803017d8` next 12 | 12 | 84 |
| D | `wnd1` next 12 | 12 | 268 |
| E | `0x80301504` 7 prologue + 5 | 12 | 720 |
| F | `TextBox` 9 | 9 | 2,640 |

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0; `check-page-offsets` 34 pairs / 13,666 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
