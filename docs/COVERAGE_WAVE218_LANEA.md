# Wave 218, lane A - `0x800FB290..0x800FBDA0` (`engine::dsys`)

Twelve bodies from the gap between the Wave 217 lane F flag probes
(`0x800FADB0..0x800FB270`) and the caption/flag cluster that Wave 218 lane B
takes from `0x800FBE60` upwards. They are the query side of the stored message
id (`0x800FB350`/`0x800FB380` plus three `is it one of two` predicates), the
`0x800FA9C0` mode wrapper `0x800FB290`, and the four bodies that maintain the
14-byte `"mt..."` code table (`0x800FB4B0`, `0x800FB670`, `0x800FB6C0`,
`0x800FB7F0`, `0x800FB900`) plus the ten-slot reset `0x800FBDA0`.

All twelve are `translated`; nothing is deferred.

Block file: `work/wave218/laneA_block.rs`
Ledger: `ledger/agents/wave218-laneA.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x800FB290` | 60 | `0x800FB290..0x800FB2CC` (60) | [`fn_800fb290`] | medium | "the game mode is 1 or 6" |
| `0x800FB350` | 48 | `0x800FB350..0x800FB380` (48) | [`fn_800fb350`] | high | read the stored message id of the selected row |
| `0x800FB380` | 64 | `0x800FB380..0x800FB3C0` (64) | [`fn_800fb380`] | high | write that message id |
| `0x800FB3C0` | 80 | `0x800FB3C0..0x800FB40C` (76) + 4 pad | [`fn_800fb3c0`] | high | id is `1` or `4` |
| `0x800FB410` | 80 | `0x800FB410..0x800FB45C` (76) + 4 pad | [`fn_800fb410`] | high | id is `3` or `6` |
| `0x800FB460` | 80 | `0x800FB460..0x800FB4AC` (76) + 4 pad | [`fn_800fb460`] | high | id is `2` or `5` |
| `0x800FB4B0` | 440 | `0x800FB4B0..0x800FB668` (440) | [`fn_800fb4b0`] | medium | publish the first free caption slot of a code |
| `0x800FB670` | 68 | `0x800FB670..0x800FB6B4` (68) | [`fn_800fb670`] | high | unpack a table entry's nine-byte code |
| `0x800FB6C0` | 296 | `0x800FB6C0..0x800FB7E8` (296) | [`fn_800fb6c0`] | high | which group holds a code (`0`/`1`/`2`) |
| `0x800FB7F0` | 260 | `0x800FB7F0..0x800FB8F4` (260) | [`fn_800fb7f0`] | high | append the successor of a code |
| `0x800FB900` | 140 | `0x800FB900..0x800FB98C` (140) | [`fn_800fb900`] | high | insert one 14-byte name into the code table |
| `0x800FBDA0` | 60 | `0x800FBDA0..0x800FBDDC` (60) | [`fn_800fbda0`] | high | reset the ten caption slots |

Every `functions.tsv` size was re-checked in `work/dol/main.elf` from the
prologue down to the closing `bclr`. In this lane **all twelve tsv values are
exact**: each `blr` sits at `start + tsv - 4`, and where the next entry does not
abut, the bytes after the `blr` are alignment zero words that the tsv does not
count:

* `0x800FB350`, `0x800FB380` and `0x800FB4B0` end exactly on their `blr` and the
  next entry begins on the following word.
* `0x800FB290`, `0x800FB3C0`, `0x800FB410`, `0x800FB460`, `0x800FB670`,
  `0x800FB7F0`, `0x800FB900` and `0x800FBDA0` are followed by one zero word
  (`+4`).
* `0x800FB6C0` is followed by two zero words (`+8`).

Not part of this lane: `0x800FB270` and below belong to Wave 217 lane F, and
`0x800FBE60` and above to Wave 218 lane B.

## Byte weight

60 + 48 + 64 + 80 + 80 + 80 + 440 + 68 + 296 + 260 + 140 + 60 = **1,676 bytes**.

## Evidence notes

### The `0x802DCD6C`/`70`/`74` "returns"

`0x800FB4B0`, `0x800FB6C0` and `0x800FB7F0` open with a `bl` to one of the
outlined register-save helpers. Ghidra renders it as a 64-bit return, but the
stub only stores `r25..r31` into the caller's frame through `r11` and leaves
`r3`/`r4` alone; the two halves of Ghidra's `undefined8` are therefore the two
incoming arguments, with `r3` in the **high** word. That is what settles the two
argument roles in each body, e.g. `0x800FB4B0`'s manager (`r3`, used with
`+0x7D4`/`+0x7DC`) versus its record (`r4`, whose first nine bytes are copied at
`0x800FB4E8`). The matching `0x802DCDBC`/`C0` tails are epilogues that restore
those registers and hand `r3` back, so no call is modelled for them.

### `0x800FB290` - the forwarded mode probe

`0x800FB290` never touches `r3..r5` before `bl 0x800FA9C0` at `0x800FB29C`,
which is exactly the code an in-line forward of three arguments compiles to; the
callee itself masks and uses all three. Ghidra prints `FUN_800fb290(void)`
because it sees no register writes, so this lane models the three arguments and
calls `0x800FA9C0` **by address** (the translated [`fn_800fa9c0`] returns `()`,
while the original consumes the `0..9` code the callee publishes in `r3`).
The address has no call site in the `main-gb` export, so the reading is
`medium`, not `high`.

### `0x800FB350` - a `void` that really returns

The export declares `void FUN_800fb350(void)`, but the body ends with
`bl 0x800CE250` / `blr` and no register clobber, and all three callers do
`rlwinm r0,r3,0,24,31` on the result before comparing it with `1`/`2`/`3`/`4`/
`5`/`6`. The id byte is therefore the return value; the three predicates apply
the `& 0xFF` themselves.

### `0x800FB3C0`/`410`/`460` - the short circuit

Each of the three calls `0x800FB350` and, **only when the first answer is not
the first id**, calls it a second time for the second id (`beq` over the second
`bl`). The Rust `||` reproduces that: two reads at most, and the same read order.

### `0x800FB4B0` - the one uninitialised key byte

The 14-byte key is built at `r1+0x24` as `'m' 't'` + nine bytes of `param_2` +
`'a'` (`r1+0x2F`) + the digit (`r1+0x30`) + a terminator (`r1+0x31`). The guard
probe at `0x800FB504` runs **before** the loop, so `r1+0x31` is never stored for
it, and the table compare is a fixed `0xE`-byte `memcmp` (`0x800FD750` compares
`0xe` bytes, not a NUL-terminated string). The guard key therefore ends in one
uninitialised stack byte; this lane writes `0`, giving `"mt"+code+"a\0\0"`, and
marks the body `medium` for that single byte. Everything else - the five even
hint digits `'0' '2' '4' '6' '8'`, the `+2` digit step with a `+1` index step,
the six probes, the two 14-byte copies, the `0x7D4` object re-read for each of
its three calls, and the `extsb r4,r0` that turns `local_31` into the second
`0x800FB900` argument - is literal.

### `0x800FB6C0` - which buffer each `0x800F92C0` fills

There are two copies of the five-byte half, and they are easy to swap:

* `0x800FB6E4`/`0x800FB6E8` - `0x800F92C0(param_1, r1+0x10)`, i.e. the
  **reference** half `param_1 + 5..+9`, made once before the walk;
* `0x800FB758`/`0x800FB760` - `0x800F92C0(name, r1+0x8)`, i.e. the candidate's
  own `+5..+9`, made inside the loop.

The accept test is therefore `memcmp(param_1, name, 10) != 0 &&
memcmp(reference, candidate_tail, 5) == 0` - the candidate must be a *different*
name that carries the *same* five-byte half, which is the "same code, other
slot" relation the caller relies on. The returned group comes from
`0x800FA7E0(name)`, so the answer is `2` whenever the walk found nothing.

### The sheet table

The three code-table bodies rebuild their base on **every** pass, because the
original does: `lwz rN,0xffffab0c(r13)` (`r13 - 0x54F4` = `0x8042A18C`),
`lwz r0,0x338C(rN)`, `mulli r0,r0,0xBF8`, `add r3,rN,r0`, `addi r3,r3,0x1A0`.
That is a `0xBF8`-stride sheet array at `+0x1A0`, *not* the `0x16518`-stride
course rows the Wave 214/215 bodies build on the same object, so the lane adds
`SHEET_STRIDE_A`/`SHEET_TABLE_A` and reuses the existing `CTX_FIELD_338C_C` for
the index word. The entry count and entry come from the already translated
[`fn_800f11d0`]/[`fn_800f11e0`], and the table probe from [`fn_800f11c0`]
(`0x800FD750`, 14-byte names, count at `+4`, names at `+8`).

### `0x800FB7F0` - a `void` return

The export prints `void`, but `r3` on exit is a leftover: the entry count on the
loop-exit path, the copy destination on a hit. Its only caller (`0x800FB4B0`)
ignores it, so the body is modelled as returning `()` and the leftover is
recorded rather than invented.

### `0x800FB900` - the template overwrite

`0x802E19F8(r1+0xA, param_1, 10)` copies ten bytes, then
`stb r29,0x13(r1)` overwrites the tenth with `param_2`, so the rendered
`"mt" + ten bytes + param_2 + digit + NUL` in Ghidra is really
`"mt" + nine bytes + param_2 + digit + NUL`. `param_3 + '0'` is added with
`addi r3,r30,0x30` before the byte store. The call is
`0x800F0170(sheet_base, name)` and its `bool` (the translated
[`fn_800f0170`]) is what `0x800FB4B0` tests.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to
`%TEMP%\opencode\wave218_laneA`, the block was appended to the copy's
`src/engine/dsys.rs` and `cargo check --lib` finished with exit code `0`; no
warning comes from `engine::dsys` (the remaining warnings are the pre-existing
`ui::menu` / `runtime::bt` cast and parenthesis warnings of the unmodified
tree).

## Deferrals

None. All twelve bodies were read in full from `work/dol/main.elf`; the only
modelled-not-observed detail is the single uninitialised guard-key byte in
`0x800FB4B0`, documented above.
