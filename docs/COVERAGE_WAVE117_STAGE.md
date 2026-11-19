# Coverage - Wave 117 (`game::stage`, closing the `0x80080000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave117-stage.csv` is the Ghidra C export the body was
written from.

The bucket held 222 entries and 220 already had `game::stage` rows, so wave 117
starts from the two entries wave 108 left: the filled-tile rectangle
`0x80084690` and the 4-byte branch thunk `0x80083B30`. The rectangle is landed
here and the thunk is deliberately skipped, which closes the bucket.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 1 | 4328 | 0 | 1 | 0 |

The row is `translated`; it is not `partial` or `stub`.

## What this pass covers

- **The filled-tile rectangle** (`0x80084690`, 4328 bytes). The body
  [`fn_80084690`] is the eight-arm dispatcher behind
  [`bg_tile_run_fill_wide`] (`0x800859F0`): it reads the direction byte at
  `*(u8*)desc+0xC00`, switches on `0x80..0x83` and `0x90..0x93` through the jump
  table at `0x80310348`, and fills one axis-aligned block of the bgdat layer.
- **The nine-word blit descriptor.** `0x800859F0` builds three sections, each
  as `(row table, width, height)`: words `0..2` (section 0, the body), `3..5`
  (section 1) and `6..8` (section 2). A section's row table stores the byte
  offset of one row in the layer stream, its height is both the row-loop bound
  and the ring size (`if (index == height) index = 0`), and word 4 is the
  section-1 run width. The body lands the `0x82`/`0x91`/`0x92` variants in one
  arm each so the shared row walk is written once.
- **The two truncated call sites, resolved from code bytes.** Wave 108 skipped
  this function because the export prints `FUN_80083fc0(uVar2,iVar4,local_28)`
  without the fourth argument at `0x800848BC` and `0x80084B0C`. The code bytes
  show that `r6` is not live-in there: it still holds `lhz r6,0xC1A(r28)` from
  `0x800848A4` and `0x80084B04`, so both calls pass the `+0xC1A` width. Those
  are the `width < x` arms of the `min(x, width)` clip, which is why the length
  looked undefined.
- **The prologue pair.** `0x802DCD68`/`0x802DCDB4` are `__savegprlr_24`/
  `__restgprlr_24` (the same artefact wave 116 documented). The export's
  `iVar4`/`uVar2` are the saved `r4`/`r3`, so the real argument order is
  `r3` owner, `r4` desc, `r5` x, `r6` y, `r7` layer; the registers are restored
  at `0x800846A8..0x800846BC` before the dispatch.
- **Shared helpers.** The body reuses [`w108_tile_emit`] (the
  `0x80083EC0`/`0x80083E20`/`0x80086B20` triple) and calls `0x80083FC0` through
  a small wrapper, because the run builder writes the terminator offset back
  into a stack word the original passes by address.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80084690` | `fn_80084690` | 4328 | medium |

Address range: `0x80084690` (1 body, 4328 bytes).

## Deliberately skipped

- **`0x80083B30`** (`thunk_FUN_8016e980`, 4 bytes). The entry is a 4-byte
  branch into `0x8016E980`, which already owns a `resource::registry` row
  (`high`, `work/ghidra/main-gb/export/8016e980_FUN_8016e980.c`). Its export is
  the target body, so a second body here would double-count the same
  instructions - the precedent of `0x800776F0` in wave 115 and `0x800C3BA0` in
  wave 116.

One landed + one skipped = the two open entries of the `0x80080000` bucket,
which is now closed (222/222).

## Verification

- Every address and byte size matches `functions.tsv` (1 row, 4328 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_80084690` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave117` reports this CSV clean.
- The module header of `src/game/stage.rs` gained a wave-117 paragraph naming
  the body and the skip (222 entries, 220 translated before this pass), a
  wave-117 line in the original-range list, and the wave-108 paragraph now
  points at the resolution instead of claiming the run length is undefined; the
  `BG_TILE_AREA_FILL` constant no longer says "untranslated".
