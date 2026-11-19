# Wave 217, lane B - `0x800F8770..0x800F9078` (`engine::dsys`)

Ten bodies of the player-sheet state run that sits directly above the record
helpers of Wave 216 lane B (`0x800F71C0`/`0x800F72F0`/`0x800F7330`) and below the
per-object step family at `0x800F9080`. All ten are `translated`; the three
`medium` entries depend on evidence the Ghidra export cannot carry (a register
save/restore stub plus float arguments that only survive in `f1`/`f2`).

Block file: `work/wave217/laneB_block.rs`
Ledger: `ledger/agents/wave217-laneB.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x800F8770` | 80 | `0x800F8770..0x800F87C0` (80) | [`fn_800f8770`] | medium | record marker byte equals `0x4B` |
| `0x800F87C0` | 32 | `0x800F87C0..0x800F87E0` (32) | [`fn_800f87c0`] | high | player index against `context + 0x9C` |
| `0x800F87E0` | 60 | `0x800F87E0..0x800F881C` (60) | [`fn_800f87e0`] | high | per-player state query, folded to `0`/`1` |
| `0x800F8980` | 240 | `0x800F8980..0x800F8A6C` (236) + 4 pad | [`fn_800f8980`] | medium | commit the sampled transform row |
| `0x800F8A80` | 68 | `0x800F8A80..0x800F8AC4` (68) | [`fn_800f8a80`] | high | release the manager slot at `+0x6DC` |
| `0x800F8B70` | 16 | `0x800F8B70..0x800F8B80` (16) | [`fn_800f8b70`] | high | tail call into `0x800F6D60` with `0x802F3408` |
| `0x800F8B80` | 168 | `0x800F8B80..0x800F8C28` (164) + 4 pad | [`fn_800f8b80`] | high | session slot still relevant? |
| `0x800F8C80` | 112 | `0x800F8C80..0x800F8CEC` (108) + 4 pad | [`fn_800f8c80`] | high | state gate of the sheet |
| `0x800F8E90` | 220 | `0x800F8E90..0x800F8F6C` (220) | [`fn_800f8e90`] | medium | timeout test (`clock >= scale * timeout`) |
| `0x800F8F70` | 264 | `0x800F8F70..0x800F9078` (264) | [`fn_800f8f70`] | high | completion step, message `0x29` |

Every size was re-checked against the prologue and the closing `bclr` in
`work/dol/main.elf` and agrees with
`work/ghidra/main-gb/export/functions.tsv`:

* `0x800F8770`, `0x800F87C0`, `0x800F87E0`, `0x800F8A80`, `0x800F8E90` and
  `0x800F8F70` end exactly on their `bclr`.
* `0x800F8980`, `0x800F8B80`, `0x800F8C80` end one alignment word short of the
  next entry, which is what the tsv counts.
* `0x800F8B70` is a four-instruction tail thunk (`lis`/`li`/`addi`/`b`) with no
  `bclr` of its own, so its 16 bytes are the whole slot up to `0x800F8B80`.

Three neighbouring entries that share the same address slots are *not* part of
this lane and were left alone: the `mr r3,r4` / `bclr` pair at `0x800F8A70`
(8 bytes), the `b 0x800F8B80` thunk at `0x800F8CF0` (4 bytes) and the
`li r3,0x32` / `bclr` leaf at `0x800F8DA0` (8 bytes).

## Byte weight

80 + 32 + 60 + 240 + 68 + 16 + 168 + 112 + 220 + 264 = **1,260 bytes**.

## Evidence notes

### `0x802DCD74` - the save stub at the head of `0x800F8980` / `0x800F8E90`

Both bodies start with `addi r11,r1,N` + `bl 0x802DCD74`, the outlined
`r27..r31` save. The stub only writes `r11`-relative slots (`r11-0x14 .. r11-4`)
and `bclr`s, so `r3`/`r4` come back unchanged: the halves of its "returned pair"
are exactly the two incoming arguments. Ghidra types both functions `void`, but
the callers show the real shapes - `0x800F7DA0` does
`FUN_800f8980(param_1, cntlzw(...) >> 5)` and `0x800F8230` does
`FUN_800f8980(param_1, 1)`.

### `0x800F8770` - the index that only lives in `r4`

The body reads the sheet table from `r13 - 0x54F4`, scales its `+0x338C` sheet
index by `0x16518`, adds `0x3394` and calls `0x800F56B0`. Ghidra renders that
call with a single argument, but `0x800F56B0` is `param_1 + param_2 * 0x1C0 + 0x18`
and the caller never sets `r4` - so the second argument is the *function's own*
`r4`, which `0x800F8B80` passes as the session id. The reconstructed signature is
therefore `(sheet object, record index)`. `r3` is rebuilt from `lis r3,1` before
any use, so the first argument is unused (kept for the register shape).

### `0x800F8980` - two floats that only live in `f1`/`f2`

`0x800F8050` writes its three-float sample at `r1+0x14..+0x1C`. The body then
subtracts the object's `+0xAC`/`+0xB0`/`+0xB4` position; the `+0xB0` term and the
three-float copy that the compiler materialises at `r1+8..+0x10` are never read
again, so both are dropped. `0x801611C0` is called with `f1 = sample[0] - 0xAC`
and `f2 = sample[2] - 0xB4` (Ghidra's export shows an empty argument list) and
its `sth`-sized result lands at `+0x6B2`. The row published afterwards is
`0x800BBBD0(context, shell byte, index, second shell byte, vtable+0xC4 result)`
followed by `0x800BBBF0(context, shell byte, index, *(object + 0x68C))`.

### `0x800F8E90` - float width of the timeout test

Ghidra widens both operands to `double` (`dVar7`, `dVar8`). The dump disagrees:
`fmr f31,f1` is a single-precision move and the product is `fmuls`, so
`0x802BE470` hands back an `f32` (its own export already ends in
`(double)(float)dVar1`) and the vtable slot `+0x78` result is an `f32` too. The
body's `ldexpf`/`unaff_GQR0` blocks at head and tail are the dead `f31`
paired-single save. The `fcmpo` + `bc 4,0` pair means "expired once the clock is
**not** below `*(r2 - 0x609C) * timeout`". The `0x801C1650` distance result is
computed and discarded.

### `0x800F8A80` / `0x800F8F70` - the `r4` that Ghidra drops twice

`0x800F8A80` reads the slot into `r4` (`lwz r4,0x6DC(r3)`) and only then loads the
manager base into `r3`, so `0x801055D0` really takes `(manager, slot)` even
though the export prints one argument. In `0x800F8F70` the `+0xBC` and `+0x78`
virtual slots are called with the object still in `r3` (`mr r29,r3` first), which
the export likewise renders argument-less.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
block was appended to the copy's `src/engine/dsys.rs` and `cargo check --lib`
exits `0` with the same 23 pre-existing warnings as the unmodified tree (none of
them from `engine::dsys`). A deliberate `compile_error!` probe confirmed the
appended block is part of the compiled unit.

## Deferrals

None. Every one of the ten bodies was read in full from `work/dol/main.elf`.
