# Coverage - Wave 82 (`game::bases`, `0x8067` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8037xxxx`,
`0x8042xxxx`) are `main.dol` addresses imported by the REL and keep their
absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants,
strings and call arguments are literal, and `medium` when the role of a field,
a global, a callee or a dropped argument had to be inferred (those bodies carry
`// observed:`/`// inferred:`/`// unknown:` notes). No body in this pass is
`low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 3,464 | 20 | 7 |

## What this pass covers

The `0x8067` bucket holds 199 functions; 55 were already in
`ledger/functions.csv` when this pass began (wave 57 and its neighbours),
leaving 144. This pass takes 27 of the remaining bodies in three runs that wave
57 recorded but did not translate.

- **Lemmy-ball helpers, `0x80670BB0..0x80670F10` (2 bodies).** Two members of
  the `0xC20`-byte `daCastleLemmyBall_c` actor whose constructor and state table
  wave 57 recorded at `0x80670640` and `0x80672500`: the `+0x544` colour-slot
  writer [`fn_80670bb0`] (merge `+0x89D` into the low byte, `0x80168880`, then
  the shared [`fn_806b58e0`] refresh) and the ball-size multiplier
  [`fn_80670f10`], which chooses between `0x807C90FC` and `0x807C9100` from two
  virtual queries on the `+0x5F8` interface.
- **Flag-pole actor member tail, `0x80675620..0x806762A0` (13 bodies).** The
  `0x4E0`-byte actor built by [`fn_806749c0`] (vtable `0x807EAE90`, model
  `+0x3F0`, clip blocks `+0x430`/`+0x49C`, interface `+0x394`); the flag-pole
  identity is `// inferred:` from the `+0x46C` clip block and `+0x4D8` scale that
  the wave-57 capture handlers [`fn_806752a0`]/[`fn_806753b0`] already use. The
  run holds the capture
  hand-off [`fn_80675620`], the clip binder pair
  [`fn_806756b0`]/[`fn_80675720`], the `swing` pose [`fn_80675790`] and its
  frame stepper [`fn_80675820`], the `swing` four-frame variant
  [`fn_806758b0`], the `wait_sea` pose [`fn_80675a60`] and its hold
  [`fn_80675af0`], the `swing_sea` pose [`fn_80675b60`], and the
  collision-gated exit [`fn_80675c10`]. The neighbouring `0x420`-byte actor
  supplies its release [`fn_806760a0`], constructor [`fn_80676220`] (vtable
  `0x807EB0F4`) and position publisher [`fn_806762a0`].
- **Larry magic-attack effect actor, `0x80679C60..0x8067A8B0` (12 bodies).**
  The `0x5A8`-byte actor whose constructor [`fn_8067a000`] installs vtable
  `0x807EBF48`: two release wrappers [`fn_80679c60`]/[`fn_80679e80`], the
  loader [`fn_8067a1e0`] that mounts `boss_ef_attack` and binds the
  `larry_attack` clips from four `+0x478`-indexed tables, the transform pair
  [`fn_8067a3e0`]/[`fn_8067a4d0`], the ice-hit burst [`fn_8067a6b0`], the
  `Wm_la_magicattack` dispatch [`fn_8067a750`], the timeout gate
  [`fn_8067a7e0`], the `0x51D` sound record [`fn_8067a840`] and the animation
  re-emitter [`fn_8067a8b0`]. The class attribution is `// inferred:` from the
  `boss_ef_attack`/`larry_attack` strings; the REL carries no class name.

Object layout used by the flag-pole run: `+0x148` collision area, `+0x394`
interface object, `+0x3EC` archive handle, `+0x3F0` model, `+0x430`/`+0x434`
first clip block, `+0x498` cached pane word, `+0x49C` second clip block,
`+0x4D8` speed-derived scale. The effect actor uses `+0x3D0` archive wrapper,
`+0x3EC` handle, `+0x3F0` model, `+0x430`/`+0x434` clip block, `+0x46C`
transform sample, `+0x478` variant index, `+0x47C` effect interface, `+0x460`
timer and `+0x4D8`-free runtime words at `+0x590..+0x5A0`.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80670BB0` | `fn_80670bb0` | 108 | medium | Lemmy-ball `+0x544` colour-slot writer |
| `0x80670F10` | `fn_80670f10` | 144 | medium | ball-size multiplier from two virtual queries |
| `0x80675620` | `fn_80675620` | 132 | medium | flag-capture hand-off |
| `0x806756B0` | `fn_806756b0` | 112 | high | first clip binder |
| `0x80675720` | `fn_80675720` | 112 | medium | second clip binder |
| `0x80675790` | `fn_80675790` | 116 | high | plain swing pose, time `8.0` |
| `0x80675820` | `fn_80675820` | 140 | medium | swing frame stepper with clamp |
| `0x806758B0` | `fn_806758b0` | 148 | high | swing variant, time `4.0` |
| `0x80675A60` | `fn_80675a60` | 116 | high | `wait_sea` pose, time `30.0` |
| `0x80675AF0` | `fn_80675af0` | 108 | high | hold current pose |
| `0x80675B60` | `fn_80675b60` | 152 | high | `swing_sea` pose, time `4.0` |
| `0x80675C10` | `fn_80675c10` | 172 | high | collision-gated sea exit |
| `0x806760A0` | `fn_806760a0` | 88 | high | release / free wrapper |
| `0x80676220` | `fn_80676220` | 120 | high | `0x420`-byte actor constructor |
| `0x806762A0` | `fn_806762a0` | 104 | medium | position publisher |
| `0x80679C60` | `fn_80679c60` | 88 | high | release with `0x80162920` |
| `0x80679E80` | `fn_80679e80` | 88 | high | release with `0x802B9280` |
| `0x8067A000` | `fn_8067a000` | 232 | high | `0x5A8`-byte effect-actor constructor |
| `0x8067A1E0` | `fn_8067a1e0` | 336 | high | `boss_ef_attack` + `larry_attack` loader |
| `0x8067A330` | `fn_8067a330` | 84 | high | reset, transform, return `1` |
| `0x8067A3E0` | `fn_8067a3e0` | 228 | high | transform / matrix writer |
| `0x8067A4D0` | `fn_8067a4d0` | 68 | high | frame-0 animation sample |
| `0x8067A6B0` | `fn_8067a6b0` | 152 | high | ice-hit bursts and sound `0x51F` |
| `0x8067A750` | `fn_8067a750` | 56 | high | `Wm_la_magicattack` dispatch |
| `0x8067A7E0` | `fn_8067a7e0` | 92 | high | timeout descriptor gate |
| `0x8067A840` | `fn_8067a840` | 92 | medium | sound `0x51D`, paired-single call address-only |
| `0x8067A8B0` | `fn_8067a8b0` | 76 | high | animation advance and re-emit |

## Address range

`0x80670BB0..0x8067A8B0`, three runs:

- Run 1 is `0x80670BB0..0x80670F10`.
- Run 2 is `0x80675620..0x806762A0`, with the deliberate hole at the two
  `0x802DCBFC` thunks `0x806761C0`/`0x806761F0`.
- Run 3 is `0x80679C60..0x8067A8B0`, with deliberate holes at
  `0x80679FA0`/`0x80679FD0` (the same thunk pair), `0x8067A520` (paired-single
  body), `0x8067A790` (bad instruction data) and `0x8067A900` (its leading call
  is that bad-data body).
- After this pass the bucket still has 117 untranslated functions, chiefly the
  `0x80670FA0..0x80671D70` animation/GQR set, the `0x80672E20..0x80673D70`
  name-compare and child-chain quartet, and the `0x8067AE50..0x8067FC80` tail
  that wave 20 started.

## Deliberate skips

- `0x806761C0`, `0x806761F0`, `0x80679FA0`, `0x80679FD0` (48 bytes each) are
  compiler thunks that forward to the unresolved indirect-jump stub
  `0x802DCBFC`. They have no Rust-visible effect and follow the module's stated
  policy for the `0x802DCD70` thunk family.
- `0x8067A520` (232 bytes) and `0x80675960` (252 bytes) are paired-single
  reconstructions (`unaff_GQR0`, `ldexpf` lanes). The integer structure is
  readable but the paired lanes are not recoverable, so `0x8067A520` stays an
  address-only call from [`fn_8067a840`].
- `0x8067A790` (44 bytes) is truncated by bad instruction data
  (`halt_baddata`). `0x8067A900` (100 bytes) is otherwise readable but its first
  call targets that body, so it is deferred rather than guessed.
- `0x80670FA0` (184), `0x80671060` (172), `0x80671510` (248) and `0x80671A80`
  (256) are recoverable Lemmy-ball effect/animation emitters; they were left for
  the next pass only because this lane's budget was spent on the three clusters
  above, not because they were rejected.
- Nothing else in the three runs was judged unreconstructable.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (27 rows, 3,464 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 55 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave82` reports
  `wave82-bases.csv: 27 rows` and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs` (the parallel lanes' files reported their own errors).
