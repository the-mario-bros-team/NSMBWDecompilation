# Coverage — wave 7 NW4R `lyt` fifth pass (`0x80180020..0x8018E500`)

## How to read this file

- **Scope** — the fifth `lyt` translation pass. It adds 25 functions to the
  semantic module `src/ui/lyt.rs` and creates no staging module.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), not file offsets.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; each row names its file.
- **Status** follows `docs/RUST_STYLE.md`:
  `translated` = full control flow and every call accounted for;
  `partial` = flow and major effects present, one operand not literal;
  `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a
  function whose bytes and control flow are exact but whose callee roles or
  object layout were inferred from call order.
- Start with the **summary**, then the **function table**. The **cluster
  notes** explain how the bodies connect to the wave-5 `Pane`,
  `DrawInfo` and `BRFNT` code already in `src/ui/lyt.rs`.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/lyt.rs` | 25 |
| bytes translated | 5,292 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 16 |
| medium confidence | 9 |
| new staging modules | 0 |
| files written | `src/ui/lyt.rs`, this document, `ledger/agents/wave7-lyt.csv` |

Address span: `0x80180020..0x8018E500`. Every function is at least 16 bytes
and outside the eight runtime-lane exclusions listed in
`docs/COVERAGE_r80180_801c0.md`.

## Function table

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x80180020` | `fn_80180020` | build a half-scale rotated or copied pane transform record | translated | high | `80180020_FUN_80180020.c` |
| `0x80180240` | `fn_80180240` | create `pan1`/`pic1`/`txt1`/`wnd1`/`bnd1` pane objects | translated | high | `80180240_FUN_80180240.c` |
| `0x80180560` | `fn_80180560` | normalise a six-float matrix into `[0.5, 1.0)` | translated | high | `80180560_FUN_80180560.c` |
| `0x80183460` | `fn_80183460` | forward zero to virtual slot `+0x14` | translated | high | `80183460_FUN_80183460.c` |
| `0x80183960` | `fn_80183960` | flip-aware texture-coordinate rectangle | translated | high | `80183960_FUN_80183960.c` |
| `0x801843D0` | `fn_801843d0` | notify a pane's child and material with key/value | translated | high | `801843d0_FUN_801843d0.c` |
| `0x801846A0` | `fn_801846a0` | construct `nw4hbm::lyt::Picture` | translated | medium | `801846a0_FUN_801846a0.c` |
| `0x80184C40` | `fn_80184c40` | install Picture-family vtable `0x8032B418` | translated | high | `80184c40_FUN_80184c40.c` |
| `0x80184C60` | `fn_80184c60` | construct `nw4hbm::lyt::TextBox` | translated | medium | `80184c60_FUN_80184c60.c` |
| `0x801860B0` | `fn_801860b0` | construct `nw4hbm::lyt::Window` | translated | medium | `801860b0_FUN_801860b0.c` |
| `0x801862C0` | `fn_801862c0` | destroy `nw4hbm::lyt::Window` | translated | high | `801862c0_FUN_801862c0.c` |
| `0x80186950` | `fn_80186950` | draw one glyph box through the text material | translated | medium | `80186950_FUN_80186950.c` |
| `0x80188440` | `fn_80188440` | select two texture-coordinate source rectangles | translated | medium | `80188440_FUN_80188440.c` |
| `0x80189740` | `fn_80189740` | copy two four-byte colours into an eight-byte record | translated | high | `80189740_FUN_80189740.c` |
| `0x80189790` | `fn_80189790` | rebuild four colour words from two source blocks | translated | high | `80189790_FUN_80189790.c` |
| `0x80189B50` | `fn_80189b50` | horizontal advance from a text-layout record | translated | medium | `80189b50_FUN_80189b50.c` |
| `0x80189BC0` | `fn_80189bc0` | vertical advance from a text-layout record | translated | medium | `80189bc0_FUN_80189bc0.c` |
| `0x80189C30` | `fn_80189c30` | third font metric from a text-layout record | translated | medium | `80189c30_FUN_80189c30.c` |
| `0x80189E30` | `fn_80189e30` | add a double delta to `f32 +0x2C` | translated | high | `80189e30_FUN_80189e30.c` |
| `0x80189E40` | `fn_80189e40` | add a double delta to `f32 +0x30` | translated | high | `80189e40_FUN_80189e40.c` |
| `0x8018AEA0` | `fn_8018aea0` | map a character code through a glyph range record | translated | high | `8018aea0_FUN_8018aea0.c` |
| `0x8018AF70` | `fn_8018af70` | build one glyph texture record from `TGLP`/`CWDH` | translated | high | `8018af70_FUN_8018af70.c` |
| `0x8018B080` | `fn_8018b080` | install font-texture vtable `0x8032C8F8` | translated | high | `8018b080_FUN_8018b080.c` |
| `0x8018B090` | `fn_8018b090` | conditional release of a font-texture object | translated | high | `8018b090_FUN_8018b090.c` |
| `0x8018E500` | `fn_8018e500` | reset a material and initialise its texture-matrix fields | translated | medium | `8018e500_FUN_8018e500.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### Pane resources and pane subclasses (`0x80180020..0x801862C0`)

`fn_80180240` is the central dispatcher. Its five tag words are the NW4R
resource tags `pan1`, `pic1`, `txt1`, `wnd1` and `bnd1`; the first four map to
the constructors translated here, and `bnd1` calls `0x8017E380` by address
because that body is outside this pass. Object sizes are `0xD4`, `0xEC`,
`0x100`, `0x104` and `0xD4`.

`fn_801846a0`, `fn_80184c60` and `fn_801860b0` all start with the already
translated `fn_801836b0` pane constructor, replace the vtable, and then fill
the subclass fields. Their animation block is built from
`0x802DC9A0`/`0x8017E5D0`/`0x8017E640`/`0x8017E7D0` and
`0x80180780`; those five bodies remain outside this lane and are kept as
addresses. `fn_801862c0` is the matching Window teardown and mirrors the
already translated `fn_801837d0` base destructor.

`fn_801843d0` is the pane-side notification used by the child/material
machinery. It resolves the child through `0x8017DFA0`, stores
`countLeadingZeros(value) >> 5` (exactly `value == 0`) at `node + 0x0E`, asks
the material through virtual `+0x24`, and optionally walks the `+0x14` child
list with virtual `+0x58`.

`fn_80180020`, `fn_80183960` and `fn_80188440` are the geometry helpers around
that tree: a half-scale rotated transform record, a flip-aware texture
coordinate rectangle built on `fn_801845a0`, and a two-corner source selector
that calls `0x801835A0` by address.

### Font metrics, glyph records and material reset (`0x80189740..0x8018E500`)

`fn_8018aea0` is the range mapper called by the already translated
`fn_8018abe0`. Its three modes are the direct offset (`0`), a direct `u16`
table (`1`), and a binary search over 4-byte `{first, last, glyph}` records
(`2`). The original computes the midpoint with a signed-division correction;
that arithmetic is preserved instead of being simplified.

`fn_8018af70` builds the 0x14-byte record consumed by the `TGLP`/`CWDH`
sections of a `BRFNT` font: the sheet image offset, three character bytes,
the filter byte, the sheet size, and the cell origin/extent words.

`fn_80189b50`, `fn_80189bc0` and `fn_80189c30` read a text-layout record's
font object at `+0x48` and call virtual `+0x0C`, `+0x10` and `+0x14`
respectively. Ghidra drops the virtual-call operand; the translations pass
the font object in `r3`, the standard PowerPC member-call convention.

`fn_80189740` copies two four-byte colours; `fn_80189790` selects the source
colour block by a selector and scales the four alpha bytes; `fn_80189e30`
and `fn_80189e40` add a double delta to one `f32` field each;
`fn_8018b080`/`fn_8018b090` install and release the font-texture vtable; and
`fn_8018e500` resets a material through `0x80188A00` before writing the
texture-matrix fields.

### Prologue helpers and virtual-call operands

Ten of the bodies enter through `0x802DCD44..0x802DCD74`
(`__savegprlr_*`). Those helpers only save and restore callee-saved
registers, so the Rust bodies omit them. Ghidra prints the saved `r3:r4`
pair as the helper's return value; where the body uses it, the doc comment
states the `(this, resource)` split.

Two constructors (`fn_80184c60`, `fn_801860b0`) are printed as `void` but
are consumed as objects by `fn_80180240`. Their Rust forms return `text_box`
and `window` because `r3` still holds the object at the end of the original
body. This is the same convention documented for the wave-5 destructors.

## Unknowns and limits

1. **`0x801835A0` is not translated.** `fn_80188440` calls it by address. Its
   GQR/paired-single scaling noise was not reconstructed; the two-float
   output buffer and the three integer arguments come from the C file.
2. **Animation-helper roles are inferred.** `0x8017E5D0`, `0x8017E640`,
   `0x8017E7D0`, `0x8017E870`, `0x8017E920` and `0x8017EF70` are only known
   from their order and argument shapes. No symbol names were invented.
3. **`fn_80184c60` reads `+0xFC` before writing it.** The byte comes from
   the freshly allocated (and therefore uninitialised) object; the original
   does the same, and the expression is preserved. The resulting value is
   always `4` for any input because the original computes
   `(old / 3) * 3 + 1` and then `(step % 3) + 3`.
4. **`fn_8018af70` divides by the `TGLP` cell dimensions.** A zero
   `cell_width` or a zero `cell_width * cell_height` traps in the original;
   the Rust code keeps the same division.
5. **`fn_8018aea0` underflow paths wrap.** Codes below the range start or
   above the range end follow the original wrapping 32-bit arithmetic and can
   read outside the record. The direct and binary-search paths are preserved
   verbatim.
6. **Material layout.** `fn_8018e500` writes `+0x4C..+0x5C`; the exact
   meaning of `+0x54 = 4` and the global word at `0x8037806C` is not proven.
   The reset callee `0x80188A00` remains outside this lane.
7. **Host safety.** All functions touch target addresses through
   `crate::platform`. Running them on a host still requires the platform
   helpers to be backed by an emulator or a mock.

## Verification

```text
cargo check
# Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
# (first run after the edit: 0 errors, no diagnostic from src/ui/lyt.rs)

node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

Final re-runs of the whole crate were blocked by two compile errors in other
lanes (the failing file changed between runs as those lanes were edited; none
of the diagnostics named `src/ui/lyt.rs`). To separate those from this pass,
`src/platform.rs` and `src/ui/lyt.rs` were copied to a scratch crate outside
the repository; that check finished with 0 errors and only dead-code warnings
for symbols whose callers live in other modules.

Only `src/ui/lyt.rs`, this document and `ledger/agents/wave7-lyt.csv` were
written. `src/lib.rs`, every `mod.rs`, `Cargo.toml`, `src/platform.rs`,
generated docs and all other lanes' files were left untouched.
