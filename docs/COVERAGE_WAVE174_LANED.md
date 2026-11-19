# Coverage - Wave 174 lane D (`game::bases` 384-byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 5 | 1,920 | 3 | 2 |

Five bodies from the 384-byte band, all exactly 384 bytes. Source of the
bodies: `work/wave174/laneD_block.rs`.

## What this pass covers

- **Spawn-cell grid scan (1 body).** `0x80651640` probes a `3 x 20` grid at
  `x = 32.0 + cam[+0x38] + cam[+0x40]`, `z = 8.0 + (probe[1] - probe[3])`,
  stepping `16.0`, and calls the ground query `0x80070BA0(x, z, this[+0x38F])`.
  A cell is free when the query returns `0` or `2`; three consecutive free
  cells accept a cell and write `(x, z - 40.0, this[+0xB4])` to the caller's
  three-float buffer. The probe record comes from
  `0x8008E410(player, this[+0x349], &probe)`, built exactly like the existing
  `0x806F03E0` body.
- **Split height step over two sub-actors (1 body).** `0x80684500` gates on a
  `0x130`-type trigger actor (`read_u16(+0x8)`) and `0x806573B0`, sets its
  `+0x5F8` flag, then measures `mover[+0xB0] - candidate[+0xB0]` for the two
  `+0x69C` entries, accepting candidates in `0.0 ..= 12.0`. Both accepted
  candidates (and the mover) are re-armed through `0x80684FA0`, and the `-0.5`
  accumulator at `+0x678` is advanced through `0x80685050`.
- **Broom-rider reinitialiser (1 body).** `0x806AAB90` copies the spawn words
  `+0x6B0`/`+0x6B4`/`+0x6B8` into the position with a `+32.0` lift,
  rebuilds the `fly` / `fly_broom` bindings at `+0x584`/`+0x5FC`, seeds
  `+0x6C8` through `0x800E81C0`, then overwrites `+0xB4 = -256.0`,
  `+0xDC..+0xE4 = 0.7` and `+0xAC = spawn_x + 272.0`.
- **Swept X-band panel stamp (1 body).** `0x806CC6F0` queries `0x800EB930`
  with `(+0xAC, +0xB0)`, lets a `0x80429DD0 + 0x9002C == 3` scene flag
  override the height with `+0x8FEC4`, and stamps the panel
  `(+0xAC, height, 6500.0)` through `0x806CC620` when the height falls inside
  `[min(+0xBC, +0xB0), max(...))`. Past frame `0xC` two extra panels at
  `x - 56.0` and `x + 56.0` follow.
- **Distinct-rider count (1 body).** `0x806D86E0` walks the `+0x518` list,
  OR-ing the `0x807CB7D0` table bit of each rider's `+0x6C` slot (and of the
  `0x8012DBE0` mount), then counts the four low bits and compares them with
  the `0x80429CA0` player count. Twin of the existing
  [`fn_806cb2f0`]/[`fn_806cc480`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80651640` | `fn_80651640` | 384 | medium |
| `0x80684500` | `fn_80684500` | 384 | medium |
| `0x806AAB90` | `fn_806aab90` | 384 | high |
| `0x806CC6F0` | `fn_806cc6f0` | 384 | high |
| `0x806D86E0` | `fn_806d86e0` | 384 | high |

## Deferred

None.

## Corrections and notes

- Both `0x80651640` and `0x806AAB90` are exported as paired-single bodies with
  `unaff_GQR0`/`ldexpf` noise. Every opcode-59 word in them decodes to plain
  scalar `fadds`/`fsubs`/`fmuls` (`0xEFC0082A` = `fadds f30,f0,f1`,
  `0xEC000072` = `fmuls f0,f0,f1`), and no `psq_l`/`psq_st` in either body
  uses a non-zero GQR. Wave 3 had deferred `0x80651640` as a "rejected
  paired-single reconstruction"; that rejection is superseded.
- `0x806AAB90`'s `+32.0` lift is not a literal: it is
  `0x807CA524 (16.0) * 2.0`, where the `2.0` is produced by the
  `0x43300000` / `xoris r4,r0,0x8000` int→float magic minus
  `0x807CA550 = 4503601774854146.0` (`2^52 + 2^31 + 2`).
- `0x80684500` loses both parameters in the `0x802DCD6C` save thunk. The body
  reads `r3 + 4` and `r4 + 4` directly at `0x80684528`/`0x80684524`, so the
  two receivers are recovered literally (same pattern as the existing
  `fn_806a3700`). The step divides the *running* accumulator
  (`fdivs f30,f30,f0` reuses `f30`), so two accepted candidates divide twice;
  a run with none passes the raw `-0.5`. Both behaviours are reproduced.
- `0x806D86E0`'s tail compares with a signed `cmp`; `0x806D8818`'s
  `neg`/`or`/`rlwinm ...,1,31,31` extracts the sign bit of `-limit | limit`,
  i.e. `(limit != 0)`, and `0x806D8834`'s `rlwinm ...,4,31,31` is bit 28 of
  `this[+4]`. The kind-1 table lookups have no slot bound check, so the body
  uses a wrapping 32-bit index exactly like the `lbzx`.
- `0x806CC6F0`'s two-branch band test uses inclusive lower and strict upper
  bounds in both arms; the `fcmpo`/`cror` folds at `0x806CC7B4`..`0x806CC7E8`
  agree with Ghidra.

## Verification

- The lane-D block type-checks standalone against stubbed helper signatures
  (`rustc --crate-type lib --edition 2021 --emit=metadata`); the temporary
  harness was removed afterwards.
- All five evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset
  was re-read from `work/rels/elf/d_basesNP.elf`.
- Only helpers that already exist in `src/game/bases.rs` are called
  (`fn_806573b0`, `fn_806843b0`, `fn_80684fa0`, `fn_80685050`, `fn_806cc620`,
  `vmethod0`, `call_f32x2_u32`, `call_f32_u32`, `call_f32_u32x2`); `0x806A9F50`
  and `0x800EB930` have no Rust body and go through `abi_call1!`/`abi_call6!`.
