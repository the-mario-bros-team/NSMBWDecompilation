# NSMBW decompilation plan

Target: `C:\Users\WitherULTRA7\Downloads\mariobroswii` (read-only source)
Game: SMNE01 — New SUPER MARIO BROS. Wii, build `REV_INFO_r37675`
Output language: **Rust** (source-level reconstruction; not byte-matching)

## Facts

- `sys/main.dol`: 3,491,808 bytes, entry `0x80004050`
  - text @ `0x80004000` (0x26C0) + `0x80006780` (0x2E7260)
  - data @ `0x802ED9E0` (0x106C0) + `0x802FE3A0` (0x532A0) + ...
  - BSS @ `0x80351680`, size `0xDE5BC`
- RELs (LZ11): `d_basesNP`, `d_enemiesNP`, `d_en_bossNP`, `d_profileNP`
- Assets: 820 U8 `.arc`, 88 BRSTM, BRSAR/BREFF/BREFT/BRFNT, BRRES/BLYT/BCSV inside arcs
- Content: 1,195 files, 352 MB

## Toolchain constraint

REA rejects Wii formats directly (verified: `main.dol` and `.arc` both
`unsupported binary format`). Even after DOL→ELF conversion, every REA native
provider rejects PowerPC on this host (verified 2026-11-01):

- Ghidra Windows P0: native PE + raw 6502 only (ELF/PowerPC unsupported)
- Hopper: no PowerPC adapter
- IDA: no verified PowerPC lane
- Kuna: x86-64 / AArch64 / ARM only

Decision: **standalone Ghidra 12.1.4** performs PowerPC analysis. REA remains
installed and connected but is outside the PPC decompilation loop.

Pipeline:
1. Convert DOL/REL to ELF (`EM_PPC`, big-endian) with in-repo Node tools.
2. Ghidra headless (`PowerPC:BE:32:default`) auto-analyzes and exports C.
3. Translate the pseudocode into Rust, organized by system.


Prerequisites: JDK 21 (portable, user-scope), Ghidra 12.1.4 (portable, user-scope).

## Order of work

0. Provenance and inventory: hash manifest, U8 index, LZ11 REL decompression,
   DOL/REL metadata, string scan.
1. Adapter and spike: `dol2elf`, `rel2elf`, `lz11`, `u8`; Ghidra install;
   prove PowerPC decompilation on the entry point.
2. Boot and CRT: entry -> `__start`, stack/BSS, `OSInit`, ctors, `main()`.
3. Core runtime and loaders: heap/arena, threads, DVD/FST, U8 mount, REL loader
   (OSLink -> real REL load addresses), PAD/WPAD, IPC.
4. Resource layer: LZ11, U8, BRRES, BCSV, BRLYT/BRAN, BRFNT, BRSTM/BRSAR/BREFF.
5. Engine framework: scene/state, actor/profile system, collision, camera,
   GX render/particles, audio, UI, save/replay.
6. Gameplay: `d_basesNP`, `d_enemiesNP`, `d_en_bossNP`, `d_profileNP`;
   player, items, enemies/bosses, stages, world map, modes.
7. Verification: coverage, xref audit, Rust translation quality, unknowns.

Ordering rationale: REL code is only meaningful after the loader stage fixes
module base addresses; engine code depends on resource schemas.

## Workspace

```
docs/      INVENTORY.md, TOOLCHAIN.md, FORMATS.md, SYMBOLS.md, LOG.md
orig/      manifest.sha256.json
work/      dol/ rels/ arcs/ ghidra/   (generated, disposable)
tools/     dol2elf.mjs rel2elf.mjs lz11.mjs u8.mjs bres.mjs
src/       Rust crate(s), mirroring main/ and rels/<module>/ systems
data/      decoded assets and schemas
build/     linker/objdiff only if matching is ever requested
tests/     parser tests
ledger/    functions.csv, coverage.md
```

## Rust policy

- Ghidra C output is retained under `work/ghidra/` as reference.
- Rust is the authoritative reconstruction: modules mirror the original
  systems; raw-memory helpers are allowed where the original is unsafe.
- No byte-matching claim; Rust cannot match the original binary.
- Runtime/tooling choices (std vs no_std, graphics/audio backend) are deferred
  until the engine layer is understood.

## Checkpoints

- C0: inventory + adapter spike (done)
- C1: boot chain decompiled and Rust skeleton for boot/CRT (in progress)
- C2: loader + REL base addresses documented
- C3: one full system (e.g. player) translated to Rust

## Status (2026-11-15)

- Stage 0 complete: manifest 1,195 files; U8 index 820 arcs / 2,933 members;
  RELs LZ11-decompressed; DOL→ELF with BSS split.
- Stage 1 complete: JDK 21 + Ghidra 12.1.4; Gekko_Broadway language installed;
  **20,252 functions exported** (10,276 main.dol + 9,976 REL).
- REA cannot decompile PowerPC on this host; standalone Ghidra is the engine.
- Stage 2 complete: boot/CRT, OS init, engine frame/dSys, OSLink, DVD/input,
  BT, lyt/UI, stage/course, WorldMap, player, enemies, bosses, audio AX, BRRES,
  scene/save; all translated code lives in semantic modules (0 staging).
- Measured coverage: **20,250 / 20,252 functions (99.99%)**; main.dol
  10,276/10,276 (100%), d_profileNP 36/36 (100%), d_en_bossNP 770/770
  (100%), d_enemiesNP 3,555/3,555 (100%), d_basesNP 5,613/5,615
  (99.96%). The two remaining rows are the documented `game::bases` deferrals
  `0x806E2BF0` (96 B) and `0x80697090` (248 B), so the `functions.tsv`
  inventory is complete. Six of the 5,613 translated `d_basesNP` rows are player
  bodies filed in `src/game/player.rs`, which is why the semantic
  `game::bases` module holds 5,607 rows.
- Waves 4-229 plus the Wave 35 body audit added 19,545 ledger functions. All
  gates green, 0 duplicate addresses, 0 missing files, 0 staging. Waves 228-238
  also work *outside* `functions.tsv`: **259** of the 2,457 untabled starts now
  have a body (251 exact `fn_<address>`, 8 suffixed/semantic), leaving
  **2,198** without one; **371** entries are aliases of other slots. They are
  tracked in `work/wave228/` through `work/wave238/` and the coverage docs.
  The 2,457-entry untabled corpus is inventoried in
  `docs/UNTABLED_POINTER_TABLES.md`; the accounting is frozen in
  `docs/COVERAGE_FINAL.md`.
- `d_en_bossNP` and `d_enemiesNP` are both complete; `d_basesNP` is complete
  except for those two deferrals.
- Wave 158 classified the remaining bases backlog: 1,176 bodies are
  reconstructible now, 34 need one pinned constant, 97 are byte-identical
  clones and 473 are `0x802DCBFC` forwarders with no shipped target.
- Wave 159 claimed 39 of the ranked small-body clusters (2,440 bytes); Wave 160
  claimed 33 more (1,872 bytes); Wave 161 claimed 55 constant getters (660
  bytes); Wave 162 claimed 29 short singles (368 bytes); Wave 163 claimed 35
  bodies from the 4-36 byte band (580 bytes); Wave 164 claimed the last ten
  36-40 byte singles (384 bytes); Wave 165 claimed 39 bodies from the 44-64
  byte band (2,208 bytes); Wave 166 claimed the other 28 (1,556 bytes); Wave
  167 claimed the last 18 (984 bytes), completing the band; Wave 168 claimed
  the first seven bodies of the 300-512 byte band (2,252 bytes); Wave 169
  claimed eight more from its 300-316 byte sub-band (2,428 bytes); Wave 170
  claimed the following eight from the 312-320 byte sub-band (2,520 bytes);
  Wave 171 claimed the eight-body 324-332 byte sub-band (2,616 bytes); Wave
  172 claimed the twenty-four-body 332-348 byte sub-band across five lanes
  (8,144 bytes); Wave 173 claimed the nineteen-body 352-364 byte sub-band
  across five lanes (6,808 bytes); Wave 174 claimed the twenty-seven-body
  368-388 byte sub-band across five lanes (10,216 bytes); Wave 175 claimed
  the twenty-six-body 392-412 byte sub-band across five lanes (10,448 bytes);
  Wave 176 claimed the twenty-four-body 412-432 byte sub-band across five lanes
  (10,132 bytes); Wave 177 claimed the twenty-one-body 432-456 byte sub-band
  across five lanes (9,404 bytes); Wave 178 claimed the twenty-body 456-492
  byte sub-band across five lanes (9,448 bytes); Wave 179 claimed the last
  fifteen bodies of the band, 492-512 bytes, across five lanes (7,508 bytes),
  completing the 300-512 byte band; Wave 180 cleared the seven deferred
  GQR/`psq` bodies (2,336 bytes); Wave 181 began the sub-300-byte sweep with
  forty 64-128 byte helpers and destructors (3,944 bytes); Wave 182 claimed
  fifty more from the same band, ten per lane (4,692 bytes); Wave 183 claimed
  another fifty from 64-128 (4,680 bytes); Wave 184 claimed forty-nine more
  (4,692 bytes) and documented one honest deferral; Wave 185 claimed fifty
  more from 64-128 (4,820 bytes); Wave 186 claimed fifty more (5,156 bytes);
  Wave 187 moved into the 128-192 band with fifty bodies (8,016 bytes);
  Wave 188 added fifty more (7,700 bytes); Wave 189 another fifty (7,716 bytes); Wave 190 another fifty (8,144 bytes); Wave 191 cleared the last twenty-six of
  128-192 (4,412 bytes).
- Next: the pointer-table backlog outside `functions.tsv`. Wave 226 found
  2,457 prologue-verified function starts with no tsv row
  (`work/wave226/laneC_untabled_entries.csv`); wave 227 grouped them by owning
  pointer word (`docs/UNTABLED_POINTER_TABLES.md`); waves 228-238 translated
  259 of them, leaving 2,198 plus 371 slot aliases. The largest remaining runs
  are the WorldMap vtable `0x80325428`, `dEn_c` `0x80314068` and
  `WM_PLAYER_VTABLE 0x8032679C`. Wave 239 is the closing verification wave:
  lane B froze the accounting in `docs/COVERAGE_FINAL.md`, lane A audited the
  repository, lanes C/D sampled bodies and module headers, and lanes E/F
  continued the two largest vtables. See `docs/COVERAGE_WAVE228_TABLES.md`
  through `docs/COVERAGE_WAVE238_AUDIT.md` and `docs/COVERAGE_FINAL.md`.


## Windows port (after decompilation)

See `docs/PORT_WINDOWS.md`. Decompilation reconstructs Wii PowerPC logic; the
port replaces the platform (GX→wgpu/D3D12, AX→WASAPI/XAudio2, MMIO→host,
Wiimote→keyboard/XInput) behind hardware-abstraction traits.

Milestones: P0 boot vertical slice → P1 asset pipeline/title → P2 1-1 slice →
P3 audio/save/menus → P4 full game. Start P0 only after boot/CRT and engine
startup are translated and the abstraction traits exist.


