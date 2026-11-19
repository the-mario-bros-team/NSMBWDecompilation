# Wave 159 lane brief

Every Wave 159 lane reads this file first and follows it exactly. Wave 159 runs
**four** subagents: one translation lane (`game::bases`) plus three read-only
lanes (a bases specification tranche, a class-P pinning pass and a
clone-mapping pass). `game::enemies`, `game::bosses`, `game::player` and
`game::stage` are closed; read-only lanes must not edit Rust or ledger files.

The translation lane works from `docs/ANALYSIS_WAVE158_BASES.md`, which ranks
dense groups of small complete bodies. Claim those before any single large
body: one reviewer pass covers a whole sibling set. `docs/SPEC_WAVE158_BASES.md`
(the 516-540 byte band) stays queued for a later wave; do not spend this lane
on it unless every ranked cluster is exhausted.

Before decoding any raw REL word, read `docs/REL_ELF_MAPPING.md` — the offset is
**not** simply `address - base`.

Repository root: `C:\Users\WitherULTRA7\wii-decomp\nsmbw`

## Lane roster

| Lane | Type | Writes | Task |
|---|---|---|---|
| `bases` | translation | `src/game/bases.rs`, `ledger/agents/wave159-bases.csv`, `docs/COVERAGE_WAVE159_BASES.md` | claim the ranked small-body clusters in `ANALYSIS_WAVE158_BASES.md` |
| `spec` | read-only | `docs/SPEC_WAVE159_BASES.md` | specify the next 300-512 byte tranche with exact decodes |
| `pin` | read-only | `docs/ANALYSIS_WAVE159_BASES_P.md` | resolve all 34 class-P rows (GQR3 word, extent fixes) |
| `clones` | read-only | `docs/ANALYSIS_WAVE159_BASES_CLONES.md` | map all 97 class-S clones to their existing bodies |

## Non-negotiable rules

1. **You own exactly the files listed for your lane.** Do not edit anything
   else. In particular never touch:
   - `src/lib.rs`, any `mod.rs`, `Cargo.toml`, `src/platform.rs`
   - `ledger/functions.csv` (generated; the coordinator merges it)
   - `docs/FUNCTION_INDEX.md`, `docs/COVERAGE.md`, `docs/PROGRESS.md`,
     `docs/GAP_REPORT.md`, `ledger/SUMMARY.md` (generated)
   - another lane's `.rs` file, CSV or coverage doc
2. **Never invent a translation.** Every ledger row must correspond to a real
   `pub unsafe fn` body you wrote in your file. If a function cannot be
   reconstructed honestly, skip it and say so in the coverage doc.
3. **Do not create a row for an address that is already in
   `ledger/functions.csv`.** Check before you write.
4. **Do not run `.\scripts\check.ps1`.** The coordinator runs the full gate.
   Translation lanes only need `cargo check --message-format=short` to be free
   of errors.
5. **No giant shell here-strings.** Use the `write` / `edit` tools for file
   content. Shell is for searching and for `cargo check`.
6. Translation lane aim: **30 or more small bodies**, or every ranked cluster
   you can verify honestly. Quality beats quantity; a short, correct lane is
   worth more than a padded one.

## CSV rules that earlier waves got wrong

1. **The `evidence` field must be a full repo-relative path**, starting with
   `work/ghidra/`:
   ```
   work/ghidra/rel-d_basesNP/export/807c39d0_FUN_807c39d0.c
   ```
   Never write just `Ghidra 807c39d0_FUN_807c39d0.c`. The path is verified to
   exist, so a bare filename fails validation.
2. **There are exactly 7 comma-separated fields.** The last one is
   `rust_file`. Separate it with a **comma**, never a semicolon:
   ```
   game::bases,807c39d0,fn_807c39d0,translated,high,work/ghidra/rel-d_basesNP/export/807c39d0_FUN_807c39d0.c,src/game/bases.rs
   ```
3. **Quote any field containing a comma, quote or newline.** The `evidence`
   field normally holds a path plus a note, so it is almost always quoted:
   ```
   game::bases,807c39d0,fn_807c39d0,translated,high,"work/ghidra/rel-d_basesNP/export/807c39d0_FUN_807c39d0.c; flag test +0x502, cleared by +0x4F0",src/game/bases.rs
   ```
4. Verify with `node tools/check-lane-csv.mjs wave159` before you report. It
   checks field count, quoting, path existence, body presence, duplicate
   addresses and byte sizes. It prints `OK` when your lane is clean.

## Research workflow

1. Read `docs/RUST_STYLE.md` and the module header of `src/game/bases.rs`.
2. Read `docs/ANALYSIS_WAVE158_BASES.md` sections **Ranked next-claim list**,
   **Quick singles**, **Class P**, **Class S** and the per-address class R
   table.
3. Use `ledger/functions.csv` (read-only) to learn which addresses are already
   translated.
4. Read the Ghidra C export for each candidate **and** the raw ELF words. The
   analysis notes which exports are truncated (`halt_baddata` is usually a tail
   call, not a truncation).
5. Prefer contiguous runs and sibling sets: a state-flag machine or an
   accessor pair shares field offsets and needs one justification.

## Body format

Match the surrounding file exactly. Every translated function gets:

```rust
/// One-line summary in plain English.
///
/// # Original
/// - Ghidra: `FUN_807C39D0` at `0x807C39D0` (52 bytes)
/// - Evidence: `work/ghidra/rel-d_basesNP/export/807c39d0_FUN_807c39d0.c`
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
pub unsafe fn fn_807c39d0(this: usize) -> u32 {
    // ...
}
```

- Function names are `fn_` + the lowercase hex address, always.
- The `# Original` block must be literal and verifiable: real Ghidra name,
  real address, real byte count from `functions.tsv`, real evidence path.
- REL lanes add one line directly after `# Original`'s header:
  `/// - Synthetic base: `0x80600000`; this is not a runtime address.`
  (`game::bases` uses `0x80600000`; `game::enemies` used `0x80A00000`.)
- `# Confidence` is `high`, `medium` or `low` followed by ` - ` and a concrete
  reason. Use `medium` when a field role or callee identity is inferred, `low`
  when only the instruction sequence is certain.
- Field roles you cannot prove stay as `// unknown:` or `// inferred:` comments.
  Never present a guess as fact.

## Verifying a candidate

- `work/ghidra/rel-d_basesNP/export/functions.tsv` lists the REL functions as
  `address<TAB>name<TAB>size<TAB>decompiled<TAB>file`.
- The evidence file is usually `<addr>_FUN_<addr>.c`, but the export can name it
  `<addr>_<symbol>.c`. Glob the address prefix to find the real file, and put
  that exact relative path in the row.
- Byte sizes in your `# Original` line must equal the `functions.tsv` size.
  Two lanes in wave 33 inherited stale counts from earlier coverage docs; always
  re-read the size from `functions.tsv`, never copy it from prose.
- `file_offset = (address - 0x80600000) + 0x1000` for `d_basesNP`; verify the
  first word of the body against the export before trusting a decoder.

## Deliverable 1 (translation lane): `ledger/agents/wave159-bases.csv`

Header, byte for byte:

```
module,address,name,status,confidence,evidence,rust_file
```

- `module`: `game::bases`.
- `address`: lowercase hex, **no** `0x` prefix, e.g. `807c39d0`.
- `name`: `fn_807c39d0`.
- `status`: `translated` (or `partial` only if you genuinely left part out).
- `confidence`: `high`, `medium` or `low`, matching the body doc block.
- `evidence`: full repo-relative path first, then optional `; ` short note.
- `rust_file`: `src/game/bases.rs`.

## Deliverable 2 (translation lane): `docs/COVERAGE_WAVE159_BASES.md`

Use this shape:

```markdown
# Coverage - Wave 159 (game::bases small-body clusters)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | N | N | N | N |

## What this pass covers

- cluster 1 ...
- cluster 2 ...

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807C39D0` | `fn_807c39d0` | 52 | high |

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/bases.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/game/bases.rs`.
```

## Read-only lane outputs

- **spec**: `docs/SPEC_WAVE159_BASES.md` — the next tranche between 300 and 512
  bytes, in descending size order, excluding any address already translated or
  claimed by the translation lane. For each body: TSV size, word count, exact
  export path, raw reachable `blr`, gap contents, field map, callee list and a
  ready/conditional verdict. Follow the shape of `docs/SPEC_WAVE158_BASES.md`.
- **pin**: `docs/ANALYSIS_WAVE159_BASES_P.md` — all 34 class-P rows. Prove the
  GQR3 word (`0x00050005`) or the corrected extent from the raw stream, list
  every `psq` with its GQR index, and give a concrete write note per address.
- **clones**: `docs/ANALYSIS_WAVE159_BASES_CLONES.md` — all 97 class-S clones.
  For each target: source body address, byte-equality proof (hash of the two
  extents), the exact Rust body to copy and any address-bearing constants that
  must change. Do not write the bodies into `bases.rs`.

## Module header upkeep

`src/game/bases.rs` starts with a `//!` block containing a numbered list of
clusters. The translation lane appends a new numbered item for its pass and
corrects any count it made stale. A stale header count is a review failure.

## Definition of done

- `cargo check --message-format=short` produces no errors (translation lane).
- `node tools/check-lane-csv.mjs wave159` reports no problem naming your CSV
  (translation lane).
- Your deliverable exists and its tables match the CSV or the TSV.
- You report: number of bodies, the address range you covered, and anything you
  deliberately skipped because it could not be reconstructed honestly.
