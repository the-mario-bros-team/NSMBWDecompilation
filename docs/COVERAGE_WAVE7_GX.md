# Coverage wave 7 — GX / render fourth pass (`0x80280000..0x802B0000`)

## How to read this file

- **Scope** — 25 previously untranslated functions from the GX/render band of
  `main.dol`, all now in `src/render/gx_band.rs` (ledger module
  `render::gx_band`). This pass continues the range inventory in
  `docs/COVERAGE_GX_BAND.md`, the second pass in `docs/COVERAGE_WAVE4_GX.md`
  and the third pass in `docs/COVERAGE_WAVE5_GX.md`.
- **What this pass adds** — the NW4R G3D "RAWR" material-resource decoder
  (`0x80283A30..0x802844E0`), the material/animation state constructors and
  release paths (`0x802845E0..0x80287190`), the TEV colour-channel emitter
  `0x8029C080`, and the matrix / orientation helpers
  (`0x80294440..0x802AD520`).
- **Addresses** are virtual `main.dol` addresses, not file offsets. Image base
  is `0x80004000`; the main text segment maps virtual `0x80006780` to file
  offset `0x27C0` in `work/dol/main.elf`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = entry/exit, every
  call and every observable memory/MMIO write is accounted for; `partial` = a
  known piece is missing and marked `// partial:`; `stub` = address only.
- **Confidence** measures the strength of the evidence, not code quality:
  `high` = the raw instructions and the Ghidra C agree, or the raw stream was
  used directly; `medium` = the flow is exact but a callee identity or field
  meaning is inferred; `low` = mostly guesswork.
- **Evidence** means the Ghidra C under `work/ghidra/main-gb/export/`, plus a
  raw-instruction check against `work/dol/main.elf`. Every row names its Ghidra
  file and the raw range.
- **Start here** — [`fn_8029c080`](../src/render/gx_band.rs) is the clearest
  TEV colour emitter in the band; [`fn_802a0a90`](../src/render/gx_band.rs) is
  the angle-to-matrix builder; [`fn_80283a90`](../src/render/gx_band.rs) starts
  the material-resource decoder.
- **Companion ledger** — `ledger/agents/wave7-gx.csv`.

## Summary

| module | functions | bytes (`functions.tsv`) | address span | status | confidence |
|---|---|---:|---|---|---|
| `render::gx_band` | 25 | 5,932 | `0x80283A30..0x802AD520` | 25 translated, 0 partial, 0 stub | 22 high, 3 medium, 0 low |

Band totals after this pass (`work/ghidra/main-gb/export/functions.tsv`,
entries `0x80280000 <= addr < 0x802B0000`):

| Metric | Count |
|---|---:|
| Function entries in band | 345 |
| Translated before wave 7 | 99 |
| Added by wave 7 | 25 |
| Translated now | 124 |
| Coverage of entries | 35.9 % |
| Deferred | 221 |
| Deferred because of paired-single/GQR (`G`) | 85 |
| Deferred because entry `< 16` bytes (`S`) | 25 |
| Deferred GX-FIFO state code, no GQR (`F`) | 1 |
| Deferred for other scope reasons (`R`) | 110 |

None of the 25 entries contains `ldexpf(` or `unaff_GQR0`; all were previously
in the `R` bucket. The `G`, `S` and `F` counts are unchanged from
`docs/COVERAGE_GX_BAND.md`.

## Function table

Evidence paths are relative to `work/ghidra/main-gb/export/`.

| address | size | Rust function | role | status | confidence | evidence |
|---|---|---:|---|---|---|---|
| `0x80283A30` | 36 | `fn_80283a30` | advance the RAWR parser LCG; return high 16 bits | translated | high | `80283a30_FUN_80283a30.c` |
| `0x80283A60` | 36 | `fn_80283a60` | resolve a kind 0/1 self-relative chunk offset | translated | high | `80283a60_FUN_80283a60.c` |
| `0x80283A90` | 116 | `fn_80283a90` | validate a RAWR header and store two views | translated | high | `80283a90_FUN_80283a90.c` |
| `0x80283B10` | 136 | `fn_80283b10` | look up a 12-byte RAWR entry and resolve its payload | translated | high | `80283b10_FUN_80283b10.c` |
| `0x80283BA0` | 32 | `fn_80283ba0` | resolve one sub-chunk pointer, skipping 8 bytes | translated | high | `80283ba0_FUN_80283ba0.c` |
| `0x80283DA0` | 100 | `fn_80283da0` | resolve a material data pointer from its relocation kind | translated | high | `80283da0_FUN_80283da0.c` |
| `0x80283E10` | 104 | `fn_80283e10` | construct material object, vtable `0x8034F298` | translated | high | `80283e10_FUN_80283e10.c` |
| `0x80283F60` | 156 | `fn_80283f60` | release material object and embedded sub-object | translated | high | `80283f60_FUN_80283f60.c` |
| `0x802842A0` | 288 | `fn_802842a0` | decode one 12-byte material colour entry | translated | high | `802842a0_FUN_802842a0.c` |
| `0x802843C0` | 280 | `fn_802843c0` | decode a nested material entry with sub-index | translated | high | `802843c0_FUN_802843c0.c` |
| `0x802844E0` | 244 | `fn_802844e0` | select a material entry and call the chunk builder | translated | medium | `802844e0_FUN_802844e0.c` |
| `0x802845E0` | 108 | `fn_802845e0` | construct sub-object, vtable `0x8034F2D0` | translated | high | `802845e0_FUN_802845e0.c` |
| `0x80284650` | 168 | `fn_80284650` | initialise material/animation state fields | translated | high | `80284650_FUN_80284650.c` |
| `0x80285500` | 112 | `fn_80285500` | finish an animation callback; optional material release | translated | high | `80285500_FUN_80285500.c` |
| `0x80285810` | 100 | `fn_80285810` | construct list/state object, vtable `0x8034F348` | translated | high | `80285810_FUN_80285810.c` |
| `0x80285880` | 96 | `fn_80285880` | destroy list/state object; optional free | translated | high | `80285880_FUN_80285880.c` |
| `0x80287130` | 88 | `fn_80287130` | construct list/state object, vtable `0x8034F3B0` | translated | high | `80287130_FUN_80287130.c` |
| `0x80287190` | 64 | `fn_80287190` | free an object when its use count is positive | translated | high | `80287190_FUN_80287190.c` |
| `0x80294440` | 220 | `fn_80294440` | return the cached 3x4 matrix, rebuilding when dirty | translated | high | `80294440_FUN_80294440.c` |
| `0x80294520` | 108 | `fn_80294520` | advance per-node expiry counters | translated | high | `80294520_FUN_80294520.c` |
| `0x80295970` | 276 | `fn_80295970` | build an orthonormal 3x4 basis from two axes | translated | high | `80295970_FUN_80295970.c` |
| `0x8029C080` | 1,940 | `fn_8029c080` | emit TEV raster/constant colour channels | translated | medium | `8029c080_FUN_8029c080.c` |
| `0x802A0A90` | 868 | `fn_802a0a90` | build a 3x4 Euler matrix from three angle bytes | translated | medium | `802a0a90_FUN_802a0a90.c` |
| `0x802AC340` | 200 | `fn_802ac340` | select two orientation floats from byte `+0xBA` | translated | high | `802ac340_FUN_802ac340.c` |
| `0x802AD520` | 56 | `fn_802ad520` | write a four-value half-angle orientation vector | translated | high | `802ad520_FUN_802ad520.c` |

Totals: **25 translated, 0 partial, 0 stubs**; **22 high, 3 medium, 0 low**.

## What the wave-7 pass adds

### 1. RAWR material-resource decoding (`0x80283A30..0x802844E0`)

The RAWR header is a 32-bit magic `0x52574152`, a 16-bit version word at
`+0x06`, and two self-relative offsets at `+0x10` and `+0x18`. The decoded
table uses 12-byte entries (`+0x0C` kind, `+0x10` payload offset) and nested
8-byte sub-entries (`+0x04` kind, `+0x08` payload offset). `0x80283A30` is the
parser's 32-bit LCG (`state * 0x19660D + 0x3C6EF35F`); its state is SDA word
`0x80429A70` (`r13-0x5C10`).

The version checks are exact, not simplified:

- `0x80283A90` accepts only the version that makes
  `((0x100 | ~version) - ((0x100 - version) >> 1)) >> 31` equal to 1. The raw
  stream implements that expression with `orc`, `subf` and `rlwinm`; in the
  shipped data it admits the `0x0100` header.
- The colour decoders `0x802842A0` and `0x802843C0` branch on version
  `< 0x101`, `== 0x101`, and `>= 0x102`, with different byte fills.
- `0x802844E0` has a second path for an already-resolved entry table. It
  indexes `+0x08` for old headers and `+0x0C` for new ones after a count check.

### 2. Material/animation state (`0x802845E0..0x80287190`)

`0x802845E0` constructs the sub-object installed at `object+0x110` by
`0x80283E10`. It installs vtable `0x8034F2D0` and the two inner vtables
`0x8034F2F4` and `0x8034F308`, clears the state words, and initialises the
animation block at `+0xF8`. `0x80284650` resets the material state used by the
animation controller: two `1.0f` fields, the `-1` sentinel at `+0xEC`, the
`0x40`/`0x7F` colour bytes, and the handle word at `+0x114`.

`0x80285500` is the animation end callback: it takes the manager lock at
`0x8027B5F0()+0x354`, releases the material handle when the callback kind is 2,
clears the owner's `+0x114`, and unlocks. The two small objects
`0x80285810`/`0x80287130` are vtable-plus-list constructors; their
counterparts `0x80285880`/`0x80287190` are the conditional free wrappers.

### 3. TEV colour and matrices (`0x80294440..0x802AD520`)

`0x8029C080` is the pass's largest function and the band's clearest TEV colour
builder. It has two independent halves:

- **Material colours.** Four RGB triples at `+0xC0/+0xC4/+0xC8/+0xCC` and four
  alpha bytes at `+0xC3/+0xC7/+0xCB/+0xCF` are refreshed from the descriptor
  at `param_2+0x20..+0x2F`. The alpha bytes are scaled only when one of the
  `+0xBC..+0xBF` flags and the `param_3+0x71` enable byte are non-zero, using
  `0x802925D0(param_2)` and `(value * scale + 0x80) >> 8`. When the owner
  (`param_2+0xC8`) has `+0x6C` set, each refreshed pair is also pushed through
  the GQR-affected `0x802946F0`.
- **TEV channels.** Three raster channels use selectors at `param_3+0x61` and
  `+0x69`; four constant channels use `+0x64` and `+0x6C`. Selector values
  `1/2/5/3/4/6` mean first, second, first×second, third, fourth,
  third×fourth, matching the mapping already documented for `0x8029B340`.
  The packed `RGBA` word is compared with the per-channel cache at
  `+0x94..+0xAF`; `param_4 != 0` forces an update.

The matrix group reads and writes the same 3x4 layout used by the earlier
waves. `0x80294440` returns the cached matrix and rebuilds it through the
GQR-quantized `0x80288E30` when byte `+0x8A` is dirty. `0x80295970` normalises
two axes and forms the third with `0x801C1530`. `0x802A0A90` turns the three
angle bytes into a full Euler matrix; `0x802AC340` and `0x802AD520` provide the
two-float orientation selector and the half-angle vector writer.

## Raw/Ghidra differences handled

1. **`0x80283B10` hidden call arguments.** Ghidra prints
   `FUN_80283a60(local_8)` with one argument, but the raw call keeps
   `entry+0x10` in `r4` and `table[1]` in `r5`. The Rust passes all three.
2. **`0x80284650` `+0xD8` argument.** Ghidra takes the low half of the
   CodeWarrior save-helper return; the raw stream stores incoming `r4`
   (`param_2`) at `+0xD8`. The Rust stores `param_2`.
3. **`0x802842A0` / `0x802843C0` copy order.** The raw stores are interleaved
   (`+4`, `+8`, `+5`, `+6`, `+7`, `+0x0C`, `+0x0D`); the Rust follows them
   instead of Ghidra's re-ordered field names.
4. **`0x80294440` matrix copy.** Six load-before-store word pairs copy the
   twelve words at `+0x8C..+0xB8`, so an aliasing destination still sees the
   original matrix.
5. **`0x8029C080` alpha gating.** The alpha scale is gated by
   `+0xBC..+0xBF` (not the RGB flags); the first owner callback is gated by
   `+0xB8/+0xB9/+0xBC/+0xBD` and the second by `+0xBA/+0xBB/+0xBE/+0xBF`.
6. **`0x802A0A90` step constant.** `r2-0x40F4` is `0x3CC90FDB`
   (`0.024543693`, i.e. `2*pi/256`), not a degrees-to-radians constant.

## Deferred addresses

The full 85-entry `G` list is unchanged from `docs/COVERAGE_GX_BAND.md`; every
one of those files still contains both `ldexpf(...)` and `unaff_GQR0`. The
following near-neighbours were opened or referenced by this pass and
deliberately **not** translated:

- **Referenced as opaque callees:** `0x802946F0` (material colour push),
  `0x802960F0` (vector normaliser), `0x802961E0` (sine/cosine pair). All three
  contain `ldexpf`/`unaff_GQR0`; their call shapes are reproduced, their float
  arithmetic is not modelled.
- **GQR entries checked around the new clusters:** `0x80280910`,
  `0x80280BA0`, `0x80281C90`, `0x802821E0`, `0x8029C820`, `0x8029CFC0`,
  `0x8029F640`, `0x8029F900`, `0x802A08A0`, `0x802A0910`, `0x802A09F0`,
  `0x802A0A40`, `0x802A0E50`, `0x802A1090`, `0x802A2DE0`, `0x802A64B0`.
- **`S` — entry `< 16` bytes (25):** unchanged; veneers, shared-`blr` helpers
  and empty bodies.
- **`F` — `0x8029BAB0` (1,484 bytes):** the GX state readback/emit routine with
  69 FIFO writes and no GQR. Still deferred until the GX state/enum map is
  established; it is the largest remaining non-GQR function in the band.
- **`R` — 110 entries:** no GQR marker and size `>= 16`, still unselected.
  Notable neighbours left for later:
  - `0x80283BD0` — the in-lane material chunk builder called by
    `fn_802844e0` by address; its C mixes pointer arithmetic with a save-helper
    frame and needs its own raw pass.
  - `0x80283BC0` — the shared-tail resolver called by `fn_802844e0`.
  - `0x80284700` / `0x80284F30` — the animation-controller update paths; their
    Ghidra C contains `extraout_r4` and a `thunk_FUN_80229190`, so they need a
    raw register pass before translation.
  - `0x80287930` — the scene-node spawn path; it mixes GQR matrix helpers with
    vtable dispatch.
  - `0x802A0660` — the billboard/matrix copy; Ghidra casts a matrix word to
    `float` and needs raw confirmation.

## Promotion status

| Cluster | Functions | Destination file | Ledger `module` |
|---|---:|---|---|
| RAWR material decoding | 11 | `src/render/gx_band.rs` | `render::gx_band` |
| Material/animation state | 6 | `src/render/gx_band.rs` | `render::gx_band` |
| TEV colour emitter | 1 | `src/render/gx_band.rs` | `render::gx_band` |
| Matrix / orientation helpers | 7 | `src/render/gx_band.rs` | `render::gx_band` |

- `src/render/gx_band.rs` is the final semantic home; there is no staging
  module.
- `ledger/agents/wave7-gx.csv` has one row per translated function.
- No `lib.rs`, `mod.rs`, `Cargo.toml`, `platform.rs` or other lane file was
  edited by this pass.

## Uncertainties

1. **Class and field names remain inferred.** The RAWR magic is observed, but
   the Nintendo class that owns the material record is not named. The Rust uses
   offset-based descriptions.
2. **`0x80283BD0` is called by address.** `fn_802844e0` needs its result but the
   function is not translated by this pass; the call is a real in-lane address,
   not an invented symbol.
3. **`0x8029C080` selector semantics.** The numeric mapping `1/2/5/3/4/6` is
   proven by the shared code path with `0x8029B340`; the human-facing GX enum
   names (colour channel / constant register) are not claimed.
4. **`0x802A0A90` axis meanings.** The switch arms are exact; whether axis 0/1/2
   are X/Y/Z in Nintendo's coordinate convention is inferred from the matrix
   shape.
5. **`0x802AC340` return convention.** The original returns two float words in
   `r3`/`r4`; the Rust signature is a `(f32, f32)` tuple and the doc records the
   original register order.
6. **Local buffer sizes.** The 48-byte matrix scratch in `fn_80294440`, the
   20-byte header buffer in `fn_802844e0` and the 8-byte resolved slots come
   from the Ghidra stack frames; the callees may write less.
7. **GQR callees stay opaque.** `0x802946F0`, `0x802960F0`, `0x802961E0`,
   `0x80288E30` and `0x801C1530` are called by address. Their float semantics
   are not reconstructed.

## Method and reproducibility

- Scope: `functions.tsv` entries with `0x80280000 <= entry < 0x802B0000`;
  345 entries. Selection excluded:
  - every file containing `ldexpf(` or `unaff_GQR0` (85 files);
  - every address already present in `ledger/functions.csv`,
    `ledger/agents/gx-band.csv`, `ledger/agents/wave4-gx.csv` or
    `ledger/agents/wave5-gx.csv` (99 addresses);
  - every entry smaller than 16 bytes (25).
  That left 136 candidates; 25 were selected for their material-resource,
  TEV-colour or matrix roles.
- Each selected entry was disassembled from `work/dol/main.elf` with a
  purpose-written PowerPC decoder and compared instruction-by-instruction with
  the Ghidra C. The six differences listed above were corrected from the raw
  stream.
- Constants used:
  - `r2 = 0x80433080`; `r2-0x4108 = 0x8042EF78` (0.0f);
    `r2-0x4100 = 0x8042EF80` (1.0f);
    `r2-0x40F4 = 0x8042EF8C` (0x3CC90FDB, `2*pi/256`);
    `r2-0x4004 = 0x8042F07C` (0.5f).
  - `r13 = 0x8042F680`; `r13-0x5C10 = 0x80429A70` (LCG state).
  - vtables: `0x8034F298`, `0x8034F2D0`, `0x8034F2F4`, `0x8034F308`,
    `0x8034F348`, `0x8034F3B0`.
- Reproduce the GQR set: for every band file, grep for `ldexpf(` or
  `unaff_GQR0`.
- Reproduce this pass: `ledger/agents/wave7-gx.csv` plus the function table
  above.
- `node tools/doc-lint.mjs src` passes with 0 violations. `cargo check` still
  reports only other lanes' errors (`src/engine/globals.rs` and
  `src/resource/bres.rs`); `src/render/gx_band.rs` itself compiles cleanly and
  was also compiled in isolation with a temporary `platform + gx_band` crate.
