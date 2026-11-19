# Coverage - wave 201 lane B (`d_basesNP` interface-dispatcher thunks)

## How to read this file

This page records a three-body pass over `d_basesNP` (`game::bases`) that
replaces three `partial` forwarders into main.dol `0x802DCBFC` with real
`interface_call` bodies.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented.
- **Confidence** - `high` = the instruction sequence is explicit and the
  dispatcher semantics are read from the shipped `main.dol` body.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`.

**Synthetic base warning.** Addresses on this page are analysis addresses from
`tools/rel2elf.mjs` with base `0x80600000`; they are **not** runtime addresses.
`0x802DCBFC` is a `main.dol` address.

## The dispatcher

`0x802DCBFC` decodes a three-word descriptor addressed by `r12`:

```
r0  = [r12+0] self_adjust
r11 = [r12+4] index
r12 = [r12+8] table offset (or the function address when index == 0)
r3  = r3 + r0
if r11 != 0: r12 = *(*(r3+r12) + r11)
ctr = r12; bctr
```

`interface_call(interface, object, offset)` in `src/game/bases.rs` models this
exactly (`interface_call_desc` with `interface + offset`). A thunk therefore
needs the descriptor base object and the receiver separately: the receiver is
the value in `r3` at the `bl`, and the descriptor base is the value in `r12`
before the call.

## Thunk shape

All three bodies share one six-instruction frame:

```
mr    r5,r3        ; r5 = param_1 (interface / descriptor base)
mr    r3,r4        ; r3 = param_2 (receiver)
addi  r12,r5,K     ; descriptor at interface + K
bl    0x802DCBFC
```

The prior `partial` bodies passed `param_2` twice and dropped `param_1`, which
was wrong: `param_1` is the interface and `param_2` is the receiver.

## Translated functions

| Address | Size | Rust | `r12` setup | Helper call | Status | Confidence |
|---|---:|---|---|---|---|---|
| `0x80714460` | 48 | `fn_80714460` | `addi r12,r5,0x24` | `interface_call(interface, object, 0x24)` | translated | high |
| `0x80743EF0` | 48 | `fn_80743ef0` | `addi r12,r5,0x18` | `interface_call(interface, object, 0x18)` | translated | high |
| `0x80744AB0` | 48 | `fn_80744ab0` | `addi r12,r5,0x18` | `interface_call(interface, object, 0x18)` | translated | high |

No literal (`lis` + `addi`) descriptor appears in these three bodies, so
`interface_call_desc` is not used.

**Totals:** 3 translated, 0 deferred, 0 stub.

## Signature change

Each body changed from `(param_1: u32, param_2: u32)` / `(first, second)` with a
`void` result to:

```rust
pub unsafe fn fn_XXXXXXXX(interface: usize, object: u32) -> u32
```

The return value is the dispatcher's `r3`; the original epilogue returns it
unchanged (`bl` result then `bclr`). Ghidra typed `0x802DCBFC` as `void`, which
hides it.

## Verification

Scratch copy of the crate (`src/` + `Cargo.toml`) with these three items
substituted:

- `cargo check --lib` - exit 0.
- `node tools/doc-lint.mjs <scratch>/src` - `ok: true`, 68 files, 0 violations.

The coordinator applies `work/wave201/laneB_block.rs` to `src/game/bases.rs`;
this lane did not edit that file.
