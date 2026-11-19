# Coverage - Wave 114 (UI: the last `0x80170000` text-run page body)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/menu.rs` | `ui::menu` | 1 | 612 | 0 | 1 |

## What this pass covers

- **Text-run page draw** (`0x801701A0`): the last untranslated address of the
  `0x80170000` bucket. It takes one page object, opens a console-stream text
  pass in a stack buffer through `0x80230780`, rejects the page when `+0x15`
  is zero or `+0x04` is zero (closing the pass with `0x802307E0(page, -1)`),
  re-opens the pass with `0x8022D5C0(ctx, 0, 0)` and then walks the embedded
  record list at `page + 8` through `0x80228FF0`.
- **Two draw passes per record**: the first sets the stream's `+0x44` scale to
  `draw_scale * record[+0x10]`, writes the flag `record[+0x1C] != 0` to
  `+0x43`, stages the `+0x0C` colour word in the four bytes at
  `ctx + 0x18..0x1B`, commits it through `0x8022D9A0`/`0x8022C9C0`, adds
  `shift_scale * scale` to both halfword positions and runs `0x80231870` over
  the `+0x1E` payload. The second pass repeats with the `+0x08` colour word,
  the plain positions and the same payload/length.
- **Aging and retirement**: the `+0x04` halfword counter is decremented; when
  it drops below one the record is unlinked and released through
  `0x801705E0`, and the walk continues with the next list node.
- **Prior skip resolved**: wave 108 skipped this address because the export
  prints the paired-single probes as discarded `ldexpf` calls and truncates
  the stream-call argument lists. Wave 110's translation of the byte-for-byte
  twin `0x8016FA10` (`src/resource/registry.rs`) supplies the missing model:
  the same `r2 - 0x5348` / `r2 - 0x5344` scales, the same `+0x43`/`+0x44`
  stream fields, the same `+0x18..+0x1B` colour staging and the same
  `0x43300000` halfword conversion. This pass keeps only the three literals
  that actually differ: the constructors (`0x80230780`/`0x802307E0`), the
  payload offset (`+0x1E` instead of `+0x1D`) and the halved `+0x06` run
  length.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801701A0` | `fn_801701a0` | 612 | medium |

## Deliberate skips

None. The only open address in the `0x80170000` bucket was `0x801701A0`, and
it is now translated. The `ldexpf` probes and the three unreachable epilogue
blocks the export reports are decompiler renderings of the paired-single
conversions; they have no observable effect and are omitted exactly as the
`0x80231870` translation in `src/resource/bres.rs` documents.

## Verification

- Every address and byte size above matches `work/ghidra/main-gb/export/functions.tsv`
  (`801701a0`, `612` bytes).
- Every evidence file exists at `work/ghidra/main-gb/export/801701a0_FUN_801701a0.c`.
- The row has exactly one `pub unsafe fn fn_801701a0` body in `src/ui/menu.rs`.
- `0x801701A0` was absent from `ledger/functions.csv` before this pass.
- `cargo check --message-format=short` reports no errors for `src/ui/menu.rs`.
- `node tools/check-lane-csv.mjs wave114` reports no problem for this lane.
