# Coverage - Wave 68 (enemies: `0x80A15870..0x80A16820`, `0x80A16D90..0x80A17E20`)

## How to read this file

This page records the Wave 68 `game::enemies` lane: two contiguous runs in the
`0x80A10000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to one real
`pub unsafe fn` body in that file, and the `evidence` column names the Ghidra C
export the body was reconstructed from. The CSV that mirrors this table is
`ledger/agents/wave68-enemies.csv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

Data values quoted in this page (the two brick tables and the four-float blocks)
were read from `work/rels/elf/d_enemiesNP.elf`, the same image Ghidra exported
the C from. They are given as literals so a reader does not have to re-open the
binary; the addresses in the code stay symbolic.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 4,656 | 19 | 11 | 0 |

## What this pass covers

The bucket `0x80A10000` holds 209 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 61 of them were already in
`ledger/functions.csv` when this lane started (29 from wave 46's
`obj_turara_rakka_Big` block, 30 from wave 54's `g3d/met.brres` and `Wm_en_iron`
blocks, plus the two early `daEnBlackPakkun_c` entries). No address in this pass
is repeated.

- **`big_renga_block`, skeleton (`0x80A15870..0x80A15BF0`).** The fifth copy of
  the five-entry descriptor skeleton: a normaliser pair
  ([`fn_80a15870`], [`fn_80a158d0`]), the constant producer
  [`fn_80a15930`] (`0x80357BF4`), the five-link `+0x30` chain walker
  [`fn_80a15a20`], the `:`-suffix name predicate [`fn_80a15b00`] and the two
  48-byte deleting-destructor forwarders [`fn_80a15bc0`]/[`fn_80a15bf0`].
- **`big_renga_block`, actor (`0x80A15C20..0x80A16820`).** The `0x820`-byte
  brick constructor [`fn_80a15c20`], the spawn initialiser [`fn_80a15cb0`]
  (type/count nibbles, start position, motion seeds, scene gate), the
  `g3d/big_renga_block.brres` binder [`fn_80a160d0`] for the nodes
  `big_renga_block` and `block_break1..4`, the break/state probe
  [`fn_80a161b0`], the per-type effect emitter [`fn_80a16410`], the frame
  handler [`fn_80a16760`] and the despawn guard [`fn_80a16820`].
- **`0x80A16D90` family, skeleton (`0x80A16D90..0x80A17120`).** A sixth copy of
  the same skeleton, but with **two** constant producers
  ([`fn_80a16e50`] = `0x80352A74`, [`fn_80a16e60`] = `0x80352A34`), hence two
  normalisers ([`fn_80a16d90`], [`fn_80a16df0`]), the walker
  [`fn_80a16f50`], the predicate [`fn_80a17030`] and the forwarders
  [`fn_80a170f0`]/[`fn_80a17120`].
- **`0x80A16D90` family, actor (`0x80A17150..0x80A17E20`).** One `0x6D0`-byte
  constructor [`fn_80a17150`] (class vtable `0x80B368F8`) shared by four
  variants; each variant is one initialiser plus one destructor that differ only
  in the two `0x24`-byte record lists they attach to `+0x148`/`+0x608` and in
  the four floats copied to `+0x648..+0x654`:

  | variant | initialiser | destructor | records | floats | `+0x6A6` |
  |---:|---|---|---|---|---:|
  | 1 | [`fn_80a17230`] | [`fn_80a17550`] | `0x80B2B208` / `0x80B2B22C` (`+x`) | `1, -1, 16, -16` | 2 |
  | 2 | [`fn_80a176d0`] | [`fn_80a17950`] | `0x80B2B298` / `0x80B2B2BC` (`-x`) | `16, -16, 1, -1` | 2 |
  | 3 | [`fn_80a17ad0`] | [`fn_80a17ca0`] | `0x80B2B330` / `0x80B2B354` (`+y`) | `-16, 16, -1, 1` | 3 |
  | 4 | [`fn_80a17e20`] | *(next pass)* | `0x80B2B3B0` / `0x80B2B3D4` (`-y`) | `-1, 1, -16, 16` | 3 |

### Identity evidence

`g3d/big_renga_block.brres` and `big_renga_block` are literal strings in the
export of [`fn_80a160d0`], and the five node names `big_renga_block`,
`block_break1`, `block_break2`, `block_break3` and `block_break4` are the five
pointers of the table at `0x80B36438`. That is the whole basis for calling the
actor a brick block; nothing else about it is named.

The two per-type tables are literal in the export and were read from the REL
image:

| address | type | contents |
|---|---|---|
| `0x80B2B178` | `u32[16]` | `6, 2, 8, 9, 0x15, 0x11, 0x19, 1, 6, 0x0C, 2, 7, 6, 6, 0x0B, 6` |
| `0x80B2B1B8` | `f32[16]` | `0, 32, 16, 16, 16, 16, 16, 16, 0, 0, 16, 16, 0, 0, 0, 0` |

The four-float blocks copied into `+0x648..+0x654` are likewise literal
(`0x80B2B250`, `0x80B2B2E0`, `0x80B2B378`, `0x80B2B3F8`). Which of the two
values is an offset and which a limit is **not** established by any export, so
the function docs say `// unknown:` rather than guessing.

The class name of the `0x6D0`-byte actor is not printed anywhere in this run,
so it is described by allocation size, vtable and offsets only.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A15870` | `fn_80a15870` | 88 | high |
| `0x80A158D0` | `fn_80a158d0` | 92 | high |
| `0x80A15930` | `fn_80a15930` | 12 | high |
| `0x80A15A20` | `fn_80a15a20` | 224 | medium |
| `0x80A15B00` | `fn_80a15b00` | 136 | medium |
| `0x80A15BC0` | `fn_80a15bc0` | 48 | medium |
| `0x80A15BF0` | `fn_80a15bf0` | 48 | medium |
| `0x80A15C20` | `fn_80a15c20` | 144 | high |
| `0x80A15CB0` | `fn_80a15cb0` | 456 | medium |
| `0x80A160D0` | `fn_80a160d0` | 220 | high |
| `0x80A161B0` | `fn_80a161b0` | 600 | medium |
| `0x80A16410` | `fn_80a16410` | 300 | medium |
| `0x80A16760` | `fn_80a16760` | 152 | high |
| `0x80A16820` | `fn_80a16820` | 176 | high |
| `0x80A16D90` | `fn_80a16d90` | 88 | high |
| `0x80A16DF0` | `fn_80a16df0` | 92 | high |
| `0x80A16E50` | `fn_80a16e50` | 12 | high |
| `0x80A16E60` | `fn_80a16e60` | 12 | high |
| `0x80A16F50` | `fn_80a16f50` | 224 | medium |
| `0x80A17030` | `fn_80a17030` | 136 | medium |
| `0x80A170F0` | `fn_80a170f0` | 48 | medium |
| `0x80A17120` | `fn_80a17120` | 48 | medium |
| `0x80A17150` | `fn_80a17150` | 224 | high |
| `0x80A17230` | `fn_80a17230` | 160 | high |
| `0x80A17550` | `fn_80a17550` | 156 | high |
| `0x80A176D0` | `fn_80a176d0` | 160 | high |
| `0x80A17950` | `fn_80a17950` | 156 | high |
| `0x80A17AD0` | `fn_80a17ad0` | 144 | high |
| `0x80A17CA0` | `fn_80a17ca0` | 156 | high |
| `0x80A17E20` | `fn_80a17e20` | 144 | high |

## Deliberate skips

- **`0x80A17FF0` (156 bytes).** The destructor half of the variant-4 pair. Its
  export is byte-identical to [`fn_80a17ca0`], so it is a trivial continuation
  and was left as the first function of the next pass rather than padding this
  lane past its thirty-body budget. [`fn_80a17e20`]'s doc names the address so
  the next pass can pick it up without re-deriving the pair.
- **`0x80A10170..0x80A10880`** (10 functions, `0x80A10880` alone is 940 bytes)
  is a separate actor block at the head of the bucket whose exports open with
  GQR/paired-single artefacts of the same kind wave 67 had to skip. It was not
  attempted here so the lane stays within one contiguous, fully verified area.
- **The rest of the bucket** (`0x80A18E30..0x80A1A3A0`,
  `0x80A1A9F0..0x80A1BD10`, `0x80A1C080..0x80A1C0B0`, `0x80A1C790..0x80A1DAD0`,
  `0x80A1E100..0x80A1EF30`, `0x80A1F390..0x80A1FFB0`) is untouched. None of
  those addresses is claimed here.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  4,656 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice.
- No address was already in `ledger/functions.csv` (all 61 bucket addresses
  already translated by waves 46 and 54 were excluded before the runs were
  chosen).
- `node tools/check-lane-csv.mjs wave68` reports `OK` with
  `wave68-enemies.csv: 30 rows`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/enemies.rs`.
