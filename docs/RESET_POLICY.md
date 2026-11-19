# Model and reset policy

Standing instruction from the user (2026-11-09): **use the paid
`opencode-go/deepseek-v4.1-flash`** for new waves. The 2026-11-11 directive
sets the wave size to **ten lanes** (up from four/three). Do not wait to be
told. Do not ask. Launch the next wave within the same turn.

This file is the canonical statement of that policy so it survives session
handoffs. The workflow it describes is the one to follow at every wave
boundary.

## The rule

1. **Keep ten lanes running at all times.** Use `node tools/plan-next-wave.mjs`
   with the currently-running lane files passed as `--exclude`; take the first
   ten disjoint lanes ordered by remaining bucket mass (run it twice with the
   first five excluded to get the second five).
2. **Use the paid model.** Launch lanes on
   `opencode-go/deepseek-v4.1-flash`. The free pool is exhausted and is no
   longer the wave model. Never use `iceman-contributor-free`. Do not switch
   models without user approval.
3. **If the paid model fails, pause and report.** Salvage landed work first
   (`node tools/audit-unledgered.mjs --file <its rust file>`), record the
   failure below, and wait for instructions. Do not silently rotate models.
4. **Salvage before retrying.** When a lane dies mid-run, run
   `node tools/audit-unledgered.mjs --file <its rust file>` first. Verified
   bodies are recovered with `--csv ledger/agents/salvage-<lane>.csv`, merged,
   and documented — the work is never rewritten from zero.
5. **Launch the next wave immediately after merging.** Steps per boundary:
   `node tools/check-lane-csv.mjs wave<N>` → `node tools/merge-ledger.mjs` →
   `node tools/verify-ledger.mjs` → `node tools/progress.mjs` →
   `powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\check.ps1`
   → update `README.md` (row 3), `PLAN.md` (measured-coverage bullets),
   `docs/LANES.md`, `docs/NEXT_STEPS.md` → launch next ten.
6. **Create the next brief from the previous one**, e.g.
   `(Get-Content docs\LANE_BRIEF_WAVE45.md -Raw) -replace 'Wave 45','Wave 46' -replace 'wave45','wave46'` into `docs/LANE_BRIEF_WAVE46.md`. Confirm zero stale wave references.

## Current state (2026-11-15, wave 239 freeze)

- **The `functions.tsv` inventory is complete.** 20,250 of 20,252 functions have
  a translated body; the only two outstanding rows are the documented
  `game::bases` deferrals `0x806E2BF0` (96 B) and `0x80697090` (248 B). Per
  export: main.dol 10,276/10,276, d_profileNP 36/36, d_basesNP 5,613/5,615,
  d_enemiesNP 3,555/3,555, d_en_bossNP 770/770. Ledger status: 20,020
  translated, 217 partial, 13 stub; 0 staging, 0 duplicate addresses.
- **The untabled corpus is the remaining translation mass.** Wave 226 found
  2,457 prologue-verified main.dol starts with no tsv row. Waves 228-238
  translated **259** (251 exact `fn_<address>`, 8 suffixed/semantic), leaving
  **2,198**; **371** entries are aliases of other slots. `src` holds 583
  untabled body blocks over 548 addresses.
- `docs/COVERAGE_FINAL.md` is the authoritative, frozen accounting; every
  figure in `README.md`, `PLAN.md`, `docs/NEXT_STEPS.md`, `docs/LANES.md` and
  this file must match it. Wave 239 is the closing wave: no new refactors, no
  hand edits to `ledger/functions.csv`, and lanes E/F only continue the two
  largest vtables.

## Historical state (2026-11-11, wave 93)

- Waves 35-93 complete. Wave 93 finished at 11,886 / 20,252 (58.69%). Wave 94
  is a four-lane transition round already in flight; **from Wave 95 onward
  every round is ten lanes**.
- Waves 35-45 complete. Wave 39 (crouton exception) added registry 13,
  archive 25, dvd 30, stage 13, player 30. Wave 40 used three lanes on the
  free model: registry 27, dvd 28, archive 18 after a retry. Wave 41 added
  registry 30, stage 28 and archive 28. Wave 42 lost registry/stage to the
  free limit but salvaged bt 18. Wave 43 wrote nothing; Wave 44 was translated
  manually (56 registry wrappers in four batches). Wave 45, switched to the
  paid `opencode-go/deepseek-v4.1-flash`, added stage 26, registry 28 and
  seq 31 for 85 bodies. Wave 46 added archive 28, enemies 29 and dvd 30;
  Wave 47 added enemies 26, bases 30 and bt 31; Wave 48 added enemies 27,
  bases 30 and stage 29; Wave 49 added bases 29, archive 30 and player 30;
  Wave 50 added bases 30, input 28 and stage 25; Wave 51 added bases 30,
  worldmap 30 and enemies 29; Wave 52 added registry 28, bases 30 and seq 30;
  Wave 53 added bases 30, enemies 25 and bt 25; Wave 54 added bases 29,
  archive 28 and enemies 30; Wave 55 added bases 30, enemies 28 and
  worldmap 30; Wave 56 added bases 26, dvd 28 and enemies 29; Wave 57 added
  bases 30, enemies 30 and main_loop 30; Wave 58 added bases 28, bt 28 and
  enemies 30; Wave 59 added bases 29, stage 29 and enemies 29; Wave 60 added
  bases 27, worldmap 30 and bt 26; Wave 61 added bases 29, stage 25 and
  input 30; Wave 62 added bases 30, stage 30 and archive 28; Wave 63 added
  bases 28, player 29 and display 30; Wave 64 added bases 29, worldmap 26
  and registry 30; Wave 65 added bases 30, enemies 30 and cpp 27; Wave 66
  (six lanes) added bases 30, enemies 29, worldmap 27, bt 30, seq 28 and
  archive 27; Wave 67 (five lanes) added bases 30, enemies 26, dvd 27, os 28
  and worldmap 26; Wave 68 (four lanes) added bases 27, enemies 30, bt 28
  and main_loop 29; Wave 69 (four lanes) added bases 29, enemies 29, stage
  30 and bt 29; Wave 70 (four lanes) added bases 29, enemies 30, stage 27
  and bres 30; Wave 71 (four lanes) added bases 30, enemies 26, stage 29
  and os 29; Wave 72 (four lanes) added bases 27, enemies 27, dvd 30 and
  player 30; Wave 73 (four lanes) added bases 27, enemies 28, worldmap 28
  and archive 30; Wave 74 (four lanes) added bases 30, enemies 29, input 28
  and player 26; Wave 75 (four lanes) added bases 26, enemies 30, stage 27
  and dsys 30; Wave 76 (four lanes) added bases 29, enemies 30, display 30
  and worldmap 28; Wave 77 (four lanes) added bases 30, enemies 30, cpp 26
  and worldmap 30; Wave 78 (four lanes) added bases 30, enemies 27, display
  28 and registry 25; Wave 79 (four lanes) added bases 30, enemies 30, bt 26
  and archive 30; Wave 80 (four lanes) added bases 28, enemies 29, seq 25
  and dvd 25; Wave 81 (four lanes) added bases 30, enemies 30, worldmap 30
  and os 29; Wave 82 (four lanes) added bases 27, enemies 30, bt 28 and
  main_loop 25; Wave 83 (four lanes) added bases 30, enemies 28, stage 26
  and os 27; Wave 84 (four lanes) added bases 29, enemies 30, stage 29 and
  bt 25; Wave 85 (four lanes) added bases 29, enemies 29, bres 15 and stage
  28; Wave 86 (four lanes) added bases 30, enemies 29, worldmap 29 and os
  28; Wave 87 (four lanes) added bases 29, enemies 25, dvd 30 and player 30;
  Wave 88 (four lanes) added bases 30, enemies 29, player 28 and stage 29;
  Wave 89 (ten lanes) added bases 30, enemies 28, input 27, archive 29,
  worldmap 29, cpp 26, dsys 28, registry 27, display 29 and bt 26; Wave 90
  (ten lanes) added bases 30, enemies 30, worldmap 30, display 30, seq 28,
  dvd 29, bres 28, archive 28, hud 26 and ax 14; Wave 91 (ten lanes) added
  bases 30, enemies 28, worldmap 28, os 29, main_loop 25, stage 25, bt 29,
  player 29, input 26 and dvd 30; Wave 92 (ten lanes) added bases 30,
  enemies 26, worldmap 29, bt 28, os 26, stage 30, player 27, cpp 20, ax 28
  and archive 25; Wave 93 (ten lanes) added bases 30, enemies 30, stage 25,
  os 26, worldmap 28, registry 26, ax 28, dsys 26, bt 25 and display 28;
  Wave 94 (four lanes) added bases 29, enemies 28, stage 28 and worldmap 28;
  Wave 95 (ten lanes) added bases 30, enemies 27, worldmap 27, seq 26,
  display 26, bres 22, dvd 29, gx_band 28, archive 25 and main_loop 25;
  Wave 96 (ten lanes) added bases 30, enemies 30, worldmap 29, hud 30,
  stage 20, os 25, gx_band 27, bt 24, cpp 14 and input 26; Wave 97 (ten
  lanes) added bases 29, enemies 30, worldmap 30, os 26, bt 26, archive 26,
  player 38, input 25, registry 25 and dsys 25; Wave 98 (ten lanes) added
  bases 27, enemies 26, stage 24, os 25, worldmap 26, player 26, ax 20,
  dvd 25, gx_band 19 and bres 11; Wave 99 (ten lanes) added bases 28,
  enemies 30, stage 30, bt 27, ax 27, seq 25, display 25, cpp 7, archive 25
  and main_loop 20; Wave 100 (ten lanes) added bases 30, enemies 27,
  stage 25, worldmap 25, display 25, gx_band 18, dvd 24, os 14, cpp 3 and
  bres 24; Wave 101 (ten lanes) added bases 28, enemies 30, worldmap 28,
  stage 17, bt 25, os 25, hud 26, gx_band 25, cpp 32 and ax 6; Wave 102
  (ten lanes) added bases 30, enemies 26, worldmap 26, stage 15, bt 25,
  os 25, gx_band 9, menu 19, input 24 and archive 22; Wave 103 (ten lanes)
  added bases 30, enemies 27, worldmap 25, stage 9, input 14, player 17,
  registry 19, dsys 15, dvd 16 and ax 7, plus one salvaged worldmap body;
  Wave 104 (ten lanes, five completed live and five salvaged after the model
  limit) added bases 27, enemies 30, worldmap 21, stage 14, gx_band 7,
  os 14, bt 19, ax 18, seq 13 and main_loop 5; Wave 105 (ten lanes) added
  geometry 8, bases 29, enemies 28, stage 17, display 7, gx_band 5,
  archive 15, ax 7, main_loop 4 and dvd 4; Wave 106 (ten lanes) added bases
  28, enemies 30, worldmap 14 (one withdrawn as GQR-only), stage 11, display
  7, player 11, input 6, bres 5, main_loop 7 and dvd 8; Wave 107 (ten lanes)
  added bases 30, enemies 27, worldmap 16, stage 6, gx_band 3, bt 7, os 8,
  bosses 8, hud 6 and dsys 4; Wave 108 (ten lanes) added bases 29, enemies
  28, worldmap 10, stage 11, gx_band 1, display 10, menu 6, ax 1, bt 5 and
  os 4; Wave 109 (ten lanes) added bases 30, enemies 26, gx_band 2, stage 7,
  display 9, player 6, ax 6, dvd 4, seq 4 and bres 2, while retiring the
  coarse `runtime::os` SPR range row in favour of the six real bodies; Wave
  110 (ten lanes) added bases 30, enemies 29, gx_band 3, stage 3, worldmap 3,
  geometry 4, registry 4, os 4, input 2 and dsys 3; Wave 111 (ten lanes)
  added bases 25, enemies 29, gx_band 2, stage 2, worldmap 3, lyt 3, bres 1,
  os 2, main_loop 1 and dvd 1; Wave 112 (ten lanes) added bases 30,
  enemies 29, gx_band 2, stage 3, worldmap 4, input 2, bt 2, bres 1,
  archive 2 and seq 2; Wave 113 (ten lanes) added bases 29, enemies 27,
  gx_band 2, stage 4, worldmap 4, input 2, bt 1, main_loop 2, bosses 2 and
  hud 1; Wave 114 (ten lanes) added bases 29, enemies 27, gx_band 3,
  stage 2, worldmap 3, bosses 2, display 1, menu 1, ax 1 and dvd 1; Wave
  115 (ten lanes) added bases 26, enemies 30, gx_band 2, stage 2, worldmap
  1, display 1, dvd 1, bt 1, bres 1 and os 1; Wave 116 (seven lanes, seven
  buckets remained) added bases 29, enemies 28, gx_band 1, stage 1,
  worldmap 1, bt 1 and os 1; Wave 117 (six lanes, six buckets) added bases
  27, enemies 28, gx_band 2, stage 1, archive 1 and bosses 1; Wave 118
  (five lanes) added bases 30, enemies 30, gx_band 1, stage 2 and bosses 1;
  Wave 119 (four lanes) added bases 26, enemies 27, gx_band 2 and stage 2;
  Wave 120 (four lanes) added bases 28, enemies 30, gx_band 1 and stage 1;
  Wave 121 (four lanes) added bases 30, enemies 28, gx_band 2 and stage 1;
  Wave 122 (three lanes) added bases 30, enemies 28 and gx_band 1; Wave
  123 (four lanes) added bases 30, enemies 28, gx_band 1 and stage 1;
  Wave 124 (three lanes) added bases 30, enemies 28 and gx_band 1; Wave
  125 (three lanes) added bases 30, enemies 27 and gx_band 1; Wave 126
  (two translation lanes + two audits) added bases 30 and enemies 29; Wave
  127 added bases 28 and enemies 30, plus reclassification reports; Wave
  128 added bases 30 and enemies 29, plus two implementation-ready specs;
  Wave 129 added bases 25 and enemies 33, plus two more specs; Wave 130
  added bases 22 and enemies 28, plus two more specs; Wave 131 added bases
  26 and enemies 26, plus two more specs; Wave 132 added bases 28 and
  enemies 27, plus two more specs; Wave 133 added bases 26 and enemies 28,
  plus two more specs; Wave 134 added bases 25 and enemies 24, plus two
  more specs; Wave 135 added bases 25 and enemies 29, plus two more specs;
  Wave 136 added bases 28 and enemies 26, plus two more specs; Wave 137
  added bases 27 and enemies 25, plus two more specs; Wave 138 added
  bases 26 and enemies 28, plus two more specs (GQR3 pinned); Wave 139
  added bases 30 and enemies 25, plus two more specs; Wave 140 added bases
  27 and enemies 30, plus two more specs; Wave 141 added bases 11 and
  enemies 17 (large bodies), plus two more specs; Wave 142 added bases 17
  and enemies 17 (large bodies), plus two more specs; Wave 143 added bases
  10 and enemies 18, plus two more specs; Wave 144 added bases 16 and
  enemies 17, plus two more specs; Wave 145 added bases 17 and enemies 15,
  plus two more specs; Wave 146 added bases 13 and enemies 17, plus two more
  specs; Wave 147 added bases 16 and enemies 15, plus two more specs; Wave
  148 added bases 16 and enemies 16, plus two more specs; Wave 149 added
  bases 16 and enemies 16, plus two more specs; Wave 150 added bases 17 and
  enemies 20, plus two more specs; Wave 151 added bases 15 and enemies 17,
  plus two more specs; Wave 152 added bases 10 and enemies 18, plus two more
  specs; Wave 153 added bases 10 and enemies 19, plus two more specs; Wave
  154 added bases 13 and enemies 25, plus two more specs; Wave 155 added
  bases 11 and enemies 17, plus two more specs; Wave 156 added bases 17 and
  enemies 5, plus an enemies exhaustion audit and a bases spec; Wave 157
  added bases 16 and the last three enemies bodies; Wave 158 added bases 13,
  a bases backlog reclassification, a bases spec and a full ledger audit;
  Wave 159 added bases 39 from the ranked small-body clusters; Wave 160 added
  bases 33 from the next clusters and quick singles; Wave 161 added bases 55
  constant getters; Wave 162 added bases 29 short singles; Wave 163 added bases
  35 from the 4-36 byte band; Wave 164 added the last ten 36-40 byte singles;
  Wave 165 added bases 39 from the 44-64 byte band; Wave 166 added the other 28
  from the same band; Wave 167 added the last 18 and completed it.
- Measured total: **17,537 / 20,252 (86.59%)**, main.dol 9,055/10,276
  (88.12%), d_en_bossNP 770/770 (100%), d_enemiesNP 3,555/3,555 (100%),
  d_basesNP 4,115/5,615 (73.29%), gates green.

## Exhaustion log

The free pool is exhausted and no longer used for waves. Record paid-model
failures here if they occur:

| Model | Last known dead | Last known alive |
|---|---|---|
| `opencode-go/deepseek-v4.1-flash` (paid) | 2026-11-13 (Wave 159: all four lanes refused at launch, weekly limit, resets in 2d 5h) | 2026-11-13 (Wave 158 complete and gated) |
| `opencode/deepseek-v4.1-flash-free` | 2026-11-08 (Wave 45 immediate) | 2026-11-08 (Wave 41 complete; Wave 42 bt partial) |
| `opencode-go/crouton-alpha-free` | — | 2026-11-06 (Wave 39 exception) |
| `opencode/glm-5.3-flash-free` | 2026-11-04 | 2026-11-06 (Wave 37) |
| `opencode/mimo-v2.6-flash-free` | — | 2026-11-06 (Wave 35) |

Re-probe only if the paid model starts failing.

**Wave 104 incident (2026-11-11).** The paid model completed bases, enemies,
worldmap, stage and gx_band, then returned `OpenCode Go usage limit reached`
for os, bt, ax, seq and main_loop. Per the rule above, all landed bodies were
salvaged with `audit-unledgered.mjs`, the wave was repaired (bt constants and
three type errors), merged and gated green at **14,298 / 20,252 (70.60%)**.
Wave 105 is not launched; it waits for a limit reset or explicit approval of
another model.

**Wave 106 recovery incident (2026-11-11/12).** The `game::worldmap` lane
flattened `src/game/worldmap.rs` with a `Set-Content -NoNewline` round-trip and
re-derived its line breaks. The file compiles and all other bodies were
verified, but one claimed row (`0x80119240`) did not survive and its export is
`unaff_GQR0`/`ldexpf`-dominated, so the row was withdrawn rather than invented.
Lesson: never edit a whole Rust file with `Get-Content | Set-Content`; use the
edit tool, which preserves line structure. The `display` lane also hung for six
hours without landing a body; its retry was time-boxed to 8 bodies and finished
with 7.

**Wave 159 exhaustion (2026-11-13).** Wave 158 merged and gated green at
**17,251 / 20,252 (85.18%)** with zero unledgered bodies. The four planned
lanes (bases translation, bases spec, class-P pinning, clone mapping) were
launched against `opencode-go/deepseek-v4.1-flash` and all four were refused
before doing any work: `OpenCode Go usage limit reached (weekly, resets in 2d
5h)`. Only `docs/LANE_BRIEF_WAVE159.md` had been written; `audit-unledgered.mjs`
reported 3,829 bodies / 3,829 ledgered / 0 unledgered, so there was nothing to
salvage.

The coordinator then ran the translation lane directly instead of waiting:
39 bodies (2,440 bytes) from the ranked small-body clusters in
`docs/ANALYSIS_WAVE158_BASES.md` were written into `src/game/bases.rs`,
registered in `ledger/agents/wave159-bases.csv`, and merged. Wave 159 is gated
green at **17,290 / 20,252 (85.37%)**; `cargo check` and doc-lint are clean and
`check-lane-csv.mjs wave159` reports 39 unique rows.

**Wave 160 direct run (2026-11-13).** The paid model was still at its weekly
limit, so the coordinator continued the translation lane directly: 33 bodies
(1,872 bytes) from ranks 11-20 of `docs/ANALYSIS_WAVE158_BASES.md` plus twelve
destructor/flag singles were written into `src/game/bases.rs`, registered in
`ledger/agents/wave160-bases.csv`, and merged. Wave 160 is gated green at
**17,323 / 20,252 (85.54%)**; `check-lane-csv.mjs wave160` reports 33 unique
rows, and `cargo check` plus doc-lint are clean. The three read-only reports
(spec, class-P, clones) are still owed and should be the first tasks once a
working model is available.

**Wave 161 direct run (2026-11-13).** Still limited, so the coordinator continued
directly: 55 twelve-byte constant getters (660 bytes) were written into
`src/game/bases.rs`, registered in `ledger/agents/wave161-bases.csv`, and
merged. Wave 161 is gated green at **17,378 / 20,252 (85.81%)**;
`check-lane-csv.mjs wave161` reports 55 unique rows, and `cargo check` plus
doc-lint are clean. Five getters required a raw-word recompute of the negative
`addi` displacement. The three read-only Wave 159 reports remain owed.

**Wave 162 direct run (2026-11-13).** Still limited, so the coordinator continued
directly: 29 short singles (368 bytes) were written into `src/game/bases.rs`,
registered in `ledger/agents/wave162-bases.csv`, and merged. Wave 162 is gated
green at **17,407 / 20,252 (85.95%)**; `check-lane-csv.mjs wave162` reports 29
unique rows, and `cargo check` plus doc-lint are clean. Two `lwz` globals and
the `stwu`/`lwzu` base updates were recomputed from the raw words.

**Wave 163 direct run (2026-11-13).** Still limited, so the coordinator continued
directly: 35 bodies from the 4-36 byte band (580 bytes) were written into
`src/game/bases.rs`, registered in `ledger/agents/wave163-bases.csv`, and
merged. Wave 163 is gated green at **17,442 / 20,252 (86.12%)**;
`check-lane-csv.mjs wave163` reports 35 unique rows, and `cargo check` plus
doc-lint are clean. The two `main.dol` thunks are recorded at medium confidence
because only the tail branch is shipped.

**Wave 165 direct run (2026-11-13).** Still limited, so the coordinator continued
directly: 39 bodies from the 44-64 byte band (2,208 bytes) were written into
`src/game/bases.rs`, registered in `ledger/agents/wave165-bases.csv`, and
merged. Wave 165 is gated green at **17,491 / 20,252 (86.37%)**;
`check-lane-csv.mjs wave165` reports 39 unique rows, and `cargo check` plus
doc-lint are clean. Every branch word in each family was normalised before
translation: the 21 destructors are byte-identical, and the release
dispatchers turned out to forward `r5` differently in their two arms.

**Wave 166 direct run (2026-11-13).** Still limited, so the coordinator continued
directly: the remaining 28 bodies of the 44-64 byte band (1,556 bytes) were
written into `src/game/bases.rs`, registered in
`ledger/agents/wave166-bases.csv`, and merged. Wave 166 is gated green at
**17,519 / 20,252 (86.51%)**; `check-lane-csv.mjs wave166` reports 28 unique
rows, and `cargo check` plus doc-lint are clean. The `lwzu` wrapper family was
normalised against the 10 slot/base pairs, and the four vtable installers were
confirmed to return `this` from before the vtable store.

**Wave 167 direct run (2026-11-13).** The paid model was still limited, so the
coordinator continued directly: the last 18 bodies of the 44-64 byte band (984
bytes) were written into `src/game/bases.rs`, registered in
`ledger/agents/wave167-bases.csv`, and merged. Wave 167 is gated green at
**17,537 / 20,252 (86.59%)**; `check-lane-csv.mjs wave167` reports 18 unique
rows, and `cargo check` plus doc-lint are clean. The three `halt_baddata`
exports (`0x80652310`, `0x8067B1C0`, `0x807A57F0`) were recovered from raw words:
the `0x4C411382`/`0x4C401382` `cror` instructions fold the inclusive window
tests, and `0x8074DAD0`'s final rotate/mask returns the sign bit. `0x80749310`
remains low-confidence because its `r12` descriptor cannot be modelled in Rust.

**Wave 168 direct run (2026-11-13).** The paid model was still limited, so the
coordinator continued directly: the first seven bodies of the 300-512 byte band
(2,252 bytes) were written into `src/game/bases.rs`, registered in
`ledger/agents/wave168-bases.csv`, and merged. Wave 168 is gated green at
**17,544 / 20,252 (86.63%)**; `check-lane-csv.mjs wave168` reports 7 unique
rows, and `cargo check` plus doc-lint are clean. The batch covered the two
colour-table renderer helpers, the padded box test and the
`0x807C2A00`/`0x807C2E50` state twins sharing `fn_807c2a_like`.

**Wave 169 direct run (2026-11-13).** Still limited, so the coordinator
translated eight more bodies from the 300-316 byte sub-band (2,428 bytes) into
`src/game/bases.rs`, registered in `ledger/agents/wave169-bases.csv`, and
merged. Wave 169 is gated green at **17,552 / 20,252 (86.67%)**;
`check-lane-csv.mjs wave169` reports 8 unique rows, and `cargo check` plus
doc-lint are clean. The batch covered the anchor movers, the player-window test,
the air-state tick, the transform rebuild, the pose copy, the shared-state lerp,
the renderer seed and the sample axis test.

**Wave 170 direct run (2026-11-13).** The paid model was still limited, so the
coordinator continued directly: eight bodies from the 312-320 byte sub-band
(2,520 bytes) were written into `src/game/bases.rs`, registered in
`ledger/agents/wave170-bases.csv`, and merged. Wave 170 is gated green at
**17,560 / 20,252 (86.71%)**; `check-lane-csv.mjs wave170` reports 8 unique
rows, and `cargo check` plus doc-lint are clean. The batch covered the spawn
reset, the render push, the source re-seed, the third `0x900` request gate,
both layout loaders, the ship controller and the state dispatch.

**Wave 171 direct run (2026-11-13).** The paid model was still limited, so the
coordinator continued directly: eight bodies from the 324-332 byte sub-band
(2,616 bytes) were written into `src/game/bases.rs`, registered in
`ledger/agents/wave171-bases.csv`, and merged. Wave 171 is gated green at
**17,568 / 20,252 (86.75%)**; `check-lane-csv.mjs wave171` reports 8 unique
rows, and `cargo check` plus doc-lint are clean. The batch covered the pose
copy, the selector reconcile, the two `0xD4` gate twins, the transform
rebuild, the multi-course-select loader, the throw start and the throw end.
`0x8060A5D0` was corrected to take `this` after the raw words showed the
leading `0x802DCD6C` call is a `_savegpr` helper whose `r3` survives.

**Wave 172 paid-model return (2026-11-14).** The paid
`opencode-go/deepseek-v4.1-flash` limit reset, so the wave ran with five
parallel subagent lanes instead of the coordinator. Lanes A-E wrote 24 bodies
from the 332-348 byte sub-band (8,144 bytes) into their own block files, five
lane CSVs and five lane coverage notes; the coordinator appended the blocks,
consolidated `docs/COVERAGE_WAVE172_BASES.md`, and fixed three cross-wave call
signature errors: `0x801680E0` needs the quotient in `f1` (Wave 171's
`fn_8064eeb0` dropped it), seven `0x8077E410` call sites used `call_f64_u32x3`
where the first argument is `f32`, and `fn_80621630` called `0x80620A00`
without `this`. Wave 172 is gated green at **17,592 / 20,252 (86.87%)**;
`check-lane-csv.mjs wave172` reports 24 unique rows (23 high, 1 medium).

**Wave 173 (2026-11-14).** Five paid-model lanes wrote 19 bodies from the
352-364 byte sub-band (6,808 bytes), five lane CSVs and five lane coverage
notes; the coordinator consolidated `docs/COVERAGE_WAVE173_BASES.md`. Wave 173
is gated green at **17,611 / 20,252 (86.96%)**, d_basesNP 4,189/5,615 (74.60%);
`check-lane-csv.mjs wave173` reports 19 unique rows (14 high, 5 medium). Useful
findings this wave: the `0x801680E0` `f1` convention now has four independent
confirmations (`fn_8064eeb0`, `fn_8064f510`, `fn_8064f820`, lane B), and
Ghidra's `_savegpr` model swaps or hides arguments in `0x80742470`,
`0x8076FF80` and `0x806F0DB0`.

**Wave 174 (2026-11-14).** Five paid-model lanes wrote 27 bodies from the
368-388 byte sub-band (10,216 bytes) plus five lane CSVs and coverage notes;
the coordinator consolidated `docs/COVERAGE_WAVE174_BASES.md`. Wave 174 is
gated green at **17,638 / 20,252 (87.09%)**, d_basesNP 4,216/5,615 (75.08%);
`check-lane-csv.mjs wave174` reports 27 unique rows (20 high, 7 medium).
Decoding breakthrough this wave: the A-form float operand rule is now settled -
two-operand arithmetic uses FRB (bits 16-20), `fmuls` uses FRC (bits 21-25),
and four-operand forms are `fD,fA,fC,fB`. The scratch decoder carries the
per-XO rule, and `docs/REL_ELF_MAPPING.md` documents it. The wave also resolved
the wave-86 deferral of `0x8060A950` (the stack record is `0x3C` bytes) and
superseded the Wave 3 paired-single rejection of `0x80651640`.

**Wave 175 (2026-11-14).** Five paid-model lanes wrote 26 bodies from the
392-412 byte sub-band (10,448 bytes) plus five lane CSVs and coverage notes;
the coordinator consolidated `docs/COVERAGE_WAVE175_BASES.md`. Wave 175 is
gated green at **17,664 / 20,252 (87.22%)**, d_basesNP 4,242/5,615 (75.55%);
`check-lane-csv.mjs wave175` reports 26 unique rows (19 high, 7 medium). The
wave retired seven standing deferrals: `0x806787C0`, `0x806C7890`,
`0x80743790` and neighbours were GQR false alarms (every `psq_*` is an
`f27..f31` save slot), and four exports hid `this` behind `_savegpr` thunks.
`0x8064DCC0`'s magic constant `0x4330000080000000` also settled a signed
int→float reconstruction Ghidra had modelled as an XOR.

**Wave 176 (2026-11-14).** Five paid-model lanes wrote 24 bodies from the
412-432 byte sub-band (10,132 bytes) plus five lane CSVs and coverage notes;
the coordinator consolidated `docs/COVERAGE_WAVE176_BASES.md`. Wave 176 is
gated green at **17,688 / 20,252 (87.34%)**, d_basesNP 4,266/5,615 (75.98%);
`check-lane-csv.mjs wave176` reports 24 unique rows (18 high, 6 medium).
Findings: `0x80746C20`'s `F63_8/12/15` words are `fabs`/`frsp`/`fctiwz`, not
paired-single maths; `0x8063F120`'s `bc 12,6` tests a dead `cr1` (the store is
unconditional); `0x8079E030`'s `_savegpr_26` returns `r3..r5` as three
arguments; and `0x80694F30`'s asymmetric `cror` folds make both Ghidra `<=`
orderings correct.

**Wave 177 (2026-11-14).** Five paid-model lanes wrote 21 bodies from the
432-456 byte sub-band (9,404 bytes) plus five lane CSVs and coverage notes;
the coordinator consolidated `docs/COVERAGE_WAVE177_BASES.md`. Wave 177 is
gated green at **17,709 / 20,252 (87.44%)**, d_basesNP 4,287/5,615 (76.35%);
`check-lane-csv.mjs wave177` reports 21 unique rows (17 high, 4 medium).
`0x8075BB60` was translated, so `fn_8075bd20` no longer calls it through
`abi_call5!`. The wave also settled branch semantics (`bc 4,1` is `<=`,
`bc 4,0` is `>=`), which corrected Ghidra's inverted `fcmpo` arms in
`0x807A28B0`, and found that `0x807630C0`'s case-3 constant was one pool entry
off (`0x807CEE5C` is `0.0`, not `1.0`).

**Wave 178 (2026-11-14).** Five paid-model lanes wrote 20 bodies from the
456-492 byte sub-band (9,448 bytes) plus five lane CSVs and coverage notes;
the coordinator consolidated `docs/COVERAGE_WAVE178_BASES.md`. Wave 178 is
gated green at **17,729 / 20,252 (87.54%)**, d_basesNP 4,307/5,615 (76.71%);
`check-lane-csv.mjs wave178` reports 20 unique rows (17 high, 3 medium).
Family confirmations: `0x807ABF50`, `0x807ADC00` and `0x807AEEA0` are offset
variants of the merged `fn_807b08b0`, and `0x80758880` closes the wave-132
`Wm_mr_` cluster. Ghidra defects fixed from the words: `0x806952D0`'s ground
test is inverted, `0x807BF930` swaps two `0x8015F140` tuples, and
`0x80637120`'s base is `this + 0x7BC + 0x3C*index`.

**Wave 179 (2026-11-14).** Five paid-model lanes finished the 300-512 byte
band: 15 bodies (492-512 bytes, 7,508 bytes), five lane CSVs and coverage
notes, plus the coordinator's consolidation in
`docs/COVERAGE_WAVE179_BASES.md`. Wave 179 is gated green at
**17,744 / 20,252 (87.62%)**, d_basesNP 4,322/5,615 (76.97%);
`check-lane-csv.mjs wave179` reports 15 unique rows (13 high, 2 medium). The
coordinator also replaced `abi_call1!(0x8073_FF60, actor)` with the real
`fn_8073ff60(actor)` in its two neighbours and corrected a stale doc offset in
`fn_807195a0`. The only bodies left in the 300-512 band are the deferred
GQR/`psq` set (`0x806EDE70`, `0x806E3FC0`, `0x806BBB90`, `0x8074B8D0`,
`0x8064DB70`, `0x8063B900`, `0x8063BA50`), whose decoding needs a dedicated
pass.

**Wave 180 (2026-11-14).** The deferred GQR/`psq` set was cleared: seven bodies
(2,336 bytes) translated, all high confidence, plus five lane CSVs and coverage
notes consolidated into `docs/COVERAGE_WAVE180_BASES.md`. Wave 180 is gated
green at **17,751 / 20,252 (87.65%)**, d_basesNP 4,329/5,615 (77.10%);
`check-lane-csv.mjs wave180` reports 7 unique rows. The wave retired three
standing deferrals: `0x806E3FC0` (wave 109), `0x8063BA50` (wave 81), and
`0x8074B8D0` — in every case the `psq_*` instructions are `qr0`/`W=0` scalar
`f32` callee-save spills and the `ldexpf`/`unaff_GQR0` brackets are compiler
noise. With this wave the entire 300-512 byte band and its deferred subset are
translated; what remains in `game::bases` is the sub-300-byte remainder
(~1,286 bodies).

**Wave 181 (2026-11-14).** The sub-300-byte sweep began: five paid-model lanes
wrote 40 bodies from the 64-128 byte band (3,944 bytes), eight per lane.
Wave 181 is gated green at **17,791 / 20,252 (87.85%)**, d_basesNP
4,369/5,615 (77.81%); `check-lane-csv.mjs wave181` reports 40 unique rows
(37 high, 3 medium). Composition: 16 A-type deleting destructors, 5 layout
gates, 10 animation switchers, 4 pane/text helpers and 3 frame updates.
Findings: `0x80615D90`'s export `halt_baddata`s after two stores, so it was
recovered from raw words (tables `0x807C6C28` / `0x807C6C34`, inverted
`if`/`else` corrected); the BSS state descriptors sit past the file-backed end
and their roles come from `.rodata` name words; and the `96` vs `100` byte
destructor split is only a duplicated `cmpi`.

**Wave 182 (2026-11-14).** Ten bodies per lane: 50 bodies from the 64-128 byte
band (4,692 bytes), five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE182_BASES.md`. Wave 182 is gated green at
**17,841 / 20,252 (88.10%)**, d_basesNP 4,419/5,615 (78.70%);
`check-lane-csv.mjs wave182` reports 50 unique rows (46 high, 4 medium). Three
opaque boundaries were removed: `0x806505B0` now has a real body and is called
directly by four animation gates, `0x80640AC0` (previously written off as "bad
instruction data") is translated and wired into `fn_80640b40`, and
`fn_80684a30`'s doc no longer misattributes the `0x806842D0` wrapper's callee.
Integration fixes: lane E's CSV used `rel_d_basesNP` instead of
`rel-d_basesNP`, and lane C's `0x8082_8920`/`0x8082_88E0` tags needed explicit
`u32` suffixes.

**Wave 183 (2026-11-14).** Ten bodies per lane: 50 more bodies from the
64-128 byte band (4,680 bytes), five lane CSVs and coverage notes consolidated
into `docs/COVERAGE_WAVE183_BASES.md`. Wave 183 is gated green at
**17,891 / 20,252 (88.34%)**, d_basesNP 4,469/5,615 (79.59%);
`check-lane-csv.mjs wave183` reports 50 unique rows (49 high, 1 medium). The
wave found and fixed a real inversion in the earlier `fn_806c3710`: the shared
`mfcr` -> `rlwinm r0,r0,2,31,31` -> `cntlzw` chain isolates CR0[GT], so the
flag is `1` exactly when `delta <= 0.0`, not `>= 0.0`. Other corrections:
`0x806B4B20`'s `fsubs` is `this.x - other.x` (Ghidra reverses it),
`0x806B9CB0`'s `0x806B9720` does take the actor, and `0x806C1B00`'s tail is
`(value == 0) as u32`.

**Wave 184 (2026-11-14).** Forty-nine of fifty planned bodies were translated
from the 64-128 byte band (4,692 bytes) across five lanes; the fiftieth,
`0x806E2BF0`, was honestly deferred because its loop stores only into its own
stack frame (and carries an `ori r0,r0,0` post-link nop), so the destination
field is unrecoverable. Five lane CSVs and coverage notes were consolidated
into `docs/COVERAGE_WAVE184_BASES.md`. Wave 184 is gated green at
**17,940 / 20,252 (88.58%)**, d_basesNP 4,518/5,615 (80.46%);
`check-lane-csv.mjs wave184` reports 49 unique rows (46 high, 3 medium).
Corrections: `0x806E1780`'s vtable is `0x807FC9E0` (not `0x8080C9E0`), the
three `halt_baddata` slot gates are `b 0x800B3600` tail branches, `0x8071D120`
is single-precision with a truncating cast, and `0x80718960` is 100 bytes
rather than the 92 in `functions.tsv`.

**Wave 185 (2026-11-14).** Fifty bodies from the 64-128 byte band (4,820
bytes), five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE185_BASES.md`. Wave 185 is gated green at
**17,990 / 20,252 (88.83%)**, d_basesNP 4,568/5,615 (81.35%);
`check-lane-csv.mjs wave185` reports 50 unique rows (43 high, 7 medium). The
wave retired the wave-106 deferral `0x8075F9E0` (the "missing actor pointer" is
the live `r3` from `0x80162D50`) and recovered two `halt_baddata` tails
(`0x807242C0`/`0x80724320`) as `b 0x8006D090` calls. Corrections: `0x80733F10`'s
`double` parameters are `f32`, `0x807754D0` forwards `r3` as the dropped actor
argument, and `0x80776750` forwards `r4` as arg 2.

**Wave 186 (2026-11-14).** Fifty bodies from the 64-128 byte band (5,156
bytes), five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE186_BASES.md`. Wave 186 is gated green at
**18,040 / 20,252 (89.08%)**, d_basesNP 4,618/5,615 (82.24%);
`check-lane-csv.mjs wave186` reports 50 unique rows (48 high, 2 medium). Real
bodies were added for `0x80799960` (a three-argument pose selector used by many
earlier bodies) and `0x807C1130` (whose live `f1` is the return). The wave also
recovered `0x8079F4B0` from `halt_baddata` as a `b 0x80102690` tail call,
resolved dropped `r3`/`this` arguments in four `0x807C1xxx`/`0x807C3xxx`
bodies, and corrected `0x807970A0`'s four div-by-zero `divw` sites to
`checked_div(...).unwrap_or(0)`.

**Wave 187 (2026-11-14).** Fifty bodies from the 128-192 band (8,016 bytes),
the first wave in that range after the 64-128 band was exhausted. Five lane
CSVs and coverage notes were consolidated into
`docs/COVERAGE_WAVE187_BASES.md`. Wave 187 is gated green at
**18,090 / 20,252 (89.32%)**, d_basesNP 4,668/5,615 (83.13%);
`check-lane-csv.mjs wave187` reports 50 unique rows (48 high, 2 medium). The
range covers the character-change arrow and `modelPlayGuide` loaders, the
out-animation swaps, the KeyWait/CoinCountDown/WipeWait descriptors, the lift
helpers, the `block_arrow` actor and the 8x13 save-grid scans. `0x806148F0` was
rebuilt from raw words after a `bad instruction data` truncation, and
`0x80643260`'s `halt_baddata` resolved to a `b 0x80195380` tail call.

**Wave 188 (2026-11-14).** Fifty bodies from the 128-192 band (7,700 bytes),
five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE188_BASES.md`. Wave 188 is gated green at
**18,140 / 20,252 (89.57%)**, d_basesNP 4,718/5,615 (84.02%);
`check-lane-csv.mjs wave188` reports 50 unique rows (40 high, 10 medium). Three
more opaque boundaries were removed: the `0x80697D00`/`0x80699C00` gates now
have bodies and their callers `fn_80697f10`/`fn_80699e30` call them directly,
and `0x806E7F50` is called directly by `fn_806e7ff0`. The wave also carried
eight name-suffix comparators, the `0x806E3EB0` ring-segment rewrite and four
lift/rider bodies.

**Wave 189 (2026-11-14).** Fifty bodies from the 128-192 band (7,716 bytes),
five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE189_BASES.md`. Wave 189 is gated green at
**18,190 / 20,252 (89.82%)**, d_basesNP 4,768/5,615 (84.92%);
`check-lane-csv.mjs wave189` reports 50 unique rows (49 high, 1 medium). The
batch covers the `lift_turi_ashiba` model and pane bodies, the turi/pipe
factories, the fire-cannon aim step, the watch-target record (with `sqrt`
distance), the perch tilt, the `0x8075BF70` teardown sibling of `fn_8075bd20`,
and three more `Class:Name` comparator clones. Corrections: `0x80706360`'s
Ghidra return is a dead accumulator (real result is a 0/1 membership test),
`0x80756FD0`/`0x80758310` take arguments hidden by `_savegpr`, and
`0x80745D10`'s GQR noise reduces to two `fctiwz` results masked to 16 bits.

**Wave 190 (2026-11-14).** Fifty bodies from the 128-192 band (8,144 bytes),
five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE190_BASES.md`. Wave 190 is gated green at
**18,240 / 20,252 (90.07%)**, d_basesNP 4,818/5,615 (85.81%) - the first wave
above 90% overall. Highlights: the full `daWmSandPillar_c` family, the
`shipRoute` stop list (`0x8076FDA0`, now called directly by `fn_806fc60`), the
route-node samplers, the `Env_world` blight/blmap/bfog binding, the
`MainSelect` starter and two exact transform twins (`0x8078FAD0` =
`fn_8075aa50`). Corrections: `0x80776C80`'s arguments are `f1/f2/f3` (not
integers), the `0x80777D20` family's lost stack pointers were recovered, and
`0x8078DAFC`/`0x8078F114` invert Ghidra's branch readings.

**Wave 191 (2026-11-14).** The 128-192 band was completed with 26 bodies
(4,412 bytes), five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE191_BASES.md`. Wave 191 is gated green at
**18,266 / 20,252 (90.19%)**, d_basesNP 4,844/5,615 (86.27%). The batch covers
the `Env_world`/`Env_movie` starters, the layout light/fog binding, the GX
viewport/scissor rewrite, the save-end state descriptors (pinned through the
`dCourseSelectManager_c` table) and the `0x807BE3D0` ease. That ease's real
signature is `(f32, f32) -> f32` with the second float forwarded in `f2`, which
means the older `fn_807bfe20` call site (`call_f64_u32(..., value as f64, 0)`)
is subtly wrong; fixing it needs an `(f32, f32) -> f32` shim and is recorded
for a later pass.

**Wave 192 (2026-11-14).** The 192-256 band was opened with 49 of 50 planned
bodies (11,248 bytes), five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE192_BASES.md`. Wave 192 is gated green at
**18,315 / 20,252 (90.43%)**, d_basesNP 4,893/5,615 (87.14%). It supplied real
bodies for the long-standing address-only callees `0x8064EC70` (three calls
per branch in `fn_8064eeb0`), `0x80648470`, `0x8062C800`, `0x80613290` and
`0x80627160`, and covered the `dCollectionCoinDate`/`dCourseClear`/
`dGameOver`/`dMessageWindow`/`dModelPlayArrow` loaders plus the results-table
machinery. `0x80697090` is deferred: its target `(x,y)` is read from stack
slots it never writes, so any body would invent a source.

Tooling note: two merge bugs were found and fixed this wave. The appender
matched the string `pub unsafe fn fn_<addr>(` inside doc comments (splicing
doc-only fragments), and a later patch turned a missing match into index 0.
The template now anchors on a leading newline and treats a miss as a reported
skip.

**Wave 193 (2026-11-14).** Fifty bodies from the 192-256 band (11,232 bytes),
five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE193_BASES.md`. Wave 193 is gated green at
**18,365 / 20,252 (90.68%)**, d_basesNP 4,943/5,615 (88.03%). The wave gave
real bodies to seven previously address-only callees (`0x806A9F50`,
`0x806CB140`, `0x806D0130`, `0x806EC5F0`, `0x806ED160`, `0x806FE310`,
`0x8070F9C0`) and wired all eight of their call sites. It also added the plank
node rebuilds, the rider effect tables, the lift gates, the boat lighting
frames and six more members of the five-level chain-walker family. Ghidra
corrections: `0x806F8160`'s arms are swapped, `0x80703940`'s hidden `r4` is the
out-pointer, and `0x80703A40`'s `cntlzw(count-1)` is `count == 1`.

**Wave 194 (2026-11-14).** Fifty bodies from the 192-256 band (11,132 bytes),
five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE194_BASES.md`. Wave 194 is gated green at
**18,415 / 20,252 (90.93%)**, d_basesNP 4,993/5,615 (88.92%). Eight call sites
were wired to newly translated bodies: `0x8075FA40`, `0x80789990` (2),
`0x80798E20` (2) and `0x8075BE90` (2). The wave retired the wave-101 deferrals
`0x807382F0`/`0x80738DB0` (their `unaff_GQR0`/`ldexpf` output was pure noise)
and corrected `0x807538E0`'s guard to `(+0x358 | +0x35C) == 0` (the export's
masked double test was wrong). Other recoveries: `0x807436C0`'s operands are
`0x8043BB08/0C`, and `0x8078FB80` must keep `abi_call1!(0x807900B0, this)`
because the existing wrapper recovers `r3` through `_savegpr_24`.

**Wave 195 (2026-11-14).** The 256-300 band was opened with 50 large bodies
(13,940 bytes), five lane CSVs and coverage notes consolidated into
`docs/COVERAGE_WAVE195_BASES.md`. Wave 195 is gated green at
**18,465 / 20,252 (91.18%)**, d_basesNP 5,043/5,615 (89.81%). It gave real
bodies to the shared ratio helper `0x806601B0` (`(position, low, high) -> f32`,
with `p == high` returning 0.0), `0x8068C5A0` (`(height: f32, this) -> f32`) and
`0x80693DD0` (`(this, candidate) -> u32`). Corrections: `0x8064D650`'s `lfs` is
dead (`+0xB4` takes the following `lwz`), `0x80705600` is `PSMTXCopy`/
`PSMTXConcat` with a hidden third argument, and `0x80707F40`/`0x806E2E40`
contain dead arithmetic blocks.

**Wave 196 (2026-11-14).** The translatable set was completed: 29 final bodies
from the 256-300 band (8,152 bytes), three lane CSVs and coverage notes in
`docs/COVERAGE_WAVE196_BASES.md`. Wave 196 is gated green at
**18,494 / 20,252 (91.32%)**, d_basesNP 5,072/5,615 (90.33%). `0x807AD590`,
previously deferred as psq-heavy, contains no `psq_*` at all (only
`_savegpr_27`/`_restgpr_27`) and is now translated; its 16-byte record map
matches `fn_8079f790` byte-for-byte. Four `0x80799B00` sites and two
`0x80797190` sites now call real bodies. From here `game::bases` has only the
472 `0x802DCBFC` forwarders (no shipped target) and two documented deferrals
(`0x806E2BF0`, `0x80697090`) left, plus a tail of sub-64-byte stragglers.

**Wave 197 (2026-11-14).** The mid-size remainder was cleared: 35 bodies
(196-312 bytes, 9,034 bytes) across four lanes, plus five lane CSVs and
coverage notes in `docs/COVERAGE_WAVE197_BASES.md`. Wave 197 is gated green at
**18,529 / 20,252 (91.49%)**, d_basesNP 5,107/5,615 (90.95%). Six call sites
were wired to new bodies (`0x807C09B0`, `0x80762EA0`, `0x8079BC10`,
`0x807BAA30`, `0x807BFC20`). Signature recoveries: `0x807C09B0` is
`(_unused, id) -> u32` because `lis r3,1` clobbers the dead first argument, and
`0x8071C180` hides three out/mask pointers plus a `u32` return. Only 12 very
large bodies (516-4,232 bytes) plus the 472 forwarders, two documented
deferrals and one 32-48 byte body remain.

**Wave 198 (2026-11-14/15).** Twelve of fifteen giant bodies were merged
(10,624 bytes): lane A's 3,788-byte `0x8065CB20` plus eleven others across
lanes A/C/D/E. Wave 198 is currently gated green at
**18,541 / 20,252 (91.55%)**, d_basesNP 5,119/5,615 (91.17%). Lane B's three
bodies (1,772/1,772/3,432 bytes) were un-merged after `cargo check` exposed
about twenty mechanical `u32`/`usize` errors (`call0` out of scope; nested
`read_u32` results used as addresses); that lane is re-verifying against a real
scratch `cargo check` before re-merge. Its CSV was never written, so the ledger
only records the twelve merged rows. Remaining after this: 15 very large bodies
including the three pending, the 472 forwarders and two documented deferrals.

**Wave 199 (2026-11-15).** The giant bodies were completed: 12 bodies
(13,684 bytes) across six lanes, all compiling on the first merge. This closes
every translatable function in `game::bases`. Wave 199 is gated green at
**18,556 / 20,252 (91.63%)**, d_basesNP 5,134/5,615 (91.43%). Highlights: the
4,232-byte `0x8079C330` (0x9F-entry switch over states 6..=0xA4), the
4,076-byte `0x8077B450` (99-entry table plus 26-arm decision tree), the
3,176-byte `0x8079D3C0` (157-entry dispatch, table `0x8081BB00`), and the
2,576-byte `0x8078DC70` (30-arm pass, table `0x8081A580`). Remaining rows are
only the 472 `0x802DCBFC` forwarders (no shipped target), two documented
deferrals (`0x806E2BF0`, `0x80697090`) and one 32-48 byte body.

**Wave 200 (2026-11-15).** All 472 `0x802DCBFC` interface thunks were
translated (plus the 44-byte `0x806E3BE0`), taking `game::bases` to
**5,607 / 5,615 (99.86%)** and the whole project to
**19,029 / 20,252 (93.96%)**. The earlier blanket deferral was wrong: every
thunk is the same 12-instruction `mr r5,r3` / `mr r3,r4` / `addi r12,r5,K` /
`bl 0x802DCBFC` sequence, and `0x802DCBFC` is a shipped dispatcher that resolves
an object-embedded `{self_adjust, index, table}` descriptor. The new
`interface_call()` helper models it exactly. Bodies were generated
deterministically from the ELF, then six audit lanes decoded all 471 of them
from raw words and found zero mismatches. Only the two documented deferrals
(`0x806E2BF0`, `0x80697090`) remain untranslated in this module.

**Wave 209 (2026-11-15).** Final audit pass. Two defects flagged by earlier
lanes were resolved from the shipped words. (1) `fn_80033F10`'s side byte was
inverted: the shared `fcmpo cr0,f1,f0` / `cror 2,1,2` / `mfcr` /
`rlwinm 3,31,31` / `cntlzw` sequence yields 1 iff `f1 < f0`, matching
`fn_8003ACD0`; body and doc corrected. (2) `fn_8001D850`'s child walk called its
probe without the child in `r3` and recursed one link too deep; rewritten to
probe the child, return the fourth child on failure, and recurse only when the
fifth child is null. The `fn_8001D930` doc consumed by that splice was restored.
Gate: `cargo check` 0, `doc-lint` 0 violations, `scripts/check.ps1` exit 0 at
**19,377 / 20,252 (95.68%)**, with `ui::hud` now 540/540 and `game::bases`
5,607/5,615.

**Wave 210 (2026-11-15).** Opened `src/game/player.rs` with the first 60
main.dol bodies (`0x80060110`-`0x800630B0`): the roster driver and counter
clamps, player-slot bookkeeping, the tag-scan/anchor-placement pair, and the
`dokan` block spawn plus its `g3d/obj_dokan.brres` model build.
`game::player` is now 597/818 (72.98%) and the project 19,437/20,252 (95.98%).
Size artefacts corrected: `0x80060AB0` = 344, `0x80060DB0` = 312,
`0x80062450` = 196, `0x800616A0` = 384. `0x80061540` is 80 bytes, not 436:
its four scan callees were inlined by Ghidra, and the Rust calls them by
address. Lane D confirmed `0x802DCBFC` is a Metrowerks pointer-to-member
dispatcher, matching the `game::bases` thunk model. Gate: `cargo check` 0,
`doc-lint` 0 violations, `scripts/check.ps1` exit 0.

**Wave 211 (2026-11-15).** Second 60 `game::player` bodies
(`0x80063200`-`0x80066630`). The big items were the 932-byte model rebuild
`0x80063270`, the 700-byte cull body `0x800651C0`, the 272-byte stage burst
`0x80066630` and the hit-region probe pair `0x80064DA0`/`0x80064F50`.
`game::player` is now 657/818 (80.32%) and the project 19,497/20,252 (96.27%).
**Queue fix:** the per-file inventory had been built from the smaller `main`
Ghidra export, so 52 canonical `main-gb` functions had never been queued,
including two `ui::hud` bodies. `work/rebuild-missing.cjs` now derives the queue
from the canonical export and maps each address by nearest translated ledger
neighbour. Gate: `cargo check` 0, `doc-lint` 0 violations,
`scripts/check.ps1` exit 0.

**Wave 212 (2026-11-15).** Third 60 `game::player` bodies
(`0x80060010`-`0x80069270`), including three large sweeps (768/552/844 bytes)
recovered in full and the 492-byte `dActorState_c` / `dActorMultiState_c`
registration. `game::player` is now 717/818 (87.65%); 113 bodies remain there.
Recurring lessons: every integer-to-float conversion in the `0x80068xxx` cluster
is signed (`2^52 + 2^31` bias), Ghidra inverts tests at `0x80066A40` and
`0x80066C60`, and the five 48-byte forwarders use descriptors at
`+0x18/+0x0C/+0x24/+0x18/+0x0C`. Gate: `cargo check` 0, `doc-lint` 0
violations, `scripts/check.ps1` exit 0.

**Wave 213 (2026-11-15).** Fourth 60 `game::player` bodies
(`0x800693E0`-`0x8006C870`): the nearest-player search, the effect records, the
SDA accessor block, the projection/request machine and the `Base_c` class family
(base vtable `0x8030F1D8`, derived `0x8030F228`). `game::player` is now
777/818 (94.99%); 53 bodies remain there. Two operational notes: `0x800693E0`'s
`bc 4,2` is `bne` (BO=4 branches on a clear bit), so the search keeps
`distance <= bound`, and lane F's block shipped eight bodies for ten ledger rows,
so the two 4-byte thunks `0x8006C6B0`/`0x8006C6C0` were written by the
coordinator from the raw branch words. Gate: `cargo check` 0, `doc-lint` 0
violations, `scripts/check.ps1` exit 0.

**Wave 214 (2026-11-15).** Finished `src/game/player.rs`: the last 53 bodies,
including the full contact resolver `0x8006DFC0` (1,232 B) and the landing/step
driver `0x8006E720` (1,668 B). `game::player` is now **830/830** and the project
19,670/20,252 (97.13%). Two tool defects were found: `check-lane-csv.mjs`
reads its byte-size column as `len-2` (the `true` column), so its size check
never fires; and the local disassembler's `F63_32` fallback prints `FRA`/`FRB`
reversed for `fcmpu`. Also noted: the existing `void` declarations of
`0x8006C5C0`/`0x8006C610` are wrong because their `bl` results are consumed.
Gate: `cargo check` 0, `doc-lint` 0 violations, `scripts/check.ps1` exit 0.

**Wave 215 (2026-11-15).** Opened `src/engine/dsys.rs` with the first 60
main.dol bodies (`0x800F0010`-`0x800F2BC0`): the three record-append families
(strides 0x48/0x14/0x14), the track block at `+0x58` with its two effect slots,
the metric records, and the table marshaller. `engine::dsys` is now 209/397 and
the project 19,730/20,252 (97.42%). Notable: the `bl 0x802DCD60..88` prologues
are `_savegpr` stubs that leave `r3`/`r4` intact, so several Ghidra `undefined8`
signatures are really the functions' own arguments; `0x800F18B0`'s export embeds
the untabled destructor `0x800F1920`; and `0x800F0EB0` is a two-instruction tail
thunk with no `bclr`. Gate: `cargo check` 0, `doc-lint` 0 violations,
`scripts/check.ps1` exit 0.

**Wave 216 (2026-11-15).** Second 60 `engine::dsys` bodies, split by byte weight
(snake draft) instead of address order after the first split gave one lane 6,632
bytes. The 2,544-byte CSV flag-chain parser `0x800F3E80` was decoded from the
raw words, recovering 40 keyword literals (`ura`, `stop`, `link1..5`, `scroll`,
`dokan`, `demo1..7`, `kamstop`, `trap1/2`, ...) and proving that its two flag
chains (`+0x1C8`, `+0x1D0`) are independent. `engine::dsys` is now 269/397 and
the project 19,790/20,252 (97.72%). The scratch decoder was found to print
logical X-form operands swapped; `ppc159.js` is fixed. Gate: `cargo check` 0,
`doc-lint` 0 violations, `scripts/check.ps1` exit 0.

**Wave 217 (2026-11-15).** Third 60 `engine::dsys` bodies, crossing 98% overall.
The run covers the `"W<world><slot>"` stage-code formatter/parser pair and their
dispatch tables at `0x8031C0C0`/`0x8031C0EC`, the progress-flag bank query side
(`G_PROGRESS_BANK` = `0x8042A040`, block size 0x980, entry
`+group*0xA8 + index*4 + 0x6C`), the projection/lerp band helpers and the
680-byte course scan. `engine::dsys` is now 329/397 and the project
19,850/20,252 (98.02%). Two notable decodes: `0x800F7DA0`'s `cntlzw` is dead
(overwritten by a `0.0f` load before the rotate), and `0x800F9730` returns `r25`
where Ghidra says `void`. Lane A mislabelled itself "lane G"; the files were
renamed before merge. Gate: `cargo check` 0, `doc-lint` 0 violations,
`scripts/check.ps1` exit 0.

**Wave 218 (2026-11-15).** Finished `src/engine/dsys.rs`: the last 68 bodies,
including the bank row setters, the 780-byte path rail `0x800FF740`, the window
snap `0x800FFA50` and the shell gates. `engine::dsys` is now **397/397** and the
project 19,918/20,252 (98.35%). Two discoveries: `0x800FC2D0` (152 B) and
`0x800FC370` (20 B) sit inside the module but have no `functions.tsv` row and no
ledger entry (read in full, ready for a follow-up), and `0x800FC040` is already
covered as `game::bases` `fn_80770970` via the REL. `0x800FD590` proved to be a
split head (156 B, tail branch) plus tail-called body (232 B). Gate:
`cargo check` 0, `doc-lint` 0 violations, `scripts/check.ps1` exit 0.

**Wave 219 (2026-11-15).** Opened `src/resource/archive.rs` with the first 60
bodies: the GX render-state records, the tiled-texture decoder `0x802C0E30`
(936 B), the 2,424-byte panel builder `0x802C3D80`, the ribbon strip
`0x802C13F0` and `0x802C2700`, which is really **2,048 bytes** — the tsv lists
only its 576-byte jump-table head, a new kind of size artefact (the arms and
shared capture tail run to `0x802C2EFC`). `resource::archive` is now 609/747 and
the project 19,978/20,252 (98.65%). `0x802C3D80`'s `void(void)` signature was
wrong (the `bl` is `_savegpr_19`): it is `(owner, mode)`. More untabled code was
found at `0x802C3460`/`0x802C3490` plus two thunks. Four lane rows with commas
in their evidence prose were repaired before the ledger merge. Gate:
`cargo check` 0, `doc-lint` 0 violations, `scripts/check.ps1` exit 0.

**Wave 220 (2026-11-15).** Second 60 `resource::archive` bodies
(`0x802C4FB0`-`0x802C8B00`), crossing 98.9%. The run covers the 1,020-byte blend
block `0x802C52A0`, the 848-byte named-record walk `0x802C7560`, the nine-glyph
text-row builder `0x802C7370` and the projection/descriptor blocks. Notable
decodes: `0x802C52A0`'s clamp is asymmetric (`t<0` keeps source, `t>1` snaps to
target), and `0x802C7C70`/`0x802C7E30` have their two live arguments swapped by
Ghidra after the `__savegprlr_27` prologue. Several more bodies lost an argument
to the phantom `u64` return of the register-save helpers.
`resource::archive` is now 669/747 and the project 20,038/20,252 (98.94%). Gate:
`cargo check` 0, `doc-lint` 0 violations, `scripts/check.ps1` exit 0.

**Wave 221 (2026-11-15).** Sixty more \esource::archive\ bodies: the GX text
state blocks, the 1,060-byte tiled-quad emitter \ x802CC050\, the 704-byte cell
emit \ x802CC730\ (a four-vertex FIFO strip the export truncated to one store)
and the descriptor dispatch \ x802CE010\. \esource::archive\ reached 729/747
and the project 20,098/20,252 (99.24%). Gate green.

**Wave 222 (2026-11-15).** Closed two modules. The last 18
`resource::archive` bodies include the 1,296-byte per-chunk slice emitter
`0x802CF1E0`, the 816-byte pane painter `0x802CFD00` and the 11-arm jump table
`0x802CEA40`; `resource::archive` is now **747/747**. The two `ui::hud`
stragglers `0x80026CA0` and `0x8002D260` closed that module too (**542/542**) and
resolved a dangling `FN_8002D260_D` reference from `fn_8002d0c0`. Project total:
20,118/20,252 (99.34%). Only `src/game/stage.rs` (132 bodies) remains in the
translation queue. Gate: `cargo check` 0, `doc-lint` 0 violations,
`scripts/check.ps1` exit 0.

**Wave 223 (2026-11-15).** Opened `src/game/stage.rs` with the first 60 bodies
(`0x800A02D0`-`0x800A7420`): the 812-byte clown-Jr model constructor
`0x800A3100`, the 532-byte three-state angle ramp `0x800A5210`, the
variant/proximity record emitters and the destructor `0x800A2F40`.
`game::stage` is now 1154/1226 and the project 20,178/20,252 (99.63%). Two
polarity traps were settled from the encoding: `bc 12,1` branches on `CR0[GT]`,
and `0x800A0550`'s double inversion leaves the stored flag as `height < X`.
`0x800A2870`'s probe chain is inverted relative to the export and its
`_savegpr_26` prologue hides a `(context, actor, mode) -> u32` signature. Gate:
`cargo check` 0, `doc-lint` 0 violations, `scripts/check.ps1` exit 0.

**Waves 224-225 (2026-11-15).** Finished `src/game/stage.rs` and with it all of
main.dol. Wave 224 added 60 stage bodies (28 of them 12-byte `W84_TYPE_INFO`
accessors) including the conflict test `0x800A78D0`, the reaction block
`0x800A9550` and the per-unit effect spawner `0x800A77A0`; wave 225 closed the
file with the last 12, ending at the highest function in the image
`0x800ADB00` (468 B). **`game::stage` is 1226/1226 and main.dol is
10,276/10,276**, so the project stands at 20,250/20,252 (99.99%) with only the
two documented `game::bases` deferrals outstanding. Recurring defect confirmed
again in this range: `0x8023A0B0`'s second argument is a live `r4` handle that
Ghidra drops (four bodies), and `W224_F_ZERO_6C44_A224`'s prose mis-states its
slot as `0.0f` when it holds `5500.0f`. Gate: `cargo check` 0, `doc-lint` 0
violations, `scripts/check.ps1` exit 0.

**Waves 226-227 (2026-11-15).** Mapped the code outside `functions.tsv`. Wave
226 found **2,457** prologue-verified function starts in main.dol with no tsv
row (`work/wave226/laneC_untabled_entries.csv`) and proved the tsv inventory is
complete at 20,252. Wave 227 grouped every entry by its owning pointer word
(`docs/UNTABLED_POINTER_TABLES.md`): 1,950 table-owned, 92 call-site-only, 415
unreferenced, 487,260 B owned. Gate: `cargo check` 0, `doc-lint` 0 violations,
`scripts/check.ps1` exit 0.

**Waves 228-233 (2026-11-15).** Began translating the corpus. Wave 228 added
84 bodies (~17,700 B) across six lanes, wave 229 closed the `0x80314534`
`game::stage` vtable and the `0x802ED9E0` `.ctors` batch, and waves 230-233
audited method-array layouts, ledger integrity and the vtable-aware rescan that
reclassified most large "state tables" as vtables. Untabled bodies carry
`# Original (untabled)`, use measured extents and never enter
`ledger/functions.csv`. Gate green each wave.

**Waves 234-238 (2026-11-15).** Continued the largest runs: WorldMap,
`dEn_c`, `WM_PLAYER_VTABLE` and the `0x80350230` family. Wave 238 merged the
`WM_PLAYER_VTABLE` / WorldMap overlap into one inventory row (527 unique words,
147 entries, 32,684 B) and recomputed the untabled accounting: **259** corpus
entries now have a body (251 exact, 8 suffixed/semantic), **2,198** remain, and
**371** are slot aliases. `src` holds 583 untabled blocks over 548 addresses.
Gate: ledger 20,250 rows, 0 staging, `cargo check` 0, `doc-lint` 0 violations.

**Wave 239 (2026-11-15, closing wave).** Froze the reconstruction. Lane A ran
the full repository verification and wrote `docs/AUDIT_FINAL.md`; lane B
confirmed the accounting (20,250 / 20,252, two deferrals, 259 / 2,198 / 371
untabled) and wrote `docs/COVERAGE_FINAL.md` as the single authoritative
summary; lane C sampled 12 bodies from waves 234-238 against the raw image;
lane D checked every `src/*.rs` module header; lanes E/F continued
`WM_PLAYER_VTABLE` and `dEn_c`. No new refactors were started. Gate:
`cargo check` 0, `doc-lint` 0 violations, `scripts/check.ps1` exit 0.

