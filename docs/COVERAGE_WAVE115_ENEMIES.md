# Coverage - Wave 115 (`game::enemies`, the `0x80B00000` bucket)

## How to read this file

This page records the Wave 115 `game::enemies` lane: 30 functions of the
`0x80B00000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 96 of that file. Every row of the machine
readable mirror (`ledger/agents/wave115-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 3,884 | 20 | 10 |

Eight of the ten `medium` rows are `partial`: the 48-byte destructor
forwarders carry a hidden `r12` this-adjust pointer that neither
`platform::call3` nor the Ghidra export can express. They are recorded as
`partial/medium` exactly as wave 13 and wave 114 recorded the identical
forwarders at `0x80A6C810..0x80A6E4A0` and `0x80ACAAC0..0x80ACE510`.

## What this pass covers

- **`0x80B0E310..0x80B0EF00`, the `larry` demo actor.** The `0x5C0`-byte
  constructor `0x80B0E490` (vtable `0x80B68448`) and the `0x718`-byte sibling
  `0x80B0EEF0` share the `+0x524`/`+0x540`/`+0x544`/`+0x584` holder chain; the
  sibling adds a `+0x5CC` animation holder (`0x803295D8` replaced by
  `0x80329548`) and a `+0x604` member with vtable `0x80B4C310`. Between them
  sit the `g3d/larry.brres` binder `0x80B0E5E0` (model `+0x540`, node
  `+0x544`, clip pair `+0x584`), the matrix rebuild `0x80B0E780` (three axis
  writes around the `0x80B31F34..0x80B31F3C` rotation constants, then
  `0x8016A170`/`0x8016A150`/`0x8016A1A0` on the model holder), the two clip
  entries `0x80B0E8C0`/`0x80B0EA10` for `demo_battleA`/`demo_battleA_wait`,
  their two frame steps `0x80B0E970`/`0x80B0EAB0` (only the first has the
  `0x80B78A48` state switch), two detach helpers and four forwarders.
- **`0x80B0F4A0..0x80B0FF50`, the `met_toge` actor.** The `0x7F8`-byte
  constructor `0x80B0FD70` (vtable `0x80B68BF8`, `+0x5BC` holder, `+0x5CA`
  byte, and the `8 x 0x40` pool at `+0x5EC` built by `0x802DC9A0` with the
  `0x80A2FC30` table and the `0x80A00240` callback), the binder `0x80B0FF50`
  (`g3d/met_toge.brres`, `met_toge` and `met_toge_shell`), the matrix rebuild
  `0x80B0F4A0` with the `+0x464` offset vector, the run-speed seed
  `0x80B0F760` (three-entry `0x80B31F98` table, side flip at `+0x348`, six
  speed words), the destructor `0x80B0FA30`, the detach helper and the two
  forwarders.
- **`0x80B0A8D0..0x80B0C0D0`, the bucket tail helpers.** A deleting
  destructor (`0x80B0A8D0`), the `0x4330` conversion that seeds the
  `+0x99C`/`+0x9A0`/`+0x9A4` timers from the word at `+0x39C`
  (`0x80B0ABF0`), the recursive class-`0x99` search (`0x80B0AC50`), the
  `0x528`-byte constructor `0x80B0B810`, the destructor `0x80B0B020` that
  resets four embedded holders and releases six more, the `+0x9E0` overlap
  timer (`0x80B0C890`) and two forwarders.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B0A8D0` | `fn_80b0a8d0` | 56 | medium |
| `0x80B0ABF0` | `fn_80b0abf0` | 88 | medium |
| `0x80B0AC50` | `fn_80b0ac50` | 104 | high |
| `0x80B0B020` | `fn_80b0b020` | 372 | high |
| `0x80B0B7B0` | `fn_80b0b7b0` | 48 | medium |
| `0x80B0B7E0` | `fn_80b0b7e0` | 48 | medium |
| `0x80B0B810` | `fn_80b0b810` | 76 | high |
| `0x80B0C890` | `fn_80b0c890` | 100 | high |
| `0x80B0E310` | `fn_80b0e310` | 88 | high |
| `0x80B0E430` | `fn_80b0e430` | 48 | medium |
| `0x80B0E460` | `fn_80b0e460` | 48 | medium |
| `0x80B0E490` | `fn_80b0e490` | 132 | high |
| `0x80B0E5E0` | `fn_80b0e5e0` | 228 | high |
| `0x80B0E780` | `fn_80b0e780` | 308 | high |
| `0x80B0E8C0` | `fn_80b0e8c0` | 152 | high |
| `0x80B0E970` | `fn_80b0e970` | 148 | high |
| `0x80B0EA10` | `fn_80b0ea10` | 140 | high |
| `0x80B0EAB0` | `fn_80b0eab0` | 104 | high |
| `0x80B0ED70` | `fn_80b0ed70` | 88 | high |
| `0x80B0EE90` | `fn_80b0ee90` | 48 | medium |
| `0x80B0EEC0` | `fn_80b0eec0` | 48 | medium |
| `0x80B0EEF0` | `fn_80b0eef0` | 212 | high |
| `0x80B0F4A0` | `fn_80b0f4a0` | 244 | high |
| `0x80B0F760` | `fn_80b0f760` | 144 | high |
| `0x80B0FA30` | `fn_80b0fa30` | 164 | high |
| `0x80B0FBF0` | `fn_80b0fbf0` | 88 | high |
| `0x80B0FD10` | `fn_80b0fd10` | 48 | medium |
| `0x80B0FD40` | `fn_80b0fd40` | 48 | medium |
| `0x80B0FD70` | `fn_80b0fd70` | 184 | high |
| `0x80B0FF50` | `fn_80b0ff50` | 280 | high |

## Deliberate skips

The bucket had 63 untranslated addresses before this pass and 33 after it.
The neighbours this pass examined and rejected are:

- **`0x80B05740` (328 bytes), `0x80B05A80` (180 bytes), `0x80B05B80` (172
  bytes).** All three are bracketed by the mutex pair `0x802DCD74` /
  `0x802DCDC0`, printed with no operands at both ends; `0x80B05740` also folds
  an unsigned conversion into the same block. The locked object and the unlock
  argument cannot be recovered, exactly like the mutex skips of waves 113 and
  114.
- **`0x80B0CD80` (292 bytes), `0x80B0CEB0` (288 bytes).** The same problem with
  the other mutex pair `0x802DCD70` / `0x802DCDBC`: the export returns the
  object and the selector as an untyped 64-bit pair and prints the unlock with
  no argument.
- **`0x80B0D380` (328 bytes), `0x80B0D4D0` (520 bytes), `0x80B0D6E0` (696
  bytes).** Every one is wrapped in a GQR conversion block (`unaff_GQR0`,
  `ldexpf`) whose quantisation feeds the values written back to the actor, the
  same reason five addresses were skipped in
  `docs/COVERAGE_WAVE113_ENEMIES.md` and `docs/COVERAGE_WAVE114_ENEMIES.md`.
- **`0x80B0D2A0` (212 bytes), `0x80B0D9A0` (332 bytes), `0x80B0DAF0` (76
  bytes), `0x80B0DB50` (64 bytes), `0x80B0DB90` (156 bytes), `0x80B0DC40`
  (320 bytes), `0x80B0DD80` (124 bytes), `0x80B0DE10` (180 bytes).** All eight
  are the consumers of the two blocked groups above: they call the GQR or
  mutex helpers, and several call sites print the callee without its declared
  actor argument (`FUN_80b0d4d0()`, `FUN_80b0cd80(param_1, ...)` into a
  zero-argument body). Translating them would require inventing the missing
  register.
- **`0x80B0AF60` (76 bytes).** Calls `FUN_80b0ac50` with a second argument that
  the callee's own body does not declare, so the register cannot be
  reconstructed. The call site in the export is exact; the argument is not.
- **Not attempted in this pass.** `0x80B04A90` (512), `0x80B04E90` (1,592),
  `0x80B058A0` (476), `0x80B07C90` (36), `0x80B07D70` (252), `0x80B0C0D0`
  (40), `0x80B0C4C0` (120), `0x80B0C540` (848), `0x80B0C900` (1,140),
  `0x80B0CFD0` (160), `0x80B0D070` (100), `0x80B0D0E0` (100), `0x80B0D150`
  (84), `0x80B0D1B0` (124) and the two big `met_toge` neighbours `0x80B0F5A0`
  (440) and `0x80B0F800` (560) are all readable, but the lane budget was spent
  on the three coherent clusters above; they are the natural start of the next
  pass.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  3,884 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave115` prints `wave115-enemies.csv: 30 rows`
  and `OK` for this lane.
- `cargo check --message-format=short` emits no error and no diagnostic naming
  `src/game/enemies.rs`.
- The module header was updated: a new cluster item 96, the cluster count
  (ninety-five to ninety-six), the function totals (2,554 to 2,584 public
  translations, 2,483 to 2,513 `fn_<addr>` bodies), the "start here" list and
  the records list.
