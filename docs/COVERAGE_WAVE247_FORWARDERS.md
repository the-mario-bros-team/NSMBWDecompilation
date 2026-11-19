# Wave 247, lane A - remaining `r12` forwarders

## Scope

Convert the 120 forwarders left `partial` by wave 246 lane A to the shared
`game::bases::interface_call(interface, object, offset)` model. They are filed
under `game::bosses` (19), `game::enemies` (90), `game::player` (9) and
`game::profile` (2). The four non-forwarders in the same bucket were left
untouched.

## Body verification

Each of the 120 addresses was read as twelve raw big-endian words from its
shipped REL (the nine `game::player` rows from `work/dol/main.elf`, the rest
from `work/rels/elf/d_*.elf`). Every body has the same 48-byte shape:

```
stwu r1,-16(r1) ; mfspr r0 ; mr r5,r3 ; mr r3,r4 ; stw r0,20(r1)
addi r12,r5,K ; bl 0x802DCBFC ; ori r0,r0,0 ; lwz r0,20(r1)
mfspr r0 ; addi r1,r1,16 ; blr
```

The verifier checked, per row, that `mr r5,r3`, `mr r3,r4`, exactly one
`addi r12,r5,K` and a `bl` whose computed absolute target is `0x802DCBFC` are
present. All 120 passed; no row needed a `lis`/`addi` descriptor or a
table-derived address. `work/wave247/forwarders_verified.csv` records the
address, `K`, target and shape verdict for every row.

`K` census for the 120 converted rows:

| module | `0x18` | `0x24` | `0x0C` | total |
|---|---:|---:|---:|---:|
| `game::bosses` | 9 | 10 | 0 | 19 |
| `game::enemies` | 44 | 45 | 1 | 90 |
| `game::player` | 4 | 4 | 1 | 9 |
| `game::profile` | 1 | 0 | 1 | 2 |
| **total** | **58** | **59** | **3** | **120** |

Together with the 25 wave-246 rows the full class is `0x18` 71, `0x24` 71,
`0x0C` 3 (145 forwarders), matching the wave-246 census.

## Rewrite

Each body now reads

```rust
pub unsafe fn fn_XXXXXXXX(interface: u32, receiver: u32) {
    // observed: `mr r5,r3`, `mr r3,r4`, `addi r12,r5,K`, `bl 0x802DCBFC`.
    interface_call(interface as usize, receiver, K);
}
```

and its `# Confidence` block was updated from `medium`/`partial` to `high`,
naming the four-instruction sequence and the descriptor offset. Local parameter
names (`param_1`/`param_2`, `state`/`object`, `first`/`second`) were preserved.
`use crate::game::bases::interface_call;` was added to `enemies.rs`, `player.rs`
and `profile.rs`; `bosses.rs` already imported it.

## Spot check

Ten rewritten rows were re-disassembled and compared against the Rust that now
compiles (`work/wave247/spotcheck10.txt`): `0x80C2E460`, `0x80C30050`,
`0x80A11530`, `0x80A2D270`, `0x80A61410`, `0x80A61440`, `0x80A79430`,
`0x80A8A910`, `0x80041BA0`, `0x80041BD0`. All ten show the same literal `K`
and a `bl 0x802DCBFC`; the Rust `interface_call` argument matches.

A whole-class source scan also confirmed all 120 functions contain
`interface_call(..., K)` with the verified `K` and that no `call3(...)` line
remains in any converted body.

## Residue

The original 124 `partial` rows are now 4 genuine non-forwarders, unchanged:

* `game::enemies` `0x80A0DB40`
* `game::enemies` `0x80AEEA30`
* `game::player` `0x80049940`
* `game::player` `0x8004DB00`

## Ledger and gate

* 120 rows re-adjudicated to `translated` / `high` in
  `ledger/agents/wave247-forwarders.csv`, merged into `ledger/functions.csv`.
* `cargo check` - clean.
* `doc-lint` (`node tools/doc-lint.mjs src`) - 0 violations.
* `scripts/check.ps1` - `cargo check`, `doc-lint`, `merge-ledger`,
  `verify-ledger --enforce-bodies`, index/coverage/progress/gap regeneration
  all pass (`OK`).
