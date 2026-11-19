# Wave 229 lane D — ledger integrity audit

Read-only structural audit of `ledger/functions.csv`. No Rust file, CSV, tool,
or existing document was modified; the only file written is this report.

Inputs:

- `ledger/functions.csv` — 20,250 data rows plus header, 7 columns, 0
  malformed/ragged rows (CSV parsed with quote handling).
- `work/ghidra/*/export/` — 9 Ghidra project directories, 7 of which have an
  `export/` folder, containing 20,259 distinct export basenames.
- `src/**/*.rs` — 45 Rust files referenced by the ledger.

Status/confidence distribution for context: `translated` 20,020, `partial` 217,
`stub` 13; `high` 12,772, `medium` 7,209, `low` 269.

## Verdict

| # | Failure class | Rows affected | Hard failures |
| --- | --- | ---: | ---: |
| 1 | `evidence` cites a file that does not exist | 0 | 0 |
| 2 | row has no usable evidence file path (inline/dep-only evidence) | **54** | 54 (data quality) |
| 3 | `rust_file` missing on disk | 0 | 0 |
| 4 | no `fn_<address>` body and address not documented in the file | 0 | 0 |
| 5 | no `fn_<address>` body, but address documented instead | 1,190 (1,189 other-named bodies + 1 inline-only) | 0 |
| 6 | address format violation | 0 | 0 |
| 7 | duplicate address (global) | 0 | 0 |
| 8 | duplicate `(module, address)` | 0 | 0 |
| 9 | `module` ↔ `rust_file` inconsistency | 0 | 0 |

All cited evidence paths exist, every `rust_file` exists, and every row has a
`fn_<address>` body or documents the address (0 unaccounted). The 54 rows that
cite no usable evidence path are the one systematic data-quality gap, and the
`801ae0a0` body anchor is the only traceability caveat.

## Check definitions

- **Evidence path.** First matching `*.c` token in the `evidence` column. Full
  `work/ghidra/<project>/export/<file>.c` paths are resolved directly; bare
  basenames (`Ghidra <name>.c`, `<addr>_FUN_<addr>.c`) are resolved against all
  `work/ghidra/*/export/` directories. A row passes if a real file exists.
- **Rust file.** `rust_file` resolved relative to the repository root and must
  be a regular file.
- **Body.** Literal `fn_<8-hex-address>` identifier in the file. Address
  documentation additionally accepts the `0x801a_e0a0` and `fn_801ae0a0`
  spellings used inside Rust comments.
- **Address format.** `^[0-9a-f]{8}$` (lowercase, no `0x`, exactly 8 digits).
- **Module/file consistency.** `rust_file` must equal
  `src/<module with :: → />.rs`, and each module must map to exactly one file
  and each file to exactly one module.

## 1. Evidence paths

| Evidence form | Rows | Result |
| --- | ---: | --- |
| Full repo-relative path `work/ghidra/<project>/export/…c` | 16,836 | all exist |
| Bare export basename (`Ghidra …c` or just `…c`) | 3,360 | all resolve to ≥1 export |
| No evidence file path | **54** | listed below |

No export path is dangling: all 16,836 explicit paths exist exactly as written
and all 3,360 bare export basenames resolve. The one `OSReset.c` occurrence is a
quoted runtime assertion string, not a path (see 1a). There are no Windows
backslash paths and no paths outside the repository.

### 1a. Rows with no evidence file path (54)

These rows carry inline raw-PPC/semantic evidence or cite a harmless string
literal instead of an export file. Each already has a matching Ghidra export on
disk; adding the suggested path would close the class. `80004050` uses
`…_entry.c` and `800e4820` is a thunk, so the basename is not always
`<addr>_FUN_<addr>.c`.

| Line | Module | Address | Status | Suggested evidence path |
| ---: | --- | --- | --- | --- |
| 664 | `boot::cache` | `80004330` | translated | `work/ghidra/main-gb/export/80004330_FUN_80004330.c` |
| 665 | `boot::entry` | `80004000` | translated | `work/ghidra/main-gb/export/80004000_FUN_80004000.c` |
| 666 | `boot::entry` | `80004030` | translated | `work/ghidra/main-gb/export/80004030_FUN_80004030.c` |
| 667 | `boot::entry` | `80004040` | translated | `work/ghidra/main-gb/export/80004040_FUN_80004040.c` |
| 668 | `boot::entry` | `80004050` | translated | `work/ghidra/main-gb/export/80004050_entry.c` |
| 669 | `boot::entry` | `800041c0` | partial | `work/ghidra/main-gb/export/800041c0_FUN_800041c0.c` |
| 670 | `boot::entry` | `80004300` | translated | `work/ghidra/main-gb/export/80004300_FUN_80004300.c` |
| 671 | `boot::init_tables` | `80004250` | translated | `work/ghidra/main-gb/export/80004250_FUN_80004250.c` |
| 672 | `boot::mem` | `80004364` | translated | `work/ghidra/main-gb/export/80004364_FUN_80004364.c` |
| 673 | `boot::mem` | `80004600` | translated | `work/ghidra/main-gb/export/80004600_FUN_80004600.c` |
| 674 | `boot::mem` | `800046b4` | translated | `work/ghidra/main-gb/export/800046b4_FUN_800046b4.c` |
| 1097 | `engine::dsys` | `800e45f0` | translated | `work/ghidra/main-gb/export/800e45f0_FUN_800e45f0.c` |
| 1098 | `engine::dsys` | `800e4660` | translated | `work/ghidra/main-gb/export/800e4660_FUN_800e4660.c` |
| 1099 | `engine::dsys` | `800e4680` | translated | `work/ghidra/main-gb/export/800e4680_FUN_800e4680.c` |
| 1100 | `engine::dsys` | `800e46e0` | translated | `work/ghidra/main-gb/export/800e46e0_FUN_800e46e0.c` |
| 1101 | `engine::dsys` | `800e4740` | translated | `work/ghidra/main-gb/export/800e4740_FUN_800e4740.c` |
| 1102 | `engine::dsys` | `800e47a0` | translated | `work/ghidra/main-gb/export/800e47a0_FUN_800e47a0.c` |
| 1103 | `engine::dsys` | `800e4820` | translated | `work/ghidra/main-gb/export/800e4820_thunk_FUN_8006a320.c` |
| 1104 | `engine::dsys` | `800e4830` | translated | `work/ghidra/main-gb/export/800e4830_FUN_800e4830.c` |
| 1105 | `engine::dsys` | `800e4850` | translated | `work/ghidra/main-gb/export/800e4850_FUN_800e4850.c` |
| 1106 | `engine::dsys` | `800e48a0` | translated | `work/ghidra/main-gb/export/800e48a0_FUN_800e48a0.c` |
| 1107 | `engine::dsys` | `800e48d0` | partial | `work/ghidra/main-gb/export/800e48d0_FUN_800e48d0.c` |
| 1485 | `engine::frame` | `800e4e40` | translated | `work/ghidra/main-gb/export/800e4e40_FUN_800e4e40.c` |
| 1628 | `engine::main_loop` | `802e9300` | partial | `work/ghidra/main-gb/export/802e9300_FUN_802e9300.c` |
| 1629 | `engine::main_loop` | `802e9398` | partial | `work/ghidra/main-gb/export/802e9398_FUN_802e9398.c` |
| 1739 | `engine::startup` | `800c9f10` | translated | `work/ghidra/main-gb/export/800c9f10_FUN_800c9f10.c` |
| 1740 | `engine::startup` | `800c9f40` | translated | `work/ghidra/main-gb/export/800c9f40_FUN_800c9f40.c` |
| 1741 | `engine::startup` | `800c9f60` | translated | `work/ghidra/main-gb/export/800c9f60_FUN_800c9f60.c` |
| 1742 | `engine::startup` | `800c9f90` | translated | `work/ghidra/main-gb/export/800c9f90_FUN_800c9f90.c` |
| 12669 | `game::scene` | `800cec10` | translated | `work/ghidra/main-gb/export/800cec10_FUN_800cec10.c` |
| 12670 | `game::scene` | `800ced70` | translated | `work/ghidra/main-gb/export/800ced70_FUN_800ced70.c` |
| 12671 | `game::scene` | `800cedb0` | translated | `work/ghidra/main-gb/export/800cedb0_FUN_800cedb0.c` |
| 12672 | `game::scene` | `800cee20` | translated | `work/ghidra/main-gb/export/800cee20_FUN_800cee20.c` |
| 12673 | `game::scene` | `800cee90` | translated | `work/ghidra/main-gb/export/800cee90_FUN_800cee90.c` |
| 12674 | `game::scene` | `800cefe0` | translated | `work/ghidra/main-gb/export/800cefe0_FUN_800cefe0.c` |
| 12675 | `game::scene` | `800cf080` | translated | `work/ghidra/main-gb/export/800cf080_FUN_800cf080.c` |
| 12676 | `game::scene` | `800cf110` | translated | `work/ghidra/main-gb/export/800cf110_FUN_800cf110.c` |
| 12677 | `game::scene` | `800cf290` | partial | `work/ghidra/main-gb/export/800cf290_FUN_800cf290.c` |
| 12678 | `game::scene` | `800cf410` | partial | `work/ghidra/main-gb/export/800cf410_FUN_800cf410.c` |
| 12679 | `game::scene` | `800cf590` | translated | `work/ghidra/main-gb/export/800cf590_FUN_800cf590.c` |
| 12680 | `game::scene` | `800cf820` | translated | `work/ghidra/main-gb/export/800cf820_FUN_800cf820.c` |
| 12681 | `game::scene` | `800cf960` | translated | `work/ghidra/main-gb/export/800cf960_FUN_800cf960.c` |
| 12682 | `game::scene` | `800cfab0` | translated | `work/ghidra/main-gb/export/800cfab0_FUN_800cfab0.c` |
| 12683 | `game::scene` | `800cfb30` | translated | `work/ghidra/main-gb/export/800cfb30_FUN_800cfb30.c` |
| 12686 | `game::scene` | `800e19b0` | partial | `work/ghidra/main-gb/export/800e19b0_FUN_800e19b0.c` |
| 12687 | `game::scene` | `800e1a20` | translated | `work/ghidra/main-gb/export/800e1a20_FUN_800e1a20.c` |
| 12688 | `game::scene` | `800e1aa0` | translated | `work/ghidra/main-gb/export/800e1aa0_FUN_800e1aa0.c` |
| 12689 | `game::scene` | `800e1be0` | translated | `work/ghidra/main-gb/export/800e1be0_FUN_800e1be0.c` |
| 12692 | `game::scene` | `800e1db0` | translated | `work/ghidra/main-gb/export/800e1db0_FUN_800e1db0.c` |
| 12694 | `game::scene` | `800e1de0` | translated | `work/ghidra/main-gb/export/800e1de0_FUN_800e1de0.c` |
| 12695 | `game::scene` | `800e1e60` | translated | `work/ghidra/main-gb/export/800e1e60_FUN_800e1e60.c` |
| 12696 | `game::scene` | `800e1f50` | translated | `work/ghidra/main-gb/export/800e1f50_FUN_800e1f50.c` |
| 12697 | `game::scene` | `800e1f60` | translated | `work/ghidra/main-gb/export/800e1f60_FUN_800e1f60.c` |
| 18796 | `runtime::link` | `801b1f20` | translated | `work/ghidra/main-gb/export/801b1f20_FUN_801b1f20.c` |

Line 665 is the `OSReset.c` row: the citation is actually the runtime assertion
string `OSReset.c:0x4A1` quoted from the binary, not a file path; the real
export is `80004000_FUN_80004000.c`. The other 53 are deliberately inline
raw-PPC evidence in boot/engine/scene/link bodies; they are not wrong, but they
cannot be mechanically checked.

Note: `boot::entry` is the only module with zero explicit-path rows (5 inline
evidence plus the incidental `OSReset.c` string, i.e. all 6 rows lack a usable
path). `game::scene` has 24 inline-evidence rows out of 31.

### 1b. Evidence provenance warnings (not failures)

- **1,501 rows** cite a bare basename that exists in more than one
  `work/ghidra/*/export/` project (typically `main` and `main-gb`). The row does
  not say which project was used, so the resolved byte source is ambiguous.
- **4 rows** (`ui::hud` lines 19612–19615, addresses `800270f0`, `800271d0`,
  `80027290`, `800272c0`) cite `work/ghidra/main/export/...`, while the other
  500 explicit `ui::hud` rows cite `main-gb`. The files exist, but the module
  mixes projects; pick one project or record why the `main` copy is canonical.

## 2. `rust_file`

All 20,250 rows reference an existing `.rs` file. 45 distinct files, all
present, non-empty, and under `src/`.

## 3–4. Body / address presence

| Result | Rows |
| --- | ---: |
| `fn_<address>` declared body present | 19,060 |
| no `fn_<address>` body; body under another name and address documented | 1,189 |
| no body at all; address documented inline | 1 |
| neither body nor address documentation | **0** |

So the requested criterion holds for all 20,250 rows.

Breakdown of the 1,190 rows without a `fn_<address>` body:

- 1,189 have a body under a different name — 1,188 semantic Rust names
  (`fn <name>`, e.g. `flush_cache`, `save_task_release`) plus the typo'd
  `fn_8060180` (line 1807, address `80600180`) — and an `# Original` section
  within 14 lines that states the address. 1,188 of them also cite the export
  path in that block; the `fn_8060180` block states the address and byte size
  (84) but not the path. Eight of the 1,188 also mention `fn_<address>` only as
  an intra-doc link (`[`fn_80afe3c0`]` etc.); those eight addresses are
  `80afe3c0` (`game::enemies`), `80094660` (`game::stage`), `8019f660`,
  `8019f700`, `8019fa00`, `8019fe50` (`resource::bres`), and `801cf8f0`,
  `801d33f0` (`runtime::dvd`). The `fn_<address>` doc links have no matching
  item and will not resolve under rustdoc.
- 1 has no named body and no `# Original` section: **`runtime::init` line 18399,
  `801ae0a0`, status `stub`**. `src/runtime/init.rs` mentions the address only
  as the callee in the inline comment at lines 276–279:
  `call1(0x801a_e0a0, 0x8038_e520)` with a note that the 36-byte helper copies
  0x1c bytes from `0x800030F0` or zeroes the first word. The Ghidra evidence
  (`801ae0a0_FUN_801ae0a0.c`) exists and matches that description, but there is
  no `fn_801ae0a0` body and no `# Original` block. This is a traceability soft
  miss: the ledger says `stub`, and the file documents the address inline
  rather than as a body. Fix by adding a small `pub unsafe fn fn_801ae0a0(...)`
  wrapper (or an `# Original` doc block) that points at the export.

No row has a `fn_<address>` name pointing at an address that is absent from the
file, and no semantic-named row lacks its named body (the only semantic row
without a `fn <name>` body, `game::bosses` `80c000f0` "entry", has
`fn_80c000f0` instead).

### Naming inconsistency (not a failure)

| `name` form | Rows | Note |
| --- | ---: | --- |
| `fn_<address>` | 18,439 | canonical; one row (`801ae0a0`) lacks the body |
| `FUN_<address>` | 621 | Ghidra spelling; body is `fn_<address>` |
| semantic name | 1,189 | 1,188 have `fn <name>`; `game::bosses` `80c000f0` uses `fn_80c000f0` |
| `fn_8060180` (malformed) | 1 | line 1807, address `80600180`; 7-digit typo for `fn_80600180` |

The 621 `FUN_` rows are concentrated in `game::bosses` (601), `game::actor`
dokan (5), `game::player` (8), `runtime::cpp` (7). They pass the body check but
break the `name == fn_<address>` convention; rename them if tooling assumes the
canonical form. The one malformed `fn_8060180` (address `80600180`) should be
renamed to `fn_80600180` in both the ledger and `src/game/bases.rs`.

## 5. Address format

- 20,250/20,250 match `^[0-9a-f]{8}$`.
- 0 uppercase/`FUN_`-style addresses in the `address` column, 0 short/long
  values, 0 `0x` prefixes, 0 non-hex values.
- Column is consistently lowercase hex, even where the evidence filename or
  Rust doc uses uppercase (`FUN_80041C00`).

## 6. Duplicates

- Global duplicate addresses: **0** (20,250 unique addresses).
- Duplicate `(module, address)` pairs: **0**.
- No two modules share a Rust file, and no module spans multiple files
  (45 modules, 45 files, bijective).

## 7. Module / file consistency

Every row satisfies `rust_file == "src/" + module.replace("::", "/") + ".rs"`:
45/45 modules, 0 mismatches. Examples: `game::actors::dokan` →
`src/game/actors/dokan.rs`; `runtime::os` → `src/runtime/os.rs`;
`audio::ax` → `src/audio/ax.rs`.

## Per-module inventory and body coverage

`fn` = row has a declared `fn_<address>` body; `sem` = row has a semantic-named
body and the address is documented; `none` = no body (address documented
inline).

| Module | Rows | `fn` | `sem` | `none` | Inline evidence |
| --- | ---: | ---: | ---: | ---: | ---: |
| `audio::ax` | 399 | 399 | 0 | 0 | 0 |
| `audio::seq` | 263 | 263 | 0 | 0 | 0 |
| `boot::cache` | 1 | 0 | 1 | 0 | 1 |
| `boot::entry` | 6 | 4 | 2 | 0 | 5 |
| `boot::init_tables` | 1 | 0 | 1 | 0 | 1 |
| `boot::mem` | 3 | 0 | 3 | 0 | 3 |
| `engine::display` | 402 | 320 | 82 | 0 | 0 |
| `engine::dsys` | 397 | 397 | 0 | 0 | 11 |
| `engine::frame` | 15 | 7 | 8 | 0 | 1 |
| `engine::globals` | 47 | 47 | 0 | 0 | 0 |
| `engine::main_loop` | 203 | 203 | 0 | 0 | 2 |
| `engine::startup` | 4 | 0 | 4 | 0 | 4 |
| `game::actor` | 57 | 55 | 2 | 0 | 0 |
| `game::actors::dokan` | 5 | 5 | 0 | 0 | 0 |
| `game::bases` | 5,607 | 5,606 | 1 | 0 | 0 |
| `game::boot_scene` | 12 | 5 | 7 | 0 | 0 |
| `game::bosses` | 770 | 770 | 0 | 0 | 0 |
| `game::enemies` | 3,555 | 3,484 | 71 | 0 | 0 |
| `game::interaction` | 17 | 4 | 13 | 0 | 0 |
| `game::path` | 37 | 37 | 0 | 0 | 0 |
| `game::player` | 830 | 830 | 0 | 0 | 0 |
| `game::profile` | 36 | 11 | 25 | 0 | 0 |
| `game::scene` | 31 | 0 | 31 | 0 | 24 |
| `game::stage` | 1,226 | 957 | 269 | 0 | 0 |
| `game::task` | 16 | 0 | 16 | 0 | 0 |
| `game::worldmap` | 1,001 | 825 | 176 | 0 | 0 |
| `game::yoshi` | 7 | 7 | 0 | 0 | 0 |
| `render::geometry` | 50 | 45 | 5 | 0 | 0 |
| `render::gx` | 3 | 3 | 0 | 0 | 0 |
| `render::gx_band` | 345 | 345 | 0 | 0 | 0 |
| `resource::archive` | 747 | 713 | 34 | 0 | 0 |
| `resource::bres` | 261 | 126 | 135 | 0 | 0 |
| `resource::course` | 19 | 0 | 19 | 0 | 0 |
| `resource::registry` | 423 | 352 | 71 | 0 | 0 |
| `runtime::bt` | 695 | 695 | 0 | 0 | 0 |
| `runtime::cpp` | 361 | 361 | 0 | 0 | 0 |
| `runtime::dvd` | 536 | 464 | 72 | 0 | 0 |
| `runtime::ids` | 4 | 4 | 0 | 0 | 0 |
| `runtime::init` | 7 | 1 | 5 | 1 | 0 |
| `runtime::input` | 388 | 380 | 8 | 0 | 0 |
| `runtime::link` | 12 | 4 | 8 | 0 | 1 |
| `runtime::os` | 537 | 451 | 86 | 0 | 0 |
| `ui::hud` | 542 | 542 | 0 | 0 | 0 |
| `ui::lyt` | 221 | 217 | 4 | 0 | 0 |
| `ui::menu` | 151 | 121 | 30 | 0 | 0 |
| **Total** | **20,250** | **19,060** | **1,189** | **1** | **53** |

## Recommended fixes (in priority order)

1. **`runtime::init` `801ae0a0`** — add `fn_801ae0a0` (or an `# Original`
   block) in `src/runtime/init.rs` for the inline `0x801a_e0a0` helper so the
   ledger's `stub` row has a concrete body anchor.
2. **54 evidence-path rows** — replace the inline prose with the suggested
   `work/ghidra/main-gb/export/...c` path listed in section 1a, keeping the
   useful raw-PPC notes after the path.
3. **621 `FUN_<address>` names** — rename to the canonical `fn_<address>` form
   (mostly `game::bosses`) if any tooling keys on the name; also rename the
   single `fn_8060180` typo to `fn_80600180`.
4. **1,501 ambiguous bare basenames** — qualify with the
   `work/ghidra/<project>/export/` prefix, especially where the same basename
   exists in both `main` and `main-gb`.
5. **4 `ui::hud` rows (lines 19612–19615)** — cite `main-gb` like the rest of
   the module, or document why `main` is authoritative.
6. **8 broken `fn_<address>` doc links** (`80afe3c0`, `80094660`, `8019f660`,
   `8019f700`, `8019fa00`, `8019fe50`, `801cf8f0`, `801d33f0`) — these refer to
   semantic-named bodies that have no item with the linked name; retarget the
   links to `fn <name>` or add the item.

The ledger itself is internally consistent: the only structural gaps are the
evidence-path representation (54 rows), one body-anchor soft miss
(`801ae0a0`), and eight unresolved `fn_<address>` doc links; there are no
dangling evidence paths, missing Rust files, malformed addresses, duplicates,
or module/file mismatches.
