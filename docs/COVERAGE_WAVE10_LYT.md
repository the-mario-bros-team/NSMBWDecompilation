# Coverage — wave 10 NW4R `lyt` pass (`0x80180440..0x8018E660`)

## How to read this file

- **Scope** — the tenth `lyt` translation pass. It adds 25 bodies to the
  semantic module `src/ui/lyt.rs` and creates no staging module.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), not file offsets. The CSV uses the plain hex spelling
  (`80180440`) used by `ledger/functions.csv`.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; each row names its file.
- **Status** follows `docs/RUST_STYLE.md`:
  `translated` = complete control flow and every call accounted for;
  `partial` = flow and major effects present, one operand not literal;
  `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a
  body whose control flow and stores are exact but whose callee roles or
  object fields were inferred from the call shape, or whose original is
  wrapped in paired-single/GQR code.
- Start with the **summary**, then the **function table**. The **cluster
  notes** explain how these bodies connect to the material, font and `RLAN`
  code already in `src/ui/lyt.rs`.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/lyt.rs` | 25 |
| bytes translated | 3,840 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 16 |
| medium confidence | 9 |
| new staging modules | 0 |
| files written | `src/ui/lyt.rs`, this document, `ledger/agents/wave10-lyt.csv` |

Address span: `0x80180440..0x8018E660`. The selection was made after checking
both `ledger/functions.csv` and the destination file: every address below was
absent from both, and every body exists in the Ghidra export. No ledger row was
created from a guess. Ten of the 25 bodies are 8- or 12-byte field accessors;
they are included because they are separate functions in `functions.tsv` with
complete control flow, and because the four key handlers in this pass call
them directly.

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---:|---:|---|
| `0x80180440` | `fn_80180440` | build a 3x4 pane matrix from a rotation/scale record | translated | medium | 280 | `80180440_FUN_80180440.c` |
| `0x801835a0` | `fn_801835a0` | read one texture-SRT entry as width/height floats | translated | medium | 272 | `801835a0_FUN_801835a0.c` |
| `0x80188a00` | `fn_80188a00` | reset a material colour block | translated | high | 908 | `80188a00_FUN_80188a00.c` |
| `0x80188dd0` | `fn_80188dd0` | store the font pointer at `+0x48` | translated | high | 8 | `80188dd0_FUN_80188dd0.c` |
| `0x80188de0` | `fn_80188de0` | load the font pointer from `+0x48` | translated | high | 8 | `80188de0_FUN_80188de0.c` |
| `0x80188d90` | `fn_80188d90` | guarded object release wrapper | translated | high | 64 | `80188d90_FUN_80188d90.c` |
| `0x801898e0` | `fn_801898e0` | select the four text colour channels | translated | high | 384 | `801898e0_FUN_801898e0.c` |
| `0x80189a60` | `fn_80189a60` | read the horizontal scale `+0x24` | translated | high | 8 | `80189a60_FUN_80189a60.c` |
| `0x80189a70` | `fn_80189a70` | read the vertical scale `+0x28` | translated | high | 8 | `80189a70_FUN_80189a70.c` |
| `0x80189a80` | `fn_80189a80` | install horizontal/vertical font scales | translated | medium | 208 | `80189a80_FUN_80189a80.c` |
| `0x80189ca0` | `fn_80189ca0` | read the vertical-layout byte `+0x43` | translated | high | 8 | `80189ca0_FUN_80189ca0.c` |
| `0x80189cb0` | `fn_80189cb0` | read the vertical advance `+0x44` | translated | high | 8 | `80189cb0_FUN_80189cb0.c` |
| `0x80189e00` | `fn_80189e00` | store two floats at `+0x2C`/`+0x30` | translated | high | 12 | `80189e00_FUN_80189e00.c` |
| `0x80189e10` | `fn_80189e10` | store one float at `+0x2C` | translated | high | 8 | `80189e10_FUN_80189e10.c` |
| `0x80189e50` | `fn_80189e50` | read the cursor `+0x2C` | translated | high | 8 | `80189e50_FUN_80189e50.c` |
| `0x80189e60` | `fn_80189e60` | read the line advance `+0x30` | translated | high | 8 | `80189e60_FUN_80189e60.c` |
| `0x8018b0d0` | `fn_8018b0d0` | apply one animation key record (modes 9/10) | translated | medium | 348 | `8018b0d0_FUN_8018b0d0.c` |
| `0x8018b4e0` | `fn_8018b4e0` | advance a text-layout cursor by whole steps | translated | medium | 248 | `8018b4e0_FUN_8018b4e0.c` |
| `0x8018b5e0` | `fn_8018b5e0` | install the key-record vtable `0x8032C8B0` | translated | high | 16 | `8018b5e0_FUN_8018b5e0.c` |
| `0x8018b5f0` | `fn_8018b5f0` | guarded object release wrapper (second copy) | translated | high | 64 | `8018b5f0_FUN_8018b5f0.c` |
| `0x8018b630` | `fn_8018b630` | apply one animation key record (second table) | translated | medium | 348 | `8018b630_FUN_8018b630.c` |
| `0x8018ba40` | `fn_8018ba40` | advance a text-layout cursor (second accessor) | translated | medium | 248 | `8018ba40_FUN_8018ba40.c` |
| `0x8018bca0` | `fn_8018bca0` | line height from font metric `+0x34` | translated | medium | 140 | `8018bca0_FUN_8018bca0.c` |
| `0x8018e560` | `fn_8018e560` | two-step text-layout release | translated | high | 88 | `8018e560_FUN_8018e560.c` |
| `0x8018e660` | `fn_8018e660` | line height (second copy) | translated | medium | 140 | `8018e660_FUN_8018e660.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### Pane transform and texture SRT (`0x80180440..0x801835A0`)

`fn_80180440` is the affine helper behind the pane transform record. It reads
`+0x00`/`+0x04` as translation, scales the angle at `+0x08` by `0.7111111`
(`0x802F5E64`) and calls the two rotation-table functions `0x801885B0` and
`0x80188630`. Their exports are paired-single/GQR code, so they stay opaque and
are called by address. The Rust body writes the same twelve `f32` slots,
including the `-0.5` half-texel terms folded into the translation column.

`fn_801835a0` completes the texture-SRT path started by the already translated
`fn_80188440`. It checks the index against the four-bit count in `+0x50`,
copies the selected `0x20`-byte record from the work buffer at `+0x58`, and
runs the palette conversion `0x801C7160` on the copy. The width and height
come from `0x801C71E0`/`0x801C7200`. The copy is kept because the palette
conversion writes into its argument.

### Material colours (`0x80188A00..0x801898E0`)

`fn_80188a00` resets the whole material colour block. It clears the eight
colour words at `+0x00..+0x1F`, the bytes at `+0x42`/`+0x43`, the manager
globals at `0x80378038`/`0x8037803C`, and then rebuilds the four channel
triples from the two source colours at `+0x18`/`+0x1C`. The selector at `+0x20`
is read four times, once per group, because the original reloads it after the
intervening stores. The alpha byte at `+0x42` scales four bytes in two passes.

`fn_801898e0` is the smaller text-colour selector. It copies two four-byte
colours into `+0x18`/`+0x1C` and `+0x08`, then chooses which one feeds each of
the `+0x0C`, `+0x10` and `+0x14` groups by the selector at `+0x20`. The final
alpha scaling touches `+0x0B`, `+0x0F`, `+0x13` and `+0x17`.

### Font metrics and field accessors (`0x80189A60..0x8018E660`)

`fn_80189a80` divides the requested horizontal and vertical sizes by the font
metrics returned from virtual slots `+0x0C` and `+0x10`, storing the results at
`+0x24`/`+0x28`. `fn_8018bca0` and its duplicate `fn_8018e660` ask the font for
the metric at virtual `+0x34`, multiply it by the scale at `+0x28`, and add the
base at `+0x50`; a null font pointer contributes zero.

The small accessors are intentionally kept as separate bodies because the
binary carries them as separate functions and because they document the
text-layout field map: font pointer `+0x48`, horizontal scale `+0x24`,
vertical scale `+0x28`, cursor `+0x2C`, line advance `+0x30`, vertical-layout
byte `+0x43` and vertical advance `+0x44`.

### Animation key records (`0x8018B0D0..0x8018BA40`)

The four key handlers are two near-duplicate pairs:

- `fn_8018b0d0` mode `10` sums `0x8018BCA0` and `0x80189E60` into `+0x30`;
  mode `9` reads the `+0x54` step count through `0x8018BD80` and advances the
  `+0x2C` cursor. `fn_8018b630` is the same shape with `0x8018E660` and
  `0x8018E740`.
- `fn_8018b4e0` and `fn_8018ba40` are the standalone advance bodies; the
  second differs only in reading `+0x54` through `0x8018E740`.

The step arithmetic is preserved literally: the difference is narrowed to
`f32`, divided by the per-step width, truncated to a signed integer, then
incremented as an unsigned value before being multiplied back in `f32` and
widened for the final store. `fn_8018b5e0` is the constructor that installs the
key-record vtable `0x8032C8B0`.

### Release wrappers (`0x80188D90`, `0x8018B5F0`, `0x8018E560`)

`fn_80188d90` and `fn_8018b5f0` are identical guarded-release bodies around
`0x802B9280`: both require a non-null object and a positive delete flag.
`fn_8018e560` adds the two-step text-layout teardown: it first calls
`fn_80188d90(record, 0)` (which performs no delete because the flag is zero)
and then calls `0x802B9280` itself when its own flag is positive.

## Callee table

| address | role | used by |
|---|---|---|
| `0x801885B0` | sine-like rotation table (opaque, GQR) | `fn_80180440` |
| `0x80188630` | cosine-like rotation table (opaque, GQR) | `fn_80180440` |
| `0x801C7160` | texture palette conversion | `fn_801835a0` |
| `0x801C71E0` | converted-record width word | `fn_801835a0` |
| `0x801C7200` | converted-record height word | `fn_801835a0` |
| `0x802B9280` | object release (`operator delete` shape) | three release wrappers |
| `0x8018BD80` | duration word `+0x54` | `fn_8018b0d0`, `fn_8018b4e0` |
| `0x8018E740` | duration word `+0x54` (duplicate) | `fn_8018b630`, `fn_8018ba40` |

## Unknowns and limits

1. **Rotation-table semantics.** `0x801885B0` and `0x80188630` are only known
   by their tables (`0x8032B5C8`/`0x8032B5D0` and the adjacent
   `0x8032B5CC`/`0x8032B5D4`) and their call order. Their GQR bodies are not
   translated; no symbol names were invented.
2. **Texture-record field roles.** The width/height assignment follows the
   order of the `0x801C71E0`/`0x801C7200` calls. The binary does not name the
   words inside the converted record.
3. **Material selector values.** `fn_80188a00` and `fn_801898e0` preserve the
   literal selector comparisons `0`, `1` and `2`; the meaning of each ordering
   (for example top/bottom/left/right) is not proven.
4. **Key-frame modes.** Only modes `9` and `10` are handled by the four key
   handlers; every other mode returns `0` or is ignored exactly as the C does.
   The step formulas are preserved, including the truncation and the unsigned
   `+1`.
5. **Virtual-call operands.** Ghidra prints the font virtual calls without
   their receiver. The translations pass the font object in `r3`, the standard
   PowerPC member-call convention. This is the same convention documented for
   the wave-7 font metrics.
6. **GQR noise.** The exports of `0x80180440`, `0x801835a0`, `0x80189a80`,
   `0x8018b0d0`, `0x8018b4e0`, `0x8018b630`, `0x8018ba40`, `0x8018bca0` and
   `0x8018e660` contain `unaff_GQR0`/`ldexpf` blocks. Those blocks are
   paired-single register spills and unsigned-to-float quantisation; they do
   not touch target memory. The scalar arithmetic is kept and the noise is
   omitted, as in the other lanes' GQR notes.
7. **Non-finite step counts.** The four key handlers narrow a quotient to a
   signed integer. The PowerPC `fctiwz` result for NaN or infinity is
   implementation-specific; Rust's `as i32` saturates instead. Every finite
   input follows the original truncation, and a zero step width is the only
   way to reach the difference in practice.
8. **Host safety.** All functions touch target addresses through
   `crate::platform`. Running them on a host still requires the platform
   helpers to be backed by an emulator or a mock.

## Verification

```text
cargo check
# Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
# cargo_exit=0; no diagnostic named src/ui/lyt.rs

node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

The final `cargo check` was run after the concurrent lanes settled; the 12
remaining warnings are in other modules (`runtime/bt.rs`, `runtime/input.rs`)
and none names this lane's file. `src/lib.rs`, every `mod.rs`, `Cargo.toml`,
`src/platform.rs`, the generated ledger index and all other lanes' files were
left untouched. No staging module was created.
