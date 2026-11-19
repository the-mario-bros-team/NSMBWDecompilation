# Coverage - Wave 101 (runtime::cpp)

## How to read this file

Each row of **Functions** is one `pub unsafe fn fn_<address>` body this pass
added to `src/runtime/cpp.rs`. The `bytes` column is the size from
`work/ghidra/main-gb/export/functions.tsv`, re-read for this pass; the body's
`# Original` block cites the same export file. Confidence matches the CSV row.

The `0x802D0000` bucket held 354 functions. Wave 100 left 32 unclaimed
addresses, all in `0x802DCCCC..0x802DCDC0`: the CodeWarrior callee-save
`__savefpr_N` / `__restfpr_N` / `__savegprlr_N` / `__restgprlr_N` entries. This
pass translates all 32, so the bucket is complete.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/cpp.rs` | `runtime::cpp` | 32 | 232 | 0 | 32 |

## What this pass covers

Address range: `0x802DCCCC..0x802DCDC0` (a 244-byte instruction area; the 32
`functions.tsv` symbols inside it are the alternate entry labels).

- **Four instruction runs, one model.** The raw bytes in `work/dol/main.elf`
  are four contiguous runs: FPR save `0x802DCCCC..0x802DCCF0` (`stfd`), FPR
  restore `0x802DCCF4..0x802DCD3C` (`lfd`), GPR save `0x802DCD40..0x802DCD88`
  (`stw`) and GPR restore `0x802DCD8C..0x802DCDD4` (`lwz`). Each run ends in a
  `blr`; `r11` is the caller's frame base and every slot is at
  `r11-0x48..r11-4`, four bytes apart for GPRs and eight for FPRs.
- **`functions.tsv` sizes are symbol extents, not routine extents.** Ghidra
  makes every alternate `stw`/`lwz` a 4-byte function because it is a valid
  branch target. A `bl` to `0x802DCD40`, for example, stores `r14..r31`
  (18 words) and returns through the `blr` at `0x802DCD88`; only the first
  `stw` lies inside the 4-byte symbol. Each body therefore covers the run from
  its own label to that `blr`, and its doc block names the start offset.
- **The register file is made explicit.** Rust cannot name `r14` or `f23`, and
  Ghidra's C cannot either (`unaff_rN` on the save side, a bare `return;` on the
  restore side). A save body takes `frame` plus the live `rN..r31` values as a
  fixed-size array and writes them; a restore body takes `frame` and returns the
  values it loads. Four private helpers (`save_gpr_run`, `load_gpr_run`,
  `save_fpr_run`, `load_fpr_run`) hold the loops.
- **The FPR entries.** `0x802DCCCC` saves `f23..f31` (the first five `stfd`;
  the run continues at `0x802DCCE0`), `0x802DCCE0` saves `f28..f31` plus the
  `blr`. `0x802DCD18` and `0x802DCD2C` are the matching `lfd` halves.
- **The GPR entries.** `0x802DCD40..0x802DCD70` are the thirteen 4-byte
  `__savegprlr_14..26` labels, `0x802DCD74` is the 24-byte
  `__savegprlr_27` (five `stw` plus `blr`); `0x802DCD8C..0x802DCDBC` and
  `0x802DCDC0` are the matching `__restgprlr` labels.

### Why this is a translation and not a re-listing

Waves 77-100 deferred this family because "the only effect is on registers".
That is true, but the effect is still exactly determined by the bytes: an
in-order store or load of a known register window to a known frame offset.
Leaving the addresses unclaimed would have left the bucket at 90% with 32 holes
whose content is fully known. The bodies are those stores and loads, with the
register arguments made explicit; nothing about the runtime behaviour is
guessed. What *is* a modelling decision is the signature, which is why every row
is `medium` rather than `high`.

### Not claimed

Two alternate entries in the same area have **no `functions.tsv` symbol** and
therefore no ledger row and no body: `0x802DCCA8` (`stfd f14..f22`) and
`0x802DCCF4` (`lfd f14..f22`). They are reachable prefixes of the runs claimed
above, not separate inventory entries. The body at `0x802DCCCC` starts at `f23`
and the body at `0x802DCD18` starts at `f23`, exactly as the symbols say.

## Functions

| address | function | bytes | role | confidence |
|---|---|---:|---|---|
| `0x802DCCCC` | `fn_802dcccc` | 20 | `__savefpr_23`: save `f23..f31` | medium |
| `0x802DCCE0` | `fn_802dcce0` | 20 | `__savefpr_28`: save `f28..f31`, `blr` | medium |
| `0x802DCD18` | `fn_802dcd18` | 20 | `__restfpr_23`: restore `f23..f31` | medium |
| `0x802DCD2C` | `fn_802dcd2c` | 20 | `__restfpr_28`: restore `f28..f31`, `blr` | medium |
| `0x802DCD40` | `fn_802dcd40` | 4 | `__savegprlr_14`: save `r14..r31` | medium |
| `0x802DCD44` | `fn_802dcd44` | 4 | `__savegprlr_15`: save `r15..r31` | medium |
| `0x802DCD48` | `fn_802dcd48` | 4 | `__savegprlr_16`: save `r16..r31` | medium |
| `0x802DCD4C` | `fn_802dcd4c` | 4 | `__savegprlr_17`: save `r17..r31` | medium |
| `0x802DCD50` | `fn_802dcd50` | 4 | `__savegprlr_18`: save `r18..r31` | medium |
| `0x802DCD54` | `fn_802dcd54` | 4 | `__savegprlr_19`: save `r19..r31` | medium |
| `0x802DCD58` | `fn_802dcd58` | 4 | `__savegprlr_20`: save `r20..r31` | medium |
| `0x802DCD5C` | `fn_802dcd5c` | 4 | `__savegprlr_21`: save `r21..r31` | medium |
| `0x802DCD60` | `fn_802dcd60` | 4 | `__savegprlr_22`: save `r22..r31` | medium |
| `0x802DCD64` | `fn_802dcd64` | 4 | `__savegprlr_23`: save `r23..r31` | medium |
| `0x802DCD68` | `fn_802dcd68` | 4 | `__savegprlr_24`: save `r24..r31` | medium |
| `0x802DCD6C` | `fn_802dcd6c` | 4 | `__savegprlr_25`: save `r25..r31` | medium |
| `0x802DCD70` | `fn_802dcd70` | 4 | `__savegprlr_26`: save `r26..r31` | medium |
| `0x802DCD74` | `fn_802dcd74` | 24 | `__savegprlr_27`: save `r27..r31`, `blr` | medium |
| `0x802DCD8C` | `fn_802dcd8c` | 4 | `__restgprlr_14`: restore `r14..r31` | medium |
| `0x802DCD90` | `fn_802dcd90` | 4 | `__restgprlr_15`: restore `r15..r31` | medium |
| `0x802DCD94` | `fn_802dcd94` | 4 | `__restgprlr_16`: restore `r16..r31` | medium |
| `0x802DCD98` | `fn_802dcd98` | 4 | `__restgprlr_17`: restore `r17..r31` | medium |
| `0x802DCD9C` | `fn_802dcd9c` | 4 | `__restgprlr_18`: restore `r18..r31` | medium |
| `0x802DCDA0` | `fn_802dcda0` | 4 | `__restgprlr_19`: restore `r19..r31` | medium |
| `0x802DCDA4` | `fn_802dcda4` | 4 | `__restgprlr_20`: restore `r20..r31` | medium |
| `0x802DCDA8` | `fn_802dcda8` | 4 | `__restgprlr_21`: restore `r21..r31` | medium |
| `0x802DCDAC` | `fn_802dcdac` | 4 | `__restgprlr_22`: restore `r22..r31` | medium |
| `0x802DCDB0` | `fn_802dcdb0` | 4 | `__restgprlr_23`: restore `r23..r31` | medium |
| `0x802DCDB4` | `fn_802dcdb4` | 4 | `__restgprlr_24`: restore `r24..r31` | medium |
| `0x802DCDB8` | `fn_802dcdb8` | 4 | `__restgprlr_25`: restore `r25..r31` | medium |
| `0x802DCDBC` | `fn_802dcdbc` | 4 | `__restgprlr_26`: restore `r26..r31` | medium |
| `0x802DCDC0` | `fn_802dcdc0` | 24 | `__restgprlr_27`: restore `r27..r31`, `blr` | medium |

## Deliberate skips

None among the 354 bucket addresses: the 32 remaining ones are all translated
above. The only addresses in the area without a row are the two unsymbolised
prefix labels `0x802DCCA8` and `0x802DCCF4`, explained under *Not claimed*.

## Verification

- Every address and byte size was re-read from `functions.tsv` for this pass;
  the total is 232 bytes (four 20-byte FPR entries, twenty-six 4-byte GPR
  entries, two 24-byte tails).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every one of the 32 rows has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/cpp.rs`.
- No address was already in `ledger/functions.csv`: the 32 were the exact
  remainder of the bucket (354 minus 322 already merged).
- `cargo check --message-format=short` reports no error and no new warning in
  `src/runtime/cpp.rs`; the file's only warnings are the two pre-existing
  `unused_assignments` at lines 8047 and 8903, outside this pass's section. At
  the time of writing the workspace build is still red from other Wave 101
  lanes (`game::worldmap`, `game::enemies`, `runtime::os`, `ui::hud`), so the
  new section was additionally type-checked on its own in a scratch crate.
- `node tools/check-lane-csv.mjs wave101` reports `wave101-cpp.csv: 32 rows`
  and `OK` for this lane.
- The module header gained a Wave 101 pass note and the three earlier notes that
  said this family "stays untranslated" were corrected; the
  `0x802D0000` bucket is now recorded as closed.
