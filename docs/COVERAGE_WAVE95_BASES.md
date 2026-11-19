# Coverage - Wave 95 (`game::bases`, `0x806A` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers
(`0x8035xxxx`, `0x8042xxxx`, `0x807Fxxxx`) keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants and
call arguments are literal, and `medium` when the role of a field, a global, a
callee or a dropped argument had to be inferred (those bodies carry
`// observed:` / `// inferred:` / `// unknown:` notes). No body in this pass is
`low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,808 | 19 | 11 |

## What this pass covers

The `0x806a` bucket holds 158 functions; 29 were already in
`ledger/functions.csv` when this pass began, leaving 129. This pass takes 30 of
them from three clusters.

- **Mounted-rider actor, `0x806A2CA0..0x806A5810` (20 bodies).** The `0xFD0`-byte
  unnamed actor whose constructor is [`fn_806a2e00`] (translated in an earlier
  wave): vtable `0x807F2868` at `+0x60`, position `+0xAC..+0xB4`, facing
  `+0xE8`/`+0xEC`, speeds `+0x114`/`+0x11C`, state `+0x360`, the mounted actor
  pointer `+0x738`, the four-axis motion record `+0xD54..+0xD74`, the
  `0x8007F7A0` timer at `+0xD78` and the flag block `+0xE8C..+0xEA8`. The
  cluster contributes the step/damage dispatch [`fn_806a2ca0`], the destructor
  [`fn_806a2ec0`], the start-up [`fn_806a3100`], the hit reaction
  [`fn_806a33e0`], the per-character radius copy [`fn_806a3570`], the `getin`
  mount [`fn_806a3700`], the detach pair
  [`fn_806a3960`]/[`fn_806a4c20`], the orientation rebuild [`fn_806a3a50`], the
  224-unit tether [`fn_806a3be0`], the knock-off queries
  [`fn_806a3d60`]/[`fn_806a3dd0`], the `ply_change` animation
  [`fn_806a4030`], the overlap scan and its box test
  [`fn_806a4110`]/[`fn_806a42c0`], the two mounted-state steps
  [`fn_806a4410`]/[`fn_806a44a0`], the reset [`fn_806a5340`] and the roar cue
  [`fn_806a5810`].
- **`0x860`-byte sibling destructor, `0x806A9870` (1 body).** The teardown of
  the class built by [`fn_806a9740`] (translated in an earlier wave): the
  `0x18`-stride callback table at `+0x6FC`, the interface pair `+0x6E0`, the
  emitter `+0x6C8`, the three transforms and three model holders, the accessor
  `+0x524` and the actor base.
- **Target seeker, `0x806AF9F0..0x806AFE38` (9 bodies).** The profile-`0x1A3`
  and `0x1A4` actor walks [`fn_806af9f0`]/[`fn_806afa50`], their combiner
  [`fn_806afab0`], the pointer and heading writers
  [`fn_806afaf0`]/[`fn_806afb00`], the re-read [`fn_806afb10`], the adoption
  [`fn_806afb70`], the follow-state step [`fn_806afd00`] and the helper release
  [`fn_806afe10`].

Start with [`fn_806a3700`] for the mount sequence and [`fn_806a42c0`] for the
overlap box; [`fn_806af9f0`] is the clearest small list walk of the pass.

## Address range

`0x806A2CA0..0x806A5810` (mounted-rider actor, 20 bodies),
`0x806A9870` (one destructor) and `0x806AF9F0..0x806AFE38` (target seeker,
9 bodies). After this pass the `0x806a` bucket still has 99 untranslated
functions.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806A2CA0` | `fn_806a2ca0` | 344 | medium |
| `0x806A2EC0` | `fn_806a2ec0` | 124 | high |
| `0x806A3100` | `fn_806a3100` | 572 | medium |
| `0x806A33E0` | `fn_806a33e0` | 280 | medium |
| `0x806A3570` | `fn_806a3570` | 152 | high |
| `0x806A3700` | `fn_806a3700` | 608 | medium |
| `0x806A3960` | `fn_806a3960` | 228 | high |
| `0x806A3A50` | `fn_806a3a50` | 392 | medium |
| `0x806A3BE0` | `fn_806a3be0` | 240 | high |
| `0x806A3CD0` | `fn_806a3cd0` | 140 | high |
| `0x806A3D60` | `fn_806a3d60` | 112 | high |
| `0x806A3DD0` | `fn_806a3dd0` | 40 | high |
| `0x806A4030` | `fn_806a4030` | 212 | high |
| `0x806A4110` | `fn_806a4110` | 420 | medium |
| `0x806A42C0` | `fn_806a42c0` | 328 | high |
| `0x806A4410` | `fn_806a4410` | 136 | medium |
| `0x806A44A0` | `fn_806a44a0` | 212 | medium |
| `0x806A4C20` | `fn_806a4c20` | 112 | high |
| `0x806A5340` | `fn_806a5340` | 136 | medium |
| `0x806A5810` | `fn_806a5810` | 80 | high |
| `0x806A9870` | `fn_806a9870` | 288 | high |
| `0x806AF9F0` | `fn_806af9f0` | 88 | high |
| `0x806AFA50` | `fn_806afa50` | 88 | high |
| `0x806AFAB0` | `fn_806afab0` | 64 | medium |
| `0x806AFAF0` | `fn_806afaf0` | 12 | high |
| `0x806AFB00` | `fn_806afb00` | 16 | high |
| `0x806AFB10` | `fn_806afb10` | 84 | high |
| `0x806AFB70` | `fn_806afb70` | 124 | high |
| `0x806AFD00` | `fn_806afd00` | 136 | medium |
| `0x806AFE10` | `fn_806afe10` | 40 | high |

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x806A23B0`, `0x806A24F0`, `0x806A4CA0`, `0x806A53E0`, `0x806A75B0` and
  `0x806A8AA0` are paired-single reconstructions: each export contains
  `unaff_GQR0`, `ldexpf` lane extraction and `WARNING: Removing unreachable
  block`, so the register class and the float lanes cannot be proven. This is
  the same failure documented for `0x806D9FC0` in wave 94.
- `0x806A6A70` and `0x806A6A80` export as `halt_baddata()` with
  `WARNING: Control flow encountered bad instruction data`, so they carry no
  reconstructable behaviour. `0x806A6AA0` calls `0x806A6A70` and stores its
  result, so it was left with them.
- The twenty 48-byte `0x802DCBFC` forwarders in this bucket
  (`0x806A1560`, `0x806A1590`, `0x806A2230`, `0x806A2260`, `0x806A6080`,
  `0x806A60B0`, `0x806A8500`, `0x806A8530`, `0x806A9550`, `0x806A9580`,
  `0x806A9670`, `0x806A96A0`, `0x806AB620`, `0x806AB650`, `0x806AE0D0`,
  `0x806AE100`, `0x806AF150`, `0x806AF180`, `0x806AF860`, `0x806AF890`) are
  the unresolved indirect-jump stub already documented in the wave-3 note; they
  duplicate the two forwarders translated at `0x806512F0`/`0x80651320` and were
  not re-emitted.
- `0x806A9990` sets `+0x6C4` and dispatches a state, but the export contains
  `WARNING: Could not recover jumptable at 0x806A99C4` and
  `0x806A99DC`, so part of the selector is unreconstructed; it was left for a
  pass that can resolve the jump table.
- The repeated class-name walkers (`0x806A13C0`, `0x806A2090`, `0x806A5EE0`,
  `0x806A8360`, `0x806AB480`, `0x806ADF30`, `0x806AEFB0`), their `:`-suffix
  comparators and their `0x8015F800`/`0x802B9280` destructors were not reached;
  they are byte-for-byte copies of the pattern already translated at
  `0x806511D0` and in the wave-7/9 state-table triples.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,808 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 30 were on the
  bucket's untranslated list before writing.
- `node tools/check-lane-csv.mjs wave95` reports
  `wave95-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`. `cargo check` validates the file with a stub
  `crate::platform` front end as well, which type-checks the whole module
  independently of the other lanes.
