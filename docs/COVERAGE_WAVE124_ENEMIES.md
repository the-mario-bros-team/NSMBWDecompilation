# Coverage - Wave 124 (`game::enemies`, the `0x80A60000` bucket remainder)

## How to read this file

This page records the Wave 124 `game::enemies` lane: 28 functions spread over
four runs of the `0x80A60000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 105 of that file. Every row of the machine
readable mirror (`ledger/agents/wave124-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 5,392 | 13 | 15 |

The bucket holds 198 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 144 were already in
`ledger/functions.csv`, leaving 54. This pass claims 28 of them, all in the
four runs below, and leaves 26 untouched.

Two rows are `partial` (the two `0x802DCBFC` deleting-destructor trampolines,
whose hidden `r12` this-adjust pointer `platform::call3` cannot model - the same
limitation recorded for the copies in waves 56, 72, 75, 80, 81, 82, 97, 98, 99,
101, 122 and 123). The other 26 rows are complete bodies.

## What this pass covers

- **Another descriptor skeleton (`0x80A66780..0x80A66950`, 4 bodies).**
  The five-step `+0x30` owner-chain walker `0x80A66780` (the `+0x0C` slot of
  each owner, re-entering on the fifth when all five report zero), the
  `:`-suffix name predicate `0x80A66860` (`0x802E1C40` trims both strings,
  `0x802E1AB4` compares the suffixes) and the two `0x802DCBFC` trampolines
  `0x80A66920`/`0x80A66950`.
- **The goal-set controller (`0x80A67A60..0x80A68A50`, 16 bodies).** One
  actor's whole "capture point" block. `0x80A67A60` is the four-bit player
  mask test; `0x80A67AE0` fills a six-float world record through `0x8006CED0`
  and rejects positions outside the scene manager's x band at
  `0x80429E00+0x38/+0x40`. `0x80A67B40` is the per-frame pass over every
  type-2 object (`0x80162DB0`): it re-colours the `0x40`-flagged objects and
  otherwise spawns the per-bucket item (`0x80B2D5F0`/`0x80B2D604`), appends the
  position to the `0x80429F10` buffer and releases the object.
  `0x80A67D50` bubble-sorts the four `+0xA38` slots (0x10 bytes each) by their
  `+0x04` key into descending order and claims the first free slot;
  `0x80A67E70` emits one trail effect per slot from the `0x80B2D618` spacing
  table; `0x80A67F40` derives the bucket from the `+0xA88` height difference in
  15.0 steps, inserts the slot (state from `0x80B2D630`, or `9`/`10` for the
  mask/attached cases) and spawns `0x80B2D640` with magnitude `0x80B2D660`;
  `0x80A68170` asks the target's `+0x60` vtable slot `+0x130` for a sub-slot
  and inserts twice. `0x80A682A0` scans the four players for one within
  `x ± 16.0`; `0x80A68380` compares the attached count with the
  `0x80429CA0` limit; `0x80A683D0` marks `+0xA94` when a bound player is still
  loaded; `0x80A68460` is the player/rider `+0xE0` attach test. `0x80A684F0`
  is the main step (180-frame entry timer, the stamp-selected quick path with
  30/90-frame retries and the `0x80056CF0` item gate, ending in the
  `0x80B725D0` record), `0x80A68850` refreshes the `+0x72C`/`+0x58C` holders,
  `0x80A68910` is the wrapper, `0x80A689C0` the shutdown hook and `0x80A68A50`
  the `-110.0` floor step that plays sound `0xB0` and enters `0x80B72610`.
- **The daikonbou support tail (`0x80A6A2F0`, `0x80A6B6A0..0x80A6C9A0`, 8
  bodies).** Three `0x8015F800` record-clear teardowns (with the optional
  `0x802B9280` release), the two class-word producers
  (`0x80353C64`/`0x80353BE4`), another five-step `+0x30` chain walker, its
  matching name predicate, and `0x80A6C9A0`, which sums the x distance of every
  valid rider over the `+0x41C` node list (`4096.0` scale, `>> 13` fixed-point
  truncation).
- **Constants.** The tables were read from `work/rels/elf/d_enemiesNP.elf` and
  are documented once in the section header: the `16.0`/`20.0`/`15.0`/`106.0`
  distances, the `256.0` search offset, the `1.833`/`-110.0` floor pair, the
  `[8,7,6,5,4,3,2,1]` state table, the `[0x10,7,6,5,4]` item ids, the
  `[8000,4000,2000,1000,800,400,200,100]` magnitudes, the
  `[8,16,24,32,20]` trail spacing and the `[1.1,1.05,0.95,0.9]` slot factors.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A66780` | `fn_80a66780` | 224 | medium |
| `0x80A66860` | `fn_80a66860` | 136 | high |
| `0x80A66920` | `fn_80a66920` | 48 | medium (partial) |
| `0x80A66950` | `fn_80a66950` | 48 | medium (partial) |
| `0x80A67A60` | `fn_80a67a60` | 120 | high |
| `0x80A67AE0` | `fn_80a67ae0` | 96 | high |
| `0x80A67B40` | `fn_80a67b40` | 520 | medium |
| `0x80A67D50` | `fn_80a67d50` | 280 | high |
| `0x80A67E70` | `fn_80a67e70` | 196 | medium |
| `0x80A67F40` | `fn_80a67f40` | 552 | medium |
| `0x80A68170` | `fn_80a68170` | 304 | medium |
| `0x80A682A0` | `fn_80a682a0` | 220 | medium |
| `0x80A68380` | `fn_80a68380` | 80 | high |
| `0x80A683D0` | `fn_80a683d0` | 136 | high |
| `0x80A68460` | `fn_80a68460` | 140 | medium |
| `0x80A684F0` | `fn_80a684f0` | 864 | medium |
| `0x80A68850` | `fn_80a68850` | 156 | medium |
| `0x80A68910` | `fn_80a68910` | 52 | high |
| `0x80A689C0` | `fn_80a689c0` | 116 | medium |
| `0x80A68A50` | `fn_80a68a50` | 228 | medium |
| `0x80A6A2F0` | `fn_80a6a2f0` | 92 | high |
| `0x80A6B6A0` | `fn_80a6b6a0` | 88 | high |
| `0x80A6B700` | `fn_80a6b700` | 92 | high |
| `0x80A6B760` | `fn_80a6b760` | 12 | high |
| `0x80A6B770` | `fn_80a6b770` | 12 | high |
| `0x80A6B860` | `fn_80a6b860` | 224 | medium |
| `0x80A6B940` | `fn_80a6b940` | 136 | high |
| `0x80A6C9A0` | `fn_80a6c9a0` | 220 | medium |

## Deliberate skips

- **`0x80A673D0` (1,080 bytes) is not claimed.** It is the `goal_set` model
  binder in front of the controller - `g3d/goal_set.brres`, the `baseA`,
  `toride_std` and `goal_flag` holders - but its export mixes `unaff_GQR0`
  conversion artefacts with the dropped `in_f31`/`in_ps31_1` parameters, so the
  two colour/scale words it consumes cannot be typed honestly. It stays an
  address-only neighbour of [`fn_80a67a60`] and [`fn_80a67b40`].
- **`0x80A656A0`/`0x80A656D0` (two 48-byte `0x802DCBFC` trampolines) and
  `0x80A69130`/`0x80A69160` (two more) are left for a later pass.** They are
  byte-identical to [`fn_80a66920`]; claiming four more `partial` rows would
  inflate the count without adding evidence.
- **`0x80A61E10` (64 bytes) is truncated**: its export ends in
  `halt_baddata` after the `+0x736` counter, so the branch it takes every 64th
  frame is not recoverable.
- **`0x80A628F0` (292), `0x80A62A30` (312), `0x80A637E0` (236),
  `0x80A638F0` (284), `0x80A63A30` (284), `0x80A63FF0` (392),
  `0x80A641F0` (380) and `0x80A6EA20` (212) are GQR-damaged.** Their exports
  pass halfwords and floats through `unaff_GQR0`/`unaff_GQR3` with
  `ldexpf` scale factors, so the source values of the stores are not typed.
  This is the same reason wave 123 recorded for the `bubble` helpers.
- **`0x80A6EFB0` (100) and `0x80A6F8C0` (48) are not claimed**: the first ends
  in `halt_baddata` in its middle branch, the second returns a `2^52`-biased
  double that the decompiler folded into a single constant.
- **`0x80A6FBC0` (624) depends on the dropped `0x80A6EFB0` branch** and keeps
  GQR prologue artefacts, so it is left with its neighbours.
- **`0x80A600B0` (240), `0x80A61410`/`0x80A61440` (48 each), `0x80A635D0`
  (488), `0x80A6A5F0`/`0x80A6A620` (48 each), `0x80A6BA00`/`0x80A6BA30` and
  `0x80A6C7E0` (48 each)** were not attempted: the 25-30 body budget was
  already met by the four runs above, and the six unclaimed `0x802DCBFC`
  trampolines would only repeat [`fn_80a66920`]. Nothing was written for them,
  so no ledger row claims them.
- **The other 26 addresses of the bucket are the ones listed above.** In
  ascending order with their `functions.tsv` sizes: `0x80A600B0` (240),
  `0x80A61410` (48), `0x80A61440` (48), `0x80A61E10` (64),
  `0x80A628F0` (292), `0x80A62A30` (312), `0x80A635D0` (488),
  `0x80A637E0` (236), `0x80A638F0` (284), `0x80A63A30` (284),
  `0x80A63FF0` (392), `0x80A641F0` (380), `0x80A656A0` (48),
  `0x80A656D0` (48), `0x80A673D0` (1,080), `0x80A69130` (48),
  `0x80A69160` (48), `0x80A6A5F0` (48), `0x80A6A620` (48),
  `0x80A6BA00` (48), `0x80A6BA30` (48), `0x80A6C7E0` (48),
  `0x80A6EA20` (212), `0x80A6EFB0` (100), `0x80A6F8C0` (48) and
  `0x80A6FBC0` (624).
- **No address already in `ledger/functions.csv` was touched.**

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  5,392 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was in
  `ledger/functions.csv` before this pass.
- `cargo check --message-format=short` emits no diagnostic naming
  `src/game/enemies.rs`.
- The module header was updated: cluster item 105, the cluster count (one
  hundred and four to one hundred and five), the function totals (2,811 to
  2,839 public translations, 2,740 to 2,768 `fn_<addr>` bodies), the "start
  here" list and the records list.
