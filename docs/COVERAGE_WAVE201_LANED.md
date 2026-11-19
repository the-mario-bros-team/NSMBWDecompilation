# Wave 201 lane D - `game::bases` interface-dispatch forwarders

Three 48-byte bodies that were previously `partial` because they call the
`main.dol` interface dispatcher `0x802DCBFC` with a hidden `r12` descriptor.
All three are now `high` confidence: in every body `r5` receives `r3` (the
interface base), `r3` receives `r4` (the receiver), and the descriptor address
is a single `addi r12,r5,K`. No `lis`/`addi` pair appears, so the descriptor is
always relative to the interface base and the right helper is
`interface_call(interface, object, offset)` - never `interface_call_desc`.

Dispatcher recap (shipped `0x802DCBFC`):

```
r0  = [r12+0] self_adjust ; r11 = [r12+4] index ; r12 = [r12+8] table/fn
r3  = r3 + r0
if r11 != 0: r12 = *(*(r3+r12) + r11)
ctr = r12 ; bctr
```

## Bodies

### `fn_806f8a10` - `0x806F8A10` (48 bytes)

Raw words:

```
806f8a18  mr   r5,r3        ; r5 = interface base (first)
806f8a1c  mr   r3,r4        ; r3 = receiver (second)
806f8a24  addi r12,r5,24    ; descriptor at first + 0x18
806f8a28  bl   0x802dcbfc
806f8a2c  ori  r0,r0,0      ; nop padding after the call
```

- `r12` source: `addi r12,r5,24` with `r5 = r3` (original `first`).
- `object`: `second` (moved `r4 -> r3`), **not** `first`.
- Final form: `interface_call(first, second as u32, 0x18)`.

### `fn_806f8a40` - `0x806F8A40` (48 bytes)

Identical sequence with `addi r12,r5,36` (`+0x24`).

- `r12` source: `addi r12,r5,36` with `r5 = r3` (original `first`).
- `object`: `second`.
- Final form: `interface_call(first, second as u32, 0x24)`.

### `fn_806f9860` - `0x806F9860` (48 bytes)

Identical sequence with `addi r12,r5,24` (`+0x18`); sits directly above the
three-descriptor registration `fn_806f98c0`, whose sibling `fn_806f9890`
forwards the neighbouring descriptor at `+0xC` instead.

- `r12` source: `addi r12,r5,24` with `r5 = r3` (original `first`).
- `object`: `second`.
- Final form: `interface_call(first, second as u32, 0x18)`.

## Notes

The old bodies passed `abi_call3!(0x802D_CBFC, second, second, first)`, which
replays the Ghidra pseudo-signature `(param_2, param_2, param_1)` as if it were
`(r3, r4, r5)`. That is wrong twice over: at the `bl` only `r3` (= `second`) is
live as an argument, and the descriptor pointer lives in `r12`, which no
`abi_callN!` can set. The dispatcher also adds `self_adjust` to `r3` before the
indirection, so the receiver can never be passed pre-adjusted.

Signatures were kept as `(first: usize, second: usize)` (no return value) to
avoid churn; nothing in the crate calls these three functions. The dispatcher's
`r3` does survive the wrapper, but it is a forwarded result only.

## Verification

Crate copied to `%TEMP%\opencode\w201laned`, the three bodies replaced in
`src/game/bases.rs`, and `cargo check --lib` exited 0 (23 pre-existing
unrelated warnings, none pointing at the patched lines).

## Deferred

None.
