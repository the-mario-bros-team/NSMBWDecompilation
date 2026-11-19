# Coverage — wave 20 NW4R `lyt` implementation pass (`0x801819C0..0x801909F0`)

## How to read this file

- **Scope** — the twentieth `lyt` translation pass. It adds 20 bodies to the
  semantic module `src/ui/lyt.rs` and creates no staging module. The pass
  finishes the NW4R layout band of `0x80180000..0x801C0000` after waves 2, 4,
  5, 7 and 17: the text-writer implementation (`ut::TextWriterBase`), the
  glyph quad emitter, the per-format font GX state, the sine/cosine table
  lookups used by the pane matrix, and the `BRLAN` animation work-buffer
  allocator.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), written as plain hex (`8018d540`) in the CSV and with the
  `0x` prefix in this document and in the Rust doc comments.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; every row names its file.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = complete control
  flow and every call accounted for; `partial` = flow and major effects
  present, one operand not literal; `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a
  body whose control flow, offsets and stores are exact but whose callee
  roles, register arguments or paired-single quantisation were recovered from
  call shapes.
- Start with the **summary**, then the **function table**. The **cluster
  notes** explain how the bodies connect to the wave-17 text writer and to
  the wave-2 `BRLAN` helpers.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/lyt.rs` | 20 |
| bytes translated | 15,368 |
| `translated` / `partial` / `stub` | 20 / 0 / 0 |
| high confidence | 1 |
| medium confidence | 19 |
| new staging modules | 0 |
| files written | `src/ui/lyt.rs`, this document, `ledger/agents/wave20-lyt.csv` |

Address span: `0x801819C0..0x801909F0`. The selection was made after checking
both the merged ledger and the destination file: every address below was
absent from `ledger/functions.csv` and from the `fn_` definitions in
`src/ui/lyt.rs`, and every body exists in the Ghidra export. No ledger row was
created from a guess. `0x8018AAC0` is 12 bytes, below the range lane's
16-byte preference; it is included because wave 7 already calls it through the
`FN_8018AAC0_FONT_INFO` constant and the body is complete.

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---:|---:|---:|---|
| `0x801819C0` | `fn_801819c0` | size and seed the `BRLAN` animation work buffer | translated | medium | 1688 | `801819c0_FUN_801819c0.c` |
| `0x801885B0` | `fn_801885b0` | sine of a 1/256-turn index, table interpolated | translated | medium | 124 | `801885b0_FUN_801885b0.c` |
| `0x80188630` | `fn_80188630` | cosine of a 1/256-turn index, table interpolated | translated | medium | 104 | `80188630_FUN_80188630.c` |
| `0x80188DF0` | `fn_80188df0` | per-format font texture GX state | translated | medium | 2384 | `80188df0_FUN_80188df0.c` |
| `0x80189CC0` | `fn_80189cc0` | advance one glyph and emit its quad state | translated | medium | 316 | `80189cc0_FUN_80189cc0.c` |
| `0x80189E70` | `fn_80189e70` | emit the GX texture state and FIFO word for one glyph | translated | medium | 644 | `80189e70_FUN_80189e70.c` |
| `0x8018AAC0` | `fn_8018aac0` | store the font-information pair at `+0x10`/`+0x14` | translated | high | 12 | `8018aac0_FUN_8018aac0.c` |
| `0x8018CA40` | `fn_8018ca40` | measure a char string into a caller rectangle | translated | medium | 272 | `8018ca40_FUN_8018ca40.c` |
| `0x8018CC80` | `fn_8018cc80` | format and print a char string with the writer clone | translated | medium | 552 | `8018cc80_FUN_8018cc80.c` |
| `0x8018D540` | `fn_8018d540` | measure/draw one char of a line | translated | medium | 988 | `8018d540_FUN_8018d540.c` |
| `0x8018DA40` | `fn_8018da40` | char print loop with tag and glyph placement | translated | medium | 1452 | `8018da40_FUN_8018da40.c` |
| `0x8018DFF0` | `fn_8018dff0` | char alignment wrapper and cursor finaliser | translated | medium | 1260 | `8018dff0_FUN_8018dff0.c` |
| `0x8018F400` | `fn_8018f400` | measure a wide string into a caller rectangle | translated | medium | 272 | `8018f400_FUN_8018f400.c` |
| `0x8018F640` | `fn_8018f640` | format and print a wide string with the writer clone | translated | medium | 552 | `8018f640_FUN_8018f640.c` |
| `0x8018FA40` | `fn_8018fa40` | print a wide string of known length, restore cursor | translated | medium | 340 | `8018fa40_FUN_8018fa40.c` |
| `0x8018FBA0` | `fn_8018fba0` | print a NUL-terminated wide string, restore cursor | translated | medium | 368 | `8018fba0_FUN_8018fba0.c` |
| `0x8018FF00` | `fn_8018ff00` | measure/draw one wide character of a line | translated | medium | 1012 | `8018ff00_FUN_8018ff00.c` |
| `0x80190300` | `fn_80190300` | wide line-rectangle accumulator | translated | medium | 280 | `80190300_FUN_80190300.c` |
| `0x80190420` | `fn_80190420` | wide print loop with tag and glyph placement | translated | medium | 1488 | `80190420_FUN_80190420.c` |
| `0x801909F0` | `fn_801909f0` | wide alignment wrapper and cursor finaliser | translated | medium | 1260 | `801909f0_FUN_801909f0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### A. Text-writer implementation (`0x8018CA40..0x801909F0`)

Wave 17 translated the `CalcStringRect`/`CalcLineWidth`/`CalcStringHeight`
wrappers and the `Printf` formatting wrappers, but left the six bodies they
called by address. This pass adds those bodies and the remaining entry points:

| layer | char | wide |
|---|---|---|
| one line step | [`fn_8018d540`] | [`fn_8018ff00`] |
| print loop | [`fn_8018da40`] | [`fn_80190420`] |
| alignment wrapper | [`fn_8018dff0`] | [`fn_801909f0`] |
| measure into rectangle | [`fn_8018ca40`] | [`fn_8018f400`] |
| format and print | [`fn_8018cc80`] | [`fn_8018f640`] |
| print and restore cursor | — | [`fn_8018fa40`], [`fn_8018fba0`] |

Every body starts from the same 0x60-byte writer record documented in waves
10 and 17: font pointer `+0x48`, horizontal scale `+0x24`, vertical scale
`+0x28`, cursor `+0x2C`/`+0x30`/`+0x34`, letter spacing `+0x4C`, line advance
`+0x50`, glyph source `+0x5C`, vertical flag `+0x43`, vertical advance `+0x44`
and the alignment/state word `+0x58`. Clones are made with
[`copy_text_state`] and released through `0x80188D90` with a zero delete flag.

The character source is `0x802DCBFC`, which returns the next character and
advances a cursor word. The tag processor is `writer + 0x5C` through virtual
`+0x0C` (print loop) or `+0x10` (line step). Glyph widths come from the font
object at `writer + 0x48` through virtual `+0x48`.

### B. Glyph emission and font texture state (`0x80188DF0..0x80189E70`)

- [`fn_80189cc0`] asks the font for one glyph record (virtual `+0x50`), scales
  the signed bytes at record `+0x04`/`+0x06`, and updates the x cursor.
- [`fn_80189e70`] rebuilds the GX texture object when the four cache words
  `0x80378038`/`0x8037803C`/`0x80378040`/`0x80378044` no longer match, then
  issues `0x801C5570(0x80, 0, 4)` and one 32-bit FIFO word at `0xCC008000`.
- [`fn_80188df0`] is the per-format font texture initialiser. It programs the
  same base state for every texture format; the indexed formats (`< 2`) swap
  the final TEV calls for the four `0x801C82F0..0x801C83D0` setups. A
  descriptor that is not `(0, -1)` is delegated to [`fn_8018a100`].

### C. Layout math and font record (`0x801885B0..0x8018AAC0`)

- [`fn_801885b0`]/[`fn_80188630`] are the two GQR-affected lookups that
  [`fn_80180440`] already called by address. The table at `0x8032B5C8` holds
  256 entries of `{sin, cos, dsin, dcos}`; the ELF dump confirms entry 0 is
  `(0, 1, 0.024541, -0.000301)` and entry 64 is `(1, 0, -0.000301,
  -0.024541)`. The index unit is 1/256 turn, exactly the `0.7111111`
  degree-to-index factor at `0x802F5E64`.
- [`fn_8018aac0`] stores the font-information pair at `+0x10`/`+0x14`; wave 7
  already uses it through the `FN_8018AAC0_FONT_INFO` constant.

### D. `BRLAN` work buffer (`0x801819C0`)

[`fn_801819c0`] is the allocator behind the wave-2 section helpers. It checks
every requested count against the packed words at `+0x4C`/`+0x50`, frees the
old buffer at `+0x58`, allocates
`patterns * 0x10 + materials * 0x20 + srt * 0x14 + colours * 4` bytes, rewrites
the packed counts, and seeds the record arrays with `[0, 0, 0, 1, 1]` plus the
five optional fixed records. The fill pattern words are `0x802F5E68` (0.0) and
`0x802F5E6C` (1.0); the five optional records are `[1,1,0,0]`, `-1`,
`[0xE4,0xC0,0xD5,0xEA]`, `[0x77,0,0,0]` and `[1,4,5,0x0F]`.

## Callee table

| address | role | used by |
|---|---|---|
| `0x80188D90` | release a cloned writer record (delete flag 0) | all wrappers |
| `0x80188DE0` | load the font pointer `+0x48` | line steps, print loops |
| `0x80189A60`/`0x80189A70` | horizontal/vertical scale getters | line steps, print loops |
| `0x80189C30` | line height | alignment wrappers |
| `0x80189CA0`/`0x80189CB0` | vertical-layout flag and advance | line steps |
| `0x80189E00`/`0x80189E10`/`0x80189E20`/`0x80189E30`/`0x80189E40` | cursor writes | all text bodies |
| `0x8018D920` | char line-rectangle accumulator (wave 17) | measure wrappers |
| `0x802DCBFC` | next-character parser | line steps, print loops |
| `0x802E15DC`/`0x802E4390` | char/wide `vsnprintf` | format wrappers |
| `0x802E441C` | wide string length | [`fn_8018fba0`] |
| `0x801D5540`/`0x801D5550` | heap alloc/free | [`fn_801819c0`] |
| `0x801C6D90`/`0x801C6FF0`/`0x801C74C0` | GX texture object, LOD, finish | [`fn_80189e70`] |
| `0x801C5570` | GX primitive setup | [`fn_80189e70`] |
| `0x801C8640..0x801C8DC0` | GX state setters (literals in the branch) | [`fn_80188df0`] |

## Unknowns and limits

1. **GQR/paired-single noise.** Every text body in this pass contains
   `unaff_GQR0`/`ldexpf` blocks and `CONCAT44(0x43300000, ...)` integer
   conversions. They are paired-single register-save and unsigned-int
   conversion sequences; the scalar stores and calls are kept.
2. **`va_list` modelling.** [`fn_8018cc80`] and [`fn_8018f640`] are variadic
   in the binary. As in wave 17, the `va_list` is an explicit pointer and the
   `format` operand is the third parameter.
3. **Glyph record layout.** [`fn_80189cc0`] allocates 0x14 bytes for the record
   the font virtual writes. The size and the field offsets are reconstructed
   from the bytes [`fn_80189e70`] reads (`+0x00`, `+0x07`, `+0x08`, `+0x0C`,
   `+0x0E`, `+0x12`) and from the two signed bytes at `+0x04`/`+0x06`. The
   record is not described by a header in this range.
4. **Trig quantisation.** The original sine/cosine bodies convert the index
   through a paired-single 8-bit store. The translation uses the exact index
   (`trunc(value) & 0xFF`) and exact fraction; the table values themselves are
   verified from the ELF, but a bit-exact quantised result is not claimed.
5. **`fn_801819c0` count names.** The object layout names the counts only by
   their bit positions (4/4/4/5/1/3/2 bits in `+0x4C`, 4/4/2 bits in `+0x50`).
   The Rust parameters are `c4..c14` and the doc marks the roles unknown.
6. **Division by texture width.** [`fn_80189e70`] divides by the glyph
   texture width from record `+0x0E` with no zero check. The Rust body returns
   zero for that undefined case rather than trapping on the host.
7. **Return widths.** The alignment and print wrappers return their measured
   width as `f32`; the original leaves the value in an `f64` register after
   summing `f32` rectangle fields, so the Rust return type is a representation
   choice, not a different value model.
8. **Host safety.** All bodies touch target memory through `crate::platform`.
   Running them on a host still requires the platform helpers to be backed by
   an emulator or a mock; the GX FIFO at `0xCC008000` is a target address, not
   host memory.

## Selection method (why these 20)

The layout band `0x80180000..0x8018B000` now has only four untranslated
function bodies; they were deliberately left:

| address | size | why |
|---|---:|---|
| `0x80180780` | 4,484 | the `lyt::AnimationTransform` constructor, a 900-line state machine; too large for a single faithful pass |
| `0x80185870` | 1,092 | five floats arrive in `f27..f31`; Ghidra's signature loses them |
| `0x80185F10` | 360 | wraps `0x80185870` with the same lost float arguments |
| `0x801886A0` | 448 | table `atan2` with paired-single quadrant arithmetic |

Everything else in the band is translated. The 20 bodies above were chosen
because each has complete control flow in one Ghidra file, no unresolved
`r13`/`r2` global, and at most a small, documented ABI reconstruction.

## Verification

```text
cargo check
# Finished `dev` profile [unoptimized + debuginfo] target(s)
# no diagnostic named src/ui/lyt.rs

node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

This pass touched only `src/ui/lyt.rs`, this document and
`ledger/agents/wave20-lyt.csv`. `src/lib.rs`, every `mod.rs`, `Cargo.toml`,
`src/platform.rs`, the generated ledger index and all other lanes' files were
left untouched. No staging module was created.
