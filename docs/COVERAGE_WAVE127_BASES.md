# Coverage - Wave 127 (`game::bases`, `0x807A` tail)

## How to read this file

This page documents the Wave 127 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the same warning line. Cross-module
pointers (`0x8037xxxx`, `0x8042xxxx`) keep their absolute `main.dol` values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

The `0x807A` bucket holds 212 functions. Before this pass 114 were translated,
leaving 98; this pass takes 28 of them, from `0x807A06B0` to `0x807AD8C0`.
Three contiguous runs were read. The remaining `0x807A` head and tail are
listed under "Deliberate skips".

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or a local byte
layout had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 3,908 | 18 | 10 |

All 28 rows are `translated`; none is `partial`. The module header now reports
3230 bodies.

## What this pass covers

- **`0x807A` ground-actor head (6 bodies).** `0x807A06B0..0x807A10B0`: the event
  dispatcher with its `-1`/`0x5B` handling and vtable `+0x68` call
  [`fn_807a06b0`]; the start reset [`fn_807a07a0`]; the effect spawn
  [`fn_807a0bd0`] with its `0x8081BEC4`/`0x8081BED8` rows, `0x800F69F0` call
  and `+0x780` handle; the landing probe [`fn_807a0d30`]; the contact/frame
  setup [`fn_807a0d80`]; and the model move [`fn_807a10b0`].
- **Shared hint-screen prologue/epilogue (12 bodies).**
  `0x807AA310..0x807AB2D0`: the display-list reset [`fn_807aa310`]; the plain
  release template [`fn_807aab10`] and its two `+0x18` twins
  [`fn_807aab50`]/[`fn_807aabb0`]; the four-player screen copy
  [`fn_807aadd0`]; the nine-slot message mask [`fn_807aae80`]; the
  factory-mask fill [`fn_807aaf40`]; the confirm-page push [`fn_807ab010`];
  the exit-animation start [`fn_807ab0a0`]; the choice commit
  [`fn_807ab100`]; the pause/tempo clear [`fn_807ab290`]; and the next-page
  push [`fn_807ab2d0`].
- **Course-setup scene tail (10 bodies).** `0x807ACCA0..0x807AD8C0`: the
  display-list reset [`fn_807acca0`]; the `0x2BD`/`0x2E3`/`0x2C0` key test
  [`fn_807acd30`]; the per-frame model update [`fn_807acd90`]; the `0x2CF`
  resource allocation [`fn_807acf30`]; the plain and `+0x18` release templates
  [`fn_807ad170`]/[`fn_807ad1b0`]; the 10x10 grid capture [`fn_807ad420`]; the
  exit-page push [`fn_807ad4f0`]; the row/confirm/cancel menu
  [`fn_807ad720`]; and the scene leave [`fn_807ad8c0`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807A06B0` | `fn_807a06b0` | 168 | high |
| `0x807A07A0` | `fn_807a07a0` | 56 | high |
| `0x807A0BD0` | `fn_807a0bd0` | 352 | medium |
| `0x807A0D30` | `fn_807a0d30` | 80 | high |
| `0x807A0D80` | `fn_807a0d80` | 148 | medium |
| `0x807A10B0` | `fn_807a10b0` | 60 | high |
| `0x807AA310` | `fn_807aa310` | 216 | high |
| `0x807AAB10` | `fn_807aab10` | 64 | high |
| `0x807AAB50` | `fn_807aab50` | 96 | high |
| `0x807AABB0` | `fn_807aabb0` | 100 | high |
| `0x807AADD0` | `fn_807aadd0` | 172 | medium |
| `0x807AAE80` | `fn_807aae80` | 188 | high |
| `0x807AAF40` | `fn_807aaf40` | 180 | medium |
| `0x807AB010` | `fn_807ab010` | 108 | medium |
| `0x807AB0A0` | `fn_807ab0a0` | 88 | high |
| `0x807AB100` | `fn_807ab100` | 352 | medium |
| `0x807AB290` | `fn_807ab290` | 56 | high |
| `0x807AB2D0` | `fn_807ab2d0` | 104 | medium |
| `0x807ACCA0` | `fn_807acca0` | 144 | high |
| `0x807ACD30` | `fn_807acd30` | 96 | high |
| `0x807ACD90` | `fn_807acd90` | 36 | high |
| `0x807ACF30` | `fn_807acf30` | 68 | high |
| `0x807AD170` | `fn_807ad170` | 64 | high |
| `0x807AD1B0` | `fn_807ad1b0` | 96 | high |
| `0x807AD420` | `fn_807ad420` | 200 | medium |
| `0x807AD4F0` | `fn_807ad4f0` | 108 | medium |
| `0x807AD720` | `fn_807ad720` | 384 | medium |
| `0x807AD8C0` | `fn_807ad8c0` | 124 | high |

**Totals:** 28 bodies, 3,908 bytes (18 high, 10 medium).

## Deliberate skips

Read and rejected, with the reason:

- **`0x807A0090` and `0x807A02C0`.** The 560-byte and 428-byte bodies are the
  paired position/aim update and the spawn-probe of the same actor. Both are
  built from `0x800F8130`/`0x800F8050`/`0x800F6410` with stacked `float`
  locals whose addresses are reused across three sub-blocks; the C does not
  expose which of the overlapping locals is live at each call, so no body was
  emitted.
- **`0x807A4450`, `0x807A58E0`, `0x807AB590`.** Each starts with an
  unrecovered stack-byte buffer (`local_27[39]`, `local_38`, `auStack_30`) that
  is filled by `0x8062ED90`/`0x800BB780` and then read back at offsets the C
  declaration order does not pin down. `0x807AD420` is the one grid body whose
  two-byte read-back is explicit (`local_27[0]` and `local_28`); the others
  were left out rather than guessing a layout.
- **Paired-single / GQR conversions.** `0x807ABF50`, `0x807AD590` and the
  `0x807A84B0` long body mix `unaff_GQR0`, `ldexpf` and `CONCAT44` lane
  splits. Their stores are visible but which GQR slot feeds each conversion is
  not decidable, so no body was emitted.
- **`halt_baddata` bodies.** `0x807ABF40` (8 bytes) is truncated by bad
  instruction data.
- **`0x802DCBFC` forwarders and this-adjusting thunks.** `0x807A3820`,
  `0x807A3850`, `0x807A4F00`, `0x807A4F30`, `0x807A6300`, `0x807A6330`,
  `0x807A9B60`, `0x807A9B90`, `0x807AC290`, `0x807AC2C0`, `0x807AC2F0`,
  `0x807ADF40`, `0x807ADF70`, `0x807AE8A0`, `0x807AF1E0`, `0x807AF210` and
  `0x807AFB30` carry no Rust-visible behaviour.
- **Duplicate release copies.** `0x807AD1B0` was taken as the representative
  `+0x18` template; the byte-identical `0x807AD210`, `0x807AE8E0` and
  `0x807AE940` copies were left out to keep the lane at its 28-body budget.
- **Everything else.** The remaining untranslated `0x807A` addresses were not
  attempted: the `0x807A2200..0x807A2CA0` actor tails, the
  `0x807A3D00..0x807A5CC0` state functions, the `0x807A6BE0..0x807A84B0`
  long bodies, `0x807A9B60`/`0x807A9B90`, the three 16-byte stubs
  `0x807AC260`/`0x807AC270`/`0x807AC280`, the `0x807AE400..0x807AEEA0`
  multi-route scene block, and the `0x807AF520..0x807AFF90` result-scene tail.

## Notes on inference

- [`fn_807a0bd0`] and [`fn_807a0d80`] are `medium`: every offset, row address
  and store is literal, but the `0x800F69F0` prototype is reconstructed from its
  eight arguments and the spawn rows are named by address only.
- [`fn_807aadd0`] is `medium`: the copy loop and all five source/destination
  offsets are literal; the `+0xE49`/`+0xE8`/`+0xDDC` roles are inferred from
  the copy direction.
- [`fn_807aaf40`] is `medium`: the manager stride, factory chain and
  `0x80371870` comparison are literal; the factory/vtable roles are inferred
  from the expression shape.
- [`fn_807ab010`], [`fn_807ab2d0`] and [`fn_807ad4f0`] are `medium`: the
  `0x8042A440` virtual slot and descriptors are literal, but the slot meaning
  is inferred. The `clz(x - 1) >> 5` and `clz(x) >> 5` forms are preserved
  verbatim with the `== 1` / `== 0` reading noted.
- [`fn_807ab100`] is `medium`: every branch, mask and descriptor is literal,
  but the message/row field roles are inferred.
- [`fn_807ad420`] is `medium`: the 10x10 strides, the `+0x500` row swap and the
  `0x40` plane bit are literal; only the two-byte layout of the `0x8062ED90`
  output buffer is inferred from the stack declaration order.
- [`fn_807ad720`] is `medium`: every branch, mask and descriptor is literal;
  the `+0x295`/`+0x296`/`+0x297` flag roles are inferred from the branch order.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (28 rows, 3,908 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 114 translated
  addresses were re-read first and none is duplicated here.
- `node tools/check-lane-csv.mjs wave127` reports `OK`; `wave127-bases.csv` has
  28 rows and 28 unique addresses.
- `cargo check --message-format=short` reports no error naming
  `src/game/bases.rs` (the remaining workspace errors are in the parallel
  `game::enemies` lane).
