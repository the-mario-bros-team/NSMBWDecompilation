# Coverage - Wave 108 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer: the `Pausewindow_c` state machine, the `dGameDisplay_c`
descriptors, the `Wm_2d_*` effects and the engine band
`0x800D0000..0x80200000`. This pass works the last open bucket, `0x800D0000`:
it entered with 11 untranslated addresses of its 251 and leaves 1. "Bodies"
counts `pub unsafe fn` functions; "bytes" is the sum of the Ghidra sizes re-read
from `work/ghidra/main-gb/export/functions.tsv` for this pass.

"high" means every store, branch and call is visible in the export. "medium"
means one field role or callee shape had to be recovered from the export plus
the surrounding call sites, and that recovery is spelled out in the body's
`# Confidence` block.

Only `src/engine/display.rs` was edited. `ledger/functions.csv` is read-only and
the coordinator merges this lane's rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 10 | 4948 | 0 | 10 |

## What this pass covers

Four clusters, all in `0x800D0610..0x800DFDD0`:

- **The four-view effect transform** (`0x800D0610`, 656 bytes).
  [`fn_800d0610`] walks the four view objects at `0x80377C88`, projects each
  live view into one 0x20-byte entry at `0x80370D10`, computes the view size,
  length and negated heading, then masks the three transform words when the
  layout mode is `1` or `5`. The active entry pointer is published at
  `r13 - 0x7A08`. The five SDA2 floats at `r2 - 0x66A0..-0x66B0` are read by
  address and their scale/offset roles are marked `// inferred:`.
- **The projected cursor/effect grid** (`0x800D19E0..0x800D27B0`, 976 + 176 +
  276 + 560 bytes). [`fn_800d19e0`] sweeps a height through the state's width
  and angle arrays and places a manager record on the first matching cell;
  [`fn_800d2460`] projects one angle and radius onto a target owner;
  [`fn_800d2690`] copies the owner position and projects every column; and
  [`fn_800d27b0`] clamps every linked record onto the four SDA2 screen bounds.
  The four bodies share the same `+0x28` position / `+0x2C` width / `+0x30`
  angle arrays and the same `0x80237BD0`/`0x80237C40` angle pair.
- **The four-player star-colour layout** (`0x800D3380`, 860 bytes).
  [`fn_800d3380`] resolves two handles per row through `0x80239E30`, builds the
  left/right widgets, reads four animation records through `0x8023B2A0` and
  stores their heading and squared length at `+0x7A4..+0x7F0`. The shared
  handle at `r13 - 0x79F8` and the `PB_star_color` / `PH_star_color` name
  tables selected by `+0x180` finish the layout.
- **The effect dispatch and ground sampler** (`0x800D6550`, `0x800D9D70`,
  `0x800DAA10`, `0x800DFDD0`, 68 + 952 + 292 + 132 bytes).
  [`fn_800d6550`] maps the `+0x154` selector to one of two rows of the
  `0x80370DCC` vector table and forwards it through vtable slot `+0x64`.
  [`fn_800d9d70`] tests one point against a ground band and writes the shadow
  record; [`fn_800daa10`] walks the actor list at `r13 - 0x5664` and returns
  the deepest band hit through the wave-99 filters; [`fn_800dfdd0`] returns the
  record at a given index of the `+0x60`/`+0x64` list.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800D0610` | `fn_800d0610` | 656 | medium |
| `0x800D19E0` | `fn_800d19e0` | 976 | medium |
| `0x800D2460` | `fn_800d2460` | 176 | medium |
| `0x800D2690` | `fn_800d2690` | 276 | medium |
| `0x800D27B0` | `fn_800d27b0` | 560 | medium |
| `0x800D3380` | `fn_800d3380` | 860 | medium |
| `0x800D6550` | `fn_800d6550` | 68 | medium |
| `0x800D9D70` | `fn_800d9d70` | 952 | medium |
| `0x800DAA10` | `fn_800daa10` | 292 | medium |
| `0x800DFDD0` | `fn_800dfdd0` | 132 | medium |

Byte total: `656 + 976 + 176 + 276 + 560 + 860 + 68 + 952 + 292 + 132 = 4948`.

## Deliberate skips

One address of the bucket stays untranslated.

**Unrecoverable vtable argument shape (1 address).** `0x800D43E0` (312 bytes)
contains two calls through `*(*param_4 + 0x28)`. The first passes `param_4` as
the receiver; the second passes `*(undefined8 *)(r2 - 0x6638)` as its first
argument and `param_4` as its second, and its result is then forwarded to
`0x80169F80` together with `param_4 + 0x98`. The export also builds a dead
`0x43300000` stack pair that no later instruction reads. Neither the receiver
nor the argument type of that second call can be recovered from the evidence,
so any Rust body would invent a calling convention; the address is left for a
pass that can read the raw instruction words.

## Verification

- Every address and byte size matches `functions.tsv`, re-read for this pass.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/display.rs`.
- No address was already in `ledger/functions.csv`; the bucket had 11
  untranslated functions before this pass and 1 after it.
- `node tools/check-lane-csv.mjs wave108` reports this lane clean.
- `cargo check --message-format=short` reports no error naming
  `src/engine/display.rs`; the remaining crate errors name other lanes' files
  that were being edited in parallel.
