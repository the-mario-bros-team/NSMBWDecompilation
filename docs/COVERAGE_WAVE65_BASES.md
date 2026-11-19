# Coverage - Wave 65 (`game::bases`, `0x8069` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address here is a **static analysis address** from the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader picks the real load address. Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the cited evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores, call arguments and constants are literal and
`medium` when a callee identity, a field role or a lost argument had to be
inferred (the body then carries `// inferred:` / `// unknown:`).

The float constants quoted below were re-read from the raw words in
`work/rels/elf/d_basesNP.elf`, not taken from the decompiler's decimal print:
`0x807C9D00` is `0.0f`, `0x807C9D18`/`0x807C9E10` are `1.0f`,
`0x807C9D1C` is `1.5f`, `0x807C9D20` is `7750.0f`, `0x807C9D28` is `0.0f`,
`0x807C9D2C` is `163.0f`, `0x807C9D30` is `80.0f`, `0x807C9DE4` is `0.0125f`,
`0x807C9DE8` is `0.5f`, `0x807C9DEC` is `-0.5f`. `0x807C9E38` is the high word
of the `2^52 + 2^31` unsigned-int conversion double, which is why `fn_806930f0`
prints a `4503601774854204.0 - ...` subtraction whose real value is `60.0`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3744 | 19 | 11 |

## What this pass covers

- **`Mask_flashlight_obake` spawn tail, `0x80690030..0x806909E0`** — the actor
  whose constructor [`fn_806901b0`] and BRRES loader [`fn_80690430`] were
  already recorded. New bodies: the destructor pair
  [`fn_80690030`] (`0x8015F800` + `0x802B9280`), the short reset/loader
  [`fn_80690570`] (interface slot `+0x8` with `(this, actor, 7)`), the record
  writer [`fn_806905c0`] (position `+0x20..+0x34`, angle `+0x3A`, unit scale
  `+0x3C`/`+0x40`, the lazily initialised `0x8082AF20` scale triple, two
  composed matrices into `+0x74` and the two-node publication through
  `0x8000D500`/`0x801685E0`), the two-interface release [`fn_80690780`], and
  the `+0x4C4` position snapshot with its `+0x5BC` record (`0.0`, `163.0`,
  byte `0x28`) that spawns a child effect [`fn_806909e0`].
- **`daHandLight_c`, `0x80690C50..0x80692230`** — the hand-light actor whose
  state table is [`fn_80690f70`] and whose class string is returned by
  [`fn_80691160`] (`0x80355D0C`). New bodies: the `+0x3D0`-gated transform
  update [`fn_80690c50`] (matrix `+0x7C` into model `+0x40C`, then the base
  update `0x800618B0`), the nine-block destructor [`fn_80690e90`]
  (`+0x590`, `+0x550`, `+0x530`, `+0x4DC`, `+0x4A4`, `+0x478`, `+0x44C`,
  `+0x40C`, `+0x3EC`, base `0x800671B0`, free `0x80162920`), its registered
  twin [`fn_80691100`], the `+0x30` child-chain walk [`fn_80691250`], the
  `:`-suffix name compare [`fn_80691330`], the `0.0125` clamped steppers
  [`fn_80691c30`] (up, ceiling) and [`fn_80691ca0`] (down, floor), the
  interface-plus-angle refresh [`fn_80691f30`], the N-frame fast-forward
  [`fn_80691f90`] (sets `+0x506`, calls vtable `+0xE4` N times), and the four
  `+0x394` state transitions [`fn_80692030`], [`fn_806920b0`],
  [`fn_80692170`] and [`fn_80692230`] (descriptors `0x8082AFD0`,
  `0x8082B010`, `0x8082B050`, `0x8082B090`).
- **Neighbouring `0x8069` helpers, `0x806927A0..0x806948F0`** — the destructor
  [`fn_806927a0`]; the rotation-order-preserving transform writer
  [`fn_80692e80`] (position/angle adjust through `0x80064AF0`, matrix `+0x7C`,
  rotation order Y, X, Z into model `+0x3F0`, scale `+0xDC`, final `0`); the
  unit-scale animation bind [`fn_80693080`]; the 60-frame velocity arm
  [`fn_806930f0`]; the interface/`+0x4AC` retire [`fn_806939f0`]; the
  `0x40`-stride slot stepper [`fn_80693b50`]; the state select
  [`fn_80693ee0`]; the `+0x590`/`+0x594` owner latch with animation bind
  [`fn_80694110`]; the `+0x474` release with its `0x4` flag step
  [`fn_806941a0`]; and the two destructors [`fn_80694860`] (sub-vtable
  `0x80329958` at `+0x1C`) and [`fn_806948f0`] (`0x800DAE00`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80690030` | `fn_80690030` | 88 | high |
| `0x80690570` | `fn_80690570` | 76 | high |
| `0x806905C0` | `fn_806905c0` | 444 | medium |
| `0x80690780` | `fn_80690780` | 80 | high |
| `0x806909E0` | `fn_806909e0` | 152 | high |
| `0x80690C50` | `fn_80690c50` | 132 | high |
| `0x80690E90` | `fn_80690e90` | 224 | high |
| `0x80691100` | `fn_80691100` | 92 | high |
| `0x80691160` | `fn_80691160` | 12 | high |
| `0x80691250` | `fn_80691250` | 224 | medium |
| `0x80691330` | `fn_80691330` | 136 | medium |
| `0x80691C30` | `fn_80691c30` | 112 | high |
| `0x80691CA0` | `fn_80691ca0` | 116 | high |
| `0x80691F30` | `fn_80691f30` | 68 | high |
| `0x80691F90` | `fn_80691f90` | 104 | high |
| `0x80692030` | `fn_80692030` | 96 | medium |
| `0x806920B0` | `fn_806920b0` | 156 | medium |
| `0x80692170` | `fn_80692170` | 156 | medium |
| `0x80692230` | `fn_80692230` | 148 | medium |
| `0x806927A0` | `fn_806927a0` | 88 | high |
| `0x80692E80` | `fn_80692e80` | 220 | high |
| `0x80693080` | `fn_80693080` | 76 | high |
| `0x806930F0` | `fn_806930f0` | 132 | medium |
| `0x806939F0` | `fn_806939f0` | 96 | high |
| `0x80693B50` | `fn_80693b50` | 60 | medium |
| `0x80693EE0` | `fn_80693ee0` | 32 | medium |
| `0x80694110` | `fn_80694110` | 116 | medium |
| `0x806941A0` | `fn_806941a0` | 80 | high |
| `0x80694860` | `fn_80694860` | 140 | high |
| `0x806948F0` | `fn_806948f0` | 88 | high |

## Address range

`0x80690030..0x806948F0`, four short runs: `0x80690030..0x806909E0`,
`0x80690C50..0x80691F90` (interrupted by the skipped `0x806914D0` and
`0x80691860`), `0x80692030..0x806930F0` (interrupted by the skipped
`0x80693060`) and `0x806939F0..0x806948F0`.

At lane start the `0x8069` bucket held 185 functions, 19 of them already
translated. This pass adds 30, leaving 136 for later waves.

## Deliberate skips

- `0x80690150`, `0x80690180`, `0x806913F0`, `0x80691420`, `0x806928C0`,
  `0x806928F0`, `0x80693650`, `0x80693680`, `0x806945E0`, `0x80694610` — the
  adjustor thunks `0x802DCBFC(p2, p2, p1)`. They only re-enter the same
  indirect-jump stub and have no Rust-visible effect, matching the policy
  already used in wave 64.
- `0x806907D0` (120) — the body is guarded by the decompiler stack artifact
  `&stack0x00000000 != (undefined1 *)0x40` and calls `0x801C0500` with an
  argument list Ghidra drops (once with one argument, once with two). The
  `0x4000 - angle` rotation is visible, but the callee contract is not, so no
  honest body is possible.
- `0x806914D0` (132), `0x80691860` (284) and `0x80691D20` (248) — the outlined
  paired-single block (`unaff_GQR3`, `ldexpf`, the `bVar1 == 4 || bVar1 == 6`
  lane split). These are the same failure class recorded for `0x806F0F20` and
  `0x80740370`: the conversion inputs are invented by the decompiler.
  [`fn_80691f30`] still calls `0x80691D20` by address with an `// unknown:`
  note.
- `0x80693060` (20) — truncated by bad instruction data (`halt_baddata()`
  after a single `+0x554 &= 0xFFFFFFFD`). Only one store survives, so the body
  is not reconstructed; [`fn_806930f0`] calls it by address.
- `0x806940A0` (112) — the player-state gate calls `0x80162D00` and
  `0x80162510` with every argument dropped by Ghidra, and the second branch
  tests `+0x556` of an object whose identity is only reachable through the
  dropped argument. Kept for a later pass.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (re-read, not copied from
  prose).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv` (checked across the whole
  `0x8069` bucket before writing).
- `node tools/check-lane-csv.mjs wave65` reports `OK` with
  `wave65-bases.csv: 30 rows`.
- `cargo check --message-format=short` produces no error or warning for
  `src/game/bases.rs`.
