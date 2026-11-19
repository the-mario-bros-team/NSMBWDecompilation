# Audit - Wave 273 (four tables advanced, one logic bug, encoding cleanup)

## 1. A logic inversion in `fn_8001e150`

The image is:

```
8001e160: cmp cr0,r6,r0        ; side byte +0x348 vs +0x3F0
8001e190: bc 4,2,0x8001e1a8    ; jump when NOT equal
8001e194: sth r5,0x3F8(r3)     ; equal: store the argument as-is
8001e1a8: neg r4,r5            ; not equal: negate
8001e1b0: sth r4,0x3F8(r3)
```

The body negated on **equality**. Corrected to negate on inequality, and the
plain-English paragraph (which repeated the inversion) aligned with the
"observed" line.

Decode note: `lfs f5,...` and `sth r5,...` both print as `r5` in the local
disassembler, but they are FPR5 and GPR5 - so the stored value really is the
`i16` argument, and the signature was already right.

## 2. Tables advanced

| Table | State | Class |
|---|---:|---|
| `0x8031bf70` | 21/60, prologue 0 | shared `dBase_c`-derived `character_SV` actor base |
| `0x80329d78` | 14/23, 1 prologue | `homebutton::gui::PaneManager` |
| `0x8034e5e8` | 12/20, 1 prologue | NW4R G3D child-container (`ScnGroup` inferred) |
| `0x8032df1c` | **42/42 closed** | NW4R LYT window-manager singleton (0x970 B) |

**Thirty vtables complete.**

Structure recorded:

- `0x8031bf70` is installed by six `d_basesNP` constructors with the identical
  relocated triple; all open with `0x800F5FD0(this)` which installs the
  grandparent `0x8031BE58`, and slot 16 is the matching destructor that
  re-installs the table.
- `0x80329d78`'s RTTI resolves the name literal
  (`0x80329DD4 = "homebutton::gui::PaneManager"`); its neighbours are
  `PaneComponent` (`0x80329CD0`), `Manager` (`0x80329E18`) and `Component`
  (`0x80329E88`) - a coherent GUI cluster for a future round.
- `0x8034e5e8` is the *most derived* of a three-level chain
  (`0x8034E690` -> `0x8034E638` -> `0x8034E5E0`) installed by `0x802481B0`.
  Wave 274 resolved its class from the slot-0 type-name records
  (`0x802F86B4` and siblings): `nw4r::g3d::AnmObjChrBlend`. At wave-273 time
  the name was still the `ScnGroup` structural inference.
- `0x8032df1c` is the LYT window manager: 21 `addi r3,-0xF8` adjustor thunks
  (10 from waves 247/248, 11 new), derived from a 0x8AC-byte archive/voice base
  whose `0x80350598`/`0x803505A8`/`0x803505BC` tables are all overwritten.

## 3. Encoding regression cleaned

604 CP1252 mojibake occurrences across 7 files (`â€”`, `Â±`, box-drawing art in
`bres.rs`/`enemies.rs`/`dsys.rs`) were found while chasing an unused-import
warning. They came from a PowerShell write path that decoded UTF-8 as CP1252.

Fixed in two passes: a true CP1252-to-UTF-8 round-trip where the bytes survived,
then collapsing the double-mangled runs to ASCII hyphens. Result: **0 remaining
markers**, 628 runs collapsed, 51 files touched. Typographic dashes and quotes
are ASCII now, so the same write path cannot corrupt them again.

Also removed an unused `fn_8018a5c0` import from `ui/menu.rs`; the warning count
is back to 26.

## 4. Gap corpus

Gap batch 3 added 12 functions (all `9421FFF0` prologues, single `blr`). Census:
**37 of 260** band candidates translated, **223 free**.

The lane also found that its local scanner treated `4E800421` (`bcctrl`) as a
terminating `bcctr`, truncating `0x800E8C00`. Same defect class as the corpus
audit, now in a tool - worth putting in the shared disassembler.

## 5. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `0x8031bf70` 11 prologue | 11 | 1,536 |
| B | `0x80329d78` 10 prologue | 10 | 2,436 |
| C | `0x8034e5e8` 10 prologue | 10 | 3,232 |
| D | `0x8032df1c` close (9 prologue + 11 thunks) | 20 | 1,064 |
| E | gap batch 3 | 12 | 792 |
| F | audit wave 272 | 0 (doc fixes) | - |

## Verification

- `cargo check` exit 0; 26 warnings; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0; `check-page-offsets` 34 pairs / 13,684 tokens,
  0 mismatches.
- `scripts/check.ps1` exit 0.
