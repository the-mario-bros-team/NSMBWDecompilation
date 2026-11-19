# Untabled pointer tables in `main.dol`

> **Status update (wave 238):** this document is the wave-227 audit, so the
> per-run tables below still describe the original corpus. Wave 231 re-derived
> the ownership and tiers (revision section below); neither section carries
> current body coverage. Recomputed from the tree in wave 238:
>
> | quantity | count |
> |---|---:|
> | corpus entries (frozen at wave 226) | 2,457 |
> | with a translated body in `src` | **259** |
> | ... exact `fn_<address>` name | 251 |
> | ... suffixed or semantic name (`fn_802413a0_w228b`, `wm_player_*`, ...) | 8 |
> | still without a body | **2,198** |
> | entries that are aliases of other slots | **371** (367 across runs) |
> | ... of the 2,198 bodyless entries | 311 (309 across runs) |
> | named by any `src` literal (wave-231 upper bound) | 628 |
> | in `functions.tsv` / `ledger/functions.csv` | 0 / 0 |
> | untabled bodies in `src` (whole tree) | 583 blocks / 548 addresses |
>
> An **alias** here is a corpus address that occupies two or more table slot
> words: one translated body serves every slot that names it. 367 of the 371
> span more than one run (the `WM_PLAYER_VTABLE`/WorldMap overlap is the
> largest); 4 repeat within a single run. They are already counted once in the
> 1,950 table-owned entries, so the alias count measures slot reuse, not extra
> bodies.
>
> None of the 2,457 has entered the inventory: every hit is code evidence
> (`# Original (untabled)`). The 628 literal count is an upper bound because a
> literal can be a prose mention, which is why it exceeds the 259 bodies. Of the
> 548 untabled-body addresses, 259 are corpus entries, 288 are other `main.dol`
> starts without a standard prologue (leaf thunks, 4-byte `bclr` stubs) and one
> is a REL address. The `functions.tsv` inventory itself is complete at
> 20,250/20,252 (two documented `game::bases` deferrals). Reproduce the
> accounting with `node work/wave238/laneF_accounting.mjs`; per-wave detail is
> in the `docs/COVERAGE_WAVE228_TABLES.md` through
> `docs/COVERAGE_WAVE237_AUDIT.md` docs.
>
> **Wave 231 rescan supersedes every count below.** Wave 231 anchored the scan
> on `vtable - 8`, which reclassifies most of the large "state tables" as
> vtables and moves 611 runs out of tier 3. Read the revision section first.

---

## Wave 249 lane F correction - the 311-slot `WM_PLAYER_VTABLE` span

> **This supersedes the 275-word / `0x80326BE8` figures for `0x8032679C` in
> every table and section below.** The method array is **311 slots**, spanning
> `0x8032679C..0x80326C78` (1,244 B). The 275-word figure is the length of the
> *maximal pointer run*, which stops at the zero pair `0x80326BE8` /
> `0x80326BEC`; it is not the end of the vtable. The 311-slot grid is a family
> of overlapping vtables, and the structure that follows it at `0x80326D0C` is
> a separate descriptor array, not more methods.

### The slot grid

| words | contents | evidence |
|---|---|---|
| `0x8032679C..0x80326BE8` | 275 methods of the primary vtable, header `0x80326794`, installed at object `+0x60` | `wm_player_init` (`WM_PLAYER_VTABLE`); `lis`/`addi` of `0x80326794` at `0x8014B30C`/`0x8014B314`, `stw r4,0x60(r27)` at `0x8014B318` |
| `0x80326BE8..0x80326BF0` | zero offset-to-top / RTTI header of a secondary vtable | `lis`/`addi` of `0x80326BE8` at `0x8014B378`/`0x8014B380`, `stw r3,0x14E0(r27)` at `0x8014B384` |
| `0x80326BF0..0x80326C78` | the secondary vtable's 34 slots (27 function words: 3 tabled, 24 untabled, 7 null); slots 277-310 of the 311-slot numbering, slots 0-33 of the secondary vtable; owns 2 corpus entries (`0x80153240`, `0x8014B430`, both translated in wave 248 lane B) | run dump; `work/wave248/laneB_report.md` |
| `0x80326C78..0x80326D0C` | 37 zero words (slots 311-347) | `work/wave248/laneB_words.mjs 80326c78 37` |

Two further zero pairs, `0x80326C18`/`0x80326C1C` and `0x80326C48`/`0x80326C4C`,
begin later sub-tables inside the same grid: `wm_player_init` stores
`0x80326C18` at `+0x14E0`, `0x80326C48` at `+0x14E8` and the null word
`0x80326C60` at `+0x14EC` (the `WM_TABLE_80326C18` / `WM_TABLE_80326C48` /
`WM_TABLE_80326C60` constants in `src/game/worldmap.rs`). Census over the
full 311 slots: **37 tabled / 265 untabled function words / 9 null words**
(302 pointers, 293 unique targets), compared with 34 / 241 / 0 over the
275-word run; the 24 extra untabled function words are the secondary vtable's
`0x80326BF0` slots and the 9 nulls are the grid's padding words (slots 275,
276, 287, 288, 299, 300 and 304-306).

### `0x80326D0C` - twelve-entry `<0xFFFFFFFF, handler, 0>` descriptor array

| field | value |
|---|---|
| span | `0x80326D0C..0x80326D9C` (144 B: 12 records of 3 words) |
| record | `0xFFFFFFFF`, handler, `0` |
| handlers | `0x8014BBE0`, `0x8014BBF0`, `0x8014BAE0`, `0x8014BFB0`, `0x8014C020`, `0x8014BE50`, `0x8014C200`, `0x8014C240`, `0x8014C170`, `0x8014C400`, `0x8014C410`, `0x8014C3A0` |
| tabled | 10 of 12; `0x8014BBE0` and `0x8014C400` are untabled, and only `0x8014BBE0` is named in `src` (the wave-248 lane B module note) |
| reachability | unreferenced: no `lis`/`addi` pair anywhere in `main.dol` names `0x80326D0C` or `0x80326D04`, and no data word holds either address |
| neighbours | 33 `{name_offset, 0x60, 0}` records at `0x80326D9C..0x80326F28` (name offsets `0x2A4..0x30C`), then the next `-1`-keyed array at `0x80326F28` |

The record form is the one wave 248 lane A documented in the `0x80325878` block
of the same world-map object: `{0xFFFFFFFF, handler, 0}` virtual-override records
next to `{name_offset, 0x60, 0}` name records. `0x80326D0C` is therefore its own
descriptor table of that family, not a continuation of `WM_PLAYER_VTABLE`. None
of the twelve handler addresses is a wave-226 corpus entry, so the 2,457-entry
census and every tier total are unchanged by this table.

---

## Wave 231 revision - vtable-aware rescan (supersedes every number below)

> **Read this section instead of the wave-227 counts.** Ownership, call-site and
> unreferenced accounting are unchanged; the *kind* and *reachability* of most
> groups are not. 432 groups moved from tier 3 to tier 1 and 179 from tier 3 to
> tier 2 once the scan also looked at `base - 8`.

### Why the wave-227 scan under-counted

Almost every large "state table" in this document is a CodeWarrior **vtable
method array**, not a flat handler array. The vtable is a two-word header
(observed as `{0, 0}`) followed by the methods; objects store the *header*
address at `+0x60` and dispatch as `[[obj+0x60] + 8 + 4n]`, so slot `n` is the
word at `header + 8 + 4n`. Wave 227 anchored on that first method word
(`header + 8`) and searched only for references to it. Constructors,
destructors and translated `src` doc comments all name `header`, never the
method array, so the scanner classified installed vtables as unreferenced and
its per-group `entries` column counted only the prologue-swept subset of the
methods. Wave 230 lane F proved the layout on `0x80314060`; this revision
applies the same test to the whole corpus.

Two consequences:

- **Structure.** 381 runs are confirmed vtable method arrays (a two-zero
  header and at least three methods) and own 1,423 of the 1,950 table-owned
  entries; another 179 runs (267 entries) are named at `base - 8` but
  have an ordinary data word there instead of a zero header. Together
  560 runs / 1,690 entries (86.7 %) are vtable
  dispatch. 521 runs have the two-zero header immediately before them
  and 524 are named at `base - 8` by a `lis`+`addi`/`ori` sequence or a
  `src` literal (475 satisfy both).
- **Reachability.** A run is now tier 1/2 if `base` **or** `base - 8` is named
  by translated code / untabled code / a data pointer. That moves
  611 real tables (1,582 entries, 361,144 B) out of tier 3.

### Method

`work/wave231/laneA_scan.mjs` reuses the wave-227 implementation verbatim
(same `work/wave226/laneC_untabled_entries.csv` corpus of 2,457 entries, same
strict maximal-run model, same primary-assignment score, same call-site and
unreferenced groups) and adds one probe:

1. `base - 8` is read from the same loaded segment. A **vtable anchor** is
   accepted when the two words at `base - 8`/`base - 4` are not code pointers.
2. A run is labelled `vtable` when a `lis`+`addi`/`ori` site or a `src` literal
   names `base - 8`, or when those two words are both zero and the array is not
   an ascending constructor list (`.ctors` is the only such list).
3. Tier 1 = `base` or `base - 8` named by a translated body (`src` literal or
   a code ref inside a `functions.tsv` function). Tier 2 = same, from untabled
   code, a live data pointer to `base`/`base - 8`, or a live neighbour within
   `0x100`. Tier 3 = nothing.
4. Per-group slots are censused over the **whole run**: `slots` = words,
   `tabled` = slot values present in any `work/ghidra/*/export/functions.tsv`
   (wave-230 lane F convention), `untabled` = the rest.
5. "Translated since wave 227" is the wave-227 tier-1 test applied to each
   corpus entry: the address appears as a literal in `src/**/*.rs`. It is an
   upper bound (a mention can be prose), so remaining counts are conservative.

Machine-readable output: `work/wave231/laneA_groups.json`. Ranked worklist:
`node work/wave231/laneA_scan.mjs`; this section: `node work/wave231/laneA_doc.mjs`.
The numbers are a snapshot of the working tree at generation time. Parallel
lanes keep adding translated bodies to `src/`, so re-running can move a few
runs from tier 3 to tier 1 and lower the `remaining` columns; ownership,
run boundaries and slot censuses do not change.

### Corrected totals

| quantity | wave 227 | wave 231 (corrected) |
|---|---:|---:|
| corpus entries | 2,457 | 2,457 |
| untabled after 228-230 (address appears in `src`) | 2,373 (wave-229 note) | **1972** (485 now named in `src`: 222 table-owned, 11 call-only, 252 previously unreferenced) |
| table-owned entries | 1,950 | 1,950 (unchanged) |
| ... in vtable dispatch (method arrays + base-8-named runs) | not measured | **1690** in 560 runs (1423 in 381 confirmed method arrays) |
| call-site-only entries | 92 (72 sites) | 92 (72 sites, unchanged) |
| unreferenced entries | 415 | 415 (unchanged) |
| runs with entries | 826 | 826 (681 real + 145 inline slots) |
| runs with a two-zero header at `base - 8` | - | 521 |
| runs named at `base - 8` by code or `src` | - | 524 |
| method slots in real tables | - | 12,434 (10,201 untabled) |
| **tier 1 / 2 / 3 runs (real tables)** | **12 / 18 / 651** | **458 / 185 / 38** |
| tier 1 / 2 / 3 entries | - | 1,602 / 315 / 29 |
| tier 1 / 2 / 3 bytes | - | 409,256 / 72,160 / 6,236 |

Recomputing the *wave-227 rule itself* on today's tree gives 14 / 18 / 649
real tables rather than 12 / 18 / 651: waves 229-230 added `src` literals that
name `0x80325428` and `0x80314068` directly, so those two runs are now tier 1
even without the `base - 8` probe. The vtable-aware delta is measured against
this recomputed baseline where it matters.

### Tier movement

| transition | groups | entries | bytes |
|---|---:|---:|---:|
| tier 3 -> 1 | 432 | 1,283 | 294,600 |
| tier 3 -> 2 | 179 | 299 | 66,544 |
| tier 2 -> 1 | 12 | 18 | 2,632 |
| **out of tier 3** | **611** | **1,582** | **361,144** |

No run moved into tier 3. What remains in tier 3 is 38 small runs
(9 of them multi-owned only);
they are almost all single pointer words in data with no reference at all, and
the largest is a 1-word slot worth 820 B.

### Corrected per-group counts (runs with >= 10 corpus entries)

`slots`/`untabled` are the whole method array, so they replace the wave-227
`words` and `entries` columns. `entries`/`remaining` are the corpus subset
owned by the run and the part of it that no `src` literal names yet.

| table (method array) | header | kind | tier | slots | untabled | entries | remaining | remaining B | evidence |
|---|---:|---|---:|---:|---:|---:|---:|---:|---|
| `0x80325428` | `0x80325420` | vtable | 1 | 276 | 234 | 83 | 79 | 20,320 | `src/game/worldmap.rs:45905`; slots 91-114 alias `0x8032679c` |
| `0x802ed9e0` | `0x802ed9d8` | table | 1 | 181 | 142 | 76 | 41 | 22,752 | `src/boot/init_tables.rs:145` |
| `0x8032679c` | `0x80326794` | vtable | 1 (was 3) | 311 | 265 | 64 | 64 | 11,088 | `src/game/worldmap.rs:5052`; slots 91-114 alias `0x80325428`; 275 methods + the `0x80326BE8` secondary vtable, see the wave-249 correction |
| `0x80314068` | `0x80314060` | vtable | 1 | 306 | 281 | 59 | 41 | 8,224 | `src/game/enemies.rs:1396` |
| `0x803084d0` | `0x803084c8` | vtable | 1 (was 3) | 275 | 223 | 44 | 43 | 10,780 | `src/game/player.rs:21304` |
| `0x80301c00` | `0x80301bf8` | vtable | 1 (was 3) | 166 | 144 | 42 | 35 | 6,448 | `src/game/enemies.rs:68400` |
| `0x80313768` | `0x80313760` | vtable | 2 (was 3) | 235 | 205 | 33 | 28 | 7,080 | `-` |
| `0x80303104` | `0x803030fc` | vtable | 1 (was 3) | 187 | 168 | 31 | 29 | 7,092 | `src/ui/hud.rs:15939` |
| `0x80312750` | `0x80312748` | vtable | 2 (was 3) | 265 | 238 | 30 | 25 | 4,112 | `-` |
| `0x8030241c` | `0x80302414` | vtable | 1 (was 3) | 199 | 176 | 25 | 25 | 5,040 | `src/game/enemies.rs:18056` |
| `0x80304e18` | `0x80304e10` | vtable | 1 (was 3) | 186 | 161 | 23 | 22 | 2,952 | `src/game/enemies.rs:69388` |
| `0x80304940` | `0x80304938` | vtable | 1 (was 3) | 186 | 162 | 22 | 20 | 3,560 | `src/game/enemies.rs:93076` |
| `0x80305958` | `0x80305950` | vtable | 1 (was 3) | 190 | 164 | 22 | 20 | 7,480 | `src/ui/hud.rs:27596` |
| `0x80314534` | `0x8031452c` | table | 1 | 68 | 65 | 22 | 0 | 0 | `src/game/enemies.rs:202946` |
| `0x803017d8` | `0x803017d0` | vtable | 1 (was 3) | 92 | 79 | 21 | 14 | 1,736 | `src/game/bases.rs:43259` |
| `0x802f8ef0` | `0x802f8ee8` | table | 1 | 42 | 38 | 20 | 0 | 0 | `src/render/geometry.rs:88` |
| `0x80304494` | `0x8030448c` | vtable | 1 (was 3) | 173 | 151 | 20 | 20 | 3,424 | `src/game/enemies.rs:37887` |
| `0x80315ef0` | `0x80315ee8` | vtable | 1 (was 3) | 46 | 37 | 19 | 19 | 5,164 | `src/game/stage.rs:40133` |
| `0x80306400` | `0x803063f8` | vtable | 1 (was 3) | 193 | 172 | 17 | 17 | 3,044 | `src/game/enemies.rs:6697` |
| `0x803280e0` | `0x803280d8` | table | 1 | 32 | 20 | 15 | 0 | 0 | `src/engine/display.rs:21360` |
| `0x80305344` | `0x8030533c` | vtable | 1 (was 3) | 169 | 146 | 13 | 13 | 2,136 | `src/game/enemies.rs:7963` |
| `0x8031c288` | `0x8031c280` | vtable | 1 (was 3) | 42 | 37 | 13 | 13 | 2,072 | `src/engine/dsys.rs:26982` |
| `0x80323370` | `0x80323368` | vtable | 1 (was 3) | 158 | 137 | 13 | 13 | 2,892 | `src/game/worldmap.rs:22327` |
| `0x80305f18` | `0x80305f10` | vtable | 1 (was 3) | 185 | 162 | 12 | 11 | 1,300 | `src/game/enemies.rs:30864` |
| `0x80308a88` | `0x80308a80` | vtable? | 1 (was 3) | 44 | 33 | 11 | 11 | 2,176 | `src/engine/display.rs:3956` |
| `0x80311f90` | `0x80311f88` | vtable | 1 (was 3) | 226 | 202 | 11 | 10 | 3,304 | `src/game/stage.rs:37266` |
| `0x80329d78` | `0x80329d70` | vtable? | 1 (was 3) | 23 | 19 | 11 | 11 | 2,720 | `src/ui/menu.rs:7897` |
| `0x8032df1c` | `0x8032df14` | vtable | 1 (was 3) | 42 | 40 | 11 | 11 | 1,232 | `src/ui/lyt.rs:12082` |
| `0x8034e5e8` | `0x8034e5e0` | vtable | 1 (was 3) | 20 | 18 | 11 | 11 | 3,560 | `src/runtime/os.rs:17725` |
| `0x80301eb0` | `0x80301ea8` | vtable | 1 (was 3) | 185 | 161 | 10 | 8 | 1,656 | `src/game/bases.rs:244089` |

### Corrections to the twelve wave-227 tier-1 runs

| table | kind / header | slots | untabled | entries | remaining | wave-227 role, corrected |
|---|---|---:|---:|---:|---:|---|
| `0x802ed9e0` | table / `0x802ed9d8` | 181 | 142 | 76 | 41 | `.ctors` flat array; base-8 is padding, **not** a vtable header |
| `0x802f8ef0` | table / `0x802f8ee8` | 42 | 38 | 20 | 0 | six 7-entry gather-kernel tables (wave 228 lane F); base-8 is data |
| `0x80343e90` | table / `0x80343e88` | 32 | 29 | 9 | 0 | WPAD/KPAD callback table; base-8 is data |
| `0x80314534` | table / `0x8031452c` | 68 | 65 | 22 | 0 | `game::stage` object vtable (wave 229 lane A); base-8 is the last method of the neighbouring `dEn_c` array, so the base-8 probe does not apply |
| `0x80310db0` | vtable? / `0x80310da8` | 16 | 12 | 3 | 0 | vtable tail (wave 228 lane D); base-8 is a small offset record word, not a zero header |
| `0x803280e0` | table / `0x803280d8` | 32 | 20 | 15 | 0 | `dScBoot_c` per-frame task table (wave 228 lane C); base-8 is data |
| `0x802f8510` | table / `0x802f8508` | 8 | 8 | 6 | 0 | 8-way curve/jump table (wave 228 lane B); base-8 is data |
| `0x802ee830` | vtable? / `0x802ee828` | 1 | 1 | 1 | 0 | single pointer slot; base-8 is named but is a `-1`/offset sentinel word |
| `0x802ef328` | table / `0x802ef320` | 6 | 3 | 1 | 0 | player find/compare function table (wave 228 lane A); base-8 is data |
| `0x802f6d30` | table / `0x802f6d28` | 5 | 4 | 3 | 0 | HID-host method table; base-8 is data |
| `0x802ef8b0` | table / `0x802ef8a8` | 4 | 4 | 2 | 2 | stage probe constant/function table; base-8 is data |
| `0x80323ab0` | table / `0x80323aa8` | 1 | 1 | 1 | 1 | single slot; base-8 is data (no ref, still tier 1 by src literal) |

### Fresh prioritised top-20

Ranking: tier, then **remaining corpus entries** (measurable bodies that no
`src` literal names yet), then untabled method slots, then table address. The
wave-227 top 40 ranked by bytes, which is why it missed that the 300-slot
`dEn_c` vtable and its siblings are live, half-completed work items rather
than unreachable state tables.

| # | method array | header | tier | slots | untabled | entries | remaining | remaining B | role |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---|
| 1 | `0x80325428` | `0x80325420` | 1 | 276 | 234 | 83 | 79 | 20,320 | world-map (`Wm`) actor vtable, header `0x80325420`; `Wm_mr_*` state strings; slots 91-114 alias `0x8032679c` |
| 2 | `0x8032679c` | `0x80326794` | 1 | 311 | 265 | 64 | 64 | 11,088 | `WM_PLAYER_VTABLE` (`wm_player_init`), header `0x80326794`; slots 91-114 alias `0x80325428`; 311 slots end at `0x80326C78` (wave-249 correction) |
| 3 | `0x803084d0` | `0x803084c8` | 1 | 275 | 223 | 44 | 43 | 10,780 | player/actor class vtable stored at `+0x60` by `0x800460D0`, header `0x803084C8` |
| 4 | `0x80314068` | `0x80314060` | 1 | 306 | 281 | 59 | 41 | 8,224 | `dEn_c` enemy vtable opened by wave 230 lane F, header `0x80314060` |
| 5 | `0x802ed9e0` | `0x802ed9d8` | 1 | 181 | 142 | 76 | 41 | 22,752 | `.ctors` C++ static-initialiser array - **not** a vtable (181 ascending pointers) |
| 6 | `0x80301c00` | `0x80301bf8` | 1 | 166 | 144 | 42 | 35 | 6,448 | `daBullet_c` vtable, temporary header `0x80301BF8` |
| 7 | `0x80303104` | `0x803030fc` | 1 | 187 | 168 | 31 | 29 | 7,092 | `dokan`/pakkun actor final vtable, header `0x803030FC` |
| 8 | `0x8030241c` | `0x80302414` | 1 | 199 | 176 | 25 | 25 | 5,040 | `daEnBlockMain_c` vtable, header `0x80302414` |
| 9 | `0x80304e18` | `0x80304e10` | 1 | 186 | 161 | 23 | 22 | 2,952 | `daEnKuriboBase_c` vtable, header `0x80304E10` |
| 10 | `0x80305958` | `0x80305950` | 1 | 190 | 164 | 22 | 20 | 7,480 | `daEnObjCoinBlock_c` vtable, header `0x80305950` |
| 11 | `0x80304940` | `0x80304938` | 1 | 186 | 162 | 22 | 20 | 3,560 | `daEnJimenPakkunBase_c` vtable, header `0x80304938` |
| 12 | `0x80304494` | `0x8030448c` | 1 | 173 | 151 | 20 | 20 | 3,424 | `daEnDpakkunBase_c` vtable, header `0x8030448C` |
| 13 | `0x80315ef0` | `0x80315ee8` | 1 | 46 | 37 | 19 | 19 | 5,164 | engine::display model-actor vtable installed by `0x800BCB80`, header `0x80315EE8` |
| 14 | `0x80306400` | `0x803063f8` | 1 | 193 | 172 | 17 | 17 | 3,044 | `daEnSuperBigPile_c` vtable, header `0x803063F8` |
| 15 | `0x803017d8` | `0x803017d0` | 1 | 92 | 79 | 21 | 14 | 1,736 | `daBossDemo_c` base vtable, header `0x803017D0` |
| 16 | `0x80305344` | `0x8030533c` | 1 | 169 | 146 | 13 | 13 | 2,136 | `daEnLkuriboBase_c` vtable, temporary header `0x8030533C` |
| 17 | `0x80323370` | `0x80323368` | 1 | 158 | 137 | 13 | 13 | 2,892 | `daEnEatCoin_c` vtable installed by `fn_80110170`, header `0x80323368` |
| 18 | `0x8031c288` | `0x8031c280` | 1 | 42 | 37 | 13 | 13 | 2,072 | engine::dsys object vtable installed by `fn_800fe1d0`, header `0x8031C280` |
| 19 | `0x80305f18` | `0x80305f10` | 1 | 185 | 162 | 12 | 11 | 1,300 | `daEnShell_c` vtable, base table header `0x80305F10` |
| 20 | `0x8032df1c` | `0x8032df14` | 1 | 42 | 40 | 11 | 11 | 1,232 | nw4hbm::lyt window-manager vtable at `+0xF8`, header `0x8032DF14` |

### Caveats

- **Ownership did not change.** `1,950` table-owned / `92` call-site-only /
  `415` unreferenced are the same sets as wave 227; only kind, counts and
  reachability changed. The `367` multi-run entries are still counted once.
- **The corpus is frozen at wave 226** so the delta is comparable. 485 of the
  2,457 entries are already named somewhere in `src` (222 table-owned, 11
  call-site-only, 252 previously unreferenced - the last group being secondary
  entry points of bodies translated in waves 228-230). Those are excluded from
  the `remaining` columns; `namedInSrc` uses the same literal test as tier 1,
  so it is an upper bound on translation coverage.
- **49 runs are named at `base - 8` without a zero header.** These are the
  least certain vtable calls: the `lis`/`addi` names the address, but the two
  words are ordinary data (a typeinfo/offset header, or the previous table).
  Each is listed in the JSON with `vtableRef: true, zeroHeader: false`.
- **36 confirmed method arrays have no reference at all** (two-zero header, no
  `base - 8` name anywhere). They stay tier 2/3 and are the best candidates for
  reachability through REL modules or runtime-built dispatch.
- **`0x802ed9e0` is not a vtable.** It is the 181-word ascending `.ctors`
  array; the two zero words before it are padding. It is shown as `table` in
  this revision and keeps its wave-227 role and tier.
- **`0x80314534` is a vtable** (wave 229 lane A) but its `base - 8` is the last
  method word of the preceding 306-slot `dEn_c` array; the probe cannot apply
  and the run keeps its own `src` evidence.
- The remaining `words`-based byte totals below are body-size sums, not table
  sizes; slot counts are the exact table sizes.

## Method and ranking

> *Legacy wave-227 method. The wave-231 revision above adds the `base - 8`
> probe and the full-run slot census; the rules below describe the original
> scan only.*

1. Every 4-byte word of every `PT_LOAD` segment is read. A word is a **text
   pointer** when it lies in `0x80004000..0x802ED9E0` and is 4-byte aligned.
   A maximal run of consecutive text pointers is a **table run**; a non-pointer
   word (usually `0x00000000` padding or a `-1` sentinel) always ends a run, so
   descriptors that embed one code pointer are not glued onto the next array.
   Run length (`words`) is exact; the byte span is `words * 4`.
2. An entry is **owned** by a run when any word inside that run equals its
   address. If several runs contain it, the run with live code evidence wins,
   then a real table, then the larger run.
3. **Reachability tiers** (used as the primary sort key):
   - **tier 1** - the run base appears in a `lis`+`addi`/`ori` sequence in
     `main.dol` text, or as a literal in `src/**/*.rs` (i.e. a translated body
     already names the table).
   - **tier 2** - no translated body loads the base, but one of: an untabled
     body loads it (`gap` ref), the base is stored as a pointer inside data, or
     another table within 0x100 B is tier 1 (adjacent array in the same
     descriptor block).
   - **tier 3** - no reference found in `main.dol` text or in any translated
     `src` literal. These may still be reachable from REL modules whose view of
     the table was not reconstructed, or only from the untabled bodies
     themselves.
4. Within a tier, groups are sorted by the summed prologue-to-`blr` size of the
   entries they own (**bytes**), then by entry count.
5. Role guesses come from, in order: the doc comment already attached to the
   table address in `src/`, the module of the neighbouring translated slots,
   and printable strings within +-512 B of the table.

## Ranked worklist (top 40)

> *Superseded by the wave-231 top-20 above. Kept for traceability: the tier and
> `entries` columns here are the wave-227 numbers, and most `table` rows are
> really vtable method arrays.*

| # | table | tier | kind | words | entries | bytes | role |
|---:|---|---:|---|---:|---:|---:|---|
| 1 | `0x802ed9e0` | 1 | table | 181 | 76 | 58612 | DOL constructor table (`.ctors`) |
| 2 | `0x802f8ef0` | 1 | table | 42 | 20 | 5272 | nw4r scene-class tables |
| 3 | `0x80343e90` | 1 | table | 32 | 9 | 4840 | WPAD/KPAD extension callback table |
| 4 | `0x80314534` | 1 | table | 68 | 22 | 3584 | `game::stage` object vtable (68 slots) |
| 5 | `0x80310db0` | 1 | table | 16 | 3 | 1744 | tail of the collision-area vtable block |
| 6 | `0x803280e0` | 1 | table | 32 | 15 | 1112 | SDA-copied state word tables |
| 7 | `0x802f8510` | 1 | table | 8 | 6 | 804 | nw4r animation curve/kernel table (8 slots) |
| 8 | `0x802ee830` | 1 | slot | 1 | 1 | 696 | single `.sdata` pointer slot |
| 9 | `0x802ef328` | 1 | table | 6 | 1 | 400 | player-cluster function table (6 slots) |
| 10 | `0x802f6d30` | 1 | table | 5 | 3 | 204 | HID-host method table |
| 11 | `0x802ef8b0` | 1 | table | 4 | 2 | 200 | stage probe constant/function table (4 slots) |
| 12 | `0x80323ab0` | 1 | slot | 1 | 1 | 172 | single slot beside `daEnemyIce_c` state names |
| 13 | `0x802f8ed4` | 2 | table | 6 | 5 | 3996 | head of the nw4r scene-class table block |
| 14 | `0x80323b48` | 2 | table | 51 | 4 | 780 | state table: daEnemyIce_c |
| 15 | `0x80310ea0` | 2 | table | 10 | 4 | 744 | game::stage pointer array |
| 16 | `0x80310cfc` | 2 | table | 7 | 2 | 612 | resource::registry pointer array |
| 17 | `0x80328188` | 2 | table | 18 | 3 | 376 | state table: Pausewindow_c |
| 18 | `0x802f00e8` | 2 | table | 3 | 1 | 328 | 3-word `game::stage` table |
| 19 | `0x80323d18` | 2 | slot | 1 | 1 | 212 | single slot beside `daFireBall_Player_c` state names |
| 20 | `0x80323a80` | 2 | table | 11 | 2 | 184 | state table: daEnemyIce_c |
| 21 | `0x80328060` | 2 | table | 11 | 1 | 136 | state table: Pausewindow_c |
| 22 | `0x8031b080` | 2 | table | 3 | 3 | 116 | effect/allocator dispatch table (3 slots) |
| 23 | `0x80310d48` | 2 | table | 6 | 1 | 116 | game::stage pointer array |
| 24 | `0x80310cd8` | 2 | table | 7 | 1 | 100 | resource::registry pointer array |
| 25 | `0x80310e48` | 2 | slot | 2 | 1 | 100 | game::stage pointer array |
| 26 | `0x80310cb8` | 2 | table | 6 | 1 | 96 | resource::registry pointer array |
| 27 | `0x80310d20` | 2 | table | 7 | 1 | 96 | resource::registry pointer array |
| 28 | `0x80310d68` | 2 | table | 6 | 1 | 96 | resource::registry pointer array |
| 29 | `0x80310d88` | 2 | table | 6 | 1 | 96 | resource::registry pointer array |
| 30 | `0x80310e38` | 2 | slot | 1 | 1 | 64 | function-pointer array |
| 31 | `0x80325428` | 3 | table | 276 | 83 | 21596 | world-map (`Wm`) actor state table (276 slots) |
| 32 | `0x80314068` | 3 | table | 306 | 59 | 12788 | `dEn_c` enemy base-state table (306 slots) |
| 33 | `0x803084d0` | 3 | table | 275 | 44 | 11352 | player (Mario) state/action table (275 slots) |
| 34 | `0x8032679c` | 3 | table | 311 | 64 | 11088 | world-map + Yoshi state table (311 slots) |
| 35 | `0x80305958` | 3 | table | 190 | 22 | 8348 | `daEnObjCoinBlock_c` actor state table (190 slots) |
| 36 | `0x80301c00` | 3 | table | 166 | 41 | 8324 | `daBullet_c` state table (166 slots) |
| 37 | `0x80313768` | 3 | table | 235 | 33 | 7732 | `dEn_c` die-state table (235 slots) |
| 38 | `0x80303104` | 3 | table | 187 | 31 | 7636 | `dokan`/pakkun fire state table (187 slots) |
| 39 | `0x8032b524` | 3 | table | 29 | 8 | 7632 | nw4r `lyt` pane vtables (29 slots) |
| 40 | `0x80327250` | 3 | table | 19 | 5 | 6848 | `daYoshi_c` state table (19 slots) |

## Tier 1 - tables already named by translated code

Highest priority: a translated body already loads these bases, so the untabled
slots are the missing methods behind a live vtable/table.

### `0x802ed9e0` - 181 words, 76 untabled entries, 58612 B

- Span: `0x802ed9e0..0x802edcb4` (724 B of pointer data; target code 58612 B)
- Untabled entries (76): 0x8001d2c0 0x8001ec70 0x800207a0 0x800233b0 0x80026140 0x80026e20 0x8002a230 0x8002b8c0 0x8002c410 0x8002e660 0x80030ab0 0x80032ab0 0x80034e10 0x80036390 0x8003d4b0 0x8003fe10 0x80042f20 0x80044c70 0x8005d750 0x80061310 0x80061b60 0x80062980 0x80063c40 0x8006c020 0x8007ec20 0x80087100 0x8008f8c0 0x80094950 0x8009f070 0x800a1e20 0x800a6a00 0x800a82c0 0x800aed40 0x800b1490 0x800b5840 0x800cd6c0 0x800cd9e0 0x800cde70 0x800d08b0 0x800d7050 0x800f2300 0x800f2a60 0x800f5f40 0x800f6eb0 0x800f91e0 0x800fd500 0x800fd9c0 0x800fdfa0 0x80101430 0x80101720 0x80102e80 0x80103730 0x80103a30 0x80105f40 0x80106d60 0x8010aef0 0x8010fc90 0x80114440 0x80115510 0x8011a380 0x8011b020 0x801221f0 0x80124680 0x80147bd0 0x801573b0 0x8015f940 0x8016c190 0x8016f190 0x8016f670 0x80190f00 0x80233110 0x8024da70 0x8027b5a0 0x802be3d0 0x802be5e0 0x802d1ce0
- Translated slots already in this run: 39 (modules: runtime::cpp 1, ui::hud 8, game::player 3, game::stage 7, engine::frame 1, engine::display 7, engine::dsys 4, engine::globals 1, game::worldmap 4, resource::registry 1, render::gx_band 2)
- Evidence: 1 code ref(s) from translated bodies: 0x801b89a0 (src/runtime/os.rs); src: src/runtime/bt.rs:849, src/runtime/os.rs:1351; base stored at 0x80006648, 0x8000664c; 39 known tsv slots
- Role: **DOL constructor table (`.ctors`)** - 181 code pointers (180 ascending) from `0x802ED9E0` to `0x802EDCB0`, then three zero words at `0x802EDCB4..0x802EDCC0`; walked by the `__init_cpp` loop that `src/runtime/os.rs` documents (its `INIT_TABLE` note now records the same 181 pointers + 3 zeros). The 76 untabled entries are C++ static initialisers (the tabled ones read `Builds the ... state descriptors`, `Registers the ... handler`), so this is 58.6 KB of global-registration code.

### `0x802f8ef0` - 42 words, 20 untabled entries, 5272 B

- Span: `0x802f8ef0..0x802f8f98` (168 B of pointer data; target code 5272 B)
- Untabled entries (20): 0x80254c20 0x80254d00 0x80254dc0 0x80255020 0x80255180 0x80255320 0x80255500 0x80255930 0x80255a00 0x80255d10 0x80255e50 0x80255f90 0x80256150 0x80256560 0x80256650 0x80256710 0x80256840 0x80256af0 0x80256c40 0x80256e20
- Translated slots already in this run: 4 (modules: runtime::os 4)
- Evidence: 1 code ref(s) from translated bodies: 0x802556e0 (src/runtime/os.rs); src: src/runtime/os.rs:130, src/runtime/os.rs:19094; 4 known tsv slots
- Role: **nw4r scene-class tables** - `src/runtime/os.rs` documents "six 7-entry tables at `0x802F8EF0`..`0x802F8F98`"; adjacent strings are `ScnObj`, `ScnLeaf`, `ScnGroup`, `ScnRoot`, `ScnMdlSimple`. Entries are the per-class matrix/scene virtual bodies.
- Nearby strings: `ScnObj`, `ScnLeaf`, `ScnGroup`, `ScnRoot`
- **Wave 228 lane F correction:** the tables hold 42 slots with **42 unique
  targets** (no repeats), 4 of them tabled, so **38 untabled** bodies - not 20.
  The 18 the list above omits are complete bodies with their own `bclr`. Measured
  code extents sum to 7,920 B (not 5,272 B). All 38 are translated in the
  Wave 228 lane F section of `src/render/geometry.rs`; per-slot map in
  `work/wave228/laneF_tables.md`.

### `0x80343e90` - 32 words, 9 untabled entries, 4840 B

- Span: `0x80343e90..0x80343f10` (128 B of pointer data; target code 4840 B)
- Untabled entries (9): 0x801e6d10 0x801e7380 0x801e83e0 0x801e8700 0x801e8850 0x801e8ee0 0x801e9290 0x801e93b0 0x801e9570
- Translated slots already in this run: 3 (modules: runtime::input 3)
- Evidence: 4 code ref(s) from translated bodies: 0x801e60c8 (src/runtime/input.rs), 0x801e6608 (src/runtime/input.rs), 0x801e6c68 (src/runtime/input.rs), 0x801e7068 (src/runtime/input.rs); 1 ref(s) from untabled code: 0x801e7398; src: src/runtime/input.rs:253, src/runtime/input.rs:2222; 3 known tsv slots
- Role: **WPAD/KPAD extension callback table** - `src/runtime/input.rs` names "table `0x80343E90`" in its module doc; four `lis`/`addi` refs in translated bodies (`0x801E60C8`, `0x801E6608`, `0x801E6C68`, `0x801E7068`) install slots and a fifth ref (`0x801E7398`) sits in untabled code. Adjacent strings are the `WARNING: ... KPAD` diagnostics.
- Nearby strings: `or device handle not assigned to channel.`, `HID Parser reports: %d`, `WARNING: Overwritten the callback needed by KPAD.`, `         Please call KPADSetSamplingCallback instead of WPADSetSamplingCallback.`

### `0x80314534` - 68 words, 22 untabled entries, 3584 B

- Span: `0x80314534..0x80314644` (272 B of pointer data; target code 3584 B)
- Untabled entries (22): 0x800a8fe0 0x800aa8b0 0x800aa980 0x800aaa10 0x800aaa80 0x800aaaf0 0x800aab80 0x800aabf0 0x800aad70 0x800ab0c0 0x800ab150 0x800ab210 0x800ab340 0x800ab4e0 0x800ae580 0x800ae670 0x800ae750 0x800ae830 0x800ae8b0 0x800ae9a0 0x800aeaa0 0x800aeb80
- Translated slots already in this run: 3 (modules: game::stage 3)
- Evidence: src: src/game/stage.rs:77335, src/game/stage.rs:77425; 3 known tsv slots
- Role: **`game::stage` object vtable (68 slots)** - `src/game/stage.rs` (line 77335) documents "vtable slots of the object behind the `0x80314534` vtable (slots 21 and 2x)". 22 untabled slots are remaining virtual methods of that class.

### `0x80310db0` - 16 words, 3 untabled entries, 1744 B

- Span: `0x80310db0..0x80310df0` (64 B of pointer data; target code 1744 B)
- Untabled entries (3): 0x8008cba0 0x8008cda0 0x8008cf80
- Translated slots already in this run: 4 (modules: game::stage 4)
- Evidence: 1 code ref(s) from translated bodies: 0x8008c888 (src/game/stage.rs); src: src/game/stage.rs:3860, src/game/stage.rs:3862; 4 known tsv slots
- Role: **tail of the collision-area vtable block** - entries sit immediately before `COLLISION_AREA_VTABLE = 0x80310DF0` (`src/game/stage.rs`); the three untabled targets `0x8008CBA0`, `0x8008CDA0`, `0x8008CF80` are collision-area methods.
- Nearby strings: `obj_coin`, `g3d/obj_coin.brres`, `obj_coin_blue`, `MH_model`

### `0x803280e0` - 32 words, 15 untabled entries, 1112 B

- Span: `0x803280e0..0x80328160` (128 B of pointer data; target code 1112 B)
- Untabled entries (15): 0x800e4fb0 0x800e50d0 0x8015b970 0x8015b9c0 0x8015ba10 0x8015ba60 0x8015baf0 0x8015bb30 0x8015bcb0 0x8015bd40 0x8015bd80 0x8015be20 0x8015bea0 0x8015bee0 0x8015c090
- Translated slots already in this run: 12 (modules: engine::display 7, engine::dsys 1, game::boot_scene 4)
- Evidence: src: src/engine/display.rs:21360; 12 known tsv slots
- Role: **SDA-copied state word tables** - `src/engine/display.rs` documents "two word tables copied into the SDA area at `r13-0x5338` (count 3) and `r13-0x5328` (count 0x20)". Adjacent strings are `Pausewindow_c::StateID_*`, so the 15 untabled entries are pause-window / boot-scene state handlers.
- Nearby strings: `Pausewindow_c::StateID_InitWait`, `Pausewindow_c::StateID_OpenAnimeEndWait`, `Pausewindow_c::StateID_ButtonChangeAnimeEndWait`, `Pausewindow_c::StateID_PauseDisp`

### `0x802f8510` - 8 words, 6 untabled entries, 804 B

- Span: `0x802f8510..0x802f8530` (32 B of pointer data; target code 804 B)
- Untabled entries (6): 0x802412a0 0x80241330 0x802413a0 0x80241490 0x80241530 0x802415d0
- Evidence: 1 code ref(s) from translated bodies: 0x802416e4 (src/runtime/os.rs)
- Role: **nw4r animation curve/kernel table (8 slots)** - loaded by the f64 curve evaluator `fn_80241690` (`src/runtime/os.rs`: "one basis/curve read per branch"); the six untabled targets `0x802412A0..0x802415D0` are the per-basis interpolation kernels.
- Nearby strings: `h'?QM@<`, `?XHZ<`, `vu<908`, `?;.<`

### `0x802ee830` - 1 words, 1 untabled entries, 696 B

- Span: `0x802ee830..0x802ee834` (4 B of pointer data; target code 696 B)
- Untabled entries (1): 0x80095950
- Evidence: src: src/game/enemies.rs:65996
- Role: **single `.sdata` pointer slot** - `src/game/enemies.rs` names the surrounding record (`EN66_DAT_802EE810..8`); the slot holds the untabled `0x80095950` (696 B), an enemy-66 callback.
- Nearby strings: `\)<eB>333=`

### `0x802ef328` - 6 words, 1 untabled entries, 400 B

- Span: `0x802ef328..0x802ef340` (24 B of pointer data; target code 400 B)
- Untabled entries (1): 0x80064880
- Translated slots already in this run: 3 (modules: game::player 3)
- Evidence: 1 code ref(s) from translated bodies: 0x80064720 (src/game/player.rs); src: src/game/player.rs:35340, src/game/player.rs:35379; 3 known tsv slots
- Role: **player-cluster function table (6 slots)** - inside the `0x80064600..0x80064A30` cluster documented in `src/game/player.rs` (wave 211); the one untabled target is `0x80064880`.

### `0x802f6d30` - 5 words, 3 untabled entries, 204 B

- Span: `0x802f6d30..0x802f6d44` (20 B of pointer data; target code 204 B)
- Untabled entries (3): 0x801f8da0 0x801f8e10 0x801f8e50
- Translated slots already in this run: 1 (modules: runtime::input 1)
- Evidence: 1 code ref(s) from translated bodies: 0x801f9e34 (src/runtime/input.rs); src: src/runtime/input.rs:11316, src/runtime/input.rs:11318; 1 known tsv slots
- Role: **HID-host method table** - `src/runtime/input.rs` documents "`0x802F6D30`: pointer table handed to the HID-host transport as its method table by `fn_801f9e10`". 3 untabled transport callbacks.
- Nearby strings: `/shared2/sys`, `/shared2/sys/SYSCONF`, `/title/00000001/00000002/data/setting.txt`

### `0x802ef8b0` - 4 words, 2 untabled entries, 200 B

- Span: `0x802ef8b0..0x802ef8c0` (16 B of pointer data; target code 200 B)
- Untabled entries (2): 0x8007c950 0x8007c9c0
- Evidence: 1 code ref(s) from translated bodies: 0x8007cb68 (src/game/stage.rs); src: src/game/stage.rs:53369, src/game/stage.rs:53370
- Role: **stage probe constant/function table (4 slots)** - referenced from `0x8007CB68` in the `fn_800717A0` probe cluster (`src/game/stage.rs`); untabled targets `0x8007C950`, `0x8007C9C0`.
- Nearby strings: `=q?8Q`, `1 1(1`, `0 0,000`

### `0x80323ab0` - 1 words, 1 untabled entries, 172 B

- Span: `0x80323ab0..0x80323ab4` (4 B of pointer data; target code 172 B)
- Untabled entries (1): 0x80115c20
- Evidence: 1 code ref(s) from translated bodies: 0x80116358 (src/game/worldmap.rs)
- Role: **single slot beside `daEnemyIce_c` state names** - target `0x80115C20`; adjacent strings `daEnemyIce_c::StateID_Freeze/Revival/Melt/Break`.
- Nearby strings: `daEnemyIce_c::StateID_Freeze`, `daEnemyIce_c::StateID_Revival`, `daEnemyIce_c::StateID_Melt`, `daEnemyIce_c::StateID_Break`

## Tier 2 - untabled-code reference / data pointer / live neighbour

- `0x802f8ed4` (6 words, 5 entries, 3996 B, tier 2) - **head of the nw4r scene-class table block** - 6-slot run immediately before the documented `0x802F8EF0` six-table block; same `ScnObj`/`ScnLeaf` strings, so it is the leading `ScnRoot`/type-info slot group.. 0x8024ebd0 0x8024f1a0 0x8024f4b0 0x8024f8c0 0x8024fbc0
- `0x80323b48` (51 words, 4 entries, 780 B, tier 2) - state-handler table for `daEnemyIce_c` (state-name strings adjacent); known slots resume in game::player (8), game::worldmap (1). 0x80115dc0 0x80115e20 0x80115ec0 0x8011a2e0
- `0x80310ea0` (10 words, 4 entries, 744 B, tier 2) - game::stage method/function-pointer table; known slots resume in game::stage (1), resource::registry (1). 0x80160300 0x801604b0 0x80160540 0x801608b0
- `0x80310cfc` (7 words, 2 entries, 612 B, tier 2) - resource::registry method/function-pointer table; known slots resume in resource::registry (1). 0x8008b2f0 0x8008c030
- `0x80328188` (18 words, 3 entries, 376 B, tier 2) - state-handler table for `Pausewindow_c` (state-name strings adjacent); known slots resume in game::scene (3), engine::display (1), game::boot_scene (1). 0x8015c370 0x8015c410 0x8015c560
- `0x802f00e8` (3 words, 1 entry, 328 B, tier 2) - **3-word `game::stage` table** - referenced from `0x8008C488`; the two untabled targets `0x8008CBA0`/`0x8008CC50` are stage helpers in the same band as the collision-area code.. 0x8008cc50
- `0x80323d18` (1 words, 1 entry, 212 B, tier 2) - **single slot beside `daFireBall_Player_c` state names** - target `0x8011A470`; adjacent strings are `daFireBall_Player_c::StateID_Move` and `Wm_mr_fireball_hit`, i.e. the world-map fireball actor.. 0x8011a470
- `0x80323a80` (11 words, 2 entries, 184 B, tier 2) - state-handler table for `daEnemyIce_c` (state-name strings adjacent); known slots resume in game::worldmap (3), game::task (2). 0x80115970 0x80115a00
- `0x80328060` (11 words, 1 entry, 136 B, tier 2) - state-handler table for `Pausewindow_c` (state-name strings adjacent); known slots resume in engine::display (3), game::task (2). 0x8015b8a0
- `0x8031b080` (3 words, 3 entries, 116 B, tier 2) - **effect/allocator dispatch table (3 slots)** - referenced from `0x800E534C`; adjacent strings `dSys_c::RootHeapMEM1/2` and `/Effect/effect_wnmario.breff`.. 0x800e4db0 0x800e4de0 0x800e4e10
- `0x80310d48` (6 words, 1 entry, 116 B, tier 2) - game::stage method/function-pointer table; known slots resume in game::stage (1). 0x8008c0b0
- `0x80310cd8` (7 words, 1 entry, 100 B, tier 2) - resource::registry method/function-pointer table; known slots resume in resource::registry (1). 0x8008bfc0
- `0x80310e48` (2 words, 1 entry, 100 B, tier 2) - game::stage method/function-pointer table; known slots resume in game::stage (1). 0x8008f380
- `0x80310cb8` (6 words, 1 entry, 96 B, tier 2) - resource::registry method/function-pointer table; known slots resume in resource::registry (1), game::stage (1). 0x8008bf50
- `0x80310d20` (7 words, 1 entry, 96 B, tier 2) - resource::registry method/function-pointer table; known slots resume in resource::registry (1). 0x80089b00
- `0x80310d68` (6 words, 1 entry, 96 B, tier 2) - resource::registry method/function-pointer table; known slots resume in resource::registry (1). 0x8008c130
- `0x80310d88` (6 words, 1 entry, 96 B, tier 2) - resource::registry method/function-pointer table; known slots resume in resource::registry (1). 0x8008c1a0
- `0x80310e38` (1 words, 1 entry, 64 B, tier 2) - function-pointer table. 0x8008f0d0

## Tier 3 - largest unowned state tables (no reference found yet)

> *Superseded: wave 231 moves 611 runs out of tier 3 and leaves only 38 real
> tables there (all single pointer words). The ten large tables in this section
> are vtables named at `base - 8`; see the corrected counts above.*

These are the biggest remaining blocks of prologue-verified code. They are
contiguous arrays of handler pointers (nearly all targets are untabled), each
next to its class's `StateID` name strings, which is why the role guesses are
strong even though the loader that indexes them has not been found. Treat them
as the bulk of the remaining `.text` gap.

| # | table | words | entries | bytes | role guess |
|---:|---|---:|---:|---:|---|
| 1 | `0x80325428` | 276 | 83 | 21596 | world-map (`Wm`) actor state table (276 slots) |
| 2 | `0x80314068` | 306 | 59 | 12788 | `dEn_c` enemy base-state table (306 slots) |
| 3 | `0x803084d0` | 275 | 44 | 11352 | player (Mario) state/action table (275 slots) |
| 4 | `0x8032679c` | 311 | 64 | 11088 | world-map + Yoshi state table (311 slots) |
| 5 | `0x80305958` | 190 | 22 | 8348 | `daEnObjCoinBlock_c` actor state table (190 slots) |
| 6 | `0x80301c00` | 166 | 41 | 8324 | `daBullet_c` state table (166 slots) |
| 7 | `0x80313768` | 235 | 33 | 7732 | `dEn_c` die-state table (235 slots) |
| 8 | `0x80303104` | 187 | 31 | 7636 | `dokan`/pakkun fire state table (187 slots) |
| 9 | `0x8032b524` | 29 | 8 | 7632 | nw4r `lyt` pane vtables (29 slots) |
| 10 | `0x80327250` | 19 | 5 | 6848 | `daYoshi_c` state table (19 slots) |
| 11 | `0x8034f9fc` | 33 | 6 | 6368 | nw4r `lyt` draw/arrange vtable block (33 slots) |
| 12 | `0x80312750` | 265 | 30 | 5644 | `dEnBoss_c` state table (265 slots) |
| 13 | `0x80315ef0` | 46 | 19 | 5164 | engine::display pointer array |
| 14 | `0x8030241c` | 199 | 25 | 5040 | state table: daEnBlockMain_c |
| 15 | `0x8032b2d0` | 8 | 5 | 4112 | ui::lyt pointer array |
| 16 | `0x8034fb40` | 10 | 4 | 3864 | resource::archive pointer array |
| 17 | `0x80304940` | 186 | 22 | 3808 | state table: daEnJimenPakkunBase_c |
| 18 | `0x8034e5e8` | 20 | 11 | 3560 | runtime::os pointer array |
| 19 | `0x80311f90` | 226 | 11 | 3440 | game::stage pointer array |
| 20 | `0x80350378` | 9 | 5 | 3232 | resource::archive pointer array |
| 21 | `0x80304e18` | 186 | 23 | 3160 | state table: daEnKuriboBase_c |
| 22 | `0x8030f15c` | 11 | 6 | 3392 | function-pointer array (11/11 covered after wave 282) |
| 23 | `0x80306400` | 193 | 17 | 3044 | state table: daEnSuperBigPile_c |
| 24 | `0x80304494` | 173 | 18 | 3032 | state table: daEnDpakkunBase_c |
| 25 | `0x80323370` | 158 | 13 | 2892 | state table: daEnEatCoin_c |
| 26 | `0x803503a8` | 5 | 3 | 2808 | resource::archive pointer array |
| 27 | `0x80329d78` | 23 | 11 | 2720 | ui::menu pointer array |
| 28 | `0x8034f790` | 5 | 2 | 2648 | function-pointer array |
| 29 | `0x8030150c` | 77 | 16 | 2628 | game::player pointer array |
| 30 | `0x80301eb0` | 185 | 10 | 2472 | game::stage pointer array |
| 31 | `0x8034fb80` | 7 | 6 | 2452 | function-pointer array |
| 32 | `0x8034eed8` | 2 | 1 | 2404 | function-pointer array |
| 33 | `0x8032b43c` | 27 | 4 | 2376 | ui::lyt pointer array |
| 34 | `0x8034f770` | 5 | 2 | 2272 | function-pointer array |
| 35 | `0x803294e0` | 14 | 6 | 2252 | render::gx_band pointer array |
| 36 | `0x8034f938` | 31 | 5 | 2240 | render::gx_band pointer array |
| 37 | `0x8034e8b0` | 16 | 2 | 2240 | runtime::os pointer array |
| 38 | `0x8034f728` | 4 | 2 | 2220 | function-pointer array |
| 39 | `0x8034f5e0` | 3 | 1 | 2208 | function-pointer array |
| 40 | `0x80308a88` | 44 | 11 | 2176 | runtime::cpp pointer array |

### `0x80325428` - 276 words, 83 untabled entries, 21596 B

- Span: `0x80325428..0x80325878`; 43 translated slots also live in this run
- Entries: 0x800e2550 0x80126510 0x801267d0 0x80126810 0x80126860 0x801268a0 0x80127050 0x80127170 0x80127240 0x801282e0 0x80128370 0x80129ca0 0x80129ea0 0x8012aa30 0x8012ab80 0x8012abf0 0x8012ac40 0x8012bd90 0x8012bf70 0x8012c220 0x8012c9b0 0x8012cb10 0x8012cba0 0x8012ce40 0x8012cf80 0x8012d250 0x8012d620 0x8012d7c0 0x8012da60 0x8012db90 0x8012dd10 0x8012dd70 0x8012edc0 0x8012edf0 0x8012f1a0 0x8012f270 0x801300e0 0x80131de0 0x80132230 0x80134ed0 0x80134fe0 0x80135d30 0x80139c50 0x80139c90 0x80139f30 0x8013ac90 0x8013ad30 0x8013b180 0x8013b5b0 0x8013b710 0x8013da60 0x8013db00 0x8013dc30 0x8013ddd0 0x8013fce0 0x8013fe00 0x8013fe80 0x8013ff70 0x80140300 0x801405e0 0x80140a30 0x80140c30 0x80140d50 0x801415b0 0x80142530 0x80143590 0x801438a0 0x80143a90 0x80143c30 0x80144aa0 0x80144d60 0x801450a0 0x80145440 0x80145520 0x80145a60 0x80145b50 0x80146060 0x801460f0 0x801462a0 0x801467b0 0x80146e90 0x801473f0 0x80147450
- Role: **world-map (`Wm`) actor state table (276 slots)** - contiguous handler array; adjacent strings are `Wm_mr_1upkira`, `Wm_mr_1upkira_s`, `Wm_mr_flaggetkira`; 83 untabled handlers cluster in `0x80126xxx..0x80147xxx` (game::worldmap). Almost certainly the state table of one `Wm` mode object; no loader reference reconstructed yet.
- Overlap: slots 91-114 (words `0x80325594..0x803255F0`, vtable `+0x174..+0x1D0`) hold the same function pointers as the `WM_PLAYER_VTABLE` run `0x8032679c` slots 91-114. Translate each target once; wave 237 lane C skipped this range.
- Nearby strings: `Wm_mr_1upkira`, `Wm_mr_1upkira_s`, `Wm_mr_1upkira_ss`

### `0x80314068` - 306 words, 59 untabled entries, 12788 B

- Span: `0x80314068..0x80314530`; 25 translated slots also live in this run
- Entries: 0x800a8b10 0x800a8c60 0x800a8d90 0x800a8e10 0x800a8ea0 0x800a9090 0x800a9140 0x800a9190 0x800a9280 0x800a9720 0x800a9810 0x800a9a90 0x800a9bf0 0x800a9d00 0x800a9ec0 0x800a9f70 0x800aa370 0x800aa540 0x800aa5e0 0x800aae80 0x800aaef0 0x800ab7c0 0x800ab870 0x800ab920 0x800ab9f0 0x800abb90 0x800abce0 0x800abdb0 0x800abf30 0x800ac0b0 0x800ac1a0 0x800ac240 0x800ac3a0 0x800ac470 0x800ac560 0x800ac7a0 0x800ac850 0x800aca40 0x800acb00 0x800acc20 0x800acd70 0x800ad000 0x800ad160 0x800ad1e0 0x800ad320 0x800ad3a0 0x800ad470 0x800ad4f0 0x800adce0 0x800ade50 0x800ae020 0x800ae130 0x800ae2d0 0x800ae320 0x800ae370 0x800ae3c0 0x800ae420 0x800ae510 0x800aec30
- Role: **`dEn_c` enemy base-state table (306 slots)** - adjacent strings `dEn_c::StateID_Ice/HitSpin/EatIn/EatNow`; 59 untabled handlers in `0x800A8B10..0x800AEC30` (game::stage), 22 translated slots also `game::stage`. The base-class state dispatch array for enemies.
- Nearby strings: `dEn_c::StateID_Ice`, `dEn_c::StateID_HitSpin`, `dEn_c::StateID_EatIn`

### `0x803084d0` - 275 words, 44 untabled entries, 11352 B

- Span: `0x803084d0..0x8030891c`; 52 translated slots also live in this run
- Entries: 0x80046bb0 0x800481c0 0x80049b60 0x80049bb0 0x80049e10 0x80049f50 0x80049fa0 0x8004a510 0x8004aa80 0x8004aac0 0x8004afe0 0x8004b090 0x8004b0d0 0x8004b220 0x8004c990 0x8004d8f0 0x8004d960 0x8004d9c0 0x8004da30 0x8004da90 0x8004e4d0 0x8004e7b0 0x8004e820 0x8004ff30 0x8004ffa0 0x800502a0 0x80050380 0x80050540 0x80050870 0x80050af0 0x80051ab0 0x80052730 0x80052880 0x80052900 0x80052fe0 0x80053090 0x80053f90 0x80053fe0 0x800576a0 0x80057760 0x80057b90 0x80058050 0x800580b0 0x800581e0
- Role: **player (Mario) state/action table (275 slots)** - all 52 translated slots in the run are `game::player`; 44 untabled handlers `0x80046BB0..0x800581E0`. Sits immediately after the 25-slot player array at `0x80308464` whose base the player code does load, so this is the continuation of the same player dispatch block.
- Nearby strings: `Wm_mr_quicksand`, `Wm_mr_sanddive_s`

### `0x8032679c` - 311 slots (275-word run), 64 untabled entries, 11088 B

- Span: `0x8032679c..0x80326c78` (311 slots; the maximal pointer run stops at
  `0x80326be8`, then the zero `0x80326be8` header and the 34-word `0x80326bf0`
  run continue the grid; see the wave-249 correction); 34 translated slots also
  live in this run
- Entries: 0x8014b580 0x8014b670 0x8014b920 0x8014c570 0x8014c730 0x8014c780 0x8014c930 0x8014cc30 0x8014d4a0 0x8014d5d0 0x8014d610 0x8014d6d0 0x8014d760 0x8014d820 0x8014d940 0x8014da20 0x8014dbb0 0x8014dc60 0x8014dda0 0x8014de00 0x8014e060 0x8014e270 0x8014e370 0x8014e570 0x8014e620 0x8014e660 0x8014e6b0 0x8014e820 0x8014e950 0x801508a0 0x80150b80 0x80150cd0 0x80150db0 0x80150e10 0x80150ee0 0x80150f60 0x80150fe0 0x80151270 0x801513d0 0x80151470 0x801518d0 0x80152360 0x801523a0 0x801523e0 0x80152440 0x80152560 0x80152640 0x80152680 0x801526d0 0x80152710 0x80152800 0x80152840 0x80152920 0x80152980 0x80152a00 0x80152ad0 0x80152c20 0x80152c80 0x80152de0 0x80152e50 0x80152fb0 0x801531a0 0x801531e0 0x80153210
- Role: **world-map + Yoshi state table (311 slots)** - adjacent strings `Wm_mr_yssweat`, `Wm_mr_yssweatrun`, `...teID_DemoCannonWarp`; 64 untabled handlers `0x8014B580..0x80153210` (game::worldmap). The `Wm`/Yoshi demo state array.
- Overlap: slots 91-114 (words `0x80326908..0x80326964`, vtable `+0x174..+0x1D0`) are the same targets as the WorldMap run `0x80325428` slots 91-114; this is why the two inventory entries are merged in the complete run inventory below.
- Nearby strings: `teID_DemoCannonWarp`, `g3d/ice.brres`, `Wm_mr_yssweatrun`

### `0x80305958` - 190 words, 22 untabled entries, 8348 B

- Span: `0x80305958..0x80305c50`; 26 translated slots also live in this run
- Entries: 0x80038670 0x80038830 0x800389e0 0x80039010 0x800392f0 0x800393a0 0x80039590 0x80039610 0x80039bf0 0x8003a010 0x8003a5d0 0x8003a7b0 0x8003aa60 0x8003ae40 0x8003aee0 0x8003b140 0x8003b200 0x8003b3e0 0x8003b830 0x8003bb40 0x8003bd60 0x8003bf90
- Role: **`daEnObjCoinBlock_c` actor state table (190 slots)** - adjacent strings `daEnObjCoinBlock_c::StateID_SerchRailID/SerchObjPolID/SerchWaterMoveID`; 22 untabled handlers `0x80038670..0x8003BF90`.
- Nearby strings: `daEnObjCoinBlock_c::StateID_SerchRailID`, `daEnObjCoinBlock_c::StateID_SerchObjPolID`, `daEnObjCoinBlock_c::StateID_SerchWaterMoveID`

### `0x80301c00` - 166 words, 41 untabled entries, 8324 B

- Span: `0x80301c00..0x80301e98`; 22 translated slots also live in this run
- Entries: 0x8001f900 0x8001fc50 0x8001fca0 0x8001fce0 0x8001fe80 0x8001ff30 0x8001ff60 0x80020700 0x80095580 0x80095890 0x80096910 0x80096a20 0x80096b30 0x80096c40 0x80096d60 0x80096df0 0x800973f0 0x800974f0 0x80097cb0 0x80097d30 0x8009f840 0x8009f9d0 0x8009fb50 0x8009fce0 0x8009fe40 0x8009ffd0 0x800a0150 0x800a03c0 0x800a10b0 0x800a1470 0x800a1600 0x800a1a10 0x800a1ac0 0x800a1bb0 0x800a1c00 0x800a1d90 0x800a7df0 0x800a7e90 0x800a7f40 0x800a8060 0x800a81e0
- Role: **`daBullet_c` state table (166 slots)** - adjacent strings `daBullet_c::StateID_EatIn/EatNow/SpiteMove/Reflect`; 41 untabled handlers `0x8001F900..0x800A81E0`.
- Nearby strings: `daBullet_c::StateID_EatIn`, `daBullet_c::StateID_EatNow`, `daBullet_c::StateID_SpiteMove`

### `0x80313768` - 235 words, 33 untabled entries, 7732 B

- Span: `0x80313768..0x80313b14`; 30 translated slots also live in this run
- Entries: 0x800a3080 0x800a3430 0x800a3560 0x800a3600 0x800a3710 0x800a37f0 0x800a3c80 0x800a3d00 0x800a3eb0 0x800a3f80 0x800a4040 0x800a4200 0x800a44e0 0x800a45b0 0x800a4650 0x800a4820 0x800a4930 0x800a4a90 0x800a4ce0 0x800a51c0 0x800a55c0 0x800a5620 0x800a5670 0x800a56d0 0x800a58b0 0x800a5a00 0x800a5b50 0x800a5bb0 0x800a6030 0x800a60f0 0x800a63e0 0x800a6730 0x800a6940
- Role: **`dEn_c` die-state table (235 slots)** - adjacent strings `dEn_c::StateID_DieFumi/DieFall/DieBigFall/DieSmoke`; 33 untabled handlers `0x800A3080..0x800A6940`. Second half of the enemy die/defeat dispatch block.
- Nearby strings: `dEn_c::StateID_DieFumi`, `dEn_c::StateID_DieFall`, `dEn_c::StateID_DieBigFall`

### `0x80303104` - 187 words, 31 untabled entries, 7636 B

- Span: `0x80303104..0x803033f0`; 19 translated slots also live in this run
- Entries: 0x800283d0 0x800284b0 0x80028fc0 0x80029180 0x80029340 0x80029500 0x80029670 0x800296d0 0x80029720 0x800297d0 0x80029830 0x800298a0 0x80029950 0x80029990 0x80029a20 0x80029b40 0x80029c20 0x8002a030 0x8002a180 0x8002cbd0 0x8002ce30 0x8002d050 0x8002d090 0x8002d340 0x8002d420 0x8002d5e0 0x8002d730 0x8002d840 0x8002d980 0x8002df30 0x8002e020
- Role: **`dokan`/pakkun fire state table (187 slots)** - adjacent strings `g3d/pakkun.brres`, `dokan_fire_st/ne/neck/wait`; 31 untabled handlers `0x800283D0..0x8002E020`.
- Nearby strings: `g3d/pakkun.brres`, `dokan_fire_st`, `dokan_fire_neck`

### `0x8032b524` - 29 words, 8 untabled entries, 7632 B

- Span: `0x8032b524..0x8032b598`; 4 translated slots also live in this run
- Entries: 0x80186410 0x80186510 0x801865c0 0x801866f0 0x80186830 0x80186a60 0x80187040 0x80187710
- Role: **nw4r `lyt` pane vtables (29 slots)** - adjacent strings `nw4hbm::lyt::Picture`, `TextBox`, `Window`; 8 untabled pane virtuals `0x80186410..0x80187710` (ui::lyt).
- Nearby strings: `nw4hbm::lyt::Picture`, `nw4hbm::lyt::TextBox`, `nw4hbm::lyt::Window`

### `0x80327250` - 19 words, 5 untabled entries, 6848 B

- Span: `0x80327250..0x8032729c`; 9 translated slots also live in this run
- Entries: 0x80154d00 0x80154d50 0x80154f60 0x80156e00 0x80156ea0
- Role: **`daYoshi_c` state table (19 slots)** - adjacent strings `daYoshi_c::StateID_Jump/Fall/Land/Crouch`; 5 untabled handlers `0x80154D00..0x80156EA0` (engine::display / game::player slots in the same run).
- Nearby strings: `c::StateID_Jump`, `daYoshi_c::StateID_Fall`, `daYoshi_c::StateID_Land`

### `0x8034f9fc` - 33 words, 6 untabled entries, 6368 B

- Span: `0x8034f9fc..0x8034fa80`; 5 translated slots also live in this run
- Entries: 0x802af980 0x802afad0 0x802afc60 0x802afe40 0x802b0070 0x802b0dc0
- Role: **nw4r `lyt` draw/arrange vtable block (33 slots)** - 6 untabled draw/arrange virtuals `0x802AF980..0x802B0DC0` next to render::gx_band and resource::archive translated slots.

### `0x80312750` - 265 words, 30 untabled entries, 5644 B

- Span: `0x80312750..0x80312b74`; 27 translated slots also live in this run
- Entries: 0x8009b400 0x8009b4e0 0x8009b5a0 0x8009b6f0 0x8009b780 0x8009b7f0 0x8009b850 0x8009b8e0 0x8009b9e0 0x8009be40 0x8009bff0 0x8009c060 0x8009c170 0x8009c270 0x8009c450 0x8009cac0 0x8009cb40 0x8009cc20 0x8009ced0 0x8009d0a0 0x8009d190 0x8009d230 0x8009d2b0 0x8009d3c0 0x8009d4e0 0x8009d600 0x8009d780 0x8009d810 0x8009d880 0x8009d980
- Role: **`dEnBoss_c` state table (265 slots)** - adjacent strings `dEnBoss_c::StateID_DieFire/DieSlide/DieShell/DieStar`; 30 untabled handlers `0x8009B400..0x8009D980`.
- Nearby strings: `oss_c::StateID_DieFire`, `dEnBoss_c::StateID_DieSlide`, `dEnBoss_c::StateID_DieShell`

## Inline pointer constants inside .text

Six entries are owned by a lone pointer word inside `.text` (an inline literal
pool, not a table). They are listed here so the per-entry accounting is complete.

| table/word | entry | bytes |
|---|---|---:|
| `0x8006fa40` | `0x80030010` | 304 |
| `0x80196580` | `0x80030950` | 88 |
| `0x801e400c` | `0x80010290` | 792 |
| `0x801808cc` | `0x80100030` | 452 |
| `0x80071f48` | `0x80170000` | 272 |
| `0x801eeb48` | `0x800db4a0` | 136 |

## Call-site-only groups

Entries with no pointer word anywhere: they are branch targets inside untabled
code (the caller is itself in the same tsv gap, so the group is a self-contained
untabled cluster). Grouped by the containing gap; `bytes` is the summed body
size of the cluster's untabled entry points.

| # | gap | entries | bytes | addresses |
|---:|---|---:|---:|---|
| 1 | `gap:0x8029d2fc` | 3 | 6684 | 0x8029d340 0x8029dd40 0x8029e5b0 |
| 2 | `gap:0x8000b52c` | 3 | 4224 | 0x8000b530 0x8000ba70 0x8000c020 |
| 3 | `gap:0x80278ab0` | 1 | 2616 | 0x80278ad0 |
| 4 | `gap:0x800c316c` | 1 | 2196 | 0x800c3170 |
| 5 | `gap:0x800e953c` | 3 | 1940 | 0x800e9540 0x800e98f0 0x800e9a80 |
| 6 | `gap:0x8004ea1c` | 4 | 1728 | 0x8004ea20 0x8004ea80 0x8004ede0 0x8004efd0 |
| 7 | `gap:0x8026e000` | 1 | 1676 | 0x8026e2b0 |
| 8 | `gap:0x8004f858` | 3 | 1524 | 0x8004f860 0x8004fa70 0x8004fbb0 |
| 9 | `gap:0x802cd4a4` | 1 | 1216 | 0x802cd4b0 |
| 10 | `gap:0x801d0ce4` | 2 | 872 | 0x801d0cf0 0x801d0f20 |
| 11 | `gap:0x801ad898` | 1 | 764 | 0x801ad8a0 |
| 12 | `gap:0x80284b18` | 1 | 684 | 0x80284c80 |
| 13 | `gap:0x800c52dc` | 2 | 604 | 0x800c52e0 0x800c5410 |
| 14 | `gap:0x800d0004` | 2 | 572 | 0x800d0030 0x800d0100 |
| 15 | `gap:0x801ab758` | 1 | 556 | 0x801ab760 |
| 16 | `gap:0x8027e3a4` | 1 | 468 | 0x8027e3b0 |
| 17 | `gap:0x80100970` | 1 | 460 | 0x80100da0 |
| 18 | `gap:0x8027debc` | 1 | 440 | 0x8027dec0 |
| 19 | `gap:0x801632f8` | 3 | 428 | 0x80163300 0x80163390 0x80163420 |
| 20 | `gap:0x800b9e38` | 1 | 384 | 0x800b9e40 |
| 21 | `gap:0x8015962c` | 1 | 364 | 0x80159630 |
| 22 | `fn:0x80061540` | 4 | 360 | 0x800613c0 0x80061420 0x80061480 0x800614e0 |
| 23 | `gap:0x800cae6c` | 1 | 344 | 0x800cae70 |
| 24 | `gap:0x800f65b8` | 1 | 328 | 0x800f65c0 |
| 25 | `gap:0x8017e0bc` | 1 | 328 | 0x8017e0c0 |
| 26 | `gap:0x802b64dc` | 1 | 320 | 0x802b64e0 |
| 27 | `gap:0x800c507c` | 1 | 288 | 0x800c5080 |
| 28 | `gap:0x801f3978` | 1 | 288 | 0x801f3980 |
| 29 | `gap:0x8023f244` | 1 | 288 | 0x8023f250 |
| 30 | `gap:0x80271d1c` | 1 | 268 | 0x80271d20 |
| 31 | `gap:0x80285a14` | 1 | 260 | 0x80285a40 |
| 32 | `gap:0x8019cc6c` | 2 | 244 | 0x8019cc70 0x8019cd30 |
| 33 | `gap:0x80020ff0` | 1 | 216 | 0x80021010 |
| 34 | `gap:0x801d50a4` | 1 | 208 | 0x801d50b0 |
| 35 | `gap:0x80021138` | 1 | 200 | 0x800211b0 |
| 36 | `gap:0x800f69e4` | 1 | 188 | 0x800f69f0 |
| 37 | `gap:0x8016ddfc` | 1 | 184 | 0x8016de00 |
| 38 | `gap:0x800decd8` | 1 | 180 | 0x800dece0 |
| 39 | `gap:0x80287254` | 1 | 168 | 0x80287360 |
| 40 | `gap:0x8019c918` | 1 | 148 | 0x8019c920 |
| 41 | `gap:0x8005134c` | 1 | 144 | 0x80051370 |
| 42 | `gap:0x800cb7d8` | 1 | 144 | 0x800cb810 |
| 43 | `gap:0x8011ed08` | 1 | 144 | 0x8011ed10 |
| 44 | `gap:0x80292868` | 1 | 140 | 0x80292890 |
| 45 | `gap:0x802dc3e8` | 1 | 132 | 0x802dc3f0 |
| 46 | `gap:0x801d47d4` | 1 | 124 | 0x801d47e0 |
| 47 | `gap:0x80259354` | 1 | 116 | 0x80259360 |
| 48 | `gap:0x80094414` | 1 | 112 | 0x800944f0 |
| 49 | `gap:0x8012e998` | 1 | 112 | 0x8012e9a0 |
| 50 | `gap:0x801d53a4` | 1 | 112 | 0x801d53b0 |
| 51 | `gap:0x802dc1c8` | 1 | 112 | 0x802dc1d0 |
| 52 | `gap:0x8003ddf4` | 1 | 108 | 0x8003df50 |
| 53 | `gap:0x80092d94` | 1 | 108 | 0x80092dc0 |
| 54 | `gap:0x800cef68` | 1 | 108 | 0x800cef70 |
| 55 | `gap:0x800cf518` | 1 | 108 | 0x800cf520 |
| 56 | `gap:0x800cf8ec` | 1 | 108 | 0x800cf8f0 |
| 57 | `gap:0x80265268` | 1 | 104 | 0x80265270 |
| 58 | `gap:0x8005c12c` | 1 | 100 | 0x8005c130 |
| 59 | `gap:0x80286798` | 1 | 100 | 0x802867a0 |
| 60 | `gap:0x80194aa8` | 1 | 88 | 0x80194d40 |
| 61 | `gap:0x8026da30` | 1 | 84 | 0x8026da40 |
| 62 | `gap:0x802dc308` | 1 | 84 | 0x802dc310 |
| 63 | `gap:0x802dc378` | 1 | 84 | 0x802dc380 |
| 64 | `gap:0x802dc488` | 1 | 84 | 0x802dc490 |
| 65 | `gap:0x8009c7b4` | 1 | 80 | 0x8009c7c0 |
| 66 | `gap:0x802dc248` | 1 | 76 | 0x802dc250 |
| 67 | `gap:0x802dc2a8` | 1 | 76 | 0x802dc2b0 |
| 68 | `gap:0x801bb958` | 1 | 72 | 0x801bb960 |
| 69 | `gap:0x8012e934` | 1 | 68 | 0x8012e940 |
| 70 | `gap:0x802d6c78` | 1 | 48 | 0x802d6c80 |
| 71 | `gap:0x8019b9bc` | 1 | 44 | 0x8019b9c0 |
| 72 | `gap:0x8016b188` | 1 | 36 | 0x8016b190 |

## Unreferenced entries

415 prologue-verified entries have neither a pointer word nor a
branch/call target in `main.dol` text. They are listed by the tsv gap they sit
in; almost all are secondary entry points of a larger untabled body (cold
blocks, error paths, per-class `execute`/`draw` halves reached through a vtable
slot that is built at run time).

| # | gap start | entries | bytes | addresses |
|---:|---|---:|---:|---|
| 1 | `0x80101bd8` | 6 | 600 | 0x80101c60 0x80101ce0 0x80101d40 0x80101dc0 0x80101e20 0x80101ea0 |
| 2 | `0x800086b8` | 5 | 4856 | 0x800086c0 0x80008a70 0x80008c50 0x80009220 0x80009400 |
| 3 | `0x800272f0` | 5 | 660 | 0x80027310 0x80027380 0x80027420 0x80027470 0x80027510 |
| 4 | `0x800fd7d4` | 5 | 780 | 0x800fda50 0x800fdc80 0x800fdcf0 0x800fdd60 0x800fdf40 |
| 5 | `0x80092d94` | 4 | 520 | 0x80092fd0 0x800930a0 0x80093170 0x800931b0 |
| 6 | `0x800e812c` | 4 | 1900 | 0x800e8160 0x800e8370 0x800e85a0 0x800e8770 |
| 7 | `0x800ea9f4` | 4 | 880 | 0x800eaa00 0x800eaa70 0x800eaac0 0x800eab20 |
| 8 | `0x800f01a4` | 4 | 836 | 0x800f01b0 0x800f02c0 0x800f0380 0x800f0440 |
| 9 | `0x8018cea8` | 4 | 1520 | 0x8018ceb0 0x8018d080 0x8018d1e0 0x8018d3d0 |
| 10 | `0x80231214` | 4 | 1584 | 0x80231220 0x80231370 0x80231490 0x802316b0 |
| 11 | `0x80236b24` | 4 | 268 | 0x80236b30 0x80236ba0 0x80236be0 0x80236c10 |
| 12 | `0x802378b4` | 4 | 268 | 0x802378c0 0x80237930 0x80237970 0x802379a0 |
| 13 | `0x8029ee40` | 4 | 1316 | 0x8029eec0 0x8029efa0 0x8029f080 0x8029f1a0 |
| 14 | `0x802a5abc` | 4 | 2180 | 0x802a5b90 0x802a5cb0 0x802a5f30 0x802a6200 |
| 15 | `0x802aaadc` | 4 | 2180 | 0x802aabb0 0x802aacd0 0x802aaf50 0x802ab220 |
| 16 | `0x8000779c` | 3 | 1592 | 0x800077a0 0x80007860 0x80007bc0 |
| 17 | `0x80009bc8` | 3 | 3724 | 0x80009bd0 0x8000a0c0 0x8000a3d0 |
| 18 | `0x8001cbec` | 3 | 336 | 0x8001cd20 0x8001cdb0 0x8001ce60 |
| 19 | `0x8002766c` | 3 | 776 | 0x800276b0 0x800277a0 0x800279f0 |
| 20 | `0x800bb2dc` | 3 | 216 | 0x800bb2e0 0x800bb330 0x800bb380 |
| 21 | `0x800bbf04` | 3 | 448 | 0x800bbf10 0x800bbf70 0x800bc070 |
| 22 | `0x800bff88` | 3 | 576 | 0x800c0010 0x800c01a0 0x800c0210 |
| 23 | `0x800ecadc` | 3 | 5160 | 0x800ecae0 0x800ede10 0x800edeb0 |
| 24 | `0x80100384` | 3 | 388 | 0x80100390 0x80100430 0x801004b0 |
| 25 | `0x80100970` | 3 | 532 | 0x80100aa0 0x80100b40 0x80100ca0 |
| 26 | `0x801709f8` | 3 | 472 | 0x80170a00 0x80170a90 0x80170b40 |
| 27 | `0x8018bb38` | 3 | 340 | 0x8018bb40 0x8018bba0 0x8018bc00 |
| 28 | `0x8018bd88` | 3 | 1472 | 0x8018bde0 0x8018bfe0 0x8018c1e0 |
| 29 | `0x8018e798` | 3 | 1472 | 0x8018e7a0 0x8018e9a0 0x8018eba0 |
| 30 | `0x8023094c` | 3 | 1512 | 0x80230950 0x80230b60 0x80230d70 |
| 31 | `0x800149b4` | 2 | 220 | 0x80014a30 0x80014b30 |
| 32 | `0x80022074` | 2 | 224 | 0x80022080 0x80022100 |
| 33 | `0x800225f4` | 2 | 344 | 0x80022600 0x800226d0 |
| 34 | `0x80043818` | 2 | 512 | 0x80043820 0x800439b0 |
| 35 | `0x80060968` | 2 | 304 | 0x80060970 0x80060a10 |
| 36 | `0x80064b04` | 2 | 168 | 0x80064b10 0x80064b50 |
| 37 | `0x800a2bec` | 2 | 648 | 0x800a2bf0 0x800a2d20 |
| 38 | `0x800a76dc` | 2 | 168 | 0x800a76e0 0x800a7740 |
| 39 | `0x800b6514` | 2 | 296 | 0x800b66f0 0x800b6790 |
| 40 | `0x800be544` | 2 | 296 | 0x800be550 0x800be5f0 |
| 41 | `0x800c06b8` | 2 | 236 | 0x800c06c0 0x800c0750 |
| 42 | `0x800d895c` | 2 | 396 | 0x800d8960 0x800d8a80 |
| 43 | `0x800e400c` | 2 | 1104 | 0x800e4010 0x800e41c0 |
| 44 | `0x800e74c0` | 2 | 184 | 0x800e74d0 0x800e7530 |
| 45 | `0x800e78d4` | 2 | 264 | 0x800e78e0 0x800e7970 |
| 46 | `0x800e8bfc` | 2 | 168 | 0x800e8c00 0x800e8c50 |
| 47 | `0x800ebaa4` | 2 | 2112 | 0x800ebab0 0x800ebbe0 |
| 48 | `0x800ef364` | 2 | 500 | 0x800ef370 0x800ef430 |
| 49 | `0x800f22fc` | 2 | 268 | 0x800f2390 0x800f2470 |
| 50 | `0x800f31d8` | 2 | 260 | 0x800f31e0 0x800f3270 |
| 51 | `0x800f5f3c` | 2 | 212 | 0x800f5fd0 0x800f6020 |
| 52 | `0x800f9bdc` | 2 | 224 | 0x800f9be0 0x800f9c50 |
| 53 | `0x800f9fb4` | 2 | 276 | 0x800f9fc0 0x800fa070 |
| 54 | `0x800fb98c` | 2 | 956 | 0x800fb9e0 0x800fbd30 |
| 55 | `0x800fc558` | 2 | 640 | 0x800fc560 0x800fc5c0 |
| 56 | `0x800fcb48` | 2 | 328 | 0x800fcb50 0x800fcbb0 |
| 57 | `0x800fe058` | 2 | 336 | 0x800fe060 0x800fe110 |
| 58 | `0x800ffe20` | 2 | 176 | 0x800fffe0 0x80100200 |
| 59 | `0x801038ec` | 2 | 336 | 0x801038f0 0x80103ac0 |
| 60 | `0x801061f8` | 2 | 1008 | 0x80106200 0x801062b0 |
| 61 | `0x80160a08` | 2 | 136 | 0x80160a20 0x80160a70 |
| 62 | `0x8016efc8` | 2 | 340 | 0x8016efd0 0x8016f050 |
| 63 | `0x8016f720` | 2 | 136 | 0x8016f740 0x8016f790 |
| 64 | `0x8016f838` | 2 | 408 | 0x8016f870 0x8016f980 |
| 65 | `0x8016fecc` | 2 | 136 | 0x8016fed0 0x8016ff20 |
| 66 | `0x802294dc` | 2 | 296 | 0x802296d0 0x80229740 |
| 67 | `0x80229b74` | 2 | 296 | 0x80229b80 0x80229bf0 |
| 68 | `0x8022e874` | 2 | 604 | 0x8022e880 0x8022e9d0 |
| 69 | `0x8000ab4c` | 1 | 760 | 0x8000ab50 |
| 70 | `0x8000af9c` | 1 | 884 | 0x8000afa0 |
| 71 | `0x8000d4fc` | 1 | 172 | 0x8000d500 |
| 72 | `0x8000d928` | 1 | 1332 | 0x8000d930 |
| 73 | `0x8000f024` | 1 | 188 | 0x8000f030 |
| 74 | `0x8000f1a4` | 1 | 180 | 0x8000f1b0 |
| 75 | `0x80012558` | 1 | 216 | 0x800125c0 |
| 76 | `0x8001db90` | 1 | 420 | 0x8001ddb0 |
| 77 | `0x80020d5c` | 1 | 268 | 0x80020d60 |
| 78 | `0x80021904` | 1 | 268 | 0x80021910 |
| 79 | `0x80027af4` | 1 | 512 | 0x80027b00 |
| 80 | `0x80027d6c` | 1 | 616 | 0x80027e00 |
| 81 | `0x8002b25c` | 1 | 248 | 0x8002b260 |
| 82 | `0x80031eb4` | 1 | 224 | 0x80031fa0 |
| 83 | `0x80035ba8` | 1 | 180 | 0x80035bb0 |
| 84 | `0x80037eec` | 1 | 324 | 0x80037ef0 |
| 85 | `0x80043f00` | 1 | 424 | 0x80043f10 |
| 86 | `0x800455fc` | 1 | 72 | 0x80045600 |
| 87 | `0x80045f18` | 1 | 180 | 0x80045f40 |
| 88 | `0x80049a10` | 1 | 112 | 0x80049a40 |
| 89 | `0x80049e04` | 1 | 152 | 0x8004a210 |
| 90 | `0x8004a348` | 1 | 144 | 0x8004a350 |
| 91 | `0x8004a50c` | 1 | 232 | 0x8004a6a0 |
| 92 | `0x8004a79c` | 1 | 116 | 0x8004a7b0 |
| 93 | `0x80052118` | 1 | 80 | 0x80052120 |
| 94 | `0x80052218` | 1 | 112 | 0x80052220 |
| 95 | `0x80052eec` | 1 | 236 | 0x80052ef0 |
| 96 | `0x80057b24` | 1 | 60 | 0x80057c40 |
| 97 | `0x80057d28` | 1 | 64 | 0x80057d30 |
| 98 | `0x8005d6a8` | 1 | 148 | 0x8005d6b0 |
| 99 | `0x8005eb04` | 1 | 384 | 0x8005eb10 |
| 100 | `0x8005ec98` | 1 | 48 | 0x8005ecb0 |
| 101 | `0x80060084` | 1 | 116 | 0x80060090 |
| 102 | `0x800618ec` | 1 | 84 | 0x800618f0 |
| 103 | `0x80065614` | 1 | 340 | 0x800656c0 |
| 104 | `0x8006607c` | 1 | 96 | 0x800660c0 |
| 105 | `0x80066214` | 1 | 80 | 0x80066380 |
| 106 | `0x8006659c` | 1 | 48 | 0x800665a0 |
| 107 | `0x80069d6c` | 1 | 200 | 0x80069e50 |
| 108 | `0x80069f88` | 1 | 124 | 0x8006a010 |
| 109 | `0x8006a564` | 1 | 104 | 0x8006a570 |
| 110 | `0x8006c1b4` | 1 | 572 | 0x8006c1c0 |
| 111 | `0x8006cb38` | 1 | 88 | 0x8006cb40 |
| 112 | `0x80072cfc` | 1 | 164 | 0x80072d00 |
| 113 | `0x8007d92c` | 1 | 1836 | 0x8007d930 |
| 114 | `0x8007fb74` | 1 | 108 | 0x8007fb80 |
| 115 | `0x80080628` | 1 | 64 | 0x80080630 |
| 116 | `0x800882d8` | 1 | 248 | 0x800882e0 |
| 117 | `0x80088dc8` | 1 | 172 | 0x80088dd0 |
| 118 | `0x8008c584` | 1 | 328 | 0x8008c590 |
| 119 | `0x8008c9d4` | 1 | 100 | 0x8008c9e0 |
| 120 | `0x8008dc40` | 1 | 92 | 0x8008dc90 |
| 121 | `0x8008e6d8` | 1 | 40 | 0x8008e6e0 |
| 122 | `0x8008e888` | 1 | 180 | 0x8008e890 |
| 123 | `0x8008eb14` | 1 | 112 | 0x8008eb20 |
| 124 | `0x8008ec3c` | 1 | 80 | 0x8008ec40 |
| 125 | `0x8008ef2c` | 1 | 204 | 0x8008ef30 |
| 126 | `0x8008f068` | 1 | 84 | 0x8008f070 |
| 127 | `0x800923c4` | 1 | 176 | 0x800923d0 |
| 128 | `0x80093434` | 1 | 144 | 0x80093440 |
| 129 | `0x80093e4c` | 1 | 192 | 0x80093e50 |
| 130 | `0x80094414` | 1 | 112 | 0x80094420 |
| 131 | `0x8009629c` | 1 | 92 | 0x800962a0 |
| 132 | `0x800973e4` | 1 | 192 | 0x800975f0 |
| 133 | `0x80097768` | 1 | 192 | 0x80097800 |
| 134 | `0x80097b28` | 1 | 172 | 0x80097b90 |
| 135 | `0x8009ad2c` | 1 | 400 | 0x8009ad30 |
| 136 | `0x8009c8c4` | 1 | 340 | 0x8009c8d0 |
| 137 | `0x8009eb0c` | 1 | 60 | 0x8009eb10 |
| 138 | `0x800a0b2c` | 1 | 320 | 0x800a0b30 |
| 139 | `0x800a4cd8` | 1 | 116 | 0x800a4d70 |
| 140 | `0x800a4e60` | 1 | 812 | 0x800a4e90 |
| 141 | `0x800a5454` | 1 | 328 | 0x800a5460 |
| 142 | `0x800a763c` | 1 | 80 | 0x800a7640 |
| 143 | `0x800a7994` | 1 | 580 | 0x800a79a0 |
| 144 | `0x800a7d24` | 1 | 68 | 0x800a7d30 |
| 145 | `0x800a889c` | 1 | 516 | 0x800a88a0 |
| 146 | `0x800aae74` | 1 | 316 | 0x800aaf80 |
| 147 | `0x800b10fc` | 1 | 196 | 0x800b1100 |
| 148 | `0x800b2c88` | 1 | 332 | 0x800b2d20 |
| 149 | `0x800b2f4c` | 1 | 68 | 0x800b2f60 |
| 150 | `0x800b32d4` | 1 | 228 | 0x800b32e0 |
| 151 | `0x800b3974` | 1 | 60 | 0x800b3980 |
| 152 | `0x800b3bdc` | 1 | 100 | 0x800b3be0 |
| 153 | `0x800b43cc` | 1 | 252 | 0x800b43d0 |
| 154 | `0x800b48e4` | 1 | 200 | 0x800b4ca0 |
| 155 | `0x800b59ac` | 1 | 76 | 0x800b5a60 |
| 156 | `0x800b69e8` | 1 | 120 | 0x800b69f0 |
| 157 | `0x800b8f98` | 1 | 108 | 0x800b9050 |
| 158 | `0x800b911c` | 1 | 68 | 0x800b9120 |
| 159 | `0x800bb87c` | 1 | 112 | 0x800bb880 |
| 160 | `0x800bc948` | 1 | 232 | 0x800bc950 |
| 161 | `0x800bca58` | 1 | 272 | 0x800bca60 |
| 162 | `0x800bfc58` | 1 | 288 | 0x800bfc60 |
| 163 | `0x800c12c4` | 1 | 512 | 0x800c12f0 |
| 164 | `0x800c1528` | 1 | 376 | 0x800c1580 |
| 165 | `0x800c8cbc` | 1 | 160 | 0x800c8cc0 |
| 166 | `0x800cd150` | 1 | 124 | 0x800cd1a0 |
| 167 | `0x800cd6b8` | 1 | 96 | 0x800cd710 |
| 168 | `0x800d0004` | 1 | 92 | 0x800d02e0 |
| 169 | `0x800d0980` | 1 | 164 | 0x800d0a70 |
| 170 | `0x800d1254` | 1 | 592 | 0x800d1260 |
| 171 | `0x800d260c` | 1 | 128 | 0x800d2610 |
| 172 | `0x800d77c4` | 1 | 80 | 0x800d77d0 |
| 173 | `0x800d787c` | 1 | 248 | 0x800d7880 |
| 174 | `0x800d8d2c` | 1 | 260 | 0x800d8d30 |
| 175 | `0x800d9124` | 1 | 352 | 0x800d9130 |
| 176 | `0x800daf20` | 1 | 68 | 0x800db130 |
| 177 | `0x800db494` | 1 | 96 | 0x800db530 |
| 178 | `0x800db6ec` | 1 | 68 | 0x800db6f0 |
| 179 | `0x800dde64` | 1 | 120 | 0x800ddf00 |
| 180 | `0x800df128` | 1 | 76 | 0x800df130 |
| 181 | `0x800dfaf4` | 1 | 108 | 0x800dfb00 |
| 182 | `0x800e0c1c` | 1 | 432 | 0x800e1690 |
| 183 | `0x800e2404` | 1 | 148 | 0x800e2410 |
| 184 | `0x800e3274` | 1 | 112 | 0x800e3280 |
| 185 | `0x800e370c` | 1 | 176 | 0x800e3710 |
| 186 | `0x800e3868` | 1 | 100 | 0x800e3870 |
| 187 | `0x800e392c` | 1 | 300 | 0x800e3930 |
| 188 | `0x800e7794` | 1 | 80 | 0x800e77c0 |
| 189 | `0x800e7e20` | 1 | 116 | 0x800e7e50 |
| 190 | `0x800e7f1c` | 1 | 96 | 0x800e7f20 |
| 191 | `0x800e7f8c` | 1 | 116 | 0x800e7f90 |
| 192 | `0x800e802c` | 1 | 116 | 0x800e8030 |
| 193 | `0x800e8b2c` | 1 | 108 | 0x800e8b30 |
| 194 | `0x800e8d24` | 1 | 132 | 0x800e8d30 |
| 195 | `0x800ee470` | 1 | 320 | 0x800ee4d0 |
| 196 | `0x800ef6fc` | 1 | 104 | 0x800ef700 |
| 197 | `0x800efc6c` | 1 | 408 | 0x800efc70 |
| 198 | `0x800f07f8` | 1 | 96 | 0x800f0800 |
| 199 | `0x800f103c` | 1 | 104 | 0x800f1040 |
| 200 | `0x800f10c4` | 1 | 116 | 0x800f10d0 |
| 201 | `0x800f1158` | 1 | 92 | 0x800f1160 |
| 202 | `0x800f1704` | 1 | 160 | 0x800f1710 |
| 203 | `0x800f2d38` | 1 | 164 | 0x800f2d40 |
| 204 | `0x800f34f4` | 1 | 60 | 0x800f35a0 |
| 205 | `0x800f54cc` | 1 | 468 | 0x800f54d0 |
| 206 | `0x800f5844` | 1 | 220 | 0x800f5850 |
| 207 | `0x800f640c` | 1 | 168 | 0x800f6410 |
| 208 | `0x800f65b8` | 1 | 252 | 0x800f6710 |
| 209 | `0x800f7494` | 1 | 512 | 0x800f7560 |
| 210 | `0x800f7b2c` | 1 | 256 | 0x800f7ca0 |
| 211 | `0x800f8124` | 1 | 188 | 0x800f8130 |
| 212 | `0x800f85e0` | 1 | 168 | 0x800f8680 |
| 213 | `0x800f881c` | 1 | 340 | 0x800f8820 |
| 214 | `0x800f8ac4` | 1 | 68 | 0x800f8ad0 |
| 215 | `0x800f9078` | 1 | 184 | 0x800f9080 |
| 216 | `0x800f9924` | 1 | 60 | 0x800f9930 |
| 217 | `0x800f9d24` | 1 | 116 | 0x800f9d30 |
| 218 | `0x800fa1c4` | 1 | 116 | 0x800fa1d0 |
| 219 | `0x800fb2cc` | 1 | 80 | 0x800fb2d0 |
| 220 | `0x800fbddc` | 1 | 124 | 0x800fbde0 |
| 221 | `0x800fc038` | 1 | 92 | 0x800fc040 |
| 222 | `0x800fc844` | 1 | 228 | 0x800fc850 |
| 223 | `0x800fd024` | 1 | 100 | 0x800fd030 |
| 224 | `0x800fd0ac` | 1 | 108 | 0x800fd0b0 |
| 225 | `0x800fd2d4` | 1 | 48 | 0x800fd2e0 |
| 226 | `0x801010b0` | 1 | 84 | 0x80101110 |
| 227 | `0x801012e4` | 1 | 312 | 0x801012f0 |
| 228 | `0x8010162c` | 1 | 52 | 0x80101630 |
| 229 | `0x80101988` | 1 | 260 | 0x80101990 |
| 230 | `0x80101fe4` | 1 | 112 | 0x80101ff0 |
| 231 | `0x80102e74` | 1 | 116 | 0x80102f10 |
| 232 | `0x80103088` | 1 | 72 | 0x80103090 |
| 233 | `0x801031a4` | 1 | 104 | 0x801031b0 |
| 234 | `0x80103728` | 1 | 128 | 0x801037c0 |
| 235 | `0x80103c80` | 1 | 132 | 0x80103ca0 |
| 236 | `0x80103dac` | 1 | 112 | 0x80103db0 |
| 237 | `0x80103e38` | 1 | 116 | 0x80103e40 |
| 238 | `0x80104594` | 1 | 1820 | 0x801045a0 |
| 239 | `0x80105364` | 1 | 160 | 0x80105370 |
| 240 | `0x80105f24` | 1 | 444 | 0x80105fd0 |
| 241 | `0x8010e5f8` | 1 | 112 | 0x8010e600 |
| 242 | `0x80128e54` | 1 | 64 | 0x80128e60 |
| 243 | `0x8012f808` | 1 | 208 | 0x8012f810 |
| 244 | `0x80133854` | 1 | 228 | 0x80133860 |
| 245 | `0x801339d8` | 1 | 128 | 0x801339e0 |
| 246 | `0x8013750c` | 1 | 60 | 0x80137510 |
| 247 | `0x80137644` | 1 | 124 | 0x80137650 |
| 248 | `0x8013e628` | 1 | 144 | 0x8013e630 |
| 249 | `0x8013e744` | 1 | 112 | 0x8013e750 |
| 250 | `0x80160ad4` | 1 | 560 | 0x80160ae0 |
| 251 | `0x80161114` | 1 | 92 | 0x80161120 |
| 252 | `0x801648a4` | 1 | 224 | 0x801648b0 |
| 253 | `0x80164d58` | 1 | 196 | 0x80164d60 |
| 254 | `0x80165774` | 1 | 160 | 0x80165780 |
| 255 | `0x80169218` | 1 | 112 | 0x80169220 |
| 256 | `0x8016a218` | 1 | 120 | 0x8016a220 |
| 257 | `0x8016aa14` | 1 | 276 | 0x8016aa70 |
| 258 | `0x8016b408` | 1 | 48 | 0x8016b410 |
| 259 | `0x8016fc6c` | 1 | 192 | 0x8016fc70 |
| 260 | `0x80170404` | 1 | 192 | 0x80170410 |
| 261 | `0x8018b22c` | 1 | 112 | 0x8018b470 |
| 262 | `0x8018b78c` | 1 | 112 | 0x8018b9d0 |
| 263 | `0x8018e5b8` | 1 | 160 | 0x8018e5c0 |
| 264 | `0x8018f868` | 1 | 452 | 0x8018f870 |
| 265 | `0x8018fd10` | 1 | 360 | 0x8018fd90 |
| 266 | `0x80196e84` | 1 | 128 | 0x80196eb0 |
| 267 | `0x80199e88` | 1 | 284 | 0x80199e90 |
| 268 | `0x8019b9f8` | 1 | 204 | 0x8019ba20 |
| 269 | `0x8019c244` | 1 | 220 | 0x8019c250 |
| 270 | `0x8019c500` | 1 | 352 | 0x8019c510 |
| 271 | `0x801bded8` | 1 | 156 | 0x801bdee0 |
| 272 | `0x801be344` | 1 | 248 | 0x801be350 |
| 273 | `0x801ec76c` | 1 | 296 | 0x801ec770 |
| 274 | `0x8022de98` | 1 | 144 | 0x8022dea0 |
| 275 | `0x8022e3c4` | 1 | 480 | 0x8022e3d0 |
| 276 | `0x8022ed0c` | 1 | 440 | 0x8022ed10 |
| 277 | `0x8022f1cc` | 1 | 340 | 0x8022f1f0 |
| 278 | `0x80230838` | 1 | 144 | 0x80230840 |
| 279 | `0x80231b78` | 1 | 340 | 0x80231b90 |
| 280 | `0x80235f34` | 1 | 144 | 0x80235f50 |
| 281 | `0x80236078` | 1 | 272 | 0x80236120 |
| 282 | `0x802364cc` | 1 | 272 | 0x802364d0 |
| 283 | `0x802367c8` | 1 | 152 | 0x802367e0 |
| 284 | `0x80236a0c` | 1 | 192 | 0x80236a10 |
| 285 | `0x80236cb4` | 1 | 144 | 0x80236ce0 |
| 286 | `0x80236e08` | 1 | 272 | 0x80236eb0 |
| 287 | `0x8023725c` | 1 | 272 | 0x80237260 |
| 288 | `0x80237558` | 1 | 152 | 0x80237570 |
| 289 | `0x8023779c` | 1 | 192 | 0x802377a0 |
| 290 | `0x80238534` | 1 | 140 | 0x80238540 |
| 291 | `0x8023a2dc` | 1 | 108 | 0x8023a2e0 |
| 292 | `0x0` | 1 | 84 | 0x802419a0 |
| 293 | `0x802426cc` | 1 | 364 | 0x802426d0 |
| 294 | `0x802a178c` | 1 | 48 | 0x802a1830 |
| 295 | `0x802ae18c` | 1 | 88 | 0x802ae3d0 |
| 296 | `0x802c0144` | 1 | 236 | 0x802c0150 |
| 297 | `0x802c6c64` | 1 | 104 | 0x802c6c70 |

## Complete run inventory

Every run with at least two owned entries, or one entry and at least 256 B,
sorted by tier then bytes. Single-slot runs below 256 B are omitted here but
remain in `laneD_groups.json`. `<0xFFFFFFFF, handler, 0>` descriptor arrays are
not pointer runs and are documented separately: `0x80326D0C` (12 records) in the
wave-249 correction above.

The WorldMap run `0x80325428` (276 words) and `WM_PLAYER_VTABLE` `0x8032679c`
share function pointers in slots 91-114, so the inventory below carries them as
one merged entry: 527 unique words, 147 owned entries, 32,684 B. Those are the
wave-231 **maximal-run** figures. The `0x8032679c` slot array is really 311
slots (`0x8032679c..0x80326c78`, wave-249 correction), so the two vtable objects
together span 563 unique words (276 + 311, less the 24 aliased slots); the extra
`0x80326bf0` run and the 37 null words are not part of the wave-226 entry census
(they own two corpus entries, both translated). The tables keep separate headers
and are still listed separately in the wave-231 tier tables above, where each
row notes the aliased slots.

| tier | table | words | entries | bytes | first entry |
|---:|---|---:|---:|---:|---|
| 1 | `0x802ed9e0` | 181 | 76 | 58612 | `0x8001d2c0` |
| 1 | `0x802f8ef0` | 42 | 20 | 5272 | `0x80254c20` |
| 1 | `0x80343e90` | 32 | 9 | 4840 | `0x801e6d10` |
| 1 | `0x80314534` | 68 | 22 | 3584 | `0x800a8fe0` |
| 1 | `0x80310db0` | 16 | 3 | 1744 | `0x8008cba0` |
| 1 | `0x803280e0` | 32 | 15 | 1112 | `0x800e4fb0` |
| 1 | `0x802f8510` | 8 | 6 | 804 | `0x802412a0` |
| 1 | `0x802ee830` | 1 | 1 | 696 | `0x80095950` |
| 1 | `0x802ef328` | 6 | 1 | 400 | `0x80064880` |
| 1 | `0x802f6d30` | 5 | 3 | 204 | `0x801f8da0` |
| 1 | `0x802ef8b0` | 4 | 2 | 200 | `0x8007c950` |
| 2 | `0x802f8ed4` | 6 | 5 | 3996 | `0x8024ebd0` |
| 2 | `0x80323b48` | 51 | 4 | 780 | `0x80115dc0` |
| 2 | `0x80310ea0` | 10 | 4 | 744 | `0x80160300` |
| 2 | `0x80310cfc` | 7 | 2 | 612 | `0x8008b2f0` |
| 2 | `0x80328188` | 18 | 3 | 376 | `0x8015c370` |
| 2 | `0x802f00e8` | 3 | 1 | 328 | `0x8008cc50` |
| 2 | `0x80323a80` | 11 | 2 | 184 | `0x80115970` |
| 2 | `0x8031b080` | 3 | 3 | 116 | `0x800e4db0` |
| 3 | `0x80325428` / `0x8032679c` | 527 | 147 | 32684 | `0x800e2550` |
| 3 | `0x80314068` | 306 | 59 | 12788 | `0x800a8b10` |
| 3 | `0x803084d0` | 275 | 44 | 11352 | `0x80046bb0` |
| 3 | `0x80305958` | 190 | 22 | 8348 | `0x80038670` |
| 3 | `0x80301c00` | 166 | 41 | 8324 | `0x8001f900` |
| 3 | `0x80313768` | 235 | 33 | 7732 | `0x800a3080` |
| 3 | `0x80303104` | 187 | 31 | 7636 | `0x800283d0` |
| 3 | `0x8032b524` | 29 | 8 | 7632 | `0x80186410` |
| 3 | `0x80327250` | 19 | 5 | 6848 | `0x80154d00` |
| 3 | `0x8034f9fc` | 33 | 6 | 6368 | `0x802af980` |
| 3 | `0x80312750` | 265 | 30 | 5644 | `0x8009b400` |
| 3 | `0x80315ef0` | 46 | 19 | 5164 | `0x800bcc40` |
| 3 | `0x8030241c` | 199 | 25 | 5040 | `0x80023cc0` |
| 3 | `0x8032b2d0` | 8 | 5 | 4112 | `0x80181910` |
| 3 | `0x8034fb40` | 10 | 4 | 3864 | `0x802b3b00` |
| 3 | `0x80304940` | 186 | 22 | 3808 | `0x80031210` |
| 3 | `0x8034e5e8` | 20 | 11 | 3560 | `0x80247840` |
| 3 | `0x80311f90` | 226 | 11 | 3440 | `0x80098660` |
| 3 | `0x80350378` | 9 | 5 | 3232 | `0x802c8620` |
| 3 | `0x80304e18` | 186 | 23 | 3160 | `0x80033230` |
| 3 | `0x8030f15c` | 11 | 6 | 3392 | `0x80069a60` |
| 3 | `0x80306400` | 193 | 17 | 3044 | `0x8003de00` |
| 3 | `0x80304494` | 173 | 18 | 3032 | `0x8002ef80` |
| 3 | `0x80323370` | 158 | 13 | 2892 | `0x801105e0` |
| 3 | `0x803503a8` | 5 | 3 | 2808 | `0x802ca8e0` |
| 3 | `0x80329d78` | 23 | 11 | 2720 | `0x80171bc0` |
| 3 | `0x8034f790` | 5 | 2 | 2648 | `0x802a9e10` |
| 3 | `0x8030150c` | 77 | 16 | 2628 | `0x8001cbf0` |
| 3 | `0x80301eb0` | 185 | 10 | 2472 | `0x80022ea0` |
| 3 | `0x8034fb80` | 7 | 6 | 2452 | `0x802b5110` |
| 3 | `0x8034eed8` | 2 | 1 | 2404 | `0x8026e940` |
| 3 | `0x8032b43c` | 27 | 4 | 2376 | `0x80184f20` |
| 3 | `0x8034f770` | 5 | 2 | 2272 | `0x802a4f70` |
| 3 | `0x803294e0` | 14 | 6 | 2252 | `0x80006d90` |
| 3 | `0x8034f938` | 31 | 5 | 2240 | `0x802ae190` |
| 3 | `0x8034e8b0` | 16 | 2 | 2240 | `0x8025ab10` |
| 3 | `0x8034f728` | 4 | 2 | 2220 | `0x802a1a20` |
| 3 | `0x8034f5e0` | 3 | 1 | 2208 | `0x80291d30` |
| 3 | `0x80308a88` | 44 | 11 | 2176 | `0x8016c920` |
| 3 | `0x80305344` | 169 | 13 | 2136 | `0x80035960` |
| 3 | `0x8034f680` | 1 | 1 | 2132 | `0x80297ec0` |
| 3 | `0x803502cc` | 11 | 1 | 2084 | `0x802c5760` |
| 3 | `0x80315778` | 17 | 7 | 2080 | `0x800b6a80` |
| 3 | `0x8031c288` | 42 | 13 | 2072 | `0x800fe360` |
| 3 | `0x803175e8` | 46 | 7 | 2032 | `0x800ca1e0` |
| 3 | `0x8030f0c0` | 11 | 4 | 1940 | `0x80069d70` |
| 3 | `0x8032b100` | 23 | 8 | 1872 | `0x8017e3c0` |
| 3 | `0x803276f8` | 18 | 3 | 1816 | `0x80157960` |
| 3 | `0x80323fe0` | 55 | 9 | 1804 | `0x8011bb30` |
| 3 | `0x8034f7f8` | 27 | 7 | 1780 | `0x802ab8f0` |
| 3 | `0x8034e8f8` | 16 | 4 | 1740 | `0x8025db30` |
| 3 | `0x803504a8` | 5 | 2 | 1676 | `0x802c38a0` |
| 3 | `0x80311370` | 5 | 4 | 1644 | `0x800909a0` |
| 3 | `0x8030f128` | 11 | 2 | 1592 | `0x80198290` |
| 3 | `0x8034e398` | 16 | 9 | 1564 | `0x80244bf0` |
| 3 | `0x8034e4b8` | 16 | 9 | 1552 | `0x802461f0` |
| 3 | `0x8034e278` | 16 | 9 | 1548 | `0x80243420` |
| 3 | `0x8034e590` | 20 | 6 | 1544 | `0x80248c50` |
| 3 | `0x8032b240` | 11 | 3 | 1524 | `0x8017fab0` |
| 3 | `0x8034f740` | 4 | 2 | 1516 | `0x802a2310` |
| 3 | `0x80305f18` | 185 | 12 | 1508 | `0x8003ca30` |
| 3 | `0x8030f060` | 10 | 5 | 1508 | `0x80069dd0` |
| 3 | `0x8031ba24` | 9 | 3 | 1452 | `0x800ee250` |
| 3 | `0x80303aa4` | 183 | 10 | 1440 | `0x8002bef0` |
| 3 | `0x8032e040` | 14 | 4 | 1408 | `0x80197650` |
| 3 | `0x803089c8` | 14 | 6 | 1404 | `0x800465f0` |
| 3 | `0x80306a28` | 78 | 8 | 1356 | `0x80040960` |
| 3 | `0x8034f670` | 1 | 1 | 1332 | `0x80297980` |
| 3 | `0x8030f090` | 10 | 5 | 1316 | `0x8006a090` |
| 3 | `0x8030f0f8` | 10 | 5 | 1308 | `0x80069bd0` |
| 3 | `0x8030f190` | 10 | 5 | 1308 | `0x80069f90` |
| 3 | `0x80318db8` | 1 | 1 | 1304 | `0x800da130` |
| 3 | `0x803500b0` | 10 | 4 | 1300 | `0x802bf210` |
| 3 | `0x80306d68` | 51 | 4 | 1296 | `0x80041ec0` |
| 3 | `0x803017d8` | 92 | 12 | 1288 | `0x8001dad0` |
| 3 | `0x803509c0` | 8 | 4 | 1264 | `0x802db1a0` |
| 3 | `0x8032df1c` | 42 | 11 | 1232 | `0x80194ab0` |
| 3 | `0x8032b000` | 6 | 4 | 1204 | `0x8017d740` |
| 3 | `0x80323d40` | 78 | 6 | 1168 | `0x8011a570` |
| 3 | `0x80350488` | 5 | 2 | 1152 | `0x802c3530` |
| 3 | `0x8031b0d4` | 48 | 2 | 1152 | `0x800e4b60` |
| 3 | `0x80311328` | 5 | 2 | 1132 | `0x8008fe30` |
| 3 | `0x8034f3b8` | 8 | 4 | 1116 | `0x80287270` |
| 3 | `0x80308a48` | 14 | 4 | 1112 | `0x800464f0` |
| 3 | `0x8034f758` | 4 | 2 | 1112 | `0x802a2940` |
| 3 | `0x80324e68` | 1 | 1 | 1104 | `0x801305b0` |
| 3 | `0x80311b84` | 7 | 6 | 1100 | `0x80092f10` |
| 3 | `0x80318e18` | 4 | 3 | 1092 | `0x800db800` |
| 3 | `0x8031bf70` | 60 | 7 | 1076 | `0x800f74a0` |
| 3 | `0x8034f350` | 6 | 1 | 1072 | `0x80285e40` |
| 3 | `0x80324778` | 54 | 5 | 1068 | `0x80123330` |
| 3 | `0x8034ffa8` | 12 | 1 | 1024 | `0x802bcde0` |
| 3 | `0x80310ba4` | 11 | 2 | 1008 | `0x802c3980` |
| 3 | `0x80329cd0` | 24 | 3 | 972 | `0x80171850` |
| 3 | `0x80308a08` | 14 | 4 | 964 | `0x80046570` |
| 3 | `0x8034f1a8` | 7 | 2 | 956 | `0x8027ce00` |
| 3 | `0x802fe450` | 14 | 4 | 936 | `0x8000f840` |
| 3 | `0x8031c338` | 4 | 3 | 920 | `0x800fe2a0` |
| 3 | `0x80323c20` | 4 | 2 | 920 | `0x80115cd0` |
| 3 | `0x803227b8` | 19 | 5 | 892 | `0x8010c170` |
| 3 | `0x80324ec4` | 1 | 1 | 876 | `0x80132f60` |
| 3 | `0x8034f8c0` | 28 | 3 | 872 | `0x802ada70` |
| 3 | `0x80318e00` | 1 | 1 | 864 | `0x800da6b0` |
| 3 | `0x802ff678` | 6 | 2 | 856 | `0x80014b80` |
| 3 | `0x80322838` | 18 | 4 | 848 | `0x8010cb40` |
| 3 | `0x8034f650` | 1 | 1 | 840 | `0x802972f0` |
| 3 | `0x8034f660` | 1 | 1 | 824 | `0x80297640` |
| 3 | `0x80310534` | 1 | 1 | 820 | `0x80086370` |
| 3 | `0x803157e0` | 17 | 4 | 816 | `0x800b6d50` |
| 3 | `0x8032b380` | 25 | 3 | 796 | `0x801847d0` |
| 3 | `0x8034f220` | 21 | 4 | 788 | `0x8027f510` |
| 3 | `0x80327d38` | 18 | 3 | 752 | `0x8015a580` |
| 3 | `0x802f11f8` | 1 | 1 | 752 | `0x800d16f0` |
| 3 | `0x80303660` | 171 | 5 | 744 | `0x8002ae50` |
| 3 | `0x803500e0` | 11 | 5 | 736 | `0x802bf440` |
| 3 | `0x80306f80` | 57 | 4 | 732 | `0x80042700` |
| 3 | `0x8034fee0` | 7 | 2 | 732 | `0x802bb2a0` |
| 3 | `0x8034ce7c` | 21 | 4 | 728 | `0x8022be10` |
| 3 | `0x802f4cd0` | 1 | 1 | 724 | `0x8011e720` |
| 3 | `0x802fe3e0` | 4 | 2 | 720 | `0x80006eb0` |
| 3 | `0x8034cdb4` | 21 | 5 | 704 | `0x8022b6e0` |
| 3 | `0x80350300` | 5 | 1 | 704 | `0x802c6230` |
| 3 | `0x80324a44` | 51 | 4 | 676 | `0x80124e80` |
| 3 | `0x80307070` | 72 | 4 | 656 | `0x80042b40` |
| 3 | `0x80350970` | 10 | 2 | 656 | `0x802daa20` |
| 3 | `0x80324e80` | 1 | 1 | 656 | `0x80131470` |
| 3 | `0x803073c8` | 55 | 1 | 616 | `0x80043a30` |
| 3 | `0x80322ef8` | 19 | 5 | 616 | `0x8010f4e0` |
| 3 | `0x803228c0` | 18 | 3 | 616 | `0x8010d3e0` |
| 3 | `0x8034f008` | 12 | 4 | 608 | `0x802724a0` |
| 3 | `0x80307a7c` | 50 | 3 | 604 | `0x80045650` |
| 3 | `0x80307c74` | 1 | 1 | 600 | `0x80049270` |
| 3 | `0x80328af0` | 10 | 1 | 592 | `0x801605b0` |
| 3 | `0x8034e350` | 16 | 4 | 592 | `0x802457c0` |
| 3 | `0x8034e470` | 16 | 4 | 592 | `0x80246dc0` |
| 3 | `0x80301354` | 7 | 5 | 588 | `0x8001bbf0` |
| 3 | `0x8030140c` | 7 | 5 | 588 | `0x8001c1d0` |
| 3 | `0x803014c4` | 7 | 5 | 588 | `0x8001c6f0` |
| 3 | `0x8034e230` | 16 | 4 | 588 | `0x80243fe0` |
| 3 | `0x803177e8` | 5 | 2 | 568 | `0x800cb230` |
| 3 | `0x8032663c` | 52 | 3 | 564 | `0x8014adb0` |
| 3 | `0x8034ccb0` | 23 | 4 | 564 | `0x8022a710` |
| 3 | `0x8032c718` | 20 | 4 | 564 | `0x8018a770` |
| 3 | `0x8032c8b8` | 3 | 1 | 564 | `0x8018b790` |
| 3 | `0x8032c900` | 3 | 1 | 564 | `0x8018b230` |
| 3 | `0x8034e830` | 9 | 3 | 560 | `0x80259560` |
| 3 | `0x80318e30` | 4 | 2 | 560 | `0x800daf40` |
| 3 | `0x803294c8` | 4 | 2 | 560 | `0x801639a0` |
| 3 | `0x80325140` | 1 | 1 | 556 | `0x8013c850` |
| 3 | `0x803130f8` | 167 | 3 | 552 | `0x8009edb0` |
| 3 | `0x8031b028` | 6 | 3 | 552 | `0x800e33c0` |
| 3 | `0x80324ed0` | 1 | 1 | 532 | `0x801332d0` |
| 3 | `0x80317838` | 4 | 3 | 528 | `0x800cceb0` |
| 3 | `0x80317e10` | 4 | 3 | 524 | `0x800d2fb0` |
| 3 | `0x803504c8` | 5 | 1 | 520 | `0x802d0890` |
| 3 | `0x80323898` | 54 | 5 | 516 | `0x80114d70` |
| 3 | `0x8034f27c` | 3 | 2 | 504 | `0x80280730` |
| 3 | `0x80324f78` | 1 | 1 | 496 | `0x80135530` |
| 3 | `0x80350540` | 21 | 6 | 492 | `0x802d4400` |
| 3 | `0x8034e710` | 4 | 3 | 492 | `0x8024d1d0` |
| 3 | `0x8034fd68` | 10 | 4 | 488 | `0x802b92e0` |
| 3 | `0x802edda8` | 1 | 1 | 488 | `0x80015860` |
| 3 | `0x8034cc34` | 3 | 1 | 484 | `0x802294e0` |
| 3 | `0x803249e8` | 19 | 3 | 480 | `0x80124c70` |
| 3 | `0x803114b0` | 5 | 4 | 480 | `0x80091170` |
| 3 | `0x8031151c` | 5 | 4 | 480 | `0x80091860` |
| 3 | `0x80309940` | 54 | 4 | 476 | `0x800621c0` |
| 3 | `0x8034cc48` | 23 | 3 | 464 | `0x8022a280` |
| 3 | `0x80315848` | 17 | 2 | 460 | `0x800b6690` |
| 3 | `0x8031bc68` | 22 | 4 | 460 | `0x800f27d0` |
| 3 | `0x8034e1a8` | 15 | 3 | 460 | `0x80242e20` |
| 3 | `0x80312320` | 14 | 2 | 456 | `0x80098530` |
| 3 | `0x8034cd18` | 21 | 3 | 456 | `0x8022af70` |
| 3 | `0x8034eb20` | 7 | 2 | 456 | `0x8026ac20` |
| 3 | `0x8034eb70` | 7 | 2 | 456 | `0x8026b750` |
| 3 | `0x8034ebc0` | 7 | 2 | 456 | `0x8026c440` |
| 3 | `0x8034ec10` | 7 | 2 | 456 | `0x8026d200` |
| 3 | `0x80322fb0` | 158 | 4 | 448 | `0x8010f8d0` |
| 3 | `0x80350320` | 5 | 3 | 448 | `0x802c66f0` |
| 3 | `0x80318e48` | 4 | 2 | 440 | `0x80044400` |
| 3 | `0x80317868` | 4 | 3 | 428 | `0x800cca80` |
| 3 | `0x8032def8` | 2 | 2 | 416 | `0x80194b90` |
| 3 | `0x802f1234` | 1 | 1 | 416 | `0x800d2060` |
| 3 | `0x8034f6c0` | 1 | 1 | 416 | `0x8029b0f0` |
| 3 | `0x80307b50` | 54 | 3 | 412 | `0x80046000` |
| 3 | `0x8034ff48` | 6 | 1 | 412 | `0x802bb660` |
| 3 | `0x803272d8` | 19 | 4 | 408 | `0x80156f40` |
| 3 | `0x80314fa0` | 2 | 2 | 408 | `0x800b0840` |
| 3 | `0x80317920` | 22 | 4 | 404 | `0x800cd7d0` |
| 3 | `0x8034ef70` | 6 | 2 | 404 | `0x8026f980` |
| 3 | `0x8034fec0` | 6 | 2 | 404 | `0x802bad00` |
| 3 | `0x803104ec` | 1 | 1 | 404 | `0x800861d0` |
| 3 | `0x80322dd8` | 19 | 4 | 396 | `0x8010eff0` |
| 3 | `0x8034fdc8` | 10 | 4 | 396 | `0x802b9b00` |
| 3 | `0x802f4aa0` | 1 | 1 | 396 | `0x80108870` |
| 3 | `0x8034eaf8` | 7 | 3 | 392 | `0x8026a750` |
| 3 | `0x8034eb48` | 7 | 3 | 392 | `0x8026b240` |
| 3 | `0x8034eb98` | 7 | 3 | 392 | `0x8026be10` |
| 3 | `0x8034ebe8` | 7 | 3 | 392 | `0x8026cc00` |
| 3 | `0x80350898` | 24 | 5 | 384 | `0x802d9af0` |
| 3 | `0x80318d80` | 2 | 2 | 384 | `0x800d9420` |
| 3 | `0x803105a0` | 1 | 1 | 384 | `0x800868e0` |
| 3 | `0x80318d90` | 1 | 1 | 380 | `0x800d9b80` |
| 3 | `0x80325028` | 1 | 1 | 380 | `0x80136eb0` |
| 3 | `0x80317880` | 4 | 3 | 376 | `0x800cc8e0` |
| 3 | `0x8034e7d0` | 9 | 2 | 376 | `0x80257900` |
| 3 | `0x8034f1f0` | 4 | 2 | 376 | `0x8027f1a0` |
| 3 | `0x802f5810` | 1 | 1 | 376 | `0x80152130` |
| 3 | `0x8031057c` | 1 | 1 | 376 | `0x80086760` |
| 3 | `0x8034fd98` | 10 | 3 | 368 | `0x802b97c0` |
| 3 | `0x8034fdf8` | 10 | 3 | 368 | `0x802b9eb0` |
| 3 | `0x8030fe48` | 6 | 3 | 364 | `0x80082370` |
| 3 | `0x8035044c` | 5 | 3 | 360 | `0x802ce7a0` |
| 3 | `0x803251ec` | 1 | 1 | 360 | `0x8013f750` |
| 3 | `0x8034f2d8` | 7 | 3 | 356 | `0x80283e80` |
| 3 | `0x8034ead8` | 6 | 2 | 352 | `0x80269270` |
| 3 | `0x802f50e4` | 1 | 1 | 352 | `0x80141cd0` |
| 3 | `0x802f4d54` | 1 | 1 | 348 | `0x80123a10` |
| 3 | `0x8034f310` | 8 | 1 | 344 | `0x80284b20` |
| 3 | `0x8034e860` | 14 | 3 | 340 | `0x80258b20` |
| 3 | `0x80328a80` | 14 | 11 | 660 | `0x8015fcd0` |
| 3 | `0x80316ba0` | 14 | 3 | 340 | `0x800c0e80` |
| 3 | `0x8034f040` | 4 | 2 | 336 | `0x80272440` |
| 3 | `0x80310ee0` | 45 | 3 | 332 | `0x8008fd20` |
| 3 | `0x80309cb0` | 51 | 2 | 332 | `0x80063630` |
| 3 | `0x802fea30` | 1 | 1 | 324 | `0x80010140` |
| 3 | `0x80317850` | 4 | 3 | 316 | `0x800ccc40` |
| 3 | `0x80302740` | 4 | 2 | 312 | `0x80025d20` |
| 3 | `0x8031bc1c` | 7 | 2 | 312 | `0x800f2530` |
| 3 | `0x8032525c` | 1 | 1 | 308 | `0x80141320` |
| 3 | `0x803074b0` | 51 | 3 | 300 | `0x80044760` |
| 3 | `0x802f3404` | 1 | 1 | 300 | `0x800f7b40` |
| 3 | `0x8032df08` | 3 | 2 | 296 | `0x80197060` |
| 3 | `0x8034f140` | 2 | 1 | 296 | `0x80279f00` |
| 3 | `0x80315894` | 17 | 1 | 292 | `0x800b6c20` |
| 3 | `0x8034f2a0` | 12 | 2 | 292 | `0x80284000` |
| 3 | `0x802eddc0` | 1 | 1 | 292 | `0x80015ad0` |
| 3 | `0x80303410` | 4 | 1 | 292 | `0x80168de0` |
| 3 | `0x8034f068` | 5 | 2 | 288 | `0x80274f10` |
| 3 | `0x80324e8c` | 1 | 1 | 288 | `0x80131870` |
| 3 | `0x802f11dc` | 1 | 1 | 284 | `0x800d0fe0` |
| 3 | `0x8031bbc0` | 1 | 1 | 284 | `0x800ee2b0` |
| 3 | `0x80328a48` | 10 | 2 | 276 | `0x8015fae0` |
| 3 | `0x80324dd4` | 1 | 1 | 276 | `0x8012d940` |
| 3 | `0x80326764` | 1 | 1 | 276 | `0x8014f9b0` |
| 3 | `0x8034eef8` | 4 | 3 | 272 | `0x8026f540` |
| 3 | `0x8030175c` | 11 | 2 | 268 | `0x8001d770` |
| 3 | `0x80301b64` | 11 | 2 | 268 | `0x8001f3e0` |
| 3 | `0x80302360` | 11 | 2 | 268 | `0x80023980` |
| 3 | `0x80302980` | 11 | 2 | 268 | `0x800268d0` |
| 3 | `0x80302cb4` | 11 | 2 | 268 | `0x80027010` |
| 3 | `0x803035e4` | 11 | 2 | 268 | `0x8002a860` |
| 3 | `0x80303e98` | 11 | 2 | 268 | `0x8002c890` |
| 3 | `0x80304400` | 11 | 2 | 268 | `0x8002ec70` |
| 3 | `0x80304888` | 11 | 2 | 268 | `0x80030f00` |
| 3 | `0x80304d48` | 11 | 2 | 268 | `0x80032f00` |
| 3 | `0x803052b0` | 11 | 2 | 268 | `0x800353f0` |
| 3 | `0x80305678` | 11 | 2 | 268 | `0x80036650` |
| 3 | `0x80305e60` | 11 | 2 | 268 | `0x8003c710` |
| 3 | `0x80306324` | 11 | 2 | 268 | `0x8003d8f0` |
| 3 | `0x80306998` | 11 | 2 | 268 | `0x80040640` |
| 3 | `0x80306c80` | 11 | 2 | 268 | `0x80041920` |
| 3 | `0x8030735c` | 11 | 2 | 268 | `0x800434f0` |
| 3 | `0x803078b0` | 11 | 2 | 268 | `0x80044f10` |
| 3 | `0x803098d4` | 11 | 2 | 268 | `0x80061e10` |
| 3 | `0x80309a1c` | 8 | 2 | 268 | `0x80062850` |
| 3 | `0x80309bd8` | 11 | 2 | 268 | `0x80062c20` |
| 3 | `0x803125e0` | 11 | 2 | 268 | `0x8009a9c0` |
| 3 | `0x80313090` | 11 | 2 | 268 | `0x8009e750` |
| 3 | `0x80313424` | 11 | 2 | 268 | `0x8009f310` |
| 3 | `0x803136a0` | 11 | 2 | 268 | `0x800a2590` |
| 3 | `0x80313e70` | 11 | 2 | 268 | `0x800a7170` |
| 3 | `0x80314f38` | 11 | 2 | 268 | `0x800b04d0` |
| 3 | `0x80323274` | 11 | 2 | 268 | `0x8010fe90` |
| 3 | `0x80323edc` | 11 | 2 | 268 | `0x8011b220` |
| 3 | `0x803265b8` | 11 | 2 | 268 | `0x8014a410` |
| 3 | `0x8034fbe8` | 8 | 2 | 268 | `0x802b7cb0` |
| 3 | `0x803509e8` | 8 | 2 | 268 | `0x802db870` |
| 3 | `0x803214b8` | 2 | 1 | 268 | `0x80102ca0` |
| 3 | `0x80324f34` | 1 | 1 | 268 | `0x80134740` |
| 3 | `0x8034f708` | 5 | 1 | 268 | `0x8029f7e0` |
| 3 | `0x80329708` | 7 | 2 | 264 | `0x8016abe0` |
| 3 | `0x80311634` | 7 | 2 | 256 | `0x80092cb0` |
| 3 | `0x8031b19c` | 3 | 2 | 256 | `0x800e6310` |
| 3 | `0x802eea98` | 1 | 1 | 256 | `0x8004ac90` |
| 3 | `0x80325034` | 1 | 1 | 256 | `0x80137030` |
| 3 | `0x8034f338` | 3 | 2 | 252 | `0x802856f0` |
| 3 | `0x80329748` | 3 | 2 | 248 | `0x8016c010` |
| 3 | `0x8031afc8` | 18 | 3 | 236 | `0x800e1ae0` |
| 3 | `0x80301244` | 7 | 3 | 232 | `0x8001b400` |
| 3 | `0x80311e80` | 14 | 2 | 232 | `0x80067320` |
| 3 | `0x8030791c` | 71 | 3 | 212 | `0x80045450` |
| 3 | `0x8034ea60` | 2 | 2 | 200 | `0x80267800` |
| 3 | `0x802fe430` | 6 | 2 | 188 | `0x8000f410` |
| 3 | `0x802fe490` | 6 | 2 | 188 | `0x8000f320` |
| 3 | `0x8032975c` | 3 | 2 | 188 | `0x8016b9c0` |
| 3 | `0x802ff644` | 11 | 2 | 184 | `0x80012910` |
| 3 | `0x80303a54` | 11 | 2 | 184 | `0x8002bd20` |
| 3 | `0x80305914` | 11 | 2 | 184 | `0x80037d80` |
| 3 | `0x80309f20` | 11 | 2 | 184 | `0x80063e60` |
| 3 | `0x80315830` | 4 | 2 | 184 | `0x800b76e0` |
| 3 | `0x80315ad8` | 11 | 2 | 184 | `0x800ba550` |
| 3 | `0x80318be0` | 11 | 2 | 184 | `0x800d7bf0` |
| 3 | `0x8031b34c` | 11 | 2 | 184 | `0x800e9fd0` |
| 3 | `0x803226a4` | 12 | 2 | 184 | `0x8010bf90` |
| 3 | `0x803237ec` | 12 | 2 | 184 | `0x801149a0` |
| 3 | `0x803246a8` | 11 | 2 | 184 | `0x80123190` |
| 3 | `0x803249b0` | 11 | 2 | 184 | `0x80124ae0` |
| 3 | `0x80313b20` | 4 | 2 | 160 | `0x800a2f00` |
| 3 | `0x802fe418` | 3 | 2 | 156 | `0x80006930` |
| 3 | `0x80317730` | 6 | 2 | 156 | `0x800cc350` |
| 3 | `0x80317750` | 6 | 2 | 156 | `0x800cc050` |
| 3 | `0x8034ec38` | 21 | 2 | 152 | `0x8026e0a0` |
| 3 | `0x803297e8` | 3 | 2 | 124 | `0x8016af50` |
| 3 | `0x8034e988` | 11 | 2 | 112 | `0x8025f230` |
| 3 | `0x8034e310` | 3 | 2 | 112 | `0x80243370` |
| 3 | `0x8034e430` | 3 | 2 | 112 | `0x80244b40` |
| 3 | `0x8034e550` | 3 | 2 | 112 | `0x80246140` |
| 3 | `0x8034e6a0` | 3 | 2 | 112 | `0x80247790` |
| 3 | `0x80329640` | 6 | 2 | 108 | `0x80169ef0` |


