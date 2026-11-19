# Coverage - Wave 118 (`game::bases`)

## How to read this file

This page documents the Wave 118 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

"Confidence" is `high` when the guards, stores, constants and call arguments are
literal, and `medium` when a callee identity, a field role or a lost receiver
had to be inferred. One body is `low` because the export itself lost a jump
table. Comments in the bodies keep `// observed:`, `// inferred:`, `// partial:`
and `// unknown:` markers.

The `0x806b` bucket holds 205 functions. Before this pass 98 were already
translated, leaving 107. This pass takes 30 of them, from `0x806B00F0` to
`0x806BEF30`, in four clusters and one helper tail.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3,068 | 17 | 12 |

One further body ([`fn_806b3bb0`]) is `partial` at `low` confidence, so the
thirty bodies are 29 `translated` + 1 `partial` in the ledger.

## What this pass covers

- **Collision-spawner state machine (`0x806B00F0..0x806B0C90`), 13 bodies.**
  The release wrapper [`fn_806b00f0`]; the record spawner [`fn_806b04e0`] that
  packs two table words and calls actor factory `0x80064610` with type `0x15A`;
  the rectangle scan [`fn_806b05a0`] over players `0..3`; the spawn gate
  [`fn_806b06b0`] keyed by the character byte `+0x349`; the two gate-driven
  transitions [`fn_806b07e0`] / [`fn_806b0900`]; the row picker
  [`fn_806b0a60`] and ten-entry walk [`fn_806b0ad0`] over the `0x78`-stride
  table at `0x807CA698`; the thirty-frame reload [`fn_806b0b90`] and countdown
  [`fn_806b0bb0`]; the two-ring seeder [`fn_806b0c20`]; and the sixteen-column
  walk [`fn_806b0c90`] over the `0xC0`-stride table at `0x807CA800`.
  Shared fields: state interface `+0x394`, wait `+0x3D0`, row `+0x3D4`,
  column `+0x3D8`, delay `+0x3DC`, mask `+0x3E0`, rotation/limit `+0x3E4`,
  armed flag `+0x3E8`, character byte `+0x349`.
- **`0x135C` record helpers (`0x806B3B50..0x806B42D0`), 4 bodies.** The
  type-`0x23A` list walk matching the byte `+0x396` [`fn_806b3b50`], the rebind
  store pair [`fn_806b3bb0`], the selector [`fn_806b3be0`] from `+0x135C`, and
  the refresh [`fn_806b42d0`] that either copies the record's `+0x394`
  halfword to `+0x1356` or sets the gone flag `+0x135A`.
- **Vtable constant getters (`0x806B4D70..0x806B4F40`), 5 bodies.** Four
  twelve-byte slots returning the same main.dol object `0x80377A04`
  ([`fn_806b4d70`], [`fn_806b4d80`], [`fn_806b4d90`], [`fn_806b4da0`]) and one
  returning `2` ([`fn_806b4f40`]).
- **Packed-nibble queries (`0x806B77D0..0x806B7860`), 4 bodies.** The four
  nibble extractors of the flags word at `+4`, clamped at `< 8` for the low
  nibble and `< 2` for the next three.
- **`wendy` ring actor helpers (`0x806BE930..0x806BEF30`), 4 bodies.** The
  five-link `+0x30` child probe with its recursive tail [`fn_806be930`], the
  colon-trimmed name compare [`fn_806bea10`], the exit body [`fn_806beeb0`]
  (three class-vtable slots plus the interface `+0x1C`), and the transform
  writer [`fn_806bef30`].

Start with [`fn_806b05a0`] for the rectangle test, [`fn_806b0ad0`] for the
table walk and [`fn_806bef30`] for the transform sequence.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806B00F0` | `fn_806b00f0` | 88 | high |
| `0x806B04E0` | `fn_806b04e0` | 188 | medium |
| `0x806B05A0` | `fn_806b05a0` | 268 | high |
| `0x806B06B0` | `fn_806b06b0` | 220 | medium |
| `0x806B07E0` | `fn_806b07e0` | 140 | medium |
| `0x806B0870` | `fn_806b0870` | 120 | medium |
| `0x806B0900` | `fn_806b0900` | 168 | medium |
| `0x806B0A60` | `fn_806b0a60` | 92 | high |
| `0x806B0AD0` | `fn_806b0ad0` | 180 | medium |
| `0x806B0B90` | `fn_806b0b90` | 12 | high |
| `0x806B0BB0` | `fn_806b0bb0` | 100 | high |
| `0x806B0C20` | `fn_806b0c20` | 84 | medium |
| `0x806B0C90` | `fn_806b0c90` | 276 | medium |
| `0x806B3B50` | `fn_806b3b50` | 88 | high |
| `0x806B3BB0` | `fn_806b3bb0` | 40 | low |
| `0x806B3BE0` | `fn_806b3be0` | 84 | high |
| `0x806B42D0` | `fn_806b42d0` | 76 | high |
| `0x806B4D70` | `fn_806b4d70` | 12 | high |
| `0x806B4D80` | `fn_806b4d80` | 12 | high |
| `0x806B4D90` | `fn_806b4d90` | 12 | high |
| `0x806B4DA0` | `fn_806b4da0` | 12 | high |
| `0x806B4F40` | `fn_806b4f40` | 8 | high |
| `0x806B77D0` | `fn_806b77d0` | 36 | high |
| `0x806B7800` | `fn_806b7800` | 36 | high |
| `0x806B7830` | `fn_806b7830` | 36 | high |
| `0x806B7860` | `fn_806b7860` | 36 | high |
| `0x806BE930` | `fn_806be930` | 224 | medium |
| `0x806BEA10` | `fn_806bea10` | 136 | medium |
| `0x806BEEB0` | `fn_806beeb0` | 128 | medium |
| `0x806BEF30` | `fn_806bef30` | 156 | medium |

**Totals:** 30 bodies, 3,068 bytes (17 high, 12 medium, 1 low); 1 of the 30 is
`partial`.

## Deliberate skips

Read and rejected, with the reason:

- **`0x802DCBFC` thunks.** `0x806B0210`, `0x806B0240`, `0x806B14A0`,
  `0x806B14D0`, `0x806B26D0`, `0x806B2700`, `0x806B3360`, `0x806B3390`,
  `0x806B4810`, `0x806B4840`, `0x806B6D80`, `0x806B6DB0`, `0x806B8E10`,
  `0x806B8E40`, `0x806BC5B0`, `0x806BC5E0`, `0x806BEAD0`, `0x806BEB00`,
  `0x806BF5D0` and `0x806BF600` only forward two unnamed arguments to the
  unresolved indirect-jump stub `0x802DCBFC`. The module header already records
  that stub as unresolved, so a body here would be a signature guess.
- **Paired-single / GQR quantization bodies.** `0x806B2BB0`, `0x806B2D10`,
  `0x806B3770`, `0x806B4F90`, `0x806B5E00`, `0x806B7340`, `0x806B7690`,
  `0x806B7890`, `0x806B7B70`, `0x806B7CD0` and `0x806B9A30` carry Ghidra's
  `unaff_GQR0`/`unaff_GQR3` reconstruction with `ldexpf` and, in five of them,
  `in_f29..in_f31`/`in_ps29_1..in_ps31_1` payload registers that never reach a
  named parameter. The constants and call sites are visible, but the
  short-to-float conversions cannot be rebuilt without guessing which paired
  slot holds which value.
- **Receiver-less prologue fragments.** `0x806B39B0` and `0x806B5180` recover
  their object through `0x802DCD70`/`0x802DCD6C` save stubs; `0x806B7340`,
  `0x806B7690`, `0x806B7890`, `0x806B7CD0` and `0x806BF1C0` do the same through
  `0x802DCD74`/`0x802DCD68`, and the exports print no owner argument, so the
  receiver would have to be invented.
- **`undefined8` fragment.** `0x806B3AC0` forwards two unnamed `undefined8`
  arguments straight into `0x800BC950`; Ghidra never recovers their type, so the
  parameter list would be a guess.
- **Truncated bodies.** `0x806B1AE0` and `0x806B9720` end in `halt_baddata()`
  ("bad instruction - truncating control flow"), so neither the tail nor the
  meaning of the entry can be recovered. `0x806B9CB0` calls `0x806B9720`
  directly; it was left out with it rather than shipped as a call to a body
  that does not exist.
- **`0x806B3BB0` is partial, not skipped.** The two stores are literal and the
  state change is literal, but the export could not recover the jump table at
  `0x806B3BD4` ("too many branches") and replaced it with a bare indirect call.
  The ledger row says `partial` and the body carries a `// partial:` note.
- **`0x802DCD74`/`0x802DCD70`/`0x802DCD68` saves.** The same compiler
  save/restore family the module header documents; they have no Rust-visible
  effect.
- **Sibling constant getters.** `0x806B4DB0` is a fifth copy of the `0x80377A04`
  getter (four are translated), and `0x806B4EC0` (`0`), `0x806BE840`
  (`0x80357874`), `0x806BF660`, `0x806BF670` (`0x80377A04`), `0x806BC310`
  (`0x8082CB44`) and `0x806BC320` (`0x8082CB04`) are the same shape. They are
  left for a later sweep so this lane stays on the clusters that carry real
  logic; nothing about them is uncertain.
- **The rest of the bucket.** The 77 other untranslated addresses in `0x806b`
  were not translated in this pass; they remain for a later sweep.

## Notes on inference

- No class name survives in these exports. The spawner is identified only by
  its state interface at `+0x394`, its `+0x3D0..+0x3E8` word block and its two
  data tables; the prose never presents it as a named class.
- [`fn_806b04e0`] prints its record as `float *`, yet uses `param_2[2]` as a
  table index. The record is read as the mixed 12-byte entry the offsets show:
  a float at `+0`, an unused word at `+4` and the index at `+8`. The index load
  is `read_i32(record + 8)`, and the doc block says so.
- The two negative table constants in `0x806B0AD0`/`0x806B0C90`
  (`-0x7F835968`, `-0x7F835800`, `-0x7F8357FC`) are the folded bases
  `0x807CA698`, `0x807CA800` and `0x807CA804`; the bodies use the positive
  addresses and keep the arithmetic in the same order as the C.
- [`fn_806b0c90`] contains Ghidra's signed modulo-by-16 sequence
  (`sign * 0x10 | (uint)(sum * 0x10000000 + sign) >> 0x1c) - sign`). It is kept
  verbatim with an `// observed:` note instead of being rewritten as `% 16`,
  because the negative-input behaviour of the two forms is not identical.
- [`fn_806be930`] and [`fn_806bea10`] are byte-for-byte twins of the already
  translated [`fn_80672e20`] and [`fn_80672f00`] at different addresses; both
  doc blocks say so and both keep their own address in the ledger.
- [`fn_806beeb0`] prints its first class-vtable call with an empty argument
  list. A virtual call still receives its receiver in `r3`, so the actor is
  passed and the doc block marks the choice `// inferred:`.
- [`fn_806bef30`] maps `+0x104` to `0x8016EEF0`, `+0x100` to `0x8016ECB0` and
  `+0x102` to `0x8016EDD0`; the same triple already appears in [`fn_80650d40`],
  which is why the Euler slots are named with `medium` confidence rather than
  `low`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 3,068 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 98 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,968 bodies.
- `node tools/check-lane-csv.mjs wave118` reports `OK` and names no problem in
  `wave118-bases.csv`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
