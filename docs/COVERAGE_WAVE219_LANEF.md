# Wave 219, lane F - `src/resource/archive.rs`, addresses `0x802C0040..0x802C3790`

Ten entries of `work/ghidra/main-gb/export/functions.tsv`, all in the
`0x802C0000` run of the `resource::archive` ledger module. All ten are
translated; none deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x802C0040` | 68 | 68 (`blr` at `0x802C0080`, 12 B padding) | project the record's four corners and close the quad |
| `0x802C0090` | 116 | 116 (`blr` at `0x802C0100`, 12 B padding) | quantise the same four floats and forward them |
| `0x802C0A30` | 68 | 68 (`blr` at `0x802C0A70`, 12 B padding) | construct the `vtable 0x80350110` option record |
| `0x802C0B40` | 28 | 28 (`blr` at `0x802C0B58`, 4 B padding) | guarded word store plus one cleared flag bit |
| `0x802C13F0` | 800 | 800 (`blr` at `0x802C170C`, 4 B padding) | emit one ribbon strip of `n + 2` vertices |
| `0x802C17E0` | 220 | 220 (`blr` at `0x802C18B8`, 4 B padding) | one-state quad setup indexed by leading zeros |
| `0x802C1A30` | 728 | 728 (`blr` at `0x802C1D04`, 4 B padding) | draw one bar with a per-mode blend ladder |
| `0x802C3000` | 32 | 32 (tail branch to `0x801C8E70`, no `blr`) | indexed three-argument GX switch |
| `0x802C3120` | 116 | 116 (`blr` at `0x802C3190`, 4 B padding) | two assignments plus one GX switch |
| `0x802C3790` | 260 | 260 (`blr` at `0x802C3890`, no padding) | deleting destructor of the `0x80350230` object |

No `functions.tsv` size in this lane is a prologue artefact. `0x802C3000` has no
`blr` at all: it loads its three arguments from the `0x802FD078` table and
tail-branches to `0x801C8E70`, so the next entry (`0x802C3020`) starts at
`0x802C3000 + 32`. Every other entry ends in a `blr` exactly at
`start + size - 4`.

Bytes translated: 2436.

## What the run is

The `0x802C0000` band is the immediate-mode GX half of the same overlay that
owns the `0x802Bxxxx` effect helpers already in this file. The ten bodies fall
into four groups.

**Quad projection pair (`0x802C0040`, `0x802C0090`).** Both take one record and
submit one quad. `0x802C0040` loads `+0x00`, `+0x04`, `+0x10`, `+0x14` into
`f1..f4` and the two `r2` constants `0x8042F1E8` (`0.0`) and `0x8042F1F8`
(`1.0`) into `f5`/`f6`, then calls the six-double projection `0x801C9C10` that
`fn_802bba30` already uses, and closes with `0x801C9D20(0, 0)`. `0x802C0090`
runs the same four corners through the `f32 -> u32` quantiser `0x802DCC4C` and
hands the four results to `0x801C9C60` **in reverse order** (`0x00`, `0x04`,
`0x10`, `0x14` are loaded `0x14 -> 0x10 -> 0x04 -> 0x00` and re-ordered into
`r3..r6`).

**Opt-in record and one bit (`0x802C0A30`, `0x802C0B40`).** `0x802C0A30` writes
the vtable word and eleven scalar fields into a `0x18`-byte record
(`+0x04`/`+0x06` zero halfwords, `+0x09 = 6`, `+0x0C = +0x0D = 1`, `+0x10`/
`+0x14` zero; `+0x0E`/`+0x0F` are left alone). `0x802C0B40` stores a non-zero
word into `+0x10` and clears bit 1 of the byte at `+0x08` with `0xFD`.

**Ribbon strip (`0x802C13F0`).** One strip of `n + 2` vertices. The caller
supplies the transform handle in `r3`, the sample count in `r4` and the seed
`y` in `f1`. The body computes `step = -(2*pi / n)` from the `r2` slot
`0x8042F254` (`6.2831855`), scales the per-sample angle by the `0x8042F260`
slot (`40.74...`), seeds two vertices `(0.0, 1.0, 0.0)` and `(0.0, y, 0.0)`,
transforms them with `0x801C1160`, starts the strip with
`0x801C5570(0xA0, 0, (n + 2) & 0xFFFF)`, and then per sample emits the current
sine/cosine vertex at radius `0.5` plus a normal built from the `i - 1` and
`i + 1` neighbours (`0x801C1530` cross product, `0x801C1470` normalise).

**GX switches and one destructor (`0x802C17E0`, `0x802C1A30`, `0x802C3000`,
`0x802C3120`, `0x802C3790`).** These program fixed GX state ladders and publish
a token through `0x802C2360`. `0x802C17E0` selects the table word with
`0x802FCFA0 + ((clz(scale) >> 3) & 0x1FFFFFFC) + index * 8`; the mask makes the
offset `4` exactly when the scale is zero and `0` otherwise, i.e. the second or
the first word of the 8-byte record. `0x802C1A30` inverts the RGB bytes with
`255 - b` when its height argument is `8`, and branches its blend/alpha ladder
on both a caller flag and the byte at `r13 - 0x4731`. `0x802C3000` reads
`(u8, u32, u8)` from the 12-byte `0x802FD078` records, `0x802C3120` sets two
selectors from its argument and the same `r13` byte, and `0x802C3790` installs
`0x80350230` plus its three sub-vtables, releases the three children at
`+0x84`/`+0x80`/`+0x8C` through the `+0x28 -> +0x08` slot, tears down the child
block with `0x802C7E30(this + 8, 0)` and optionally frees the object.

## Notes on fidelity

* **`0x802C13F0` (medium).** The Ghidra export for this body is unusable at the
  edges: it opens with fifteen "removing unreachable block" warnings, invents an
  `unaff_GQR0`, and closes with seven `ldexpf` calls that are really the
  `psq_l` register restores. The raw words were disassembled instead. The block
  at `0x802C1650..0x802C1694` is four `ps_merge00` (primary opcode `4`, XO `20`)
  instructions plus `psq_l`/`psq_st` pairs; decoding them shows that only two
  stores survive, both writing the `r2[-0x3E28]` (`0.0`) word into the `y` of
  the two neighbour vectors before the cross product. The two
  `0x802DCD6C`/`0x802DCDB8` calls in the export are
  `__savegprlr_25`/`__restgprlr_25`, so they are not reproduced as calls.
* **Integer-to-float idiom.** `0x802C13F0` converts `i`, `i - 1` and `i + 1`
  with `xoris`/`lfd 0x43300000...`/`fsubs 0x4330000080000000`. On PowerPC
  `fsubs` rounds the exact double difference to a single, so the sequence is
  the plain signed integer as an `f32`; `to_f32_signed_f` reproduces it,
  including the `-1` wrap at `i = 0`.
* **`0x802C17E0` table index.** Ghidra prints
  `(countLeadingZeros(p) >> 3) & 0x1FFFFFFC`; the emitted `rlwinm` is
  `r3,r3,29,3,29`, whose mask differs on unreachable leading-zero counts
  (`4..7`). The transcription keeps Ghidra's expression, which agrees with the
  instruction on the reachable values (`clz == 32` -> `4`, otherwise `0`).
* **`0x802C1A30` colour block.** The four `subfic rX,rY,255` bytes are written
  at `r1+0x10` and then copied as one word to `r1+0x0C`, which is the pointer
  actually handed to `0x801C8430`. The redundant first store is folded into the
  single packed word.

## Verification

Scratch copy at `%TEMP%\opencode\wave219_laneF` (`src/`, `Cargo.toml`,
`Cargo.lock`), this block appended to
`src/resource/archive.rs`, then `cargo check --lib --offline`:

```
    Checking nsmbw v0.1.0 (...\wave219_laneF)
warning: `nsmbw` (lib) generated 26 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.17s
EXIT=0
```

The 26 warnings are pre-existing (`direct cast of function item into an
integer`, all in `src/ui/menu.rs` and other modules); nothing in this block
produces a diagnostic.
