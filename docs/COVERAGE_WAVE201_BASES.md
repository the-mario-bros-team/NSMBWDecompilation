# Coverage - Wave 201 (`game::bases` interface-dispatch completions)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 16 | 1,948 | 16 | 0 |

These sixteen bodies previously carried "cannot model the hidden `r12`
this-adjust pointer" comments and called `0x802DCBFC` with Ghidra's
pseudo-arguments. All are now modelled through the dispatcher helpers.

## Results by shape

- **Embedded descriptor (`interface + K`), 14 bodies.** Lanes A–D and E's
  `fn_806f9890`: `interface_call(interface, object, K)` with `K` in
  `{0x0C, 0x18, 0x24}`. The receiver is `r4` moved into `r3`; the descriptor
  base is the original `r3`.
- **Dynamic descriptor table, 2 bodies.** `fn_80762890` and `fn_8075c550`
  compute `r12 = table + 12 * [object + field]` (`0x807CEE4C` and
  `0x807CEC20`), so the descriptor address is not a compile-time constant. Their
  bodies read the triple and dispatch inline.
- **`fn_8064b660`** was corrected from `this + 0x438` to **`this + 0x42C`**;
  `+0x438` is written by `fn_8064b1c0` and never read by the dispatcher. The
  class initialiser `0x8064B2E0` fills `+0x42C` from
  `0x807C85A0 + ((*(this+4) & 3) * 12)`, and all four records are
  `{0, -1, fn}`.

## Dispatcher correction (affects all 473 thunks)

The shortcut branch `0x4180000C` is `bt 0` (`BO=12`, `BI=0`), i.e. **branch if
`cr0.LT`** - the condition is `index < 0`, not `index == 0`. Data confirms it:
`0x807CEE4C` holds `{0, -1, 0x80762C50}`, a negative index pointing at a `blr`
stub. `interface_call_desc` now reads the index as `i32` and shortcuts on
`index < 0`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations.
- All sixteen were verified in scratch copies by their lanes before merge.
- Ledger: 5,606 translated + 1 remaining `partial` (`0x806B3BB0`) in
  `game::bases`.
