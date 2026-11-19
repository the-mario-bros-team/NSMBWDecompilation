# Coverage - Wave 80 (`game::bases`, `0x806E` bucket)

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
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. In the tables below,
"role" is the plain-English behaviour, and confidence is `high` when guards,
stores, constants, strings and call arguments are literal, `medium` when the
role of a field or callee had to be inferred (those bodies carry `// observed:`,
`// inferred:` or `// unknown:` notes), and `low` when only the instruction
sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 3,012 | 19 | 9 |

No body in this pass is `low` confidence.

## What this pass covers

The `0x806E` bucket held 229 functions at lane start; 84 were already in
`ledger/functions.tsv` when this pass began, leaving 145. This pass takes the
`lift_suisha` rider-lift tail, the `lift_zen_hammer` constructor pair, and the
`lift_zen_kaihei`/`lift_zen_shiro` member tail.

- **`lift_suisha` rider-lift tail, `0x806E20B0..0x806E2FD0` (19 bodies).** The
  class's model loader [`fn_806e2320`] and plank factory [`fn_806e3480`] were
  translated in an earlier wave; this pass completes the state tail around
  them. The actor owns four inline `0xE0`-byte planks from `+0x430` to `+0x7B0`
  and keeps its spawn position at `+0x8A8..+0x8B0`, the plank angle at `+0x8C4`
  and the tilt at `+0x8C6`.
  - Plank lifecycle: the record reset [`fn_806e20b0`], the destructor
    [`fn_806e2100`] and the four per-plank loops (motion reset
    [`fn_806e2510`], offset apply [`fn_806e2560`], motion step plus offset
    [`fn_806e25d0`], sub-object teardown [`fn_806e2660`]) plus the previous
    position copy [`fn_806e27a0`] and the stop loop [`fn_806e2ca0`].
  - Rider placement: the occupancy count [`fn_806e2830`], the sign-selected
    angle pick [`fn_806e2890`], the angle quantiser [`fn_806e27d0`] and the
    ground probes [`fn_806e2430`]/[`fn_806e2b10`].
  - Presentation: the stage start [`fn_806e2180`], the collision/previous
    position setup [`fn_806e2460`], the matrix writer [`fn_806e2d00`], the
    tilt decay [`fn_806e2e10`] and the spawn-position reset [`fn_806e2fd0`].

- **`lift_zen_hammer`, `0x806E4330..0x806E44B0` (2 bodies).** The `0x5E0`-byte
  lift actor whose loader is the already translated [`fn_806e4540`]
  (`g3d/lift_zen_hammer.brres` into `+0x3EC`/`+0x408`/`+0x40C`). The
  constructor [`fn_806e44b0`] installs vtable `0x807FCF88`, the accessor
  `+0x3EC`, the model holder `+0x40C` and the four-controller array `+0x418`
  (`0x802DC9A0`, `0xE4` bytes, two callbacks); the release helper
  [`fn_806e4330`] wraps `0x8015F800` plus the optional `0x802B9280` free.

- **`lift_zen_kaihei` / `lift_zen_shiro` tail, `0x806E4E90..0x806E5DC0`
  (7 bodies).** The kaihei class whose model/clip loader is the already
  translated [`fn_806e4da0`] gains the clip play [`fn_806e4e90`], the
  rider-axis position save [`fn_806e52a0`] and apply [`fn_806e52d0`], the
  packed-flag state selector [`fn_806e5300`] and the per-frame composite
  [`fn_806e5680`]. Two neighbouring `lift_zen_shiro` bodies follow: the actor
  kind search [`fn_806e5d60`] and the spawn-descriptor copy
  [`fn_806e5dc0`].

Recurring layout facts used by this pass: `+0x60` is the class vtable, `+0x7C`
the NW4R transform matrix, `+0xAC`/`+0xB0`/`+0xB4` the position,
`+0x100`/`+0x102`/`+0x104` the rotation angles, `+0x394` the state interface
object, `+0x3D0` the resource accessor, `+0x3F0` the model holder and `+0x3FC`
the clip. Transform writers run `0x801C0BD0` (build matrix),
`0x8016ECB0`/`0x8016EDD0`/`0x8016EEF0` (apply angles), `0x8016A170` (bind
matrix), `0x8016A150` (bind scale) and `0x8016A1A0` (publish).

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x806E20B0` | `fn_806e20b0` | 72 | medium | plank record reset |
| `0x806E2100` | `fn_806e2100` | 128 | high | plank destructor |
| `0x806E2180` | `fn_806e2180` | 236 | medium | stage start and state selection |
| `0x806E2430` | `fn_806e2430` | 48 | medium | two-float ground probe |
| `0x806E2460` | `fn_806e2460` | 164 | medium | plank collision box and initial position |
| `0x806E2510` | `fn_806e2510` | 72 | high | per-plank motion reset |
| `0x806E2560` | `fn_806e2560` | 112 | high | per-plank offset apply |
| `0x806E25D0` | `fn_806e25d0` | 132 | high | motion step plus offset apply |
| `0x806E2660` | `fn_806e2660` | 72 | high | per-plank sub-object teardown |
| `0x806E27A0` | `fn_806e27a0` | 40 | high | previous-position copy |
| `0x806E27D0` | `fn_806e27d0` | 88 | high | angle quarter quantiser |
| `0x806E2830` | `fn_806e2830` | 84 | high | occupancy count |
| `0x806E2890` | `fn_806e2890` | 180 | high | sign-selected angle pick |
| `0x806E2B10` | `fn_806e2b10` | 216 | medium | per-plank ground probes |
| `0x806E2C50` | `fn_806e2c50` | 72 | high | per-plank motion reset twin |
| `0x806E2CA0` | `fn_806e2ca0` | 72 | high | per-plank motion stop |
| `0x806E2D00` | `fn_806e2d00` | 272 | medium | matrix writer with plank angle |
| `0x806E2E10` | `fn_806e2e10` | 48 | high | tilt decay |
| `0x806E2FD0` | `fn_806e2fd0` | 172 | medium | spawn position restore |
| `0x806E4330` | `fn_806e4330` | 88 | high | release helper |
| `0x806E44B0` | `fn_806e44b0` | 132 | high | `lift_zen_hammer` constructor |
| `0x806E4E90` | `fn_806e4e90` | 120 | high | clip bind and play |
| `0x806E52A0` | `fn_806e52a0` | 36 | high | rider-axis position save |
| `0x806E52D0` | `fn_806e52d0` | 40 | high | rider-axis offset apply |
| `0x806E5300` | `fn_806e5300` | 112 | high | packed-flag state event |
| `0x806E5680` | `fn_806e5680` | 68 | medium | per-frame composite |
| `0x806E5D60` | `fn_806e5d60` | 96 | medium | actor kind search |
| `0x806E5DC0` | `fn_806e5dc0` | 40 | high | descriptor copy |

## Address range

`0x806E20B0..0x806E5DC0`, four runs with deliberate holes:

- Run 1 covers `0x806E20B0..0x806E2FD0`. Three addresses inside it are not part
  of this pass: `0x806E2320` and `0x806E23E0` were already translated by the
  wave-28 `lift_suisha` pass, and `0x806E26B0` is rejected below.
- Run 2 is the pair `0x806E4330`/`0x806E44B0`; `0x806E3480` and `0x806E4540`
  inside the surrounding range were already translated.
- Run 3 covers `0x806E4E90..0x806E5680`; `0x806E4DA0` inside it was already
  translated.
- Run 4 is the pair `0x806E5D60`/`0x806E5DC0`; `0x806E5DF0` above it was
  already translated.
- Everything else in the bucket (`0x806E0110..0x806E20B0`,
  `0x806E3090..0x806E4330`, `0x806E5680..0x806E5D60`, `0x806E5DF0..0x806EFA90`)
  is untouched.

## Deliberate skips

- `0x806E2950` (440 bytes, rider scan and commit) and `0x806E2E40` (272 bytes,
  tilt target) are **not** translated. Both depend on the callee's
  out-parameter, but the export models it as the low half of a 64-bit return
  from `0x802DCD5C` (`*(undefined2 *)uVar12 = ...`) while the caller passes the
  same pointer in `r4`. The write target cannot be assigned to the argument or
  the return half without guessing, so no body was written rather than pick one.
- `0x806E26B0` (228 bytes) — the paired-single angle pass. Its body is a
  GQR1/GQR3 reconstruction with `unaff_GQR0`/`unaff_GQR3` halfword lanes and
  `ldexpf` artefacts; the register state is not recoverable from the export.
  [`fn_806e2180`] calls it as `abi_call1!(0x806E26B0, actor)`.
- `0x806E3090` (728 bytes) — the same actor's per-frame driver. It uses the same
  paired-single idiom plus `CONCAT44(0x43300000, ...)` integer conversions; the
  control flow is readable but the float lanes are not, so it is deferred.
- `0x806E4F10` (28 bytes) — a jump-table dispatch on the `+0x588` state byte.
  Ghidra reports "Could not recover jumptable ... Treating indirect jump as
  call", so the targets are unknown. [`fn_806e5680`] calls it by address and
  the same for its body `0x806E4F30` (880 bytes), which is entered from that
  switch.
- `0x806E2BF0` (96 bytes) is an empty `do { } while` over the plank array with
  no memory access at all. It has no observable behaviour to reconstruct, so it
  was left out instead of emitting a placeholder loop.
- The rest of the bucket is untouched. Nothing else in this range was judged
  unreconstructable.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (28 rows, 3,012 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 84 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave80` reports
  `wave80-bases.csv: 28 rows` and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
