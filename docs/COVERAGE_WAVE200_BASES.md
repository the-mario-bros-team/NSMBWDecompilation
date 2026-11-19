# Coverage - Wave 200 (`game::bases` interface thunks)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 473 | 23,332 | 473 | 0 |

The 472 "forwarders" the project had carried as untranslatable are in fact a
single uniform thunk family, plus the 44-byte `0x806E3BE0` of the same shape.

## The dispatcher

Every thunk is 48 bytes / 12 instructions:

```
stwu r1,-16(r1); mflr r0; mr r5,r3; mr r3,r4; stw r0,20(r1);
addi r12,r5,K; bl 0x802DCBFC; ori r0,r0,0; lwz r0,20(r1);
mtlr r0; addi r1,r1,16; blr
```

`0x802DCBFC` is shipped in `main.dol` and is a small interface dispatcher:

```
r0  = [desc+0]    self_adjust
r11 = [desc+4]    slot index
r12 = [desc+8]    vtable offset, or a direct function address when index == 0
r3  = r3 + r0
if r11 != 0: r12 = *(*(r3 + r12) + r11)
ctr = r12; bctr
```

So each thunk forwards `(interface, object)` to the method described by the
descriptor embedded at `interface + K`. `interface_call()` in `bases.rs` models
this exactly.

## Distribution

| Descriptor offset | Bodies |
|---|---:|
| `0x0C` | 54 |
| `0x18` | 234 + `0x806E3BE0` |
| `0x24` | 184 |

All 472 decode as one family (the `mr r5,r3` / `mr r3,r4` shuffle is present in
every body).

## Verification

Six audit lanes decoded all 471 generated bodies from the raw ELF words and
compared:

- the `addi r12,r5,K` immediate against the Rust constant and the manifest;
- the `bl` displacement, recomputed to resolve at `0x802DCBFC`;
- the size against `functions.tsv` (48 bytes for every thunk);
- the doc block's address, size and evidence path.

**Result: 471/471 bodies, zero mismatches, zero corrections.** A scratch-copy
`cargo check` of all six blocks also exited 0 before the merge.

## Remaining

Only the two documented deferrals are still untranslated in `game::bases`:

- `0x806E2BF0` - its loop writes only into its own frame (post-link `nop`).
- `0x80697090` - its target `(x, y)` is read from stack slots it never writes.
