# Wave 223, lane E - `0x800A0FB0..0x800A7330` (`game::stage`)

Ten bodies from the `0x800Axxxx` band of `src/game/stage.rs`: the second
variant-record publisher (`+0x414..+0x432`), the three-way draw predicate with
its gated vtable wrapper, the two type-info stubs, one scalar deleting
destructor, the two `0x114`-stride quad-submission loops, the `+0xD14`/`+0xD24`
size report and the path-tail comparison that duplicates `0x8009E910`.

All ten are `translated`; nothing is deferred.

Block file: `work/wave223/laneE_block.rs`
Ledger: `ledger/agents/wave223-laneE.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x800A0FB0` | 252 | `0x800A0FB0..0x800A10A8` (252) | [`fn_800a0fb0`] | high | publish `+0x414..+0x432` from a second actor |
| `0x800A2870` | 336 | `0x800A2870..0x800A29BC` (332) + 4 pad | [`fn_800a2870`] | medium | four-stage "keep this actor active" predicate |
| `0x800A2B20` | 80 | `0x800A2B20..0x800A2B6C` (76) + 4 pad | [`fn_800a2b20`] | medium | gate the predicate on owner `+0x1524` |
| `0x800A2B70` | 12 | `0x800A2B70..0x800A2B7C` (12) | [`fn_800a2b70`] | high | return the shared type-info record |
| `0x800A2BA0` | 12 | `0x800A2BA0..0x800A2BAC` (12) | [`fn_800a2ba0`] | high | same stub, second copy |
| `0x800A2EA0` | 88 | `0x800A2EA0..0x800A2EF4` (84) + 4 pad | [`fn_800a2ea0`] | high | scalar deleting destructor |
| `0x800A3AE0` | 408 | `0x800A3AE0..0x800A3C74` (404) + 4 pad | [`fn_800a3ae0`] | medium | motion block + two quad submissions |
| `0x800A4B20` | 232 | `0x800A4B20..0x800A4C04` (228) + 4 pad | [`fn_800a4b20`] | medium | plain position quad submissions |
| `0x800A4C40` | 152 | `0x800A4C40..0x800A4CD4` (148) + 4 pad | [`fn_800a4c40`] | medium | store `+0xD24`/`+0xE8`/`+0xD14` and report |
| `0x800A7330` | 136 | `0x800A7330..0x800A73B4` (132) + 4 pad | [`fn_800a7330`] | high | compare the tails after the last `:` |

Every `functions.tsv` size was re-checked in `work/dol/main.elf` from the
prologue down to the closing `bclr`. Two entries abut exactly
(`0x800A0FB0`'s `blr` sits at `start + 252 - 4`; both 12-byte type-info stubs end
on their `bclr`), and the other eight are the `blr` extent rounded up to the
next listed entry with four zero bytes behind it.

## Byte weight

252 + 336 + 80 + 12 + 12 + 88 + 408 + 232 + 152 + 136 = **1,708 bytes**.

## Evidence notes

### The `0x802DCD70` / `0x802DCDBC` "64-bit return"

`0x800A2870` opens with `addi r11,r1,0x30` / `bl 0x802DCD70` and closes with
`bl 0x802DCDBC`. Those two addresses are not functions: `0x802DCD70` is the
body of the Metrowerks `_savegpr_26` block (`stw r26,-0x18(r11)` ...
`stw r31,-4(r11)` / `blr`) and `0x802DCDBC` is `_restgpr_26`. Both leave `r3`
and `r4` alone, so the export's `uVar9 = FUN_802dcd70()` is a phantom 64-bit
return and its `iVar2`/`iVar4` split of it are really `param_1` and `param_2`.
The disassembly's `mr r26,r3` / `mr r27,r4` / `mr r28,r5` settles the three-arg
prototype. The `0x802DCD60`/`0x802DCDAC` pair that waves 85 and 105 read the
same way is unrelated allocator code; this one is the plain register spill.

Because `_restgpr_26` is a real `bl`, `r3` survives it: the body's last
instructions on every path are `li r3,0`/`li r3,1`/`lwz r3,0x8(r1)`, so the
function does return a value even though the export declares `void`. That is
why [`fn_800a2870`] and [`fn_800a2b20`] are typed `-> u32`.

### The `0x80064A30` import thunk

`0x800A0FB0`'s compare is a `bl 0x80064A30`, which is `lwz r12,0xffffa680(r13)`
/ `mtspr 288,r12` / `bcctr`, i.e. a jump through the loader slot at
`r13 - 0x5980` = `0x80429D00` (in `.bss`, zero in the image). The stub body
directly behind it, `0x80064A40`, is
`fcmpo cr0,f1,f2` / `mfcr r3` / `rlwinm r3,r3,1,31,31`, so the resolved ABI is
two `f32` in and the `f1 < f2` bit in `r3`. [`fn_800a0fb0`] keeps that ABI.

Both operands of the compare are the object's `+0xAC` and `+0xD0` floats added
together, and the byte lands at `+0x430` - the same slot the already translated
`0x80097F70` fills with its variant byte, which is why the record bytes are
named the same way here.

### `0x800A2870` - the boolean algebra of the three probes

The export's `if (((iVar3 == 0) || ...))` chain is inverted with respect to the
code. The disassembly tests each probe and branches to the arithmetic when it
is **zero**:

```
bl 0x80070CB0(probe)      ; beq -> 0x2910
bl 0x80070CB0(other)      ; beq -> 0x2910
bl 0x80070760(probe)      ; beq -> 0x2910
li r3,0; b end            ; all three non-zero
```

so "all three non-zero" answers `0` outright, and any zero enters the compare
chain. The `countLeadingZeros` pair is the export's rendering of
`mfcr` + `rlwinm r0,r0,2,31,31` (`CR0[GT]`) + `cntlzw` + `>> 5`, i.e. the
negation of `probe->+0xEC > 0.0f`; row `1` is chosen when that negation holds.
`r2 - 0x6D58` is `0.0f` and `r2 - 0x6D54` is `2.0f` (both verified in the
image), so the last test is `2.0f * -speed < table[row*0x24 + *(ctx+4)*4]`
answering `0`.

The table at `0x802F07C0` is `3 7 10 4 2 4 2 16 2` for row `0` and
`10 7 12 4 14 12 ...` for row `1` (nine `f32` per row).

### `0x800A3AE0` / `0x800A4B20` - the two `0x114`-stride loops

Both bodies walk two sub-objects (`+0x9C0` and `+0x798`, step `0x45` words =
`0x114` bytes) and call `vtable+0xB0(sub, &position, &descriptor, &scale)`. The
descriptors differ: `0x800A3AE0` passes `{0x600, 0, *(u16 *)(this + 0x104)}`
and `0x800A4B20` passes `{0, 0, ...}`. `0x800A4B20` follows every submission
with `vtable+0x30(sub, value & 0xFFFF, 3)` where the value comes from
`this->vtable+0x3A8`.

`0x800A3AE0`'s `+0x2FC`/`+0x300` slots both take `this`: the disassembly leaves
`r3` set to the actor across the `bcctr`, while the export prints `()` for the
first one. The `frsp` after each `fneg` is the compiler's rounding of the
single-precision negations, so the `f32` locals in the Rust body are exact.

### `0x800A4C40` - the constants are `1.5f`, `4096.0f` and `1.5 * 2^52`

`r2 - 0x6D40` = `0x8042C340` holds `0x4330000080000000` (`1.5 * 2^52`),
`r2 - 0x6D28` = `0x8042C358` is `4096.0f` and `r2 - 0x6D24` = `0x8042C35C` is
`1.5f`. The `xoris r6,r6,0x8000` pairs are the unsigned-byte form of the
`0x43300000` conversion, so the table byte is really an unsigned scale; the
body is written with the explicit bias subtraction, and the
`fctiwz` + `sth` pair is the truncating `as i32 as u16`.

The final vcall prints as `(**(code **)(... + 0xd4))(param_1,&DAT_80357edc)`
with no arguments in the export; the register file still has `r3 = this` and
`r4 = 0x80357EDC`, so both are restored in the translation.

### `0x800A7330` is byte-for-byte `0x8009E910`

Same `lis`/`addi` pair on `0x8037` as the two 12-byte stubs, and the same
`strrchr(':')` / `vtable+0x20` / `strrchr(':')` / `strcmp(+1, +1)` body as the
already translated [`fn_8009e910`], which is why it carries the same
`high` confidence. Like that body it advances the second `0x802E1C40` result
without a null check (`NULL + 1` = `1`), which is kept as written.
