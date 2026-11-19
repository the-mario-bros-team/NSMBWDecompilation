# Coverage - Wave 89 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. This pass adds 29 bodies from the `0x800D0000` bucket, the
same actor/effect band that waves 63 and 76 cover. Each body is a literal
reconstruction of one Ghidra export; the doc block above the Rust function names
the original address, the byte count taken from
`work/ghidra/main-gb/export/functions.tsv` and the evidence file. "high"
confidence means every store, branch and call is visible in the export;
"medium" means the export dropped a virtual receiver or float operand, an SDA
slot's owner is unrecovered, or a field role is inferred from a sibling body.
Every field whose role is not proven is marked `// unknown:` or `// inferred:`
in the Rust comment.

Only `src/engine/display.rs` was edited; `ledger/functions.csv` is read-only and
the coordinator merges this lane's rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 29 | 2452 | 10 | 19 |

## What this pass covers

The `0x800D0000` bucket entered this pass with 100 untranslated of its 251
functions. This pass takes five coherent slices of it and leaves the
paired-single/GQR bodies and the large state drivers for a later pass.

- **The record/unlink cluster** (`0x800D3A30..0x800D4C70`, 5 bodies).
  [`fn_800d3a30`] unlinks the current animation record and the variant record
  selected by `+0x770`, [`fn_800d3c80`] clears the cached record pointer,
  [`fn_800d4670`] switches the variant byte at `+0x152` and rebuilds the
  animation through `0x800D4CD0`/`0x800D3C80`, and [`fn_800d4c10`] /
  [`fn_800d4c70`] are the mode-3/mode-2 seeds over `0x800D4AD0`.
- **The embedded-node release shims** (`0x800D5730..0x800D5C90`, 4 bodies).
  [`fn_800d5730`] starts an animation on the node returned by slot `+0x28`;
  [`fn_800d5c80`] releases the node at `+0x60`; [`fn_800d5c10`] and
  [`fn_800d5c90`] release the node at `+0x28` through `0x80168FE0` /
  `0x801680E0` and, when `0x800D5BE0` reports idle, dispatch the object's
  `0x88`/`0x90` slot with a 64-bit operand.
- **The wrapper dispatch trio** (`0x800D6DF0..0x800D7020`, 4 bodies).
  [`fn_800d6df0`] forwards the wrapper's `+0x04` actor to `0x800D5580`;
  [`fn_800d6e90`], [`fn_800d7000`] and [`fn_800d7020`] run the actor's
  `+0x18`/`+0x20`/`+0x24` slots, the last one behind the `+0x08` flag.
- **The motion controller** (`0x800D7780..0x800D7FC0`, 6 bodies).
  [`fn_800d7780`] reduces the shared tick counter `r13 - 0x5480` into the
  motion's step range, [`fn_800d7820`] invalidates slot `+0x10` when the tick
  changes and then clamps through `0x800D7480`, [`fn_800d79b0`] rewinds to the
  first step and enters the `0x80371720` state, [`fn_800d7c20`] /
  [`fn_800d7c50`] are the two `0x802DCBFC` forwarding thunks, and
  [`fn_800d7fc0`] is the release/optional-free wrapper.
- **The course cursor** (`0x800D9290..0x800D96D0`, 4 bodies).
  [`fn_800d9290`] advances or reverses the frame counter with the `+0x26` wrap
  bit, [`fn_800d9370`] converts the selected frame record into a position and a
  heading, [`fn_800d9560`] integrates the velocity and starts the next frame,
  and [`fn_800d96d0`] is the squared-radius proximity test.
- **The interface shims and the packet copy** (`0x800DC800..0x800DE290`,
  6 bodies). [`fn_800dc800`] and [`fn_800dced0`] are empty vtable hooks,
  [`fn_800dc970`] runs the `+0x5C` interface slot, [`fn_800dcee0`] and
  [`fn_800dcf00`] run the embedded `+0x0C` interface slots and
  [`fn_800de290`] copies one `0x40`-byte record off the `+0x5C` stream cursor.

Two facts carry across the whole slice. First, every object here starts with a
vtable word at `+0x00`, so the receivers of the dropped virtual calls can be
recovered from the double indirection. Second, the motion fields `+0x48`,
`+0x58`, `+0x64`, `+0x68`, `+0x6C` and `+0x70` are written by `0x800D7350`,
which is still untranslated; their roles are labelled `// inferred:` from that
body's stores rather than presented as fact.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800D3A30` | `fn_800d3a30` | 156 | medium |
| `0x800D3C80` | `fn_800d3c80` | 40 | high |
| `0x800D4670` | `fn_800d4670` | 260 | medium |
| `0x800D4C10` | `fn_800d4c10` | 84 | medium |
| `0x800D4C70` | `fn_800d4c70` | 92 | high |
| `0x800D5730` | `fn_800d5730` | 80 | medium |
| `0x800D5C10` | `fn_800d5c10` | 100 | medium |
| `0x800D5C80` | `fn_800d5c80` | 8 | high |
| `0x800D5C90` | `fn_800d5c90` | 100 | medium |
| `0x800D6DF0` | `fn_800d6df0` | 8 | high |
| `0x800D6E90` | `fn_800d6e90` | 20 | medium |
| `0x800D7000` | `fn_800d7000` | 20 | medium |
| `0x800D7020` | `fn_800d7020` | 32 | medium |
| `0x800D7780` | `fn_800d7780` | 68 | medium |
| `0x800D7820` | `fn_800d7820` | 92 | medium |
| `0x800D79B0` | `fn_800d79b0` | 40 | medium |
| `0x800D7C20` | `fn_800d7c20` | 48 | medium |
| `0x800D7C50` | `fn_800d7c50` | 48 | medium |
| `0x800D7FC0` | `fn_800d7fc0` | 88 | high |
| `0x800D9290` | `fn_800d9290` | 216 | high |
| `0x800D9370` | `fn_800d9370` | 168 | medium |
| `0x800D9560` | `fn_800d9560` | 360 | medium |
| `0x800D96D0` | `fn_800d96d0` | 68 | high |
| `0x800DC800` | `fn_800dc800` | 4 | high |
| `0x800DC970` | `fn_800dc970` | 16 | medium |
| `0x800DCED0` | `fn_800dced0` | 4 | high |
| `0x800DCEE0` | `fn_800dcee0` | 24 | medium |
| `0x800DCF00` | `fn_800dcf00` | 16 | medium |
| `0x800DE290` | `fn_800de290` | 192 | high |

Byte total: `632` (record/unlink cluster) + `288` (node and animation shims) +
`80` (wrapper dispatch trio) + `384` (motion controller) + `812` (course
cursor) + `256` (interface shims and packet copy) = `2452`.

## Deliberate skips

71 addresses of the bucket stay untranslated. They fall into four groups, all
skipped for a concrete, stated reason rather than for lack of budget:

- **Paired-single / GQR bodies.** `0x800D2CF0`, `0x800D6460`, `0x800D6550`,
  `0x800D9D70`, `0x800DAA10`, `0x800DAB40` and `0x800DAC40` are printed with
  `unaff_GQR0`, `ldexpf` and `CONCAT13/CONCAT22` quantisation stubs. The export
  no longer contains the paired-single semantics, so any Rust body would be an
  invented approximation.
- **`unaff_r2`-dependent movement bodies.** `0x800D19E0`, `0x800D27B0`,
  `0x800D3380`, `0x800D36E0`, `0x800D38A0`, `0x800D3CE0`, `0x800D3EF0`,
  `0x800D43E0`, `0x800D4AD0`, `0x800D4CD0`, `0x800D4D80`, `0x800D5170`,
  `0x800D5330`, `0x800D57F0` and `0x800D70E0` index the SDA2 float bank through
  `unaff_r2` without naming the slot. Those constants are recoverable only by
  cross-referencing every call site, which is a separate pass.
- **Large state drivers.** `0x800D0610` (656 bytes), `0x800D5DD0` (1,040
  bytes), `0x800D7C80` (824 bytes) and `0x800DD2B0` (1,212 bytes) are multi-table
  state machines whose f64 virtual-call operands are all GQR-converted in the
  export. [`fn_800d9560`] is the largest body taken because its conversions are
  plain integer casts.
- **Remaining dispatch and preset bodies.** `0x800D0340`, `0x800D0470`,
  `0x800D65E0`, `0x800D6940`, `0x800D69B0`, `0x800D7350`, `0x800D74C0`,
  `0x800D8850`, `0x800D8B00`, `0x800D8BB0`, `0x800D8C60`, `0x800D8E40` and
  `0x800D8EA0` mix `unaff_r2` scale constants, `unaff_r13` list anchors and
  f64 vtable operands; `0x800D2460`, `0x800D2690` and `0x800D9760` carry the
  same "removing unreachable block" warnings around GQR quantisation.
  `0x800D16B0` is the `0x802DCBFC` thunk whose operands the export drops
  entirely, so only a `low`-confidence forwarding guess is possible.
  `0x800D8670` is a 4-byte thunk to `0x800D8680` and is already covered by that
  body. `0x800DDAB0`, `0x800DDAF0` and `0x800DDB50` are release variants whose
  free call prints without its receiver.
- **Uniform release/thunk banks.** `0x800DD1C0`, `0x800DD1F0`, `0x800DD220`,
  `0x800DD250`, `0x800DD280`, `0x800DE4B0` and `0x800DE4E0` are seven identical
  `0x802DCBFC(second, second, first)` thunks; `0x800DD770`, `0x800DD7D0` and
  `0x800DE850` repeat the release wrapper already covered by [`fn_800d7fc0`].
  Two of each shape are enough to document the pattern; the rest carry no new
  behaviour and were left rather than padded into the lane.
  The four `dReplayPlay_c` bodies (`0x800DD950`, `0x800DDC50`, `0x800DE020`,
  `0x800DE510`) and the `0x800DF990..0x800DFF30` record-selector family are also
  left: they read SDA2 floats or return 64-bit pairs the export splits across
  registers.

## Verification

- Every address and byte size matches `functions.tsv`; the lane validator
  cross-checks the `# Original` sizes and reported no mismatch over the 29 rows
  (total 2,452 bytes).
- Every evidence file exists at the cited path and every row has exactly one
  `pub unsafe fn fn_<addr>` body in `src/engine/display.rs`.
- `ledger/functions.csv` was read only; all 29 addresses were missing from it
  before this pass, and `ledger/agents/wave89-display.csv` has 29 unique rows.
- `node tools/check-lane-csv.mjs wave89` reports `OK`.
- `cargo check --message-format=short` reports no error and no warning in
  `src/engine/display.rs`; the errors in the tree belong to other lanes.
