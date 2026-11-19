# Coverage - Wave 200 lane C (`game::bases` interface thunks)

Lane C of wave 200 audits the third block of the `0x802DCBFC` interface-thunk
family in `rel/d_basesNP.elf` (synthetic analysis base `0x80600000`; not a
runtime address). Every body in this lane was already generated, so this pass is
an audit, not a translation pass: the generated `interface_call` bodies were
checked word for word against the shipped REL, against
`work/ghidra/rel-d_basesNP/export/functions.tsv`, and against an independent
decoder (`work/thunks.cjs`).

| File | Module | Bodies | Range | High | Medium |
|---|---|---:|---|---:|---:|
| `work/wave200/laneC_block.rs` | `game::bases` | 79 | `0x80685680` .. `0x806C2FE0` | 79 | 0 |

Ledger rows: `ledger/agents/wave200-lanec.csv` (one row per address, all
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

Exactly two descriptor offsets occur in this lane, in strict alternation:

| `K` | Offset | Bodies |
|---|---|---:|
| `0x18` | 24 | 39 |
| `0x24` | 36 | 40 |

The lane opens on `0x24` (`0x80685680`) and closes on `0x24` (`0x806C2FE0`).
Run-length encoding the manifest in order gives 79 runs of length 1, alternating
`0x24, 0x18, 0x24, 0x18, ...`; the single extra `0x24` is the closing body.
No `0x0C` offset appears in this lane (it is confined to lane A).

## Verification method

1. **Whole-lane word decode.** All 79 addresses were read straight from the ELF
   (`work/rels/elf/d_basesNP.elf`, program headers, `u32` at vaddr) with
   `ppc159.js` (`LEN=13`, `STOP=0`). For each body all 12 words were compared
   against the expected sequence above, plus the 13th word to bound the body.
   The `bl` displacement was recomputed as `0x802DCBFC - (addr + 24)` from the
   raw word, so a wrong code base or a retargeted dispatcher could not pass
   silently.
2. **Rust body parse.** `work/wave200/laneC_block.rs` was parsed for
   `pub unsafe fn fn_<addr>(interface: usize, object: u32) -> u32` and its
   `interface_call(interface, object, 0x..)` constant; 79 bodies were found, in
   exactly the manifest order, with no duplicates or extras.
3. **Size cross-check.** `functions.tsv` reports `48` bytes for all 79
   addresses, matching the 12 decoded words. For every one of the 79, the word
   immediately after the `blr` is the first instruction of another function in
   `functions.tsv`, so no body was truncated. (Ten of those successors open
   `stwu r1,-32(r1)` rather than `-16`; that is the neighbour's own frame, not a
   continuation of the thunk.)
4. **Independent decoder cross-check.** `node work/thunks.cjs` decodes all 472
   wave-200 thunks through a different path (fixed `0x80600000`/file-offset
   mapping instead of program headers). All 79 lane C entries agree on family
   (`A`), source register (`r5`), offset and 48-byte size.
5. **Individual spot checks.** 17 addresses were disassembled one by one with
   `ppc159.js` (`LEN=12`, `STOP=0`), including the first and last of the lane
   plus both offset values: `0x80685680`, `0x806877F0`, `0x8068B380`,
   `0x80690150`, `0x806945E0`, `0x80699280`, `0x806A2260`, `0x806A9550`,
   `0x806AE0D0`, `0x806B0210`, `0x806B3390`, `0x806B8E10`, `0x806BA970`,
   `0x806BEB00`, `0x806C0B10`, `0x806C2FB0`, `0x806C2FE0`. All 17 had 12 words,
   `mr r5,r3`, `mr r3,r4`, a trailing `blr`, and `addi r12,r5,K` equal to the
   body's offset.
6. **Docstring consistency.** Each block's `# Original` line, evidence path,
   `# Safety`/`# Confidence` note and "interface + 0x.." prose were checked
   against the body's own constant: 79/79 agree (address, `FUN_` name, 48-byte
   size, `<addr>_FUN_<addr>.c` export path, offset).
7. **Evidence presence.** All 79 `work/ghidra/rel-d_basesNP/export/
   <addr>_FUN_<addr>.c` files exist. Their Ghidra bodies are the expected
   `func_0x802dcbfc(param_2, param_2, param_1)` call, i.e. the same forwarding
   with Ghidra's permuted parameter names.

## Corrections

**None.** No body in `work/wave200/laneC_block.rs` was wrong: all 79 offsets,
both register moves, the dispatcher target and the 48-byte sizes agree with the
ELF, `functions.tsv`, the independent decoder and the manifest. No file was
edited other than adding the two audit outputs
(`ledger/agents/wave200-lanec.csv` and this document). `src/game/bases.rs` and
the other lanes were left untouched; the bodies are not yet merged into
`bases.rs` (that is `work/wave200/merge.cjs`'s job).
