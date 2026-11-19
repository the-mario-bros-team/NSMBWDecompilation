# Coverage - Wave 128 (`game::bases`, `0x8073` bucket)

## How to read this file

This page documents the Wave 128 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader picks the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the same warning line. Cross-module
pointers (`0x8037xxxx`, `0x8042xxxx`) keep their absolute `main.dol` values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv` for this pass.

The `0x8073` bucket holds 183 functions. Before this pass 87 were translated,
leaving 96; this pass takes 30 of them, from `0x80730BE0` to `0x8073FDD0`.
Four clusters were read end to end; 66 addresses in the bucket remain, and the
ones that were read and rejected are listed under "Deliberate skips".

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or a local record
layout had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

One correction matters for anyone reading the older bodies in this file:
`0x80429E00` is a **pointer** to the scene object. The raw words at
`0x80731F38` are `lis r5,0x8043; lwz r5,-0x6200(r5); lfs f2,0x38(r5)`, so the
camera fields are `*(0x80429E00) + 0x38..0x44`. This pass reads it that way and
does not edit the few earlier bodies that used direct address arithmetic.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,028 | 17 | 13 |

All 30 rows are `translated`; none is `partial`. The module header now reports
3260 bodies.

## What this pass covers

- **`0xA08` rotating-dokan state machine (7 bodies).**
  `0x80730BE0..0x80732230`: the post-construction arm rebuild with its five
  `0x8007FB10` binds and the `0x807316F0` callback (`fn_80730be0`); the six
  `1.0f` scale words plus the `+0x9E4` countdown reset (`fn_80731c60`); the
  scale clamp that is the exact twin of the translated `fn_8072e800`
  (`fn_80731cb0`); the camera-rectangle visibility test (`fn_80731f30`); the
  countdown handler that arms descriptor `0x80830CA0` (`fn_80732010`); and the
  two `0x200`-step arm sweeps that end on `0x80830CE0` and `0x80830C60`
  (`fn_807320f0`, `fn_80732230`). Those three descriptors are the ones
  `fn_807323d0` registers, so the state attribution is literal.
- **Release templates (3 bodies).** `0x80733790..0x807347D0`: the two-holder
  release that returns `1` (`fn_80733790`), the full `+0x438`/`+0x51C` teardown
  with the `0x80162920` free (`fn_80734400`), and the registered-state delete
  pair `0x8015F800`/`0x802B9280` (`fn_807347d0`).
- **`0x468` board actor (9 bodies).** `0x8073B5C0..0x8073C700`: the constructor
  with the two vtables (`fn_8073b5c0`); the start reset with its `0xE000`/
  `0x2000` angle and the `0x8008C3E0` setup (`fn_8073b6a0`); the matrix build
  from position plus three angles (`fn_8073b8b0`); the rider gate
  (`fn_8073bb50`); the spawn reset with the camera-derived height
  (`fn_8073bc00`); the per-frame camera follow with its `21.0` travel latch and
  `3000.0` clamp (`fn_8073bcc0`); the `0x222` position sound (`fn_8073be40`);
  the `16.0`/`736.0` box vector (`fn_8073c3c0`) and the `+0x430` model release
  (`fn_8073c700`).
- **Tile board (11 bodies).** `0x8073F7D0..0x8073FDD0`: the four `0xC`-byte
  lane records at `+0x450`, the sixteen `0x20`-byte tiles at `+0x480..+0x680`,
  the `+0x40C` player-list fill, and the allocate / invalidate / decay /
  occlusion helpers (`fn_8073f990` through `fn_8073fdd0`), plus the isolated
  `g3d/tarzantsuta.brres` loader at `0x8073F8E0` and the second delete template
  at `0x8073F7D0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80730BE0` | `fn_80730be0` | 548 | medium |
| `0x80731C60` | `fn_80731c60` | 68 | high |
| `0x80731CB0` | `fn_80731cb0` | 88 | medium |
| `0x80731F30` | `fn_80731f30` | 128 | high |
| `0x80732010` | `fn_80732010` | 164 | high |
| `0x807320F0` | `fn_807320f0` | 280 | high |
| `0x80732230` | `fn_80732230` | 232 | high |
| `0x80733790` | `fn_80733790` | 60 | high |
| `0x80734400` | `fn_80734400` | 140 | high |
| `0x807347D0` | `fn_807347d0` | 88 | high |
| `0x8073B5C0` | `fn_8073b5c0` | 136 | medium |
| `0x8073B6A0` | `fn_8073b6a0` | 296 | medium |
| `0x8073B8B0` | `fn_8073b8b0` | 220 | high |
| `0x8073BB50` | `fn_8073bb50` | 164 | medium |
| `0x8073BC00` | `fn_8073bc00` | 168 | medium |
| `0x8073BCC0` | `fn_8073bcc0` | 384 | medium |
| `0x8073BE40` | `fn_8073be40` | 96 | medium |
| `0x8073C3C0` | `fn_8073c3c0` | 40 | high |
| `0x8073C700` | `fn_8073c700` | 40 | high |
| `0x8073F7D0` | `fn_8073f7d0` | 64 | high |
| `0x8073F8E0` | `fn_8073f8e0` | 172 | high |
| `0x8073F990` | `fn_8073f990` | 32 | high |
| `0x8073F9B0` | `fn_8073f9b0` | 40 | high |
| `0x8073F9E0` | `fn_8073f9e0` | 192 | medium |
| `0x8073FAA0` | `fn_8073faa0` | 156 | medium |
| `0x8073FB40` | `fn_8073fb40` | 76 | high |
| `0x8073FB90` | `fn_8073fb90` | 296 | medium |
| `0x8073FCC0` | `fn_8073fcc0` | 32 | high |
| `0x8073FCE0` | `fn_8073fce0` | 232 | medium |
| `0x8073FDD0` | `fn_8073fdd0` | 396 | medium |

**Totals:** 30 bodies, 5,028 bytes (17 high, 13 medium).

## Deliberate skips

Read and rejected, with the reason:

- **`0x80730360` (212 bytes).** The bucket head that three translated bodies
  call. It is a `GQR3` user: `unaff_GQR3` feeds the `ldexpf` ladder and the
  `CONCAT22`/`CONCAT31` lane splits that build the two `psq_l` outputs, so the
  quantiser slot is not pinned (`docs/REL_ELF_MAPPING.md` keeps `qr3` as
  class C). Two `Removing unreachable block` warnings sit on the same body.
- **`0x80731FB0` (64 bytes).** The gate body itself is two lines, but its
  `FUN_80731780()` call has no argument in the export and no provable value in
  `r3` (the function takes no parameter at all). Every caller of `0x80731FB0`
  is emitted by address; inventing a receiver for the inner call was not an
  option.
- **`0x8073B1B0` and `0x8073B240`.** Both invoke a virtual slot with the
  receiver and arguments dropped by the export (`+0x4A4` slot `+0xC`; `+0x430`
  and `+0x45C` slot `+0x14`), so no honest call shape exists.
- **`0x8073B440`, `0x8073C120`.** Byte-identical copies of `0x807347D0`; one
  representative was taken.
- **`0x8073DA10`.** Truncated by `halt_baddata` after the `+0x8F4` decrement.
- **`0x802DCBFC` forwarders.** `0x80730A90`, `0x80730AC0`, `0x80732770`,
  `0x807327A0`, `0x80733360`, `0x80733390`, `0x807348F0`, `0x80734920`,
  `0x80735260`, `0x8073A290`, `0x8073A2C0`, `0x8073AB30`, `0x8073AB60`,
  `0x8073B560`, `0x8073B590`, `0x8073C240`, `0x8073C270`, `0x8073ECB0` and
  `0x8073ECE0` carry no Rust-visible behaviour.
- **Everything else.** The remaining 66 untranslated `0x8073` addresses were
  not attempted: the `0x80730BA0`/`0x80731330` leaf helpers, the long
  `0x807310C0..0x807318F0` and `0x80733F70..0x80734E30` state bodies, the
  `0x80736890..0x807390D0` multi-route scene block, the `0x8073AF20` and
  `0x8073C730` model holders, the paired-single converters `0x8073F000` and
  `0x8073F440`, and the stacked-buffer bodies `0x8073F810` and `0x8073FF60`.

## Notes on inference

- [`fn_80730be0`] is `medium`: every table address (`0x807CD6F0` eight-step
  angle ramp, `0x807CD700` pitch pair, `0x807CD708` `+32.0`/`-48.0` height
  pair), the `0xE4` arm stride and the five binder blocks are literal; the
  `0x802DCD60` save thunk is the reason the export prints `void FUN(void)`, so
  the `this` in `r3` is restored from the instruction stream.
- [`fn_80731cb0`] is `medium` only because the C prints both scalars as
  `double`; the call sites and the sibling `fn_8072e800` load single-precision
  constants, so the body is `f32`, with a floor of `1.0f` (`0x807CD710`) rather
  than the sibling's zero.
- [`fn_8073b5c0`], [`fn_8073b6a0`], [`fn_8073bb50`], [`fn_8073bc00`],
  [`fn_8073bcc0`] and [`fn_8073be40`] are `medium`: the class name is not
  recoverable from the REL, so the object is described by its two vtables
  (`0x803017D0` then `0x8080E900`), its `0x468` allocation and its literal
  constants (`21.0` travel latch, `12.0` rise threshold, `1.2`/`-1.2`
  direction step, `0.5`, `2.0`, `3000.0` ceiling). The `0x8008C3E0` animation
  setup and the `0x8019_8660` sound context at `0x80429D5C` are called by
  address.
- [`fn_8073f9e0`], [`fn_8073faa0`], [`fn_8073fb90`], [`fn_8073fce0`] and
  [`fn_8073fdd0`] are `medium`: the lane stride `0xC`, the tile stride `0x20`,
  the `0x450`/`0x480`/`0x680` bounds and the nine `0x800DF860` arguments are
  literal; the meanings of the tile bytes `+0x1C`/`+0x1D` and of the retired
  slot `+0x43C` are inferred from the copy and branch directions.
- The `GQR0`/`ldexpf` pairs printed at the head and tail of `0x8073FAA0` and
  `0x8073FB90` return their value to nothing; they are the prologue/epilogue
  spill noise described in `docs/REL_ELF_MAPPING.md` and are omitted.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,028 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 87 translated
  addresses were re-read first and none is duplicated here.
- `node tools/check-lane-csv.mjs wave128` reports `OK`; `wave128-bases.csv` has
  30 rows and 30 unique addresses.
- `cargo check --message-format=short` reports no error naming
  `src/game/bases.rs`.
