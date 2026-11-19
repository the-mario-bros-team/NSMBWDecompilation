# Wave 216, lane A - `0x800F2EB0..0x800F71C0` (`engine::dsys`)

Ten bodies of the stage/route CSV loader cluster. All ten are `translated`;
nine are `high` and one (`0x800F6940`) is `medium` because the encoded divisor
expression is suspicious even though it was reproduced instruction for
instruction.

Block file: `work/wave216/laneA_block.rs`
Ledger: `ledger/agents/wave216-laneA.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x800F2EB0` | 112 | `0x800F2EB0..0x800F2F20` (112) | [`fn_800f2eb0`] | high | find the `0x54`-byte route entry whose name matches |
| `0x800F33D0` | 112 | `0x800F33D0..0x800F3440` (112) | [`fn_800f33d0`] | high | publish the three stage limits and register the cleanup |
| `0x800F3E80` | 2544 | `0x800F3E80..0x800F4870` (2544) | [`fn_800f3e80`] | high | parse one CSV field and raise the scene flag bits |
| `0x800F5A80` | 8 | `0x800F5A80..0x800F5A88` (8) | [`fn_800f5a80`] | high | release the object at `+0xC` (tail thunk) |
| `0x800F5C10` | 300 | `0x800F5C10..0x800F5D3C` (296) | [`fn_800f5c10`] | high | recursive two-part course-name search |
| `0x800F5D40` | 280 | `0x800F5D40..0x800F5E58` (280) | [`fn_800f5d40`] | high | compact a node chain into the five-byte slot table |
| `0x800F5F10` | 44 | `0x800F5F10..0x800F5F3C` (44) | [`fn_800f5f10`] | high | CR/LF probe |
| `0x800F6940` | 164 | `0x800F6940..0x800F69E4` (164) | [`fn_800f6940`] | medium | set up one `+0x174..+0x180` interpolation |
| `0x800F6AB0` | 156 | `0x800F6AB0..0x800F6B4C` (156) | [`fn_800f6ab0`] | high | step that interpolation (tail call into `0x800F6390`) |
| `0x800F71C0` | 40 | `0x800F71C0..0x800F71E8` (40) | [`fn_800f71c0`] | high | `0x800F7270(obj, 0) -> +8` |

All ten sizes were re-checked against the prologue (`stwu r1,-N` or a leaf) and
the closing `bclr` and agree with `work/ghidra/main-gb/export/functions.tsv`.
Three of them are special cases that were verified rather than assumed:

* `0x800F5A80` is a two-instruction tail thunk (`lwz r3,0xC(r3)`,
  `b 0x802B9290`) with no `bclr` of its own; the next function starts at
  `0x800F5A88`, so the tsv's 8 bytes are exact.
* `0x800F5C10` is the only body whose tsv size overstates the code: the `bclr`
  is at `0x800F5D38` (real extent 296) and the four bytes at `0x800F5D3C` are
  alignment padding before `0x800F5D40`.
* `0x800F6AB0` ends in a tail branch (`b 0x800F6390` at `0x800F6B40`) that is
  *not* the epilogue; the `bclr` at `0x800F6B48` belongs to the countdown arm, so
  the extent runs to `0x800F6B4C`.

## Byte weight

112 + 112 + 2544 + 8 + 300 + 280 + 44 + 164 + 156 + 40 = **3,760 bytes**.

## Evidence notes

### `0x800F3E80` - the field parser (2544 bytes)

This is the largest body attempted so far, so it was decoded from the raw words
(`work/dol/main.elf`, `0x800F3E80..0x800F486C`) and cross-checked against
`work/ghidra/main-gb/export/800f3e80_FUN_800f3e80.c`.

The structure the export loses is that there are **two independent flag
chains**, not one nested `if`: every `+0x1C8` arm (and the failed tail of that
chain) branches to `0x800F4604`, the first compare of the `+0x1D0` chain. One
field therefore can raise one bit in each word. The `"anchor"` case in between
is the only order-sensitive part: a six-byte `memcmp` match consumes the rest of
the `+0x1C8` chain.

The keyword literals are not in the export - it prints `unaff_r13 + -0x6d64`
style expressions. Reading the image at those addresses gives the actual text,
which is what the tables `KEY_TABLE_1C8A_A`, `KEY_TABLE_1C8B_A` and
`KEY_TABLE_1D0_A` carry:

| SDA offset | address | text | mask (chain / field) |
|---|---|---|---|
| `-0x6D64` | `0x8042891C` | `ura` | `0x1` / `+0x1C8` |
| `-0x6D60` | `0x80428920` | `stop` | `0x2` |
| `-0x6D58` | `0x80428928` | `link1` | `0x4` |
| `-0x6D50` | `0x80428930` | `link2` | `0x8` |
| `-0x6D48` | `0x80428938` | `link3` | `0x10` |
| `-0x6D40` | `0x80428940` | `link4` | `0x20` |
| `-0x6D38` | `0x80428948` | `link5` | `0x40` |
| `-0x6D30` | `0x80428950` | `scroll` | `0x80` |
| `-0x6D28` | `0x80428958` | `scale` | `0x100` |
| `-0x6D20` | `0x80428960` | `dokan` | `0x400` |
| `-0x6D18` | `0x80428968` | `switch` | `0x800` |
| - | `0x8031BD68` | `crossroad` | `0x40000` |
| `-0x6D10` | `0x80428970` | `focus` | `0x80000` |
| `-0x6D08` | `0x80428978` | `anchor` | `0x1000` / `0x2000` / `+0x1CC` |
| `-0x6D00` | `0x80428980` | `tilt` | `0x4000` |
| `-0x6CF8` | `0x80428988` | `demo1` | `0x8000` |
| `-0x6CF0` | `0x80428990` | `demo2` | `0x10000` |
| `-0x6CE8` | `0x80428998` | `demo3` | `0x20000` |
| `-0x6CE0` | `0x804289A0` | `demo4` | `0x2000000` |
| `-0x6CD8` | `0x804289A8` | `demo5` | `0x4000000` |
| `-0x6CD0` | `0x804289B0` | `demo6` | `0x8000000` |
| `-0x6CC8` | `0x804289B8` | `demo7` | `0x10000000` |
| `-0x6CC0` | `0x804289C0` | `camstop` | `0x20000000` |
| `-0x6CB8` | `0x804289C8` | `noshift` | `0x40000000` |
| `-0x6CB0` | `0x804289D0` | `board` | `0x100000` (5-byte `memcmp`) |
| - | `0x8031BD74` | `demostop` | `0x200000` |
| `-0x6CA8` | `0x804289D8` | `scrollY` | `0x400000` |
| `-0x6CA0` | `0x804289E0` | `sand` | `0x800000` |
| `-0x6C98` | `0x804289E8` | `ice` | `0x1000000` |
| `-0x6C90` | `0x804289F0` | `scrollA` | `0x80000000` |
| `-0x6C88` | `0x804289F8` | `kuribo1` | `0x2` / `+0x1D0` |
| `-0x6C80` | `0x80428A00` | `kuribo2` | `0x4` |
| `-0x6C78` | `0x80428A08` | `Puku1` | `0x8` |
| `-0x6C70` | `0x80428A10` | `Puku2` | `0x10` |
| `-0x6C68` | `0x80428A18` | `Pak1` | `0x20` |
| `-0x6C60` | `0x80428A20` | `Pak2` | `0x40` |
| `-0x6C58` | `0x80428A28` | `Pak3` | `0x80` |
| `-0x6C50` | `0x80428A30` | `Hbros1` | `0x100` |
| `-0x6C48` | `0x80428A38` | `trap1` | `0x400` |
| `-0x6C40` | `0x80428A40` | `trap2` | `0x800` |

Two details worth recording:

* `board` is the one key compared with `0x802E1BD0(field, "board", 5)`, i.e. a
  five-byte prefix compare, not `strcmp`; the table keeps the bound and
  `key_eq_a` selects the callee accordingly.
* the `"anchor"` suffix case is length-based: `6` sets both `0x1000` and
  `0x2000`; `7`/`8` inspect `field[6]` (`x` -> `0x1000`, `y` -> `0x2000`,
  `'1'..'9'` -> `atoi(&field[6])` into the record byte `+0x1CC`) and then
  `field[7]` (a digit -> `atoi(&field[7])` into the same byte).

The local field buffer is the `0x48`-byte view at `r1 + 8`; in the original it
overlaps the `r22..r31` save slots, and the copy loop has no bound. The
reconstruction uses a 0x50-byte array so the same writes stay inside a Rust
object, and documents the missing check in the body's `# Safety` section.

### `0x800F5C10` - recursion depth protocol

The Ghidra export shows a `pair = FUN_802dcd68()` whose high word becomes the
table base and whose low word is used as the four-byte key. `0x802DCD68` is the
`_savegpr_r24` stub, which never touches `r3`/`r4`, so the pair is simply
`(param_1, param_2)`. Likewise the `FUN_802dcdb4(uVar5)` tail is
`_restgpr_r24` and the `uVar5` argument is just the value already in `r3`.

The depth counter at `r13 - 0x5540` (`0x8042A140`) is incremented on entry and
decremented on the shared tail, so both the "found" `break` and the exhausted
loop decrement it; only the `depth + 1 >= limit` arm restores the pre-bump
value. The recursive call passes `reset = 0` and drops the first-name buffer.

### `0x800F6940` - the encoded divisor is `fsubs`

`0x800F6940` builds the per-frame step as

```text
+0x178 = ((float)target - (float)current)
         / ((float)(2^52 + (count ^ 0x80000000)) - (float)4503601774854144.0)
```

with the subtraction at `0x800F697C` encoded as **`fsubs`** (`ec002028`), not
`fsub`. Both operands round to the same single-precision value
(`4503601774854144.0f`), so the divisor is `0.0f` and the quotient is `±inf`
(`NaN` if the numerator is zero too). The export hides this: Ghidra prints a
double subtraction followed by casts.

The body reproduces the encoded arithmetic exactly (rounding both doubles to
`f32` before subtracting) and is marked `medium` for that reason only. The field
offsets, the six stores, the `0x800F6320` call and the `lha`/`sth` pair at
`+0x102` are all `high` confidence.

### `0x800F33D0` - dead stack copies

The three `stfs` at `0x800F340C..0x800F3414` write the three limit floats into
the function's own frame and are never read; only the `+0x18..+0x20` stores into
`0x8031BCC0` survive. The observed values are `2160.0f`, `-30.0f` and
`-478.0f`; the body copies them at run time instead of hard-coding them.

### `0x800F5D40` - two different counters

`+0x15F28` is used twice: the per-column slot at
`ctx + 0x10000 + index * 0x18 + 0x5F28` holds the threshold that decides whether
the chain is written at all, while the *global* cursor `ctx + 0x15F28` (no index
term, `r31 = ctx + 0x10000` only) is the running destination index. The
reconstruction keeps both.

## Deferrals

None. Every one of the ten bodies was read in full from `work/dol/main.elf` and
reconstructed; no byte range was left untranslated.

## Verification

* Scratch copy: `src/`, `Cargo.toml` and `Cargo.lock` copied to
  `%TEMP%\opencode\scratch216a`, `work/wave216/laneA_block.rs` appended to
  `src/engine/dsys.rs`, `cargo check --lib` exits 0 with no warning from
  `src/engine/dsys.rs`.
* `node tools/check-lane-csv.mjs wave216` run in a scratch tree that carries the
  block: `10 rows, 10 unique addresses, translated/high=9 translated/medium=1,
  OK`.
* Files written: `work/wave216/laneA_block.rs`,
  `ledger/agents/wave216-laneA.csv`, this document. `src/engine/dsys.rs` was not
  edited.
