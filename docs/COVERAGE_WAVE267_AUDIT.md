# Audit - Wave 267 (sixteen vtables, four new tables decoded)

## 1. Two in-flight tables closed

| Table | Slots | Class |
|---|---:|---|
| `0x80304940` | **186/186** | `daEnJimenPakkunBase_c` |
| `0x80305F18` | **185/185** | `daEnShell_c` |

**Sixteen vtables complete**: 275, 278, 311, 45, 306, 235, 265, 183, 182, 187,
186, 199, 190, 193, 186, 185.

## 2. Four new tables decoded

| Run | Slots | Class | Install | Coverage |
|---|---:|---|---|---:|
| `0x80304494` | 173 | `daEnDpakkunBase_c` | 4 `d_enemiesNP` ctors (`0x80A8EBD0`, `0x80A8F590`, `0x80A8FCD0`, `0x80A90B40`) | 145/173 |
| `0x80315EF0` | 46 | Kinopio (Toad) panel/avatar | `fn_800bcb80` (`stw +0`) | 22/46 |
| `0x803175E8` | 46 | Mario/Luigi character-model record (0x874 B) | `FUN_800CA060` (`stw +0`) | 27/46 |
| `0x8031C288` | 42 | `daYoshi_c` actor (0x404 B) | `0x800FE1D0` (`stw +0`) | 18/42 |

None is a dispatch table: all have zero headers, distinct targets and target
containment 0.00. Three install at object offset `+0`; one at `+0x60`.

`0x8031C288`'s adjacent state array (`0x802F1350`, 0x24-byte records) names
`wait`/`walk`/`run`/`b_dash`/`jump`/`jump2`/`2jmp_c_*`/`turn`, confirming the
Yoshi actor identity.

## 3. Cross-table sharing

Two pairs of new tables share methods:

- `0x80315EF0` (Kinopio) and `0x803175E8` (Mario/Luigi model) share seven
  `0x800Dxxxx` display methods. Lane D translated them as `_D267`; lane C's
  section defers to those bodies rather than duplicating.
- `0x8031C288` (Yoshi) and `0x80315EE8` share slot 23 `0x800D6200`; lane E
  skipped it and took the next prologue target.

The duplicate gate reported 0 failing throughout, so the "one owner per address"
rule held while four lanes wrote the same files concurrently.

## 4. Identity conflict to arbitrate

`0x80304494` is labelled `daEnDpakkunBase_c` (adjacent
`daEnDpakkunBase_c::StateID_Attack/Disappear/DieIceBreak/DieVanish` plus
`g3d/pakkun.brres`, `dead_leaf`, `dead_ice`). Wave 262 lane B used the same class
name for `0x80303104`. Both tables are in the pakkun family and the strings sit
close together, so one label is wrong or they are base/derived siblings.

Not changed on assumption: the `d_enemiesNP` constructor chain and the state
descriptor strings should decide it in a dedicated audit.

Lane C267 also corrected the survey's `wanwan_boss_iggy`/`kanaami` label for
`0x80315EF0` - those strings belong to a neighbouring model block, and the class
is Kinopio.

## 5. Measurement notes

- `bcctrl` (`0x4E800421`) falls through; conditional `bclr` falls through too.
  Lane C measured `0x800D3210` at 96 B across two `bcctrl` returns.
- Lane D documented a **forward-`b` trap**: `0x800D4790` is 272 B by linear walk
  but 484 B when the forward branch is followed.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | close `0x80304940` + `0x80305f18` | 24 | 196 |
| B | `0x80304494` first 12 | 12 | ~2,000 |
| C | `0x80315EF0` six distinct | 6 | 2,436 |
| D | `0x803175E8` twelve | 12 | ~2,700 |
| E | `0x8031C288` twelve | 12 | 1,988 |

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0; `check-page-offsets` exit 0 (0 mismatches, 38
  report-only ambiguities).
- `scripts/check.ps1` exit 0.
