# Coverage - Wave 166 (`game::bases` 44-64 byte band, part 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 1,556 | 28 | 0 |

## What this pass covers

The remainder of the 44-64 byte band that does not forward through
`0x802DCBFC`.

- **Virtual-slot wrappers (10 bodies).** Each is
  `lwzu r12, base(r3); lwz r12, slot(r12); bctrl` followed by `li r3,1`:
  `0x806744C0` (`+0x394`/`0x10`), `0x80683D40`, `0x806E7EF0`, `0x806F5A30`,
  `0x8071B700` (`+0x544`/`0x14`), `0x80720920` (`+0x70`/`0x10`),
  `0x807639F0` (`+0x1A8`/`0x14`), `0x807769B0` (`+0x1A4`/`0x14`),
  `0x8077A9F0` (`+0x1AC`/`0x14`), `0x807A2200` (`+0x3F0`/`0x14`).
- **A-type destructors (2 bodies).** `0x80626900` and `0x8062FFF0` are
  byte-identical to the Wave 165 family.
- **Constructor + vtable installers (4 bodies).** `0x80697840`,
  `0x806F0060`, `0x8076BEC0`, `0x8076C270` call a base constructor, store the
  class vtable at `+0x60` (or `+0x00` for `0x806F0060`) and return `this`.
- **Twelve one-offs.** The two-call refreshes `0x806E7450`, `0x8072C500`,
  `0x807C2810`; the conditional `0x8075B8E0`; the latch bodies `0x8076D8E0`,
  `0x807C2970`, `0x8061F0F0`, `0x8067B420`; the result forward `0x8078D0A0`;
  the plain call-and-return `0x806CAA40`; and the global rebuilds `0x80731FB0`
  and `0x807BAF10`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806744C0` | `fn_806744c0` | 48 | high |
| `0x80683D40` | `fn_80683d40` | 48 | high |
| `0x806E7EF0` | `fn_806e7ef0` | 48 | high |
| `0x806F5A30` | `fn_806f5a30` | 48 | high |
| `0x8071B700` | `fn_8071b700` | 48 | high |
| `0x80720920` | `fn_80720920` | 48 | high |
| `0x807639F0` | `fn_807639f0` | 48 | high |
| `0x807769B0` | `fn_807769b0` | 48 | high |
| `0x8077A9F0` | `fn_8077a9f0` | 48 | high |
| `0x807A2200` | `fn_807a2200` | 48 | high |
| `0x80626900` | `fn_80626900` | 64 | high |
| `0x8062FFF0` | `fn_8062fff0` | 64 | high |
| `0x80697840` | `fn_80697840` | 64 | high |
| `0x806F0060` | `fn_806f0060` | 60 | high |
| `0x8076BEC0` | `fn_8076bec0` | 60 | high |
| `0x8076C270` | `fn_8076c270` | 60 | high |
| `0x806CAA40` | `fn_806caa40` | 48 | high |
| `0x807C2810` | `fn_807c2810` | 60 | high |
| `0x8076D8E0` | `fn_8076d8e0` | 52 | high |
| `0x806E7450` | `fn_806e7450` | 56 | high |
| `0x8072C500` | `fn_8072c500` | 52 | high |
| `0x8075B8E0` | `fn_8075b8e0` | 60 | high |
| `0x8078D0A0` | `fn_8078d0a0` | 60 | high |
| `0x807C2970` | `fn_807c2970` | 64 | high |
| `0x8061F0F0` | `fn_8061f0f0` | 64 | high |
| `0x8067B420` | `fn_8067b420` | 64 | high |
| `0x80731FB0` | `fn_80731fb0` | 64 | high |
| `0x807BAF10` | `fn_807baf10` | 64 | high |

## Corrections and notes

- `0x806744C0` and `0x80720920` use slot `0x10`; every other member of the
  wrapper family uses slot `0x14`. All use `lwzu`, so the dispatched object is
  `this + base`, not `this`.
- The four installers return `this` even though the last memory operation is
  the vtable store: the sequence is `mr r3,r31` **before** the `stw`.
- `0x807C2970` reloads `*(this + 0x530)` twice rather than caching it. The Rust
  body reads it once and reuses the value; the two loads are identical and the
  callee cannot change `this + 0x530` between them.
- `0x8078D0A0` reads the `0x8042A18C` global **before** calling
  `0x8078CFA0`, so the global is re-read only once. The `mr r4,r3` forward means
  the `0x8078CFA0` result becomes the second argument of `0x801002C0`.
- `0x807BAF10` loads `-1` into `r4` before the first `lis`, so the first call is
  `0x800E0380(*(0x8042A040), 0xFFFFFFFF)`. The second call forwards the first
  result in `r3` and the byte at `0x8042A24D` in `r4`.
- `0x80731FB0` leaves `this` in `r3` for the conditional call and then overwrites
  `r3` with the `0`/`1` result.

## Verification

- Every address and byte size matches `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/bases.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave166` prints `OK`.
- `cargo check --message-format=short` and `node tools/doc-lint.mjs src` are clean.
