# Coverage - Wave 206 lane B (`ui::hud`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 1096 | 8 | 2 |

Addresses `0x8003_1180`-`0x8003_1CC0`, the ten bodies that open the
`0x80031xxx` block; wave 206 lane C continues the same block at `0x8003_1DF0`
and there is no overlap with lanes A/C-F.

## What landed

- **Interface forwards.** `0x80031180` and `0x800311B0` are the `0x802DCBFC`
  thunks with the descriptor at `a + 0x18` and `a + 0x24`, byte-identical to
  `0x8002BDE0`/`0x8002BE10` (wave 205 lane B) and
  `0x80033180`/`0x800331B0` (wave 206 lane C). `r12` carries the selector, `r5`
  carries `a`, `r3`/`r4` both carry `b`.
- **Type-word accessors.** `0x800311E0`, `0x800311F0` and `0x80031200` are three
  more `lis`/`addi`/`blr` copies returning `0x80377A04`, four bytes apart, with
  four bytes of alignment padding after each.
- **Quicksand probe.** `0x800319A0` gates on
  `0x80070CB0(this + 0x1EC)` (bit field `+0x88 & 0x1FE000`), then repeats the
  [`fn_800275f0`] cell test: `0x80070BA0` bit `0x8000` set and `0x80070BF0`
  cell type `3` in bits `16..23`.
- **Quicksand spawn.** `0x80031A30` resolves the position with `0x8006CED0` and
  calls virtual slot `+0xA0` of the `+0x720` member with
  `("Wm_en_quicksand", 0, {x, y, 5500.0}, 0, 0)`.
- **Ground height.** `0x80031B30` writes `+0xB4` as either
  `*(+0x5EC) + 0x8042BB20[+0x391]` (table `1500.0`, `-2500.0`, `1.0`) or the
  flat `-2500.0` when `+0x38F` is set.
- **Hit record.** `0x80031B70` spawns `"Wm_en_hit"` at `{x, y, 5500.0}`,
  plays sound event `0x17B` through `*(r13 - 0x5924)` at the two-float
  projection of `+0xAC`, then fills `+0x414..+0x432`: `±1.5` side colour,
  `3.9`/`-4.0`/`-0.24375`, the three words of `0x802EE640`, the side byte and
  `0xFF`.
- **Ground cells.** `0x80031CC0` splits `*(+0x164) + *(+0x16C) + 3.0` either
  side of the centre, looks both up, and stores `1`/`2` (hit, by `+0x391`) or
  `3` (no hit) into `+0x1E7`.

## Corrections and recoveries

- **`0x80031CC0`'s size in the lane list is a prologue artefact.** The wave-206
  size list carries `20` bytes for this address, but `functions.tsv` says `304`
  and the body is `0x8003_1CC0..0x8003_1DF0` (terminal `bclr` at
  `0x8003_1DEC`), i.e. exactly `304` bytes. The `# Original` line uses the
  `functions.tsv` value.
- **`0x80031B70` and `0x80031CC0` export GQR noise.** Both decompiles print
  `unaff_GQR0`, `ldexpf` and "removing unreachable block" fragments. Those are
  the `psq_st`/`psq_l` saves and restores of `f31`/`f30`/`f29` with `GQR0 = 0`
  and the duplicated epilogue; they carry no value and are not reproduced.
- **`0x80031B70`'s side flag is a `fcmpo`/`cror`/`cntlzw` triple.** The
  disassembly is `fcmpo cr0, f1, f0` on `x - *(+0xB8)` against `0.0`,
  `cror 2,1,2` (so CR0.EQ becomes `x >= 0`), `mfcr`, `rlwinm r0,r0,3,31,31`
  (extracts CR0.EQ) and `cntlzw`/`rlwinm` to land on `1` when the comparison
  fails. Ghidra folds the same expression into
  `countLeadingZeros((uint)(T <= x))`; the Rust body keeps the failure case
  explicit so unordered also yields `1`.
- **The `0x802EE630` record is data, not a string table.** It is
  `{0, 0, 0, 0, 0x80357BF4, 0xFFFFFFFF, 0xFFFFFFFF, 0x00FF0000}` in `.data`;
  `+0x10..+0x18` and the `+0x1D` byte are copied into the hit record, while the
  first four words are copied to the frame and then dropped as dead stores.
- **`0x80031B70`'s third and fourth operands to `0x8006A4B0` are a two-float
  record, not three.** The export labels an eight-byte stack area; only
  `+0x00`/`+0x04` are written before the `0x80197F00` call.
- **`0x80031A30` reads its `z` from SDA2.** The `5500.0` third component is a
  plain `lfs r0,0xffff845c(r2)` (`SDA2_BASE - 0x7BA4`) that the export prints as
  `*(unaff_r2 + -0x7ba4)` because the word is not a labelled constant; the same
  word feeds `0x80031B70`'s hit vector.
- **`0x80031B30`'s table index is unchecked.** `rlwinm r0,r0,2,0,29` scales
  `+0x391` by four without a bound test, so a value above `2` reads past the
  three-word `0x8042BB20` table; the reconstruction keeps that behaviour.
