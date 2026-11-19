# Coverage - Wave 133 (`game::enemies`)

## How to read this file

This page records the Wave 133 `game::enemies` lane: 28 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 115. Every row of the
machine-readable mirror (`ledger/agents/wave133-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Data constants
are cited by address and read through `platform::read_u32` / `read_f32`, never
transcribed as invented values. Raw words were decoded from
`work/rels/elf/d_enemiesNP.elf` at the mapping in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 6,116 | 19 | 9 |

The lane has two halves:

- **12 bodies** are the rows of `docs/SPEC_WAVE132_ENEMIES.md`: the eleven marked
  **ready** plus the one **conditional** row (`0x80B1B3F0`). They were written
  from that page's instruction-level decode and re-checked against the Ghidra C
  export.
- **16 bodies** are still-open entries of the `0x80A10000` planner bucket. They
  fall into two coherent clusters, which is why they were picked: the
  `block_cloud` actor (`0x80A1F390..0x80A1FFB0`) and its `0x80A1E000` sibling
  (`0x80A1E170..0x80A1EF30`), plus the two-area collision removal
  `0x80A18E30`. After this pass the bucket has 18 untranslated rows left (34
  before the pass).

## What this pass covers

- **The twelve specification bodies (`0x80A0E450..0x80B1B3F0`).** The
  `lift_trampoline`/`lift_trampoline_long` variant initialiser with its
  `0x8007FA40` layer emitter, the `1/240` ramp block that rescales two extent
  tables, the player-height probe that re-arms the pad block, the `fishbone`
  per-frame step (cue `519`, aim gate, two pad slots), the `+0x360` three-state
  machine that resets the `walk` clip on a side change, the `go_out_ed` landing
  trigger with `Wm_en_landsmoke_s` and cue `539`, the fixed-point pivot step, the
  four-slot `256 x 160` proximity test, the `ice`/`ice_C1` rig binder, the
  eight-slot respawn allocator, the side-flip gate with its lazily built table
  and the `run` clip at a random frame offset.
- **The `block_cloud` cluster (7 bodies).** `0x80A1F510` (`0x7B0`-byte
  constructor), `0x80A1F950` (`g3d/block_cloud.brres` + `wait` binder),
  `0x80A1F760` (frame step), `0x80A1FA70` (impact step: `Wm_en_blockcloud`, cue
  `0x27A`, splash), `0x80A1FCE0` (impact step plus the `+0x6B8` holder slot) and
  the two release helpers `0x80A1F390`/`0x80A1FFB0`.
- **The `0x80A1E000` sibling cluster (8 bodies).** `0x80A1E7E0` (target search
  over kinds `0x1A3`/`0x1A4`), `0x80A1E880` (capture of distance, yaw and
  pitch), `0x80A1EDF0` (search + capture + follow entry), `0x80A1E170` (the
  `+0x70C` state-`10` gate), `0x80A1E200` (the `+0x67D` latch effect),
  `0x80A1E2A0` (the 716-byte joint `+0x394` handler), `0x80A1EE80` (cardinal
  angle gate) and `0x80A1EF30` (unconditional wrapper).
- **The two-area removal `0x80A18E30`.** Both collision areas of the same actor
  family are unregistered and `+0x10C` is cleared.

### Confirmations against the specification

Every field offset, constant address, store order and call argument of the
twelve specification bodies matched the raw ELF words, and the twelve sizes
re-read from `functions.tsv` equal the sizes in the specification. Six details
worth recording:

- **`0x80AB9AF0`'s float class.** The four `0x8007FA40` arguments are `lfs`
  loads (`0x80AB9BB4` `lfs f1`, `0x80AB9BBC` `lfs f2`, `0x80AB9BC4` `lfs f4`),
  so on PowerPC they enter the FPRs as doubles; the body passes them through the
  existing `call12_f64x4` adapter, which is the ABI-faithful shape.
- **`0x80A36FC0`'s stack blocks.** The raw words put the effect position at
  `r1+0x18` (`x`, `y`, `5500.0`) and the scale triplet at `r1+0x24`; the export's
  `local_18..local_2C` names are offset-shifted, so the specification's layout
  was used.
- **`0x80A2E970`'s `2^52` conversion.** `CONCAT44(0x43300000, rand)` minus the
  in-REL bias is the *unsigned* conversion, i.e. `(f32)rand` for `rand` in
  `0..7`; the body keeps that reading.
- **`0x80B1B3F0`'s table.** `0x80B32488` really is
  `s16[8] {12743, -12743, 13653, -13653, 120, 180, 13653, 0}` in the image, and
  the product is `mulli`-scaled by `13653` before the `extsh` narrowing.
- **`0x80A35370`'s divide.** The quotient of the signed divide-by-ten (not the
  raw frame) multiplies the random count; Rust's `/` truncates toward zero,
  which is the compiler's sign-fixed sequence.
- **`0x80A368B0`'s NaN arm.** The two rejection branches are `fcmpo` + `bge`,
  i.e. "not less than", so an unordered distance also advances to the next slot.
  The body spells this `!(distance < threshold)` rather than the export's
  `threshold <= distance`, which differs for NaN.

### Confirmations against the bucket exports

- **`0x80A1F510`'s twice-written `+0x60`.** `0x80301EA8` is stored first and
  replaced by the in-REL class block `0x80B38C44` before the next constructor;
  both stores are kept because the intermediate value is observable.
- **`0x80A1FA70`'s splash position.** The `+0xB4` word is stored into the record
  first, then overwritten by the single-precision return of `0x80068BC0`, and
  only then is `y` raised by `8.0`. `0x80A1FBE0` confirms the order (`stfs` to
  `+0x18`, `fadds`, `stfs` to `+0x14`).
- **`0x80A1E880`'s dead `z` load.** The target's `+0xB4` word is copied into the
  stack vector but never read back; only `x`/`y` reach the distance, yaw and raw
  offset arms.
- **`0x80A1E2A0`'s 20-byte record.** `{x, y, z, byte +0x694, word +0x70C, 0, 0,
  0}` is passed as five words with the byte and the state word packed
  big-endian.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AB9AF0` | `fn_80ab9af0` | 284 | high |
| `0x80A7EB60` | `fn_80a7eb60` | 280 | high |
| `0x80A0E450` | `fn_80a0e450` | 276 | high |
| `0x80AB22D0` | `fn_80ab22d0` | 276 | medium |
| `0x80A37700` | `fn_80a37700` | 256 | high |
| `0x80A36FC0` | `fn_80a36fc0` | 248 | high |
| `0x80B1B3F0` | `fn_80b1b3f0` | 248 | medium |
| `0x80A368B0` | `fn_80a368b0` | 244 | high |
| `0x80AFFAF0` | `fn_80affaf0` | 244 | high |
| `0x80A2E970` | `fn_80a2e970` | 240 | high |
| `0x80A600B0` | `fn_80a600b0` | 240 | medium |
| `0x80A35370` | `fn_80a35370` | 232 | high |
| `0x80A1E2A0` | `fn_80a1e2a0` | 716 | medium |
| `0x80A1FA70` | `fn_80a1fa70` | 412 | medium |
| `0x80A1F950` | `fn_80a1f950` | 276 | high |
| `0x80A1E880` | `fn_80a1e880` | 272 | medium |
| `0x80A1F760` | `fn_80a1f760` | 168 | high |
| `0x80A1E200` | `fn_80a1e200` | 156 | medium |
| `0x80A1E7E0` | `fn_80a1e7e0` | 156 | high |
| `0x80A1F510` | `fn_80a1f510` | 152 | high |
| `0x80A1EE80` | `fn_80a1ee80` | 144 | high |
| `0x80A1E170` | `fn_80a1e170` | 136 | high |
| `0x80A1EDF0` | `fn_80a1edf0` | 100 | medium |
| `0x80A1F390` | `fn_80a1f390` | 88 | high |
| `0x80A1FFB0` | `fn_80a1ffb0` | 88 | high |
| `0x80A18E30` | `fn_80a18e30` | 68 | high |
| `0x80A1FCE0` | `fn_80a1fce0` | 64 | medium |
| `0x80A1EF30` | `fn_80a1ef30` | 52 | high |

### Why nine rows are `medium`

- **`0x80AB22D0`, `0x80A1EDF0`, `0x80A1FCE0`.** Ghidra prints the in-REL
  callees as zero-argument calls; the receiver is the still-live `r3`, which is
  provable from the instruction stream but not from the export signature. The
  roles of the aim block, the `0x80A1E990` placement step and the `+0x6B8`
  holder slot are inferred.
- **`0x80B1B3F0`.** The one conditional row: the byte at `0x8042C1A0 + side`
  lives in `main.dol`, is sign-extended and multiplied by `13653`. The portable
  body reads the shipped address at run time and reproduces the arithmetic, but
  a byte-exact comparison still needs that table.
- **`0x80A600B0`, `0x80A1E200`, `0x80A1E2A0`, `0x80A1E880`, `0x80A1FA70`.**
  Every branch, store, string, cue and token is literal; the field roles
  (lateral speed, latch, mode bytes, capture offsets, splash nibbles) and the
  `0x802BE470`/`0x80237ED0`/`0x807073D0` helper roles are inferred from the
  values compared and written.

## Deliberate skips

- **`0x80A12110`, `0x80A18730`, `0x80A18750`, `0x80A18770`, `0x80A1B2E0`.**
  Their exports end in `halt_baddata()` after a partial instruction, so no
  reachable body exists without inventing instructions.
- **`0x80A1D530`, `0x80A1E990`, `0x80A1EBC0`.** All three consume a
  `GQR1..GQR7` quantiser (`unaff_GQR3`, type codes 4/5/6/7). They stay class C
  under the wave-127 rules until the `main.dol` GQR words are pinned. Its single
  caller in this lane (`0x80A1EDF0`) cites it by address only.
- **`0x80A1E100`.** The export widens two float fields to `double` for
  `0x800146B0`/`0x800146F0` and reorders the sign (`-(16.0 + y)`), so the
  register class of the second argument is not established from the export
  alone; nothing was claimed.
- **The remaining `0x80A10000` rows** (18 still open, including the 940-byte
  `0x80A10880`, the 636-byte `0x80A1C8F0` and the 484-byte `0x80A10220`) are
  readable and were left for a later pass once the body target was met. Nothing
  was written for them, so no ledger row claims them.
- **The four `0x802DCBFC` forwarders of the bucket** (`0x80A197F0`,
  `0x80A19820`, `0x80A1DA70`, `0x80A1DAA0`) were not taken: the same three
  shapes are already carried by wave 132 (`0x80A03270` group) and adding four
  more identical `partial` rows would pad the lane without new evidence.
- **No address already in `ledger/functions.csv` was touched**, and no other
  lane's file, CSV or doc was modified.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  6,116 bytes (3,068 for the twelve specification rows and 3,048 for the sixteen
  bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was in
  `ledger/functions.csv` before this pass.
- The module header was updated: cluster item 115, the cluster count (114 to
  115), the function totals (3,068 to 3,096 public translations, 2,997 to 3,025
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs` (the two errors reported during this pass are in
  `src/game/bases.rs`, the parallel lane).
- `node tools/check-lane-csv.mjs wave133` reports `wave133-enemies.csv: 28 rows`,
  28 unique addresses and `OK`.
