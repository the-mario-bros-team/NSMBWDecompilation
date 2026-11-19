# Coverage - Wave 118 (`game::enemies`, the `0x80A90000` bucket)

## How to read this file

This page records the Wave 118 `game::enemies` lane: 30 functions of the
`0x80A90000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 99 of that file. Every row of the machine
readable mirror (`ledger/agents/wave118-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 6,360 | 11 | 19 |

The bucket holds 183 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 122 were already in
`ledger/functions.csv`, leaving 61. This pass claims 30 and leaves 31.

## What this pass covers

Five runs chosen for shared field layouts and for the class blocks that the
earlier waves had already half translated.

- **Run 1 (`0x80A91BB0..0x80A91F40`, four bodies).** The world-map cloud actor.
  `0x80A91D30` (left untranslated) raises the literal effect name
  `Wm_en_movecloud` (`0x80B508F8`), and the two already-translated neighbours
  `0x80A92050`/`0x80A920D0` reset the actor and bind `g3d/jugem.brres` plus the
  `cloud_wait` clip. This run adds the `+0x7C` matrix rebuild with its
  `+0x6EC..+0x6F4` node-0 probe, the stage-height getter `+0x38`/`+0x8FEA8`,
  the per-side contact store behind the `0x80B74038` state test, and the
  four-player state poll that falls back to `0x80A92050`.
- **Run 2 (`0x80A9D690..0x80A9D930` and `0x80A9DDB0..0x80A9E2F0`, nine
  bodies).** The descriptor helpers and the `kanibo`/`kanibo_rock` actor. The
  two constant producers return the `main.dol` records `0x80357BF4` and
  `0x80357BB4`; the chain walker and the `:`-suffix predicate repeat the
  wave-31 bodies at `0x80A9A600`/`0x80A9A6E0`; the last thunk is the usual
  `0x802DCBFC` forwarder. The actor block binds `g3d/kanibo.brres`
  (`0x80B5251C`), the `kanibo_rock` model (`0x80B52538`) and its clip, rebuilds
  the matrix from `+0x100..+0x104`, raises the three `Wm_ob_cmn*` break effects
  with sound `0x2C7` and retires through `0x80162510`.
- **Run 3 (`0x80A98950` and `0x80A98C10`, two bodies).** The hit handler and
  the model binder of the `pukupuku` actor whose `0x608`-byte constructor
  landed in wave 31 (`0x80A98250`). Both take their receiver from the
  `0x802DCD74` register-save prologue; the binder loads
  `g3d/pukupuku.brres`, binds the model with id `0x23`, attaches the `+0x584`
  clip with the `1.0` blend and wires the `+0x5C0` part animator.
- **Run 4 (`0x80A9B070..0x80A9B310`, four bodies).** The
  `killer_rot_houdai` slot binder, slot writer, release forwarder and release
  sweep that wave 31 deferred because of the hidden-`r12` `0x802DCBxx`
  helpers. The slot tables are `+0x544` (`0x40` bytes), `+0x744` (`0x38`),
  `+0xB04` (`0xC`) and the per-slot bytes at `+0xC58`/`+0xC80`/`+0xC81`/
  `+0xC88`.
- **Run 5 (`0x80A9EC50..0x80A9FF20`, eleven bodies).** The `daEnKanon_c`
  matrix, state-gate, contact and hit-box helpers. The model strings are
  `karon` (`0x80B528B0`) and `g3d/karon.brres` (`0x80B528A0`), the six-part
  name table is `0x80B52888`, and the shared hit effect is `Wm_mr_softhit`
  (`0x80B516E4`). The run adds the `+0x54C` matrix with the `+/-0x834` sway and
  the nine cached node words, the six `karon_parts` matrices, the
  `0x80B74708` damage gate, the `+0x3F4` contact callback with its `Turn` to
  `Walk` transition, the second contact handler with sounds `0x1D5`/`0x1D2`,
  the full model binder, the clip selector (`+0x84C`), two hit-box tests and
  the contact-flag frame tail.

### Why nineteen rows are `medium`

- **A receiver comes from a register-save prologue.** `0x802DCD64`, `6C`, `70`
  and `74` are Metrowerks `__savegprlr_*` helpers, not logic; they preserve
  `r3`. The export prints them as returning calls, so
  `0x80A91F40`, `0x80A98C10`, `0x80A9B070`, `0x80A9B310` and `0x80A9EE00`
  read the actor back with `call0_ret_u32`, exactly as the accepted `0x80A7A700`
  treatment does.
- **A hidden-`r12` stub is involved.** `0x802DCBFC` (and its
  `0x802DCBC0`/`0x802DCB90` queries) adjust `r12`; only the visible arguments
  can be modelled, so `0x80A9D930`, `0x80A9B1B0`, `0x80A9B280` and the two
  release paths are marked `medium`.
- **A callee is truncated or unnamed.** `0x80A91D10` is `halt_baddata` in the
  export; `0x80A91DA0` calls it by address. `0x8004A6A0`, `0x800964A0`,
  `0x80064A30`, `0x80064DA0` and `0x800954D0` have no established name, so
  their roles are inferred from the stores they guard.
- **An `f64` argument pair is widened from `f32`.** `0x8042CC88`/`0x8042CC8C`
  are read as the single-precision words earlier waves documented; the
  `0x800E20A0` call receives them through `call6_f64x2`.
- **The `2^52` idiom.** `0x80A9FF20` builds its probe offset with
  `CONCAT44(0x43300000, byte ^ 0x80000000) - 0x80B2F338`; that is the
  Metrowerks signed-byte-to-double conversion, kept as
  `read_u8(...) as i8 as f32` with an `// observed:` note.
- **A stack argument block has no printed width.** The binders pass adjacent
  stack slots to `0x801650D0`/`0x80166830`; the translation passes one-word
  arrays and marks the block width `// unknown:` in the doc.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A91BB0` | `fn_80a91bb0` | 352 | medium |
| `0x80A91D70` | `fn_80a91d70` | 36 | high |
| `0x80A91DA0` | `fn_80a91da0` | 252 | medium |
| `0x80A91F40` | `fn_80a91f40` | 260 | medium |
| `0x80A9D690` | `fn_80a9d690` | 12 | high |
| `0x80A9D6A0` | `fn_80a9d6a0` | 12 | high |
| `0x80A9D790` | `fn_80a9d790` | 224 | high |
| `0x80A9D870` | `fn_80a9d870` | 136 | medium |
| `0x80A9D930` | `fn_80a9d930` | 48 | medium |
| `0x80A9DDB0` | `fn_80a9ddb0` | 220 | high |
| `0x80A9DE90` | `fn_80a9de90` | 172 | high |
| `0x80A9DF40` | `fn_80a9df40` | 228 | high |
| `0x80A9E090` | `fn_80a9e090` | 64 | medium |
| `0x80A9E2F0` | `fn_80a9e2f0` | 36 | medium |
| `0x80A98950` | `fn_80a98950` | 264 | medium |
| `0x80A98C10` | `fn_80a98c10` | 424 | medium |
| `0x80A9B070` | `fn_80a9b070` | 312 | medium |
| `0x80A9B1B0` | `fn_80a9b1b0` | 208 | medium |
| `0x80A9B280` | `fn_80a9b280` | 48 | medium |
| `0x80A9B310` | `fn_80a9b310` | 164 | medium |
| `0x80A9EC50` | `fn_80a9ec50` | 428 | high |
| `0x80A9EE00` | `fn_80a9ee00` | 276 | medium |
| `0x80A9EFE0` | `fn_80a9efe0` | 160 | medium |
| `0x80A9F080` | `fn_80a9f080` | 388 | medium |
| `0x80A9F460` | `fn_80a9f460` | 456 | medium |
| `0x80A9FAF0` | `fn_80a9faf0` | 380 | high |
| `0x80A9FD50` | `fn_80a9fd50` | 140 | high |
| `0x80A9FDE0` | `fn_80a9fde0` | 136 | medium |
| `0x80A9FE70` | `fn_80a9fe70` | 176 | high |
| `0x80A9FF20` | `fn_80a9ff20` | 348 | medium |

## Deliberate skips

- **`0x80A91D10` (32 bytes).** The export is `halt_baddata()` only; the body is
  called by address from [`fn_80a91da0`] and marked `// unknown:`.
- **`0x80A91D30` (56 bytes).** The guard is readable but Ghidra lost the jump
  table ("Treating indirect jump as call"); the `Wm_en_movecloud` effect call is
  the only surviving statement, so the function cannot be reconstructed
  honestly.
- **`0x80A99360` (228 bytes).** The export is dominated by `GQR`/`ldexpf`
  paired-single artefacts; the instruction stream cannot be separated from the
  decompiler noise.
- **`0x80A9E0D0` (24 bytes) and `0x80A9F750` (28 bytes).** Both end in
  `halt_baddata` after two or three readable stores.
- **`0x80A9B3C0` (1,196 bytes).** The `killer_rot_houdai` state step combines a
  `GQR0` block, a 32-word stack-table copy and a `countLeadingZeros` branch;
  the wave-31 deferral stands.
- **`0x80A92F40` (1,172 bytes), `0x80A969C0` and `0x80A96DC0`.** The first is a
  long cloud-actor frame step whose prologue receiver and player loop would need
  a full read; `0x80A969C0` is `GQR`-dominated and `0x80A96DC0` is a 32-word
  table copy. All three were left for a pass with a larger budget.
- **Not attempted in this pass.** The remaining 31 addresses of the bucket:
  four `0x802DCBFC` forwarders (`0x80A90AE0`, `0x80A90B10`, `0x80A915E0`,
  `0x80A91610`), their `0x80A95D00`/`0x80A95D30` and `0x80A981F0`/`0x80A98220`
  siblings, the `0x80A9A7A0`/`0x80A9A7D0` pair, `0x80A9B3C0`, and the larger
  `0x80A96230` (572 bytes) and `0x80A9D0F0` (1,244 bytes) blocks.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  6,360 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave118` prints `wave118-enemies.csv: 30 rows`
  and `OK`.
- `cargo check --message-format=short` emits no error and no diagnostic naming
  `src/game/enemies.rs`.
- The module header was updated: a new cluster item 99, the cluster count
  (ninety-eight to ninety-nine), the function totals (2,640 to 2,670 public
  translations, 2,569 to 2,599 `fn_<addr>` bodies), the "start here" list and
  the records list.
