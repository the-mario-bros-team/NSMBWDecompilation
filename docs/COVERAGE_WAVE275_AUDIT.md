# Audit - Wave 275 (five tables closed, two body bugs)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80329CD0` | **24/24** | `homebutton::gui::PaneComponent` |
| `0x80329E18` | **14/14** | `homebutton::gui::Manager` |
| `0x8034E278` | **16/16** | `nw4r::g3d::AnmObjMatClrOverride` |
| `0x8031BF70` | **60/60** | `character_SV` base |
| `0x80308A08` / `0x80308A48` | **14/14** each | timed cue-group records of `daPlBase_c + 0xDEC` |

**Thirty-nine vtables complete.**

The homebutton GUI cluster is now mapped as a hierarchy: `Manager` derives from
`Interface`, `PaneManager` lists `Manager` as its second base (only slot 5
differs), and seven of the nine `PaneComponent` leaves are shared with the
`Component` base run `0x80329E80` (slots 9, 10, 11, 12, 15, 17 and 19); slots
20 (`0x80172D00`) and 23 (`0x80172E30`) are PaneComponent-only.

## 2. Two body defects found by the audit, verified and fixed

### `fn_80171850` (`ui/menu.rs`) - wrong return on the no-match path

`r31` is set to `1` only at `0x801719A0`, on the `+0x5C` match path; every other
path falls into the epilogue with `r31 == 0`. The body answered `1` on the
`+0x5C == 0` arm. Verified by dumping all 130 instructions of the 520-byte body:
the only constant writes to GPR31 are `li r31,0` at `0x80171870` and the single
`li r31,1` at `0x801719A0`.

### `fn_80172a50` (`ui/menu.rs`) - quad geometry

The image loads `cx = [frame+0x90]`, `cy = [frame+0xA0]`,
`w = 0.5*[frame+0x4C]`, `h = 0.5*[frame+0x50]` and emits four quads tracing the
frame rectangle:

| quad | f1 | f2 | f3 | f4 |
|---|---|---|---|---|
| 1 | `cx-w` | `cy-h` | `cx+w` | `cy-h` |
| 2 | `cx+w` | `cy-h` | `cx+w` | `cy+h` |
| 3 | `cx+w` | `cy+h` | `cx-w` | `cy+h` |
| 4 | `cx-w` | `cy+h` | `cx-w` | `cy-h` |

The body omitted the centre loads, used `0.0` for `y`, and derived all four
quads from a single `+0x50` extent. Rewritten to the table above. The "dead
loads" paragraph was the inverse of the image and now documents the real
geometry.

Colour handling was checked and is correct: the base word is `0xFF0000FF` at
`0x802F5A5C`, and the non-zero-byte arm writes bytes 0 and 2 (big-endian), giving
`0x0000FFFF`.

## 3. A body-defect class to sweep

`0x802DCD74` / `0x802DCDC0` are `_savegpr_27` / `_restgpr_27` compiler helpers,
not getters. Several **older** bodies in the `0x8005xxxx`/`0x8019xxxx` cue region
still model them as `call_u64`. New bodies ignore them correctly. Queued as its
own sweep.

## 4. Survey reconciliation

Lane D found the plan's "14/12 uncovered, 9 prologue" for `0x80308A08` was the
*tsv-only* count. Under the full tsv+ledger+Rust census the two cue tables had 5
uncovered each - exactly 10 prologue targets - all translated.

## 5. Gap corpus

58 of 260 band candidates translated, **202 free**. The "49" figure in earlier
notes counted three documented-but-deferred candidates (`0x800211B0` with its
undeclared `f31` input, `0x800D03A0`, `0x8002B260`) that are not translated.

## 6. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | close `character_SV` | 27 | 340 |
| B | close PaneComponent + Manager | 9 + 1 | 444 |
| C | close MatClrOverride + open TexPatOverride | 3 + 10 | 1,980 |
| D | close two cue tables | 10 | 2,636 |
| E | gap batch 5 | 12 | 540 |
| F | audit wave 274 | 0 (2 fixes) | - |

## Verification

- `cargo check` exit 0; 26 warnings; `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0; `check-page-offsets` 34 pairs / 13,701 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
