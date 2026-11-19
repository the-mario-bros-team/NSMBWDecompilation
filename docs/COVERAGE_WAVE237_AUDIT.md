# Audit - Wave 237 (batch 10, regression gate, vtable overlap)

## Bodies translated (54)

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `WM_PLAYER_VTABLE` 103-114 | 12 | 1,404 |
| C | WorldMap vtable 116-125 | 10 | 2,728 |
| D | `dEn_c` 76-89 | 10 | 1,488 |
| E | `0x80350230` family, first 10 uncovered | 10 | ~1,600 |

## Regression gate: clean, and it caught a fresh regression

| Check | Result |
|---|---|
| Arity | 2,408 mismatches, **0 live** (under-supplied set unchanged) |
| Duplicate bodies | 32 flagged, **0 real** (2 doc false positives; a body-text hash pass found no duplicated `fn_*`) |
| Literal provenance | 9,924 checked, **1 new page-offset mismatch found and fixed** |

The new mismatch was introduced by **this wave's own lane E**:
`fn_802c3980` cited `0x8030D1A4` where `lis`/`addi` gives `0x802FD1A4` - the
same `addi`-as-`ori` class. The correct page holds the documented five-word
table `0,1,2,8,6`; the old page holds unrelated packed data. Four sites fixed.

## Vtable overlap discovered

`WM_PLAYER_VTABLE` (`0x80326794`) and the WorldMap vtable run share function
pointers: slots 91-102 and 103-114 of the WorldMap run are the same addresses as
wave-236/237 `WM_PLAYER_VTABLE` work. Lane C detected this, skipped them, and
advanced to slots 116-125 rather than creating duplicate bodies. The two
`docs/UNTABLED_POINTER_TABLES.md` entries that described the overlapping ranges
were merged in the complete run inventory by wave 238 lane A, and each of the
two surviving tier rows now names the aliased slots.

## New table decoded: `0x80350230`

Not CodeWarrior-style: four Itanium-style sub-vtables, each with an 8-byte
`(offset-to-top, RTTI)` header.

| Sub-vtable | Slots |
|---|---:|
| primary `0x80350230` | 1 |
| `0x8035023C` | 2 |
| `0x8035024C` | 11 |
| `0x80350280` | 28 |

42 slots, boundary at `0x803502F8` (the unrelated `VTABLE_803502F8_F`). Six were
already translated; ten new ones landed, including an 804-byte UV/alpha
recompute.

## Other findings

- **Resolved (wave 238 lane A):** `dEn_c` slot 82 passes `this + 0x394` and
  queries tag `0x803585C4`; `stage::fn_80098ad0` loads the vtable word but also
  passes `this + 0x394` and queries `0x803576B4`. The descriptors are different
  because the two functions ask different tag questions; the asymmetry was the
  pre-fix `fn_80098ad0` Rust body, which passed the *stored word* instead of
  `this + 0x394`. The image (`lwzu r12,r3,0x394` + `bcctr` with `r3 =
  this+0x394`) settles it; that body is fixed.
- **Extent typo resolved:** wave 237 lane B's slot-110 note now says 756 B
  (`0x8004FFA0..0x80050290`), matching its own total.
- `dEn_c` slots 84/87 are a genuine byte-identical pair at two distinct
  addresses; both are translated.
- Wave 237 lane E's report and lane F's note now print the corrected
  `0x802FD1A4`.

## Docs repaired (lane A)

`.ctors` terminator corrected to `0x802EDCB4/8/C` in `init_tables.rs` **and**
`runtime/os.rs` `INIT_TABLE` (which had claimed `0x802edcb8`); stale page values
in `worldmap.rs`, `enemies.rs` and three audit docs; dedupe delegate notes in
`runtime/link.rs` and `worldmap.rs`; arity-growth notes in two older coverage
docs.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0; ledger 20,250 rows.
