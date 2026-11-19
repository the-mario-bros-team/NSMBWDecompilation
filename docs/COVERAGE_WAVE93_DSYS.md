# Coverage - Wave 93 (engine::dsys)

## How to read this file

`engine::dsys` reconstructs the `dSys_c` engine start-up together with the
display, text, sprite and notification helpers that sit around it in the main
`DOL` (bucket `0x800E0000`, 189 functions in total).

Each row below is one `pub unsafe fn fn_<address>` in `src/engine/dsys.rs`. The
doc block above that body cites the Ghidra export it came from, the byte count
from `work/ghidra/main-gb/export/functions.tsv`, and a confidence level:

- `high` means every guard, store and call argument is literal in the export;
- `medium` means a field role, a callee identity or a saved-register recovery
  was inferred and is marked `// inferred:` or `// unknown:` in the body;
- `low` means the instruction sequence is certain but the callee's argument
  list is not recoverable from the export, so it stays a bare `call0` / `call3`.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 26 | 5556 | 10 | 13 | 3 |

The file already held 76 bodies (18 `dSys_c` start-up, 30 Wave 75 and 28
Wave 89); this pass adds 26 more, for 102 bodies total. The bucket still has 47
untranslated entries, all listed under **Deliberately skipped** below.

## What this pass covers

- **Drawing-band selection, `0x800E1F80` (1 body).** The `-1` early exit, the
  `5` short path, the `0x8005FB90` lookup with its virtual `+0xE0` veto, the
  `0x800B3540` band write and the three refinement arms (`8`, `9`, `< 8`) that
  read the two eight-entry tables at `0x802F2E98` / `0x802F2EC0`.
- **Two scalar float helpers, `0x800E2DB0..0x800E2DC0` (2 bodies).** A
  square-and-scale and a three-term dot product; both round to `f32` at the
  same points the export does.
- **Slot sweep, `0x800E3F20` (1 body).** The 64-slot pass over `+0x488` /
  `+0x5C8` / `+0x588` with the 64-bit `1 << index` mask from `0x802DD1EC` and
  the single `0x8019BD20(renderer, 8)` flush.
- **One-shot library colour setup, `0x800E54E0..0x800E5790` (3 bodies).** These
  are the three bodies Wave 89 skipped for want of a float-argument call
  helper; the export turned out to need `(f64 x4, u32, u32)` for only one of
  them, and everything else is plain integer arguments.
- **Text-stream dispatcher, `0x800E6290` (1 body).** The low-nibble `2` test,
  the `opcode >> 4` argument to `0x800E5DD0`, the `0x800E6090` drawing step and
  the two-byte cursor advance.
- **Notification list lookup, `0x800E76A0` (1 body).** The `+4` next / `+8`
  payload traversal that mirrors [`fn_800e7590`], the three-part key test, the
  `s16` counter at `+0x1C` and the retire path.
- **Cursor-widget initialiser, `0x800E7BA0` (1 body).** Vtable `0x8031B1A8`,
  the three `+0x900FF` mode arms, the two metric pairs and the
  [`fn_800e8010`] / [`fn_800e8110`] hand-off.
- **Fixed word to scaled float, `0x800E9020` (1 body).** The *signed* `2^52`
  widening (`word ^ 0x80000000` against the real bias `0x8042CD88`) followed by
  one `f32` multiply.
- **State switch, `0x800E9D10` (1 body).** The `0x801C14C0` magnitude of the
  three floats at `source + 0x628`, the threshold test and the virtual `+0x18`
  call with `0x80371B48`.
- **Two dispatch thunks, `0x800EA000..0x800EA030` (2 bodies).** One-line
  forwarders onto `0x802DCBFC`; both recorded as `low` because that callee
  reads its vtable descriptor from `r12`, which the thunk does not pass.
- **The six `dWanKusari_c` state descriptors, `0x800EA060` (1 body).** Two
  hundred and eighteen literal stores collapsed into one six-iteration loop:
  nine words from `0x8031B1B8 + i*0x24` into `0x80371B08 + i*0x40` in the
  `[6,7,8,3,4,5,0,1,2]` order, plus `0x8015F7C0` and `0x802DC780` per state.
- **`g3d_wanwan.brres` construction, `0x800EA630` (1 body).** The resource load,
  the `(count + 1) * 4 & 4` name selector and the six-argument widget bind.
- **Animated node, `0x800EA6D0..0x800EA980` (4 bodies).** The `+0x4C` finished
  guard with state slot `+0x14`, the frame rebuild from position plus three
  `u16` angles, the speed decay / clamp / integration step with its floor test,
  and the two-sine reseed.
- **Sprite corners, `0x800EAF90` (1 body).** Both corner pairs share the same
  arithmetic; only the `(byte)(mode - 2) > 1` guard separates them.
- **Sprite matrix and step, `0x800EB290..0x800EB410` (2 bodies).** The
  descriptor-filtered child lookup plus translation matrix push, and the
  `counter % stride` writer with its two state slots.
- **Sprite record accessors, `0x800EFB50..0x800EFF30` (3 bodies).** The draw
  frame rebuild from the three optional record indices, and the `f32` reader /
  writer pair around the same `0x8025C660` / `0x8025C470` lookup path.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800E1F80` | `fn_800e1f80` | 280 | medium |
| `0x800E2DB0` | `fn_800e2db0` | 16 | high |
| `0x800E2DC0` | `fn_800e2dc0` | 24 | high |
| `0x800E3F20` | `fn_800e3f20` | 236 | medium |
| `0x800E54E0` | `fn_800e54e0` | 268 | high |
| `0x800E55F0` | `fn_800e55f0` | 412 | high |
| `0x800E5790` | `fn_800e5790` | 140 | high |
| `0x800E6290` | `fn_800e6290` | 116 | low |
| `0x800E76A0` | `fn_800e76a0` | 244 | medium |
| `0x800E7BA0` | `fn_800e7ba0` | 208 | medium |
| `0x800E9020` | `fn_800e9020` | 52 | high |
| `0x800E9D10` | `fn_800e9d10` | 124 | medium |
| `0x800EA000` | `fn_800ea000` | 48 | low |
| `0x800EA030` | `fn_800ea030` | 48 | low |
| `0x800EA060` | `fn_800ea060` | 1236 | high |
| `0x800EA630` | `fn_800ea630` | 148 | medium |
| `0x800EA6D0` | `fn_800ea6d0` | 28 | high |
| `0x800EA6F0` | `fn_800ea6f0` | 224 | medium |
| `0x800EA8D0` | `fn_800ea8d0` | 172 | high |
| `0x800EA980` | `fn_800ea980` | 116 | medium |
| `0x800EAF90` | `fn_800eaf90` | 168 | high |
| `0x800EB290` | `fn_800eb290` | 380 | medium |
| `0x800EB410` | `fn_800eb410` | 168 | medium |
| `0x800EFB50` | `fn_800efb50` | 284 | medium |
| `0x800EFE60` | `fn_800efe60` | 196 | medium |
| `0x800EFF30` | `fn_800eff30` | 220 | medium |

## Field notes

- `CONCAT44(0x43300000, x)` is the CodeWarrior `2^52` widening. `0x800E9020`
  uses the **signed** form (`x ^ 0x80000000` against `2^52 + 2^31` at
  `0x8042CD88`), while `0x800EB410` uses the **unsigned** form against the bias
  at `0x8042CE00`. Both biases are read from their real addresses through the
  `read_f64` helper instead of being assumed to be `2^52`.
- `0x802DD1EC` renders as a 64-bit left shift of the `(hi, lo)` pair. `0x800E3F20`
  calls it with `(0, 1, index)` to build the `1 << index` slot mask and splits
  the `u64` result back over the two halves of the field; the new local helper
  `call3_u64` keeps the 64-bit result instead of dropping the high word.
- `unaff_r13` is the SDA base `0x8042F680`, `unaff_r2` the second base
  `0x80433080`. Every offset used by this pass is spelled out as a named
  constant (`F_*`, `G_*`) with its resolved absolute address in the comment.
- `0x802DCD74` is a Metrowerks register-save stub. When a body needs the object
  register it survives only as the fake 64-bit return of that stub; the two
  bodies that depend on it (`0x800E3F20`, and the skipped `0x800EAD90` family)
  are marked `// inferred:` at exactly that point.
- Eight bodies needed float-argument or 64-bit-result call shapes, so
  `src/engine/dsys.rs` gained seven small `extern "C"` wrappers:
  `call_f32_arg_ret`, `call_ptr_f32_ret`,
  `call_f64x3_ptr`, `call_f64x4_u32x2`, `call_f64_u32x2`, `call_f64_void` and
  `call3_u64`. They are documented in the module's "Local helpers" section and
  rely on the 32-bit PowerPC EABI assigning the float and integer register files
  independently.

## Deliberately skipped

47 of the 73 remaining bucket entries were left alone. Every case below was read
in full first; the reason is the one that actually blocks an honest body.

- **Paired-single mis-render (`ldexpf` / `unaff_GQR0`), 27 bodies** -
  `0x800E26F0`, `0x800E27C0`, `0x800E28A0`, `0x800E2DE0`, `0x800E2F10`,
  `0x800E3040`, `0x800E3160`, `0x800E5820`, `0x800E5AD0`, `0x800E5DD0`,
  `0x800E6090`, `0x800E63A0`, `0x800E6680`, `0x800E7EE0`, `0x800E9060`,
  `0x800E9130`, `0x800EA7D0`, `0x800EB6E0`, `0x800EC3B0`, `0x800EDF70`,
  `0x800EE170`, `0x800EEE70`, `0x800EF230`, `0x800EF890`, `0x800EF940`,
  `0x800EF9F0`, `0x800EFAA0`. Each export inserts literal `ldexpf(bits & 0x3f)`
  calls for what were `ps_sum0`/`ps_madd` paired-single pairs. The bogus call
  is interleaved with the real arithmetic, so at least one operand of every
  float expression is unrecoverable. Transcribing the `ldexpf` calls would be
  inventing behaviour, and dropping them would be inventing operands.
- **Register-save stub carries two independent values, 6 bodies** -
  `0x800EAD90`, `0x800EAE90`, `0x800EB040`, `0x800EE850`, `0x800EF570`,
  `0x800EF770`. In these the `0x802DCD74` / `0x802DCD6C` fake return supplies
  *both* the object base and a second live value (a variant index, a record
  index, or the argument of the following call). Recovering both from one
  fake pair would be two guesses stacked on each other, so nothing was written.
- **Six-argument mixed float calls, 3 bodies** - `0x800EE610`, `0x800EE9B0`,
  `0x800EEFF0`. They pass `(f64, f64, u32, u32, pointer, u32)` to
  `0x800F1390` / `0x800F11F0` / `0x800F1550` and also rely on the same
  fake-return pair for the object base; the argument order cannot be pinned
  down from the export alone.
- **Large bodies built on object layouts this pass could not prove, 5 bodies** -
  `0x800E3BA0` (892 bytes, fully unrolled eight-slot store block whose base is
  the fake-return register), `0x800E0540` (1756 bytes, three 0x3FC0-byte stack
  copies plus a per-entry record rewrite), `0x800E71C0` (768 bytes, the
  seventeen-arm message opcode switch - valuable, but two of its pointers come
  from the `0x802DCD54` fake pair), `0x800EEAE0` (908 bytes, nested resource
  list scan), `0x800EB930` (372 bytes, needs the untranslated `0x800EB6E0`).
- **Call sites whose arguments the export drops, 3 bodies** - `0x800E20A0`,
  `0x800E24B0`, `0x800E2580`. The first is rendered with `undefined8`
  parameters throughout; the second passes both halves of a fake return into
  `0x800E1F80`; the third returns a `CONCAT44` pair of floats, i.e. a two-float
  structure return, which the module has no ABI-faithful shape for.
- **`0x800E79F0`, `0x800E7A80`, `0x800E7B10`** - all three are printed with
  `undefined8` leading parameters (register pairs) and seven arguments, so the
  real parameter list is not knowable from the export.

Everything in that list is a candidate for a later pass that reads the raw
PowerPC disassembly rather than the decompiler output.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/engine/dsys.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/engine/dsys.rs` (the remaining errors belong to other lanes' files:
  `src/runtime/bt.rs`, `src/resource/registry.rs`).
- `node tools/check-lane-csv.mjs wave93` reports `OK`.
