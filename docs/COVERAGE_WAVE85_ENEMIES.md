# Coverage - Wave 85 (enemies: the `0x80A10000` bucket's opening runs)

## How to read this file

This page records the Wave 85 `game::enemies` lane: a pass over the
`0x80A10000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below is one real `pub unsafe fn fn_<addr>`
body in that file, and the `evidence` column of the machine-readable mirror
(`ledger/agents/wave85-enemies.csv`) names the Ghidra C export each body was
reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32` / `read_u16` / `read_u8`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 3,480 | 14 | 15 |

The bucket holds 209 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 91 were already in
`ledger/functions.csv` before this pass, 29 are claimed here and 89 remain.

## What this pass covers

Three runs.

**Cluster 1, `0x80A10170..0x80A107B0`, the segmented trail actor.** The run
opens with the history registration pair and then the four frame helpers that
maintain it:

- `0x80A10170` seeds the trail history: the record count `400` goes to
  `+0x928`, `0x802B9250` allocates `0x2BD0` bytes (`400 * 0x1C`) and
  `0x802DC7E0` registers them with element size `0x1C` and the element callback
  [`fn_80a101e0`] in `src/game/enemies.rs`;
  the registered block lands at `+0x92C`;
- `0x80A101E0` is that element callback: it releases the block through
  `0x802B9280` when the block is non-null and the count is positive, and
  returns its first argument;
- `0x80A10410` shifts the 400-record ring one record toward the tail and seeds
  the head record from `+0x5C4`/`+0x5C8`, the three angle halfwords
  `+0x5D0..+0x5D4`, the `0x802BE470` segment length and the
  `0x800708C0`/`0x80070820` angle pair. On side 0 the stored halfword angle is
  negated;
- `0x80A10590` binds the two collision descriptors (`0x8006D090` on `+0x1EC`)
  and the two walk clips (`0x800B66F0`/`0x800B6790`) on every segment;
- `0x80A10630` recomputes the two offset pairs `+0xE0`/`+0xE4` and
  `+0x184`/`+0x188` per segment from the target object at `+0x224`;
- `0x80A106C0` resets the two matrix blocks `+0xC4`/`+0x168` per segment;
- `0x80A10730` (no argument; it fetches the actor with the frame entry
  `0x802DCD74`) starts the `walk_normal` clip and the `+0x48` animation per
  segment;
- `0x80A107B0` clears the three speed words `+0xE8..+0xF0`, calls
  [`fn_80a10730`] in `src/game/enemies.rs`
  and seeds the per-segment angles from the five words at
  `0x80B2ADA0..0x80B2ADB0`.

Every segment walk uses the same chain: count `+0x540` plus one, first segment
`+0x544`, next segment read from `+0x238`.

**Cluster 2, `0x80A17FF0..0x80A18880`, the `daikonbou` actor.** The run is the
release helper, a sixth copy of the descriptor skeleton's producers and the
actor's placement helpers:

- `0x80A17FF0` releases the five holders `+0x608`, `+0x5B8`, `+0x588`,
  `+0x544` and `+0x524`, runs the base teardown `0x80095130`, and - for a
  positive mode - the heap release `0x80162920`;
- the six 12-byte producers `0x80A18170..0x80A181C0` all return the shared word
  `0x80377A04`;
- `0x80A18280` is the model binder: `0x800690C0` builds the `+0x524`
  parameter object, `0x800DF180` opens the manager and looks up
  `daikonbou`/`g3d/daikonbou.brres`, `0x80239E30`/`0x8023A200`/`0x8023A270`
  supply the node and the two models, and `0x80169D90`/`0x80166830`/`0x80167420`
  build the `+0x544`, `+0x5B8` and `+0x588` holders. Spawn flag bit 25 selects
  one of the two blend rates `0x80B2B494`/`0x80B2B498`;
- `0x80A184A0` ticks the `+0x544` holder through vtable slot `+0x14` and
  returns 1;
- `0x80A18560` rebuilds the `+0x7C` matrix from the actor position raised by
  `0x80B2B49C`, runs the three angle setters and commits the matrix and the
  `+0xDC` companion into `+0x544`;
- `0x80A18650` integrates the pitch from the height above `+0xBC`, clamped to
  `0xA00`, into `+0x102`;
- `0x80A186A0` registers the two parking arms at `+0x148`/`+0x5F0`, sets the
  mode byte `+0x68E = 3` and copies the four-word height block into
  `+0x630..+0x63C`;
- `0x80A18790` spawns effect `0x2DE` with X/Y from the two height probes
  `0x800B30C0`/`0x800B30E0` and Z from `+0xB4`;
- `0x80A18800` spawns the `Wm_ob_cmnspark` effect at the actor position;
- `0x80A18860`/`0x80A18880` are the two `countLeadingZeros(v - 1) >> 5` /
  `countLeadingZeros(v - 2) >> 5` predicates, i.e. the branch-free
  `scene_state(+0x160) == 1` and `== 2` tests on the object behind
  `0x80429F10`.

**Cluster 3, `0x80A194B0..0x80A19850`, the `0x730`-byte sibling's skeleton.**
Two state normalisers (`0x80A194B0`, `0x80A19510` - both
`0x8015F800` plus a conditional `0x802B9280`), the five-link `+0x30` chain
walker `0x80A19650`, the `:`-suffix name predicate `0x80A19730` and the
allocating constructor `0x80A19850` (size `0x730`, class vtable `0x80B37918`,
and the same `+0x524`/`+0x544`/`+0x584`/`+0x5C8`/`+0x5F4` holder layers that
[`fn_80a18280`] in `src/game/enemies.rs`
consumes).

### Identity evidence

- **`g3d/daikonbou.brres` is read, not guessed.** `0x80A18280` passes the two
  strings the export prints as `s_daikonbou_80b373c8` and
  `s_g3d_daikonbou_brres_80b373b4` to `0x800DF180`, and the same model name is
  looked up three times on `+0x540`.
- **The two family members share their holders.** The constructor at
  `0x80A19850` initialises exactly the holder chain (`+0x524`, `+0x544`,
  `+0x584`, `+0x5C8`, `+0x5F4`) that the binder at `0x80A18280` fills, and both
  bind a `0x80B3xxxx` animation-holder vtable; the offsets are inherited from
  the binder rather than asserted independently.
- **The trail actor is described by offsets only.** Its class name is not
  printed anywhere in this run, so the module text and this page name it by
  structure: the `+0x928`/`+0x92C` 400-record history and the `+0x540`/`+0x544`
  `+0x238` segment chain.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A10170` | `fn_80a10170` | 92 | medium |
| `0x80A101E0` | `fn_80a101e0` | 64 | high |
| `0x80A10410` | `fn_80a10410` | 380 | medium |
| `0x80A10590` | `fn_80a10590` | 156 | medium |
| `0x80A10630` | `fn_80a10630` | 144 | medium |
| `0x80A106C0` | `fn_80a106c0` | 108 | high |
| `0x80A10730` | `fn_80a10730` | 120 | medium |
| `0x80A107B0` | `fn_80a107b0` | 192 | medium |
| `0x80A17FF0` | `fn_80a17ff0` | 156 | high |
| `0x80A18170` | `fn_80a18170` | 12 | high |
| `0x80A18180` | `fn_80a18180` | 12 | high |
| `0x80A18190` | `fn_80a18190` | 12 | high |
| `0x80A181A0` | `fn_80a181a0` | 12 | high |
| `0x80A181B0` | `fn_80a181b0` | 12 | high |
| `0x80A181C0` | `fn_80a181c0` | 12 | high |
| `0x80A18280` | `fn_80a18280` | 456 | medium |
| `0x80A184A0` | `fn_80a184a0` | 48 | medium |
| `0x80A18560` | `fn_80a18560` | 232 | medium |
| `0x80A18650` | `fn_80a18650` | 76 | medium |
| `0x80A186A0` | `fn_80a186a0` | 144 | medium |
| `0x80A18790` | `fn_80a18790` | 112 | medium |
| `0x80A18800` | `fn_80a18800` | 84 | high |
| `0x80A18860` | `fn_80a18860` | 28 | high |
| `0x80A18880` | `fn_80a18880` | 28 | high |
| `0x80A194B0` | `fn_80a194b0` | 88 | high |
| `0x80A19510` | `fn_80a19510` | 92 | high |
| `0x80A19650` | `fn_80a19650` | 224 | medium |
| `0x80A19730` | `fn_80a19730` | 136 | medium |
| `0x80A19850` | `fn_80a19850` | 248 | medium |

## Deliberate skips

- **`0x80A10220` (484 bytes) and `0x80A10880` (940 bytes) stay untranslated.**
  Both exports carry unresolved `unaff_GQR0` paired-single conversions
  (`ldexpf` of the `GQR0` fields) at the top of the body, so the two float
  arguments cannot be reconstructed from the C alone. Waves 55, 83 and 84
  recorded the same limit for `0x80AC1800`, `0x80AC0750` and `0x80AC0840`.
  Nothing in this pass depends on their bodies; `0x80A10880` calls
  `0x80A10410`, which is translated here.
- **The three `halt_baddata` exports (`0x80A18730`, `0x80A18750`,
  `0x80A18770`, 24/24/28 bytes) stay untranslated.** Ghidra truncated their
  control flow at a bad instruction, so there is no instruction sequence to
  reconstruct. They sit inside the `daikonbou` run but are unreferenced by the
  translated bodies.
- **`0x80A197F0` and `0x80A19820` (48 bytes each) stay untranslated.** They are
  the skeleton's two `0x802DCBFC` deleting-destructor forwarders, byte-identical
  to the low-confidence copies already recorded in waves 75, 80, 81, 83 and 84
  (including `fn_80acf7a0`/`fn_80acf7d0`). Their hidden `r12` this-adjust
  pointer cannot be modelled, so repeating them would add no evidence.
- **The rest of the bucket.** The remaining 89 addresses are the large
  untranslated blocks at `0x80A10880`'s neighbours and the actor runs between
  `0x80A1A150` and `0x80A1FFB0`, at up to 716 bytes. They lie outside this
  lane's budget and are left for a later pass.
- **`fn_80a107b0` and `fn_80a10410` are `medium`, not `high`.** Their
  arithmetic is literal, but the roles of the `0x80B2ADA0` angle table and of
  the `+0x214..+0x220` offset block are inferred from their consumers; the
  bodies record that with `// unknown:` / `// observed:` notes rather than
  asserting a meaning. `fn_80a107b0` also notes that the export indexes a
  five-word local, so a sixth segment would read past it.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  3,480 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 91 existing rows are untouched).
- `node tools/check-lane-csv.mjs wave85` reports 29 rows for this CSV and `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`.
