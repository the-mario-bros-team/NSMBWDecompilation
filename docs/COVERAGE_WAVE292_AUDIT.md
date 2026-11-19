# Audit - Wave 292 (five tables closed, warnings reduced)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80306F80` | **57/57** | ice-lift `daIceAshibaWater_c` family |
| `0x80306D68` | **51/51** | `obj_dokan` family base |
| `0x8034FB40` | **10/10** | layout header (installed by the `.brlyt` parser) |
| `0x8030EFC8` | **11/11** | `dStateID_c`-shaped record |
| `0x803226A4` | **12/12** | `W110_GOAL_STATE_VTABLE` |

**Ninety-seven vtables complete.**

Notable structure:

- `0x80306F80` and `0x80306D68` share 45 targets but none of their 12 uncovered
  ones; both are bases for REL-derived tables (`0x807EFCFC`/`0x807EFDF4`/
  `0x807EFEEC`, and `0x807EE1E8`/`2D8`/`3C8`/`4B8`).
- `0x8030EFC8` shares 9 targets with sibling `0x8030EF94`; the differences are
  slot 0 (`0x80067A20` vs `0x80067A80`) and slot 7 (raw `+8` getter vs chain
  resolver). Four of its slots already had `game::task` bodies, reused.
- `0x8034FB40` slot 1 (`0x802B3B00`, 3,592 B) is the resource-side twin of
  `ui::lyt::fn_801825a0`.
- `0x803226A4` is the derived state table over base `0x80328A18`: 6 inherited,
  6 overridden, installed at eight `0x80374C90 + 0x40n` records.

## 2. Warnings: 63 -> 21

Lane C fixed **42** warnings, all real:

| Category | Before | After |
|---|---:|---:|
| `unused_assignments` in `os.rs` | 32 | 0 |
| `unnecessary parentheses` in `bt.rs` | 10 | 0 |
| pre-existing elsewhere | 21 | 21 |

The unused assignments were seeds (`let mut block = 0`) left behind when the
consuming helper call was removed; each was folded into the real assignment.
One no-op `else { done += 1 }` was dropped only after proving it unreachable, and
`fn_401bba30`'s three SDA reads were preserved via `let _ = if ...`.

## 3. Body defect fixed

`fn_8010b890` omitted its publication `stw r30,-0x53C0(r13)` (`r30` loaded from
`r2-0x60FC` at `0x8010B904`). Added, with the register evidence documented.

This is the third dropped-write/tail class this stretch: `wave104`'s gate
argument, the EGG destroy tails, and now this store. Body docs should account for
every store in the extent, not only every call.

## 4. Survey

| Metric | Wave 290 | Wave 292 |
|---|---:|---:|
| Fully covered runs | 111 | **117** |
| Distinct prologue targets (runs) | 508 | **484** |
| Runs with >=4 prologue | 22 | **16** |
| Uncovered words | 4,485 | **4,420** |

Six runs completed; no regressions; interior labels unchanged at 2,127. The
remaining 4-prologue runs are now `0x80311370`, `0x8031151c`, `0x80322dd8`,
`0x80324a44`, `0x803276f8` plus the five closed this wave.

## 5. Audit

188 wave-291 bodies re-derived with no body-level findings. The audit also
re-verified the two EGG destroy bodies at 136 B with their previous-heap tails,
and corrected wave-291 lane D's "44-byte leaves" (they are 4-byte thunks).

Process note: wave-291's `b_verify.txt` and `a_verify_sites.txt` were truncated
and restored; both reproduce the published figures.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | actor pair close | 12 | ~2,000 |
| B | layout header + state record close | 8 + 3 | ~4,200 |
| C | warnings hygiene | 42 fixed | - |
| D | `W110_GOAL_STATE_VTABLE` close | 7 | ~350 |
| E | gap batch 22 | 12 | ~1,420 |
| F | survey + audit | 0 | - |

## Verification

- `cargo check` exit 0 (**21 warnings**); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 63 pairs, 0 mismatches;
  `check-helper-calls` 0 refs; `check-source-integrity` 0 findings.
- `scripts/check.ps1` exit 0.
