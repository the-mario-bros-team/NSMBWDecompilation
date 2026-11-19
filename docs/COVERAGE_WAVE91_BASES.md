# Coverage - Wave 91 (`game::bases`, `0x8079` bucket continuation)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8031xxxx`,
`0x8037xxxx`, `0x8042xxxx`) are `main.dol` data addresses imported by the module
and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants
and call arguments are literal, and `medium` when the role of a field, a
global, a callee or a dropped argument had to be inferred (those bodies carry
`// observed:`/`// inferred:`/`// unknown:` notes). No body in this pass is
`low`.

The pass adds three small host-side ABI wrappers (`call_f64x4`,
`call_f64x2_u32x4`, `call_f64_u32x4`) because the `0x8079` tail calls
`0x800F6810`, `0x800F6110` and `0x80105060` with mixed double/word argument
lists that the existing `call_f32*` helpers cannot model.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 6,276 | 11 | 19 |

## What this pass covers

The `0x8079` bucket holds 184 functions; 35 were already in
`ledger/functions.csv` when this pass began (the five sub-state setters at
`0x807982A0..0x807984F0` and the 30 wave-90 families), leaving 149. This pass
takes 30 of the remaining bodies from the previously unreached
`0x807948A0..0x8079FE20` window.

- **Placement-record start-up, `0x807948A0` (1 body).**
  [`fn_807948a0`] is the sibling of the wave-90 vine actor: it indexes the same
  `0x8081B4F0` table with the low byte of `+0x04`, resolves the name at
  `0x8081B4D4[index]`, and arms `+0x330`/`+0x344` when one of the two course
  queries (`0x800FAE80`/`0x800FAF70`, chosen by `0x807D0038[index]`) succeeds.
- **Four-slot `character_SV` player panel, `0x80795140..0x80796180` (13
  bodies).** The in-place constructor [`fn_80795140`] installs the class
  vtable `0x8081BD78` at `+0x60`, the `+0x1CC` holder and the `0x80377DC8`
  vector, and publishes itself at `0x8042A1A0`. [`fn_807954c0`] selects the
  slot-0 model from `0x80354E70` and binds `character_SV`;
  [`fn_80795550`]/[`fn_807955d0`] build and tear down three `0x295` children
  linked through `+0x184`/`+0x188`; [`fn_80795620`] rebuilds the
  `0x80354E50` presence words and the `0x80832EB8` character mirror;
  [`fn_807957b0`]/[`fn_80795850`] find a child with `0x80781C30` and arm its
  `+0x124` byte or drive its model interface (slots `+0x30`/`+0x40`/`+0x48`);
  [`fn_80795920`] refreshes one model per active slot through `0x807C3B60`;
  [`fn_80795a20`] applies the `+0x298` scale while state `+0x228` is not `9`;
  [`fn_80795c40`] writes the transform from the `0x8031D3C0` per-character
  offset table; [`fn_80795d00`] is the six-way layout dispatcher for panel
  states `9` and `0xC`; and [`fn_80795fe0`]/[`fn_80796180`] fill the
  two-label text block, including the `10000`-accumulator swap retry.
- **Course-record enemy tail, `0x80798510..0x80798D40` (7 bodies).**
  [`fn_80798510`], [`fn_807986b0`] and [`fn_807987e0`] spawn the `0x282`
  effect actor, seed the `+0x1FC..+0x204` position block, play sound `7` and
  move the state word `+0x228` through `7`, `8` and `9`;
  [`fn_807987e0`] additionally copies the position to every child and keeps
  the first id from a six-record scan (`0x80772280`, `0x80772370`,
  `0x80774FA0`). [`fn_80798b10`] is the `W`/`W...S` record-name gate with the
  player idle latch, [`fn_80798c30`] rebuilds the `+0x23C` tree link,
  [`fn_80798d10`] writes the `0x807D01B0` rotation halfword and [`fn_80798d40`]
  remaps record kinds `0x15`/`0x16` and builds the three-vector rotation probe.
- **Siblings at the `0x8079E4E0..0x8079EDD0` window (3 bodies).**
  [`fn_8079e4e0`] gates on `0x800F6AB0`, selects state `0xD` and spawns the
  `0x8079EE70` effect; [`fn_8079ec40`] maps modes `0`, `1`, `2` to states
  `0xC`, `0`, `0xD` with their three-float scale triples; [`fn_8079edd0`]
  plays `0x80104FD0` for the actor and every armed child.
- **Layout/state bodies `0x8079F260..0x8079F320` (3 bodies).**
  [`fn_8079f260`] mirrors layout bit 4 of `+0x78` into `+0x308` and hides pane
  `0x10`; [`fn_8079f2c0`] restores that pane when `0x800FDD60` is nonzero; and
  [`fn_8079f320`] cancels the `0x800FDB30` result in modes `5` and `6`.
- **Fugu actor, `0x8079FBD0..0x8079FE20` (3 bodies).** The loader
  [`fn_8079fe20`] names `fugu`, `g3d/fugu.brres` and `character_SV`; the
  in-place constructor [`fn_8079fbd0`] is the wave-90 `character_SV` layout
  with class vtable `0x8081BFD0`, and [`fn_8079fc50`] is its destructor.

Start with [`fn_80795140`] for the panel layout and [`fn_8079fe20`] for the
`character_SV` loader shape.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x807948A0` | `fn_807948a0` | 212 | medium | placement-record start-up |
| `0x80795140` | `fn_80795140` | 204 | medium | panel in-place constructor |
| `0x807954C0` | `fn_807954c0` | 132 | medium | panel model and `character_SV` start-up |
| `0x80795550` | `fn_80795550` | 128 | medium | three `0x295` child chain |
| `0x807955D0` | `fn_807955d0` | 72 | high | refresh then child teardown |
| `0x80795620` | `fn_80795620` | 396 | medium | presence and character mirror refresh |
| `0x807957B0` | `fn_807957b0` | 160 | high | child arm/disable |
| `0x80795850` | `fn_80795850` | 200 | medium | child model interface drive |
| `0x80795920` | `fn_80795920` | 252 | medium | per-slot character refresh |
| `0x80795A20` | `fn_80795a20` | 124 | high | state-gated scale |
| `0x80795C40` | `fn_80795c40` | 184 | medium | transform writer |
| `0x80795D00` | `fn_80795d00` | 732 | medium | layout dispatcher and label rebuild |
| `0x80795FE0` | `fn_80795fe0` | 416 | medium | two-label text append |
| `0x80796180` | `fn_80796180` | 304 | high | text control block fill |
| `0x80798510` | `fn_80798510` | 176 | medium | `0x282` spawn and state 7 |
| `0x807986B0` | `fn_807986b0` | 188 | medium | `0x282` spawn and state 8 |
| `0x807987E0` | `fn_807987e0` | 376 | medium | position copy, six-record scan, state 9 |
| `0x80798B10` | `fn_80798b10` | 284 | medium | `W`/`S` name gate |
| `0x80798C30` | `fn_80798c30` | 132 | medium | tree link rebuild |
| `0x80798D10` | `fn_80798d10` | 36 | high | rotation halfword write |
| `0x80798D40` | `fn_80798d40` | 220 | medium | record-kind remap and probe |
| `0x8079E4E0` | `fn_8079e4e0` | 112 | medium | state `0xD` effect handler |
| `0x8079EC40` | `fn_8079ec40` | 224 | high | three-mode state/scale selector |
| `0x8079EDD0` | `fn_8079edd0` | 160 | high | per-child effect player |
| `0x8079F260` | `fn_8079f260` | 96 | high | pane flag mirror |
| `0x8079F2C0` | `fn_8079f2c0` | 84 | medium | pane restore |
| `0x8079F320` | `fn_8079f320` | 108 | high | collision result gate |
| `0x8079FBD0` | `fn_8079fbd0` | 128 | high | fugu in-place constructor |
| `0x8079FC50` | `fn_8079fc50` | 140 | high | fugu destructor |
| `0x8079FE20` | `fn_8079fe20` | 296 | medium | `g3d/fugu.brres` loader |

## Address range

`0x807948A0` (placement-record start-up), `0x80795140..0x80796180` (four-slot
player panel), `0x80798510..0x80798D40` (course-record enemy tail),
`0x8079E4E0..0x8079EDD0`, `0x8079F260..0x8079F320` and
`0x8079FBD0..0x8079FE20` (fugu actor). After this pass the `0x8079` bucket
still has 119 untranslated functions.

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x80794BF0` (12 bytes) is referenced by `0x80794980` as a float-returning
  helper, but its export is only `halt_baddata()` (truncated by bad instruction
  data). Without its body the two ratio computations cannot be reconstructed.
- `0x80794980` (624 bytes) consumes `0x80794BF0`, and its `ldexpf`/`unaff_GQR0`
  prologue leaves the paired-single storage class ambiguous. It also calls
  `0x801680E0` with the float argument dropped in the state-4 branch, so the
  body cannot be reproduced literally.
- `0x80795AA0` (412 bytes) is the panel's per-frame driver. Its only float is a
  paired-single move with `ldexpf`/`unaff_GQR0` artifacts; the scalar outline is
  readable but the storage class of the `+0x2E4..+0x2EC` triple is not.
- `0x80795D00`'s neighbours `0x807962B0` (1428 bytes), `0x80796850` (448) and
  the following panel/menu bodies were not reached.
- The `0x8079C330` (4232 bytes) and `0x8079D3C0` (3176 bytes) bodies are the
  large constructor/destructor pair that opens the second half of the bucket;
  they were not attempted in this pass.
- `0x80798CC0` (68 bytes) calls `0x80798D10` with both of its arguments dropped
  by Ghidra; the three halfword copies that follow are literal, but the call
  cannot be reproduced faithfully without inventing the arguments, so
  [`fn_80798d10`] is claimed instead.
- `0x8079F790` (360 bytes) and `0x8079F900` (260) split the 64-bit
  `0x802DCD54`/`0x802DCD74` results by hand: the low half is stored as two
  32-bit values at `+0x50`/`+0x54` and the export does not say which half is the
  actor pointer, so they were deferred. `0x8079F040` (512) was deferred with
  them because its `0x8079B260` gate and six-state `+0x2F4` body build on the
  same family.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 6,276 bytes),
  re-read from the TSV rather than from prose; the one stale count found during
  the pass (`0x8079FBD0`, 28 → 128) was corrected before the CSV was written.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 35 pre-existing rows of
  this bucket (`0x807982A0`, `0x80798310`, `0x80798370`, `0x80798410`,
  `0x807984F0` and the 30 wave-90 bodies) were listed before writing and none
  is in this pass.
- `node tools/check-lane-csv.mjs wave91` reports `wave91-bases.csv: 30 rows`
  and `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/bases.rs` (the crate still fails on other lanes' files, which are
  outside this lane).
