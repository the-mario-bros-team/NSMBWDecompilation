# Coverage wave 4 — GX/render second pass (`0x80280000..0x802B0000`)

## How to read this file

- **Scope** — 25 previously untranslated functions from the GX/render band of
  `main.dol`, all now in `src/render/gx_band.rs` (ledger module
  `render::gx_band`). This is a continuation of the first pass documented in
  `docs/COVERAGE_GX_BAND.md`; that file remains the full range inventory and
  the authoritative deferred list.
- **What this pass adds** — the remaining scalar setters for the
  `0x104`-byte scene/texture object, the TEV channel-count wrapper at
  `0x8029B340`, two small vtable-installing constructors, and a group of
  scalar/struct helpers (3-float copies, two volume scales, an interval pair
  and one header-relative table lookup).
- **Addresses** are virtual `main.dol` addresses, not file offsets. Image base
  is `0x80004000`; text virtual `0x80006780` maps to file offset `0x27C0`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = entry/exit, every
  call and every observable memory/MMIO write is accounted for; `partial` = a
  known piece is missing and marked `// partial:`; `stub` = address only.
- **Confidence** measures the strength of the evidence, not code quality:
  `high` = raw instructions and Ghidra C agree; `medium` = the flow is exact
  but a callee identity, field meaning or role is inferred; `low` = mostly
  guesswork.
- **Evidence** means the Ghidra C under `work/ghidra/main-gb/export/`, plus a
  raw check against `work/dol/main.elf`. Every row names its Ghidra file.
- **Start here** — `fn_8029b340` (TEV channel setup) is the most
  representative entry; `fn_80281550` is the template for the nine setters;
  `fn_802967b0` shows the `clz` interval test used by the `0x80296xxx` pair.
- **Companion ledger** — `ledger/agents/wave4-gx.csv`.

## Summary

| module | functions | bytes (`functions.tsv`) | address span | status | confidence |
|---|---:|---:|---|---|---|
| `render::gx_band` | 25 | 1,340 | `0x80281550..0x802AB4C0` | 25 translated, 0 partial, 0 stub | 20 high, 5 medium, 0 low |

Band totals after this pass (`work/ghidra/main-gb/export/functions.tsv`,
entries `0x80280000 <= addr < 0x802B0000`):

| Metric | Count |
|---|---:|
| Function entries in band | 345 |
| Translated before wave 4 | 24 |
| Added by wave 4 | 25 |
| Translated now | 49 |
| Coverage of entries | 14.2 % |
| Deferred | 296 |
| Deferred because of paired-single/GQR (`G`) | 85 |
| Deferred because entry `< 16` bytes (`S`) | 25 |
| Deferred GX-FIFO state code, no GQR (`F`) | 1 |
| Deferred for other scope reasons (`R`) | 185 |

All 25 wave-4 entries were previously in the `R` (unselected) bucket; the
`G`, `S` and `F` counts are unchanged from `docs/COVERAGE_GX_BAND.md`.

## Function table

Evidence paths are relative to `work/ghidra/main-gb/export/`.

### `render::gx_band` — `src/render/gx_band.rs`

| address | size | Rust function | role | status | confidence | evidence |
|---|---:|---|---|---|---|---|
| `0x80281550` | 32 | `fn_80281550` | set float `+0xE4`; change bit 4 | translated | high | `80281550_FUN_80281550.c` |
| `0x80281590` | 32 | `fn_80281590` | set word `+0xF8`; change bit `0x10` | translated | high | `80281590_FUN_80281590.c` |
| `0x802815D0` | 32 | `fn_802815d0` | set float `+0xB0`; change bit `0x10` | translated | high | `802815d0_FUN_802815d0.c` |
| `0x802815F0` | 32 | `fn_802815f0` | set float `+0xB4`; change bit `0x20` | translated | high | `802815f0_FUN_802815f0.c` |
| `0x802816C0` | 32 | `fn_802816c0` | set word `+0xBC`; change bit `0x10` | translated | high | `802816c0_FUN_802816c0.c` |
| `0x802816E0` | 48 | `fn_802816e0` | set `max(v,0)` float `+0xC0`; change bit `0x10` | translated | high | `802816e0_FUN_802816e0.c` |
| `0x80281710` | 56 | `fn_80281710` | set `max(v+1,0)` float `+0xC4`; change bit `0x10` | translated | high | `80281710_FUN_80281710.c` |
| `0x80281750` | 56 | `fn_80281750` | set clamped float in array `+0xC8+i*4`; bit `0x10` | translated | high | `80281750_FUN_80281750.c` |
| `0x80281790` | 56 | `fn_80281790` | set clamped float in array `+0xD4+i*4`; bit `0x10` | translated | high | `80281790_FUN_80281790.c` |
| `0x80296760` | 20 | `fn_80296760` | copy an 8-byte interval pair | translated | high | `80296760_FUN_80296760.c` |
| `0x80296780` | 40 | `fn_80296780` | initialise pair from a node, or clear it | translated | high | `80296780_FUN_80296780.c` |
| `0x802967B0` | 44 | `fn_802967b0` | return 1 when the pair is empty (`clz(diff) >> 5`) | translated | high | `802967b0_FUN_802967b0.c` |
| `0x802967E0` | 64 | `fn_802967e0` | return the pair's node when non-empty, else 0 | translated | high | `802967e0_FUN_802967e0.c` |
| `0x8029B340` | 224 | `fn_8029b340` | turn a channel flag word into a count; emit TEV channels | translated | medium | `8029b340_FUN_8029b340.c` |
| `0x8029D2C0` | 60 | `fn_8029d2c0` | construct object with vtable `0x8034F6E0` | translated | high | `8029d2c0_FUN_8029d2c0.c` |
| `0x8029F460` | 60 | `fn_8029f460` | construct object with vtable `0x8034F700` | translated | high | `8029f460_FUN_8029f460.c` |
| `0x802A0830` | 60 | `fn_802a0830` | matrix-helper wrapper: `helper(a,b,dst)`; return `dst` | translated | medium | `802a0830_FUN_802a0830.c` |
| `0x802A0870` | 28 | `fn_802a0870` | copy three floats (loads before stores) | translated | high | `802a0870_FUN_802a0870.c` |
| `0x802A0890` | 16 | `fn_802a0890` | store three float arguments as a 3-vector | translated | high | `802a0890_FUN_802a0890.c` |
| `0x802A0950` | 28 | `fn_802a0950` | scale `+0x30 * +0x38 * owner[+0x58]` | translated | medium | `802a0950_FUN_802a0950.c` |
| `0x802A0970` | 116 | `fn_802a0970` | mode-selected scale factor times `owner[+0x5C]` | translated | medium | `802a0970_FUN_802a0970.c` |
| `0x802A8990` | 56 | `fn_802a8990` | pack two 3-vectors plus a scalar into 7 fields | translated | high | `802a8990_FUN_802a8990.c` |
| `0x802A89D0` | 60 | `fn_802a89d0` | copy a 7-field record | translated | high | `802a89d0_FUN_802a89d0.c` |
| `0x802AB490` | 44 | `fn_802ab490` | read one word from a header-relative table | translated | medium | `802ab490_FUN_802ab490.c` |
| `0x802AB4C0` | 44 | `fn_802ab4c0` | set/clear bit 3 of `object[+0x04]` | translated | high | `802ab4c0_FUN_802ab4c0.c` |

Totals: **25 translated, 0 partial, 0 stubs**; **20 high, 5 medium, 0 low**.

## What the wave-4 pass adds

### 1. Remaining scalar setters for the `0x104`-byte object

The first pass translated the setters whose fields were already pinned
(`0x80281370..0x802817D0`). The nine added here complete the scalar bank:

- two words (`+0xBC`, `+0xF8`);
- five floats (`+0xB0`, `+0xB4`, `+0xC0`, `+0xC4`, `+0xE4`);
- two indexed float arrays (`+0xC8 + i*4`, `+0xD4 + i*4`).

Every one reproduces the same store order as the original:
`load old -> compare -> beqlr when equal -> lhz mask -> store -> ori bit ->
sth mask`. Two details are worth keeping:

- `0x80281710` uses `fadds` (single precision) before the clamp, not `fadd`.
  The raw word at `0x80281718` is `0xEC21102A` (`fadds f1,f1,f2`) and
  `f2 = r2-0x4578 = 1.0f`. Rounding to `f32` before the comparison is
  therefore observable and is preserved.
- `0x802816E0` and both array setters clamp with `fcmpo` + `bge`. For NaN the
  branch is taken (CR0[LT] is clear), so NaN is stored unclamped. Rust's
  `value < 0.0` has the same behaviour.

### 2. TEV channel setup (`0x8029B340`)

This is the one function in the pass that names a GX operation with
confidence: the count produced from the flag word is passed to `0x801C4A20`,
whose body stores `value & 0xF` into a GX state word and raises a dirty bit —
the `GXSetNumChans` shape. The flag bits are `0x10`, `0x20` and `0x40`; the
selectors are recorded in `+0xD4/+0xD8/+0xDC` and the two per-channel
emitters `0x8029B420` / `0x8029B710` receive the same flag word.

**Raw/Ghidra difference (new finding).** Ghidra prints the counter as
`0/1/2/3`. The raw stream tests each flag with `rlwinm. r0,r0,0,MB,MB`, which
leaves the mask value itself (`0x10`, `0x20`, `0x40`) in `r0`, and for the
first channel executes `addi r0,r0,1`. The byte stored at `+0xD0` is
therefore `0x11` when only bit `0x10` is set (not 1). Because `0x801C4A20`
masks with `0xF`, the low nibble still carries the channel count (1), so the
GX write is unaffected; the selector words do keep the high bit. The Rust
translation follows the raw value and documents the discrepancy at the call
site and in the ledger row.

### 3. Scalar and struct helpers

- `0x802A0950` / `0x802A0970` are pure `lfs`/`fmuls` scale factors. They are
  the `0x802A` neighbours of the GQR cluster and were checked specifically to
  confirm they contain **no** `psq_l`/`psq_st`: their raw streams use only
  scalar float instructions, and their Ghidra C contains no `ldexpf`.
- `0x802A0870`, `0x802A0890`, `0x802A8990`, `0x802A89D0` move whole words so
  bit patterns survive; the copies load everything before storing, which keeps
  them safe when source and destination alias.
- `0x80296760..0x802967E0` are one 8-byte interval pair (`begin`, `node`) with
  the end offset at `node[+0x08]`. `0x802967B0` returns "is empty" via
  `countLeadingZeros(end - begin) >> 5`; `0x802967E0` returns the node only
  when that test is false. `src/ui/lyt.rs` names `0x802967B0` as a callee but
  does not translate it; the body now exists here.
- `0x802AB490` walks a self-relative header (`*table`, then `[+0x04]`, then
  `[+0x00]`) to read one indexed word. The arithmetic is exact; the table's
  owner is marked `// unknown:`.

## Deferred: paired-single / GQR (`G`, 85 entries, 99,372 bytes)

These entries contain both `ldexpf(...)` and `unaff_GQR0` in the Ghidra C;
the raw stream uses `psq_l`/`psq_st` with GQR configuration. The C float
expressions are not trustworthy, so they are **explicitly out of scope** until
a GQR-aware pass (or a full raw-instruction reconstruction) exists. This list
is unchanged from `docs/COVERAGE_GX_BAND.md` and is the definitive deferral
for this lane.

**`0x80280000..0x8028FFFF` (23):**
`80280910`, `80280BA0`, `80281C90`, `802821E0`, `802850F0`, `80286CA0`,
`80287C10`, `80288390`, `80288490`, `80288E30`, `80289410`, `8028A6F0`,
`8028B6C0`, `8028C7A0`, `8028CEE0`, `8028DC70`, `8028DFC0`, `8028E310`,
`8028E890`, `8028ED30`, `8028F2D0`, `8028F860`, `8028FEA0`.

**`0x80290000..0x8029FFFF` (28):**
`80290690`, `80290CF0`, `80291100`, `80291A10`, `80292C90`, `802941F0`,
`802946F0`, `80295860`, `80295A90`, `80295C20`, `802960F0`, `80296160`,
`802961E0`, `80296260`, `802963B0`, `80296530`, `802965B0`, `802966F0`,
`80296820`, `80298720`, `80298B80`, `80298ED0`, `80299510`, `80299840`,
`8029C820`, `8029CFC0`, `8029F640`, `8029F900`.

**`0x802A0000..0x802AFFFF` (34):**
`802A08A0`, `802A0910`, `802A09F0`, `802A0A40`, `802A0E50`, `802A1090`,
`802A2DE0`, `802A3160`, `802A35C0`, `802A3A20`, `802A3E00`, `802A40C0`,
`802A4400`, `802A47C0`, `802A5810`, `802A64B0`, `802A6850`, `802A6CB0`,
`802A6F80`, `802A6FB0`, `802A71D0`, `802A7CA0`, `802A8020`, `802A8440`,
`802A8930`, `802A8960`, `802A8A10`, `802A8D10`, `802A90C0`, `802A9640`,
`802AA830`, `802AE9C0`, `802AF080`, `802AF4F0`.

Notes on the ones nearest this pass:

- `0x802A08A0` and `0x802A0910` sit immediately after `0x802A0890` and were
  raw-checked during this pass: both contain `psq_l`/`psq_st` and
  paired-single arithmetic (`ps_mul`, `ps_sum0`, …), so they stay deferred.
- `0x802A8930` / `0x802A8960` are the GQR neighbours of `0x802A8990` /
  `0x802A89D0` and were skipped for the same reason.
- `0x8029F640` is the explicit `psq_l`/`psq_st` quad emitter and remains the
  highest-value target for a GQR-aware pass.

## Other deferred entries

Unchanged from `docs/COVERAGE_GX_BAND.md`:

- **`S` — entry `< 16` bytes (25):** veneers, thunks and shared-`blr` helpers
  with no meaningful body.
- **`F` — `0x8029BAB0` (1,484 bytes):** GX state readback/emit with 69 FIFO
  writes and no GQR, but it needs a GX enum/state map.
- **`R` — 185 entries:** no GQR marker and size `>= 16`, still unselected.
  The largest remaining clusters are the texture-grid allocate/free/apply
  group (`0x80280E60`, `0x802810A0`, `0x80281170`, `0x80281390`), the
  `0x104`-byte pool allocator (`0x80282FF0`, `0x802830A0`), and the large
  `0x8029C080` / `0x8029B710` dispatchers.

## Uncertainties

1. **`0x8029B340` counter value.** The raw stream stores `0x11` for the first
   channel where Ghidra prints `1`. The translation follows the raw bytes; the
   low nibble is the value GX consumes, but the high bit's purpose is unknown.
2. **TEV naming.** `0x801C4A20` is called "GXSetNumChans-shaped" because its
   body writes `value & 0xF` into one state word and raises a dirty bit. No
   symbol table is present, so the name is a shape inference.
3. **The `0x104`-byte object's new fields.** `+0xB0/+0xB4/+0xC0/+0xC4/+0xE4`
   and the arrays at `+0xC8`/`+0xD4` are named only by offset and change bit.
   Their consumer is outside this lane.
4. **`0x802A0950` / `0x802A0970` role.** The arithmetic is exact (`fmuls` on
   scalar floats), but "volume/scale factor" is inferred from the owner layout
   and the neighbouring code; no consumer was reconstructed.
5. **`0x802AB490` table.** The self-relative header chain is exact; whether it
   is a REL, BRRES or other section table is not established.
6. **`0x802A0830` callee.** `0x801C0540` is a paired-single matrix helper; its
   Ghidra C is a mis-render, so only the argument shuffle and return value are
   claimed.

## Method and reproducibility

- Scope: `functions.tsv` entries with `0x80280000 <= entry < 0x802B0000`;
  345 entries. Selection excluded every file containing `ldexpf(` or
  `unaff_GQR0` (85 files, matching `docs/COVERAGE_GX_BAND.md`), then excluded
  the 24 addresses already present in `ledger/agents/gx-band.csv`.
- For each of the 25 selected entries the raw instruction stream was read from
  `work/dol/main.elf` and compared with the Ghidra C. The two non-obvious
  findings are documented above (`0x80281710` `fadds`, `0x8029B340` `0x11`)
  plus the already-known `0x802814D0` and `0x8029F4A0` mis-renders from the
  first pass.
- Constants used:
  - `r2 = 0x80433080`;
  - `r2 - 0x4574 = 0x8042EB0C = 0.0f`;
  - `r2 - 0x4578 = 0x8042EB08 = 1.0f`.
- Reproduce the GQR set: for every band file, grep for `ldexpf(` or
  `unaff_GQR0`.
- Reproduce this pass: `ledger/agents/wave4-gx.csv` plus the function table
  above.
- `cargo check` and `node tools/doc-lint.mjs src` pass with the new code.
  `src/render/gx_band.rs` is the only Rust file touched by this pass.
