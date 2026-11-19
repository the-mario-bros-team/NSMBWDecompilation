# Audit - Wave 286 (six tables closed, savegpr below 150)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80328AF0` | **10/10** | `d_profile` base handle table |
| `0x80324778` | **54/54** | WorldMap ice projectile (`VTABLE_80324770`) |
| `0x803227B8` | **18** | `select_cursor` (5-record 0x264 layout controller) |
| `0x80322EF8` | **18** | `cursor` (8-player-record controller) |

**Seventy-five vtables complete.**

Boundary findings:

- The **19th word** of each cursor run (`0x80322800`, `0x80322F40`) is the
  **creator field of the next class descriptor** `{creator, magic, count}`, not a
  vtable slot. Counting it would have added a phantom slot and made the next
  class's table look like a continuation.
- `0x80328AE8` is the **base** handle table; the `"d_profile"` literal follows
  the *derived* table at `0x80310E98` (ctor `0x8008F870`), which is why the
  name-proximity heuristic pointed at the wrong one.
- The two cursor controllers share 13 targets (the `dBase_c` family and name
  getter) and have 6 distinct each - siblings, not copies.

## 2. `_savegpr`/`_restgpr` sweep, part 11

| Metric | Before | After |
|---|---:|---:|
| Modelled helper calls | 196 | **145** |
| Bodies containing them | 99 | **74** |
| Bodies changed | - | 25 |
| Calls removed | - | **51** |
| Genuine calls found | - | **0** |

All 25 in `enemies.rs` (`0x80A01120..0x80A71700`), verified against
`d_enemiesNP.elf`. Corrections this round were mostly semantic:

- two `call_u64` "returns" were really **argument pairs** (`fn_80a238b0` is
  `(actor, tag)`);
- three epilogue "arguments" were **allocation results** (now `-> usize`);
- two bodies with three exit calls each collapsed to plain `return`s;
- 12 bodies gained first parameters, 8 correctly stay argument-free.

Two live-register scan hits were verified as disassembler artefacts
(`srawi r4,r0,1` and `oris r6,r0,0x8000` write the register) rather than real
parameters.

## 3. Survey

| Metric | Wave 285 | Wave 286 |
|---|---:|---:|
| Fully covered runs | 82 | **90** |
| Distinct prologue targets (runs) | 620 | **588** |
| Runs with >=4 prologue | 48 | **42** |
| Max prologue in one run | 5 | 5 |

Eight runs completed this wave (seven from wave 285 plus the DVD-reader table
written after the prior snapshot). The remaining ten 5-prologue runs are the four
survivors from this wave's A/B lanes plus `0x80323898` (enemy ice),
`0x80327250` (fukidashi/Yoshi info) and four audio tables
(`0x8030f060`/`090`/`0f8`/`190`) sharing their five covered slots.

## 4. Audit findings

92 wave-285 bodies reproduced with **no body-level defects**. Doc fixes included
three wipe slot-1 extents (240 -> 228 bytes), the ScnMdl install offset, and a
re-attribution of the `0x80352670` block to **`daBossDemo_c`** (its strings are
`daBossDemo_c::StateID_*`) - reversing wave 285's `daBullet_c` claim.

Evidence hygiene note: lane C's `c_scan_start.txt` was overwritten with the
after-state, so the audit re-derived the 50 call sites from `c_bodies_out.txt`.

## 5. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `d_profile` + ice projectile close | 8 + 7 | 2,544 |
| B | cursor pair close | 11 | 1,512 |
| C | savegpr sweep part 11 | 25 fixed | - |
| D | survey refresh | 0 | - |
| E | gap batch 16 | 12 | ~1,220 |
| F | audit wave 285 | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0 (30 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 47 pairs / 13,779 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
