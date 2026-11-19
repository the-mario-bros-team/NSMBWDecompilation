# Audit - Wave 276 (three tables closed, compiler-helper defect class quantified)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80329E88` | **21/21** | `homebutton::gui::Component` |
| `0x8034E398` | **16/16** | `nw4r::g3d::AnmObjTexPatOverride` |
| - | - | (`0x8034E4B8` opened at 12/16) |

**Forty-one vtables complete.**

`Component` shares 18 of its 21 slots with `PaneComponent` (7 of 9 leaves are in
`0x80329E80`; slots 20 and 23 are `PaneComponent`-only). All three `0x120`-stride
NW4R override tables have pairwise distinct slot words.

Install-site note: `Component`'s `lis`/`addi` are **24 bytes apart**, so an
adjacent-pair sweep misses it. Several lanes rely on that sweep; the scanner
should match by register, not by adjacency.

## 2. Two more body defects

- **`fn_8019b9c0`** (`audio/ax.rs`): `cmpi r4,1` + `bc 4,2` skips the call when
  `mode != 1`, so it calls `0x8019BBA0` only for `mode == 1`. The body had
  `if mode != 1`. Fixed.
- The audit also confirmed the two fixes applied after wave 275
  (`fn_80171850` returning 0 on the no-match path, and `fn_80172a50`'s four-edge
  geometry).

## 3. Compiler-helper defect class quantified

`0x802DCD74`/`0x802DCDC0` are `_savegpr_27`/`_restgpr_27`: literal register
spill/restore leaves. A sweep of the cue region found **every** reference to the
family was a compiler helper, never a real callee:

| Region | Result |
|---|---|
| Cue bodies (`0x8005xxxx`/`0x8019xxxx`) | **17 bodies fixed, 32 bogus calls removed**, 0 genuine |
| Repo-wide remaining | **616 modelled helper calls in 324 functions** |

Four callers also had to pass the real incoming `r3`/`r4` once the fake
"returns" were removed. `FN_802DCD74`/`FN_802DCDC0` were deleted; the sibling
constants stay because non-cue bodies still reference them.

This is a correctness class, not a cosmetic one: a body that reads the "return
value" of a save/restore helper is reading whatever the caller passed in `r3`.
It needs a multi-wave sweep.

## 4. Survey

| Metric | Wave 272 | Wave 276 |
|---|---:|---:|
| Fully covered runs | 37 | **52** |
| Uncovered prologue words | 1,103 | **945** |
| Distinct prologue targets | 979 | **888** |
| Max prologue in one run | 11 | **9** |

Next targets: `0x80323fe0` (9, `Wm_en_magmawave`), `0x80301c00` (8,
`daBullet_c`), the `hanachan`/`Wm_en_burst` cluster (8+7+6), and three NW4R G3D
tables (6 each).

One **prose-only target** found: `0x802449b0` is cited in a comment in
`runtime/os.rs` with no body - queued.

## 5. Gap corpus

70 of 260 band candidates translated. Two scanner gaps found: opcode-63 A-form
instructions were mis-read as `fcmpu` (`fmr`/`fctiwz`), and a candidate whose doc
used `- Address:` instead of `Extent:` was invisible to the picker.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | close TexPat + open TexSrt | 4 + 10 | ~1,900 |
| B | close Component | 2 | 12 |
| C | `_savegpr` sweep (cue region) | 17 fixed | - |
| D | survey refresh | 0 | - |
| E | gap batch 6 | 12 | 540 |
| F | audit wave 275 | 0 (1 fix) | - |

## Verification

- `cargo check` exit 0 (28 warnings, two of them benign reassignment notices);
  `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 34 pairs / 13,702 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
