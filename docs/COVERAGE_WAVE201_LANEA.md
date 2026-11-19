# Wave 201 lane A - `game::bases` interface-dispatch forwarders

Three 48-byte bodies that were previously `partial` because they call the
`main.dol` interface dispatcher `0x802DCBFC` with a hidden `r12` descriptor.
All three are now `high` confidence: the descriptor base is formed by a single
`addi r12,r5,K` from `r5 = r3`, and the receiver is `r4` moved into `r3`.

Dispatcher recap (shipped `0x802DCBFC`):

```
r0  = [r12+0] self_adjust ; r11 = [r12+4] index ; r12 = [r12+8] table/fn
r3  = r3 + r0
if r11 != 0: r12 = *(*(r3+r12) + r11)
ctr = r12 ; bctr
```

Because the descriptor address is fully static in each body, they map to the
`interface_call(interface, object, offset)` helper (descriptor at
`interface + offset`), not to `interface_call_desc`.

## Bodies

### `fn_806512f0` - `0x806512F0` (48 bytes)

Raw words:

```
806512f8  mr   r5,r3          ; r5 = interface base (param_1)
806512fc  mr   r3,r4          ; r3 = receiver (param_2)
80651304  addi r12,r5,0x18    ; descriptor at param_1 + 0x18
80651308  bl   0x802dcbfc
```

- `r12` source: `addi r12,r5,0x18`, `r5 = r3` (original `param_1`).
- `object`: `param_2` (moved `r4 -> r3`), **not** `param_1`.
- Final form: `interface_call(param_1 as usize, param_2, 0x18)`.

### `fn_80651320` - `0x80651320` (48 bytes)

Identical sequence with `addi r12,r5,0x24`.

- `r12` source: `addi r12,r5,0x24`, `r5 = r3` (original `param_1`).
- `object`: `param_2`.
- Final form: `interface_call(param_1 as usize, param_2, 0x24)`.

### `fn_80714430` - `0x80714430` (48 bytes)

Identical sequence with `addi r12,r5,0x18`.

- `r12` source: `addi r12,r5,0x18`, `r5 = r3` (original `param_1`).
- `object`: `param_2`.
- Final form: `interface_call(param_1 as usize, param_2, 0x18)`.

## Notes

The old bodies passed `platform::call3(0x802D_CBFC, param_2, param_2, param_1)`,
which replays the Ghidra pseudo-signature `(param_2, param_2, param_1)` as if it
were `(r3, r4, r5)`. That is wrong: at the `bl`, `r3` is `param_2`, `r4` is dead
(`param_2` also), and `r5` is `param_1`. The dispatcher only consumes `r3` and
`r12`, so the correct model is the descriptor helper with `object = param_2`.

Signatures were kept as `(param_1: u32, param_2: u32)` to avoid churn; no Rust
caller references these three functions.

## Verification

Crate copied to a scratch directory, the three bodies replaced, and
`cargo check --lib` exited 0 (pre-existing unrelated warnings only).
