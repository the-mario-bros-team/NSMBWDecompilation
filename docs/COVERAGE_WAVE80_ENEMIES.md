# Coverage - Wave 80 (enemies: `0x80AD0000` bucket continuation)

## How to read this file

This page records the Wave 80 `game::enemies` lane: a second pass over the
`0x80AD0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to one real
`pub unsafe fn fn_<addr>` body in that file, and the evidence column of the
machine-readable mirror (`ledger/agents/wave80-enemies.csv`) names the Ghidra
C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32` / `read_u16`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 4,872 | 16 | 9 |

Four rows are `low` confidence: the 48-byte `0x802DCBFC` forwarders, whose
hidden-`r12` this-adjust is not modelled. The bucket holds 189 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 63 were already in
`ledger/functions.csv` before this pass, 29 are claimed here and 97 remain.

## What this pass covers

Two clusters, both contiguous apart from the four functions skipped for
paired-single (`GQR`) noise (see below).

- **Run 1 (`0x80AD6200..0x80AD8A60`, fifteen bodies).** The `0x4C30` container
  family opened in wave 65. Three further allocating constructors -
  `0x4CE0`, `0x4D40` and `0x4D88` bytes - repeat the family header
  verbatim: the intermediate vtable `0x80B5C250` at `+0x60`, the `+0x524`
  model block, the `+0x544` owner, the four method tables at
  `+0x564..+0x570` with the back-pointer at `+0x574`, the `+0x57C`
  sub-object, the `+0x5DC` animation object, the `+0x6DC` model block and
  the `0x2290`-element `+0x6F8` container. They differ only in size, final
  vtable (`0x80B5D494` / `0x80B5C9E4` / `0x80B5C738` / `0x80B5D74C`) and in
  the tail: a single `0xA4`-byte node at `+0x4C30` with its holder at
  `+0x4CD4`, or a real two-element `0xA4` array plus a two-element `8`-byte
  array at `+0x4D78` whose constructor/destructor pair is the bare `blr` stub
  `0x80A2FC30` and `0x80A00240`.
  Around the constructors sit the deleting-destructor copy, two 48-byte
  forwarders, the two node-height steps (`+0x4D20`/`+0x4D24` from the signed
  side byte `+0x4C2C` scaled by `16.0f`), the dual-node step of a larger
  sibling, the `0x4C30` instance's frame step, its placement helper and the
  follow-target search/compute/enter trio - the same code shape as the coin
  block `0x80A42810..0x80A42BC0`, including the `2 x 0xAC/0xB0` offset and the
  two `0x807073D0` angle corrections.
- **Run 2 (`0x80AD8E90..0x80AD9D10`, fourteen bodies).** The `0x640`-byte
  actor that loads `g3d/ben.brres` and binds the model `ben` with the per-side
  clip table `ben_right`/`ben_left` (two strings at `0x80B5DE04`/`0x80B5DE10`,
  pointer table at `0x80B5DE20`). Its constructor, per-side binder, the
  lazily-filled four-direction tables at `0x80B76940` and `0x80B76968`/
  `0x80B76990`, the `0x8007FA40` animator install with its three contact
  callbacks, and the state entries they drive: the `+0x550`/`+0x348`
  direction/side gates, the `+0xAC` x comparisons, and the two turn steps
  built on the `-2560`/`-16384` and `2560`/`16384` `s16` pairs.

### Identity evidence

- **The family link is the shared header and the `0xA4` node.**
  `0x80AD69A0` creates the `+0x4C30` array with element size `0xA4`, and
  `0x80AD6380` initialises the holder at `+0x4CD4`, which is exactly
  `0x4C30 + 0xA4` - the second node. Wave 65's [`fn_80ad4f40`] builds the
  same header into a `0x4C30` allocation.
- **The ben strings were read, not guessed.** `0x80B5DE28` is
  `"g3d/ben.brres"`, `0x80B5DE38` is `"ben"`, `0x80B5DE20` holds
  `0x80B5DE04` and `0x80B5DE10`, which are `"ben_right"` and `"ben_left"`.
- **The table values were read from the REL image.**
  `0x80B309C4 = 0.0f`, `0x80B309D4 = 16.0f`, `0x80B309F0 = 8.0f`,
  `0x80B309F8 = 16.0f`, `0x80B30A08/0A0C/0A10 = 0.0f/-16.0f/16.0f`,
  `0x80B30A20/0A24/0A28/0A2C/0A30/0A34 = -8.0f/40.0f/-40.0f/8.0f/-32.0f/32.0f`;
  the `s16` pairs are `0`/`-32768` at `0x80B30A18`, `-2560`/`-16384` at
  `0x80B30A38`/`0x80B30A3C` and `2560`/`16384` at `0x80B30A40`/`0x80B30A44`.
  The bodies load them; no float or halfword is hard-coded.
- **The direction tables are BSS-resident.** `0x80B76938..0x80B76A3C` lie in
  BSS, so the initialisation stores are reproduced verbatim and the docs say
  the guard byte is a one-shot latch.
- **The `0x80B767C8`/`0x80B76848`/`0x80B76888`/`0x80B768C8`/`0x80B76908`
  words are BSS name keys.** They carry no bytes in the image, so the docs
  cite the address only and call them name words, not strings.
- **The two `0x802DCBFC` forwarders** are the same argument reshuffle already
  documented for this module; their hidden `r12` this-adjust is not modelled
  and each doc says so.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AD6200` | `fn_80ad6200` | 88 | high |
| `0x80AD6320` | `fn_80ad6320` | 48 | low |
| `0x80AD6350` | `fn_80ad6350` | 48 | low |
| `0x80AD6380` | `fn_80ad6380` | 296 | high |
| `0x80AD6880` | `fn_80ad6880` | 288 | high |
| `0x80AD69A0` | `fn_80ad69a0` | 344 | high |
| `0x80AD6ED0` | `fn_80ad6ed0` | 128 | medium |
| `0x80AD6F50` | `fn_80ad6f50` | 116 | medium |
| `0x80AD7260` | `fn_80ad7260` | 176 | medium |
| `0x80AD7EA0` | `fn_80ad7ea0` | 280 | high |
| `0x80AD83A0` | `fn_80ad83a0` | 272 | medium |
| `0x80AD8720` | `fn_80ad8720` | 112 | high |
| `0x80AD8790` | `fn_80ad8790` | 140 | medium |
| `0x80AD8820` | `fn_80ad8820` | 224 | medium |
| `0x80AD8A60` | `fn_80ad8a60` | 92 | medium |
| `0x80AD8E90` | `fn_80ad8e90` | 88 | high |
| `0x80AD8FB0` | `fn_80ad8fb0` | 48 | low |
| `0x80AD8FE0` | `fn_80ad8fe0` | 48 | low |
| `0x80AD9010` | `fn_80ad9010` | 108 | high |
| `0x80AD9130` | `fn_80ad9130` | 184 | medium |
| `0x80AD93C0` | `fn_80ad93c0` | 140 | high |
| `0x80AD9450` | `fn_80ad9450` | 28 | high |
| `0x80AD9470` | `fn_80ad9470` | 272 | high |
| `0x80AD9580` | `fn_80ad9580` | 224 | high |
| `0x80AD9660` | `fn_80ad9660` | 224 | high |
| `0x80AD9740` | `fn_80ad9740` | 364 | high |
| `0x80AD9B70` | `fn_80ad9b70` | 200 | high |
| `0x80AD9C80` | `fn_80ad9c80` | 92 | medium |
| `0x80AD9D10` | `fn_80ad9d10` | 200 | high |

## Deliberate skips

- **`0x80AD6CA0`** (100 bytes) is the `+0x4CD4` effect spawn. Its export
  shows a ten-argument call (`0x800DB180`) with five `double` parameters
  (`0.0, 0.0, 0.0, 0.0, -22.0`) followed by five words. Whether the callee
  really takes doubles or the Ghidra signature widened single-precision
  arguments cannot be settled from the C export, and the module has no
  ten-argument ABI adapter. Left rather than guessed.
- **`0x80AD7310`** (140 bytes) is a two-holder transform step whose export
  carries `GQR0` paired-single noise (`ldexpf` on a fake register) and a
  `double` first parameter, so the argument ABI is not certain.
- **`0x80AD8900`** (308 bytes) is the follow-target position step read by
  `fn_80ad83a0`. Every float pair comes from `GQR3`-paired-single registers
  that Ghidra never binds; the caller keeps it as a bare address
  (`EN80_FN_80AD8900`).
- **`0x80AD98B0`** (544 bytes) is the four-player overlap test. Its export has
  five `GQR0`-noise blocks around the float comparisons, so its ABI cannot be
  written faithfully; `fn_80ad9c80` keeps it as a bare address
  (`EN80_FN_80AD98B0`).
- **The rest of the bucket.** `0x80AD0960`, `0x80AD0A90`,
  `0x80AD3C10..0x80AD48C0` (13 functions, several with `GQR0` noise),
  `0x80ADA1A0..0x80ADAFC0`, `0x80ADB3D0..0x80ADCC80`,
  `0x80ADD0E0..0x80ADE450` and `0x80ADE970..0x80ADFDD0` are ordinary clean
  exports but lie outside this lane's 29-body budget. They are left for a
  later pass in the same bucket.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  4,872 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 63 existing rows are untouched).
- `node tools/check-lane-csv.mjs wave80` reports
  `wave80-enemies.csv: 29 rows`, 29 unique addresses and `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`.
