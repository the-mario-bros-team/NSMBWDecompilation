# Audit - Wave 226 (untabled code, tool repairs, structural gap)

This wave was a repair wave. Every `functions.tsv` function was already
translated (20,250 / 20,252; the two remaining are documented `game::bases`
deferrals), so the lanes worked on code and tooling **outside** that inventory.

## 1. Untabled bodies translated (6)

| Address | Bytes | File | What |
|---|---:|---|---|
| `0x800FC2D0` | 156 | `src/engine/dsys.rs` | sector code (5/4/2/3) to unit axis vector |
| `0x800FC370` | 16 | `src/engine/dsys.rs` | "current shell is 8" (`cntlzw >> 5`) |
| `0x802C3460` | 28 | `src/resource/archive.rs` | vtable `+0x48` wrapper: bit-0 guard, tail-call `+0x40` (`0x802C3980`) |
| `0x802C3490` | 28 | `src/resource/archive.rs` | vtable `+0x4C` wrapper: tail-call `+0x44` (`0x802C3CB0`) |
| `0x802C3510` | 8 | `src/resource/archive.rs` | adjustor: `this - 8` -> `fn_802c34b0` |
| `0x802C3520` | 8 | `src/resource/archive.rs` | adjustor: `this - 0x14` -> `fn_802c34b0` |

They are deliberately **not** in `ledger/functions.csv` (no `functions.tsv`
row exists), so the canonical 20,252 denominator stays honest; their lane CSVs
are archived under `work/wave226/untabled-ledger/`.

Findings: neither dsys body is referenced anywhere in main.dol (all executable
segments scanned for branches, pointers and `lis`/`addi`/`ori` halves), so they
are unreferenced main-DOL code (REL-side helpers or dead code) and their roles
are marked `// inferred:`. The archive wrappers are virtuals `+0x48`/`+0x4C` of
the class based at `0x80350230` (`VTABLE_80350230_F`), and they forward to
`+0x40`/`+0x44`, so the bit-0 guard is **not** self-recursive.

## 2. The structural gap: 2,457 functions outside `functions.tsv`

Lane C walked every branch in `.text` and found **0 untranslated entry points
reachable by direct branch or call** - the nine blocks that survive filtering
are Ghidra-folded tail/return blocks already present in their callers' bodies.

But outside every known body there are **829,484 bytes (27.2% of `.text`)**
holding **2,457 prologue-verified function starts**. None has a `bl` edge; they
are entered only through function-pointer tables (e.g. `0x80154F60` from the
word at `0x80327268`). So the Ghidra inventory itself is incomplete: the true
function count is at least 20,252 + 2,457.

Inventory: `docs/UNTABLED_MAIN_DOL.md`, with machine-readable evidence in
`work/wave226/laneC_untabled_entries.csv` (2,457 rows) and
`laneC_regions.json`, produced by six `laneC_*.mjs` scanners.

Also noted: 221 tsv sizes are **truncated** (e.g. `0x802C2700` recorded as
576 B while its jump-table arms and shared tail run to 2,048 B).

## 3. `tools/check-lane-csv.mjs` - the size cross-check is now live

It read the tsv size from column `len - 2`, which is the `decompiled` column of
a 5-column tsv, so the check never fired. Four corrections were needed:

1. resolve the `size` column by header name (fallback to the old index);
2. tolerate the backtick form ``` `0xADDR` (N bytes) ``` that every modern
   `# Original` line uses;
3. anchor the search to the row's **own** body, not the first address mention
   in the file (a neighbour's comment could carry a different extent);
4. skip the synthetic `smoke` fixture, whose 1-byte row shadowed the real
   364-byte `boot` row.

Proof: a deliberately corrupted `(92 bytes)` -> `(9999 bytes)` is now reported
as `byte size 9999 in doc, 92 in functions.tsv`; restoring it clears it.

**Triage needed:** 46 rows across 28 historical waves now fail the live check
(194 waves are size-clean). They are listed in lane D's report and are the
subject of round 227.

## 4. Disassembler - `fcmpu`/`fcmpo` decoded properly

The script had **no** case for either: both are opcode 63 with the XO at bits
21-30 (`fcmpu` 0, `fcmpo` 32), so they fell through to the arithmetic fallback,
which reads bits 6-10 (`fd`) where `crfD` lives and printed a phantom `f<crf>`
as the first operand. The fix adds an explicit case
(`fcmpo cr0,f1,f0`, FRA = bits 11-15, FRB = bits 16-20).

`FRA`/`FRB` order was already correct - the brief's suspicion was wrong - and
**no committed body flips polarity** under the corrected output:
`fn_80033f10`, `fn_8003acd0` and `fn_800a0550` all match
`1 iff A < B (or unordered)`.

Open items: `.text` has 3,984 op-63 compares whose committed docs sometimes lost
the mnemonic (`fcmpu` vs `fcmpo`), and `cror` / `lfsx` / `lfdx` / `stfsx` remain
undecoded in the script.

## 5. Stale source repaired

- `fn_8006c5c0` / `fn_8006c610` in `src/game/player.rs` were declared `void`,
  but both epilogues restore only `r1`/`r31`, so the trailing predicate's `r3`
  **is** the return value. They now return `u32`, and
  `fn_8006c8f0`/`fn_8006c9f0` call them instead of re-expressing the sequence
  inline (the image itself branches there: `0x8006C8FC: bl 0x8006C5C0`,
  `0x8006CA04: bl 0x8006C610`).
- `W224_F_ZERO_6C44_A224` was retired: its address was right but the name and
  doc claimed `0.0f` when the image word is `0x45ABE000` (`5500.0f`). It was a
  duplicate of the correctly-named `W224B_F_5500`, now the only name for the
  slot. Reader audit: no site compares `+0xAEC`/`+0xAE0` to zero, so this was
  naming debt, not a behaviour bug.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- Untabled bodies compile; untabled lane CSVs excluded from the ledger by
  design.
- Ledger unchanged at 20,250 rows (99.99%), 0 duplicates.
- `scripts/check.ps1` exit 0.
