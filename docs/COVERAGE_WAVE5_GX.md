# Coverage wave 5 — GX/render third pass (`0x80280000..0x802B0000`)

## How to read this file

- **Scope** — 25 previously untranslated functions from the GX/render band of
  `main.dol`, all now in `src/render/gx_band.rs` (ledger module
  `render::gx_band`). This pass continues the work documented in
  `docs/COVERAGE_GX_BAND.md` (full range inventory) and
  `docs/COVERAGE_WAVE4_GX.md` (second pass).
- **What this pass adds** — the texture-grid allocate/free/apply family around
  `0x80280E60..0x80281C00`, the TEV per-channel and stage/material emitters
  `0x8029B420`/`0x8029B710`, two GX draw-state dispatchers, one matrix wrapper,
  and five vtable-installing state constructors.
- **Addresses** are virtual `main.dol` addresses, not file offsets. Image base
  is `0x80004000`; the main text segment maps virtual `0x80006780` to file
  offset `0x2B00` in `work/dol/main.elf`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = entry/exit, every call
  and every observable memory/MMIO write is accounted for; `partial` = a known
  piece is missing and marked `// partial:`; `stub` = address only.
- **Confidence** measures the strength of the evidence, not code quality:
  `high` = raw instructions and Ghidra C agree; `medium` = the flow is exact
  but a callee identity or field meaning is inferred; `low` = mostly guesswork.
- **Evidence** means the Ghidra C under `work/ghidra/main-gb/export/`, plus a
  raw-instruction check against `work/dol/main.elf`. Every row names its Ghidra
  file.
- **Start here** — [`fn_80280e60`](../src/render/gx_band.rs) is the texture
  grid entry point; [`fn_8029b420`](../src/render/gx_band.rs) is the
  per-channel TEV emitter; [`fn_8029ed70`](../src/render/gx_band.rs) is the
  clearest draw-state dispatcher.
- **Companion ledger** — `ledger/agents/wave5-gx.csv`.

## Summary

| module | functions | bytes (`functions.tsv`) | address span | status | confidence |
|---|---|---:|---|---|---|
| `render::gx_band` | 25 | 5,260 | `0x80280E60..0x802AC2E0` | 25 translated, 0 partial, 0 stub | 16 high, 9 medium, 0 low |

Band totals after this pass (`work/ghidra/main-gb/export/functions.tsv`,
entries `0x80280000 <= addr < 0x802B0000`):

| Metric | Count |
|---|---:|
| Function entries in band | 345 |
| Translated before wave 5 | 49 |
| Added by wave 5 | 25 |
| Translated now | 74 |
| Coverage of entries | 21.4 % |
| Deferred | 271 |
| Deferred because of paired-single/GQR (`G`) | 85 |
| Deferred because entry `< 16` bytes (`S`) | 25 |
| Deferred GX-FIFO state code, no GQR (`F`) | 1 |
| Deferred for other scope reasons (`R`) | 160 |

All 25 wave-5 entries were previously in the `R` (unselected) bucket; the
`G`, `S` and `F` counts are unchanged from `docs/COVERAGE_GX_BAND.md`.

## Function table

Evidence paths are relative to `work/ghidra/main-gb/export/`.

### `render::gx_band` — `src/render/gx_band.rs`

| address | size | Rust function | role | status | confidence | evidence |
|---|---|---:|---|---|---|---|
| `0x80280E60` | 572 | `fn_80280e60` | allocate texture grid; clamp sizes; retry via pool | translated | medium | `80280e60_FUN_80280e60.c` |
| `0x802810A0` | 204 | `fn_802810a0` | release texture grid and remove it from the pool | translated | medium | `802810a0_FUN_802810a0.c` |
| `0x80281170` | 512 | `fn_80281170` | apply material and sampler state to every grid cell | translated | medium | `80281170_FUN_80281170.c` |
| `0x80281390` | 208 | `fn_80281390` | release handles whose texture is at `+0x38 == 1` | translated | medium | `80281390_FUN_80281390.c` |
| `0x80281820` | 160 | `fn_80281820` | store selector, register in pool, set format `0x0F` | translated | high | `80281820_FUN_80281820.c` |
| `0x802818C0` | 144 | `fn_802818c0` | set format `0x10` when not in mode 1 | translated | high | `802818c0_FUN_802818c0.c` |
| `0x80281950` | 124 | `fn_80281950` | apply one value to a single grid row | translated | high | `80281950_FUN_80281950.c` |
| `0x802819D0` | 24 | `fn_802819d0` | return first cell's info word, or 0 | translated | high | `802819d0_FUN_802819d0.c` |
| `0x802819F0` | 116 | `fn_802819f0` | row walk with `0x80261FB0` | translated | high | `802819f0_FUN_802819f0.c` |
| `0x80281A70` | 116 | `fn_80281a70` | row walk with `0x802620D0` | translated | high | `80281a70_FUN_80281a70.c` |
| `0x80281AF0` | 136 | `fn_80281af0` | grid walk with `0x80262200` | translated | high | `80281af0_FUN_80281af0.c` |
| `0x80281B80` | 116 | `fn_80281b80` | row walk with `0x802622A0` | translated | high | `80281b80_FUN_80281b80.c` |
| `0x80281C00` | 136 | `fn_80281c00` | grid walk with `0x802627E0` | translated | high | `80281c00_FUN_80281c00.c` |
| `0x8029B290` | 172 | `fn_8029b290` | fill base GX state object; vtable `0x8034F6C8` | translated | high | `8029b290_FUN_8029b290.c` |
| `0x8029B420` | 748 | `fn_8029b420` | emit per-channel TEV state from a descriptor | translated | medium | `8029b420_FUN_8029b420.c` |
| `0x8029B710` | 928 | `fn_8029b710` | emit TEV stages and channel material flags | translated | medium | `8029b710_FUN_8029b710.c` |
| `0x8029ED70` | 208 | `fn_8029ed70` | material channel setup + GX draw-state block | translated | medium | `8029ed70_FUN_8029ed70.c` |
| `0x802A1050` | 60 | `fn_802a1050` | matrix helper wrapper: `helper(a,b,dst)` | translated | medium | `802a1050_FUN_802a1050.c` |
| `0x802A16D0` | 188 | `fn_802a16d0` | sibling draw-state block without local matrix | translated | medium | `802a16d0_FUN_802a16d0.c` |
| `0x802A18A0` | 60 | `fn_802a18a0` | construct object with vtable `0x8034F720` | translated | high | `802a18a0_FUN_802a18a0.c` |
| `0x802A22D0` | 60 | `fn_802a22d0` | construct object with vtable `0x8034F738` | translated | high | `802a22d0_FUN_802a22d0.c` |
| `0x802A2900` | 60 | `fn_802a2900` | construct object with vtable `0x8034F750` | translated | high | `802a2900_FUN_802a2900.c` |
| `0x802A2DA0` | 60 | `fn_802a2da0` | construct object with vtable `0x8034F768` | translated | high | `802a2da0_FUN_802a2da0.c` |
| `0x802A6470` | 60 | `fn_802a6470` | construct object with vtable `0x8034F788` | translated | high | `802a6470_FUN_802a6470.c` |
| `0x802AC2E0` | 88 | `fn_802ac2e0` | load base or concatenated matrix, then end draw block | translated | high | `802ac2e0_FUN_802ac2e0.c` |

Totals: **25 translated, 0 partial, 0 stubs**; **16 high, 9 medium, 0 low**.

## What the wave-5 pass adds

### 1. Texture-grid state (`0x80280E60..0x80281C00`)

The `0x104`-byte object from section 2 owns a grid of texture handles: rows at
`+0x8C`, columns at `+0x90`, a cell stride of four bytes, and a row stride of
`0x10`. This pass translates the full lifecycle:

- `0x80280E60` clamps the requested grid to at most 2 rows by 4 columns,
  creates each handle with format `0x10` (or `0x1F` when the alpha selector is
  `0xFF`), and retries with `0x11`/`0x1F` after consulting the `0x80282EF0`
  pool. A failed retry releases every partial handle.
- `0x802810A0` releases every cell and removes the object from the pool.
- `0x80281170` copies a `0x28`-byte material block per row, applies it to every
  live cell, and resets the four `2x3` matrices at `+0x2C..+0x88`.
- `0x80281390` releases handles whose texture object has `u16[+0x38] == 1`.
- `0x80281820` / `0x802818C0` set formats `0x0F` / `0x10` across the grid.
- `0x80281950..0x80281C00` are six row/grid walkers with different callbacks.

Every texture-API callee (`0x80264AE0`, `0x80264CD0`, `0x80264F90`,
`0x802627C0`, `0x80261DF0`, `0x802629F0`, `0x80262E90`, `0x80263110`,
`0x80262DB0`, `0x802627E0`, `0x802640E0`, `0x802620D0`, `0x80261FB0`,
`0x80262200`, `0x802622A0`, `0x80262540`, `0x80282EF0`, `0x80283260`,
`0x80283420`, `0x801A3130`) is out of lane. They are called by absolute
address and marked `// unknown:` at first use; no symbol name is claimed.

**Raw/Ghidra detail.** `0x80280E60`'s Ghidra signature and body look like a
`void` function with a 64-bit return from the CodeWarrior save helper. The raw
stream sets `r20 = r3` (object) and `r4` (row count) directly, and returns 1/0
in `r3`; the Rust reproduces that. The pool walk uses `node-0x54` for capacity
and `node-0x70 * node-0x6C` for the area, both confirmed at
`0x80280F20..0x80280F3C`.

### 2. TEV channel and material state (`0x8029B290..0x8029B710`)

`0x8029B290` is the base constructor for the `0x8029B3xx` objects. Its raw body
writes vtable `0x8034F6C8`, a `[1,1,1,1,0,0]` float header, and two identical
12-word blocks. The raw loop is a CTR loop of `(0xC3 - 0x34) / 0x30 = 2`
iterations; Ghidra prints a conditional `do/while`, which is equivalent but
harder to read.

`0x8029B420` and `0x8029B710` are the two halves of the per-channel/material
emitter called by `fn_8029b340`:

- `0x8029B420` walks the channel descriptor, sets the active channel, forwards
  five per-channel byte groups to the `0x801C81xx..0x801C85xx` setters, and
  emits the colour mask (`0`, the `+0xD8` selector, or `0xFF`).
- `0x8029B710` writes three stage descriptors through `0x801C6AF0`, forwards
  the `+0x98/+0x9C` words, and expands the `1/2/5/3/4/6` selector values into
  the two four-byte channel groups at `+0xB8` and `+0xBC`.

The selector mapping is the same one used by `fn_8029b340` (`1 = X`, `2 = Y`,
`5 = XY`, `3 = Z`, `4 = W`, `6 = ZW`).

### 3. Draw-state dispatch (`0x8029ED70..0x802AC2E0`)

- `0x8029ED70` and `0x802A16D0` call `fn_8029b340`, then emit a fixed GX state
  block: vertex format, attribute `0xD` from an SDA table, state flush,
  capabilities `9` and optional `0xD`, two array attributes, and a draw-block
  terminator. `0x8029ED70` additionally loads a local 52-byte identity matrix.
- `0x802A1050` is the `(a, b, dst)` wrapper around the opaque paired-single
  helper `0x801C1530`.
- `0x802AC2E0` loads either `object+0x84` or a concatenation of
  `descriptor+4` with that matrix, then calls `0x801C9940(matrix, 0)` and
  `0x801C9A60(0)`.

The SDA tables are `0x80429AA0` (`0x8029ED70`) and `0x80429AC0`
(`0x802A16D0`), both reached as `r13 - 0x5BE0` / `r13 - 0x5BC0` with the
CRT-established `r13 = 0x8042F680`. Each contains `0x00010000` followed by
`0x01000101`.

### 4. Vtable constructors (`0x802A18A0..0x802A6470`)

Five 60-byte constructors call `0x8029B290(object)` and overwrite the vtable
word. The only difference between them is the constant:
`0x8034F720`, `0x8034F738`, `0x8034F750`, `0x8034F768`, `0x8034F788`.

## Deferred: paired-single / GQR (`G`, 85 entries, 99,372 bytes)

The full 85-entry list is unchanged from `docs/COVERAGE_GX_BAND.md` and
`docs/COVERAGE_WAVE4_GX.md`. Every one of those files contains both
`ldexpf(...)` and `unaff_GQR0`; none was translated here. The following
near-neighbours of the wave-5 clusters were checked and **deferred** for the
GQR reason:

- Near the texture grid: `0x80280910`, `0x80280BA0`, `0x80281C90`,
  `0x802821E0`.
- Near the GX vertex/object family: `0x8029F640`, `0x8029F900`,
  `0x802A08A0`, `0x802A0910`, `0x802A09F0`, `0x802A0A40`, `0x802A0E50`,
  `0x802A1090`.
- Next to the wave-5 constructors: `0x802A2DE0`, `0x802A64B0`.

The raw streams of all 25 selected functions were also scanned for
`psq_l`/`psq_st` opcodes (`56`, `57`, `60`, `61`); none was found.

## Other deferred entries

- **`S` — entry `< 16` bytes (25):** unchanged; veneers and thunks.
- **`F` — `0x8029BAB0` (1,484 bytes):** GX state readback/emit with 69 FIFO
  writes and no GQR, still waiting for a GX enum/state map.
- **`R` — 160 entries:** no GQR marker and size `>= 16`, still unselected.
  Notable neighbours left for later:
  - `0x80281EF0` — the texture-grid deletion callback passed to
    `0x80264CD0`; its body walks the grid and calls `fn_802810a0`.
  - `0x80282FF0` / `0x802830A0` — the `0x104`-byte pool allocator/user.
  - `0x8029C080` — 1,940-byte dispatcher.
  - `0x802AB550` / `0x802AB680` — layout/material state objects with their own
    vtable and allocation family; a separate pass should establish their class
    layout before translating.

## Promotion status

| Cluster | Functions | Destination file | Ledger `module` |
|---|---:|---|---|
| Texture-grid state | 13 | `src/render/gx_band.rs` | `render::gx_band` |
| TEV/material state | 3 | `src/render/gx_band.rs` | `render::gx_band` |
| Draw-state dispatch | 4 | `src/render/gx_band.rs` | `render::gx_band` |
| Vtable constructors | 5 | `src/render/gx_band.rs` | `render::gx_band` |

- `src/render/gx_band.rs` is the final semantic home; there is no staging
  module.
- `ledger/agents/wave5-gx.csv` has one row per translated function.
- No `lib.rs`, `mod.rs`, `Cargo.toml`, `platform.rs` or other lane file was
  edited by this pass.

## Uncertainties

1. **GX setter identities.** Every `0x801C7xxx..0x801C9xxx` callee is called
   by address; the names in the Rust docs describe only their argument shape.
   No GX enum name is claimed.
2. **Texture API identities.** `0x80264AE0` is called a manager accessor
   because `0x80264CD0` and `0x80264F90` bracket it; the actual texture class
   layout is not established.
3. **TEV descriptor groups.** The `+0x0C/+0x1C/+0x30/+0x40/+0x54/+0x58`
   offsets in `0x8029B420` and the `+0x61/+0x64/+0x69/+0x6C` bytes in
   `0x8029B710` are named only by offset and selector value.
4. **SDA tables.** `0x80429AA0` and `0x80429AC0` are exact addresses for
   `main.dol`, but whether they are GX attribute tables or texture-coordinate
   tables is not established.
5. **Local buffer sizes.** The 52-byte matrix buffer in `fn_8029ed70` and the
   56-byte buffer in `fn_802ac2e0` come from the Ghidra stack frames; the
   callees may write fewer bytes. The Rust arrays are sized to the frame.
6. **`0x8029B420` float range.** The `0.0/100.0` pair and the
   `descriptor+0x74..+0x80` alternative are exact, but their meaning
   (texture-coordinate range, indirect scale, or something else) is unknown.
7. **`0x8029B420` raw/Ghidra difference.** Ghidra prints the `0x801C7E00`
   third argument as a signed byte; the raw stream uses `extsb`, so the Rust
   casts through `i8`.

## Method and reproducibility

- Scope: `functions.tsv` entries with `0x80280000 <= entry < 0x802B0000`;
  345 entries.
- Selection excluded every file containing `ldexpf(` or `unaff_GQR0` (85
  files), every address already present in `ledger/agents/gx-band.csv` and
  `ledger/agents/wave4-gx.csv` (49 addresses), and every entry smaller than
  16 bytes (25). That left 186 candidates; 25 were selected for their
  texture/state/TEV/draw roles.
- Each selected entry was disassembled from `work/dol/main.elf` with the same
  PowerPC decoder used for the wave-4 raw checks, then compared with the
  Ghidra C. The main discrepancies handled were:
  - `0x80280E60`: object/row-count recovery from the save-helper artifact and
    the 1/0 return value.
  - `0x8029B290`: CTR loop count 2 instead of a conditional loop.
  - `0x8029B420`: `extsb` on the `0x801C7E00` third argument.
  - `0x8029ED70`: the local matrix frame is 52 bytes; `0x802A16D0` is the same
    sequence without it.
- Constants used:
  - `r2 = 0x80433080`;
  - `r2 - 0x4578 = 0x8042EB08 = 1.0f`;
  - `r2 - 0x4574 = 0x8042EB0C = 0.0f`;
  - `r2 - 0x4168 = 0x8042EF18 = 1.0f`;
  - `r2 - 0x4164 = 0x8042EF1C = 0.0f`;
  - `r2 - 0x4160 = 0x8042EF20 = 100.0f`;
  - `r13 = 0x8042F680`; `r13 - 0x5BE0 = 0x80429AA0`;
    `r13 - 0x5BC0 = 0x80429AC0`.
- Reproduce the GQR set: for every band file, grep for `ldexpf(` or
  `unaff_GQR0`.
- Reproduce this pass: `ledger/agents/wave5-gx.csv` plus the function table
  above.
- `cargo check` was run; the only errors in the tree are in other lanes'
  files, not in `src/render/gx_band.rs`. The module was also compiled in
  isolation with a temporary `platform + gx_band` crate to confirm that its
  `no_std` imports and raw-pointer helpers stand on their own.
  `node tools/doc-lint.mjs src` passes with 0 violations.
