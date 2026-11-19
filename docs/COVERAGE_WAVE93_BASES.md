# Coverage - Wave 93 (`game::bases`, `0x8070` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image.
Cross-module pointers (`0x8042xxxx`, `0x8080xxxx`) are `main.dol` data addresses
imported by the module and keep their absolute values.

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
| `src/game/bases.rs` | `game::bases` | 30 | 4,760 | 16 | 14 |

## What this pass covers

The `0x8070` bucket holds 204 functions; 71 were already in
`ledger/functions.csv` when this pass began, leaving 133. This pass takes 30 of
them from two contiguous runs, `0x8070A220..0x8070B3A0` and
`0x8070BAD0..0x8070C7A0`.

- **Sled-brake actor, `0x8070A220..0x8070B3A0` (16 bodies).** A small actor that
  owns the `+0x6D8` source object, the `+0x540`/`+0x544` model holder, the
  `+0x5BC` animation block, the `+0x6AC` brake interface and the `+0x70C`
  sound interface. It contributes the per-frame driver [`fn_8070a220`], the
  transform writer [`fn_8070a3b0`] and its adjusted variant [`fn_8070bce0`],
  the travel-range reset [`fn_8070a4b0`], the `sled_to_L`/`sled_to_R` model bind
  [`fn_8070a570`] with the `Wm_ig_liftbrake` sound pair
  [`fn_8070a610`]/[`fn_8070a680`], the phase query [`fn_8070a6e0`], the anchor
  probe [`fn_8070a740`], the brake step [`fn_8070a7c0`] (six recorded sample
  slots, direction latch and distance-proportional advance), the animation/state
  reset [`fn_8070aab0`], the full destructor [`fn_8070ac80`] and the three
  registered releases [`fn_8070af20`], [`fn_8070b300`], [`fn_8070b340`] and
  [`fn_8070b3a0`] (the last a byte-identical second instantiation).
- **`+0x544` model-lift family, `0x8070BAD0..0x8070C7A0` (14 bodies).** An
  unrelated actor whose animation holder is `+0x544`, model-binding cache
  `+0x5BC`, secondary clip `+0x5C0`, effect interface `+0x8C0`/`+0x9E8` and
  variant byte `+0x348`. It contributes the rail-colour refresh
  [`fn_8070bad0`], the reference-point/axis rebuild [`fn_8070bde0`], the height
  limiter [`fn_8070beb0`] with its target drop [`fn_8070be60`], the
  `Wm_en_landsmoke` contact effect [`fn_8070c340`], the `Wm_ig_bite` frame tick
  [`fn_8070c410`], the `0x564`/`0x565` reports
  [`fn_8070c4f0`]/[`fn_8070c590`], the `Wm_ob_witchcraft` spell
  [`fn_8070c600`], the blend stepper [`fn_8070c6d0`], the basis-column offset
  [`fn_8070c720`] and the two model binds [`fn_8070c060`]/[`fn_8070c7a0`].

Start with [`fn_8070a7c0`] for the sled brake arithmetic and [`fn_8070c410`]
for the normalise-and-emit pattern of the second actor.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8070A220` | `fn_8070a220` | 100 | high |
| `0x8070A3B0` | `fn_8070a3b0` | 156 | high |
| `0x8070A480` | `fn_8070a480` | 40 | high |
| `0x8070A4B0` | `fn_8070a4b0` | 184 | medium |
| `0x8070A570` | `fn_8070a570` | 160 | medium |
| `0x8070A610` | `fn_8070a610` | 112 | high |
| `0x8070A680` | `fn_8070a680` | 88 | high |
| `0x8070A6E0` | `fn_8070a6e0` | 76 | high |
| `0x8070A740` | `fn_8070a740` | 100 | high |
| `0x8070A7C0` | `fn_8070a7c0` | 752 | medium |
| `0x8070AAB0` | `fn_8070aab0` | 104 | high |
| `0x8070AC80` | `fn_8070ac80` | 164 | high |
| `0x8070AF20` | `fn_8070af20` | 88 | high |
| `0x8070B300` | `fn_8070b300` | 64 | medium |
| `0x8070B340` | `fn_8070b340` | 96 | high |
| `0x8070B3A0` | `fn_8070b3a0` | 100 | high |
| `0x8070BAD0` | `fn_8070bad0` | 228 | medium |
| `0x8070BCE0` | `fn_8070bce0` | 244 | high |
| `0x8070BDE0` | `fn_8070bde0` | 128 | medium |
| `0x8070BE60` | `fn_8070be60` | 72 | medium |
| `0x8070BEB0` | `fn_8070beb0` | 196 | medium |
| `0x8070C060` | `fn_8070c060` | 212 | high |
| `0x8070C340` | `fn_8070c340` | 196 | medium |
| `0x8070C410` | `fn_8070c410` | 212 | medium |
| `0x8070C4F0` | `fn_8070c4f0` | 156 | medium |
| `0x8070C590` | `fn_8070c590` | 112 | medium |
| `0x8070C600` | `fn_8070c600` | 204 | medium |
| `0x8070C6D0` | `fn_8070c6d0` | 76 | high |
| `0x8070C720` | `fn_8070c720` | 124 | medium |
| `0x8070C7A0` | `fn_8070c7a0` | 216 | high |

## Address range

`0x8070A220..0x8070B3A0` (the sled-brake actor, 16 bodies) and
`0x8070BAD0..0x8070C7A0` (the `+0x544` model-lift family, 14 bodies). After this
pass the `0x8070` bucket still has 103 untranslated functions.

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x80700F90`, `0x80700FC0`, `0x807017C0`, `0x807017F0`, `0x80704E00`,
  `0x80704E30`, `0x807054E0`, `0x80705510`, `0x80706CF0`, `0x80706D20`,
  `0x80707CE0`, `0x80707D10`, `0x807083F0`, `0x80708420`, `0x8070B040`,
  `0x8070B070`, `0x8070F210` and `0x8070F240` are one-line tails that call
  `0x802DCBFC`, the unresolved indirect-jump stub documented in the wave-3
  note. They carry no reconstructable behaviour.
- `0x8070B550` (668 bytes) is a paired-single reconstruction: the export
  contains `unaff_GQR0`, two `ldexpf` lane extractions and a
  `WARNING: Removing unreachable block (ram,0x8070b7d0)`, so its register class
  cannot be proven. The 0x30-entry vertex-table writes it performs are not
  recoverable from that body.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 4,760 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 71 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave93` reports
  `wave93-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/bases.rs` (the crate still fails on other lanes' files, which are
  outside this lane).
