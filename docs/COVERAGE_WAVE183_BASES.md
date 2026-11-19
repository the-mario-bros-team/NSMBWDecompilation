# Coverage - Wave 183 (`game::bases` sub-300 sweep, part 3)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 4,680 | 49 | 1 |

Third sub-300 wave, 10 bodies per lane. Covers `0x80684B10` through
`0x806CB210`.

## Composition

- **Destructors / release wrappers (26).** The dominant family again:
  `0x8015F800`, `0x802BE7C0`, `0x800A2F40` and `0x80169D20(this, -1)` teardown
  variants, all returning `this` and freeing when `free > 0`.
- **State/vtable tails (8).** `0x806A81B0`, `0x806A8210`, `0x806A93D0`,
  `0x806A9430`, `0x806AB340`, `0x806AEDF0`, `0x806AEE50`, `0x806AF740` -
  class state destructors whose class names come from the
  `..._c::StateID_*` strings emitted ahead of each vtable cluster.
- **Hooks and predicates (5).** `0x806907D0` (event rotation hook),
  `0x80696E00` (ground hook), `0x806C1B00` (folded predicate), `0x80698A20`
  (vtable product), `0x806CB210` (handle resolve).
- **Factories (2).** `0x8068F0D0` (`0x398`, vtable `0x807EF664`), `0x80687380`
  (`0x14`-stride array manager).
- **Arithmetic/state helpers (9).** including `0x806861E0`, `0x806914D0`
  (`sin * 1024.0`), `0x806940A0`, `0x806B4ED0`, `0x806B4B20`.

## Corrections and cross-checks

- **`fn_806c3710` flag was inverted.** `0x806B4B20` and `fn_806c3710` share the
  `mfcr` → `rlwinm r0,r0,2,31,31` → `cntlzw` chain, which isolates **CR0[GT]**:
  the flag is `1` exactly when `delta <= 0.0`, not `delta >= 0.0`. The body and
  its doc are corrected, and the doc now notes `0x806C3790` ignores the value.
- `0x806B4B20`'s raw `fsubs f1,f2,f1` with `f2 = this.x` proves
  `this.x - other.x`; Ghidra reverses it.
- `0x806B9CB0`'s `0x806B9720` does take the actor (`mr r3`); Ghidra drops it.
- `0x806C1B00`'s `cntlzw` tail is `(value == 0) as u32`, not `!= 0`.
- `0x80064540`'s argument is `this` (`mr r31,r3` at `0x806C1B10`).
- `0x8068F0D0`'s constructor argument (`r3` = fresh object) was restored from
  raw words.
- `0x806907D0` is **medium**: the local decoder prints `X31_104`/`X31_922` at
  `0x806907FC..0x80690800`; the surviving `extsh` + `addi 0x4000` pair and
  Ghidra agree on the negation, so that reading is followed and the
  discrepancy documented.
- `0x806AB8E0`'s callee `0x8027A200` has no established argument list; no
  class name is claimed because no vtable word in this REL references it.
- `0x8082BD04` is outside the REL, so it is passed as an opaque pointer
  literal, matching the neighbouring `0x806ABCD0`.

## Verification

- `cargo check` succeeds with 4,469 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave183`: 50 rows, 50 unique addresses,
  49 high + 1 medium.
- All 50 evidence exports exist and every size matches `functions.tsv`
  (4,680 bytes total).
