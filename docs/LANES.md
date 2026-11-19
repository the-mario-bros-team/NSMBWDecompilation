# Parallel lanes

## Current wave status

- **Wave 4 (complete):** 295 functions across 12 lanes; **1,000 / 20,252**
  (4.94%), 0 duplicate addresses, gates green.
- **Wave 5 (complete):** 361 new translations plus 30 backfilled runtime rows
  across 14 lanes; **1,391 / 20,252 (6.87%)**, gates green. One duplicate
  (`0x801AB640`) was resolved in favour of `runtime::os::cancel_alarm`.
- **Wave 6 (complete):** Bluetooth, Metrowerks CRT/`cpp`, player, stage, GX,
  DVD, input, `d_basesNP`, `d_enemiesNP`, `d_en_bossNP`.
- **Waves 7-40 (complete):** continued REL passes and main.dol bands. Wave 35
  work was largely lost to free-model quota exhaustion; Wave 36 added
  main-loop/archive/worldmap/dvd; Wave 37 died on GLM quota; Wave 38 (crouton)
  cut across bases/cpp/display/os/registry/seq/stage; Wave 39 (crouton, after
  deepseek quota died mid-launch) added registry 13, archive 25, dvd 30,
  stage 13, player 30; Wave 40 (three lanes per the 2026-11-08 directive)
  added registry 27, dvd 28 and archive 18 after a quota retry; Wave 41
  (three lanes) added registry 30, stage 28 and archive 28; Wave 42's three
  lanes hit the free limit immediately, but the bt lane had already landed
  18 bodies, which were salvaged; Wave 44 (manual, quota still exhausted)
  added 56 registry leaf, lookup and virtual-dispatch wrappers in four
  batches. Wave 45 (paid `opencode-go/deepseek-v4.1-flash`) added stage 26,
  registry 28 and seq 31; Wave 46 added archive 28, enemies 29 and dvd 30;
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
  30 and bt 29; Wave 70 (four lanes) added bases 29, enemies 30, stage 27 and
  bres 30; Wave 71 (four lanes) added bases 30, enemies 26, stage 29 and
  os 29; Wave 72 (four lanes) added bases 27, enemies 27, dvd 30 and player
  30; Wave 73 (four lanes) added bases 27, enemies 28, worldmap 28 and
  archive 30; Wave 74 (four lanes) added bases 30, enemies 29, input 28 and
  player 26; Wave 75 (four lanes) added bases 26, enemies 30, stage 27 and
  dsys 30; Wave 76 (four lanes) added bases 29, enemies 30, display 30 and
  worldmap 28; Wave 77 (four lanes) added bases 30, enemies 30, cpp 26 and
  worldmap 30; Wave 78 (four lanes) added bases 30, enemies 27, display 28
  and registry 25; Wave 79 (four lanes) added bases 30, enemies 30, bt 26
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
  Wave 94 (four-lane transition) added bases 29, enemies 28, stage 28 and
  worldmap 28; Wave 95 (ten lanes) added bases 30, enemies 27, worldmap 27,
  seq 26, display 26, bres 22, dvd 29, gx_band 28, archive 25 and main_loop
  25; Wave 96 (ten lanes) added bases 30, enemies 30, worldmap 29, hud 30,
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
  Wave 104 (ten lanes) added bases 27, enemies 30, worldmap 21, stage 14,
  gx_band 7 live, plus 14 os, 19 bt, 18 ax, 13 seq and 5 main_loop salvaged
  after the paid model hit its usage limit mid-wave; Wave 105 (ten lanes)
  added geometry 8, bases 29, enemies 28, stage 17, display 7, gx_band 5,
  archive 15, ax 7, main_loop 4 and dvd 4; Wave 106 (ten lanes) added bases
  28, enemies 30, worldmap 14 (one withdrawn as GQR-only), stage 11, display
  7, player 11, input 6, bres 5, main_loop 7 and dvd 8; Wave 107 (ten lanes)
  added bases 30, enemies 27, worldmap 16, stage 6, gx_band 3, bt 7, os 8,
  bosses 8, hud 6 and dsys 4; Wave 108 (ten lanes) added bases 29, enemies
  28, worldmap 10, stage 11, gx_band 1, display 10, menu 6, ax 1, bt 5 and
  os 4; Wave 109 (ten lanes) added bases 30, enemies 26, gx_band 2, stage 7,
  display 9, player 6, ax 6, dvd 4, seq 4 and bres 2; Wave 110 (ten lanes)
  added bases 30, enemies 29, gx_band 3, stage 3, worldmap 3, geometry 4,
  registry 4, os 4, input 2 and dsys 3; Wave 111 (ten lanes) added bases 25,
  enemies 29, gx_band 2, stage 2, worldmap 3, lyt 3, bres 1, os 2,
  main_loop 1 and dvd 1; Wave 112 (ten lanes) added bases 30, enemies 29,
  gx_band 2, stage 3, worldmap 4, input 2, bt 2, bres 1, archive 2 and seq 2;
  Wave 113 (ten lanes) added bases 29, enemies 27, gx_band 2, stage 4,
  worldmap 4, input 2, bt 1, main_loop 2, bosses 2 and hud 1; Wave 114 (ten
  lanes) added bases 29, enemies 27, gx_band 3, stage 2, worldmap 3, bosses
  2, display 1, menu 1, ax 1 and dvd 1; Wave 115 (ten lanes) added bases 26,
  enemies 30, gx_band 2, stage 2, worldmap 1, display 1, dvd 1, bt 1,
  bres 1 and os 1; Wave 116 (seven lanes) added bases 29, enemies 28,
  gx_band 1, stage 1, worldmap 1, bt 1 and os 1; Wave 117 (six lanes) added
  bases 27, enemies 28, gx_band 2, stage 1, archive 1 and bosses 1; Wave 118
  (five lanes) added bases 30, enemies 30, gx_band 1, stage 2 and bosses 1;
  Wave 119 (four lanes) added bases 26, enemies 27, gx_band 2 and stage 2;
  Wave 120 (four lanes) added bases 28, enemies 30, gx_band 1 and stage 1;
  Wave 121 (four lanes) added bases 30, enemies 28, gx_band 2 and stage 1;
  Wave 122 (three lanes) added bases 30, enemies 28 and gx_band 1; Wave 123
  (four lanes) added bases 30, enemies 28, gx_band 1 and stage 1; Wave 124
  (three lanes) added bases 30, enemies 28 and gx_band 1; Wave 125
  (three lanes) added bases 30, enemies 27 and gx_band 1; Wave 126 two
  translation lanes added bases 30 and enemies 29, with two audits finding
  zero unledgered bodies; Wave 127 added bases 28 and enemies 30, with two
  reclassification reports; Wave 128 added bases 30 and enemies 29, with two
  implementation-ready specs; Wave 129 added bases 25 and enemies 33, with
  two more specs; Wave 130 added bases 22 and enemies 28, with two more
  specs; Wave 131 added bases 26 and enemies 26, with two more specs; Wave
  132 added bases 28 and enemies 27, with two more specs; Wave 133 added
  bases 26 and enemies 28, with two more specs; Wave 134 added bases 25 and
  enemies 24, with two more specs; Wave 135 added bases 25 and enemies 29,
  with two more specs; Wave 136 added bases 28 and enemies 26, with two more
  specs; Wave 137 added bases 27 and enemies 25, with two more specs; Wave
  138 added bases 26 and enemies 28, with two more specs (GQR3 pinned);
  Wave 139 added bases 30 and enemies 25, with two more specs; Wave 140
  added bases 27 and enemies 30, with two more specs; Wave 141 added bases
  11 and enemies 17 (large bodies), with two more specs; Wave 142 added
  bases 17 and enemies 17 (large bodies), with two more specs; Wave 143
  added bases 10 and enemies 18, with two more specs; Wave 144 added bases
  16 and enemies 17, with two more specs; Wave 145 added bases 17 and
  enemies 15, with two more specs; Wave 146 added bases 13 and enemies 17,
  with two more specs; Wave 147 added bases 16 and enemies 15, with two more
  specs; Wave 148 added bases 16 and enemies 16, with two more specs; Wave
  149 added bases 16 and enemies 16, with two more specs; Wave 150 added
  bases 17 and enemies 20, with two more specs; Wave 151 added bases 15 and
  enemies 17, with two more specs; Wave 152 added bases 10 and enemies 18,
  with two more specs; Wave 153 added bases 10 and enemies 19, with two more
  specs; Wave 155 added bases 11 and enemies 17, with two more specs; Wave
  156 added bases 17 and enemies 5, plus an enemies audit and a bases spec.
  Wave 157 added bases 16 and the last three enemies bodies. Wave 158 added
  bases 13 plus a bases backlog reclassification, a bases spec and a full
  ledger audit. Wave 159 added bases 39 from the ranked small-body clusters,
  run directly by the coordinator while the paid model was at its weekly limit.
  Wave 160 added bases 33 from the next ranked clusters and quick singles, and
  Wave 161 added bases 55 constant getters, Wave 162 added bases 29 short
  singles, Wave 163 added bases 35 from the 4-36 byte band, Wave 164 added the
  last ten 36-40 byte singles, and Wave 165 added bases 39 from the 44-64 byte
  band (21 destructors, 8 release dispatchers, 10 allocator factories) and
  Wave 166 added the other 28 from the 44-64 byte band (10 virtual-slot
  wrappers, 2 destructors, 4 vtable installers, 12 one-offs), and Wave 167
  added the last 18 (float/state predicates, copy loops, descriptor
  dispatchers and the state-retire twins), completing the band.
  `d_en_bossNP` is complete at 770/770; `d_enemiesNP` is now complete at
  3,555/3,555. Current measured total **17,537 / 20,252 (86.59%)**, 0
  duplicate addresses, gates green.
- **Wave 168 supply:** `game::bases` only. The 44-64 byte band is finished;
  continue with the 300-512 byte spec band and the ranked clusters in
  `docs/ANALYSIS_WAVE158_BASES.md` (rank 21 onward and the quick singles), then
  the class-P pinning and clone batches once those reports exist. The
  516-540 byte eight in `docs/SPEC_WAVE158_BASES.md` are still queued.
  Model policy (2026-11-09): use the paid
  `opencode-go/deepseek-v4.1-flash`; never `iceman-contributor-free`.
- **Wave 169 supply:** `game::bases` only. The first 15 bodies of the 300-512
  byte band are done (Waves 168-169); continue through the 300-332 sub-band and
  then upward. Current measured total **17,552 / 20,252 (86.67%)**, 0 duplicate
  addresses, gates green. The 516-540 byte eight in
  `docs/SPEC_WAVE158_BASES.md` are still queued. Model policy (2026-11-09): use
  the paid `opencode-go/deepseek-v4.1-flash`; never
  `iceman-contributor-free`.
- **Wave 187 supply:** `game::bases` only. Waves 168-187 cover 553 bodies; the
  300-512 band, its GQR/`psq` set and the 64-128 band are complete, and
  128-192 is in progress. Next: finish 128-192, then 192-256. Remaining supply
  is 496 bodies / ~53 KB (472 are the untranslatable `0x802DCBFC`
  forwarders, 2 are documented deferrals, and 15 are very large bodies).
  Current measured total **18,541 / 20,252 (91.55%)**, 0 duplicate addresses,
  gates green. The 516-540 byte eight in `docs/SPEC_WAVE158_BASES.md` are still
  queued. Model policy (2026-11-09): use the paid
  `opencode-go/deepseek-v4.1-flash`; never `iceman-contributor-free`.
- **Wave 198 supply:** `game::bases` only. Waves 168-198 have translated
  5,607 of 5,615 bodies (99.86%). The 472 `0x802DCBFC` interface thunks were
  all translated in Wave 200 via the `interface_call` model. Left only: 2
  documented deferrals (`0x806E2BF0`, `0x80697090`). Current measured total
  **20,250 / 20,252 (99.99%)** (Waves 200-225: interface thunks, dispatch
  completions, `0x806B3BB0`, all 542 `ui::hud` bodies, all 830 `game::player`
  bodies, all 397 `engine::dsys` bodies, all 747 `resource::archive` bodies
  and all 1226 `game::stage` bodies), 0 duplicate addresses, gates green.
  Every module is complete except the two documented `game::bases` deferrals
  (`0x806E2BF0`, `0x80697090`).
- **Waves 226-239 supply: untabled pointer-table bodies.** The `functions.tsv`
  inventory is finished at **20,250 / 20,252 (99.99%)**; no lane should expect
  more tsv rows. Wave 226 found **2,457** prologue-verified function starts in
  main.dol with no tsv row; wave 227 grouped them by owning pointer word into
  `docs/UNTABLED_POINTER_TABLES.md` (1,950 owned by a pointer run, 92
  call-site-only, 415 unreferenced, 487,260 B owned). Waves 228-238 translated
  **259** of them (251 exact `fn_<address>`, 8 suffixed/semantic), leaving
  **2,198** without a body; **371** corpus entries are aliases of other slots
  (367 span more than one run, 311 are among the bodyless entries). `src` holds
  583 untabled body blocks over 548 addresses: the 259 corpus entries, 288
  other main.dol starts without a standard prologue, and one REL address.
  Wave 229 closed the `0x80314534` `game::stage` vtable and the `0x802ED9E0`
  `.ctors` batch; wave 231 re-derived vtable ownership; waves 234-238 extended
  the WorldMap, `dEn_c`, `WM_PLAYER_VTABLE` and `0x80350230` runs.
  **Wave 239 is the closing wave:** it freezes the accounting in
  `docs/COVERAGE_FINAL.md` and runs the final repository audit. Untabled bodies
  use `# Original (untabled)` with measured extents and take no `functions.tsv`
  size and no `ledger/agents/` row. Gates green, 0 duplicate addresses,
  0 staging.