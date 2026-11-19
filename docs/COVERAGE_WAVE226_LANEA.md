# Wave 226, lane A - the two untabled `engine::dsys` bodies

Two bodies that sit between `0x800FC1F0` and `0x800FC390` in `main.dol`:

| address | code extent | closing instruction | body |
|---|---|---|---|
| `0x800FC2D0` | 156 B (`0x2D0..0x36C`) | `bclr 20,0` at `0x800FC368` | write the unit axis vector of a sector code to a three-float destination |
| `0x800FC370` | 16 B (`0x370..0x384`) | `bclr 20,0` at `0x800FC380` | `1` when the current shell byte is `8`, else `0` |

Both are translated; neither is deferred. Neither has a `functions.tsv` row, so
neither is an inventory entry, neither is a ledger size, and both blocks are
written as `# Original (untabled)` rather than as `# Original` with a faked
byte count. The CSV rows carry the seven canonical fields and no size field.

## Extents and the `152` / `20` in the instructions

Measured in `work/dol/main.elf`:

* `0x800FC2D0` - leaf, no `stwu`. The compare chain and five constant-pair
  arms run from `0x800FC2D0` to the `bclr 20,0` at `0x800FC368`, i.e.
  `0x2D0..0x36C` = **156 bytes**. The four bytes at `0x800FC36C` are zero
  alignment padding; `0x800FC370` starts the next body.
* `0x800FC370` - leaf: `lbz` / `addi` / `cntlzw` / `rlwinm` / `bclr`, i.e.
  `0x370..0x384` = **16 bytes**. The twelve bytes at `0x800FC384..0x800FC390`
  are zero alignment padding before the already-translated `0x800FC390`.

The instructions' `152` for the first body is the offset `0x98` of the closing
`bclr` - it stops *at* `0x800FC368` and drops the instruction itself. The `20`
for the second is the 16 code bytes plus the first zero pad word. Both bodies
are leaves, so their own `bclr` belongs to the code; the measured extents are
156 and 16, and no other figure is used in the block, the CSV or this document.

## How they are reached

Neither address has a static reference in `main.dol`:

* every executable `PT_LOAD` segment was scanned instruction by instruction
  (66,348 branch opcodes, both `AA = 0` PC-relative and `AA = 1` absolute
  forms) - no `b`/`bl` targets either address;
* every loadable segment was scanned for the pointer words `0x800FC2D0` /
  `0x800FC370` and for the `lis`/`addi`/`ori` halfword `0xC2D0` - no reference.

So both are unreferenced from main-`DOL` code: either static helpers of a module
that is loaded from a REL, or dead code the linker kept. The translations rest
on the instruction stream alone, and the role notes are marked `// inferred:`.

## What the two bodies are

`0x800FC2D0(param_1, param_2)` is a compare chain on the sector code
(`cmpi cr0,4,r4,5`, then `4`, then `2`, then `3`, then a `b` to the same body as
`2`). Each arm loads the `0.0f` at `0x8042D024` and one unit constant, then
writes three floats:

| code | `+0` | `+4` | `+8` |
|---|---|---|---|
| `5` | `+1.0f` | `0.0f` | `0.0f` |
| `4` | `-1.0f` | `0.0f` | `0.0f` |
| `2` | `0.0f` | `0.0f` | `+1.0f` |
| `3` | `0.0f` | `0.0f` | `-1.0f` |
| other | `0.0f` | `0.0f` | `+1.0f` |

That is the horizontal-direction table of the collision code around
[`fn_800fc1f0`], which returns `2`/`3` for the `z` split and `4`/`5` for the
`x` split. Note that the table's signs are the reverse of the offset that
[`fn_800fc1f0`] classifies (`dx = a - b` positive gives code `4`, and code `4`
writes `-x`), so if the two are a producer/consumer pair the table emits the
opposite direction; the code only proves the mapping, not the pairing.

`0x800FC370()` is the third member of the `0x800FC810` (`== 5`) /
`0x800FC830` (`== 7`) predicate family: `lbz r3,-0x5433(r13)` /
`addi r0,r3,-8` / `cntlzw r0,r0` / `rlwinm r3,r0,27,5,31`, i.e.
`(shell - 8 == 0)` folded to `0`/`1` over `G_SHELL_BYTE_5433_C`
(`0x8042A24D`). Shell `8` is the shell for which `0x800FCE60` uses the
`0x800FC400` bank test instead of the per-slot scan.

## Constants

Nothing new is introduced. Both bodies reuse names that earlier lanes already
defined in `src/engine/dsys.rs`:

* `F_SECTOR_SIGN_B218` = `r2 - 0x605C` = `0x8042D024`, image word `0x00000000`
  (`0.0f`);
* `F_BAND_MAX_D` = `r2 - 0x606C` = `0x8042D014`, image word `0x3F800000`
  (`1.0f`);
* `F_BAND_MIN_D` = `r2 - 0x6070` = `0x8042D010`, image word `0xBF800000`
  (`-1.0f`);
* `G_SHELL_BYTE_5433_C` = `r13 - 0x5433` = `0x8042A24D`;
* the module-local `read_f32` helper and `platform::write_f32`.

The image words were read from `work/dol/main.elf` (program headers of the
loadable `.data` segment), not assumed from the neighbouring docs.

## Placement

`work/wave226/laneA_block.rs` belongs immediately after `fn_800fc1f0` and
before the `0x800FC390` section banner, which keeps the image order.

## Verification

* Block copied into a scratch copy of the crate
  (`%TEMP%\opencode\wave226-lanea-crate`), inserted after `fn_800fc1f0`;
  `cargo check --lib` exits `0`.
* `node tools/check-lane-csv.mjs wave226` in the same scratch copy: the two
  rows pass every check, including the `# Original`/body test. The byte-size
  cross-check does not fire for these addresses because `functions.tsv` has no
  row for them - which is exactly why no size is claimed.
* `work/wave226/laneA_disasm.txt` holds the raw `ppc159.js` output for
  `0x800FC1F0` through `0x800FC390`, i.e. the evidence for both bodies and for
  the padding bytes quoted above.
