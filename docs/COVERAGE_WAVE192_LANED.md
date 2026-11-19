# Coverage - wave 192, lane D (`d_basesNP`, band `0x806627F0..0x80678300`)

## How to read this file

Lane D of wave 192 translated all ten assigned functions of the base gameplay
REL into `game::bases`. The bodies are in `work/wave192/laneD_block.rs` and are
appended to `src/game/bases.rs` by the coordinator; the machine-readable ledger
is `ledger/agents/wave192-laned.csv`.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented.
- **Confidence** - `high` = flow, constants and call shapes cross-checked
  against the raw words and against an already-translated sibling; `medium` =
  flow is literal but a role or argument is inferred.

**Synthetic base warning.** Every address here is an analysis address with base
`0x80600000`; these are **not** runtime addresses. `0x8008xxxx`, `0x8016xxxx`,
`0x8019xxxx`, `0x8042xxxx`, `0x807Cxxxx` and `0x8082xxxx` values are imported
main.dol / REL globals and are used verbatim.

## Inventory

| Metric | Lane D |
|---|---:|
| Assigned functions | 10 |
| Translated | 10 |
| Deferred | 0 |
| Bytes selected | 2,348 |
| high / medium | 3 / 7 |

## Clusters

### `+0x30` chain walkers (`0x806627F0`, `0x80665240`, `0x80666860`, `0x806675E0`, `0x8066F500`, `0x80673BA0`)

Six of the ten bodies are the *same* 224-byte routine, each self-recursive on
its own address: five unrolled "probe `[node+0x30]` through vtable slot `+0xC`,
then advance to that child" steps, followed by a `bl` back into the same
function with the fifth child. Ghidra renders this as nested `if (iVar1 == 0)`
blocks with a recursive call in the innermost one; the raw words show a flat
sequence of five `lwz r12,0x0 / lwz r12,0xC / bcctr` probes with a `cmpi` +
`bc 4,2` early exit. Translated as a `for _ in 0..5` loop plus a recursive tail,
the same shape already used by [`fn_80691250`](../src/game/bases.rs).

The routine returns the node whose *child* answers the query, not the child
itself, so the semantic is "parent of the first matching child". No in-REL
caller or vtable reference to these addresses exists, so the class they belong
to is not named here.

### Lemmy-ball state helpers (`0x80671510`, `0x80671A80`)

Both are members of the `0xC20`-byte Lemmy-ball actor already covered by
`fn_80671280`, `fn_80671620`, `fn_80671740`, `fn_80671810`, `fn_806719f0`,
`fn_80671bd0` and `fn_80671cd0`.

- `0x80671510` clears `+0x8A0`, keeps the `fn_80671280` step count as an `f32`
  (the `xoris 0x8000` / `0x43300000` pair with `0x807C9110` =
  `4503601774854144.0` is the signed-int widening), binds the `wait_ball_big`
  clip (`0x807E9ED0`) into `+0x544`/`+0x584`, seeds the frame read from
  `0x80168080(+0x584)` through `0x80168FE0`, and queues sound `0x54B`.
- `0x80671A80` advances `+0x544`, rebuilds `+0x8AC` and `+0x8B0` from `1.2`
  (`0x807C9128`) and `0.8` (`0x807C9130`) with step `0.013333334`
  (`0x807C912C`) through `0x8015F470`, multiplies both by actor slot `+0x284`
  into `+0xDC`/`+0xE0`, and computes
  `+0xB0 = +0x6C4 + +0xE0 * (slot288 / slot284)`.

### Sea-pose flag-pole (`0x80675960`)

Unregisters the `+0x148` collision area, advances `+0x3F0` (`+0x1C`) and
`+0x49C` (`+0x14`), and when `0x80168080(+0x430) > 60.0` (`0x807C927C`) resets
the block and restarts both clips at the `+0x4D8` scale. The tail guard
(`0x80169020` finished, `0x8008C4B0(+0x148, 1) as u16 == 0`) raises the
`0x80829E98` descriptor on `+0x394` and runs its `+0x1C` slot. This is the same
tail as the already-translated [`fn_80675c10`](../src/game/bases.rs), which uses
`0x80829F18`; `0x80829E98` was read from the REL and is the **empty string**.

### Shared time scaler (`0x80678300`)

Computes `32.0 * +0xDC + 8.0` (`0x807C93B0`/`0x807C93B4`) once, then walks the
`+0x4DC` list through `+0xC`: entries whose `+4` actor passes
[`fn_80678280`](../src/game/bases.rs) get flag bit `8` set; otherwise the
`+0x38C` state (`3` or `0`) and the `+0x274 & 0x3F` mask decide whether bit `8`
is cleared (mask clear) or set with scaling suppressed (mask set). Unsuppressed
entries rescale `+0x24` by `new / +0x4C8`, and `+0x4C8` is written last.

## Translated functions

| Address | Rust | Role (plain English) | Status | Confidence |
|---|---|---|---|---|
| `0x806627F0` | `fn_806627f0` | `+0x30` chain walk, five unrolled probes + recursive tail | translated | medium |
| `0x80665240` | `fn_80665240` | same chain walk, own recursive target | translated | medium |
| `0x80666860` | `fn_80666860` | same chain walk, own recursive target | translated | medium |
| `0x806675E0` | `fn_806675e0` | same chain walk, own recursive target | translated | medium |
| `0x8066F500` | `fn_8066f500` | same chain walk, own recursive target | translated | medium |
| `0x80671510` | `fn_80671510` | enter the `wait_ball_big` Lemmy-ball state, sound `0x54B` | translated | medium |
| `0x80671A80` | `fn_80671a80` | rebuild the two Lemmy-ball scale tracks and derive `+0xB0` | translated | high |
| `0x80673BA0` | `fn_80673ba0` | same chain walk, own recursive target | translated | medium |
| `0x80675960` | `fn_80675960` | end the flag-pole sea pose after frame `60.0` | translated | high |
| `0x80678300` | `fn_80678300` | rescale every `+0x4DC` entry by the shared time ratio | translated | high |

**Totals:** 10 translated, 0 deferred, 0 partial, 0 stub.

## Deferred

None. All ten assigned functions are literal translations; none contained
`unaff_GQR*`/`ldexpf` reconstruction beyond the `psq_l`/`psq_st` prologue noise
that the existing `fn_80671620`/`fn_80671810` bodies already ignore.

## Unknowns preserved

1. **Chain-walk semantics.** `+0x30` is documented only as the chain link and
   slot `+0xC` only as a zero/non-zero query. Neither the class nor the query
   name is invented; the six bodies are labelled medium for that reason.
2. **`0x80168FE0` role in `0x80671510`.** The call shape (`f1` = frame read
   back from `0x80168080(+0x584)`, `r3` = `+0x584`) is literal; the pairing is
   named from `fn_80671740`/`fn_80671bd0` and marked medium.
3. **`0x80829E98`.** Read from the REL as the empty string. It is passed
   verbatim, not substituted.
4. **`fn_80678280`'s state constants.** `+0x38C == 3` is reused exactly as
   translated in that body; no new meaning is attached.

## Verification

`work/wave192/laneD_block.rs` was appended to a scratch copy of the crate and
`cargo check --offline` finished with no errors and no warning naming any of the
ten bodies. `src/game/bases.rs` itself was not modified.
