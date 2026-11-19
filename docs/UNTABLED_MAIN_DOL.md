# Untabled entry points in `main.dol` `.text`

Wave 226, lane C. Audit only: nothing here is translated and no source file was
edited. All numbers are reproducible from the scripts listed in
[Reproduction](#reproduction).

- Binary: `work/dol/main.elf` (3,493,216 bytes, 13 program headers)
- Executable segments: `0x80004000..0x800066C0` (0x26C0) and
  `0x80006780..0x802ED9E0` (0x2E7260) -> **3,053,856 B of `.text`, 763,464 words**
- Inventory: `work/ghidra/main-gb/export/functions.tsv` (10,276 rows, all
  `decompiled=true`) and `ledger/functions.csv` (20,250 rows)
- Ledger rows whose address is inside the two executable segments: **10,276**
  - they are exactly the tsv set; the other 9,974 ledger rows are REL analysis
    addresses starting at `0x805000E0` (e.g. `game::bases`, `game::enemies`).
  - therefore, for `main.dol`, "absent from the tsv" and "absent from the
    ledger" are the same condition. Nothing is in the ledger but missing from
    the tsv.

## Verdict

| Question | Count |
|---|---:|
| Distinct in-range branch/call targets (all `b`/`bl`/`bc`/`bcl` in `.text`) | 56,291 |
| ... that are a tsv/ledger entry | 8,441 |
| ... that are not a tsv/ledger entry | 47,850 |
| ... that fall inside a translated body (Ghidra-folded tail block) | 38,228 |
| ... outside every translated body | 9,622 |
| ... actually reached by a control-flow walk from translated entries | **42** |
| ... entry-shaped (preceded by `blr`, unconditional `b`, or zero padding) | 19 |
| ... distinct blocks after merging intra-body targets | **9** |
| ... **genuinely untranslated** (not present in an adjacent translated body) | **0** |

**Headline: 0 untabled entry points are reached by direct branch/call from the
translated corpus.** The nine blocks that survive every filter are Ghidra-folded
tail blocks whose code already sits inside the decompiled body (and therefore
the Rust body) of the function that jumps to them. Section
[Table A](#table-a---the-nine-reached-blocks-all-already-covered) proves each one
against its `.c` export.

**Second headline: the inventory is not complete at the byte level.** 829,484 B
(27.2% of `.text`) lies outside every translated body and contains **2,457**
prologue-verified function starts. None of them is reachable by a direct branch
from translated code; they are entered through function-pointer words in data
(e.g. `0x80154F60` <- word at `0x80327268`). They are the real gap in the
inventory, but they are outside this lane's "reached by branch or call" scope.
See [The 27.2% hole](#the-272-hole-829484-b-2457-untabled-entries).

## Method

1. Parse the ELF program headers (offset 28, entry size 42, count 44), keep
   `PT_LOAD` segments with `PF_X`, and build a VA->word map. Decoding outside
   those two segments is impossible by construction, so nothing in `.data` was
   mistaken for code.
2. Sweep every 4-byte word in both segments and decode the three PC-relative
   branch forms:
   - `opcode 18` (`b`/`bl`/`ba`/`bla`): `LI = sign_extend(w & 0x03FF_FFFC, 26)`
   - `opcode 16` (`bc`/`bcl`): `BD = sign_extend(w & 0x0000_FFFC, 16)`
   - `AA` (bit 1) selects absolute; bit 0 is `LK`, i.e. `bl` vs `b`
   - `bclr`/`bcctr` are indirect and cannot contribute a static target
   Targets must be 4-byte aligned and inside an executable segment.
3. Subtract every tsv address. For the remainder, test containment in the
   translated body `[start, start + size)` from the tsv `size` column. Those are
   the "folded tail block" candidates (`0x800613C0`-style helpers that Ghidra
   merged into the caller).
4. For the residue, do a real reachability walk instead of trusting raw word
   hits: decode from every tsv entry, follow fallthrough, conditional targets
   and calls, stop at `blr`/`bctr`/unconditional `b`, and treat a `b` target as
   a new function when it leaves the current `[start, start+size)`. This removes
   the ~9,500 "targets" that are only data words happening to decode as a
   branch. Every discovered non-tsv destination is then walked in turn.
5. Filter the survivors by entry shape (previous word is `blr`, an unconditional
   `b`, or zero padding), merge blocks that are internal to an earlier block
   (a `bc` loop head inside the same function), and verify each surviving block
   against the Ghidra C export of the function that reaches it.

## Table A - the nine reached blocks (all already covered)

`size` is the CFG extent of the block (branches followed inside the block,
terminator included), so it can be a few bytes larger than the linear span.

| # | Address | Size | How it is reached | Where the code already lives | Verdict |
|---:|---|---:|---|---|---|
| 1 | `0x800613C0` | 92 B | `b` at `0x80061568`, inside `FUN_80061540` | C export arm `param_1 == 0`: loop `FUN_80162D50(0x1e9, ...)` over record id `>> 4 & 0xf` | folded, covered |
| 2 | `0x80061420` | 92 B | `b` at `0x80061570`, inside `FUN_80061540` | same C export, arm `param_1 == 1`: `0x1c0`, `>> 0x18` | folded, covered |
| 3 | `0x80061480` | 88 B | `b` at `0x80061578`, inside `FUN_80061540` | same C export, arm `param_1 == 2`: `0x1a3`, byte at `+0x39C` | folded, covered |
| 4 | `0x800614E0` | 88 B | `b` at `0x80061580`, inside `FUN_80061540` | same C export, arm `param_1 == 3`: `0x1a4`, byte at `+0x3E2` | folded, covered |
| 5 | `0x800D03A0` | 200 B | `b` at `0x800D0504`, inside `FUN_800D0470` | C export tail: `FUN_802B9210(0x24)`, `*puVar2 = 0 ...`, `FUN_802DCDC0(); return;` | folded, covered |
| 6 | `0x8016160C` | 8 B | `b` at `0x801615A4`, inside `FUN_80161580` | C export ends `return 1;` (this is `li r3,1; blr`) | folded, covered |
| 7 | `0x801D6B54` | 8 B | `bc` at `0x801D6B40`/`0x801D6B48`, inside `FUN_801D68C0` (tsv body ends exactly here) | C export contains `return 0;` (this is `li r3,0; blr`) | folded, covered |
| 8 | `0x801D6F10` | 16 B | `bc` at `0x801D6EFC`/`0x801D6F04`, inside `FUN_801D6B60` (tsv body ends at `0x801D6F0C`) | C export label `LAB_801D6EF4`, `iVar6 = 0; return iVar6;` | folded, covered |
| 9 | `0x802E8BA4` | 4 B | `b` at `0x802E8C08`, inside `FUN_802E8BE8` | C export `return param_1 + 6;`; the block is the shared `blr` epilogue | folded, covered |

Notes on the four `FUN_80061540` blocks: they are real standalone lookups in the
raw binary (each has its own `stwu r1,-16(r1)` frame and a
`FUN_80162D50` scan loop), but the tsv body for `FUN_80061540` already contains
all four arms, so the Rust body for `fn_80061540` necessarily covers them. They
are `b`-targets, not `bl`-targets, which is why Ghidra never created separate
functions for them.

### Suppressed (internal, not entry points)

Ten of the 19 entry-shaped hits are loop heads or join points inside one of the
nine blocks above and were merged away:

| Address | Inside | Reason |
|---|---|---|
| `0x800613DC`, `0x800613F0` | `0x800613C0` | `bc`/`b` loop head and scan block |
| `0x8006143C`, `0x80061450` | `0x80061420` | same |
| `0x8006149C`, `0x800614AC` | `0x80061480` | same |
| `0x800614FC`, `0x8006150C` | `0x800614E0` | same |
| `0x800D0410`, `0x800D0420` | `0x800D03A0` | `bc` arms of the inlined dispatch |

The other 23 of the 42 reached targets are data-shaped hits: addresses inside a
block that were decoded out of `.text` immediates, or conditional targets with
no valid prologue/boundary. They were rejected by the entry-shape test.

## The 27.2% hole (829,484 B, 2,457 untabled entries)

This is the finding that matters more than the nine folded tails.

```
.text bytes                  3,053,856
sum of tsv body sizes        2,224,372   (72.8%)
outside every tsv body         829,484   (27.2%)
gap regions                      7,299   (1,293 of them contain code)
stwu r1,-N(r1) starts in gaps    2,457
  ... with a literal 32-bit pointer word somewhere in the image   1,950
  ... with no literal pointer (register/offset-computed)             507
`bl`-targets inside the hole          0
```

Every function in the hole is entered by something other than a direct call:
`b`/`bc` from other hole code, or a pointer word (vtable slot / callback table).
That is also why Ghidra's function creation never saw them: it followed calls,
and the holes contain no internal `bl` edges at all.

Spot-checked entries (all disassemble as normal NSMBW function bodies; each was
verified to start with an `stwu` prologue after padding):

| Address | Size (CFG) | Region | Pointer word at | What it looks like |
|---|---:|---|---|---|
| `0x80154F60` | 6,328 B | `0x80154F5C` +6,340 | `0x80327268` | big display helper, `stwu r1,-0x1C0(r1)`, saves f24-f31 |
| `0x8006A9B0` | 220 B | `0x8006A9A8` +5,944 | `0x8030F110` | 14 functions in this region, all pointer-referenced |
| `0x8000B530` | 1,340 B | `0x8000B52C` +4,244 | none | `stwu r1,-0xA0(r1)`, calls `0x802DCD5C`, `0x801C74C0` |
| `0x800067E0` | 108 B | `0x800067A4` +204 | `0x8034FBC0` | boot/OS helper; same shape as `0x80006930`, `0x80006D90` |

A different failure mode shows up at `0x802C2700`: it *is* a tsv entry, but its
recorded size (576 B, ending `0x802C2940`) is truncated. The body dispatches
through a `bctr` jump table at `0x802C2808`, and the next tsv body does not
start until `0x802C2F00`, so 0x5C0 B of the same function sits in the hole. Its C
export ends in `do { ... } while(true);`, which is the tell. There are 221 tsv
bodies whose last word is not a terminator, and 189 gap regions that begin
immediately after such a body; those are continuation bytes, not new functions.

Largest code regions in the hole (full list in
`work/wave226/laneC_regions.json`, per-entry list in
`work/wave226/laneC_untabled_entries.csv`):

| Region start | Size | Untabled entries | Of which pointer-referenced |
|---|---:|---:|---:|
| `0x800ADCD4` | 10,092 | 21 | 21 |
| `0x80147BC0` | 9,888 | 1 | 1 |
| `0x8029D2FC` | 6,772 | 3 | 0 |
| `0x80186A5C` | 6,628 | 3 | 3 |
| `0x80154F5C` | 6,340 | 1 | 1 |
| `0x8006A9A8` | 5,944 | 14 | 14 |
| `0x80297204` | 5,404 | 4 | 4 |
| `0x800ECADC` | 5,188 | 3 | 0 |
| `0x80121D84` | 5,036 | 3 | 3 |
| `0x800086B8` | 4,888 | 5 | 0 |
| `0x8026E000` | 4,816 | 4 | 3 |
| `0x80029664` | 4,316 | 14 | 14 |
| `0x8000B52C` | 4,244 | 3 | 0 |
| `0x80098F6C` | 4,116 | 6 | 6 |
| `0x802B0DB4` | 3,804 | 1 | 1 |
| `0x80182594` | 3,788 | 2 | 2 |
| `0x80009BC8` | 3,752 | 3 | 0 |
| `0x80038800` | 3,744 | 7 | 7 |
| `0x800A5454` | 3,740 | 11 | 10 |
| `0x802B3AF8` | 3,656 | 1 | 1 |

Independent corroboration that the hole is not data: none of the 2,457 entry
addresses occurs anywhere in `src/` (68 Rust files, 275 distinct `80xxxxxx`
address tokens), and the C export directory contains exactly one `.c` file per
tsv row (10,276), i.e. no orphan decompilations were left behind for these
bodies.

## Recommended actions

1. **Do not translate the nine Table A blocks.** They are already inside the
   bodies of `fn_80061540`, `fn_800d0470`, `fn_80161580`, `fn_801d68c0`,
   `fn_801d6b60` and `fn_802e8be8`. If address/symbol fidelity matters later,
   record them as `folded into <owner>` notes rather than as new ledger rows.
   (This wave forbids ledger edits; that note belongs in a future lane CSV.)
2. **Open a pointer-table audit wave for the 27.2% hole.** Seed the worklist
   from literal function-pointer words in the data segments
   (`0x802FE3A0..0x80351640`, `0x8042B080..0x8042FC40`, `0x80427680..`), then
   sweep the 2,457 entries in
   `work/wave226/laneC_untabled_entries.csv`. Treat that CSV as a candidate
   list, not a verdict: 1,950 have a literal pointer and can be triaged
   immediately; 507 need a `lis`/`addi` or register-offset chase.
3. **Qualify the completeness claim.** `docs/COVERAGE.md` reports
   "20,250 / 20,250" for a ledger whose `main.dol` share is only 72.8% of
   `.text`. Add a byte-level coverage line (`2,224,372 / 3,053,856 B, 72.8%`)
   so the next wave does not start from a false premise.
4. **Repair truncated tsv sizes.** 221 functions end on a non-terminator; 189
   of them are immediately followed by a continuation region. `FUN_802C2700`
   is a concrete example: 576 B recorded, but the next tsv body does not start
   until `0x802C2F00` (0x5C0 B of continuation). Re-export those 221 bodies
   from Ghidra before trusting any future size-based subtraction.
5. **Extend `tools/gap-report.mjs`.** It only compares tsv addresses against
   ledger addresses, so it can never see the hole. Add a prologue sweep
   (`stwu r1` preceded by padding) over the executable segments and fail when
   uncovered prologues exist.

## Limitations

- "Genuinely untranslated" is decided against the Ghidra C export and the tsv
  body, not against the Rust source. For the nine blocks this is safe: the C
  body that reaches them contains the corresponding statements. It would not
  catch a Rust body that dropped a folded arm that Ghidra did decompile.
- The 2,457 figure counts `stwu r1,-N(r1)` starts preceded by zero padding or a
  NOP. Leaf functions without a stack frame that follow a `blr` without padding
  are not counted, so the true number of untabled entries is a bit higher.
- Pointer detection is a literal 32-bit word match over all `PT_LOAD`
  segments. Addresses built with `lis`/`addi` pairs, SDA offsets (`r13`/`r2`)
  or `bctr` jump tables are missed; those entries are in the "507" bucket.
- REL modules (`0x80500000`+) are out of scope. Their 9,974 ledger rows were
  only used to prove that the `main.dol` ledger slice equals the tsv slice.

## Reproduction

Scripts (new, all under `work/wave226/`, none of them touches source or the
ledger):

| Script | Purpose |
|---|---|
| `laneC_stats.mjs` | ELF/tsv/ledger accounting and the headline counters |
| `laneC_scan.mjs` | naive all-word branch-target set (`laneC_scan.json`) |
| `laneC_closure2.mjs` | bounded reachability walk from every tsv entry (`laneC_reach2.json`) |
| `laneC_final.mjs` | entry-shape filter + CFG merge (`laneC_final.json`) |
| `laneC_chain.mjs` | reachability chain: which tsv body reaches each target |
| `laneC_corpuslist.mjs` | hole census -> `laneC_untabled_entries.csv`, `laneC_regions.json` |

```powershell
node work/wave226/laneC_stats.mjs
node work/wave226/laneC_scan.mjs
node work/wave226/laneC_closure2.mjs
node work/wave226/laneC_final.mjs
node work/wave226/laneC_chain.mjs
node work/wave226/laneC_corpuslist.mjs
```

Disassembly used the standard helper, e.g.

```powershell
$env:LEN='60'; $env:STOP='0'; node "$env:TEMP\opencode\ppc159.js" work/dol/main.elf 800613c0
```
