# Coverage - Wave 165 (`game::bases` 44-64 byte band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 39 | 2,208 | 39 | 0 |

## What this pass covers

Three clusters from the 44-64 byte band, all of which recur across the REL.

- **A-type destructors (21 bodies, 1,344 bytes).** `0x8061AFF0`,
  `0x8061E710`, `0x80624190`, `0x80644FE0`, `0x80645D70`, `0x80646C90`,
  `0x8064A210`, `0x8064A250`, `0x80668CA0`, `0x806CEAC0`, `0x806CEB00`,
  `0x806DB300`, `0x806F1D30`, `0x806F2420`, `0x80705C20`, `0x807208D0`,
  `0x80730BA0`, `0x80764690`, `0x807AE8A0`, `0x807AFB30`, `0x807C1AE0`.
  Each checks `this != 0` and `count > 0`, calls `0x802B9280(this, count)`,
  and returns `this` unchanged.
- **Registered class releases (8 bodies, 384 bytes).** `0x8078A3A0`,
  `0x8078A740`, `0x8078B5B0`, `0x8078B7D0`, `0x8078B9F0`, `0x8078BC10`,
  `0x8078BE30`, `0x8078C060` dispatch a fixed class id through
  `0x800F29F0`/`0x800F2A00`.
- **Allocator + constructor factories (10 bodies, 480 bytes).**
  `0x80656CF0`, `0x8066BE70`, `0x80683720`, `0x806856B0`, `0x806BEB30`,
  `0x806BF630`, `0x806D2840`, `0x80709E90`, `0x807262C0`, `0x80757A80`
  allocate a fixed size through `0x801628C0` and run the matching constructor
  when the allocation succeeds.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8061AFF0` | `fn_8061aff0` | 64 | high |
| `0x8061E710` | `fn_8061e710` | 64 | high |
| `0x80624190` | `fn_80624190` | 64 | high |
| `0x80644FE0` | `fn_80644fe0` | 64 | high |
| `0x80645D70` | `fn_80645d70` | 64 | high |
| `0x80646C90` | `fn_80646c90` | 64 | high |
| `0x8064A210` | `fn_8064a210` | 64 | high |
| `0x8064A250` | `fn_8064a250` | 64 | high |
| `0x80668CA0` | `fn_80668ca0` | 64 | high |
| `0x806CEAC0` | `fn_806ceac0` | 64 | high |
| `0x806CEB00` | `fn_806ceb00` | 64 | high |
| `0x806DB300` | `fn_806db300` | 64 | high |
| `0x806F1D30` | `fn_806f1d30` | 64 | high |
| `0x806F2420` | `fn_806f2420` | 64 | high |
| `0x80705C20` | `fn_80705c20` | 64 | high |
| `0x807208D0` | `fn_807208d0` | 64 | high |
| `0x80730BA0` | `fn_80730ba0` | 64 | high |
| `0x80764690` | `fn_80764690` | 64 | high |
| `0x807AE8A0` | `fn_807ae8a0` | 64 | high |
| `0x807AFB30` | `fn_807afb30` | 64 | high |
| `0x807C1AE0` | `fn_807c1ae0` | 64 | high |
| `0x8078A3A0` | `fn_8078a3a0` | 48 | high |
| `0x8078A740` | `fn_8078a740` | 48 | high |
| `0x8078B5B0` | `fn_8078b5b0` | 48 | high |
| `0x8078B7D0` | `fn_8078b7d0` | 48 | high |
| `0x8078B9F0` | `fn_8078b9f0` | 48 | high |
| `0x8078BC10` | `fn_8078bc10` | 48 | high |
| `0x8078BE30` | `fn_8078be30` | 48 | high |
| `0x8078C060` | `fn_8078c060` | 48 | high |
| `0x80656CF0` | `fn_80656cf0` | 48 | high |
| `0x8066BE70` | `fn_8066be70` | 48 | high |
| `0x80683720` | `fn_80683720` | 48 | high |
| `0x806856B0` | `fn_806856b0` | 48 | high |
| `0x806BEB30` | `fn_806beb30` | 48 | high |
| `0x806BF630` | `fn_806bf630` | 48 | high |
| `0x806D2840` | `fn_806d2840` | 48 | high |
| `0x80709E90` | `fn_80709e90` | 48 | high |
| `0x807262C0` | `fn_807262c0` | 48 | high |
| `0x80757A80` | `fn_80757a80` | 48 | high |

## Corrections and notes

- The 21 destructors are byte-identical apart from the `bl 0x802B9280`
  displacement, which was confirmed by normalising every branch word.
- The release dispatchers do **not** pass `id, this, 0` to both arms. A null
  `this` leaves `r4` holding the incoming second argument and zeroes `r5`/`r6`,
  so the first arm is `0x800F29F0(id, arg, 0, 0)`. A non-null `this` moves it
  into `r4`, forwards the second argument in `r5` and zeroes `r6`/`r7`, so the
  second arm is `0x800F2A00(id, this, arg, 0, 0)`.
- The constructor targets of the ten factories are not translated yet, so the
  bodies use `abi_call1!` with the address and note the identity is unknown.
- `0x802DCBFC` stubs in this size band (473 of them) remain deferred, as in
  every previous wave: the stub target has no shipped body, so the `r12`
  argument cannot be proven.

## Verification

- Every address and byte size matches `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/bases.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave165` prints `OK`.
- `cargo check --message-format=short` and `node tools/doc-lint.mjs src` are clean.
