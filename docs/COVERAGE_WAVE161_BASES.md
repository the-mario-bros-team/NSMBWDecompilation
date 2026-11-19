# Coverage - Wave 161 (`game::bases` constant getters)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 55 | 660 | 55 | 0 |

## What this pass covers

All 55 bodies are twelve-byte `lis`/`addi`/`blr` accessors that return a
`main.dol` (or BSS) object address. They are the cheapest remaining supply: one
shifted constant, no loads, no branches, no side effects.

| target | bodies | addresses |
|---|---:|---|
| `0x80377A04` | 39 | `0x8065BFB0..0x8065C000`, `0x806654F0..0x80665520`, `0x80666AD0`/`E0`, `0x806A9700..0x806A9730`, `0x806AB6B0..0x806AB6D0`, `0x806B4DB0`, `0x806BAAB0`/`C0`, `0x806BF660`/`670`, `0x806C3930..0x806C3980`, `0x806C62A0..0x806C62C0`, `0x8071F940`, `0x80721020`, `0x80722930..0x80722960` |
| `0x80352670` | 2 | `0x80662700`, `0x80665150` |
| `0x80352730` | 1 | `0x80665120` |
| `0x803526F0` | 1 | `0x80665130` |
| `0x803526B0` | 1 | `0x80665140` |
| `0x80357E9C` | 1 | `0x806A5DD0` |
| `0x80357E5C` | 1 | `0x806A5DE0` |
| `0x80357DDC` | 1 | `0x806A5DF0` |
| `0x80357874` | 1 | `0x806BE840` |
| `0x80358104` | 1 | `0x806E9B20` |
| `0x80357BF4` | 1 | `0x806F8780` |
| `0x8082CB44` | 3 | `0x806A8270`, `0x806AEEB0`, `0x806BC310` |
| `0x8082CB04` | 2 | `0x806AEEC0`, `0x806BC320` |

## Negative-displacement corrections

Five getters use `addi r3,r3,-0x34BC`/`-0x34FC` on `lis r3,0x8083`, which wraps
to `0x8082CB44`/`0x8082CB04`, and `0x806E9B20` uses `addi r3,r3,-0x7EFC` on
`lis r3,0x8036`, which wraps to `0x80358104`. The generated constants were
recomputed from the raw words rather than copied from the printed EA.

## Functions

| address | target | bytes | confidence |
|---|---|---:|---|
| `0x8065BFB0` | `0x80377A04` | 12 | high |
| `0x8065BFC0` | `0x80377A04` | 12 | high |
| `0x8065BFD0` | `0x80377A04` | 12 | high |
| `0x8065BFE0` | `0x80377A04` | 12 | high |
| `0x8065BFF0` | `0x80377A04` | 12 | high |
| `0x8065C000` | `0x80377A04` | 12 | high |
| `0x80662700` | `0x80352670` | 12 | high |
| `0x80665120` | `0x80352730` | 12 | high |
| `0x80665130` | `0x803526F0` | 12 | high |
| `0x80665140` | `0x803526B0` | 12 | high |
| `0x80665150` | `0x80352670` | 12 | high |
| `0x806654F0` | `0x80377A04` | 12 | high |
| `0x80665500` | `0x80377A04` | 12 | high |
| `0x80665510` | `0x80377A04` | 12 | high |
| `0x80665520` | `0x80377A04` | 12 | high |
| `0x80666AD0` | `0x80377A04` | 12 | high |
| `0x80666AE0` | `0x80377A04` | 12 | high |
| `0x806A5DD0` | `0x80357E9C` | 12 | high |
| `0x806A5DE0` | `0x80357E5C` | 12 | high |
| `0x806A5DF0` | `0x80357DDC` | 12 | high |
| `0x806A8270` | `0x8082CB44` | 12 | high |
| `0x806A9700` | `0x80377A04` | 12 | high |
| `0x806A9710` | `0x80377A04` | 12 | high |
| `0x806A9720` | `0x80377A04` | 12 | high |
| `0x806A9730` | `0x80377A04` | 12 | high |
| `0x806AB6B0` | `0x80377A04` | 12 | high |
| `0x806AB6C0` | `0x80377A04` | 12 | high |
| `0x806AB6D0` | `0x80377A04` | 12 | high |
| `0x806AEEB0` | `0x8082CB44` | 12 | high |
| `0x806AEEC0` | `0x8082CB04` | 12 | high |
| `0x806B4DB0` | `0x80377A04` | 12 | high |
| `0x806BAAB0` | `0x80377A04` | 12 | high |
| `0x806BAAC0` | `0x80377A04` | 12 | high |
| `0x806BC310` | `0x8082CB44` | 12 | high |
| `0x806BC320` | `0x8082CB04` | 12 | high |
| `0x806BE840` | `0x80357874` | 12 | high |
| `0x806BF660` | `0x80377A04` | 12 | high |
| `0x806BF670` | `0x80377A04` | 12 | high |
| `0x806C3930` | `0x80377A04` | 12 | high |
| `0x806C3940` | `0x80377A04` | 12 | high |
| `0x806C3950` | `0x80377A04` | 12 | high |
| `0x806C3960` | `0x80377A04` | 12 | high |
| `0x806C3970` | `0x80377A04` | 12 | high |
| `0x806C3980` | `0x80377A04` | 12 | high |
| `0x806C62A0` | `0x80377A04` | 12 | high |
| `0x806C62B0` | `0x80377A04` | 12 | high |
| `0x806C62C0` | `0x80377A04` | 12 | high |
| `0x806E9B20` | `0x80358104` | 12 | high |
| `0x806F8780` | `0x80357BF4` | 12 | high |
| `0x8071F940` | `0x80377A04` | 12 | high |
| `0x80721020` | `0x80377A04` | 12 | high |
| `0x80722930` | `0x80377A04` | 12 | high |
| `0x80722940` | `0x80377A04` | 12 | high |
| `0x80722950` | `0x80377A04` | 12 | high |
| `0x80722960` | `0x80377A04` | 12 | high |

## Verification

- Every address and byte size matches `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/rel-d_basesNP/export/` path.
- Every row has exactly one body in `src/game/bases.rs`.
- No address was already in `ledger/functions.csv` (all 55 were absent).
- `node tools/check-lane-csv.mjs wave161` prints `OK`.
- `cargo check --message-format=short` and `node tools/doc-lint.mjs src` are clean.
