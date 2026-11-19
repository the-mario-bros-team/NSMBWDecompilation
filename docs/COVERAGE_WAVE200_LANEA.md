# Coverage - Wave 200 lane A (`game::bases` interface thunks)

Lane A of wave 200 audits the first block of the `0x802DCBFC` interface-thunk
family in `rel/d_basesNP.elf` (synthetic analysis base `0x80600000`; not a
runtime address). Every body in this lane was already generated, so this pass is
an audit, not a translation pass: the generated `interface_call` bodies were
checked word for word against the shipped REL and against
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

| File | Module | Bodies | Range | High | Medium |
|---|---|---:|---|---:|---:|
| `work/wave200/laneA_block.rs` | `game::bases` | 79 | `0x806017B0` .. `0x8064C370` | 79 | 0 |

Ledger rows: `ledger/agents/wave200-lanea.csv` (one row per address, all
`translated` / `high`).

## The thunk shape

All 79 bodies are the same 12-instruction, 48-byte sequence:

```
stwu  r1,-16(r1)      ; 0x9421fff0
mflr  r0              ; 0x7c0802a6
mr    r5,r3           ; 0x7c651b78   interface -> r5
mr    r3,r4           ; 0x7c832378   object    -> r3
stw   r0,20(r1)       ; 0x90010014
addi  r12,r5,K        ; 0x39850000 | K
bl    0x802DCBFC      ; shipped dispatcher
ori   r0,r0,0         ; 0x60000000 (nop pad)
lwz   r0,20(r1)       ; 0x80010014
mtlr  r0              ; 0x7c0803a6
addi  r1,r1,16        ; 0x38210010
blr                   ; 0x4e800020
```

The only varying instruction is `addi r12,r5,K`. The model is
`interface_call(interface, object, K)`: `r5 = interface` becomes the descriptor
base, `r3 = object` the receiver, and the constant `K` the descriptor offset
into the interface.

## Offset distribution

Three descriptor offsets occur in this lane:

| `K` | Offset | Bodies |
|---|---|---:|
| `0x0C` | 12 | 37 |
| `0x18` | 24 | 39 |
| `0x24` | 36 | 3 |

The three `0x24` bodies are `0x8064A120`, `0x8064B190` and `0x8064C370`; the
last two are the pair at the end of the lane. `0x0C`/`0x18` alternate in pairs
across almost the whole range.

## Verification method

1. **Whole-lane word decode.** All 79 addresses were read straight from the
   ELF (`work/rels/elf/d_basesNP.elf`, program headers, `u32` at vaddr) and
   each of the 12 words compared against the expected shape above, including
   the exact `bl` displacement to `0x802DCBFC` (computed as
   `0x802DCBFC - (addr + 24)`), so a wrong code base could not pass silently.
   The `addi r12,r5,K` immediate was decoded as a signed 16-bit field and
   compared both to `K` in the Rust body and to the manifest entry.
2. **Rust body parse.** `work/wave200/laneA_block.rs` was parsed for
   `pub unsafe fn fn_<addr>(interface: usize, object: u32) -> u32` and its
   `interface_call(interface, object, 0x..)` constant; 79 bodies were found,
   exactly the 79 manifest addresses, with no duplicates or extras.
3. **Size cross-check.** `functions.tsv` reports 48 bytes for all 79 addresses,
   matching the 12 decoded words.
4. **Family batch check.** `node work/thunks.cjs` independently decodes all
   472 wave-200 thunks; every entry reports family `A` (the
   `mr r5,r3` / `mr r3,r4` pair present) with offsets 12/24/36 only.
5. **Individual spot checks.** 16 addresses were disassembled one by one with
   `ppc159.js` (`LEN=12`, `STOP=0`), including the first (`0x806017B0`) and
   last (`0x8064C370`) of the lane and both `0x24` endpoints: `0x806017B0`,
   `0x80604360`, `0x80604390`, `0x806064E0`, `0x80609B10`, `0x80611930`,
   `0x8061BC80`, `0x8061FAB0`, `0x80624E80`, `0x8062EFF0`, `0x806360D0`,
   `0x8063AEB0`, `0x806415A0`, `0x806465C0`, `0x8064B190`, `0x8064C370`.
   All 16 matched the expected shape and offset (`0x0C`, `0x18` or `0x24`).
6. **Evidence presence.** All 79 `work/ghidra/rel-d_basesNP/export/
   <addr>_FUN_<addr>.c` files exist. Their Ghidra bodies are the expected
   `func_0x802dcbfc(param_2, param_2, param_1)` call, which is the same
   forwarding with Ghidra's permuted parameter names.

## Corrections

**None.** No body in `work/wave200/laneA_block.rs` was wrong: all 79 offsets,
both register moves, the dispatcher target and the 48-byte sizes agree with the
ELF, `functions.tsv` and the manifest. No file was edited other than adding the
two audit outputs (`ledger/agents/wave200-lanea.csv` and this document).
