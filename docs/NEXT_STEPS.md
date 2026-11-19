# Next steps

This file is for whoever continues the reconstruction. It is intentionally
concrete: source path, destination, and gate commands.

## Run the gate

```powershell
.\scripts\check.ps1
```

It runs `cargo check`, doc-lint, ledger merge, function index, coverage, and
progress. A lane is not mergeable while any step fails.

## What is already done

- Inventory, U8 index, 820 arcs extracted, four RELs LZ11-decompressed.
- Ghidra export: 10,276 main.dol functions + 9,976 REL functions.
- REL→ELF with 198,456 relocations applied (0 failures).
- Boot/CRT, OS init, engine frame/dSys, OSLink, DVD/input, BT, lyt/UI,
  stage/course, WorldMap, player, enemies, bosses (partial), audio AX, BRRES
  loaders, scene/save.
- All translated functions live in semantic modules; `src/subsystems/` is
  tombstones only.
- Waves 4-229 plus the Wave 35 body audit added 19,545 ledger functions:
  **20,250 / 20,252 (99.99%)**, 0 duplicate addresses, gates green. The two
  remaining rows are the documented `game::bases` deferrals `0x806E2BF0` and
  `0x80697090`, so the `functions.tsv` inventory is complete. The ledger status
  split is **20,225 translated / 23 partial / 2 stub**; wave 250 froze that
  split and the residue inventory is in `docs/COVERAGE_FINAL.md`.
- Waves 228-238 work outside `functions.tsv`. Wave 226 found 2,457
  prologue-verified function starts that have no tsv row; **259** of them now
  have a translated body in the modules that own the referenced data, leaving
  **2,198** without one. **371** of the corpus entries are aliases of other
  slots (the same address occupies two or more slot words; 367 of those span
  more than one run), so one body serves every slot that names it. These bodies
  carry `# Original (untabled)` and are tracked in `work/wave228/` ...
  `work/wave238/` and the coverage docs, not in the ledger; the wave-238
  accounting is `work/wave238/laneF_accounting.json` and the frozen summary is
  `docs/COVERAGE_FINAL.md`.
- **Wave 239 froze the accounting.** 20,250 / 20,252 tabled bodies, two
  documented deferrals, 259 / 2,198 / 371 untabled, 0 staging, 0 duplicate
  addresses. All figures in this file, `README.md`, `PLAN.md`, `docs/LANES.md`
  and `docs/RESET_POLICY.md` follow `docs/COVERAGE_FINAL.md`.
- This build has **no BCSV container**; the former `bcsv` lane is now
  `resource::registry` (`dRes_c` / `info_c` archive registry).
- See `docs/PROGRESS.md` for the measured percentage and `docs/GAP_REPORT.md`
  for the ranked remaining clusters.
- **Model policy:** `docs/RESET_POLICY.md`. Waves run four lanes on the paid
  `opencode-go/deepseek-v4.1-flash` only, never `iceman-contributor-free`.
  When its weekly limit stops a wave, salvage with
  `node tools/audit-unledgered.mjs --file <module>` and record the failure
  before waiting for the reset. Plan lanes with
  `node tools/plan-next-wave.mjs`.
- **Wave 160 ran directly while the paid model was limited.** The coordinator
  wrote 33 bodies into `src/game/bases.rs` (1,872 bytes), the lane CSV and
  `docs/COVERAGE_WAVE160_BASES.md`.
- **Wave 161 ran directly while the paid model was limited.** The coordinator
  wrote 55 constant getters into `src/game/bases.rs` (660 bytes), the lane CSV
  and `docs/COVERAGE_WAVE161_BASES.md`.
- **Wave 162 ran directly while the paid model was limited.** The coordinator
  wrote 29 short singles into `src/game/bases.rs` (368 bytes), the lane CSV and
  `docs/COVERAGE_WAVE162_BASES.md`.
- **Wave 163 ran directly while the paid model was limited.** The coordinator
  wrote 35 bodies from the 4-36 byte band into `src/game/bases.rs` (580 bytes),
  the lane CSV and `docs/COVERAGE_WAVE163_BASES.md`.
- **Wave 164 ran directly while the paid model was limited.** The coordinator
  wrote the last ten 36-40 byte singles into `src/game/bases.rs` (384 bytes),
  the lane CSV and `docs/COVERAGE_WAVE164_BASES.md`.
- **Wave 165 ran directly while the paid model was limited.** The coordinator
  wrote 39 bodies from the 44-64 byte band into `src/game/bases.rs` (2,208
  bytes), the lane CSV and `docs/COVERAGE_WAVE165_BASES.md`.
- **Wave 166 ran directly while the paid model was limited.** The coordinator
  wrote the remaining 28 bodies of the 44-64 byte band into `src/game/bases.rs`
  (1,556 bytes), the lane CSV and `docs/COVERAGE_WAVE166_BASES.md`.
- **Wave 167 ran directly while the paid model was limited.** The coordinator
  wrote the last 18 bodies of the 44-64 byte band (984 bytes), the lane CSV and
  `docs/COVERAGE_WAVE167_BASES.md`. The band is now complete apart from the
  deferred `0x802DCBFC` forwarders. Next: the 300-512 byte spec band, or the
  class-P / clone work if a read-only lane becomes available.
- **Wave 168 ran directly while the paid model was limited.** The coordinator
  wrote the first seven bodies of the 300-512 byte band (2,252 bytes), the lane
  CSV and `docs/COVERAGE_WAVE168_BASES.md`. The batch covers the colour-table
  renderer helpers, the padded box test and the `0x807C2A00`/`0x807C2E50` state
  twins.
- **Wave 169 ran directly while the paid model was limited.** The coordinator
  wrote eight more bodies from the 300-316 byte sub-band (2,428 bytes), the lane
  CSV and `docs/COVERAGE_WAVE169_BASES.md`. The batch covers the anchor movers,
  the player-window test, the air-state tick, the transform rebuild, the pose
  copy, the shared-state lerp, the renderer seed and the sample axis test.
- **Wave 170 ran directly while the paid model was limited.** The coordinator
  wrote eight bodies from the 312-320 byte sub-band (2,520 bytes), the lane CSV
  and `docs/COVERAGE_WAVE170_BASES.md`. The batch covers the spawn reset, the
  render push, the source re-seed, the third `0x900` request gate, both layout
  loaders, the ship controller and the state dispatch.
- **Wave 171 ran directly while the paid model was limited.** The coordinator
  wrote eight bodies from the 324-332 byte sub-band (2,616 bytes), the lane CSV
  and `docs/COVERAGE_WAVE171_BASES.md`. The batch covers the pose copy, the
  selector reconcile, the two gate twins, the transform rebuild, the
  multi-course-select loader, the throw start and the throw end.
- **Wave 172 ran on the paid model with five lanes.** Lanes A-E wrote 24 bodies
  from the 332-348 byte sub-band (8,144 bytes), five lane CSVs and five lane
  coverage docs. The coordinator consolidated them into
  `docs/COVERAGE_WAVE172_BASES.md` and fixed three cross-wave call-signature
  errors (`0x801680E0`, seven `0x8077E410` sites, `0x80620A00`).
- **Wave 173 ran on the paid model with five lanes.** Lanes A-E wrote 19 bodies
  from the 352-364 byte sub-band (6,808 bytes) plus five lane CSVs and coverage
  notes; the coordinator consolidated `docs/COVERAGE_WAVE173_BASES.md`. The
  batch covers the score-screen tick, the `daWaterMove_c` destructor, the ship
  update, the camera hand-off twins, the death handler, the subtree collector,
  the swept-box test, the sprite placements and the lift/spawn helpers.
- **Wave 174 ran with five paid-model lanes.** Lanes A-E wrote 27 bodies from
  the 368-388 byte sub-band (10,216 bytes) plus five lane CSVs and coverage
  notes; the coordinator consolidated `docs/COVERAGE_WAVE174_BASES.md`. The
  batch covers the result-panel spawner, the cursor grid, the dokan arm, the
  ship prop and input, the `Wm_2d_continue` spawn (resolving the wave-86
  deferral), the element-map rebuilds, the broom reinit, the rider list and the
  pipe transform.
- **Wave 175 ran with five paid-model lanes.** Lanes A-E wrote 26 bodies from
  the 392-412 byte sub-band (10,448 bytes) plus five lane CSVs and coverage
  notes; the coordinator consolidated `docs/COVERAGE_WAVE175_BASES.md`. The
  batch covers the pane row writer, the lift-probe scanner, the torch loader,
  the world-select reset, the shot/prop/map-burst helpers, the player-record
  rebuild, the height/pendulum updates, the cloud machine, the ground-probe
  snap, the lift clamp and the element-row writer.
- **Wave 176 ran with five paid-model lanes.** Lanes A-E wrote 24 bodies from
  the 412-432 byte sub-band (10,132 bytes) plus five lane CSVs and coverage
  notes; the coordinator consolidated `docs/COVERAGE_WAVE176_BASES.md`. The
  batch covers the board cursor, the dust spawner, the pickup scan, the part
  rebuilds, the course-entry hand-over, the aim pair, the pendulum frame, the
  lift constructors, the scene-list scan, the route/palm/bros loaders, the
  highlight fades, the slot reservation and the sand-pillar contact.
- **Wave 177 ran with five paid-model lanes.** Lanes A-E wrote 21 bodies from
  the 432-456 byte sub-band (9,404 bytes) plus five lane CSVs and coverage
  notes; the coordinator consolidated `docs/COVERAGE_WAVE177_BASES.md` and
  switched `fn_8075bd20` to the real `fn_8075bb60`. The batch covers the
  subsystem allocator, the state gates, the plank classifier, the antlion and
  yoshi-egg loaders, the firepillar burst, the transform copy, the pane bits,
  the id collector, the effect queue, the node offsets, the `cobTsuta` pose,
  the snapshot spawn, the scene shutdown, the pitch ramp and the launch
  reaction.
- **Wave 178 ran with five paid-model lanes.** Lanes A-E wrote 20 bodies from
  the 456-492 byte sub-band (9,448 bytes) plus five lane CSVs and coverage
  notes; the coordinator consolidated `docs/COVERAGE_WAVE178_BASES.md`. The
  batch covers the date-file pad, the window-message writer, the emitter and
  pane inits, the gun-part dispatcher, the aim sweep, the scripted-screen
  handler, the castle loader, the key-gate wall, the lift recount, the
  carousel, the smoke ring, the ship ride state machine and the bubble pass.
- **Wave 179 ran with five paid-model lanes and completed the 300-512 band.**
  Lanes A-E wrote the last 15 bodies (492-512 bytes, 7,508 bytes) plus five
  lane CSVs and coverage notes; the coordinator consolidated
  `docs/COVERAGE_WAVE179_BASES.md` and wired `fn_8073ff60` into its two
  neighbours. The batch covers the gabon rock re-arm, the balloon and layout
  loaders, the koopaJr electric start, the intensity ramp, the firepillar
  phase, the screen-state selector, the aim machine, the UI projection, the
  marker records, the pipe appear/rise machines and the arrival sub-machine.
- **Wave 180 cleared the deferred GQR/`psq` set.** Seven bodies (2,336 bytes)
  that had been skipped since waves 81/103/109 were translated: every `psq_*`
  is a `qr0`/`W=0` scalar `f32` spill and every `ldexpf` guard is compiler
  noise. `0x8063B900`/`0x8063BA50` are exact `+0x244`/`0x248` twins, and
  `0x806BBB90`'s `extsh r4,r3` was confirmed against `main.elf` callers.
- **Wave 181 began the sub-300-byte sweep.** Five lanes wrote 40 bodies from the
  64-128 byte band (3,944 bytes), eight per lane; 16 of them are the established
  A-type deleting destructor family. The wave also recovered `0x80615D90` from
  raw words after its export `halt_baddata`d, and mapped the BSS state
  descriptors through their `.rodata` name words.
- **Wave 182 continued the sub-300 sweep with 10 bodies per lane.** Fifty bodies
  from the 64-128 byte band (4,692 bytes) were translated. The wave added real
  bodies for `0x806505B0` (used by four animation gates), `0x80640AC0`
  (previously "bad instruction data") and `0x806842D0`, removing three opaque
  boundaries, and corrected `fn_80684a30`'s doc claim about its allocator
  wrapper.
- **Wave 183 continued at ten bodies per lane.** Fifty more 64-128 byte bodies
  (4,680 bytes), dominated by destructor and vtable-tail families. The wave
  found and fixed a real inversion in the already-translated `fn_806c3710`
  (the flag is `delta <= 0.0`, not `>= 0.0`) by cross-checking the shared
  `mfcr`/`rlwinm`/`cntlzw` idiom against the new `fn_806b4b20`.
- **Wave 184 processed 49 of 50 bodies.** The wave covered another 64-128 slice
  (4,692 bytes), dominated by destructors, player-slot gates and sink clamps. It
  documented one genuine deferral, `0x806E2BF0`, whose loop writes only into its
  own frame (and carries an `ori r0,r0,0` post-link nop), so the destination
  field is unrecoverable.
- **Wave 185 processed 50 bodies.** Another 64-128 slice (4,820 bytes). It
  retired the wave-106 deferral `0x8075F9E0` (the "missing actor pointer" is the
  live `r3` from `0x80162D50`) and recovered two `halt_baddata` tails
  (`0x807242C0`/`0x80724320`) as `b 0x8006D090` calls.
- **Wave 186 processed 50 bodies.** Another 5,156 bytes. It gave real bodies to
  `0x80799960` (three-argument pose selector) and `0x807C1130` (live-`f1`
  return), recovered `0x8079F4B0` from `halt_baddata` as a `b 0x80102690` tail,
  and resolved dropped `r3`/`this` arguments in four `0x807C1xxx`/`0x807C3xxx`
  bodies.
- **Wave 187 moved to the 128-192 band.** Fifty bodies (8,016 bytes) covering
  the character-change arrow and `modelPlayGuide` loaders, the out-animation
  swaps, the KeyWait/CoinCountDown/WipeWait descriptors, the lift helpers, the
  `block_arrow` actor and the save-grid scans. `0x806148F0` was rebuilt from raw
  words after a `bad instruction data` truncation.
- **Wave 188 processed 50 bodies.** Another 7,700 bytes from 128-192. It gave
  real bodies to the `0x80697D00`/`0x80699C00` gates (now called directly by
  `fn_80697f10`/`fn_80699e30`) and to `0x806E7F50` (called by `fn_806e7ff0`).
  Eight name-suffix comparators and four ring/segment math bodies were added.
- **Wave 189 processed 50 bodies.** Another 7,716 bytes from 128-192: the
  `lift_turi_ashiba` bodies, the turi/pipe factories, the fire-cannon aim step,
  the watch-target record, the perch tilt and three more `Class:Name`
  comparator clones. `0x80706360`'s Ghidra return was a dead accumulator; the
  real result is a 0/1 membership test.
- **Wave 190 processed 50 bodies.** Another 8,144 bytes from 128-192, crossing
  90% overall. It covers the `daWmSandPillar_c` family, the `shipRoute` stop
  list (`0x8076FDA0`, now called directly by `fn_806fc60`), the route-node
  samplers, the `Env_world` blight/blmap/bfog binding and the `MainSelect`
  starter.
- **Wave 191 finished the 128-192 band.** The last 26 bodies (4,412 bytes)
  cover the `Env_world`/`Env_movie` starters, the layout light/fog binding, the
  GX viewport/scissor rewrite, the save-end state descriptors and the
  `0x807BE3D0` ease (whose real signature is `(f32, f32) -> f32`).
- **Wave 192 opened the 192-256 band.** 49 of 50 bodies (11,248 bytes), with
  `0x80697090` deferred because its target `(x,y)` is read from stack slots it
  never writes. The wave gave real bodies to `0x8064EC70` (called three times
  per branch by `fn_8064eeb0`), `0x80648470`, `0x8062C800`, `0x80613290` and
  `0x80627160`, and covered the `dCollectionCoinDate`/`dCourseClear`/
  `dGameOver`/`dMessageWindow` loaders plus the results-table machinery.
- **Wave 193 processed 50 bodies.** Another slice of 192-256. It gave real
  bodies to `0x806A9F50`, `0x806CB140`, `0x806D0130`, `0x806EC5F0`,
  `0x806ED160`, `0x806FE310` and `0x8070F9C0`, and wired every one of those
  call sites to the real function (8 call sites total). It also covered the
  plank-node rebuilds, the rider effect tables, the lift gates and the boat
  lighting frames.
- **Wave 194 processed 50 bodies.** Another 192-256 slice. Eight call sites
  were wired to newly translated bodies (`0x8075FA40`, `0x80789990` x2,
  `0x80798E20` x2, `0x8075BE90` x2). The wave retired the wave-101 deferrals
  `0x807382F0`/`0x80738DB0` (only GQR noise), and corrected `0x807538E0`'s
  guarded test to `(+0x358 | +0x35C) == 0` (the export's masked double test was
  wrong).
- **Wave 195 opened the 256-300 band.** Fifty large bodies (13,940 bytes). It
  gave real bodies to the shared ratio helper `0x806601B0`
  (`(position, low, high) -> f32`, with `p == high` returning 0.0) and to
  `0x8068C5A0` (`(height, this) -> f32`) and `0x80693DD0`. It also decoded
  `0x80705600` to `PSMTXCopy`/`PSMTXConcat` and found dead arithmetic in
  `0x80707F40`/`0x806E2E40`.
- **Wave 196 completed the translatable set.** Twenty-nine final bodies from
  256-300. `0x807AD590`, previously deferred as psq-heavy, contains no `psq_*`
  at all and is now translated. `game::bases` can be treated as done: the only
  untranslated rows are the 472 `0x802DCBFC` forwarders (no shipped target) and
  the two bodies that read state the code never writes. (This estimate was
  later corrected: 63 genuine bodies remained.)
- **Wave 197 cleared the mid-size remainder.** Thirty-five bodies (196-312
  bytes) across four lanes, with five call sites wired to new bodies
  (`0x807C09B0`, `0x80762EA0`, `0x8079BC10`, `0x807BAA30`, `0x807BFC20`).
  Signature recoveries: `0x807C09B0(_unused, id)` (the `lis r3,1` clobbers the
  dead first argument) and `0x8071C180`'s three out/mask pointers plus `u32`
  return. Remaining after this: 12 very large bodies (516-4,232 bytes).
- **Wave 198 translated 12 of 15 giant bodies.** Lanes A, C, D and E merged
  (up to 3,788 bytes each). Lane B's three bodies (1,772/1,772/3,432 bytes) were
  un-merged because they carried ~20 mechanical `u32`/`usize` errors; that lane
  was sent back to fix its block against a real scratch `cargo check`; after
  fixes (and a real signed/unsigned timer correction) all three merged, so
  Wave 198 actually delivered all 15 giant bodies (17,600 bytes).
- **Wave 199 finished the giants.** Six lanes translated the last 12 bodies
  (13,684 bytes), including the 4,232-byte `0x8079C330` (a 0x9F-entry switch
  over states 6..=0xA4) and the 4,076-byte `0x8077B450` (99-entry jump table
  plus a 26-arm decision tree). All twelve compiled on the first merge. The
  only remaining untranslated rows are the 472 `0x802DCBFC` forwarders, the two
  documented deferrals and one 32-48 byte body.
- **Wave 200 translated all 472 interface thunks.** Every 48-64 byte
  "forwarder" is the same `mr r5,r3` / `mr r3,r4` / `addi r12,r5,K` /
  `bl 0x802DCBFC` thunk into the shipped main.dol dispatcher. That dispatcher
  resolves an object-embedded `{self_adjust, index, table}` descriptor; the
  helper `interface_call` models it exactly. Six audit lanes verified all 471
  generated bodies against the ELF (zero mismatches), and the merge took
  `game::bases` to 5,607/5,615 (99.86%). Only the two documented deferrals
  remain.
- **Waves 224-225 finished `src/game/stage.rs` and all of main.dol.** The last
  72 stage bodies include the conflict test `0x800A78D0`, the reaction block
  `0x800A9550`, the effect submitter `0x800AADE0` and the 468-byte highest
  function `0x800ADB00`. `game::stage` is 1226/1226 and main.dol is
  **10,276/10,276**: every function in `work/ghidra/main-gb/export/functions.tsv`
  is translated. Only the two documented `game::bases` deferrals remain.
- **Wave 203 opened main.dol (`ui::hud`).** Six lanes translated the first 60
  hud bodies (1,344 + 1,244 + 1,232 + 2,360 + 2,628 + 1,700 bytes). The 12-byte
  bodies are `lis/addi/blr` accessors returning `0x80377A04`, not `blr` stubs.
  Lane D corrected two manifest sizes (`0x80021C80` = 544, `0x80021F50` = 292)
  and lane E found `0x800A77A0` needs eight integer arguments, marshalled through
  a local `extern "C"` pointer.
- **Wave 204 added 60 more `ui::hud` bodies.** The 12-byte accessors are *not* one
  family: `0x80026BB0` returns `0x80377A04`, but `0x8002A800..0x8002A850` form a
  descending `0x40`-stride BSS run (`0x8035324C`..`0x8035338C`) and
  `0x800268B0`/`0x800268C0` return `0x80357BF4`/`0x80357BB4`. Several manifest
  sizes were wrong and corrected from `blr`+prologue extents: `0x80025800` = 508,
  `0x80024E80` = 132, `0x80028780` = 424, `0x80028930` = 560.
- **Wave 205 added 60 more `ui::hud` bodies.** Lane C proved its run is the
  compiler's out-of-line duplicate of wave 204 lane F (`fc.exe` byte-identical
  for two bodies, `bl`-only differences elsewhere). The `daEnJimenFPakkun_c`
  render/contact cluster landed, plus the four-corner overlap test, the
  orbit/attach bodies and the open-door burst. Size corrections: `0x8002AC90`
  = 216, `0x8002D0C0` = 404, `0x8002BE60` = 140.
- **Wave 206 added 60 more `ui::hud` bodies.** Crossed 95% overall. The
  `nokonokoB` and `kuriboBig` model/anim clusters, the piranha hit/contact
  handlers and the death sequences landed. More size corrections:
  `0x80031CC0` = 304, `0x80035B10` = 152, `0x80034140` = 276,
  `0x80034260` = 132. A fifth accessor target appeared (`0x80357D74`).
- **Wave 207 added 60 more `ui::hud` bodies.** The `togezo` scene band, the
  `daEnObjCoinBlock_c` state descriptors (including the 1,480-byte
  `0x80037750`), the egg/partner hold machine and the impact responders landed.
  Lane F flagged a **polarity conflict**: `fn_8003ACD0` and `fn_80033F10` share
  the same `fcmpo`/`cror`/`cntlzw` sequence but their docs record opposite
  sides; one needs correcting in the final audit.
- **Wave 208 finished `src/ui/hud.rs`.** The last 48 bodies landed, including the
  1,500-byte static registration of the seven shell states and the
  `Wm_en_quicksand` band. `ui::hud` is now 540/540. remaining main.dol work:
  825 bodies (114,976 bytes) spread over `player.rs`, `dsys.rs`,
  `archive.rs`, `stage.rs` and smaller files.
- **Waves 209-210 closed out `ui::hud` and opened `src/game/player.rs`.**
  Wave 209 was an audit round that fixed two real defects (`fn_80033F10`
  side polarity, `fn_8001D850` child walk). Wave 210 translated the first
  60 `game::player` bodies (`0x80060110`-`0x800630B0`), mostly the roster
  driver, player-slot bookkeeping and the dokan-block cluster. `game::player`
  is now 597/818 (72.98%); 221 bodies remain in that file.
- **Wave 211 added 60 more `game::player` bodies and corrected the queue.**
  The per-file inventory had been built from the smaller `main` Ghidra
  export, so 52 canonical `main-gb` functions were never queued (including
  two `ui::hud` bodies). `work/rebuild-missing.cjs` now derives the queue
  from `main-gb` and maps each address to its source file by nearest
  translated ledger neighbour: dsys 248, player 233, archive 198, stage 132,
  hud 2.
- **Wave 212 added 60 more `game::player` bodies**, including three large
  sweeps (768/552/844 bytes) recovered in full. `game::player` is now
  717/818 (87.65%); 113 bodies remain in that file. Repeated results from the
  lane work: every integer-to-float conversion in the `0x80068xxx` cluster is
  signed (bias `2^52 + 2^31`, not `2^52`), and Ghidra inverts several tests
  in this range (`0x80066A40`, `0x80066C60`), which the disassembly corrects.
- **Wave 213 added 60 more `game::player` bodies.** `game::player` is now
  777/818 (94.99%); 53 bodies remain there. The branch polarity of
  `0x800693E0` was settled from the raw `bc 4,2` (BO=4 branches on a clear
  bit, i.e. `bne`), which matters because reading it as "branch if equal"
  inverts the nearest-player search. Lane C also found that a wave-98 `f64`
  helper mis-models `0x80082240`/`0x800B30E0` (both are `f32`), so new `f32`
  thunks were added instead of reusing it.
- **Wave 214 finished `src/game/player.rs`.** The last 53 bodies include the
  full contact resolver `0x8006DFC0` (1,232 B), the landing/step driver
  `0x8006E720` (1,668 B), the sloped-ground entry and the single-point
  contact test. `game::player` is now 830/830. Two tool defects were found by
  the lanes: `check-lane-csv.mjs` reads its byte-size column as `len-2` (the
  `true` column), so its size check never fires, and the local disassembler's
  `F63_32` fallback prints `FRA`/`FRB` reversed for `fcmpu`.
- **Wave 215 opened `src/engine/dsys.rs`.** The first 60 dsys bodies cover the
  record-append families (`+0xD8`/0x48, `+0x688`/0x14, `+0x548`/0x14), the
  track block at `+0x58` and its two effect slots, and the table
  marshaller. `engine::dsys` is now 209/397; 188 bodies remain.
- **Wave 216 added 60 more `engine::dsys` bodies** with **byte-balanced lanes**
  (snake draft by size) instead of address order, after the first split gave
  one lane 6,632 bytes. The 2,544-byte CSV flag-chain parser `0x800F3E80`
  was decoded in full, recovering 40 keyword literals from the image. The
  scratch decoder was found to print logical X-form operands swapped
  (`and`/`andc`/`nor`/`xor`/`xnor`); fixed in `%TEMP%/opencode/ppc159.js`.
- **Wave 217 added 60 more `engine::dsys` bodies.** Crossed 98%. The run is
  the progress-flag bank query side (`G_PROGRESS_BANK` = `0x8042A040`), plus
  the `"W<world><slot>"` stage-code formatter/parser pair and their two
  dispatch tables at `0x8031C0C0` (11 words) and `0x8031C0EC` (41 words).
  `engine::dsys` is now 329/397; 68 bodies remain.
- **Wave 218 finished `engine::dsys`.** The last 68 bodies include the bank
  row setters, the path rail rebuild (`0x800FF740`, 780 B), the window
  snap `0x800FFA50` and the shell gates. `engine::dsys` is now 397/397.
  Lane B found two **untabled** bodies inside the range (`0x800FC2D0`,
  `0x800FC370`) that have no `functions.tsv` row and no ledger entry, and
  `0x800FC040` is already covered as `game::bases` `fn_80770970` (shared
  with the REL).
- **Wave 219 opened `src/resource/archive.rs`.** The first 60 archive bodies
  cover the GX render-state records, the tiled-texture decoder
  (`0x802C0E30`, 936 B), the 2,424-byte panel builder `0x802C3D80`, the
  ribbon strip `0x802C13F0` and the 2,048-byte draw-and-capture
  `0x802C2700` (tsv lists only its 576-byte jump-table head).
  `resource::archive` is now 609/747; 138 bodies remain.
- **Wave 220 added 60 more `resource::archive` bodies**, crossing 98.9%. The
  run covers the projection/descriptor blocks (`0x802C6AE0`, `0x802C70B0`,
  `0x802C6FF0`), the nine-glyph text-row builder `0x802C7370`, the 848-byte
  named-record walk `0x802C7560` and the 1,020-byte blend block
  `0x802C52A0`. `resource::archive` is now 669/747; 78 bodies remain.
- **Wave 221 added 60 more `resource::archive` bodies.** The run covers the GX
  text state blocks, the 1,060-byte tiled-quad emitter `0x802CC050`, the
  704-byte cell emit `0x802CC730` (a four-vertex FIFO strip that the export
  truncated to one store) and the descriptor dispatch `0x802CE010`.
  `resource::archive` reached 729/747.
- **Wave 222 finished `resource::archive` and closed `ui::hud`.** The last 18
  archive bodies include the 1,296-byte per-chunk slice emitter
  `0x802CF1E0`; the two hud stragglers `0x80026CA0` and `0x8002D260` resolved
  a dangling `FN_8002D260_D` reference from `fn_8002d0c0`. Both modules are
  now complete at 747/747 and 542/542. Only `src/game/stage.rs` (132 bodies)
  remains in the queue.
- **Wave 223 opened `src/game/stage.rs`.** The first 60 stage bodies include
  the clown-Jr model constructor `0x800A3100` (812 B), the 532-byte
  three-state angle ramp `0x800A5210`, the variant/proximity record emitters
  and the destructor `0x800A2F40`. `game::stage` is now 1154/1226; 72 bodies
  remain.
- **Waves 224-225 finished `src/game/stage.rs` and all of main.dol.** The last
  72 stage bodies include the conflict test `0x800A78D0`, the reaction block
  `0x800A9550`, the effect submitter `0x800AADE0` and the 468-byte highest
  function `0x800ADB00`. `game::stage` is 1226/1226 and main.dol is
  **10,276/10,276**: every function in `work/ghidra/main-gb/export/functions.tsv`
  is translated. Only the two documented `game::bases` deferrals remain.
- **Waves 226-239 mapped and began the untabled corpus.** Wave 226 found the
  2,457 prologue-verified starts outside `functions.tsv`; wave 227 grouped them
  by owning pointer word; wave 231 re-derived ownership with vtable-aware
  scanning. Waves 228-238 translated **259** corpus entries (583 untabled body
  blocks / 548 addresses in `src`), leaving **2,198**; **371** entries are slot
  aliases. Wave 239 closed the reconstruction: lane A audited ledger and gates,
  lane B froze the accounting in `docs/COVERAGE_FINAL.md`, lanes C/D sampled
  wave-234-238 bodies and module headers, and lanes E/F extended
  `WM_PLAYER_VTABLE` and `dEn_c`.
- **Wave 249 closed the ledger status.** Lane D re-read the 46 machine-state
  partials and converted 23, taking the ledger to **20,225 translated / 23
  partial / 2 stub**. Lane F re-audited the wave-240-249 untabled citations.
  Wave 250 lane F froze that split here and in `docs/COVERAGE_FINAL.md`; no
  code changed.

## Final status residue (23 partial + 2 stub)

This is the defensible residue. Every one of the 23 `partial` rows has its
control flow and observable data effects translated; the remaining gap is a
machine or ABI property that cannot be written in portable Rust without
changing the calling contract. None is an unread body waiting for another
translation pass.

| Category | Rows | Addresses | Remaining gap |
|---|---:|---|---|
| Machine state with no Rust form | 8 | `0x801ACE30`, `0x800041C0`, `0x802E9300`, `0x802E9398`, `0x802EA6BC`, `0x802EC9CC`, `0x802ECCB4`, `0x801A9D40` | `rfi`, `mtmsr`, LR/MSR composition, GPR/SPR/FPR save-restore. |
| Caller-owned descriptors | 5 | `0x80234A80`, `0x80236230`, `0x802365E0`, `0x80236FC0`, `0x80237370` | `resource::bres` heap/format/sink entries keep the caller's FPR/GPR descriptor as an explicit `args` argument. |
| `va_list` overflow area | 2 | `0x80107050`, `0x80107170` | Variadic formatter and text measure; the overflow argument area belongs to the caller's frame. |
| Unrecovered jump tables | 2 | `0x80A0DB40`, `0x80133E10` | Switch targets behind `0x80A0DB80` / `0x80133E58` are not in the shipped words. |
| Paired-single / missing mode | 3 | `0x80292C90`, `0x80299840`, `0x802A71D0` | GQR/`psq` direction builders, grid mode, and `ps5[0]`/`ps6[0]` edge offsets. |
| One-off limits | 3 | `0x80AEEA30`, `0x80086B20`, `0x80176410` | Caller's uninitialised `0x10(r1)` slot; dispatcher forwards only `r3` while the call passes `r4`/`r5`; GQR0/`psq_l` gauge decode. |

The two `stub` rows are runtime services dispatched by address: `sdk_panic`
(`0x801AD4E0`) and `set_exception_handler` (`0x801AD610`). Their bodies are
intentionally reduced because no translated caller needs their internals.

Closing any of the 25 needs an ABI/`asm` facility or caller-frame work, not a
new lane over unread code. Do not spend translation lanes on them; spend them on
the untabled corpus below.

## Highest-value next lanes

### 1. Untabled pointer-table bodies (biggest remaining mass)

The `functions.tsv` inventory is complete; what is left is the corpus of
2,457 prologue-verified function starts that have no tsv row. Wave 227
inventoried and ranked them in `docs/UNTABLED_POINTER_TABLES.md`; **259** now
have a translated body (10.5%), leaving **2,198**, and **371** entries are known
to be aliases of other slots (367 across runs). The wave-231 rescan is still the
best map: 628 entries are named by a `src` literal (an upper bound), and the
remaining mass sits in the large vtables, not in tier 3.

| Bucket (wave 227) | Entries | Bytes | Where |
|---|---:|---:|---|
| Owned by a pointer run (826 runs, 494 tables >= 3 words) | 1,950 | 487,260 | `docs/UNTABLED_POINTER_TABLES.md` |
| Call-site only (72 gaps) | 92 | - | same doc, "Call-site-only groups" |
| Unreferenced (state-table halves, vtable-built slots) | 415 | - | same doc, "Unreferenced entries" |

Start with the tier-1 runs, which already have a live translated reference.
The `.ctors` array `0x802ED9E0` (76/76 corpus bodies) and the `0x80314534`
`game::stage` vtable (22/22) are complete. The WorldMap vtable `0x80325428`
(55 of 135 corpus entries, 176/276 slots including tabled and non-corpus
bodies), the `dEn_c` vtable `0x80314068` (41 of 108) and
`WM_PLAYER_VTABLE 0x8032679C` (44 of 108) are the three largest remaining items;
slots shared by two of those runs are aliases and need one body, not one per
slot. Then take the tier-3 state tables once their loaders are known.

### 2. REL second passes

| Source | Remaining | Destination | Coverage doc |
|---|---:|---|---|
| `d_basesNP` | 2 | `game::bases` | `COVERAGE_REL_d_basesNP.md` |
| `d_enemiesNP` | 0 | `game::enemies` | `COVERAGE_WAVE5_ENEMIES.md` |
| `d_en_bossNP` | 0 | `game::bosses` | `COVERAGE_WAVE34_BOSSES.md` |

The two `d_basesNP` rows are the documented deferrals `0x806E2BF0` and
`0x80697090`; both read state the code never writes, so they stay deferred
until a caller or a data-bearing trace settles them. The Wave 158
reclassification (`docs/ANALYSIS_WAVE158_BASES.md`) still explains the
`0x802DCBFC` forwarders, all of which were translated in wave 200.

Use `StateID` registrations and constructor/vtable adjacency as the selection
signal. Never guess handler ownership.

### 3. main.dol untabled remainder

Every function in `functions.tsv` for main.dol is translated
(10,276/10,276). The remaining main.dol code is the untabled corpus: the
`docs/UNTABLED_POINTER_TABLES.md` runs plus the 415 unreferenced entries. Bands
that look "uncovered" in the tsv (for example `0x80005000-0x80014000` or
`0x80040000-0x80080000`) are gaps between tabled functions, not missing
analysis; their prologue-verified starts are already counted in the 2,457.
Route them to `render::geometry`, `game::actor`/`game::player`,
`game::stage`/`resource::course`, `game::worldmap`/`ui`,
`ui::lyt`/`audio::ax`, `runtime::dvd`/`runtime::input`, `runtime::bt` and
`resource::archive` by the owning pointer word, not by address order.

### 4. GX paired-single

The deferred `psq_l`/`psq_st` bodies were cleared in wave 180 (seven bodies,
2,336 B): every `psq_*` is a `qr0`/`W=0` scalar `f32` spill and the `ldexpf`
guards were compiler noise. Three rows remain partial for paired-single reasons
(`0x80292C90`, `0x80299840`, `0x802A71D0`); they are in the final residue table
above. New GQR-only material should still get a GQR-aware instruction pass
before translation; do not guess its semantics.

### 5. Resources and port

- BRRES payload parsing (`MDL0`/`TEX0`) and BCSV tables.
- BRSAR/BRSTM audio bank/stream parsing.
- Port prerequisites are in `docs/PORT_WINDOWS.md`; the game logic is not yet
  abstracted from absolute Wii addresses, so P0 cannot run translated code
  without a memory/host bus layer.

## Lane brief template

```
Read docs/RUST_STYLE.md and the relevant COVERAGE doc.
Source: work/ghidra/<export> (range or cluster).
Destination: src/<system>/<module>.rs (append; no new staging).
Select 20-30 untranslated functions; check ledger/functions.csv.
Keep # Original + confidence; synthetic-base note for RELs.
Add ledger/agents/<wave>.csv and docs/COVERAGE_<wave>.md.
Run .\scripts\check.ps1. Fix only your files.
```

## Rules

- Do not add code to `src/subsystems/`; it only shrinks.
- Do not hand-edit generated files (`ledger/functions.csv`, `FUNCTION_INDEX`,
  `COVERAGE`, `PROGRESS`).
- Do not invent Nintendo symbols. Address names are honest names.
- Keep the documentation standard; it is why the tree is readable.
