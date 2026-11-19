# Coverage - Wave 93 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. This pass adds 28 bodies from the `0x800D0000` bucket, the
engine band that the pause-menu controller, the water-effect actor and the
save-image work of waves 16/20/26 already used. Each body is a literal
reconstruction of one Ghidra export; the doc block above the Rust function names
the original address, the byte count taken from
`work/ghidra/main-gb/export/functions.tsv` and the evidence file.

"high" means every store, branch and call is visible in the export and the
translation does not depend on an inferred layout. "medium" means the export
drops a virtual receiver, a callee operand or a 64-bit debug-context word, an
SDA2 float's value is not recovered, or the `0x14`-byte effect-record stride is
inferred from the sibling `0x800D8E40`. Every field whose role is not proven is
marked `// unknown:` or `// inferred:` in the Rust comment.

Only `src/engine/display.rs` was edited; `ledger/functions.csv` is read-only and
the coordinator merges this lane's rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 28 | 8448 | 6 | 22 |

## What this pass covers

The bucket entered this pass with 71 untranslated of its 251 functions. This
pass closes three coherent slices and leaves the paired-single bodies, the
jumptable bodies and the duplicate thunk banks for a later pass.

- **The marker / effect selectors** (`0x800D0340..0x800D8EA0`, 15 bodies).
  [`fn_800d5330`] is the course-marker constructor (two pools, the
  `0x803033F0` model and the SDA2 rotation defaults); [`fn_800d36e0`] rebuilds
  its two layout nodes and [`fn_800d70e0`] turns the pad bits into the four
  marker modes. [`fn_800d7350`] and [`fn_800d74c0`] are the sweep geometry and
  its per-frame step. [`fn_800d3ce0`] and [`fn_800d3ef0`] map an instance id to
  a `P_R*`/`P_L_low_walk*` animation name. [`fn_800d4ad0`] and
  [`fn_800d4d80`] apply one archive entry and one model transform.
  [`fn_800d8850`], [`fn_800d8bb0`] and [`fn_800d8e40`] are the three
  per-player pad-effect writers; [`fn_800d8ea0`] advances their two damped
  float channels. [`fn_800d0340`] drains the list at `r13 - 0x56B0`;
  [`fn_800d6940`] forwards a two-word transform read.
- **The `dPosGoAndComeExeFrm_c` descriptor table** (`0x800D7C80`, 1 body,
  824 bytes). [`fn_800d7c80`] registers the four
  `dPosGoAndComeExeFrm_c::StateID_A/M/D/W` descriptors at
  `0x80371720/60/A0/E0` with handler `0x800D7FC0`. Its `dReplayPlay_c` sibling
  is [`fn_800de510`] below. The four registration blocks share the private
  helper `w93_register_state`, which reproduces the name call, the vtable
  store, the nine `+0x0C..+0x2C` words and the `0x802DC780` registration in the
  observed order.
- **The replay and `dConnect_c` helpers** (`0x800DD770..0x800DFE60`, 12
  bodies). [`fn_800dd950`] constructs the `dRemoconMng_c::dConnect_c` state
  object; [`fn_800dd770`], [`fn_800ddab0`] and [`fn_800ddaf0`] are its three
  release shims. [`fn_800ddc50`] recomputes the replay HUD's `+0xA8` flag
  halfword and [`fn_800de020`] decodes one event record from the replay
  stream. [`fn_800de510`] registers the four `dReplayPlay_c` states at
  `0x803719B0/F0/1A30/1A70`. [`fn_800df990`] and [`fn_800dfb70`] resolve and
  spawn archive records, [`fn_800dfc70`] and [`fn_800dfd20`] read the current
  node id and count the `+0x64` list, and [`fn_800dfe60`] resolves one partner
  index before forwarding it to `0x800DFF30`.

The bucket keeps two facts across the whole slice. First, `G_PLAYER_SLOTS`
(`r13 - 0x5658`) is the container the three effect writers index with
`player * 4 + 4`, and each slot keeps its route byte at `+0x55` and its view id
at `+0x04` - the same model wave 38 documented for the four-slot container.
Second, the two descriptor tables use the same `0x40`-byte state record as the
pause window: vtable at `+0x00`, nine handler words at `+0x0C..+0x2C`, and
`0x802DC780` as the registrar, so they are registered with the wave-90 helper
shape instead of a new one.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800D0340` | `fn_800d0340` | 96 | medium |
| `0x800D36E0` | `fn_800d36e0` | 328 | medium |
| `0x800D3CE0` | `fn_800d3ce0` | 520 | high |
| `0x800D3EF0` | `fn_800d3ef0` | 356 | medium |
| `0x800D4AD0` | `fn_800d4ad0` | 316 | medium |
| `0x800D4D80` | `fn_800d4d80` | 264 | medium |
| `0x800D5330` | `fn_800d5330` | 360 | medium |
| `0x800D6940` | `fn_800d6940` | 100 | medium |
| `0x800D70E0` | `fn_800d70e0` | 548 | medium |
| `0x800D7350` | `fn_800d7350` | 300 | medium |
| `0x800D74C0` | `fn_800d74c0` | 216 | high |
| `0x800D7C80` | `fn_800d7c80` | 824 | medium |
| `0x800D8850` | `fn_800d8850` | 268 | medium |
| `0x800D8BB0` | `fn_800d8bb0` | 176 | medium |
| `0x800D8E40` | `fn_800d8e40` | 96 | medium |
| `0x800D8EA0` | `fn_800d8ea0` | 544 | medium |
| `0x800DD770` | `fn_800dd770` | 88 | high |
| `0x800DD950` | `fn_800dd950` | 280 | medium |
| `0x800DDAB0` | `fn_800ddab0` | 64 | medium |
| `0x800DDAF0` | `fn_800ddaf0` | 96 | high |
| `0x800DDC50` | `fn_800ddc50` | 532 | high |
| `0x800DE020` | `fn_800de020` | 488 | medium |
| `0x800DE510` | `fn_800de510` | 824 | medium |
| `0x800DF990` | `fn_800df990` | 176 | medium |
| `0x800DFB70` | `fn_800dfb70` | 108 | medium |
| `0x800DFC70` | `fn_800dfc70` | 100 | medium |
| `0x800DFD20` | `fn_800dfd20` | 176 | high |
| `0x800DFE60` | `fn_800dfe60` | 204 | medium |

Byte total: `4488` (marker / effect selectors) + `824` (descriptor table) +
`3136` (replay and `dConnect_c` helpers) = `8448`.

## Deliberate skips

43 addresses of the bucket stay untranslated. Five groups have a hard reason and
one group is simply outside this pass's 28-body budget; the distinction matters,
so they are listed separately.

**Hard blockers - paired-single / GQR quantisation and removed blocks (17
addresses).** `0x800D0470`, `0x800D0610`, `0x800D19E0`, `0x800D2460`,
`0x800D2690`, `0x800D27B0`, `0x800D3380`, `0x800D43E0`, `0x800D4CD0`,
`0x800D5170`, `0x800D57F0`, `0x800D5DD0`, `0x800D6460`, `0x800D65E0`,
`0x800D9D70`, `0x800DAA10` and `0x800DAC40` are printed with `unaff_GQR0` /
`unaff_GQR3`, `ldexpf`, `CONCAT13/CONCAT22` quantisation stubs or
`extraout_f1`, several with `WARNING: Removing unreachable block` where the
paired-single semantics used to be. Any Rust body would be an invented
approximation, so they stay open. `0x800D5170` also blocks `0x800D38A0`, which
calls it by address.

**Hard blockers - unrecovered jump tables (2 addresses).** `0x800D6550` and
`0x800D69B0` both carry `Could not recover jumptable ... Treating indirect jump
as call`. Their dispatch order is not visible in the export.

**Hard blockers - no observable body (1 address).** `0x800DFDD0` is printed as a
pure `+0x64` list walk with no store and no call after the dropped
`0x800DFD20` result; the export itself has lost the effect, so a Rust body would
be empty.

**Hard blockers - dropped extra-out registers (4 addresses).** `0x800D8B00` and
`0x800D8C60` read their player byte from `extraout_r4`, which the export never
defines; `0x800D2CF0` reads its scale from `extraout_f1` and its kind from the
low word of the `0x802DCD74` 64-bit result while leaving two register
parameters unused; `0x800DAB40` calls `0x80101AA0` with no printed operand at
all. Recovering those register contents needs the disassembly, which is a
separate pass.

**Duplicate shapes - one representative is enough (9 addresses).**
`0x800D8670` is a 4-byte thunk into the already-translated `fn_800d8680`;
`0x800DD1C0`, `0x800DD1F0`, `0x800DD220`, `0x800DD250` and `0x800DD280` are
five identical `0x802DCBFC(second, second, first)` constructor thunks of exactly
the shape documented by `fn_8015dd70`; `0x800DD7D0`, `0x800DDB50` and
`0x800DE850` are byte-for-byte the same releases as [`fn_800dd770`] and
[`fn_800ddaf0`]. The representative bodies carry the pattern; the rest add no
new behaviour and were left rather than padded into the lane.

**Left at the budget, not blocked (10 addresses).** `0x800D16B0`,
`0x800D38A0`, `0x800D9760`, `0x800DD2B0`, `0x800DE4B0`, `0x800DE4E0`,
`0x800DFA40`, `0x800DFBE0`, `0x800DFCE0` and `0x800DFF30` are readable exports
that did not fit the lane. Four deserve a note for the next pass:
`0x800DD2B0` is a 1,212-byte `dRemoconMng_c::dConnect_c::dExte*` descriptor
table and can be written with the same `w93_register_state` helper as
[`fn_800de510`]; `0x800D9760` is the `0x80377C88` table walk and belongs with
the effect writers; `0x800DFA40` and `0x800DFF30` are the two callees the
replay helpers call by address; `0x800D16B0`, `0x800DE4B0` and `0x800DE4E0`
are operand-less `0x802DCBFC` forwarding thunks.

## Verification

- Every address and byte size matches `functions.tsv`; the 28 `# Original`
  blocks total 8,448 bytes, re-read from the TSV for this pass.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/display.rs`.
- No address was already in `ledger/functions.csv`; the bucket had 71
  untranslated functions before this pass and 43 after it.
- `cargo check --message-format=short` reports no error or warning naming
  `src/engine/display.rs`. The remaining errors in the tree belong to other
  lanes (`game::worldmap`, `game::stage`, `engine::dsys`,
  `resource::registry`, `runtime::bt`).
- `node tools/check-lane-csv.mjs wave93` reports `OK`; this lane's CSV has 28
  unique rows and no duplicate address.
