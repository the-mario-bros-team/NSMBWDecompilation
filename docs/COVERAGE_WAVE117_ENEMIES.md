# Coverage - Wave 117 (`game::enemies`, the `0x80A30000` bucket)

## How to read this file

This page records the Wave 117 `game::enemies` lane: 28 functions of the
`0x80A30000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 98 of that file. Every row of the machine
readable mirror (`ledger/agents/wave117-enemies.csv`) is one real
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
| `src/game/enemies.rs` | `game::enemies` | 28 | 4,740 | 18 | 10 |

The bucket holds 149 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 87 were already in
`ledger/functions.csv`, leaving 62. This pass claims 28 and leaves 34.

## What this pass covers

Three short runs chosen for shared field layouts.

- **Run 1 (`0x80A36A70..0x80A38050`, eight bodies).** The
  `Wm_en_brakesmoke` world-map actor. Two literal strings identify it:
  `Wm_en_brakesmoke` (`0x80B3DE28`) is handed as the effect name to slot
  `+0xA0` of the object at `+0x70C`, and `go_out_st` (`0x80B3DDE0`) is the clip
  bound to the `+0x584` holder. The run contributes the side-change step (saved
  byte `+0x86C`, probe `0x80096240`, target slot `+0x834`, state
  `0x80B70558`), the `+0x04` bit-1 gate over the proximity probe `0x80A368B0`,
  the two state entries (`+0x5C0` timer `0x1E`, `+0x360` flag, sound `0x219`
  and state `0x80B70458`), the `go_out_st` clip entry, the release step, the
  speed reset and the `+0x834`/`+0x838` target-position getter. No class name is
  printed for the actor itself, so its fields are described by offset.
- **Run 2 (`0x80A3CA80` and `0x80A3DBE0`, two bodies).** The two array-container
  destructors of the bucket. Each unlinks six fixed-capacity arrays through
  `0x802DCA98` with element release thunks `0x80A00200` (`0xC`-byte), `0x80A22AE0`
  (`4`-byte) and `0x80A00240` (`8`-byte), releases the `+0x544` object, resets
  the model holder `+0x524` and runs the base teardown `0x80095130`; the block
  itself is freed only for a positive flag.
- **Run 3 (`0x80A3F280..0x80A3FFF0`, eighteen bodies).** The local helpers of
  the `0xA95`-state enemy actor whose frame steps were translated in wave 77
  (`docs/COVERAGE_WAVE77_ENEMIES.md`, `0x80A401D0..0x80A40740`). The mode word
  is `+0xA7C`, the parent handle `+0x6BC` (resolved through `0x80162D00`), the
  trample counter `+0x740`, the re-arm countdown `+0xA88` and the
  retire/active latches `+0xA9A`/`+0xA97`/`+0xA90` are shared with that pass.
  This run adds the mode-gated launch step, the contact callback, the
  re-arm guard `0x80A3FAC0` and its callers, the parent-follow placements, the
  two random-speed steps, the stage-landing step, the `0x71C` yaw flip, the
  `+0x740` bounce ladder and the `0x8007FBF0` mode-`0x10` re-entry.

### Why ten rows are `medium`

- **A callee is truncated or unnamed.** `0x80A40FA0` (contact effect) is cut by
  `halt_baddata` in the export; the call is made by address and its role is
  inferred. The same applies to `0x80101AA0` (`0x80A3F7A0`), `0x80027E00`
  (`0x80A3F280`) and `0x800777B0` (`0x80A3F450`).
- **A table entry's meaning is not established.** `0x80A3FD70` reads the four
  tables `0x80B2C588`, `0x80B2C5A8`, `0x80B2C5B8` and the `+0xA78` mask;
  `0x80A3FFF0` reads the `0x802EE4E0` ladder and the `0x80B2C5E8` classes. The
  reads, masks and shifts are literal, the semantic name of each entry is not.
- **A virtual-slot role is inferred.** The `+0xA0` slot of `0x80A38050` and the
  `+0x1C` slot of `0x80A36F30` are called with literal arguments, but the
  export attaches no name to either slot; `0x80A36F30` is still `high` because
  the call itself is fully explicit.
- **The stage object is folded into `_DAT_80429DD0`.** `0x80A3FB80` and
  `0x80A3F450` read offsets `+0x9002C` and `+0x8FEC4` off that global; the
  model follows [`fn_80a98ea0`]'s accepted treatment of the same global.
- **A hidden receiver.** `0x80A36B10` calls `0x80A368B0` with no printed
  argument; the actor is live in `r3` and is passed explicitly, marked
  `// inferred:` in the body.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A36A70` | `fn_80a36a70` | 116 | high |
| `0x80A36B10` | `fn_80a36b10` | 120 | medium |
| `0x80A36B90` | `fn_80a36b90` | 108 | high |
| `0x80A36E70` | `fn_80a36e70` | 176 | high |
| `0x80A36F30` | `fn_80a36f30` | 132 | high |
| `0x80A37690` | `fn_80a37690` | 84 | high |
| `0x80A37DE0` | `fn_80a37de0` | 108 | high |
| `0x80A38050` | `fn_80a38050` | 108 | medium |
| `0x80A3CA80` | `fn_80a3ca80` | 288 | high |
| `0x80A3DBE0` | `fn_80a3dbe0` | 284 | high |
| `0x80A3F280` | `fn_80a3f280` | 100 | medium |
| `0x80A3F360` | `fn_80a3f360` | 228 | medium |
| `0x80A3F450` | `fn_80a3f450` | 224 | medium |
| `0x80A3F530` | `fn_80a3f530` | 164 | medium |
| `0x80A3F660` | `fn_80a3f660` | 108 | high |
| `0x80A3F6F0` | `fn_80a3f6f0` | 176 | high |
| `0x80A3F7A0` | `fn_80a3f7a0` | 152 | medium |
| `0x80A3F840` | `fn_80a3f840` | 164 | high |
| `0x80A3F9A0` | `fn_80a3f9a0` | 132 | high |
| `0x80A3FA30` | `fn_80a3fa30` | 132 | high |
| `0x80A3FAC0` | `fn_80a3fac0` | 100 | high |
| `0x80A3FB80` | `fn_80a3fb80` | 152 | medium |
| `0x80A3FC20` | `fn_80a3fc20` | 144 | high |
| `0x80A3FCC0` | `fn_80a3fcc0` | 168 | high |
| `0x80A3FD70` | `fn_80a3fd70` | 320 | medium |
| `0x80A3FEC0` | `fn_80a3fec0` | 100 | high |
| `0x80A3FF30` | `fn_80a3ff30` | 172 | high |
| `0x80A3FFF0` | `fn_80a3fff0` | 480 | medium |

## Deliberate skips

- **`0x80A3F630` (32 bytes, `0x80A3F6D0` 16 bytes).** Both exports print a
  readable guard (`+0xA7C` against `0`, `0xF` and `0xD`; `+0xA90` against `0`)
  that returns early, and then `halt_baddata()` with Ghidra's "Control flow
  encountered bad instruction data" warning. The only honest translation would
  be the guard plus a stop, which changes observable behaviour; they are left
  for the lane that can recover the truncated instructions.
- **`0x80A3F8F0` (176 bytes).** The speed term is read from `fRam00000398`, an
  unresolved base that Ghidra could neither attribute to a register nor to a
  global; only the fallback path of the `0x80162D50(0x79, 0)` lookup is
  affected, so the body cannot be reconstructed without inventing the value.
- **Not attempted in this pass.** `0x80A334D0` (352), `0x80A33630` (380),
  `0x80A337B0` (4), `0x80A337C0` (32), `0x80A34F80` (348), `0x80A35170` (464),
  `0x80A35370` (232), `0x80A35570` (28), `0x80A35ED0` (228), `0x80A360D0` (228),
  `0x80A365B0` (192), `0x80A36670` (132), `0x80A368B0` (244, GQR block),
  `0x80A369B0` (152), `0x80A36A50` (32, truncated), `0x80A36C10` (608),
  `0x80A36FC0` (248), `0x80A370D0` (292), `0x80A372A0` (448), `0x80A37460` (228),
  `0x80A37560` (296), `0x80A37700` (256), `0x80A37830` (1,148),
  `0x80A37CB0` (304, GQR block), `0x80A390F0` (300), `0x80A39410` (324),
  `0x80A39650` (312), `0x80A397B0` (228), `0x80A398C0` (132),
  `0x80A3A4A0` (2,620), `0x80A3DEA0` (2,940) and the remaining neighbours of the
  three runs above are readable but the lane budget was spent on those runs;
  they are the natural start of the next pass.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  4,740 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave117` prints
  `wave117-enemies.csv: 28 rows` and `OK`.
- `cargo check --message-format=short` emits no error and no diagnostic naming
  `src/game/enemies.rs`.
- The module header was updated: a new cluster item 98, the cluster count
  (ninety-seven to ninety-eight), the function totals (2,612 to 2,640 public
  translations, 2,541 to 2,569 `fn_<addr>` bodies), the "start here" list and
  the records list.
