# Coverage - Wave 97 (engine::dsys)

## How to read this file

`engine::dsys` reconstructs the `dSys_c` engine start-up together with the
display, text, sprite and notification helpers around it in the main `DOL`
(bucket `0x800E0000`, 189 functions in total).

Each row below is one `pub unsafe fn fn_<address>` in `src/engine/dsys.rs`. The
doc block above that body cites the Ghidra export it came from, the byte count
from `work/ghidra/main-gb/export/functions.tsv`, and a confidence level:

- `high` means every guard, store and call argument is literal in the export;
- `medium` means a field role, a callee identity or a prologue-pair recovery was
  inferred and is marked `// inferred:` or `// unknown:` in the body;
- `low` means the instruction sequence is certain but the callee's argument list
  is not recoverable from the export.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 25 | 7432 | 3 | 22 |

The file already held 102 bodies (18 `dSys_c` start-up, 30 Wave 75, 28 Wave 89
and 26 Wave 93); this pass adds 25 more, for 127 bodies total. The bucket still
has 22 untranslated entries, all listed under **Deliberately skipped** below.

## What this pass covers

- **Band selection and progress pair, `0x800E24B0..0x800E27C0` (4 bodies).**
  The vtable `+0x3C0` band lookup, the packed two-float progress return (its
  high half is the `G_5620` phase, its low half the parity-corrected metric),
  the two normalised deltas that divide by that pair, and the single-value
  variant used by the cursor widget.
- **Message opcode expander, `0x800E71C0` (1 body).** The `0x802D7820` opcode
  reader, the literal-run copy, the allocator substitution path, and the
  seventeen-arm switch that dispatches to the message builders already present
  in this file. The wide result is copied from `context + 4` with the exported
  `0x80004364`.
- **Three animation bindings, `0x800E79F0..0x800E7B10` (3 bodies).** Each copies
  one word out of the slot array, calls a different five-argument track
  constructor (`0x80166BD0`, `0x80167800`, `0x80165FD0`), binds the two 64-bit
  handles with `0x80169F80`, and finishes with a per-variant setter.
- **Widget delta and record distance, `0x800E7EE0..0x800E9060` (2 bodies).** The
  widget `+0x38` subtraction and the three-float offset magnitude divided by
  `fn_800e9020` with an upper clamp.
- **Angle, midpoint and the two transform passes, `0x800EA7D0..0x800EAE90`
  (3 bodies).** The `0x801611C0` angle pair with its `0x80237BA0` square-root
  correction, plus the `+0x20` and `+0x60` per-slot transform passes that share
  `0x8023D300` / `0x8023D460`.
- **Water-full variants, hit test and nearest record, `0x800EB040..0x800EB930`
  (3 bodies).** The six-variant constructor (including the magma-heat and
  poison-water fog layer for variants 2 and 3), the 0x1C-byte hit test with its
  two square-root corrections, and the 0x50-entry scan that dispatches between
  the two hit-test helpers.
- **Radial-gradient bake, `0x800EDF70` (1 body).** The 0x80 x 0x80 texture
  format/palette setup and the two nested texel passes.
- **Aspect transform, variant build, retire and list add,
  `0x800EE170..0x800EE9B0` (4 bodies).** The three-matrix aspect correction, the
  `g3d_model.brres` base/alternate construction, the keyed retire path, and the
  list-add pass.
- **Phase binding, readiness and predicates, `0x800EEE70..0x800EFAA0`
  (4 bodies).** The two phase records for variant 2/submode 1, the two-slot
  readiness test, and the `+0x70` / `+0x9C` reached-target predicates.

### Evidence that unblocked the pass

Two patterns that Wave 93 read as blockers turned out to be harmless once the
whole export was considered:

1. The `ldexpf` prologue at the head of several bodies is the paired-single
   register save of `f27..f31` (`psq_st` with a GQR). The saved words are never
   read again, so the block is a dead store and the real body starts after it.
2. The `0x802DCD54/68/6C/70/74` stubs return a pair of live words. Reading both
   halves with `platform::call_u64` keeps the object base and the second value
   instead of guessing one of them.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800E24B0` | `fn_800e24b0` | 152 | medium |
| `0x800E2580` | `fn_800e2580` | 368 | medium |
| `0x800E26F0` | `fn_800e26f0` | 196 | high |
| `0x800E27C0` | `fn_800e27c0` | 216 | medium |
| `0x800E71C0` | `fn_800e71c0` | 768 | medium |
| `0x800E79F0` | `fn_800e79f0` | 144 | medium |
| `0x800E7A80` | `fn_800e7a80` | 144 | medium |
| `0x800E7B10` | `fn_800e7b10` | 144 | medium |
| `0x800E7EE0` | `fn_800e7ee0` | 60 | high |
| `0x800E9060` | `fn_800e9060` | 196 | high |
| `0x800EA7D0` | `fn_800ea7d0` | 252 | medium |
| `0x800EAD90` | `fn_800ead90` | 244 | medium |
| `0x800EAE90` | `fn_800eae90` | 244 | medium |
| `0x800EB040` | `fn_800eb040` | 588 | medium |
| `0x800EB6E0` | `fn_800eb6e0` | 588 | medium |
| `0x800EB930` | `fn_800eb930` | 372 | medium |
| `0x800EDF70` | `fn_800edf70` | 444 | medium |
| `0x800EE170` | `fn_800ee170` | 212 | medium |
| `0x800EE610` | `fn_800ee610` | 568 | medium |
| `0x800EE850` | `fn_800ee850` | 220 | medium |
| `0x800EE9B0` | `fn_800ee9b0` | 300 | medium |
| `0x800EEE70` | `fn_800eee70` | 368 | medium |
| `0x800EF230` | `fn_800ef230` | 308 | medium |
| `0x800EF9F0` | `fn_800ef9f0` | 168 | medium |
| `0x800EFAA0` | `fn_800efaa0` | 168 | medium |

## Field notes

- `CONCAT44(0x43300000, x)` is the CodeWarrior `2^52` widening. Where the
  export uses it only to recover a signed integer, the body writes the
  equivalent `as i32 as f64` cast and says so in the confidence line.
- The pair-returning functions (`fn_800e2580`, `fn_800e26f0`) return a `u64`
  whose two halves are `f32` bit patterns, because that is exactly what the
  original puts in the two result registers.
- `record_address` keeps the `base + index*0x48 + 0xD8` arithmetic signed. Two
  call sites still forward a `-1` lookup result on the failure path, and the
  original adds the same offset in that case.
- The 0x48-byte record layout is used consistently: `+0x100` is the target
  position, `+0xD8` the first timed slot, and `+0x48` the stride.

## Deliberately skipped

22 bucket entries remain. Each was read in full before being left out.

- **Large float/vector bodies not attempted after the lane reached its target,
  12 bodies** - `0x800E2DE0`, `0x800E2F10`, `0x800E3040`, `0x800E3160`,
  `0x800E5820`, `0x800E5AD0`, `0x800E5DD0`, `0x800E6090`, `0x800E63A0`,
  `0x800E6680`, `0x800E9130`, `0x800EC3B0`. In each of these the leading
  `ldexpf` block is the same dead `f27..f31` save, but the remaining body is
  276-1836 bytes of float and GQR work. Reconstructing them honestly needs each
  surviving `in_ps*` operand proved first; that was left for a later pass rather
  than guessed.
- **Object base and index share one fake pair, 2 bodies** - `0x800E20A0`,
  `0x800E3BA0`. The band call and the 892-byte unrolled store block both need
  two different values from the `0x802DCD74` pair; one of the two would have to
  be guessed.
- **Packed-pair return with an ambiguous half, 1 body** - `0x800E28A0`. The two
  display metrics feed a packed pair, but the export reuses `dVar4` for both the
  metric and the bias before the pair is formed, so the half assignment is not
  provable.
- **Large field-by-field stack copy, 1 body** - `0x800E0540` (1756 bytes). Three
  0x3FC0-byte copies with per-field unpacking; the source layout is not proven
  and a byte-for-byte body would be speculation.
- **Nested resource scan with aliased locals, 1 body** - `0x800EEAE0` (908
  bytes). The second pass reuses the first pass's `local_34`/`local_38` names
  for different records, so the append positions cannot be tied to the first
  traversal.
- **Inconsistent printed arity, 3 bodies** - `0x800EEFF0`, `0x800EF890`,
  `0x800EF940`. `0x80166360` and `0x80167B80` appear with one argument in one
  place and two in another, so the track-handle argument cannot be pinned down.
- **Byte-wise sign test with unknown intent, 1 body** - `0x800EF570`. The low
  half of the `0x802DCD6C` pair is consumed by a byte-wise sign expression whose
  purpose is not evident from the export.
- **Ambiguous running index, 1 body** - `0x800EF770`. The first loop's count is
  reused as the second loop's starting index, and the export does not show where
  that index is initialised, so the append positions are ambiguous.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/engine/dsys.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/engine/dsys.rs`.
- `node tools/check-lane-csv.mjs wave97` reports `OK`.
