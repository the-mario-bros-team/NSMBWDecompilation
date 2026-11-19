# Coverage - Wave 253 lane A (`main` vs `main-gb` function sizes)

Resolution of the `0x801406F0` size disagreement plus a full scan of the
`work/ghidra/main` / `work/ghidra/main-gb` inventories. Analysis only apart
from one source doc correction; no body was added or rewritten.

## Verdict for `0x801406F0`

**828 B.** The image is unambiguous:

```
0x801406F0  9421FFC0  stwu r1,-0x40(r1)      <- prologue
...
0x80140A14  80010044  lwz r0,0x44(r1)        <- state-4 tail converges here
0x80140A20  7C0803A6  mtlr r0
0x80140A24  38210040  addi r1,r1,0x40
0x80140A28  4E800020  bclr 20,0               <- last instruction
0x80140A2C  00000000  (one zero word)
0x80140A30  9421FFF0  stwu r1,-0x10(r1)      <- separate untabled body
```

`0x80140A28 + 4 - 0x801406F0 = 0x33C = 828`. There is no symbol boundary
inside the four-state ladder, and the following 192-byte body (`0x80140A30`,
already translated as `fn_80140a30`) starts after a single zero word, exactly
as `src/game/worldmap.rs` documents.

| source | value | correct? |
|---|---:|---|
| `work/ghidra/main-gb/export/functions.tsv` | 828 | **yes** |
| recursive descent on `work/dol/main.elf` | 828 | **yes** |
| `work/ghidra/main/export/functions.tsv` | 612 | no - truncated |

Why the `main` row is 612 B: it ends at `0x80140954`, in the middle of the
state-1 ladder (`li r3,2` before the state-2 store). The stock PowerPC language
used by that project cannot decode the `psq_l`/GQR angle reads at
`0x80140758`, `0x80140780`, `0x801407A8` and `0x801407D0`, so its body extent
stops early. This is the same defect class `docs/TOOLCHAIN.md` records as
`549/9,518` `halt_baddata` functions for the stock pass.

### Doc fix

`src/game/worldmap.rs`:

- `fn_801406e0`'s sibling note said `0x801406F0` was `612 B`. It now says
  `828 B` and points at the size note.
- `fn_801406f0`'s `# Original` block gained an **Extent** bullet: prologue,
  closing `bclr` at `0x80140A28`, the one-word gap, the untabled `0x80140A30`
  body, and an explicit warning that the `main` `612 B` row is a truncated
  artefact.

`docs/COVERAGE_WAVE252_AUDIT.md`'s open finding now carries the resolution and
a pointer to this page. The `0x801406F0` `# Original` doc block already said
`828` and cited `main-gb`, so `tools/check-lane-csv.mjs` never flagged it. The
wave-107 agent row (`ledger/agents/wave107-worldmap.csv`,
`game::worldmap,801406f0,...`) has no size column and already cites the
`main-gb` export, so the CSV itself needed no change.

## Scan of the two inventories

| | rows |
|---|---:|
| `main` | 9,518 |
| `main-gb` | 10,276 |
| addresses only in `main-gb` | 762 |
| addresses only in `main` | 0 |
| **shared addresses with different sizes** | **549** |

549 is exactly the stock pass's documented `halt_baddata` count, and every
shared disagreement is a body that `main-gb` either extends or splits. The
complete 549-row table (plus the 762 missing rows) with both tsv sizes, both
next-start distances, the descent extent, the gap and the zero padding is in
`work/wave253/laneA_scan.txt`; a ledger cross-check is in
`work/wave253/laneA_ledger_hits.json`.

### Agreement of recursive descent

| result | rows |
|---|---:|
| descent == `main-gb` | 533 |
| descent == `main` | **0** |
| descent != either | 16 |

**No address is measured as `main`'s value.** The 16 exceptions split into
three explainable groups:

| address | main | main-gb | descent | explanation |
|---|---:|---:|---:|---|
| `0x800757B0` | 60 | 40 | 1256 | `main` over-reads into the sibling stub at `0x800757E0`; `main-gb`'s 40 B is the real stub (`b 0x80075820` at `0x800757D4`). Descent follows that `b` into the shared body. |
| `0x80241690` | 416 | 1124 | 128 | Discontiguous body: an early unconditional branch leaves the first block, so descent stops. `main-gb`'s 1124 B is right but overlaps its own next row `0x80241710` (404 B) - a Ghidra body overlap, not an extent. |
| `0x802C2700` | 32 | 576 | 2048 | Both tsv rows are short. The body runs uninterrupted to `bclr 20,0` at `0x802C2EFC`; `0x802C2700 + 2048 = 0x802C2F00`, the next row in both tables. |
| 13 rows (below) | `< gb` | `gb` | `gb + 4..32` | `main-gb`'s row stops before the terminal restore/return sequence (see next section). |

The 13 rows whose `main-gb` size is short of the true extent by the epilogue:

| address | main | main-gb | descent | delta |
|---|---:|---:|---:|---:|
| `0x800D19E0` | 20 | 976 | 980 | +4 |
| `0x800EEE70` | 36 | 368 | 384 | +16 |
| `0x800F6110` | 96 | 512 | 520 | +8 |
| `0x801177B0` | 20 | 592 | 596 | +4 |
| `0x80118450` | 16 | 768 | 772 | +4 |
| `0x8011D930` | 136 | 236 | 256 | +20 |
| `0x8017BA40` | 20 | 4692 | 4696 | +4 |
| `0x801C5BC0` | 28 | 548 | 560 | +12 |
| `0x80251440` | 932 | 1256 | 1260 | +4 |
| `0x8025FE30` | 20 | 1940 | 1948 | +8 |
| `0x8028A6F0` | 20 | 4012 | 4044 | +32 |
| `0x8029F900` | 20 | 3392 | 3404 | +12 |
| `0x802A6850` | 20 | 1108 | 1112 | +4 |

All 13 end with the same shape: the last `main-gb` instruction is the `bl` into
a `0x802DCDxx` register-restore helper, and the 1-8 instruction epilogue after
it (`lwz r0`, `mtlr r0`, `addi r1`, `bclr`) is still part of the body.
`0x800EEE70` and `0x8011D930` were read instruction-by-instruction to confirm
the pattern:

```
0x800EEFDC  481EDDD9  bl 0x802DCDB4     <- main-gb row ends at 0x800EEFE0
0x800EEFE0  80010054  lwz r0,0x54(r1)
0x800EEFE4  7C0803A6  mtlr r0
0x800EEFE8  38210050  addi r1,r1,0x50
0x800EEFEC  4E800020  bclr 20,0         <- true end (384 B)
```

So `main-gb` is canonical for identity and for ordinary bodies, but its `size`
column may omit a trailing epilogue. Prefer recursive descent for an exact
extent when the row ends on a `bl` to a restore helper.

### No other doc relies on a `main` size

A scan of every `0xADDR` (`N bytes`) doc block in `src/` found **0** blocks
quoting a `main` value. A prose scan found six other lines that mention a
`main` size, and all six already label it as an artefact:

| file | address | wording |
|---|---|---|
| `src/game/player.rs` | `0x80063270` | "that is a transcription slip" |
| `src/game/player.rs` | `0x80066630` | "that is a split artefact" |
| `src/game/player.rs` | `0x80064A50` | 16 B is the gap to the next 4-byte entry, not the extent |
| `src/ui/hud.rs` | `0x80025800` | "Correction to the manifest" |
| `src/ui/hud.rs` | `0x8003CF40` | "prologue artefact of the `main` analysis" |
| `src/ui/lyt.rs` | `0x801835A0` | 16 bytes is the store width, not a function size |

The `0x801406F0` sibling note was the only unqualified use of a `main` size.

### Ledger

All 1,311 affected addresses (549 shared + 762 `main-gb`-only) are already
ledger rows, and every one of them cites a `main-gb` export (1,304) or a
non-directory evidence note (7). No ledger evidence path points at the
truncated `main` project, so `ledger/functions.csv` needs no change.

## Reproduction

```
node work/wave253/laneA_scan.mjs        # both inventories + descent (laneA_scan.txt)
node work/wave253/laneA_scan.mjs --dump 80140a2c 80140b00
node work/wave253/laneA_summarize.mjs   # counts and the two main>gb rows
node work/wave253/laneA_ledger.mjs      # ledger cross-check
node work/wave253/laneA_doc_sizes.mjs   # `# Original` size claims
node work/wave253/laneA_prose.mjs       # prose size claims
```
