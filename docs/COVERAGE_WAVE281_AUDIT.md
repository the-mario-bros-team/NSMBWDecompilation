# Audit - Wave 281 (five tables closed, savegpr part 6)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80328A88` | **14/14** | `dScBoot_c` state-object vtable |
| `0x80328188` | **18/18** | `dScBoot_c` primary vtable (`VTABLE_BOOT_BASE`) |
| `0x80315894` | **17/17** | `Wm_en_burst_s` small-head |
| `0x8034FB80` | **7/7** | BRLAN/RLYT animation-target container |

**Fifty-five vtables complete.**

## 2. Identity corrections

- `0x80328188` is the **`dScBoot_c` primary vtable**, not `Pausewindow_c`: the
  `Pausewindow_c::StateID_*` strings merely precede it in `.data` (the same false
  lead wave 228 corrected for `0x803280E0`). Its `0x803281D0` word is **not** a
  terminator but the header of `VTABLE_BOOT_IFACE` (10 slots), and 7 of its
  slots override the base `dScene_c` table while 11 are shared.
- `0x8034FB80` is **not** an EGG allocator structure. The "EGG heap" label came
  from nearby `"cannot allocate from heap..."`/`eggExpHeap.cpp` strings; the code
  is a **BRLAN/RLYT animation-target container**: `'timg'` texture resolution in
  slot 2 and `RLPA`/`RLVI`/`RLVC`/`RLMC`/`RLTS`/`RLTP`/`RLIM` target dispatch in
  slots 5/6, with 16-byte `{owner, u16 index, u8 mode}` records.
- `0x80328A88`'s installed value is the header `0x80328A80`, materialised only at
  `0x8015FC90` and installed at `boot+0x8C`/`boot+0xC8`. No RTTI accessor, so
  identity came from the constructor.

## 3. `_savegpr`/`_restgpr` sweep, part 6

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 427 | **389** |
| Bodies containing them | 223 | **199** |
| Bodies examined / changed | 25 / 24 | - |
| Calls removed this wave | - | **38** |
| Constants deleted | - | 21 |
| Genuine calls found | - | **0** |

Several bodies gained their real parameters instead of a fake 64-bit "context"
pair (`fn_80174240`, `fn_80156c30`, `fn_80159880`), and `fn_80156d70` now
returns its `r3` result. `fn_801d6290` needed no edit: its supposed "site" was
module-comment prose.

Counting note: the quoted 442/224 baseline was not reproducible by a code scan
(it scores comment mentions). The reproducible metric reads 427/223 pre-edit and
389/199 after.

## 4. Survey

68 fully covered runs, **807** distinct prologue targets window-wide, max 6 per
run. New top targets: `0x80328a88` (done this wave), `0x80328188` (done),
`0x8034fb80` (done), `0x8030f15c` (effect-record base), and the EN59 `dEn`
sub-object vtables `0x80302d80`/`0x80301eb0` (185 slots each).

Gap corpus: **138 of 672** covered, 529 eligible.

## 5. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `dScBoot_c` state object close | 11 | 660 |
| B | `dScBoot_c` primary close | 9 | 640 |
| C | savegpr sweep part 6 | 24 fixed | - |
| D | small-head close + BRLAN/RLYT close | 1 + 7 | ~1,300 |
| E | gap batch 11 | 12 | ~1,000 |
| F | audit wave 280 | 0 (doc fixes) | - |

## 6. Doc debt

`docs/UNTABLED_POINTER_TABLES.md:1334` still says 4 tabled / 340 bytes for
`0x80328a88`; measured is 3 tabled / 660 bytes for the 11 new bodies, with the
installed value being the header `0x80328a80`. Queued for the doc-correction
lane.

## Verification

- `cargo check` exit 0 (30 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 39 pairs / 13,744 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
