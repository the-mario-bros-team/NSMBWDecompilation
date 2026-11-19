# HANDOFF — NSMBW Wii Rust decompilation

Written at the end of wave 293 (2026-11-16). This document is self-contained: it
describes the project, the current verified state, the rules that keep the tree
consistent, the tooling (with exact commands), the known traps, and a concrete
plan for finishing the work.

This copy is intended to live at **`D:\nsmbw`**. If you extract or move the
project somewhere else, adjust the absolute paths below; every other path in
this document is relative to the project root.

Workspace: `D:\nsmbw` (the original machine path was
`C:\Users\WitherULTRA7\wii-decomp\nsmbw`; NOT `nmbw` — that path never existed
and several earlier prompts wasted time on it).

---

## 0. Start here (first 30 minutes)

1. Read this document top to bottom. It is written to be enough on its own.
2. Run the gate once, before trusting anything:
   ```powershell
   cd D:\nsmbw
   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/check.ps1
   ```
   It must print `OK`. If it does not, fix that first; every later decision
   assumes a green tree.
3. Skim, in this order:
   - `docs/COVERAGE_WAVE293_AUDIT.md` — what the last wave actually did.
   - `docs/COVERAGE.md` — current ledger totals.
   - `work/wave293/survey.md` and `work/wave293/d_scan_after.txt` — the backlog.
   - `work/wave293/INSTRUCTIONS.md` — the lane brief format.
   - One recent body, e.g. `fn_802b3b00` in `src/resource/archive.rs`, to see
     the evidence-block style.
4. Do **not** regenerate the images or the Ghidra exports; they are canonical.
5. Do **not** start a wave until you have read section 4 (rules) and section 6
   (traps). The traps have each caused a real regression.

The single most useful meta-rule: **trust the gate and the image, not the prose
in old reports.** Re-derive extents and call targets before changing a body.

---

## 1. What this project is

A from-scratch Rust reimplementation of **New Super Mario Bros. Wii** (`main.dol`
plus four RELs), built function by function from the shipped PowerPC code. It is
not a port of decompiled C: each body is written to match the shipped
instruction stream, with an evidence block that states the address, extent, the
table slot that reaches it, and the observed instructions.

`#![no_std]`-style host build. It compiles and links on the host; nothing runs
the game.

### Images (canonical inputs — do not regenerate)

| Path | What |
|---|---|
| `work/dol/main.elf` | `main.dol` as ELF (segment-aware reads) |
| `work/rels/elf/d_basesNP.elf` | REL, synthetic base `0x80600000` |
| `work/rels/elf/d_enemiesNP.elf` | REL |
| `work/rels/elf/d_en_bossNP.elf` | REL |
| `work/rels/elf/d_profileNP.elf` | REL |
| `work/ghidra/<image>/export/functions.tsv` | Ghidra inventory (20,252 addresses) |
| `work/ghidra/main-gb/export/` | Ghidra exports; `main-gb` is authoritative |

`main` export is **incomplete/truncated** (stock PPC language cannot decode
`psq_l`/GQR; 549 size disagreements; descent matches `main-gb` 533 times and
`main` **zero** times). Use `main-gb` for sizes.

### Where the code lives

`src/` — 68 files, ~52 MB. The active ones:

| File | Contents |
|---|---|
| `src/game/player.rs`, `game/worldmap.rs` | player / world-map actors and state machines |
| `src/game/enemies.rs` | enemy actors, `dEn_c` families |
| `src/game/stage.rs` | stage/BG/scene, many shared actor bodies |
| `src/game/bosses.rs` | boss actors |
| `src/game/bases.rs` | `dBase_c` and the `interface_call*` dispatcher family |
| `src/game/scene.rs`, `boot_scene.rs`, `task.rs`, `path.rs`, `actors/dokan.rs` | scenes, task system, paths, dokan actors |
| `src/ui/hud.rs`, `menu.rs`, `lyt.rs` | HUD, menus, NW4R layout |
| `src/resource/archive.rs`, `bres.rs`, `registry.rs` | RARC/BRRES, resource registry |
| `src/engine/display.rs`, `dsys.rs`, `frame.rs`, `globals.rs`, `main_loop.rs` | engine |
| `src/runtime/os.rs`, `cpp.rs`, `dvd.rs`, `input.rs`, `bt.rs`, `init.rs` | SDK/C++ runtime |
| `src/audio/ax.rs`, `seq.rs` | AX/sequence audio |
| `src/render/gx_band.rs`, `geometry.rs` | GX |

### Ledger (the coverage bookkeeping)

- `ledger/agents/*.csv` — **source of truth**, one fragment per wave/lane.
  Header: `module,address,name,status,confidence,evidence,rust_file`.
- `ledger/functions.csv` — merged by `node tools/merge-ledger.mjs`. Never hand-edit.
- Addresses: plain lowercase hex (`80012340`), no `0x`.
- Evidence: comma-free (use `; `), paths must exist.
- `status` is `translated` | `partial` | `stub`.
- Untabled bodies (no `functions.tsv` row) must **not** be added to the ledger;
  they carry `# Original (untabled)` instead.

---

## 2. Verified state at end of wave 293

| Metric | Value |
|---|---|
| Ghidra inventory | **20,250 ledgered / 20,252** (99.99%); headroom 2 |
| Ledger status | **20,226 translated / 22 partial / 2 stub** |
| Bodies check | `bodies.ok: true`, 19,064 `fn_<addr>` + 1,186 semantic, **0 gaps** |
| Duplicate bodies | 0 failing (name pass + semantic claimed-address pass) |
| Named vtables/method arrays closed | **103** |
| Gap-function corpus | **283 of 672** covered, 384 eligible |
| `_savegpr`/`_restgpr` modelled calls | **0** (was 616 in 324 bodies) |
| `cargo check` | 0 errors, **21 warnings** |
| Pointer-run survey | 540 runs, **130** fully covered, 3,996 uncovered slots |

The 22 partials are documented, defensible residue: 8 machine-state
(`rfi`/`mtmsr`/register-only), 5 caller-owned `bres` descriptors, 2 `va_list`,
2 unrecovered jump tables, 3 paired-single/missing-mode, 2 one-off. The 2 stubs
(`sdk_panic`, `set_exception_handler`) are 300-byte bodies that are forwarded,
not re-created. Details: `docs/COVERAGE_FINAL.md`.

The 2 headroom addresses are the frozen `game::bases` deferrals
`0x806E2BF0` (96 B) and `0x80697090` (248 B).

---

## 3. What remains (the actual backlog)

### A. Pointer-run prologue targets (named tables)

Wave 293 survey, refreshed after the wave's bodies landed
(`work/wave293/d_scan_after.mjs`, output in `work/wave293/after/`): **441
distinct prologue targets in runs of >=4 words**, 562 window-wide. Seven runs
completed this wave and none regressed (`130` fully covered). No run now exceeds
4 uncovered prologue targets. The five remaining 4-prologue runs are:

| Run | Slots | Uncov | Pro | Identity |
|---|---:|---:|---:|---|
| `0x803114b0` | 5 | 4 | 4 | next stage-effect sibling |
| `0x803228c0` | 18 | 4 | 4 | next cursor/actor table |
| `0x80323b48` | 51 | 4 | 4 | next actor-family table |
| `0x80327d38` | 18 | 4 | 4 | next display table |
| `0x8032b000` | 6 | 4 | 4 | next small table |

Everything with `>= 4` uncovered prologue targets is in that list; the rest of
the backlog is runs with 3 or fewer (`0x8034ef10`, `0x8034cd18`, `0x80316ba0`,
`0x80316be0`, `0x8034e1a8`, …).

Method: the survey is reproducible with `work/wave293/d_scan.mjs` (copy and
retarget `OUT`). Re-run it every few waves; closing tables reshuffles the rank.

### B. Gap-function corpus (functions no table names)

672 prologue-shaped functions found in code gaps. 283 done. The picker is
`work/wave292/e_pick.mjs`; candidates in `work/wave266/e_candidates.tsv`.
Three are **deferred on purpose**:

- `0x800211B0` — reads non-volatile `f31` as an undeclared third float input.
- `0x800D03A0` — clean next-body-after-extent.
- `0x8002B260` — in-band but not yet analysed.

Each batch is 12 bodies, all `9421xxxx` prologues with a single `blr`, verified
against `main.elf` before writing. Continue until the picker reports 0 eligible.

### C. The untabled pointer corpus

`docs/UNTABLED_POINTER_TABLES.md` (94 KB) tracks pointer tables and the 2,457
untabled entries found in wave 226. The survey's "interior labels" (2,127 slots)
are jump-table arms, **not** bodies — do not translate them.

### D. Doc debt

- `docs/COVERAGE_FINAL.md` and `docs/NEXT_STEPS.md` are **stale** (last touched
  wave 250). Update them from the current ledger before relying on them.
- Per-wave audits `docs/COVERAGE_WAVE2xx_AUDIT.md` are accurate for their wave but
  contain superseded claims; the latest (wave 293) is authoritative.
- `docs/UNTABLED_POINTER_TABLES.md` has had several span corrections; re-derive
  spans from the image when a table's extent matters.

### E. Evidence hygiene

- Some lane reports reference scratch files that were later overwritten or
  truncated (e.g. `work/wave259/b/disasm.txt`, `work/wave291/*_verify.txt`).
  Audits should re-derive rather than trust them.
- `work/backup_src/` is a rolling mirror of `src/` (robocopy `/MIR`). Refresh it
  before each wave.

---

## 4. Rules that keep the tree consistent

1. **One canonical owner per address.** Other files may define the same name only
   as a one-line delegation with a `// canonical:` marker, or as a
   `#[cfg(target_arch)]` pair. The gate enforces this.
2. **Untabled bodies** get `# Original (untabled)` with address, measured extent,
   table slot/word/offset when reachable, and the observed instruction stream.
   They do **not** get ledger rows.
3. **Ledger rows** must exist in some `functions.tsv` and have a body whose
   `# Original` cites the address (or a semantic name the ledger names).
4. **Extents are measured by recursive descent**:
   - `bcctrl` (`0x4E800421`, LK=1) is a **call** and falls through.
   - Conditional `bclr`/`bc` fall through; only the taken path matters.
   - Unconditional `bclr 20,0`, tail `bcctr` (LK=0) and an unconditional `b`
     stop the walk.
   - A forward intra-body `b` must be followed unless it targets a known
     function start (then it is a tail branch).
   - Descent stops at a known next start.
5. **Never whole-file-write a source file from a stale read.** Read, modify, write
   once. A lane once truncated `src/ui/lyt.rs` to 0 bytes this way; the gate now
   fails that shape, and `work/backup_src/` exists for recovery.
6. **New constants** are suffixed with lane+wave (`_A293`, `_B292`) to avoid
   cross-lane duplicates.
7. **Do not tidy sibling asymmetries away.** When two twins differ in polarity,
   offset, or a dead branch, that difference is usually real. Document it.

---

## 5. Tooling and exact commands

Run everything from `C:\Users\WitherULTRA7\wii-decomp\nsmbw`.

| Command | Purpose |
|---|---|
| `powershell -NoProfile -ExecutionPolicy Bypass -File scripts/check.ps1` | full gate (cargo, doc-lint, integrity, merge-ledger, verify, duplicates, helpers, page offsets, index, coverage, progress, gap) |
| `cargo check` | compile |
| `node tools/doc-lint.mjs src` | doc-comment rules (# Original, # Confidence, etc.) |
| `node tools/verify-ledger.mjs --enforce-bodies` | tsv membership, headroom, body presence |
| `node tools/check-dupe-bodies.mjs` | one canonical body per address |
| `node tools/check-helper-calls.mjs` | no modelled `_savegpr`/`_restgpr` calls |
| `node tools/check-page-offsets.mjs` | documented `lis`/`addi` pairs and name-vs-value |
| `node tools/check-source-integrity.mjs [--record]` | no truncation/encoding damage |
| `node tools/merge-ledger.mjs` | rebuild `ledger/functions.csv` from `ledger/agents/*.csv` |
| `node tools/index-functions.mjs` | rebuild `docs/FUNCTION_INDEX.md` |

Disassembler (preserved for this handoff): `node tools/disasm-ppc.js <elf> <hexaddr>...`
Honours `$env:LEN` (instruction count) and `$env:STOP`. The image does not need
relocation: pass addresses as they appear in Ghidra.

Reading raw data:
```powershell
node -e "const b=require('fs').readFileSync('work/dol/main.elf'); ..."
```
Use the program headers (`e_phoff 0x1c`, `e_phentsize 0x2a`, `e_phnum 0x2c`)
to map address to file offset. `main.dol` segments end at `0x8042FBE0`;
`0x8043xxxx` is unmapped except the stack/SDA-2 windows.

REL addressing: synthetic base `0x80600000`; Ghidra REL addresses already include
the base. REL file offsets are `address - 0x80600000 + 0x1000`.

---

## 6. Known traps (each one has caused a real bug)

| Trap | Symptom | Rule |
|---|---|---|
| `bcctrl` vs `bcctr` | body truncated at a call | `0x4E800421` is a call, falls through |
| `addi` sign extension | wrong page (`0x8043xxxx` from `-0x47C0`) | sign-extend the 16-bit immediate; the page-offset gate catches documented pairs |
| `cror 2,1,2` vs `2,0,2` | inverted comparison | `2,1,2` merges LT|GT (`>=`); `2,0,2` merges LT|EQ (`<=`); `bc 4,2` is branch-if-false |
| opcode-63 A-form | `fmr` printed as `fcmpu` | decode by XO: 72 `fmr`, 12 `frsp`, 32 `fcmpo`, 0 `fcmpu`, 23 `fsel`, 15 `fctiwz` |
| opcode-59 A-form | `fmuls` operand order | `fmuls fD,fA,fC` — third is FRC, not FRB |
| X31 `add` | printed `X31_266` | `add rD,rA,rB`, not an unknown op |
| Pair `r3:r4` | high/low halves reversed | the first word is `r3`, the second `r4`; check `CONCAT44` order |
| `_savegpr_*` calls | fake return values | the band `0x802DCD40..D4` is a save/restore table; the "return" is the incoming register |
| Misattributed body | a copy of a neighbour | compare `bl` targets against the Ghidra export; check body size |
| Dropped stores/tails | body "complete" but short | account for **every store and tail call** in the extent |
| Semantic-name owners | censuses report false gaps | a body may be named `wm_player_enter`; search by `# Original` address, not by `fn_<hex>` |
| Boundary creator word | one phantom slot | a 17-word run may be 16 methods + next class's `{creator, magic, count}` |

---

## 7. Suggested next waves (concrete)

The current workflow is 6 parallel lanes per wave, ~10–12 bodies each, plus one
audit lane. A good next wave:

- **A** — `0x803114b0` + `0x803228c0` (both 4 prologue): decode, translate.
- **B** — `0x80323b48` + `0x80327d38` (both 4 prologue): decode, translate.
- **C** — `0x8032b000` + the next-ranked run from a fresh survey.
- **D** — survey refresh (`work/wave293/d_scan.mjs`) and update the ranked list.
- **E** — gap batch 24: next 12 from `e_pick.mjs`.
- **F** — audit the previous wave's bodies; use the store-coverage checker
  (`work/wave293/f_*`) to catch dropped writes.

Lane prompt template (this is what has been working):

> Wave N, lane X: <task>. Work in
> `C:\Users\WitherULTRA7\wii-decomp\nsmbw`; read `work/waveN/INSTRUCTIONS.md`.
> Decode from `work/dol/main.elf`: span, header, install site, identity, census.
> `bcctrl` and conditional `bclr`/`bc` fall through. Translate the uncovered
> targets into <file> under `_X<N>`, each with an `# Original (untabled)` block
> accounting for every store and call. Scan first for existing bodies.
> CRITICAL: read the current file, modify, write once. Verify with `cargo check`,
> `node tools/doc-lint.mjs src`, `node tools/check-dupe-bodies.mjs`,
> `node tools/check-page-offsets.mjs`, `node tools/check-helper-calls.mjs`,
> `node tools/check-source-integrity.mjs`.

---

## 8. Definition of "finished"

1. Gap corpus picker reports **0 eligible** (currently 384).
2. Survey reports **0 uncovered prologue targets** in runs (currently 441;
   562 window-wide).
3. Ledger stays 20,250 rows; statuses reach 20,248 translated + 2 documented
   stubs, or the 22 partials are re-justified with new evidence.
4. `cargo check` 0 errors; warnings no higher than 21.
5. All six gate tools green, `scripts/check.ps1` OK.
6. `docs/COVERAGE_FINAL.md` and `docs/NEXT_STEPS.md` rewritten from the current
   ledger (they are stale at handoff).

---

## 9. Quick sanity checks before trusting this document

```powershell
cd D:\nsmbw
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/check.ps1
node tools/verify-ledger.mjs --enforce-bodies
node tools/check-helper-calls.mjs
node tools/check-source-integrity.mjs
```

Expected: `OK`; 20,250 rows with `bodies.ok: true`; `no live helper references`;
68 files, 0 findings. If any fail, read the tool's output first — each is
designed to name the exact file and reason.
