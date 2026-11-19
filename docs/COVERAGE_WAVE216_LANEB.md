# Wave 216, lane B - `0x800F39D0..0x800F72F0` (`engine::dsys`)

Ten bodies of the `W%X/pointW%X.csv` / `W%X/routeW%X.csv` loader family and the
two small record-ring helpers above it. All ten are `translated`; the three
`medium` entries depend on evidence the Ghidra export cannot carry (a register
save/restore stub, the register split between `f1..f4` and `r3`, and the
inferred role of the eleven CSV probes).

Block file: `work/wave216/laneB_block.rs`
Ledger: `ledger/agents/wave216-laneB.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x800F39D0` | 1104 | `0x800F39D0..0x800F3E20` (1104) | [`fn_800f39d0`] | medium | point + route CSV parser |
| `0x800F5420` | 172 | `0x800F5420..0x800F54CC` (172) | [`fn_800f5420`] | high | copy the `K`-named archive entries into the records |
| `0x800F5770` | 40 | `0x800F5770..0x800F5798` (40) | [`fn_800f5770`] | high | one of the two words of a `0x1C0`-byte record |
| `0x800F59E0` | 152 | `0x800F59E0..0x800F5A78` (152) | [`fn_800f59e0`] | high | allocate + clear the `5`-byte slot pool |
| `0x800F6810` | 252 | `0x800F6810..0x800F690C` (252) | [`fn_800f6810`] | medium | rebuild the `+0x7C`/`+0x158` draw frames |
| `0x800F6910` | 12 | `0x800F6910..0x800F691C` (12) | [`fn_800f6910`] | high | tail thunk: `1.0f` third argument |
| `0x800F6D60` | 112 | `0x800F6D60..0x800F6DD0` (112) | [`fn_800f6d60`] | high | current world id present in a word array |
| `0x800F7060` | 320 | `0x800F7060..0x800F71A0` (320) | [`fn_800f7060`] | medium | build the 20-byte doubly-linked name ring |
| `0x800F71F0` | 124 | `0x800F71F0..0x800F726C` (124) | [`fn_800f71f0`] | high | index of a name in the ring |
| `0x800F72F0` | 52 | `0x800F72F0..0x800F7324` (52) | [`fn_800f72f0`] | high | re-seat the ring cursor |

All ten sizes were re-checked against the prologue (`stwu r1,-N` or a leaf) and
the closing `bclr` and agree with `work/ghidra/main-gb/export/functions.tsv`
byte for byte. Two of them are special cases that were verified rather than
assumed:

* `0x800F5770` is a leaf with two `bclr`s (one per arm); the extent runs to the
  second one at `0x800F5794`, giving 40 bytes.
* `0x800F6910` is a three-instruction tail thunk (`fmr f4,f2`,
  `lfs f3,1.0f`, `b 0x800F6810`), so it has no `bclr` of its own; the thunk
  body ends at `0x800F6920` after the alignment word, i.e. 12 bytes.

## Byte weight

1,104 + 172 + 40 + 152 + 252 + 12 + 112 + 320 + 124 + 52 = **2,340 bytes**.

## Evidence notes

### `0x800F39D0` - the point/route parser

Ghidra renders the eleven probes as a sixteen-level nested `if`, which is
faithful but hard to read; `work/dol/main.elf` shows eleven separate
`bl 0x800F5F10` + `bc 4,2,0x800F3CC4` guard pairs, and every failing probe
jumps to the same epilogue (the record counter bump). The reconstructed body
keeps that shape with one labelled block, so each guard still reads as
"end-of-record -> skip the rest of the record".

Two details that the export loses:

* the second field reader `0x800F3E20` call is preceded by a `mr r7,r6`, so the
  two `%X` conversions in both name templates take the same world id twice;
* the localised name's tenth byte is bumped by the loader's `+8` word
  (`lbz`/`extsb`/`add`/`stb` at `0x800F3A64..0x800F3A78` and again at
  `0x800F3D4C..0x800F3D60`); the `a`-suffixed template is what makes room for
  that byte.

### `0x800F6810` - register split

Ghidra prints the object as the function's *last* parameter and the three
transfer functions as `undefined8`s. The dump shows `r3 = object` throughout
(`lfs f5,0xB4(r3)` / `addi r3,r3,0x7C` / `addi r3,r3,0x158`) while `f1` is the
added `y` offset, `f2..f4` are the three values forwarded to `0x8016A130`
(`fmr f29,f2` / `fmr f30,f3` / `fmr f31,f4`, used at `0x800F68C0..0x800F68D0`).
The integer and float register files are independent in the EABI, so the
signature is `(object, a, b, c, d)`. The `ldexpf`/`unaff_GQR0` block is the dead
`f29..f31` paired-single save, and the three `stfs` at `r1+0x1C..+0x24` are dead
copies of values that travel in `f1..f3`.

### `0x800F7060` - save/restore stub and return value

`0x802DCD58` (head) and `0x802DCDA4` (tail) are the outlined `r20..r31`
save/restore stubs: they only touch `r11`-relative stack slots and `bclr`, so
`r3` survives the head call, which is why `r21` is `param_1`. The `li r3,1` at
`0x800F7184` is therefore the return value the body leaves in `r3` when the
restore stub unwinds - the Ghidra export types the function `void`, but
`fn_800f7060` returns `1`.

### `0x800F6D60` - the unused first argument

`0x800F6D60` receives the id context in `r3` but overwrites it with the load of
`r13 - 0x54D4` before the callback, so the first parameter is unused. It is kept
in the signature so the call shape matches the original register state.

## Deferrals

None. Every one of the ten bodies was read in full from
`work/dol/main.elf` and reconstructed.
