# Wave 200 - lane F coverage

Lane F of the wave-200 `game::bases` interface-thunk sweep:

- audited/translated bodies: **76**
- range: `0x8074_2920` .. `0x807C_3D40`
- source block: `work/wave200/laneF_block.rs`
- ledger: `ledger/agents/wave200-lanef.csv`
- Rust destination: `src/game/bases.rs` (appended by the coordinator; not edited here)

## Shape

Every body is the same 48-byte (`12` word) REL thunk:

```
stwu r1,-16(r1); mflr r0; mr r5,r3; mr r3,r4; stw r0,20(r1);
addi r12,r5,K; bl 0x802DCBFC; ori r0,r0,0; lwz r0,20(r1);
mtlr r0; addi r1,r1,16; blr
```

so all 76 are modelled as `interface_call(interface, object, K)` against the
shipped `0x802DCBFC` dispatcher already present in `src/game/bases.rs`.

## Per-offset histogram

| `K` (`addi r12,r5,K`) | bodies |
| --- | ---: |
| `0x0C` | 12 |
| `0x18` | 37 |
| `0x24` | 27 |
| **total** | **76** |

## Method

1. Read the raw words for every one of the 76 addresses with
   `ppc159.js work/rels/elf/d_basesNP.elf <addr>` (`LEN=12`) - all 76, not a
   sample, so the first (`0x80742920`) and last (`0x807C3D40`) are included.
2. Verified per address, all 76: exactly 12 words / 48 bytes; `mr r5,r3` and
   `mr r3,r4` present; `bl 0x802DCBFC`; `addi r12,r5,K` immediate equal to the
   body's descriptor offset; and the prologue/epilogue sequence
   (`stwu`/`mflr`/`stw`/`ori r0,r0,0`/`lwz`/`mtlr`/`addi r1,r1,16`/`blr`).
3. Cross-checked the 48-byte size against
   `work/ghidra/rel-d_basesNP/export/functions.tsv` (all 76 match).
4. Cross-checked the `interface_call(interface, object, K)` argument of each
   body against the `addi` immediate and against the doc block's stated offset
   (all 76 match); confirmed each `work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`
   exists and shows the same `func_0x802dcbfc` forwarding.

## Corrections

None. No body in `work/wave200/laneF_block.rs` was wrong; no deferrals.
