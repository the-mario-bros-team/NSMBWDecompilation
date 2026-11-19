# Coverage - Wave 101 (`game::bases`, `0x8073` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the
`d_basesNP` REL. Every address on this page is a **static analysis address**
under the canonical synthetic base **`0x80600000`** (produced by
`tools/rel2elf.mjs`), never a runtime address: the Wii REL loader chooses the
real load address, so the numbers only identify positions inside the REL image.
In this image a virtual address maps to the REL file at
`file_offset = address - 0x80600000`; that is how the float constants cited
below were checked. Cross-module pointers (`0x8035xxxx`, `0x8042xxxx`) keep
their absolute values, and data/name pointers at `0x8081xxxx..0x8083xxxx` are
relocated at load time.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (checked with a glob, never
assumed). "Confidence" is `high` when guards, stores, constants and call
arguments are literal, and `medium` when a field role, a global, a callee or a
vtable slot had to be inferred (those bodies carry `// observed:` /
`// inferred:` / `// unknown:` notes). No body in this pass is `low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 4,424 | 16 | 12 |

## What this pass covers

The `0x8073` bucket holds 183 functions; 59 were already in
`ledger/functions.csv` when this pass began, leaving 124. This pass takes 28 of
them: one contiguous window plus four standalone helpers.

- **The unnamed `0x8073` course actor, collision and callback half
  (`0x80738050..0x80739950`, 24 bodies).** The class has no name string in its
  window; its identity is carried by the `0x8083` name objects it plays and by
  the `0x8073A2F0`-style neighbours. Its layout, as proven by the bodies below,
  is: class vtable `+0x60`, NW4R animator `+0x394` (slot `+0x18` starts a clip,
  the `+0x28` state object answers "is this clip playing?" through slot
  `+0x14`), clip interface `+0x434`, model holder `+0x3F0`, motion object
  `+0x1EC`, three 16-byte collision records at `+0x46C`/`+0x47C`/`+0x48C`, a
  bitfield pair at `+0x4AC`/`+0x590`, state byte `+0x38C`, character byte
  `+0x38D`, ground word `+0xEC` and half extents `+0x464`/`+0x468`.
  - teardown and model shutdown: [`fn_80738050`], [`fn_807380c0`],
    [`fn_80739650`];
  - player binding and character rebinding: [`fn_807383f0`],
    [`fn_80739080`], the character resolver `0x8005F900` used by both;
  - collision-record builders: [`fn_80738580`] (kind `0x81`, `4096.0f`
    fixed point, `4.0f` margin) and [`fn_80738690`] (kind `1`, flat), both
    handing the three records to `0x8006D090`;
  - the three touch callbacks [`fn_80738950`] (hit test [`fn_807387a0`]),
    [`fn_80738a40`] (always-false [`fn_80738890`]) and [`fn_80738b30`]
    (animator test [`fn_807388a0`]), all sharing the "both clips idle" gate and
    the clip restarter [`fn_80738780`];
  - the mounted state machine: child-list scan [`fn_80738d40`], landing latch
    and sound `0x26F` [`fn_80738f00`], flat reset [`fn_80738ff0`], pose reset
    [`fn_80739300`], driven frame and sound `0x26D` [`fn_807394b0`], release
    and sound `0x26C` [`fn_807395b0`], the direction-toggling twins
    [`fn_807396f0`]/[`fn_807397e0`], and the 60-frame release
    [`fn_80739880`]/[`fn_80739950`].
- **Four standalone helpers at `0x8073A170..0x8073AB90` (4 bodies).** The two
  release wrappers [`fn_8073a170`]/[`fn_8073aa10`] (the same
  `0x8015F800` + `0x802B9280` shape as the wave-99 [`fn_807a80d0`]), the
  transform copy [`fn_8073a6e0`] (twelve words plus the two angle applies) and
  the `0x398`-byte constructor [`fn_8073ab90`] with class table `0x8080E64C`.

Start with [`fn_80738950`] for the callback contract (it shows the clip
queries, the `0xE` mask and the hit test in one body), [`fn_80738580`] for the
collision records, and [`fn_807394b0`] for the sound/lifecycle pattern.

## Address range

`0x80738050..0x80739950` in one contiguous window (24 of its 28 functions) plus
`0x8073A170`, `0x8073A6E0`, `0x8073AA10` and `0x8073AB90`. After this pass the
`0x8073` bucket still has 96 untranslated functions, mostly the paired-single
bodies and the `0x8073B000..0x8073FF60` block.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80738050` | `fn_80738050` | 112 | high |
| `0x807380C0` | `fn_807380c0` | 48 | medium |
| `0x807383F0` | `fn_807383f0` | 112 | medium |
| `0x80738580` | `fn_80738580` | 268 | high |
| `0x80738690` | `fn_80738690` | 240 | high |
| `0x80738780` | `fn_80738780` | 24 | high |
| `0x807387A0` | `fn_807387a0` | 240 | medium |
| `0x80738890` | `fn_80738890` | 8 | high |
| `0x807388A0` | `fn_807388a0` | 172 | medium |
| `0x80738950` | `fn_80738950` | 236 | high |
| `0x80738A40` | `fn_80738a40` | 236 | high |
| `0x80738B30` | `fn_80738b30` | 252 | high |
| `0x80738D40` | `fn_80738d40` | 108 | medium |
| `0x80738F00` | `fn_80738f00` | 228 | medium |
| `0x80738FF0` | `fn_80738ff0` | 132 | medium |
| `0x80739080` | `fn_80739080` | 80 | high |
| `0x80739300` | `fn_80739300` | 348 | medium |
| `0x807394B0` | `fn_807394b0` | 252 | medium |
| `0x807395B0` | `fn_807395b0` | 132 | medium |
| `0x80739650` | `fn_80739650` | 68 | high |
| `0x807396F0` | `fn_807396f0` | 156 | high |
| `0x807397E0` | `fn_807397e0` | 156 | high |
| `0x80739880` | `fn_80739880` | 188 | high |
| `0x80739950` | `fn_80739950` | 140 | high |
| `0x8073A170` | `fn_8073a170` | 88 | high |
| `0x8073A6E0` | `fn_8073a6e0` | 236 | medium |
| `0x8073AA10` | `fn_8073aa10` | 88 | high |
| `0x8073AB90` | `fn_8073ab90` | 76 | medium |

**Totals:** 28 translated, 0 partial, 0 stub (16 high, 12 medium).

## Deliberate skips

Four functions inside the same window were read and rejected rather than
guessed:

- `0x807382F0` (256 bytes) and `0x80738460` (276 bytes) are the two record
  builders for the mirrored/derived states. Both open with a
  `unaff_GQR0`/`ldexpf` paired-single save bracket before the first real
  statement, and `0x80738460` also has an unreachable block removed by Ghidra
  at `0x80738554`. The incoming PSQ registers cannot be reconstructed, so a
  Rust body would have to be invented.
- `0x80738DB0` (248 bytes) and `0x807390D0` (548 bytes) read the caller's frame
  pointer through the `0x802DCD74`/`0x802DCDC0` save/restore pair and then treat
  the high half of the returned 64-bit value as the actor. That value is an
  artifact of the paired-single save bracket, not an argument; the same trap is
  documented for `0x8077CED0` in `docs/COVERAGE_WAVE100_BASES.md`.
  `0x807390D0` additionally needs `countLeadingZeros` and the dead
  `unreachable` block at `0x80739384` interpreted.
- `0x807373B0` (844 bytes), the neighbour that paints floor tiles through
  `0x80077860`, is the same paired-single failure plus four removed unreachable
  blocks; it was already rejected by wave 30 as well.

The remaining 92 untranslated functions of the bucket lie outside this pass's
selected window (budget, not doubt); `0x807373B0` is one of them.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (28 rows, 4,424 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 59 translated
  addresses were re-read first and none is duplicated here.
- The constants cited in [`fn_80738580`], [`fn_80738690`], [`fn_80739300`] and
  [`fn_80739880`] were read from the REL bytes at
  `file_offset = address - 0x80600000` (`0x807CDBA8` = 1.0f,
  `0x807CDBD0` = {1.0, -1.0}, `0x807CDBE0` = 0.5f, `0x807CDBE4` = 0.0f,
  `0x807CDBF0` = 4096.0f, `0x807CDBF4` = 4.0f, `0x807CDC10` = {3.0, -3.0},
  `0x807CDC18` = {4.0, -4.0}, `0x807CDC24` = 0.0625f,
  `0x807CDC28` = -1.0f, `0x807CDC2C` = 2.0f, `0x807CDC4C` = 1/60,
  `0x807CDC50` = 60.0f).
- `node tools/check-lane-csv.mjs wave101` reports
  `wave101-bases.csv: 28 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; the crate's remaining errors belong to the other
  wave-101 lanes (`ui/hud.rs`, `game/worldmap.rs`, `game/enemies.rs`,
  `runtime/os.rs`).
