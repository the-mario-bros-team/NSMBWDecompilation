# Wave 33 lane brief

Every Wave 33 lane reads this file first and follows it exactly.

Repository root: `C:\Users\WitherULTRA7\wii-decomp\nsmbw`

## Non-negotiable rules

1. **You own exactly one Rust file and two new deliverable files.** Do not edit
   anything else. In particular never touch:
   - `src/lib.rs`, any `mod.rs`, `Cargo.toml`, `src/platform.rs`
   - `ledger/functions.csv` (generated; the coordinator merges it)
   - `docs/FUNCTION_INDEX.md`, `docs/COVERAGE.md`, `docs/PROGRESS.md`,
     `docs/GAP_REPORT.md`, `ledger/SUMMARY.md` (generated)
   - another lane's `.rs` file, CSV or coverage doc
2. **Never invent a translation.** Every ledger row must correspond to a real
   `pub unsafe fn fn_<addr>` body you wrote in your file. If a function cannot
   be reconstructed honestly, skip it and say so in the coverage doc.
3. **Do not create a row for an address that is already in
   `ledger/functions.csv`.** Check before you write.
4. **Do not run `.\scripts\check.ps1`.** The coordinator runs the full gate.
   You only need `cargo check --message-format=short` to be free of errors.
5. **No giant shell here-strings.** Use the `write` / `edit` tools for file
   content. Shell is for searching and for `cargo check`.
6. Aim for **25-30 functions**. Quality beats quantity. A short, correct lane
   is worth more than a padded one.

## Research workflow

1. Read `docs/RUST_STYLE.md` and the module header of your destination file.
2. Read the most recent `docs/COVERAGE_WAVE*.md` for your module to see what is
   already done and which clusters remain.
3. Use `ledger/functions.csv` (read-only) to learn which addresses in your
   range are already translated.
4. Read the Ghidra C export for each candidate before writing anything.
5. Prefer contiguous runs and call-graph clusters: a constructor plus its
   state handlers share field offsets and are much easier to justify.

## Body format

Match the surrounding file exactly. Every translated function gets:

```rust
/// One-line summary in plain English.
///
/// # Original
/// - Ghidra: `FUN_80210100` at `0x80210100` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80210100_FUN_80210100.c`
///
/// # Arguments
/// - `this`: ...
///
/// # Returns
/// ...
///
/// # Side effects
/// ...
///
/// # Safety
/// ...
///
/// # Confidence
/// high - the guard, two stores and the call are literal in the export.
pub unsafe fn fn_80210100(this: usize) -> u32 {
    // ...
}
```

- Function names are `fn_` + the lowercase hex address, always.
- The `# Original` block must be literal and verifiable: real Ghidra name,
  real address, real byte count from `functions.tsv`, real evidence path.
- REL lanes add one line directly after `# Original`'s header:
  `/// - Synthetic base: `0x80A00000`; this is not a runtime address.`
- `# Confidence` is `high`, `medium` or `low` followed by ` - ` and a concrete
  reason. Use `medium` when a field role or callee identity is inferred, `low`
  when only the instruction sequence is certain.
- Field roles you cannot prove stay as `// unknown:` or `// inferred:` comments.
  Never present a guess as fact.

## Verifying a candidate

- `work/ghidra/main-gb/export/functions.tsv` lists main.dol functions as
  `address<TAB>name<TAB>...<TAB>size` (tab separated, one header line).
  REL exports have their own `functions.tsv` under
  `work/ghidra/rel-<module>/export/`.
- Addresses in `functions.tsv` are 1-based hex **without** a `0x` prefix when
  the column has no prefix; never treat a column index as an address.
- The evidence file is usually `<addr>_FUN_<addr>.c`, but REL exports can name
  it `<addr>_<symbol>.c` (for example `80a000f0_entry.c`). Glob the address
  prefix to find the real file, and put that exact relative path in the row.
- Byte sizes in your `# Original` line must equal the `functions.tsv` size.

## Deliverable 1: `ledger/agents/wave33-<lane>.csv`

Header, byte for byte:

```
module,address,name,status,confidence,evidence,rust_file
```

- `module`: the Rust module path, e.g. `game::enemies`.
- `address`: lowercase hex, **no** `0x` prefix, e.g. `80a12340`.
- `name`: `fn_80a12340`.
- `status`: `translated` (or `partial` only if you genuinely left part out).
- `confidence`: `high`, `medium` or `low`, matching the body doc block.
- `evidence`: the real relative evidence path first, then optional
  `; ` short note. **Quote the field** whenever it contains a comma or quote.
- `rust_file`: the file you wrote the body into, e.g. `src/game/enemies.rs`.

Quote any field containing `,` or `"`. Do not add blank lines.

## Deliverable 2: `docs/COVERAGE_WAVE33_<LANE>.md`

Use this shape:

```markdown
# Coverage - Wave 33 (<lane title>)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/...` | `...` | N | N | N | N |

## What this pass covers

- cluster 1 ...
- cluster 2 ...

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A12340` | `fn_80a12340` | 96 | high |

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `<file>`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `<file>`.
```

## Module header upkeep

Most modules start with a `//!` block containing a numbered list of clusters.
Append a new numbered item for your pass and correct any count you made stale
(for example "eight constructors" becoming "nine constructors"). A stale header
count is a review failure.

## Definition of done

- `cargo check --message-format=short` produces no errors.
- Your CSV parses as exactly 7 columns per row and every `address` is unique.
- Your coverage doc exists and its table matches the CSV.
- You report: number of bodies, the address range you covered, and anything you
  deliberately skipped because it could not be reconstructed honestly.
