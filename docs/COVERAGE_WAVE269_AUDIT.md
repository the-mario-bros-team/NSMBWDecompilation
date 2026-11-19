# Audit - Wave 269 (twenty-one vtables complete)

## 1. Five tables closed, one opened

| Table | Slots | Class |
|---|---:|---|
| `0x80304494` | **173/173** | `daEnDpakkunBase_c` |
| `0x80315EF0` | **46/46** | Kinopio panel/avatar |
| `0x803175E8` | **46/46** | Mario/Luigi character-model record |
| `0x8031C288` | **42/42** | `daYoshi_c` |
| `0x80323D40` | **78/78** | `daFireBall_Base_c` |
| `0x80305344` | 155/169 | `daEnLkuriboBase_c` (opened) |

**Twenty-one vtables complete.**

## 2. New shared-base relationships

- `daYoshi_c` (`0x8031C288`): 11 of its 12 new methods are the same slot indices
  in base table `0x803189A8` (ctor `0x800D5330`); the runs agree on 20/42 slots.
  Slot 21 is the one genuine override - the base stores `0x800D5DD0` directly,
  Yoshi uses a 4-byte trampoline.
- `daEnLkuriboBase_c` (`0x80305344`): installed by two `d_enemiesNP`
  constructors (`0x80ACD9A0`, `0x80ACE540`) at `+0x60`, then overwritten with
  leaf tables `0x80B6A808`/`0x80B6ABC0`. Target containment 0/169.
- Kinopio and the Mario/Luigi record share five `0x800BD7xx` methods at the same
  slot indices. One canonical copy was written; the other table reuses it.

## 3. Concurrency: five lanes, zero duplicate bodies

Wave 269 had five lanes writing `stage.rs`/`dsys.rs`/`player.rs`/`worldmap.rs`
simultaneously, with several shared methods. Every collision was resolved by the
lane that noticed it deleting **its own** copy:

| Address | Lanes | Resolution |
|---|---|---|
| `0x800BD710..760` | B269, C269 | C269 canonical in `stage.rs`; B269 skipped |
| `0x800BD6D0` | B269, D269 | D269 canonical in `dsys.rs` |
| `0x800BD620` | B269 only | B269's sole unique body |

The duplicate gate reported 0 failing at every checkpoint after the merges.

## 4. Control-flow decode subtleties recorded

- `bclr 12,1` = return-while-GT; `bclr 4,2` = return-when-EQ-clear.
- `bcctr 20,0` with LK=1 is a call and falls through; a tail `bcctr` forwards
  `r3`, so bodies using it must be typed `-> u32` when the caller consumes it.
- `fn_800bd620` selects a float row from `+0x160` bits: `0x00800000`/`0x01000000`
  force index 0, `0x00400000` picks row `+0x48` vs `+0x38`, and `0x00000008`
  gates the additive `+0x58` row. A `0.05f` correction applies only when
  `+0x152 == 3`.
- `0x80035960` (Lkuribo slot) is byte-identical to tabled `fn_800387d0`;
  `0x80044AF0`/`0x80044BF0` are byte-identical twins.

## 5. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `0x80304494` last 16 (closes) | 16 | 136 |
| B | `0x80315EF0` unique slot 44 (closes) | 1 | 116 |
| C | `0x803175E8` last 7 (closes) | 7 | ~250 |
| D | `0x8031C288` last 12 (closes) | 12 | 60 |
| E | `0x80323D40` last 21 (closes) | 21 | 440 |
| F | `0x80305344` first 12 | 12 | 1,844 |

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0; `check-page-offsets` 32 pairs / 13,633 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
