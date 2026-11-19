# Audit - Wave 278 (four more tables, savegpr part 3)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x8034E860` | **14/14** | `nw4r::g3d::ScnRoot` |
| `0x80323FE0` | **55/55** | `Wm_en_magmawave` |
| `0x803294E0` | **14/14** | derived `nw4r::lyt::Layout`-style vtable |

**Forty-five vtables complete.** (`0x8034E590` and `0x80315778` opened at 12/20
and 11/17.)

`0x803294E0` is a **derived** table: its base `0x8034F878` has the same 14
methods except slots 0 and 3 (deleting destructor, clone+unlink). Install sites
are only two `stw r4,0(r31)` with `r31 = this+0x10`, so it is the secondary
vtable of a two-base layout object.

`0x80315778` is the `Wm_en_burst_big` class, and the survey's three "siblings"
are actually a base/derived chain: `0x803158D8` -> `0x80315840`/`0x80315848`
(this table's parent) -> `0x80315770`, with `0x8031588C` -> `0x803157E0` as the
parallel variant. Each level has its own slot-0 destructor.

## 2. `_savegpr`/`_restgpr` sweep, part 3

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 567 | **525** |
| Bodies containing them | 299 | **274** |
| Bodies changed this wave | - | 25 |
| Calls removed this wave | - | **42** |
| Constants deleted | - | 5 |
| Genuine calls found | - | **0** |

Bodies were `0x800D0470..0x800EAD90` (14 in `display.rs`, 11 in `dsys.rs`). One
body (`fn_800d0470`) turned out to be a leaf with no save helper at all; its two
alias-modelled calls were spurious and were removed.

The audit independently decoded the helper stubs: `0x802DCD60/AC` = `_22`,
`64/B0` = `_23`, `6C/B8` = `_25`, `70/BC` = `_26`, `74/C0` = `_27`, spill via
`r11`.

## 3. Body defect fixed

`fn_8001f900` used the entry copy of `+0x04` for the `+0x5F0`/`+0x5F4` stores,
but the image **re-reads** `lwz r5,0x4(r31)` at `0x8001F994` after
`fn_8001fa70`. Behaviour coincided only because that callee does not write `+4`.
Fixed to re-read at the store site.

## 4. Gap corpus

102 of 672 candidates covered, 565 eligible. Batch 8 was mostly deleting
destructors for `lyt`/`bt`/`archive`/`gx_band` classes, plus
`0x800B2F60` (`[-1,1)` random scale) and `0x800B9120` (vtable `+0x10` call).

## 5. Findings

- `0x802449B0` returns `0x802F8588`; the only naming word in the DOL and all four
  RELs is `0x8034E284`.
- `0x80323FE0` has **27** `0x9421xxxx` prologue-shaped first words, not 26.
- `0x803294E0` slot 13 is a byte-identical inline of the translated
  `fn_802ac8b0` applied to `*(this+0x10)`, so it delegates.
- A lane repaired a **foreign** doc-lint break in `display.rs` (blank line inside
  a doc block), comment-only.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | close ScnRoot + open `0x8034e590` | 4 + 8 | 2,288 |
| B | close magmawave + open burst_big | 3 + 8 | ~2,400 |
| C | savegpr sweep part 3 | 25 fixed | - |
| D | `0x803294e0` close | 13 | 2,456 |
| E | gap batch 8 | 12 | ~900 |
| F | audit wave 277 | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0 (30 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 39 pairs / 13,743 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
