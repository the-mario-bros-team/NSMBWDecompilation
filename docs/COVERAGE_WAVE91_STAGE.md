# Coverage - Wave 91 (`game::stage`, the `0x8007xxxx` BG scene run)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column of `ledger/agents/wave91-stage.csv` points at the Ghidra C export each
body was written from. Roles the export only hints at are marked `inferred` or
`unknown` in the body, never here.

This pass is the first to name the small-data block the `0x8007xxxx` BG scene
code reads:

* **`r2` float/double block** (`0x8042BC40..0x8042BE2C`, `SDA2_BASE - 0x7440` ..
  `- 0x722C`): `0.0f`, `1.0f`, `-999999.0f`, the `4096.0f` fixed-point scale,
  the `2^52 + 2^31` signed-integer bias, `32.0f`/`48.0f`/`64.0f`/`72.0f`/
  `80.0f`/`88.0f` and the six aspect-fit ratios `17/14` .. `2.0`.
* **Aspect-fit tables** (`0x802EF658..0x802EF8AC`): five pairs of `f32` tables
  (eleven entries for modes `0/1/6/7`, eight or nine for the others) plus the
  four-entry function table at `0x802EF8B0`.
* **Scene object** (`0x90A50` bytes, published at `r13-0x58B0`): vtable
  `0x8030F490` at `+0x60`, the 0x1C-byte callback block at `+0x900AC`, the
  0x2000-byte collision table at `+0x70`, the two 40-word `-999999.0f` rows at
  `+0x8FECC` (stride `0xA0`), the mode bytes `+0x900FE`/`+0x900FF`, the blend
  factor `+0x90110`, the transition kind `+0x90114`, the four bounds
  `+0x8FE64`/`+0x8FE68`/`+0x8FE6C`/`+0x8FE70` and the texture manager at
  `+0x90A50`.
* **Trigger record** (`[scene+8]`, `[scene+0xC]`, `[scene+0x10]`): flags word,
  two span words and one depth word, all 20.12 fixed point.
* **Tile table** (`r13-0x58A4`): `0x20`-byte entries, tile id at `+0`, kind
  halfword at `+4`, terminated by kind `0x2EB`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 25 | 6360 | 4 | 21 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

Five contiguous runs of the remaining `0x8007xxxx` band, each a cluster that
shares field offsets with the neighbours already translated in wave 83:

- **Trigger-record decoders** (`0x80072DB0..0x80073070`). Three near-identical
  bodies that turn a record pointer (at `+8`, `+0xC`, `+0x10`) into three world
  coordinates, with the bit-0 "square vs ordered span" selector; the third one
  also mirrors x as `-x - 1.0f`.
- **Angle-scan wrappers** (`0x80073B40`, `0x800743D0`). Both walk the same
  "0, +`+0x8FEA0`, -`+0x8FEA0`" probe chain around `0x80073FC0` /`0x80074AF0`.
- **Scene constructor and cell probes** (`0x80077060..0x80077B00`). The
  0x90A50-byte constructor with its 0x8FECC sentinel rows and texture manager,
  the model-cell lookup wrapper pair, the four-unit collision-file loader and
  the zoom-to-scale writer.
- **Per-mode aspect-fit selectors** (`0x80078860..0x800795E0`). The 23-case
  blend of `0x80078860`, the table picker `0x80078D00`, its two clamps
  (`0x80078EF0`, `0x80079060`), the `1.0/ratio` writer `0x800791D0` and the four
  292-byte tabled getters.
- **Draw tail** (`0x8007AB70..0x8007FB10`). The two-column sort/blit pass, the
  two trigger-height clamps, the visible-cell counter and the two `0x8007FA40`
  forwarders.

The public entry points of this pass are [`fn_80077060`] (scene constructor),
[`fn_8007e860`] (visible-cell pass), [`fn_800791d0`] (scene scale) and
[`fn_8007fb10`] (draw-record forwarder).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80072DB0` | `fn_80072db0` | 344 | medium |
| `0x80072F10` | `fn_80072f10` | 344 | medium |
| `0x80073070` | `fn_80073070` | 356 | medium |
| `0x80073B40` | `fn_80073b40` | 128 | medium |
| `0x800743D0` | `fn_800743d0` | 148 | medium |
| `0x80077060` | `fn_80077060` | 808 | high |
| `0x80077520` | `fn_80077520` | 116 | medium |
| `0x80077650` | `fn_80077650` | 148 | medium |
| `0x80077A00` | `fn_80077a00` | 252 | medium |
| `0x80077B00` | `fn_80077b00` | 60 | high |
| `0x80078860` | `fn_80078860` | 316 | medium |
| `0x80078D00` | `fn_80078d00` | 432 | medium |
| `0x80078EF0` | `fn_80078ef0` | 296 | medium |
| `0x80079060` | `fn_80079060` | 356 | medium |
| `0x800791D0` | `fn_800791d0` | 128 | medium |
| `0x80079250` | `fn_80079250` | 292 | medium |
| `0x80079380` | `fn_80079380` | 292 | medium |
| `0x800794B0` | `fn_800794b0` | 292 | medium |
| `0x800795E0` | `fn_800795e0` | 292 | medium |
| `0x8007AB70` | `fn_8007ab70` | 196 | medium |
| `0x8007BBA0` | `fn_8007bba0` | 68 | medium |
| `0x8007BBF0` | `fn_8007bbf0` | 80 | medium |
| `0x8007E860` | `fn_8007e860` | 464 | medium |
| `0x8007FAD0` | `fn_8007fad0` | 52 | high |
| `0x8007FB10` | `fn_8007fb10` | 100 | high |

Address range: `0x80072DB0..0x8007FB10` (five runs, 25 bodies, 6360 bytes).

## Deliberately skipped

Skipped because the body could not be reconstructed honestly:

- `0x80072CB0` (76) - the only real call is `FUN_8006fa40(int, float*)`, whose
  two arguments the export drops (it prints `FUN_8006fa40();`) and whose second
  argument is an out-pointer that never appears in the body, so neither
  argument can be recovered.
- `0x80078670`, `0x800789A0`, `0x80078A70`, `0x80078B40`, `0x80078C10`,
  `0x8007BA70`, `0x8007BC40`, `0x8007C7D0`, `0x8007BD40`, `0x8007E0E0`,
  `0x8007FBF0`, `0x8007FFA0`, `0x8007A6C0`, `0x80079E80` and the rest of the
  GQR band - every one of them builds its floats through `unaff_GQR0`/
  `unaff_GQR3` paired-single blocks with `ldexpf` rounding, so the numeric order
  cannot be stated from the export.
- `0x80070130`, `0x80070530`, `0x80070EF0`, `0x80071060`, `0x80071210`,
  `0x800714D0`, `0x800717A0`, `0x80071A60`, `0x80071DC0`, `0x80072180`,
  `0x800728C0`, `0x80073290`, `0x80073670`, `0x80073BC0`, `0x80073FC0`,
  `0x80074470`, `0x80074AF0`, `0x80075560`, `0x80075820`, `0x80075CA0`,
  `0x80075FD0`, `0x80076530`, `0x800766F0`, `0x800768D0` - the same GQR issue in
  the `0x80070xxx..0x80076xxx` band.
- `0x80077B40` (568) - `FUN_8016a010()` and `FUN_8016a5e0(puVar5 + 2)` are
  printed without their leading arguments, and the `+0x8FEA0`-family packing of
  the `FUN_80077EB0` call cannot be reconstructed from what is shown.
- `0x800775A0` (100) / `0x800777B0` (96) / `0x80077810` (68) / `0x80077730`
  (120) / `0x80077700` (40) - all five are thin wrappers whose argument lists
  are cut: `FUN_80077520()` is printed with no arguments while the wrapper reads
  `in_r7`, `FUN_80077730()` writes its results into stack slots the callers then
  read, and `0x80077700` returns `extraout_r4` (a register left over from the
  callee). Nothing here can be written without inventing the cut arguments.
- `0x80079780` (596), `0x80079BF0` (644), `0x8007AC40` (3632) and the remaining
  `0x8007A6C0..0x8007CD70` band - the large effect passes; all of them mix GQR
  prologues with the `0x8008E310` section walk and deserve a dedicated pass.
- `0x8007E4E0` (780), `0x8007E0E0` (136) - GQR again.
- `0x8007FA40` (132) - the first statement is `uVar2 = extraout_f1` (the value
  the caller left in `f1`) handed straight to `0x8007FBF0` together with
  `in_r10`; neither the `f1` nor the `r10` argument is visible in this body.

## Verification

- Every address and byte size matches `functions.tsv` (25 rows, 6360 bytes).
- Every evidence file exists at the cited path under `work/ghidra/main-gb/`.
- Every row has exactly one body in `src/game/stage.rs`, and each body's
  `# Original` line cites the same address and size.
- No address in this CSV was already in `ledger/functions.csv` (checked against
  the merged ledger before writing; the `0x8007xxxx` band had 105 translated
  addresses and none of them is in this CSV).
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`; the crate's remaining diagnostics belong to
  `src/engine/main_loop.rs`, which another lane owns.
- `node tools/check-lane-csv.mjs wave91` prints `OK`; the wave-91 stage rows are
  clean.
