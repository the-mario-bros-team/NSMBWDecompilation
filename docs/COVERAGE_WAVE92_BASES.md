# Coverage - Wave 92 (`game::bases`, `0x8065` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8031xxxx`,
`0x8035xxxx`, `0x8042xxxx`) are `main.dol` data addresses imported by the module
and keep their absolute values.

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
| `src/game/bases.rs` | `game::bases` | 30 | 6,508 | 12 | 18 |

## What this pass covers

The `0x8065` bucket holds 185 functions; 51 were already in
`ledger/functions.csv` when this pass began (the wave-68 head
`0x80650020..0x80653C50` plus the wave-3 and wave-59 bodies), leaving 134.
This pass takes 30 of them from two windows, `0x80654600..0x806556B0` and
`0x80655F20..0x8065A0E0`.

- **Rider-platform actor tail, `0x80654600..0x806556B0` (15 bodies).** This is
  the family that wave 68 opened with `0x80654240`/`0x80654300`; the shared
  layout is `+0x5A0` mover, `+0x394` state interface, `+0x6A0..+0x6A8` state
  bytes and the `+0x684`/`+0x690` orientation block. [`fn_80654600`] emits
  `Wm_en_quicksand` through the `+0x468` interface, [`fn_80654880`] writes the
  per-slot effect anchor from the rider's model transform,
  [`fn_80654960`] ages the five occupancy bytes, [`fn_806549a0`] builds the two
  anchor triples, [`fn_80654a50`] walks the `+0x5B4` rider chain with masks
  `0x40`/`0x20`, and [`fn_80654b90`] maps the landing query to the
  `0x282`/`0x283`/`0x284` effects. Around them sit the constructor
  [`fn_80654e30`] (which registers [`fn_80654240`] through `0x8007FA40`), the
  per-frame driver [`fn_80654f60`], the release [`fn_80655060`], the mover step
  [`fn_806551d0`], the re-entry [`fn_80655270`], the detach
  [`fn_80655300`], the direction/velocity probe [`fn_80655360`], the per-player
  restore [`fn_806555a0`] and the registered callback [`fn_806556b0`].
- **`0x806590B0..0x8065A0E0` motion actor (13 bodies).** An unrelated class
  whose `+0x60` vtable and `+0x394` interface follow the same base-actor
  pattern. It contributes two release instantiations
  ([`fn_806590b0`]/[`fn_80659110`]), the constant getter [`fn_80659170`], the
  `+0x30` chain walk [`fn_80659260`], the `:`-suffix name compare
  [`fn_80659340`], the `0x478`-byte constructor [`fn_80659460`] with vtable
  `0x807E6698`, the matrix rebuild [`fn_80659730`], the volume test
  [`fn_806598c0`] and the four motion bodies
  [`fn_806599d0`]/[`fn_80659b60`]/[`fn_80659d30`]/[`fn_80659ea0`] with the spin
  clamp [`fn_8065a0e0`]. They share the `+0x428..+0x430` velocity triple, the
  `+0x43C`/`+0x100` heading pair and the `+0x434`/`+0x438` countdowns.
- **Two isolated bodies.** [`fn_80655f20`] is the `0x408`-byte allocator with
  vtable `0x807E5E90`; [`fn_80657150`] is a three-word in-place header
  constructor with vtable `0x807E63E8`.

Start with [`fn_80654e30`] for the platform layout and [`fn_80659460`] for the
second actor's constructor shape.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80654600` | `fn_80654600` | 108 | high | `Wm_en_quicksand` effect body |
| `0x80654880` | `fn_80654880` | 216 | medium | per-slot effect anchor |
| `0x80654960` | `fn_80654960` | 64 | high | occupancy aging loop |
| `0x806549A0` | `fn_806549a0` | 168 | medium | two-triple anchor writer |
| `0x80654A50` | `fn_80654a50` | 308 | medium | rider chain pick and status |
| `0x80654B90` | `fn_80654b90` | 672 | medium | three-way landing spawn |
| `0x80654E30` | `fn_80654e30` | 304 | medium | platform constructor |
| `0x80654F60` | `fn_80654f60` | 200 | high | per-frame driver |
| `0x80655060` | `fn_80655060` | 60 | high | animation/mover release |
| `0x806551D0` | `fn_806551d0` | 148 | medium | mover step and snap |
| `0x80655270` | `fn_80655270` | 144 | medium | range re-entry |
| `0x80655300` | `fn_80655300` | 92 | high | detach and flag re-arm |
| `0x80655360` | `fn_80655360` | 576 | medium | direction/velocity probe |
| `0x806555A0` | `fn_806555a0` | 248 | medium | per-player restore |
| `0x806556B0` | `fn_806556b0` | 420 | medium | registered frame callback |
| `0x80655F20` | `fn_80655f20` | 76 | high | `0x408`-byte allocator |
| `0x80657150` | `fn_80657150` | 76 | high | in-place header constructor |
| `0x806590B0` | `fn_806590b0` | 88 | high | release instantiation |
| `0x80659110` | `fn_80659110` | 92 | high | release instantiation (copy) |
| `0x80659170` | `fn_80659170` | 12 | high | `0x80357BF4` getter |
| `0x80659260` | `fn_80659260` | 224 | medium | `+0x30` chain walk |
| `0x80659340` | `fn_80659340` | 136 | medium | `:`-suffix name compare |
| `0x80659460` | `fn_80659460` | 136 | high | `0x478`-byte constructor |
| `0x80659730` | `fn_80659730` | 308 | medium | matrix rebuild |
| `0x806598C0` | `fn_806598c0` | 92 | medium | volume test |
| `0x806599D0` | `fn_806599d0` | 288 | medium | velocity and retire |
| `0x80659B60` | `fn_80659b60` | 300 | medium | fall clamp and tilt |
| `0x80659D30` | `fn_80659d30` | 316 | medium | normalized vector step |
| `0x80659EA0` | `fn_80659ea0` | 460 | medium | tilt clamp and capture |
| `0x8065A0E0` | `fn_8065a0e0` | 176 | high | spin clamp and cue |

## Address range

`0x80654600..0x806556B0` (the rider-platform tail, 15 bodies) and
`0x80655F20..0x8065A0E0` (the second actor plus two isolated bodies, 15 bodies).
After this pass the `0x8065` bucket still has 104 untranslated functions.

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x80655EC0`, `0x80655EF0`, `0x80656C90`, `0x80656CC0`, `0x80659400`,
  `0x8065A810` and `0x8065A840` are one-line tails that call `0x802DCBFC`, the
  unresolved indirect-jump stub documented in the wave-3 note. They carry no
  reconstructable behaviour.
- `0x80656260` (392 bytes) is a paired-single reconstruction
  (`ldexpf`/`unaff_GQR0` artifacts around a `CONCAT44` bias) whose storage class
  cannot be read; `0x806564A0` consumes it and is deferred with it.
- `0x80656210` (72 bytes) is typed `double` yet returns single-precision scene
  arithmetic; the register class of its result is ambiguous, so it is left for a
  pass that can prove the ABI.
- `0x80658140` (280 bytes) has the same paired-single artifacts as `0x80656260`
  and was not attempted.
- `0x80656CF0` (48 bytes) and `0x80656F60` (264 bytes) are the allocator and
  constructor of a third class; the constructor was not reached.
- `0x80657390` (24 bytes) is a single virtual `+0xD4` dispatch that Ghidra
  reports as an unrecovered jump table; it was left untranslated because the
  target list is not visible.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 6,508 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 51 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave92` reports
  `wave92-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/bases.rs` (the crate still fails on other lanes' files, which are
  outside this lane).
