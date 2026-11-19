# Audit - Wave 279 (five tables closed, savegpr part 4)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x8034E590` | **20/20** | `nw4r::g3d::AnmObjChrRes` |
| `0x80315778` | **17/17** | `Wm_en_burst_big` |
| `0x80315848` | **17/17** | `Wm_en_burst_s` (parent) |
| `0x8034F938` | **31/31** | `txt1` / `nw4hbm::lyt::TextBox` |

**Forty-nine vtables complete.**

## 2. Two structural corrections

- `0x8034E590`'s installed vtable pointer is the **header `0x8034E588`**, not the
  method array. That base shift makes slot 1's task-8 `bcctr` at `vtable+0x24`
  resolve to slot 7 (`0x80248D20`), which is type-consistent; it also explains
  slot 4's already-translated dispatch. An off-by-8 here would shift every
  dispatch slot by two methods.
- `0x8034F938` is the `txt1` TextBox table, not a plain pointer array. Its base
  is `0x8034F7F0` (`Pane`, 27 slots, 19 shared / 8 overridden / 4 new), and the
  ctor/dtor install only the header.

## 3. Census reconciliations

- `0x80315848`'s plan census (13 uncovered / 7 prologue) was **pre-wave-278**:
  wave 278 had already translated five of those seven prologue targets as child
  slots. Lane start was 9/17 with 8 uncovered; all eight were translated once,
  with shared words referenced, closing both tables.
- `0x8034F938` went 19/31 to 31/31 in one pass because its 12 uncovered were a
  contiguous tail, not scattered leaves.

## 4. `_savegpr`/`_restgpr` sweep, part 4

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 525 | **489** |
| Bodies containing them | 274 | **249** |
| Bodies changed this wave | - | 25 |
| Calls removed this wave | - | **36** |
| Constants deleted | - | 12 |
| Genuine calls found | - | **0** |

Two signature corrections fell out: `fn_800f0cc0` and `fn_800fa9c0` now return
`u32` instead of publishing through the rest stub, and `fn_800eee70`'s helper
pair was spurious (its entry has no save helper).

## 5. Warnings triage

All **30** tree-wide warnings are `unused_parens` (10), `unused_assignments` (15)
or `function_casts_as_integer` (5). There are **no `dead_code` or unused-item
warnings**, so the four added during the sweep waves are noise (dead cursor
increments, one dead store), not masked rot. Nothing changed.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | close AnmObjChrRes | 8 | ~200 |
| B | close burst_big + parent | 6 + 2 | ~700 |
| C | savegpr sweep part 4 | 25 fixed | - |
| D | `0x8034f938` close | 12 | 2,548 |
| E | gap batch 9 | 12 | ~860 |
| F | audit wave 278 + warnings | 0 (doc fixes) | - |

## 7. Findings

- `0x80160A20`/`0x80160A70` are the CodeWarrior static-initializer walker
  (null-terminated `mtctr`/`bcctrl` run); `4E800421` as fall-through keeps the
  single `blr` terminator correct.
- `0x801BB960` is the fatal `"DBExceptionDestination\n"` report; the trampoline
  branches there (`4BFFFFA4`), correcting an older doc that said `0x801BB95C`.
- Gap corpus: **114 of 672** covered, 553 eligible.

## Verification

- `cargo check` exit 0 (30 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 39 pairs / 13,737 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
