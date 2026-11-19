# Coverage - Wave 200 lane D (`game::bases` interface thunks)

Lane D of wave 200 audits the fourth block of the `0x802DCBFC` interface-thunk
family in `rel/d_basesNP.elf` (synthetic analysis base `0x80600000`; not a
runtime address). Every body in this lane was already generated, so this pass is
an audit, not a translation pass: the generated `interface_call` bodies were
checked word for word against the shipped REL and against
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

| File | Module | Bodies | Range | High | Medium |
|---|---|---:|---|---:|---:|
| `work/wave200/laneD_block.rs` | `game::bases` | 79 | `0x806C5D20` .. `0x806FAAD0` | 79 | 0 |

Ledger rows: `ledger/agents/wave200-laned.csv` (one row per address, all
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
| `0x0C` | 12 | 1 |
| `0x18` | 24 | 40 |
| `0x24` | 36 | 38 |

The single `0x0C` body is `0x806DBEA0`, the odd member of the `0x806DBE70` /
`0x806DBEA0` pair; every other body in the lane is one of the alternating
`0x18` / `0x24` pair members. The addresses themselves are sparse across the
whole `0x806C5D20` .. `0x806FAAD0` span (only thunk sites were extracted), and
the lane's last body `0x806FAAD0` is an offset-`0x18` member.

## Verification method

1. **Whole-lane `ppc159.js` decode.** All 79 addresses were disassembled
   individually from the shipped ELF (`LEN=16`, `STOP=0`, output captured per
   address), not merely a sample. For each address the first 12 words were
   compared against the fixed shape above, including the `bl` displacement
   decoded from the raw word (`(addr + 24) + LI`, sign-extended 26-bit field):
   all 79 resolve to `0x802DCBFC` and nothing else, so a wrong code base or a
   shifted function start could not pass silently.
2. **`addi r12,r5,K` immediate.** Decoded as a signed 16-bit field and compared
   both to the `interface_call(..., 0x..)` constant in
   `work/wave200/laneD_block.rs` and to the `off` field of the lane D manifest
   entry. 79/79 agree; no body encodes an offset it does not dispatch.
3. **Register moves.** `mr r5,r3` (`0x7c651b78`) and `mr r3,r4` (`0x7c832378`)
   are present in every body at word 2 and word 3, and the `ori r0,r0,0` nop
   pad, `lwz r0,20(r1)`, `mtlr r0`, `addi r1,r1,16`, `blr` epilogue is intact
   in all 79.
4. **Rust body parse.** `work/wave200/laneD_block.rs` was parsed for
   `pub unsafe fn fn_<addr>(interface: usize, object: u32) -> u32` plus its
   `interface_call(interface, object, 0x..)` constant: 79 bodies, all unique,
   sorted, exactly the 79 manifest addresses, no extras, 79 `# Original` and
   79 `# Confidence` doc sections.
5. **Size cross-check.** `functions.tsv` reports 48 bytes for all 79 addresses,
   matching the 12 decoded words and the `(48 bytes)` note in every doc block.
6. **Independent decoder cross-check.** `node work/thunks.cjs` re-decodes all
   472 wave-200 thunks straight from the ELF by pointer arithmetic. All 79 lane
   D addresses appear with family `A`, sizes 48, and offsets 12/24/36; 0
   missing, 0 mismatched against the manifest.
7. **Evidence presence.** All 79 `work/ghidra/rel-d_basesNP/export/
   <addr>_FUN_<addr>.c` files exist. Their bodies are
   `func_0x802dcbfc(param_2, param_2, param_1)`, i.e. the same forwarding under
   Ghidra's permuted parameter names (`param_1 = interface`,
   `param_2 = object`).

## Corrections

**None.** No body in `work/wave200/laneD_block.rs` was wrong: all 79 offsets,
both register moves, the dispatcher target and the 48-byte sizes agree with the
ELF, `functions.tsv` and the manifest, so the generated block was left byte for
byte unchanged. The one residue worth knowing about is cosmetic: `0x806DBEA0`
writes its constant as `0xC` (uppercase nibble), which is the same style the
other wave-200 lanes use and compiles identically.

No file was edited other than adding the two audit outputs
(`ledger/agents/wave200-laned.csv` and this document). `src/game/bases.rs`, the
lane block and every other lane were left untouched.
