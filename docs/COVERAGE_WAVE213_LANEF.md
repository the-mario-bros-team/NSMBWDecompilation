# Coverage - Wave 213, lane F (`game::player`, the `0x8006C5C0..0x8006C894` `Base_c` object family)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 612 | 9 | 1 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred.

Body block: `work/wave213/laneF_block.rs`. Ledger rows:
`ledger/agents/wave213-laneF.csv`.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name of the body, which the Rust function keeps as
  `fn_<address>`; every new constant in the block is prefixed `W213F_`.
* **Size** - the `functions.tsv` value. Eight of the ten were re-derived from
  the prologue and the closing `blr` and match exactly. `0x8006C5C0` (60 B
  real vs 64 in the tsv) and `0x8006C610` (52 B real vs 56 in the tsv) end
  4 bytes before the next aligned slot; both are explicit 20-byte prologue
  functions ending in a real epilogue, not 16/20-byte prologue artefacts.
  The three 4-byte thunks are single `b` words, matching the tsv.
* **Confidence** - strength of the evidence: `high` here means every address,
  field offset, immediate and branch is literal in the disassembly.
* `r13 = 0x8042F680` and `r2 = 0x80433080` throughout, so the SDA accesses
  read as `r13 - 0x58F0` = `0x80429D90`, `r13 - 0x58E8` = `0x80429D98`,
  `r13 - 0x58E4` = `0x80429D9C`, `r13 - 0x58E0` = `0x80429DA0`,
  `r13 - 0x52D8` = `0x8042A3A8`, `r13 - 0x52D4` = `0x8042A3AC`,
  `r13 - 0x52C8` = `0x8042A3B8`, `r2 - 0x7450` = `0x8042BC30` (`0.0f`) and
  `r2 - 0x744C` = `0x8042BC34` (`1.0f`).

## What this pass covers

All ten bodies belong to one class family. The two vtables are adjacent in
`main.dol` `.data` and were dumped to identify the slots:

| vtable | installed by | `+0xC` | `+0x24` | `+0x28` | `+0x30` | `+0x48` | `+0x4C` |
|---|---|---|---|---|---|---|---|
| `0x8030F1D8` (base) | `0x8006C420` | `0x8006C540` | `0x8006C5C0` | `0x8006C600` | `0x8006C610` | `0x8006C490` | `0x8006C660` |
| `0x8030F228` (derived) | `0x8006C6D0` | `0x8006C870` | `0x8006C8F0` | `0x8006C980` | `0x8006C9F0` | `0x8006C7F0` | `0x8006C660` |

The family name is readable in the data the descriptor getter returns:
`0x8030F1B8 + 0xA` is the ASCII run `Base_c`. Slots `+0x8`/`+0x14`/`+0x20`/
`+0x2C`/`+0x38..+0x44` of both vtables are shared `0x80161D80..0x801628B0`
display-object implementations that live outside this lane; the derived table
keeps `+0x3C..+0x44` and replaces `+0xC`, `+0x24`, `+0x28`, `+0x30` and
`+0x48`.

### The constructor `0x8006C6D0` (280 B, high)

`0x8006C420(this)` (base constructor: base vtable, `+0x64 = 0`,
`+0x68 = vtable[0x4C](this)`, `+0x6C = 0x801017B0(id)`), then the derived
vtable `0x8030F228` at `+0x60`; the intrusive list node `{0, 0, this}` at
`+0x70..+0x78` is pushed onto the SDA list `0x80429DA0` with `0x80160FE0`; two
`float[3]` are copied from `*(0x80429D98)` into `+0xAC..+0xB4` and
`+0xB8..+0xC0`; two `int16[3]` from `*(0x80429D9C)` into `+0x100..+0x104` and
`+0x106..+0x10A`; `+0x124 = 1`; `+0xD0..+0xD8 = 0.0f`;
`+0xDC..+0xE4 = 1.0f`; and `+0x120 = moduledescriptor[*(u16 *)(this+8)] + 8`.

The SDA pointer is reloaded between the two copies of each pair, and the
`lha`/`sth` copies are reproduced as raw halfword copies (the sign extension
is truncated again by the store).

### The destructor `0x8006C7F0` (116 B, high)

`(this, mode)`: null-safe; restores the derived vtable, unlinks `+0x70`,
runs the base teardown `0x8006C490(this, 0)` and, for a signed `mode > 0`,
frees the block through `0x80162920`; always returns `this` (dangling after a
free, as in the image).

### The three 4-byte `b` thunks `0x8006C600` / `0x8006C6B0` / `0x8006C6C0` (4 B each, high)

Decoded from the raw words rather than from the decompiler output:

| address | raw word | branch | target |
|---|---|---|---|
| `0x8006C600` | `0x480F5B10` | `b 0x80162110` | empty leaf (`blr` only) |
| `0x8006C6B0` | `0x480F6450` | `b 0x80162B00` | `(id, owner) -> u32` resource create |
| `0x8006C6C0` | `0x480F6460` | `b 0x80162B20` | `(this, a, b)` -> `0x80162A70(this, 0, a, b)` |

The tail branches forward the caller's registers unchanged, so the Rust
signatures are the targets' own arities; `0x8006C600` forwards nothing
usable because its target ignores every register.

### The virtual leaves

* `0x8006C5C0` (60 B, high, base slot `+0x24`) - indirect call through
  `vtable[0x4C]` with `this` as the only argument (for both known vtables that
  is the descriptor getter) and then `0x801620E0(this)`. Both results are
  discarded; the declared `void` is faithful. `// unknown:` why the slot is
  invoked for its side effect only.
* `0x8006C610` (52 B, high, base slot `+0x30`) - publishes `0x800B2EC0()`
  into SDA `0x80429D90`, then calls the `0x801621E0` predicate.
* `0x8006C660` (12 B, high, slot `+0x4C`) - returns the static descriptor
  `0x8030F1B8`; `lis`/`addi` immediates are the whole body.
* `0x8006C690` (28 B, medium) - publishes the `0x8006C670` (`li r3,2; blr`)
  and `0x8006C680` (`blr`) stubs into SDA `0x8042A3A8` / `0x8042A3AC`. The
  stores are literal; the role (a static class-registration hook) is inferred
  from the two stub bodies, hence `medium`.
* `0x8006C870` (44 B, high, derived slot `+0xC`) - `((v | -v) >> 31)` of
  `0x8006C540()`, i.e. "the base predicate is true". The `neg` at
  `0x8006C880` (`0x7C0300D0`, XO 104) is printed as an unnamed `X31_104`
  mnemonic by the raw disassembler.

## Deferrals

None. All ten bodies were translated; no body needed to be guessed.
