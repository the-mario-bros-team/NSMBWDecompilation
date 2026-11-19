# Coverage - Wave 110 (`game::bases`, `0x8071` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address on this page is a **static analysis address** under the
canonical synthetic base **`0x80600000`** (produced by `tools/rel2elf.mjs`),
never a runtime address: the Wii REL loader chooses the real load address, so
the numbers only identify positions inside the REL image.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (globbed by address
prefix, never assumed). "Confidence" is `high` when guards, stores, constants
and call arguments are literal, and `medium` when a callee receiver, a field
role or a dropped argument had to be inferred; those bodies carry
`// observed:` / `// inferred:` / `// unknown:` notes. No body in this pass is
`low`.

The bucket holds 159 functions; 42 were already in `ledger/functions.csv` when
this pass began, leaving 117. This pass takes 30 of them, in three families.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,028 | 19 | 11 |

## What this pass covers

- **The `magic_boss_kameck_block` block actor (`0x80710120..0x807128A0`,
  19 bodies).** The range object created at `+0x5B0` by `0x8070F370` owns three
  callbacks: [`fn_80710120`] (the `0xD4` state query plus the `0x51` retreat),
  [`fn_807101f0`] (the `0xD4`-only variant) and [`fn_80710290`] (which adds the
  kind-`1` hand-off to `0x80056370` with id `5` or `6`). The attack start
  [`fn_80710520`] binds the `magic_boss_kameck_block` pane; the per-frame
  [`fn_807105c0`] and the countdown [`fn_807116c0`] drive the `+0x1610` timer
  from the global mode word `0x8082F76C`; [`fn_80710690`] spawns the
  `Wm_ka_change_s`/`_m`/`_l` effect at the record's grid position and
  [`fn_80711270`] switches between the `boss_kameck_block_S`/`_M`/`_L` models
  and the matching `Wm_ka_block_s`/`_m`/`_l` effect. Around them sit the range
  release [`fn_80711550`], the object release wrapper [`fn_80711d10`], the three
  passes over the four `0x60`-byte child slots at `+0x504`
  ([`fn_80712030`], [`fn_80712080`], [`fn_807120e0`]), the player-mask
  collector [`fn_80712130`], the transform writer [`fn_80712610`] and the four
  getters of the main.dol pointer `0x80377A04`
  ([`fn_80712870`]..[`fn_807128a0`]).
- **The `0x80712FF0..0x807137A0` run (3 bodies).** [`fn_80712ff0`] scales the
  `+0x544` model uniformly, rotates the position by the `+0x100..+0x104` angle
  triple, rebuilds the `+0x7C` matrix and merges a rounded scale byte into the
  animation record. [`fn_80713690`] and [`fn_807137a0`] are the twin
  normalised-interval samplers used by the sibling angle bodies; the second one
  reads the three halfwords as signed values.
- **The Morton-pillar tail (`0x80715E20..0x807168F0`, 8 bodies).** These
  continue the `0x7C8`-byte `boss_morton_ashiba` object built by
  [`fn_80715840`]: the model-record writer [`fn_80715e20`], the `16.0` lift and
  binding restart pair [`fn_80716240`]/[`fn_807162e0`], the height-driven scale
  [`fn_807164b0`] (`(600 - h)^2 * 300 / 600^2`, clamped at `15.9`), and the four
  height clamps that arm states `0x8082FAE0`, `0x8082FD20` and `0x8082FDE0`
  ([`fn_807166b0`], [`fn_80716750`], [`fn_80716860`], [`fn_807168f0`]).

Start with [`fn_80710120`] for the range callback shape, [`fn_80710690`] for the
effect spawn with six arguments, and [`fn_807164b0`] for the pillar scale.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80710120` | `fn_80710120` | 208 | high |
| `0x807101F0` | `fn_807101f0` | 156 | high |
| `0x80710290` | `fn_80710290` | 292 | medium |
| `0x80710520` | `fn_80710520` | 148 | high |
| `0x807105C0` | `fn_807105c0` | 192 | medium |
| `0x80710690` | `fn_80710690` | 532 | high |
| `0x80711270` | `fn_80711270` | 676 | medium |
| `0x80711550` | `fn_80711550` | 60 | high |
| `0x807116C0` | `fn_807116c0` | 212 | high |
| `0x80711D10` | `fn_80711d10` | 88 | high |
| `0x80712030` | `fn_80712030` | 72 | high |
| `0x80712080` | `fn_80712080` | 84 | high |
| `0x807120E0` | `fn_807120e0` | 72 | high |
| `0x80712130` | `fn_80712130` | 284 | medium |
| `0x80712610` | `fn_80712610` | 124 | high |
| `0x80712870` | `fn_80712870` | 12 | high |
| `0x80712880` | `fn_80712880` | 12 | high |
| `0x80712890` | `fn_80712890` | 12 | high |
| `0x807128A0` | `fn_807128a0` | 12 | high |
| `0x80712FF0` | `fn_80712ff0` | 316 | medium |
| `0x80713690` | `fn_80713690` | 260 | high |
| `0x807137A0` | `fn_807137a0` | 260 | high |
| `0x80715E20` | `fn_80715e20` | 124 | high |
| `0x80716240` | `fn_80716240` | 140 | high |
| `0x807162E0` | `fn_807162e0` | 80 | high |
| `0x807164B0` | `fn_807164b0` | 216 | medium |
| `0x807166B0` | `fn_807166b0` | 92 | medium |
| `0x80716750` | `fn_80716750` | 108 | medium |
| `0x80716860` | `fn_80716860` | 92 | medium |
| `0x807168F0` | `fn_807168f0` | 92 | medium |

**Totals:** 30 translated, 0 partial, 0 stub (19 high, 11 medium, 5,028 bytes).

## Deliberate skips

Read and rejected, with the reason:

- **GQR / paired-single bodies.** `0x80711590` (272 bytes) opens with an
  `unaff_GQR0` bracket and `ldexpf` calls, and `0x80713250` (244) and `0x80713B00`
  (376) open with both `unaff_GQR0` and `unaff_GQR3` brackets plus `ldexpf`
  conversions. The incoming GQR state cannot be reconstructed from the export, so
  the halfword scaling inside them is lost. Same reason as `0x806E3EB0` in wave
  109 and `0x80790280` in wave 107.
- **`0x802DCBFC` indirect-jump stubs.** `0x80711E30` (48 bytes), `0x80711E60`
  (48), `0x807157B0` (48), `0x807157E0` (48), `0x80717A80` (48) and `0x80717AB0`
  (48) are tail-call trampolines with no recoverable target arguments.
- **`0x807103D0`** (296 bytes) rebuilds the `0x7C` transform from the record
  table and the global timer words `0x8082F760`/`0x8082F774`, then feeds three
  values to the unresolved `0x8070F9C0` math helper and stores only its `x`
  result. The helper's calling convention (a double plus two `float *`) does not
  prove which of the two outputs is written, so the body is not claimed. It stays
  a by-address call in [`fn_807105c0`] and [`fn_807116c0`] with an `// unknown:`
  note.
- **Dropped-`this` mixed bodies.** `0x80716590` (148 bytes) calls
  `FUN_80715ed0()` and then `0x800D8C60` with the receiver dropped; unlike the
  five clamp bodies landed here, the first call's object is not provable from the
  surrounding fields, so it is left for a pass that can read the disassembly.
- **Assignment targets for `0x800955F0`.** The five clamp bodies
  [`fn_807164b0`], [`fn_807166b0`], [`fn_80716750`], [`fn_80716860`] and
  [`fn_807168f0`] all call `func_0x800955f0()` without an argument. They are
  landed as `medium` because the callee demonstrably operates on the same actor
  whose height is compared on the next line; the note is repeated in each body.
- The remaining untranslated functions of the bucket were not opened in this pass
  (budget, not doubt), including the `0x8071D500` and `0x8071E130` giants and the
  `0x8071Fxxx` tail.

## Notes on inference

- The block actor's state descriptors (`0x8082F788`, `0x8082F7C8`, `0x8082F808`,
  `0x8082F848`) are BSS objects: no exported function in this image names them,
  so their roles are described only by the transitions that use them. The
  `0x8082F76C` mode word (values `0..3`) is written by `0x8070F7C0`, the frame
  companion of this family.
- `0x80805EA4` is the record table behind `+0x7B0`: `0x8070F370` computes
  `variant * 0x14 + 0x80805EA4` and guards it with `variant < 0x80805EA0` (29).
- `0x807CCA90` and `0x807CCB88` are the `0x4330000080000000` doubles (2^52 + 2^31)
  used by the PowerPC signed int-to-double idiom, while `0x807CCB98` is the plain
  `0x4330000000000000` (2^52) variant. Where the export keeps the bit
  construction explicit the translation keeps it too, so the provenance stays
  visible.
- The pillar family (cluster 3) is tied to `boss_morton_ashiba` by the
  constructor [`fn_80715840`] and the loader [`fn_80715930`] recorded in earlier
  waves; `+0x670` is the base height and `+0x680` the height armed by
  [`fn_80715f20`].
- `0x80377A04` is outside the REL's data range and therefore a main.dol table;
  the four 12-byte getters return it without dereferencing it.
- All float and double constants are read from their REL addresses with
  `platform::read_f32` / `read_f64`, never inlined as guessed decimals.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,028 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 42 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,743 bodies.
- `node tools/check-lane-csv.mjs wave110` reports 30 rows for
  `wave110-bases.csv` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
