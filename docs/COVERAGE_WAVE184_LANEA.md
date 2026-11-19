# Wave 184 lane A coverage (`game::bases`)

Lane A translated 10 of 10 assigned bodies. Block output:
`work/wave184/laneA_block.rs`; ledger: `ledger/agents/wave184-lanea.csv`.
Nothing was deferred.

## Clusters

| Address | Size | Shape |
| --- | --- | --- |
| `0x806CBAF0` | 88 | `0x8015F800(x, 0)` + optional `0x802B9280(x)` destructor |
| `0x806CCED0` | 88 | same destructor |
| `0x806D26C0` | 88 | same destructor |
| `0x806D00E0` | 68 | scene ground-band window test (`0x80429E00 + 0x3C`/`+0x44`) |
| `0x806D0780` | 120 | player-slot release, pending word `+0x648`, id `0xC` |
| `0x806D22B0` | 120 | player-slot release, pending word `+0x53C`, id `0` |
| `0x806D3AC0` | 120 | player-slot release, pending word `+0x75C`, id `0xD` |
| `0x806E0EA0` | 92 | binds the `+0x6FC` partner and selects state `0x8082DEE8` |
| `0x806E12B0` | 76 | loader guard open/close around `+0x3D0` |
| `0x806E1780` | 76 | `0x3A8`-byte factory, vtable `0x807FC9E0` at `+0x60` |

## Notes

- The three 88-byte bodies are byte-identical to the already-translated
  `fn_80650980`/`fn_806511d0`; the block reuses that exact shape.
- `0x806D0780`/`0x806D22B0`/`0x806D3AC0` are the same gate as
  `fn_806df770`/`fn_80727220`: signed slot byte at `+0x430`, scene table
  pointer at `0x8042A320`, stride `0x23C`, gate byte `+0x381`, state word
  `+0x368`. Ghidra's `halt_baddata` is the `b 0x800B3600` tail branch
  (`0x806D07F4`, `0x806D2324`, `0x806D3B34`), emitted as `abi_call2!` because
  the C signature is `void`.
- `0x806D22B0` is named by the translated `fn_806d2390`, which calls it with a
  single actor argument; that confirms the one-parameter signature.
- `0x806D00E0` dereferences `0x80429E00` before reading `+0x3C`/`+0x44`
  (`lwz r3,-0x6200(r3)`); the two floats' meaning is inferred from the sibling
  band test in `fn_806513f0`, hence `medium` confidence.
- `0x806E1780` writes `0x807FC9E0`, not `0x8080C9E0`: the raw
  `lis r3,0x8080; addi r3,r3,-13856` encodes `0x807FC9E0`, matching Ghidra's
  `DAT_807fc9e0`.
