# Audit - Wave 290 (five tables closed, sweep residue re-measured)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x8034E230` / `0x8034E350` / `0x8034E470` | **16/16** each | `AnmObjMatClrRes` / `AnmObjTexPatRes` / `AnmObjTexSrtRes` |
| `0x8034FD68` / `0x8034FDC8` | **10/10** each | EGG expanded heap (`EXPH`, kind 1) / unit heap (`UNTH`, kind 3) |

**Ninety-one vtables complete.**

Notes:

- The three animation-entry vtables share no slot words; their 12 uncovered
  bodies each are structurally identical apart from RTTI list, stride
  (`0x5C`/`0x44`/`0xE8`), refresh kernel and result zero-fill width.
- The "EGG heap factory" label was wrong: these are heap **object** vtables,
  distinguished by the magic strings their create callees materialise
  (`EXPH`/`UNTH`; siblings `FRMH` kinds 2/4 remain).
- `0x8034FDC8`'s type-C slot 6 is the `subfc`/`addze`/`and` masked size select;
  slot 9 trims and resizes the backing block in the parent heap.

## 2. Two body fixes

- `fn_80248d20`'s kernel third argument was `[+0x18] * (1 + scale)`; the image
  computes `[+0x18] + [+0x1C] * scale` (`lfs f2,0x1c(r3)` × `r13-0x5C78` added
  to `lfs f0,0x18(r3)`). Fixed.
- `fn_8018af70`'s CWDH chain followed `+0x08`; the link is at `+0x04`
  (`lwz r6,0x4(r6)`), while `+0x08` is the record offset. Fixed.

## 3. Helper sweep: residue was mis-measured, now swept and re-counted

Wave 289 claimed zero literal helper calls and a 220/89 residue. Both were
wrong:

- the token regex was **case-sensitive**, and `runtime/os.rs` spells the
  addresses `0x802d_cd70`;
- the body-attribution script used `indexOf(line)`, so every identical helper
  line collapsed into one body.

True residue before this wave: **310 refs / 167 bodies**. `input.rs` was
60 bodies (not 40) and one body had 2 sites, not 18.

**This wave swept `runtime/input.rs` completely:** 113 sites in 60 bodies, all
confirmed at prologue/epilogue positions in the DOL (41 saves, 72 restores, 0
mid-body), 113 calls removed, 24 constants deleted, 24 signatures rebound
(`r3` → `device`/`record`/`event`, `r3:r4` → two parameters), 12 call sites
adjusted, 60/60 `- Removed:` bullets.

Two sites named the *wrong* stub (`fn_801f3310` called `_restgpr_25` not 27;
`fn_801f0b10` called `_savegpr_25` not 27) - still display helpers, still
removed, and the bullets record the real numbers.

Remaining residue (address-based): `os.rs` **203/89**, `stage.rs` 58/44,
`bt.rs` 41/28, `registry.rs` 6/5, `bres.rs` 2/1.

## 4. Survey

| Metric | Wave 288 | Wave 290 |
|---|---:|---:|
| Fully covered runs | 102 | **111** |
| Distinct prologue targets (runs) | 538 | **508** |
| Runs with >=4 prologue | 31 | **22** |
| Uncovered slots | 4,608 | **4,485** |

Nine runs closed, no regressions, max prologue still 4.

## 5. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `input.rs` helper sweep | 60 fixed | - |
| B | three animation-entry tables close | 36 | ~3,000 |
| C | two EGG heap tables close | 20 | 944 |
| D | survey refresh | 0 | - |
| E | gap batch 20 | 12 | ~1,370 |
| F | audit wave 289 + recovery proof | 0 (1 fix) | - |

## Verification

- `cargo check` exit 0 (31 warnings); `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 63 pairs / 13,845 tokens,
  0 mismatches; `check-source-integrity` 68 files, 0 findings.
- `scripts/check.ps1` exit 0.
