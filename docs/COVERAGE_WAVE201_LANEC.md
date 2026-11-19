# Coverage: wave 201 lane C - `0x802DCBFC` interface thunks

## Scope

Three 48-byte bodies in `src/game/bases.rs` that were previously recorded as
opaque `abi_call3!(0x802D_CBFC, second, second, first)` forwarders with a
"hidden r12 this-adjust pointer" caveat. All three only set up the dispatcher's
`r12` descriptor pointer and tail-call `0x802DCBFC` in `main.dol`, so the
hidden pointer was never hidden: it is a compile-time-selected `addi` from the
`interface` argument.

Bodies were disassembled directly from `work/rels/elf/d_basesNP.elf`
(`$env:LEN='36'`, `$env:STOP='0'`) and cross-checked against
`work/ghidra/rel-d_basesNP/export/functions.tsv` (48 bytes each).

## Shapes

All three share the same prologue:

```
stwu r1,-16(r1) ; mfspr r0,256 ; mr r5,r3 ; mr r3,r4 ; stw r0,20(r1)
addi r12,r5,<K>    ; bl 0x802DCBFC ; ori r0,r0,0 ; <epilogue> ; bclr
```

so the register roles are:

- `r3` at the call = incoming `r4` -> **the dispatched receiver is the second
  parameter** (the doubled `second` argument in the old body was right for
  `r3`/`r4`, but modelled the call as a raw triple).
- `r5` (incoming `r3`) is the **interface** whose descriptor is read; the
  `addi r12,r5,K` selects it.
- The `ori r0,r0,0` after the `bl` is a scheduling nop, not a call.
- `r3` survives the call untouched, so the thunk returns the dispatched
  method's `r3`.

`K` per function:

| function | address | size | `r12` setup | helper call |
| --- | --- | --- | --- | --- |
| `fn_80744ae0` | `0x80744AE0` | 48 | `addi r12,r5,0x24` | `interface_call(interface, object, 0x24)` |
| `fn_806f51c0` | `0x806F51C0` | 48 | `addi r12,r5,0x18` | `interface_call(interface, object, 0x18)` |
| `fn_806f51f0` | `0x806F51F0` | 48 | `addi r12,r5,0x24` | `interface_call(interface, object, 0x24)` |

No `lis`/`addi` literal descriptor form appears in any of the three bodies, so
`interface_call_desc` is not used.

## Signature change

The old stubs took `(first: usize, second: usize)` and returned nothing. The
replacement takes `(interface: usize, object: u32) -> u32`, matching the
already-established thunks in the same file (`fn_80601780`, `fn_806017b0`,
`fn_806e3be0`). No Rust caller existed for any of the three old stubs (grep over
`src/` shows only the module-header mention at `src/game/bases.rs:1069`), so the
signature change is source-compatible.

## Verification

Scratch copy of `Cargo.toml` + `src/` with the three items replaced by
`work/wave201/laneC_block.rs`: `cargo check --lib` exit 0 (23 pre-existing
warnings, none in `src/game/bases.rs` from these bodies).

## Deferred

None.

## Files

- `work/wave201/laneC_block.rs`
- `ledger/agents/wave201-lanec.csv`
- `docs/COVERAGE_WAVE201_LANEC.md`
