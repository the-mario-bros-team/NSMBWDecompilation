# Coverage — wave 17 NW4R `lyt` text-writer pass (`0x80189E20..0x8018F510`)

## How to read this file

- **Scope** — the seventeenth `lyt` translation pass. It adds 22 bodies to the
  semantic module `src/ui/lyt.rs` and creates no staging module. The pass
  continues the `0x80180000..0x801C0000` range after the wave-10 selection; it
  covers the text-writer (`ut::TextWriterBase`) family and the remaining
  text-layout field accessors.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), written as plain hex (`80189e20`) in the CSV and with the
  `0x` prefix in this document and in the Rust doc comments.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; every row names its file.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = complete control flow
  and every call accounted for; `partial` = flow and major effects present, one
  operand not literal; `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a body
  whose clone/measure flow and stores are exact but whose argument registers or
  callee roles were recovered from the call shape, or whose original is
  wrapped in paired-single/GQR register-save noise.
- Start with the **summary**, then the **function table**. The **cluster
  notes** explain how the bodies connect to the font/text code already in
  `src/ui/lyt.rs`.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/lyt.rs` | 22 |
| bytes translated | 5,072 |
| `translated` / `partial` / `stub` | 22 / 0 / 0 |
| high confidence | 8 |
| medium confidence | 14 |
| new staging modules | 0 |
| files written | `src/ui/lyt.rs`, this document, `ledger/agents/wave17-lyt.csv` |

Address span: `0x80189E20..0x8018F510`. The selection was made after checking
both the merged ledger (122 earlier `ui::lyt` rows) and the destination file:
every address below was absent from both, and every body exists in the Ghidra
export. No ledger row was created from a guess. The remaining untranslated
functions in `0x80188XXX..0x8018EXXX` are the two rotation-table bodies
`0x801885B0`/`0x801886A0` (paired-single, opaque by design), the large
`0x80188DF0` GX-state copy, and the `0x8018C000`–`0x8018F000` twins that call
the two print implementations listed in the callee table; those twins were left
for a later pass because their Ghidra signatures lose the first three
parameters entirely.

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---:|---:|---:|---|
| `0x80189E20` | `fn_80189e20` | store one line height at `+0x30` | translated | high | 8 | `80189e20_FUN_80189e20.c` |
| `0x8018A100` | `fn_8018a100` | program default GX TEV state for the font texture | translated | medium | 764 | `8018a100_FUN_8018a100.c` |
| `0x8018BD80` | `fn_8018bd80` | read the animation step count `+0x54` | translated | high | 8 | `8018bd80_FUN_8018bd80.c` |
| `0x8018C3B0` | `fn_8018c3b0` | `printf` a char string into a caller rectangle | translated | medium | 368 | `8018c3b0_FUN_8018c3b0.c` |
| `0x8018C520` | `fn_8018c520` | measure a char string of known length, return width | translated | medium | 320 | `8018c520_FUN_8018c520.c` |
| `0x8018C660` | `fn_8018c660` | measure a NUL-terminated char string, return width | translated | medium | 336 | `8018c660_FUN_8018c660.c` |
| `0x8018C7B0` | `fn_8018c7b0` | measure a char string of known length, return height | translated | medium | 320 | `8018c7b0_FUN_8018c7b0.c` |
| `0x8018C8F0` | `fn_8018c8f0` | measure a NUL-terminated char string, return height | translated | medium | 336 | `8018c8f0_FUN_8018c8f0.c` |
| `0x8018CB50` | `fn_8018cb50` | measure a NUL-terminated char string into a rectangle | translated | medium | 304 | `8018cb50_FUN_8018cb50.c` |
| `0x8018D920` | `fn_8018d920` | accumulate the union of the per-character line rectangles | translated | medium | 276 | `8018d920_FUN_8018d920.c` |
| `0x8018E6F0` | `fn_8018e6f0` | store the horizontal scale at `+0x50` | translated | high | 8 | `8018e6f0_FUN_8018e6f0.c` |
| `0x8018E700` | `fn_8018e700` | store the letter spacing at `+0x4C` | translated | high | 8 | `8018e700_FUN_8018e700.c` |
| `0x8018E720` | `fn_8018e720` | read the letter spacing `+0x4C` | translated | high | 8 | `8018e720_FUN_8018e720.c` |
| `0x8018E740` | `fn_8018e740` | read the step count `+0x54` (duplicate) | translated | high | 8 | `8018e740_FUN_8018e740.c` |
| `0x8018E770` | `fn_8018e770` | store the glyph source pointer at `+0x5C` | translated | high | 8 | `8018e770_FUN_8018e770.c` |
| `0x8018E790` | `fn_8018e790` | read the glyph source pointer `+0x5C` | translated | high | 8 | `8018e790_FUN_8018e790.c` |
| `0x8018ED70` | `fn_8018ed70` | `printf` a wide string into a caller rectangle | translated | medium | 368 | `8018ed70_FUN_8018ed70.c` |
| `0x8018EEE0` | `fn_8018eee0` | measure a wide string of known length, return width | translated | medium | 320 | `8018eee0_FUN_8018eee0.c` |
| `0x8018F020` | `fn_8018f020` | measure a NUL-terminated wide string, return width | translated | medium | 336 | `8018f020_FUN_8018f020.c` |
| `0x8018F170` | `fn_8018f170` | measure a wide string of known length, return height | translated | medium | 320 | `8018f170_FUN_8018f170.c` |
| `0x8018F2B0` | `fn_8018f2b0` | measure a NUL-terminated wide string, return height | translated | medium | 336 | `8018f2b0_FUN_8018f2b0.c` |
| `0x8018F510` | `fn_8018f510` | measure a NUL-terminated wide string into a rectangle | translated | medium | 304 | `8018f510_FUN_8018f510.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### A. Text-writer state and the two implementations (`0x8018C3B0..0x8018F510`)

Every wrapper in this cluster starts from the same 0x60-byte writer state
record documented in the wave-10 notes (font pointer `+0x48`, horizontal scale
`+0x24`, vertical scale `+0x28`, cursor `+0x2C`, line advance `+0x30`,
vertical-layout byte `+0x43`, vertical advance `+0x44`, base `+0x50`). The
wrapper clones the record with [`copy_text_state`] (words `+0x00..+0x3F`, the
halfword and two bytes at `+0x40`/`+0x42`/`+0x43`, words `+0x44..+0x5F`; the
padding byte `+0x41` is skipped exactly as in the C), measures with the clone,
copies the resulting cursor back where the variant requires it, and releases
the clone through `0x80188D90` with a zero delete flag.

Two implementation bodies are shared:

- `0x8018D920` (char) — translated as [`fn_8018d920`]; it walks a string,
  calls `0x8018D540` once per character, and unions the four-float
  rectangles.
- `0x80190300` (wide) — the same shape with a two-byte character stride; not
  translated, called by address.

The `printf` variants format into a 0x100-element scratch buffer. The char
wrapper uses the global scratch pointer at `0x80378060` when it is non-null,
and the wide wrapper uses `0x80378068`; otherwise both fall back to a stack
buffer. The format arguments arrive as a `va_list` pointer, modelled as the
fourth Rust parameter.

The width/height pairs are real duplicates apart from the returned extent:
`right - left` for [`fn_8018c520`]/[`fn_8018c660`]/[`fn_8018eee0`]/
[`fn_8018f020`], and `bottom - top` for [`fn_8018c7b0`]/[`fn_8018c8f0`]/
[`fn_8018f170`]/[`fn_8018f2b0`]. [`fn_8018cb50`] and [`fn_8018f510`] write the
whole rectangle to the caller instead of returning one extent.

### B. GX TEV initialiser (`0x8018A100`)

`0x8018A100` is the only body in this pass that touches GX rather than the text
record. It seeds `0x80378034` from the byte flag at `0x80378030`, then issues 34
state calls. The first call `0x801C8920` takes four floats, a selector and a
pointer to one 4-byte value; the file's `call_f32x4_u32_ptr` helper models that
register layout. All remaining calls take only integer arguments, so they are
issued through `platform::call0..call7`. The literals are copied exactly from
the C export; the GX field names are not known (`// unknown:`).

### C. Text-record accessors (`0x80189E20`, `0x8018BD80`, `0x8018E6F0..0x8018E790`)

The eight small bodies complete the field map already started in wave 10:

| field | writer | reader |
|---|---|---|
| `+0x30` line advance | `fn_80189e20` | `fn_80189e60` (wave 10) |
| `+0x4C` letter spacing | `fn_8018e700` | `fn_8018e720` |
| `+0x50` horizontal scale | `fn_8018e6f0` | — |
| `+0x54` animation step count | — | `fn_8018bd80`, `fn_8018e740` |
| `+0x5C` glyph source pointer | `fn_8018e770` | `fn_8018e790` |

`fn_8018bd80` and `fn_8018e740` are the bodies behind the two calls that
wave 10 had to leave as opaque `platform::call1(FN_8018BD80/FN_8018E740)`
uses in [`fn_8018b0d0`], [`fn_8018b630`], [`fn_8018b4e0`] and
[`fn_8018ba40`].

## Callee table

| address | role | used by |
|---|---|---|
| `0x8018D540` | advance one character and write its rectangle (`988` bytes, not translated) | `fn_8018d920` |
| `0x80190300` | wide twin of the line accumulator (`280` bytes, not translated) | five wide wrappers |
| `0x802DC69C` | char string length | three char wrappers |
| `0x802E441C` | wide string length | three wide wrappers |
| `0x802E15DC` | char `vsnprintf` | `fn_8018c3b0` |
| `0x802E4390` | wide `vsnwprintf` | `fn_8018ed70` |
| `0x801C8920` | GX texture-coordinate/range setter (four `f32`) | `fn_8018a100` |
| `0x801C37C0..0x801C8DC0` | 20 further GX state setters | `fn_8018a100` |

## Unknowns and limits

1. **`va_list` modelling.** `0x8018C3B0` and `0x8018ED70` are variadic in the
   binary. Ghidra drops the `format` and `va_list` operands of the
   `vsnprintf`/`vsnwprintf` calls, so the reconstruction takes an explicit
   `args: usize` pointer. The buffer flow and the clone/measure/release order
   are exact.
2. **Dropped callback prototypes.** `0x8018C520`, `0x8018C660`, `0x8018C7B0`,
   `0x8018C8F0`, `0x8018CB50` and all six wide twins are printed by Ghidra
   without a full prototype. The operands were recovered from the `r3`/`r4`/
   `r5` uses and from the shared `0x8018D920`/`0x80190300` call shape; the
   clone/measure/release sequence is exact.
3. **GQR noise.** The exports of `0x8018A100`, `0x8018D920` and the wide twins
   contain `unaff_GQR0`/`ldexpf` blocks. They are paired-single register-save
   and quantisation noise and do not touch target memory; the scalar stores and
   calls are kept.
4. **GX field names.** None of the 21 GX callees in `fn_8018a100` is named by
   the binary. The translation preserves the literal arguments and marks the
   fields `// unknown:`; it does not invent GX enum names.
5. **Rotation-table bodies.** `0x801885B0` and `0x801886A0` remain untranslated.
   Their paired-single bodies were already documented as opaque in the wave-10
   pass; they are the only neighbours in the `0x80188XXX` band that this pass
   did not take.
6. **Scratch-buffer globals.** `0x80378060` and `0x80378068` live in BSS and are
   only read by the two `printf` wrappers. The translation preserves the
   "global if non-null, stack otherwise" fallback; no writer for those globals
   is in this range.
7. **Host safety.** All bodies touch target memory through `crate::platform`.
   Running them on a host still requires the platform helpers to be backed by
   an emulator or a mock.

## Verification

```text
cargo check
# Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
# 11 warnings, all in src/runtime/bt.rs and src/runtime/input.rs
# no diagnostic named src/ui/lyt.rs

node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

The final `cargo check` was run after the concurrent lanes settled. This pass
touched only `src/ui/lyt.rs`, this document and `ledger/agents/wave17-lyt.csv`.
`src/lib.rs`, every `mod.rs`, `Cargo.toml`, `src/platform.rs`, the generated
ledger index and all other lanes' files were left untouched. No staging module
was created.
