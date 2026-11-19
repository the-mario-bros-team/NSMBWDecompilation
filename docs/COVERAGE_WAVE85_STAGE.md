# Coverage - Wave 85 (`game::stage`, the `0x800Cxxxx` lift-actor / mask band)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column of `ledger/agents/wave85-stage.csv` points at the Ghidra C export each
body was written from. Roles that the export only hints at are marked
`inferred` or `unknown` in the body, never here.

Object layouts observed by this pass:

* **Lift actor** (`0x800C0310..0x800C1810`): model resource at `+0x1C`, the
  three part nodes at `+0x20`/`+0x60`/`+0xA0`, the lift kind at `+0x130`, the
  scale float at `+0x124`, the settled flag at `+0x67`, the vtable at `+0x6C`
  (the `0x800C0D70` constructor installs `0x80316B98`), and the `0x38`-byte
  interpolation block starting at `+0x00` (`+0x40`/`+0x44` are the live
  position, `+0x48`/`+0x4C` the previous one).
* **Lift cell table** (behind `0x80164B60`): count (u16) at `+0`, record array
  at `+4` with stride `0x44`, palette at `+8` (stride 4), and a flags word at
  `+4` whose bit 4 is set by the reset pass. `0x800C0AE0` opens the
  `0x8025FDC0`/`0x8025FD50` iterator over that table.
* **Cave/mask records** (`0x800CAA10..0x800CB1C0`): lift state at `+0x86C`,
  flag bit 0 of `+0x17C`, the cave guard byte at `+0x151`, and the `0x6C`-byte
  strip object built by `0x800CB0F0` (vtable `0x803177E0`, child handle at `+4`,
  source vector at `+8..+0x10`, scale floats at `+0x64`/`+0x68`).
* **dMaskMng auxiliaries** (`0x800CBF60..0x800CD610`): first object vtable
  `0x80317748`, second `0x80317728` with its `+0x14` byte, row record vtable
  `0x80317830` (`+4..+0x18` two copies of the `0x80377DC8` descriptor), and the
  mask-manager record with its two objects at `+0x24`/`+0x28`.
* **Message accessors** (`0x800CDC40..0x800CE020`): the current message object
  lives at `0x80359CB4`; the record kind is its first byte and `0x0E` marks the
  "already reset" state.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 28 | 3528 | 14 | 14 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

Four untranslated runs of the `0x800Cxxxx` band, one cluster at a time:

- **Lift-actor model and cell code** (`0x800C0310..0x800C0D70`). The
  `lift_han_*` model loader that indexes the `0x80316708`/`0x803167C8` name
  tables and binds the L/M/R parts from `0x80316940`; the shadow-draw pass with
  its `0x802F8FF0` virtual filter, draw iterator, scale matrix and submit; the
  palette/layer reset that opens the `0x8025FDC0` iterator and re-keys cell 0;
  the bounds-checked `0x44`-byte cell position writer and its four-byte palette
  twin; the object constructor with its two `0x8015FC90` interfaces; and the two
  64-byte release wrappers.
- **Lift interpolation reset** (`0x800C1810`): six zero words plus the lazily
  seeded eight-word `0x80359440` bias table (`-0.1, 0.1, 0.1, 0.1, -0.1, -0.1,
  0.1, -0.1`).
- **Cave/mask records** (`0x800CAA10..0x800CB1C0`). The state override that
  honours the cave guard, the `cave_bti` archive load, the strip draw and its
  count/size wrapper, the `0x34`-byte mask record with its twelve-word copy, the
  `0x6C`-byte strip constructor, and the save/draw/restore wrapper.
- **Message/mask tail** (`0x800CBF60..0x800CE020`). The two auxiliary mask
  objects (`0x80317748`/`0x80317728`) with their `0x80164FD0` render binders,
  the mask row record, the two release wrappers, the `0x800CD250` refresh pass
  that re-seeds both objects with `(0, 0, 7000)`, and the three message
  accessors.

The public entry points of this pass are [`fn_800c0310`] (lift model loader),
[`fn_800c0bf0`] (cell position writer), [`fn_800caa10`] (lift state override)
and [`fn_800ccd90`] (mask row record).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C0310` | `fn_800c0310` | 292 | high |
| `0x800C0440` | `fn_800c0440` | 244 | medium |
| `0x800C0AE0` | `fn_800c0ae0` | 272 | medium |
| `0x800C0BF0` | `fn_800c0bf0` | 248 | medium |
| `0x800C0CF0` | `fn_800c0cf0` | 128 | high |
| `0x800C0D70` | `fn_800c0d70` | 264 | high |
| `0x800C1070` | `fn_800c1070` | 64 | medium |
| `0x800C10B0` | `fn_800c10b0` | 64 | medium |
| `0x800C1230` | `fn_800c1230` | 84 | medium |
| `0x800C1810` | `fn_800c1810` | 148 | high |
| `0x800CAA10` | `fn_800caa10` | 92 | high |
| `0x800CAB10` | `fn_800cab10` | 32 | medium |
| `0x800CACB0` | `fn_800cacb0` | 112 | medium |
| `0x800CAFD0` | `fn_800cafd0` | 120 | high |
| `0x800CB080` | `fn_800cb080` | 100 | high |
| `0x800CB0F0` | `fn_800cb0f0` | 208 | medium |
| `0x800CB1C0` | `fn_800cb1c0` | 84 | medium |
| `0x800CBF60` | `fn_800cbf60` | 60 | high |
| `0x800CBFA0` | `fn_800cbfa0` | 164 | high |
| `0x800CC250` | `fn_800cc250` | 68 | high |
| `0x800CC2A0` | `fn_800cc2a0` | 164 | high |
| `0x800CCD90` | `fn_800ccd90` | 112 | high |
| `0x800CD0D0` | `fn_800cd0d0` | 64 | medium |
| `0x800CD110` | `fn_800cd110` | 64 | medium |
| `0x800CD610` | `fn_800cd610` | 168 | medium |
| `0x800CDC40` | `fn_800cdc40` | 28 | high |
| `0x800CDC60` | `fn_800cdc60` | 16 | high |
| `0x800CE020` | `fn_800ce020` | 64 | medium |

Address range: `0x800C0310..0x800CE020` (four runs).

## Deliberately skipped

Skipped because the body could not be reconstructed honestly:

- `0x800C0570` (328), `0x800C10F0` (280), `0x800C18B0` (292), `0x800C3A10`
  (188) and `0x800C9680` (688) - all five wrap their arithmetic in
  `unaff_GQR0`/`unaff_GQR3` paired-single blocks. `0x800C0570` and `0x800C3A10`
  also build their `u16`/`u32` arguments through `CONCAT` chains whose byte
  order depends on the live GQR mode. `0x800C3A10` was already deferred by
  wave 71 for the same reason.
- `0x800C10F0` additionally converts through the `0x43300000`/`2^52` double
  trick (`(int)(fVar1 / fVar3) ^ 0x80000000`), which is the usual
  float-to-int rounding idiom but cannot be separated from the GQR prologue.
- `0x800C18B0` loops over `FUN_800c17a0()` with no printed argument; the loop
  bound is `this + 0x38` in steps of 8 but the query's argument registers are
  not recoverable.
- `0x800C1E90..0x800C2690` (the line-state wrappers) - each is a two-line body
  such as `FUN_800c1e90((double)*(float *)(unaff_r2 + -0x6844));` where the
  pointer arguments were dropped by the decompiler, and `0x800C2030` prints no
  arguments at all. `0x800C1A90` (484 bytes), which they call, is a paired-single
  block. The wrappers cannot be grounded without their callees.
- `0x800C9F40` (20) - `do { FUN_800e4e40(); } while (true);`, an intentional
  infinite loop with no return path.
- `0x800C9F90` (208) and `0x800C9A10` - owned by the engine lane; `0x800C9A10`
  is already translated as `crate::engine::frame::fn_800c9a10` and is therefore
  not duplicated here.
- `0x800C75B0` (4772) - the largest untranslated entry of the band; its export
  mixes GQR blocks, `undefined8` returns and unreachable-block warnings, so it
  needs a dedicated pass.
- `0x800CD220` (12 bytes) and the other sub-16-byte getters of the band are
  single loads and stay address calls under the module policy.
- `0x800CDED0` (324) - the message-record reset that `0x800CE020` calls; it
  needs its own layout study and is left for a later pass.

## Verification

- Every address and byte size matches `functions.tsv` (28 rows, 3528 bytes);
  `tools/check-lane-csv.mjs wave85` cross-checks each `# Original` size.
- Every evidence file exists at the cited path under `work/ghidra/main-gb/`.
- Every row has exactly one body in `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv` (the one
  candidate that was, `0x800C9A10`, was removed from the lane).
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`; the crate finishes with only unrelated warnings from
  other modules.
- `node tools/check-lane-csv.mjs wave85` prints `OK`; the wave-85 stage rows are
  clean.
