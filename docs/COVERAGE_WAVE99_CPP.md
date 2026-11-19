# Coverage - Wave 99 (runtime::cpp)

## How to read this file

Each row is one `pub unsafe fn fn_<address>` body in `src/runtime/cpp.rs` that
this pass vouched for. The `bytes` column is the exact size from
`work/ghidra/main-gb/export/functions.tsv`, re-read for this pass. The body's
`# Original` block cites the same export file. Confidence matches the CSV row.

Field offsets are relative to the first argument. SDA cells are given as
absolute addresses, resolved with `r13 = 0x8042F680` and `r2 = 0x80433080` (the
baselines Waves 77, 89, 92 and 96 used). Float constants were only trusted when
they live in a loaded segment; the SDA cells at `0x8042F5xx` are written by the
game's constructors, so they are named by role and marked `// unknown:` where
the numeric value would matter.

**Every signature in this pass was checked against the raw instruction stream.**
The export loses the argument registers of any function whose first call is one
of the CodeWarrior `__savegprlr_N` helpers (Ghidra models the helper as
returning the incoming `r3`/`r4` in a 64-bit value). `work/dol/main.elf` was
disassembled at each prologue to recover the real registers; the reasoning is
repeated in each body's `# Confidence` note. Two register facts from the
`0x802D2310`/`0x802D1E20` pair settle the convention for this band: the high half
of the pseudo-return is `r3` and the low half is `r4`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/cpp.rs` | `runtime::cpp` | 7 | 3584 | 4 | 3 |

## What this pass covers

Address range: `0x802D0030..0x802D79A0`. The whole `0x802D0000` bucket was
re-scanned; of the 42 addresses Wave 96 left untranslated, these seven can be
reconstructed honestly (see **Deliberate skips** for the other 35).

- **The console panel slot family (`0x802D1E20`, `0x802D2310`).** One shared
  vocabulary: the three 12-byte slots at `0x80424820` (`handle`, `token`,
  `flags`), the published rectangle at `0x80424850`, the content edges
  `0x80424868`/`6C` and the letterbox spans `0x80424870`/`74`.
  - `0x802D1E20` is the publish half. `mulli r29,r23,12` at `0x802D1E6C` makes
    the first argument the slot index; slots 0/1 snap the scaled panel size to
    even, centre it against `0x8042F5A4`/`A8` and clamp the margins; slot 2 is
    the plain panel; slot 3 asks `0x802D2600` whether to pass GX mode 4 or 6.
    Each arm stores through `0x802D3700` and clears bit `0x10` at handle
    `+0x18`.
  - `0x802D2310` is the commit half. It maps the pending index (0 -> 1,
    1 -> 0), compares the caller's token with `slot+4` (`cmpl r4,r0` at
    `0x802D2364`), builds a 0x90-byte layout record through `0x802D0E50`,
    forces its flag word to `0x40` (and mode `1`/flags `0x41` when the
    inherited mode is not already `1`), pushes it through `0x802D0F40` and runs
    the GX sequence. The whole draw block is skipped when `r3 != 0`
    (`0x802D236C`), and the tail clears the pending index and bumps
    `0x8042AF30`.
- **`0x802D12E0` - the layout-record rectangle.** The body behind
  [`fn_802d17c0`]: it projects `+0x40`/`+0x44` (either by the installed
  `0x8042AF18` quad or through `0x802D1B90`, whose `(f1, f2, r3, r4, r5)` shape
  is read off `0x802D1340..0x802D1354`), picks a virtual-console mode from the
  flag word (`0x20` -> 0, `0x08` -> 2, else `0x8042AF08`), divides by the
  `u16` pair in the 12-byte records at `0x803504E0` (`608x456`, `832x456`,
  `640x456`), and writes `+0x60..+0x84`. The `CONCAT44(0x43300000, x) - 2^52`
  chains are the compiler's conversions and are spelled as plain Rust casts.
- **`0x802D2550` - the panel quad republish.** Six `lfs` immediately before the
  `0x802D3200` call fix that call's six float arguments, which Wave 96 believed
  unrecoverable. `0x802D3090` turns them into the four-word clip rectangle and
  `0x802D3230`/`0x802D3270(0, 0)` consume it.
- **`0x802D79A0` and its wrapper `0x802D7860`.** A tagged-section search: the
  section record at `container+0x10` carries a count (`+8`) and a tag byte
  (`+0x0B`) that indexes the descriptor tables at `0x802FD2E8` (shift counts
  `32, 24, 16, 8, 0, 0`) and `0x802FD300` (mask pairs). The identifier is split
  into a shifted high field compared with `key` and a low field compared with
  `mask`; the hit address is `entries + index*(u16 at entries+10) + 0x10`.
  `0x802D7860` adds `this[+8]` and eight to the entry's first word and forwards
  `r4`/`r5` unchanged.
- **`0x802D0030` - the GX quad submit.** The export collapsed the display list
  into one `_DAT_cc008000` store plus two synthetic calls; the raw words show
  the real shape: four vertices, each a two-byte colour value (`0x0000`,
  `0x0101`, `0x0202`, `0x0303`) followed by the object's `+0x50`/`+0x54` pair as
  two `f32` stores. Wave 96's "four GX FIFO bursts" concern is answered by
  writing the same 40 bytes to `0xCC008000` in the same order.

### New names introduced

`PANEL_AXIS_TABLE`, `SDA_AXIS_SELECT`, `SDA_RENDER_WIDTH`, `SDA_RENDER_HEIGHT`,
`PANEL_ROOT`, `PANEL_RECT`, `PANEL_QUAD`, `PANEL_EDGES`, `PANEL_SPANS`,
`PANEL_SLOTS`, `PANEL_SLOT_STRIDE`, `PENDING_PANEL_SLOT`, `PANEL_COMMIT_COUNT`,
`SECTION_FIELD_TABLE`, `SECTION_MASK_TABLE`, `SDA2_SLOT_SCALE_0`,
`SDA2_SLOT_SCALE_1`, `SDA2_EDGE_MIN`, `SDA2_PANEL_ORIGIN_X`,
`SDA2_PANEL_ORIGIN_Y`, `SDA2_PANEL_FILL`, `SDA2_GX_TAG`, `SDA2_QUAD_DEPTH`,
`SDA2_QUAD_LEVEL`, `GX_FIFO`, and the local helpers `layout_axis` and
`snap_even`. No existing constant or body was renamed, and the existing
`fn_802d1b90`/`fn_802d3200`/`fn_802d3230`/`fn_802d3270` stubs are left untouched
(the new bodies call the raw addresses, exactly as [`fn_802d0f40`] already did).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802D0030` | `fn_802d0030` | 372 | high |
| `0x802D12E0` | `fn_802d12e0` | 1240 | high |
| `0x802D1E20` | `fn_802d1e20` | 968 | medium |
| `0x802D2310` | `fn_802d2310` | 572 | medium |
| `0x802D2550` | `fn_802d2550` | 160 | high |
| `0x802D7860` | `fn_802d7860` | 60 | medium |
| `0x802D79A0` | `fn_802d79a0` | 212 | high |

## Deliberate skips

35 addresses in the bucket still have no body and no ledger row.

1. **The register-save chain, 32 addresses.** `0x802DCCCC`, `0x802DCCE0`,
   `0x802DCD18`, `0x802DCD2C`, `0x802DCD40..0x802DCD74` and
   `0x802DCD8C..0x802DCDC0` are the Metrowerks `__savegprlr_N` /
   `__restgprlr_N` / `__savefpr_N` / `__restfpr_N` helpers (twenty-six 4-byte
   entries plus four 20/24-byte runs). `docs/QUALITY.md` §5.5 excludes the range
   from translation scheduling, and the coverage doc for Wave 96 re-read the raw
   words. Each is an alternate entry point into one spill run: a Rust body could
   only be a lie about what the bytes do.
2. **`0x802DCBFC`, 1 address.** An indirect jump through `r12` with a table
   Ghidra could not recover (`UNRECOVERED_JUMPTABLE`), and no caller sets
   `r12` before the call. Nothing to reconstruct.
3. **`0x802DADD0`, 1 address.** The `psq_l` body from `docs/QUALITY.md` §5.3:
   its arithmetic is three copies of a `GQR0` type ladder and six `ldexpf`
   artefacts. Left to a re-export with the GQR values set, as §5.3 prescribes.
4. **`0x802DFF64`, 1 address.** The `%a`/`%f` formatter, same `ldexpf`/`GQR0`
   problem, 968 bytes of type ladders. Same disposition.

Counts: 32 + 1 + 1 + 1 = 35, and 35 + 7 landed = the 42 Wave 96 left behind.

## Verification

- Every address and byte size was re-read from `functions.tsv` for this pass.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every one of the 7 rows has exactly one body in `src/runtime/cpp.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error in `src/runtime/cpp.rs`;
  the file's only warnings are two pre-existing never-read warnings outside this
  pass's section.
- `node tools/check-lane-csv.mjs wave99` reports `wave99-cpp.csv: 7 rows` and
  `OK`.
