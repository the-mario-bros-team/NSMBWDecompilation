# Coverage - Wave 96 (`game::bases`, `0x8066` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers
(`0x8035xxxx`, `0x8042xxxx`, `0x807Exxxx`) keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/8066xxxx_FUN_8066xxxx.c` (checked with a
glob, never assumed). "Role" below is the plain-English behaviour. Confidence
is `high` when guards, stores, constants and call arguments are literal, and
`medium` when the role of a field, a global, a callee or a virtual slot had to
be inferred (those bodies carry `// observed:` / `// inferred:` /
`// unknown:` notes). No body in this pass is `low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3,444 | 12 | 18 |

## What this pass covers

The `0x8066` bucket holds 156 functions; 28 were already in
`ledger/functions.csv` when this pass began, leaving 128. This pass takes 30
from six clusters.

- **Four-player placement actor, `0x806629F0..0x80664580` (9 bodies).** The
  `0x490`-byte actor built by [`fn_806629f0`] (vtable `0x807E79A4`):
  [`fn_80662d30`] gathers the active players into `+0x408..+0x414` and
  bubble-sorts them by the `x` coordinate at `+0xAC` (largest first),
  [`fn_80663180`] reads one bit of the global player mask `0x80429CA8`,
  [`fn_80663000`] compares the approach coordinate `+0x3EC` with the chosen
  actor `+0x470`, [`fn_80663210`]/[`fn_80663290`] are the two start gates,
  [`fn_806632d0`]/[`fn_80664050`] toggle the `+0x15C` mode word of the
  `0x80429F10` manager, and [`fn_80664580`] is the start-up that clears the
  mask and enters state `+0x360 = 1`.
- **`0x530`-byte transform actor, `0x80665440..0x80665750` (2 bodies).** The
  constructor with vtable `0x807E7DA8`, collision block `+0x3D0`, transform
  `+0x3F0`, interface `+0x404` (`0x80310ED8`) and the transform writer that
  rebuilds `+0x7C` from the world position and `+0x100..+0x104`.
- **`0x528`-byte transform sibling, `0x80666A60..0x80666E40` (3 bodies).** The
  constructor with vtable `0x807E8108`, the offset writer that applies the
  scale `0x807C8C98` and the offset triple `0x807E80D8..E0`, and the timer
  start for `+0x430`.
- **Ground-state actor, `0x80667D00..0x80668300` (5 bodies).** The probe
  [`fn_80667d00`] (flags `+0x274`, probe `+0x2DC`, returns `0`/`1`/`2`), the
  point emitter [`fn_80667de0`], the material reset [`fn_80667e20`], the frame
  driver [`fn_80667f60`] (sound cue `0x271`) and the generic destructor
  [`fn_80668300`].
- **`lift_zen_kaiten` elevator, `0x80668480..0x80668840` (4 bodies).** The
  `0x3D8`-byte actor with vtable `0x807E85E0`: constructor, matrix writer
  [`fn_80668720`], variant initialiser [`fn_806687a0`] (three parallel
  12-byte-stride tables at `0x807C8D40`/`44`/`48`, the `lift_zen_kaitenA`
  pane through [`fn_80668670`], six `0xE4`-byte objects at `+0x418`) and the
  destructor [`fn_80668840`].
- **Rider/board tail, `0x80669230..0x80669530` (7 bodies).** The slot query
  [`fn_80669230`], the interface/x-span gate [`fn_806692b0`], the slot claim
  [`fn_806693d0`], and the two `0x60`-byte element array at `+0x69C..+0x75C`
  (activate, virtual release, deactivate) plus the per-variant pointer table
  [`fn_80669530`].

Start with [`fn_80662d30`] for the sort and [`fn_806687a0`] for the variant
table walk; [`fn_806693d0`] is the smallest complete body of the pass.

## Address range

`0x806629F0..0x80664580` (9 bodies), `0x80665440..0x80665750` (2),
`0x80666A60..0x80666E40` (3), `0x80667D00..0x80668300` (5),
`0x80668480..0x80668840` (4) and `0x80669230..0x80669530` (7). After this pass
the `0x8066` bucket still has 98 untranslated functions.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806629F0` | `fn_806629f0` | 92 | high |
| `0x80662D30` | `fn_80662d30` | 220 | medium |
| `0x80663000` | `fn_80663000` | 68 | high |
| `0x80663180` | `fn_80663180` | 40 | high |
| `0x80663210` | `fn_80663210` | 116 | medium |
| `0x80663290` | `fn_80663290` | 40 | medium |
| `0x806632D0` | `fn_806632d0` | 60 | medium |
| `0x80664050` | `fn_80664050` | 48 | medium |
| `0x80664580` | `fn_80664580` | 120 | medium |
| `0x80665440` | `fn_80665440` | 164 | high |
| `0x80665750` | `fn_80665750` | 196 | medium |
| `0x80666A60` | `fn_80666a60` | 108 | high |
| `0x80666D00` | `fn_80666d00` | 264 | medium |
| `0x80666E40` | `fn_80666e40` | 40 | high |
| `0x80667D00` | `fn_80667d00` | 200 | medium |
| `0x80667DE0` | `fn_80667de0` | 64 | medium |
| `0x80667E20` | `fn_80667e20` | 84 | medium |
| `0x80667F60` | `fn_80667f60` | 180 | medium |
| `0x80668300` | `fn_80668300` | 88 | high |
| `0x80668480` | `fn_80668480` | 76 | high |
| `0x80668720` | `fn_80668720` | 128 | medium |
| `0x806687A0` | `fn_806687a0` | 160 | medium |
| `0x80668840` | `fn_80668840` | 148 | high |
| `0x80669230` | `fn_80669230` | 124 | medium |
| `0x806692B0` | `fn_806692b0` | 248 | medium |
| `0x806693D0` | `fn_806693d0` | 24 | high |
| `0x806693F0` | `fn_806693f0` | 104 | high |
| `0x80669460` | `fn_80669460` | 96 | medium |
| `0x806694C0` | `fn_806694c0` | 104 | high |
| `0x80669530` | `fn_80669530` | 40 | medium |

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x80662E10`, `0x80662F40` and `0x80663050` are paired-single
  reconstructions: each export contains `unaff_GQR0`, `ldexpf` lane extraction
  and `WARNING: Removing unreachable block`, so the register class and the
  float lanes cannot be proven. This is the same failure documented for
  `0x806A23B0` in wave 95 and `0x806D9FC0` in wave 94.
- `0x80664610` is the 1,292-byte state machine of the same family; its case
  `7` contains a `unaff_GQR0` lane blend for the squared distance, so it was
  left with the three bodies above even though cases `0..6` and `8..12` are
  readable.
- `0x80667CF0` exports as `halt_baddata()` with `WARNING: Control flow
  encountered bad instruction data`, so it carries no reconstructable
  behaviour.
- `0x80667DD0` is a 4-byte alias (`thunk_FUN_80667de0`) of [`fn_80667de0`];
  [`fn_80667f60`] calls it by address, and no separate body was emitted.
- `0x80662990`, `0x806629C0`, `0x806653E0`, `0x80665410`, `0x80668420` and
  `0x80668450` are the 48-byte `0x802DCBFC` forwarders already documented at
  `0x806512F0`/`0x80651320`; they duplicate that unresolved stub and were not
  re-emitted.
- Not reached: the repeated class-name walkers and `:`-suffix comparators
  (`0x806627F0`, `0x80665240`, `0x80666860`, `0x806675E0`, `0x8066BC70`,
  `0x8066F500` and their neighbours) are byte-identical copies of the pattern
  translated at `0x806511D0`; the large bodies `0x80661070` (3,432 bytes),
  `0x806639A0` (1,096), `0x80669560` (916), `0x8066D800`, `0x8066DDC0` and
  `0x8066FA10` were not entered in this pass.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 3,444 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 30 were on the
  bucket's untranslated list before writing.
- `node tools/check-lane-csv.mjs wave96` reports
  `wave96-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; the only errors in the crate at the end of the pass
  belong to the other wave-96 lanes.
